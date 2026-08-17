// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_H_

#include "cc/paint/paint_canvas.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/frame_owner_properties.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/frame_replication_state.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/tree_scope_type.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/core/frame/frame_client.h"
#include "third_party/blink/renderer/core/frame/frame_types.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class RemoteFrameClient : public FrameClient {
 public:
  ~RemoteFrameClient() override = default;

  unsigned BackForwardLength() override = 0;

  // Create a new RemoteFrame child. This needs to be a client API
  // so that the appropriate WebRemoteFrameImpl is created first before
  // the core frame. In the future we should only create a WebRemoteFrame
  // when we pass a RemoteFrame handle outside of blink.
  virtual void CreateRemoteChild(
      const RemoteFrameToken& token,
      const std::optional<FrameToken>& opener_frame_token,
      mojom::blink::TreeScopeType tree_scope_type,
      mojom::blink::FrameReplicationStatePtr replication_state,
      mojom::blink::FrameOwnerPropertiesPtr owner_properties,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      mojom::blink::RemoteFrameInterfacesFromBrowserPtr
          remote_frame_interfaces) = 0;

  // Creates a `RemoteFrame` for each node in `params`. The resulting tree of
  // `RemoteFrames` has the same structure as `params`, with this `RemoteFrame`
  // at the root. This needs to be a client API so that the appropriate
  // `WebRemoteFrameImpl` is created first before the core frame. In the future
  // we should only create a `WebRemoteFrame` when we pass a `RemoteFrame`
  // handle outside of blink.
  virtual void CreateRemoteChildren(
      const Vector<mojom::blink::CreateRemoteChildParamsPtr>& params) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_CLIENT_H_
