//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_PARSER_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_PARSER_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/random/bit_gen_ref.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/ext/transport/chttp2/transport/hpack_parse_result.h"
#include "src/core/ext/transport/chttp2/transport/hpack_parser_table.h"
#include "src/core/ext/transport/chttp2/transport/legacy_frame.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_refcount.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/random_early_detection.h"

// IWYU pragma: no_include <type_traits>

namespace grpc_core {

// Top level interface for parsing a sequence of header, continuation frames.
class HPackParser {
 public:
  // What kind of stream boundary is provided by this frame?
  enum class Boundary : uint8_t {
    // More continuations are expected
    None,
    // This marks the end of headers, so data frames should follow
    EndOfHeaders,
    // This marks the end of headers *and* the end of the stream
    EndOfStream
  };
  // What kind of priority is represented in the next frame
  enum class Priority : uint8_t {
    // No priority field
    None,
    // Yes there's a priority field
    Included
  };
  // Details about a frame we only need to know for logging
  struct LogInfo {
    // The stream ID
    uint32_t stream_id;
    // Headers or trailers?
    enum Type : uint8_t {
      kHeaders,
      kTrailers,
      kDontKnow,
    };
    Type type;
    // Client or server?
    bool is_client;
  };

  HPackParser();
  ~HPackParser();

  // Non-copyable
  HPackParser(const HPackParser&) = delete;
  HPackParser& operator=(const HPackParser&) = delete;
  HPackParser(HPackParser&&) = default;
  HPackParser& operator=(HPackParser&&) = default;

  // Begin parsing a new frame
  // Sink receives each parsed header,
  void BeginFrame(grpc_metadata_batch* metadata_buffer,
                  uint32_t metadata_size_soft_limit,
                  uint32_t metadata_size_hard_limit, Boundary boundary,
                  Priority priority, LogInfo log_info);
  // Start throwing away any received headers after parsing them.
  void StopBufferingFrame() { metadata_buffer_ = nullptr; }
  // Parse one slice worth of data
  grpc_error_handle Parse(const grpc_slice& slice, bool is_last,
                          absl::BitGenRef bitsrc,
                          CallTracerAnnotationInterface* call_tracer);
  // Reset state ready for the next BeginFrame
  void FinishFrame();

  // Retrieve the associated hpack table (for tests, debugging)
  HPackTable* hpack_table() { return &state_.hpack_table; }
  // Is the current frame a boundary of some sort
  bool is_boundary() const { return boundary_ != Boundary::None; }
  // Is the current frame the end of a stream
  bool is_eof() const { return boundary_ == Boundary::EndOfStream; }

  // How many bytes are buffered (for tests to assert on)
  size_t buffered_bytes() const { return unparsed_bytes_.size(); }

 private:
  // Helper classes: see implementation
  class Parser;
  class Input;

  // Helper to parse a string and turn it into a slice with appropriate memory
  // management characteristics
  class String {
   public:
    // StringResult carries both a HpackParseStatus and the parsed string
    struct StringResult;

    String() : value_(absl::Span<const uint8_t>()) {}
    String(const String&) = delete;
    String& operator=(const String&) = delete;
    String(String&& other) noexcept : value_(std::move(other.value_)) {
      other.value_ = absl::Span<const uint8_t>();
    }
    String& operator=(String&& other) noexcept {
      value_ = std::move(other.value_);
      other.value_ = absl::Span<const uint8_t>();
      return *this;
    }

    // Take the value and leave this empty
    Slice Take();

    // Return a reference to the value as a string view
    absl::string_view string_view() const;

    // Parse a non-binary string
    static StringResult Parse(Input* input, bool is_huff, size_t length);

    // Parse a binary string
    static StringResult ParseBinary(Input* input, bool is_huff, size_t length);

   private:
    void AppendBytes(const uint8_t* data, size_t length);
    explicit String(std::vector<uint8_t> v) : value_(std::move(v)) {}
    explicit String(absl::Span<const uint8_t> v) : value_(v) {}
    String(grpc_slice_refcount* r, const uint8_t* begin, const uint8_t* end)
        : value_(Slice::FromRefcountAndBytes(r, begin, end)) {}

    // Parse some huffman encoded bytes, using output(uint8_t b) to emit each
    // decoded byte.
    template <typename Out>
    static HpackParseStatus ParseHuff(Input* input, uint32_t length,
                                      Out output);

    // Parse some uncompressed string bytes.
    static StringResult ParseUncompressed(Input* input, uint32_t length,
                                          uint32_t wire_size);

    // Turn base64 encoded bytes into not base64 encoded bytes.
    static StringResult Unbase64(String s);

    // Main loop for Unbase64
    static std::optional<std::vector<uint8_t>> Unbase64Loop(const uint8_t* cur,
                                                            const uint8_t* end);

    std::variant<Slice, absl::Span<const uint8_t>, std::vector<uint8_t>> value_;
  };

  // Prefix for a string
  struct StringPrefix {
    // Number of bytes in input for string
    uint32_t length;
    // Is it huffman compressed
    bool huff;

    std::string ToString() const {
      return absl::StrCat(length, " bytes ",
                          huff ? "huffman compressed" : "uncompressed");
    }
  };

  // Current parse state
  // ┌───┐
  // │Top│
  // └┬─┬┘
  //  │┌▽────────────────┐
  //  ││ParsingKeyLength │
  //  │└┬───────────────┬┘
  //  │┌▽─────────────┐┌▽──────────────┐
  //  ││ParsingKeyBody││SkippingKeyBody│
  //  │└┬─────────────┘└───┬───────────┘
  // ┌▽─▽────────────────┐┌▽──────────────────┐
  // │ParsingValueLength ││SkippingValueLength│
  // └┬─────────────────┬┘└┬──────────────────┘
  // ┌▽───────────────┐┌▽──▽─────────────┐
  // │ParsingValueBody││SkippingValueBody│
  // └────────────────┘└─────────────────┘
  enum class ParseState : uint8_t {
    // Start of one opcode
    kTop,
    // Parsing a literal keys length
    kParsingKeyLength,
    // Parsing a literal key
    kParsingKeyBody,
    // Skipping a literal key
    kSkippingKeyBody,
    // Parsing a literal value length
    kParsingValueLength,
    // Parsing a literal value
    kParsingValueBody,
    // Reading a literal value length (so we can skip it)
    kSkippingValueLength,
    // Skipping a literal value
    kSkippingValueBody,
  };

  // Shared state for Parser instances between slices.
  struct InterSliceState {
    HPackTable hpack_table;
    // Error so far for this frame (set by class Input)
    HpackParseResult frame_error;
    // Error so far for this field (set by class Input)
    HpackParseResult field_error;
    // Length of frame so far.
    uint32_t frame_length = 0;
    // Length of the string being parsed
    uint32_t string_length;
    // RED for overly large metadata sets
    RandomEarlyDetection metadata_early_detection;
    // Should the current header be added to the hpack table?
    bool add_to_table;
    // Is the string being parsed huffman compressed?
    bool is_string_huff_compressed;
    // Is the value being parsed binary?
    bool is_binary_header;
    // How many more dynamic table updates are allowed
    uint8_t dynamic_table_updates_allowed;
    // Current parse state
    ParseState parse_state = ParseState::kTop;
    std::variant<const HPackTable::Memento*, Slice> key;
  };

  grpc_error_handle ParseInput(Input input, bool is_last,
                               CallTracerAnnotationInterface* call_tracer);
  void ParseInputInner(Input* input);
  GPR_ATTRIBUTE_NOINLINE
  void HandleMetadataSoftSizeLimitExceeded(Input* input);

  // Target metadata buffer
  grpc_metadata_batch* metadata_buffer_ = nullptr;

  // Bytes that could not be parsed last parsing round
  std::vector<uint8_t> unparsed_bytes_;
  // How many bytes would be needed before progress could be made?
  size_t min_progress_size_ = 0;
  // Buffer kind of boundary
  // TODO(ctiller): see if we can move this argument to Parse, and avoid
  // buffering.
  Boundary boundary_;
  // Buffer priority
  // TODO(ctiller): see if we can move this argument to Parse, and avoid
  // buffering.
  Priority priority_;
  // Information for logging
  LogInfo log_info_;
  InterSliceState state_;
};

}  // namespace grpc_core

// wraps grpc_chttp2_hpack_parser_parse to provide a frame level parser for
// the transport
grpc_error_handle grpc_chttp2_header_parser_parse(void* hpack_parser,
                                                  grpc_chttp2_transport* t,
                                                  grpc_chttp2_stream* s,
                                                  const grpc_slice& slice,
                                                  int is_last);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_PARSER_H
