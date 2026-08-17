// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_ENCODER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_index_format.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_enum_conversions.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/modules/webgpu/gpu_programmable_pass_encoder.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"

namespace blink {

class GPUBindGroup;
class GPURenderBundle;
class GPURenderBundleDescriptor;
class GPURenderBundleEncoderDescriptor;

class GPURenderBundleEncoder : public DawnObject<wgpu::RenderBundleEncoder>,
                               public GPUProgrammablePassEncoder {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPURenderBundleEncoder* Create(
      GPUDevice* device,
      const GPURenderBundleEncoderDescriptor* webgpu_desc,
      ExceptionState& exception_state);
  explicit GPURenderBundleEncoder(
      GPUDevice* device,
      wgpu::RenderBundleEncoder render_bundle_encoder,
      const String& label);

  GPURenderBundleEncoder(const GPURenderBundleEncoder&) = delete;
  GPURenderBundleEncoder& operator=(const GPURenderBundleEncoder&) = delete;

  // gpu_render_bundle_encoder.idl {{{
  void setBindGroup(uint32_t index, DawnObject<wgpu::BindGroup>* bindGroup) {
    GetHandle().SetBindGroup(
        index, bindGroup ? bindGroup->GetHandle() : wgpu::BindGroup(nullptr), 0,
        nullptr);
  }
  void setBindGroup(uint32_t index,
                    GPUBindGroup* bindGroup,
                    const Vector<uint32_t>& dynamicOffsets);
  void setBindGroup(uint32_t index,
                    GPUBindGroup* bind_group,
                    base::span<const uint32_t> dynamic_offsets_data,
                    uint64_t dynamic_offsets_data_start,
                    uint32_t dynamic_offsets_data_length,
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
  void setPipeline(const DawnObject<wgpu::RenderPipeline>* pipeline) {
    GetHandle().SetPipeline(pipeline->GetHandle());
  }
  void setIndexBuffer(const DawnObject<wgpu::Buffer>* buffer,
                      const V8GPUIndexFormat& format,
                      uint64_t offset) {
    GetHandle().SetIndexBuffer(buffer->GetHandle(), AsDawnEnum(format), offset);
  }
  void setIndexBuffer(const DawnObject<wgpu::Buffer>* buffer,
                      const V8GPUIndexFormat& format,
                      uint64_t offset,
                      uint64_t size) {
    GetHandle().SetIndexBuffer(buffer->GetHandle(), AsDawnEnum(format), offset,
                               size);
  }
  void setVertexBuffer(uint32_t slot,
                       const DawnObject<wgpu::Buffer>* buffer,
                       uint64_t offset) {
    GetHandle().SetVertexBuffer(
        slot, buffer ? buffer->GetHandle() : wgpu::Buffer(nullptr), offset);
  }
  void setVertexBuffer(uint32_t slot,
                       const DawnObject<wgpu::Buffer>* buffer,
                       uint64_t offset,
                       uint64_t size) {
    GetHandle().SetVertexBuffer(
        slot, buffer ? buffer->GetHandle() : wgpu::Buffer(nullptr), offset,
        size);
  }
  void draw(uint32_t vertexCount,
            uint32_t instanceCount,
            uint32_t firstVertex,
            uint32_t firstInstance) {
    GetHandle().Draw(vertexCount, instanceCount, firstVertex, firstInstance);
  }
  void drawIndexed(uint32_t indexCount,
                   uint32_t instanceCount,
                   uint32_t firstIndex,
                   int32_t baseVertex,
                   uint32_t firstInstance) {
    GetHandle().DrawIndexed(indexCount, instanceCount, firstIndex, baseVertex,
                            firstInstance);
  }
  void drawIndirect(const DawnObject<wgpu::Buffer>* indirectBuffer,
                    uint64_t indirectOffset) {
    GetHandle().DrawIndirect(indirectBuffer->GetHandle(), indirectOffset);
  }
  void drawIndexedIndirect(const DawnObject<wgpu::Buffer>* indirectBuffer,
                           uint64_t indirectOffset) {
    GetHandle().DrawIndexedIndirect(indirectBuffer->GetHandle(),
                                    indirectOffset);
  }
  GPURenderBundle* finish(const GPURenderBundleDescriptor* webgpu_desc);
  // }}} End of WebIDL binding implementation.

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_ENCODER_H_
