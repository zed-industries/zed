// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_DEVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_DEVICE_H_

#include <bitset>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_blend_factor.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_texture_format.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_callback.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;
class ExternalTextureCache;
class GPUAdapter;
class GPUAdapterInfo;
class GPUBuffer;
class GPUBufferDescriptor;
class GPUCommandEncoder;
class GPUCommandEncoderDescriptor;
class GPUBindGroup;
class GPUBindGroupDescriptor;
class GPUBindGroupLayout;
class GPUBindGroupLayoutDescriptor;
class GPUComputePipeline;
class GPUComputePipelineDescriptor;
class GPUDeviceDescriptor;
class GPUDeviceLostInfo;
class GPUError;
class GPUExternalTexture;
class GPUExternalTextureDescriptor;
class GPUPipelineLayout;
class GPUPipelineLayoutDescriptor;
class GPUQuerySet;
class GPUQuerySetDescriptor;
class GPUQueue;
class GPURenderBundleEncoder;
class GPURenderBundleEncoderDescriptor;
class GPURenderPipeline;
class GPURenderPipelineDescriptor;
class GPUSampler;
class GPUSamplerDescriptor;
class GPUShaderModule;
class GPUShaderModuleDescriptor;
class GPUSupportedFeatures;
class GPUSupportedLimits;
class GPUTexture;
class GPUTextureDescriptor;
class ScriptState;
class V8GPUErrorFilter;

// Singleton warnings are messages that can only be raised once per device. They
// should be used for warnings of behavior that is not invalid but may have
// performance issues or side effects that the developer may overlook since a
// regular warning is not raised.
enum class GPUSingletonWarning {
  kNonPreferredFormat,
  kDepthKey,
  kCount,  // Must be last
};

class GPUDevice final : public EventTarget,
                        public ExecutionContextClient,
                        public DawnObject<wgpu::Device> {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(GPUDevice, Dispose);

 public:
  // Creates the device without a handle so that some callbacks can be set up
  // in the wgpu::DeviceDescriptor that will be used to create this device.
  // Initialize is where the handle is given, and the rest of the initialization
  // happens.
  GPUDevice(ExecutionContext* execution_context,
            scoped_refptr<DawnControlClientHolder> dawn_control_client,
            GPUAdapter* adapter,
            const String& label);

  // The second step of the initialization once we have the result from
  // wgpu::Adapter::RequestDevice. The lost_info if non-null will be used to
  // resolve the lost property.
  void Initialize(wgpu::Device handle,
                  const GPUDeviceDescriptor* descriptor,
                  GPUDeviceLostInfo* lost_info);

  GPUDevice(const GPUDevice&) = delete;
  GPUDevice& operator=(const GPUDevice&) = delete;

  ~GPUDevice() override;

  void Trace(Visitor* visitor) const override;

  // gpu_device.idl {{{
  GPUAdapter* adapter() const;
  GPUSupportedFeatures* features() const;
  GPUSupportedLimits* limits() const { return limits_.Get(); }
  GPUAdapterInfo* adapterInfo() const;
  ScriptPromise<GPUDeviceLostInfo> lost(ScriptState* script_state);
  // }}} End of WebIDL binding implementation.

  GPUQueue* queue();

  void destroy(v8::Isolate* isolate);

  GPUBuffer* createBuffer(const GPUBufferDescriptor* descriptor,
                          ExceptionState& exception_state);
  GPUTexture* createTexture(const GPUTextureDescriptor* descriptor,
                            ExceptionState& exception_state);
  GPUSampler* createSampler(const GPUSamplerDescriptor* descriptor);

  GPUExternalTexture* importExternalTexture(
      const GPUExternalTextureDescriptor* descriptor,
      ExceptionState& exception_state);

  GPUBindGroup* createBindGroup(const GPUBindGroupDescriptor* descriptor,
                                ExceptionState& exception_state);
  GPUBindGroupLayout* createBindGroupLayout(
      const GPUBindGroupLayoutDescriptor* descriptor,
      ExceptionState& exception_state);
  GPUPipelineLayout* createPipelineLayout(
      const GPUPipelineLayoutDescriptor* descriptor);

  GPUShaderModule* createShaderModule(
      const GPUShaderModuleDescriptor* descriptor);
  GPURenderPipeline* createRenderPipeline(
      ScriptState* script_state,
      const GPURenderPipelineDescriptor* descriptor);
  GPUComputePipeline* createComputePipeline(
      const GPUComputePipelineDescriptor* descriptor,
      ExceptionState& exception_state);
  ScriptPromise<GPURenderPipeline> createRenderPipelineAsync(
      ScriptState* script_state,
      const GPURenderPipelineDescriptor* descriptor,
      ExceptionState&);
  ScriptPromise<GPUComputePipeline> createComputePipelineAsync(
      ScriptState* script_state,
      const GPUComputePipelineDescriptor* descriptor);

  GPUCommandEncoder* createCommandEncoder(
      const GPUCommandEncoderDescriptor* descriptor);
  GPURenderBundleEncoder* createRenderBundleEncoder(
      const GPURenderBundleEncoderDescriptor* descriptor,
      ExceptionState& exception_state);

  GPUQuerySet* createQuerySet(const GPUQuerySetDescriptor* descriptor,
                              ExceptionState& exception_state);

  void pushErrorScope(const V8GPUErrorFilter& filter);
  ScriptPromise<IDLNullable<GPUError>> popErrorScope(ScriptState* script_state);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(uncapturederror, kUncapturederror)

  // EventTarget overrides.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  bool IsDestroyed() const;
  std::string GetFormattedLabel() const;
  void InjectError(wgpu::ErrorType type, const char* message);
  void AddConsoleWarning(const String& message);
  void AddConsoleWarning(const char* message);
  void AddConsoleWarning(wgpu::StringView message);
  void AddSingletonWarning(GPUSingletonWarning type);

  void TrackTextureWithMailbox(GPUTexture* texture);
  void UntrackTextureWithMailbox(GPUTexture* texture);

  bool ValidateTextureFormatUsage(V8GPUTextureFormat format,
                                  ExceptionState& exception_state);
  bool ValidateBlendFactor(V8GPUBlendFactor blend_factor,
                           ExceptionState& exception_state);

  // Store the buffer in a weak hash set so we can unmap it when the
  // device is destroyed.
  void TrackMappableBuffer(GPUBuffer* buffer);
  // Untrack the GPUBuffer. This is called eagerly when the buffer is
  // destroyed.
  void UntrackMappableBuffer(GPUBuffer* buffer);

  // Helper used to set the wgpu::DeviceDescriptor callbacks during the first
  // steps of GPUDevice creation. Note that this helper should only ever be
  // called once per GPUDevice.
  void SetDescriptorCallbacks(wgpu::DeviceDescriptor& dawn_desc);

 private:
  using LostProperty = ScriptPromiseProperty<GPUDeviceLostInfo, IDLUndefined>;

  // Used by USING_PRE_FINALIZER.
  void Dispose();
  void DissociateMailboxes();
  void UnmapAllMappableBuffers(v8::Isolate* isolate);

  void OnUncapturedError(const wgpu::Device& device,
                         wgpu::ErrorType errorType,
                         wgpu::StringView message);
  void OnLogging(wgpu::LoggingType loggingType, wgpu::StringView message);
  void OnDeviceLost(
      std::unique_ptr<
          WGPURepeatingCallback<wgpu::UncapturedErrorCallback<void>>>,
      const wgpu::Device& device,
      wgpu::DeviceLostReason reason,
      wgpu::StringView message);

  void OnPopErrorScopeCallback(
      ScriptPromiseResolver<IDLNullable<GPUError>>* resolver,
      wgpu::PopErrorScopeStatus status,
      wgpu::ErrorType type,
      wgpu::StringView message);

  void OnCreateRenderPipelineAsyncCallback(
      const String& label,
      ScriptPromiseResolver<GPURenderPipeline>* resolver,
      wgpu::CreatePipelineAsyncStatus status,
      wgpu::RenderPipeline render_pipeline,
      wgpu::StringView message);
  void OnCreateComputePipelineAsyncCallback(
      const String& label,
      ScriptPromiseResolver<GPUComputePipeline>* resolver,
      wgpu::CreatePipelineAsyncStatus status,
      wgpu::ComputePipeline compute_pipeline,
      wgpu::StringView message);

  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }

  Member<GPUAdapter> adapter_;
  Member<GPUSupportedFeatures> features_;
  Member<GPUSupportedLimits> limits_;
  Member<GPUAdapterInfo> adapter_info_;
  Member<GPUQueue> queue_;
  Member<LostProperty> lost_property_;
  std::unique_ptr<WGPURepeatingCallback<wgpu::LoggingCallback<void>>>
      logging_callback_;

  static constexpr int kMaxAllowedConsoleWarnings = 500;
  int allowed_console_warnings_remaining_ = kMaxAllowedConsoleWarnings;

  // Textures with mailboxes that should be dissociated before device.destroy().
  HeapHashSet<WeakMember<GPUTexture>> textures_with_mailbox_;

  HeapHashSet<WeakMember<GPUBuffer>> mappable_buffers_;

  Member<ExternalTextureCache> external_texture_cache_;

  std::bitset<static_cast<size_t>(GPUSingletonWarning::kCount)>
      singleton_warning_fired_;

  // This attribute records that whether GPUDevice is destroyed (via destroy()).
  bool destroyed_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_DEVICE_H_
