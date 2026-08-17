// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_IMPORT_WRITER_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_IMPORT_WRITER_H__

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

// Helper class for parsing framework import mappings and generating
// import statements.
class ImportWriter {
 public:
  ImportWriter(const std::string& generate_for_named_framework,
               const std::string& named_framework_to_proto_path_mappings_path,
               const std::string& runtime_import_prefix,
               bool for_bundled_proto);
  ~ImportWriter() = default;

  void AddFile(const FileDescriptor* file, const std::string& header_extension);
  void AddRuntimeImport(const std::string& header_name);
  // This can return an empty string if there is no module for the file. It also
  // does not handle bundled proto files.
  std::string ModuleForFile(const FileDescriptor* file);

  void PrintFileImports(io::Printer* p) const;
  void PrintRuntimeImports(io::Printer* p, bool default_cpp_symbol) const;

 private:
  void ParseFrameworkMappings();

  const std::string generate_for_named_framework_;
  const std::string named_framework_to_proto_path_mappings_path_;
  const std::string runtime_import_prefix_;
  absl::flat_hash_map<std::string, std::string> proto_file_to_framework_name_;
  bool for_bundled_proto_;
  bool need_to_parse_mapping_file_;

  std::vector<std::string> protobuf_imports_;
  std::vector<std::string> other_framework_imports_;
  std::vector<std::string> other_imports_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_IMPORT_WRITER_H__
