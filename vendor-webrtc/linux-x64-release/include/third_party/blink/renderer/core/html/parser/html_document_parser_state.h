// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_STATE_H_

#include "third_party/blink/renderer/core/html/nesting_level_incrementer.h"
#include "third_party/blink/renderer/core/html/parser/parser_synchronization_policy.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Determines how preloads will be processed when available in the background.
// It is important to process preloads quickly so the request can be started as
// soon as possible. An experiment will be run to pick the best option which
// will then be hard coded.
enum class PreloadProcessingMode {
  // Preloads will be processed once the posted task is run.
  kNone,
  // Preloads will be checked each iteration of the parser and dispatched
  // immediately.
  kImmediate,
  // The parser will yield if there are pending preloads so the task can be run.
  kYield,
};

// This class encapsulates the internal state needed for synchronous foreground
// HTML parsing (e.g. if HTMLDocumentParser::PumpTokenizer yields, this class
// tracks what should be done after the pump completes.)
class HTMLDocumentParserState
    : public GarbageCollected<HTMLDocumentParserState> {
  friend class EndIfDelayedForbiddenScope;
  friend class ShouldCompleteScope;
  friend class AttemptToEndForbiddenScope;

 public:
  // Keeps track of whether the parser needs to complete tokenization work,
  // optionally followed by EndIfDelayed.
  enum class DeferredParserState {
    // Indicates that a tokenizer pump has either completed or hasn't been
    // scheduled.
    kNotScheduled = 0,  // Enforce ordering in this enum.
    // Indicates that a tokenizer pump is scheduled and hasn't completed yet.
    kScheduled = 1,
    // Indicates that a tokenizer pump, followed by EndIfDelayed, is scheduled.
    kScheduledWithEndIfDelayed = 2
  };

  explicit HTMLDocumentParserState(ParserSynchronizationPolicy mode,
                                   int budget);

  void Trace(Visitor* v) const {}

  void SetState(DeferredParserState state) {
    DCHECK(!(state == DeferredParserState::kScheduled && ShouldComplete()));
    state_ = state;
  }
  DeferredParserState GetState() const { return state_; }

  int GetDefaultBudget() const { return budget_; }

  bool IsScheduled() const { return state_ >= DeferredParserState::kScheduled; }
  const char* GetStateAsString() const {
    switch (state_) {
      case DeferredParserState::kNotScheduled:
        return "not_scheduled";
      case DeferredParserState::kScheduled:
        return "scheduled";
      case DeferredParserState::kScheduledWithEndIfDelayed:
        return "scheduled_with_end_if_delayed";
    }
  }

  bool NeedsLinkHeaderPreloadsDispatch() const {
    return needs_link_header_dispatch_;
  }
  void DispatchedLinkHeaderPreloads() { needs_link_header_dispatch_ = false; }

  bool SeenFirstByte() const { return have_seen_first_byte_; }
  void MarkSeenFirstByte() { have_seen_first_byte_ = true; }

  bool EndWasDelayed() const { return end_was_delayed_; }
  void SetEndWasDelayed(bool new_value) { end_was_delayed_ = new_value; }

  bool AddedPendingParserBlockingStylesheet() const {
    return added_pending_parser_blocking_stylesheet_;
  }
  void SetAddedPendingParserBlockingStylesheet(bool new_value) {
    added_pending_parser_blocking_stylesheet_ = new_value;
  }

  bool WaitingForStylesheets() const { return is_waiting_for_stylesheets_; }
  void SetWaitingForStylesheets(bool new_value) {
    is_waiting_for_stylesheets_ = new_value;
  }

  // Keeps track of whether Document::Finish has been called whilst parsing.
  // ShouldAttemptToEndOnEOF() means that the parser should close when there's
  // no more input.
  bool ShouldAttemptToEndOnEOF() const { return should_attempt_to_end_on_eof_; }
  void SetAttemptToEndOnEOF() {
    // Should only ever call ::Finish once.
    DCHECK(!should_attempt_to_end_on_eof_);
    // This method should only be called from ::Finish.
    should_attempt_to_end_on_eof_ = true;
  }

  bool ShouldEndIfDelayed() const { return end_if_delayed_forbidden_ == 0; }
  bool ShouldComplete() const {
    return should_complete_ || GetMode() != kAllowDeferredParsing;
  }
  bool IsSynchronous() const {
    return mode_ == ParserSynchronizationPolicy::kForceSynchronousParsing;
  }
  ParserSynchronizationPolicy GetMode() const { return mode_; }

  void MarkYield() { times_yielded_++; }
  int TimesYielded() const { return times_yielded_; }

  NestingLevelIncrementer ScopedPumpSession() {
    return NestingLevelIncrementer(pump_session_nesting_level_);
  }
  bool InPumpSession() const { return pump_session_nesting_level_; }
  bool InNestedPumpSession() const { return pump_session_nesting_level_ > 1; }

  bool ShouldYieldForPreloads() const {
    return preload_processing_mode_ == PreloadProcessingMode::kYield;
  }

  bool ShouldProcessPreloads() const {
    return preload_processing_mode_ == PreloadProcessingMode::kImmediate;
  }

  void SetExitedHeader() { exited_header_ = true; }
  bool HaveExitedHeader() const { return exited_header_; }

 private:
  void EnterEndIfDelayedForbidden() { end_if_delayed_forbidden_++; }
  void ExitEndIfDelayedForbidden() {
    DCHECK(end_if_delayed_forbidden_);
    end_if_delayed_forbidden_--;
  }

  void EnterAttemptToEndForbidden() {
    DCHECK(should_attempt_to_end_on_eof_);
    should_attempt_to_end_on_eof_ = false;
  }

  void EnterShouldComplete() { should_complete_++; }
  void ExitShouldComplete() {
    DCHECK(should_complete_);
    should_complete_--;
  }

  DeferredParserState state_;
  ParserSynchronizationPolicy mode_;
  const PreloadProcessingMode preload_processing_mode_;
  unsigned end_if_delayed_forbidden_ = 0;
  unsigned should_complete_ = 0;
  unsigned times_yielded_ = 0;
  unsigned pump_session_nesting_level_ = 0;
  int budget_;

  // Set to non-zero if Document::Finish has been called and we're operating
  // asynchronously.
  bool should_attempt_to_end_on_eof_ = false;
  bool needs_link_header_dispatch_ = true;
  bool have_seen_first_byte_ = false;
  bool end_was_delayed_ = false;
  bool added_pending_parser_blocking_stylesheet_ = false;
  bool is_waiting_for_stylesheets_ = false;
  bool exited_header_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_STATE_H_
