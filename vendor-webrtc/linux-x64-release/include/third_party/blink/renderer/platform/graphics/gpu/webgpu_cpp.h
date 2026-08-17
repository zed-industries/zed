// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CPP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CPP_H_

#include "ui/gl/buildflags.h"

// Include dawn/wire/client/webgpu_cpp.h if we have USE_DAWN to link directly to
// the wire. Otherwise, use the normal C++ header which will default to the proc
// table. The procs would be empty since use_dawn is false.

#if BUILDFLAG(USE_DAWN)
#include <dawn/wire/client/webgpu_cpp.h>  // IWYU pragma: export
#else
#include <dawn/webgpu_cpp.h>  // IWYU pragma: export
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CPP_H_
