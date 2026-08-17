// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_BUFFER_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class GPUCommandBuffer : public DawnObject<wgpu::CommandBuffer> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit GPUCommandBuffer(GPUDevice* device,
                            wgpu::CommandBuffer command_buffer,
                            const String& label);

  GPUCommandBuffer(const GPUCommandBuffer&) = delete;
  GPUCommandBuffer& operator=(const GPUCommandBuffer&) = delete;

 private:
  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COMMAND_BUFFER_H_
