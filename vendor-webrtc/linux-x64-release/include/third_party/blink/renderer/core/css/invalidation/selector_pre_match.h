// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_SELECTOR_PRE_MATCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_SELECTOR_PRE_MATCH_H_

namespace blink {

// Describes the result of a pre-match operation in which the components of a
// selector are scanned to determine whether it can ever match an element.
enum class SelectorPreMatch { kNeverMatches, kMayMatch };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_SELECTOR_PRE_MATCH_H_
