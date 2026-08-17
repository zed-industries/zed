// Protocol Buffers - Google's data interchange format
// Copyright 2022 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_TRACKER_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_TRACKER_H__

#include <vector>

#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {

// Generates printer substitutions for message-level tracker callbacks.
std::vector<google::protobuf::io::Printer::Sub> MakeTrackerCalls(
    const google::protobuf::Descriptor* message, const Options& opts);

// Generates printer substitutions for field-specific tracker callbacks.
std::vector<google::protobuf::io::Printer::Sub> MakeTrackerCalls(
    const google::protobuf::FieldDescriptor* field, const Options& opts);

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_TRACKER_H__
