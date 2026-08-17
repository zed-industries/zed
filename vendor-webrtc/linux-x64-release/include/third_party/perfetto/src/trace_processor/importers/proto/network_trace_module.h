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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_NETWORK_TRACE_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_NETWORK_TRACE_MODULE_H_

#include <cstdint>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/protozero/scattered_heap_buffer.h"
#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

#include "protos/perfetto/trace/android/network_trace.pbzero.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace trace_processor {

class NetworkTraceModule : public ProtoImporterModule {
 public:
  explicit NetworkTraceModule(TraceProcessorContext* context);
  ~NetworkTraceModule() override = default;

  // Tokenize and de-intern NetworkPacketBundles so that bundles of multiple
  // packets are sorted appropriately. This splits bundles with per-packet
  // details (packet_timestamps and packet_lengths) into one NetworkTraceEvent
  // per packet. Bundles with aggregates (i.e. total_packets) are forwarded
  // after de-interning the packet context.
  ModuleResult TokenizePacket(
      const protos::pbzero::TracePacket::Decoder& decoder,
      TraceBlobView* packet,
      int64_t ts,
      RefPtr<PacketSequenceStateGeneration> state,
      uint32_t field_id) override;

  void ParseTracePacketData(const protos::pbzero::TracePacket::Decoder& decoder,
                            int64_t ts,
                            const TracePacketData&,
                            uint32_t field_id) override;

 private:
  void ParseGenericEvent(int64_t ts,
                         int64_t dur,
                         int64_t length,
                         int64_t count,
                         protos::pbzero::NetworkPacketEvent::Decoder& evt);

  void ParseNetworkPacketEvent(int64_t ts, protozero::ConstBytes blob);
  void ParseNetworkPacketBundle(int64_t ts, protozero::ConstBytes blob);

  // Helper to simplify pushing a TracePacket to the sorter. The caller fills in
  // the packet buffer and uses this to push for sorting and reset the buffer.
  void PushPacketBufferForSort(int64_t timestamp,
                               RefPtr<PacketSequenceStateGeneration> state);

  StringId GetIpProto(protos::pbzero::NetworkPacketEvent::Decoder& evt);

  TraceProcessorContext* context_;
  protozero::HeapBuffered<protos::pbzero::TracePacket> packet_buffer_;

  bool loaded_package_names_ = false;
  base::FlatHashMap<int64_t, StringId> package_names_;

  const StringId net_arg_length_;
  const StringId net_arg_ip_proto_;
  const StringId net_arg_tcp_flags_;
  const StringId net_arg_tag_;
  const StringId net_arg_uid_;
  const StringId net_arg_local_port_;
  const StringId net_arg_remote_port_;
  const StringId net_arg_icmp_type_;
  const StringId net_arg_icmp_code_;
  const StringId net_ipproto_tcp_;
  const StringId net_ipproto_udp_;
  const StringId net_ipproto_icmp_;
  const StringId net_ipproto_icmpv6_;
  const StringId packet_count_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_NETWORK_TRACE_MODULE_H_
