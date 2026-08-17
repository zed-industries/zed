//===-- list.h --------------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SCUDO_LIST_H_
#define SCUDO_LIST_H_

#include "internal_defs.h"
#include "type_traits.h"

namespace scudo {

// Intrusive POD singly and doubly linked list.
// An object with all zero fields should represent a valid empty list. clear()
// should be called on all non-zero-initialized objects before using.
//
// The intrusive list requires the member `Next` (and `Prev` if doubly linked
// list)` defined in the node type. The type of `Next`/`Prev` can be a pointer
// or an index to an array. For example, if the storage of the nodes is an
// array, instead of using a pointer type, linking with an index type can save
// some space.
//
// There are two things to be noticed while using an index type,
//   1. Call init() to set up the base address of the array.
//   2. Define `EndOfListVal` as the nil of the list.

template <class T, bool LinkWithPtr = isPointer<decltype(T::Next)>::value>
class LinkOp {
public:
  LinkOp() = default;
  LinkOp(UNUSED T *BaseT, UNUSED uptr BaseSize) {}
  void init(UNUSED T *LinkBase, UNUSED uptr Size) {}
  T *getBase() const { return nullptr; }
  uptr getSize() const { return 0; }

  T *getNext(T *X) const { return X->Next; }
  void setNext(T *X, T *Next) const { X->Next = Next; }

  T *getPrev(T *X) const { return X->Prev; }
  void setPrev(T *X, T *Prev) const { X->Prev = Prev; }

  T *getEndOfListVal() const { return nullptr; }
};

template <class T> class LinkOp<T, /*LinkWithPtr=*/false> {
public:
  using LinkTy = typename assertSameType<
      typename removeConst<decltype(T::Next)>::type,
      typename removeConst<decltype(T::EndOfListVal)>::type>::type;

  LinkOp() = default;
  LinkOp(T *BaseT, uptr BaseSize)
      : Base(BaseT), Size(static_cast<LinkTy>(BaseSize)) {}
  void init(T *LinkBase, uptr BaseSize) {
    Base = LinkBase;
    Size = static_cast<LinkTy>(BaseSize);
  }
  T *getBase() const { return Base; }
  LinkTy getSize() const { return Size; }

  T *getNext(T *X) const {
    DCHECK_NE(getBase(), nullptr);
    if (X->Next == getEndOfListVal())
      return nullptr;
    DCHECK_LT(X->Next, Size);
    return &Base[X->Next];
  }
  // Set `X->Next` to `Next`.
  void setNext(T *X, T *Next) const {
    if (Next == nullptr) {
      X->Next = getEndOfListVal();
    } else {
      assertElementInRange(Next);
      X->Next = static_cast<LinkTy>(Next - Base);
    }
  }

  T *getPrev(T *X) const {
    DCHECK_NE(getBase(), nullptr);
    if (X->Prev == getEndOfListVal())
      return nullptr;
    DCHECK_LT(X->Prev, Size);
    return &Base[X->Prev];
  }
  // Set `X->Prev` to `Prev`.
  void setPrev(T *X, T *Prev) const {
    if (Prev == nullptr) {
      X->Prev = getEndOfListVal();
    } else {
      assertElementInRange(Prev);
      X->Prev = static_cast<LinkTy>(Prev - Base);
    }
  }

  LinkTy getEndOfListVal() const { return T::EndOfListVal; }

private:
  void assertElementInRange(T *X) const {
    DCHECK_GE(reinterpret_cast<uptr>(X), reinterpret_cast<uptr>(Base));
    DCHECK_LE(static_cast<LinkTy>(X - Base), Size);
  }

protected:
  T *Base = nullptr;
  LinkTy Size = 0;
};

template <class T> class IteratorBase : public LinkOp<T> {
public:
  IteratorBase(const LinkOp<T> &Link, T *CurrentT)
      : LinkOp<T>(Link), Current(CurrentT) {}

  IteratorBase &operator++() {
    Current = this->getNext(Current);
    return *this;
  }
  bool operator!=(IteratorBase Other) const { return Current != Other.Current; }
  T &operator*() { return *Current; }

private:
  T *Current;
};

template <class T> struct IntrusiveList : public LinkOp<T> {
  IntrusiveList() = default;
  void init(T *Base, uptr BaseSize) { LinkOp<T>::init(Base, BaseSize); }

  bool empty() const { return Size == 0; }
  uptr size() const { return Size; }

  T *front() { return First; }
  const T *front() const { return First; }
  T *back() { return Last; }
  const T *back() const { return Last; }

  void clear() {
    First = Last = nullptr;
    Size = 0;
  }

  typedef IteratorBase<T> Iterator;
  typedef IteratorBase<const T> ConstIterator;

  Iterator begin() {
    return Iterator(LinkOp<T>(this->getBase(), this->getSize()), First);
  }
  Iterator end() {
    return Iterator(LinkOp<T>(this->getBase(), this->getSize()), nullptr);
  }

  ConstIterator begin() const {
    return ConstIterator(LinkOp<const T>(this->getBase(), this->getSize()),
                         First);
  }
  ConstIterator end() const {
    return ConstIterator(LinkOp<const T>(this->getBase(), this->getSize()),
                         nullptr);
  }

  void checkConsistency() const;

protected:
  uptr Size = 0;
  T *First = nullptr;
  T *Last = nullptr;
};

template <class T> void IntrusiveList<T>::checkConsistency() const {
  if (Size == 0) {
    CHECK_EQ(First, nullptr);
    CHECK_EQ(Last, nullptr);
  } else {
    uptr Count = 0;
    for (T *I = First;; I = this->getNext(I)) {
      Count++;
      if (I == Last)
        break;
    }
    CHECK_EQ(this->size(), Count);
    CHECK_EQ(this->getNext(Last), nullptr);
  }
}

template <class T> struct SinglyLinkedList : public IntrusiveList<T> {
  using IntrusiveList<T>::First;
  using IntrusiveList<T>::Last;
  using IntrusiveList<T>::Size;
  using IntrusiveList<T>::empty;
  using IntrusiveList<T>::setNext;
  using IntrusiveList<T>::getNext;
  using IntrusiveList<T>::getEndOfListVal;

  void push_back(T *X) {
    setNext(X, nullptr);
    if (empty())
      First = X;
    else
      setNext(Last, X);
    Last = X;
    Size++;
  }

  void push_front(T *X) {
    if (empty())
      Last = X;
    setNext(X, First);
    First = X;
    Size++;
  }

  void pop_front() {
    DCHECK(!empty());
    First = getNext(First);
    if (!First)
      Last = nullptr;
    Size--;
  }

  // Insert X next to Prev
  void insert(T *Prev, T *X) {
    DCHECK(!empty());
    DCHECK_NE(Prev, nullptr);
    DCHECK_NE(X, nullptr);
    setNext(X, getNext(Prev));
    setNext(Prev, X);
    if (Last == Prev)
      Last = X;
    ++Size;
  }

  void extract(T *Prev, T *X) {
    DCHECK(!empty());
    DCHECK_NE(Prev, nullptr);
    DCHECK_NE(X, nullptr);
    DCHECK_EQ(getNext(Prev), X);
    setNext(Prev, getNext(X));
    if (Last == X)
      Last = Prev;
    Size--;
  }

  void append_back(SinglyLinkedList<T> *L) {
    DCHECK_NE(this, L);
    if (L->empty())
      return;
    if (empty()) {
      *this = *L;
    } else {
      setNext(Last, L->First);
      Last = L->Last;
      Size += L->size();
    }
    L->clear();
  }
};

template <class T> struct DoublyLinkedList : IntrusiveList<T> {
  using IntrusiveList<T>::First;
  using IntrusiveList<T>::Last;
  using IntrusiveList<T>::Size;
  using IntrusiveList<T>::empty;
  using IntrusiveList<T>::setNext;
  using IntrusiveList<T>::getNext;
  using IntrusiveList<T>::setPrev;
  using IntrusiveList<T>::getPrev;
  using IntrusiveList<T>::getEndOfListVal;

  void push_front(T *X) {
    setPrev(X, nullptr);
    if (empty()) {
      Last = X;
    } else {
      DCHECK_EQ(getPrev(First), nullptr);
      setPrev(First, X);
    }
    setNext(X, First);
    First = X;
    Size++;
  }

  // Inserts X before Y.
  void insert(T *X, T *Y) {
    if (Y == First)
      return push_front(X);
    T *Prev = getPrev(Y);
    // This is a hard CHECK to ensure consistency in the event of an intentional
    // corruption of Y->Prev, to prevent a potential write-{4,8}.
    CHECK_EQ(getNext(Prev), Y);
    setNext(Prev, X);
    setPrev(X, Prev);
    setNext(X, Y);
    setPrev(Y, X);
    Size++;
  }

  void push_back(T *X) {
    setNext(X, nullptr);
    if (empty()) {
      First = X;
    } else {
      DCHECK_EQ(getNext(Last), nullptr);
      setNext(Last, X);
    }
    setPrev(X, Last);
    Last = X;
    Size++;
  }

  void pop_front() {
    DCHECK(!empty());
    First = getNext(First);
    if (!First)
      Last = nullptr;
    else
      setPrev(First, nullptr);
    Size--;
  }

  // The consistency of the adjacent links is aggressively checked in order to
  // catch potential corruption attempts, that could yield a mirrored
  // write-{4,8} primitive. nullptr checks are deemed less vital.
  void remove(T *X) {
    T *Prev = getPrev(X);
    T *Next = getNext(X);
    if (Prev) {
      CHECK_EQ(getNext(Prev), X);
      setNext(Prev, Next);
    }
    if (Next) {
      CHECK_EQ(getPrev(Next), X);
      setPrev(Next, Prev);
    }
    if (First == X) {
      DCHECK_EQ(Prev, nullptr);
      First = Next;
    } else {
      DCHECK_NE(Prev, nullptr);
    }
    if (Last == X) {
      DCHECK_EQ(Next, nullptr);
      Last = Prev;
    } else {
      DCHECK_NE(Next, nullptr);
    }
    Size--;
  }
};

} // namespace scudo

#endif // SCUDO_LIST_H_
