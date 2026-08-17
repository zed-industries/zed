// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_ENCODER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace blink {

class ExceptionState;
class GPUCommandBuffer;
class GPUCommandBufferDescriptor;
class GPUCommandEncoderDescriptor;
class GPUComputePassDescriptor;
class GPUComputePassEncoder;
class GPURenderPassDescriptor;
class GPURenderPassEncoder;
class GPUTexelCopyBufferInfo;
class GPUTexelCopyTextureInfo;

class GPUCommandEncoder : public DawnObject<wgpu::CommandEncoder> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUCommandEncoder* Create(
      GPUDevice* device,
      const GPUCommandEncoderDescriptor* webgpu_desc);
  explicit GPUCommandEncoder(GPUDevice* device,
                             wgpu::CommandEncoder command_encoder,
                             const String& label);

  GPUCommandEncoder(const GPUCommandEncoder&) = delete;
  GPUCommandEncoder& operator=(const GPUCommandEncoder&) = delete;

  // gpu_command_encoder.idl {{{
  GPURenderPassEncoder* beginRenderPass(
      const GPURenderPassDescriptor* descriptor,
      ExceptionState& exception_state);
  GPUComputePassEncoder* beginComputePass(
      const GPUComputePassDescriptor* descriptor,
      ExceptionState& exception_state);
  void copyBufferToBuffer(DawnObject<wgpu::Buffer>* source,
                          DawnObject<wgpu::Buffer>* destination,
                          ExceptionState& exception_state) {
    if (!RuntimeEnabledFeatures::WebGPUCopyBufferToBufferOverloadEnabled()) {
      exception_state.ThrowTypeError("Offsets and size are required.");
      return;
    }
    copyBufferToBuffer(source, 0, destination, 0, exception_state);
  }
  void copyBufferToBuffer(DawnObject<wgpu::Buffer>* source,
                          DawnObject<wgpu::Buffer>* destination,
                          uint64_t size,
                          ExceptionState& exception_state) {
    if (!RuntimeEnabledFeatures::WebGPUCopyBufferToBufferOverloadEnabled()) {
      exception_state.ThrowTypeError("Offsets are required.");
      return;
    }
    copyBufferToBuffer(source, 0, destination, 0, size, exception_state);
  }
  void copyBufferToBuffer(DawnObject<wgpu::Buffer>* source,
                          uint64_t source_offset,
                          DawnObject<wgpu::Buffer>* destination,
                          uint64_t destination_offset,
                          ExceptionState& exception_state) {
    if (!RuntimeEnabledFeatures::WebGPUCopyBufferToBufferOverloadEnabled()) {
      exception_state.ThrowTypeError("Size is required.");
      return;
    }
    DCHECK(source);
    // Underflow in the size calculation is acceptable because a GPU validation
    // error will be fired if the resulting size is a very large positive
    // integer. The offset is validated to be less than the buffer size before
    // we compute the remaining size in the buffer.
    copyBufferToBuffer(source, source_offset, destination, destination_offset,
                       source->GetHandle().GetSize() - source_offset,
                       exception_state);
  }
  void copyBufferToBuffer(DawnObject<wgpu::Buffer>* source,
                          uint64_t source_offset,
                          DawnObject<wgpu::Buffer>* destination,
                          uint64_t destination_offset,
                          uint64_t size,
                          ExceptionState& exception_state) {
    DCHECK(source);
    DCHECK(destination);
    GetHandle().CopyBufferToBuffer(source->GetHandle(), source_offset,
                                   destination->GetHandle(), destination_offset,
                                   size);
  }
  void copyBufferToTexture(GPUTexelCopyBufferInfo* source,
                           GPUTexelCopyTextureInfo* destination,
                           const V8GPUExtent3D* copy_size,
                           ExceptionState& exception_state);
  void copyTextureToBuffer(GPUTexelCopyTextureInfo* source,
                           GPUTexelCopyBufferInfo* destination,
                           const V8GPUExtent3D* copy_size,
                           ExceptionState& exception_state);
  void copyTextureToTexture(GPUTexelCopyTextureInfo* source,
                            GPUTexelCopyTextureInfo* destination,
                            const V8GPUExtent3D* copy_size,
                            ExceptionState& exception_state);
  void pushDebugGroup(String groupLabel) {
    std::string label = groupLabel.Utf8();
    GetHandle().PushDebugGroup(label.c_str());
  }
  void popDebugGroup() { GetHandle().PopDebugGroup(); }
  void insertDebugMarker(String markerLabel) {
    std::string label = markerLabel.Utf8();
    GetHandle().InsertDebugMarker(label.c_str());
  }
  void resolveQuerySet(DawnObject<wgpu::QuerySet>* querySet,
                       uint32_t firstQuery,
                       uint32_t queryCount,
                       DawnObject<wgpu::Buffer>* destination,
                       uint64_t destinationOffset) {
    GetHandle().ResolveQuerySet(querySet->GetHandle(), firstQuery, queryCount,
                                destination->GetHandle(), destinationOffset);
  }
  void writeTimestamp(DawnObject<wgpu::QuerySet>* querySet,
                      uint32_t queryIndex,
                      ExceptionState& exception_state);
  void clearBuffer(DawnObject<wgpu::Buffer>* buffer, uint64_t offset) {
    GetHandle().ClearBuffer(buffer->GetHandle(), offset);
  }
  void clearBuffer(DawnObject<wgpu::Buffer>* buffer,
                   uint64_t offset,
                   uint64_t size) {
    GetHandle().ClearBuffer(buffer->GetHandle(), offset, size);
  }
  GPUCommandBuffer* finish(const GPUCommandBufferDescriptor* descriptor);
  // }}} End of WebIDL binding implementation.

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_ENCODER_H_
