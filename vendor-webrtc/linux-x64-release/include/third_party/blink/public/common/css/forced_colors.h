// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_FORCED_COLORS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_FORCED_COLORS_H_

namespace blink {

// Use for passing forced colors from the OS to the renderer.
enum class ForcedColors {
  kNone,
  kActive,
  kMaxValue = kActive,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_FORCED_COLORS_H_
