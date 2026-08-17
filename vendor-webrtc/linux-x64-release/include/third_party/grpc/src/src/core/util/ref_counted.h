//
//
// Copyright 2017 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_UTIL_REF_COUNTED_H
#define GRPC_SRC_CORE_UTIL_REF_COUNTED_H

#include <grpc/support/port_platform.h>

#include <atomic>
#include <cassert>
#include <cinttypes>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "src/core/util/atomic_utils.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/down_cast.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// RefCount is a simple atomic ref-count.
//
// This is a C++ implementation of gpr_refcount, with inline functions. Due to
// inline functions, this class is significantly more efficient than
// gpr_refcount and should be preferred over gpr_refcount whenever possible.
//
// TODO(soheil): Remove gpr_refcount after submitting the GRFC and the paragraph
//               above.
class RefCount {
 public:
  using Value = intptr_t;

  RefCount() : RefCount(1) {}

  // `init` is the initial refcount stored in this object.
  //
  // `trace` is a string to be logged with trace events; if null, no
  // trace logging will be done.  Tracing is a no-op in non-debug builds.
  explicit RefCount(
      Value init,
      const char*
#ifndef NDEBUG
          // Leave unnamed if NDEBUG to avoid unused parameter warning
          trace
#endif
      = nullptr)
      :
#ifndef NDEBUG
        trace_(trace),
#endif
        value_(init) {
  }

  // Increases the ref-count by `n`.
  void Ref(Value n = 1) {
#ifndef NDEBUG
    const Value prior = value_.fetch_add(n, std::memory_order_relaxed);
    if (trace_ != nullptr) {
      LOG(INFO) << trace_ << ":" << this << " ref " << prior << " -> "
                << prior + n;
    }
#else
    value_.fetch_add(n, std::memory_order_relaxed);
#endif
  }
  void Ref(const DebugLocation& location, const char* reason, Value n = 1) {
#ifndef NDEBUG
    const Value prior = value_.fetch_add(n, std::memory_order_relaxed);
    if (trace_ != nullptr) {
      LOG(INFO) << trace_ << ":" << this << " " << location.file() << ":"
                << location.line() << " ref " << prior << " -> " << prior + n
                << " " << reason;
    }
#else
    // Use conditionally-important parameters
    (void)location;
    (void)reason;
    value_.fetch_add(n, std::memory_order_relaxed);
#endif
  }

  // Similar to Ref() with an assert on the ref-count being non-zero.
  void RefNonZero() {
#ifndef NDEBUG
    const Value prior = value_.fetch_add(1, std::memory_order_relaxed);
    if (trace_ != nullptr) {
      LOG(INFO) << trace_ << ":" << this << " ref " << prior << " -> "
                << prior + 1;
    }
    assert(prior > 0);
#else
    value_.fetch_add(1, std::memory_order_relaxed);
#endif
  }
  void RefNonZero(const DebugLocation& location, const char* reason) {
#ifndef NDEBUG
    const Value prior = value_.fetch_add(1, std::memory_order_relaxed);
    if (trace_ != nullptr) {
      LOG(INFO) << trace_ << ":" << this << " " << location.file() << ":"
                << location.line() << " ref " << prior << " -> " << prior + 1
                << " " << reason;
    }
    assert(prior > 0);
#else
    // Avoid unused-parameter warnings for debug-only parameters
    (void)location;
    (void)reason;
    RefNonZero();
#endif
  }

  bool RefIfNonZero() {
#ifndef NDEBUG
    if (trace_ != nullptr) {
      const Value prior = get();
      LOG(INFO) << trace_ << ":" << this << " ref_if_non_zero " << prior
                << " -> " << prior + 1;
    }
#endif
    return IncrementIfNonzero(&value_);
  }
  bool RefIfNonZero(const DebugLocation& location, const char* reason) {
#ifndef NDEBUG
    if (trace_ != nullptr) {
      const Value prior = get();
      LOG(INFO) << trace_ << ":" << this << " " << location.file() << ":"
                << location.line() << " ref_if_non_zero " << prior << " -> "
                << prior + 1 << " " << reason;
    }
#endif
    // Avoid unused-parameter warnings for debug-only parameters
    (void)location;
    (void)reason;
    return IncrementIfNonzero(&value_);
  }

  // Decrements the ref-count and returns true if the ref-count reaches 0.
  bool Unref() {
#ifndef NDEBUG
    // Grab a copy of the trace flag before the atomic change, since we
    // will no longer be holding a ref afterwards and therefore can't
    // safely access it, since another thread might free us in the interim.
    auto* trace = trace_;
#endif
    const Value prior = value_.fetch_sub(1, std::memory_order_acq_rel);
#ifndef NDEBUG
    if (trace != nullptr) {
      LOG(INFO) << trace << ":" << this << " unref " << prior << " -> "
                << prior - 1;
    }
    DCHECK_GT(prior, 0);
#endif
    return prior == 1;
  }
  bool Unref(const DebugLocation& location, const char* reason) {
#ifndef NDEBUG
    // Grab a copy of the trace flag before the atomic change, since we
    // will no longer be holding a ref afterwards and therefore can't
    // safely access it, since another thread might free us in the interim.
    auto* trace = trace_;
#endif
    const Value prior = value_.fetch_sub(1, std::memory_order_acq_rel);
#ifndef NDEBUG
    if (trace != nullptr) {
      LOG(INFO) << trace << ":" << this << " " << location.file() << ":"
                << location.line() << " unref " << prior << " -> " << prior - 1
                << " " << reason;
    }
    DCHECK_GT(prior, 0);
#else
    // Avoid unused-parameter warnings for debug-only parameters
    (void)location;
    (void)reason;
#endif
    return prior == 1;
  }

 private:
  Value get() const { return value_.load(std::memory_order_relaxed); }

#ifndef NDEBUG
  const char* trace_;
#endif
  std::atomic<Value> value_{0};
};

// PolymorphicRefCount enforces polymorphic destruction of RefCounted.
class PolymorphicRefCount {
 public:
  virtual ~PolymorphicRefCount() = default;
};

// NonPolymorphicRefCount does not enforce polymorphic destruction of
// RefCounted. Please refer to RefCounted for more details, and
// when in doubt use PolymorphicRefCount.
class NonPolymorphicRefCount {
 public:
  ~NonPolymorphicRefCount() = default;
};

// Behavior of RefCounted<> upon ref count reaching 0.

// Default behavior: Delete the object.
struct UnrefDelete {
  template <typename T>
  void operator()(T* p) const {
    delete p;
  }
};

// Do not delete the object upon unref.  This is useful in cases where all
// existing objects must be tracked in a registry but the object's entry in
// the registry cannot be removed from the object's dtor due to
// synchronization issues.  In this case, the registry can be cleaned up
// later by identifying entries for which RefIfNonZero() returns null.
struct UnrefNoDelete {
  template <typename T>
  void operator()(T* /*p*/) const {}
};

// Call the object's dtor but do not delete it.  This is useful for cases
// where the object is stored in memory allocated elsewhere (e.g., the call
// arena).
struct UnrefCallDtor {
  template <typename T>
  void operator()(T* p) const {
    p->~T();
  }
};

// Call the Destroy method on the object. This is useful when the object
// needs precise control of how it's deallocated.
struct UnrefCallDestroy {
  template <typename T>
  void operator()(T* p) const {
    p->Destroy();
  }
};

// A base class for reference-counted objects.
// New objects should be created via new and start with a refcount of 1.
// When the refcount reaches 0, executes the specified UnrefBehavior.
//
// This will commonly be used by CRTP (curiously-recurring template pattern)
// e.g., class MyClass : public RefCounted<MyClass>
//
// Use PolymorphicRefCount and NonPolymorphicRefCount to select between
// different implementations of RefCounted.
//
// Note that NonPolymorphicRefCount does not support polymorphic destruction.
// So, use NonPolymorphicRefCount only when both of the following conditions
// are guaranteed to hold:
// (a) Child is a concrete leaf class in RefCounted<Child>, and
// (b) you are guaranteed to call Unref only on concrete leaf classes and not
//     their parents.
//
// The following example is illegal, because calling Unref() will not call
// the dtor of Child.
//
//    class Parent : public RefCounted<Parent, NonPolymorphicRefCount> {}
//    class Child : public Parent {}
//
//    Child* ch;
//    ch->Unref();
//
template <typename Child, typename Impl = PolymorphicRefCount,
          typename UnrefBehavior = UnrefDelete>
class RefCounted : public Impl {
 public:
  using RefCountedChildType = Child;
  using RefCountedUnrefBehaviorType = UnrefBehavior;

  // Not copyable nor movable.
  RefCounted(const RefCounted&) = delete;
  RefCounted& operator=(const RefCounted&) = delete;

  // Note: Depending on the Impl used, this dtor can be implicitly virtual.
  ~RefCounted() = default;

  // Ref() for mutable types.
  GRPC_MUST_USE_RESULT RefCountedPtr<Child> Ref() {
    IncrementRefCount();
    return RefCountedPtr<Child>(static_cast<Child*>(this));
  }
  GRPC_MUST_USE_RESULT RefCountedPtr<Child> Ref(const DebugLocation& location,
                                                const char* reason) {
    IncrementRefCount(location, reason);
    return RefCountedPtr<Child>(static_cast<Child*>(this));
  }

  // Ref() for const types.
  GRPC_MUST_USE_RESULT RefCountedPtr<const Child> Ref() const {
    IncrementRefCount();
    return RefCountedPtr<const Child>(static_cast<const Child*>(this));
  }
  GRPC_MUST_USE_RESULT RefCountedPtr<const Child> Ref(
      const DebugLocation& location, const char* reason) const {
    IncrementRefCount(location, reason);
    return RefCountedPtr<const Child>(static_cast<const Child*>(this));
  }

  template <
      typename Subclass,
      std::enable_if_t<std::is_base_of<Child, Subclass>::value, bool> = true>
  RefCountedPtr<Subclass> RefAsSubclass() {
    IncrementRefCount();
    return RefCountedPtr<Subclass>(
        DownCast<Subclass*>(static_cast<Child*>(this)));
  }
  template <
      typename Subclass,
      std::enable_if_t<std::is_base_of<Child, Subclass>::value, bool> = true>
  RefCountedPtr<Subclass> RefAsSubclass(const DebugLocation& location,
                                        const char* reason) {
    IncrementRefCount(location, reason);
    return RefCountedPtr<Subclass>(
        DownCast<Subclass*>(static_cast<Child*>(this)));
  }

  // RefIfNonZero() for mutable types.
  GRPC_MUST_USE_RESULT RefCountedPtr<Child> RefIfNonZero() {
    return RefCountedPtr<Child>(refs_.RefIfNonZero() ? static_cast<Child*>(this)
                                                     : nullptr);
  }
  GRPC_MUST_USE_RESULT RefCountedPtr<Child> RefIfNonZero(
      const DebugLocation& location, const char* reason) {
    return RefCountedPtr<Child>(refs_.RefIfNonZero(location, reason)
                                    ? static_cast<Child*>(this)
                                    : nullptr);
  }

  // RefIfNonZero() for const types.
  GRPC_MUST_USE_RESULT RefCountedPtr<const Child> RefIfNonZero() const {
    return RefCountedPtr<const Child>(
        refs_.RefIfNonZero() ? static_cast<const Child*>(this) : nullptr);
  }
  GRPC_MUST_USE_RESULT RefCountedPtr<const Child> RefIfNonZero(
      const DebugLocation& location, const char* reason) const {
    return RefCountedPtr<const Child>(refs_.RefIfNonZero(location, reason)
                                          ? static_cast<const Child*>(this)
                                          : nullptr);
  }

  // TODO(roth): Once all of our code is converted to C++ and can use
  // RefCountedPtr<> instead of manual ref-counting, make this method
  // private, since it will only be used by RefCountedPtr<>, which is a
  // friend of this class.
  void Unref() const {
    if (GPR_UNLIKELY(refs_.Unref())) {
      unref_behavior_(static_cast<const Child*>(this));
    }
  }
  void Unref(const DebugLocation& location, const char* reason) const {
    if (GPR_UNLIKELY(refs_.Unref(location, reason))) {
      unref_behavior_(static_cast<const Child*>(this));
    }
  }

 protected:
  // Note: Tracing is a no-op on non-debug builds.
  explicit RefCounted(const char* trace = nullptr,
                      intptr_t initial_refcount = 1)
      : refs_(initial_refcount, trace) {}

  // Note: Tracing is a no-op on non-debug builds.
  explicit RefCounted(UnrefBehavior b, const char* trace = nullptr,
                      intptr_t initial_refcount = 1)
      : refs_(initial_refcount, trace), unref_behavior_(b) {}

 private:
  // Allow RefCountedPtr<> to access IncrementRefCount().
  template <typename T>
  friend class RefCountedPtr;

  void IncrementRefCount() const { refs_.Ref(); }
  void IncrementRefCount(const DebugLocation& location,
                         const char* reason) const {
    refs_.Ref(location, reason);
  }

  mutable RefCount refs_;
  GPR_NO_UNIQUE_ADDRESS UnrefBehavior unref_behavior_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_REF_COUNTED_H
