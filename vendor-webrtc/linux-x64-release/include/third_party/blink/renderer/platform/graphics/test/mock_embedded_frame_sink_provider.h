// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/390223051): Remove C-library calls to fix the errors.
#pragma allow_unsafe_libc_calls
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_EMBEDDED_FRAME_SINK_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_EMBEDDED_FRAME_SINK_PROVIDER_H_

#include <memory>
#include <utility>

#include "components/viz/common/surfaces/frame_sink_id.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/frame_sinks/embedded_frame_sink.mojom-blink.h"
#include "third_party/blink/renderer/platform/graphics/test/mock_compositor_frame_sink.h"
#include "third_party/blink/renderer/platform/graphics/test/mock_frame_sink_bundle.h"
#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// A Provider that creates and binds a MockCompositorFrameSink when requested.
class MockEmbeddedFrameSinkProvider
    : public mojom::blink::EmbeddedFrameSinkProvider {
 public:
  // EmbeddedFrameSinkProvider implementation.
  MOCK_METHOD3(
      RegisterEmbeddedFrameSink,
      void(const viz::FrameSinkId&,
           const viz::FrameSinkId&,
           mojo::PendingRemote<mojom::blink::EmbeddedFrameSinkClient>));
  void RegisterEmbeddedFrameSinkBundle(
      const viz::FrameSinkBundleId&,
      mojo::PendingReceiver<viz::mojom::blink::FrameSinkBundle> receiver,
      mojo::PendingRemote<viz::mojom::blink::FrameSinkBundleClient> client)
      override {
    mock_frame_sink_bundle_ = std::make_unique<MockFrameSinkBundle>(
        std::move(receiver), std::move(client));
    CreateFrameSinkBundle_();
  }
  MOCK_METHOD0(CreateFrameSinkBundle_, void());
  void CreateCompositorFrameSink(
      const viz::FrameSinkId& frame_sink_id,
      mojo::PendingRemote<viz::mojom::blink::CompositorFrameSinkClient> client,
      mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSink> sink)
      override {
    mock_compositor_frame_sink_ = std::make_unique<MockCompositorFrameSink>(
        std::move(sink),
        num_expected_set_needs_begin_frame_on_sink_construction_);
    CreateCompositorFrameSink_(frame_sink_id);
  }
  MOCK_METHOD1(CreateCompositorFrameSink_, void(const viz::FrameSinkId&));
  void CreateBundledCompositorFrameSink(
      const viz::FrameSinkId& frame_sink_id,
      const viz::FrameSinkBundleId& bundle_id,
      mojo::PendingRemote<viz::mojom::blink::CompositorFrameSinkClient> client,
      mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSink> sink)
      override {
    CreateCompositorFrameSink(frame_sink_id, std::move(client),
                              std::move(sink));
  }

  MOCK_METHOD5(
      CreateSimpleCompositorFrameSink,
      void(const viz::FrameSinkId&,
           const viz::FrameSinkId&,
           mojo::PendingRemote<mojom::blink::EmbeddedFrameSinkClient>,
           mojo::PendingRemote<viz::mojom::blink::CompositorFrameSinkClient>,
           mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSink>));
  MOCK_METHOD2(ConnectToEmbedder,
               void(const viz::FrameSinkId&,
                    mojo::PendingReceiver<mojom::blink::SurfaceEmbedder>));

  MOCK_METHOD1(RegisterFrameSinkHierarchy, void(const viz::FrameSinkId&));
  MOCK_METHOD1(UnregisterFrameSinkHierarchy, void(const viz::FrameSinkId&));

  // Utility method to create a scoped EmbeddedFrameSinkProvider override.
  std::unique_ptr<TestingPlatformSupport::ScopedOverrideMojoInterface>
  CreateScopedOverrideMojoInterface(
      mojo::Receiver<mojom::blink::EmbeddedFrameSinkProvider>* receiver) {
    using mojom::blink::EmbeddedFrameSinkProvider;

    return std::make_unique<
        TestingPlatformSupport::ScopedOverrideMojoInterface>(WTF::BindRepeating(
        [](mojo::Receiver<EmbeddedFrameSinkProvider>* receiver,
           const char* interface_name, mojo::ScopedMessagePipeHandle pipe) {
          if (strcmp(interface_name, EmbeddedFrameSinkProvider::Name_))
            return;
          receiver->reset();
          receiver->Bind(mojo::PendingReceiver<EmbeddedFrameSinkProvider>(
              std::move(pipe)));
        },
        WTF::Unretained(receiver)));
  }

  // Similar to above but allows for an override that binds multiple concurrent
  // receivers.
  std::unique_ptr<TestingPlatformSupport::ScopedOverrideMojoInterface>
  CreateScopedOverrideMojoInterface(
      mojo::ReceiverSet<mojom::blink::EmbeddedFrameSinkProvider>& receivers) {
    using mojom::blink::EmbeddedFrameSinkProvider;

    return std::make_unique<
        TestingPlatformSupport::ScopedOverrideMojoInterface>(WTF::BindRepeating(
        [](EmbeddedFrameSinkProvider* impl,
           mojo::ReceiverSet<EmbeddedFrameSinkProvider>* receivers,
           const char* interface_name, mojo::ScopedMessagePipeHandle pipe) {
          if (strcmp(interface_name, EmbeddedFrameSinkProvider::Name_))
            return;
          receivers->Add(impl, mojo::PendingReceiver<EmbeddedFrameSinkProvider>(
                                   std::move(pipe)));
        },
        WTF::Unretained(this), WTF::Unretained(&receivers)));
  }

  MockCompositorFrameSink& mock_compositor_frame_sink() const {
    return *mock_compositor_frame_sink_;
  }
  MockFrameSinkBundle& mock_frame_sink_bundle() const {
    DCHECK(mock_frame_sink_bundle_);
    return *mock_frame_sink_bundle_;
  }
  void set_num_expected_set_needs_begin_frame_on_sink_construction(int value) {
    num_expected_set_needs_begin_frame_on_sink_construction_ = value;
  }

 private:
  std::unique_ptr<MockCompositorFrameSink> mock_compositor_frame_sink_;
  std::unique_ptr<MockFrameSinkBundle> mock_frame_sink_bundle_;

  // Amount of SetNeedsBeginFrame() calls to be expected by
  // MockCompositorFrameSink, passed on its construction.
  int num_expected_set_needs_begin_frame_on_sink_construction_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_EMBEDDED_FRAME_SINK_PROVIDER_H_
