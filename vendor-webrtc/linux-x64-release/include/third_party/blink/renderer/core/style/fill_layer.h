/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Graham Dennis (graham.dennis@gmail.com)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILL_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILL_LAYER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_size.h"
#include "third_party/blink/renderer/platform/graphics/blend_mode.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct FillSize {
  DISALLOW_NEW();
  FillSize() : type(EFillSizeType::kSizeLength) {}

  FillSize(EFillSizeType t, const LengthSize& l) : type(t), size(l) {}

  bool operator==(const FillSize& o) const {
    return type == o.type && size == o.size;
  }
  bool operator!=(const FillSize& o) const { return !(*this == o); }

  EFillSizeType type;
  LengthSize size;
};

struct FillRepeat {
  EFillRepeat x{EFillRepeat::kRepeatFill};
  EFillRepeat y{EFillRepeat::kRepeatFill};

  bool operator==(const FillRepeat& r) const { return x == r.x && y == r.y; }
  bool operator!=(const FillRepeat& r) const { return !(*this == r); }
};

class FillLayerWrapper;

class CORE_EXPORT FillLayer {
  DISALLOW_NEW();

 public:
  explicit FillLayer(EFillLayerType, bool use_initial_values = false);

  void Trace(Visitor* visitor) const;

  StyleImage* GetImage() const { return image_.Get(); }
  const Length& PositionX() const { return position_x_; }
  const Length& PositionY() const { return position_y_; }
  BackgroundEdgeOrigin BackgroundXOrigin() const {
    return static_cast<BackgroundEdgeOrigin>(background_x_origin_);
  }
  BackgroundEdgeOrigin BackgroundYOrigin() const {
    return static_cast<BackgroundEdgeOrigin>(background_y_origin_);
  }
  EFillAttachment Attachment() const {
    return static_cast<EFillAttachment>(attachment_);
  }
  EFillBox Clip() const { return static_cast<EFillBox>(clip_); }
  EFillBox Origin() const { return static_cast<EFillBox>(origin_); }
  const FillRepeat& Repeat() const { return repeat_; }
  EFillMaskMode MaskMode() const {
    return static_cast<EFillMaskMode>(mask_mode_);
  }
  enum CompositingOperator CompositingOperator() const {
    return static_cast<enum CompositingOperator>(compositing_operator_);
  }
  CompositeOperator Composite() const;
  BlendMode GetBlendMode() const { return static_cast<BlendMode>(blend_mode_); }
  const LengthSize& SizeLength() const { return size_length_; }
  EFillSizeType SizeType() const {
    return static_cast<EFillSizeType>(size_type_);
  }
  FillSize Size() const {
    return FillSize(static_cast<EFillSizeType>(size_type_), size_length_);
  }

  const FillLayer* Next() const;
  FillLayer* Next();
  FillLayer* EnsureNext();

  bool IsImageSet() const { return image_set_; }
  bool IsPositionXSet() const { return pos_x_set_; }
  bool IsPositionYSet() const { return pos_y_set_; }
  bool IsBackgroundXOriginSet() const { return background_x_origin_set_; }
  bool IsBackgroundYOriginSet() const { return background_y_origin_set_; }
  bool IsAttachmentSet() const { return attachment_set_; }
  bool IsClipSet() const { return clip_set_; }
  bool IsOriginSet() const { return origin_set_; }
  bool IsRepeatSet() const { return repeat_set_; }
  bool IsMaskModeSet() const { return mask_mode_set_; }
  bool IsCompositingOperatorSet() const { return compositing_operator_set_; }

  bool IsBlendModeSet() const { return blend_mode_set_; }
  bool IsSizeSet() const {
    return size_type_ != static_cast<unsigned>(EFillSizeType::kSizeNone);
  }

  void SetImage(StyleImage* i) {
    image_ = i;
    image_set_ = true;
  }
  void SetPositionX(const Length& position) {
    position_x_ = position;
    pos_x_set_ = true;
    background_x_origin_set_ = false;
    background_x_origin_ = static_cast<unsigned>(BackgroundEdgeOrigin::kLeft);
  }
  void SetPositionY(const Length& position) {
    position_y_ = position;
    pos_y_set_ = true;
    background_y_origin_set_ = false;
    background_y_origin_ = static_cast<unsigned>(BackgroundEdgeOrigin::kTop);
  }
  void SetBackgroundXOrigin(BackgroundEdgeOrigin origin) {
    background_x_origin_ = static_cast<unsigned>(origin);
    background_x_origin_set_ = true;
  }
  void SetBackgroundYOrigin(BackgroundEdgeOrigin origin) {
    background_y_origin_ = static_cast<unsigned>(origin);
    background_y_origin_set_ = true;
  }
  void SetAttachment(EFillAttachment attachment) {
    DCHECK(!cached_properties_computed_);
    attachment_ = static_cast<unsigned>(attachment);
    attachment_set_ = true;
  }
  void SetClip(EFillBox b) {
    DCHECK(!cached_properties_computed_);
    clip_ = static_cast<unsigned>(b);
    clip_set_ = true;
  }
  void SetOrigin(EFillBox b) {
    DCHECK(!cached_properties_computed_);
    origin_ = static_cast<unsigned>(b);
    origin_set_ = true;
  }
  void SetRepeat(const FillRepeat& r) {
    repeat_ = r;
    repeat_set_ = true;
  }
  void SetMaskMode(const EFillMaskMode& m) {
    mask_mode_ = static_cast<unsigned>(m);
    mask_mode_set_ = true;
  }
  void SetCompositingOperator(enum CompositingOperator c) {
    compositing_operator_ = static_cast<unsigned>(c);
    compositing_operator_set_ = true;
  }
  void SetBlendMode(BlendMode b) {
    blend_mode_ = static_cast<unsigned>(b);
    blend_mode_set_ = true;
  }
  void SetSizeType(EFillSizeType b) { size_type_ = static_cast<unsigned>(b); }
  void SetSizeLength(const LengthSize& length) { size_length_ = length; }
  void SetSize(const FillSize& f) {
    size_type_ = static_cast<unsigned>(f.type);
    size_length_ = f.size;
  }

  void ClearImage() {
    image_.Clear();
    image_set_ = false;
  }
  void ClearPositionX() {
    pos_x_set_ = false;
    background_x_origin_set_ = false;
  }
  void ClearPositionY() {
    pos_y_set_ = false;
    background_y_origin_set_ = false;
  }

  void ClearAttachment() { attachment_set_ = false; }
  void ClearClip() { clip_set_ = false; }
  void ClearOrigin() { origin_set_ = false; }
  void ClearRepeat() { repeat_set_ = false; }
  void ClearMaskMode() { mask_mode_set_ = false; }
  void ClearCompositingOperator() { compositing_operator_set_ = false; }
  void ClearBlendMode() { blend_mode_set_ = false; }
  void ClearSize() {
    size_type_ = static_cast<unsigned>(EFillSizeType::kSizeNone);
  }

  FillLayer& operator=(const FillLayer&);
  FillLayer(const FillLayer&);

  bool operator==(const FillLayer&) const;
  bool operator!=(const FillLayer& o) const { return !(*this == o); }

  bool VisuallyEqual(const FillLayer&) const;

  bool ImageOccludesNextLayers(const Document&, const ComputedStyle&) const;
  bool ClipOccludesNextLayers() const;

  EFillLayerType GetType() const { return static_cast<EFillLayerType>(type_); }

  void FillUnsetProperties();
  void CullEmptyLayers();

  static bool ImagesIdentical(const FillLayer*, const FillLayer*);

  EFillBox LayersClipMax() const {
    ComputeCachedPropertiesIfNeeded();
    return static_cast<EFillBox>(layers_clip_max_);
  }
  bool AnyLayerUsesContentBox() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_uses_content_box_;
  }
  bool AnyLayerHasImage() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_has_image_;
  }
  bool AnyLayerHasUrlImage() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_has_url_image_;
  }
  bool AnyLayerHasLocalAttachment() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_has_local_attachment_;
  }
  bool AnyLayerHasLocalAttachmentImage() const {
    ComputeCachedPropertiesIfNeeded();
    // Note that this can have false-positive in rare cases.
    return any_layer_has_local_attachment_ && any_layer_has_image_;
  }
  bool AnyLayerHasFixedAttachmentImage() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_has_fixed_attachment_image_;
  }
  bool AnyLayerHasDefaultAttachmentImage() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_has_default_attachment_image_;
  }
  bool AnyLayerUsesCurrentColor() const {
    ComputeCachedPropertiesIfNeeded();
    return any_layer_uses_current_color_;
  }

  static EFillAttachment InitialFillAttachment(EFillLayerType) {
    return EFillAttachment::kScroll;
  }
  static EFillBox InitialFillClip(EFillLayerType) { return EFillBox::kBorder; }
  static EFillBox InitialFillOrigin(EFillLayerType type) {
    return type == EFillLayerType::kBackground ? EFillBox::kPadding
                                               : EFillBox::kBorder;
  }
  static FillRepeat InitialFillRepeat(EFillLayerType) {
    return {EFillRepeat::kRepeatFill, EFillRepeat::kRepeatFill};
  }
  static EFillMaskMode InitialFillMaskMode(EFillLayerType) {
    return EFillMaskMode::kMatchSource;
  }
  static enum CompositingOperator InitialFillCompositingOperator(
      EFillLayerType) {
    return CompositingOperator::kAdd;
  }
  static BlendMode InitialFillBlendMode(EFillLayerType) {
    return BlendMode::kNormal;
  }
  static EFillSizeType InitialFillSizeType(EFillLayerType) {
    return EFillSizeType::kSizeLength;
  }
  static LengthSize InitialFillSizeLength(EFillLayerType) {
    return LengthSize();
  }
  static FillSize InitialFillSize(EFillLayerType type) {
    return FillSize(InitialFillSizeType(type), InitialFillSizeLength(type));
  }
  static Length InitialFillPositionX(EFillLayerType) {
    return Length::Percent(0.0);
  }
  static Length InitialFillPositionY(EFillLayerType) {
    return Length::Percent(0.0);
  }
  static StyleImage* InitialFillImage(EFillLayerType) { return nullptr; }

 private:
  friend class ComputedStyle;

  bool ImageIsOpaque(const Document&, const ComputedStyle&) const;
  bool ImageTilesLayer() const;
  bool LayerPropertiesEqual(const FillLayer&) const;

  EFillBox EffectiveClip() const;
  void ComputeCachedPropertiesIfNeeded() const {
    if (!cached_properties_computed_) {
      ComputeCachedProperties();
    }
  }
  void ComputeCachedProperties() const;

  Member<FillLayerWrapper> next_;
  Member<StyleImage> image_;

  Length position_x_;
  Length position_y_;

  LengthSize size_length_;
  FillRepeat repeat_;

  unsigned attachment_ : 2;            // EFillAttachment
  unsigned clip_ : 3;                  // EFillBox
  unsigned origin_ : 3;                // EFillBox
  unsigned compositing_operator_ : 4;  // CompositingOperator
  unsigned size_type_ : 2;             // EFillSizeType
  unsigned blend_mode_ : 5;            // BlendMode
  unsigned background_x_origin_ : 2;   // BackgroundEdgeOrigin
  unsigned background_y_origin_ : 2;   // BackgroundEdgeOrigin
  unsigned mask_mode_ : 2;             // EFillMaskMode
  unsigned image_set_ : 1;
  unsigned attachment_set_ : 1;
  unsigned clip_set_ : 1;
  unsigned origin_set_ : 1;
  unsigned repeat_set_ : 1;
  unsigned mask_mode_set_ : 1;
  unsigned pos_x_set_ : 1;
  unsigned pos_y_set_ : 1;
  unsigned background_x_origin_set_ : 1;
  unsigned background_y_origin_set_ : 1;
  unsigned compositing_operator_set_ : 1;
  unsigned blend_mode_set_ : 1;

  unsigned type_ : 1;  // EFillLayerType

  // EFillBox, maximum clip_ value from this to bottom layer
  mutable unsigned layers_clip_max_ : 3;
  // True if any of this or subsequent layers has content-box clip or origin.
  mutable unsigned any_layer_uses_content_box_ : 1;
  // True if any of this or subsequent layers has image.
  mutable unsigned any_layer_has_image_ : 1;
  // True if any of this of subsequent layers has a url() image.
  mutable unsigned any_layer_has_url_image_ : 1;
  // True if any of this or subsequent layers has local attachment.
  mutable unsigned any_layer_has_local_attachment_ : 1;
  // True if any of this or subsequent layers has fixed attachment image.
  mutable unsigned any_layer_has_fixed_attachment_image_ : 1;
  // True if any of this or subsequent layers has default attachment image.
  mutable unsigned any_layer_has_default_attachment_image_ : 1;
  // True if any of this or subsequent layers depends on the value of
  // currentColor.
  mutable unsigned any_layer_uses_current_color_ : 1;
  // Set once any of the above is accessed. The layers will be frozen
  // thereafter.
  mutable unsigned cached_properties_computed_ : 1;
};

class FillLayerWrapper : public GarbageCollected<FillLayerWrapper> {
 public:
  explicit FillLayerWrapper(EFillLayerType type) : layer(type) {}

  void Trace(Visitor* visitor) const { visitor->Trace(layer); }
  FillLayer layer;
};

inline const FillLayer* FillLayer::Next() const {
  return next_ ? &next_->layer : nullptr;
}
inline FillLayer* FillLayer::Next() {
  return next_ ? &next_->layer : nullptr;
}
inline FillLayer* FillLayer::EnsureNext() {
  if (!next_) {
    next_ = MakeGarbageCollected<FillLayerWrapper>(GetType());
  }
  return &next_->layer;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILL_LAYER_H_
