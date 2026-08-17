/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_DATA_LAYER_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_DATA_LAYER_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {
class DataLayerChain;

// Data structure which either directly or indirectly (i.e. by transforming
// the contents of another DataLayer) provides the data of a column of a
// table.
class DataLayer : public RefCounted {
 public:
  // Arguments for MakeChain on how the inner chain should be interpreted.
  struct ChainCreationArgs {
    constexpr explicit ChainCreationArgs(
        bool _does_layer_order_chain_contents = false)
        : does_layer_order_chain_contents(_does_layer_order_chain_contents) {}

    // Indicates whether the current data layer orders the inner chain.
    // Currently used by ArrangementOverlay to decide whether the arrangement
    // orders a given chain.
    bool does_layer_order_chain_contents;
  };
  virtual ~DataLayer();

  // Creates a DataLayerChain for a terminal DataLayer. This means the
  // DataLayer directly should return the data it contains inside.
  std::unique_ptr<DataLayerChain> MakeChain();

  // Creates a DataLayerChain for a non-terminal DataLayer. This means
  // the DataLayer should transform the contents of the inner chain.
  std::unique_ptr<DataLayerChain> MakeChain(
      std::unique_ptr<DataLayerChain>,
      ChainCreationArgs = ChainCreationArgs());

 protected:
  // TODO(b/325583551): remove this when possible.
  enum class Impl {
    kArrangement,
    kDenseNull,
    kDummy,
    kId,
    kNull,
    kNumericDouble,
    kNumericUint32,
    kNumericInt32,
    kNumericInt64,
    kRange,
    kSelector,
    kSetId,
    kString,
  };
  explicit DataLayer(Impl impl) : impl_(impl) {}

 private:
  Impl impl_;
};

// Corresponds to a series of DataLayer chained together. Provides
// functionality for querying the transformed data of the entire chain.
class DataLayerChain {
 public:
  // Index vector related data required to Filter using IndexSearch.
  struct Indices {
    enum class State {
      // We can't guarantee that data is in monotonic order.
      kNonmonotonic,
      // Data is in monotonic order.
      kMonotonic,
    };
    static Indices Create(const std::vector<uint32_t>& raw, State state) {
      std::vector<Token> tokens;
      tokens.reserve(raw.size());
      for (auto r : raw) {
        tokens.push_back({r, r});
      }
      return Indices{std::move(tokens), state};
    }
    static Indices CreateWithIndexPayloadForTesting(
        const std::vector<uint32_t>& raw,
        State state) {
      std::vector<Token> tokens;
      tokens.reserve(raw.size());
      for (uint32_t i = 0; i < raw.size(); ++i) {
        tokens.push_back(Token{raw[i], i});
      }
      return Indices{std::move(tokens), state};
    }
    std::vector<Token> tokens;
    State state = State::kNonmonotonic;
  };

  // Index vector related data required to Filter using IndexSearch.
  struct OrderedIndices {
    const uint32_t* data = nullptr;
    uint32_t size = 0;
    Indices::State state = Indices::State::kNonmonotonic;
  };

  virtual ~DataLayerChain();

  // Start of public API.

  // Checks whether element at the provided index match |op| and |value|.
  //
  // Returns true if the element matches, false otherwise.
  virtual SingleSearchResult SingleSearch(FilterOp op,
                                          SqlValue value,
                                          uint32_t row) const = 0;

  // Searches for elements which match |op| and |value| between |range.start|
  // and |range.end|.
  //
  // Returns either a range or BitVector which indicate the positions in
  // |range| which match the constraint. If a BitVector is returned, it will
  // be *precisely* as large as |range.end|.
  //
  // Notes for callers:
  //  * Callers should note that the return value of this function corresponds
  //    to positions in the storage.
  //
  // Notes for implementors:
  //  * Implementations should ensure that the return value is empty or *only*
  //    includes positions in |range|. Callers are free to assume this and can
  //    optimize based on it.
  //  * Implementations should ensure that, if they return a BitVector, it is
  //    precisely of size |range.end|.
  PERFETTO_ALWAYS_INLINE RangeOrBitVector Search(FilterOp op,
                                                 SqlValue value,
                                                 Range range) const {
    PERFETTO_DCHECK(range.end <= size());
    switch (ValidateSearchConstraints(op, value)) {
      case SearchValidationResult::kAllData:
        return RangeOrBitVector(range);
      case SearchValidationResult::kNoData:
        return RangeOrBitVector(Range());
      case SearchValidationResult::kOk:
        return SearchValidated(op, value, range);
    }
    PERFETTO_FATAL("For GCC");
  }

  // Searches for elements which match |op| and |value| at the positions given
  // by |indices| array.
  //
  // Returns either a range of BitVector which indicate the positions in
  // |indices| which match the constraint. If a BitVector is returned, it will
  // be *precisely* as large as |indices_count|.
  //
  // Notes for callers:
  //  * Callers should note that the return value of this function corresponds
  //    to positions in |indices| *not* positions in the storage.
  //
  // Notes for implementors:
  //  * Implementations should ensure that, if they return a BitVector, it is
  //    precisely of size |indices_count|.
  PERFETTO_ALWAYS_INLINE void IndexSearch(FilterOp op,
                                          SqlValue value,
                                          Indices& indices) const {
    switch (ValidateSearchConstraints(op, value)) {
      case SearchValidationResult::kAllData:
        return;
      case SearchValidationResult::kNoData:
        indices.tokens.clear();
        return;
      case SearchValidationResult::kOk:
        IndexSearchValidated(op, value, indices);
        return;
    }
    PERFETTO_FATAL("For GCC");
  }

  // Searches for elements which match |op| and |value| at the positions given
  // by OrderedIndicesdata.
  //
  // Returns a Range into OrderedIndicesdata of OrderedIndicesthat pass the
  // constraint.
  //
  // Notes for callers:
  //  * Should not be called on:
  //    - kGlob and kRegex as those operations can't use the sorted state
  //      hence they can't return a Range.
  //    - kNe as this is inherently unsorted. Use kEq and then reverse the
  //      result.
  //  * Callers should note that the return value of this function corresponds
  //    to positions in |indices| *not* positions in the storage.
  PERFETTO_ALWAYS_INLINE Range
  OrderedIndexSearch(FilterOp op,
                     SqlValue value,
                     const OrderedIndices& indices) const {
    switch (ValidateSearchConstraints(op, value)) {
      case SearchValidationResult::kAllData:
        return {0, indices.size};
      case SearchValidationResult::kNoData:
        return {};
      case SearchValidationResult::kOk:
        return OrderedIndexSearchValidated(op, value, indices);
    }
    PERFETTO_FATAL("For GCC");
  }

  // Stable sorts an array of Token elements between |start| and |end|
  // using a comparator defined by looking up the elements in this chain using
  // the index given by Token::index. |direction| indicates the direction of
  // the sort (ascending or descending).
  //
  // In simple terms the expectation is for implementations do something like:
  // ```
  // std::stable_sort(start, index, [](const Token& a, const Token& b) {
  //  return Get(a.index) < Get(b.index);
  // });
  // ```
  // with |Get| being a function to lookup the element in this chain.
  virtual void StableSort(Token* start,
                          Token* end,
                          SortDirection direction) const = 0;

  // Removes all indices pointing to values that are duplicates, as a result the
  // indices will only point to distinct (not duplicated) values.
  //
  // Notes for implementors:
  // * Each layer that might introduce duplicates is responsible for removing
  // them.
  virtual void Distinct(Indices&) const = 0;

  // After calling this function Indices will have at most one element. If
  // present it will point to the first index with the largest value in the
  // chain.
  virtual std::optional<Token> MaxElement(Indices&) const = 0;

  // After calling this function Indices will have at most one element. If
  // present it will point to the first index with the smallest value in the
  // chain.
  virtual std::optional<Token> MinElement(Indices&) const = 0;

  // Returns a string which represents the column for debugging purposes.
  //
  // Warning: the format of the string returned by this class is *not* stable
  // and should be relied upon for anything except printing for debugging
  // purposes.
  virtual std::string DebugString() const = 0;

  // Number of elements in stored data.
  virtual uint32_t size() const = 0;

  // End of public API.
  // The below methods might be public but are only intended for implementations
  // of DataLayerChain.

  // Verifies whether any further filtering is needed and if not, whether the
  // search would return all values or none of them. This allows for skipping
  // the |Search| and |IndexSearch| in special cases.
  //
  // Notes for callers:
  // * The SqlValue and FilterOp have to be valid in Sqlite: it will crash if
  //   either: value is NULL and operation is different than "IS NULL" and "IS
  //   NOT NULL" or the operation is "IS NULL" or "IS NOT NULL" and value is
  //   different than NULL.
  virtual SearchValidationResult ValidateSearchConstraints(FilterOp,
                                                           SqlValue) const = 0;

  // Post-validated implementation of |Search|. See |Search|'s documentation.
  virtual RangeOrBitVector SearchValidated(FilterOp, SqlValue, Range) const = 0;

  // Post-validated implementation of |IndexSearch|. See |IndexSearch|'s
  // documentation.
  virtual void IndexSearchValidated(FilterOp, SqlValue, Indices&) const = 0;

  // Post-validated implementation of |OrderedIndexSearch|. See
  // |OrderedIndexSearch|'s documentation.
  Range OrderedIndexSearchValidated(FilterOp op,
                                    SqlValue value,
                                    const OrderedIndices& indices) const;

  // Returns the SqlValue representing the value at a given index.
  //
  // This function might be very tempting to use as it appears cheap on the
  // surface but because of how DataLayerChains might be layered on top of each
  // other, this might require *several* virtual function calls per index.
  // If you're tempted to use this, please consider instead create a new
  // "vectorized" function instead and only using this as a last resort.
  //
  // The correct "class" of algorithms to use this function are cases where you
  // have a set of indices you want to lookup and based on the value returned
  // you will only use a fraction of them. In this case, it might be worth
  // paying the non-vectorized lookup to vastly reduce how many indices need
  // to be translated.
  //
  // An example of such an algorithm is binary search on indices.
  virtual SqlValue Get_AvoidUsingBecauseSlow(uint32_t index) const = 0;
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_DATA_LAYER_H_
