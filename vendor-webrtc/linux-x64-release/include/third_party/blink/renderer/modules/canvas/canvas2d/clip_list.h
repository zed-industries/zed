// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CLIP_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CLIP_LIST_H_

#include <stddef.h>

#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkPath.h"

// https://github.com/include-what-you-use/include-what-you-use/issues/1546
// IWYU pragma: no_forward_declare WTF::internal::__thisIsHereToForceASemicolonAfterThisMacro

class SkMatrix;

namespace cc {
class PaintCanvas;
}

namespace blink {

class ClipList {
  DISALLOW_NEW();

 public:
  ClipList() = default;
  ClipList(const ClipList&);
  ~ClipList() = default;

  void ClipPath(const SkPath&, AntiAliasingMode, const SkMatrix&);
  void Playback(cc::PaintCanvas*) const;
  SkPath IntersectPathWithClip(const SkPath& path) const;

 private:
  struct ClipOp {
    SkPath path_;
    AntiAliasingMode anti_aliasing_mode_;

    ClipOp();
    ClipOp(const ClipOp&);
    ClipOp& operator=(const ClipOp&);
  };

  // Number of clip ops that can be stored in a ClipList without resorting to
  // dynamic allocation
  static const size_t kCInlineClipOpCapacity = 4;

  WTF::Vector<ClipOp, kCInlineClipOpCapacity> clip_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CLIP_LIST_H_
