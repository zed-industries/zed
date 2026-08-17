// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_ANY_H_
#define FUZZTEST_FUZZTEST_INTERNAL_ANY_H_

#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>

#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"

namespace fuzztest::internal {

// Base class for both implementations of Any below, and should not be used
// directly. The caller to the constructor decides if they want a copy operation
// or not.
class AnyBase {
 public:
  template <typename T>
  explicit AnyBase(std::in_place_t, std::true_type, T* value) {
    static constexpr VTable kVTable = {type_id<T>, DestroyImpl<T>, CopyImpl<T>};
    vtable_ = &kVTable;
    value_ = const_cast<void*>(static_cast<const void*>(value));
  }
  template <typename T>
  explicit AnyBase(std::in_place_t, std::false_type, T* value) {
    static constexpr VTable kVTable = {type_id<T>, DestroyImpl<T>, nullptr};
    vtable_ = &kVTable;
    value_ = const_cast<void*>(static_cast<const void*>(value));
  }

  bool has_value() const {
    FUZZTEST_INTERNAL_CHECK((vtable_ == nullptr) == (value_ == nullptr),
                            "Inconsistent state between value and vtable.");
    return value_ != nullptr;
  }

  template <typename T>
  bool Has() const {
    return has_value() && vtable_->type_id == type_id<T>;
  }

  template <typename T>
  T& GetAs() & {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(has_value(), "Object is empty!");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(Has<T>(), "Wrong type!");
    return *static_cast<T*>(value_);
  }

  template <typename T>
  const T& GetAs() const& {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(has_value(), "Object is empty!");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(Has<T>(), "Wrong type!");
    return *static_cast<T*>(value_);
  }

  template <typename T>
  T&& GetAs() && {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(has_value(), "Object is empty!");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(Has<T>(), "Wrong type!");
    return std::move(*static_cast<T*>(value_));
  }

  template <typename T>
  const T&& GetAs() const&& {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(has_value(), "Object is empty!");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(Has<T>(), "Wrong type!");
    return std::move(*static_cast<const T*>(value_));
  }

 protected:
  AnyBase() : vtable_(nullptr), value_(nullptr) {}

  AnyBase(AnyBase&& other) noexcept
      : vtable_(std::exchange(other.vtable_, nullptr)),
        value_(std::exchange(other.value_, nullptr)) {}

  AnyBase& operator=(AnyBase&& other) {
    if (this == &other) return *this;
    Destroy();
    vtable_ = std::exchange(other.vtable_, nullptr);
    value_ = std::exchange(other.value_, nullptr);
    return *this;
  }

  ~AnyBase() { Destroy(); }

  void Destroy() {
    if (has_value()) vtable_->destroy(value_);
  }

  void CopyFrom(const AnyBase& other) {
    FUZZTEST_INTERNAL_CHECK(!has_value(), "CopyFrom called on a full object");
    if (other.has_value()) {
      vtable_ = other.vtable_;
      value_ = vtable_->copy(other.value_);
    }
  }

 private:
  struct VTable {
    TypeId type_id;
    void (*destroy)(void*);
    void* (*copy)(void*);
  };

  template <typename T>
  static void DestroyImpl(void* p) {
    delete static_cast<T*>(p);
  }

  template <typename T>
  static void* CopyImpl(void* p) {
    return new T(*static_cast<T*>(p));
  }

  const VTable* vtable_;
  void* value_;
};

// These classes are similar to `std::any` but we implement our own because:
//  - We need a move only implementation of it for certain cases (MoveOnlyAny),
//    and `std::any` requires copyability.
//  - The implicit conversions of `std::any` are dangerous and easy to use
//    wrong. The conversions here are explicit and require an
//    `std::in_place_type<T>` tag.
//  - `std::any` causes compile time issues when mixed with other generic
//    types like std::pair/std::tuple/etc, because of their aggressive SFINAE
//    checks.
//
//  To access the object you use Has<T>()/GetAs<T>() instead of any_cast.
class MoveOnlyAny : private AnyBase {
 public:
  template <typename T, typename... U>
  explicit MoveOnlyAny(std::in_place_type_t<T>, U&&... args)
      : AnyBase(std::in_place, std::false_type{},
                new T(std::forward<U>(args)...)) {}

  MoveOnlyAny() = default;
  MoveOnlyAny(const MoveOnlyAny& other) = delete;
  MoveOnlyAny(MoveOnlyAny&& other) = default;
  MoveOnlyAny& operator=(const MoveOnlyAny& other) = delete;
  MoveOnlyAny& operator=(MoveOnlyAny&& other) = default;
  ~MoveOnlyAny() = default;

  using AnyBase::GetAs;
  using AnyBase::Has;
  using AnyBase::has_value;
};

class CopyableAny : private AnyBase {
 public:
  template <typename T, typename... U>
  explicit CopyableAny(std::in_place_type_t<T>, U&&... args)
      : AnyBase(std::in_place, std::true_type{},
                new T(std::forward<U>(args)...)) {}

  CopyableAny() = default;
  CopyableAny(const CopyableAny& other) { CopyFrom(other); }
  CopyableAny(CopyableAny&& other) = default;
  CopyableAny& operator=(const CopyableAny& other) {
    *this = CopyableAny(other);
    return *this;
  }
  CopyableAny& operator=(CopyableAny&& other) = default;
  ~CopyableAny() = default;

  using AnyBase::GetAs;
  using AnyBase::Has;
  using AnyBase::has_value;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_ANY_H_
