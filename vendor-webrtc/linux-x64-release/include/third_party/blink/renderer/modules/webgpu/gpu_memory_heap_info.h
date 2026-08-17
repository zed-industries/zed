// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MEMORY_HEAP_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MEMORY_HEAP_INFO_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"

namespace blink {

class GPUMemoryHeapInfo : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit GPUMemoryHeapInfo(const wgpu::MemoryHeapInfo&);

  GPUMemoryHeapInfo(const GPUMemoryHeapInfo&) = delete;
  GPUMemoryHeapInfo& operator=(const GPUMemoryHeapInfo&) = delete;

  // gpu_memory_heap_info.idl {{{
  uint64_t size() const;
  uint32_t properties() const;
  // }}} End of WebIDL binding implementation.

 private:
  wgpu::MemoryHeapInfo info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MEMORY_HEAP_INFO_H_
