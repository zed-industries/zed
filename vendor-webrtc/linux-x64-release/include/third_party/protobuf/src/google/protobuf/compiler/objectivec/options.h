// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_OPTIONS_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_OPTIONS_H__

#include <string>

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// Generation options, documented within generator.cc.
struct GenerationOptions {
  std::string generate_for_named_framework;
  std::string named_framework_to_proto_path_mappings_path;
  std::string runtime_import_prefix;

  bool headers_use_forward_declarations = false;
  bool strip_custom_options = true;
  bool generate_minimal_imports = true;

  // These are experiments that are not officially supported. They can change
  // in behavior or go away at any time.
  bool experimental_multi_source_generation = false;
  bool experimental_strip_nonfunctional_codegen = false;

  // The name of the pragma that will be used to indicate the start of the
  // metadata annotations. Must be set (along with `annotation_guard_name`) for
  // cross-references to be generated.
  std::string annotation_pragma_name;
  // The name of the preprocessor guard that will be used to guard the metadata
  // annotations. Must be set (along with `annotation_pragma_name`) for
  // cross-references to be generated.
  std::string annotation_guard_name;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_OPTIONS_H__
