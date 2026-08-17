// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_STYLUS_WRITING_GESTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_STYLUS_WRITING_GESTURE_H_

#include "third_party/blink/public/mojom/input/handwriting_gesture_result.mojom-blink.h"
#include "third_party/blink/public/mojom/input/stylus_writing_gesture.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// Class that receives the stylus writing gesture data and performs the
// corresponding gesture action.
class StylusWritingGesture {
 public:
  virtual ~StylusWritingGesture() = default;

  static mojom::blink::HandwritingGestureResult ApplyGesture(
      LocalFrame*,
      mojom::blink::StylusWritingGestureDataPtr);

 protected:
  StylusWritingGesture(const gfx::Rect& start_rect,
                       const String& text_alternative);

  // Apply this gesture in the current focused input element from the set
  // parameters and return true if the gesture coordinates are over a valid text
  // input position and the gesture was applied. Return false otherwise so that
  // the text alternative can be inserted at the current cursor.
  virtual bool MaybeApplyGesture(LocalFrame*) = 0;

  // Get the text index of the start_point_ in the current frame's focused
  // input.
  wtf_size_t GetStartTextIndex(LocalFrame*);

  // Start rectangle of the gesture.
  gfx::Rect start_rect_;
  // Alternate text to be inserted in case the gesture could not be applied.
  String text_alternative_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_STYLUS_WRITING_GESTURE_H_
