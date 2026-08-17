// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_TEST_MESSAGES_H_
#define IPCZ_SRC_IPCZ_TEST_MESSAGES_H_

#include <cstddef>
#include <cstdint>

#include "ipcz/driver_object.h"
#include "ipcz/driver_transport.h"
#include "ipcz/message.h"

namespace ipcz::test::msg {

#pragma pack(push, 1)

// clang-format off
#include "ipcz/message_macros/message_params_declaration_macros.h"
#include "ipcz/test_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_versions_declaration_macros.h"
#include "ipcz/test_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_base_declaration_macros.h"
#include "ipcz/test_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_declaration_macros.h"
#include "ipcz/test_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"

#include "ipcz/message_macros/message_listener_declaration_macros.h"
#include "ipcz/test_messages_generator.h"
#include "ipcz/message_macros/undef_message_macros.h"
// clang-format on

#pragma pack(pop)

}  // namespace ipcz::test::msg

#endif  // IPCZ_SRC_IPCZ_TEST_MESSAGES_H_
