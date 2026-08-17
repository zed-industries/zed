/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RESOLUTION_UNITS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RESOLUTION_UNITS_H_

#include "third_party/blink/renderer/platform/resolution_units.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// These conversions are defined in css-values
const double kCssPixelsPerCentimeter = kCssPixelsPerInch / kCentimetersPerInch;
const double kCssPixelsPerMillimeter =
    kCssPixelsPerCentimeter / kMillimetersPerCentimeter;
const double kCssPixelsPerQuarterMillimeter =
    kCssPixelsPerCentimeter / kQuarterMillimetersPerCentimeter;
const double kCssPixelsPerPica = kCssPixelsPerInch / kPicasPerInch;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RESOLUTION_UNITS_H_
