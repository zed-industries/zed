/*
 * Copyright 2023 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_CODE_GENERATOR_H_
#define FLATBUFFERS_CODE_GENERATOR_H_

#include <string>

#include "flatbuffers/idl.h"

namespace flatbuffers {

struct CodeGenOptions {
  std::string output_path;
};

// A code generator interface for producing converting flatbuffer schema into
// code.
class CodeGenerator {
 public:
  virtual ~CodeGenerator() = default;

  enum Status {
    OK = 0,
    ERROR = 1,
    FAILED_VERIFICATION = 2,
    NOT_IMPLEMENTED = 3
  };

  std::string status_detail;

  // Generate code from the provided `parser`.
  //
  // DEPRECATED: prefer using the other overload of GenerateCode for bfbs.
  virtual Status GenerateCode(const Parser &parser, const std::string &path,
                              const std::string &filename) = 0;

  // Generate code from the provided `parser` and place it in the output.
  virtual Status GenerateCodeString(const Parser &parser,
                                    const std::string &filename,
                                    std::string &output) {
    (void)parser;
    (void)filename;
    (void)output;
    return Status::NOT_IMPLEMENTED;
  }

  // Generate code from the provided `buffer` of given `length`. The buffer is a
  // serialized reflection.fbs.
  virtual Status GenerateCode(const uint8_t *buffer, int64_t length,
                              const CodeGenOptions &options) = 0;

  virtual Status GenerateMakeRule(const Parser &parser, const std::string &path,
                                  const std::string &filename,
                                  std::string &output) = 0;

  virtual Status GenerateGrpcCode(const Parser &parser, const std::string &path,
                                  const std::string &filename) = 0;

  virtual Status GenerateRootFile(const Parser &parser,
                                  const std::string &path) = 0;

  virtual bool IsSchemaOnly() const = 0;

  virtual bool SupportsBfbsGeneration() const = 0;

  virtual bool SupportsRootFileGeneration() const = 0;

  virtual IDLOptions::Language Language() const = 0;

  virtual std::string LanguageName() const = 0;

 protected:
  CodeGenerator() = default;

 private:
  // Copying is not supported.
  CodeGenerator(const CodeGenerator &) = delete;
  CodeGenerator &operator=(const CodeGenerator &) = delete;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_CODE_GENERATOR_H_
