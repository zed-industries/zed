// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_EVENT_H_

namespace blink {

// Paint events that either PaintTiming or FirstMeaningfulPaintDetector receive
// SwapPromise swap times for.
enum class PaintEvent {
  kFirstPaint,
  kFirstContentfulPaint,
  kProvisionalFirstMeaningfulPaint,
  kFirstImagePaint
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_EVENT_H_
