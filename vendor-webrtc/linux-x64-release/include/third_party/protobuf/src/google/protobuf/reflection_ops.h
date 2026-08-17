// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This header is logically internal, but is made public because it is used
// from protocol-compiler-generated code, which may reside in other components.

#ifndef GOOGLE_PROTOBUF_REFLECTION_OPS_H__
#define GOOGLE_PROTOBUF_REFLECTION_OPS_H__

#include "google/protobuf/message.h"
#include "google/protobuf/port.h"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// Basic operations that can be performed using reflection.
// These can be used as a cheap way to implement the corresponding
// methods of the Message interface, though they are likely to be
// slower than implementations tailored for the specific message type.
//
// This class should stay limited to operations needed to implement
// the Message interface.
//
// This class is really a namespace that contains only static methods.
class PROTOBUF_EXPORT ReflectionOps {
 public:
  ReflectionOps() = delete;

  static void Copy(const Message& from, Message* to);
  static void Merge(const Message& from, Message* to);
  static void Clear(Message* message);
  static bool IsInitialized(const Message& message);
  static bool IsInitialized(const Message& message, bool check_fields,
                            bool check_descendants);
  static void DiscardUnknownFields(Message* message);

  // Finds all unset required fields in the message and adds their full
  // paths (e.g. "foo.bar[5].baz") to *names.  "prefix" will be attached to
  // the front of each name.
  static void FindInitializationErrors(const Message& message,
                                       const std::string& prefix,
                                       std::vector<std::string>* errors);
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_REFLECTION_OPS_H__
