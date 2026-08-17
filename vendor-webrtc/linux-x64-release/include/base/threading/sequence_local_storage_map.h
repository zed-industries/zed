// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SEQUENCE_LOCAL_STORAGE_MAP_H_
#define BASE_THREADING_SEQUENCE_LOCAL_STORAGE_MAP_H_

#include <variant>

#include "base/auto_reset.h"
#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/flat_map.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "third_party/abseil-cpp/absl/meta/type_traits.h"

namespace base {
namespace internal {

// A SequenceLocalStorageMap holds (slot_id) -> (value, destructor) items for a
// sequence. When a task runs, it is expected that a pointer to its sequence's
// SequenceLocalStorageMap is set in TLS using
// ScopedSetSequenceLocalStorageMapForCurrentThread. When a
// SequenceLocalStorageMap is destroyed, it invokes the destructors associated
// with values stored within it.
// The Get() and Set() methods should not be accessed directly.
// Use SequenceLocalStorageSlot to Get() and Set() values in the current
// sequence's SequenceLocalStorageMap.
class BASE_EXPORT SequenceLocalStorageMap {
 public:
  SequenceLocalStorageMap();

  SequenceLocalStorageMap(const SequenceLocalStorageMap&) = delete;
  SequenceLocalStorageMap& operator=(const SequenceLocalStorageMap&) = delete;

  ~SequenceLocalStorageMap();

  // Returns the SequenceLocalStorage bound to the current thread. It is invalid
  // to call this outside the scope of a
  // ScopedSetSequenceLocalStorageForCurrentThread.
  static SequenceLocalStorageMap& GetForCurrentThread();

  // Indicates whether the current thread has a SequenceLocalStorageMap
  // available and thus whether it can safely call GetForCurrentThread and
  // dereference SequenceLocalStorageSlots.
  static bool IsSetForCurrentThread();

  // A `Value` holds an `ExternalValue` or an `InlineValue`. `InlineValue` is
  // most efficient, but can only be used with types that have a size and an
  // alignment smaller than a pointer and are trivially relocatable.
  struct BASE_EXPORT ExternalValue {
    // `value_` is not a raw_ptr<...> for performance reasons
    // (based on analysis of sampling profiler data and tab_search:top100:2020).
    RAW_PTR_EXCLUSION void* value;

    template <class T>
    void emplace(T* ptr) {
      value = static_cast<void*>(ptr);
    }

    template <class T, class Deleter>
    void Destroy() {
      Deleter()(std::addressof(value_as<T>()));
    }

    template <typename T>
    T& value_as() LIFETIME_BOUND {
      return *static_cast<T*>(value);
    }

    template <typename T>
    const T& value_as() const LIFETIME_BOUND {
      return *static_cast<const T*>(value);
    }
  };

  struct BASE_EXPORT alignas(sizeof(void*)) InlineValue {
    // Holds a T if small.
    char bytes[sizeof(void*)];

    template <class T, class... Args>
    void emplace(Args&&... args) {
      static_assert(sizeof(T) <= sizeof(void*),
                    "Type T is too big for storage inline.");
      static_assert(absl::is_trivially_relocatable<T>(),
                    "T doesn't qualify as trivially relocatable, which "
                    "precludes it from storage inline.");
      static_assert(std::alignment_of<T>::value <= sizeof(T),
                    "Type T has alignment requirements that preclude its "
                    "storage inline.");
      new (&bytes) T(std::forward<Args>(args)...);
    }

    template <class T>
    void Destroy() {
      value_as<T>().~T();
    }

    template <typename T>
    T& value_as() {
      return *reinterpret_cast<T*>(bytes);
    }

    template <typename T>
    const T& value_as() const {
      return *reinterpret_cast<const T*>(bytes);
    }
  };

  // There's no need for a tagged union (std::variant) since the value
  // type is implicitly determined by T being stored.
  union Value {
    ExternalValue external_value;
    InlineValue inline_value;
  };

  using DestructorFunc = void(Value*);

  template <class T, class Deleter>
  static DestructorFunc* MakeExternalDestructor() {
    return [](Value* value) { value->external_value.Destroy<T, Deleter>(); };
  }
  template <class T>
  static DestructorFunc* MakeInlineDestructor() {
    return [](Value* value) { value->inline_value.Destroy<T>(); };
  }

  // Holds a value alongside its destructor. Calls the destructor on the
  // value upon destruction.
  class BASE_EXPORT ValueDestructorPair {
   public:
    ValueDestructorPair();
    ValueDestructorPair(ExternalValue value, DestructorFunc* destructor);
    ValueDestructorPair(InlineValue value, DestructorFunc* destructor);

    ValueDestructorPair(const ValueDestructorPair&) = delete;
    ValueDestructorPair& operator=(const ValueDestructorPair&) = delete;

    ~ValueDestructorPair();

    ValueDestructorPair(ValueDestructorPair&& value_destructor_pair);

    ValueDestructorPair& operator=(ValueDestructorPair&& value_destructor_pair);

    explicit operator bool() const;

    Value* get() { return destructor_ != nullptr ? &value_ : nullptr; }
    const Value* get() const {
      return destructor_ != nullptr ? &value_ : nullptr;
    }

    Value* operator->() { return get(); }
    const Value* operator->() const { return get(); }

   private:
    Value value_;
    // `destructor_` is not a raw_ptr<...> for performance reasons
    // (based on analysis of sampling profiler data and tab_search:top100:2020).
    RAW_PTR_EXCLUSION DestructorFunc* destructor_;
  };

  // Returns true if a value is stored in |slot_id|.
  bool Has(int slot_id) const;

  // Resets the value stored in |slot_id|.
  void Reset(int slot_id);

  // Returns the value stored in |slot_id| or nullptr if no value was stored.
  Value* Get(int slot_id);

  // Stores |value_destructor_pair| in |slot_id|. Overwrites and destroys any
  // previously stored value.
  Value* Set(int slot_id, ValueDestructorPair value_destructor_pair);

 private:
  // Map from slot id to ValueDestructorPair.
  // flat_map was chosen because there are expected to be relatively few entries
  // in the map. For low number of entries, flat_map is known to perform better
  // than other map implementations.
  base::flat_map<int, ValueDestructorPair> sls_map_;
};

// Within the scope of this object,
// SequenceLocalStorageMap::GetForCurrentThread() will return a reference to the
// SequenceLocalStorageMap object passed to the constructor. There can be only
// one ScopedSetSequenceLocalStorageMapForCurrentThread instance per scope.
class BASE_EXPORT
    [[maybe_unused,
      nodiscard]] ScopedSetSequenceLocalStorageMapForCurrentThread {
 public:
  ScopedSetSequenceLocalStorageMapForCurrentThread(
      SequenceLocalStorageMap* sequence_local_storage);

  ScopedSetSequenceLocalStorageMapForCurrentThread(
      const ScopedSetSequenceLocalStorageMapForCurrentThread&) = delete;
  ScopedSetSequenceLocalStorageMapForCurrentThread& operator=(
      const ScopedSetSequenceLocalStorageMapForCurrentThread&) = delete;

  ~ScopedSetSequenceLocalStorageMapForCurrentThread();

 private:
  const base::AutoReset<SequenceLocalStorageMap*> resetter_;
};
}  // namespace internal
}  // namespace base

#endif  // BASE_THREADING_SEQUENCE_LOCAL_STORAGE_MAP_H_
