// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_RESULT_H_

namespace blink {

// Used as the return type of some paint methods.
enum PaintResult {
  // The layer/object is fully painted. This includes cases that nothing needs
  // painting regardless of the paint rect.
  kFullyPainted,
  // Some part of the layer/object is out of the cull rect and may be not fully
  // painted. The results cannot be cached because they may change when cull
  // rect changes.
  kMayBeClippedByCullRect,

  kMaxPaintResult = kMayBeClippedByCullRect,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_RESULT_H_
