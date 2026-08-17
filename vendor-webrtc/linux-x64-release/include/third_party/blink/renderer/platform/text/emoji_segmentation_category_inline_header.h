// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_INLINE_HEADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_INLINE_HEADER_H_

#include "third_party/blink/renderer/platform/text/character.h"
#include "third_party/blink/renderer/platform/text/emoji_segmentation_category.h"

namespace blink {

namespace {

EmojiSegmentationCategory GetEmojiSegmentationCategory(UChar32 codepoint) {
  if (codepoint <= 0x7F) {
    if (Character::IsEmojiKeycapBase(codepoint)) {
      return EmojiSegmentationCategory::KEYCAP_BASE;
    }
    return EmojiSegmentationCategory::kMaxCategory;
  }

  // For the grammar to work, we need to check for more specific character
  // classes first, then expand towards more generic ones. So we match single
  // characters and small ranges first, then return EMOJI and
  // EMOJI_TEXT_PRESENTATION for the remaining ones.
  if (codepoint == kCombiningEnclosingKeycapCharacter) {
    return EmojiSegmentationCategory::COMBINING_ENCLOSING_KEYCAP;
  }
  if (codepoint == kCombiningEnclosingCircleBackslashCharacter) {
    return EmojiSegmentationCategory::COMBINING_ENCLOSING_CIRCLE_BACKSLASH;
  }
  if (codepoint == kZeroWidthJoinerCharacter) {
    return EmojiSegmentationCategory::ZWJ;
  }
  if (codepoint == kVariationSelector15Character) {
    return EmojiSegmentationCategory::VS15;
  }
  if (codepoint == kVariationSelector16Character) {
    return EmojiSegmentationCategory::VS16;
  }
  if (codepoint == 0x1F3F4) {
    return EmojiSegmentationCategory::TAG_BASE;
  }
  if (Character::IsEmojiTagSequence(codepoint)) {
    return EmojiSegmentationCategory::TAG_SEQUENCE;
  }
  if (codepoint == kCancelTag) {
    // http://www.unicode.org/reports/tr51/#def_emoji_tag_sequence
    // defines a TAG_TERM grammar rule for U+E007F CANCEL TAG.
    return EmojiSegmentationCategory::TAG_TERM;
  }
  if (Character::IsEmojiModifierBase(codepoint)) {
    return EmojiSegmentationCategory::EMOJI_MODIFIER_BASE;
  }
  if (Character::IsModifier(codepoint)) {
    return EmojiSegmentationCategory::EMOJI_MODIFIER;
  }
  if (Character::IsRegionalIndicator(codepoint)) {
    return EmojiSegmentationCategory::REGIONAL_INDICATOR;
  }

  if (Character::IsEmojiEmojiDefault(codepoint)) {
    return EmojiSegmentationCategory::EMOJI_EMOJI_PRESENTATION;
  }
  if (Character::IsEmojiTextDefault(codepoint)) {
    return EmojiSegmentationCategory::EMOJI_TEXT_PRESENTATION;
  }
  if (Character::IsEmoji(codepoint)) {
    return EmojiSegmentationCategory::EMOJI;
  }

  // Ragel state machine will interpret unknown category as "any".
  return EmojiSegmentationCategory::kMaxCategory;
}

}  // namespace

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_EMOJI_SEGMENTATION_CATEGORY_INLINE_HEADER_H_
