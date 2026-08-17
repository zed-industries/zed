// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_FRAME_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_FRAME_HOST_H_

#include <optional>

#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/associated_receiver_set.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "third_party/blink/public/common/associated_interfaces/associated_interface_provider.h"
#include "third_party/blink/public/common/frame/frame_visual_properties.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink.h"
#include "third_party/blink/renderer/core/messaging/blink_transferable_message.h"

namespace blink {

// This class implements a RemoteFrameHost that can be attached to the
// AssociatedInterfaceProvider so that it will be called when the renderer
// normally sends a request to the browser process. But for a unittest
// setup it can be intercepted by this class.
class FakeRemoteFrameHost : public mojom::blink::RemoteFrameHost {
 public:
  FakeRemoteFrameHost() = default;

  mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
  BindNewAssociatedRemote();

  void SetInheritedEffectiveTouchAction(cc::TouchAction touch_action) override;
  void UpdateRenderThrottlingStatus(bool is_throttled,
                                    bool subtree_throttled,
                                    bool display_locked) override;
  void VisibilityChanged(mojom::blink::FrameVisibility visibility) override;
  void DidFocusFrame() override;
  void CheckCompleted() override;
  void CapturePaintPreviewOfCrossProcessSubframe(
      const gfx::Rect& clip_rect,
      const base::UnguessableToken& guid) override;
  void SetIsInert(bool inert) override;
  void DidChangeOpener(
      const std::optional<LocalFrameToken>& opener_frame) override;
  void AdvanceFocus(blink::mojom::FocusType focus_type,
                    const LocalFrameToken& source_frame_token) override;
  void RouteMessageEvent(
      const std::optional<LocalFrameToken>& source_frame_token,
      const scoped_refptr<const SecurityOrigin>& source_origin,
      const String& target_origin,
      BlinkTransferableMessage message) override;
  void PrintCrossProcessSubframe(const gfx::Rect& rect,
                                 int document_cookie) override;
  void Detach() override;
  void UpdateViewportIntersection(
      blink::mojom::blink::ViewportIntersectionStatePtr intersection_state,
      const std::optional<FrameVisualProperties>& visual_properties) override;

  void SynchronizeVisualProperties(
      const blink::FrameVisualProperties& properties) override;

  void OpenURL(mojom::blink::OpenURLParamsPtr params) override;

 private:
  mojo::AssociatedReceiver<mojom::blink::RemoteFrameHost> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_FRAME_HOST_H_
