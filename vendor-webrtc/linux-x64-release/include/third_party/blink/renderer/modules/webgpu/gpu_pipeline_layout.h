// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_LAYOUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_LAYOUT_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class GPUPipelineLayoutDescriptor;

class GPUPipelineLayout : public DawnObject<wgpu::PipelineLayout> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUPipelineLayout* Create(
      GPUDevice* device,
      const GPUPipelineLayoutDescriptor* webgpu_desc);
  explicit GPUPipelineLayout(GPUDevice* device,
                             wgpu::PipelineLayout pipeline_layout,
                             const String& label);

  GPUPipelineLayout(const GPUPipelineLayout&) = delete;
  GPUPipelineLayout& operator=(const GPUPipelineLayout&) = delete;

 private:
  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_LAYOUT_H_
