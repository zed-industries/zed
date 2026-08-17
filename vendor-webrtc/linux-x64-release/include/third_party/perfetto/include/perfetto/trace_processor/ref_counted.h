/*
 * Copyright (C) 2018 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_REF_COUNTED_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_REF_COUNTED_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <utility>

#include "perfetto/base/logging.h"

// A non-thread-safe refcounted implementation.
// Unlike std::shared_ptr The target class needs to explicitly derive
// RefCounted.

// Usage:
// class MyRefcountedThing : public RefCounted {};
// ...
// RefPtr<MyRefcountedThing> shareable_ptr(new MyRefcountedThing());
// auto copy = shareable_ptr;

namespace perfetto {
namespace trace_processor {

// The base class that refcounted classes should inherit.
class RefCounted {
 public:
  RefCounted() = default;
  RefCounted(RefCounted&&) noexcept = default;
  RefCounted& operator=(RefCounted&&) noexcept = default;
  RefCounted(const RefCounted&) = delete;
  RefCounted& operator=(const RefCounted&) = delete;

 private:
  template <typename T>
  friend class RefPtr;

  void AddRef() const {
    PERFETTO_DCHECK(refcount_ >= 0);
    ++refcount_;
  }
  bool Release() const {
    PERFETTO_DCHECK(refcount_ > 0);
    return --refcount_ == 0;
  }

  mutable intptr_t refcount_ = 0;
};

// The RAII smart-pointer.
template <typename T>
class RefPtr {
 public:
  static_assert(std::is_base_of<RefCounted, T>::value,
                "T must be a descendant of RefCounted");

  // Adopt a newly created object.
  RefPtr() : ptr_(nullptr) {}
  explicit RefPtr(T* ptr) : ptr_(ptr) {
    if (ptr_)
      ptr_->AddRef();
  }

  ~RefPtr() { reset(); }

  void reset() {
    auto* old_ptr = ptr_;
    ptr_ = nullptr;
    if (old_ptr && old_ptr->Release())
      delete old_ptr;
  }

  // This case is really the move-assignment operator=(&&).
  void reset(T* new_obj) { *this = RefPtr<T>(new_obj); }

  // Releases the pointer owned by this RefPtr *without* decrementing the
  // refcount. Callers *must* call |FromReleasedUnsafe| at a later date with
  // this pointer to avoid memory leaks.
  T* ReleaseUnsafe() {
    PERFETTO_DCHECK(ptr_->refcount_ > 0);
    auto* old_ptr = ptr_;
    ptr_ = nullptr;
    return old_ptr;
  }

  // Creates a RefPtr from a pointer returned by |ReleaseUnsafe|. Passing a
  // pointer from any other source results in undefined behaviour.
  static RefPtr<T> FromReleasedUnsafe(T* ptr) {
    PERFETTO_DCHECK(ptr->refcount_ > 0);
    RefPtr<T> res;
    res.ptr_ = ptr;
    return res;
  }

  // Move operators.
  RefPtr(RefPtr&& move_from) noexcept {
    ptr_ = move_from.ptr_;
    move_from.ptr_ = nullptr;
  }

  RefPtr& operator=(RefPtr&& move_from) noexcept {
    this->~RefPtr();
    new (this) RefPtr(std::move(move_from));
    return *this;
  }

  // Copy operators.
  RefPtr(const RefPtr& copy_from) {
    ptr_ = copy_from.ptr_;
    if (ptr_)
      ptr_->AddRef();
  }

  RefPtr& operator=(const RefPtr& copy_from) {
    if (this != &copy_from) {
      this->~RefPtr();
      new (this) RefPtr(copy_from);
    }
    return *this;
  }

  template <typename U>
  bool operator==(const RefPtr<U>& rhs) const {
    return ptr_ == rhs.ptr_;
  }
  template <typename U>
  bool operator!=(const RefPtr<U>& rhs) const {
    return !(*this == rhs);
  }

  bool operator==(std::nullptr_t) const noexcept { return ptr_ == nullptr; }
  bool operator!=(std::nullptr_t) const noexcept { return ptr_ != nullptr; }

  T* get() const { return ptr_; }
  T* operator->() const { return ptr_; }
  T& operator*() const { return *ptr_; }
  explicit operator bool() const { return ptr_ != nullptr; }

 private:
  T* ptr_ = nullptr;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_REF_COUNTED_H_
