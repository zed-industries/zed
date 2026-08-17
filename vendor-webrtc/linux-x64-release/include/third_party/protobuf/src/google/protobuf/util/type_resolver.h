// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Defines a TypeResolver for the Any message.

#ifndef GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_H__
#define GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_H__

#include <string>

// This inclusion is unused, but we cannot remove it without first fixing IWYU
// issues in upstream dependencies.
#include "google/protobuf/any.pb.h"  // IWYU pragma: keep
#include "google/protobuf/type.pb.h"
#include "absl/status/status.h"
#include "google/protobuf/port.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
class DescriptorPool;
namespace util {

// Abstract interface for a type resolver.
//
// Implementations of this interface must be thread-safe.
class PROTOBUF_EXPORT TypeResolver {
 public:
  TypeResolver() {}
  TypeResolver(const TypeResolver&) = delete;
  TypeResolver& operator=(const TypeResolver&) = delete;
  virtual ~TypeResolver() {}

  // Resolves a type url for a message type.
  virtual absl::Status ResolveMessageType(
      const std::string& type_url, google::protobuf::Type* message_type) = 0;

  // Resolves a type url for an enum type.
  virtual absl::Status ResolveEnumType(const std::string& type_url,
                                       google::protobuf::Enum* enum_type) = 0;
};

}  // namespace util
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_H__
