// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// An RAII type for printing a namespace.
//
// Example:
// {
//   Printer printer(output_stream.get(), '$');
//   const NamespacePrinter namespace_printer(&printer, {"a", "b", "c"});
//   // namespace opening will be opened here
//   ...
//   // namespace closing will be emitted here
// }
//
// By default, the filename will be converted to a macro by substituting '/' and
// '.' characters with '_'.  If a different transformation is required, an
// optional transformation function can be provided.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_NAMESPACE_PRINTER_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_NAMESPACE_PRINTER_H__

#include <string>
#include <vector>

#include "google/protobuf/io/printer.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {

// An RAII type for printing a namespace.
class PROTOC_EXPORT NamespacePrinter final {
 public:
  explicit NamespacePrinter(google::protobuf::io::Printer* p,
                            std::vector<std::string> namespace_components);
  ~NamespacePrinter();

 private:
  google::protobuf::io::Printer* const p_;
  const std::vector<std::string> namespace_components_;
};

#include "google/protobuf/port_undef.inc"

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_NAMESPACE_PRINTER_H__
