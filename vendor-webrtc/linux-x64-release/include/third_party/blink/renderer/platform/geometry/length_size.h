/*
    Copyright (C) 1999 Lars Knoll (knoll@kde.org)
    Copyright (C) 2006, 2008 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_SIZE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_SIZE_H_

#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LengthSize {
  DISALLOW_NEW();

 public:
  LengthSize() = default;

  LengthSize(const Length& width, const Length& height)
      : width_(width), height_(height) {}

  bool operator==(const LengthSize& o) const {
    return width_ == o.width_ && height_ == o.height_;
  }
  bool operator!=(const LengthSize& o) const { return !(*this == o); }

  void SetWidth(const Length& width) { width_ = width; }
  const Length& Width() const { return width_; }

  void SetHeight(const Length& height) { height_ = height; }
  const Length& Height() const { return height_; }

 private:
  Length width_;
  Length height_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_SIZE_H_
