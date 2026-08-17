// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TRUNCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TRUNCATION_H_

#include <limits>

namespace blink {

// The two truncation values below are used as tokens representing truncation
// state for a text fragment (in LayoutNG) or text box (in legacy layout), are
// intended to be relative to |m_start|. They are set directly into
// |m_truncation|. In the case where there is some truncation of the text but it
// is not full, |m_truncation| is set to the character offset from |m_start|
//  representing the characters that are not truncated.
//
// Thus the maximum possible length of the text displayed before an ellipsis in
// a single NGTextFragment or InlineTextBox is
// |std::numeric_limits<uint16_t>::max() - 2| to allow for the no-truncation and
// full-truncation states.
constexpr uint16_t kCNoTruncation = std::numeric_limits<uint16_t>::max();
constexpr uint16_t kCFullTruncation = std::numeric_limits<uint16_t>::max() - 1;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TRUNCATION_H_
