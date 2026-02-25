// helix-ws-test-server is a standalone WebSocket server for E2E testing of the
// Zed <-> Helix sync protocol. It runs the REAL production HelixAPIServer handlers
// with an in-memory store, so the same message processing code runs in both
// tests and production.
//
// The server implements 7 phases:
//
//	Phase 1: Basic thread creation (new chat_message, no thread ID)
//	Phase 2: Follow-up on existing thread (same thread ID)
//	Phase 3: New thread (simulates context exhaustion -> new thread)
//	Phase 4: Follow-up to non-visible thread (Thread A while Thread B is active)
//	Phase 5: Simulate user input (Zed -> Helix sync direction)
//	Phase 6: Query UI state (verify Zed reports active thread)
//	Phase 7: Open thread + follow-up (open_thread command then chat_message)
//
// Exit codes: 0 = all tests passed, 1 = test failure
package main

import (
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

type testDriver struct {
	mu sync.Mutex

	srv   *server.HelixAPIServer
	store *memorystore.MemoryStore

	phase   int
	done    chan struct{}
	agentID string // agent connection ID (discovered at runtime)

	// Track thread IDs from thread_created events
	threadIDs []string

	// Track all sync events for validation
	events []types.SyncMessage

	// Track completions per thread
	completions map[string][]string // threadID -> list of request_ids

	// Track UI state responses (from query_ui_state)
	uiStateResponses []types.SyncMessage
}

func newTestDriver(srv *server.HelixAPIServer, store *memorystore.MemoryStore) *testDriver {
	return &testDriver{
		srv:         srv,
		store:       store,
		completions: make(map[string][]string),
		done:        make(chan struct{}),
	}
}

// syncEventCallback is called by the production handler after every sync event.
func (d *testDriver) syncEventCallback(sessionID string, syncMsg *types.SyncMessage) {
	d.mu.Lock()
	d.events = append(d.events, *syncMsg)

	switch syncMsg.EventType {
	case "agent_ready":
		if d.phase == 0 {
			d.phase = 1
			d.mu.Unlock()
			d.runPhase1()
			return
		}

	case "thread_created", "user_created_thread":
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		if acpThreadID == "" {
			acpThreadID, _ = syncMsg.Data["context_id"].(string)
		}
		if acpThreadID != "" {
			d.threadIDs = append(d.threadIDs, acpThreadID)
			log.Printf("[test-server] Thread #%d: %s (event=%s)", len(d.threadIDs), truncate(acpThreadID, 16), syncMsg.EventType)
		}

	case "message_completed":
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		requestID, _ := syncMsg.Data["request_id"].(string)
		d.completions[acpThreadID] = append(d.completions[acpThreadID], requestID)
		currentPhase := d.phase
		d.mu.Unlock()

		log.Printf("[test-server] Completed: thread=%s req=%s (phase %d)",
			truncate(acpThreadID, 12), requestID, currentPhase)

		go d.advanceAfterCompletion(currentPhase)
		return

	case "ui_state_response":
		d.uiStateResponses = append(d.uiStateResponses, *syncMsg)
		currentPhase := d.phase
		queryID, _ := syncMsg.Data["query_id"].(string)
		activeView, _ := syncMsg.Data["active_view"].(string)
		threadID, _ := syncMsg.Data["thread_id"].(string)
		d.mu.Unlock()

		log.Printf("[test-server] UI state: query_id=%s active_view=%s thread_id=%s (phase %d)",
			queryID, activeView, truncate(threadID, 12), currentPhase)

		if currentPhase == 6 {
			go d.advanceAfterUiState()
		}
		return

	case "thread_title_changed":
		title, _ := syncMsg.Data["title"].(string)
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		log.Printf("[test-server] Title changed: thread=%s title=%q", truncate(acpThreadID, 12), title)

	case "thread_load_error":
		errMsg, _ := syncMsg.Data["error"].(string)
		acpThreadID, _ := syncMsg.Data["acp_thread_id"].(string)
		log.Printf("[test-server] THREAD LOAD ERROR: %s (thread=%s)", errMsg, truncate(acpThreadID, 12))
	}

	d.mu.Unlock()
}

// --- Command helpers ---

func (d *testDriver) sendChatMessage(message, requestID, agentName string, acpThreadID ...string) {
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

func (d *testDriver) sendOpenThread(acpThreadID string) {
	cmd := types.ExternalAgentCommand{
		Type: "open_thread",
		Data: map[string]interface{}{
			"acp_thread_id": acpThreadID,
		},
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
		time.Sleep(500 * time.Millisecond)
		close(d.done)
	}
}

func (d *testDriver) advanceAfterUiState() {
	time.Sleep(1 * time.Second)
	d.mu.Lock()
	d.phase = 7
	d.mu.Unlock()
	d.runPhase7()
}

func (d *testDriver) runPhase1() {
	log.Println("\n==================================================")
	log.Println("  PHASE 1: Basic thread creation")
	log.Println("==================================================")
	d.sendChatMessage("What is 2 + 2? Reply with just the number.", "req-phase1", "zed-agent")
}

func (d *testDriver) runPhase2() {
	log.Println("\n==================================================")
	log.Println("  PHASE 2: Follow-up on existing thread")
	log.Println("==================================================")
	d.mu.Lock()
	if len(d.threadIDs) == 0 {
		d.mu.Unlock()
		log.Fatal("[test-server] ERROR: No thread IDs captured from phase 1!")
	}
	tid := d.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[test-server] Using thread from phase 1: %s", truncate(tid, 16))
	d.sendChatMessage("What is 3 + 3? Reply with just the number.", "req-phase2", "zed-agent", tid)
}

func (d *testDriver) runPhase3() {
	log.Println("\n==================================================")
	log.Println("  PHASE 3: New thread (simulating thread transition)")
	log.Println("==================================================")
	d.sendChatMessage("What is 10 + 10? Reply with just the number.", "req-phase3", "zed-agent")
}

func (d *testDriver) runPhase4() {
	log.Println("\n==================================================")
	log.Println("  PHASE 4: Follow-up to non-visible thread")
	log.Println("==================================================")
	d.mu.Lock()
	if len(d.threadIDs) < 2 {
		d.mu.Unlock()
		log.Fatal("[test-server] ERROR: Need at least 2 threads for phase 4!")
	}
	tid := d.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[test-server] Sending back to Thread A (non-visible): %s", truncate(tid, 16))
	d.sendChatMessage("What is 5 + 5? Reply with just the number.", "req-phase4", "zed-agent", tid)
}

func (d *testDriver) runPhase5() {
	log.Println("\n==================================================")
	log.Println("  PHASE 5: Simulate user input (Zed -> Helix sync)")
	log.Println("==================================================")
	d.mu.Lock()
	if len(d.threadIDs) == 0 {
		d.mu.Unlock()
		log.Fatal("[test-server] ERROR: No thread IDs available for phase 5!")
	}
	tid := d.threadIDs[0]
	d.mu.Unlock()

	log.Printf("[test-server] Sending simulate_user_input to thread: %s", truncate(tid, 16))
	d.sendSimulateUserInput(tid, "This message was typed by the user in Zed", "req-phase5", "zed-agent")
}

func (d *testDriver) runPhase6() {
	log.Println("\n==================================================")
	log.Println("  PHASE 6: Query UI state")
	log.Println("==================================================")
	d.sendQueryUiState("query-phase6")
}

func (d *testDriver) runPhase7() {
	log.Println("\n==================================================")
	log.Println("  PHASE 7: Open thread + follow-up chat")
	log.Println("==================================================")
	d.mu.Lock()
	if len(d.threadIDs) < 2 {
		d.mu.Unlock()
		log.Fatal("[test-server] ERROR: Need at least 2 threads for phase 7!")
	}
	// Open Thread B (created in phase 3), then send a follow-up
	tid := d.threadIDs[1]
	d.mu.Unlock()

	log.Printf("[test-server] Opening Thread B: %s", truncate(tid, 16))
	d.sendOpenThread(tid)

	// Wait for Zed to open the thread before sending follow-up
	time.Sleep(3 * time.Second)

	log.Printf("[test-server] Sending follow-up to Thread B after open_thread")
	d.sendChatMessage("What is 8 + 8? Reply with just the number.", "req-phase7", "zed-agent", tid)
}

// --- Validation ---

func (d *testDriver) validate() bool {
	d.mu.Lock()
	defer d.mu.Unlock()

	log.Println("\n==================================================")
	log.Println("  VALIDATION")
	log.Println("==================================================")

	var errors []string

	// --- Event-level validation ---
	log.Printf("[test-server] Total sync events: %d", len(d.events))
	log.Printf("[test-server] Thread IDs seen: %d", len(d.threadIDs))
	log.Printf("[test-server] Completions: %v", d.completions)

	// Phase 1: Basic thread creation
	threadCreatedEvents := d.filterEvents("thread_created")
	if len(threadCreatedEvents) < 1 {
		errors = append(errors, "Phase 1: No thread_created event")
	}
	if !d.hasCompletion("req-phase1") {
		errors = append(errors, "Phase 1: No message_completed for req-phase1")
	}

	// Phase 2: Follow-up on existing thread
	if !d.hasCompletion("req-phase2") {
		errors = append(errors, "Phase 2: No message_completed for req-phase2")
	}

	// Phase 3: New thread creation
	if len(d.threadIDs) < 2 {
		errors = append(errors, fmt.Sprintf("Phase 3: Expected at least 2 threads, got %d", len(d.threadIDs)))
	} else if d.threadIDs[0] == d.threadIDs[1] {
		errors = append(errors, "Phase 3: New thread has same ID as first thread!")
	} else {
		log.Printf("[test-server] Phase 3: New thread created: %s", truncate(d.threadIDs[1], 12))
	}
	if !d.hasCompletion("req-phase3") {
		errors = append(errors, "Phase 3: No message_completed for req-phase3")
	}

	// Phase 4: Follow-up to non-visible thread
	if !d.hasCompletion("req-phase4") {
		errors = append(errors, "Phase 4: No message_completed for req-phase4")
	}

	// Phase 5: Simulate user input
	if !d.hasCompletion("req-phase5") {
		errors = append(errors, "Phase 5: No message_completed for req-phase5")
	}
	userMsgs := d.filterEventsByFunc(func(e types.SyncMessage) bool {
		return e.EventType == "message_added" &&
			e.Data["role"] == "user" &&
			strings.Contains(fmt.Sprint(e.Data["content"]), "typed by the user in Zed")
	})
	if len(userMsgs) == 0 {
		errors = append(errors, "Phase 5: No message_added with role='user' containing simulated input text")
	} else {
		log.Println("[test-server] Phase 5: User message synced back to Helix")
	}

	// Phase 6: query_ui_state
	if len(d.uiStateResponses) == 0 {
		errors = append(errors, "Phase 6: No ui_state_response received")
	} else {
		resp := d.uiStateResponses[0]
		queryID, _ := resp.Data["query_id"].(string)
		activeView, _ := resp.Data["active_view"].(string)
		if queryID != "query-phase6" {
			errors = append(errors, fmt.Sprintf("Phase 6: ui_state_response query_id=%q, expected 'query-phase6'", queryID))
		}
		if activeView == "" {
			errors = append(errors, "Phase 6: ui_state_response active_view is empty")
		} else {
			threadID, _ := resp.Data["thread_id"].(string)
			entryCount, _ := resp.Data["entry_count"].(float64) // JSON numbers are float64
			log.Printf("[test-server] Phase 6: UI state - active_view=%s, thread_id=%s, entry_count=%.0f",
				activeView, truncate(threadID, 12), entryCount)
		}
	}

	// Phase 7: open_thread + follow-up
	if !d.hasCompletion("req-phase7") {
		errors = append(errors, "Phase 7: No message_completed for req-phase7")
	}

	// Too many threads (follow-ups should not create new threads)
	// Phases 1, 3 each create one thread = 2 threads total.
	// Phases 2, 4, 5, 7 use existing threads.
	if len(threadCreatedEvents) > 2 {
		errors = append(errors, fmt.Sprintf("Too many thread_created events (%d, expected 2)", len(threadCreatedEvents)))
	}

	// --- STORE STATE VALIDATION (production handlers actually worked) ---
	log.Println("\n--------------------------------------------------")
	log.Println("  STORE STATE VALIDATION (production handlers)")
	log.Println("--------------------------------------------------")

	sessions := d.store.GetAllSessions()
	interactions := d.store.GetAllInteractions()

	log.Printf("[test-server] Sessions in store: %d", len(sessions))
	log.Printf("[test-server] Interactions in store: %d", len(interactions))

	if len(sessions) < 2 {
		errors = append(errors, fmt.Sprintf("Expected at least 2 sessions (Thread A + Thread B), got %d", len(sessions)))
	}

	// Check that sessions have ZedThreadID metadata
	sessionsWithThread := 0
	for _, s := range sessions {
		if s.Metadata.ZedThreadID != "" {
			sessionsWithThread++
			log.Printf("[test-server] Session %s: ZedThreadID=%s, Owner=%s, Name=%q",
				truncate(s.ID, 12), truncate(s.Metadata.ZedThreadID, 12), s.Owner, s.Name)
		}
	}
	if sessionsWithThread < 2 {
		errors = append(errors, fmt.Sprintf("Expected at least 2 sessions with ZedThreadID, got %d", sessionsWithThread))
	}

	// Check completed interactions have non-empty ResponseMessage
	completedInteractions := 0
	for _, i := range interactions {
		if i.State == types.InteractionStateComplete {
			completedInteractions++
			if i.ResponseMessage == "" {
				errors = append(errors, fmt.Sprintf("Interaction %s: complete but empty ResponseMessage (accumulation bug!)",
					truncate(i.ID, 12)))
			} else {
				log.Printf("[test-server] Completed interaction %s: %d bytes response, session=%s",
					truncate(i.ID, 12), len(i.ResponseMessage), truncate(i.SessionID, 12))
			}
		}
	}

	// HARD FAIL: We need completed interactions in the store
	// The handler processes N×message_added + message_completed per phase.
	// Some interactions get reused across phases (server-initiated follow-ups reuse
	// the thread's existing interaction), so the count may be lower than the phase count.
	if completedInteractions < 2 {
		errors = append(errors, fmt.Sprintf("Expected at least 2 completed interactions, got %d", completedInteractions))
	}

	// Check context mappings
	mappings := d.srv.ContextMappings()
	log.Printf("[test-server] Context mappings: %d entries", len(mappings))
	for threadID, sessionID := range mappings {
		log.Printf("[test-server]   %s -> %s", truncate(threadID, 12), truncate(sessionID, 12))
	}

	// Verify multi-thread: Thread A and Thread B map to different sessions
	if len(d.threadIDs) >= 2 {
		sessionA := mappings[d.threadIDs[0]]
		sessionB := mappings[d.threadIDs[1]]
		if sessionA == sessionB {
			errors = append(errors, fmt.Sprintf("Thread A and Thread B map to same session: %s", truncate(sessionA, 12)))
		} else {
			log.Printf("[test-server] Multi-thread: Thread A -> %s, Thread B -> %s (different sessions)",
				truncate(sessionA, 12), truncate(sessionB, 12))
		}
	}

	// --- STREAMING VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Println("  STREAMING VALIDATION")
	log.Println("--------------------------------------------------")

	completionPhases := []string{"req-phase1", "req-phase2", "req-phase3", "req-phase4", "req-phase5", "req-phase7"}
	for _, reqID := range completionPhases {
		firstAddedIdx := -1
		completedIdx := -1
		addedCount := 0

		for i, evt := range d.events {
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
				log.Printf("[test-server] Streaming %s: %d message_added before message_completed", reqID, addedCount)
			} else {
				errors = append(errors, fmt.Sprintf("Streaming %s: message_added did NOT arrive before message_completed", reqID))
			}
		}
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
			log.Printf("[test-server] Interaction %s (session %s): %d bytes, lastMsgID=%s, offset=%d",
				truncate(i.ID, 12), truncate(i.SessionID, 12),
				len(i.ResponseMessage), truncate(i.LastZedMessageID, 12), i.LastZedMessageOffset)
		}
	}

	// --- THREAD TITLE VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Println("  THREAD TITLE VALIDATION")
	log.Println("--------------------------------------------------")

	titleChangeEvents := d.filterEvents("thread_title_changed")
	if len(titleChangeEvents) > 0 {
		log.Printf("[test-server] Thread title changes: %d events", len(titleChangeEvents))
		for _, e := range titleChangeEvents {
			title, _ := e.Data["title"].(string)
			threadID, _ := e.Data["acp_thread_id"].(string)
			log.Printf("[test-server]   Thread %s -> %q", truncate(threadID, 12), title)
		}
		// Verify at least one session name was updated from the default
		updatedNames := 0
		for _, s := range sessions {
			if s.Name != "" && s.Name != "New Conversation" && s.Name != "New Chat" {
				updatedNames++
				log.Printf("[test-server] Session %s name: %q", truncate(s.ID, 12), s.Name)
			}
		}
		if updatedNames > 0 {
			log.Printf("[test-server] Thread title -> session name sync: %d sessions updated", updatedNames)
		}
	} else {
		// Not a failure - Zed may not generate titles for very short prompts
		log.Println("[test-server] No thread_title_changed events (Zed may not generate titles for short prompts)")
	}

	// --- USER MESSAGE INTERACTION VALIDATION ---
	log.Println("\n--------------------------------------------------")
	log.Println("  USER MESSAGE INTERACTION VALIDATION")
	log.Println("--------------------------------------------------")

	// Phase 5 sends simulate_user_input, which should create an interaction with PromptMessage
	userInteractions := 0
	for _, i := range interactions {
		if i.PromptMessage != "" && strings.Contains(i.PromptMessage, "typed by the user in Zed") {
			userInteractions++
			log.Printf("[test-server] User interaction %s: PromptMessage=%q (session %s)",
				truncate(i.ID, 12), truncate(i.PromptMessage, 50), truncate(i.SessionID, 12))
		}
	}
	if userInteractions == 0 {
		// Check if any interaction has a PromptMessage related to Phase 5
		// The simulate_user_input creates a user message that goes through handleMessageAdded(role=user)
		log.Println("[test-server] NOTE: No interaction found with Phase 5 user message in PromptMessage")
	} else {
		log.Printf("[test-server] User-initiated interactions: %d", userInteractions)
	}

	// --- SUMMARY ---
	if len(errors) > 0 {
		fmt.Println()
		for _, e := range errors {
			log.Printf("[test-server] FAIL: %s", e)
		}
		return false
	}

	fmt.Println()
	log.Println("[test-server] Phase 1: Basic thread creation - PASSED")
	log.Println("[test-server] Phase 2: Follow-up on existing thread - PASSED")
	log.Println("[test-server] Phase 3: New thread via WebSocket - PASSED")
	log.Println("[test-server] Phase 4: Follow-up to non-visible thread - PASSED")
	log.Println("[test-server] Phase 5: Zed -> Helix user message sync - PASSED")
	log.Println("[test-server] Phase 6: Query UI state - PASSED")
	log.Println("[test-server] Phase 7: Open thread + follow-up - PASSED")
	log.Println("[test-server] Store state: Sessions and interactions created correctly - PASSED")
	log.Println("[test-server] Accumulation: ResponseMessage content preserved - PASSED")

	totalCompletions := 0
	for _, v := range d.completions {
		totalCompletions += len(v)
	}
	log.Printf("[test-server] Total threads: %d, Total completions: %d, Sessions: %d, Interactions: %d",
		len(d.threadIDs), totalCompletions, len(sessions), len(interactions))
	return true
}

// --- helpers ---

func (d *testDriver) filterEvents(eventType string) []types.SyncMessage {
	var out []types.SyncMessage
	for _, e := range d.events {
		if e.EventType == eventType {
			out = append(out, e)
		}
	}
	return out
}

func (d *testDriver) filterEventsByFunc(fn func(types.SyncMessage) bool) []types.SyncMessage {
	var out []types.SyncMessage
	for _, e := range d.events {
		if fn(e) {
			out = append(out, e)
		}
	}
	return out
}

func (d *testDriver) hasCompletion(requestID string) bool {
	for _, e := range d.events {
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
	// Create in-memory store and no-op pubsub
	store := memorystore.New()
	ps := pubsub.NewNoop()

	// Create HelixAPIServer with production handlers + in-memory store
	srv := server.NewTestServer(store, ps)

	// Create test driver
	driver := newTestDriver(srv, store)

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

	select {
	case <-driver.done:
	case <-time.After(300 * time.Second):
		driver.mu.Lock()
		eventTypes := make([]string, len(driver.events))
		for i, e := range driver.events {
			eventTypes[i] = e.EventType
		}
		driver.mu.Unlock()
		log.Printf("[test-server] TIMEOUT at phase %d. Events: %v", driver.phase, eventTypes)
		os.Exit(1)
	}

	if driver.validate() {
		log.Println("\n[test-server] ALL TESTS PASSED (7 phases, production handlers, in-memory store)")
		os.Exit(0)
	} else {
		os.Exit(1)
	}
}
