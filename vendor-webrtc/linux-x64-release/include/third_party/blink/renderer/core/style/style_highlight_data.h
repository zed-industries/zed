// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_HIGHLIGHT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_HIGHLIGHT_DATA_H_

#include <optional>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class ComputedStyle;
using CustomHighlightsStyleMap =
    HeapHashMap<AtomicString, Member<const ComputedStyle>>;

class CORE_EXPORT StyleHighlightData final {
  DISALLOW_NEW();

 public:
  bool operator==(const StyleHighlightData&) const;

  const ComputedStyle* Style(
      PseudoId,
      const AtomicString& pseudo_argument = g_null_atom) const;
  const ComputedStyle* Selection() const;
  const ComputedStyle* SearchTextCurrent() const;
  const ComputedStyle* SearchTextNotCurrent() const;
  const ComputedStyle* TargetText() const;
  const ComputedStyle* SpellingError() const;
  const ComputedStyle* GrammarError() const;
  const ComputedStyle* CustomHighlight(const AtomicString&) const;
  const CustomHighlightsStyleMap& CustomHighlights() const {
    return custom_highlights_;
  }
  void SetSelection(const ComputedStyle*);
  void SetSearchTextCurrent(const ComputedStyle*);
  void SetSearchTextNotCurrent(const ComputedStyle*);
  void SetTargetText(const ComputedStyle*);
  void SetSpellingError(const ComputedStyle*);
  void SetGrammarError(const ComputedStyle*);
  void SetCustomHighlight(const AtomicString&, const ComputedStyle*);

  bool DependsOnSizeContainerQueries() const;

  void Trace(Visitor*) const;

 private:
  Member<const ComputedStyle> selection_;
  Member<const ComputedStyle> search_text_current_;
  Member<const ComputedStyle> search_text_not_current_;
  Member<const ComputedStyle> target_text_;
  Member<const ComputedStyle> spelling_error_;
  Member<const ComputedStyle> grammar_error_;
  CustomHighlightsStyleMap custom_highlights_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_HIGHLIGHT_DATA_H_
