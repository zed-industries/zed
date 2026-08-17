// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_REMOTE_FRAME_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_REMOTE_FRAME_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/frame_replication_state.mojom-forward.h"
#include "third_party/blink/public/mojom/frame/user_activation_notification_type.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/user_activation_update_types.mojom-shared.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/public/web/web_frame.h"
#include "ui/events/types/scroll_types.h"

namespace blink {

namespace mojom {
enum class TreeScopeType;
}
class InterfaceRegistry;
class WebLocalFrameClient;
class WebString;
class WebView;
struct FramePolicy;
struct FrameVisualProperties;
struct WebFrameOwnerProperties;

class BLINK_EXPORT WebRemoteFrame : public WebFrame {
 public:
  // Factory methods for creating a WebRemoteFrame.
  static WebRemoteFrame* Create(mojom::TreeScopeType,
                                const RemoteFrameToken& frame_token);

  static WebRemoteFrame* CreateMainFrame(
      WebView*,
      const RemoteFrameToken& frame_token,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      WebFrame* opener,
      CrossVariantMojoAssociatedRemote<mojom::RemoteFrameHostInterfaceBase>
          remote_frame_host,
      CrossVariantMojoAssociatedReceiver<mojom::RemoteFrameInterfaceBase>
          receiver,
      mojom::FrameReplicationStatePtr replicated_state);

  // Specialized factory methods to allow the embedder to replicate the frame
  // tree between processes.
  // TODO(dcheng): The embedder currently does not replicate local frames in
  // insertion order, so the local child version takes |previous_sibling| to
  // ensure that it is inserted into the correct location in the list of
  // children. If |previous_sibling| is null, the child is inserted at the
  // beginning.
  virtual WebLocalFrame* CreateLocalChild(
      mojom::TreeScopeType,
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
      std::unique_ptr<WebPolicyContainer> policy_container) = 0;

  // Returns the frame associated with the |frame_token|.
  static WebRemoteFrame* FromFrameToken(const RemoteFrameToken& frame_token);

  // Set security origin replicated from another process.
  virtual void SetReplicatedOrigin(
      const WebSecurityOrigin&,
      bool is_potentially_trustworthy_opaque_origin) = 0;

  virtual void DidStartLoading() = 0;

  // Unique name is an opaque identifier for maintaining association with
  // session restore state for this frame.
  virtual WebString UniqueName() const = 0;

  virtual const FrameVisualProperties& GetPendingVisualPropertiesForTesting()
      const = 0;

  RemoteFrameToken GetRemoteFrameToken() const {
    return GetFrameToken().GetAs<RemoteFrameToken>();
  }

  // Ad Tagging ---------------------------------------------------------

  // True if the frame is thought (heuristically) to be created for
  // advertising purposes.
  bool IsAdFrame() const override = 0;

 protected:
  explicit WebRemoteFrame(mojom::TreeScopeType scope,
                          const RemoteFrameToken& frame_token)
      : WebFrame(scope, frame_token) {}

  // Inherited from WebFrame, but intentionally hidden: it never makes sense
  // to call these on a WebRemoteFrame.
  bool IsWebLocalFrame() const override = 0;
  WebLocalFrame* ToWebLocalFrame() override = 0;
  bool IsWebRemoteFrame() const override = 0;
  WebRemoteFrame* ToWebRemoteFrame() override = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_REMOTE_FRAME_H_
