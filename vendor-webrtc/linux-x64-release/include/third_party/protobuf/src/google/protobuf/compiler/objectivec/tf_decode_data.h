// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_TF_DECODE_DATA_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_TF_DECODE_DATA_H__

#include <cstddef>
#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"

// Must be included last
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// TODO: PROTOC_EXPORT is only used to allow the CMake build to
// find/link these in the unittest, this is not public api.

// Generate decode data needed for ObjC's GPBDecodeTextFormatName() to transform
// the input into the expected output.
class PROTOC_EXPORT TextFormatDecodeData {
 public:
  TextFormatDecodeData() = default;
  ~TextFormatDecodeData() = default;

  TextFormatDecodeData(const TextFormatDecodeData&) = delete;
  TextFormatDecodeData& operator=(const TextFormatDecodeData&) = delete;

  void AddString(int32_t key, absl::string_view input_for_decode,
                 absl::string_view desired_output);
  size_t num_entries() const { return entries_.size(); }
  std::string Data() const;

  static std::string DecodeDataForString(absl::string_view input_for_decode,
                                         absl::string_view desired_output);

 private:
  typedef std::pair<int32_t, std::string> DataEntry;
  std::vector<DataEntry> entries_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_TF_DECODE_DATA_H__
