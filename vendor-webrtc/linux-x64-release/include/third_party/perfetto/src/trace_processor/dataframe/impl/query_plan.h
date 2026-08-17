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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_QUERY_PLAN_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_QUERY_PLAN_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/base64.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/dataframe/impl/bytecode_core.h"
#include "src/trace_processor/dataframe/impl/bytecode_instructions.h"
#include "src/trace_processor/dataframe/impl/bytecode_registers.h"
#include "src/trace_processor/dataframe/impl/slab.h"
#include "src/trace_processor/dataframe/impl/types.h"
#include "src/trace_processor/dataframe/specs.h"
#include "src/trace_processor/util/status_macros.h"

namespace perfetto::trace_processor::dataframe::impl {

static constexpr uint32_t kMaxColumns = 64;

// A QueryPlan encapsulates all the information needed to execute a query,
// including the bytecode instructions and interpreter configuration.
struct QueryPlan {
  // Contains various parameters required for execution of this query plan.
  struct ExecutionParams {
    // The maximum number of rows it's possible for this query plan to return.
    uint32_t max_row_count = 0;

    // The number of rows this query plan estimates it will return.
    uint32_t estimated_row_count = 0;

    // An estimate for the cost of executing the query plan.
    double estimated_cost = 0;

    // Number of filter values used by this query.
    uint32_t filter_value_count = 0;

    // Register holding the final filtered indices.
    bytecode::reg::ReadHandle<Span<uint32_t>> output_register;

    // Maps column indices to output offsets.
    std::array<uint32_t, kMaxColumns> col_to_output_offset;

    // Number of output indices per row.
    uint32_t output_per_row = 0;
  };
  static_assert(std::is_trivially_copyable_v<ExecutionParams>);
  static_assert(std::is_trivially_destructible_v<ExecutionParams>);

  // Serializes the query plan to a Base64-encoded string.
  // This allows plans to be stored or transmitted between processes.
  std::string Serialize() const {
    size_t size = sizeof(size_t) +
                  (bytecode.size() * sizeof(bytecode::Bytecode)) +
                  sizeof(params);
    std::string res(size, '\0');
    char* p = res.data();
    {
      size_t bytecode_size = bytecode.size();
      memcpy(p, &bytecode_size, sizeof(bytecode_size));
      p += sizeof(bytecode_size);
    }
    {
      memcpy(p, bytecode.data(), bytecode.size() * sizeof(bytecode::Bytecode));
      p += bytecode.size() * sizeof(bytecode::Bytecode);
    }
    {
      memcpy(p, &params, sizeof(params));
      p += sizeof(params);
    }
    PERFETTO_CHECK(p == res.data() + res.size());
    return base::Base64Encode(base::StringView(res));
  }

  // Deserializes a query plan from a Base64-encoded string.
  // Returns the reconstructed QueryPlan.
  static QueryPlan Deserialize(std::string_view serialized) {
    QueryPlan res;
    std::optional<std::string> raw_data = base::Base64Decode(
        base::StringView(serialized.data(), serialized.size()));
    PERFETTO_CHECK(raw_data);
    const char* p = raw_data->data();
    size_t bytecode_size;
    {
      memcpy(&bytecode_size, p, sizeof(bytecode_size));
      p += sizeof(bytecode_size);
    }
    {
      for (uint32_t i = 0; i < bytecode_size; ++i) {
        res.bytecode.emplace_back();
      }
      memcpy(res.bytecode.data(), p,
             bytecode_size * sizeof(bytecode::Bytecode));
      p += bytecode_size * sizeof(bytecode::Bytecode);
    }
    {
      memcpy(&res.params, p, sizeof(res.params));
      p += sizeof(res.params);
    }
    PERFETTO_CHECK(p == raw_data->data() + raw_data->size());
    return res;
  }

  ExecutionParams params;
  bytecode::BytecodeVector bytecode;
};

// Builder class for creating query plans.
//
// QueryPlans contain the bytecode instructions and interpreter configuration
// needed to execute a query.
class QueryPlanBuilder {
 public:
  static base::StatusOr<QueryPlan> Build(
      uint32_t row_count,
      const std::vector<Column>& columns,
      std::vector<FilterSpec>& specs,
      const std::vector<DistinctSpec>& distinct,
      const std::vector<SortSpec>& sort_specs,
      const LimitSpec& limit_spec,
      uint64_t cols_used) {
    QueryPlanBuilder builder(row_count, columns);
    RETURN_IF_ERROR(builder.Filter(specs));
    builder.Distinct(distinct);
    if (builder.CanUseMinMaxOptimization(sort_specs, limit_spec)) {
      builder.MinMax(sort_specs[0]);
      builder.Output({}, cols_used);
    } else {
      builder.Sort(sort_specs);
      builder.Output(limit_spec, cols_used);
    }
    return std::move(builder).Build();
  }

 private:
  // Represents register types for holding indices.
  using IndicesReg = std::variant<bytecode::reg::RwHandle<Range>,
                                  bytecode::reg::RwHandle<Span<uint32_t>>>;

  // Indicates that the bytecode does not change the estimated or maximum number
  // of rows.
  struct UnchangedRowCount {};

  // Indicates that the bytecode reduces the estimated number of rows by 2.
  struct Div2RowCount {};

  // Indicates that the bytecode reduces the estimated number of rows by 2 *
  // log(row_count).
  struct DoubleLog2RowCount {};

  // Indicates that the bytecode produces *exactly* one row and the estimated
  // and maximum should be set to 1.
  struct OneRowCount {};

  // Indicates that the bytecode produces *exactly* zero rows and the estimated
  // and maximum should be set to 0.
  struct ZeroRowCount {};

  // Indicates that the bytecode produces `limit` rows starting at `offset`.
  struct LimitOffsetRowCount {
    uint32_t limit;
    uint32_t offset;
  };
  using RowCountModifier = std::variant<UnchangedRowCount,
                                        Div2RowCount,
                                        DoubleLog2RowCount,
                                        OneRowCount,
                                        ZeroRowCount,
                                        LimitOffsetRowCount>;

  // State information for a column during query planning.
  struct ColumnState {
    std::optional<bytecode::reg::RwHandle<Slab<uint32_t>>> prefix_popcount;
  };

  // Constructs a builder for the given number of rows and columns.
  QueryPlanBuilder(uint32_t row_count, const std::vector<Column>& columns);

  // Adds filter operations to the query plan based on filter specifications.
  // Optimizes the order of filters for efficiency.
  base::Status Filter(std::vector<FilterSpec>& specs);

  // Adds distinct operations to the query plan based on distinct
  // specifications. Distinct are applied after filters, in reverse order of
  // specification.
  void Distinct(const std::vector<DistinctSpec>& distinct_specs);

  // Adds min/max operations to the query plan given a single column which
  // should be sorted on.
  void MinMax(const SortSpec& spec);

  // Adds sort operations to the query plan based on sort specifications.
  // Sorts are applied after filters and disinct.
  void Sort(const std::vector<SortSpec>& sort_specs);

  // Configures output handling for the filtered rows.
  // |cols_used_bitmap| is a bitmap with bits set for columns that will be
  // accessed.
  void Output(const LimitSpec&, uint64_t cols_used_bitmap);

  // Finalizes and returns the built query plan.
  QueryPlan Build() &&;

  // Processes non-string filter constraints.
  void NonStringConstraint(
      const FilterSpec& c,
      const NonStringType& type,
      const NonStringOp& op,
      const bytecode::reg::ReadHandle<CastFilterValueResult>& result);

  // Processes string filter constraints.
  base::Status StringConstraint(
      const FilterSpec& c,
      const StringOp& op,
      const bytecode::reg::ReadHandle<CastFilterValueResult>& result);

  // Processes null filter constraints.
  void NullConstraint(const NullOp&, FilterSpec&);

  // Attempts to apply optimized filtering on sorted data.
  // Returns true if the optimization was applied.
  bool TrySortedConstraint(
      const FilterSpec& fs,
      const StorageType& ct,
      const NonNullOp& op,
      const bytecode::reg::RwHandle<CastFilterValueResult>& result);

  // Adds overlay translation for handling special column properties like
  // nullability.
  bytecode::reg::RwHandle<Span<uint32_t>> MaybeAddOverlayTranslation(
      const FilterSpec& c);

  // Ensures indices are stored in a Slab, converting from Range if necessary.
  PERFETTO_NO_INLINE bytecode::reg::RwHandle<Span<uint32_t>>
  EnsureIndicesAreInSlab();

  // Adds a new bytecode instruction of type T to the plan.
  template <typename T>
  T& AddOpcode(RowCountModifier rc) {
    return AddOpcode<T>(bytecode::Index<T>(), rc, T::kCost);
  }

  // Adds a new bytecode instruction of type T with the given option value.
  template <typename T>
  T& AddOpcode(uint32_t option, RowCountModifier rc) {
    return static_cast<T&>(AddRawOpcode(option, rc, T::kCost));
  }

  // Adds a new bytecode instruction of type T with the given option value.
  template <typename T>
  T& AddOpcode(uint32_t option, RowCountModifier rc, bytecode::Cost cost) {
    return static_cast<T&>(AddRawOpcode(option, rc, cost));
  }

  PERFETTO_NO_INLINE bytecode::Bytecode& AddRawOpcode(uint32_t option,
                                                      RowCountModifier rc,
                                                      bytecode::Cost cost);

  // Sets the result to an empty set. Use when a filter guarantees no matches.
  void SetGuaranteedToBeEmpty();

  // Returns the prefix popcount register for the given column.
  bytecode::reg::ReadHandle<Slab<uint32_t>> PrefixPopcountRegisterFor(
      uint32_t col);

  bool CanUseMinMaxOptimization(const std::vector<SortSpec>&, const LimitSpec&);

  // Reference to the columns being queried.
  const std::vector<Column>& columns_;

  // The query plan being built.
  QueryPlan plan_;

  // State information for each column during planning.
  std::vector<ColumnState> column_states_;

  // Number of registers allocated so far.
  uint32_t register_count_ = 0;

  // Current register holding the set of matching indices.
  IndicesReg indices_reg_;
};

}  // namespace perfetto::trace_processor::dataframe::impl

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_QUERY_PLAN_H_
