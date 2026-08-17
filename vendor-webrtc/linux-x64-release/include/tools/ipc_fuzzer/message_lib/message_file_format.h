// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_FILE_FORMAT_H_
#define TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_FILE_FORMAT_H_

#include <stdint.h>

// Message file contains IPC messages and message names. Each message type
// has a NameTableEntry mapping the type to a name.
//
// |========================|
// | FileHeader             |
// |========================|
// | Message                |
// |------------------------|
// | Message                |
// |------------------------|
// | ...                    |
// |========================|
// | NameTableEntry         |
// |------------------------|
// | NameTableEntry         |
// |------------------------|
// | ...                    |
// |------------------------|
// | type = 0x0002070f      |
// | string_table_offset = ----+
// |------------------------|  |
// | ...                    |  |
// |========================|  |
// | message name           |  |
// |------------------------|  |
// | message name           |  |
// |------------------------|  |
// | ...                    |  |
// |------------------------|  |
// | "FrameHostMsg_Detach"  <--+
// |------------------------|
// | ...                    |
// |========================|

namespace ipc_fuzzer {

struct FileHeader {
  static const uint32_t kMagicValue = 0x1bcf11ee;
  static const uint32_t kCurrentVersion = 1;

  uint32_t magic;
  uint32_t version;
  uint32_t message_count;
  uint32_t name_count;
};

struct NameTableEntry {
  uint32_t type;
  uint32_t string_table_offset;
};

}  // namespace ipc_fuzzer

#endif  // TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_FILE_FORMAT_H_
