// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_ASPECT_RATIO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_ASPECT_RATIO_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

enum class EAspectRatioType { kAuto, kAutoAndRatio, kRatio };

class CORE_EXPORT StyleAspectRatio {
  DISALLOW_NEW();

 public:
  // Style data for aspect-ratio: auto || <ratio>
  StyleAspectRatio(EAspectRatioType type, gfx::SizeF ratio);

  // 0/x and x/0 are valid (and computed style needs to serialize them
  // as such), but they are not useful for layout, so we map it to auto here.
  EAspectRatioType GetType() const {
    if (layout_ratio_.IsEmpty()) {
      return EAspectRatioType::kAuto;
    }
    return GetTypeForComputedStyle();
  }

  EAspectRatioType GetTypeForComputedStyle() const {
    return static_cast<EAspectRatioType>(type_);
  }

  bool IsAuto() const { return GetType() == EAspectRatioType::kAuto; }

  // We have two representations of the aspect-ratio, one for style, and one
  // for layout.
  // Layout uses a fixed-precision representation of the aspect-ratio to reduce
  // any floating point errors that may occur. However the conversion to this
  // representation can be lossy, so we still have the float representation for
  // computed style.
  gfx::SizeF GetRatio() const { return ratio_; }
  PhysicalSize GetLayoutRatio() const { return layout_ratio_; }

  bool operator==(const StyleAspectRatio& o) const {
    return type_ == o.type_ && ratio_ == o.ratio_;
  }

  bool operator!=(const StyleAspectRatio& o) const { return !(*this == o); }

 private:
  unsigned type_ : 2;  // EAspectRatioType
  gfx::SizeF ratio_;
  PhysicalSize layout_ratio_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_ASPECT_RATIO_H_
