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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FRAME_DECODER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FRAME_DECODER_H_

#include <cstdint>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/importers/etm/error_logger.h"
#include "src/trace_processor/importers/etm/opencsd.h"

namespace perfetto::trace_processor::etm {

class FrameDecoder {
 public:
  base::Status Init();

  base::StatusOr<bool> TraceDataIn(const ocsd_datapath_op_t op,
                                   const ocsd_trc_index_t index,
                                   const uint32_t data_block_size,
                                   const uint8_t* data_block,
                                   uint32_t* num_bytes_processed);

  base::Status Attach(uint8_t cs_trace_id, ITrcDataIn* data_in);

 private:
  ocsd_demux_stats_t demux_stats_;
  ErrorLogger error_logger_;
  TraceFormatterFrameDecoder frame_decoder_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FRAME_DECODER_H_
