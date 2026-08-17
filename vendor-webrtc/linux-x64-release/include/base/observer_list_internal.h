// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_OBSERVER_LIST_INTERNAL_H_
#define BASE_OBSERVER_LIST_INTERNAL_H_

#include <string>
#include <type_traits>

#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/linked_list.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/weak_ptr.h"
#include "base/observer_list_types.h"

#if DCHECK_IS_ON()
#include "base/debug/stack_trace.h"
#endif

namespace base {
namespace internal {

// Adapter for putting raw pointers into an ObserverList<Foo>::Unchecked.
template <base::RawPtrTraits ptr_traits = RawPtrTraits::kEmpty,
          bool use_raw_pointer = false>
class BASE_EXPORT UncheckedObserverAdapter {
 public:
  explicit UncheckedObserverAdapter(const void* observer)
      : ptr_(const_cast<void*>(observer)) {}
  UncheckedObserverAdapter(const UncheckedObserverAdapter&) = delete;
  UncheckedObserverAdapter& operator=(const UncheckedObserverAdapter&) = delete;
  UncheckedObserverAdapter(UncheckedObserverAdapter&& other) = default;
  UncheckedObserverAdapter& operator=(UncheckedObserverAdapter&& other) =
      default;

  void MarkForRemoval() { ptr_ = nullptr; }

  bool IsMarkedForRemoval() const { return !ptr_; }
  bool IsEqual(const void* rhs) const { return ptr_ == rhs; }

  template <class ObserverType>
  static ObserverType* Get(const UncheckedObserverAdapter& adapter) {
    static_assert(
        !std::is_base_of_v<CheckedObserver, ObserverType>,
        "CheckedObserver classes must not use ObserverList<T>::Unchecked.");
    return static_cast<ObserverType*>(adapter.ptr_);
  }

#if DCHECK_IS_ON()
  std::string GetCreationStackString() const {
    return "Observer created at:\n" + stack_.ToString();
  }
#endif  // DCHECK_IS_ON()

 private:
  using StorageType =
      std::conditional_t<use_raw_pointer, void*, raw_ptr<void, ptr_traits>>;
  StorageType ptr_;
#if DCHECK_IS_ON()
  base::debug::StackTrace stack_;
#endif  // DCHECK_IS_ON()
};

// Adapter for CheckedObserver types so that they can use the same syntax as a
// raw pointer when stored in the std::vector of observers in an ObserverList.
// It wraps a WeakPtr<CheckedObserver> and allows a "null" pointer via
// destruction to be distinguished from an observer marked for deferred removal
// whilst an iteration is in progress.
class BASE_EXPORT CheckedObserverAdapter {
 public:
  explicit CheckedObserverAdapter(const CheckedObserver* observer);

  // Move-only construction and assignment is required to store this in STL
  // types.
  CheckedObserverAdapter(CheckedObserverAdapter&& other);
  CheckedObserverAdapter& operator=(CheckedObserverAdapter&& other);
  CheckedObserverAdapter(const CheckedObserverAdapter&) = delete;
  CheckedObserverAdapter& operator=(const CheckedObserverAdapter&) = delete;
  ~CheckedObserverAdapter();

  void MarkForRemoval() {
    DCHECK(weak_ptr_);
    weak_ptr_ = nullptr;
  }

  bool IsMarkedForRemoval() const {
    // If |weak_ptr_| was invalidated then this attempt to iterate over the
    // pointer is a UAF. Tip: If it's unclear where the `delete` occurred, try
    // adding CHECK(!IsInObserverList()) to the ~CheckedObserver() (destructor)
    // override. However, note that this is not always a bug: a destroyed
    // observer can exist in an ObserverList so long as nothing iterates over
    // the ObserverList before the list itself is destroyed.
    CHECK(!weak_ptr_.WasInvalidated());
    return weak_ptr_ == nullptr;
  }

  bool IsEqual(const CheckedObserver* rhs) const {
    // Note that inside an iteration, ObserverList::HasObserver() may call this
    // and |weak_ptr_| may be null due to a deferred removal, which is fine.
    return weak_ptr_.get() == rhs;
  }

  template <class ObserverType>
  static ObserverType* Get(const CheckedObserverAdapter& adapter) {
    static_assert(
        std::is_base_of_v<CheckedObserver, ObserverType>,
        "Observers should inherit from base::CheckedObserver. "
        "Use ObserverList<T>::Unchecked to observe with raw pointers.");
    DCHECK(adapter.weak_ptr_);
    return static_cast<ObserverType*>(adapter.weak_ptr_.get());
  }

#if DCHECK_IS_ON()
  std::string GetCreationStackString() const { return stack_.ToString(); }
#endif

 private:
  WeakPtr<CheckedObserver> weak_ptr_;
#if DCHECK_IS_ON()
  base::debug::StackTrace stack_;
#endif
};

// Wraps a pointer in a stack-allocated, base::LinkNode. The node is
// automatically removed from the linked list upon destruction (of the node, not
// the pointer). Nodes are detached from the list via Invalidate() in the
// destructor of ObserverList. This invalidates all WeakLinkNodes. There is no
// threading support.
template <class ObserverList>
class WeakLinkNode : public base::LinkNode<WeakLinkNode<ObserverList>> {
 public:
  WeakLinkNode() = default;
  explicit WeakLinkNode(ObserverList* list) { SetList(list); }
  WeakLinkNode(const WeakLinkNode&) = delete;
  WeakLinkNode& operator=(const WeakLinkNode&) = delete;

  ~WeakLinkNode() { Invalidate(); }

  bool IsOnlyRemainingNode() const {
    return list_ &&
           list_->live_iterators_.head() == list_->live_iterators_.tail();
  }

  void SetList(ObserverList* list) {
    DCHECK(!list_);
    DCHECK(list);
    list_ = list;
    list_->live_iterators_.Append(this);
  }

  void Invalidate() {
    if (list_) {
      list_ = nullptr;
      this->RemoveFromList();
    }
  }

  ObserverList* get() const {
#if EXPENSIVE_DCHECKS_ARE_ON()
    if (list_) {
      DCHECK_CALLED_ON_VALID_SEQUENCE(list_->iteration_sequence_checker_);
    }
#endif  // EXPENSIVE_DCHECKS_ARE_ON()
    return list_;
  }
  ObserverList* operator->() const { return get(); }
  explicit operator bool() const { return get(); }

 private:
  // `list_` is not a raw_ptr<...> for performance reasons: on-stack pointer +
  // based on analysis of sampling profiler data and tab_search:top100:2020.
  RAW_PTR_EXCLUSION ObserverList* list_ = nullptr;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_OBSERVER_LIST_INTERNAL_H_
