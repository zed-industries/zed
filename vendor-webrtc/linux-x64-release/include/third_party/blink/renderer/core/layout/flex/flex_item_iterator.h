// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_ITERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/flex/flex_line.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BlockBreakToken;

// A utility class for flexbox layout which given a list of flex lines and a
// break token will iterate through unfinished flex items.
//
// NextItem() is used to iterate through the flex items. This will be done in
// item order. If there are child break tokens, though, their nodes will be
// processed first, in break token order. When we're through those, we proceed
// to the next sibling item of the last break token - unless we have
// already seen and started all children (in which case the parent break token
// will be marked as such; |HasSeenAllChildren()| will return true).
//
// This class does not handle modifications to its arguments after it has been
// constructed.
class CORE_EXPORT FlexItemIterator {
  STACK_ALLOCATED();

 public:
  FlexItemIterator(const FlexLineVector& flex_lines,
                   const BlockBreakToken* break_token,
                   bool is_column);

  // Returns the next flex item which should be laid out, along with its
  // respective break token. |broke_before_row| will be true if the current
  // flex row broke before, represented by its first child's break token.
  // |broke_before_row| should always be false for column flex containers.
  struct Entry;
  Entry NextItem(bool broke_before_row);

  bool HasMoreBreakTokens() const { return break_token_; }

  // Move the iterator to the next line, unless we are already at the start of a
  // line.
  void NextLine();

 private:
  FlexItemData* FindNextItem(const BlockBreakToken* item_break_token = nullptr);
  void AdjustItemIndexForNewLine();

  FlexItemData* next_unstarted_item_ = nullptr;
  const FlexLineVector& flex_lines_;
  const BlockBreakToken* break_token_;
  bool is_column_ = false;

  // An index into break_token_'s ChildBreakTokens() vector. Used for keeping
  // track of the next child break token to inspect.
  wtf_size_t child_token_idx_ = 0;
  // An index into the flex_lines_ vector. Used for keeping track of the next
  // flex line to inspect.
  wtf_size_t flex_line_idx_ = 0;
  // An index into the flex_lines_'s line_items_ vector. Used for keeping track
  // of the next flex item to inspect.
  wtf_size_t flex_item_idx_ = 0;
  // Stores the next item index to process for each line, if applicable.
  Vector<wtf_size_t> next_item_idx_for_line_;
};

struct FlexItemIterator::Entry {
  STACK_ALLOCATED();

 public:
  Entry(FlexItemData* flex_item,
        wtf_size_t flex_item_idx,
        wtf_size_t flex_line_idx,
        const BlockBreakToken* token)
      : flex_item(flex_item),
        flex_item_idx(flex_item_idx),
        flex_line_idx(flex_line_idx),
        token(token) {}

  FlexItemData* flex_item;
  wtf_size_t flex_item_idx;
  wtf_size_t flex_line_idx;
  const BlockBreakToken* token;

  bool operator==(const FlexItemIterator::Entry& other) const {
    return flex_item == other.flex_item &&
           flex_item_idx == other.flex_item_idx &&
           flex_line_idx == other.flex_line_idx && token == other.token;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_ITERATOR_H_
