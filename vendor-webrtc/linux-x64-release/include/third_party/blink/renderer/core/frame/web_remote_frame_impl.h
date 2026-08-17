// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_REMOTE_FRAME_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_REMOTE_FRAME_IMPL_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/frame_owner_properties.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/tree_scope_type.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/user_activation_update_types.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_remote_frame.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/remote_frame.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class FrameOwner;
struct FrameVisualProperties;
class RemoteFrame;
class RemoteFrameClientImpl;
enum class WebFrameLoadType;
class WebFrameWidgetImpl;
class WebView;
class WindowAgentFactory;

class CORE_EXPORT WebRemoteFrameImpl final
    : public GarbageCollected<WebRemoteFrameImpl>,
      public WebRemoteFrame {
 public:
  static WebRemoteFrameImpl* CreateMainFrame(
      WebView*,
      const RemoteFrameToken& frame_token,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      WebFrame* opener,
      mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
          remote_frame_host,
      mojo::PendingAssociatedReceiver<mojom::blink::RemoteFrame> receiver,
      mojom::blink::FrameReplicationStatePtr replicated_state);

  static WebRemoteFrameImpl* CreateForFencedFrame(
      mojom::blink::TreeScopeType,
      const RemoteFrameToken& frame_token,
      const base::UnguessableToken& devtools_frame_token,
      HTMLFrameOwnerElement* frame_owner,
      mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
          remote_frame_host,
      mojo::PendingAssociatedReceiver<mojom::blink::RemoteFrame> receiver,
      mojom::blink::FrameReplicationStatePtr replicated_state);

  WebRemoteFrameImpl(mojom::blink::TreeScopeType,
                     const RemoteFrameToken& frame_token);
  ~WebRemoteFrameImpl() override;

  // WebFrame methods:
  void Close(DetachReason detach_reason) override;
  WebView* View() const override;

  // WebRemoteFrame methods:
  WebLocalFrame* CreateLocalChild(
      mojom::blink::TreeScopeType,
      const WebString& name,
      const FramePolicy&,
      WebLocalFrameClient*,
      InterfaceRegistry*,
      WebFrame* previous_sibling,
      const WebFrameOwnerProperties&,
      const LocalFrameToken& frame_token,
      WebFrame* opener,
      const DocumentToken& document_token,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>,
      std::unique_ptr<blink::WebPolicyContainer> policy_container) override;
  void SetReplicatedOrigin(
      const WebSecurityOrigin&,
      bool is_potentially_trustworthy_opaque_origin) override;
  void DidStartLoading() override;
  v8::Local<v8::Object> GlobalProxy(v8::Isolate*) const override;
  WebString UniqueName() const override;
  const FrameVisualProperties& GetPendingVisualPropertiesForTesting()
      const override;
  bool IsAdFrame() const override;
  void InitializeCoreFrame(
      Page&,
      FrameOwner*,
      WebFrame* parent,
      WebFrame* previous_sibling,
      FrameInsertType,
      const AtomicString& name,
      WindowAgentFactory*,
      const base::UnguessableToken& devtools_frame_token,
      mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
          remote_frame_host,
      mojo::PendingAssociatedReceiver<mojom::blink::RemoteFrame> receiver);
  RemoteFrame* GetFrame() const { return frame_.Get(); }

  WebRemoteFrameImpl* CreateRemoteChild(
      mojom::blink::TreeScopeType,
      const RemoteFrameToken& frame_token,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      WebFrame* opener,
      mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
          remote_frame_host,
      mojo::PendingAssociatedReceiver<mojom::blink::RemoteFrame> receiver,
      mojom::blink::FrameReplicationStatePtr replicated_state,
      mojom::blink::FrameOwnerPropertiesPtr owner_properties);

  static WebRemoteFrameImpl* FromFrame(RemoteFrame&);

  void Trace(Visitor*) const;

  gfx::Rect GetCompositingRect();

  void SetReplicatedState(mojom::FrameReplicationStatePtr replicated_state);
  void SetReplicatedState(
      mojom::blink::FrameReplicationStatePtr replicated_state);
  void SetFrameOwnerProperties(
      mojom::blink::FrameOwnerPropertiesPtr owner_properties);

 private:
  friend class RemoteFrameClientImpl;

  void SetCoreFrame(RemoteFrame*);
  void InitializeFrameVisualProperties(WebFrameWidgetImpl* ancestor_widget,
                                       WebView* web_view);

  // Inherited from WebFrame, but intentionally hidden: it never makes sense
  // to call these on a WebRemoteFrameImpl.
  bool IsWebLocalFrame() const override;
  WebLocalFrame* ToWebLocalFrame() override;
  const WebLocalFrame* ToWebLocalFrame() const override;
  bool IsWebRemoteFrame() const override;
  WebRemoteFrame* ToWebRemoteFrame() override;
  const WebRemoteFrame* ToWebRemoteFrame() const override;

  // TODO(dcheng): Inline this field directly rather than going through Member.
  Member<RemoteFrameClientImpl> frame_client_;
  Member<RemoteFrame> frame_;

  // Oilpan: WebRemoteFrameImpl must remain alive until close() is called.
  // Accomplish that by keeping a self-referential Persistent<>. It is
  // cleared upon close().
  SelfKeepAlive<WebRemoteFrameImpl> self_keep_alive_{this};
};

template <>
struct DowncastTraits<WebRemoteFrameImpl> {
  static bool AllowFrom(const WebFrame& frame) {
    return frame.IsWebRemoteFrame();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_REMOTE_FRAME_IMPL_H_
