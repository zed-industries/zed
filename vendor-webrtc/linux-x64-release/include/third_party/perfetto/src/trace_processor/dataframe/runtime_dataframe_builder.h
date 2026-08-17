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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_RUNTIME_DATAFRAME_BUILDER_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_RUNTIME_DATAFRAME_BUILDER_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/containers/null_term_string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/dataframe/dataframe.h"
#include "src/trace_processor/dataframe/impl/bit_vector.h"
#include "src/trace_processor/dataframe/impl/flex_vector.h"
#include "src/trace_processor/dataframe/impl/query_plan.h"
#include "src/trace_processor/dataframe/impl/types.h"
#include "src/trace_processor/dataframe/specs.h"
#include "src/trace_processor/dataframe/value_fetcher.h"
#include "src/trace_processor/util/status_macros.h"

namespace perfetto::trace_processor::dataframe {

// Builds a Dataframe instance row by row at runtime.
//
// This class allows constructing a `Dataframe` incrementally. It infers
// column types (`int64_t`, `double`, `StringPool::Id`) based on the first
// non-null value encountered in each column. Null values are tracked
// efficiently using a `BitVector` (created only if nulls exist), and the
// underlying data storage only stores non-null values (SparseNull
// representation).
//
// Upon calling `Build()`, the builder analyzes the collected data to:
// - Determine the final optimal storage type for integer columns (downcasting
//   `int64_t` to `uint32_t` or `int32_t` if possible, or using `Id` type).
// - Determine the final sort state (`IdSorted`, `SetIdSorted`, `Sorted`,
//   `Unsorted`) by analyzing the collected values. Nullable columns are always
//   `Unsorted`.
// - Construct the final `Dataframe` object.
//
// Usage Example:
// ```cpp
// // Assume MyFetcher inherits from ValueFetcher and provides data for rows.
// struct MyFetcher : ValueFetcher {
//   // ... implementation to fetch data for current row ...
// };
//
// std::vector<std::string> col_names = {"ts", "value", "name"};
// StringPool pool;
// RuntimeDataframeBuilder builder(col_names, &pool);
// for (MyFetcher fetcher; fetcher.Next();) {
//   if (!builder.AddRow(&fetcher)) {
//     // Handle error (e.g., type mismatch)
//     PERFETTO_ELOG("Failed to add row: %s", builder.status().message());
//     break;
//   }
// }
//
// base::StatusOr<Dataframe> df = std::move(builder).Build();
// if (!df.ok()) {
//   // Handle build error
//   PERFETTO_ELOG("Failed to build dataframe: %s", df.status().message());
// } else {
//   // Use the dataframe *df...
// }
// ```
class RuntimeDataframeBuilder {
 public:
  // Constructs a RuntimeDataframeBuilder.
  //
  // Args:
  //   names: A vector of strings representing the names of the columns
  //          to be built. The order determines the column order as well.
  //   pool: A pointer to a `StringPool` instance used for interning
  //         string values encountered during row addition. Must remain
  //         valid for the lifetime of the builder and the resulting
  //         Dataframe.
  RuntimeDataframeBuilder(std::vector<std::string> names, StringPool* pool)
      : string_pool_(pool) {
    for (uint32_t i = 0; i < names.size(); ++i) {
      column_states_.emplace_back();
    }
    for (auto& name : names) {
      column_names_.emplace_back(std::move(name));
    }
  }
  ~RuntimeDataframeBuilder() = default;

  // Movable but not copyable
  RuntimeDataframeBuilder(RuntimeDataframeBuilder&&) noexcept;
  RuntimeDataframeBuilder& operator=(RuntimeDataframeBuilder&&) noexcept;
  RuntimeDataframeBuilder(const RuntimeDataframeBuilder&) = delete;
  RuntimeDataframeBuilder& operator=(const RuntimeDataframeBuilder&) = delete;

  // Adds a row to the dataframe using data provided by the Fetcher.
  //
  // Template Args:
  //   ValueFetcherImpl: A concrete class derived from `ValueFetcher` that
  //                     provides methods like `GetValueType(col_idx)` and
  //                     `GetInt64Value(col_idx)`, `GetDoubleValue(col_idx)`,
  //                     `GetStringValue(col_idx)` for the current row.
  // Args:
  //   fetcher: A pointer to an instance of `ValueFetcherImpl`, configured
  //            to provide data for the row being added. The fetcher only
  //            needs to be valid for the duration of this call.
  // Returns:
  //   true: If the row was added successfully.
  //   false: If an error occurred (e.g., type mismatch). Check `status()` for
  //          details. The builder should not be used further if false is
  //          returned.
  //
  // Implementation Notes:
  // 1) Infers column types (int64_t, double, StringPool::Id) based on the first
  //    non-null value encountered. Stores integer types smaller than int64_t
  //    (i.e. Id, uint32_t, int32_t) initially as int64_t, with potential
  //    downcasting occurring during Build().
  // 2) Tracks null values sparsely: only non-null values are appended to the
  //    internal data storage vectors. A BitVector is created and maintained
  //    only if null values are encountered for a column.
  // 3) Performs strict type checking against the inferred type for subsequent
  //    rows. If a type mismatch occurs, sets an error status (retrievable via
  //    status()) and returns false.
  template <typename ValueFetcherImpl>
  bool AddRow(ValueFetcherImpl* fetcher) {
    static_assert(std::is_base_of_v<ValueFetcher, ValueFetcherImpl>,
                  "ValueFetcherImpl must inherit from ValueFetcher");
    PERFETTO_CHECK(current_status_.ok());

    for (uint32_t i = 0; i < column_names_.size(); ++i) {
      ColumnState& state = column_states_[i];
      typename ValueFetcherImpl::Type fetched_type = fetcher->GetValueType(i);
      switch (fetched_type) {
        case ValueFetcherImpl::kInt64: {
          auto* r = EnsureColumnType<int64_t>(i, state.data);
          if (PERFETTO_UNLIKELY(!r)) {
            return false;
          }
          r->push_back(fetcher->GetInt64Value(i));
          break;
        }
        case ValueFetcherImpl::kDouble: {
          auto* r = EnsureColumnType<double>(i, state.data);
          if (PERFETTO_UNLIKELY(!r)) {
            return false;
          }
          r->push_back(fetcher->GetDoubleValue(i));
          break;
        }
        case ValueFetcherImpl::kString: {
          auto* r = EnsureColumnType<StringPool::Id>(i, state.data);
          if (PERFETTO_UNLIKELY(!r)) {
            return false;
          }
          r->push_back(string_pool_->InternString(fetcher->GetStringValue(i)));
          break;
        }
        case ValueFetcherImpl::kNull:
          if (PERFETTO_UNLIKELY(!state.null_overlay)) {
            state.null_overlay =
                impl::BitVector::CreateWithSize(row_count_, true);
          }
          break;
      }
      if (PERFETTO_UNLIKELY(state.null_overlay)) {
        state.null_overlay->push_back(fetched_type != ValueFetcherImpl::kNull);
      }
    }  // End column loop
    row_count_++;
    return true;
  }

  // Finalizes the builder and attempts to construct the Dataframe.
  // This method consumes the builder (note the && qualifier).
  //
  // Returns:
  //   StatusOr<Dataframe>: On success, contains the built `Dataframe`.
  //                        On failure (e.g., if `AddRow` previously failed),
  //                        contains an error status retrieved from `status()`.
  //
  // Implementation wise, the collected data for each column is analyzed to:
  // - Determine the final optimal storage type (e.g., downcasting int64_t to
  //   uint32_t/int32_t if possible, using Id type if applicable).
  // - Determine the final nullability overlay (NonNull or SparseNull).
  // - Determine the final sort state (IdSorted, SetIdSorted, Sorted, Unsorted)
  //   by analyzing the collected non-null values.
  // - Construct and return the final `Dataframe` instance.
  base::StatusOr<Dataframe> Build() && {
    RETURN_IF_ERROR(current_status_);
    std::vector<impl::Column> columns;
    for (uint32_t i = 0; i < column_names_.size(); ++i) {
      auto& state = column_states_[i];
      switch (state.data.index()) {
        case base::variant_index<DataVariant, std::nullopt_t>():
          columns.emplace_back(impl::Column{
              impl::Storage{impl::FlexVector<uint32_t>()},
              CreateNullStorageFromBitvector(std::move(state.null_overlay)),
              Unsorted{}});
          break;
        case base::variant_index<DataVariant, impl::FlexVector<int64_t>>(): {
          auto& data =
              base::unchecked_get<impl::FlexVector<int64_t>>(state.data);
          bool is_id_sorted = data.empty() || data[0] == 0;
          bool is_setid_sorted = data.empty() || data[0] == 0;
          bool is_sorted = true;
          int64_t min = data.empty() ? 0 : data[0];
          int64_t max = data.empty() ? 0 : data[0];
          for (uint32_t j = 1; j < data.size(); ++j) {
            is_id_sorted = is_id_sorted && (data[j] == j);
            is_setid_sorted =
                is_setid_sorted && (data[j] == data[j - 1] || data[j] == j);
            is_sorted = is_sorted && data[j - 1] <= data[j];
            min = std::min(min, data[j]);
            max = std::max(max, data[j]);
          }
          bool is_nullable = state.null_overlay.has_value();
          columns.emplace_back(impl::Column{
              CreateIntegerStorage(std::move(data), is_id_sorted, min, max),
              CreateNullStorageFromBitvector(std::move(state.null_overlay)),
              GetIntegerSortStateFromProperties(is_nullable, is_id_sorted,
                                                is_setid_sorted, is_sorted)});
          break;
        }
        case base::variant_index<DataVariant, impl::FlexVector<double>>(): {
          auto& data =
              base::unchecked_get<impl::FlexVector<double>>(state.data);
          SortState sort_state =
              GetSortStateForDouble(state.null_overlay.has_value(), data);
          columns.emplace_back(impl::Column{
              impl::Storage{std::move(data)},
              CreateNullStorageFromBitvector(std::move(state.null_overlay)),
              sort_state});
          break;
        }
        case base::variant_index<DataVariant,
                                 impl::FlexVector<StringPool::Id>>(): {
          auto& data =
              base::unchecked_get<impl::FlexVector<StringPool::Id>>(state.data);
          SortState sort_state = GetStringSortState(
              state.null_overlay.has_value(), data, string_pool_);
          columns.emplace_back(impl::Column{
              impl::Storage{std::move(data)},
              CreateNullStorageFromBitvector(std::move(state.null_overlay)),
              sort_state});
          break;
        }
      }
    }
    return Dataframe(std::move(column_names_), std::move(columns), row_count_,
                     string_pool_);
  }

  // Returns the current status of the builder.
  //
  // If `AddRow` returned `false`, this method can be used to retrieve the
  // `base::Status` object containing the error details (e.g., type mismatch).
  //
  // Returns:
  //   const base::Status&: The current status. `ok()` will be true unless
  //                        an error occurred during a previous `AddRow` call.
  const base::Status& status() const { return current_status_; }

 private:
  using DataVariant = std::variant<std::nullopt_t,
                                   impl::FlexVector<int64_t>,
                                   impl::FlexVector<double>,
                                   impl::FlexVector<StringPool::Id>>;
  struct ColumnState {
    DataVariant data = std::nullopt;
    std::optional<impl::BitVector> null_overlay;
  };

  template <typename T>
  PERFETTO_ALWAYS_INLINE impl::FlexVector<T>* EnsureColumnType(
      uint32_t i,
      DataVariant& data) {
    switch (data.index()) {
      case base::variant_index<DataVariant, std::nullopt_t>():
        data = impl::FlexVector<T>();
        return &base::unchecked_get<impl::FlexVector<T>>(data);
      case base::variant_index<DataVariant, impl::FlexVector<T>>():
        return &base::unchecked_get<impl::FlexVector<T>>(data);
      default:
        current_status_ = base::ErrStatus(
            "Type mismatch in column '%s' at row %u. Existing type != "
            "fetched type.",
            column_names_[i].c_str(), row_count_);
        return nullptr;
    }
  }

  static impl::Storage CreateIntegerStorage(impl::FlexVector<int64_t> data,
                                            bool is_id_sorted,
                                            int64_t min,
                                            int64_t max) {
    if (is_id_sorted) {
      return impl::Storage{
          impl::Storage::Id{static_cast<uint32_t>(data.size())}};
    }
    if (IsRangeFullyRepresentableByType<uint32_t>(min, max)) {
      return impl::Storage{
          impl::Storage::Uint32{DowncastFromInt64<uint32_t>(data)}};
    }
    if (IsRangeFullyRepresentableByType<int32_t>(min, max)) {
      return impl::Storage{
          impl::Storage::Int32{DowncastFromInt64<int32_t>(data)}};
    }
    return impl::Storage{impl::Storage::Int64{std::move(data)}};
  }

  static impl::NullStorage CreateNullStorageFromBitvector(
      std::optional<impl::BitVector> bit_vector) {
    if (bit_vector) {
      return impl::NullStorage{
          impl::NullStorage::SparseNull{*std::move(bit_vector)}};
    }
    return impl::NullStorage{impl::NullStorage::NonNull{}};
  }

  template <typename T>
  static bool IsRangeFullyRepresentableByType(int64_t min, int64_t max) {
    // The <= for max is intentional because we're checking representability
    // of min/max, not looping or similar.
    PERFETTO_DCHECK(min <= max);
    return min >= std::numeric_limits<T>::min() &&
           max <= std::numeric_limits<T>::max();
  }

  template <typename T>
  static impl::FlexVector<T> DowncastFromInt64(
      const impl::FlexVector<int64_t>& data) {
    auto res = impl::FlexVector<T>::CreateWithSize(data.size());
    for (uint32_t i = 0; i < data.size(); ++i) {
      PERFETTO_DCHECK(IsRangeFullyRepresentableByType<T>(data[i], data[i]));
      res[i] = static_cast<T>(data[i]);
    }
    return res;
  }

  static SortState GetSortStateForDouble(bool is_nullable,
                                         const impl::FlexVector<double>& data) {
    if (is_nullable) {
      return SortState{Unsorted{}};
    }
    for (uint32_t i = 1; i < data.size(); ++i) {
      if (data[i - 1] > data[i]) {
        return SortState{Unsorted{}};
      }
    }
    return SortState{Sorted{}};
  }

  static SortState GetStringSortState(
      bool is_nullable,
      const impl::FlexVector<StringPool::Id>& data,
      StringPool* pool) {
    if (is_nullable) {
      return SortState{Unsorted{}};
    }
    if (!data.empty()) {
      NullTermStringView prev = pool->Get(data[0]);
      for (uint32_t i = 1; i < data.size(); ++i) {
        NullTermStringView curr = pool->Get(data[i]);
        if (prev > curr) {
          return SortState{Unsorted{}};
        }
        prev = curr;
      }
    }
    return SortState{Sorted{}};
  }

  static SortState GetIntegerSortStateFromProperties(bool is_nullable,
                                                     bool is_id_sorted,
                                                     bool is_setid_sorted,
                                                     bool is_sorted) {
    if (is_nullable) {
      return SortState{Unsorted{}};
    }
    if (is_id_sorted) {
      PERFETTO_DCHECK(is_setid_sorted);
      PERFETTO_DCHECK(is_sorted);
      return SortState{IdSorted{}};
    }
    if (is_setid_sorted) {
      PERFETTO_DCHECK(is_sorted);
      return SortState{SetIdSorted{}};
    }
    if (is_sorted) {
      return SortState{Sorted{}};
    }
    return SortState{Unsorted{}};
  }

  StringPool* string_pool_;
  uint32_t row_count_ = 0;
  std::vector<std::string> column_names_;
  std::vector<ColumnState> column_states_;
  base::Status current_status_ = base::OkStatus();
};

}  // namespace perfetto::trace_processor::dataframe

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_RUNTIME_DATAFRAME_BUILDER_H_
