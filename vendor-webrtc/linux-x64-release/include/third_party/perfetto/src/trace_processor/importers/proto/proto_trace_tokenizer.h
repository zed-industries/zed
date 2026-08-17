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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_TOKENIZER_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <utility>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/protozero/field.h"
#include "perfetto/protozero/proto_utils.h"
#include "perfetto/public/compiler.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"
#include "src/trace_processor/util/gzip_utils.h"

#include "protos/perfetto/trace/trace.pbzero.h"
#include "src/trace_processor/util/status_macros.h"
#include "src/trace_processor/util/trace_blob_view_reader.h"

namespace perfetto::trace_processor {

// Reads a protobuf trace in chunks and extracts boundaries of trace packets
// with their timestamps.
class ProtoTraceTokenizer {
 public:
  ProtoTraceTokenizer();

  template <typename Callback = base::Status(TraceBlobView)>
  base::Status Tokenize(TraceBlobView tbv, Callback callback) {
    reader_.PushBack(std::move(tbv));

    for (;;) {
      size_t start_offset = reader_.start_offset();
      size_t avail = reader_.avail();

      // The header must be at least 2 bytes (1 byte for tag, 1 byte for
      // size/varint) and can be at most 20 bytes (10 bytes for tag + 10 bytes
      // for size/varint).
      const size_t kMinHeaderBytes = 2;
      const size_t kMaxHeaderBytes = 20;
      std::optional<TraceBlobView> header = reader_.SliceOff(
          start_offset,
          std::min(std::max(avail, kMinHeaderBytes), kMaxHeaderBytes));

      // This means that kMinHeaderBytes was not available. Just wait for the
      // next round.
      if (PERFETTO_UNLIKELY(!header)) {
        return base::OkStatus();
      }

      uint64_t tag;
      const uint8_t* tag_start = header->data();
      const uint8_t* tag_end = protozero::proto_utils::ParseVarInt(
          tag_start, header->data() + header->size(), &tag);

      if (PERFETTO_UNLIKELY(tag_end == tag_start)) {
        return header->size() < kMaxHeaderBytes
                   ? base::OkStatus()
                   : base::ErrStatus("Failed to parse tag");
      }

      if (PERFETTO_UNLIKELY(tag != kTracePacketTag)) {
        // Other field. Skip.
        auto field_type = static_cast<uint8_t>(tag & 0b111);
        switch (field_type) {
          case static_cast<uint8_t>(
              protozero::proto_utils::ProtoWireType::kVarInt): {
            uint64_t varint;
            const uint8_t* varint_start = tag_end;
            const uint8_t* varint_end = protozero::proto_utils::ParseVarInt(
                tag_end, header->data() + header->size(), &varint);
            if (PERFETTO_UNLIKELY(varint_end == varint_start)) {
              return header->size() < kMaxHeaderBytes
                         ? base::OkStatus()
                         : base::ErrStatus("Failed to skip varint");
            }
            PERFETTO_CHECK(reader_.PopFrontBytes(
                static_cast<size_t>(varint_end - tag_start)));
            continue;
          }
          case static_cast<uint8_t>(
              protozero::proto_utils::ProtoWireType::kLengthDelimited): {
            uint64_t varint;
            const uint8_t* varint_start = tag_end;
            const uint8_t* varint_end = protozero::proto_utils::ParseVarInt(
                tag_end, header->data() + header->size(), &varint);
            if (PERFETTO_UNLIKELY(varint_end == varint_start)) {
              return header->size() < kMaxHeaderBytes
                         ? base::OkStatus()
                         : base::ErrStatus("Failed to skip delimited");
            }

            size_t size_incl_header =
                static_cast<size_t>(varint_end - tag_start) + varint;
            if (size_incl_header > avail) {
              return base::OkStatus();
            }
            PERFETTO_CHECK(reader_.PopFrontBytes(size_incl_header));
            continue;
          }
          case static_cast<uint8_t>(
              protozero::proto_utils::ProtoWireType::kFixed64): {
            size_t size_incl_header =
                static_cast<size_t>(tag_end - tag_start) + 8;
            if (size_incl_header > avail) {
              return base::OkStatus();
            }
            PERFETTO_CHECK(reader_.PopFrontBytes(size_incl_header));
            continue;
          }
          case static_cast<uint8_t>(
              protozero::proto_utils::ProtoWireType::kFixed32): {
            size_t size_incl_header =
                static_cast<size_t>(tag_end - tag_start) + 4;
            if (size_incl_header > avail) {
              return base::OkStatus();
            }
            PERFETTO_CHECK(reader_.PopFrontBytes(size_incl_header));
            continue;
          }
          default:
            return base::ErrStatus("Unknown field type");
        }
      }

      uint64_t field_size;
      const uint8_t* size_start = tag_end;
      const uint8_t* size_end = protozero::proto_utils::ParseVarInt(
          size_start, header->data() + header->size(), &field_size);

      // If we had less than the maximum number of header bytes, it's possible
      // that we just need more to actually parse. Otherwise, this is an error.
      if (PERFETTO_UNLIKELY(size_start == size_end)) {
        return header->size() < kMaxHeaderBytes
                   ? base::OkStatus()
                   : base::ErrStatus("Failed to parse TracePacket size");
      }

      // Empty packets can legitimately happen if the producer ends up emitting
      // no data: just ignore them.
      auto hdr_size = static_cast<size_t>(size_end - header->data());
      if (PERFETTO_UNLIKELY(field_size == 0)) {
        PERFETTO_CHECK(reader_.PopFrontBytes(hdr_size));
        continue;
      }

      // If there's not enough bytes in the reader, then we cannot do anymore.
      size_t size_incl_header = hdr_size + field_size;
      if (size_incl_header > avail) {
        return base::OkStatus();
      }

      auto packet = reader_.SliceOff(start_offset + hdr_size, field_size);
      PERFETTO_CHECK(packet);
      PERFETTO_CHECK(reader_.PopFrontBytes(hdr_size + field_size));
      protos::pbzero::TracePacket::Decoder decoder(packet->data(),
                                                   packet->length());
      if (!decoder.has_compressed_packets()) {
        RETURN_IF_ERROR(callback(std::move(*packet)));
        continue;
      }

      if (!util::IsGzipSupported()) {
        return base::ErrStatus(
            "Cannot decode compressed packets. Zlib not enabled");
      }

      protozero::ConstBytes field = decoder.compressed_packets();
      TraceBlobView compressed_packets = packet->slice(field.data, field.size);
      TraceBlobView packets;
      RETURN_IF_ERROR(Decompress(std::move(compressed_packets), &packets));

      const uint8_t* start = packets.data();
      const uint8_t* end = packets.data() + packets.length();
      const uint8_t* ptr = start;
      while ((end - ptr) > 2) {
        const uint8_t* packet_outer = ptr;
        if (PERFETTO_UNLIKELY(*ptr != kTracePacketTag)) {
          return base::ErrStatus("Expected TracePacket tag");
        }
        uint64_t packet_size = 0;
        ptr = protozero::proto_utils::ParseVarInt(++ptr, end, &packet_size);
        const uint8_t* packet_start = ptr;
        ptr += packet_size;
        if (PERFETTO_UNLIKELY((ptr - packet_outer) < 2 || ptr > end)) {
          return base::ErrStatus("Invalid packet size");
        }
        TraceBlobView sliced =
            packets.slice(packet_start, static_cast<size_t>(packet_size));
        RETURN_IF_ERROR(callback(std::move(sliced)));
      }
    }
  }

 private:
  static constexpr uint8_t kTracePacketTag =
      protozero::proto_utils::MakeTagLengthDelimited(
          protos::pbzero::Trace::kPacketFieldNumber);

  base::Status Decompress(TraceBlobView input, TraceBlobView* output);

  // Used to glue together trace packets that span across two (or more)
  // Parse() boundaries.
  util::TraceBlobViewReader reader_;

  // Allows support for compressed trace packets.
  util::GzipDecompressor decompressor_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_TOKENIZER_H_
