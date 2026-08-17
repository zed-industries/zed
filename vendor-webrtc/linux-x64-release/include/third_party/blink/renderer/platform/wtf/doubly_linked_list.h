/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DOUBLY_LINKED_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DOUBLY_LINKED_LIST_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

// This class allows nodes to share code without dictating data member layout.
template <typename T>
class DoublyLinkedListNode {
  USING_FAST_MALLOC(DoublyLinkedListNode);

 public:
  DoublyLinkedListNode();

  void SetPrev(T*);
  void SetNext(T*);

  T* Prev() const;
  T* Next() const;
};

template <typename T>
inline DoublyLinkedListNode<T>::DoublyLinkedListNode() {
  SetPrev(nullptr);
  SetNext(nullptr);
}

template <typename T>
inline void DoublyLinkedListNode<T>::SetPrev(T* prev) {
  static_cast<T*>(this)->prev_ = prev;
}

template <typename T>
inline void DoublyLinkedListNode<T>::SetNext(T* next) {
  static_cast<T*>(this)->next_ = next;
}

template <typename T>
inline T* DoublyLinkedListNode<T>::Prev() const {
  return static_cast<const T*>(this)->prev_;
}

template <typename T>
inline T* DoublyLinkedListNode<T>::Next() const {
  return static_cast<const T*>(this)->next_;
}

template <typename T, typename PointerType = T*>
class DoublyLinkedList {
  USING_FAST_MALLOC(DoublyLinkedList);

 public:
  DoublyLinkedList();
  DoublyLinkedList(const DoublyLinkedList&) = delete;
  DoublyLinkedList& operator=(const DoublyLinkedList&) = delete;

  bool empty() const;
  wtf_size_t size() const;  // This is O(n).
  void Clear();

  T* Head() const;
  T* RemoveHead();

  T* Tail() const;

  void Push(T*);
  void Append(T*);
  void Remove(T*);

  struct AddResult {
    STACK_ALLOCATED();

   public:
    T* node;
    bool is_new_entry;

    explicit operator bool() const { return is_new_entry; }
  };

  // The following two functions can be used to implement a sorted
  // version of the doubly linked list. It's guaranteed that the list
  // will be sorted by only using Insert(). However the caller is the
  // responsible of inserting the nodes in the right order if
  // InsertAfter() is used. The main use case of the latter is to
  // cheaply insert several consecutive items without having to
  // traverse the whole list.
  //
  // CompareFunc should return -1 if the first argument is strictly
  // less than the second, 0 if they are equal or 1 otherwise.
  template <typename CompareFunc>
  AddResult Insert(std::unique_ptr<T>, const CompareFunc&);
  AddResult InsertAfter(std::unique_ptr<T> node, T* insertion_point);

  template <typename CompareFunc>
  AddResult Insert(T*, const CompareFunc&);
  AddResult InsertAfter(T* node, T* insertion_point);

 protected:
  PointerType head_;
  PointerType tail_;

 private:
  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(!IsStackAllocatedTypeV<T>);
      static_assert(
          !IsGarbageCollectedType<T>::value ||
              !std::is_same<PointerType, T*>::value,
          "Cannot use DoublyLinkedList<> with garbage collected types.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

template <typename T, typename PointerType>
inline DoublyLinkedList<T, PointerType>::DoublyLinkedList()
    : head_(nullptr), tail_(nullptr) {}

template <typename T, typename PointerType>
inline bool DoublyLinkedList<T, PointerType>::empty() const {
  return !head_;
}

template <typename T, typename PointerType>
inline wtf_size_t DoublyLinkedList<T, PointerType>::size() const {
  wtf_size_t size = 0;
  for (T* node = head_; node; node = node->Next())
    ++size;
  return size;
}

template <typename T, typename PointerType>
inline void DoublyLinkedList<T, PointerType>::Clear() {
  head_ = nullptr;
  tail_ = nullptr;
}

template <typename T, typename PointerType>
inline T* DoublyLinkedList<T, PointerType>::Head() const {
  return head_;
}

template <typename T, typename PointerType>
inline T* DoublyLinkedList<T, PointerType>::Tail() const {
  return tail_;
}

template <typename T, typename PointerType>
inline void DoublyLinkedList<T, PointerType>::Push(T* node) {
  if (!head_) {
    DCHECK(!tail_);
    head_ = node;
    tail_ = node;
    node->SetPrev(nullptr);
    node->SetNext(nullptr);
    return;
  }

  DCHECK(tail_);
  head_->SetPrev(node);
  node->SetNext(head_);
  node->SetPrev(nullptr);
  head_ = node;
}

template <typename T, typename PointerType>
inline void DoublyLinkedList<T, PointerType>::Append(T* node) {
  if (!tail_) {
    DCHECK(!head_);
    head_ = node;
    tail_ = node;
    node->SetPrev(nullptr);
    node->SetNext(nullptr);
    return;
  }

  DCHECK(head_);
  tail_->SetNext(node);
  node->SetPrev(tail_);
  node->SetNext(nullptr);
  tail_ = node;
}

template <typename T, typename PointerType>
inline void DoublyLinkedList<T, PointerType>::Remove(T* node) {
  if (node->Prev()) {
    DCHECK_NE(node, head_);
    node->Prev()->SetNext(node->Next());
  } else {
    DCHECK_EQ(node, head_);
    head_ = node->Next();
  }

  if (node->Next()) {
    DCHECK_NE(node, tail_);
    node->Next()->SetPrev(node->Prev());
  } else {
    DCHECK_EQ(node, tail_);
    tail_ = node->Prev();
  }
}

template <typename T, typename PointerType>
inline T* DoublyLinkedList<T, PointerType>::RemoveHead() {
  T* node = Head();
  if (node)
    Remove(node);
  return node;
}

template <typename T, typename PointerType>
template <typename CompareFunc>
inline typename DoublyLinkedList<T, PointerType>::AddResult
DoublyLinkedList<T, PointerType>::Insert(std::unique_ptr<T> node,
                                         const CompareFunc& compare_func) {
  DCHECK(node);
  auto result = Insert(node.get(), compare_func);
  if (result.is_new_entry)
    node.release();
  return result;
}

template <typename T, typename PointerType>
template <typename CompareFunc>
inline typename DoublyLinkedList<T, PointerType>::AddResult
DoublyLinkedList<T, PointerType>::Insert(T* node,
                                         const CompareFunc& compare_func) {
  DCHECK(node);
  T* iter = head_;
  while (iter && compare_func(iter, node) < 0)
    iter = iter->Next();

  if (iter && !compare_func(iter, node))
    return {iter, false};

  return InsertAfter(node, iter ? iter->Prev() : tail_);
}

template <typename T, typename PointerType>
inline typename DoublyLinkedList<T, PointerType>::AddResult
DoublyLinkedList<T, PointerType>::InsertAfter(std::unique_ptr<T> node,
                                              T* insertion_point) {
  DCHECK(node);
  auto result = InsertAfter(node.get(), insertion_point);
  if (result.is_new_entry)
    node.release();
  return result;
}

template <typename T, typename PointerType>
inline typename DoublyLinkedList<T, PointerType>::AddResult
DoublyLinkedList<T, PointerType>::InsertAfter(T* node, T* insertion_point) {
  DCHECK(node);

  if (!insertion_point) {
    Push(node);
    return {head_, true};
  }

  node->SetNext(insertion_point->Next());
  if (insertion_point->Next())
    insertion_point->Next()->SetPrev(node);
  node->SetPrev(insertion_point);
  insertion_point->SetNext(node);

  if (insertion_point == tail_)
    tail_ = node;

  return {node, true};
}

}  // namespace WTF

using WTF::DoublyLinkedListNode;
using WTF::DoublyLinkedList;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DOUBLY_LINKED_LIST_H_
