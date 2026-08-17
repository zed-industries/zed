// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_H_

#include "base/memory/scoped_refptr.h"
#include "gpu/command_buffer/client/webgpu_interface.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GPU;
class GPUAdapterInfo;
class GPUDevice;
class GPUDeviceDescriptor;
class GPURequestAdapterOptions;
class GPUSupportedFeatures;
class GPUSupportedLimits;
class GPUMemoryHeapInfo;

class GPUAdapter final : public ScriptWrappable, DawnObject<wgpu::Adapter> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GPUAdapter(GPU* gpu,
             wgpu::Adapter handle,
             scoped_refptr<DawnControlClientHolder> dawn_control_client,
             const GPURequestAdapterOptions* options);

  GPUAdapter(const GPUAdapter&) = delete;
  GPUAdapter& operator=(const GPUAdapter&) = delete;

  void Trace(Visitor* visitor) const override;

  GPU* gpu() const { return gpu_.Get(); }
  GPUSupportedFeatures* features() const;
  GPUSupportedLimits* limits() const { return limits_.Get(); }
  GPUAdapterInfo* info() const;
  bool isFallbackAdapter() const;
  wgpu::BackendType backendType() const;
  bool SupportsMultiPlanarFormats() const;

  ScriptPromise<GPUDevice> requestDevice(ScriptState* script_state,
                                         GPUDeviceDescriptor* descriptor);

  // Console warnings should generally be attributed to a GPUDevice, but in
  // cases where there is no device warnings can be surfaced here. It's expected
  // that very few warning will need to be shown for a given adapter, and as a
  // result the maximum allowed warnings is lower than the per-device count.
  void AddConsoleWarning(ExecutionContext* execution_context,
                         const char* message);

  GPUAdapterInfo* CreateAdapterInfoForAdapter();

  bool IsXRCompatible() const { return is_xr_compatible_; }

 private:
  void OnRequestDeviceCallback(GPUDevice* device,
                               const GPUDeviceDescriptor* descriptor,
                               ScriptPromiseResolver<GPUDevice>* resolver,
                               wgpu::RequestDeviceStatus status,
                               wgpu::Device dawn_device,
                               wgpu::StringView error_message);

  void SetLabelImpl(const String&) override {
    // There isn't a wgpu::Adapter::SetLabel, just skip.
  }

  Member<GPU> gpu_;
  bool is_fallback_adapter_;
  wgpu::BackendType backend_type_;
  wgpu::AdapterType adapter_type_;
  bool is_consumed_ = false;
  bool is_xr_compatible_ = false;
  Member<GPUSupportedLimits> limits_;
  Member<GPUSupportedFeatures> features_;
  Member<GPUAdapterInfo> info_;

  String vendor_;
  String architecture_;
  String device_;
  String description_;
  uint32_t subgroup_min_size_;
  uint32_t subgroup_max_size_;
  String driver_;
  HeapVector<Member<GPUMemoryHeapInfo>> memory_heaps_;
  std::optional<uint32_t> d3d_shader_model_;
  std::optional<uint32_t> vk_driver_version_;
  wgpu::PowerPreference power_preference_;

  static constexpr int kMaxAllowedConsoleWarnings = 50;
  int allowed_console_warnings_remaining_ = kMaxAllowedConsoleWarnings;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_H_
