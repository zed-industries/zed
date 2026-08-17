// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// This header is used to emit a Foo_Base class declaration for each message
// Foo. The main purpose of Foo_Base is to define the list of version metadata
// for the Foo message, and to act as a base class for the generated Foo class
// (see message_declaration_macros.h) so that class can introspect its own
// version metadata. The version metadata cannot be defined by macros in that
// header, because that header already needs to emit accessor methods for each
// version.

#define IPCZ_MSG_BEGIN_INTERFACE(name)
#define IPCZ_MSG_END_INTERFACE()
#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl)                           \
  class name##_Base : public MessageWithParams<name##_Params> { \
   public:                                                      \
    using ParamsType = name##_Params;                           \
    using VersionsType = name##_Versions;                       \
    static_assert(sizeof(ParamsType) % 8 == 0, "Invalid size"); \
    name##_Base() = default;                                    \
    explicit name##_Base(decltype(kIncoming))                   \
        : MessageWithParams(kIncoming) {}                       \
    ~name##_Base() = default;                                   \
    static constexpr internal::VersionMetadata kVersions[] = {
#define IPCZ_MSG_END() \
  }                    \
  ;                    \
  }                    \
  ;

#define IPCZ_MSG_BEGIN_VERSION(version)                                   \
  {VersionsType::V##version::kVersion, ParamsType::v##version##_offset(), \
   sizeof(ParamsType::V##version),                                        \
   absl::MakeSpan(VersionsType::V##version::kParams)},

#define IPCZ_MSG_END_VERSION(version)
#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
