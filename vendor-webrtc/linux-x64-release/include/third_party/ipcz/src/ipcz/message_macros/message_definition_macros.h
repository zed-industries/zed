// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Generated implementation correpsonding to the declarations from
// message_declaration_macros.h. This also defines storage for the version
// metadata declared in message_base_declaration_macros.h.

#define IPCZ_MSG_BEGIN_INTERFACE(name)
#define IPCZ_MSG_END_INTERFACE()
#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl)                                          \
  name::name() = default;                                                      \
  name::name(decltype(kIncoming)) : name##_Base(kIncoming) {}                  \
  name::~name() = default;                                                     \
  bool name::Deserialize(const DriverTransport::RawMessage& message,           \
                         const DriverTransport& transport) {                   \
    return DeserializeFromTransport(                                           \
        sizeof(ParamsType), absl::MakeSpan(kVersions), message, transport);    \
  }                                                                            \
  bool name::DeserializeRelayed(absl::Span<const uint8_t> data,                \
                                absl::Span<DriverObject> objects) {            \
    return DeserializeFromRelay(sizeof(ParamsType), absl::MakeSpan(kVersions), \
                                data, objects);                                \
  }                                                                            \
  constexpr internal::VersionMetadata name##_Base::kVersions[];

#define IPCZ_MSG_END()
#define IPCZ_MSG_BEGIN_VERSION(version)
#define IPCZ_MSG_END_VERSION(version)
#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
