// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// An RAII type for printing an ifdef guard.
//
// This can be used to ensure that appropriate ifdef guards are applied in
// a generated header file.
//
// Example:
// {
//   Printer printer(output_stream.get(), '$');
//   const IfdefGuardPrinter ifdef_guard(&printer, output_path);
//   // #ifdef guard will be emitted here
//   ...
//   // #endif will be emitted here
// }
//
// By default, the filename will be converted to a macro by substituting '/' and
// '.' characters with '_'.  If a different transformation is required, an
// optional transformation function can be provided.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_IFNDEF_GUARD_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_IFNDEF_GUARD_H__

#include <string>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/io/printer.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {

class PROTOC_EXPORT IfdefGuardPrinter final {
 public:
  explicit IfdefGuardPrinter(google::protobuf::io::Printer* p,
                             absl::string_view filename);

  explicit IfdefGuardPrinter(
      google::protobuf::io::Printer* p, absl::string_view filename,
      absl::AnyInvocable<std::string(absl::string_view)> make_ifdef_identifier);

  ~IfdefGuardPrinter();

 private:
  google::protobuf::io::Printer* const p_;
  const std::string ifdef_identifier_;
};

#include "google/protobuf/port_undef.inc"

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_IFNDEF_GUARD_H__
