// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_FIELD_ACCESS_LISTENER_H__
#define GOOGLE_PROTOBUF_FIELD_ACCESS_LISTENER_H__

#include <cstddef>

#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"


namespace google {
namespace protobuf {

// A default/no-op implementation of message hooks.
//
// See go/statically-dispatched-message-hooks for details.
template <typename Proto>
struct NoOpAccessListener {
  // Number of fields are provided at compile time for the trackers to be able
  // to have stack allocated bitmaps for the fields. This is useful for
  // performance critical trackers. This is also to avoid cyclic dependencies
  // if the number of fields is needed.
  static constexpr int kFields = Proto::_kInternalFieldNumber;
  // Default constructor is called during the static global initialization of
  // the program.
  // We provide a pointer to extract the name of the proto not to get cyclic
  // dependencies on GetDescriptor() and OnGetMetadata() calls. If you want
  // to differentiate the protos during the runtime before the start of the
  // program, use this functor to get its name. We either way need it for
  // LITE_RUNTIME protos as they don't have descriptors at all.
  explicit NoOpAccessListener(absl::string_view (* /*name_extractor*/)()) {}
  // called repeatedly during serialization/deserialization/ByteSize of
  // Reflection as:
  //   AccessListener<MessageT>::OnSerialize(this);
  static void OnSerialize(const MessageLite* /*msg*/) {}
  static void OnDeserialize(const MessageLite* /*msg*/) {}
  static void OnByteSize(const MessageLite* /*msg*/) {}
  static void OnMergeFrom(const MessageLite* /*to*/,
                          const MessageLite* /*from*/) {}

  // NOTE: This function can be called pre-main. Make sure it does not make
  // the state of the listener invalid.
  static void OnGetMetadata() {}

  // called from accessors as:
  //   AccessListener<MessageT>::On$operation(this, &field_storage_);
  // If you need to override this with type, in your hook implementation
  // introduce
  // template <int kFieldNum, typename T>
  // static void On$operation(const MessageLite* msg,
  //                          const T* field) {}
  // And overloads for std::nullptr_t for incomplete types such as Messages,
  // Maps. Extract them using reflection if you need. Consequently, second
  // argument can be null pointer.
  // For an example, see proto_hooks/testing/memory_test_field_listener.h
  // And argument template deduction will deduce the type itself without
  // changing the generated code.

  // add_<field>(f)
  template <int kFieldNum>
  static void OnAdd(const MessageLite* /*msg*/, const void* /*field*/) {}

  // add_<field>()
  template <int kFieldNum>
  static void OnAddMutable(const MessageLite* /*msg*/, const void* /*field*/) {}

  // <field>() and <repeated_field>(i)
  template <int kFieldNum>
  static void OnGet(const MessageLite* /*msg*/, const void* /*field*/) {}

  // clear_<field>()
  template <int kFieldNum>
  static void OnClear(const MessageLite* /*msg*/, const void* /*field*/) {}

  // has_<field>()
  template <int kFieldNum>
  static void OnHas(const MessageLite* /*msg*/, const void* /*field*/) {}

  // <repeated_field>()
  template <int kFieldNum>
  static void OnList(const MessageLite* /*msg*/, const void* /*field*/) {}

  // mutable_<field>()
  template <int kFieldNum>
  static void OnMutable(const MessageLite* /*msg*/, const void* /*field*/) {}

  // mutable_<repeated_field>()
  template <int kFieldNum>
  static void OnMutableList(const MessageLite* /*msg*/, const void* /*field*/) {
  }

  // release_<field>()
  template <int kFieldNum>
  static void OnRelease(const MessageLite* /*msg*/, const void* /*field*/) {}

  // set_<field>() and set_<repeated_field>(i)
  template <int kFieldNum>
  static void OnSet(const MessageLite* /*msg*/, const void* /*field*/) {}

  // <repeated_field>_size()
  template <int kFieldNum>
  static void OnSize(const MessageLite* /*msg*/, const void* /*field*/) {}

  // unknown_fields()
  static void OnUnknownFields(const MessageLite* /*msg*/) {}

  // mutable_unknown_fields()
  static void OnMutableUnknownFields(const MessageLite* /*msg*/) {}

  static void OnHasExtension(const MessageLite* /*msg*/, int /*extension_tag*/,
                             const void* /*field*/) {}
  // TODO: Support clear in the proto compiler.
  static void OnClearExtension(const MessageLite* /*msg*/,
                               int /*extension_tag*/, const void* /*field*/) {}
  static void OnExtensionSize(const MessageLite* /*msg*/, int /*extension_tag*/,
                              const void* /*field*/) {}
  static void OnGetExtension(const MessageLite* /*msg*/, int /*extension_tag*/,
                             const void* /*field*/) {}
  static void OnMutableExtension(const MessageLite* /*msg*/,
                                 int /*extension_tag*/, const void* /*field*/) {
  }
  static void OnSetExtension(const MessageLite* /*msg*/, int /*extension_tag*/,
                             const void* /*field*/) {}
  static void OnReleaseExtension(const MessageLite* /*msg*/,
                                 int /*extension_tag*/, const void* /*field*/) {
  }
  static void OnAddExtension(const MessageLite* /*msg*/, int /*extension_tag*/,
                             const void* /*field*/) {}
  static void OnAddMutableExtension(const MessageLite* /*msg*/,
                                    int /*extension_tag*/,
                                    const void* /*field*/) {}
  static void OnListExtension(const MessageLite* /*msg*/, int /*extension_tag*/,
                              const void* /*field*/) {}
  static void OnMutableListExtension(const MessageLite* /*msg*/,
                                     int /*extension_tag*/,
                                     const void* /*field*/) {}
};

}  // namespace protobuf
}  // namespace google

#ifndef REPLACE_PROTO_LISTENER_IMPL
namespace google {
namespace protobuf {
template <class T>
using AccessListener = NoOpAccessListener<T>;
}  // namespace protobuf
}  // namespace google
#else
// You can put your implementations of hooks/listeners here.
// All hooks are subject to approval by protobuf-team@.

#endif  // !REPLACE_PROTO_LISTENER_IMPL

#endif  // GOOGLE_PROTOBUF_FIELD_ACCESS_LISTENER_H__
