// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Definitions corresponding to the declarations in
// message_listener_declaration_macros.h. Primarily this emits a large switch
// block to validate messages and deserialize any driver objects they contain,
// before forwarding to the generic OnMessage() -> DispatchMessage(). See
// message_listener_dispatch_macros.h for the generated code that actually
// routes them to specific virtual methods.

#define IPCZ_MSG_BEGIN_INTERFACE(name)                                  \
  bool name##MessageListener::OnMessage(Message& message) {             \
    return DispatchMessage(message);                                    \
  }                                                                     \
  bool name##MessageListener::OnTransportMessage(                       \
      const DriverTransport::RawMessage& raw_message,                   \
      const DriverTransport& transport, IpczDriverHandle envelope) {    \
    if (raw_message.data.size() >= sizeof(internal::MessageHeaderV0)) { \
      const auto& header =                                              \
          *reinterpret_cast<const internal::MessageHeaderV0*>(          \
              raw_message.data.data());                                 \
      switch (header.message_id) {
#define IPCZ_MSG_END_INTERFACE()                                      \
  default:                                                            \
    break;                                                            \
    }                                                                 \
    }                                                                 \
    Message message;                                                  \
    message.SetEnvelope(                                              \
        DriverObject(*transport.driver_object().driver(), envelope)); \
    return message.DeserializeUnknownType(raw_message, transport) &&  \
           OnMessage(message);                                        \
    }

#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl)                                 \
  case name::kId: {                                                   \
    name message(Message::kIncoming);                                 \
    message.SetEnvelope(                                              \
        DriverObject(*transport.driver_object().driver(), envelope)); \
    if (!message.Deserialize(raw_message, transport)) {               \
      return false;                                                   \
    }                                                                 \
    return OnMessage(message);                                        \
  }

#define IPCZ_MSG_END()

#define IPCZ_MSG_BEGIN_VERSION(version)
#define IPCZ_MSG_END_VERSION(version)

#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
