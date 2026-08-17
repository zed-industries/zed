/*
    Copyright (C) 1999 Lars Knoll (knoll@kde.org)
    Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
    Copyright (C) 2011 Rik Cabanier (cabanier@adobe.com)
    Copyright (C) 2011 Adobe Systems Incorporated. All rights reserved.
    Copyright (C) 2012 Motorola Mobility, Inc. All rights reserved.

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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_FUNCTIONS_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace gfx {
class PointF;
class SizeF;
}

namespace blink {

class Length;
class LengthSize;

struct LengthPoint;

PLATFORM_EXPORT int IntValueForLength(const Length&, int maximum_value);
PLATFORM_EXPORT float FloatValueForLength(const Length&,
                                          float maximum_value,
                                          const EvaluationInput& = {});
PLATFORM_EXPORT LayoutUnit
MinimumValueForLengthInternal(const Length&,
                              LayoutUnit maximum_value,
                              const EvaluationInput&);

inline LayoutUnit MinimumValueForLength(const Length& length,
                                        LayoutUnit maximum_value,
                                        const EvaluationInput& input = {}) {
  if (length.IsFixed()) [[likely]] {
    return LayoutUnit(length.Pixels());
  }

  return MinimumValueForLengthInternal(length, maximum_value, input);
}

PLATFORM_EXPORT LayoutUnit ValueForLength(const Length&,
                                          LayoutUnit maximum_value,
                                          const EvaluationInput& input = {});
PLATFORM_EXPORT gfx::SizeF SizeForLengthSize(const LengthSize&,
                                             const gfx::SizeF& box_size);
PLATFORM_EXPORT gfx::PointF PointForLengthPoint(const LengthPoint&,
                                                const gfx::SizeF& box_size);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_FUNCTIONS_H_
