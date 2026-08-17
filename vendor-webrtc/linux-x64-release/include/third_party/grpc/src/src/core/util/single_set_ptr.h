// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_SINGLE_SET_PTR_H
#define GRPC_SRC_CORE_UTIL_SINGLE_SET_PTR_H

#include <grpc/support/port_platform.h>

#include <atomic>
#include <memory>

#include "absl/log/check.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

template <class T, class Deleter = std::default_delete<T>>
class SingleSetPtr {
 public:
  SingleSetPtr() = default;
  explicit SingleSetPtr(T* p) : p_{p} {}
  explicit SingleSetPtr(std::unique_ptr<T, Deleter> p) : p_{p.release()} {}
  ~SingleSetPtr() { Delete(p_.load(std::memory_order_relaxed)); }

  SingleSetPtr(const SingleSetPtr&) = delete;
  SingleSetPtr& operator=(const SingleSetPtr&) = delete;
  SingleSetPtr(SingleSetPtr&& other) noexcept
      : p_(other.p_.exchange(nullptr)) {}
  SingleSetPtr& operator=(SingleSetPtr&& other) noexcept {
    Set(other.p_.exchange(nullptr, std::memory_order_acq_rel));
    return *this;
  }

  // Set the pointer;
  // if already set, return the pre-set value and delete ptr;
  // if deleted, return nullptr and delete ptr.
  T* Set(T* ptr) {
    T* expected = nullptr;
    if (!p_.compare_exchange_strong(expected, ptr, std::memory_order_acq_rel,
                                    std::memory_order_acquire)) {
      Delete(ptr);
      return expected;
    }
    return ptr;
  }

  // Set the pointer from a compatible unique_ptr - with the same caveats as
  // above.
  T* Set(std::unique_ptr<T, Deleter> ptr) { return Set(ptr.release()); }

  // Clear the pointer.
  void Reset() { Delete(p_.exchange(nullptr, std::memory_order_acq_rel)); }

  bool is_set() const { return p_.load(std::memory_order_relaxed) != nullptr; }

  T* Get() const { return p_.load(std::memory_order_acquire); }

  T* operator->() const {
    T* p = Get();
    DCHECK_NE(p, nullptr);
    return p;
  }

  T& operator*() const { return *Get(); }

 private:
  static void Delete(T* p) {
    if (p == nullptr) return;
    Deleter()(p);
  }
  std::atomic<T*> p_{nullptr};
};

template <class T>
class SingleSetRefCountedPtr {
 public:
  SingleSetRefCountedPtr() = default;
  explicit SingleSetRefCountedPtr(RefCountedPtr<T> p) : p_{p.Release()} {}

  bool is_set() const { return p_.is_set(); }

  RefCountedPtr<T> GetOrCreate() {
    T* p = Get();
    if (p == nullptr) p = Set(MakeRefCounted<T>());
    return p->Ref();
  }
  T* Get() const { return p_.Get(); }

  T* operator->() const {
    T* p = Get();
    DCHECK_NE(p, nullptr);
    return p;
  }

  T& operator*() const { return *Get(); }

 private:
  struct UnrefDeleter {
    void operator()(T* p) { p->Unref(); }
  };

  T* Set(RefCountedPtr<T> p) { return p_.Set(p.release()); }

  SingleSetPtr<T, UnrefDeleter> p_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_SINGLE_SET_PTR_H
