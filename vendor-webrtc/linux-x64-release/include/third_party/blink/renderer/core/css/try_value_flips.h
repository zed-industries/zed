// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_VALUE_FLIPS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_VALUE_FLIPS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/style/position_try_fallbacks.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class CSSPropertyValueSet;
class CSSValue;
class WritingDirectionMode;
class TryTacticTransform;

// A single position-try-fallback can specify a number of "flips" called
// try-tactics. This makes it easy for authors to try mirrored versions
// of manually specified positions.
//
// This class is responsible for carrying out those flips, or rather
// generating CSSPropertyValueSets which carry out those flips
// using CSSFlipRevertValues.
//
// https://drafts.csswg.org/css-anchor-position-1/#propdef-position-try-fallbacks
class CORE_EXPORT TryValueFlips {
  DISALLOW_NEW();

 public:
  // Returns a CSSPropertyValueSet containing CSSFlipRevertValues
  // corresponding to the incoming TryTacticList. TryTacticLists which
  // represents the same transform (see TryTacticTransform) will return
  // the same CSSPropertyValueSet pointer.
  //
  // This will end up in OutOfFlowData::try_tactics_set_.
  const CSSPropertyValueSet* FlipSet(const TryTacticList&) const;

  // If the specified TryTacticTransform affects the CSSValue, returns
  // a rewritten value according to that transform. Otherwise, returns
  // the incoming CSSValue.
  static const CSSValue* FlipValue(CSSPropertyID from_property,
                                   const CSSValue*,
                                   const TryTacticTransform&,
                                   const WritingDirectionMode&);

  void Trace(Visitor*) const;

 private:
  const CSSPropertyValueSet* CreateFlipSet(const TryTacticTransform&) const;

  // There are only seven possible transforms, plus the initial state
  // (see TryTacticTransform). The CSSPropertyValueSets corresponding to
  // these transforms are independent of the underlying style being
  // transformed, which makes it trivial to cache the sets.
  static constexpr wtf_size_t kCachedFlipSetsSize = 7;
  mutable HeapVector<Member<const CSSPropertyValueSet>, kCachedFlipSetsSize>
      cached_flip_sets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_VALUE_FLIPS_H_
