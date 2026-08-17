/*
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_INFO_H_

namespace blink {

enum AnimatedPropertyType {
  kAnimatedUnknown = 0,
  kAnimatedAngle,
  kAnimatedBoolean,
  kAnimatedColor,
  kAnimatedEnumeration,
  kAnimatedInteger,
  kAnimatedIntegerOptionalInteger,
  kAnimatedLength,
  kAnimatedLengthList,
  kAnimatedNumber,
  kAnimatedNumberList,
  kAnimatedNumberOptionalNumber,
  kAnimatedPath,
  kAnimatedPoint,
  kAnimatedPoints,
  kAnimatedPreserveAspectRatio,
  kAnimatedRect,
  kAnimatedString,
  kAnimatedStringList,
  kAnimatedTransform,
  kAnimatedTransformList,
  kNumberOfAnimatedPropertyTypes
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_PROPERTY_INFO_H_
