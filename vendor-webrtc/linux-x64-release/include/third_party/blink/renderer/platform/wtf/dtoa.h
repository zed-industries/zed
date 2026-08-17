/*
 *  Copyright (C) 2003, 2008, 2012 Apple Inc. All rights reserved.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DTOA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DTOA_H_

#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// Size = 80 for sizeof(DtoaBuffer) + some sign bits, decimal point, 'e',
// exponent digits.
const unsigned kNumberToStringBufferLength = 96;
typedef char NumberToStringBuffer[kNumberToStringBufferLength];

WTF_EXPORT const char* NumberToString(double, NumberToStringBuffer);
WTF_EXPORT const char* NumberToFixedPrecisionString(
    double,
    unsigned significant_figures,
    NumberToStringBuffer);
WTF_EXPORT const char* NumberToFixedWidthString(double,
                                                unsigned decimal_places,
                                                NumberToStringBuffer);

WTF_EXPORT double ParseDouble(const LChar* string,
                              size_t length,
                              size_t& parsed_length);
WTF_EXPORT double ParseDouble(const UChar* string,
                              size_t length,
                              size_t& parsed_length);

namespace internal {

void InitializeDoubleConverter();

}  // namespace internal

}  // namespace WTF

using WTF::NumberToFixedPrecisionString;
using WTF::NumberToFixedWidthString;
using WTF::NumberToString;
using WTF::NumberToStringBuffer;
using WTF::ParseDouble;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DTOA_H_
