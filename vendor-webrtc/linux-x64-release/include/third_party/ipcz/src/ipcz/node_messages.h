// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_MESSAGES_H_
#define IPCZ_SRC_IPCZ_NODE_MESSAGES_H_

#include <cstddef>
#include <cstdint>

#include "ipcz/driver_object.h"
#include "ipcz/driver_transport.h"
#include "ipcz/features.h"
#include "ipcz/handle_type.h"
#include "ipcz/link_side.h"
#include "ipcz/message.h"
#include "ipcz/node_name.h"
#include "ipcz/node_type.h"
#include "ipcz/router_descriptor.h"
#include "ipcz/sequence_number.h"
#include "ipcz/sublink_id.h"

namespace ipcz::msg {

// Bump this version number up by 1 when adding new protocol features so that
// they can be detected during NodeLink establishment.
constexpr uint32_t kProtocolVersion = 0;

#pragma pack(push, 1)

// clang-format off
#include "ipcz/message_macros/message_params_declaration_macros.h"
#include "ipcz/node_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_versions_declaration_macros.h"
#include "ipcz/node_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_base_declaration_macros.h"
#include "ipcz/node_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_declaration_macros.h"
#include "ipcz/node_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_listener_declaration_macros.h"
#include "ipcz/node_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"
// clang-format on

#pragma pack(pop)

}  // namespace ipcz::msg

#endif  // IPCZ_SRC_IPCZ_NODE_MESSAGES_H_
