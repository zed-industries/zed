// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SEQUENCE_LOCAL_STORAGE_SLOT_H_
#define BASE_THREADING_SEQUENCE_LOCAL_STORAGE_SLOT_H_

#include <memory>
#include <type_traits>
#include <utility>

#include "base/base_export.h"
#include "base/threading/sequence_local_storage_map.h"
#include "third_party/abseil-cpp/absl/meta/type_traits.h"

namespace base {

namespace internal {
BASE_EXPORT int GetNextSequenceLocalStorageSlotNumber();
}

// SequenceLocalStorageSlot allows arbitrary values to be stored and retrieved
// from a sequence. Values are deleted when the sequence is deleted.
//
// Example usage:
//
// int& GetSequenceLocalStorage()
//     static SequenceLocalStorageSlot<int> sls_value;
//     return sls_value->GetOrCreateValue();
// }
//
// void Read() {
//   int value = GetSequenceLocalStorage();
//   ...
// }
//
// void Write() {
//   GetSequenceLocalStorage() = 42;
// }
//
// void PostTasks() {
//   // Since Read() runs on the same sequence as Write(), it
//   // will read the value "42". A Read() running on a different
//   // sequence would not see that value.
//   scoped_refptr<base::SequencedTaskRunner> task_runner = ...;
//   task_runner->PostTask(FROM_HERE, base::BindOnce(&Write));
//   task_runner->PostTask(FROM_HERE, base::BindOnce(&Read));
// }
//
// SequenceLocalStorageSlot must be used within the scope of a
// ScopedSetSequenceLocalStorageMapForCurrentThread object.
// Note: this is true on all ThreadPool workers and on threads bound to a
// MessageLoop.
// SequenceLocalStorageSlot is implemented by either [Generic/Small]
// variants depending on the type. SequenceLocalStorageSlot itself
// doesn't support forward declared types and thus the variant
// [Generic/Small] needs to be specified explicitly.

// Generic implementation for SequenceLocalStorageSlot.
template <typename T, typename Deleter = std::default_delete<T>>
class GenericSequenceLocalStorageSlot {
 public:
  GenericSequenceLocalStorageSlot()
      : slot_id_(internal::GetNextSequenceLocalStorageSlotNumber()) {}

  GenericSequenceLocalStorageSlot(const GenericSequenceLocalStorageSlot&) =
      delete;
  GenericSequenceLocalStorageSlot& operator=(
      const GenericSequenceLocalStorageSlot&) = delete;

  ~GenericSequenceLocalStorageSlot() = default;

  explicit operator bool() const {
    return internal::SequenceLocalStorageMap::GetForCurrentThread().Has(
        slot_id_);
  }

  // Default-constructs the value for the current sequence if not
  // already constructed. Then, returns the value.
  T& GetOrCreateValue() {
    auto* slot =
        internal::SequenceLocalStorageMap::GetForCurrentThread().Get(slot_id_);
    if (!slot) {
      return emplace();
    }
    return slot->external_value.value_as<T>();
  }

  // Returns a pointer to the value for the current sequence. May be
  // nullptr if the value was not constructed on the current sequence.
  T* GetValuePointer() {
    auto* value =
        internal::SequenceLocalStorageMap::GetForCurrentThread().Get(slot_id_);
    if (value) {
      return std::addressof(value->external_value.value_as<T>());
    }
    return nullptr;
  }
  const T* GetValuePointer() const {
    return const_cast<GenericSequenceLocalStorageSlot*>(this)
        ->GetValuePointer();
  }

  T* operator->() { return GetValuePointer(); }
  const T* operator->() const { return GetValuePointer(); }

  T& operator*() { return *GetValuePointer(); }
  const T& operator*() const { return *GetValuePointer(); }

  void reset() {
    internal::SequenceLocalStorageMap::GetForCurrentThread().Reset(slot_id_);
  }

  // Constructs this slot's sequence-local value with |args...| and returns a
  // pointer to the created object.
  template <class... Args>
  T& emplace(Args&&... args) {
    T* value_ptr = new T(std::forward<Args>(args)...);
    Adopt(value_ptr);
    return *value_ptr;
  }

 private:
  // Takes ownership of |value_ptr|.
  void Adopt(T* value_ptr) {
    // Since SequenceLocalStorageMap needs to store values of various types
    // within the same map, the type of value_destructor_pair.value is void*
    // (std::unique_ptr<void> is invalid). Memory is freed by calling
    // |value_destructor_pair.destructor| in the destructor of
    // ValueDestructorPair which is invoked when the value is overwritten by
    // another call to SequenceLocalStorageMap::Set or when the
    // SequenceLocalStorageMap is deleted.
    internal::SequenceLocalStorageMap::ExternalValue value;
    value.emplace(value_ptr);
    internal::SequenceLocalStorageMap::ValueDestructorPair
        value_destructor_pair(
            std::move(value),
            internal::SequenceLocalStorageMap::MakeExternalDestructor<
                T, Deleter>());

    internal::SequenceLocalStorageMap::GetForCurrentThread().Set(
        slot_id_, std::move(value_destructor_pair));
  }

  // |slot_id_| is used as a key in SequenceLocalStorageMap
  const int slot_id_;
};

// Implementation for SequenceLocalStorageSlot optimized for small and trivial
// objects.
template <class T>
class SmallSequenceLocalStorageSlot {
 public:
  SmallSequenceLocalStorageSlot()
      : slot_id_(internal::GetNextSequenceLocalStorageSlotNumber()) {}

  SmallSequenceLocalStorageSlot(const SmallSequenceLocalStorageSlot&) = delete;
  SmallSequenceLocalStorageSlot& operator=(
      const SmallSequenceLocalStorageSlot&) = delete;

  ~SmallSequenceLocalStorageSlot() = default;

  explicit operator bool() const {
    return internal::SequenceLocalStorageMap::GetForCurrentThread().Has(
        slot_id_);
  }

  // Default-constructs the value for the current sequence if not
  // already constructed. Then, returns the value.
  T& GetOrCreateValue() {
    auto* slot =
        internal::SequenceLocalStorageMap::GetForCurrentThread().Get(slot_id_);
    if (!slot) {
      return emplace();
    }
    return slot->inline_value.value_as<T>();
  }

  // Returns a pointer to the value for the current sequence. May be
  // nullptr if the value was not constructed on the current sequence.
  T* GetValuePointer() {
    auto* slot =
        internal::SequenceLocalStorageMap::GetForCurrentThread().Get(slot_id_);
    if (!slot) {
      return nullptr;
    }
    return &slot->inline_value.value_as<T>();
  }
  const T* GetValuePointer() const {
    return const_cast<SmallSequenceLocalStorageSlot*>(this)->GetValuePointer();
  }

  T* operator->() { return GetValuePointer(); }
  const T* operator->() const { return GetValuePointer(); }

  T& operator*() { return *GetValuePointer(); }
  const T& operator*() const { return *GetValuePointer(); }

  void reset() {
    internal::SequenceLocalStorageMap::GetForCurrentThread().Reset(slot_id_);
  }

  // Constructs this slot's sequence-local value with |args...| and returns a
  // pointer to the created object.
  template <class... Args>
  T& emplace(Args&&... args) {
    internal::SequenceLocalStorageMap::InlineValue value;
    value.emplace<T>(std::forward<Args>(args)...);
    internal::SequenceLocalStorageMap::ValueDestructorPair
        value_destructor_pair(
            std::move(value),
            internal::SequenceLocalStorageMap::MakeInlineDestructor<T>());

    return internal::SequenceLocalStorageMap::GetForCurrentThread()
        .Set(slot_id_, std::move(value_destructor_pair))
        ->inline_value.value_as<T>();
  }

 private:
  // |slot_id_| is used as a key in SequenceLocalStorageMap
  const int slot_id_;
};

template <typename T,
          typename Deleter = std::default_delete<T>,
          bool IsSmall =
              sizeof(T) <= sizeof(void*) && absl::is_trivially_relocatable<T>()>
struct SequenceLocalStorageSlot;

template <typename T, typename Deleter>
struct SequenceLocalStorageSlot<T, Deleter, false>
    : GenericSequenceLocalStorageSlot<T, Deleter> {};

template <typename T>
struct SequenceLocalStorageSlot<T, std::default_delete<T>, true>
    : SmallSequenceLocalStorageSlot<T> {};

}  // namespace base
#endif  // BASE_THREADING_SEQUENCE_LOCAL_STORAGE_SLOT_H_
