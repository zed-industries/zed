#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_SPECS_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_SPECS_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "src/trace_processor/dataframe/type_set.h"

namespace perfetto::trace_processor::dataframe {

// -----------------------------------------------------------------------------
// Column value Types
// -----------------------------------------------------------------------------

// Represents values where the index of the value in the table is the same as
// the value. This allows for zero memory overhead as values don't need to be
// explicitly stored. Operations on column with this type can be highly
// optimized.
struct Id {};

// Represents values where the value is a 32-bit unsigned integer.
struct Uint32 {};

// Represents values where the value is a 32-bit signed integer.
struct Int32 {};

// Represents values where the value is a 64-bit signed integer.
struct Int64 {};

// Represents values where the value is a double.
struct Double {};

// Represents values where the value is a string.
struct String {};

// TypeSet of all possible storage value types.
using StorageType = TypeSet<Id, Uint32, Int32, Int64, Double, String>;

// -----------------------------------------------------------------------------
// Operation Types
// -----------------------------------------------------------------------------

// Filters only cells which compare equal to the given value.
struct Eq {};

// Filters only cells which do not compare equal to the given value.
struct Ne {};

// Filters only cells which are less than the given value.
struct Lt {};

// Filters only cells which are less than or equal to the given value.
struct Le {};

// Filters only cells which are greater than the given value.
struct Gt {};

// Filters only cells which are greater than or equal to the given value.
struct Ge {};

// Filters only cells which match the given glob pattern.
struct Glob {};

// Filters only cells which match the given regex pattern.
struct Regex {};

// Filters only cells which are not NULL.
struct IsNotNull {};

// Filters only cells which are NULL.
struct IsNull {};

// TypeSet of all possible operations for filter conditions.
using Op = TypeSet<Eq, Ne, Lt, Le, Gt, Ge, Glob, Regex, IsNotNull, IsNull>;

// -----------------------------------------------------------------------------
// Sort State Types
// -----------------------------------------------------------------------------

// Represents a column sorted by its id property.
// This is a special state that should only be applied to Id columns, indicating
// the natural ordering where indices equal values.
struct IdSorted {};

// Represents a column which has two properties:
// 1) is sorted in ascending order
// 2) for each unique value `v` in the column, the first occurrence of `v` is
//    at index `v` in the column.
//
// In essence, this means that the columns end up looking like:
// [0, 0, 0, 3, 3, 5, 5, 7, 7, 7, 10]
//
// This state can only be applied to Uint32 columns.
struct SetIdSorted {};

// Represents a column which is sorted in ascending order by its value.
struct Sorted {};

// Represents a column which is not sorted.
struct Unsorted {};

// TypeSet of all possible column sort states.
using SortState = TypeSet<IdSorted, SetIdSorted, Sorted, Unsorted>;

// -----------------------------------------------------------------------------
// Nullability Types
// -----------------------------------------------------------------------------

// Represents a column that doesn't contain NULL values.
struct NonNull {};

// Represents a column that contains NULL values with the storage only
// containing data for non-NULL values.
struct SparseNull {};

// Represents a column that contains NULL values with the storage containing
// data for all values (with undefined values at positions that would be NULL).
struct DenseNull {};

// TypeSet of all possible column nullability states.
using Nullability = TypeSet<NonNull, SparseNull, DenseNull>;

// -----------------------------------------------------------------------------
// Filter Specifications
// -----------------------------------------------------------------------------

// Specifies a filter operation to be applied to column data.
// This is used to generate query plans for filtering rows.
struct FilterSpec {
  // Index of the column in the dataframe to filter.
  uint32_t col;

  // Original index from the client query (used for tracking).
  uint32_t source_index;

  // Operation to apply (e.g., equality).
  Op op;

  // Output parameter: index for the filter value in query execution.
  // This is populated during query planning.
  std::optional<uint32_t> value_index;
};

// -----------------------------------------------------------------------------
// Distinct Specifications
// -----------------------------------------------------------------------------

// Specifies a distinct operation to be applied to the dataframe rows.
struct DistinctSpec {
  // Index of the column in the dataframe to perform a distinct on.
  uint32_t col;
};

// -----------------------------------------------------------------------------
// Sort Specifications
// -----------------------------------------------------------------------------

// Defines the direction for sorting.
enum class SortDirection : uint32_t {
  kAscending,
  kDescending,
};

// Specifies a sort operation to be applied to the dataframe rows.
struct SortSpec {
  // Index of the column in the dataframe to sort by.
  uint32_t col;

  // Direction of the sort (ascending or descending).
  SortDirection direction;
};

// -----------------------------------------------------------------------------
// Limit Specification
// -----------------------------------------------------------------------------

// Specifies limit and offset parameters for a query.
struct LimitSpec {
  std::optional<uint32_t> limit;
  std::optional<uint32_t> offset;
};

}  // namespace perfetto::trace_processor::dataframe

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_SPECS_H_
