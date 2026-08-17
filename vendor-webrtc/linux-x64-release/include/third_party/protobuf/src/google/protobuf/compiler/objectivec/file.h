// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_FILE_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_FILE_H__

#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/objectivec/enum.h"
#include "google/protobuf/compiler/objectivec/extension.h"
#include "google/protobuf/compiler/objectivec/message.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class FileGenerator {
 public:
  // Wrapper for some common state that is shared between file generations to
  // improve performance when more than one file is generated at a time.
  struct CommonState {
    // `include_custom_options` will cause any custom options to be included
    // in the calculations around files defining extensions.
    explicit CommonState(bool include_custom_options)
        : include_custom_options(include_custom_options) {}

    std::vector<const FileDescriptor*>
    CollectMinimalFileDepsContainingExtensions(const FileDescriptor* file);

   private:
    struct MinDepsEntry {
      bool has_extensions;
      // The minimal dependencies that cover all the dependencies with
      // extensions.
      absl::flat_hash_set<const FileDescriptor*> min_deps;
      absl::flat_hash_set<const FileDescriptor*> transitive_deps;
    };
    const MinDepsEntry& CollectMinimalFileDepsContainingExtensionsInternal(
        const FileDescriptor* file);
    absl::flat_hash_map<const FileDescriptor*, MinDepsEntry> deps_info_cache;
    const bool include_custom_options;
  };

  FileGenerator(Edition edition, const FileDescriptor* file,
                const GenerationOptions& generation_options,
                CommonState& common_state);
  ~FileGenerator() = default;

  FileGenerator(const FileGenerator&) = delete;
  FileGenerator& operator=(const FileGenerator&) = delete;

  void GenerateHeader(io::Printer* p, absl::string_view info_path) const;
  void GenerateSource(io::Printer* p) const;

  size_t NumEnums() const { return enum_generators_.size(); }
  size_t NumMessages() const { return message_generators_.size(); }

  void GenerateGlobalSource(io::Printer* p) const;
  void GenerateSourceForMessage(size_t idx, io::Printer* p) const;
  void GenerateSourceForEnums(io::Printer* p) const;

 private:
  enum class GeneratedFileType : int { kHeader, kSource };
  struct GeneratedFileOptions {
    std::vector<std::string> ignored_warnings;
    std::vector<const FileDescriptor*> forced_files_to_import;
    std::vector<std::string> extra_system_headers;
  };

  void GenerateFile(io::Printer* p, GeneratedFileType file_type,
                    const GeneratedFileOptions& file_options,
                    std::function<void()> body) const;
  void GenerateFile(io::Printer* p, GeneratedFileType file_type,
                    std::function<void()> body) const {
    GeneratedFileOptions file_options;
    GenerateFile(p, file_type, file_options, body);
  }

  void EmitRootImplementation(
      io::Printer* p,
      const std::vector<const FileDescriptor*>& deps_with_extensions) const;
  void EmitRootExtensionRegistryImplementation(
      io::Printer* p,
      const std::vector<const FileDescriptor*>& deps_with_extensions) const;
  void EmitFileDescription(io::Printer* p) const;

  enum class PublicDepsHandling : int {
    kAsUsed,        // No special handing, require references to import then.
    kForceInclude,  // Always treat them as needed.
    kExclude,       // Never treat them as needed.
  };
  // `public_deps_handling` controls how the public imports in this file should
  // be handed.
  void DetermineNeededDeps(absl::flat_hash_set<const FileDescriptor*>* deps,
                           PublicDepsHandling public_deps_handling) const;

  bool HeadersUseForwardDeclarations() const {
    // The bundled protos (WKTs) don't make use of forward declarations.
    return !is_bundled_proto_ &&
           generation_options_.headers_use_forward_declarations;
  }

  const Edition edition_;
  const FileDescriptor* file_;
  const GenerationOptions& generation_options_;
  mutable CommonState* common_state_;
  const std::string root_class_name_;
  const std::string file_description_name_;
  const bool is_bundled_proto_;

  std::vector<std::unique_ptr<EnumGenerator>> enum_generators_;
  std::vector<std::unique_ptr<MessageGenerator>> message_generators_;
  // The first file_scoped_extension_count_ are the extensions at file level
  // scope. This can be less than file_->extension_count() when custom options
  // are being filtered away.
  size_t file_scoped_extension_count_;
  std::vector<std::unique_ptr<ExtensionGenerator>> extension_generators_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_FILE_H__
