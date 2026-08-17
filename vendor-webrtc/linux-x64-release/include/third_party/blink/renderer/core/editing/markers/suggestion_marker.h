// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/styleable_marker.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class SuggestionMarkerProperties;

// A subclass of StyleableMarker used to store information specific to
// suggestion markers (used to represent Android SuggestionSpans or ChromeOS's
// custom text spans). In addition
// to the formatting information StyleableMarker holds, we also store a list of
// suggested replacements for the marked region of text. In addition, each
// SuggestionMarker is tagged with an integer so browser code can identify which
// SuggestionMarker a suggestion replace operation pertains to.
class CORE_EXPORT SuggestionMarker final : public StyleableMarker {
 public:
  enum class SuggestionType {
    kMisspelling,
    kNotMisspelling,
    kAutocorrect,
    kGrammar,
  };
  enum class RemoveOnFinishComposing { kRemove, kDoNotRemove };

  SuggestionMarker(unsigned start_offset,
                   unsigned end_offset,
                   const SuggestionMarkerProperties&);
  SuggestionMarker(const SuggestionMarker&) = delete;
  SuggestionMarker& operator=(const SuggestionMarker&) = delete;

  // DocumentMarker implementations
  MarkerType GetType() const final;

  // SuggestionMarker-specific
  int32_t Tag() const;
  SuggestionType GetSuggestionType() const;
  const Vector<String>& Suggestions() const;
  bool IsMisspelling() const;
  bool NeedsRemovalOnFinishComposing() const;
  Color SuggestionHighlightColor() const;

  // Replace the suggestion at suggestion_index with new_suggestion.
  void SetSuggestion(unsigned suggestion_index, const String& new_suggestion);

 private:
  static int32_t NextTag();

  static int32_t current_tag_;

  // We use a signed int for the tag since it's passed to Java (as an opaque
  // identifier), and Java does not support unsigned ints.
  const int32_t tag_;
  Vector<String> suggestions_;
  const SuggestionType suggestion_type_;
  const RemoveOnFinishComposing remove_on_finish_composing_;
  const Color suggestion_highlight_color_;
};

template <>
struct DowncastTraits<SuggestionMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return marker.GetType() == DocumentMarker::kSuggestion;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_H_
