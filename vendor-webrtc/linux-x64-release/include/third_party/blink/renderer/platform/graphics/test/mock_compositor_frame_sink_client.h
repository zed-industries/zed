// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_COMPOSITOR_FRAME_SINK_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_COMPOSITOR_FRAME_SINK_CLIENT_H_

#include "mojo/public/cpp/bindings/receiver.h"
#include "services/viz/public/mojom/compositing/compositor_frame_sink.mojom-blink.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace blink {

class MockCompositorFrameSinkClient
    : public viz::mojom::blink::CompositorFrameSinkClient {
 public:
  // mojom::blink::CompositorFrameSinkClient implementation:
  MOCK_METHOD1(DidReceiveCompositorFrameAck,
               void(WTF::Vector<viz::ReturnedResource>));
  MOCK_METHOD3(OnBeginFrame,
               void(const viz::BeginFrameArgs&,
                    const WTF::HashMap<uint32_t, viz::FrameTimingDetails>&,
                    WTF::Vector<viz::ReturnedResource>));
  MOCK_METHOD1(ReclaimResources, void(WTF::Vector<viz::ReturnedResource>));
  MOCK_METHOD2(WillDrawSurface,
               void(const viz::LocalSurfaceId&, const gfx::Rect&));
  MOCK_METHOD1(OnBeginFramePausedChanged, void(bool paused));
  MOCK_METHOD1(OnCompositorFrameTransitionDirectiveProcessed,
               void(uint32_t sequence_id));
  MOCK_METHOD1(OnSurfaceEvicted, void(const viz::LocalSurfaceId&));

 private:
  mojo::Receiver<viz::mojom::blink::CompositorFrameSinkClient> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_COMPOSITOR_FRAME_SINK_CLIENT_H_
