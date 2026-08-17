/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_REQUESTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_REQUESTER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/spellcheck/text_checking.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LocalDOMWindow;
class SpellCheckRequester;
class WebTextCheckClient;

class CORE_EXPORT SpellCheckRequest final
    : public GarbageCollected<SpellCheckRequest> {
 public:
  static const int kUnrequestedTextCheckingSequence = -1;

  static SpellCheckRequest* Create(const EphemeralRange& checking_range,
                                   int request_number);

  SpellCheckRequest(Range* checking_range, const String&, int request_number);
  ~SpellCheckRequest();
  void Dispose();

  Range* CheckingRange() const { return checking_range_.Get(); }
  Element* RootEditableElement() const { return root_editable_element_.Get(); }

  void SetCheckerAndSequence(SpellCheckRequester*, int sequence);
  int Sequence() const { return sequence_; }
  String GetText() const { return text_; }

  bool IsValid() const;
  void DidSucceed(const Vector<TextCheckingResult>&);
  void DidCancel();

  int RequestNumber() const { return request_number_; }

  void Trace(Visitor*) const;

 private:
  Member<SpellCheckRequester> requester_;
  Member<Range> checking_range_;
  Member<Element> root_editable_element_;
  int sequence_ = kUnrequestedTextCheckingSequence;
  String text_;
  int request_number_;
};

class CORE_EXPORT SpellCheckRequester final
    : public GarbageCollected<SpellCheckRequester> {
 public:
  explicit SpellCheckRequester(LocalDOMWindow&);
  SpellCheckRequester(const SpellCheckRequester&) = delete;
  SpellCheckRequester& operator=(const SpellCheckRequester&) = delete;
  ~SpellCheckRequester();
  void Trace(Visitor*) const;

  // Returns true if a request is initiated. Returns false otherwise.
  bool RequestCheckingFor(const EphemeralRange&);
  bool RequestCheckingFor(const EphemeralRange&, int request_num);
  void CancelCheck();

  int LastRequestSequence() const { return last_request_sequence_; }

  int LastProcessedSequence() const { return last_processed_sequence_; }

  // Returns the total length of all text that has been requested for checking.
  int SpellCheckedTextLength() const { return spell_checked_text_length_; }

  // Called to clean up pending requests when no more checking is needed. For
  // example, when document is closed.
  void Deactivate();

 private:
  friend class SpellCheckRequest;

  WebTextCheckClient* GetTextCheckerClient() const;
  void TimerFiredToProcessQueuedRequest();
  void InvokeRequest(SpellCheckRequest*);
  void EnqueueRequest(SpellCheckRequest*);
  bool EnsureValidRequestQueueFor(int sequence);
  void DidCheckSucceed(int sequence, const Vector<TextCheckingResult>&);
  void DidCheckCancel(int sequence);
  void DidCheck(int sequence);

  void ClearProcessingRequest();

  Member<LocalDOMWindow> window_;

  int last_request_sequence_ = 0;
  int last_processed_sequence_ = 0;
  wtf_size_t spell_checked_text_length_ = 0;

  TaskHandle timer_to_process_queued_request_;

  Member<SpellCheckRequest> processing_request_;

  typedef HeapDeque<Member<SpellCheckRequest>> RequestQueue;
  RequestQueue request_queue_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_REQUESTER_H_
