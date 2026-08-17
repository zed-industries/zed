// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_PHASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_PHASE_H_

namespace blink {

// These are the hit-testable paint phases.
// See: core/paint/paint_phase.h for more information.
enum class HitTestPhase {
  kSelfBlockBackground,
  kDescendantBlockBackgrounds,
  kFloat,
  kForeground,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_PHASE_H_
