/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_LIST_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class EventQueue;
class SourceBuffer;

class SourceBufferList final : public EventTarget,
                               public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SourceBufferList(ExecutionContext*, EventQueue*);
  ~SourceBufferList() override;

  unsigned length() const { return list_.size(); }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(addsourcebuffer, kAddsourcebuffer)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(removesourcebuffer, kRemovesourcebuffer)

  SourceBuffer* item(unsigned index) const {
    return (index < list_.size()) ? list_[index].Get() : nullptr;
  }

  void Add(SourceBuffer*);
  void insert(wtf_size_t position, SourceBuffer*);
  void Remove(SourceBuffer*);
  wtf_size_t Find(SourceBuffer* buffer) { return list_.Find(buffer); }
  bool Contains(SourceBuffer* buffer) {
    return list_.Find(buffer) != kNotFound;
  }
  void Clear();

  // EventTarget interface
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextClient::GetExecutionContext();
  }

  void Trace(Visitor*) const override;

 private:
  void ScheduleEvent(const AtomicString&);

  Member<EventQueue> async_event_queue_;

  // If any portion of an attached HTMLMediaElement (HTMLME) and the MediaSource
  // Extensions (MSE) API is alive (having pending activity or traceable from a
  // GC root), the whole group is not GC'ed. Here, using Member,
  // instead of Member, because the |list_|'s SourceBuffers' wrappers need to
  // remain alive at least to successfully dispatch any events enqueued by
  // behavior of the HTMLME+MSE API. Member usage here keeps the
  // SourceBuffers in |list_|, and their wrappers, from being collected if we
  // are alive or traceable from a GC root.
  // For instance, suppose the only reference to the group of HTMLME+MSE API
  // objects is one held by the app to the wrapper of this SourceBufferList.
  // The app could do any of a number of actions on any of the SourceBuffers in
  // |list_|, such as appending more media, causing events to be enqueued for
  // later dispatch on at least those SourceBuffers. None of the app's event
  // listeners on objects in HTMLME+MSE are counted as references, but events
  // pending for dispatch to them must keep the HTML+MSE group of objects alive
  // through at least their dispatch.
  HeapVector<Member<SourceBuffer>> list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_LIST_H_
