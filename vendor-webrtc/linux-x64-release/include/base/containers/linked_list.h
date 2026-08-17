// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_LINKED_LIST_H_
#define BASE_CONTAINERS_LINKED_LIST_H_

#include <utility>

#include "base/base_export.h"
#include "base/memory/raw_ptr_exclusion.h"

// Simple LinkedList type. (See the Q&A section to understand how this
// differs from std::list).
//
// To use, start by declaring the class which will be contained in the linked
// list, as extending LinkNode (this gives it next/previous pointers).
//
//   class MyNodeType : public LinkNode<MyNodeType> {
//     ...
//   };
//
// Next, to keep track of the list's head/tail, use a LinkedList instance:
//
//   LinkedList<MyNodeType> list;
//
// To add elements to the list, use any of LinkedList::Append,
// LinkNode::InsertBefore, or LinkNode::InsertAfter:
//
//   LinkNode<MyNodeType>* n1 = ...;
//   LinkNode<MyNodeType>* n2 = ...;
//   LinkNode<MyNodeType>* n3 = ...;
//
//   list.Append(n1);
//   list.Append(n3);
//   n2->InsertBefore(n3);
//
// Lastly, to iterate through the linked list forwards:
//
//   for (LinkNode<MyNodeType>* node = list.head();
//        node != list.end();
//        node = node->next()) {
//     MyNodeType* value = node->value();
//     ...
//   }
//
// Or to iterate the linked list backwards:
//
//   for (LinkNode<MyNodeType>* node = list.tail();
//        node != list.end();
//        node = node->previous()) {
//     MyNodeType* value = node->value();
//     ...
//   }
//
// Questions and Answers:
//
// Q. Should I use std::list or base::LinkedList?
//
// A. The main reason to use base::LinkedList over std::list is
//    performance. If you don't care about the performance differences
//    then use an STL container, as it makes for better code readability.
//
//    Comparing the performance of base::LinkedList<T> to std::list<T*>:
//
//    * Erasing an element of type T* from base::LinkedList<T> is
//      an O(1) operation. Whereas for std::list<T*> it is O(n).
//      That is because with std::list<T*> you must obtain an
//      iterator to the T* element before you can call erase(iterator).
//
//    * Insertion operations with base::LinkedList<T> never require
//      heap allocations.
//
// Q. How does base::LinkedList implementation differ from std::list?
//
// A. Doubly-linked lists are made up of nodes that contain "next" and
//    "previous" pointers that reference other nodes in the list.
//
//    With base::LinkedList<T>, the type being inserted already reserves
//    space for the "next" and "previous" pointers (base::LinkNode<T>*).
//    Whereas with std::list<T> the type can be anything, so the implementation
//    needs to glue on the "next" and "previous" pointers using
//    some internal node type.

namespace base {

template <typename T>
class LinkedList;

namespace internal {

// Base class for LinkNode<T> type
class BASE_EXPORT LinkNodeBase {
 public:
  void RemoveFromList();

 protected:
  LinkNodeBase();
  LinkNodeBase(LinkNodeBase* previous, LinkNodeBase* next);
  LinkNodeBase(LinkNodeBase&& rhs);
  LinkNodeBase(const LinkNodeBase&) = delete;
  ~LinkNodeBase() = default;

  LinkNodeBase& operator=(const LinkNodeBase&) = delete;

  // Calling these with |e| as a different LinkNode type as |this| is
  // unsafe. These are protected and only called from LinkNode<T> to
  // ensure safety.
  void InsertBeforeBase(LinkNodeBase* e);
  void InsertAfterBase(LinkNodeBase* e);

  LinkNodeBase* previous_base() const { return previous_; }
  LinkNodeBase* next_base() const { return next_; }

  // Make `previous_` and `next_` point to `this`. Can only be called when
  // `next_` and `previous_` are nullptr or already point to `this`.
  void MakeSelfReferencingBase();

 private:
  // `previous_` and `next_` are not a raw_ptr<...> for performance reasons:
  // on-stack pointer + a large number of non-PA pointees through WeakLinkNode +
  // based on analysis of sampling profiler data and tab_search:top100:2020.
  RAW_PTR_EXCLUSION LinkNodeBase* previous_ = nullptr;
  RAW_PTR_EXCLUSION LinkNodeBase* next_ = nullptr;
};

}  // namespace internal

template <typename T>
class LinkNode : public internal::LinkNodeBase {
 public:
  LinkNode() = default;
  LinkNode(LinkNode<T>* previous, LinkNode<T>* next)
      : internal::LinkNodeBase(previous, next) {}

  LinkNode(LinkNode<T>&&) = default;

  LinkNode(const LinkNode&) = delete;
  LinkNode& operator=(const LinkNode&) = delete;

  // Insert |this| into the linked list, before |e|. |this| must not
  // already be in a list.
  void InsertBefore(LinkNode<T>* e) { InsertBeforeBase(e); }

  // Insert |this| into the linked list, after |e|. |this| must not
  // already be in a list.
  void InsertAfter(LinkNode<T>* e) { InsertAfterBase(e); }

  LinkNode<T>* previous() const {
    return static_cast<LinkNode<T>*>(previous_base());
  }

  LinkNode<T>* next() const { return static_cast<LinkNode<T>*>(next_base()); }

  // Cast from the node-type to the value type.
  const T* value() const { return static_cast<const T*>(this); }

  T* value() { return static_cast<T*>(this); }

 private:
  friend class LinkedList<T>;

  // Make this node point to itself like a LinkedList root node.
  void MakeSelfReferencing() { MakeSelfReferencingBase(); }
};

template <typename T>
class LinkedList {
 public:
  // The "root" node is self-referential, and forms the basis of a circular
  // list (root_.next() will point back to the start of the list,
  // and root_->previous() wraps around to the end of the list).
  LinkedList() : root_(&root_, &root_) {}
  LinkedList(const LinkedList&) = delete;
  LinkedList& operator=(const LinkedList&) = delete;

  // Use move constructor with care. Returning a LinkedList from a function may
  // be unsafe if the nodes are allocated on the stack. This operation is O(1)
  // as only head and tail nodes are modified. `other` is left empty.
  LinkedList(LinkedList&& other) : root_(std::move(other.root_)) {
    other.root_.MakeSelfReferencing();
  }

  // Appends |e| to the end of the linked list.
  void Append(LinkNode<T>* e) { e->InsertBefore(&root_); }

  LinkNode<T>* head() const { return root_.next(); }

  LinkNode<T>* tail() const { return root_.previous(); }

  const LinkNode<T>* end() const { return &root_; }

  bool empty() const { return head() == end(); }

 private:
  LinkNode<T> root_;
};

}  // namespace base

#endif  // BASE_CONTAINERS_LINKED_LIST_H_
