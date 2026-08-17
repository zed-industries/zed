/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_WITH_INDEX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_WITH_INDEX_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// For use when you want to get the index for a node repeatedly and
// only want to walk the child list to figure out the index once.
class NodeWithIndex {
  STACK_ALLOCATED();

 public:
  explicit NodeWithIndex(Node& node) : node_(&node), index_(-1) {}

  Node& GetNode() const { return *node_; }

  // TODO(tkent): Should be unsigned.
  int Index() const {
    if (!HasIndex())
      index_ = GetNode().NodeIndex();
    DCHECK(HasIndex());
    DCHECK_EQ(index_, static_cast<int>(GetNode().NodeIndex()));
    return index_;
  }

 private:
  bool HasIndex() const { return index_ >= 0; }

  Node* node_;
  mutable int index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_WITH_INDEX_H_
