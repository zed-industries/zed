// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef CONFORMANCE_BINARY_JSON_CONFORMANCE_SUITE_H
#define CONFORMANCE_BINARY_JSON_CONFORMANCE_SUITE_H

#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "json/json.h"
#include "conformance_test.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/util/type_resolver.h"

namespace google {
namespace protobuf {

class BinaryAndJsonConformanceSuite : public ConformanceTestSuite {
 public:
  BinaryAndJsonConformanceSuite() = default;

 private:
  void RunSuiteImpl() override;
  bool ParseJsonResponse(const conformance::ConformanceResponse& response,
                         Message* test_message);
  bool ParseResponse(const conformance::ConformanceResponse& response,
                     const ConformanceRequestSetting& setting,
                     Message* test_message) override;
  void SetTypeUrl(absl::string_view type_url) {
    type_url_ = std::string(type_url);
  }

  template <typename MessageType>
  void RunValidBinaryProtobufTest(const std::string& test_name,
                                  ConformanceLevel level,
                                  const std::string& input_protobuf,
                                  const std::string& equivalent_text_format);

  template <typename MessageType>
  void RunValidProtobufTest(const std::string& test_name,
                            ConformanceLevel level,
                            const std::string& input_protobuf,
                            const std::string& equivalent_text_format);

  void RunDelimitedFieldTests();

  template <typename MessageType>
  friend class BinaryAndJsonConformanceSuiteImpl;

  std::unique_ptr<google::protobuf::util::TypeResolver> type_resolver_;
  std::string type_url_;
};

template <typename MessageType>
class BinaryAndJsonConformanceSuiteImpl {
 public:
  explicit BinaryAndJsonConformanceSuiteImpl(
      BinaryAndJsonConformanceSuite* suite, bool run_proto3_tests);

 private:
  using ConformanceRequestSetting =
      BinaryAndJsonConformanceSuite::ConformanceRequestSetting;
  using ConformanceLevel = BinaryAndJsonConformanceSuite::ConformanceLevel;
  constexpr static ConformanceLevel RECOMMENDED = ConformanceLevel::RECOMMENDED;
  constexpr static ConformanceLevel REQUIRED = ConformanceLevel::REQUIRED;

  void RunAllTests();

  void RunBinaryPerformanceTests();
  void RunJsonPerformanceTests();
  void RunJsonTests();
  void RunJsonTestsForStoresDefaultPrimitive();
  void RunJsonTestsForFieldNameConvention();
  void RunJsonTestsForNonRepeatedTypes();
  void RunJsonTestsForRepeatedTypes();
  void RunJsonTestsForNullTypes();
  void RunJsonTestsForWrapperTypes();
  void RunJsonTestsForFieldMask();
  void RunJsonTestsForStruct();
  void RunJsonTestsForValue();
  void RunJsonTestsForAny();
  void RunJsonTestsForUnknownEnumStringValues();
  void RunValidJsonTest(const std::string& test_name, ConformanceLevel level,
                        const std::string& input_json,
                        const std::string& equivalent_text_format);
  void RunValidJsonTestWithMessage(const std::string& test_name,
                                   ConformanceLevel level,
                                   const std::string& input_json,
                                   const std::string& equivalent_text_forma,
                                   const Message& prototype);
  void RunValidJsonTestWithProtobufInput(
      const std::string& test_name, ConformanceLevel level,
      const MessageType& input, const std::string& equivalent_text_format);
  void RunValidJsonIgnoreUnknownTest(const std::string& test_name,
                                     ConformanceLevel level,
                                     const std::string& input_json,
                                     const std::string& equivalent_text_format);
  void RunValidProtobufTest(const std::string& test_name,
                            ConformanceLevel level,
                            const std::string& input_protobuf,
                            const std::string& equivalent_text_format);
  void RunValidBinaryProtobufTest(const std::string& test_name,
                                  ConformanceLevel level,
                                  const std::string& input_protobuf);
  void RunValidBinaryProtobufTest(const std::string& test_name,
                                  ConformanceLevel level,
                                  const std::string& input_protobuf,
                                  const std::string& expected_protobuf);
  void RunBinaryPerformanceMergeMessageWithField(
      const std::string& test_name, const std::string& field_proto);

  void RunValidProtobufTestWithMessage(
      const std::string& test_name, ConformanceLevel level,
      const Message* input, const std::string& equivalent_text_format);

  typedef std::function<bool(const Json::Value&)> Validator;
  void RunValidJsonTestWithValidator(const std::string& test_name,
                                     ConformanceLevel level,
                                     const std::string& input_json,
                                     const Validator& validator);
  void ExpectParseFailureForJson(const std::string& test_name,
                                 ConformanceLevel level,
                                 const std::string& input_json);
  void ExpectSerializeFailureForJson(const std::string& test_name,
                                     ConformanceLevel level,
                                     const std::string& text_format);
  void ExpectParseFailureForProtoWithProtoVersion(const std::string& proto,
                                                  const std::string& test_name,
                                                  ConformanceLevel level);
  void ExpectParseFailureForProto(const std::string& proto,
                                  const std::string& test_name,
                                  ConformanceLevel level);
  void ExpectHardParseFailureForProto(const std::string& proto,
                                      const std::string& test_name,
                                      ConformanceLevel level);
  void TestPrematureEOFForType(google::protobuf::FieldDescriptor::Type type);
  void TestIllegalTags();
  void TestUnmatchedGroup();
  void TestUnknownWireType();
  void TestOneofMessage();
  void TestUnknownMessage();
  void TestUnknownOrdering();
  void TestValidDataForType(
      google::protobuf::FieldDescriptor::Type,
      std::vector<std::pair<std::string, std::string>> values);
  void TestValidDataForRepeatedScalarMessage();
  void TestValidDataForMapType(google::protobuf::FieldDescriptor::Type,
                               google::protobuf::FieldDescriptor::Type);
  void TestValidDataForOneofType(google::protobuf::FieldDescriptor::Type);
  void TestMergeOneofMessage();
  void TestOverwriteMessageValueMap();
  void TestBinaryPerformanceForAlternatingUnknownFields();
  void TestBinaryPerformanceMergeMessageWithRepeatedFieldForType(
      google::protobuf::FieldDescriptor::Type);
  void TestBinaryPerformanceMergeMessageWithUnknownFieldForType(
      google::protobuf::FieldDescriptor::Type);
  void TestJsonPerformanceMergeMessageWithRepeatedFieldForType(
      google::protobuf::FieldDescriptor::Type, std::string field_value);

  enum class Packed {
    kUnspecified = 0,
    kTrue = 1,
    kFalse = 2,
  };
  const FieldDescriptor* GetFieldForType(
      FieldDescriptor::Type type, bool repeated,
      Packed packed = Packed::kUnspecified) const;
  const FieldDescriptor* GetFieldForMapType(
      FieldDescriptor::Type key_type, FieldDescriptor::Type value_type) const;
  const FieldDescriptor* GetFieldForOneofType(FieldDescriptor::Type type,
                                              bool exclusive = false) const;
  std::string SyntaxIdentifier() const;

  BinaryAndJsonConformanceSuite& suite_;
  bool run_proto3_tests_;
};

}  // namespace protobuf
}  // namespace google

#endif  // CONFORMANCE_BINARY_JSON_CONFORMANCE_SUITE_H
