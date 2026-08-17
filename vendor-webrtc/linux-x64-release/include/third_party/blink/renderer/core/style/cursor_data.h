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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_DATA_H_

#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "ui/gfx/geometry/point.h"

namespace blink {

class CursorData {
  DISALLOW_NEW();

 public:
  CursorData(StyleImage* image,
             bool hot_spot_specified,
             const gfx::Point& hot_spot)
      : image_(image),
        hot_spot_specified_(hot_spot_specified),
        hot_spot_(hot_spot) {}

  bool operator==(const CursorData& o) const {
    return hot_spot_ == o.hot_spot_ && base::ValuesEquivalent(image_, o.image_);
  }

  bool operator!=(const CursorData& o) const { return !(*this == o); }

  StyleImage* GetImage() const { return image_.Get(); }
  void SetImage(StyleImage* image) { image_ = image; }

  bool HotSpotSpecified() const { return hot_spot_specified_; }

  // Hot spot in the image in logical pixels.
  const gfx::Point& HotSpot() const { return hot_spot_; }

  void Trace(Visitor* visitor) const { visitor->Trace(image_); }

 private:
  Member<StyleImage> image_;
  bool hot_spot_specified_;
  gfx::Point hot_spot_;  // for CSS3 support
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::CursorData)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_DATA_H_
