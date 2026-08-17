/*
    Copyright (C) 1999 Lars Knoll (knoll@kde.org)
    Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
    Copyright (c) 2012, Google Inc. All rights reserved.

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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_BOX_H_

#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PLATFORM_EXPORT LengthBox {
  DISALLOW_NEW();

 public:
  LengthBox() = default;

  LengthBox(Length::Type t) : left_(t), right_(t), top_(t), bottom_(t) {}

  LengthBox(int v)
      : left_(Length::Fixed(v)),
        right_(Length::Fixed(v)),
        top_(Length::Fixed(v)),
        bottom_(Length::Fixed(v)) {}

  LengthBox(const Length& t, const Length& r, const Length& b, const Length& l)
      : left_(l), right_(r), top_(t), bottom_(b) {}

  LengthBox(int t, int r, int b, int l)
      : left_(Length::Fixed(l)),
        right_(Length::Fixed(r)),
        top_(Length::Fixed(t)),
        bottom_(Length::Fixed(b)) {}

  const Length& Left() const { return left_; }
  const Length& Right() const { return right_; }
  const Length& Top() const { return top_; }
  const Length& Bottom() const { return bottom_; }

  bool operator==(const LengthBox& o) const {
    return left_ == o.left_ && right_ == o.right_ && top_ == o.top_ &&
           bottom_ == o.bottom_;
  }

  bool operator!=(const LengthBox& o) const { return !(*this == o); }

  bool NonZero() const {
    return !(left_.IsZero() && right_.IsZero() && top_.IsZero() &&
             bottom_.IsZero());
  }

  // Must be public for SET_VAR in ComputedStyle.h
  Length left_;
  Length right_;
  Length top_;
  Length bottom_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_BOX_H_
