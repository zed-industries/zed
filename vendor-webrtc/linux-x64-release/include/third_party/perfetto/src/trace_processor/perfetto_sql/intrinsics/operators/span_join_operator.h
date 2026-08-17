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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_SPAN_JOIN_OPERATOR_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_SPAN_JOIN_OPERATOR_H_

#include <cstddef>
#include <cstdint>
#include <limits>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_splitter.h"
#include "perfetto/ext/base/string_utils.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/sqlite/bindings/sqlite_module.h"
#include "src/trace_processor/sqlite/sqlite_engine.h"

namespace perfetto::trace_processor {

class PerfettoSqlEngine;
struct SpanJoinOperatorModule;

// Implements the SPAN JOIN operation between two tables on a particular column.
//
// Span:
// A span is a row with a timestamp and a duration. It is used to model
// operations which run for a particular *span* of time.
//
// We draw spans like so (time on the x-axis):
// start of span->[ time where operation is running ]<- end of span
//
// Multiple spans can happen in parallel:
// [      ]
//    [        ]
//   [                    ]
//  [ ]
//
// The above for example, models scheduling activity on a 4-core computer for a
// short period of time.
//
// Span join:
// The span join operation can be thought of as the intersection of span tables.
// That is, the join table has a span for each pair of spans in the child tables
// where the spans overlap. Because many spans are possible in parallel, an
// extra metadata column (labelled the "join column") is used to distinguish
// between the spanned tables.
//
// For a given join key suppose these were the two span tables:
// Table 1:   [        ]              [      ]         [ ]
// Table 2:          [      ]            [  ]           [      ]
// Output :          [ ]                 [  ]           []
//
// All other columns apart from timestamp (ts), duration (dur) and the join key
// are passed through unchanged.
struct SpanJoinOperatorModule : public sqlite::Module<SpanJoinOperatorModule> {
 public:
  static constexpr uint32_t kSourceGeqOpCode =
      SQLITE_INDEX_CONSTRAINT_FUNCTION + 1;

  struct Vtab;

  // Enum indicating whether the queries on the two inner tables should
  // emit shadows.
  enum class EmitShadowType {
    // Used when the table should emit all shadow slices (both present and
    // missing partition shadows).
    kAll,

    // Used when the table should only emit shadows for partitions which are
    // present.
    kPresentPartitionOnly,

    // Used when the table should emit no shadow slices.
    kNone,
  };

  // Parsed version of a table descriptor.
  struct TableDescriptor {
    static base::Status Parse(const std::string& raw_descriptor,
                              TableDescriptor* descriptor);

    bool IsPartitioned() const { return !partition_col.empty(); }

    std::string name;
    std::string partition_col;
  };

  // Contains the definition of the child tables.
  class TableDefinition {
   public:
    TableDefinition() = default;

    TableDefinition(std::string name,
                    std::string partition_col,
                    std::vector<std::pair<SqlValue::Type, std::string>> cols,
                    EmitShadowType emit_shadow_type,
                    uint32_t ts_idx,
                    std::optional<uint32_t> dur_idx,
                    uint32_t partition_idx);

    static base::Status Create(PerfettoSqlEngine* engine,
                               const TableDescriptor& desc,
                               EmitShadowType emit_shadow_type,
                               TableDefinition* defn);

    // Creates an SQL query from the constraints and index,
    std::string CreateSqlQuery(base::StringSplitter&,
                               sqlite3_value** argv) const;

    // Creates the section of the "CREATE TABLE" corresponding to this
    // definition.
    std::string CreateVtabCreateTableSection() const;

    // Returns whether this table should emit present partition shadow slices.
    bool ShouldEmitPresentPartitionShadow() const {
      return emit_shadow_type_ == EmitShadowType::kAll ||
             emit_shadow_type_ == EmitShadowType::kPresentPartitionOnly;
    }

    // Returns whether this table should emit missing partition shadow slices.
    bool ShouldEmitMissingPartitionShadow() const {
      return emit_shadow_type_ == EmitShadowType::kAll;
    }

    // Returns whether the table is partitioned.
    bool IsPartitioned() const { return !partition_col_.empty(); }

    const std::string& name() const { return name_; }
    const std::string& partition_col() const { return partition_col_; }
    const std::vector<std::pair<SqlValue::Type, std::string>>& columns() const {
      return cols_;
    }

    uint32_t ts_idx() const { return ts_idx_; }
    std::optional<uint32_t> dur_idx() const { return dur_idx_; }
    uint32_t partition_idx() const { return partition_idx_; }

   private:
    EmitShadowType emit_shadow_type_ = EmitShadowType::kNone;

    std::string name_;
    std::string partition_col_;
    std::vector<std::pair<SqlValue::Type, std::string>> cols_;

    uint32_t ts_idx_ = std::numeric_limits<uint32_t>::max();
    std::optional<uint32_t> dur_idx_;
    uint32_t partition_idx_ = std::numeric_limits<uint32_t>::max();
  };

  // Stores information about a single subquery into one of the two child
  // tables.
  //
  // This class is implemented as a state machine which steps from one slice to
  // the next.
  class Query {
   public:
    // Enum encoding the current state of the query in the state machine.
    enum class State {
      // Encodes that the current slice is a real slice (i.e. comes directly
      // from the cursor).
      kReal,

      // Encodes that the current slice is on a partition for which there is a
      // real slice present.
      kPresentPartitionShadow,

      // Encodes that the current slice is on a partition(s) for which there is
      // no real slice for those partition(s).
      kMissingPartitionShadow,

      // Encodes that this query has reached the end.
      kEof,
    };

    Query(Vtab*, const TableDefinition*);
    virtual ~Query();

    Query(Query&&) noexcept = default;
    Query& operator=(Query&&) = default;

    enum class InitialEofBehavior {
      kTreatAsEof,
      kTreatAsMissingPartitionShadow
    };

    // Initializes the query with the given constraints and query parameters.
    base::Status Initialize(
        std::string sql,
        InitialEofBehavior eof_behavior = InitialEofBehavior::kTreatAsEof);

    // Forwards the query to the next valid slice.
    base::Status Next();

    // Rewinds the query to the first valid slice
    // This is used in the mixed partitioning case where the query with no
    // partitions is rewound to the start on every new partition.
    base::Status Rewind();

    // Reports the column at the given index to given context.
    void ReportSqliteResult(sqlite3_context* context, size_t index);

    // Returns whether the cursor has reached eof.
    bool IsEof() const { return state_ == State::kEof; }

    // Returns whether the current slice pointed to is a real slice.
    bool IsReal() const { return state_ == State::kReal; }

    // Returns the first partition this slice covers (for real/single partition
    // shadows, this is the same as partition()).
    // This partition encodes a [start, end] (closed at start and at end) range
    // of partitions which works as the partitions are integers.
    int64_t FirstPartition() const {
      PERFETTO_DCHECK(!IsEof());
      return IsMissingPartitionShadow() ? missing_partition_start_
                                        : partition();
    }

    // Returns the last partition this slice covers (for real/single partition
    // shadows, this is the same as partition()).
    // This partition encodes a [start, end] (closed at start and at end) range
    // of partitions which works as the partitions are integers.
    int64_t LastPartition() const {
      PERFETTO_DCHECK(!IsEof());
      return IsMissingPartitionShadow() ? missing_partition_end_ - 1
                                        : partition();
    }

    // Returns the end timestamp of this slice adjusted to ensure that -1
    // duration slices always returns ts.
    int64_t AdjustedTsEnd() const {
      PERFETTO_DCHECK(!IsEof());
      return ts_end_ - ts() == -1 ? ts() : ts_end_;
    }

    int64_t ts() const {
      PERFETTO_DCHECK(!IsEof());
      return ts_;
    }
    int64_t partition() const {
      PERFETTO_DCHECK(!IsEof() && defn_->IsPartitioned());
      return partition_;
    }

    int64_t raw_ts_end() const {
      PERFETTO_DCHECK(!IsEof());
      return ts_end_;
    }

    const TableDefinition* definition() const { return defn_; }

   private:
    Query(Query&) = delete;
    Query& operator=(const Query&) = delete;

    // Returns whether the current slice pointed to is a valid slice.
    bool IsValidSlice();

    // Forwards the query to the next valid slice.
    base::Status FindNextValidSlice();

    // Advances the query state machine by one slice.
    base::Status NextSliceState();

    // Forwards the cursor to point to the next real slice.
    base::Status CursorNext();

    // Returns whether the current slice pointed to is a present partition
    // shadow.
    bool IsPresentPartitionShadow() const {
      return state_ == State::kPresentPartitionShadow;
    }

    // Returns whether the current slice pointed to is a missing partition
    // shadow.
    bool IsMissingPartitionShadow() const {
      return state_ == State::kMissingPartitionShadow;
    }

    // Returns whether the current slice pointed to is an empty shadow.
    bool IsEmptyShadow() const {
      PERFETTO_DCHECK(!IsEof());
      return (!IsReal() && ts_ == ts_end_) ||
             (IsMissingPartitionShadow() &&
              missing_partition_start_ == missing_partition_end_);
    }

    int64_t CursorTs() const {
      PERFETTO_DCHECK(!cursor_eof_);
      auto ts_idx = static_cast<int>(defn_->ts_idx());
      return sqlite3_column_int64(stmt_->sqlite_stmt(), ts_idx);
    }

    int64_t CursorDur() const {
      PERFETTO_DCHECK(!cursor_eof_);
      if (!defn_->dur_idx().has_value()) {
        return 0;
      }
      auto dur_idx = static_cast<int>(defn_->dur_idx().value());
      return sqlite3_column_int64(stmt_->sqlite_stmt(), dur_idx);
    }

    int64_t CursorPartition() const {
      PERFETTO_DCHECK(!cursor_eof_);
      PERFETTO_DCHECK(defn_->IsPartitioned());
      auto partition_idx = static_cast<int>(defn_->partition_idx());
      return sqlite3_column_int64(stmt_->sqlite_stmt(), partition_idx);
    }

    State state_ = State::kMissingPartitionShadow;
    bool cursor_eof_ = false;

    // Only valid when |state_| != kEof.
    int64_t ts_ = 0;
    int64_t ts_end_ = std::numeric_limits<int64_t>::max();

    // Only valid when |state_| == kReal or |state_| == kPresentPartitionShadow.
    int64_t partition_ = std::numeric_limits<int64_t>::min();

    // Only valid when |state_| == kMissingPartitionShadow.
    int64_t missing_partition_start_ = 0;
    int64_t missing_partition_end_ = 0;

    std::string sql_query_;
    std::optional<SqliteEngine::PreparedStatement> stmt_;

    const TableDefinition* defn_ = nullptr;
    Vtab* vtab_ = nullptr;
  };

  // Columns of the span operator table.
  enum Column {
    kTimestamp = 0,
    kDuration = 1,
    kPartition = 2,
    // All other columns are dynamic depending on the joined tables.
  };

  // Enum indicating the possible partitionings of the two tables in span join.
  enum class PartitioningType {
    // Used when both tables don't have a partition specified.
    kNoPartitioning = 0,

    // Used when both tables have the same partition specified.
    kSamePartitioning = 1,

    // Used when one table has a partition and the other table doesn't.
    kMixedPartitioning = 2
  };

  // Identifier for a column by index in a given table.
  struct ColumnLocator {
    const TableDefinition* defn;
    size_t col_index;
  };

  struct Context {
    explicit Context(PerfettoSqlEngine* _engine) : engine(_engine) {}

    PerfettoSqlEngine* engine;
  };
  struct Vtab : public sqlite3_vtab {
    bool IsLeftJoin() const {
      return base::CaseInsensitiveEqual(module_name, "span_left_join");
    }
    bool IsOuterJoin() const {
      return base::CaseInsensitiveEqual(module_name, "span_outer_join");
    }

    const std::string& partition_col() const {
      return t1_defn.IsPartitioned() ? t1_defn.partition_col()
                                     : t2_defn.partition_col();
    }

    std::string GetNameForGlobalColumnIndex(const TableDefinition& defn,
                                            int global_column);

    std::string BestIndexStrForDefinition(const sqlite3_index_info* info,
                                          const TableDefinition& defn);

    void PopulateColumnLocatorMap(uint32_t);

    PerfettoSqlEngine* engine;
    std::string module_name;
    std::string create_table_stmt;
    TableDefinition t1_defn;
    TableDefinition t2_defn;
    PartitioningType partitioning;
    base::FlatHashMap<size_t, ColumnLocator> global_index_to_column_locator;
  };

  // Base class for a cursor on the span table.
  struct Cursor final : public sqlite3_vtab_cursor {
    explicit Cursor(Vtab* _vtab)
        : t1(_vtab, &_vtab->t1_defn), t2(_vtab, &_vtab->t2_defn), vtab(_vtab) {}

    bool IsOverlappingSpan() const;
    base::Status FindOverlappingSpan();
    Query* FindEarliestFinishQuery();

    Query t1;
    Query t2;

    Query* next_query = nullptr;

    // Only valid for kMixedPartition.
    int64_t last_mixed_partition_ = std::numeric_limits<int64_t>::min();

    Vtab* vtab;
  };

  static constexpr bool kSupportsWrites = false;

  static int Create(sqlite3*,
                    void*,
                    int,
                    const char* const*,
                    sqlite3_vtab**,
                    char**);
  static int Destroy(sqlite3_vtab*);

  static int Connect(sqlite3*,
                     void*,
                     int,
                     const char* const*,
                     sqlite3_vtab**,
                     char**);
  static int Disconnect(sqlite3_vtab*);

  static int BestIndex(sqlite3_vtab*, sqlite3_index_info*);

  static int Open(sqlite3_vtab*, sqlite3_vtab_cursor**);
  static int Close(sqlite3_vtab_cursor*);

  static int Filter(sqlite3_vtab_cursor*,
                    int,
                    const char*,
                    int,
                    sqlite3_value**);
  static int Next(sqlite3_vtab_cursor*);
  static int Eof(sqlite3_vtab_cursor*);
  static int Column(sqlite3_vtab_cursor*, sqlite3_context*, int);
  static int Rowid(sqlite3_vtab_cursor*, sqlite_int64*);

  static int FindFunction(sqlite3_vtab*,
                          int,
                          const char*,
                          FindFunctionFn**,
                          void**);

  // This needs to happen at the end as it depends on the functions
  // defined above.
  static constexpr sqlite3_module kModule = CreateModule();
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_SPAN_JOIN_OPERATOR_H_
