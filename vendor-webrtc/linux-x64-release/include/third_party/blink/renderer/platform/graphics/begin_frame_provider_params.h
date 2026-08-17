// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_PARAMS_H_

#include "components/viz/common/surfaces/frame_sink_id.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

struct PLATFORM_EXPORT BeginFrameProviderParams final {
  viz::FrameSinkId parent_frame_sink_id;
  viz::FrameSinkId frame_sink_id;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_PARAMS_H_
