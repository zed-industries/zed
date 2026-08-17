// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_NATIVE_TEST_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_NATIVE_TEST_SUPPORT_H_

#include <dawn/dawn_proc_table.h>
#include <webgpu/webgpu.h>

namespace blink {

const DawnProcTable& GetDawnNativeProcs();
WGPUInstance MakeNativeWGPUInstance();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_NATIVE_TEST_SUPPORT_H_
