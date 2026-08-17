// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_RESULTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_RESULTS_H_

#include "third_party/blink/renderer/core/editing/finder/find_buffer.h"
#include "third_party/blink/renderer/core/editing/iterators/text_searcher_icu.h"

namespace blink {

// All match results for this buffer. We can iterate through the
// BufferMatchResults one by one using the Iterator.
class CORE_EXPORT FindResults {
  STACK_ALLOCATED();

 public:
  class CORE_EXPORT Iterator {
    STACK_ALLOCATED();

   public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = MatchResultICU;
    using difference_type = std::ptrdiff_t;
    using pointer = MatchResultICU*;
    using reference = MatchResultICU&;

    Iterator() = default;
    Iterator(const FindBuffer* find_buffer,
             TextSearcherICU* text_searcher,
             const Vector<std::unique_ptr<TextSearcherICU>>& extra_searchers);

    bool operator==(const Iterator& other) const {
      return IsAtEnd() == other.IsAtEnd();
    }

    bool operator!=(const Iterator& other) const {
      return IsAtEnd() != other.IsAtEnd();
    }

    const MatchResultICU operator*() const;

    void operator++();

   private:
    bool IsAtEnd() const;
    std::optional<MatchResultICU> EarliestMatch() const;

    const FindBuffer* find_buffer_ = nullptr;
    Vector<TextSearcherICU*, 1> text_searcher_list_;
    Vector<std::optional<MatchResultICU>, 1> match_list_;
  };

  FindResults();
  FindResults(const FindBuffer* find_buffer,
              TextSearcherICU* text_searcher,
              const Vector<UChar>& buffer,
              const Vector<Vector<UChar>>* extra_buffers,
              const String& search_text,
              const FindOptions options);

  Iterator begin() const;
  Iterator end() const;

  bool IsEmpty() const;

  MatchResultICU front() const;
  MatchResultICU back() const;

  unsigned CountForTesting() const;

 private:
  String search_text_;
  const FindBuffer* find_buffer_;
  TextSearcherICU* text_searcher_;
  Vector<std::unique_ptr<TextSearcherICU>> extra_searchers_;
  bool empty_result_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_RESULTS_H_
