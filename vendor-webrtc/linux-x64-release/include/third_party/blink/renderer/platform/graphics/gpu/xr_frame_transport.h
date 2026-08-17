// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_FRAME_TRANSPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_FRAME_TRANSPORT_H_

#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/public/platform/web_graphics_context_3d_provider.h"
#include "third_party/blink/renderer/platform/context_lifecycle_notifier.h"
#include "third_party/blink/renderer/platform/graphics/gpu/drawing_buffer.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace gfx {
class GpuFence;
}

namespace gpu {
class SharedImageInterface;
namespace gles2 {
class GLES2Interface;
}
}  // namespace gpu

namespace blink {

class DawnControlClientHolder;
class ImageToBufferCopier;
class Image;

class PLATFORM_EXPORT XRFrameTransport final
    : public GarbageCollected<XRFrameTransport>,
      public device::mojom::blink::XRPresentationClient {
 public:
  explicit XRFrameTransport(
      ContextLifecycleNotifier* context,
      scoped_refptr<base::SequencedTaskRunner> task_runner);
  ~XRFrameTransport() override;

  void BindSubmitFrameClient(
      mojo::PendingReceiver<device::mojom::blink::XRPresentationClient>
          receiver);

  void PresentChange();

  void SetTransportOptions(
      device::mojom::blink::XRPresentationTransportOptionsPtr);

  bool DrawingIntoSharedBuffer();

  // Call before finalizing the frame's image snapshot.
  void FramePreImage(gpu::gles2::GLES2Interface*);
  void FramePreImageWebGPU(scoped_refptr<DawnControlClientHolder>);

  bool FrameSubmit(device::mojom::blink::XRPresentationProvider*,
                   gpu::gles2::GLES2Interface*,
                   gpu::SharedImageInterface*,
                   DrawingBuffer::Client*,
                   scoped_refptr<Image> image_ref,
                   int16_t vr_frame_id);

  bool FrameSubmitWebGPU(device::mojom::blink::XRPresentationProvider*,
                         scoped_refptr<DawnControlClientHolder>,
                         wgpu::Device,
                         int16_t vr_frame_id);

  void FrameSubmitMissing(device::mojom::blink::XRPresentationProvider*,
                          gpu::gles2::GLES2Interface*,
                          int16_t vr_frame_id);
  void FrameSubmitMissingWebGPU(device::mojom::blink::XRPresentationProvider*,
                                scoped_refptr<DawnControlClientHolder>,
                                int16_t vr_frame_id);

  void RegisterFrameRenderedCallback(base::RepeatingClosure callback);

  void Trace(Visitor*) const;

 private:
  void WaitForPreviousTransfer();
  base::TimeDelta WaitForPreviousRenderToFinish();
  base::TimeDelta WaitForGpuFenceReceived();

  // XRPresentationClient
  void OnSubmitFrameTransferred(bool success) override;
  void OnSubmitFrameRendered() override;
  void OnSubmitFrameGpuFence(gfx::GpuFenceHandle) override;

  HeapMojoReceiver<device::mojom::blink::XRPresentationClient, XRFrameTransport>
      submit_frame_client_receiver_;

  // Used to keep the image alive until the next frame if using
  // waitForPreviousTransferToFinish.
  scoped_refptr<Image> previous_image_;

  bool waiting_for_previous_frame_transfer_ = false;
  bool last_transfer_succeeded_ = false;
  base::TimeDelta frame_wait_time_;
  bool waiting_for_previous_frame_render_ = false;
  // If using GpuFence to separate frames, need to wait for the previous
  // frame's fence, but not if this is the first frame. Separately track
  // if we're expecting a fence and the received fence itself.
  bool waiting_for_previous_frame_fence_ = false;
  std::unique_ptr<gfx::GpuFence> previous_frame_fence_;

  device::mojom::blink::XRPresentationTransportOptionsPtr transport_options_;

  base::RepeatingClosure on_submit_frame_rendered_callback_;

  std::unique_ptr<ImageToBufferCopier> frame_copier_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_FRAME_TRANSPORT_H_
