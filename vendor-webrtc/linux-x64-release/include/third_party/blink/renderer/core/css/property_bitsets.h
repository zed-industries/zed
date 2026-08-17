// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_BITSETS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_BITSETS_H_

#include <bitset>
#include "third_party/blink/renderer/core/css/properties/css_bitset.h"

namespace blink {

// Properties whose presence signals that we may have to go through
// the logic of logical properties replacing other properties, if present.
// Equivalent to checking prop.IsInLogicalPropertyGroup() && prop.IsSurrogate(),
// but faster.
extern const CSSBitset kLogicalGroupProperties;

// For properties that are not behind runtime flags (which are nearly all,
// in practice), we can avoid resolving and looking them up to check the
// exposure; we can just check this bitmap instead (which fits neatly into
// two rather hot cache lines). This saves a little time in parsing.
extern const CSSBitset kKnownExposedProperties;

// Properties that have the IsSurrogate() bit set; i.e., they may need
// to go through SurrogateFor() to be resolved against direction and
// writing mode.
extern const CSSBitset kSurrogateProperties;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_BITSETS_H_
