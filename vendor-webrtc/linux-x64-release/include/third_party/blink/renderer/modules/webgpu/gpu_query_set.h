// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUERY_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUERY_SET_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_query_type.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

class GPUQuerySetDescriptor;
class V8GPUQueryType;

class GPUQuerySet : public DawnObject<wgpu::QuerySet> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUQuerySet* Create(GPUDevice* device,
                             const GPUQuerySetDescriptor* webgpu_desc);
  explicit GPUQuerySet(GPUDevice* device,
                       wgpu::QuerySet querySet,
                       const String& label);

  GPUQuerySet(const GPUQuerySet&) = delete;
  GPUQuerySet& operator=(const GPUQuerySet&) = delete;

  // gpu_queryset.idl {{{
  void destroy();
  V8GPUQueryType type() const;
  uint32_t count() const;
  // }}} End of WebIDL binding implementation.

 private:
  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_QUERY_SET_H_
