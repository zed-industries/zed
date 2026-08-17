// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_ANNOTATION_TEST_UTIL_H__
#define GOOGLE_PROTOBUF_COMPILER_ANNOTATION_TEST_UTIL_H__

#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/testing/googletest.h"
#include <gtest/gtest.h>
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"

// Utilities that assist in writing tests for generator annotations.
// See java/internal/annotation_unittest.cc for an example.
namespace google {
namespace protobuf {
namespace compiler {
namespace annotation_test_util {

// Struct that contains the file generated from a .proto file and its
// GeneratedCodeInfo. For example, the Java generator will fill this struct
// (for some 'foo.proto') with:
//   file_path = "Foo.java"
//   file_content = content of Foo.java
//   file_info = parsed content of Foo.java.pb.meta
struct ExpectedOutput {
  std::string file_path;
  std::string file_content;
  GeneratedCodeInfo file_info;
  explicit ExpectedOutput(const std::string& file_path)
      : file_path(file_path) {}
};

// Creates a file with name `filename` and content `data` in temp test
// directory.
void AddFile(absl::string_view filename, absl::string_view data);

// Runs proto compiler. Captures proto file structure in FileDescriptorProto.
// Files will be generated in TestTempDir() folder. Callers of this
// function must read generated files themselves.
//
// filename: source .proto file used to generate code.
// plugin_specific_args: command line arguments specific to current generator.
//     For Java, this value might be "--java_out=annotate_code:test_temp_dir"
// cli: instance of command line interface to run generator. See Java's
//     annotation_unittest.cc for an example of how to initialize it.
// file: output parameter, will be set to the descriptor of the proto file
//     specified in filename.
bool RunProtoCompiler(const std::string& filename,
                      const std::string& plugin_specific_args,
                      CommandLineInterface* cli, FileDescriptorProto* file);

bool DecodeMetadata(const std::string& path, GeneratedCodeInfo* info);

// Finds all of the Annotations for a given source file and path.
// See Location.path in https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto for
// explanation of what path vector is.
void FindAnnotationsOnPath(
    const GeneratedCodeInfo& info, const std::string& source_file,
    const std::vector<int>& path,
    std::vector<const GeneratedCodeInfo::Annotation*>* annotations);

// Finds the Annotation for a given source file and path (or returns null if it
// couldn't). If there are several annotations for given path, returns the first
// one. See Location.path in
// https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto for explanation of what path
// vector is.
const GeneratedCodeInfo::Annotation* FindAnnotationOnPath(
    const GeneratedCodeInfo& info, const std::string& source_file,
    const std::vector<int>& path);

// Returns true if at least one of the provided annotations covers a given
// substring with the given semantic in file_content.
bool AtLeastOneAnnotationMatchesSubstring(
    const std::string& file_content,
    const std::vector<const GeneratedCodeInfo::Annotation*>& annotations,
    const std::string& expected_text,
    absl::optional<GeneratedCodeInfo::Annotation::Semantic> expected_semantic =
        absl::nullopt);

// Returns true if the provided annotation covers a given substring in
// file_content.
bool AnnotationMatchesSubstring(const std::string& file_content,
                                const GeneratedCodeInfo::Annotation* annotation,
                                const std::string& expected_text);

// Returns the text spanned by the annotation if the span is valid; otherwise
// returns nullopt.
absl::optional<absl::string_view> GetAnnotationSubstring(
    absl::string_view file_content,
    const GeneratedCodeInfo::Annotation& annotation);

}  // namespace annotation_test_util
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_ANNOTATION_TEST_UTIL_H__
