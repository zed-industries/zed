// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WEAK_CELL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WEAK_CELL_H_

#include <type_traits>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

template <typename T>
class WeakCellFactory;

// A `WeakCell<T>` provides a GC-safe pattern for classes that want to:
// - expose weak references to themselves
// - invalidate the weak references *before* the object becomes unreachable.
//
// This differs from `WeakMember<T>` in the last point, as `WeakMember<T>` only
// becomes implicitly null after the referenced `T` is no longer reachable. In
// Chrome, a common use of early invalidation is to cancel callbacks that have
// not yet run.
//
// If early invalidation is not needed, please use `WeakMember<T>`!
//
// Like many Oilpan types, this class is thread-unsafe.
//
// Note: This is the GC-safe version of `WeakPtrFactory<T>` + `WeakPtr<T>`.
// `WeakPtrFactory<T>` + `WeakPtr<T>` are GC-unsafe and should not be embedded
// in objects that live on the Oilpan heap.
template <typename T>
class WeakCell : public GarbageCollected<WeakCell<T>> {
 public:
  // Returns a pointer to the referenced object, or null if:
  // - the cell has been invalidated by its factory
  // - or the referenced object is no longer reachable.
  T* Get() const { return ptr_; }

  void Trace(Visitor* v) const { v->Trace(ptr_); }

  // Internal helpers for `WeakCellFactory` implementation.
  explicit WeakCell(base::PassKey<WeakCellFactory<T>>, T* ptr) : ptr_(ptr) {}
  void Invalidate(base::PassKey<WeakCellFactory<T>>) { ptr_ = nullptr; }

 private:
  WeakMember<T> ptr_;
};

// A `WeakCellFactory<T>` vends out a pointer to a `WeakCell<T>`, and allows the
// owning class to invalidate `WeakCell<T>`s that have already been handed out.
//
// Usage overview:
//
// class DatabaseScheduler : public GarbageCollected<DatabaseScheduler> {
//  public:
//   ...
//
//   void DoWork();
//   void CancelWork();
//
//  private:
//   // Note: field ordering for `WeakCellFactory` does not matter, and it does
//   // *not* have to be the last field in a class.
//   WeakCellFactory<DatabaseScheduler> weak_factory_{this};
//   // Note: this is *not* a cross-thread task runner. In Blink, many task
//   // queues are multiplexed onto one thread.
//   scoped_refptr<base::TaskRunner> db_task_queue;
// };
//
// void DatabaseScheduler::DoWork() {
//   // IMPORTANT: the `WrapPersistent()` around the `WeakCell<T>` argument is
//   // mandatory, as `WeakCell<T>` itself is allocated on the Oilpan heap.
//   db_task_queue_->PostTask(
//       FROM_HERE,
//       base::BindOnce(&DatabaseScheduler::DoRealWork,
//                      WrapPersistent(weak_factory_.GetWeakCell())));
// }
//
// void DatabaseScheduler::CancelWork() {
//   // Any already-posted but not-yet-run tasks using a `WeakCell<T>` as the
//   // receiver will not run.
//   // However, any subsequent calls to `DoWork()` above *will* schedule new
//   // callbacks that will run unless `CancelWork()` is called again.
//   weak_factory_.Invalidate();
// }
template <typename T>
class WeakCellFactory {
  DISALLOW_NEW();

 public:
  explicit WeakCellFactory(T* ptr) : ptr_(ptr) {}

  WeakCell<T>* GetWeakCell() {
    if (!weak_cell_) {
      weak_cell_ = MakeGarbageCollected<WeakCell<T>>(
          base::PassKey<WeakCellFactory>(), ptr_);
    }
    DCHECK(weak_cell_);
    return weak_cell_.Get();
  }

  bool HasWeakCells() const { return weak_cell_; }

  // Invalidates the previous `WeakCell<T>` so that `previous_cell->Get()`
  // returns null. Future calls to `GetWeakCell()` will return a *new* and
  // *non-null* cell.
  void Invalidate() {
    if (!weak_cell_) return;
    weak_cell_->Invalidate(base::PassKey<WeakCellFactory>());
    weak_cell_ = nullptr;
  }

  void Trace(Visitor* v) const {
    v->Trace(ptr_);
    v->Trace(weak_cell_);
  }

 private:
  const WeakMember<T> ptr_;
  Member<WeakCell<T>> weak_cell_;
};

}  // namespace blink

namespace base {

template <typename T>
struct IsWeakReceiver<blink::Persistent<blink::WeakCell<T>>> : std::true_type {
};

template <typename T>
struct BindUnwrapTraits<blink::Persistent<blink::WeakCell<T>>> {
  static T* Unwrap(const blink::Persistent<blink::WeakCell<T>>& wrapped) {
    return wrapped->Get();
  }
};

template <typename T>
struct MaybeValidTraits<blink::Persistent<blink::WeakCell<T>>> {
  static constexpr bool MaybeValid(
      const blink::Persistent<blink::WeakCell<T>>& p) {
    // Not necessarily called on `Persistent<T>` and `WeakCell<T>`'s owning
    // thread, so the only possible implementation is to assume the weak cell
    // has not been invalidated.
    return true;
  }
};

template <typename T>
struct IsWeakReceiver<blink::UnwrappingCrossThreadHandle<blink::WeakCell<T>>>
    : std::true_type {};

template <typename T>
struct BindUnwrapTraits<
    blink::UnwrappingCrossThreadHandle<blink::WeakCell<T>>> {
  static T* Unwrap(
      const blink::UnwrappingCrossThreadHandle<blink::WeakCell<T>>& wrapped) {
    return wrapped.GetOnCreationThread()->Get();
  }
};

template <typename T>
struct MaybeValidTraits<
    blink::UnwrappingCrossThreadHandle<blink::WeakCell<T>>> {
  static constexpr bool MaybeValid(
      const blink::UnwrappingCrossThreadHandle<blink::WeakCell<T>>& p) {
    // Not necessarily called on `UnwrappingCrossThreadHandle<T>` and
    // `WeakCell<T>`'s owning thread, so the only possible implementation is to
    // assume the weak cell has not been invalidated.
    return true;
  }
};

}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WEAK_CELL_H_
