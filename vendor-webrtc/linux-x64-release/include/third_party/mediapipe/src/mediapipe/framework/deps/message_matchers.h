// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_DEPS_MESSAGE_MATCHERS_H_
#define MEDIAPIPE_DEPS_MESSAGE_MATCHERS_H_

#include <memory>

#include "gmock/gmock.h"
#include "mediapipe/framework/port/core_proto_inc.h"

namespace mediapipe {

class ProtoMatcher {
 public:
  using is_gtest_matcher = void;
  using MessageType = proto_ns::MessageLite;

  explicit ProtoMatcher(const MessageType& message)
      : message_(CloneMessage(message)) {}

  bool MatchAndExplain(const MessageType& m,
                       testing::MatchResultListener*) const {
    return EqualsMessage(*message_, m);
  }
  bool MatchAndExplain(const MessageType* m,
                       testing::MatchResultListener*) const {
    return EqualsMessage(*message_, *m);
  }

  void DescribeTo(std::ostream* os) const {
    *os << "has the same serialization as " << ExpectedMessageDescription();
  }

  void DescribeNegationTo(std::ostream* os) const {
    *os << "does not have the same serialization as "
        << ExpectedMessageDescription();
  }

 private:
  std::unique_ptr<MessageType> CloneMessage(const MessageType& message) {
    std::unique_ptr<MessageType> clone(message.New());
    clone->CheckTypeAndMergeFrom(message);
    return clone;
  }

  bool EqualsMessage(const proto_ns::MessageLite& m_1,
                     const proto_ns::MessageLite& m_2) const {
    std::string s_1, s_2;
    m_1.SerializeToString(&s_1);
    m_2.SerializeToString(&s_2);
    return s_1 == s_2;
  }

  std::string ExpectedMessageDescription() const {
#if defined(MEDIAPIPE_PROTO_LITE)
    return "the expected message";
#else
    return message_->DebugString();
#endif
  }

  const std::shared_ptr<MessageType> message_;
};

inline ProtoMatcher EqualsProto(const proto_ns::MessageLite& message) {
  return ProtoMatcher(message);
}

// for Pointwise
MATCHER(EqualsProto, "") {
  const auto& a = ::testing::get<0>(arg);
  const auto& b = ::testing::get<1>(arg);
  return ::testing::ExplainMatchResult(EqualsProto(b), a, result_listener);
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_MESSAGE_MATCHERS_H_
