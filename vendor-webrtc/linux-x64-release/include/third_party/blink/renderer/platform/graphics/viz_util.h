// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIZ_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIZ_UTIL_H_

#include <stdint.h>

#include "components/viz/common/surfaces/frame_sink_bundle_id.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Generates a new FrameSinkBundleId which is unique within the calling process
// and suitable for use with the given frame sink client ID.
PLATFORM_EXPORT viz::FrameSinkBundleId GenerateFrameSinkBundleId(
    uint32_t client_id);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIZ_UTIL_H_
