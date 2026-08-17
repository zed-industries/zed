// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_INIT_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_INIT_UTIL_H_

#include "media/base/video_types.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class ExceptionState;
class VideoFrameInit;
class VideoFrameBufferInit;

// Parses VideoFrameInit including validation and computation of sizes. Stores
// exception in |exception_state| if validation fails.
struct ParsedVideoFrameInit {
  ParsedVideoFrameInit(const VideoFrameInit* init,
                       media::VideoPixelFormat format,
                       const gfx::Size& coded_size,
                       const gfx::Rect& default_visible_rect,
                       const gfx::Size& default_display_size,
                       ExceptionState& exception_state);

  gfx::Rect visible_rect;
  gfx::Size display_size;
};

// Parses display size for both VideoFrameInit and VideoFrameBufferInit. Returns
// an empty size with exception stored in |exception_state| if validation fails.
gfx::Size ParseAndValidateDisplaySize(const VideoFrameInit* init,
                                      ExceptionState& exception_state);
gfx::Size ParseAndValidateDisplaySize(const VideoFrameBufferInit* init,
                                      ExceptionState& exception_state);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_INIT_UTIL_H_
