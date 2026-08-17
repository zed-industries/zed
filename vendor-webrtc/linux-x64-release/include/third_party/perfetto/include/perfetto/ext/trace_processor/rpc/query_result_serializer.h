/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_RPC_QUERY_RESULT_SERIALIZER_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_RPC_QUERY_RESULT_SERIALIZER_H_

#include <cstdint>
#include <memory>
#include <vector>

#include <limits.h>
#include <stddef.h>
#include <stdint.h>

namespace perfetto {

namespace protos::pbzero {
class QueryResult;
}  // namespace protos::pbzero

namespace trace_processor {

class Iterator;
class IteratorImpl;

// This class serializes a TraceProcessor query result (i.e. an Iterator)
// into batches of QueryResult (trace_processor.proto). This class
// returns results in batches, allowing to deal with O(M) results without
// full memory buffering. It works as follows:
// - The iterator is passed in the constructor.
// - The client is expected to call Serialize(out_buf) until EOF is reached.
// - For each Serialize() call, this class will serialize a batch of cells,
//   stopping when either when a number of cells (|cells_per_batch_|) is reached
//   or when the batch size exceeds (batch_split_threshold_).
//   A batch is guaranteed to contain a number of cells that is an integer
//   multiple of the column count (i.e. a batch is not truncated in the middle
//   of a row).
// The intended use case is streaaming these batches onto through a
// chunked-encoded HTTP response, or through a repetition of Wasm calls.
class QueryResultSerializer {
 public:
  static constexpr uint32_t kDefaultBatchSplitThreshold = 128 * 1024;
  explicit QueryResultSerializer(Iterator);
  ~QueryResultSerializer();

  // No copy or move.
  QueryResultSerializer(const QueryResultSerializer&) = delete;
  QueryResultSerializer& operator=(const QueryResultSerializer&) = delete;

  // Appends the data to the passed protozero message. It returns true if more
  // chunks are available (i.e. it returns NOT(|eof_reached_||)). The caller is
  // supposed to keep calling this function until it returns false.
  bool Serialize(protos::pbzero::QueryResult*);

  // Like the above but stitches everything together in a vector. Incurs in
  // extra copies.
  bool Serialize(std::vector<uint8_t>*);

  void set_batch_size_for_testing(uint32_t cells_per_batch, uint32_t thres) {
    cells_per_batch_ = cells_per_batch;
    batch_split_threshold_ = thres;
  }

 private:
  void SerializeMetadata(protos::pbzero::QueryResult*);
  void SerializeBatch(protos::pbzero::QueryResult*);
  void MaybeSerializeError(protos::pbzero::QueryResult*);

  std::unique_ptr<IteratorImpl> iter_;
  const uint32_t num_cols_;
  bool did_write_metadata_ = false;
  bool eof_reached_ = false;
  uint32_t col_ = UINT32_MAX;

  // These params specify the thresholds for splitting the results in batches,
  // in terms of: (1) max cells (row x cols); (2) serialized batch size in
  // bytes, whichever is reached first. Note also that the byte limit is not
  // 100% accurate and can occasionally yield to batches slighly larger than
  // the limit (it splits on the next row *after* the limit is hit).
  // Overridable for testing only.
  uint32_t cells_per_batch_ = 50000;
  uint32_t batch_split_threshold_ = kDefaultBatchSplitThreshold;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_RPC_QUERY_RESULT_SERIALIZER_H_
