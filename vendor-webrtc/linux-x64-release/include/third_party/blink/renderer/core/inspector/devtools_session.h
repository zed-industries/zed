// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_SESSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_SESSION_H_

#include <memory>
#include <type_traits>
#include "base/functional/callback.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_session_state.h"
#include "third_party/blink/renderer/core/inspector/protocol/forward.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8-inspector-protocol.h"

namespace blink {

class DevToolsAgent;
class Document;
class DocumentLoader;
class InspectorAgent;
class LocalFrame;
class InspectorAccessibilityAgent;
class InspectorAuditsAgent;
class InspectorCSSAgent;
class InspectorCacheStorageAgent;
class InspectorDOMAgent;
class InspectorDOMDebuggerAgent;
class InspectorDOMSnapshotAgent;
class InspectorEmulationAgent;
class InspectorIOAgent;
class InspectorLogAgent;
class InspectorNetworkAgent;
class InspectorOverlayAgent;
class InspectorPageAgent;
class InspectorPerformanceAgent;
class InspectorWebAudioAgent;

class CORE_EXPORT DevToolsSession : public GarbageCollected<DevToolsSession>,
                                    public mojom::blink::DevToolsSession,
                                    public protocol::FrontendChannel,
                                    public v8_inspector::V8Inspector::Channel {
 public:
  DevToolsSession(
      DevToolsAgent*,
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsSessionHost>
          host_remote,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsSession>
          main_receiver,
      mojo::PendingReceiver<mojom::blink::DevToolsSession> io_receiver,
      mojom::blink::DevToolsSessionStatePtr reattach_session_state,
      const String& script_to_evaluate_on_load,
      bool client_expects_binary_responses,
      bool client_is_trusted,
      const String& session_id,
      bool session_waits_for_debugger,
      scoped_refptr<base::SequencedTaskRunner> mojo_task_runner);
  DevToolsSession(const DevToolsSession&) = delete;
  DevToolsSession& operator=(const DevToolsSession&) = delete;
  ~DevToolsSession() override;

  void ConnectToV8(v8_inspector::V8Inspector*, int context_group_id);
  v8_inspector::V8InspectorSession* V8Session() { return v8_session_.get(); }

  template <typename Agent, typename... Args>
  Agent* CreateAndAppend(Args&&... args) {
    if (!IsDomainAvailableToUntrustedClient<Agent>() && !client_is_trusted_) {
      return nullptr;
    }
    auto agent = MakeGarbageCollected<Agent>(std::forward<Args>(args)...);
    Append(agent);
    return agent;
  }
  void Detach();
  void DetachFromV8();
  void Trace(Visitor*) const;

  // protocol::FrontendChannel implementation.
  void FlushProtocolNotifications() override;

  // Core probes.
  void DidStartProvisionalLoad(LocalFrame*);
  void DidFailProvisionalLoad(LocalFrame*);
  void DidCommitLoad(LocalFrame*, DocumentLoader*);
  void PaintTiming(Document* document, const char* name, double timestamp);
  void DomContentLoadedEventFired(LocalFrame*);

  const String& script_to_evaluate_on_load() const {
    return script_to_evaluate_on_load_;
  }

 private:
  class IOSession;

  // mojom::blink::DevToolsSession implementation.
  void DispatchProtocolCommand(int call_id,
                               const String& method,
                               base::span<const uint8_t> message) override;
  void UnpauseAndTerminate() override;

  void DispatchProtocolCommandImpl(int call_id,
                                   const String& method,
                                   base::span<const uint8_t> message);

  // protocol::FrontendChannel implementation.
  void SendProtocolResponse(
      int call_id,
      std::unique_ptr<protocol::Serializable> message) override;
  void SendProtocolNotification(
      std::unique_ptr<protocol::Serializable> message) override;
  void FallThrough(int call_id,
                   crdtp::span<uint8_t> method,
                   crdtp::span<uint8_t> message) override;

  // v8_inspector::V8Inspector::Channel implementation.
  void sendResponse(
      int call_id,
      std::unique_ptr<v8_inspector::StringBuffer> message) override;
  void sendNotification(
      std::unique_ptr<v8_inspector::StringBuffer> message) override;
  void flushProtocolNotifications() override;

  bool IsDetached();
  void SendProtocolResponse(int call_id, std::vector<uint8_t> message);

  // Converts to JSON if requested by the client.
  blink::mojom::blink::DevToolsMessagePtr FinalizeMessage(
      std::vector<uint8_t> message,
      std::optional<int> call_id) const;

  template <typename T>
  bool IsDomainAvailableToUntrustedClient() {
    return std::disjunction_v<std::is_same<T, InspectorAuditsAgent>,
                              std::is_same<T, InspectorCSSAgent>,
                              std::is_same<T, InspectorCacheStorageAgent>,
                              std::is_same<T, InspectorAccessibilityAgent>,
                              std::is_same<T, InspectorDOMAgent>,
                              std::is_same<T, InspectorDOMDebuggerAgent>,
                              std::is_same<T, InspectorDOMSnapshotAgent>,
                              std::is_same<T, InspectorEmulationAgent>,
                              std::is_same<T, InspectorIOAgent>,
                              std::is_same<T, InspectorLogAgent>,
                              std::is_same<T, InspectorNetworkAgent>,
                              std::is_same<T, InspectorOverlayAgent>,
                              std::is_same<T, InspectorPageAgent>,
                              std::is_same<T, InspectorPerformanceAgent>,
                              std::is_same<T, InspectorWebAudioAgent>>;
  }
  void Append(InspectorAgent*);

  Member<DevToolsAgent> agent_;
  // DevToolsSession is not tied to ExecutionContext
  HeapMojoAssociatedReceiver<mojom::blink::DevToolsSession, DevToolsSession>
      receiver_{this, nullptr};
  // DevToolsSession is not tied to ExecutionContext
  HeapMojoAssociatedRemote<mojom::blink::DevToolsSessionHost> host_remote_{
      nullptr};
  IOSession* io_session_;
  std::unique_ptr<v8_inspector::V8InspectorSession> v8_session_;
  std::unique_ptr<protocol::UberDispatcher> inspector_backend_dispatcher_;
  InspectorSessionState session_state_;
  HeapVector<Member<InspectorAgent>> agents_;
  // Notifications are lazily serialized to shift the serialization overhead
  // from performance measurements. We may want to revisit this.
  // See https://bugs.chromium.org/p/chromium/issues/detail?id=1044989#c8
  Vector<base::OnceCallback<std::vector<uint8_t>()>> notification_queue_;
  const bool client_expects_binary_responses_;
  const bool client_is_trusted_;
  InspectorAgentState v8_session_state_;
  InspectorAgentState::Bytes v8_session_state_cbor_;
  String script_to_evaluate_on_load_;
  const String session_id_;
  // This is only relevant until the initial attach to v8 and is never reset
  // once the session stops waiting.
  const bool session_waits_for_debugger_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_SESSION_H_
