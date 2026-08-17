/*
 * Copyright 2017 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_FLATC_H_
#define FLATBUFFERS_FLATC_H_

#include <functional>
#include <limits>
#include <list>
#include <memory>
#include <string>

#include "flatbuffers/code_generator.h"
#include "flatbuffers/flatbuffers.h"
#include "flatbuffers/idl.h"
#include "flatbuffers/util.h"

namespace flatbuffers {

extern void LogCompilerWarn(const std::string &warn);
extern void LogCompilerError(const std::string &err);

struct FlatCOptions {
  IDLOptions opts;

  std::string program_name;

  std::string output_path;

  std::vector<std::string> filenames;

  std::list<std::string> include_directories_storage;
  std::vector<const char *> include_directories;
  std::vector<const char *> conform_include_directories;
  std::vector<bool> generator_enabled;
  size_t binary_files_from = std::numeric_limits<size_t>::max();
  std::string conform_to_schema;
  std::string annotate_schema;
  bool annotate_include_vector_contents = true;
  bool any_generator = false;
  bool print_make_rules = false;
  bool raw_binary = false;
  bool schema_binary = false;
  bool grpc_enabled = false;
  bool requires_bfbs = false;
  bool file_names_only = false;

  std::vector<std::shared_ptr<CodeGenerator>> generators;
};

struct FlatCOption {
  std::string short_opt;
  std::string long_opt;
  std::string parameter;
  std::string description;
};

class FlatCompiler {
 public:
  typedef void (*WarnFn)(const FlatCompiler *flatc, const std::string &warn,
                         bool show_exe_name);

  typedef void (*ErrorFn)(const FlatCompiler *flatc, const std::string &err,
                          bool usage, bool show_exe_name);

  // Parameters required to initialize the FlatCompiler.
  struct InitParams {
    InitParams() : warn_fn(nullptr), error_fn(nullptr) {}

    WarnFn warn_fn;
    ErrorFn error_fn;
  };

  explicit FlatCompiler(const InitParams &params) : params_(params) {}

  bool RegisterCodeGenerator(const FlatCOption &option,
                             std::shared_ptr<CodeGenerator> code_generator);

  int Compile(const FlatCOptions &options);

  std::string GetShortUsageString(const std::string &program_name) const;
  std::string GetUsageString(const std::string &program_name) const;

  // Parse the FlatC options from command line arguments.
  FlatCOptions ParseFromCommandLineArguments(int argc, const char **argv);

 private:
  void ParseFile(flatbuffers::Parser &parser, const std::string &filename,
                 const std::string &contents,
                 const std::vector<const char *> &include_directories) const;

  void LoadBinarySchema(Parser &parser, const std::string &filename,
                        const std::string &contents);

  void Warn(const std::string &warn, bool show_exe_name = true) const;

  void Error(const std::string &err, bool usage = true,
             bool show_exe_name = true) const;

  void AnnotateBinaries(const uint8_t *binary_schema,
                        uint64_t binary_schema_size,
                        const FlatCOptions &options);

  void ValidateOptions(const FlatCOptions &options);

  Parser GetConformParser(const FlatCOptions &options);

  std::unique_ptr<Parser> GenerateCode(const FlatCOptions &options,
                                       Parser &conform_parser);

  std::map<std::string, std::shared_ptr<CodeGenerator>> code_generators_;

  InitParams params_;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_FLATC_H_
