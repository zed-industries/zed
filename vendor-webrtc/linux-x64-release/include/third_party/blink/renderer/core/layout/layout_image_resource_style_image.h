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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_STYLE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_STYLE_IMAGE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/layout/layout_image_resource.h"
#include "third_party/blink/renderer/core/style/style_image.h"

namespace blink {

class LayoutObject;

class LayoutImageResourceStyleImage final : public LayoutImageResource {
 public:
  explicit LayoutImageResourceStyleImage(StyleImage*);
  ~LayoutImageResourceStyleImage() override;

  void Initialize(LayoutObject*) override;
  void Shutdown() override;

  bool HasImage() const override { return true; }
  scoped_refptr<Image> GetImage(const gfx::SizeF&) const override;
  bool ErrorOccurred() const override { return style_image_->ErrorOccurred(); }

  NaturalSizingInfo GetNaturalDimensions(float multiplier) const override;
  RespectImageOrientationEnum ImageOrientation() const override;
  WrappedImagePtr ImagePtr() const override { return style_image_->Data(); }

  void Trace(Visitor*) const override;

 private:
  Member<StyleImage> style_image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_IMAGE_RESOURCE_STYLE_IMAGE_H_
