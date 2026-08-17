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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_DECODER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_DECODER_H_

#include <stdint.h>
#include <cstdint>
#include <memory>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/importers/etm/error_logger.h"
#include "src/trace_processor/importers/etm/opencsd.h"

namespace perfetto::trace_processor::etm {

class MappingVersion;
class TargetMemoryReader;

// Wrapper around the open_csd packet processor. This class will take ETM traces
// as input and output a stream of ETM elements.
class EtmV4Decoder : private ITrcGenElemIn {
 public:
  class Delegate {
   public:
    virtual ~Delegate();
    virtual ocsd_datapath_resp_t TraceElemIn(const ocsd_trc_index_t index_sop,
                                             const uint8_t trc_chan_id,
                                             const OcsdTraceElement& elem,
                                             const MappingVersion* mapping) = 0;
  };

  static base::StatusOr<std::unique_ptr<EtmV4Decoder>> Create(
      Delegate* delegate,
      TargetMemoryReader* reader,
      const EtmV4Config& config);

  base::StatusOr<bool> Reset(ocsd_trc_index_t index);
  base::StatusOr<bool> Data(const ocsd_trc_index_t,
                            const size_t size,
                            const uint8_t* data,
                            uint32_t* num_bytes_processed);
  base::StatusOr<bool> Flush(ocsd_trc_index_t index);
  base::StatusOr<bool> Eot(ocsd_trc_index_t index);

 private:
  EtmV4Decoder(Delegate* delegate, TargetMemoryReader* reader);
  base::Status Init(const EtmV4Config& config);
  ocsd_datapath_resp_t TraceElemIn(const ocsd_trc_index_t,
                                   const uint8_t,
                                   const OcsdTraceElement& elem) override;

  Delegate* const delegate_;
  TargetMemoryReader* memory_reader_;

  ErrorLogger error_logger_;
  TrcIDecode instruction_decoder_;
  TrcPktDecodeEtmV4I packet_decoder_;
  TrcPktProcEtmV4I packet_processor_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_DECODER_H_
