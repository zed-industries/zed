// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_BIDI_PARAGRAPH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_BIDI_PARAGRAPH_H_

#include <unicode/ubidi.h>

#include <optional>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// BidiParagraph resolves bidirectional runs in a paragraph using ICU BiDi.
// http://userguide.icu-project.org/transforms/bidi
//
// Given a string of a paragraph, it runs Unicode Bidirectional Algorithm in
// UAX#9 and create logical runs.
// http://unicode.org/reports/tr9/
// It can also create visual runs once lines breaks are determined.
class PLATFORM_EXPORT BidiParagraph {
  STACK_ALLOCATED();

 public:
  BidiParagraph() = default;
  BidiParagraph(const WTF::String& text,
                std::optional<TextDirection> base_direction) {
    SetParagraph(text, base_direction);
  }

  // Splits the given paragraph to bidi runs and resolves the bidi embedding
  // level of each run.
  //
  // Returns false on failure. Nothing other than the destructor should be
  // called.
  bool SetParagraph(const WTF::String&,
                    std::optional<TextDirection> base_direction);

  // @return the entire text is unidirectional.
  bool IsUnidirectional() const {
    return ubidi_getDirection(ubidi_.get()) != UBIDI_MIXED;
  }

  // The base direction (a.k.a. paragraph direction) of this block.
  // This is determined by the 'direction' property of the block, or by the
  // heuristic rules defined in UAX#9 if 'unicode-bidi: plaintext'.
  TextDirection BaseDirection() const { return base_direction_; }

  // Compute the base direction for a given string using the heuristic
  // rules defined in UAX#9. It determines the direction by the first strong
  // character, or returns `nullopt` if no strong characters are found before
  // the first segment break.
  // http://unicode.org/reports/tr9/#The_Paragraph_Level
  static std::optional<TextDirection> BaseDirectionForString(
      const StringView&,
      bool (*stop_at)(UChar) = nullptr);

  // Same as `BaseDirectionForString().value_or(kLtr)`, with an optimized code
  // path for when the default (no strong characters) is LTR.
  static TextDirection BaseDirectionForStringOrLtr(
      const StringView& text,
      bool (*stop_at)(UChar) = nullptr);

  // Create a string that enforces directional override by wrapping the given
  // string with a Unicode BiDi override character (LRO or ROL) and PDF.
  // https://unicode.org/reports/tr9/#Explicit_Directional_Overrides
  // https://unicode.org/reports/tr9/#Terminating_Explicit_Directional_Embeddings_and_Overrides
  static WTF::String StringWithDirectionalOverride(const StringView& text,
                                                   TextDirection direction);

  struct Run {
    Run(unsigned start, unsigned end, UBiDiLevel level)
        : start(start), end(end), level(level) {
      DCHECK_GT(end, start);
    }

    unsigned Length() const { return end - start; }
    TextDirection Direction() const { return DirectionFromLevel(level); }

    bool operator==(const Run& other) const {
      return start == other.start && end == other.end && level == other.level;
    }

    unsigned start;
    unsigned end;
    UBiDiLevel level;
  };
  using Runs = Vector<Run, 32>;

  // Get a list of `Run` in the logical order (before bidi reorder.)
  // `text` must be the same one as `SetParagraph`.
  // This is higher-level API for `GetLogicalRun`.
  void GetLogicalRuns(const WTF::String& text, Runs* runs) const;

  // Returns the end offset of a logical run that starts from the |start|
  // offset.
  unsigned GetLogicalRun(unsigned start, UBiDiLevel*) const;

  // Get a list of `Run` in the visual order (after bidi reorder.)
  // `text` must be the same one as `SetParagraph`.
  // This is higher-level API for `GetLogicalRuns` and `IndicesInVisualOrder`.
  void GetVisualRuns(const WTF::String& text, Runs* runs) const;

  // Create a list of indices in the visual order.
  // A wrapper for ICU |ubidi_reorderVisual()|.
  static void IndicesInVisualOrder(
      const Vector<UBiDiLevel, 32>& levels,
      Vector<int32_t, 32>* indices_in_visual_order_out);

 private:
  template <typename TChar>
  static std::optional<TextDirection> BaseDirectionForString(
      base::span<const TChar>,
      bool (*stop_at)(UChar));

  struct UBiDiDeleter {
    void operator()(UBiDi* ubidi) const { ubidi_close(ubidi); }
  };
  using UBidiPtr = std::unique_ptr<UBiDi, UBiDiDeleter>;

  UBidiPtr ubidi_;
  TextDirection base_direction_ = TextDirection::kLtr;
};

// static
inline TextDirection BidiParagraph::BaseDirectionForStringOrLtr(
    const StringView& text,
    bool (*stop_at)(UChar)) {
  if (text.empty() || text.Is8Bit()) {
    // The result is LTR when 8 bits string and the distinction between LTR or
    // neutral is not needed, because U+0000-00FF are LTR or neutral.
    return TextDirection::kLtr;
  }
  return BaseDirectionForString(text, stop_at).value_or(TextDirection::kLtr);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_BIDI_PARAGRAPH_H_
