/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_TYPES_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_TYPES_H_

#include <cstddef>
#include <cstdint>
#include <string>
#include <type_traits>
#include <utility>
#include <variant>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/dataframe/impl/bit_vector.h"
#include "src/trace_processor/dataframe/impl/flex_vector.h"
#include "src/trace_processor/dataframe/specs.h"
#include "src/trace_processor/dataframe/type_set.h"

namespace perfetto::trace_processor::dataframe::impl {

// Type categories for column content and operations.
// These define which operations can be applied to which content types.

// Set of content types that aren't string-based.
using NonStringType = TypeSet<Id, Uint32, Int32, Int64, Double>;

// Set of content types that are numeric in nature.
using IntegerOrDoubleType = TypeSet<Uint32, Int32, Int64, Double>;

// Set of operations applicable to non-null values.
using NonNullOp = TypeSet<Eq, Ne, Lt, Le, Gt, Ge, Glob, Regex>;

// Set of operations applicable to non-string values.
using NonStringOp = TypeSet<Eq, Ne, Lt, Le, Gt, Ge>;

// Set of operations applicable to string values.
using StringOp = TypeSet<Eq, Ne, Lt, Le, Gt, Ge, Glob, Regex>;

// Set of operations applicable to only string values.
using OnlyStringOp = TypeSet<Glob, Regex>;

// Set of operations applicable to ranges.
using RangeOp = TypeSet<Eq, Lt, Le, Gt, Ge>;

// Set of inequality operations (Lt, Le, Gt, Ge).
using InequalityOp = TypeSet<Lt, Le, Gt, Ge>;

// Set of null operations (IsNotNull, IsNull).
using NullOp = TypeSet<IsNotNull, IsNull>;

// Indicates an operation applies to both bounds of a range.
struct BothBounds {};

// Indicates an operation applies to the lower bound of a range.
struct BeginBound {};

// Indicates an operation applies to the upper bound of a range.
struct EndBound {};

// Which bounds should be modified by a range operation.
using BoundModifier = TypeSet<BothBounds, BeginBound, EndBound>;

// Represents a filter operation where we are performing an equality operation
// on a sorted column.
struct EqualRange {};

// Represents a filter operation where we are performing a lower bound operation
// on a sorted column.
struct LowerBound {};

// Represents a filter operation where we are performing an upper bound
// operation on a sorted column.
struct UpperBound {};

// Set of operations that can be applied to a sorted column.
using EqualRangeLowerBoundUpperBound =
    TypeSet<EqualRange, LowerBound, UpperBound>;

// Type tag indicating nulls should be placed at the start during
// partitioning/sorting.
struct NullsAtStart {};

// Type tag indicating nulls should be placed at the end during
// partitioning/sorting.
struct NullsAtEnd {};

// TypeSet defining the possible placement locations for nulls.
using NullsLocation = TypeSet<NullsAtStart, NullsAtEnd>;

// Type tag for finding the minimum value.
struct MinOp {};

// Type tag for finding the maximum value.
struct MaxOp {};

// TypeSet combining Min and Max operations.
using MinMaxOp = TypeSet<MinOp, MaxOp>;

// Storage implementation for column data. Provides physical storage
// for different types of column content.
class Storage {
 public:
  // Storage representation for Id columns.
  struct Id {
    uint32_t size;  // Number of rows in the column

    static const void* data() { return nullptr; }
  };
  using Uint32 = FlexVector<uint32_t>;
  using Int32 = FlexVector<int32_t>;
  using Int64 = FlexVector<int64_t>;
  using Double = FlexVector<double>;
  using String = FlexVector<StringPool::Id>;

  Storage(Storage::Id data) : type_(dataframe::Id{}), data_(data) {}
  Storage(Storage::Uint32 data)
      : type_(dataframe::Uint32{}), data_(std::move(data)) {}
  Storage(Storage::Int32 data)
      : type_(dataframe::Int32{}), data_(std::move(data)) {}
  Storage(Storage::Int64 data)
      : type_(dataframe::Int64{}), data_(std::move(data)) {}
  Storage(Storage::Double data)
      : type_(dataframe::Double{}), data_(std::move(data)) {}
  Storage(Storage::String data)
      : type_(dataframe::String{}), data_(std::move(data)) {}

  // Type-safe access to storage with unchecked variant access.
  template <typename T>
  auto& unchecked_get() {
    using U = StorageType::VariantTypeAtIndex<T, Variant>;
    return base::unchecked_get<U>(data_);
  }

  template <typename T>
  const auto& unchecked_get() const {
    using U = StorageType::VariantTypeAtIndex<T, Variant>;
    return base::unchecked_get<U>(data_);
  }

  // Get raw pointer to storage data for a specific type.
  template <typename T>
  auto* unchecked_data() {
    return unchecked_get<T>().data();
  }

  template <typename T>
  const auto* unchecked_data() const {
    return unchecked_get<T>().data();
  }

  // Returns a raw byte pointer to the underlying data.
  // Returns nullptr if the storage type is Id (which has no buffer).
  const uint8_t* byte_data() const {
    switch (type_.index()) {
      case StorageType::GetTypeIndex<dataframe::Id>():
        return nullptr;
      case StorageType::GetTypeIndex<dataframe::Uint32>():
        return reinterpret_cast<const uint8_t*>(
            base::unchecked_get<Storage::Uint32>(data_).data());
      case StorageType::GetTypeIndex<dataframe::Int32>():
        return reinterpret_cast<const uint8_t*>(
            base::unchecked_get<Storage::Int32>(data_).data());
      case StorageType::GetTypeIndex<dataframe::Int64>():
        return reinterpret_cast<const uint8_t*>(
            base::unchecked_get<Storage::Int64>(data_).data());
      case StorageType::GetTypeIndex<dataframe::Double>():
        return reinterpret_cast<const uint8_t*>(
            base::unchecked_get<Storage::Double>(data_).data());
      case StorageType::GetTypeIndex<dataframe::String>():
        return reinterpret_cast<const uint8_t*>(
            base::unchecked_get<Storage::String>(data_).data());
      default:
        PERFETTO_FATAL("Should not reach here");
    }
  }

  StorageType type() const { return type_; }

 private:
  // Variant containing all possible storage representations.
  using Variant = std::variant<Id, Uint32, Int32, Int64, Double, String>;
  StorageType type_;
  Variant data_;
};

// Stores any information about nulls in the column.
class NullStorage {
 private:
  template <typename T>
  static constexpr uint32_t TypeIndex() {
    return base::variant_index<Variant, T>();
  }

 public:
  // Used for non-null columns which don't need any storage for nulls.
  struct NonNull {};

  // Used for nullable columns where nulls do *not* reserve a slot in `Storage`.
  struct SparseNull {
    // 1 = non-null element in storage.
    // 0 = null with no corresponding entry in storage.
    BitVector bit_vector;
  };

  // Used for nullable columns where nulls reserve a slot in `Storage`.
  struct DenseNull {
    // 1 = non-null element in storage.
    // 0 = null with entry in storage with unspecified value
    BitVector bit_vector;
  };

  NullStorage(NonNull n) : nullability_(dataframe::NonNull{}), data_(n) {}
  NullStorage(SparseNull s)
      : nullability_(dataframe::SparseNull{}), data_(std::move(s)) {}
  NullStorage(DenseNull d)
      : nullability_(dataframe::DenseNull{}), data_(std::move(d)) {}

  // Type-safe unchecked access to variant data.
  template <typename T>
  T& unchecked_get() {
    return base::unchecked_get<T>(data_);
  }

  template <typename T>
  const T& unchecked_get() const {
    return base::unchecked_get<T>(data_);
  }

  BitVector& GetNullBitVector() {
    switch (data_.index()) {
      case TypeIndex<SparseNull>():
        return unchecked_get<SparseNull>().bit_vector;
      case TypeIndex<DenseNull>():
        return unchecked_get<DenseNull>().bit_vector;
      default:
        PERFETTO_FATAL("Unsupported overlay type");
    }
  }
  const BitVector& GetNullBitVector() const {
    switch (data_.index()) {
      case TypeIndex<SparseNull>():
        return unchecked_get<SparseNull>().bit_vector;
      case TypeIndex<DenseNull>():
        return unchecked_get<DenseNull>().bit_vector;
      default:
        PERFETTO_FATAL("Unsupported overlay type");
    }
  }

  Nullability nullability() const { return nullability_; }

 private:
  // Variant containing all possible overlay types.
  using Variant = std::variant<NonNull, SparseNull, DenseNull>;
  Nullability nullability_;
  Variant data_;
};

// Represents a complete column in the dataframe.
struct Column {
  Storage storage;
  NullStorage null_storage;
  SortState sort_state;
};

// Handle for referring to a filter value during query execution.
struct FilterValueHandle {
  uint32_t index;  // Index into the filter value array
};

// Result of casting a filter value for comparison during query execution.
struct CastFilterValueResult {
  enum Validity : uint8_t { kValid, kAllMatch, kNoneMatch };

  // Cast value for Id columns.
  struct Id {
    bool operator==(const Id& other) const { return value == other.value; }
    uint32_t value;
  };
  using Value =
      std::variant<Id, uint32_t, int32_t, int64_t, double, const char*>;

  bool operator==(const CastFilterValueResult& other) const {
    return validity == other.validity && value == other.value;
  }

  static constexpr CastFilterValueResult Valid(Value value) {
    return CastFilterValueResult{Validity::kValid, value};
  }

  static constexpr CastFilterValueResult NoneMatch() {
    return CastFilterValueResult{Validity::kNoneMatch, Id{0}};
  }

  static constexpr CastFilterValueResult AllMatch() {
    return CastFilterValueResult{Validity::kAllMatch, Id{0}};
  }

  // Status of the casting result.
  Validity validity;

  // Variant of all possible cast value types.
  Value value;
};

// Represents a contiguous range of indices [b, e).
// Used for efficient representation of sequential row indices.
struct Range {
  uint32_t b;  // Beginning index (inclusive)
  uint32_t e;  // Ending index (exclusive)

  // Get the number of elements in the range.
  size_t size() const { return e - b; }
  bool empty() const { return b == e; }
};

// Represents a contiguous sequence of elements of an arbitrary type T.
// Basically a very simple backport of std::span to C++17.
template <typename T>
struct Span {
  using value_type = T;
  using const_iterator = T*;

  T* b;
  T* e;

  Span(T* _b, T* _e) : b(_b), e(_e) {}

  T* begin() const { return b; }
  T* end() const { return e; }
  size_t size() const { return static_cast<size_t>(e - b); }
  bool empty() const { return b == e; }
};

}  // namespace perfetto::trace_processor::dataframe::impl

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_TYPES_H_
