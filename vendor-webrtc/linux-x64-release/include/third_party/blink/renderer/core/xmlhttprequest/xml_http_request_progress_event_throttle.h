/*
 * Copyright (C) 2010 Julien Chaffraix <jchaffraix@webkit.org>
 * All right reserved.
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
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_PROGRESS_EVENT_THROTTLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_PROGRESS_EVENT_THROTTLE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Event;
class XMLHttpRequest;

// This class implements the XHR2 ProgressEvent dispatching:
//   "dispatch a progress event named progress about every 50ms or for every
//   byte received, whichever is least frequent".
//
// readystatechange events also go through this class since ProgressEvents and
// readystatechange events affect each other.
//
// In the comments for this class:
// - "progress" event means an event named "progress"
// - ProgressEvent means an event using the ProgressEvent interface defined in
//   the spec.
class XMLHttpRequestProgressEventThrottle final
    : public GarbageCollected<XMLHttpRequestProgressEventThrottle>,
      public TimerBase {
  // Need to promptly stop this timer when it is deemed finalizable.
  USING_PRE_FINALIZER(XMLHttpRequestProgressEventThrottle, Stop);

 public:
  explicit XMLHttpRequestProgressEventThrottle(XMLHttpRequest*);
  ~XMLHttpRequestProgressEventThrottle() override;

  enum DeferredEventAction {
    kIgnore,
    kClear,
    kFlush,
  };

  // Dispatches a ProgressEvent.
  //
  // Special treatment for events named "progress" is implemented to dispatch
  // them at the required frequency. If this object is suspended, the given
  // ProgressEvent overwrites the existing. I.e. only the latest one gets
  // queued. If the timer is running, this method just updates
  // m_lengthComputable, m_loaded and m_total. They'll be used on next
  // fired() call.
  // For an event named "progress", a readyStateChange will be dispatched
  // as well.
  void DispatchProgressEvent(const AtomicString&,
                             bool length_computable,
                             uint64_t loaded,
                             uint64_t total);
  // Dispatches the given event after operation about the "progress" event
  // depending on the value of the ProgressEventAction argument.
  void DispatchReadyStateChangeEvent(Event*, DeferredEventAction);

  void Trace(Visitor*) const;

 private:
  // Dispatches a "progress" progress event and usually a readyStateChange
  // event as well.
  void DispatchProgressProgressEvent(Event*);

  // The main purpose of this class is to throttle the "progress"
  // ProgressEvent dispatching. This class represents such a deferred
  // "progress" ProgressEvent.
  class DeferredEvent {
    DISALLOW_NEW();

   public:
    DeferredEvent();
    void Set(bool length_computable, uint64_t loaded, uint64_t total);
    void Clear();
    bool IsSet() const { return is_set_; }
    Event* Take();

   private:
    uint64_t loaded_;
    uint64_t total_;
    bool length_computable_;

    bool is_set_;
  };

  void Fired() override;

  Member<XMLHttpRequest> target_;

  // A slot for the deferred "progress" ProgressEvent. When multiple events
  // arrive, only the last one is stored and others are discarded.
  DeferredEvent deferred_;

  // True if any "progress" progress event has been dispatched since
  // |m_target|'s readyState changed.
  bool has_dispatched_progress_progress_event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_PROGRESS_EVENT_THROTTLE_H_
