/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2006 Allan Sandfeld Jensen (kde@carewolf.com)
 *           (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2004, 2005, 2006, 2007, 2009, 2010, 2011 Apple Inc.
 *               All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_image_resource.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"
#include "third_party/blink/renderer/core/layout/natural_sizing_info.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"

namespace blink {

class HTMLAreaElement;
class HTMLMapElement;
class SVGImage;

// LayoutImage is used to display any image type.
//
// There is 2 types of images:
// * normal images, e.g. <image>, <picture>.
// * content images with "content: url(path/to/image.png)".
// We store the type inside |is_generated_content_|.
//
// The class is image type agnostic as it only manipulates decoded images.
// See LayoutImageResource that holds this image.
class CORE_EXPORT LayoutImage : public LayoutReplaced {
 public:
  LayoutImage(Element*);
  ~LayoutImage() override;
  void Trace(Visitor*) const override;

  static LayoutImage* CreateAnonymous(Document&);

  bool IsUnsizedImage() const;

  void SetImageResource(LayoutImageResource*);

  LayoutImageResource* ImageResource() {
    NOT_DESTROYED();
    return image_resource_.Get();
  }
  const LayoutImageResource* ImageResource() const {
    NOT_DESTROYED();
    return image_resource_.Get();
  }
  ImageResourceContent* CachedImage() const {
    NOT_DESTROYED();
    return image_resource_ ? image_resource_->CachedImage() : nullptr;
  }

  HTMLMapElement* ImageMap() const;
  void AreaElementFocusChanged(HTMLAreaElement*);

  void SetIsGeneratedContent(bool generated = true) {
    NOT_DESTROYED();
    is_generated_content_ = generated;
  }

  bool IsGeneratedContent() const {
    NOT_DESTROYED();
    return is_generated_content_;
  }

  inline void SetImageDevicePixelRatio(float factor) {
    NOT_DESTROYED();
    image_device_pixel_ratio_ = factor;
  }
  float ImageDevicePixelRatio() const {
    NOT_DESTROYED();
    return image_device_pixel_ratio_;
  }

  void NaturalSizeChanged() override {
    NOT_DESTROYED();
    // The replaced content transform depends on the intrinsic size (see:
    // FragmentPaintPropertyTreeBuilder::UpdateReplacedContentTransform).
    SetNeedsPaintPropertyUpdate();
    if (image_resource_)
      ImageChanged(image_resource_->ImagePtr(), CanDeferInvalidation::kNo);
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutImage";
  }

  class MutableForPainting : public LayoutObject::MutableForPainting {
   public:
    void UpdatePaintedRect(const PhysicalRect& paint_rect);

   private:
    friend class LayoutImage;
    explicit MutableForPainting(const LayoutImage& image)
        : LayoutObject::MutableForPainting(image) {}
  };
  MutableForPainting GetMutableForPainting() const {
    NOT_DESTROYED();
    return MutableForPainting(*this);
  }

 protected:
  SVGImage* EmbeddedSVGImage() const;
  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;

  void Paint(const PaintInfo&) const final;

  bool IsLayoutImage() const final {
    NOT_DESTROYED();
    return true;
  }

  void WillBeDestroyed() override;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  bool CanBeSelectionLeafInternal() const final {
    NOT_DESTROYED();
    return true;
  }

 private:
  bool IsImage() const override {
    NOT_DESTROYED();
    return true;
  }

  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const override;

  bool ForegroundIsKnownToBeOpaqueInRect(
      const PhysicalRect& local_rect,
      unsigned max_depth_to_test) const final;
  bool ComputeBackgroundIsKnownToBeObscured() const final;

  bool BackgroundShouldAlwaysBeClipped() const override {
    NOT_DESTROYED();
    return true;
  }

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) final;

  void InvalidatePaintAndMarkForLayoutIfNeeded(CanDeferInvalidation);
  bool UpdateNaturalSizeIfNeeded();
  bool NeedsLayoutOnNaturalSizeChange() const;

  // The natural dimensions for the image.
  PhysicalNaturalSizingInfo natural_dimensions_;

  // This member wraps the associated decoded image.
  //
  // This field is set using setImageResource above which can be called in
  // several ways:
  // * For normal images, from the network stack (ImageLoader) once we have
  // some image data.
  // * For generated content, the resource is loaded during style resolution
  // and thus is stored in ComputedStyle (see ContentData::image) that gets
  // propagated to the anonymous LayoutImage in LayoutObject::createObject.
  Member<LayoutImageResource> image_resource_;
  float image_device_pixel_ratio_ = 1.0f;
  bool did_increment_visually_non_empty_pixel_count_ = false;

  // This field stores whether this image is generated with 'content'.
  bool is_generated_content_ = false;

  friend class MutableForPainting;
  PhysicalRect last_paint_rect_;
};

template <>
struct DowncastTraits<LayoutImage> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutImage();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_H_
