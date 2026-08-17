// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CAPITALIZE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CAPITALIZE_H_

#include <unicode/utypes.h>
#include "third_party/blink/renderer/platform/platform_export.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// Capitalize (titlecase) each word of a string.
// https://drafts.csswg.org/css-text-3/#valdef-text-transform-capitalize
PLATFORM_EXPORT WTF::String Capitalize(const WTF::String&,
                                       UChar previous_character = ' ');

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CAPITALIZE_H_
