/*
    Copyright (C) 2004, 2005, 2007 Nikolas Zimmermann <zimmermann@kde.org>
                  2004, 2005 Rob Buis <buis@kde.org>
    Copyright (C) Research In Motion Limited 2010. All rights reserved.

    Based on khtml code by:
    Copyright (C) 2000-2003 Lars Knoll (knoll@kde.org)
              (C) 2000 Antti Koivisto (koivisto@kde.org)
              (C) 2000-2003 Dirk Mueller (mueller@kde.org)
              (C) 2002-2003 Apple Computer, Inc.

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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_UNZOOMED_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_UNZOOMED_LENGTH_H_

#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// A wrapper around a Length that has not been subjected to zoom adjustment when
// computed. Used for stroke-width.
class UnzoomedLength {
  DISALLOW_NEW();

 public:
  explicit UnzoomedLength(const Length& length) : length_(length) {}

  bool IsZero() const { return length_.IsZero(); }

  bool operator==(const UnzoomedLength& other) const {
    return length_ == other.length_;
  }
  bool operator!=(const UnzoomedLength& other) const {
    return !operator==(other);
  }

  const Length& length() const { return length_; }

 private:
  Length length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_UNZOOMED_LENGTH_H_
