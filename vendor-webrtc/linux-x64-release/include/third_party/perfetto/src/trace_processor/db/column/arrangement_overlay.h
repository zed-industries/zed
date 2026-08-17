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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_ARRANGEMENT_OVERLAY_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_ARRANGEMENT_OVERLAY_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/overlay_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {

// Storage responsible for rearranging the elements of another Storage. It deals
// with duplicates, permutations and selection; for selection only, it's more
// efficient to use `SelectorOverlay`.
class ArrangementOverlay final : public OverlayLayer {
 public:
  ArrangementOverlay(const std::vector<uint32_t>* arrangement,
                     DataLayerChain::Indices::State arrangement_state);
  ~ArrangementOverlay() override;

  void Flatten(uint32_t* start, const uint32_t* end, uint32_t stride) override;

  std::unique_ptr<DataLayerChain> MakeChain(
      std::unique_ptr<DataLayerChain>,
      ChainCreationArgs = ChainCreationArgs());

 private:
  class ChainImpl : public DataLayerChain {
   public:
    ChainImpl(std::unique_ptr<DataLayerChain> inner,
              const std::vector<uint32_t>* arrangement,
              Indices::State arrangement_state,
              bool does_arrangement_order_storage);

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

    uint32_t size() const override {
      return static_cast<uint32_t>(arrangement_->size());
    }

    std::string DebugString() const override { return "ArrangementOverlay"; }

   private:
    std::unique_ptr<DataLayerChain> inner_;
    const std::vector<uint32_t>* arrangement_;
    const Indices::State arrangement_state_;
    const bool does_arrangement_order_storage_;
  };

  std::unique_ptr<DataLayerChain> inner_;
  const std::vector<uint32_t>* arrangement_;
  const DataLayerChain::Indices::State arrangement_state_;
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_ARRANGEMENT_OVERLAY_H_
