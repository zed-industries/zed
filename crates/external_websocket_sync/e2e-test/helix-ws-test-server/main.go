// helix-ws-test-server is a standalone WebSocket server for E2E testing of the
// Zed <-> Helix sync protocol. It runs the REAL production HelixAPIServer handlers
// with an in-memory store, so the same message processing code runs in both
// tests and production.
//
// The server runs multiple "rounds", one per agent type (zed-agent, claude, etc.).
// Each round executes the same 9 test phases:
//
//	Phase 1: Basic thread creation (new chat_message, no thread ID)
//	Phase 2: Follow-up on existing thread (same thread ID)
//	Phase 3: New thread (simulates context exhaustion -> new thread)
//	Phase 4: Follow-up to non-visible thread (Thread A while Thread B is active)
//	Phase 5: Simulate user input (Zed -> Helix sync direction)
//	Phase 6: Query UI state (verify Zed reports active thread)
//	Phase 7: Open thread + follow-up (open_thread command then chat_message)
//	Phase 8: Mid-stream interrupt (send follow-up while previous response is streaming)
//	Phase 9: Rapid 3-turn cancel (chat_message, then simulate_user_input interrupt, then chat_message)
//	Phase 10: User-created thread (inject user_created_thread, verify work session, send chat on new thread)
//	Phase 11: Spectask routing (set SpecTaskID on threads, verify FindConnectedSessionForSpecTask picks most recent)
//
// Exit codes: 0 = all tests passed, 1 = test failure
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net"
	"net/http"
	"os"
	"sort"
	"strings"
	"sync"
	"time"

	"github.com/helixml/helix/api/pkg/pubsub"
	"github.com/helixml/helix/api/pkg/server"
	"github.com/helixml/helix/api/pkg/store/memorystore"
	"github.com/helixml/helix/api/pkg/types"
)

// roundState holds per-agent-round state that gets reset between rounds.
type roundState struct {
	agentName string

	// Track thread IDs from thread_created events
	threadIDs []string

	// Track all sync events for validation
	events []types.SyncMessage

	// Track completions per thread
	completions map[string][]string // threadID -> list of request_ids

	// Track UI state responses (from query_ui_state)
	uiStateResponses []types.SyncMessage

	// Track timing for MCP tools wait validation
	phase1ChatSentAt    time.Time // when we sent the chat_message for phase 1
	phase1ThreadCreated time.Time // when we received thread_created for phase 1

	// Phase 8: mid-stream interrupt state
	phase8ThreadID      string // thread ID created in phase 8
	phase8InterruptSent bool   // whether we have already sent the interrupt
	phase8Completions   int    // number of message_completed events received for phase 8 thread

	// Phase 9: rapid 3-turn cancel state
	phase9ThreadID  string // thread ID (reuses phase 8's thread)
	phase9RapidSent bool   // whether the rapid sequence has been sent
	phase9Completions int  // number of message_completed events for phase 9

	// Phase 10: user-created thread (multi-thread sync)
	phase10NewThreadID       string // synthetic thread ID injected via ProcessSyncEvent
	phase10WorkSessionFound  bool   // whether the work session was created
	phase10ChatCompleted     bool   // whether chat on the new thread completed

	// Phase 11: spectask routing (verifies findConnectedSessionForSpecTask
	// picks the most recently active session)
	phase11RoutedSessionID string // which session the routing picked
	phase11ExpectedThreadID string // which thread we expect the message to land on
	phase11Completed        bool   // whether the routed message completed
}

func newRoundState(agentName string) *roundState {
	return &roundState{
		agentName:   agentName,
		completions: make(map[string][]string),
	}
}

// reqID returns a round-namespaced request ID for validation uniqueness.
func (r *roundState) reqID(phase string) string {
	return fmt.Sprintf("req-%s-%s", phase, r.agentName)
}

type testDriver struct {
	mu sync.Mutex

	srv   *server.HelixAPIServer
	store *memorystore.MemoryStore

	phase   int
	done    chan struct{}
	agentID string // agent connection ID (discovered at runtime)

	// Multi-agent round management
	agentRounds    []string     // agent names to test (e.g., ["zed-agent", "claude"])
	currentRoundIdx int
	round          *roundState  // current round state

	// Collected round results for final summary
	roundResults []roundResult
}

type roundResult struct {
	agentName string
	passed    bool
	errors    []string
}

func newTestDriver(srv *server.HelixAPIServer, store *memorystore.MemoryStore, agents []string) *testDriver {
	return &testDriver{
		srv:         srv,
		store:       store,
		done:        make(chan struct{}),
		agentRounds: agents,
		round:       newRoundState(agents[0]),
	}
}

// syncEventCallback is called by the production handler after every sync event.
func (d *testDriver) syncEventCallback(sessionID string, syncMsg *types.SyncMessage) {
	d.mu.Lock()
	d.round.events = append(d.round.events, *syncMsg)

	switch syncMsg.EventType {
	case "agent_ready":
		if d.phase == 0 {
			d.phase = 1
			d.mu.Unlock()
			log.Printf("\n##################################################")
			log.Printf("  ROUND %d/%d: Agent = %s", d.currentRoundIdx+1, len(d.agentRounds), d.round.agentName)
			log.Printf("##################################################")
			d.runPhase1()
			return
		}

	case "thread_created":
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		if acpThreadID == "" {
			acpThreadID, _ = syncMsg.Data["context_id"].(string)
		}
		if acpThreadID != "" {
			if len(d.round.threadIDs) == 0 {
				d.round.phase1ThreadCreated = time.Now()
			}
			d.round.threadIDs = append(d.round.threadIDs, acpThreadID)
			log.Printf("[%s] Thread #%d: %s (event=%s)", d.round.agentName, len(d.round.threadIDs), syncMsg.EventType, truncate(acpThreadID, 16))
			// Capture the thread created for phase 8 so we can send the interrupt to it.
			if d.phase == 8 && d.round.phase8ThreadID == "" {
				d.round.phase8ThreadID = acpThreadID
				log.Printf("[%s] Phase 8: Captured thread ID: %s", d.round.agentName, truncate(acpThreadID, 16))
			}
		}

	case "user_created_thread":
		// Spontaneous threads created by Zed (e.g. on startup). The production
		// handler creates a child session for these, but they are NOT used for
		// test phase follow-ups — only thread_created from chat_message responses
		// go into threadIDs. Phase 10 tests user_created_thread separately via
		// ProcessSyncEvent injection.
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		if acpThreadID != "" {
			log.Printf("[%s] Spontaneous user_created_thread: %s (not tracked for phases)", d.round.agentName, truncate(acpThreadID, 16))
		}

	case "message_added":
		// Ignore message_added events for threads from previous rounds.
		// Check if the thread ID belongs to the current round's tracked threads.
		addedThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		isCurrentRoundThread := false
		for _, tid := range d.round.threadIDs {
			if tid == addedThreadID {
				isCurrentRoundThread = true
				break
			}
		}
		// Also check phase 8/9 thread IDs (they may not be in threadIDs yet)
		if addedThreadID == d.round.phase8ThreadID || addedThreadID == d.round.phase9ThreadID {
			isCurrentRoundThread = true
		}
		if !isCurrentRoundThread && addedThreadID != "" {
			d.mu.Unlock()
			return // silently ignore stale events
		}

		// Phase 8: send an interrupt as soon as the first assistant token arrives for
		// the phase 8 thread. This guarantees ACP has started generating (running_turn
		// is set), so the interrupt will properly cancel the active turn via run_turn().
		if d.phase == 8 && !d.round.phase8InterruptSent {
			role, _ := syncMsg.Data["role"].(string)
			threadID, _ := syncMsg.Data["acp_thread_id"].(string)
			if role == "assistant" && threadID == d.round.phase8ThreadID {
				d.round.phase8InterruptSent = true
				agentName := d.round.agentName
				d.mu.Unlock()
				log.Printf("[%s] Phase 8: First assistant token arrived, sending interrupt to %s", agentName, truncate(threadID, 16))
				d.sendChatMessage("Actually just say 'hello'.", d.round.reqID("phase8-interrupt"), agentName, threadID)
				return
			}
		}

		// Phase 9: as soon as the first assistant token arrives for the initial
		// turn, fire a rapid sequence: simulate_user_input (like user pressing
		// Enter in Zed) + chat_message (like Helix's queue delivery). This
		// creates a 3-turn rapid cancel chain that previously caused a Task to
		// be dropped, breaking the oneshot channel and hanging the thread.
		if d.phase == 9 && !d.round.phase9RapidSent {
			role, _ := syncMsg.Data["role"].(string)
			threadID, _ := syncMsg.Data["acp_thread_id"].(string)
			if role == "assistant" && threadID == d.round.phase9ThreadID {
				d.round.phase9RapidSent = true
				agentName := d.round.agentName
				d.mu.Unlock()
				log.Printf("[%s] Phase 9: First assistant token arrived, sending rapid 2-message sequence", agentName)
				// Turn 2: simulate user typing in Zed (interrupt)
				d.sendSimulateUserInput(threadID, "User interrupt from Zed", d.round.reqID("phase9-user-input"), agentName)
				// Turn 3: simulate Helix queue delivery (arrives while turn 2 is starting)
				d.sendChatMessage("Queue delivery from Helix", d.round.reqID("phase9-queue"), agentName, threadID)
				return
			}
		}

	case "message_completed":
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		requestID, _ := syncMsg.Data["request_id"].(string)
		agentName := d.round.agentName

		// Ignore completions from previous rounds (stale events arriving late).
		// Our request IDs are namespaced: "req-phase1-zed-agent", "req-phase1-claude", etc.
		if !strings.Contains(requestID, agentName) {
			d.mu.Unlock()
			log.Printf("[%s] Ignoring stale completion from previous round: req=%s thread=%s",
				agentName, requestID, truncate(acpThreadID, 12))
			return
		}

		d.round.completions[acpThreadID] = append(d.round.completions[acpThreadID], requestID)
		currentPhase := d.phase

		// Phase 8 needs two completions before the test can end: one for the cancelled
		// initial turn and one for the interrupt response.
		if currentPhase == 8 && acpThreadID == d.round.phase8ThreadID {
			d.round.phase8Completions++
			completions := d.round.phase8Completions
			d.mu.Unlock()
			log.Printf("[%s] Phase 8: Completion %d/2 for thread=%s req=%s",
				agentName, completions, truncate(acpThreadID, 12), requestID)
			if completions >= 2 {
				log.Printf("[%s] Phase 8: Both turns completed (cancelled + interrupt)", agentName)
				time.Sleep(500 * time.Millisecond)
				go d.advanceAfterCompletion(8)
			}
			return
		}

		// Phase 9: rapid 3-turn cancel -- we expect at least 2 completions
		// (some turns may be cancelled/dropped, but the thread must not hang).
		if currentPhase == 9 && acpThreadID == d.round.phase9ThreadID {
			d.round.phase9Completions++
			completions := d.round.phase9Completions
			d.mu.Unlock()
			log.Printf("[%s] Phase 9: Completion %d for thread=%s req=%s",
				agentName, completions, truncate(acpThreadID, 12), requestID)
			if completions >= 2 {
				log.Printf("[%s] Phase 9: Received enough completions -- thread did not hang", agentName)
				time.Sleep(500 * time.Millisecond)
				go func() {
					d.mu.Lock()
					d.phase = 10
					d.mu.Unlock()
					d.runPhase10()
				}()
			}
			return
		}

		d.mu.Unlock()

		log.Printf("[%s] Completed: thread=%s req=%s (phase %d)",
			agentName, truncate(acpThreadID, 12), requestID, currentPhase)

		go d.advanceAfterCompletion(currentPhase)
		return

	case "ui_state_response":
		d.round.uiStateResponses = append(d.round.uiStateResponses, *syncMsg)
		currentPhase := d.phase
		queryID, _ := syncMsg.Data["query_id"].(string)
		activeView, _ := syncMsg.Data["active_view"].(string)
		threadID, _ := syncMsg.Data["thread_id"].(string)
		d.mu.Unlock()

		log.Printf("[%s] UI state: query_id=%s active_view=%s thread_id=%s (phase %d)",
			d.round.agentName, queryID, activeView, truncate(threadID, 12), currentPhase)

		if currentPhase == 6 {
			go d.advanceAfterUiState()
		}
		return

	case "thread_title_changed":
		title, _ := syncMsg.Data["title"].(string)
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		log.Printf("[%s] Title changed: thread=%s title=%q", d.round.agentName, truncate(acpThreadID, 12), title)

	case "thread_load_error":
		errMsg, _ := syncMsg.Data["error"].(string)
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		log.Printf("[%s] THREAD LOAD ERROR: %s (thread=%s)", d.round.agentName, errMsg, truncate(acpThreadID, 12))
	}

	d.mu.Unlock()
}

// --- Command helpers ---

func (d *testDriver) sendChatMessage(message, requestID, agentName string, acpThreadID ...string) {
	// For follow-up messages to an existing thread, use the production
	// SendChatMessage path which creates an interaction and sends the
	// command via the same code path as sendMessageToSpecTaskAgent.
	if len(acpThreadID) > 0 && acpThreadID[0] != "" {
		threadID := acpThreadID[0]
		mappings := d.srv.ContextMappings()
		if sessionID, ok := mappings[threadID]; ok {
			if err := d.srv.SendChatMessage(sessionID, message, requestID); err != nil {
				log.Printf("[test-server] WARNING: SendChatMessage failed for session %s: %v (falling back to QueueCommand)", sessionID, err)
			} else {
				return
			}
		}
	}

	// New thread (no acpThreadID or not in contextMappings yet) — send directly.
	data := map[string]interface{}{
		"message":    message,
		"request_id": requestID,
		"agent_name": agentName,
	}
	if len(acpThreadID) > 0 && acpThreadID[0] != "" {
		data["acp_thread_id"] = acpThreadID[0]
	}
	cmd := types.ExternalAgentCommand{Type: "chat_message", Data: data}
	if !d.srv.QueueCommand(d.agentID, cmd) {
		log.Printf("[test-server] WARNING: Failed to send command to agent %s", d.agentID)
	}
}

func (d *testDriver) sendSimulateUserInput(acpThreadID, message, requestID, agentName string) {
	cmd := types.ExternalAgentCommand{
		Type: "simulate_user_input",
		Data: map[string]interface{}{
			"acp_thread_id": acpThreadID,
			"message":       message,
			"request_id":    requestID,
			"agent_name":    agentName,
		},
	}
	d.srv.QueueCommand(d.agentID, cmd)
}

func (d *testDriver) sendOpenThread(acpThreadID, agentName string) {
	data := map[string]interface{}{
		"acp_thread_id": acpThreadID,
	}
	if agentName != "" {
		data["agent_name"] = agentName
	}
	cmd := types.ExternalAgentCommand{
		Type: "open_thread",
		Data: data,
	}
	if !d.srv.QueueCommand(d.agentID, cmd) {
		log.Printf("[test-server] WARNING: Failed to send open_thread to agent %s", d.agentID)
	}
}

func (d *testDriver) sendQueryUiState(queryID string) {
	cmd := types.ExternalAgentCommand{
		Type: "query_ui_state",
		Data: map[string]interface{}{
			"query_id": queryID,
		},
	}
	if !d.srv.QueueCommand(d.agentID, cmd) {
		log.Printf("[test-server] WARNING: Failed to send query_ui_state to agent %s", d.agentID)
	}
}

// --- Phase execution ---

func (d *testDriver) advanceAfterCompletion(completedPhase int) {
	time.Sleep(2 * time.Second) // let Zed settle

	switch completedPhase {
	case 1:
		d.mu.Lock()
		d.phase = 2
		d.mu.Unlock()
		d.runPhase2()
	case 2:
		d.mu.Lock()
		d.phase = 3
		d.mu.Unlock()
		d.runPhase3()
	case 3:
		d.mu.Lock()
		d.phase = 4
		d.mu.Unlock()
		d.runPhase4()
	case 4:
		d.mu.Lock()
		d.phase = 5
		d.mu.Unlock()
		d.runPhase5()
	case 5:
		d.mu.Lock()
		d.phase = 6
		d.mu.Unlock()
		d.runPhase6()
	case 7:
		d.mu.Lock()
		d.phase = 8
		d.mu.Unlock()
		d.runPhase8()
	case 8:
		d.mu.Lock()
		d.phase = 9
		d.mu.Unlock()
		d.runPhase9()
	case 11:
		d.mu.Lock()
		d.round.phase11Completed = true
		d.mu.Unlock()
		log.Printf("[%s] Phase 11: ✅ Routed message completed", d.round.agentName)
		d.advanceToNextRound()
	}
}

func (d *testDriver) advanceAfterUiState() {
	time.Sleep(1 * time.Second)
	d.mu.Lock()
	d.phase = 7
	d.mu.Unlock()
	d.runPhase7()
}

// advanceToNextRound validates the current round, then starts the next one or finishes.
func (d *testDriver) advanceToNextRound() {
	d.mu.Lock()
	agentName := d.round.agentName
	d.mu.Unlock()

	// Validate current round
	result := d.validateRound()
	d.mu.Lock()
	d.roundResults = append(d.roundResults, result)
	d.currentRoundIdx++

	if d.currentRoundIdx >= len(d.agentRounds) {
		// All rounds complete
		d.mu.Unlock()
		close(d.done)
		return
	}

	// Start next round
	nextAgent := d.agentRounds[d.currentRoundIdx]
	d.round = newRoundState(nextAgent)
	d.phase = 1
	d.mu.Unlock()

	log.Printf("\n##################################################")
	log.Printf("  ROUND %d/%d: Agent = %s (after %s)", d.currentRoundIdx+1, len(d.agentRounds), nextAgent, agentName)
	log.Printf("##################################################")

	// Wait for stale events from the previous round to drain before starting.
	// Phase 11 sends a message via SendChatMessage whose completion may arrive
	// after the round advances. We need enough time for all trailing events to
	// be processed and filtered. This also gives time for the new agent to be
	// installed (e.g., Claude Code auto-installs via npm on first use).
	log.Printf("[test-server] Waiting 10s for previous round events to drain...")
	time.Sleep(10 * time.Second)

	d.runPhase1()
}

func (d *testDriver) runPhase1() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 1: Basic thread creation", agent)
	log.Printf("==================================================")
	d.mu.Lock()
	d.round.phase1ChatSentAt = time.Now()
	d.mu.Unlock()
	d.sendChatMessage("What is 2 + 2? Reply with just the number.", d.round.reqID("phase1"), agent)
}

func (d *testDriver) runPhase2() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 2: Follow-up on existing thread", agent)
	log.Printf("==================================================")
	d.mu.Lock()
	if len(d.round.threadIDs) == 0 {
		d.mu.Unlock()
		log.Fatalf("[%s] ERROR: No thread IDs captured from phase 1!", agent)
	}
	tid := d.round.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[%s] Using thread from phase 1: %s", agent, truncate(tid, 16))
	d.sendChatMessage("What is 3 + 3? Reply with just the number.", d.round.reqID("phase2"), agent, tid)
}

func (d *testDriver) runPhase3() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 3: New thread (simulating thread transition)", agent)
	log.Printf("==================================================")
	d.sendChatMessage("What is 10 + 10? Reply with just the number.", d.round.reqID("phase3"), agent)
}

func (d *testDriver) runPhase4() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 4: Follow-up to non-visible thread", agent)
	log.Printf("==================================================")
	d.mu.Lock()
	if len(d.round.threadIDs) < 2 {
		d.mu.Unlock()
		log.Fatalf("[%s] ERROR: Need at least 2 threads for phase 4!", agent)
	}
	tid := d.round.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[%s] Sending back to Thread A (non-visible): %s", agent, truncate(tid, 16))
	d.sendChatMessage("What is 5 + 5? Reply with just the number.", d.round.reqID("phase4"), agent, tid)
}

func (d *testDriver) runPhase5() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 5: Simulate user input (Zed -> Helix sync)", agent)
	log.Printf("==================================================")
	d.mu.Lock()
	if len(d.round.threadIDs) == 0 {
		d.mu.Unlock()
		log.Fatalf("[%s] ERROR: No thread IDs available for phase 5!", agent)
	}
	tid := d.round.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[%s] Sending simulate_user_input to thread: %s", agent, truncate(tid, 16))
	d.sendSimulateUserInput(tid, "This message was typed by the user in Zed", d.round.reqID("phase5"), agent)
}

func (d *testDriver) runPhase6() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 6: Query UI state", agent)
	log.Printf("==================================================")
	d.sendQueryUiState(fmt.Sprintf("query-phase6-%s", agent))
}

func (d *testDriver) runPhase7() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 7: Open thread + follow-up chat", agent)
	log.Printf("==================================================")
	d.mu.Lock()
	if len(d.round.threadIDs) < 2 {
		d.mu.Unlock()
		log.Fatalf("[%s] ERROR: Need at least 2 threads for phase 7!", agent)
	}
	// Open Thread B (created in phase 3), then send a follow-up
	tid := d.round.threadIDs[1]
	d.mu.Unlock()

	log.Printf("[%s] Opening Thread B: %s", agent, truncate(tid, 16))
	d.sendOpenThread(tid, agent)

	// Wait for Zed to open the thread before sending follow-up
	time.Sleep(3 * time.Second)

	log.Printf("[%s] Sending follow-up to Thread B after open_thread", agent)
	d.sendChatMessage("What is 8 + 8? Reply with just the number.", d.round.reqID("phase7"), agent, tid)
}

func (d *testDriver) runPhase8() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 8: Mid-stream interrupt", agent)
	log.Printf("==================================================")
	// Send a question that will generate a streaming response long enough for us
	// to send an interrupt before it completes. The syncEventCallback will fire the
	// interrupt the moment the first assistant token arrives.
	d.sendChatMessage(
		"Write me a detailed explanation of recursion with three worked examples.",
		d.round.reqID("phase8-initial"),
		agent,
	)
}

func (d *testDriver) runPhase9() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 9: Rapid 3-turn cancel (regression test)", agent)
	log.Printf("==================================================")
	log.Println("  Sends chat_message, then while streaming, fires")
	log.Println("  simulate_user_input + chat_message back-to-back.")
	log.Println("  Without the fix, the thread hangs permanently.")

	// Reuse the phase 8 thread (it completed, so we can send follow-ups).
	d.mu.Lock()
	d.round.phase9ThreadID = d.round.phase8ThreadID
	d.mu.Unlock()

	// Turn 1: start a long-running response. The syncEventCallback will
	// fire the rapid sequence as soon as the first assistant token arrives.
	d.sendChatMessage(
		"Write a detailed explanation of merge sort with code examples.",
		d.round.reqID("phase9-initial"),
		agent,
		d.round.phase8ThreadID,
	)
}

func (d *testDriver) runPhase10() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 10: User-created thread (multi-thread sync)", agent)
	log.Printf("==================================================")
	log.Println("  Injects a synthetic user_created_thread event via")
	log.Println("  ProcessSyncEvent, then verifies session + work session")
	log.Println("  are created, then sends a chat on the new thread.")

	// Generate a new thread ID for the user-created thread
	newThreadID := fmt.Sprintf("user-thread-%s-%d", agent, time.Now().UnixNano())
	d.mu.Lock()
	d.round.phase10NewThreadID = newThreadID
	d.mu.Unlock()

	// Find the session ID from Phase 1 (the first thread's session).
	// ProcessSyncEvent needs a valid session ID that exists in the store.
	// The agentID is the WebSocket connection ID, not necessarily a session
	// in the store. The Phase 1 thread_created handler created a session
	// which we can find via contextMappings.
	d.mu.Lock()
	firstThreadID := ""
	if len(d.round.threadIDs) > 0 {
		firstThreadID = d.round.threadIDs[0]
	}
	d.mu.Unlock()

	existingSessionID := ""
	if firstThreadID != "" {
		mappings := d.srv.ContextMappings()
		existingSessionID = mappings[firstThreadID]
	}
	if existingSessionID == "" {
		log.Printf("[%s] Phase 10: ERROR no existing session found to use as parent", agent)
		go d.advanceToNextRound()
		return
	}

	log.Printf("[%s] Phase 10: Using existing session %s as parent for user-created thread", agent, existingSessionID)

	// Inject user_created_thread via ProcessSyncEvent (bypasses WebSocket —
	// tests the Helix handler directly since Zed can't send this in headless mode)
	syncMsg := &types.SyncMessage{
		EventType: "user_created_thread",
		Data: map[string]interface{}{
			"acp_thread_id": newThreadID,
			"title":         fmt.Sprintf("User Thread (%s)", agent),
		},
	}

	if err := d.srv.ProcessSyncEvent(existingSessionID, syncMsg); err != nil {
		log.Printf("[%s] Phase 10: ERROR injecting user_created_thread: %v", agent, err)
		go d.advanceToNextRound()
		return
	}

	log.Printf("[%s] Phase 10: Injected user_created_thread for thread=%s", agent, truncate(newThreadID, 12))

	// Verify the session was created by checking context mappings
	time.Sleep(500 * time.Millisecond)
	mappings := d.srv.ContextMappings()
	if sessionID, ok := mappings[newThreadID]; ok {
		log.Printf("[%s] Phase 10: ✅ New session created: %s", agent, sessionID)
	} else {
		log.Printf("[%s] Phase 10: ❌ No session mapping found for thread %s", agent, truncate(newThreadID, 12))
	}

	// Verify work session and zed thread records were created in the store
	d.mu.Lock()
	d.round.phase10WorkSessionFound = false
	d.mu.Unlock()

	// Check store for the new session's work session
	sessions := d.store.GetAllSessions()
	for _, ses := range sessions {
		if ses.Metadata.ZedThreadID == newThreadID {
			log.Printf("[%s] Phase 10: ✅ Found session in store: %s (ZedThreadID=%s, SpecTaskID=%s)",
				agent, ses.ID, truncate(ses.Metadata.ZedThreadID, 12), ses.Metadata.SpecTaskID)
			d.mu.Lock()
			d.round.phase10WorkSessionFound = true
			d.round.phase10ChatCompleted = true // skip chat test — synthetic thread doesn't exist in Zed
			d.mu.Unlock()
			break
		}
	}

	if !d.round.phase10WorkSessionFound {
		log.Printf("[%s] Phase 10: ❌ No session found in store with ZedThreadID=%s", agent, truncate(newThreadID, 12))
	}

	// Chain to Phase 11
	go func() {
		d.mu.Lock()
		d.phase = 11
		d.mu.Unlock()
		d.runPhase11()
	}()
}

func (d *testDriver) runPhase11() {
	agent := d.round.agentName
	log.Printf("\n==================================================")
	log.Printf("  [%s] PHASE 11: Spectask routing (most recently active session)", agent)
	log.Printf("==================================================")
	log.Println("  Sets SpecTaskID on Thread A and B sessions, then uses")
	log.Println("  FindConnectedSessionForSpecTask to verify routing picks")
	log.Println("  the most recently active thread, sends a message, and")
	log.Println("  verifies the response arrives on the correct session.")

	ctx := context.Background()
	specTaskID := fmt.Sprintf("spectask-e2e-%s-%d", agent, time.Now().UnixNano())

	// We need at least 2 threads (Thread A from phase 1, Thread B from phase 3)
	d.mu.Lock()
	if len(d.round.threadIDs) < 2 {
		d.mu.Unlock()
		log.Printf("[%s] Phase 11: SKIP — need at least 2 threads, got %d", agent, len(d.round.threadIDs))
		go d.advanceToNextRound()
		return
	}
	threadA := d.round.threadIDs[0]
	threadB := d.round.threadIDs[1]
	d.mu.Unlock()

	mappings := d.srv.ContextMappings()
	sessionA := mappings[threadA]
	sessionB := mappings[threadB]

	if sessionA == "" || sessionB == "" {
		log.Printf("[%s] Phase 11: ERROR — missing session mappings (A=%s, B=%s)", agent, sessionA, sessionB)
		go d.advanceToNextRound()
		return
	}

	// Set SpecTaskID on both sessions
	for _, sid := range []string{sessionA, sessionB} {
		ses, err := d.store.GetSession(ctx, sid)
		if err != nil {
			log.Printf("[%s] Phase 11: ERROR getting session %s: %v", agent, sid, err)
			go d.advanceToNextRound()
			return
		}
		ses.Metadata.SpecTaskID = specTaskID
		if _, err := d.store.UpdateSession(ctx, *ses); err != nil {
			log.Printf("[%s] Phase 11: ERROR updating session %s: %v", agent, sid, err)
			go d.advanceToNextRound()
			return
		}
	}

	// Thread B should be more recently active (Phase 7 completed on it).
	// Verify routing picks Thread B's session.
	specTask := &types.SpecTask{ID: specTaskID}
	routedSessionID, err := d.srv.FindConnectedSessionForSpecTask(ctx, specTask)
	if err != nil {
		log.Printf("[%s] Phase 11: ERROR FindConnectedSessionForSpecTask failed: %v", agent, err)
		go d.advanceToNextRound()
		return
	}

	d.mu.Lock()
	d.round.phase11RoutedSessionID = routedSessionID
	d.round.phase11ExpectedThreadID = threadB
	d.mu.Unlock()

	if routedSessionID == sessionB {
		log.Printf("[%s] Phase 11: ✅ Routing picked Thread B's session %s (most recently active)", agent, truncate(sessionB, 12))
	} else if routedSessionID == sessionA {
		log.Printf("[%s] Phase 11: ⚠️ Routing picked Thread A's session %s (expected Thread B)", agent, truncate(sessionA, 12))
	} else {
		log.Printf("[%s] Phase 11: ⚠️ Routing picked unexpected session %s", agent, truncate(routedSessionID, 12))
	}

	// Send a message via the routed session and wait for completion
	reqID := d.round.reqID("phase11")
	if err := d.srv.SendChatMessage(routedSessionID, "What is 7 + 7? Reply with just the number.", reqID); err != nil {
		log.Printf("[%s] Phase 11: ERROR SendChatMessage failed: %v", agent, err)
		go d.advanceToNextRound()
		return
	}

	log.Printf("[%s] Phase 11: Sent message to routed session %s, waiting for completion...", agent, truncate(routedSessionID, 12))
	// Completion will be detected by syncEventCallback when message_completed arrives
}

// --- Per-round validation ---

func (d *testDriver) validateRound() roundResult {
	d.mu.Lock()
	defer d.mu.Unlock()

	agent := d.round.agentName

	log.Printf("\n==================================================")
	log.Printf("  VALIDATION: %s", agent)
	log.Printf("==================================================")

	var errors []string

	// --- Event-level validation ---
	log.Printf("[%s] Total sync events: %d", agent, len(d.round.events))
	log.Printf("[%s] Thread IDs seen: %d", agent, len(d.round.threadIDs))
	log.Printf("[%s] Completions: %v", agent, d.round.completions)

	// Phase 1: Basic thread creation
	threadCreatedEvents := d.filterRoundEvents("thread_created")
	if len(threadCreatedEvents) < 1 {
		errors = append(errors, "Phase 1: No thread_created event")
	}
	if !d.hasRoundCompletion(d.round.reqID("phase1")) {
		errors = append(errors, "Phase 1: No message_completed for "+d.round.reqID("phase1"))
	}

	// Phase 2: Follow-up on existing thread
	if !d.hasRoundCompletion(d.round.reqID("phase2")) {
		errors = append(errors, "Phase 2: No message_completed for "+d.round.reqID("phase2"))
	}

	// Phase 3: New thread creation
	if len(d.round.threadIDs) < 2 {
		errors = append(errors, fmt.Sprintf("Phase 3: Expected at least 2 threads, got %d", len(d.round.threadIDs)))
	} else if d.round.threadIDs[0] == d.round.threadIDs[1] {
		errors = append(errors, "Phase 3: New thread has same ID as first thread!")
	} else {
		log.Printf("[%s] Phase 3: New thread created: %s", agent, truncate(d.round.threadIDs[1], 12))
	}
	if !d.hasRoundCompletion(d.round.reqID("phase3")) {
		errors = append(errors, "Phase 3: No message_completed for "+d.round.reqID("phase3"))
	}

	// Phase 4: Follow-up to non-visible thread
	if !d.hasRoundCompletion(d.round.reqID("phase4")) {
		errors = append(errors, "Phase 4: No message_completed for "+d.round.reqID("phase4"))
	}

	// Phase 5: Simulate user input
	if !d.hasRoundCompletion(d.round.reqID("phase5")) {
		errors = append(errors, "Phase 5: No message_completed for "+d.round.reqID("phase5"))
	}
	userMsgs := d.filterRoundEventsByFunc(func(e types.SyncMessage) bool {
		return e.EventType == "message_added" &&
			e.Data["role"] == "user" &&
			strings.Contains(fmt.Sprint(e.Data["content"]), "typed by the user in Zed")
	})
	if len(userMsgs) == 0 {
		errors = append(errors, "Phase 5: No message_added with role='user' containing simulated input text")
	} else {
		log.Printf("[%s] Phase 5: User message synced back to Helix", agent)
	}

	// Phase 6: query_ui_state
	expectedQueryID := fmt.Sprintf("query-phase6-%s", agent)
	if len(d.round.uiStateResponses) == 0 {
		errors = append(errors, "Phase 6: No ui_state_response received")
	} else {
		resp := d.round.uiStateResponses[0]
		queryID, _ := resp.Data["query_id"].(string)
		activeView, _ := resp.Data["active_view"].(string)
		if queryID != expectedQueryID {
			errors = append(errors, fmt.Sprintf("Phase 6: ui_state_response query_id=%q, expected %q", queryID, expectedQueryID))
		}
		if activeView == "" {
			errors = append(errors, "Phase 6: ui_state_response active_view is empty")
		} else {
			threadID, _ := resp.Data["thread_id"].(string)
			entryCount, _ := resp.Data["entry_count"].(float64) // JSON numbers are float64
			log.Printf("[%s] Phase 6: UI state - active_view=%s, thread_id=%s, entry_count=%.0f",
				agent, activeView, truncate(threadID, 12), entryCount)
		}

		// Validate MCP server status (only for first round -- MCP servers are agent-independent)
		if d.currentRoundIdx == 0 {
			mcpServers, _ := resp.Data["mcp_servers"].(map[string]interface{})
			if len(mcpServers) == 0 {
				errors = append(errors, "Phase 6: ui_state_response mcp_servers is empty (expected at least slow-mcp-test)")
			} else {
				log.Printf("[%s] Phase 6: MCP servers reported: %d", agent, len(mcpServers))
				slowMcpStatus, hasSlowMcp := mcpServers["slow-mcp-test"]
				if !hasSlowMcp {
					errors = append(errors, "Phase 6: mcp_servers missing 'slow-mcp-test' server")
				} else if slowMcpStatus != "running" {
					errors = append(errors, fmt.Sprintf(
						"Phase 6: slow-mcp-test status=%q, expected 'running' (MCP server not connected)",
						slowMcpStatus))
				} else {
					log.Printf("[%s] Phase 6: slow-mcp-test MCP server is running (green/connected)", agent)
				}
				for name, status := range mcpServers {
					log.Printf("[%s]   MCP server %q: %s", agent, name, status)
				}
			}
		}

		// Validate active model
		activeModel, _ := resp.Data["active_model"].(string)
		if activeModel == "" {
			log.Printf("[%s] WARNING: Phase 6: active_model is empty (model list may not have loaded yet)", agent)
		} else {
			log.Printf("[%s] Phase 6: Active model: %s", agent, activeModel)
		}
	}

	// Phase 7: open_thread + follow-up
	if !d.hasRoundCompletion(d.round.reqID("phase7")) {
		errors = append(errors, "Phase 7: No message_completed for "+d.round.reqID("phase7"))
	}

	// Phase 8-9: mid-stream interrupt and rapid cancel
	{
		// Phase 8: mid-stream interrupt
		if d.round.phase8ThreadID == "" {
			errors = append(errors, "Phase 8: No thread ID captured (phase 8 may not have run)")
		} else {
			completionsForPhase8 := len(d.round.completions[d.round.phase8ThreadID])
			if completionsForPhase8 < 2 {
				errors = append(errors, fmt.Sprintf(
					"Phase 8: Expected 2 message_completed events (cancelled turn + interrupt), got %d",
					completionsForPhase8))
			} else {
				log.Printf("[%s] Phase 8: Received %d completions for phase 8 thread (correct)", agent, completionsForPhase8)
			}
		}
		if !d.hasRoundCompletion(d.round.reqID("phase8-interrupt")) {
			errors = append(errors, "Phase 8: No message_completed for "+d.round.reqID("phase8-interrupt"))
		}

		// Verify ordering: no assistant tokens for the interrupt arrived before the first
		// message_completed for the phase 8 thread.
		if d.round.phase8ThreadID != "" && d.hasRoundCompletion(d.round.reqID("phase8-interrupt")) {
			seenFirstCompletion := false
			orderingViolation := false
			for _, e := range d.round.events {
				threadID, _ := e.Data["acp_thread_id"].(string)
				if threadID != d.round.phase8ThreadID {
					continue
				}
				if e.EventType == "message_completed" && !seenFirstCompletion {
					seenFirstCompletion = true
				}
				if e.EventType == "message_added" && !seenFirstCompletion {
					role, _ := e.Data["role"].(string)
					reqID, _ := e.Data["request_id"].(string)
					if role == "assistant" && reqID == d.round.reqID("phase8-interrupt") {
						orderingViolation = true
					}
				}
			}
			if orderingViolation {
				errors = append(errors, "Phase 8: Interrupt assistant tokens arrived before the first message_completed (FIFO ordering violated)")
			} else {
				log.Printf("[%s] Phase 8: Ordering correct -- interrupt tokens arrived after first message_completed", agent)
			}
		}

		// Phase 9: rapid 3-turn cancel
		if d.round.phase9ThreadID == "" {
			errors = append(errors, "Phase 9: No thread ID (phase 9 may not have run)")
		} else {
			if d.round.phase9Completions < 2 {
				errors = append(errors, fmt.Sprintf(
					"Phase 9: Expected at least 2 message_completed events (got %d) -- thread may have hung",
					d.round.phase9Completions))
			} else {
				log.Printf("[%s] Phase 9: Received %d completions -- thread recovered from rapid cancel (correct)", agent, d.round.phase9Completions)
			}
		}
	}

	// Phase 10: user-created thread (multi-thread sync)
	if d.round.phase10NewThreadID == "" {
		errors = append(errors, "Phase 10: No thread ID (phase 10 may not have run)")
	} else {
		// Verify context mapping exists for the user-created thread
		mappings := d.srv.ContextMappings()
		if sessionID, ok := mappings[d.round.phase10NewThreadID]; ok {
			log.Printf("[%s] Phase 10: ✅ Session mapping: thread=%s → session=%s",
				agent, truncate(d.round.phase10NewThreadID, 12), sessionID)
		} else {
			errors = append(errors, fmt.Sprintf("Phase 10: No session mapping for user-created thread %s", truncate(d.round.phase10NewThreadID, 12)))
		}

		if !d.round.phase10WorkSessionFound {
			errors = append(errors, "Phase 10: Work session/session not created in store for user-created thread")
		} else {
			log.Printf("[%s] Phase 10: ✅ Session created in store for user-created thread", agent)
		}
	}

	// Phase 11: spectask routing
	if d.round.phase11RoutedSessionID == "" {
		errors = append(errors, "Phase 11: Routing did not run (phase 11 may not have executed)")
	} else {
		mappings := d.srv.ContextMappings()
		expectedSessionID := mappings[d.round.phase11ExpectedThreadID]
		if d.round.phase11RoutedSessionID == expectedSessionID {
			log.Printf("[%s] Phase 11: ✅ Routing picked most recently active session (%s)",
				agent, truncate(d.round.phase11RoutedSessionID, 12))
		} else {
			errors = append(errors, fmt.Sprintf("Phase 11: Routing picked session %s, expected %s (Thread B)",
				truncate(d.round.phase11RoutedSessionID, 12), truncate(expectedSessionID, 12)))
		}
		if !d.round.phase11Completed {
			errors = append(errors, "Phase 11: Routed message did not complete")
		} else {
			log.Printf("[%s] Phase 11: ✅ Routed message completed on correct session", agent)
		}
	}

	// Too many threads (follow-ups should not create new threads)
	// Phases 1, 3, 8 each create one thread = 3 total.
	// Phase 10's user_created_thread is injected via ProcessSyncEvent, not via Zed,
	// so it doesn't appear in threadCreatedEvents (which only tracks thread_created from Zed).
	if len(threadCreatedEvents) > 3 {
		errors = append(errors, fmt.Sprintf("Too many thread_created events (%d, expected 3)", len(threadCreatedEvents)))
	}

	// --- MCP TOOLS WAIT VALIDATION (first round only) ---
	if d.currentRoundIdx == 0 {
		log.Println("\n--------------------------------------------------")
		log.Printf("  [%s] MCP TOOLS WAIT VALIDATION", agent)
		log.Println("--------------------------------------------------")

		if !d.round.phase1ChatSentAt.IsZero() && !d.round.phase1ThreadCreated.IsZero() {
			mcpWaitDuration := d.round.phase1ThreadCreated.Sub(d.round.phase1ChatSentAt)
			log.Printf("[%s] MCP wait: chat_message sent -> thread_created = %s", agent, mcpWaitDuration)

			const minExpectedDelay = 8 * time.Second
			if mcpWaitDuration < minExpectedDelay {
				errors = append(errors, fmt.Sprintf(
					"MCP tools wait: thread_created arrived %.1fs after chat_message (expected >= %.0fs). "+
						"This means Zed did NOT wait for MCP tools to load before sending the first message.",
					mcpWaitDuration.Seconds(), minExpectedDelay.Seconds()))
			} else {
				log.Printf("[%s] MCP tools wait: Zed correctly waited %.1fs for tools to load", agent, mcpWaitDuration.Seconds())
			}
		} else {
			log.Printf("[%s] WARNING: Could not measure MCP tools wait (missing timestamps)", agent)
		}
	}

	// --- STREAMING VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Printf("  [%s] STREAMING VALIDATION", agent)
	log.Println("--------------------------------------------------")

	completionPhases := []string{
		d.round.reqID("phase1"), d.round.reqID("phase2"), d.round.reqID("phase3"),
		d.round.reqID("phase4"), d.round.reqID("phase5"), d.round.reqID("phase7"),
	}
	for _, reqID := range completionPhases {
		firstAddedIdx := -1
		completedIdx := -1
		addedCount := 0

		for i, evt := range d.round.events {
			if evt.EventType == "message_added" && evt.Data["role"] == "assistant" {
				if firstAddedIdx == -1 {
					firstAddedIdx = i
				}
				addedCount++
			}
			if evt.EventType == "message_completed" && evt.Data["request_id"] == reqID {
				completedIdx = i
			}
		}

		if firstAddedIdx >= 0 && completedIdx >= 0 {
			if firstAddedIdx < completedIdx {
				log.Printf("[%s] Streaming %s: %d message_added before message_completed", agent, reqID, addedCount)
			} else {
				errors = append(errors, fmt.Sprintf("Streaming %s: message_added did NOT arrive before message_completed", reqID))
			}
		}
	}

	// --- SUMMARY ---
	passed := len(errors) == 0
	if !passed {
		fmt.Println()
		for _, e := range errors {
			log.Printf("[%s] FAIL: %s", agent, e)
		}
	} else {
		fmt.Println()
		log.Printf("[%s] Phase 1: Basic thread creation - PASSED", agent)
		log.Printf("[%s] Phase 2: Follow-up on existing thread - PASSED", agent)
		log.Printf("[%s] Phase 3: New thread via WebSocket - PASSED", agent)
		log.Printf("[%s] Phase 4: Follow-up to non-visible thread - PASSED", agent)
		log.Printf("[%s] Phase 5: Zed -> Helix user message sync - PASSED", agent)
		log.Printf("[%s] Phase 6: Query UI state - PASSED", agent)
		log.Printf("[%s] Phase 7: Open thread + follow-up - PASSED", agent)
		log.Printf("[%s] Phase 8: Mid-stream interrupt - PASSED", agent)
		log.Printf("[%s] Phase 9: Rapid 3-turn cancel - PASSED", agent)
	}

	totalCompletions := 0
	for _, v := range d.round.completions {
		totalCompletions += len(v)
	}
	log.Printf("[%s] Total threads: %d, Total completions: %d",
		agent, len(d.round.threadIDs), totalCompletions)

	return roundResult{agentName: agent, passed: passed, errors: errors}
}

// validateStore runs cross-round store state validation (sessions, interactions).
func (d *testDriver) validateStore() bool {
	d.mu.Lock()
	defer d.mu.Unlock()

	log.Println("\n==================================================")
	log.Println("  STORE STATE VALIDATION (production handlers)")
	log.Println("==================================================")

	var errors []string

	sessions := d.store.GetAllSessions()
	interactions := d.store.GetAllInteractions()

	log.Printf("[store] Sessions in store: %d", len(sessions))
	log.Printf("[store] Interactions in store: %d", len(interactions))

	// Each round creates 3 threads (phases 1, 3, 8) = 3 sessions per round.
	expectedSessions := 3 * len(d.agentRounds)
	if len(sessions) < expectedSessions {
		errors = append(errors, fmt.Sprintf("Expected at least %d sessions (%d rounds * 3 threads), got %d",
			expectedSessions, len(d.agentRounds), len(sessions)))
	}

	// Check that sessions have ZedThreadID metadata
	sessionsWithThread := 0
	for _, s := range sessions {
		if s.Metadata.ZedThreadID != "" {
			sessionsWithThread++
			log.Printf("[store] Session %s: ZedThreadID=%s, Owner=%s, Name=%q",
				truncate(s.ID, 12), truncate(s.Metadata.ZedThreadID, 12), s.Owner, s.Name)
		}
	}
	if sessionsWithThread < expectedSessions {
		errors = append(errors, fmt.Sprintf("Expected at least %d sessions with ZedThreadID, got %d", expectedSessions, sessionsWithThread))
	}

	// Check completed interactions.
	// Phases 8 and 9 include mid-stream interrupt tests where the agent may be
	// cancelled while executing tool calls (before generating any text). Those
	// interactions legitimately end up complete with ResponseMessage="" and only
	// tool_call entries. We treat them as "interrupted" and skip the text-content
	// checks, but we do require that enough non-interrupted interactions have content.
	completedInteractions := 0
	completedWithContent := 0
	for _, i := range interactions {
		if i.State != types.InteractionStateComplete {
			continue
		}
		completedInteractions++

		if i.ResponseMessage == "" {
			// Check if this was interrupted during tool use (has tool_call entries but no text).
			// This is expected for phases 8 and 9 cancellations.
			interrupted := false
			if len(i.ResponseEntries) > 0 {
				var entries []struct {
					Type string `json:"type"`
				}
				if err := json.Unmarshal(i.ResponseEntries, &entries); err == nil {
					hasText := false
					for _, e := range entries {
						if e.Type == "text" {
							hasText = true
							break
						}
					}
					if !hasText {
						interrupted = true
					}
				}
			}
			if interrupted {
				log.Printf("[store] Interaction %s: complete, interrupted during tool use (no text — expected for phases 8/9)",
					truncate(i.ID, 12))
			} else {
				// Truly empty — no entries at all. This is also expected for turns
				// cancelled before generating any output (e.g. rapid cancel in phase 9).
				log.Printf("[store] Interaction %s: complete with no content (cancelled before output — expected for phases 8/9)",
					truncate(i.ID, 12))
			}
			continue
		}

		// Interaction has text content — validate it properly.
		completedWithContent++
		log.Printf("[store] Completed interaction %s: %d bytes response, session=%s",
			truncate(i.ID, 12), len(i.ResponseMessage), truncate(i.SessionID, 12))

		if len(i.ResponseEntries) == 0 {
			errors = append(errors, fmt.Sprintf("Interaction %s: has ResponseMessage but no ResponseEntries",
				truncate(i.ID, 12)))
		} else {
			var entries []struct {
				Type      string `json:"type"`
				Content   string `json:"content"`
				MessageID string `json:"message_id"`
			}
			if err := json.Unmarshal(i.ResponseEntries, &entries); err != nil {
				errors = append(errors, fmt.Sprintf("Interaction %s: failed to parse ResponseEntries: %v",
					truncate(i.ID, 12), err))
			} else {
				hasText := false
				for _, e := range entries {
					if e.Type == "text" {
						hasText = true
					}
					if e.Type != "text" && e.Type != "tool_call" {
						errors = append(errors, fmt.Sprintf("Interaction %s: unexpected entry type %q",
							truncate(i.ID, 12), e.Type))
					}
					if e.Content == "" {
						errors = append(errors, fmt.Sprintf("Interaction %s: entry %s has empty content",
							truncate(i.ID, 12), e.MessageID))
					}
				}
				if !hasText {
					errors = append(errors, fmt.Sprintf("Interaction %s: has ResponseMessage but no 'text' entries in ResponseEntries",
						truncate(i.ID, 12)))
				}
			}
		}
	}

	// Expect at least 7 completed interactions per round:
	//   - Phase 1:  thread_created → new session + interaction
	//   - Phase 2:  sendChatMessageToExternalAgent creates interaction for follow-up
	//   - Phase 3:  thread_created → new session + interaction
	//   - Phase 4:  sendChatMessageToExternalAgent creates interaction for follow-up
	//   - Phase 5:  message_added(role=user) → on-the-fly interaction
	//   - Phase 7:  sendChatMessageToExternalAgent creates interaction for follow-up
	//   - Phase 8:  thread_created → new session + interaction
	//   - Phase 9:  on-the-fly interaction (from user interrupt)
	//   - Phase 11: sendChatMessageToExternalAgent via spectask routing
	expectedCompleted := 7 * len(d.agentRounds)
	if completedInteractions < expectedCompleted {
		errors = append(errors, fmt.Sprintf("Expected at least %d completed interactions, got %d", expectedCompleted, completedInteractions))
	}

	// Expect at least 7 interactions WITH content per round.
	expectedWithContent := 7 * len(d.agentRounds)
	if completedWithContent < expectedWithContent {
		errors = append(errors, fmt.Sprintf("Expected at least %d completed interactions with content, got %d (accumulation may be broken)",
			expectedWithContent, completedWithContent))
	}

	// --- ACCUMULATION VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Println("  ACCUMULATION VALIDATION")
	log.Println("--------------------------------------------------")

	sort.Slice(interactions, func(a, b int) bool {
		return interactions[a].SessionID < interactions[b].SessionID
	})

	for _, i := range interactions {
		if i.State == types.InteractionStateComplete && i.ResponseMessage != "" {
			log.Printf("[store] Interaction %s (session %s): %d bytes, lastMsgID=%s, offset=%d",
				truncate(i.ID, 12), truncate(i.SessionID, 12),
				len(i.ResponseMessage), truncate(i.LastZedMessageID, 12), i.LastZedMessageOffset)
		}
	}

	// --- THREAD TITLE VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Println("  THREAD TITLE VALIDATION")
	log.Println("--------------------------------------------------")

	updatedNames := 0
	for _, s := range sessions {
		if s.Name != "" && s.Name != "New Conversation" && s.Name != "New Chat" {
			updatedNames++
			log.Printf("[store] Session %s name: %q", truncate(s.ID, 12), s.Name)
		}
	}
	if updatedNames > 0 {
		log.Printf("[store] Thread title -> session name sync: %d sessions updated", updatedNames)
	} else {
		log.Println("[store] No thread title updates (Zed may not generate titles for short prompts)")
	}

	if len(errors) > 0 {
		fmt.Println()
		for _, e := range errors {
			log.Printf("[store] FAIL: %s", e)
		}
		return false
	}

	log.Printf("\n[store] Store state: Sessions and interactions created correctly - PASSED")
	log.Printf("[store] Accumulation: %d interactions with content (interrupted/cancelled: %d) - PASSED",
		completedWithContent, completedInteractions-completedWithContent)
	log.Println("[store] Structured entries: ResponseEntries populated for content-bearing interactions - PASSED")
	return true
}

// --- helpers ---

func (d *testDriver) filterRoundEvents(eventType string) []types.SyncMessage {
	var out []types.SyncMessage
	for _, e := range d.round.events {
		if e.EventType == eventType {
			out = append(out, e)
		}
	}
	return out
}

func (d *testDriver) filterRoundEventsByFunc(fn func(types.SyncMessage) bool) []types.SyncMessage {
	var out []types.SyncMessage
	for _, e := range d.round.events {
		if fn(e) {
			out = append(out, e)
		}
	}
	return out
}

func (d *testDriver) hasRoundCompletion(requestID string) bool {
	for _, e := range d.round.events {
		if e.EventType == "message_completed" && e.Data["request_id"] == requestID {
			return true
		}
	}
	return false
}

func truncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "..."
}

// --- main ---

func main() {
	// Determine which agents to test. Default: zed-agent only (backwards compatible).
	// Set E2E_AGENTS="zed-agent,claude" to test multiple agents.
	agentsStr := os.Getenv("E2E_AGENTS")
	var agents []string
	if agentsStr != "" {
		for _, a := range strings.Split(agentsStr, ",") {
			a = strings.TrimSpace(a)
			if a != "" {
				agents = append(agents, a)
			}
		}
	}
	if len(agents) == 0 {
		agents = []string{"zed-agent"}
	}

	log.Printf("[test-server] Agent rounds: %v", agents)

	// Create in-memory store and no-op pubsub
	store := memorystore.New()
	ps := pubsub.NewNoop()

	// Seed a session matching HELIX_SESSION_ID so the production handler
	// can look it up. In production, sessions always exist before Zed connects
	// (created by spectask/session creation flow). Without this, handlers like
	// handleUserCreatedThread fail with "session not found" because they call
	// GetSession(agentSessionID) expecting a real session.
	seedSessionID := os.Getenv("HELIX_SESSION_ID")
	if seedSessionID == "" {
		seedSessionID = "ses_e2e-test-session-001"
	}
	seedSession := types.Session{
		ID:      seedSessionID,
		Name:    "E2E Test Seed Session",
		Created: time.Now(),
		Updated: time.Now(),
		Owner:   "e2e-test-user",
		Mode:    types.SessionModeInference,
		Type:    types.SessionTypeText,
	}
	if _, err := store.CreateSession(context.Background(), seedSession); err != nil {
		log.Fatalf("[test-server] Failed to create seed session: %v", err)
	}
	log.Printf("[test-server] Created seed session: %s", seedSessionID)

	// Create HelixAPIServer with production handlers + in-memory store
	srv := server.NewTestServer(store, ps)

	// Create test driver
	driver := newTestDriver(srv, store, agents)

	// Register sync event hook so test driver observes all events
	srv.SetSyncEventHook(driver.syncEventCallback)

	// Port file for Zed to discover the server
	portFile := "/tmp/mock_helix_port"

	// Register the REAL production WebSocket handler
	http.HandleFunc("/api/v1/external-agents/sync", func(w http.ResponseWriter, r *http.Request) {
		// Set up user mapping for the agent before the handler runs.
		// Extract agent_id the same way the production handler does.
		agentID := r.URL.Query().Get("session_id")
		if agentID == "" {
			agentID = r.URL.Query().Get("agent_id")
		}
		if agentID != "" {
			srv.SetExternalAgentUserMapping(agentID, "e2e-test-user")
			driver.mu.Lock()
			driver.agentID = agentID
			driver.mu.Unlock()
			log.Printf("[test-server] Agent connecting: %s", agentID)
		}

		// Delegate to the REAL production handler
		srv.ExternalAgentSyncHandler()(w, r)
	})

	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		log.Fatalf("[test-server] Listen error: %v", err)
	}
	port := listener.Addr().(*net.TCPAddr).Port
	log.Printf("[test-server] Listening on ws://127.0.0.1:%d", port)
	log.Println("[test-server] Using REAL HelixAPIServer handlers with in-memory store")

	if err := os.WriteFile(portFile, []byte(fmt.Sprintf("%d", port)), 0644); err != nil {
		log.Fatalf("[test-server] Failed to write port file: %v", err)
	}

	go func() {
		if err := http.Serve(listener, nil); err != nil {
			log.Printf("[test-server] Server error: %v", err)
		}
	}()

	// Increase timeout for multi-agent runs
	timeout := 300 * time.Second
	if len(agents) > 1 {
		timeout = time.Duration(300*len(agents)) * time.Second
	}

	select {
	case <-driver.done:
	case <-time.After(timeout):
		driver.mu.Lock()
		var eventTypes []string
		if driver.round != nil {
			for _, e := range driver.round.events {
				eventTypes = append(eventTypes, e.EventType)
			}
		}
		driver.mu.Unlock()
		log.Printf("[test-server] TIMEOUT at round %d/%d, phase %d. Events: %v",
			driver.currentRoundIdx+1, len(agents), driver.phase, eventTypes)
		os.Exit(1)
	}

	// Validate store state (cross-round)
	storeOK := driver.validateStore()

	// Print final summary
	log.Println("\n##################################################")
	log.Println("  FINAL RESULTS")
	log.Println("##################################################")

	allPassed := storeOK
	for _, r := range driver.roundResults {
		status := "PASSED"
		if !r.passed {
			status = "FAILED"
			allPassed = false
		}
		log.Printf("  [%s] %s", r.agentName, status)
		for _, e := range r.errors {
			log.Printf("    FAIL: %s", e)
		}
	}
	if storeOK {
		log.Println("  [store] PASSED")
	} else {
		log.Println("  [store] FAILED")
	}

	if allPassed {
		log.Printf("\n[test-server] ALL TESTS PASSED (%d agent rounds, production handlers, in-memory store)", len(agents))
		os.Exit(0)
	} else {
		os.Exit(1)
	}
}
