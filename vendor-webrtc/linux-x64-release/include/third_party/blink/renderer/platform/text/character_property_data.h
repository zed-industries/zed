// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_PROPERTY_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_PROPERTY_DATA_H_

#include <unicode/uobject.h>

namespace blink {

static const UChar32 kIsCJKIdeographOrSymbolArray[] = {
    // 0x2C7 Caron, Mandarin Chinese 3rd Tone
    0x2C7,
    // 0x2CA Modifier Letter Acute Accent, Mandarin Chinese 2nd Tone
    0x2CA,
    // 0x2CB Modifier Letter Grave Access, Mandarin Chinese 4th Tone
    0x2CB,
    // 0x2D9 Dot Above, Mandarin Chinese 5th Tone
    0x2D9, 0x2020, 0x2021, 0x2030, 0x203B, 0x203C, 0x2042, 0x2047, 0x2048,
    0x2049, 0x2051, 0x20DD, 0x20DE, 0x2100, 0x2103, 0x2105, 0x2109, 0x210A,
    0x2113, 0x2116, 0x2121, 0x212B, 0x213B, 0x2150, 0x2151, 0x2152, 0x217F,
    0x2189, 0x2307, 0x23F0, 0x23F3, 0x2312, 0x23CE, 0x2423, 0x25A0, 0x25A1,
    0x25A2, 0x25AA, 0x25AB, 0x25B1, 0x25B2, 0x25B3, 0x25B6, 0x25B7, 0x25BC,
    0x25BD, 0x25C0, 0x25C1, 0x25C6, 0x25C7, 0x25C9, 0x25CB, 0x25CC, 0x25EF,
    0x2605, 0x2606, 0x260E, 0x2616, 0x2617, 0x261D, 0x2640, 0x2642, 0x267F,
    0x2693, 0x26A0, 0x26A1, 0x26BD, 0x26BE, 0x26CE, 0x26D4,
    // AIRPLANE added for PILOT emoji sequences.
    0x26EA, 0x26F5, 0x26F9, 0x26FA, 0x26FD, 0x2705, 0x2708, 0x2713, 0x271A,
    0x2728, 0x273F, 0x2740, 0x274C, 0x274E, 0x27B0, 0x27BF, 0x2B1A, 0x2B1B,
    0x2B1C, 0x2B50, 0x2B55, 0xFE10, 0xFE11, 0xFE12, 0xFE19, 0xFF1D,
    // Emoji.
    0x1F100, 0x1F004, 0x1F0CF, 0x1F18E};

static const UChar32 kIsCJKIdeographOrSymbolRanges[] = {
    // STAFF OF AESCULAPIUS..SCALES for emoji sequences for doctor and judge
    // professions.
    0x2695, 0x2696,
    // cjkIdeographRanges
    // CJK Radicals Supplement and Kangxi Radicals.
    0x2E80, 0x2FDF,
    // CJK Strokes.
    0x31C0, 0x31EF,
    // CJK Unified Ideographs Extension A.
    0x3400, 0x4DBF,
    // The basic CJK Unified Ideographs block.
    0x4E00, 0x9FFF,
    // CJK Compatibility Ideographs.
    0xF900, 0xFAFF,
    // Unicode Plane 2: Supplementary Ideographic Plane. This plane includes:
    // CJK Unified Ideographs Extension B to F.
    // CJK Compatibility Ideographs Supplement.
    0x20000, 0x2FFFF,

    // cjkSymbolRanges
    0x2156, 0x215A, 0x2160, 0x216B, 0x2170, 0x217B, 0x231A, 0x231B, 0x23E9,
    0x23EC, 0x23BE, 0x23CC, 0x2460, 0x2492, 0x249C, 0x24FF, 0x25CE, 0x25D3,
    0x25E2, 0x25E6, 0x25FD, 0x25FE, 0x2600, 0x2603, 0x2660, 0x266F,
    // Emoji HEAVY HEART EXCLAMATION MARK ORNAMENT..HEAVY BLACK HEART
    // Needed in order not to break Emoji heart-kiss sequences in
    // CachingWordShapeIterator.
    // cmp. http://www.unicode.org/emoji/charts/emoji-zwj-sequences.html
    0x2614, 0x2615, 0x2648, 0x2653, 0x26AA, 0x26AB, 0x26C4, 0x26C5, 0x26F2,
    0x26F3, 0x2753, 0x2757, 0x2763, 0x2764, 0x2672, 0x267D, 0x2776, 0x277F,
    0x2795, 0x2797,
    // Hand signs needed in order
    // not to break Emoji modifier sequences.
    0x270A, 0x270D,
    // Ideographic Description Characters, with CJK Symbols and Punctuation,
    // excluding 0x3030.
    // Exclude Hangul Tone Marks (0x302E .. 0x302F) because Hangul is not Han
    // and no other Hangul are included.
    // Then Hiragana 0x3040 .. 0x309F, Katakana 0x30A0 .. 0x30FF, Bopomofo
    // 0x3100 .. 0x312F
    0x2FF0, 0x302D, 0x3031, 0x312F,
    // More Bopomofo and Bopomofo Extended 0x31A0 .. 0x31BF
    0x3190, 0x31BF,
    // Enclosed CJK Letters and Months (0x3200 .. 0x32FF).
    // CJK Compatibility (0x3300 .. 0x33FF).
    0x3200, 0x33FF,
    // Yijing Hexagram Symbols
    0x4DC0, 0x4DFF,
    // http://www.unicode.org/Public/MAPPINGS/VENDORS/APPLE/JAPANESE.TXT
    0xF860, 0xF862,
    // CJK Compatibility Forms.
    // Small Form Variants (for CNS 11643).
    0xFE30, 0xFE6F,
    // Halfwidth and Fullwidth Forms
    // Usually only used in CJK
    0xFF00, 0xFF0C, 0xFF0E, 0xFF1A, 0xFF1F, 0xFFEF,
    // Ideographic Symbols and Punctuation
    0x16FE0, 0x16FFF,
    // Tangut
    0x17000, 0x187FF,
    // Tangut Components
    0x18800, 0x18AFF,
    // Kana Supplement
    0x1B000, 0x1B0FF,
    // Kana Extended-A
    0x1B100, 0x1B12F,
    // Nushu
    0x1B170, 0x1B2FF,
    // Emoji.
    0x1F110, 0x1F129, 0x1F130, 0x1F149, 0x1F150, 0x1F169, 0x1F170, 0x1F189,
    0x1F191, 0x1F19A, 0x1F1E6, 0x1F1FF, 0x1F200, 0x1F6FF,
    // Modifiers
    0x1F3FB, 0x1F3FF,

    // Transport
    0x1F6DC, 0x1F6DF,

    // Colored circles and squares for use with emoji.
    0x1F7E0, 0x1F7EB,

    // Math
    0x1F7F0, 0x1F7F0,

    0x1F900, 0x1F90F,
    // ZIPPER-MOUTH FACE...SIGN OF THE HORNS
    0x1F910, 0x1F918, 0x1F919, 0x1F97F, 0x1F980, 0x1F9BF, 0x1F9C0, 0x1F9FF,
    // Clothing, heart and Medical symbols
    0x1FA70, 0x1FA7C,
    // Toys and sport symbols
    0x1FA80, 0x1FA88,
    // Miscellaneous objects
    // Animals and nature
    0x1FA90, 0x1FABD,
    // Animal
    // Body parts
    // People
    0x1FABF, 0x1FAC5,
    // animal-mammal
    0x1FACE, 0x1FACF,
    // Food and drink
    0x1FAD0, 0x1FADB,
    // Face
    0x1FAE0, 0x1FAE8,
    // Hand
    0x1FAF0, 0x1FAF8,
    };

// https://html.spec.whatwg.org/C/#prod-potentialcustomelementname
static const UChar32 kIsPotentialCustomElementNameCharArray[] = {
    '-', '.', '_', 0xB7,
};

static const UChar32 kIsPotentialCustomElementNameCharRanges[] = {
    '0',    '9',    'a',    'z',    0xC0,    0xD6,    0xD8,   0xF6,
    0xF8,   0x2FF,  0x300,  0x37D,  0x37F,   0x1FFF,  0x200C, 0x200D,
    0x203F, 0x2040, 0x2070, 0x218F, 0x2C00,  0x2FEF,  0x3001, 0xD7FF,
    0xF900, 0xFDCF, 0xFDF0, 0xFFFD, 0x10000, 0xEFFFF,
};

// http://unicode.org/reports/tr9/#Directional_Formatting_Characters
static const UChar32 kIsBidiControlArray[] = {0x061C, 0x200E, 0x200F};

static const UChar32 kIsBidiControlRanges[] = {
    0x202A, 0x202E, 0x2066, 0x2069,
};

// https://unicode.org/Public/UNIDATA/Blocks.txt
static const UChar32 kIsHangulRanges[] = {
    // Hangul Jamo
    0x1100, 0x11FF,
    // Hangul Compatibility Jamo
    0x3130, 0x318F,
    // Hangul Jamo Extended-A
    0xA960, 0xA97F,
    // Hangul Syllables
    0xAC00, 0xD7AF,
    // Hangul Jamo Extended-B
    0xD7B0, 0xD7FF,
    // Halfwidth Hangul Jamo
    // https://www.unicode.org/charts/nameslist/c_FF00.html
    0xFFA0, 0xFFDC,
};

// Freezed trie tree, see character_property_data_generator.cc.
extern const int32_t kSerializedCharacterDataSize;
extern const uint8_t kSerializedCharacterData[];

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_PROPERTY_DATA_H_
