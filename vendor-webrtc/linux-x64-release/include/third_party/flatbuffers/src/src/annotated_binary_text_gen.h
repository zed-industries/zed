/*
 * Copyright 2021 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_ANNOTATED_BINARY_TEXT_GEN_H_
#define FLATBUFFERS_ANNOTATED_BINARY_TEXT_GEN_H_

#include <map>
#include <memory>
#include <string>

#include "binary_annotator.h"

namespace flatbuffers {

class AnnotatedBinaryTextGenerator {
 public:
  struct Options {
    // The maximum number of raw bytes to print per line in the output. 8 is a
    // good default due to the largest type (double) being 8 bytes long.
    size_t max_bytes_per_line = 8;

    // The output file postfix, appended between the filename and the extension.
    // Example binary1.bin -> binary1_annotated.bin
    std::string output_postfix = "";

    // The output file extension, replacing any extension given. If empty, don't
    // change the provided extension. AFB = Annotated Flatbuffer Binary
    //
    // Example: binary1.bin -> binary1.afb
    std::string output_extension = "afb";

    // Controls.
    bool include_vector_contents = true;
  };

  explicit AnnotatedBinaryTextGenerator(
      const Options &options, std::map<uint64_t, BinarySection> annotations,
      const uint8_t *const binary, const int64_t binary_length)
      : annotations_(std::move(annotations)),
        binary_(binary),
        binary_length_(binary_length),
        options_(options) {}

  // Generate the annotated binary for the given `filename`. Returns true if the
  // annotated binary was successfully saved.
  bool Generate(const std::string &filename, const std::string &schema_filename,
                const std::string &output_filename = "");

 private:
  const std::map<uint64_t, BinarySection> annotations_;

  // The binary data itself.
  const uint8_t *binary_;
  const int64_t binary_length_;

  // Output configuration
  const Options options_;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_ANNOTATED_BINARY_TEXT_GEN_H_
