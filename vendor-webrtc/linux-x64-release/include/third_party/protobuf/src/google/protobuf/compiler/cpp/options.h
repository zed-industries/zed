// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: rennie@google.com (Jeffrey Rennie)

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_OPTIONS_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_OPTIONS_H__

#include <string>

#include "absl/container/flat_hash_set.h"

namespace google {
namespace protobuf {
namespace compiler {
class AccessInfoMap;
class SplitMap;

namespace cpp {

enum class EnforceOptimizeMode {
  kNoEnforcement,  // Use the runtime specified by the file specific options.
  kSpeed,          // Full runtime with a generated code implementation.
  kCodeSize,       // Full runtime with a reflective implementation.
  kLiteRuntime,
};

enum class BoundsCheckMode {
  kNoEnforcement,       // No enforcement.
  kReturnDefaultValue,  // Return default value if out of bounds.
  kAbort,               // TrapOrAbort if out of bounds.
};

struct FieldListenerOptions {
  bool inject_field_listener_events = false;
  absl::flat_hash_set<std::string> forbidden_field_listener_events;
};

// Generator options (see generator.cc for a description of each):
struct Options {
  const AccessInfoMap* access_info_map = nullptr;
  const SplitMap* split_map = nullptr;
  std::string dllexport_decl;
  std::string runtime_include_base;
  std::string annotation_pragma_name;
  std::string annotation_guard_name;
  FieldListenerOptions field_listener_options;
  EnforceOptimizeMode enforce_mode = EnforceOptimizeMode::kNoEnforcement;
  int num_cc_files = 0;
  BoundsCheckMode bounds_check_mode = BoundsCheckMode::kNoEnforcement;
  bool proto_h = false;
  bool transitive_pb_h = true;
  bool annotate_headers = false;
  bool lite_implicit_weak_fields = false;
  bool descriptor_implicit_weak_messages = false;
  bool bootstrap = false;
  bool opensource_runtime = false;
  bool annotate_accessor = false;
  bool force_split = false;
  // TODO: clean this up after the change is rolled out for 2
  // weeks.
  bool profile_driven_cluster_aux_subtable = true;
#ifdef PROTOBUF_STABLE_EXPERIMENTS
  bool force_eagerly_verified_lazy = true;
  bool force_inline_string = true;
#else   // PROTOBUF_STABLE_EXPERIMENTS
  bool force_eagerly_verified_lazy = false;
  bool force_inline_string = false;
#endif  // !PROTOBUF_STABLE_EXPERIMENTS
  bool strip_nonfunctional_codegen = false;
};

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_OPTIONS_H__
