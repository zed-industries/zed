/*
 * Copyright (C) Research In Motion Limited 2010-2011. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_CHARACTER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_CHARACTER_DATA_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {

struct SvgCharacterData {
  DISALLOW_NEW();
  SvgCharacterData();

  static float EmptyValue() { return std::numeric_limits<float>::quiet_NaN(); }
  static bool IsEmptyValue(float value) { return std::isnan(value); }

  bool HasX() const { return !IsEmptyValue(x); }
  bool HasY() const { return !IsEmptyValue(y); }
  bool HasDx() const { return !IsEmptyValue(dx); }
  bool HasDy() const { return !IsEmptyValue(dy); }
  bool HasRotate() const { return !IsEmptyValue(rotate); }

  float x;
  float y;
  float dx;
  float dy;
  float rotate;
  bool anchored_chunk = false;
};

inline SvgCharacterData::SvgCharacterData()
    : x(EmptyValue()),
      y(EmptyValue()),
      dx(EmptyValue()),
      dy(EmptyValue()),
      rotate(EmptyValue()) {}

std::ostream& operator<<(std::ostream& ostream, const SvgCharacterData& data);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_CHARACTER_DATA_H_
