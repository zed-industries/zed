// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMPUTE_PIPELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMPUTE_PIPELINE_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_conversions.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class GPUBindGroupLayout;
class GPUComputePipelineDescriptor;
struct OwnedProgrammableStage;

wgpu::ComputePipelineDescriptor AsDawnType(
    GPUDevice* device,
    const GPUComputePipelineDescriptor* webgpu_desc,
    std::string* label,
    OwnedProgrammableStage* computeStage);

class GPUComputePipeline : public DawnObject<wgpu::ComputePipeline> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUComputePipeline* Create(
      GPUDevice* device,
      const GPUComputePipelineDescriptor* webgpu_desc);
  explicit GPUComputePipeline(GPUDevice* device,
                              wgpu::ComputePipeline compute_pipeline,
                              const String& label);

  GPUComputePipeline(const GPUComputePipeline&) = delete;
  GPUComputePipeline& operator=(const GPUComputePipeline&) = delete;

  GPUBindGroupLayout* getBindGroupLayout(uint32_t index);

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMPUTE_PIPELINE_H_
