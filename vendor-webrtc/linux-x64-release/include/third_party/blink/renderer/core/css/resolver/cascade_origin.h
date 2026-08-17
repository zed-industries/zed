// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_ORIGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_ORIGIN_H_

#include <cstdint>

namespace blink {

// Represents the origin criteria described by css-cascade [1].
//
// [1] https://www.w3.org/TR/css-cascade-3/#cascade-origin
enum class CascadeOrigin : uint8_t {
  kNone = 0,
  kUserAgent = 0b0001,
  kUser = 0b0010,
  // https://drafts.csswg.org/css-cascade-5/#preshint
  kAuthorPresentationalHint = 0b0011,
  kAuthor = 0b0100,
  kAnimation = 0b0101,
  // The lower four bits of kAuthor, kUser and kUserAgent can be inverted to
  // efficiently produce a "cascade correct" value when compared with the values
  // specified in this enum:
  //
  // kAuthor important:    ~0b0100 == 0b1011 (> kAnimation)
  // kUser important:      ~0b0010 == 0b1101 (> kAuthor important)
  // kUserAgent important: ~0b0001 == 0b1110 (> kUser important)
  //
  // Because kTransition has a higher priority than anything else, it's set to
  // 0b10000, which is greater than kUserAgent important. Although 0b1111 is
  // available, we avoid using that such that the fourth bit can be used as
  // as quick is-important check.
  kTransition = 0b10000,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_ORIGIN_H_
