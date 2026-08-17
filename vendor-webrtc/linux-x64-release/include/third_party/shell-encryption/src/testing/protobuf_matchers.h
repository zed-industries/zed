/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RLWE_TESTING_PROTOBUF_MATCHERS_H_
#define RLWE_TESTING_PROTOBUF_MATCHERS_H_

#include <google/protobuf/message.h>
#include <google/protobuf/message_lite.h>
#include <google/protobuf/util/message_differencer.h>
#include <gmock/gmock.h>

namespace rlwe {
namespace testing {

class EqualsProtoImpl
    : public ::testing::MatcherInterface<const google::protobuf::Message&> {
 public:
  EqualsProtoImpl(const google::protobuf::Message& other) : other_(&other) {}

  inline bool MatchAndExplain(
      const google::protobuf::Message& message,
      ::testing::MatchResultListener* listener) const override {
    if (!google::protobuf::util::MessageDifferencer::Equals(message, *other_)) {
      *listener << "protobufs were not equal";
      return false;
    }
    return true;
  }

  inline void DescribeTo(std::ostream* os) const override {
    *os << "is equal to another protocol buffer";
  }

  inline void DescribeNegationTo(std::ostream* os) const override {
    *os << "is not equal to another protocol buffer";
  }

 private:
  const google::protobuf::Message* other_;  // not owned
};

inline ::testing::Matcher<const google::protobuf::Message&> EqualsProto(
    const google::protobuf::Message& other) {
  return ::testing::Matcher<const google::protobuf::Message&>(new EqualsProtoImpl(other));
}

class EqualsProtoLiteImpl
    : public ::testing::MatcherInterface<const google::protobuf::MessageLite&> {
 public:
  EqualsProtoLiteImpl(const google::protobuf::MessageLite& other) : other_(&other) {}

  bool MatchAndExplain(
      const google::protobuf::MessageLite& message,
      ::testing::MatchResultListener* listener) const override {
    // TODO(b/159369884): Implement robust equality checks.
    if (message.SerializeAsString() != other_->SerializeAsString()) {
      *listener << "protobufs were not equal";
      return false;
    }
    return true;
  }

  void DescribeTo(std::ostream* os) const override {
    *os << "is equal to another protocol buffer";
  }

  void DescribeNegationTo(std::ostream* os) const override {
    *os << "is not equal to another protocol buffer";
  }

 private:
  const google::protobuf::MessageLite* other_;  // not owned
};

inline ::testing::Matcher<const google::protobuf::MessageLite&> EqualsProto(
    const google::protobuf::MessageLite& other) {
  return ::testing::Matcher<const google::protobuf::MessageLite&>(
      new EqualsProtoLiteImpl(other));
}

}  // namespace testing
}  // namespace rlwe

#endif  // RLWE_TESTING_PROTOBUF_MATCHERS_H_
