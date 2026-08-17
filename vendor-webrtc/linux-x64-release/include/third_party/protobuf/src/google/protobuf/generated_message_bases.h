// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// This file contains helpers for generated code.
//
//  Nothing in this file should be directly referenced by users of protobufs.

#ifndef GOOGLE_PROTOBUF_GENERATED_MESSAGE_BASES_H__
#define GOOGLE_PROTOBUF_GENERATED_MESSAGE_BASES_H__

#include <cstddef>
#include <cstdint>

#include "absl/base/attributes.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/generated_message_util.h"
#include "google/protobuf/message.h"

// Must come last:
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// To save code size, protos without any fields are derived from ZeroFieldsBase
// rather than Message.
class PROTOBUF_EXPORT ZeroFieldsBase : public Message {
 public:
  ABSL_ATTRIBUTE_REINITIALIZES void Clear() PROTOBUF_FINAL { Clear(*this); }
  size_t ByteSizeLong() const PROTOBUF_FINAL { return ByteSizeLong(*this); }
  int GetCachedSize() const { return _impl_._cached_size_.Get(); }
  ::uint8_t* _InternalSerialize(
      ::uint8_t* target, io::EpsCopyOutputStream* stream) const PROTOBUF_FINAL {
    return _InternalSerialize(*this, target, stream);
  }

 protected:
  using Message::Message;
  ~ZeroFieldsBase() PROTOBUF_OVERRIDE;

  static void SharedDtor(MessageLite& msg);
  static void MergeImpl(MessageLite& to, const MessageLite& from);
  static void CopyImpl(Message& to, const Message& from);
  void InternalSwap(ZeroFieldsBase* other);
  static void Clear(MessageLite& msg);
  static size_t ByteSizeLong(const MessageLite& base);
  static ::uint8_t* _InternalSerialize(const MessageLite& msg,
                                       ::uint8_t* target,
                                       io::EpsCopyOutputStream* stream);

  // The following naming is required to match protobuf naming conventions.
  struct {
    internal::CachedSize _cached_size_;  // NOLINT
  } _impl_;                              // NOLINT
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_GENERATED_MESSAGE_BASES_H__
