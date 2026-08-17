// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BIND_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BIND_GROUP_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class ExceptionState;
class GPUBindGroupDescriptor;

class GPUBindGroup : public DawnObject<wgpu::BindGroup> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUBindGroup* Create(GPUDevice* device,
                              const GPUBindGroupDescriptor* webgpu_desc,
                              ExceptionState& exception_state);
  explicit GPUBindGroup(GPUDevice* device,
                        wgpu::BindGroup bind_group,
                        const String& label);

  GPUBindGroup(const GPUBindGroup&) = delete;
  GPUBindGroup& operator=(const GPUBindGroup&) = delete;

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BIND_GROUP_H_
