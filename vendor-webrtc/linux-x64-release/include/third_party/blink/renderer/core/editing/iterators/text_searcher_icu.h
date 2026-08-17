// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_SEARCHER_ICU_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_SEARCHER_ICU_H_

#include <optional>

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

struct UStringSearch;

namespace blink {

struct CORE_EXPORT MatchResultICU {
  wtf_size_t start;
  wtf_size_t length;

  bool operator==(const MatchResultICU& other) const {
    return start == other.start && length == other.length;
  }

  bool operator!=(const MatchResultICU& other) const {
    return !operator==(other);
  }
};

class CORE_EXPORT TextSearcherICU {
 public:
  enum ConstructLocalTag { kConstructLocal };

  // Instantiate with the global UStringSearch instance.
  // We can't have multiple instances constructed by this.
  TextSearcherICU();
  // Instantiate with a local UStringSearch instance.
  explicit TextSearcherICU(ConstructLocalTag);
  TextSearcherICU(const TextSearcherICU&) = delete;
  TextSearcherICU& operator=(const TextSearcherICU&) = delete;
  ~TextSearcherICU();

  void SetPattern(const StringView& pattern, FindOptions options);
  void SetText(base::span<const UChar> text);
  void SetOffset(wtf_size_t);
  std::optional<MatchResultICU> NextMatchResult();

 private:
  void SetPattern(base::span<const UChar> pattern);
  void SetCaseSensitivity(bool case_sensitive);
  bool ShouldSkipCurrentMatch(const MatchResultICU&) const;
  std::optional<MatchResultICU> NextMatchResultInternal();
  bool IsCorrectKanaMatch(base::span<const UChar> text,
                          const MatchResultICU&) const;

  UStringSearch* searcher_ = nullptr;
  wtf_size_t text_length_ = 0;
  Vector<UChar> normalized_search_text_;
  FindOptions options_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_SEARCHER_ICU_H_
