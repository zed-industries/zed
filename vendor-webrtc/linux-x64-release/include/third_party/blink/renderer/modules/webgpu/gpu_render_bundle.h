// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class GPUDevice;

class GPURenderBundle : public DawnObject<wgpu::RenderBundle> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit GPURenderBundle(GPUDevice* device,
                           wgpu::RenderBundle render_bundle,
                           const String& label);

  GPURenderBundle(const GPURenderBundle&) = delete;
  GPURenderBundle& operator=(const GPURenderBundle&) = delete;

 private:
  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_RENDER_BUNDLE_H_
