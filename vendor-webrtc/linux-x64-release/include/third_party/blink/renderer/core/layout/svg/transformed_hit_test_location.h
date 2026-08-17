// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORMED_HIT_TEST_LOCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORMED_HIT_TEST_LOCATION_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/hit_test_location.h"

namespace blink {

class AffineTransform;

// Helper class handling the application of a AffineTransform to a
// HitTestLocation - producing a new, transformed, HitTestLocation if needed.
//
// Encapsulates logic to avoid creating/copying the HitTestLocation for example
// if the AffineTransform is the identity.
class CORE_EXPORT TransformedHitTestLocation {
  DISALLOW_NEW();

 public:
  // The AffineTransform passed is expected to be the "forward"
  // transform. The inverse will computed and applied (as needed.)
  //
  // If the transform is singular, the bool operator will return
  // false, in which case the object cannot (must not) be used.
  TransformedHitTestLocation(const HitTestLocation&, const AffineTransform&);

  // The AffineTransform passed will be applied without computing the inverse.
  enum InverseTag { kDontComputeInverse };
  TransformedHitTestLocation(const HitTestLocation&,
                             const AffineTransform&,
                             InverseTag);

  const HitTestLocation* operator->() const {
    DCHECK(location_);
    return location_;
  }
  const HitTestLocation& operator*() const {
    DCHECK(location_);
    return *location_;
  }
  explicit operator bool() const { return location_; }

 private:
  std::optional<HitTestLocation> storage_;
  const HitTestLocation* location_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORMED_HIT_TEST_LOCATION_H_
