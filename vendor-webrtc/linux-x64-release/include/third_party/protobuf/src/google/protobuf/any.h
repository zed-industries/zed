// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_ANY_H__
#define GOOGLE_PROTOBUF_ANY_H__

#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/port.h"
#include "absl/strings/cord.h"
#include "google/protobuf/message_lite.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class FieldDescriptor;
class Message;

namespace internal {

// "google.protobuf.Any".
PROTOBUF_EXPORT extern const char kAnyFullTypeName[];
// "type.googleapis.com/".
PROTOBUF_EXPORT extern const char kTypeGoogleApisComPrefix[];
// "type.googleprod.com/".
PROTOBUF_EXPORT extern const char kTypeGoogleProdComPrefix[];

std::string GetTypeUrl(absl::string_view message_name,
                       absl::string_view type_url_prefix);

template <typename T>
absl::string_view GetAnyMessageName() {
  return T::FullMessageName();
}

// Helper class used to implement google::protobuf::Any.
#define URL_TYPE std::string
#define VALUE_TYPE std::string

// Helper functions that only require 'lite' messages to work.
PROTOBUF_EXPORT bool InternalPackFromLite(const MessageLite& message,
                                          absl::string_view type_url_prefix,
                                          absl::string_view type_name,
                                          URL_TYPE* dst_url,
                                          VALUE_TYPE* dst_value);
PROTOBUF_EXPORT bool InternalUnpackToLite(absl::string_view type_name,
                                          absl::string_view type_url,
                                          const VALUE_TYPE& value,
                                          MessageLite* dst_message);
PROTOBUF_EXPORT bool InternalIsLite(absl::string_view type_name,
                                    absl::string_view type_url);

// Packs a message using the default type URL prefix: "type.googleapis.com".
// The resulted type URL will be "type.googleapis.com/<message_full_name>".
// Returns false if serializing the message failed.
template <typename T>
bool InternalPackFrom(const T& message, URL_TYPE* dst_url,
                      VALUE_TYPE* dst_value) {
  return InternalPackFromLite(message, kTypeGoogleApisComPrefix,
                              GetAnyMessageName<T>(), dst_url, dst_value);
}
PROTOBUF_EXPORT bool InternalPackFrom(const Message& message, URL_TYPE* dst_url,
                                      VALUE_TYPE* dst_value);

// Packs a message using the given type URL prefix. The type URL will be
// constructed by concatenating the message type's full name to the prefix
// with an optional "/" separator if the prefix doesn't already end with "/".
// For example, both InternalPackFrom(message, "type.googleapis.com") and
// InternalPackFrom(message, "type.googleapis.com/") yield the same result type
// URL: "type.googleapis.com/<message_full_name>".
// Returns false if serializing the message failed.
template <typename T>
bool InternalPackFrom(const T& message, absl::string_view type_url_prefix,
                      URL_TYPE* dst_url, VALUE_TYPE* dst_value) {
  return InternalPackFromLite(message, type_url_prefix, GetAnyMessageName<T>(),
                              dst_url, dst_value);
}
PROTOBUF_EXPORT bool InternalPackFrom(const Message& message,
                                      absl::string_view type_url_prefix,
                                      URL_TYPE* dst_url, VALUE_TYPE* dst_value);

// Unpacks the payload into the given message. Returns false if the message's
// type doesn't match the type specified in the type URL (i.e., the full
// name after the last "/" of the type URL doesn't match the message's actual
// full name) or parsing the payload has failed.
template <typename T>
bool InternalUnpackTo(absl::string_view type_url, const VALUE_TYPE& value,
                      T* message) {
  return InternalUnpackToLite(GetAnyMessageName<T>(), type_url, value, message);
}
PROTOBUF_EXPORT bool InternalUnpackTo(absl::string_view type_url,
                                      const VALUE_TYPE& value,
                                      Message* message);

// Checks whether the type specified in the type URL matches the given type.
// A type is considered matching if its full name matches the full name after
// the last "/" in the type URL.
template <typename T>
bool InternalIs(absl::string_view type_url) {
  return InternalIsLite(GetAnyMessageName<T>(), type_url);
}

#undef URL_TYPE
#undef VALUE_TYPE

// Get the proto type name from Any::type_url value. For example, passing
// "type.googleapis.com/rpc.QueryOrigin" will return "rpc.QueryOrigin" in
// *full_type_name. Returns false if the type_url does not have a "/"
// in the type url separating the full type name.
//
// NOTE: this function is available publicly as a static method on the
// generated message type: google::protobuf::Any::ParseAnyTypeUrl()
bool ParseAnyTypeUrl(absl::string_view type_url, std::string* full_type_name);

// Get the proto type name and prefix from Any::type_url value. For example,
// passing "type.googleapis.com/rpc.QueryOrigin" will return
// "type.googleapis.com/" in *url_prefix and "rpc.QueryOrigin" in
// *full_type_name. Returns false if the type_url does not have a "/" in the
// type url separating the full type name.
bool ParseAnyTypeUrl(absl::string_view type_url, std::string* url_prefix,
                     std::string* full_type_name);

// See if message is of type google.protobuf.Any, if so, return the descriptors
// for "type_url" and "value" fields.
bool GetAnyFieldDescriptors(const Message& message,
                            const FieldDescriptor** type_url_field,
                            const FieldDescriptor** value_field);

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_ANY_H__
