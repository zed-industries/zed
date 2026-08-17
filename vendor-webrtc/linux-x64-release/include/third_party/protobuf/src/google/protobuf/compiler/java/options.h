// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_OPTIONS_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_OPTIONS_H__

#include <string>

#include "google/protobuf/port.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// Generator options
struct Options {
  Options()
      : generate_immutable_code(false),
        generate_mutable_code(false),
        generate_shared_code(false),
        enforce_lite(false),
        annotate_code(false),
        strip_nonfunctional_codegen(false),
        jvm_dsl(true) {}

  bool generate_immutable_code;
  bool generate_mutable_code;
  bool generate_shared_code;
  // When set, the protoc will generate the current files and all the transitive
  // dependencies as lite runtime.
  bool enforce_lite;
  bool opensource_runtime = google::protobuf::internal::IsOss();
  // If true, we should build .meta files and emit @Generated annotations into
  // generated code.
  bool annotate_code;
  // Name of a file where we will write a list of generated .meta file names,
  // one per line.
  std::string annotation_list_file;
  // Name of a file where we will write a list of generated file names, one
  // per line.
  std::string output_list_file;
  // If true, strip out nonfunctional codegen.
  bool strip_nonfunctional_codegen;

  // If true, generate JVM-specific DSL code.  This defaults to true for
  // compatibility with the old behavior.
  bool jvm_dsl;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_OPTIONS_H__
