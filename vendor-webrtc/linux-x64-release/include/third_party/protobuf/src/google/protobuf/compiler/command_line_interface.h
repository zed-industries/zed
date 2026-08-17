// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// Implements the Protocol Compiler front-end such that it may be reused by
// custom compilers written to support other languages.

#ifndef GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_H__
#define GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_H__

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/btree_map.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/descriptor_database.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Descriptor;           // descriptor.h
class DescriptorDatabase;   // descriptor_database.h
class DescriptorPool;       // descriptor.h
class FileDescriptor;       // descriptor.h
class FileDescriptorSet;    // descriptor.h
class FileDescriptorProto;  // descriptor.pb.h
template <typename T>
class RepeatedPtrField;          // repeated_field.h
class SimpleDescriptorDatabase;  // descriptor_database.h

namespace compiler {

class CodeGenerator;     // code_generator.h
class GeneratorContext;  // code_generator.h
class DiskSourceTree;    // importer.h

struct TransitiveDependencyOptions {
  bool include_json_name = false;
  bool include_source_code_info = false;
  bool retain_options = false;
};

// This class implements the command-line interface to the protocol compiler.
// It is designed to make it very easy to create a custom protocol compiler
// supporting the languages of your choice.  For example, if you wanted to
// create a custom protocol compiler binary which includes both the regular
// C++ support plus support for your own custom output "Foo", you would
// write a class "FooGenerator" which implements the CodeGenerator interface,
// then write a main() procedure like this:
//
//   int main(int argc, char* argv[]) {
//     google::protobuf::compiler::CommandLineInterface cli;
//
//     // Support generation of C++ source and headers.
//     google::protobuf::compiler::cpp::CppGenerator cpp_generator;
//     cli.RegisterGenerator("--cpp_out", &cpp_generator,
//       "Generate C++ source and header.");
//
//     // Support generation of Foo code.
//     FooGenerator foo_generator;
//     cli.RegisterGenerator("--foo_out", &foo_generator,
//       "Generate Foo file.");
//
//     return cli.Run(argc, argv);
//   }
//
// The compiler is invoked with syntax like:
//   protoc --cpp_out=outdir --foo_out=outdir --proto_path=src src/foo.proto
//
// The .proto file to compile can be specified on the command line using either
// its physical file path, or a virtual path relative to a directory specified
// in --proto_path. For example, for src/foo.proto, the following two protoc
// invocations work the same way:
//   1. protoc --proto_path=src src/foo.proto (physical file path)
//   2. protoc --proto_path=src foo.proto (virtual path relative to src)
//
// If a file path can be interpreted both as a physical file path and as a
// relative virtual path, the physical file path takes precedence.
//
// For a full description of the command-line syntax, invoke it with --help.
class PROTOC_EXPORT CommandLineInterface {
 public:
  static const char* const kPathSeparator;

  CommandLineInterface();
  CommandLineInterface(const CommandLineInterface&) = delete;
  CommandLineInterface& operator=(const CommandLineInterface&) = delete;
  ~CommandLineInterface();

  // Register a code generator for a language.
  //
  // Parameters:
  // * flag_name: The command-line flag used to specify an output file of
  //   this type.  The name must start with a '-'.  If the name is longer
  //   than one letter, it must start with two '-'s.
  // * generator: The CodeGenerator which will be called to generate files
  //   of this type.
  // * help_text: Text describing this flag in the --help output.
  //
  // Some generators accept extra parameters.  You can specify this parameter
  // on the command-line by placing it before the output directory, separated
  // by a colon:
  //   protoc --foo_out=enable_bar:outdir
  // The text before the colon is passed to CodeGenerator::Generate() as the
  // "parameter".
  void RegisterGenerator(const std::string& flag_name, CodeGenerator* generator,
                         const std::string& help_text);

  // Register a code generator for a language.
  // Besides flag_name you can specify another option_flag_name that could be
  // used to pass extra parameters to the registered code generator.
  // Suppose you have registered a generator by calling:
  //   command_line_interface.RegisterGenerator("--foo_out", "--foo_opt", ...)
  // Then you could invoke the compiler with a command like:
  //   protoc --foo_out=enable_bar:outdir --foo_opt=enable_baz
  // This will pass "enable_bar,enable_baz" as the parameter to the generator.
  void RegisterGenerator(const std::string& flag_name,
                         const std::string& option_flag_name,
                         CodeGenerator* generator,
                         const std::string& help_text);

  // Enables "plugins".  In this mode, if a command-line flag ends with "_out"
  // but does not match any registered generator, the compiler will attempt to
  // find a "plugin" to implement the generator.  Plugins are just executables.
  // They should live somewhere in the PATH.
  //
  // The compiler determines the executable name to search for by concatenating
  // exe_name_prefix with the unrecognized flag name, removing "_out".  So, for
  // example, if exe_name_prefix is "protoc-" and you pass the flag --foo_out,
  // the compiler will try to run the program "protoc-gen-foo".
  //
  // The plugin program should implement the following usage:
  //   plugin [--out=OUTDIR] [--parameter=PARAMETER] PROTO_FILES < DESCRIPTORS
  // --out indicates the output directory (as passed to the --foo_out
  // parameter); if omitted, the current directory should be used.  --parameter
  // gives the generator parameter, if any was provided (see below).  The
  // PROTO_FILES list the .proto files which were given on the compiler
  // command-line; these are the files for which the plugin is expected to
  // generate output code.  Finally, DESCRIPTORS is an encoded FileDescriptorSet
  // (as defined in descriptor.proto).  This is piped to the plugin's stdin.
  // The set will include descriptors for all the files listed in PROTO_FILES as
  // well as all files that they import.  The plugin MUST NOT attempt to read
  // the PROTO_FILES directly -- it must use the FileDescriptorSet.
  //
  // The plugin should generate whatever files are necessary, as code generators
  // normally do.  It should write the names of all files it generates to
  // stdout.  The names should be relative to the output directory, NOT absolute
  // names or relative to the current directory.  If any errors occur, error
  // messages should be written to stderr.  If an error is fatal, the plugin
  // should exit with a non-zero exit code.
  //
  // Plugins can have generator parameters similar to normal built-in
  // generators. Extra generator parameters can be passed in via a matching
  // "_opt" parameter. For example:
  //   protoc --plug_out=enable_bar:outdir --plug_opt=enable_baz
  // This will pass "enable_bar,enable_baz" as the parameter to the plugin.
  //
  void AllowPlugins(const std::string& exe_name_prefix);

  // Run the Protocol Compiler with the given command-line parameters.
  // Returns the error code which should be returned by main().
  //
  // It may not be safe to call Run() in a multi-threaded environment because
  // it calls strerror().  I'm not sure why you'd want to do this anyway.
  int Run(int argc, const char* const argv[]);

  // DEPRECATED. Calling this method has no effect. Protocol compiler now
  // always try to find the .proto file relative to the current directory
  // first and if the file is not found, it will then treat the input path
  // as a virtual path.
  void SetInputsAreProtoPathRelative(bool /* enable */) {}

  // Provides some text which will be printed when the --version flag is
  // used.  The version of libprotoc will also be printed on the next line
  // after this text.
  void SetVersionInfo(const std::string& text) { version_info_ = text; }


  // Configure protoc to act as if we're in opensource.
  void set_opensource_runtime(bool opensource) {
    opensource_runtime_ = opensource;
  }

 private:
  // -----------------------------------------------------------------

  class ErrorPrinter;
  class GeneratorContextImpl;
  class MemoryOutputStream;
  using GeneratorContextMap =
      absl::flat_hash_map<std::string, std::unique_ptr<GeneratorContextImpl>>;

  // Clear state from previous Run().
  void Clear();

  // Remaps the proto file so that it is relative to one of the directories
  // in proto_path_.  Returns false if an error occurred.
  bool MakeProtoProtoPathRelative(DiskSourceTree* source_tree,
                                  std::string* proto,
                                  DescriptorDatabase* fallback_database);

  // Remaps each file in input_files_ so that it is relative to one of the
  // directories in proto_path_.  Returns false if an error occurred.
  bool MakeInputsBeProtoPathRelative(DiskSourceTree* source_tree,
                                     DescriptorDatabase* fallback_database);

  // Fails if these files use proto3 optional and the code generator doesn't
  // support it. This is a permanent check.
  bool EnforceProto3OptionalSupport(
      const std::string& codegen_name, uint64_t supported_features,
      const std::vector<const FileDescriptor*>& parsed_files) const;

  bool EnforceEditionsSupport(
      const std::string& codegen_name, uint64_t supported_features,
      Edition minimum_edition, Edition maximum_edition,
      const std::vector<const FileDescriptor*>& parsed_files) const;

  bool EnforceProtocEditionsSupport(
      const std::vector<const FileDescriptor*>& parsed_files) const;


  // Return status for ParseArguments() and InterpretArgument().
  enum ParseArgumentStatus {
    PARSE_ARGUMENT_DONE_AND_CONTINUE,
    PARSE_ARGUMENT_DONE_AND_EXIT,
    PARSE_ARGUMENT_FAIL
  };

  // Parse all command-line arguments.
  ParseArgumentStatus ParseArguments(int argc, const char* const argv[]);

  // Read an argument file and append the file's content to the list of
  // arguments. Return false if the file cannot be read.
  bool ExpandArgumentFile(const char* file,
                          std::vector<std::string>* arguments);

  // Parses a command-line argument into a name/value pair.  Returns
  // true if the next argument in the argv should be used as the value,
  // false otherwise.
  //
  // Examples:
  //   "-Isrc/protos" ->
  //     name = "-I", value = "src/protos"
  //   "--cpp_out=src/foo.pb2.cc" ->
  //     name = "--cpp_out", value = "src/foo.pb2.cc"
  //   "foo.proto" ->
  //     name = "", value = "foo.proto"
  bool ParseArgument(const char* arg, std::string* name, std::string* value);

  // Interprets arguments parsed with ParseArgument.
  ParseArgumentStatus InterpretArgument(const std::string& name,
                                        const std::string& value);

  // Print the --help text to stderr.
  void PrintHelpText();

  // Loads proto_path_ into the provided source_tree.
  bool InitializeDiskSourceTree(DiskSourceTree* source_tree,
                                DescriptorDatabase* fallback_database);

  // Verify that all the input files exist in the given database.
  bool VerifyInputFilesInDescriptors(DescriptorDatabase* fallback_database);

  // Parses input_files_ into parsed_files
  bool ParseInputFiles(DescriptorPool* descriptor_pool,
                       DiskSourceTree* source_tree,
                       std::vector<const FileDescriptor*>* parsed_files);

  bool SetupFeatureResolution(DescriptorPool& pool);

  // Generate the given output file from the given input.
  struct OutputDirective;  // see below
  bool GenerateOutput(const std::vector<const FileDescriptor*>& parsed_files,
                      const OutputDirective& output_directive,
                      GeneratorContext* generator_context);
  bool GeneratePluginOutput(
      const std::vector<const FileDescriptor*>& parsed_files,
      const std::string& plugin_name, const std::string& parameter,
      GeneratorContext* generator_context, std::string* error);

  // Implements --encode and --decode.
  bool EncodeOrDecode(const DescriptorPool* pool);

  // Implements the --descriptor_set_out option.
  bool WriteDescriptorSet(
      const std::vector<const FileDescriptor*>& parsed_files);

  // Implements the --edition_defaults_out option.
  bool WriteEditionDefaults(const DescriptorPool& pool);

  // Implements the --dependency_out option
  bool GenerateDependencyManifestFile(
      const std::vector<const FileDescriptor*>& parsed_files,
      const GeneratorContextMap& output_directories,
      DiskSourceTree* source_tree);

  // Implements the --print_free_field_numbers. This function prints free field
  // numbers into stdout for the message and it's nested message types in
  // post-order, i.e. nested types first. Printed range are left-right
  // inclusive, i.e. [a, b].
  //
  // Groups:
  // For historical reasons, groups are considered to share the same
  // field number space with the parent message, thus it will not print free
  // field numbers for groups. The field numbers used in the groups are
  // excluded in the free field numbers of the parent message.
  //
  // Extension Ranges:
  // Extension ranges are considered ocuppied field numbers and they will not be
  // listed as free numbers in the output.
  void PrintFreeFieldNumbers(const Descriptor* descriptor);

  // Get all transitive dependencies of the given file (including the file
  // itself), adding them to the given list of FileDescriptorProtos.  The
  // protos will be ordered such that every file is listed before any file that
  // depends on it, so that you can call DescriptorPool::BuildFile() on them
  // in order.  Any files in *already_seen will not be added, and each file
  // added will be inserted into *already_seen.  If include_source_code_info
  // (from TransitiveDependencyOptions) is true then include the source code
  // information in the FileDescriptorProtos. If include_json_name is true,
  // populate the json_name field of FieldDescriptorProto for all fields.
  void GetTransitiveDependencies(
      const FileDescriptor* file,
      absl::flat_hash_set<const FileDescriptor*>* already_seen,
      RepeatedPtrField<FileDescriptorProto>* output,
      const TransitiveDependencyOptions& options =
          TransitiveDependencyOptions());


  // -----------------------------------------------------------------

  // The name of the executable as invoked (i.e. argv[0]).
  std::string executable_name_;

  // Version info set with SetVersionInfo().
  std::string version_info_;

  // Registered generators.
  struct GeneratorInfo {
    std::string flag_name;
    std::string option_flag_name;
    CodeGenerator* generator;
    std::string help_text;
  };

  const GeneratorInfo* FindGeneratorByFlag(const std::string& name) const;
  const GeneratorInfo* FindGeneratorByOption(const std::string& option) const;

  absl::btree_map<std::string, GeneratorInfo> generators_by_flag_name_;
  absl::flat_hash_map<std::string, GeneratorInfo> generators_by_option_name_;
  // A map from generator names to the parameters specified using the option
  // flag. For example, if the user invokes the compiler with:
  //   protoc --foo_out=outputdir --foo_opt=enable_bar ...
  // Then there will be an entry ("--foo_out", "enable_bar") in this map.
  absl::flat_hash_map<std::string, std::string> generator_parameters_;
  // Similar to generator_parameters_, stores the parameters for plugins but the
  // key is the actual plugin name e.g. "protoc-gen-foo".
  absl::flat_hash_map<std::string, std::string> plugin_parameters_;

  // See AllowPlugins().  If this is empty, plugins aren't allowed.
  std::string plugin_prefix_;

  // Maps specific plugin names to files.  When executing a plugin, this map
  // is searched first to find the plugin executable.  If not found here, the
  // PATH (or other OS-specific search strategy) is searched.
  absl::flat_hash_map<std::string, std::string> plugins_;

  // Stuff parsed from command line.
  enum Mode {
    MODE_COMPILE,  // Normal mode:  parse .proto files and compile them.
    MODE_ENCODE,   // --encode:  read text from stdin, write binary to stdout.
    MODE_DECODE,   // --decode:  read binary from stdin, write text to stdout.
    MODE_PRINT,    // Print mode: print info of the given .proto files and exit.
  };

  Mode mode_ = MODE_COMPILE;

  enum PrintMode {
    PRINT_NONE,         // Not in MODE_PRINT
    PRINT_FREE_FIELDS,  // --print_free_fields
  };

  PrintMode print_mode_ = PRINT_NONE;

  enum ErrorFormat {
    ERROR_FORMAT_GCC,  // GCC error output format (default).
    ERROR_FORMAT_MSVS  // Visual Studio output (--error_format=msvs).
  };

  ErrorFormat error_format_ = ERROR_FORMAT_GCC;

  // True if we should treat warnings as errors that fail the compilation.
  bool fatal_warnings_ = false;

  std::vector<std::pair<std::string, std::string>>
      proto_path_;                        // Search path for proto files.
  std::vector<std::string> input_files_;  // Names of the input proto files.

  // Names of proto files which are allowed to be imported. Used by build
  // systems to enforce depend-on-what-you-import.
  absl::flat_hash_set<std::string> direct_dependencies_;
  bool direct_dependencies_explicitly_set_ = false;

  // If there's a violation of depend-on-what-you-import, this string will be
  // presented to the user. "%s" will be replaced with the violating import.
  std::string direct_dependencies_violation_msg_;

  // output_directives_ lists all the files we are supposed to output and what
  // generator to use for each.
  struct OutputDirective {
    std::string name;          // E.g. "--foo_out"
    CodeGenerator* generator;  // NULL for plugins
    std::string parameter;
    std::string output_location;
  };
  std::vector<OutputDirective> output_directives_;

  // When using --encode or --decode, this names the type we are encoding or
  // decoding.  (Empty string indicates --decode_raw.)
  std::string codec_type_;

  // If --descriptor_set_in was given, these are filenames containing
  // parsed FileDescriptorSets to be used for loading protos.  Otherwise, empty.
  std::vector<std::string> descriptor_set_in_names_;

  // If --descriptor_set_out was given, this is the filename to which the
  // FileDescriptorSet should be written.  Otherwise, empty.
  std::string descriptor_set_out_name_;

  std::string edition_defaults_out_name_;
  Edition edition_defaults_minimum_;
  Edition edition_defaults_maximum_;

  // If --dependency_out was given, this is the path to the file where the
  // dependency file will be written. Otherwise, empty.
  std::string dependency_out_name_;

  bool experimental_editions_ = false;

  // True if --include_imports was given, meaning that we should
  // write all transitive dependencies to the DescriptorSet.  Otherwise, only
  // the .proto files listed on the command-line are added.
  bool imports_in_descriptor_set_;

  // True if --include_source_info was given, meaning that we should not strip
  // SourceCodeInfo from the DescriptorSet.
  bool source_info_in_descriptor_set_ = false;

  // True if --retain_options was given, meaning that we shouldn't strip any
  // options from the DescriptorSet, even if they have RETENTION_SOURCE
  // specified.
  bool retain_options_in_descriptor_set_ = false;

  // Was the --disallow_services flag used?
  bool disallow_services_ = false;

  // When using --encode, this will be passed to SetSerializationDeterministic.
  bool deterministic_output_ = false;

  bool opensource_runtime_ = google::protobuf::internal::IsOss();

};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_H__
