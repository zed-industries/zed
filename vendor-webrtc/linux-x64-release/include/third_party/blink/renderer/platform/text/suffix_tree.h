/*
 * Copyright (C) 2010 Adam Barth. All rights reserved.
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
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SUFFIX_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SUFFIX_TREE_H_

#include <algorithm>
#include <utility>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class UnicodeCodebook {
  STATIC_ONLY(UnicodeCodebook);

 public:
  static int CodeWord(UChar c) { return c; }
  enum { kCodeSize = 1 << 8 * sizeof(UChar) };
};

class ASCIICodebook {
  STATIC_ONLY(ASCIICodebook);

 public:
  static int CodeWord(UChar c) { return c & (kCodeSize - 1); }
  enum { kCodeSize = 1 << (8 * sizeof(char) - 1) };
};

template <typename Codebook>
class SuffixTree {
  USING_FAST_MALLOC(SuffixTree);

 public:
  SuffixTree(const String& text, unsigned depth) : leaf_(true), depth_(depth) {
    Build(text);
  }
  SuffixTree(const SuffixTree&) = delete;
  SuffixTree& operator=(const SuffixTree&) = delete;

  bool MightContain(const String& query) {
    Node* current = &root_;
    int limit = std::min(depth_, query.length());
    for (int i = 0; i < limit; ++i) {
      auto iter = current->Find(Codebook::CodeWord(query[i]));
      if (iter == current->End())
        return false;
      current = iter->second;
    }
    return true;
  }

 private:
  class Node {
    USING_FAST_MALLOC(Node);

   public:
    Node(bool is_leaf = false) : is_leaf_(is_leaf) {}
    Node(const Node&) = delete;
    Node& operator=(const Node&) = delete;

    ~Node() {
      for (const auto& pair : children_) {
        Node* child = pair.second;
        if (child && !child->is_leaf_)
          delete child;
      }
    }

    Node*& At(int key) {
      auto it = Find(key);
      if (it != children_.end())
        return it->second;
      children_.emplace_back(key, nullptr);
      return children_.back().second;
    }

    typename Vector<std::pair<int, Node*>>::iterator Find(int key) {
      return std::ranges::find(children_, key, &std::pair<int, Node*>::first);
    }

    typename Vector<std::pair<int, Node*>>::iterator End() {
      return children_.end();
    }

   private:
    // TODO(tsepez): convert to base::flat_map when allowed in blink.
    Vector<std::pair<int, Node*>> children_;
    const bool is_leaf_;
  };

  void Build(const String& text) {
    for (unsigned base = 0; base < text.length(); ++base) {
      Node* current = &root_;
      unsigned limit = std::min(base + depth_, text.length());
      for (unsigned offset = 0; base + offset < limit; ++offset) {
        DCHECK_NE(current, &leaf_);
        Node*& child = current->At(Codebook::CodeWord(text[base + offset]));
        if (!child)
          child = base + offset + 1 == limit ? &leaf_ : new Node();
        current = child;
      }
    }
  }

  // Instead of allocating a fresh empty leaf node for ever leaf in the tree
  // (there can be a lot of these), we alias all the leaves to this "static"
  // leaf node.
  Node leaf_;

  Node root_;
  unsigned depth_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SUFFIX_TREE_H_
