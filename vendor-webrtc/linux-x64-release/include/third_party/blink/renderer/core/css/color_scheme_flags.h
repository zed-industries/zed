// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_SCHEME_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_SCHEME_FLAGS_H_

namespace blink {

// A set of flags extracted from a computed list of color-schemes. Contains one
// flag for each known color-scheme, and a flag for the 'only' keyword.
enum class ColorSchemeFlag : uint8_t {
  kNormal = 0,
  kDark = 1,
  kLight = 2,
  kOnly = 4,
};

// Bitset for ColorSchemeFlag
using ColorSchemeFlags = uint8_t;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_SCHEME_FLAGS_H_
