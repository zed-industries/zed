// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_TEST_TEXTPROTO_H__
#define GOOGLE_PROTOBUF_TEST_TEXTPROTO_H__

#include <gmock/gmock.h>
#include "absl/log/absl_check.h"
#include "absl/memory/memory.h"
#include "google/protobuf/dynamic_message.h"
#include "google/protobuf/text_format.h"

// This file contains private helpers for dealing with textprotos in our
// tests.  We make no guarantees about the behavior in real-world situations,
// and these are only meant for basic unit-tests of protobuf internals.
namespace google {
namespace protobuf {

MATCHER_P(EqualsProto, textproto, "") {
  auto msg = absl::WrapUnique(arg.New());
  return TextFormat::ParseFromString(textproto, msg.get()) &&
         msg->DebugString() == arg.DebugString();
}

MATCHER_P3(EqualsProtoSerialized, pool, type, textproto, "") {
  const Descriptor* desc = pool->FindMessageTypeByName(type);
  DynamicMessageFactory factory(pool);
  auto msg = absl::WrapUnique(factory.GetPrototype(desc)->New());
  return TextFormat::ParseFromString(textproto, msg.get()) &&
         arg.SerializeAsString() == msg->SerializeAsString();
}

class ParseTextOrDie {
 public:
  explicit ParseTextOrDie(absl::string_view text) : text_(text) {}
  template <typename Proto>
  operator Proto() {  // NOLINT(google-explicit-constructor)
    Proto ret;
    ABSL_CHECK(TextFormat::ParseFromString(text_, &ret));
    return ret;
  }

 private:
  absl::string_view text_;
};

}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_TEST_TEXTPROTO_H__
