/*
 * Copyright (C) 2009, 2012 Ericsson AB. All rights reserved.
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer
 *    in the documentation and/or other materials provided with the
 *    distribution.
 * 3. Neither the name of Ericsson nor the names of its contributors
 *    may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/loader/threadable_loader.h"
#include "third_party/blink/renderer/core/loader/threadable_loader_client.h"
#include "third_party/blink/renderer/modules/eventsource/event_source_parser.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class DOMWrapperWorld;
class EventSourceInit;
class ExceptionState;
class ResourceResponse;

class MODULES_EXPORT EventSource final
    : public EventTarget,
      public ThreadableLoaderClient,
      public ActiveScriptWrappable<EventSource>,
      public ExecutionContextLifecycleObserver,
      public EventSourceParser::Client {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static EventSource* Create(ExecutionContext*,
                             const String& url,
                             const EventSourceInit*,
                             ExceptionState&);

  EventSource(ExecutionContext*, const KURL&, const EventSourceInit*);
  ~EventSource() override;

  static const uint64_t kDefaultReconnectDelay;

  String url() const;
  bool withCredentials() const;

  enum State : int16_t { kConnecting = 0, kOpen = 1, kClosed = 2 };

  State readyState() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(open, kOpen)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)

  void close();

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver
  //
  // Note: We don't need to inherit from ExecutionContextLifecycleStateObserver
  // since ScopedPageLoadDeferrer calls Page::setDefersLoading() and it defers
  // delivery of events from the loader, and therefore the methods of this class
  // for receiving asynchronous events from the loader won't be invoked.
  void ContextDestroyed() override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  void Trace(Visitor*) const override;

 private:
  void DidReceiveResponse(uint64_t, const ResourceResponse&) override;
  void DidReceiveData(base::span<const char>) override;
  void DidFinishLoading(uint64_t) override;
  void DidFail(uint64_t, const ResourceError&) override;
  void DidFailRedirectCheck(uint64_t) override;

  void OnMessageEvent(const AtomicString& event,
                      const String& data,
                      const AtomicString& id) override;
  void OnReconnectionTimeSet(uint64_t reconnection_time) override;

  void ScheduleInitialConnect();
  void Connect();
  void NetworkRequestEnded();
  void ScheduleReconnect();
  void ConnectTimerFired(TimerBase*);
  void AbortConnectionAttempt();

  // The original URL specified when constructing EventSource instance. Used
  // for the 'url' attribute getter.
  const KURL url_;
  // The URL used to connect to the server, which may be different from
  // |m_url| as it may be redirected.
  KURL current_url_;
  bool with_credentials_;
  State state_;

  Member<EventSourceParser> parser_;
  Member<ThreadableLoader> loader_;
  HeapTaskRunnerTimer<EventSource> connect_timer_;

  uint64_t reconnect_delay_;
  String event_stream_origin_;
  uint64_t resource_identifier_ = 0;

  // The world in which this EventSource was created. We need to store this
  // because EventSource::Connect can be triggered by |connect_timer_|.
  Member<const DOMWrapperWorld> world_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_H_
