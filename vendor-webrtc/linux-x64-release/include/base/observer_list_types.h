// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_OBSERVER_LIST_TYPES_H_
#define BASE_OBSERVER_LIST_TYPES_H_

#include "base/base_export.h"
#include "base/memory/weak_ptr.h"

namespace base {
namespace internal {
class CheckedObserverAdapter;
}

// A CheckedObserver serves as a base class for an observer interface designed
// to be used with base::ObserverList. It helps detect potential use-after-free
// issues that can occur when observers fail to remove themselves from an
// observer list upon destruction.
//
// A CheckedObserver will CHECK() if an ObserverList iteration is attempted over
// a destroyed Observer.
//
// Note that a CheckedObserver subclass must be deleted on the same thread as
// the ObserverList(s) it is added to. This is DCHECK()ed via WeakPtr.
class BASE_EXPORT CheckedObserver {
 public:
  CheckedObserver();
  CheckedObserver(const CheckedObserver&) = delete;
  CheckedObserver& operator=(const CheckedObserver&) = delete;

  virtual ~CheckedObserver();

  // Returns whether |this| is in any ObserverList. Subclasses can CHECK() this
  // in their destructor to obtain a nicer stacktrace.
  bool IsInObserverList() const;

 private:
  friend class internal::CheckedObserverAdapter;

  // Must be mutable to allow ObserverList<const Foo>.
  mutable WeakPtrFactory<CheckedObserver> factory_{this};
};

}  // namespace base

#endif  // BASE_OBSERVER_LIST_TYPES_H_
