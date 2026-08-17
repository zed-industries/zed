// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_CONVERSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_CONVERSIONS_H_

#include <memory>

#include "base/check.h"
#include "base/containers/heap_array.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_enum_conversions.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

// This file provides helpers for converting WebGPU objects, descriptors,
// and enums from Blink to Dawn types.

namespace blink {

class ExceptionState;
class GPUTexelCopyBufferLayout;
class GPUTexelCopyTextureInfo;
class V8UnionGPUAutoLayoutModeOrGPUPipelineLayout;

// These conversions are used multiple times and are declared here. Conversions
// used only once, for example for object construction, are defined
// individually.
wgpu::TextureFormat AsDawnType(SkColorType color_type);
wgpu::PipelineLayout AsDawnType(
    V8UnionGPUAutoLayoutModeOrGPUPipelineLayout* webgpu_layout);

// Conversion for convenience types that are dict|sequence<Number> and other
// types that recursively use them. A return value of false means that the
// conversion failed and a TypeError was recorded in the ExceptionState.
bool ConvertToDawn(const V8GPUColor* in, wgpu::Color* out, ExceptionState&);
bool ConvertToDawn(const V8GPUExtent3D* in,
                   wgpu::Extent3D* out,
                   GPUDevice* device,
                   ExceptionState&);
bool ConvertToDawn(const V8GPUOrigin3D* in,
                   wgpu::Origin3D* out,
                   ExceptionState&);
bool ConvertToDawn(const V8GPUOrigin2D* in,
                   wgpu::Origin2D* out,
                   ExceptionState&);
bool ConvertToDawn(const GPUTexelCopyTextureInfo* in,
                   wgpu::TexelCopyTextureInfo* out,
                   ExceptionState&);

const char* ValidateTexelCopyBufferLayout(
    const GPUTexelCopyBufferLayout* webgpu_layout,
    wgpu::TexelCopyBufferLayout* layout);

// WebGPU objects are converted to Dawn objects by getting the opaque handle
// which can be passed to Dawn.
template <typename Handle>
Handle AsDawnType(const DawnObject<Handle>* object) {
  DCHECK(object);
  return object->GetHandle();
}

template <typename WebGPUType>
using TypeOfDawnType = decltype(AsDawnType(std::declval<const WebGPUType*>()));

// Helper for converting a list of objects to Dawn structs or handles
template <typename WebGPUType>
std::unique_ptr<TypeOfDawnType<WebGPUType>[]> AsDawnType(
    const HeapVector<Member<WebGPUType>>& webgpu_objects) {
  using DawnType = TypeOfDawnType<WebGPUType>;

  wtf_size_t count = webgpu_objects.size();
  // TODO(enga): Pass in temporary memory or an allocator so we don't make a
  // separate memory allocation here.
  std::unique_ptr<DawnType[]> dawn_objects =
      std::make_unique<DawnType[]>(count);
  for (wtf_size_t i = 0; i < count; ++i) {
    if (webgpu_objects[i]) {
      dawn_objects[i] = AsDawnType(webgpu_objects[i].Get());
    } else {
      // Construct a default object if it is null
      dawn_objects[i] = {};
    }
  }
  return dawn_objects;
}
template <typename WebGPUType, typename DawnType>
bool ConvertToDawn(const HeapVector<Member<WebGPUType>>& in,
                   std::unique_ptr<DawnType[]>* out,
                   ExceptionState& exception_state) {
  wtf_size_t count = in.size();
  // TODO(enga): Pass in temporary memory or an allocator so we don't make a
  // separate memory allocation here.
  *out = std::make_unique<DawnType[]>(count);
  for (wtf_size_t i = 0; i < count; ++i) {
    if (in[i]) {
      if (!ConvertToDawn(in[i].Get(), &(*out)[i], exception_state)) {
        return false;
      }
    } else {
      // Construct a default object if it is null
      (*out)[i] = {};
    }
  }
  return true;
}

template <typename DawnEnum, typename WebGPUEnum>
base::HeapArray<DawnEnum> AsDawnEnum(const Vector<WebGPUEnum>& webgpu_enums) {
  wtf_size_t count = webgpu_enums.size();
  // TODO(enga): Pass in temporary memory or an allocator so we don't make a
  // separate memory allocation here.
  base::HeapArray<DawnEnum> dawn_enums =
      base::HeapArray<DawnEnum>::Uninit(count);
  for (wtf_size_t i = 0; i < count; ++i) {
    dawn_enums[i] = AsDawnEnum(webgpu_enums[i]);
  }
  return dawn_enums;
}

// For sequence of nullable enums, convert null value to undefined
// dawn_enums should be a pre-allocated array with a size of count
template <typename DawnEnum, typename WebGPUEnum>
base::HeapArray<DawnEnum> AsDawnEnum(
    const Vector<std::optional<WebGPUEnum>>& webgpu_enums) {
  wtf_size_t count = webgpu_enums.size();
  // TODO(enga): Pass in temporary memory or an allocator so we don't make a
  // separate memory allocation here.
  base::HeapArray<DawnEnum> dawn_enums =
      base::HeapArray<DawnEnum>::Uninit(count);
  for (wtf_size_t i = 0; i < count; ++i) {
    if (webgpu_enums[i].has_value()) {
      dawn_enums[i] = AsDawnEnum(webgpu_enums[i].value());
    } else {
      // Undefined is always 0
      dawn_enums[i] = static_cast<DawnEnum>(0);
    }
  }
  return dawn_enums;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_CONVERSIONS_H_
