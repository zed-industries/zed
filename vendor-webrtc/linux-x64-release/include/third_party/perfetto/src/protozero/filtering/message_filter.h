/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_PROTOZERO_FILTERING_MESSAGE_FILTER_H_
#define SRC_PROTOZERO_FILTERING_MESSAGE_FILTER_H_

#include <stdint.h>

#include <memory>
#include <string>
#include <unordered_map>

#include "src/protozero/filtering/filter_bytecode_parser.h"
#include "src/protozero/filtering/message_tokenizer.h"
#include "src/protozero/filtering/string_filter.h"

namespace protozero {

// A class to filter binary-encoded proto messages using an allow-list of field
// ids, also known as "filter bytecode". The filter determines which fields are
// allowed to be passed through in output and strips all the other fields.
// See go/trace-filtering for full design.
// This class takes in input:
// 1) The filter bytecode, loaded once via the LoadFilterBytecode() method.
// 2) A proto-encoded binary message. The message doesn't have to be contiguous,
//    it can be passed as an array of arbitrarily chunked fragments.
// The FilterMessage*() method returns in output a proto message, stripping out
// all unknown fields. If the input is malformed (e.g., unknown proto field wire
// types, lengths out of bound) the whole filtering failed and the |error| flag
// of the FilteredMessage object is set to true.
// The filtering operation is based on rewriting a copy of the message into a
// self-allocated buffer, which is then returned in the output. The input buffer
// is NOT altered.
// Note also that the process of rewriting the protos gets rid of most redundant
// varint encoding (if present). So even if all fields are allow-listed, the
// output might NOT be bitwise identical to the input (but it will be
// semantically equivalent).
// Furthermore the enable_field_usage_tracking() method allows to keep track of
// a histogram of allowed / denied fields. It slows down filtering and is
// intended only on host tools.
class MessageFilter {
 public:
  class Config {
   public:
    bool LoadFilterBytecode(const void* filter_data, size_t len);
    bool SetFilterRoot(std::initializer_list<uint32_t> field_ids);

    const FilterBytecodeParser& filter() const { return filter_; }
    const StringFilter& string_filter() const { return string_filter_; }
    StringFilter& string_filter() { return string_filter_; }
    uint32_t root_msg_index() const { return root_msg_index_; }

   private:
    FilterBytecodeParser filter_;
    StringFilter string_filter_;
    uint32_t root_msg_index_ = 0;
  };

  MessageFilter();
  explicit MessageFilter(Config);
  ~MessageFilter();

  struct InputSlice {
    const void* data;
    size_t len;
  };

  struct FilteredMessage {
    FilteredMessage(std::unique_ptr<uint8_t[]> d, size_t s)
        : data(std::move(d)), size(s) {}
    std::unique_ptr<uint8_t[]> data;
    size_t size;  // The used bytes in |data|. This is <= sizeof(data).
    bool error = false;
  };

  // Loads the filter bytecode that will be used to filter any subsequent
  // message. Must be called before the first call to FilterMessage*().
  // |filter_data| must point to a byte buffer for a proto-encoded ProtoFilter
  // message (see proto_filter.proto).
  bool LoadFilterBytecode(const void* filter_data, size_t len) {
    return config_.LoadFilterBytecode(filter_data, len);
  }

  // This affects the filter starting point of the subsequent FilterMessage*()
  // calls. By default the filtering process starts from the message @ index 0,
  // the root message passed to proto_filter when generating the bytecode
  // (in typical tracing use-cases, this is perfetto.protos.Trace). However, the
  // caller (TracingServiceImpl) might want to filter packets from the 2nd level
  // (perfetto.protos.TracePacket) because the root level is prepended after
  // the fact. This call allows to change the root message for the filter.
  // The argument |field_ids| is an array of proto field ids and determines the
  // path to the new root. For instance, in the case of [1,2,3] SetFilterRoot
  // will identify the sub-message for the field "root.1.2.3" and use that.
  // In order for this to succeed all the fields in the path must be allowed
  // in the filter and must be a nested message type.
  bool SetFilterRoot(std::initializer_list<uint32_t> field_ids) {
    return config_.SetFilterRoot(field_ids);
  }

  // Takes an input message, fragmented in arbitrary slices, and returns a
  // filtered message in output.
  FilteredMessage FilterMessageFragments(const InputSlice*, size_t num_slices);

  // Helper for tests, where the input is a contiguous buffer.
  FilteredMessage FilterMessage(const void* data, size_t len) {
    InputSlice slice{data, len};
    return FilterMessageFragments(&slice, 1);
  }

  // When enabled returns a map of "field path" to "usage counter".
  // The key (std::string) is a binary buffer (i.e. NOT an ASCII/UTF-8 string)
  // which contains a varint for each field. Consider the following:
  // message Root { Sub1 f1 = 1; };
  // message Sub1 { Sub2 f2 = 7;}
  // message Sub2 { string f3 = 5; }
  // The field .f1.f2.f3 will be encoded as \x01\0x07\x05.
  // The value is the number of times that field has been encountered. If the
  // field is not allow-listed in the bytecode (the field is stripped in output)
  // the count will be negative.
  void enable_field_usage_tracking(bool x) { track_field_usage_ = x; }
  const std::unordered_map<std::string, int32_t>& field_usage() const {
    return field_usage_;
  }

  const Config& config() const { return config_; }

  // Returns the helper class used to perform string filtering.
  StringFilter& string_filter() { return config_.string_filter(); }

 private:
  // This is called by FilterMessageFragments().
  // Inlining allows the compiler turn the per-byte call/return into a for loop,
  // while, at the same time, keeping the code easy to read and reason about.
  // It gives a 20-25% speedup (265ms vs 215ms for a 25MB trace).
  void FilterOneByte(uint8_t octet) PERFETTO_ALWAYS_INLINE;

  // No-inline because this is a slowpath (only when usage tracking is enabled).
  void IncrementCurrentFieldUsage(uint32_t field_id,
                                  bool allowed) PERFETTO_NO_INLINE;

  // Gets into an error state which swallows all the input and emits no output.
  void SetUnrecoverableErrorState();

  // We keep track of the nest of messages in a stack. Each StackState
  // object corresponds to a level of nesting in the proto message structure.
  // Every time a new field of type len-delimited that has a corresponding
  // sub-message in the bytecode is encountered, a new StackState is pushed in
  // |stack_|. stack_[0] is a sentinel to prevent over-popping without adding
  // extra branches in the fastpath.
  // |stack_|. stack_[1] is the state of the root message.
  struct StackState {
    uint32_t in_bytes = 0;  // Number of input bytes processed.

    // When |in_bytes| reaches this value, the current state should be popped.
    // This is set when recursing into nested submessages. This is 0 only for
    // stack_[0] (we don't know the size of the root message upfront).
    uint32_t in_bytes_limit = 0;

    // This is set when a len-delimited message is encountered, either a string
    // or a nested submessage that is NOT allow-listed in the bytecode.
    // This causes input bytes to be consumed without being parsed from the
    // input stream. If |passthrough_eaten_bytes| == true, they will be copied
    // as-is in output (e.g. in the case of an allowed string/bytes field).
    uint32_t eat_next_bytes = 0;

    // Keeps tracks of the stream_writer output counter (out_.written()) then
    // the StackState is pushed. This is used to work out, when popping, how
    // many bytes have been written for the current submessage.
    uint32_t out_bytes_written_at_start = 0;

    uint32_t field_id = 0;   // The proto field id for the current message.
    uint32_t msg_index = 0;  // The index of the message filter in the bytecode.

    // This is a pointer to the proto preamble for the current submessage
    // (it's nullptr for stack_[0] and non-null elsewhere). This will be filled
    // with the actual size of the message (out_.written() -
    // |out_bytes_written_at_start|) when finishing (popping) the message.
    // This must be filled using WriteRedundantVarint(). Note that the
    // |size_field_len| is variable and depends on the actual length of the
    // input message. If the output message has roughly the same size of the
    // input message, the length will not be redundant.
    // In other words: the length of the field is reserved when the submessage
    // starts. At that point we know the upper-bound for the output message
    // (a filtered submessage can be <= the original one, but not >). So we
    // reserve as many bytes it takes to write the input length in varint.
    // Then, when the message is finalized and we know the actual output size
    // we backfill the field.
    // Consider the example of a submessage where the input size = 130 (>127,
    // 2 varint bytes) and the output is 120 bytes. The length will be 2 bytes
    // wide even though could have been encoded with just one byte.
    uint8_t* size_field = nullptr;
    uint32_t size_field_len = 0;

    // The pointer to the start of the string to update the string if it is
    // filtered.
    uint8_t* filter_string_ptr = nullptr;

    // How |eat_next_bytes| should be handled. It seems that keeping this field
    // at the end rather than next to |eat_next_bytes| makes the filter a little
    // (but measurably) faster. (likely something related with struct layout vs
    // cache sizes).
    enum FilterAction {
      kDrop,
      kPassthrough,
      kFilterString,
    };
    FilterAction action = FilterAction::kDrop;
  };

  uint32_t out_written() { return static_cast<uint32_t>(out_ - &out_buf_[0]); }

  Config config_;

  std::unique_ptr<uint8_t[]> out_buf_;
  uint8_t* out_ = nullptr;
  uint8_t* out_end_ = nullptr;

  MessageTokenizer tokenizer_;
  std::vector<StackState> stack_;

  bool error_ = false;
  bool track_field_usage_ = false;
  std::unordered_map<std::string, int32_t> field_usage_;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_MESSAGE_FILTER_H_
