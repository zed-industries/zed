// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_EXTENSION_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_EXTENSION_H__

#include <string>

#include "absl/container/btree_set.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class ExtensionGenerator {
 public:
  ExtensionGenerator(absl::string_view root_or_message_class_name,
                     const FieldDescriptor* descriptor,
                     const GenerationOptions& generation_options);
  ~ExtensionGenerator() = default;

  ExtensionGenerator(const ExtensionGenerator&) = delete;
  ExtensionGenerator& operator=(const ExtensionGenerator&) = delete;

  void GenerateMembersHeader(io::Printer* printer) const;
  void GenerateStaticVariablesInitialization(io::Printer* printer) const;
  void DetermineObjectiveCClassDefinitions(
      absl::btree_set<std::string>* fwd_decls) const;
  void DetermineNeededFiles(
      absl::flat_hash_set<const FileDescriptor*>* deps) const;

 private:
  std::string root_or_message_class_name_;
  std::string method_name_;
  const FieldDescriptor* descriptor_;
  const GenerationOptions& generation_options_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MESSAGE_H__
