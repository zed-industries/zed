/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELEMENT_CURSOR_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELEMENT_CURSOR_H_

#include <cstdint>
#include <limits>
#include <memory>
#include <optional>

#include "perfetto/base/status.h"
#include "src/trace_processor/importers/etm/etm_v4_decoder.h"
#include "src/trace_processor/importers/etm/opencsd.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/etm_tables_py.h"

namespace perfetto::trace_processor::etm {

class MappingVersion;
class TargetMemory;
class TargetMemoryReader;
struct InstructionRangeSqlValue;

class ElementTypeMask {
 public:
  static constexpr bool IsCompatibleValue(ocsd_gen_trc_elem_t type) {
#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wtautological-constant-out-of-range-compare"
#endif
    return type < 64;
#if defined(__clang__)
#pragma clang diagnostic pop
#endif
  }
  ElementTypeMask() : mask_(0) {}

  ElementTypeMask& operator&=(const ElementTypeMask& o) {
    mask_ &= o.mask_;
    return *this;
  }

  void clear() { mask_ = 0; }
  void set_all() { mask_ = std::numeric_limits<uint64_t>::max(); }

  bool empty() const { return mask_ == 0; }

  bool matches(ocsd_gen_trc_elem_t type) const {
    return mask_ & (1ull << type);
  }

  void set_bit(ocsd_gen_trc_elem_t type) { mask_ &= (1ull << type); }

 private:
  friend class ElementCursor;
  uint64_t mask_;
};

// Helper class to feed data to an `EtmV4Decoder` that offers a Sqlite friendly
// API.
// Given a trace this class will allow you to iterate the ETM elements contained
// in it. It also give you the ability to filter out some elements.
class ElementCursor : public EtmV4Decoder::Delegate {
 public:
  explicit ElementCursor(TraceStorage* storage);
  ~ElementCursor() override;

  base::Status Filter(std::optional<tables::EtmV4TraceTable::Id> trace_id,
                      ElementTypeMask type_mask);

  base::Status Next();
  bool Eof() { return !needs_flush_ && data_ == data_end_; }

  tables::EtmV4TraceTable::Id trace_id() const { return *trace_id_; }
  ocsd_trc_index_t index() const {
    return static_cast<uintptr_t>(data_ - data_start_);
  }
  uint32_t element_index() const { return element_index_; }

  const OcsdTraceElement& element() const { return *element_; }

  const TraceStorage* storage() const { return storage_; }

  const MappingVersion* mapping() const { return mapping_; }

  bool has_instruction_range() const {
    return element_->getType() == OCSD_GEN_TRC_ELEM_INSTR_RANGE;
  }

  std::unique_ptr<InstructionRangeSqlValue> GetInstructionRange() const;

 private:
  void SetAtEof();
  base::Status ResetDecoder(tables::EtmV4ConfigurationTable::Id config_id);
  ocsd_datapath_resp_t TraceElemIn(const ocsd_trc_index_t index_sop,
                                   const uint8_t trc_chan_id,
                                   const OcsdTraceElement& elem,
                                   const MappingVersion* mapping) override;

  TraceStorage* storage_;
  ElementTypeMask type_mask_;
  std::unique_ptr<TargetMemoryReader> reader_;
  std::unique_ptr<EtmV4Decoder> decoder_;
  // Configuration used to create the above decoder
  std::optional<tables::EtmV4ConfigurationTable::Id> config_id_;
  std::optional<tables::EtmV4TraceTable::Id> trace_id_;

  const uint8_t* data_start_;
  const uint8_t* data_;
  const uint8_t* data_end_;
  bool needs_flush_ = false;
  uint32_t element_index_;
  const OcsdTraceElement* element_;
  const MappingVersion* mapping_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELEMENT_CURSOR_H_
