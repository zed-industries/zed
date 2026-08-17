// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_LAYOUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_LAYOUT_H_

#include <stdint.h>

#include "media/base/video_frame.h"
#include "media/base/video_frame_layout.h"
#include "media/base/video_types.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ExceptionState;
class PlaneLayout;

class VideoFrameLayout {
 public:
  VideoFrameLayout();

  VideoFrameLayout(media::VideoPixelFormat format,
                   const gfx::Size& coded_size,
                   ExceptionState& exception_state);

  VideoFrameLayout(media::VideoPixelFormat format,
                   const gfx::Size& coded_size,
                   const HeapVector<Member<PlaneLayout>>& layout,
                   ExceptionState& exception_state);

  // End offset of the last (by memory address) plane.
  uint32_t Size() const;

  media::VideoPixelFormat Format() const;
  wtf_size_t NumPlanes() const;
  uint32_t Offset(wtf_size_t i) const;
  uint32_t Stride(wtf_size_t i) const;
  media::VideoFrameLayout ToMediaLayout();

 private:
  struct Plane {
    uint32_t offset = 0;
    uint32_t stride = 0;
  };

  media::VideoPixelFormat format_;
  gfx::Size coded_size_;
  Vector<Plane> planes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_LAYOUT_H_
