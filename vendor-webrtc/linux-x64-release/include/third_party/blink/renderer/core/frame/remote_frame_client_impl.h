// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_IMPL_H_

#include "third_party/blink/renderer/core/frame/remote_frame_client.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
class WebRemoteFrameImpl;

class RemoteFrameClientImpl final : public RemoteFrameClient {
 public:
  explicit RemoteFrameClientImpl(WebRemoteFrameImpl*);

  void Trace(Visitor*) const override;

  // FrameClient overrides:
  bool InShadowTree() const override;
  void Detached(FrameDetachType) override;

  // RemoteFrameClient overrides:
  void CreateRemoteChild(
      const RemoteFrameToken& token,
      const std::optional<FrameToken>& opener_frame_token,
      mojom::blink::TreeScopeType tree_scope_type,
      mojom::blink::FrameReplicationStatePtr replication_state,
      mojom::blink::FrameOwnerPropertiesPtr owner_properties,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      mojom::blink::RemoteFrameInterfacesFromBrowserPtr remote_frame_interfaces)
      override;
  unsigned BackForwardLength() override;
  void CreateRemoteChildren(
      const Vector<mojom::blink::CreateRemoteChildParamsPtr>& params) override;

  WebRemoteFrameImpl* GetWebFrame() const { return web_frame_.Get(); }

 private:
  WebRemoteFrameImpl* CreateRemoteChildImpl(
      const RemoteFrameToken& token,
      const std::optional<FrameToken>& opener_frame_token,
      mojom::blink::TreeScopeType tree_scope_type,
      mojom::blink::FrameReplicationStatePtr replication_state,
      mojom::blink::FrameOwnerPropertiesPtr owner_properties,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      mojom::blink::RemoteFrameInterfacesFromBrowserPtr
          remote_frame_interfaces);

  Member<WebRemoteFrameImpl> web_frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_IMPL_H_
