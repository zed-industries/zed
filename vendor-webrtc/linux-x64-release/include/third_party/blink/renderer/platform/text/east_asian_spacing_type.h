// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EAST_ASIAN_SPACING_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EAST_ASIAN_SPACING_TYPE_H_

#include <stdint.h>

namespace blink {

// Represents the East Asian Spacing property, as defined in
// https://unicode.org/reports/tr59/.
enum class EastAsianSpacingType : uint16_t {
  kNarrow,
  kOther,
  kConditional,
  kWide,
  // When adding values, ensure `CharacterProperty` has enough storage.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EAST_ASIAN_SPACING_TYPE_H_
