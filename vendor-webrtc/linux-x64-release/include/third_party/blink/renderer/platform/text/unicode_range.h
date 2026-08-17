/*
 * Copyright (C) 2007 Apple Computer, Inc.
 *
 * Portions are Copyright (C) 1998 Netscape Communications Corporation.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 *
 * Alternatively, the contents of this file may be used under the terms
 * of either the Mozilla Public License Version 1.1, found at
 * http://www.mozilla.org/MPL/ (the "MPL") or the GNU General Public
 * License Version 2.0, found at http://www.fsf.org/copyleft/gpl.html
 * (the "GPL"), in which case the provisions of the MPL or the GPL are
 * applicable instead of those above.  If you wish to allow use of your
 * version of this file only under the terms of one of those two
 * licenses (the MPL or the GPL) and not to allow others to use your
 * version of this file under the LGPL, indicate your decision by
 * deletingthe provisions above and replace them with the notice and
 * other provisions required by the MPL or the GPL, as the case may be.
 * If you do not delete the provisions above, a recipient may use your
 * version of this file under any of the LGPL, the MPL or the GPL.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_UNICODE_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_UNICODE_RANGE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

// The following constants define unicode subranges
// values below cRangeNum must be continuous so that we can map to
// a lang group directly.
// All ranges we care about should fit within 32 bits.

// Frequently used range definitions
const unsigned char kCRangeCyrillic = 0;
const unsigned char kCRangeGreek = 1;
const unsigned char kCRangeTurkish = 2;
const unsigned char kCRangeHebrew = 3;
const unsigned char kCRangeArabic = 4;
const unsigned char kCRangeBaltic = 5;
const unsigned char kCRangeThai = 6;
const unsigned char kCRangeKorean = 7;
const unsigned char kCRangeJapanese = 8;
const unsigned char kCRangeSChinese = 9;
const unsigned char kCRangeTChinese = 10;
const unsigned char kCRangeDevanagari = 11;
const unsigned char kCRangeTamil = 12;
const unsigned char kCRangeArmenian = 13;
const unsigned char kCRangeBengali = 14;
const unsigned char kCRangeCanadian = 15;
const unsigned char kCRangeEthiopic = 16;
const unsigned char kCRangeGeorgian = 17;
const unsigned char kCRangeGujarati = 18;
const unsigned char kCRangeGurmukhi = 19;
const unsigned char kCRangeKhmer = 20;
const unsigned char kCRangeMalayalam = 21;

const unsigned char kCRangeSpecificItemNum = 22;

// range/rangeSet grow to this place 22-29

const unsigned char kCRangeSetStart =
    30;  // range set definition starts from here
const unsigned char kCRangeSetLatin = 30;
const unsigned char kCRangeSetCJK = 31;
const unsigned char kCRangeSetEnd = 31;  // range set definition ends here

// less frequently used range definition
const unsigned char kCRangeSurrogate = 32;
const unsigned char kCRangePrivate = 33;
const unsigned char kCRangeMisc = 34;
const unsigned char kCRangeUnassigned = 35;
const unsigned char kCRangeSyriac = 36;
const unsigned char kCRangeThaana = 37;
const unsigned char kCRangeOriya = 38;
const unsigned char kCRangeTelugu = 39;
const unsigned char kCRangeKannada = 40;
const unsigned char kCRangeSinhala = 41;
const unsigned char kCRangeLao = 42;
const unsigned char kCRangeTibetan = 43;
const unsigned char kCRangeMyanmar = 44;
const unsigned char kCRangeCherokee = 45;
const unsigned char kCRangeOghamRunic = 46;
const unsigned char kCRangeMongolian = 47;
const unsigned char kCRangeMathOperators = 48;
const unsigned char kCRangeMiscTechnical = 49;
const unsigned char kCRangeControlOpticalEnclose = 50;
const unsigned char kCRangeBoxBlockGeometrics = 51;
const unsigned char kCRangeMiscSymbols = 52;
const unsigned char kCRangeDingbats = 53;
const unsigned char kCRangeBraillePattern = 54;
const unsigned char kCRangeYi = 55;
const unsigned char kCRangeCombiningDiacriticalMarks = 56;
const unsigned char kCRangeSpecials = 57;

const unsigned char kCRangeTableBase =
    128;  // values over 127 are reserved for internal use only
const unsigned char kCRangeTertiaryTable = 145;  // leave room for 16 subtable
                                                 // indices (cRangeTableBase + 1
                                                 // .. cRangeTableBase + 16)

PLATFORM_EXPORT unsigned FindCharUnicodeRange(UChar32);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_UNICODE_RANGE_H_
