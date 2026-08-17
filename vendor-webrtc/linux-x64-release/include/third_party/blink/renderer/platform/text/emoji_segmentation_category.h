// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_H_

#include <cstdint>

namespace blink {

// Must match the categories defined in
// `third-party/emoji-segmenter/src/emoji_presentation_scanner.rl`.
enum class EmojiSegmentationCategory : uint8_t {
  EMOJI = 0,
  EMOJI_TEXT_PRESENTATION = 1,
  EMOJI_EMOJI_PRESENTATION = 2,
  EMOJI_MODIFIER_BASE = 3,
  EMOJI_MODIFIER = 4,
  EMOJI_VS_BASE = 5,
  REGIONAL_INDICATOR = 6,
  KEYCAP_BASE = 7,
  COMBINING_ENCLOSING_KEYCAP = 8,
  COMBINING_ENCLOSING_CIRCLE_BACKSLASH = 9,
  ZWJ = 10,
  VS15 = 11,
  VS16 = 12,
  TAG_BASE = 13,
  TAG_SEQUENCE = 14,
  TAG_TERM = 15,

  kMaxCategory = 16,
  kInvalidCacheEntry = 17
};

// These operators are needed for the generated code at
// `third_party/emoji-segmenter`.
inline bool operator<(EmojiSegmentationCategory a, uint8_t b) {
  return static_cast<uint8_t>(a) < b;
}
inline bool operator>(EmojiSegmentationCategory a, uint8_t b) {
  return static_cast<uint8_t>(a) > b;
}
inline bool operator<=(EmojiSegmentationCategory a, uint8_t b) {
  return static_cast<uint8_t>(a) <= b;
}
inline bool operator<=(uint8_t a, EmojiSegmentationCategory b) {
  return a <= static_cast<uint8_t>(b);
}
inline uint8_t operator-(EmojiSegmentationCategory a, uint8_t b) {
  return static_cast<uint8_t>(a) - b;
}

// If this function returns true for categories of any characters in a string,
// it may produce Emoji presentation, and thus cannot skip being run through the
// Ragel emoji presentation scanner in `SymbolsIterator`. See also
// `Character::ParticipatesInEmojiGrammar`.
inline bool IsEmojiPresentationCategory(EmojiSegmentationCategory emoji) {
  return emoji != EmojiSegmentationCategory::kMaxCategory &&
         emoji != EmojiSegmentationCategory::KEYCAP_BASE &&
         emoji != EmojiSegmentationCategory::EMOJI_TEXT_PRESENTATION &&
         emoji != EmojiSegmentationCategory::VS15;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_H_
