// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MAP_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MAP_MODE_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_map_mode.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class GPUMapMode : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GPUMapMode(const GPUMapMode&) = delete;
  GPUMapMode& operator=(const GPUMapMode&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_MAP_MODE_H_
