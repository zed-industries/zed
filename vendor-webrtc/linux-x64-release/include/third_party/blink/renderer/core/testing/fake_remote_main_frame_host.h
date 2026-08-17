// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_MAIN_FRAME_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_MAIN_FRAME_HOST_H_

#include "mojo/public/cpp/bindings/associated_receiver_set.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

// This class implements a RemoteMainFrameHost that will be called when the
// renderer normally sends a request to the browser process. But for a unittest
// setup it can be intercepted by this class with passing a corresponding
// fake remote.
class FakeRemoteMainFrameHost : public mojom::blink::RemoteMainFrameHost {
 public:
  FakeRemoteMainFrameHost() = default;

  mojo::PendingAssociatedRemote<mojom::blink::RemoteMainFrameHost>
  BindNewAssociatedRemote();

  // blink::mojom::RemoteMainFrameHost overrides:
  void FocusPage() override;
  void TakeFocus(bool reverse) override;
  void UpdateTargetURL(
      const KURL&,
      mojom::blink::RemoteMainFrameHost::UpdateTargetURLCallback) override;
  void RouteCloseEvent() override;

 private:
  mojo::AssociatedReceiver<mojom::blink::RemoteMainFrameHost> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_REMOTE_MAIN_FRAME_HOST_H_
