// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_FRAME_SINK_BUNDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_FRAME_SINK_BUNDLE_H_

#include <utility>

#include "base/memory/read_only_shared_memory_region.h"
#include "build/build_config.h"
#include "components/viz/common/quads/compositor_frame.h"
#include "gpu/ipc/common/mailbox.mojom-blink.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/viz/public/mojom/compositing/frame_sink_bundle.mojom-blink.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace blink {

class MockFrameSinkBundle : public viz::mojom::blink::FrameSinkBundle {
 public:
  MockFrameSinkBundle(
      mojo::PendingReceiver<viz::mojom::blink::FrameSinkBundle> receiver,
      mojo::PendingRemote<viz::mojom::blink::FrameSinkBundleClient> client)
      : receiver_(this, std::move(receiver)), client_(std::move(client)) {}

  MockFrameSinkBundle(const MockFrameSinkBundle&) = delete;
  MockFrameSinkBundle& operator=(const MockFrameSinkBundle&) = delete;

  void Disconnect() { receiver_.reset(); }
  void FlushReceiver() { receiver_.FlushForTesting(); }

  viz::mojom::blink::FrameSinkBundleClient& client() { return *client_.get(); }

  // viz::mojom::blink::FrameSinkBundle implementation:
  MOCK_METHOD2(InitializeCompositorFrameSinkType,
               void(uint32_t, viz::mojom::CompositorFrameSinkType));
  MOCK_METHOD2(SetNeedsBeginFrame, void(uint32_t, bool));
  MOCK_METHOD1(SetWantsBeginFrameAcks, void(uint32_t));
  MOCK_METHOD1(Submit,
               void(WTF::Vector<viz::mojom::blink::BundledFrameSubmissionPtr>));
#if BUILDFLAG(IS_ANDROID)
  MOCK_METHOD2(SetThreads, void(uint32_t, const WTF::Vector<viz::Thread>&));
#endif

 private:
  mojo::Receiver<viz::mojom::blink::FrameSinkBundle> receiver_{this};
  mojo::Remote<viz::mojom::blink::FrameSinkBundleClient> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_FRAME_SINK_BUNDLE_H_
