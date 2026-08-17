// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_INFO_H_

#include <optional>

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GPUMemoryHeapInfo;

class GPUAdapterInfo : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GPUAdapterInfo(const String& vendor,
                 const String& architecture,
                 uint32_t subgroup_min_size,
                 uint32_t subgroup_max_size,
                 bool isFallbackAdapter,
                 const String& device = String(),
                 const String& description = String(),
                 const String& driver = String(),
                 const String& backend = String(),
                 const String& type = String(),
                 const std::optional<uint32_t> d3d_shader_model = std::nullopt,
                 const std::optional<uint32_t> vk_driver_version = std::nullopt,
                 const String& power_preference = String());

  GPUAdapterInfo(const GPUAdapterInfo&) = delete;
  GPUAdapterInfo& operator=(const GPUAdapterInfo&) = delete;

  void AppendMemoryHeapInfo(GPUMemoryHeapInfo*);

  // gpu_adapter_info.idl {{{
  const String& vendor() const;
  const String& architecture() const;
  const String& device() const;
  const String& description() const;
  uint32_t subgroupMinSize() const;
  uint32_t subgroupMaxSize() const;
  bool isFallbackAdapter() const;
  const String& driver() const;
  const String& backend() const;
  const String& type() const;
  const HeapVector<Member<GPUMemoryHeapInfo>>& memoryHeaps() const;
  const std::optional<uint32_t>& d3dShaderModel() const;
  const std::optional<uint32_t>& vkDriverVersion() const;
  const String& powerPreference() const;
  // }}} End of WebIDL binding implementation.

  void Trace(Visitor*) const override;

 private:
  String vendor_;
  String architecture_;
  uint32_t subgroup_min_size_;
  uint32_t subgroup_max_size_;
  bool is_fallback_adapter_;
  String device_;
  String description_;
  String driver_;
  String backend_;
  String type_;
  HeapVector<Member<GPUMemoryHeapInfo>> memory_heaps_;
  std::optional<uint32_t> d3d_shader_model_;
  std::optional<uint32_t> vk_driver_version_;
  String power_preference_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ADAPTER_INFO_H_
