// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace WTF {

template <>
struct HashTraits<wgpu::Buffer> : GenericHashTraits<wgpu::Buffer> {
  STATIC_ONLY(HashTraits);
  static unsigned GetHash(const wgpu::Buffer& buffer) {
    return HashPointer(buffer.Get());
  }
  static bool Equal(const wgpu::Buffer& a, const wgpu::Buffer& b) {
    return a.Get() == b.Get();
  }

  static constexpr bool kEmptyValueIsZero = true;
  static std::nullptr_t EmptyValue() { return nullptr; }
  static std::nullptr_t DeletedValue() { return nullptr; }
};

}  // namespace WTF
namespace blink {

class GPUAdapter;
class GPUBuffer;
class GPURequestAdapterOptions;
class NavigatorBase;
class ScriptState;
class DawnControlClientHolder;
class V8GPUTextureFormat;
class WGSLLanguageFeatures;

struct BoxedMappableWGPUBufferHandles
    : public RefCounted<BoxedMappableWGPUBufferHandles> {
 public:
  void insert(const wgpu::Buffer& buffer) { contents_.insert(buffer); }
  void erase(const wgpu::Buffer& buffer) { contents_.erase(buffer); }

  void ClearAndDestroyAll();

 private:
  HashSet<wgpu::Buffer> contents_;
};

class MODULES_EXPORT GPU final : public ScriptWrappable,
                                 public Supplement<NavigatorBase>,
                                 public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Getter for navigator.gpu
  static GPU* gpu(NavigatorBase&);

  explicit GPU(NavigatorBase&);

  GPU(const GPU&) = delete;
  GPU& operator=(const GPU&) = delete;

  ~GPU() override;

  // ScriptWrappable overrides
  void Trace(Visitor* visitor) const override;

  // ExecutionContextLifecycleObserver overrides
  void ContextDestroyed() override;

  // gpu.idl {{{
  ScriptPromise<IDLNullable<GPUAdapter>> requestAdapter(
      ScriptState* script_state,
      const GPURequestAdapterOptions* options);
  V8GPUTextureFormat getPreferredCanvasFormat();
  WGSLLanguageFeatures* wgslLanguageFeatures() const;
  // }}} End of WebIDL binding implementation.

  static wgpu::TextureFormat GetPreferredCanvasFormat();

  // Store the buffer in a weak hash set so we can destroy it when the
  // context is destroyed.
  void TrackMappableBuffer(GPUBuffer* buffer);
  // Untrack the GPUBuffer. This is called eagerly when the buffer is
  // destroyed.
  void UntrackMappableBuffer(GPUBuffer* buffer);

  BoxedMappableWGPUBufferHandles* GetMappableBufferHandles() const {
    return mappable_buffer_handles_.get();
  }

  void SetDawnControlClientHolderForTesting(
      scoped_refptr<DawnControlClientHolder> dawn_control_client);

 private:
  void OnRequestAdapterCallback(
      ScriptState* script_state,
      const GPURequestAdapterOptions* options,
      ScriptPromiseResolver<IDLNullable<GPUAdapter>>* resolver,
      wgpu::RequestAdapterStatus status,
      wgpu::Adapter adapter,
      wgpu::StringView error_message);

  void RecordAdapterForIdentifiability(ScriptState* script_state,
                                       const GPURequestAdapterOptions* options,
                                       GPUAdapter* adapter) const;

  void RequestAdapterImpl(ScriptState* script_state,
                          const GPURequestAdapterOptions* options,
                          ScriptPromiseResolver<IDLNullable<GPUAdapter>>*);

  Member<WGSLLanguageFeatures> wgsl_language_features_;

  scoped_refptr<DawnControlClientHolder> dawn_control_client_;
  WTF::Vector<base::OnceCallback<void()>>
      dawn_control_client_initialized_callbacks_;
  HeapHashSet<WeakMember<GPUBuffer>> mappable_buffers_;
  // Mappable buffers remove themselves from this set on destruction.
  // It is boxed in a scoped_refptr so GPUBuffer can access it in its
  // destructor.
  scoped_refptr<BoxedMappableWGPUBufferHandles> mappable_buffer_handles_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_H_
