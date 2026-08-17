// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUEUE_H_

#include <optional>

#include "base/containers/span.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"

namespace blink {

class ExceptionState;
class GPUBuffer;
class GPUCommandBuffer;
class GPUImageCopyExternalImage;
class GPUImageCopyTextureTagged;
class GPUTexelCopyBufferLayout;
class GPUTexelCopyTextureInfo;
class ScriptState;
class StaticBitmapImage;
struct ExternalTextureSource;

class GPUQueue : public DawnObject<wgpu::Queue> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit GPUQueue(GPUDevice* device, wgpu::Queue queue, const String& label);

  GPUQueue(const GPUQueue&) = delete;
  GPUQueue& operator=(const GPUQueue&) = delete;

  // gpu_queue.idl {{{
  void submit(ScriptState* script_state,
              const HeapVector<Member<GPUCommandBuffer>>& buffers);
  ScriptPromise<IDLUndefined> onSubmittedWorkDone(ScriptState* script_state);
  void writeBuffer(ScriptState* script_state,
                   GPUBuffer* buffer,
                   uint64_t buffer_offset,
                   const MaybeShared<DOMArrayBufferView>& data,
                   uint64_t data_element_offset,
                   ExceptionState& exception_state);
  void writeBuffer(ScriptState* script_state,
                   GPUBuffer* buffer,
                   uint64_t buffer_offset,
                   const MaybeShared<DOMArrayBufferView>& data,
                   uint64_t data_element_offset,
                   uint64_t data_element_count,
                   ExceptionState& exception_state);
  void writeBuffer(ScriptState* script_state,
                   GPUBuffer* buffer,
                   uint64_t buffer_offset,
                   const DOMArrayBufferBase* data,
                   uint64_t data_byte_offset,
                   ExceptionState& exception_state);
  void writeBuffer(ScriptState* script_state,
                   GPUBuffer* buffer,
                   uint64_t buffer_offset,
                   const DOMArrayBufferBase* data,
                   uint64_t data_byte_offset,
                   uint64_t byte_size,
                   ExceptionState& exception_state);
  void writeTexture(ScriptState* script_state,
                    GPUTexelCopyTextureInfo* destination,
                    const MaybeShared<DOMArrayBufferView>& data,
                    GPUTexelCopyBufferLayout* data_layout,
                    const V8GPUExtent3D* write_size,
                    ExceptionState& exception_state);
  void writeTexture(ScriptState* script_state,
                    GPUTexelCopyTextureInfo* destination,
                    const DOMArrayBufferBase* data,
                    GPUTexelCopyBufferLayout* data_layout,
                    const V8GPUExtent3D* write_size,
                    ExceptionState& exception_state);
  void copyExternalImageToTexture(GPUImageCopyExternalImage* copyImage,
                                  GPUImageCopyTextureTagged* destination,
                                  const V8GPUExtent3D* copySize,
                                  ExceptionState& exception_state);
  // }}} End of WebIDL binding implementation.

 private:
  void CopyFromVideoElement(const ExternalTextureSource source,
                            const wgpu::Extent2D& video_frame_natural_size,
                            const wgpu::Origin2D& origin,
                            const wgpu::Extent3D& copy_size,
                            const wgpu::TexelCopyTextureInfo& destination,
                            bool dst_premultiplied_alpha,
                            PredefinedColorSpace dst_color_space,
                            bool flipY);
  bool CopyFromCanvasSourceImage(StaticBitmapImage* image,
                                 const wgpu::Origin2D& origin,
                                 const wgpu::Extent3D& copy_size,
                                 const wgpu::TexelCopyTextureInfo& destination,
                                 bool dst_premultiplied_alpha,
                                 PredefinedColorSpace dst_color_space,
                                 bool flipY);
  void WriteBufferImpl(ScriptState* script_state,
                       GPUBuffer* buffer,
                       uint64_t buffer_offset,
                       base::span<const uint8_t> data,
                       unsigned data_bytes_per_element,
                       uint64_t data_byte_offset,
                       std::optional<uint64_t> byte_size,
                       ExceptionState& exception_state);
  void WriteTextureImpl(ScriptState* script_state,
                        GPUTexelCopyTextureInfo* destination,
                        base::span<const uint8_t> data,
                        GPUTexelCopyBufferLayout* data_layout,
                        const V8GPUExtent3D* write_size,
                        ExceptionState& exception_state);

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUEUE_H_
