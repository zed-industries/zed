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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_INTERPRETER_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_INTERPRETER_H_

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <cstring>

#include <functional>
#include <limits>
#include <numeric>
#include <optional>
#include <string_view>
#include <type_traits>
#include <unordered_set>
#include <utility>
#include <variant>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/containers/null_term_string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/dataframe/impl/bit_vector.h"
#include "src/trace_processor/dataframe/impl/bytecode_core.h"
#include "src/trace_processor/dataframe/impl/bytecode_instructions.h"
#include "src/trace_processor/dataframe/impl/bytecode_registers.h"
#include "src/trace_processor/dataframe/impl/slab.h"
#include "src/trace_processor/dataframe/impl/types.h"
#include "src/trace_processor/dataframe/specs.h"
#include "src/trace_processor/dataframe/value_fetcher.h"
#include "src/trace_processor/util/glob.h"
#include "src/trace_processor/util/regex.h"

namespace perfetto::trace_processor::dataframe::impl::bytecode {
namespace comparators {

// Returns an appropriate comparator functor for the given integer/double type
// and operation. Currently only supports equality comparison.
template <typename T, typename Op>
auto IntegerOrDoubleComparator() {
  if constexpr (std::is_same_v<Op, Eq>) {
    return std::equal_to<T>();
  } else if constexpr (std::is_same_v<Op, Ne>) {
    return std::not_equal_to<T>();
  } else if constexpr (std::is_same_v<Op, Lt>) {
    return std::less<T>();
  } else if constexpr (std::is_same_v<Op, Le>) {
    return std::less_equal<T>();
  } else if constexpr (std::is_same_v<Op, Gt>) {
    return std::greater<T>();
  } else if constexpr (std::is_same_v<Op, Ge>) {
    return std::greater_equal<T>();
  } else {
    static_assert(std::is_same_v<Op, Eq>, "Unsupported op");
  }
}

template <typename T>
struct StringComparator {
  bool operator()(StringPool::Id lhs, NullTermStringView rhs) const {
    if constexpr (std::is_same_v<T, Lt>) {
      return pool_->Get(lhs) < rhs;
    } else if constexpr (std::is_same_v<T, Le>) {
      return pool_->Get(lhs) <= rhs;
    } else if constexpr (std::is_same_v<T, Gt>) {
      return pool_->Get(lhs) > rhs;
    } else if constexpr (std::is_same_v<T, Ge>) {
      return pool_->Get(lhs) >= rhs;
    } else {
      static_assert(std::is_same_v<T, Lt>, "Unsupported op");
    }
  }
  const StringPool* pool_;
};
struct StringLessInvert {
  bool operator()(NullTermStringView lhs, StringPool::Id rhs) const {
    return lhs < pool_->Get(rhs);
  }
  const StringPool* pool_;
};
struct Glob {
  bool operator()(StringPool::Id lhs, const util::GlobMatcher& matcher) const {
    return matcher.Matches(pool_->Get(lhs));
  }
  const StringPool* pool_;
};
struct GlobFullStringPool {
  bool operator()(StringPool::Id lhs, const BitVector& matches) const {
    return matches.is_set(lhs.raw_id());
  }
};
struct Regex {
  bool operator()(StringPool::Id lhs, const regex::Regex& pattern) const {
    return pattern.Search(pool_->Get(lhs).c_str());
  }
  const StringPool* pool_;
};

}  // namespace comparators

// Handles invalid cast filter value results for filtering operations.
// If the cast result is invalid, updates the range or span accordingly.
//
// Returns true if the result is valid, false otherwise.
template <typename T>
PERFETTO_ALWAYS_INLINE bool HandleInvalidCastFilterValueResult(
    const CastFilterValueResult& value,
    T& update) {
  static_assert(std::is_same_v<T, Range> || std::is_same_v<T, Span<uint32_t>>);
  if (PERFETTO_UNLIKELY(value.validity != CastFilterValueResult::kValid)) {
    if (value.validity == CastFilterValueResult::kNoneMatch) {
      update.e = update.b;
    }
    return false;
  }
  return true;
}

// The Interpreter class implements a virtual machine that executes bytecode
// instructions for dataframe query operations. It maintains an internal
// register state, processes sequences of bytecode operations, and applies
// filter and transformation operations to data columns. The interpreter is
// designed for high-performance data filtering and manipulation, with
// specialized handling for different data types and comparison operations.
//
// This class is templated on a subclass of ValueFetcher, which is used to
// fetch filter values for each filter spec.
template <typename FilterValueFetcherImpl>
class Interpreter {
 public:
  static_assert(std::is_base_of_v<ValueFetcher, FilterValueFetcherImpl>,
                "FilterValueFetcherImpl must be a subclass of ValueFetcher");

  Interpreter(BytecodeVector bytecode,
              const Column* columns,
              const StringPool* string_pool)
      : bytecode_(std::move(bytecode)),
        columns_(columns),
        string_pool_(string_pool) {}

  // Not movable because it's a very large object and the move cost would be
  // high. Prefer constructing in place.
  Interpreter(Interpreter&&) = delete;
  Interpreter& operator=(Interpreter&&) = delete;

#define PERFETTO_DATAFRAME_BYTECODE_CASE_FN(...)                            \
  case base::variant_index<bytecode::BytecodeVariant,                       \
                           bytecode::__VA_ARGS__>(): {                      \
    this->__VA_ARGS__(static_cast<const bytecode::__VA_ARGS__&>(bytecode)); \
    break;                                                                  \
  }

  // Executes the bytecode sequence, processing each bytecode instruction in
  // turn, and dispatching to the appropriate function in this class.
  PERFETTO_ALWAYS_INLINE void Execute(
      FilterValueFetcherImpl& filter_value_fetcher) {
    filter_value_fetcher_ = &filter_value_fetcher;
    for (const auto& bytecode : bytecode_) {
      switch (bytecode.option) {
        PERFETTO_DATAFRAME_BYTECODE_LIST(PERFETTO_DATAFRAME_BYTECODE_CASE_FN)
        default:
          PERFETTO_ASSUME(false);
      }
    }
    filter_value_fetcher_ = nullptr;
  }

  // Returns the value of the specified register if it contains the expected
  // type. Returns nullptr if the register holds a different type or is empty.
  template <typename T>
  PERFETTO_ALWAYS_INLINE const T* GetRegisterValue(reg::ReadHandle<T> reg) {
    if (std::holds_alternative<T>(registers_[reg.index])) {
      return &base::unchecked_get<T>(registers_[reg.index]);
    }
    return nullptr;
  }

  // Sets the value of the specified register for testing purposes.
  //
  // Makes it easier to test certain bytecode instructions which depend on
  // the preexisting value of a register.
  template <typename T>
  void SetRegisterValueForTesting(reg::WriteHandle<T> reg, T value) {
    WriteToRegister(reg, std::move(value));
  }

  const Column* columns() const { return columns_; }

 private:
  PERFETTO_ALWAYS_INLINE void InitRange(const bytecode::InitRange& init) {
    using B = bytecode::InitRange;
    WriteToRegister(init.arg<B::dest_register>(),
                    Range{0, init.arg<B::size>()});
  }

  PERFETTO_ALWAYS_INLINE void AllocateIndices(
      const bytecode::AllocateIndices& ai) {
    using B = bytecode::AllocateIndices;

    if (auto* exist_slab =
            MaybeReadFromRegister(ai.arg<B::dest_slab_register>())) {
      // Ensure that the slab is the same size as the requested size.
      PERFETTO_DCHECK(exist_slab->size() == ai.arg<B::size>());

      // Update the span to point to the pre-allocated slab.
      WriteToRegister(ai.arg<B::dest_span_register>(),
                      Span<uint32_t>{exist_slab->begin(), exist_slab->end()});
    } else {
      auto slab = Slab<uint32_t>::Alloc(ai.arg<B::size>());
      Span<uint32_t> span{slab.begin(), slab.end()};
      WriteToRegister(ai.arg<B::dest_slab_register>(), std::move(slab));
      WriteToRegister(ai.arg<B::dest_span_register>(), span);
    }
  }

  // Fills a SlabSegment with sequential values starting from source.begin().
  PERFETTO_ALWAYS_INLINE void Iota(const bytecode::Iota& r) {
    using B = bytecode::Iota;
    const auto& source = ReadFromRegister(r.arg<B::source_register>());

    auto& update = ReadFromRegister(r.arg<B::update_register>());
    PERFETTO_DCHECK(source.size() <= update.size());
    auto* end = update.b + source.size();
    std::iota(update.b, end, source.b);
    update.e = end;
  }

  // Attempts to cast a filter value to the specified type and stores the
  // result. Currently only supports casting to Id type.
  template <typename T>
  PERFETTO_ALWAYS_INLINE void CastFilterValue(
      const bytecode::CastFilterValueBase& f) {
    using B = bytecode::CastFilterValueBase;
    FilterValueHandle handle = f.arg<B::fval_handle>();
    typename FilterValueFetcherImpl::Type filter_value_type =
        filter_value_fetcher_->GetValueType(handle.index);

    using ValueType =
        StorageType::VariantTypeAtIndex<T, CastFilterValueResult::Value>;
    CastFilterValueResult result;
    if constexpr (std::is_same_v<T, Id>) {
      auto op = *f.arg<B::op>().TryDowncast<NonStringOp>();
      uint32_t result_value;
      result.validity = CastFilterValueToInteger(
          handle, filter_value_type, filter_value_fetcher_, op, result_value);
      if (PERFETTO_LIKELY(result.validity == CastFilterValueResult::kValid)) {
        result.value = CastFilterValueResult::Id{result_value};
      }
    } else if constexpr (IntegerOrDoubleType::Contains<T>()) {
      auto op = *f.arg<B::op>().TryDowncast<NonStringOp>();
      ValueType result_value;
      result.validity = CastFilterValueToIntegerOrDouble(
          handle, filter_value_type, filter_value_fetcher_, op, result_value);
      if (PERFETTO_LIKELY(result.validity == CastFilterValueResult::kValid)) {
        result.value = result_value;
      }
    } else if constexpr (std::is_same_v<T, String>) {
      static_assert(std::is_same_v<ValueType, const char*>);
      auto op = *f.arg<B::op>().TryDowncast<StringOp>();
      const char* result_value;
      result.validity = CastFilterValueToString(
          handle, filter_value_type, filter_value_fetcher_, op, result_value);
      if (PERFETTO_LIKELY(result.validity == CastFilterValueResult::kValid)) {
        result.value = result_value;
      }
    } else {
      static_assert(std::is_same_v<T, Id>, "Unsupported type");
    }
    WriteToRegister(f.arg<B::write_register>(), result);
  }

  template <typename T, typename RangeOp>
  PERFETTO_ALWAYS_INLINE void SortedFilter(
      const bytecode::SortedFilterBase& f) {
    using B = bytecode::SortedFilterBase;

    const auto& value = ReadFromRegister(f.arg<B::val_register>());
    Range& update = ReadFromRegister(f.arg<B::update_register>());
    if (!HandleInvalidCastFilterValueResult(value, update)) {
      return;
    }
    using M = StorageType::VariantTypeAtIndex<T, CastFilterValueResult::Value>;
    M val = base::unchecked_get<M>(value.value);
    if constexpr (std::is_same_v<T, Id>) {
      uint32_t inner_val = val.value;
      if constexpr (std::is_same_v<RangeOp, EqualRange>) {
        bool in_bounds = inner_val >= update.b && inner_val < update.e;
        update.b = inner_val;
        update.e = inner_val + in_bounds;
      } else if constexpr (std::is_same_v<RangeOp, LowerBound> ||
                           std::is_same_v<RangeOp, UpperBound>) {
        if (inner_val >= update.b && inner_val < update.e) {
          BoundModifier bound = f.arg<B::write_result_to>();
          auto& res = bound.Is<BeginBound>() ? update.b : update.e;
          res = inner_val + std::is_same_v<RangeOp, UpperBound>;
        } else {
          update.e = update.b;
        }
      } else {
        static_assert(std::is_same_v<RangeOp, EqualRange>, "Unsupported op");
      }
    } else if constexpr (IntegerOrDoubleType::Contains<T>()) {
      BoundModifier bound_modifier = f.arg<B::write_result_to>();
      const auto* data =
          columns_[f.arg<B::col>()].storage.template unchecked_data<T>();
      SortedIntegerOrDoubleFilter<RangeOp>(data, val, bound_modifier, update);
    } else if constexpr (std::is_same_v<T, String>) {
      BoundModifier bound_modifier = f.arg<B::write_result_to>();
      const auto* data =
          columns_[f.arg<B::col>()].storage.template unchecked_data<String>();
      SortedStringFilter<RangeOp>(data, val, bound_modifier, update);
    } else {
      static_assert(std::is_same_v<T, Id>, "Unsupported type");
    }
  }

  PERFETTO_ALWAYS_INLINE void Uint32SetIdSortedEq(
      const bytecode::Uint32SetIdSortedEq& bytecode) {
    using B = bytecode::Uint32SetIdSortedEq;

    const CastFilterValueResult& cast_result =
        ReadFromRegister(bytecode.arg<B::val_register>());
    auto& update = ReadFromRegister(bytecode.arg<B::update_register>());
    if (!HandleInvalidCastFilterValueResult(cast_result, update)) {
      return;
    }
    using ValueType =
        StorageType::VariantTypeAtIndex<Uint32, CastFilterValueResult::Value>;
    auto val = base::unchecked_get<ValueType>(cast_result.value);
    const auto& col = columns_[bytecode.arg<B::col>()];
    const auto* storage = col.storage.template unchecked_data<Uint32>();
    const auto* start =
        std::clamp(storage + val, storage + update.b, storage + update.e);
    update.b = static_cast<uint32_t>(start - storage);
    const auto* it = start;
    for (; it != storage + update.e; ++it) {
      if (*it != val) {
        break;
      }
    }
    update.e = static_cast<uint32_t>(it - storage);
  }

  template <typename RangeOp, typename DataType>
  PERFETTO_ALWAYS_INLINE void SortedIntegerOrDoubleFilter(
      const DataType* data,
      DataType val,
      BoundModifier bound_modifier,
      Range& update) {
    auto* begin = data + update.b;
    auto* end = data + update.e;
    if constexpr (std::is_same_v<RangeOp, EqualRange>) {
      PERFETTO_DCHECK(bound_modifier.Is<BothBounds>());
      const DataType* eq_start = std::lower_bound(begin, end, val);
      for (auto* it = eq_start; it != end; ++it) {
        if (std::not_equal_to<>()(*it, val)) {
          update.b = static_cast<uint32_t>(eq_start - data);
          update.e = static_cast<uint32_t>(it - data);
          return;
        }
      }
      update.e = update.b;
    } else if constexpr (std::is_same_v<RangeOp, LowerBound>) {
      auto& res = bound_modifier.Is<BeginBound>() ? update.b : update.e;
      res = static_cast<uint32_t>(std::lower_bound(begin, end, val) - data);
    } else if constexpr (std::is_same_v<RangeOp, UpperBound>) {
      auto& res = bound_modifier.Is<BeginBound>() ? update.b : update.e;
      res = static_cast<uint32_t>(std::upper_bound(begin, end, val) - data);
    } else {
      static_assert(std::is_same_v<RangeOp, EqualRange>, "Unsupported op");
    }
  }

  template <typename RangeOp>
  PERFETTO_ALWAYS_INLINE void SortedStringFilter(const StringPool::Id* data,
                                                 const char* val,
                                                 BoundModifier bound_modifier,
                                                 Range& update) {
    const StringPool::Id* begin = data + update.b;
    const StringPool::Id* end = data + update.e;
    if constexpr (std::is_same_v<RangeOp, EqualRange>) {
      PERFETTO_DCHECK(bound_modifier.Is<BothBounds>());
      std::optional<StringPool::Id> id =
          string_pool_->GetId(base::StringView(val));
      if (!id) {
        update.e = update.b;
        return;
      }
      const StringPool::Id* eq_start = std::lower_bound(
          begin, end, val, comparators::StringComparator<Lt>{string_pool_});
      for (const StringPool::Id* it = eq_start; it != end; ++it) {
        if (*it != *id) {
          update.b = static_cast<uint32_t>(eq_start - data);
          update.e = static_cast<uint32_t>(it - data);
          return;
        }
      }
      update.e = update.b;
    } else if constexpr (std::is_same_v<RangeOp, LowerBound>) {
      auto& res = bound_modifier.Is<BeginBound>() ? update.b : update.e;
      const auto* it = std::lower_bound(
          begin, end, val, comparators::StringComparator<Lt>{string_pool_});
      res = static_cast<uint32_t>(it - data);
    } else if constexpr (std::is_same_v<RangeOp, UpperBound>) {
      auto& res = bound_modifier.Is<BeginBound>() ? update.b : update.e;
      const auto* it = std::upper_bound(
          begin, end, val, comparators::StringLessInvert{string_pool_});
      res = static_cast<uint32_t>(it - data);
    } else {
      static_assert(std::is_same_v<RangeOp, EqualRange>, "Unsupported op");
    }
  }

  template <typename T, typename Op>
  PERFETTO_ALWAYS_INLINE void NonStringFilter(
      const bytecode::NonStringFilterBase& nf) {
    using B = bytecode::NonStringFilterBase;
    const auto& value = ReadFromRegister(nf.arg<B::val_register>());
    auto& update = ReadFromRegister(nf.arg<B::update_register>());
    if (!HandleInvalidCastFilterValueResult(value, update)) {
      return;
    }
    const auto& source = ReadFromRegister(nf.arg<B::source_register>());
    using M = StorageType::VariantTypeAtIndex<T, CastFilterValueResult::Value>;
    if constexpr (std::is_same_v<T, Id>) {
      update.e = IdentityFilter(
          source.b, source.e, update.b,
          base::unchecked_get<M>(value.value).value,
          comparators::IntegerOrDoubleComparator<uint32_t, Op>());
    } else if constexpr (IntegerOrDoubleType::Contains<T>()) {
      const auto* data =
          columns_[nf.arg<B::col>()].storage.template unchecked_data<T>();
      update.e = Filter(data, source.b, source.e, update.b,
                        base::unchecked_get<M>(value.value),
                        comparators::IntegerOrDoubleComparator<M, Op>());
    } else {
      static_assert(std::is_same_v<T, Id>, "Unsupported type");
    }
  }

  template <typename Op>
  PERFETTO_ALWAYS_INLINE void StringFilter(
      const bytecode::StringFilterBase& sf) {
    using B = bytecode::StringFilterBase;
    const auto& filter_value = ReadFromRegister(sf.arg<B::val_register>());
    auto& update = ReadFromRegister(sf.arg<B::update_register>());
    if (!HandleInvalidCastFilterValueResult(filter_value, update)) {
      return;
    }
    const char* val = base::unchecked_get<const char*>(filter_value.value);
    const auto& source = ReadFromRegister(sf.arg<B::source_register>());
    const StringPool::Id* ptr =
        columns_[sf.arg<B::col>()].storage.template unchecked_data<String>();
    update.e = FilterStringOp<Op>(ptr, source.b, source.e, update.b, val);
  }

  template <typename NullOp>
  PERFETTO_ALWAYS_INLINE void NullFilter(
      const bytecode::NullFilterBase& filter) {
    using B = bytecode::NullFilterBase;
    const auto& column = columns_[filter.arg<B::col>()];
    const auto& overlay = column.null_storage;
    auto& update = ReadFromRegister(filter.arg<B::update_register>());
    static constexpr bool kInvert = std::is_same_v<NullOp, IsNull>;
    update.e = overlay.GetNullBitVector().template PackLeft<kInvert>(
        update.b, update.e, update.b);
  }

  PERFETTO_ALWAYS_INLINE void NullIndicesStablePartition(
      const bytecode::NullIndicesStablePartition& partition) {
    using B = bytecode::NullIndicesStablePartition;

    const auto& column = columns_[partition.arg<B::col>()];
    const auto& location = partition.arg<B::nulls_location>();
    const auto& overlay = column.null_storage;
    auto& update = ReadFromRegister(partition.arg<B::partition_register>());
    const auto& dest = partition.arg<B::dest_non_null_register>();

    const auto& bit_vector = overlay.GetNullBitVector();
    auto* partition_ptr = std::stable_partition(
        update.b, update.e, [&](uint32_t i) { return !bit_vector.is_set(i); });
    if (PERFETTO_UNLIKELY(location.Is<NullsAtEnd>())) {
      auto non_null_size = static_cast<uint32_t>(update.e - partition_ptr);
      std::rotate(update.b, partition_ptr, update.e);
      WriteToRegister(dest, Span<uint32_t>{update.b, update.b + non_null_size});
    } else {
      WriteToRegister(dest, Span<uint32_t>(partition_ptr, update.e));
    }
  }

  template <typename T>
  PERFETTO_ALWAYS_INLINE void StableSortIndices(
      const bytecode::StableSortIndices<T>& sort_op) {
    using B = bytecode::StableSortIndicesBase;

    auto& indices_to_sort =
        ReadFromRegister(sort_op.template arg<B::update_register>());
    uint32_t col_idx = sort_op.template arg<B::col>();
    SortDirection direction = sort_op.template arg<B::direction>();
    const auto* data = columns_[col_idx].storage.template unchecked_data<T>();
    if (direction == SortDirection::kAscending) {
      auto asc_comparator = [&](uint32_t idx_a, uint32_t idx_b) {
        if constexpr (std::is_same_v<T, Id>) {
          base::ignore_result(data);
          return idx_a < idx_b;
        } else if constexpr (std::is_same_v<T, String>) {
          const auto& value_a = data[idx_a];
          const auto& value_b = data[idx_b];
          return string_pool_->Get(value_a) < string_pool_->Get(value_b);
        } else {
          const auto& value_a = data[idx_a];
          const auto& value_b = data[idx_b];
          return value_a < value_b;
        }
      };
      std::stable_sort(indices_to_sort.b, indices_to_sort.e, asc_comparator);
    } else {  // kDescending
      auto desc_comparator = [&](uint32_t idx_a, uint32_t idx_b) {
        if constexpr (std::is_same_v<T, Id>) {
          base::ignore_result(data);
          return idx_a > idx_b;
        } else if constexpr (std::is_same_v<T, String>) {
          const auto& value_a = data[idx_a];
          const auto& value_b = data[idx_b];
          return string_pool_->Get(value_a) > string_pool_->Get(value_b);
        } else {
          const auto& value_a = data[idx_a];
          const auto& value_b = data[idx_b];
          return value_a > value_b;
        }
      };
      std::stable_sort(indices_to_sort.b, indices_to_sort.e, desc_comparator);
    }
  }

  PERFETTO_ALWAYS_INLINE void StrideCopy(
      const bytecode::StrideCopy& stride_copy) {
    using B = bytecode::StrideCopy;
    const auto& source =
        ReadFromRegister(stride_copy.arg<B::source_register>());
    auto& update = ReadFromRegister(stride_copy.arg<B::update_register>());
    uint32_t stride = stride_copy.arg<B::stride>();
    PERFETTO_DCHECK(source.size() * stride <= update.size());
    uint32_t* write_ptr = update.b;
    for (const uint32_t* it = source.b; it < source.e; ++it) {
      *write_ptr = *it;
      write_ptr += stride;
    }
    PERFETTO_DCHECK(write_ptr == update.b + source.size() * stride);
    update.e = write_ptr;
  }

  PERFETTO_ALWAYS_INLINE void PrefixPopcount(
      const bytecode::PrefixPopcount& popcount) {
    using B = bytecode::PrefixPopcount;
    auto dest_register = popcount.arg<B::dest_register>();
    if (MaybeReadFromRegister<Slab<uint32_t>>(dest_register)) {
      return;
    }
    const auto& overlay = columns_[popcount.arg<B::col>()].null_storage;
    WriteToRegister(dest_register, overlay.GetNullBitVector().PrefixPopcount());
  }

  PERFETTO_ALWAYS_INLINE void TranslateSparseNullIndices(
      const bytecode::TranslateSparseNullIndices& bytecode) {
    using B = bytecode::TranslateSparseNullIndices;
    const auto& overlay = columns_[bytecode.arg<B::col>()].null_storage;
    const auto& bv =
        overlay.template unchecked_get<NullStorage::SparseNull>().bit_vector;

    const auto& source = ReadFromRegister(bytecode.arg<B::source_register>());
    auto& update = ReadFromRegister(bytecode.arg<B::update_register>());
    PERFETTO_DCHECK(source.size() <= update.size());

    const Slab<uint32_t>& popcnt =
        ReadFromRegister(bytecode.arg<B::popcount_register>());
    uint32_t* out = update.b;
    for (uint32_t* it = source.b; it != source.e; ++it, ++out) {
      uint32_t s = *it;
      *out = static_cast<uint32_t>(popcnt[s / 64] +
                                   bv.count_set_bits_until_in_word(s));
    }
    update.e = out;
  }

  PERFETTO_ALWAYS_INLINE void StrideTranslateAndCopySparseNullIndices(
      const bytecode::StrideTranslateAndCopySparseNullIndices& bytecode) {
    using B = bytecode::StrideTranslateAndCopySparseNullIndices;
    const auto& overlay = columns_[bytecode.arg<B::col>()].null_storage;
    const auto& bv =
        overlay.template unchecked_get<NullStorage::SparseNull>().bit_vector;

    auto& update = ReadFromRegister(bytecode.arg<B::update_register>());
    uint32_t stride = bytecode.arg<B::stride>();
    uint32_t offset = bytecode.arg<B::offset>();
    const Slab<uint32_t>& popcnt =
        ReadFromRegister(bytecode.arg<B::popcount_register>());
    for (uint32_t* it = update.b; it != update.e; it += stride) {
      uint32_t index = *it;
      if (bv.is_set(index)) {
        it[offset] = static_cast<uint32_t>(
            popcnt[index / 64] + bv.count_set_bits_until_in_word(index));
      } else {
        it[offset] = std::numeric_limits<uint32_t>::max();
      }
    }
  }

  PERFETTO_ALWAYS_INLINE void StrideCopyDenseNullIndices(
      const bytecode::StrideCopyDenseNullIndices& bytecode) {
    using B = bytecode::StrideCopyDenseNullIndices;
    const auto& overlay = columns_[bytecode.arg<B::col>()].null_storage;
    const auto& bv =
        overlay.template unchecked_get<NullStorage::DenseNull>().bit_vector;

    auto& update = ReadFromRegister(bytecode.arg<B::update_register>());
    uint32_t stride = bytecode.arg<B::stride>();
    uint32_t offset = bytecode.arg<B::offset>();
    for (uint32_t* it = update.b; it != update.e; it += stride) {
      it[offset] = bv.is_set(*it) ? *it : std::numeric_limits<uint32_t>::max();
    }
  }

  PERFETTO_ALWAYS_INLINE void AllocateRowLayoutBuffer(
      const bytecode::AllocateRowLayoutBuffer& bytecode) {
    using B = bytecode::AllocateRowLayoutBuffer;
    uint32_t size = bytecode.arg<B::buffer_size>();
    auto dest_reg = bytecode.arg<B::dest_buffer_register>();
    // Return early if buffer already allocated.
    if (MaybeReadFromRegister(dest_reg)) {
      return;
    }
    WriteToRegister(dest_reg, Slab<uint8_t>::Alloc(size));
  }

  PERFETTO_ALWAYS_INLINE void CopyToRowLayoutNonNull(
      const bytecode::CopyToRowLayoutNonNull& bytecode) {
    using B = bytecode::CopyToRowLayoutNonNull;
    uint32_t col_idx = bytecode.arg<B::col>();
    const auto& source =
        ReadFromRegister(bytecode.arg<B::source_indices_register>());
    auto& dest_buffer =
        ReadFromRegister(bytecode.arg<B::dest_buffer_register>());
    uint32_t stride = bytecode.arg<B::row_layout_stride>();
    uint32_t offset = bytecode.arg<B::row_layout_offset>();
    uint32_t copy_size = bytecode.arg<B::copy_size>();

    uint8_t* dest_addr = dest_buffer.data() + offset;
    const auto& storage = columns_[col_idx].storage;
    const uint8_t* source_base = storage.byte_data();

    for (uint32_t* ptr = source.b; ptr != source.e; ++ptr) {
      // If the pointer is null, then use the index directly as the
      // value we need to copy.
      const uint8_t* source_addr = source_base
                                       ? source_base + (*ptr * copy_size)
                                       : reinterpret_cast<uint8_t*>(ptr);
      // TODO(lalitm): consider branching over the size to help the
      // compiler figure out more optimized copy loops.
      memcpy(dest_addr, source_addr, copy_size);
      dest_addr += stride;
    }
  }

  PERFETTO_ALWAYS_INLINE void CopyToRowLayoutDenseNull(
      const bytecode::CopyToRowLayoutDenseNull& bytecode) {
    using B = bytecode::CopyToRowLayoutDenseNull;
    uint32_t col_idx = bytecode.arg<B::col>();
    const auto& source =
        ReadFromRegister(bytecode.arg<B::source_indices_register>());
    auto& dest_buffer =
        ReadFromRegister(bytecode.arg<B::dest_buffer_register>());
    uint32_t stride = bytecode.arg<B::row_layout_stride>();
    uint32_t offset = bytecode.arg<B::row_layout_offset>();
    uint32_t copy_size = bytecode.arg<B::copy_size>();

    uint8_t* dest_addr = dest_buffer.data() + offset;
    const auto& col = columns_[col_idx];
    const auto& storage = col.storage;
    const auto& bv = col.null_storage.GetNullBitVector();
    const uint8_t* source_base = storage.byte_data();

    for (uint32_t* ptr = source.b; ptr != source.e; ++ptr) {
      // If the pointer is null, then use the index directly as the
      // value we need to copy.
      bool is_non_null = bv.is_set(*ptr);
      *dest_addr = static_cast<uint8_t>(is_non_null);
      if (is_non_null) {
        const uint8_t* source_addr =
            source_base ? source_base + (*ptr * copy_size)
                        : reinterpret_cast<const uint8_t*>(ptr);

        // TODO(lalitm): consider branching over the size to help the
        // compiler figure out more optimized copy loops.
        memcpy(dest_addr + 1, source_addr, copy_size);
      } else {
        memset(dest_addr + 1, 0, copy_size);
      }
      dest_addr += stride;
    }
  }

  PERFETTO_ALWAYS_INLINE void CopyToRowLayoutSparseNull(
      const bytecode::CopyToRowLayoutSparseNull& bytecode) {
    using B = bytecode::CopyToRowLayoutSparseNull;
    uint32_t col_idx = bytecode.arg<B::col>();
    const auto& source =
        ReadFromRegister(bytecode.arg<B::source_indices_register>());
    auto& dest_buffer =
        ReadFromRegister(bytecode.arg<B::dest_buffer_register>());
    const auto& popcount_slab =
        ReadFromRegister(bytecode.arg<B::popcount_register>());
    uint32_t stride = bytecode.arg<B::row_layout_stride>();
    uint32_t offset = bytecode.arg<B::row_layout_offset>();
    uint32_t copy_size = bytecode.arg<B::copy_size>();

    uint8_t* dest_addr = dest_buffer.data() + offset;
    const auto& col = columns_[col_idx];
    const auto& storage = col.storage;
    const auto& bv = col.null_storage.GetNullBitVector();
    const uint8_t* source_base = storage.byte_data();

    for (uint32_t* ptr = source.b; ptr != source.e; ++ptr) {
      bool is_non_null = bv.is_set(*ptr);
      *dest_addr = static_cast<uint8_t>(is_non_null);
      if (is_non_null) {
        auto storage_idx = static_cast<uint32_t>(
            popcount_slab[*ptr / 64] + bv.count_set_bits_until_in_word(*ptr));

        // If the pointer is null, then use the index directly as the
        // value we need to copy.
        const uint8_t* source_addr =
            source_base ? source_base + (storage_idx * copy_size)
                        : reinterpret_cast<const uint8_t*>(&storage_idx);

        // TODO(lalitm): consider branching over the size to help the
        // compiler figure out more optimized copy loops.
        memcpy(dest_addr + 1, source_addr, copy_size);
      } else {
        memset(dest_addr + 1, 0, copy_size);
      }
      dest_addr += stride;
    }
  }

  PERFETTO_ALWAYS_INLINE void Distinct(const bytecode::Distinct& bytecode) {
    using B = bytecode::Distinct;
    auto& indices = ReadFromRegister(bytecode.arg<B::indices_register>());
    if (indices.empty()) {
      return;
    }

    const auto& buffer = ReadFromRegister(bytecode.arg<B::buffer_register>());
    uint32_t stride = bytecode.arg<B::total_row_stride>();
    const uint8_t* row_ptr = buffer.data();

    std::unordered_set<std::string_view> seen_rows;
    seen_rows.reserve(indices.size());
    uint32_t* write_ptr = indices.b;
    for (const uint32_t* it = indices.b; it != indices.e; ++it) {
      std::string_view row_view(reinterpret_cast<const char*>(row_ptr), stride);
      *write_ptr = *it;
      write_ptr += seen_rows.insert(row_view).second;
      row_ptr += stride;
    }
    indices.e = write_ptr;
  }

  PERFETTO_ALWAYS_INLINE void LimitOffsetIndices(
      const bytecode::LimitOffsetIndices& bytecode) {
    using B = bytecode::LimitOffsetIndices;
    uint32_t offset_value = bytecode.arg<B::offset_value>();
    uint32_t limit_value = bytecode.arg<B::limit_value>();
    auto& span = ReadFromRegister(bytecode.arg<B::update_register>());

    // Apply offset
    auto original_size = static_cast<uint32_t>(span.size());
    uint32_t actual_offset = std::min(offset_value, original_size);
    span.b += actual_offset;

    // Apply limit
    auto size_after_offset = static_cast<uint32_t>(span.size());
    uint32_t actual_limit = std::min(limit_value, size_after_offset);
    span.e = span.b + actual_limit;
  }

  template <typename T, typename Op>
  PERFETTO_ALWAYS_INLINE void FindMinMaxIndex(
      const bytecode::FindMinMaxIndex<T, Op>& bytecode) {
    using B = bytecode::FindMinMaxIndexBase;  // Use base for arg names
    uint32_t col = bytecode.template arg<B::col>();
    auto& indices =
        ReadFromRegister(bytecode.template arg<B::update_register>());
    if (indices.empty()) {
      return;
    }

    const auto* data = columns_[col].storage.template unchecked_data<T>();
    auto get_value = [&](uint32_t idx) {
      if constexpr (std::is_same_v<T, Id>) {
        base::ignore_result(data);
        return idx;
      } else if constexpr (std::is_same_v<T, String>) {
        return string_pool_->Get(data[idx]);
      } else {
        return data[idx];
      }
    };
    uint32_t best_idx = *indices.b;
    auto best_val = get_value(best_idx);
    for (const uint32_t* it = indices.b + 1; it != indices.e; ++it) {
      uint32_t current_idx = *it;
      auto current_val = get_value(current_idx);
      bool current_is_better;
      if constexpr (std::is_same_v<Op, MinOp>) {
        current_is_better = current_val < best_val;
      } else {
        current_is_better = current_val > best_val;
      }
      if (current_is_better) {
        best_idx = current_idx;
        best_val = current_val;
      }
    }
    *indices.b = best_idx;
    indices.e = indices.b + 1;
  }

  template <typename Op>
  PERFETTO_ALWAYS_INLINE uint32_t* FilterStringOp(const StringPool::Id* data,
                                                  const uint32_t* begin,
                                                  const uint32_t* end,
                                                  uint32_t* output,
                                                  const char* val) {
    if constexpr (std::is_same_v<Op, Eq>) {
      return StringFilterEq(data, begin, end, output, val);
    } else if constexpr (std::is_same_v<Op, Ne>) {
      return StringFilterNe(data, begin, end, output, val);
    } else if constexpr (std::is_same_v<Op, Glob>) {
      return StringFilterGlob(data, begin, end, output, val);
    } else if constexpr (std::is_same_v<Op, Regex>) {
      return StringFilterRegex(data, begin, end, output, val);
    } else {
      return Filter(data, begin, end, output, NullTermStringView(val),
                    comparators::StringComparator<Op>{string_pool_});
    }
  }

  PERFETTO_ALWAYS_INLINE uint32_t* StringFilterEq(const StringPool::Id* data,
                                                  const uint32_t* begin,
                                                  const uint32_t* end,
                                                  uint32_t* output,
                                                  const char* val) {
    std::optional<StringPool::Id> id =
        string_pool_->GetId(base::StringView(val));
    if (!id) {
      return output;
    }
    static_assert(sizeof(StringPool::Id) == 4, "Id should be 4 bytes");
    return Filter(reinterpret_cast<const uint32_t*>(data), begin, end, output,
                  id->raw_id(), std::equal_to<>());
  }

  PERFETTO_ALWAYS_INLINE uint32_t* StringFilterNe(const StringPool::Id* data,
                                                  const uint32_t* begin,
                                                  const uint32_t* end,
                                                  uint32_t* output,
                                                  const char* val) {
    std::optional<StringPool::Id> id =
        string_pool_->GetId(base::StringView(val));
    if (!id) {
      memcpy(output, begin, size_t(end - begin));
      return output + (end - begin);
    }
    static_assert(sizeof(StringPool::Id) == 4, "Id should be 4 bytes");
    return Filter(reinterpret_cast<const uint32_t*>(data), begin, end, output,
                  id->raw_id(), std::not_equal_to<>());
  }

  PERFETTO_ALWAYS_INLINE uint32_t* StringFilterGlob(const StringPool::Id* data,
                                                    const uint32_t* begin,
                                                    const uint32_t* end,
                                                    uint32_t* output,
                                                    const char* val) {
    auto matcher = util::GlobMatcher::FromPattern(val);
    // If glob pattern doesn't involve any special characters, the function
    // called should be equality.
    if (matcher.IsEquality()) {
      return StringFilterEq(data, begin, end, output, val);
    }
    // For very big string pools (or small ranges) or pools with large
    // strings run a standard glob function.
    if (size_t(end - begin) < string_pool_->size() ||
        string_pool_->HasLargeString()) {
      return Filter(data, begin, end, output, matcher,
                    comparators::Glob{string_pool_});
    }
    // TODO(lalitm): the BitVector can be placed in a register removing to
    // need to allocate every time.
    auto matches =
        BitVector::CreateWithSize(string_pool_->MaxSmallStringId().raw_id());
    PERFETTO_DCHECK(!string_pool_->HasLargeString());
    for (auto it = string_pool_->CreateSmallStringIterator(); it; ++it) {
      auto id = it.StringId();
      matches.change_assume_unset(id.raw_id(),
                                  matcher.Matches(string_pool_->Get(id)));
    }
    return Filter(data, begin, end, output, matches,
                  comparators::GlobFullStringPool{});
  }

  PERFETTO_ALWAYS_INLINE uint32_t* StringFilterRegex(const StringPool::Id* data,
                                                     const uint32_t* begin,
                                                     const uint32_t* end,
                                                     uint32_t* output,
                                                     const char* val) {
    auto regex = regex::Regex::Create(val);
    if (!regex.ok()) {
      return output;
    }
    return Filter(data, begin, end, output, regex.value(),
                  comparators::Regex{string_pool_});
  }

  // Filters an existing index buffer in-place, based on data comparisons
  // performed using a separate set of source indices.
  //
  // This function iterates synchronously through two sets of indices:
  // 1. Source Indices: Provided by [begin, end), pointed to by `it`. These
  //    indices are used *only* to look up data values (`data[*it]`).
  // 2. Destination/Update Indices: Starting at `o_start`, pointed to by
  //    `o_read` (for reading the original index) and `o_write` (for writing
  //    (for reading the original index) and `o_write` (for writing kept
  //    indices). This buffer is modified *in-place*.
  //
  // For each step `i`:
  //   - It retrieves the data value using the i-th source index:
  //   `data[begin[i]]`.
  //   - It compares this data value against the provided `value`.
  //   - It reads the i-th *original* index from the destination buffer:
  //   `o_read[i]`.
  //   - If the comparison is true, it copies the original index `o_read[i]`
  //     to the current write position `*o_write` and advances `o_write`.
  //
  // The result is that the destination buffer `[o_start, returned_pointer)`
  // contains the subset of its *original* indices for which the comparison
  // (using the corresponding source index for data lookup) was true.
  //
  // Use Case Example (SparseNull Filter):
  //   - `[begin, end)` holds translated storage indices (for correct data
  //     lookup).
  //   - `o_start` points to the buffer holding original table indices (that
  //     was have already been filtered by `NullFilter<IsNotNull>`).
  //   - This function further filters the original table indices in `o_start`
  //     based on data comparisons using the translated indices.
  //
  // Args:
  //   data: Pointer to the start of the column's data storage.
  //   begin: Pointer to the first index in the source span (for data lookup).
  //   end: Pointer one past the last index in the source span.
  //   o_start: Pointer to the destination/update buffer (filtered in-place).
  //   value: The value to compare data against.
  //   comparator: Functor implementing the comparison logic.
  //
  // Returns:
  //   A pointer one past the last index written to the destination buffer.
  template <typename Comparator, typename ValueType, typename DataType>
  [[nodiscard]] PERFETTO_ALWAYS_INLINE static uint32_t* Filter(
      const DataType* data,
      const uint32_t* begin,
      const uint32_t* end,
      uint32_t* o_start,
      const ValueType& value,
      const Comparator& comparator) {
    const uint32_t* o_read = o_start;
    uint32_t* o_write = o_start;
    for (const uint32_t* it = begin; it != end; ++it, ++o_read) {
      *o_write = *o_read;
      o_write += comparator(data[*it], value);
    }
    return o_write;
  }

  // Similar to Filter but operates directly on the identity values (indices)
  // rather than dereferencing through a data array.
  template <typename Comparator>
  [[nodiscard]] PERFETTO_ALWAYS_INLINE static uint32_t* IdentityFilter(
      const uint32_t* begin,
      const uint32_t* end,
      uint32_t* o_start,
      uint32_t value,
      Comparator comparator) {
    const uint32_t* o_read = o_start;
    uint32_t* o_write = o_start;
    for (const uint32_t* it = begin; it != end; ++it, ++o_read) {
      *o_write = *o_read;
      o_write += comparator(*it, value);
    }
    return o_write;
  }

  // Attempts to cast a filter value to a numeric type, dispatching to the
  // appropriate type-specific conversion function.
  template <typename T>
  [[nodiscard]] PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastFilterValueToIntegerOrDouble(
      FilterValueHandle handle,
      typename FilterValueFetcherImpl::Type filter_value_type,
      FilterValueFetcherImpl* fetcher,
      NonStringOp op,
      T& out) {
    if constexpr (std::is_same_v<T, double>) {
      return CastFilterValueToDouble(handle, filter_value_type, fetcher, op,
                                     out);
    } else if constexpr (std::is_integral_v<T>) {
      return CastFilterValueToInteger<T>(handle, filter_value_type, fetcher, op,
                                         out);
    } else {
      static_assert(std::is_same_v<T, double>, "Unsupported type");
    }
  }

  // Attempts to cast a filter value to an integer type, handling various edge
  // cases such as out-of-range values and non-integer inputs.
  template <typename T>
  [[nodiscard]] PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastFilterValueToInteger(
      FilterValueHandle handle,
      typename FilterValueFetcherImpl::Type filter_value_type,
      FilterValueFetcherImpl* fetcher,
      NonStringOp op,
      T& out) {
    static_assert(std::is_integral_v<T>, "Unsupported type");

    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl::kInt64)) {
      int64_t res = fetcher->GetInt64Value(handle.index);
      bool is_small = res < std::numeric_limits<T>::min();
      bool is_big = res > std::numeric_limits<T>::max();
      if (PERFETTO_UNLIKELY(is_small || is_big)) {
        switch (op.index()) {
          case NonStringOp::GetTypeIndex<Lt>():
          case NonStringOp::GetTypeIndex<Le>():
            if (is_small) {
              return CastFilterValueResult::kNoneMatch;
            }
            break;
          case NonStringOp::GetTypeIndex<Gt>():
          case NonStringOp::GetTypeIndex<Ge>():
            if (is_big) {
              return CastFilterValueResult::kNoneMatch;
            }
            break;
          case NonStringOp::GetTypeIndex<Eq>():
            return CastFilterValueResult::kNoneMatch;
          case NonStringOp::GetTypeIndex<Ne>():
            // Do nothing.
            break;
          default:
            PERFETTO_FATAL("Invalid numeric filter op");
        }
        return CastFilterValueResult::kAllMatch;
      }
      out = static_cast<T>(res);
      return CastFilterValueResult::kValid;
    }
    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl::kDouble)) {
      double d = fetcher->GetDoubleValue(handle.index);

      // We use the constants directly instead of using numeric_limits for
      // int64_t as the casts introduces rounding in the doubles as a double
      // cannot exactly represent int64::max().
      constexpr double kMin =
          std::is_same_v<T, int64_t>
              ? -9223372036854775808.0
              : static_cast<double>(std::numeric_limits<T>::min());
      constexpr double kMax =
          std::is_same_v<T, int64_t>
              ? 9223372036854775808.0
              : static_cast<double>(std::numeric_limits<T>::max());

      // NaNs always compare false to any value (including other NaNs),
      // regardless of the operator.
      if (PERFETTO_UNLIKELY(std::isnan(d))) {
        return CastFilterValueResult::kNoneMatch;
      }

      // The greater than or equal is intentional to account for the fact that
      // twos-complement integers are not symmetric around zero (i.e.
      // -9223372036854775808 can be represented but 9223372036854775808
      // cannot).
      bool is_big = d >= kMax;
      bool is_small = d < kMin;
      if (PERFETTO_LIKELY(d == trunc(d) && !is_small && !is_big)) {
        out = static_cast<T>(d);
        return CastFilterValueResult::kValid;
      }
      switch (op.index()) {
        case NonStringOp::GetTypeIndex<Lt>():
          return CastDoubleToIntHelper<T, std::ceil>(is_small, is_big, d, out);
        case NonStringOp::GetTypeIndex<Le>():
          return CastDoubleToIntHelper<T, std::floor>(is_small, is_big, d, out);
        case NonStringOp::GetTypeIndex<Gt>():
          return CastDoubleToIntHelper<T, std::floor>(is_big, is_small, d, out);
        case NonStringOp::GetTypeIndex<Ge>():
          return CastDoubleToIntHelper<T, std::ceil>(is_big, is_small, d, out);
        case NonStringOp::GetTypeIndex<Eq>():
          return CastFilterValueResult::kNoneMatch;
        case NonStringOp::GetTypeIndex<Ne>():
          // Do nothing.
          return CastFilterValueResult::kAllMatch;
        default:
          PERFETTO_FATAL("Invalid numeric filter op");
      }
    }
    return CastStringOrNullFilterValueToIntegerOrDouble(filter_value_type, op);
  }

  // Attempts to cast a filter value to a double, handling integer inputs and
  // various edge cases.
  [[nodiscard]] PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastFilterValueToDouble(
      FilterValueHandle filter_value_handle,
      typename FilterValueFetcherImpl::Type filter_value_type,
      FilterValueFetcherImpl* fetcher,
      NonStringOp op,
      double& out) {
    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl::kDouble)) {
      out = fetcher->GetDoubleValue(filter_value_handle.index);
      return CastFilterValueResult::kValid;
    }
    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl::kInt64)) {
      int64_t i = fetcher->GetInt64Value(filter_value_handle.index);
      auto iad = static_cast<double>(i);
      auto iad_int = static_cast<int64_t>(iad);

      // If the integer value can be converted to a double while preserving the
      // exact integer value, then we can use the double value for comparison.
      if (PERFETTO_LIKELY(i == iad_int)) {
        out = iad;
        return CastFilterValueResult::kValid;
      }

      // This can happen in cases where we round `i` up above
      // numeric_limits::max(). In that case, still consider the double larger.
      bool overflow_positive_to_negative = i > 0 && iad_int < 0;
      bool iad_greater_than_i = iad_int > i || overflow_positive_to_negative;
      bool iad_less_than_i = iad_int < i && !overflow_positive_to_negative;
      switch (op.index()) {
        case NonStringOp::GetTypeIndex<Lt>():
          out = iad_greater_than_i
                    ? iad
                    : std::nextafter(iad,
                                     std::numeric_limits<double>::infinity());
          return CastFilterValueResult::kValid;
        case NonStringOp::GetTypeIndex<Le>():
          out = iad_less_than_i
                    ? iad
                    : std::nextafter(iad,
                                     -std::numeric_limits<double>::infinity());
          return CastFilterValueResult::kValid;
        case NonStringOp::GetTypeIndex<Gt>():
          out = iad_less_than_i
                    ? iad
                    : std::nextafter(iad,
                                     -std::numeric_limits<double>::infinity());
          return CastFilterValueResult::kValid;
        case NonStringOp::GetTypeIndex<Ge>():
          out = iad_greater_than_i
                    ? iad
                    : std::nextafter(iad,
                                     std::numeric_limits<double>::infinity());
          return CastFilterValueResult::kValid;
        case NonStringOp::GetTypeIndex<Eq>():
          return CastFilterValueResult::kNoneMatch;
        case NonStringOp::GetTypeIndex<Ne>():
          // Do nothing.
          return CastFilterValueResult::kAllMatch;
        default:
          PERFETTO_FATAL("Invalid numeric filter op");
      }
    }
    return CastStringOrNullFilterValueToIntegerOrDouble(filter_value_type, op);
  }

  // Converts a double to an integer type using the specified function (e.g.,
  // trunc, floor). Used as a helper for various casting operations.
  template <typename T, double (*fn)(double)>
  PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastDoubleToIntHelper(bool no_data, bool all_data, double d, T& out) {
    if (no_data) {
      return CastFilterValueResult::kNoneMatch;
    }
    if (all_data) {
      return CastFilterValueResult::kAllMatch;
    }
    out = static_cast<T>(fn(d));
    return CastFilterValueResult::kValid;
  }

  // Handles conversion of strings or nulls to integer or double types for
  // filtering operations.
  PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastStringOrNullFilterValueToIntegerOrDouble(
      typename FilterValueFetcherImpl::Type filter_value_type,
      NonStringOp op) {
    if (filter_value_type == FilterValueFetcherImpl::kString) {
      if (op.index() == NonStringOp::GetTypeIndex<Eq>() ||
          op.index() == NonStringOp::GetTypeIndex<Ge>() ||
          op.index() == NonStringOp::GetTypeIndex<Gt>()) {
        return CastFilterValueResult::kNoneMatch;
      }
      PERFETTO_DCHECK(op.index() == NonStringOp::GetTypeIndex<Ne>() ||
                      op.index() == NonStringOp::GetTypeIndex<Le>() ||
                      op.index() == NonStringOp::GetTypeIndex<Lt>());
      return CastFilterValueResult::kAllMatch;
    }

    PERFETTO_DCHECK(filter_value_type == FilterValueFetcherImpl::kNull);

    // Nulls always compare false to any value (including other nulls),
    // regardless of the operator.
    return CastFilterValueResult::kNoneMatch;
  }

  PERFETTO_ALWAYS_INLINE static CastFilterValueResult::Validity
  CastFilterValueToString(
      FilterValueHandle handle,
      typename FilterValueFetcherImpl::Type filter_value_type,
      FilterValueFetcherImpl* fetcher,
      const StringOp& op,
      const char*& out) {
    if (PERFETTO_LIKELY(filter_value_type ==
                        FilterValueFetcherImpl ::kString)) {
      out = fetcher->GetStringValue(handle.index);
      return CastFilterValueResult::kValid;
    }
    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl ::kNull)) {
      // Nulls always compare false to any value (including other nulls),
      // regardless of the operator.
      return CastFilterValueResult::kNoneMatch;
    }
    if (PERFETTO_LIKELY(filter_value_type == FilterValueFetcherImpl ::kInt64 ||
                        filter_value_type ==
                            FilterValueFetcherImpl ::kDouble)) {
      switch (op.index()) {
        case Op::GetTypeIndex<Eq>():
        case Op::GetTypeIndex<Ge>():
        case Op::GetTypeIndex<Gt>():
        case Op::GetTypeIndex<Ne>():
          return CastFilterValueResult::kAllMatch;
        case Op::GetTypeIndex<Le>():
        case Op::GetTypeIndex<Lt>():
        case Op::GetTypeIndex<Glob>():
        case Op::GetTypeIndex<Regex>():
          return CastFilterValueResult::kNoneMatch;
        default:
          PERFETTO_FATAL("Invalid string filter op");
      }
    }
    PERFETTO_FATAL("Invalid filter spec value");
  }

  // Access a register for reading/writing with type safety through the handle.
  template <typename T>
  PERFETTO_ALWAYS_INLINE T& ReadFromRegister(reg::RwHandle<T> r) {
    return base::unchecked_get<T>(registers_[r.index]);
  }

  // Access a register for reading only with type safety through the handle.
  template <typename T>
  PERFETTO_ALWAYS_INLINE const T& ReadFromRegister(reg::ReadHandle<T> r) const {
    return base::unchecked_get<T>(registers_[r.index]);
  }

  // Conditionally access a register if it contains the expected type.
  // Returns nullptr if the register holds a different type.
  template <typename T>
  PERFETTO_ALWAYS_INLINE T* MaybeReadFromRegister(reg::WriteHandle<T> reg) {
    if (std::holds_alternative<T>(registers_[reg.index])) {
      return &base::unchecked_get<T>(registers_[reg.index]);
    }
    return nullptr;
  }

  // Writes a value to the specified register, handling type safety through the
  // handle.
  template <typename T>
  PERFETTO_ALWAYS_INLINE void WriteToRegister(reg::WriteHandle<T> r, T value) {
    registers_[r.index] = std::move(value);
  }

  // The sequence of bytecode instructions to execute
  BytecodeVector bytecode_;
  // Register file holding intermediate values
  std::array<reg::Value, reg::kMaxRegisters> registers_;

  // Pointer to the source for filter values.
  FilterValueFetcherImpl* filter_value_fetcher_;
  // Pointer to the data columns being processed
  const Column* columns_;
  // Pointer to the string pool (for string operations)
  const StringPool* string_pool_;
};

}  // namespace perfetto::trace_processor::dataframe::impl::bytecode

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_INTERPRETER_H_
