/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008, 2013 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NINE_PIECE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NINE_PIECE_IMAGE_H_

#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/border_image_length_box.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/length_box.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

enum ENinePieceImageRule {
  kStretchImageRule,
  kRoundImageRule,
  kSpaceImageRule,
  kRepeatImageRule
};

class CORE_EXPORT NinePieceImageData
    : public GarbageCollected<NinePieceImageData> {
 public:
  NinePieceImageData() = default;
  NinePieceImageData(const NinePieceImageData&) = default;

  bool operator==(const NinePieceImageData&) const;
  bool operator!=(const NinePieceImageData& o) const { return !(*this == o); }

  void Trace(Visitor* visitor) const { visitor->Trace(image); }

  unsigned single_owner : 1 = true;  // Managed by the owning NinePieceImage.
  unsigned fill : 1 = false;
  unsigned horizontal_rule : 2 = kStretchImageRule;  // ENinePieceImageRule
  unsigned vertical_rule : 2 = kStretchImageRule;    // ENinePieceImageRule
  Member<StyleImage> image;
  LengthBox image_slices{Length::Percent(100), Length::Percent(100),
                         Length::Percent(100), Length::Percent(100)};
  BorderImageLengthBox border_slices{1.0, 1.0, 1.0, 1.0};
  BorderImageLengthBox outset{0, 0, 0, 0};
};

class CORE_EXPORT NinePieceImage {
  DISALLOW_NEW();

 public:
  NinePieceImage();
  NinePieceImage(StyleImage*,
                 LengthBox image_slices,
                 bool fill,
                 const BorderImageLengthBox& border_slices,
                 const BorderImageLengthBox& outset,
                 ENinePieceImageRule horizontal_rule,
                 ENinePieceImageRule vertical_rule);

  NinePieceImage(const NinePieceImage& other) : data_(other.data_) {
    data_->single_owner = false;
  }
  NinePieceImage(NinePieceImage&&) = default;

  NinePieceImage& operator=(const NinePieceImage& other) {
    data_ = other.data_;
    data_->single_owner = false;
    return *this;
  }
  NinePieceImage& operator=(NinePieceImage&&) = default;

  static NinePieceImage MaskDefaults();

  bool operator==(const NinePieceImage& other) const {
    return base::ValuesEquivalent(data_, other.data_);
  }
  bool operator!=(const NinePieceImage& other) const {
    return !(*this == other);
  }

  bool HasImage() const { return data_->image; }
  StyleImage* GetImage() const { return data_->image.Get(); }
  void SetImage(StyleImage* image) { Access()->image = image; }

  const LengthBox& ImageSlices() const { return data_->image_slices; }
  void SetImageSlices(const LengthBox& slices) {
    Access()->image_slices = slices;
  }

  bool Fill() const { return data_->fill; }
  void SetFill(bool fill) { Access()->fill = fill; }

  const BorderImageLengthBox& BorderSlices() const {
    return data_->border_slices;
  }
  void SetBorderSlices(const BorderImageLengthBox& slices) {
    Access()->border_slices = slices;
  }

  const BorderImageLengthBox& Outset() const { return data_->outset; }
  void SetOutset(const BorderImageLengthBox& outset) {
    Access()->outset = outset;
  }

  ENinePieceImageRule HorizontalRule() const {
    return static_cast<ENinePieceImageRule>(data_->horizontal_rule);
  }
  void SetHorizontalRule(ENinePieceImageRule rule) {
    Access()->horizontal_rule = rule;
  }

  ENinePieceImageRule VerticalRule() const {
    return static_cast<ENinePieceImageRule>(data_->vertical_rule);
  }
  void SetVerticalRule(ENinePieceImageRule rule) {
    Access()->vertical_rule = rule;
  }

  void CopyImageSlicesFrom(const NinePieceImage& other) {
    Access()->image_slices = other.data_->image_slices;
    Access()->fill = other.data_->fill;
  }

  void CopyBorderSlicesFrom(const NinePieceImage& other) {
    Access()->border_slices = other.data_->border_slices;
  }

  void CopyOutsetFrom(const NinePieceImage& other) {
    Access()->outset = other.data_->outset;
  }

  void CopyRepeatFrom(const NinePieceImage& other) {
    Access()->horizontal_rule = other.data_->horizontal_rule;
    Access()->vertical_rule = other.data_->vertical_rule;
  }

  static LayoutUnit ComputeOutset(const BorderImageLength& outset_side,
                                  int border_side) {
    if (outset_side.IsNumber()) {
      return LayoutUnit(outset_side.Number() * border_side);
    }
    DCHECK(outset_side.length().IsFixed());
    return LayoutUnit(outset_side.length().Pixels());
  }

  void Trace(Visitor* visitor) const { visitor->Trace(data_); }

 private:
  // Used by MaskDefaults().
  explicit NinePieceImage(NinePieceImageData* data) : data_(data) {
    DCHECK(!data_->single_owner);
  }

  NinePieceImageData* Access() {
    if (!data_->single_owner) {
      data_ = MakeGarbageCollected<NinePieceImageData>(*data_);
      data_->single_owner = true;
    }
    return data_.Get();
  }

  Member<NinePieceImageData> data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NINE_PIECE_IMAGE_H_
