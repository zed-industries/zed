// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_LINE_CONSUMER_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_LINE_CONSUMER_H__

#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/io/zero_copy_stream.h"

// Must be included last
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// TODO: PROTOC_EXPORT is only used to allow the CMake build to
// find/link these in the unittest, this is not public api.

// Helper for parsing simple files.
class PROTOC_EXPORT LineConsumer {
 public:
  LineConsumer() = default;
  virtual ~LineConsumer() = default;
  virtual bool ConsumeLine(absl::string_view line, std::string* out_error) = 0;
};

bool PROTOC_EXPORT ParseSimpleFile(absl::string_view path,
                                   LineConsumer* line_consumer,
                                   std::string* out_error);

bool PROTOC_EXPORT ParseSimpleStream(io::ZeroCopyInputStream& input_stream,
                                     absl::string_view stream_name,
                                     LineConsumer* line_consumer,
                                     std::string* out_error);

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_LINE_CONSUMER_H__
