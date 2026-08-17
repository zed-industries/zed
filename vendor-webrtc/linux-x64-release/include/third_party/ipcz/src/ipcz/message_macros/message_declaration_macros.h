// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// This header emits the usable Foo class for each message Foo, which may be
// used directly by ipcz implementation. In particular this header is used to
// generate version struct accessors (v0, v1, etc.) which expose all available
// message parameters. See message_params_declaration_macros.h for their
// definitions.

#define IPCZ_MSG_BEGIN_INTERFACE(name)
#define IPCZ_MSG_END_INTERFACE()

#define IPCZ_MSG_ID(x) static constexpr uint8_t kId = x

#define IPCZ_MSG_BEGIN(name, id_decl)                            \
  class name : public name##_Base {                              \
   public:                                                       \
    id_decl;                                                     \
    name();                                                      \
    explicit name(decltype(kIncoming));                          \
    ~name();                                                     \
    bool Deserialize(const DriverTransport::RawMessage& message, \
                     const DriverTransport& transport);          \
    bool DeserializeRelayed(absl::Span<const uint8_t> data,      \
                            absl::Span<DriverObject> objects);

#define IPCZ_MSG_END() \
  }                    \
  ;

#define IPCZ_MSG_BEGIN_VERSION(version)                                     \
  static_assert(version < std::size(kVersions) &&                           \
                    kVersions[version].version_number == version,           \
                "Invalid version declaration(s). Message versions must be " \
                "declared sequentially starting from 0.");                  \
                                                                            \
  ParamsType::V##version* v##version() {                                    \
    return params().v##version();                                           \
  }                                                                         \
  const ParamsType::V##version* v##version() const {                        \
    return params().v##version();                                           \
  }

#define IPCZ_MSG_END_VERSION(version)
#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
