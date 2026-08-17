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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_DENSE_NULL_OVERLAY_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_DENSE_NULL_OVERLAY_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/overlay_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {

// Overlay which introduces the layer of nullability but without changing the
// "spacing" of the underlying storage i.e. this overlay simply "masks" out
// rows in the underlying storage with nulls.
class DenseNullOverlay final : public OverlayLayer {
 public:
  explicit DenseNullOverlay(const BitVector* non_null);
  ~DenseNullOverlay() override;

  void Flatten(uint32_t* start, const uint32_t* end, uint32_t stride) override;

  std::unique_ptr<DataLayerChain> MakeChain(
      std::unique_ptr<DataLayerChain>,
      ChainCreationArgs = ChainCreationArgs());

 private:
  class ChainImpl : public DataLayerChain {
   public:
    ChainImpl(std::unique_ptr<DataLayerChain> inner, const BitVector* non_null);

    SingleSearchResult SingleSearch(FilterOp,
                                    SqlValue,
                                    uint32_t) const override;

    SearchValidationResult ValidateSearchConstraints(FilterOp,
                                                     SqlValue) const override;

    RangeOrBitVector SearchValidated(FilterOp, SqlValue, Range) const override;

    void IndexSearchValidated(FilterOp, SqlValue, Indices&) const override;

    void StableSort(Token* start, Token* end, SortDirection) const override;

    void Distinct(Indices&) const override;

    std::optional<Token> MaxElement(Indices&) const override;

    std::optional<Token> MinElement(Indices&) const override;

    SqlValue Get_AvoidUsingBecauseSlow(uint32_t index) const override;

    uint32_t size() const override { return non_null_->size(); }

    std::string DebugString() const override { return "DenseNullOverlay"; }

   private:
    std::unique_ptr<DataLayerChain> inner_;
    const BitVector* non_null_ = nullptr;
  };

  const BitVector* non_null_ = nullptr;
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_DENSE_NULL_OVERLAY_H_
