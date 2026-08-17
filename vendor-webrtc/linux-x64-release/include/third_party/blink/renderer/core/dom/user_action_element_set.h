/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_USER_ACTION_ELEMENT_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_USER_ACTION_ELEMENT_SET_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Node;
class Element;

class UserActionElementSet final {
  DISALLOW_NEW();

 public:
  bool IsFocused(const Node* node) { return HasFlags(node, kIsFocusedFlag); }
  bool HasFocusWithin(const Node* node) {
    return HasFlags(node, kHasFocusWithinFlag);
  }
  bool IsActive(const Node* node) { return HasFlags(node, kIsActiveFlag); }
  bool IsInActiveChain(const Node* node) {
    return HasFlags(node, kInActiveChainFlag);
  }
  bool IsDragged(const Node* node) { return HasFlags(node, kIsDraggedFlag); }
  bool IsHovered(const Node* node) { return HasFlags(node, kIsHoveredFlag); }
  HeapVector<Member<Element>> ActiveElements() const {
    return GetAllWithFlags(kIsActiveFlag);
  }

  void SetFocused(Node* node, bool enable) {
    SetFlags(node, enable, kIsFocusedFlag);
  }
  void SetHasFocusWithin(Node* node, bool enable) {
    SetFlags(node, enable, kHasFocusWithinFlag);
  }
  void SetActive(Node* node, bool enable) {
    SetFlags(node, enable, kIsActiveFlag);
  }
  void SetInActiveChain(Node* node, bool enable) {
    SetFlags(node, enable, kInActiveChainFlag);
  }
  void SetDragged(Node* node, bool enable) {
    SetFlags(node, enable, kIsDraggedFlag);
  }
  void SetHovered(Node* node, bool enable) {
    SetFlags(node, enable, kIsHoveredFlag);
  }

  UserActionElementSet();

  void DidDetach(Element&);

  void Trace(Visitor*) const;

 private:
  enum ElementFlags {
    kIsActiveFlag = 1,
    kInActiveChainFlag = 1 << 1,
    kIsHoveredFlag = 1 << 2,
    kIsFocusedFlag = 1 << 3,
    kIsDraggedFlag = 1 << 4,
    kHasFocusWithinFlag = 1 << 5,
  };

  void SetFlags(Node* node, bool enable, unsigned flags) {
    enable ? SetFlags(node, flags) : ClearFlags(node, flags);
  }
  void SetFlags(Node*, unsigned);
  void ClearFlags(Node*, unsigned);
  bool HasFlags(const Node*, unsigned flags) const;
  HeapVector<Member<Element>> GetAllWithFlags(const unsigned flags) const;

  void SetFlags(Element*, unsigned);
  void ClearFlags(Element*, unsigned);
  bool HasFlags(const Element*, unsigned flags) const;

  typedef HeapHashMap<Member<Element>, unsigned> ElementFlagMap;
  ElementFlagMap elements_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_USER_ACTION_ELEMENT_SET_H_
