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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <cstdint>
#include <utility>
#include <vector>

#include "absl/log/log.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/call/metadata_compression_traits.h"
#include "src/core/ext/transport/chttp2/transport/hpack_constants.h"
#include "src/core/ext/transport/chttp2/transport/hpack_encoder_table.h"
#include "src/core/ext/transport/chttp2/transport/http2_ztrace_collector.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/timeout_encoding.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Forward decl for encoder
class HPackCompressor;

namespace hpack_encoder_detail {

class Encoder {
 public:
  Encoder(HPackCompressor* compressor, bool use_true_binary_metadata,
          SliceBuffer& output);

  void Encode(const Slice& key, const Slice& value);
  template <typename MetadataTrait>
  void Encode(MetadataTrait, const typename MetadataTrait::ValueType& value);

  void AdvertiseTableSizeChange();
  void EmitIndexed(uint32_t index);
  GRPC_MUST_USE_RESULT
  uint32_t EmitLitHdrWithNonBinaryStringKeyIncIdx(Slice key_slice,
                                                  Slice value_slice);
  GRPC_MUST_USE_RESULT
  uint32_t EmitLitHdrWithBinaryStringKeyIncIdx(Slice key_slice,
                                               Slice value_slice);
  void EmitLitHdrWithBinaryStringKeyNotIdx(Slice key_slice, Slice value_slice);
  void EmitLitHdrWithBinaryStringKeyNotIdx(uint32_t key_index,
                                           Slice value_slice);
  void EmitLitHdrWithNonBinaryStringKeyNotIdx(Slice key_slice,
                                              Slice value_slice);

  void EncodeAlwaysIndexed(uint32_t* index, absl::string_view key, Slice value,
                           size_t transport_length);
  void EncodeIndexedKeyWithBinaryValue(uint32_t* index, absl::string_view key,
                                       Slice value);

  void EncodeRepeatingSliceValue(const absl::string_view& key,
                                 const Slice& slice, uint32_t* index,
                                 size_t max_compression_size);

  void NoteEncodingError() { saw_encoding_errors_ = true; }
  bool saw_encoding_errors() const { return saw_encoding_errors_; }

  HPackEncoderTable& hpack_table();

 private:
  const bool use_true_binary_metadata_;
  bool saw_encoding_errors_ = false;
  HPackCompressor* const compressor_;
  SliceBuffer& output_;
};

// Compressor is partially specialized on CompressionTraits, but leaves
// MetadataTrait as variable.
// Via MetadataMap::StatefulCompressor it builds compression state for
// HPackCompressor.
// Each trait compressor gets to have some persistent state across the channel
// (declared as Compressor member variables).
// The compressors expose a single method:
// void EncodeWith(MetadataTrait, const MetadataTrait::ValueType, Encoder*);
// This method figures out how to encode the value, and then delegates to
// Encoder to perform the encoding.
template <typename MetadataTrait, typename CompressionTraits>
class Compressor;

// No compression encoder: just emit the key and value as literals.
template <typename MetadataTrait>
class Compressor<MetadataTrait, NoCompressionCompressor> {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    const Slice& slice = MetadataValueAsSlice<MetadataTrait>(value);
    if (absl::EndsWith(MetadataTrait::key(), "-bin")) {
      encoder->EmitLitHdrWithBinaryStringKeyNotIdx(
          Slice::FromStaticString(MetadataTrait::key()), slice.Ref());
    } else {
      encoder->EmitLitHdrWithNonBinaryStringKeyNotIdx(
          Slice::FromStaticString(MetadataTrait::key()), slice.Ref());
    }
  }
};

// Frequent key with no value compression encoder
template <typename MetadataTrait>
class Compressor<MetadataTrait, FrequentKeyWithNoValueCompressionCompressor> {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    const Slice& slice = MetadataValueAsSlice<MetadataTrait>(value);
    encoder->EncodeRepeatingSliceValue(MetadataTrait::key(), slice,
                                       &some_sent_value_,
                                       HPackEncoderTable::MaxEntrySize());
  }

 private:
  // Some previously sent value with this tag.
  uint32_t some_sent_value_ = 0;
};

// Helper to determine if two objects have the same identity.
// Equivalent here => equality, but equality does not imply equivalency.
// For example, two slices with the same contents are equal, but not
// equivalent.
// Used as a much faster check for equality than the full equality check,
// since many metadatum that are stable have the same root object in metadata
// maps.
template <typename T>
static bool IsEquivalent(T a, T b) {
  return a == b;
}

template <typename T>
static bool IsEquivalent(const Slice& a, const Slice& b) {
  return a.is_equivalent(b);
}

template <typename T>
static void SaveCopyTo(const T& value, T& copy) {
  copy = value;
}

static inline void SaveCopyTo(const Slice& value, Slice& copy) {
  copy = value.Ref();
}

template <typename MetadataTrait>
class Compressor<MetadataTrait, StableValueCompressor> {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    auto& table = encoder->hpack_table();
    if (previously_sent_value_ == value &&
        table.ConvertibleToDynamicIndex(previously_sent_index_)) {
      encoder->EmitIndexed(table.DynamicIndex(previously_sent_index_));
      return;
    }
    previously_sent_index_ = 0;
    auto key = MetadataTrait::key();
    const Slice& value_slice = MetadataValueAsSlice<MetadataTrait>(value);
    if (hpack_constants::SizeForEntry(key.size(), value_slice.size()) >
        HPackEncoderTable::MaxEntrySize()) {
      encoder->EmitLitHdrWithNonBinaryStringKeyNotIdx(
          Slice::FromStaticString(key), value_slice.Ref());
      return;
    }
    encoder->EncodeAlwaysIndexed(
        &previously_sent_index_, key, value_slice.Ref(),
        hpack_constants::SizeForEntry(key.size(), value_slice.size()));
    SaveCopyTo(value, previously_sent_value_);
  }

 private:
  // Previously sent value
  typename MetadataTrait::ValueType previously_sent_value_{};
  // And its index in the table
  uint32_t previously_sent_index_ = 0;
};

template <typename MetadataTrait, typename MetadataTrait::ValueType known_value>
class Compressor<
    MetadataTrait,
    KnownValueCompressor<typename MetadataTrait::ValueType, known_value>> {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    if (value != known_value) {
      LOG(ERROR) << "Not encoding bad " << MetadataTrait::key() << " header";
      encoder->NoteEncodingError();
      return;
    }
    Slice encoded(MetadataTrait::Encode(known_value));
    const auto encoded_length = encoded.length();
    encoder->EncodeAlwaysIndexed(&previously_sent_index_, MetadataTrait::key(),
                                 std::move(encoded),
                                 MetadataTrait::key().size() + encoded_length +
                                     hpack_constants::kEntryOverhead);
  }

 private:
  uint32_t previously_sent_index_ = 0;
};
template <typename MetadataTrait, size_t N>
class Compressor<MetadataTrait, SmallIntegralValuesCompressor<N>> {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    uint32_t* index = nullptr;
    auto& table = encoder->hpack_table();
    if (static_cast<size_t>(value) < N) {
      index = &previously_sent_[static_cast<uint32_t>(value)];
      if (table.ConvertibleToDynamicIndex(*index)) {
        encoder->EmitIndexed(table.DynamicIndex(*index));
        return;
      }
    }
    auto key = Slice::FromStaticString(MetadataTrait::key());
    auto encoded_value = MetadataTrait::Encode(value);
    if (index != nullptr) {
      *index = encoder->EmitLitHdrWithNonBinaryStringKeyIncIdx(
          std::move(key), std::move(encoded_value));
    } else {
      encoder->EmitLitHdrWithNonBinaryStringKeyNotIdx(std::move(key),
                                                      std::move(encoded_value));
    }
  }

 private:
  uint32_t previously_sent_[N] = {};
};

class SliceIndex {
 public:
  void EmitTo(absl::string_view key, const Slice& value, Encoder* encoder);

 private:
  struct ValueIndex {
    ValueIndex(Slice value, uint32_t index)
        : value(std::move(value)), index(index) {}
    Slice value;
    uint32_t index;
  };
  std::vector<ValueIndex> values_;
};

template <typename MetadataTrait>
class Compressor<MetadataTrait, SmallSetOfValuesCompressor> {
 public:
  void EncodeWith(MetadataTrait, const Slice& value, Encoder* encoder) {
    index_.EmitTo(MetadataTrait::key(), value, encoder);
  }

 private:
  SliceIndex index_;
};

struct PreviousTimeout {
  Timeout timeout = Timeout::FromDuration(Duration::Zero());
  // Dynamic table index of a previously sent timeout
  // 0 is guaranteed not in the dynamic table so is a safe initializer
  uint32_t index = 0;
};

class TimeoutCompressorImpl {
 public:
  void EncodeWith(absl::string_view key, Timestamp deadline, Encoder* encoder);

 private:
  static constexpr const size_t kNumPreviousValues = 5;
  PreviousTimeout previous_timeouts_[kNumPreviousValues];
  uint32_t next_previous_value_ = 0;
};

template <typename MetadataTrait>
class Compressor<MetadataTrait, TimeoutCompressor>
    : public TimeoutCompressorImpl {
 public:
  void EncodeWith(MetadataTrait, const typename MetadataTrait::ValueType& value,
                  Encoder* encoder) {
    TimeoutCompressorImpl::EncodeWith(MetadataTrait::key(), value, encoder);
  }
};

template <>
class Compressor<HttpStatusMetadata, HttpStatusCompressor> {
 public:
  void EncodeWith(HttpStatusMetadata, uint32_t status, Encoder* encoder);
};

template <>
class Compressor<HttpMethodMetadata, HttpMethodCompressor> {
 public:
  void EncodeWith(HttpMethodMetadata, HttpMethodMetadata::ValueType method,
                  Encoder* encoder);
};

template <>
class Compressor<HttpSchemeMetadata, HttpSchemeCompressor> {
 public:
  void EncodeWith(HttpSchemeMetadata, HttpSchemeMetadata::ValueType value,
                  Encoder* encoder);
};

}  // namespace hpack_encoder_detail

class HPackCompressor {
  class SliceIndex;

 public:
  HPackCompressor() = default;
  ~HPackCompressor() = default;

  HPackCompressor(const HPackCompressor&) = delete;
  HPackCompressor& operator=(const HPackCompressor&) = delete;
  HPackCompressor(HPackCompressor&&) = default;
  HPackCompressor& operator=(HPackCompressor&&) = default;

  // Maximum table size we'll actually use.
  static constexpr uint32_t kMaxTableSize = 1024 * 1024;

  void SetMaxTableSize(uint32_t max_table_size);
  void SetMaxUsableSize(uint32_t max_table_size);

  uint32_t test_only_table_size() const {
    return table_.test_only_table_size();
  }

  struct EncodeHeaderOptions {
    uint32_t stream_id;
    bool is_end_of_stream;
    bool use_true_binary_metadata;
    size_t max_frame_size;
    CallTracerInterface* call_tracer;
    Http2ZTraceCollector* ztrace_collector;
  };

  template <typename HeaderSet>
  bool EncodeHeaders(const EncodeHeaderOptions& options,
                     const HeaderSet& headers, grpc_slice_buffer* output) {
    SliceBuffer raw;
    hpack_encoder_detail::Encoder encoder(
        this, options.use_true_binary_metadata, raw);
    headers.Encode(&encoder);
    Frame(options, raw, output);
    return !encoder.saw_encoding_errors();
  }

  template <typename HeaderSet>
  bool EncodeRawHeaders(const HeaderSet& headers, SliceBuffer& output) {
    hpack_encoder_detail::Encoder encoder(this, true, output);
    headers.Encode(&encoder);
    return !encoder.saw_encoding_errors();
  }

 private:
  static constexpr size_t kNumFilterValues = 64;
  static constexpr uint32_t kNumCachedGrpcStatusValues = 16;
  friend class hpack_encoder_detail::Encoder;

  void Frame(const EncodeHeaderOptions& options, SliceBuffer& raw,
             grpc_slice_buffer* output);

  // maximum number of bytes we'll use for the decode table (to guard against
  // peers ooming us by setting decode table size high)
  uint32_t max_usable_size_ = hpack_constants::kInitialTableSize;
  // if non-zero, advertise to the decoder that we'll start using a table
  // of this size
  bool advertise_table_size_change_ = false;
  HPackEncoderTable table_;

  grpc_metadata_batch::StatefulCompressor<hpack_encoder_detail::Compressor>
      compression_state_;
};

namespace hpack_encoder_detail {

template <typename MetadataTrait>
void Encoder::Encode(MetadataTrait,
                     const typename MetadataTrait::ValueType& value) {
  compressor_->compression_state_
      .Compressor<MetadataTrait, typename MetadataTrait::CompressionTraits>::
          EncodeWith(MetadataTrait(), value, this);
}

inline HPackEncoderTable& Encoder::hpack_table() { return compressor_->table_; }

}  // namespace hpack_encoder_detail

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_H
