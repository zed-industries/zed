// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// This header is used to emit a Foo_Versions struct for each message Foo. The
// Foo_Versions struct contains parameter metadata for each defined version of a
// message. The structure looks something like this:
//
//     struct Foo_Versions {
//       using ParamsType = Foo_Params;
//
//       struct V0 {
//         using VersionParams = ParamsType::V0;
//         static constexpr internal::ParamMetadata kParams[] = {
//           {offsetof(VersionParams, field1), sizeof(VersionParams::field1),
//            ...},
//           {offsetof(VersionParams, field2), sizeof(VersionParams::field2),
//            ...},
//            ...etc.
//         };
//       };
//       struct V1 {
//         ...
//       };
//     };
//
// This structure is in turn used by message_base_declaration_macros.h to
// generated an aggregated array of version metadata that can be used at runtime
// for message validation.

#define IPCZ_MSG_BEGIN_INTERFACE(name)
#define IPCZ_MSG_END_INTERFACE()
#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl) \
  struct name##_Versions {            \
    using ParamsType = name##_Params;

#define IPCZ_MSG_END() \
  }                    \
  ;

#define IPCZ_MSG_BEGIN_VERSION(version)           \
  struct V##version {                             \
    static constexpr int kVersion = version;      \
    using VersionParams = ParamsType::V##version; \
    static constexpr internal::ParamMetadata kParams[] = {
#define IPCZ_MSG_END_VERSION(version) \
  }                                   \
  ;                                   \
  }                                   \
  ;

#define IPCZ_MSG_PARAM(type, name)                                \
  {offsetof(VersionParams, name), sizeof(VersionParams::name), 0, \
   internal::ParamType::kData},
#define IPCZ_MSG_PARAM_ARRAY(type, name)                                     \
  {offsetof(VersionParams, name), sizeof(VersionParams::name), sizeof(type), \
   internal::ParamType::kDataArray},
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)                        \
  {offsetof(VersionParams, name), sizeof(VersionParams::name), 0, \
   internal::ParamType::kDriverObject},
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)                  \
  {offsetof(VersionParams, name), sizeof(VersionParams::name), 0, \
   internal::ParamType::kDriverObjectArray},
