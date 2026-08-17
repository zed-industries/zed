// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Generates the message dispatch implementation for an interface. Essentially a
// big switch which routes to an appropriate virtual method. Messages dispatched
// here have already been validated. See message_listener_definition_macros.h.

#define IPCZ_MSG_BEGIN_INTERFACE(name)                            \
  bool name##MessageListener::DispatchMessage(Message& message) { \
    switch (message.header().message_id) {
#define IPCZ_MSG_END_INTERFACE() \
  default:                       \
    return true;                 \
    }                            \
    }

#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl) \
  case msg::name::kId:                \
    return On##name(static_cast<name&>(message));

#define IPCZ_MSG_END()
#define IPCZ_MSG_BEGIN_VERSION(version)
#define IPCZ_MSG_END_VERSION(version)
#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
