// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_PROTO_LOADER_H_
#define BASE_TEST_TEST_PROTO_LOADER_H_

#include <memory>
#include <string>
#include <string_view>

#include "base/files/file_path.h"

namespace base {

#if defined(COMPONENT_BUILD)
#if defined(WIN32)

#if defined(PROTO_TEST_IMPLEMENTATION)
#define PROTO_TEST_EXPORT __declspec(dllexport)
#else
#define PROTO_TEST_EXPORT __declspec(dllimport)
#endif  // defined(PROTO_TEST_IMPLEMENTATION)

#else  // defined(WIN32)
#if defined(PROTO_TEST_IMPLEMENTATION)
#define PROTO_TEST_EXPORT __attribute__((visibility("default")))
#else
#define PROTO_TEST_EXPORT
#endif
#endif
#else  // defined(COMPONENT_BUILD)
#define PROTO_TEST_EXPORT
#endif

// This class works around the fact that chrome only includes the lite runtime
// of protobufs. Lite protobufs inherit from |MessageLite| and cannot be used to
// parse from text format. Parsing from text
// format is useful in tests. We cannot include the full version of a protobuf
// in test code because it would clash with the lite version.
//
// This class uses the protobuf descriptors (generated at compile time) to
// generate a |Message| that can be used to parse from text. This message can
// then be serialized to binary which can be parsed by the |MessageLite|.
class PROTO_TEST_EXPORT TestProtoSetLoader {
 public:
  explicit TestProtoSetLoader(const base::FilePath& descriptor_path);
  explicit TestProtoSetLoader(std::string_view descriptor_binary_proto);
  ~TestProtoSetLoader();
  TestProtoSetLoader(const TestProtoSetLoader&) = delete;
  TestProtoSetLoader& operator=(const TestProtoSetLoader&) = delete;

  // Parse a text proto into a binary proto. `type_name` is the full message
  // type name including the package. This CHECK fails if the type is not
  // found, or if the text cannot be parsed.
  std::string ParseFromText(std::string_view type_name,
                            const std::string& proto_text) const;

  // Returns the text proto format of `message`. This CHECK fails on error.
  std::string PrintToText(std::string_view type_name,
                          const std::string& message) const;

 private:
  // Hide dependencies of protobuf_full from the header, so that
  // proto_test_support doesn't need to add protobuf_full as a public dep.
  class State;

  std::unique_ptr<State> state_;
};

// Same as TestProtoSetLoader, but for a single message type.
class PROTO_TEST_EXPORT TestProtoLoader {
 public:
  TestProtoLoader(const base::FilePath& descriptor_path,
                  std::string_view type_name);
  TestProtoLoader(std::string_view descriptor_binary_proto,
                  std::string_view type_name);
  ~TestProtoLoader();
  TestProtoLoader(const TestProtoLoader&) = delete;
  TestProtoLoader& operator=(const TestProtoLoader&) = delete;

  void ParseFromText(const std::string& proto_text, std::string& message) const;
  void PrintToText(const std::string& message, std::string& proto_text) const;

 private:
  TestProtoSetLoader set_loader_;
  std::string type_name_;
};

}  // namespace base

#endif  // BASE_TEST_TEST_PROTO_LOADER_H_
