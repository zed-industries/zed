// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_TRACKED_REF_H_
#define BASE_TASK_THREAD_POOL_TRACKED_REF_H_

#include <optional>

#include "base/atomic_ref_count.h"
#include "base/check.h"
#include "base/gtest_prod_util.h"
#include "base/memory/ptr_util.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/waitable_event.h"

namespace base {
namespace internal {

// TrackedRefs are effectively a ref-counting scheme for objects that have a
// single owner.
//
// Deletion is still controlled by the single owner but ~T() itself will block
// until all the TrackedRefs handed by its TrackedRefFactory have been released
// (by ~TrackedRef<T>()).
//
// Just like WeakPtrFactory: TrackedRefFactory<T> should be the last member of T
// to ensure ~TrackedRefFactory<T>() runs first in ~T().
//
// The owner of a T should hence be certain that the last TrackedRefs to T are
// already gone or on their way out before destroying it or ~T() will hang
// (indicating a bug in the tear down logic -- proper refcounting on the other
// hand would result in a leak).
//
// TrackedRefFactory only makes sense to use on types that are always leaked in
// production but need to be torn down in tests (blocking destruction is
// impractical in production).
//
// Why would we ever need such a thing? In thread_pool there is a clear
// ownership hierarchy with mostly single owners and little refcounting. In
// production nothing is ever torn down so this isn't a problem. In tests
// however we must JoinForTesting(). At that point, all the raw back T* refs
// used by the worker threads are problematic because they can result in use-
// after-frees if a worker outlives the deletion of its corresponding
// ThreadPool/TaskTracker/ThreadGroup/etc.
//
// JoinForTesting() isn't so hard when all workers are managed. But with cleanup
// semantics (reclaiming a worker who's been idle for too long) it becomes
// tricky because workers can go unaccounted for before they exit their main
// (https://crbug.com/827615).
//
// For that reason and to clearly document the ownership model, thread_pool
// uses TrackedRefs.
//
// On top of being a clearer ownership model than proper refcounting, a hang in
// tear down in a test with out-of-order tear down logic is much preferred to
// letting its worker thread and associated constructs outlive the test
// (potentially resulting in flakes in unrelated tests running later in the same
// process).
//
// Note: While there's nothing thread_pool specific about TrackedRefs it
// requires an ownership model where all the TrackedRefs are released on other
// threads in sync with ~T(). This isn't a typical use case beyond shutting down
// ThreadPool in tests and as such this is kept internal here for now.

template <class T>
class TrackedRefFactory;

// TrackedRef<T> can be used like a T*.
template <class T>
class TrackedRef {
 public:
  // Moveable and copyable.
  TrackedRef(TrackedRef<T>&& other)
      : ptr_(other.ptr_), factory_(other.factory_) {
    // Null out |other_|'s factory so its destructor doesn't decrement
    // |live_tracked_refs_|.
    other.factory_ = nullptr;
  }
  TrackedRef(const TrackedRef<T>& other)
      : ptr_(other.ptr_), factory_(other.factory_) {
    factory_->live_tracked_refs_.Increment();
  }

  // Intentionally not assignable for now because it makes the logic slightly
  // convoluted and it's not a use case that makes sense for the types using
  // this at the moment.
  TrackedRef& operator=(TrackedRef<T>&& other) = delete;
  TrackedRef& operator=(const TrackedRef<T>& other) = delete;

  ~TrackedRef() {
    if (factory_ && !factory_->live_tracked_refs_.Decrement()) {
      DCHECK(factory_->ready_to_destroy_);
      DCHECK(!factory_->ready_to_destroy_->IsSignaled());
      factory_->ready_to_destroy_->Signal();
    }
  }

  T& operator*() const { return *ptr_; }

  T* operator->() const { return ptr_; }

  explicit operator bool() const { return ptr_ != nullptr; }

  bool operator==(const void* compared_ptr) const {
    return ptr_ == compared_ptr;
  }

  // Returns the raw pointer stored in this TrackedRef. This is occasionally
  // useful for operations in scope but, as with other smart pointers, it
  // shouldn't be used beyond the scope of this TrackedRef.
  T* get() const { return ptr_; }

 private:
  friend class TrackedRefFactory<T>;

  TrackedRef(T* ptr, TrackedRefFactory<T>* factory)
      : ptr_(ptr), factory_(factory) {
    factory_->live_tracked_refs_.Increment();
  }

  raw_ptr<T, LeakedDanglingUntriaged> ptr_;
  raw_ptr<TrackedRefFactory<T>, LeakedDanglingUntriaged> factory_;
};

// TrackedRefFactory<T> should be the last member of T.
template <class T>
class TrackedRefFactory {
 public:
  explicit TrackedRefFactory(T* ptr)
      : ptr_(ptr), self_ref_(TrackedRef<T>(ptr_.get(), this)) {
    DCHECK(ptr_);
  }

  TrackedRefFactory(const TrackedRefFactory&) = delete;
  TrackedRefFactory& operator=(const TrackedRefFactory&) = delete;

  ~TrackedRefFactory() {
    // Enter the destruction phase.
    ready_to_destroy_.emplace();

    // Release self-ref. If this was the last one it will signal the event right
    // away. Otherwise it establishes an happens-after relationship between
    // |ready_to_destroy.emplace()| and the eventual
    // |ready_to_destroy_->Signal()|.
    self_ref_.reset();

    ready_to_destroy_->Wait();
  }

  TrackedRef<T> GetTrackedRef() {
    // TrackedRefs cannot be obtained after |live_tracked_refs_| has already
    // reached zero. In other words, the owner of a TrackedRefFactory shouldn't
    // vend new TrackedRefs while it's being destroyed (owners of TrackedRefs
    // may still copy/move their refs around during the destruction phase).
    DCHECK(!live_tracked_refs_.IsZero());
    return TrackedRef<T>(ptr_.get(), this);
  }

 private:
  friend class TrackedRef<T>;
  FRIEND_TEST_ALL_PREFIXES(TrackedRefTest, CopyAndMoveSemantics);

  const raw_ptr<T> ptr_;

  // The number of live TrackedRefs vended by this factory.
  AtomicRefCount live_tracked_refs_{0};

  // Non-null during the destruction phase. Signaled once |live_tracked_refs_|
  // reaches 0. Note: making this optional and only initializing it in the
  // destruction phase avoids keeping a handle open for the entire session.
  std::optional<WaitableEvent> ready_to_destroy_;

  // TrackedRefFactory holds a TrackedRef as well to prevent
  // |live_tracked_refs_| from ever reaching zero before ~TrackedRefFactory().
  std::optional<TrackedRef<T>> self_ref_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_TRACKED_REF_H_
