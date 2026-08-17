// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_FILE_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_FILE_H__

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/cpp/enum.h"
#include "google/protobuf/compiler/cpp/extension.h"
#include "google/protobuf/compiler/cpp/helpers.h"
#include "google/protobuf/compiler/cpp/message.h"
#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/compiler/cpp/service.h"
#include "google/protobuf/compiler/scc.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {
class PROTOC_EXPORT FileGenerator {
 public:
  FileGenerator(const FileDescriptor* file, const Options& options);

  FileGenerator(const FileGenerator&) = delete;
  FileGenerator& operator=(const FileGenerator&) = delete;

  ~FileGenerator() = default;

  // info_path, if non-empty, should be the path (relative to printer's
  // output) to the metadata file describing this proto header.
  void GenerateProtoHeader(io::Printer* p, absl::string_view info_path);
  // info_path, if non-empty, should be the path (relative to printer's
  // output) to the metadata file describing this PB header.
  void GeneratePBHeader(io::Printer* p, absl::string_view info_path);
  void GenerateSource(io::Printer* p);

  // The following member functions are used when the lite_implicit_weak_fields
  // option is set. In this mode the code is organized a bit differently to
  // promote better linker stripping of unused code. In particular, we generate
  // one .cc file per message, one .cc file per extension, and a main pb.cc file
  // containing everything else.

  int NumMessages() const { return message_generators_.size(); }
  int NumExtensions() const { return extension_generators_.size(); }
  // Generates the source file for one message.
  void GenerateSourceForMessage(int idx, io::Printer* p);
  // Generates the source file for one extension.
  void GenerateSourceForExtension(int idx, io::Printer* p);
  // Generates a source file containing everything except messages and
  // extensions.
  void GenerateGlobalSource(io::Printer* p);

 private:
  // Generates a file, setting up the necessary accoutrements that start and
  // end the file, calling `cb` in between.
  //
  // This includes header guards and file-global variables.
  void GenerateFile(io::Printer* p, GeneratedFileType file_type,
                    std::function<void()> cb);

  // Generates a static initializers with all the existing values from
  // `static_initializers_`.
  // They run in `PROTOBUF_ATTRIBUTE_INIT_PRIORITY1` and
  // `PROTOBUF_ATTRIBUTE_INIT_PRIORITY2` priority respectively.
  void GenerateStaticInitializer(io::Printer* p);

  // Shared code between the two header generators.
  void GenerateSharedHeaderCode(io::Printer* p);

  // Internal type used by GenerateForwardDeclarations (defined in file.cc).
  class ForwardDeclarations;
  struct CrossFileReferences;

  void IncludeFile(absl::string_view google3_name, io::Printer* p) {
    DoIncludeFile(google3_name, false, p);
  }
  void IncludeFileAndExport(absl::string_view google3_name, io::Printer* p) {
    DoIncludeFile(google3_name, true, p);
  }
  void DoIncludeFile(absl::string_view google3_name, bool do_export,
                     io::Printer* p);

  std::string CreateHeaderInclude(absl::string_view basename,
                                  const FileDescriptor* file);
  void GetCrossFileReferencesForField(const FieldDescriptor* field,
                                      CrossFileReferences* refs);
  void GetCrossFileReferencesForFile(const FileDescriptor* file,
                                     CrossFileReferences* refs);
  void GenerateInternalForwardDeclarations(const CrossFileReferences& refs,
                                           io::Printer* p);
  void GenerateSourceIncludes(io::Printer* p);
  void GenerateSourcePrelude(io::Printer* p);
  void GenerateSourceDefaultInstance(int idx, io::Printer* p);

  void GenerateInitForSCC(const SCC* scc, const CrossFileReferences& refs,
                          io::Printer* p);
  void GenerateReflectionInitializationCode(io::Printer* p);

  // For other imports, generates their forward-declarations.
  void GenerateForwardDeclarations(io::Printer* p);

  // Generates top or bottom of a header file.
  void GenerateTopHeaderGuard(io::Printer* p, GeneratedFileType file_type);
  void GenerateBottomHeaderGuard(io::Printer* p, GeneratedFileType file_type);

  // Generates #include directives.
  void GenerateLibraryIncludes(io::Printer* p);
  void GenerateDependencyIncludes(io::Printer* p);

  // Generate a pragma to pull in metadata using the given info_path (if
  // non-empty). info_path should be relative to printer's output.
  void GenerateMetadataPragma(io::Printer* p, absl::string_view info_path);

  // Generates a couple of different pieces before definitions:
  void GenerateGlobalStateFunctionDeclarations(io::Printer* p);

  // Generates types for classes.
  void GenerateMessageDefinitions(io::Printer* p);

  void GenerateEnumDefinitions(io::Printer* p);

  // Generates generic service definitions.
  void GenerateServiceDefinitions(io::Printer* p);

  // Generates extension identifiers.
  void GenerateExtensionIdentifiers(io::Printer* p);

  // Generates inline function definitions.
  void GenerateInlineFunctionDefinitions(io::Printer* p);

  void GenerateProto2NamespaceEnumSpecializations(io::Printer* p);

  // Sometimes the names we use in a .proto file happen to be defined as
  // macros on some platforms (e.g., macro/minor used in plugin.proto are
  // defined as macros in sys/types.h on FreeBSD and a few other platforms).
  // To make the generated code compile on these platforms, we either have to
  // undef the macro for these few platforms, or rename the field name for all
  // platforms. Since these names are part of protobuf public API, renaming is
  // generally a breaking change so we prefer the #undef approach.
  void GenerateMacroUndefs(io::Printer* p);

  // Calculates if we should skip importing a specific dependency.
  bool ShouldSkipDependencyImports(const FileDescriptor* dep) const;

  bool IsDepWeak(const FileDescriptor* dep) const {
    if (weak_deps_.count(dep) != 0) {
      ABSL_CHECK(!options_.opensource_runtime);
      return true;
    }
    return false;
  }

  // For testing only.  Returns the descriptors ordered topologically.
  std::vector<const Descriptor*> MessagesInTopologicalOrder() const;
  friend class FileGeneratorFriendForTesting;

  absl::flat_hash_set<const FileDescriptor*> weak_deps_;

  std::vector<absl::AnyInvocable<void(io::Printer*)>> static_initializers_[2];

  const FileDescriptor* file_;
  Options options_;

  MessageSCCAnalyzer scc_analyzer_;

  // This member is unused and should be deleted once all old-style variable
  // maps are gone.
  // TODO
  absl::flat_hash_map<absl::string_view, std::string> variables_;

  // Contains the post-order walk of all the messages (and nested messages)
  // defined in this file. If you need a pre-order walk just reverse iterate.
  std::vector<std::unique_ptr<MessageGenerator>> message_generators_;
  std::vector<int> message_generators_topologically_ordered_;
  std::vector<std::unique_ptr<EnumGenerator>> enum_generators_;
  std::vector<std::unique_ptr<ServiceGenerator>> service_generators_;
  std::vector<std::unique_ptr<ExtensionGenerator>> extension_generators_;
};

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_FILE_H__
