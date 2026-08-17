// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Defines utilities for the TypeResolver.

#ifndef GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_UTIL_H__
#define GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_UTIL_H__

#include "google/protobuf/type.pb.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
class DescriptorPool;
namespace util {
class TypeResolver;

// Creates a TypeResolver that serves type information in the given descriptor
// pool. Caller takes ownership of the returned TypeResolver.
PROTOBUF_EXPORT TypeResolver* NewTypeResolverForDescriptorPool(
    absl::string_view url_prefix, const DescriptorPool* pool);

// Performs a direct conversion from a descriptor to a type proto.
PROTOBUF_EXPORT google::protobuf::Type ConvertDescriptorToType(
    absl::string_view url_prefix, const Descriptor& descriptor);

// Performs a direct conversion from an enum descriptor to a type proto.
PROTOBUF_EXPORT google::protobuf::Enum ConvertDescriptorToType(
    const EnumDescriptor& descriptor);

}  // namespace util
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_UTIL_TYPE_RESOLVER_UTIL_H__
