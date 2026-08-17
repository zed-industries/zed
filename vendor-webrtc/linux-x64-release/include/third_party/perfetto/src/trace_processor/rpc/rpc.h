/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_RPC_RPC_H_
#define SRC_TRACE_PROCESSOR_RPC_RPC_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/protozero/proto_ring_buffer.h"
#include "perfetto/protozero/field.h"
#include "perfetto/trace_processor/basic_types.h"

namespace perfetto {

namespace protos::pbzero {
class ComputeMetricResult;
class DisableAndReadMetatraceResult;
}  // namespace protos::pbzero

namespace trace_processor {

class Iterator;
class TraceProcessor;

// This class handles the binary {,un}marshalling for the Trace Processor RPC
// API (see protos/perfetto/trace_processor/trace_processor.proto).
// This is to deal with cases where the client of the trace processor is not
// some in-process C++ code but a remote process:
// There are two use cases of this:
//   1. The JS<>WASM interop for the web-based UI.
//   2. The HTTP RPC mode of trace_processor_shell that allows the UI to talk
//      to a native trace processor instead of the bundled WASM one.
// This class has (a subset of) the same methods of the public TraceProcessor
// interface, but the methods just take and return proto-encoded binary buffers.
// This class does NOT define how the transport works (e.g. HTTP vs WASM interop
// calls), it just deals with {,un}marshalling.
// This class internally creates and owns a TraceProcessor instance, which
// lifetime is tied to the lifetime of the Rpc instance.
class Rpc {
 public:
  // The unique_ptr argument is optional. If non-null it will adopt the passed
  // instance and allow to directly query that. If null, a new instanace will be
  // created internally by calling Parse().
  explicit Rpc(std::unique_ptr<TraceProcessor>);
  Rpc();
  ~Rpc();

  // 1. TraceProcessor byte-pipe RPC interface.
  // This is a bidirectional channel with a remote TraceProcessor instance. All
  // it needs is a byte-oriented pipe (e.g., a TCP socket, a pipe(2) between two
  // processes or a postmessage channel in the JS+Wasm case). The messages
  // exchanged on these pipes are TraceProcessorRpc protos (defined in
  // trace_processor.proto). This has been introduced in Perfetto v15.

  // Pushes data received by the RPC channel into the parser. Inbound messages
  // are tokenized and turned into TraceProcessor method invocations. |data|
  // does not need to be a whole TraceProcessorRpc message. It can be a portion
  // of it or a union of >1 messages.
  // Responses are sent throught the RpcResponseFunction (below).
  void OnRpcRequest(const void* data, size_t len);

  // The size argument is a uint32_t and not size_t to avoid ABI mismatches
  // with Wasm, where size_t = uint32_t.
  // (nullptr, 0) has the semantic of "close the channel" and is issued when an
  // unrecoverable wire-protocol framing error is detected.
  using RpcResponseFunction =
      std::function<void(const void* /*data*/, uint32_t /*len*/)>;
  void SetRpcResponseFunction(RpcResponseFunction f) {
    rpc_response_fn_ = std::move(f);
  }

  // 2. TraceProcessor legacy RPC endpoints.
  // The methods below are exposed for the old RPC interfaces, where each RPC
  // implementation deals with the method demuxing: (i) wasm_bridge.cc has one
  // exported C function per method (going away soon); (ii) httpd.cc has one
  // REST endpoint per method. Over time this turned out to have too much
  // duplicated boilerplate and we moved to the byte-pipe model above.
  // We still keep these endpoints around, because httpd.cc still  exposes the
  // individual REST endpoints to legacy clients (TP's Python API). The
  // mainteinance cost of those is very low. Both the new byte-pipe and the
  // old endpoints run exactly the same code. The {de,}serialization format is
  // the same, the only difference is only who does the method demuxing.
  // The methods of this class are mirrors (modulo {un,}marshalling of args) of
  // the corresponding names in trace_processor.h . See that header for docs.

  base::Status Parse(const uint8_t*, size_t);
  base::Status NotifyEndOfFile();
  std::string GetCurrentTraceName();
  std::vector<uint8_t> ComputeMetric(const uint8_t*, size_t);
  void EnableMetatrace(const uint8_t*, size_t);
  std::vector<uint8_t> DisableAndReadMetatrace();
  std::vector<uint8_t> GetStatus();

  // Creates a new RPC session by deleting all tables and views that have been
  // created (by the UI or user) after the trace was loaded; built-in
  // tables/view created by the ingestion process are preserved.
  void RestoreInitialTables();

  // Runs a query and returns results in batch. Each batch is a proto-encoded
  // TraceProcessor.QueryResult message and contains a variable number of rows.
  // The callbacks are called inline, so the whole callstack looks as follows:
  // Query(..., callback)
  //   callback(..., has_more=true)
  //   ...
  //   callback(..., has_more=false)
  //   (Query() returns at this point).
  using QueryResultBatchCallback = std::function<
      void(const uint8_t* /*buf*/, size_t /*len*/, bool /*has_more*/)>;
  void Query(const uint8_t*, size_t, const QueryResultBatchCallback&);

 private:
  void ParseRpcRequest(const uint8_t*, size_t);
  void ResetTraceProcessor(const uint8_t*, size_t);
  base::Status RegisterSqlPackage(protozero::ConstBytes);
  void ResetTraceProcessorInternal(const Config&);
  void MaybePrintProgress();
  Iterator QueryInternal(const uint8_t*, size_t);
  void ComputeMetricInternal(const uint8_t*,
                             size_t,
                             protos::pbzero::ComputeMetricResult*);
  void DisableAndReadMetatraceInternal(
      protos::pbzero::DisableAndReadMetatraceResult*);

  Config trace_processor_config_;
  std::unique_ptr<TraceProcessor> trace_processor_;
  RpcResponseFunction rpc_response_fn_;
  protozero::ProtoRingBuffer rxbuf_;
  int64_t tx_seq_id_ = 0;
  int64_t rx_seq_id_ = 0;
  bool eof_ = false;
  int64_t t_parse_started_ = 0;
  size_t bytes_last_progress_ = 0;
  size_t bytes_parsed_ = 0;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_RPC_RPC_H_
