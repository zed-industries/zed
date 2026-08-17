// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_SHADER_MODULE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_SHADER_MODULE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"

namespace blink {
class GPUCompilationInfo;
class GPUShaderModuleDescriptor;

class GPUShaderModule : public DawnObject<wgpu::ShaderModule> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUShaderModule* Create(GPUDevice* device,
                                 const GPUShaderModuleDescriptor* webgpu_desc);
  explicit GPUShaderModule(GPUDevice* device,
                           wgpu::ShaderModule shader_module,
                           const String& label);

  GPUShaderModule(const GPUShaderModule&) = delete;
  GPUShaderModule& operator=(const GPUShaderModule&) = delete;
  ~GPUShaderModule() override;

  ScriptPromise<GPUCompilationInfo> getCompilationInfo(
      ScriptState* script_state);

 private:
  void OnCompilationInfoCallback(
      ScriptPromiseResolver<GPUCompilationInfo>* resolver,
      wgpu::CompilationInfoRequestStatus status,
      const wgpu::CompilationInfo* info);

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }

  // Holds an estimate of the memory used by Tint for this shader module.
  V8ExternalMemoryAccounter tint_memory_estimate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_SHADER_MODULE_H_
