// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PROGRAMMABLE_STAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PROGRAMMABLE_STAGE_H_

#include <memory>
#include <optional>
#include <string>

#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"

namespace blink {

class GPUProgrammableStage;

struct OwnedProgrammableStage {
  OwnedProgrammableStage() = default;

  //  This struct should be non-copyable non-movable because it contains
  //  self-referencing pointers that would be invalidated when moved / copied.
  OwnedProgrammableStage(const OwnedProgrammableStage& desc) = delete;
  OwnedProgrammableStage(OwnedProgrammableStage&& desc) = delete;
  OwnedProgrammableStage& operator=(const OwnedProgrammableStage& desc) =
      delete;
  OwnedProgrammableStage& operator=(OwnedProgrammableStage&& desc) = delete;

  std::optional<std::string> entry_point;
  std::unique_ptr<std::string[]> constantKeys;
  std::unique_ptr<wgpu::ConstantEntry[]> constants;
  uint32_t constantCount = 0;
};

void GPUProgrammableStageAsWGPUProgrammableStage(
    const GPUProgrammableStage* descriptor,
    OwnedProgrammableStage* dawn_programmable_stage);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PROGRAMMABLE_STAGE_H_
