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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/etm/opencsd.h"
#include "src/trace_processor/importers/perf/aux_data_tokenizer.h"
#include "src/trace_processor/importers/perf/perf_data_tokenizer.h"
#include "src/trace_processor/tables/etm_tables_py.h"

namespace perfetto::trace_processor {
class TraceProcessorContext;
namespace etm {
class FrameDecoder;
class EtmV4Stream : public perf_importer::AuxDataStream, public ITrcDataIn {
 public:
  EtmV4Stream(TraceProcessorContext* context,
              FrameDecoder* frame_decoder,
              tables::EtmV4ConfigurationTable::Id config_id);

  ~EtmV4Stream() override;

  ocsd_datapath_resp_t TraceDataIn(const ocsd_datapath_op_t op,
                                   const ocsd_trc_index_t index,
                                   const uint32_t dataBlockSize,
                                   const uint8_t* pDataBlock,
                                   uint32_t* numBytesProcessed) override;

  base::Status Parse(perf_importer::AuxRecord aux, TraceBlobView data) override;

  void OnDataLoss(uint64_t) override;
  base::Status NotifyEndOfStream() override;

  base::Status OnItraceStartRecord(
      perf_importer::ItraceStartRecord start) override;

 private:
  struct SessionState {
    explicit SessionState(tables::EtmV4SessionTable::Id in_session_id)
        : session_id(in_session_id) {}
    tables::EtmV4SessionTable::Id session_id;
    std::vector<TraceBlobView> traces_;
  };

  base::Status ParseFramedData(uint64_t offset, TraceBlobView data);

  void StartChunkedTrace();
  void WriteChunkedTrace(const uint8_t* src, uint32_t size);
  void EndChunkedTrace();

  void StartSession(std::optional<int64_t> start_ts);
  void AddTrace(TraceBlobView trace);
  void EndSession();

  TraceProcessorContext* const context_;
  FrameDecoder* const frame_decoder_;
  tables::EtmV4ConfigurationTable::Id config_id_;

  bool stream_active_ = true;
  ocsd_trc_index_t index_ = 0;
  std::optional<SessionState> session_;

  // For framed ETM data we get data in 16B or less chunks. This buffer is used
  // to create a contiguous memory buffer out of that.
  //
  // TODO(carlscab): This could probably be made more efficient but keep in mind
  // in the case of framed data we might get "spurious" starts, that is a start
  // followed by no data before the end. Framed ETM data usually only contains
  // data for one stream that means all the other streams will get such spurious
  // starts. So we delay the creation of the builder to seeing actual data
  std::vector<uint8_t> buffer_;
};

}  // namespace etm
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_H_
