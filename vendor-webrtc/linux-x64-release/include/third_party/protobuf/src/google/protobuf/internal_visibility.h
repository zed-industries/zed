// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
#ifndef GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_H__
#define GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_H__

namespace google {
namespace protobuf {

class Arena;
class Message;
class MessageLite;

namespace internal {

class InternalVisibilityForTesting;
class InternalMetadata;

// Empty class to use as a mandatory 'internal token' for functions that have to
// be public, such as arena constructors, but that are for internal use only.
class InternalVisibility {
 private:
  // Note: we don't use `InternalVisibility() = default` here, but default the
  // ctor outside of the class to force a private ctor instance.
  explicit constexpr InternalVisibility();

  friend class ::google::protobuf::Arena;
  friend class ::google::protobuf::Message;
  friend class ::google::protobuf::MessageLite;
  friend class ::google::protobuf::internal::InternalMetadata;

  friend class InternalVisibilityForTesting;
};

inline constexpr InternalVisibility::InternalVisibility() = default;

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_H__
