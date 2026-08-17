// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CHILD_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CHILD_ITERATOR_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/table/table_layout_algorithm_types.h"

namespace blink {

class BlockBreakToken;

// A utility class for table layout which given the first child and a break
// token will iterate through unfinished children.
//
// NextChild() is used to iterate through the children. This will be done in
// child layout order (i.e. in this order: top captions, table header, table
// bodies, table footer, bottom captions). If there are child break tokens,
// though, their nodes will be processed first, in break token order.
class CORE_EXPORT TableChildIterator {
  STACK_ALLOCATED();

 public:
  TableChildIterator(const TableGroupedChildren&, const BlockBreakToken*);

  class Entry {
    STACK_ALLOCATED();

   public:
    Entry(BlockNode node,
          const BlockBreakToken* token,
          wtf_size_t section_index)
        : node(node), token(token), section_index(section_index) {}

    const BlockNode GetNode() const { return node; }
    const BlockBreakToken* GetBreakToken() const { return token; }
    wtf_size_t GetSectionIndex() const {
      DCHECK(!node.IsTableCaption());
      return section_index;
    }
    explicit operator bool() const { return !!node; }

   private:
    BlockNode node;
    const BlockBreakToken* token;
    wtf_size_t section_index;
  };

  // Returns the next node which should be laid out, along with its
  // respective break token.
  Entry NextChild();

 private:
  BlockNode CurrentChild() const;
  void AdvanceChild();

  const TableGroupedChildren* grouped_children_;
  const BlockBreakToken* break_token_;

  // The sections iterator is used to walk through the table sections in layout
  // order, i.e. table header, table bodies, table footer. If it is unset, it
  // means that we're processing top captions. If it's at end(), it means that
  // we should look for bottom captions.
  std::optional<TableGroupedChildrenIterator> section_iterator_;

  // An index into break_token_'s ChildBreakTokens() vector. Used for keeping
  // track of the next child break token to inspect.
  wtf_size_t child_token_idx_ = 0;

  // An index into the current table caption. We're walking through the captions
  // twice. First we look for top captions. Then we walk through the sections
  // iterator. Then we walk through the captions again, looking for bottom
  // captions.
  wtf_size_t caption_idx_ = 0;

  wtf_size_t section_idx_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CHILD_ITERATOR_H_
