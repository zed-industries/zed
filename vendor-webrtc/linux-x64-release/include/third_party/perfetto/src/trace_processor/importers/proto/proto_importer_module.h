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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_IMPORTER_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_IMPORTER_MODULE_H_

#include <cstdint>
#include <optional>
#include <string>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"

namespace perfetto {

namespace protos::pbzero {
class TraceConfig_Decoder;
class TracePacket_Decoder;
}  // namespace protos::pbzero

namespace trace_processor {

class TraceBlobView;
class TraceProcessorContext;

// This file contains a base class for ProtoTraceReader/Parser modules.
// A module implements support for a subset of features of the TracePacket
// proto format.
// To add and integrate a new module:
// (1) Add MyModule as a subclass of ProtoImporterModule,
//     overriding the TokenizePacket(), ParsePacket() and/or ParseTraceConfig()
//     methods.
// (2) In the constructor call the RegisterForField method for every field
//     that the module knows how to handle.
// (3) Create a module instance and add it to TraceProcessorContext's |modules|
//     vector in either default_modules.cc or additional_modules.cc.
// See GraphicsEventModule for an example.

class ModuleResult {
 public:
  // Allow auto conversion from base::Status to Handled / Error result.
  ModuleResult(const base::Status& status)
      : ignored_(false),
        error_(status.ok() ? std::nullopt
                           : std::make_optional(status.message())) {}

  // Constructs a result that indicates the module ignored the packet and is
  // deferring the handling of the packet to other modules.
  static ModuleResult Ignored() { return ModuleResult(true); }

  // Constructs a result that indicates the module handled the packet. Other
  // modules will not be notified about the packet.
  static ModuleResult Handled() { return ModuleResult(false); }

  // Constructs a result that indicates an error condition while handling the
  // packet. Other modules will not be notified about the packet.
  static ModuleResult Error(const std::string& message) {
    return ModuleResult(message);
  }

  bool ignored() const { return ignored_; }
  bool ok() const { return !error_.has_value(); }
  const std::string& message() const { return *error_; }

  base::Status ToStatus() const {
    PERFETTO_DCHECK(!ignored_);
    if (error_)
      return base::Status(*error_);
    return base::OkStatus();
  }

 private:
  explicit ModuleResult(bool ignored) : ignored_(ignored) {}
  explicit ModuleResult(const std::string& error)
      : ignored_(false), error_(error) {}

  bool ignored_;
  std::optional<std::string> error_;
};

// Base class for modules.
class ProtoImporterModule {
 public:
  ProtoImporterModule();

  virtual ~ProtoImporterModule();

  // Called by ProtoTraceReader during the tokenization stage, i.e. before
  // sorting. It's called for each TracePacket that contains fields for which
  // the module was registered. If this returns a result other than
  // ModuleResult::Ignored(), tokenization of the packet will be aborted after
  // the module.
  virtual ModuleResult TokenizePacket(
      const protos::pbzero::TracePacket_Decoder&,
      TraceBlobView* packet,
      int64_t packet_timestamp,
      RefPtr<PacketSequenceStateGeneration> sequence_state,
      uint32_t field_id);

  // Called by ProtoTraceReader during the tokenization stage i.e. before
  // sorting. Indicates that sequence with id |packet_sequence_id| has cleared
  // its incremental state. This should be used to clear any cached state the
  // tokenizer has built up while reading packets until this point for this
  // packet sequence.
  virtual void OnIncrementalStateCleared(uint32_t /* packet_sequence_id */) {}

  // Called by ProtoTraceReader during the tokenization stage i.e. before
  // sorting. Indicates that sequence with id |packet_sequence_id| has a packet
  // with first_packet_on_sequence = true. This implies that there was no data
  // loss, including ring buffer overwrites, on this sequence.
  virtual void OnFirstPacketOnSequence(uint32_t /* packet_sequence_id */) {}

  // ParsePacket functions are called by ProtoTraceParser after the sorting
  // stage for each non-ftrace TracePacket that contains fields for which the
  // module was registered.
  virtual void ParseTracePacketData(const protos::pbzero::TracePacket_Decoder&,
                                    int64_t ts,
                                    const TracePacketData&,
                                    uint32_t /*field_id*/);

  // Called by ProtoTraceParser for trace config packets after the sorting
  // stage, on all existing modules.
  virtual void ParseTraceConfig(const protos::pbzero::TraceConfig_Decoder&);

  virtual void NotifyEndOfFile() {}

 protected:
  void RegisterForField(uint32_t field_id, TraceProcessorContext*);
  // Primarily intended for special modules that need to get all TracePacket's,
  // for example for trace proto content analysis. Most modules need to register
  // for specific fields using the method above.
  void RegisterForAllFields(TraceProcessorContext*);
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_IMPORTER_MODULE_H_
