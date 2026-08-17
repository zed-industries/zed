// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_IDLE_SPELL_CHECK_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_IDLE_SPELL_CHECK_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class ColdModeSpellCheckRequester;
class Document;
class Element;
class IdleDeadline;
class LocalDOMWindow;
class SpellCheckRequester;

#define FOR_EACH_IDLE_SPELL_CHECK_CONTROLLER_STATE(V) \
  V(Inactive)                                         \
  V(HotModeRequested)                                 \
  V(InHotModeInvocation)                              \
  V(ColdModeTimerStarted)                             \
  V(ColdModeRequested)                                \
  V(InColdModeInvocation)

// Main class for the implementation of idle time spell checker.
// See design doc for details: https://goo.gl/zONC3v
class CORE_EXPORT IdleSpellCheckController final
    : public GarbageCollected<IdleSpellCheckController>,
      public ExecutionContextLifecycleObserver {
 public:
  explicit IdleSpellCheckController(LocalDOMWindow&, SpellCheckRequester&);
  IdleSpellCheckController(const IdleSpellCheckController&) = delete;
  IdleSpellCheckController& operator=(const IdleSpellCheckController&) = delete;
  ~IdleSpellCheckController() override;

  enum class State {
#define V(state) k##state,
    FOR_EACH_IDLE_SPELL_CHECK_CONTROLLER_STATE(V)
#undef V
  };

  State GetState() const { return state_; }

  // Transit to HotModeRequested, if possible. Called by operations that need
  // spell checker to follow up.
  void RespondToChangedSelection();
  void RespondToChangedContents();
  void RespondToChangedEnablement();

  // Cleans everything up and makes the callback inactive. Should be called when
  // document is detached or spellchecking is globally disabled.
  void Deactivate();

  // Called when spellchecking is disabled on the specific element.
  void SetSpellCheckingDisabled(const Element&);

  ColdModeSpellCheckRequester& GetColdModeRequester() const {
    return *cold_mode_requester_;
  }

  // Exposed for testing only.
  SpellCheckRequester& GetSpellCheckRequester() const;
  void ForceInvocationForTesting();
  void SetNeedsMoreColdModeInvocationForTesting();
  void SkipColdModeTimerForTesting();
  int IdleCallbackHandle() const { return idle_callback_handle_; }

  void Trace(Visitor*) const override;

 private:
  friend class Internals;

  // For testing and debugging only.
  const char* GetStateAsString() const;

  class IdleCallback;

  LocalDOMWindow& GetWindow() const;

  // Return the document to work on. Callable only when GetExecutionContext()
  // is non-null.
  Document& GetDocument() const;

  bool IsInInvocation() const {
    return state_ == State::kInHotModeInvocation ||
           state_ == State::kInColdModeInvocation;
  }

  // Returns whether spell checking is globally enabled.
  bool IsSpellCheckingEnabled() const;

  // Called by RespondTo*() functions to transit to HotModeRequested state.
  void SetNeedsInvocation();

  // Called at idle time as entrance function.
  void Invoke(IdleDeadline*);

  // Functions for hot mode.
  void HotModeInvocation(IdleDeadline*);
  bool NeedsHotModeCheckingUnderCurrentSelection() const;

  // Transit to ColdModeTimerStarted, if possible. Sets up a timer, and requests
  // cold mode invocation if no critical operation occurs before timer firing.
  void SetNeedsColdModeInvocation();

  // Functions for cold mode.
  void ColdModeTimerFired();
  void ColdModeInvocation(IdleDeadline*);

  // Implements |ExecutionContextLifecycleObserver|.
  void ContextDestroyed() final;

  void DisposeIdleCallback();

  State state_ = State::kInactive;
  int idle_callback_handle_;
  uint64_t last_processed_undo_step_sequence_ = 0;
  const Member<ColdModeSpellCheckRequester> cold_mode_requester_;
  Member<SpellCheckRequester> spell_check_requeseter_;
  TaskHandle cold_mode_timer_;

  bool needs_invocation_for_changed_selection_ = false;
  bool needs_invocation_for_changed_contents_ = false;
  bool needs_invocation_for_changed_enablement_ = false;

  friend class IdleSpellCheckControllerTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_IDLE_SPELL_CHECK_CONTROLLER_H_
