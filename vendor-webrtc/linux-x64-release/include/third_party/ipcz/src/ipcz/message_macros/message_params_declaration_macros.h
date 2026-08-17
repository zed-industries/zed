// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Defines actual wire structures used to encode a message Foo. The main
// structure is Foo_Params, which consists first of a StructHeader, followed by
// a substructure for each defined version of the message. Version structures
// are concatenated in-order to form the full message at its latest known
// version, so a Foo with versions 0, 1, and 2 will have params laid out as:
//
//     struct Foo_Params {
//       internal::StructHeader header;
//       V0 v0;
//       V1 v1;
//       V2 v2;
//     };
//
// This is to say that V1 does not aggregate V0 and V2 does not aggregate V0 or
// V1. Each version structure contains only the fields added in that version.
//
// Macros in this header specifically emit declarations of each version struct's
// layout, as well as accessors for each version. In practice the whole output
// looks more like this, modulo some hidden utility methods and a sloppier
// arrangement because macros are macros:
//
//     struct Foo_Params {
//        struct V0 {
//          uint32_t some_field;
//          uint64_t some_other_field;
//        };
//
//        struct V1 {
//          uint32_t new_thing;
//        };
//
//        struct V2 {
//          uint64_t one_more_thing;
//          uint32_t and_another;
//        };
//
//        V0* v0() { return LargeEnoughForV0() ? &v0_ : nullptr; }
//        const V0* v0() const { return LargeEnoughForV0() ? &v0_ : nullptr; }
//
//        V1* v1() { return LargeEnoughForV1() ? &v1_ : nullptr; }
//        const V1* v1() const { return LargeEnoughForV1() ? &v1_ : nullptr; }
//
//        V2* v2() { return LargeEnoughForV2() ? &v2_ : nullptr; }
//        const V2* v2() const { return LargeEnoughForV2() ? &v2_ : nullptr; }
//
//        internal::StructHeader header;
//       private:
//        V0 v0_;
//        V1 v1_;
//        V2 v2_;
//      };
//
// Version structures are hidden behind pointer accessors because a validated
// message will always have its parameter payload mapped to this structure, and
// this structure represents the latest known version of the message layout. If
// If the message came from an older version however, memory beyond `v0_` may
// not actually belong to the structure and may not even be safe to address.
//
// Hiding versions >= 1 behind the above accessors ensures that versioned
// structures are used safely by the ipcz implementation.

#define IPCZ_MSG_BEGIN_INTERFACE(name)
#define IPCZ_MSG_END_INTERFACE()

#define IPCZ_MSG_ID(x) static constexpr uint8_t kId = x

#define IPCZ_MSG_BEGIN(name, id_decl)  \
  struct name##_Versions;              \
  struct IPCZ_ALIGN(8) name##_Params { \
    friend class name##_Base;          \
    using TheseParams = name##_Params; \
    name##_Params();                   \
    ~name##_Params();                  \
    id_decl;                           \
    internal::StructHeader header;

#define IPCZ_MSG_END() \
  }                    \
  ;

#define IPCZ_MSG_BEGIN_VERSION(version) struct IPCZ_ALIGN(8) V##version {
#define IPCZ_MSG_END_VERSION(version)                             \
  }                                                               \
  ;                                                               \
                                                                  \
 private:                                                         \
  V##version v##version##_;                                       \
  static constexpr size_t v##version##_offset() {                 \
    return offsetof(TheseParams, v##version##_);                  \
  }                                                               \
  static constexpr size_t v##version##_required_size() {          \
    return v##version##_offset() + sizeof(v##version##_);         \
  }                                                               \
  bool LargeEnoughForV##version() const {                         \
    return header.size >= v##version##_required_size();           \
  }                                                               \
                                                                  \
 public:                                                          \
  V##version* v##version() {                                      \
    return LargeEnoughForV##version() ? &v##version##_ : nullptr; \
  }                                                               \
  const V##version* v##version() const {                          \
    return LargeEnoughForV##version() ? &v##version##_ : nullptr; \
  }

#define IPCZ_MSG_PARAM(type, name) type name;
#define IPCZ_MSG_PARAM_ARRAY(type, name) uint32_t name;
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name) uint32_t name;
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name) \
  internal::DriverObjectArrayData name;
