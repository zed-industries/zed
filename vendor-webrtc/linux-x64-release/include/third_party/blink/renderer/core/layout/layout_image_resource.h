/*
 * Copyright (C) 1999 Lars Knoll <knoll@kde.org>
 * Copyright (C) 1999 Antti Koivisto <koivisto@kde.org>
 * Copyright (C) 2006 Allan Sandfeld Jensen <kde@carewolf.com>
 * Copyright (C) 2006 Samuel Weinig <sam.weinig@gmail.com>
 * Copyright (C) 2004, 2005, 2006, 2007, 2009, 2010 Apple Inc.
 *               All rights reserved.
 * Copyright (C) 2010 Patrick Gansterer <paroga@paroga.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_content.h"
#include "third_party/blink/renderer/core/style/style_image.h"

namespace blink {

class LayoutObject;
struct NaturalSizingInfo;

class CORE_EXPORT LayoutImageResource
    : public GarbageCollected<LayoutImageResource> {
 public:
  LayoutImageResource();
  LayoutImageResource(const LayoutImageResource&) = delete;
  LayoutImageResource& operator=(const LayoutImageResource&) = delete;
  virtual ~LayoutImageResource();
  virtual void Trace(Visitor* visitor) const;

  virtual void Initialize(LayoutObject*);
  virtual void Shutdown();

  void SetImageResource(ImageResourceContent*);
  ImageResourceContent* CachedImage() const { return cached_image_.Get(); }
  virtual bool HasImage() const { return cached_image_ != nullptr; }
  ResourcePriority ComputeResourcePriority() const;

  void ResetAnimation();
  bool MaybeAnimated() const;

  virtual scoped_refptr<Image> GetImage(const gfx::SizeF&) const;
  scoped_refptr<Image> GetImage(const gfx::Size&) const;
  virtual bool ErrorOccurred() const {
    return cached_image_ && cached_image_->ErrorOccurred();
  }

  // Replace the resource this object references with a reference to
  // the "broken image".
  void UseBrokenImage();

  virtual NaturalSizingInfo GetNaturalDimensions(float multiplier) const;
  virtual RespectImageOrientationEnum ImageOrientation() const;
  virtual WrappedImagePtr ImagePtr() const { return cached_image_.Get(); }

 protected:
  // Returns an image based on the passed device scale factor.
  static Image* BrokenImage(double device_pixel_ratio);
  double DevicePixelRatio() const;

  FRIEND_TEST_ALL_PREFIXES(LayoutImageResourceTest, BrokenImageHighRes);

  Member<LayoutObject> layout_object_;
  Member<ImageResourceContent> cached_image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_H_
