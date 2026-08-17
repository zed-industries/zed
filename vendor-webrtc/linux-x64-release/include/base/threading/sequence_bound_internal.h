// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SEQUENCE_BOUND_INTERNAL_H_
#define BASE_THREADING_SEQUENCE_BOUND_INTERNAL_H_

#include <memory>
#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/location.h"
#include "base/memory/aligned_memory.h"
#include "base/memory/raw_ptr.h"
#include "base/task/sequenced_task_runner.h"

namespace base::sequence_bound_internal {

struct CrossThreadTraits {
  template <typename Signature>
  using CrossThreadTask = OnceCallback<Signature>;

  template <typename Functor, typename... Args>
  static inline auto BindOnce(Functor&& functor, Args&&... args) {
    return ::base::BindOnce(std::forward<Functor>(functor),
                            std::forward<Args>(args)...);
  }

  template <typename T>
  static inline auto Unretained(T ptr) {
    return ::base::Unretained(ptr);
  }

  static inline bool PostTask(SequencedTaskRunner& task_runner,
                              const Location& location,
                              OnceClosure&& task) {
    return task_runner.PostTask(location, std::move(task));
  }

  static inline bool PostTaskAndReply(SequencedTaskRunner& task_runner,
                                      const Location& location,
                                      OnceClosure&& task,
                                      OnceClosure&& reply) {
    return task_runner.PostTaskAndReply(location, std::move(task),
                                        std::move(reply));
  }

  template <typename TaskReturnType, typename ReplyArgType>
  static inline bool PostTaskAndReplyWithResult(
      SequencedTaskRunner& task_runner,
      const Location& location,
      OnceCallback<TaskReturnType()>&& task,
      OnceCallback<void(ReplyArgType)>&& reply) {
    return task_runner.PostTaskAndReplyWithResult(location, std::move(task),
                                                  std::move(reply));
  }

  // Accept RepeatingCallback here since it's convertible to a OnceCallback.
  template <template <typename> class CallbackType>
  static constexpr bool IsCrossThreadTask =
      IsBaseCallback<CallbackType<void()>>;
};

template <typename T, typename CrossThreadTraits>
class Storage {
 public:
  using element_type = T;

  bool is_null() const { return ptr_ == nullptr; }
  auto GetPtrForBind() const { return CrossThreadTraits::Unretained(ptr_); }
  auto GetRefForBind() const { return std::cref(*ptr_); }

  // Marked NO_SANITIZE because cfi doesn't like casting uninitialized memory to
  // `T*`. However, this is safe here because:
  //
  // 1. The cast is well-defined (see https://eel.is/c++draft/basic.life#6) and
  // 2. The resulting pointer is only ever dereferenced on `task_runner`.
  //    By the time SequenceBound's constructor returns, the task to construct
  //    `T` will already be posted; thus, subsequent dereference of `ptr_` on
  //    `task_runner` are safe.
  template <typename... Args>
  NO_SANITIZE("cfi-unrelated-cast")
  void Construct(SequencedTaskRunner& task_runner, Args&&... args) {
    // TODO(crbug.com/40245687): Use universal forwarding and assert that
    // T is constructible from args for better error messages.
    DCHECK(!alloc_);
    DCHECK(!ptr_);

    // Allocate space for but do not construct an instance of `T`.
    // AlignedAlloc() requires alignment be a multiple of sizeof(void*).
    alloc_ = AlignedAlloc(
        sizeof(T), sizeof(void*) > alignof(T) ? sizeof(void*) : alignof(T));
    ptr_ = reinterpret_cast<T*>(alloc_.get());

    // Ensure that `ptr_` will be initialized.
    CrossThreadTraits::PostTask(
        task_runner, FROM_HERE,
        CrossThreadTraits::BindOnce(&InternalConstruct<Args...>,
                                    CrossThreadTraits::Unretained(ptr_),
                                    std::forward<Args>(args)...));
  }

  // Marked NO_SANITIZE since:
  // 1. SequenceBound can be moved before `ptr_` is constructed on its managing
  //    `SequencedTaskRunner` but
  // 2. Implicit conversions to non-virtual base classes are allowed before the
  //    lifetime of the object that `ptr_` points at has begun (see
  //    https://eel.is/c++draft/basic.life#6).
  template <typename U>
  NO_SANITIZE("cfi-unrelated-cast")
  void TakeFrom(Storage<U, CrossThreadTraits>&& other) {
    // Subtle: this must not use static_cast<>, since the lifetime of the
    // managed `T` may not have begun yet. However, the standard explicitly
    // still allows implicit conversion to a non-virtual base class.
    ptr_ = std::exchange(other.ptr_, nullptr);
    alloc_ = std::exchange(other.alloc_, nullptr);
  }

  void Destruct(SequencedTaskRunner& task_runner) {
    CrossThreadTraits::PostTask(
        task_runner, FROM_HERE,
        CrossThreadTraits::BindOnce(
            &InternalDestruct,
            CrossThreadTraits::Unretained(std::exchange(ptr_, nullptr)),
            CrossThreadTraits::Unretained(std::exchange(alloc_, nullptr))));
  }

 private:
  // Needed to allow conversions from compatible `U`s.
  template <typename U, typename V>
  friend class Storage;

  // Helpers for constructing and destroying `T` on its managing
  // `SequencedTaskRunner`.
  template <typename... Args>
  static void InternalConstruct(T* ptr, std::decay_t<Args>&&... args) {
    new (ptr) T(std::move(args)...);
  }

  static void InternalDestruct(T* ptr, void* alloc) {
    ptr->~T();
    AlignedFree(alloc);
  }

  // Pointer to the managed `T`.
  raw_ptr<T> ptr_ = nullptr;

  // Storage originally allocated by `AlignedAlloc()`. Maintained separately
  // from  `ptr_` since the original, unadjusted pointer needs to be passed to
  // `AlignedFree()`.
  raw_ptr<void> alloc_ = nullptr;
};

template <typename T, typename CrossThreadTraits>
struct Storage<std::unique_ptr<T>, CrossThreadTraits> {
 public:
  using element_type = T;

  bool is_null() const { return ptr_ == nullptr; }
  auto GetPtrForBind() const { return CrossThreadTraits::Unretained(ptr_); }
  auto GetRefForBind() const { return std::cref(*ptr_); }

  template <typename U>
  void Construct(SequencedTaskRunner& task_runner, std::unique_ptr<U> arg) {
    // TODO(crbug.com/40245687): Use universal forwarding and assert that
    // there is one arg that is a unique_ptr for better error messages.
    DCHECK(!ptr_);

    ptr_ = arg.release();
    // No additional storage needs to be allocated since `T` is already
    // constructed and lives on the heap.
  }

  template <typename U>
  void TakeFrom(Storage<std::unique_ptr<U>, CrossThreadTraits>&& other) {
    ptr_ = std::exchange(other.ptr_, nullptr);
  }

  void Destruct(SequencedTaskRunner& task_runner) {
    CrossThreadTraits::PostTask(
        task_runner, FROM_HERE,
        CrossThreadTraits::BindOnce(
            &InternalDestruct,
            CrossThreadTraits::Unretained(std::exchange(ptr_, nullptr))));
  }

 private:
  // Needed to allow conversions from compatible `U`s.
  template <typename U, typename V>
  friend class Storage;

  static void InternalDestruct(T* ptr) { delete ptr; }

  // Pointer to the heap-allocated `T`.
  raw_ptr<T> ptr_ = nullptr;
};

}  // namespace base::sequence_bound_internal

#endif  // BASE_THREADING_SEQUENCE_BOUND_INTERNAL_H_
