// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/tokens/tokens.h"

namespace blink {

// An impression represents a click on an anchor tag that has special Conversion
// Measurement attributes declared. When the anchor is clicked, an impression is
// generated from these attributes and associated with the resulting navigation.
// When an action is performed on the linked site at a later date, the
// impression information is used to provide context about the initial
// navigation that resulted in that action.
struct BLINK_COMMON_EXPORT Impression {
  // Indicates the attributionsrc request associated with `this`.
  // Data parameters will be used from the attributionsrc response.
  AttributionSrcToken attribution_src_token;

  // Whether there were any impression string associated with the attributionsrc
  // tag. This is only used for internal metric collection purpose.
  bool is_empty_attribution_src_tag = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_H_
