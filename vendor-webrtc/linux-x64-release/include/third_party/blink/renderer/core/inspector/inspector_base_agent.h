/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_BASE_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_BASE_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/core_probe_sink.h"
#include "third_party/blink/renderer/core/inspector/inspector_session_state.h"
#include "third_party/blink/renderer/core/inspector/protocol/protocol.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalFrame;

class CORE_EXPORT InspectorAgent : public GarbageCollected<InspectorAgent> {
 public:
  InspectorAgent() = default;
  virtual ~InspectorAgent() = default;
  virtual void Trace(Visitor* visitor) const {}

  virtual void Restore() {}
  virtual void DidCommitLoadForLocalFrame(LocalFrame*) {}
  virtual void FlushPendingProtocolNotifications() {}

  virtual void Init(CoreProbeSink*,
                    protocol::UberDispatcher*,
                    InspectorSessionState*) = 0;
  virtual void Dispose() = 0;
};

template <typename DomainMetainfo>
class InspectorBaseAgent : public InspectorAgent,
                           public DomainMetainfo::BackendClass {
 public:
  void Init(CoreProbeSink* instrumenting_agents,
            protocol::UberDispatcher* dispatcher,
            InspectorSessionState* session_state) override {
    instrumenting_agents_ = instrumenting_agents;
    frontend_.reset(
        new typename DomainMetainfo::FrontendClass(dispatcher->channel()));
    DomainMetainfo::DispatcherClass::wire(dispatcher, this);

    agent_state_.InitFrom(session_state);
  }

  protocol::Response disable() override {
    return protocol::Response::Success();
  }

  void Dispose() override {
    disable();
    frontend_.reset();
    instrumenting_agents_ = nullptr;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(instrumenting_agents_);
    InspectorAgent::Trace(visitor);
  }

 protected:
  InspectorBaseAgent() : agent_state_(DomainMetainfo::domainName) {}
  ~InspectorBaseAgent() override {
    CHECK(!frontend_);  // Ensure Dispose() has been called.
  }
  typename DomainMetainfo::FrontendClass* GetFrontend() const {
    return frontend_.get();
  }
  Member<CoreProbeSink> instrumenting_agents_;
  InspectorAgentState agent_state_;

 private:
  std::unique_ptr<typename DomainMetainfo::FrontendClass> frontend_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_BASE_AGENT_H_
