/*
 * Copyright (C) 2006 Lars Knoll <lars@trolltech.com>
 * Copyright (C) 2007, 2011, 2012 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_BREAK_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_BREAK_ITERATOR_H_

#include <unicode/brkiter.h>

#include <memory>
#include <type_traits>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/character.h"
#include "third_party/blink/renderer/platform/text/character_break_iterator.h"
#include "third_party/blink/renderer/platform/text/layout_locale.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

struct PLATFORM_EXPORT ReturnBreakIteratorToPool {
  void operator()(void* ptr) const;
};

//
// LineBreakIterator is stocked in a pool to save the construction time.
// `PooledBreakIterator`, when destructed, returns the instance to the pool
// instead of deleting it.
//
using PooledBreakIterator =
    std::unique_ptr<TextBreakIterator, ReturnBreakIteratorToPool>;

//
// Returns a new instance from a pool, or create a new one if the pool is empty.
//
PLATFORM_EXPORT PooledBreakIterator
AcquireLineBreakIterator(StringView, const AtomicString& locale);

// Note: The returned iterator is good only until you get another iterator, with
// the exception of acquireLineBreakIterator.

// This is similar to character break iterator in most cases, but is subject to
// platform UI conventions. One notable example where this can be different
// from character break iterator is Thai prepend characters, see bug 24342.
// Use this for insertion point and selection manipulations.
PLATFORM_EXPORT TextBreakIterator* CursorMovementIterator(
    base::span<const UChar>);
PLATFORM_EXPORT TextBreakIterator* WordBreakIterator(const String&,
                                                     wtf_size_t start,
                                                     wtf_size_t length);
PLATFORM_EXPORT TextBreakIterator* WordBreakIterator(base::span<const UChar>);
PLATFORM_EXPORT TextBreakIterator* SentenceBreakIterator(
    base::span<const UChar>);

// Before calling this, check if the iterator is not at the end. Otherwise,
// it may not work as expected.
// See https://ssl.icu-project.org/trac/ticket/13447 .
PLATFORM_EXPORT bool IsWordTextBreak(TextBreakIterator*);

// A Unicode Line Break Word Identifier (key "lw".)
// https://www.unicode.org/reports/tr35/#UnicodeLineBreakWordIdentifier
enum class LineBreakType : uint8_t {
  kNormal,

  // word-break:break-all allows breaks between letters/numbers, but prohibits
  // break before/after certain punctuation.
  kBreakAll,

  // Allows breaks at every grapheme cluster boundary.
  // Terminal style line breaks described in UAX#14: Examples of Customization
  // http://unicode.org/reports/tr14/#Examples
  // CSS is discussing to add this feature crbug.com/720205
  // Used internally for word-break:break-word.
  kBreakCharacter,

  // word-break:keep-all doesn't allow breaks between all kind of
  // letters/numbers except some south east asians'.
  kKeepAll,

  // `lw=phrase`, which prioritize keeping natural phrases (of multiple words)
  // together when breaking.
  // https://www.unicode.org/reports/tr35/#UnicodeLineBreakWordIdentifier
  kPhrase,
};

// Determines break opportunities around collapsible space characters (space,
// newline, and tabulation characters.)
enum class BreakSpaceType : uint8_t {
  // Break after a run of white space characters.
  // This is the default mode, matching the ICU behavior.
  kAfterSpaceRun,

  // white-spaces:break-spaces allows breaking after any preserved white-space,
  // even when these are leading spaces so that we can avoid breaking
  // the word in case of overflow.
  kAfterEverySpace,
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, LineBreakType);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, BreakSpaceType);

class PLATFORM_EXPORT LazyLineBreakIterator final {
  STACK_ALLOCATED();

 public:
  explicit LazyLineBreakIterator(
      const String& string,
      const LayoutLocale* locale = nullptr,
      LineBreakType break_type = LineBreakType::kNormal)
      : string_(string),
        locale_(locale),
        break_type_(break_type) {
  }

  LazyLineBreakIterator(const String& string,
                        const AtomicString& locale,
                        LineBreakType break_type = LineBreakType::kNormal)
      : LazyLineBreakIterator(string, LayoutLocale::Get(locale), break_type) {}

  // Create an instance with the same settings as `other`, except `string`.
  LazyLineBreakIterator(const LazyLineBreakIterator& other, String string)
      : LazyLineBreakIterator(std::move(string),
                              other.Locale(),
                              other.BreakType()) {
    SetBreakSpace(other.BreakSpace());
    SetStrictness(other.Strictness());
  }

  const String& GetString() const { return string_; }

  void ResetStringAndReleaseIterator(String string,
                                     const LayoutLocale* locale) {
    string_ = string;
    start_offset_ = 0;
    SetLocale(locale);
    ReleaseIterator();
  }

  // Set the start offset. Text before this offset is disregarded. Properly
  // setting the start offset improves the performance significantly, because
  // ICU break iterator computes all the text from the beginning.
  unsigned StartOffset() const { return start_offset_; }
  void SetStartOffset(unsigned offset) {
    CHECK_LE(offset, string_.length());
    start_offset_ = offset;
    ReleaseIterator();
  }

  const LayoutLocale* Locale() const { return locale_; }
  void SetLocale(const LayoutLocale* locale) {
    if (locale == locale_) {
      return;
    }
    locale_ = locale;
    InvalidateLocaleWithKeyword();
  }

  LineBreakType BreakType() const { return break_type_; }
  void SetBreakType(LineBreakType break_type);
  BreakSpaceType BreakSpace() const { return break_space_; }
  void SetBreakSpace(BreakSpaceType break_space) { break_space_ = break_space; }
  LineBreakStrictness Strictness() const { return strictness_; }
  void SetStrictness(LineBreakStrictness strictness);

  // Enable/disable breaking at soft hyphens (U+00AD). Enabled by default.
  bool IsSoftHyphenEnabled() const { return !disable_soft_hyphen_; }
  void EnableSoftHyphen(bool value) { disable_soft_hyphen_ = !value; }

  inline bool IsBreakable(unsigned pos) const {
    // No need to scan the entire string for the next breakable position when
    // all we need to determine is whether the current position is breakable.
    // Limit length to pos + 1.
    // TODO(layout-dev): We should probably try to break out an actual
    // IsBreakable method from NextBreakablePosition and get rid of this hack.
    const unsigned len = std::min(pos + 1, string_.length());
    unsigned next_breakable = NextBreakablePosition(pos, len);
    return pos == next_breakable;
  }

  // Returns the break opportunity at or after |offset|.
  unsigned NextBreakOpportunity(unsigned offset) const;
  unsigned NextBreakOpportunity(unsigned offset, unsigned len) const;

  // Returns the break opportunity at or before |offset|.
  unsigned PreviousBreakOpportunity(unsigned offset, unsigned min = 0) const;

  static bool IsBreakableSpace(UChar ch) {
    return ch == kSpaceCharacter || ch == kTabulationCharacter ||
           ch == kNewlineCharacter;
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(TextBreakIteratorTest, Strictness);

  template <typename CharacterType>
  struct Context;

  const AtomicString& LocaleWithKeyword() const;
  void InvalidateLocaleWithKeyword();

  void ReleaseIterator() const {
    iterator_ = nullptr;
    character_iterator_.reset();
  }

  // Obtain text break iterator, possibly previously cached, where this iterator
  // is (or has been) initialized to use the previously stored string as the
  // primary breaking context and using previously stored prior context if
  // non-empty.
  TextBreakIterator* GetIterator() const {
    if (iterator_) {
      return iterator_.get();
    }

    // Create the iterator, or get one from the cache, for the text after
    // |start_offset_|. Because ICU TextBreakIterator computes all characters
    // from the beginning of the given text, using |start_offset_| improves the
    // performance significantly.
    //
    // For this reason, the offset for the TextBreakIterator must be adjusted by
    // |start_offset_|.
    CHECK_LE(start_offset_, string_.length());
    const AtomicString& locale = LocaleWithKeyword();
    iterator_ =
        AcquireLineBreakIterator(StringView{string_, start_offset_}, locale);
    return iterator_.get();
  }

  CharacterBreakIterator& GetCharacterBreakIterator() const {
    if (!character_iterator_) {
      character_iterator_.emplace(StringView(string_, start_offset_));
    }
    return *character_iterator_;
  }

  template <typename CharacterType, LineBreakType, BreakSpaceType>
  unsigned NextBreakablePosition(unsigned pos,
                                 const CharacterType* str,
                                 unsigned len) const;
  template <typename CharacterType, LineBreakType>
  unsigned NextBreakablePosition(unsigned pos,
                                 const CharacterType* str,
                                 unsigned len) const;
  template <LineBreakType>
  unsigned NextBreakablePosition(unsigned pos, unsigned len) const;
  unsigned NextBreakablePositionBreakCharacter(unsigned pos) const;
  unsigned NextBreakablePosition(unsigned pos, unsigned len) const;

  String string_;
  const LayoutLocale* locale_ = nullptr;
  mutable AtomicString locale_with_keyword_;
  mutable PooledBreakIterator iterator_;
  mutable std::optional<CharacterBreakIterator> character_iterator_;
  unsigned start_offset_ = 0;
  LineBreakType break_type_;
  BreakSpaceType break_space_ = BreakSpaceType::kAfterSpaceRun;
  LineBreakStrictness strictness_ = LineBreakStrictness::kDefault;
  bool disable_soft_hyphen_ = false;
};

inline const AtomicString& LazyLineBreakIterator::LocaleWithKeyword() const {
  if (!locale_with_keyword_) {
    if (!locale_) {
      locale_with_keyword_ = g_empty_atom;
    } else if (strictness_ == LineBreakStrictness::kDefault &&
               break_type_ != LineBreakType::kPhrase) {
      locale_with_keyword_ = locale_->LocaleString();
    } else {
      locale_with_keyword_ = locale_->LocaleWithBreakKeyword(
          strictness_, break_type_ == LineBreakType::kPhrase);
    }
    DCHECK(locale_with_keyword_);
  }
  return locale_with_keyword_;
}

inline void LazyLineBreakIterator::InvalidateLocaleWithKeyword() {
  if (locale_with_keyword_) {
    locale_with_keyword_ = AtomicString();
    ReleaseIterator();
  }
}

inline void LazyLineBreakIterator::SetBreakType(LineBreakType break_type) {
  if (break_type_ != break_type) {
    if (break_type_ == LineBreakType::kPhrase ||
        break_type == LineBreakType::kPhrase) {
      InvalidateLocaleWithKeyword();
    }
    break_type_ = break_type;
  }
}

inline void LazyLineBreakIterator::SetStrictness(
    LineBreakStrictness strictness) {
  if (strictness_ != strictness) {
    strictness_ = strictness;
    InvalidateLocaleWithKeyword();
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_BREAK_ITERATOR_H_
