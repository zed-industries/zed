// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_CRACKER_H_
#define TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_CRACKER_H_

#include <stdint.h>
#include <string.h>
#include "ipc/ipc_message.h"

// Means for updating protected message fields.
class MessageCracker : public IPC::Message {
 public:
  MessageCracker(const MessageCracker&) = delete;
  MessageCracker& operator=(const MessageCracker&) = delete;

  static void CopyMessageID(IPC::Message* dst, IPC::Message* src) {
    memcpy(ToCracker(dst)->mutable_payload(),
           ToCracker(src)->payload(),
           sizeof(int));
  }

  static void SetMessageType(IPC::Message* message, uint32_t type) {
    ToCracker(message)->header()->type = type;
  }

 private:
  static MessageCracker* ToCracker(IPC::Message* message) {
    return reinterpret_cast<MessageCracker*>(message);
  }
};

#endif  // TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_CRACKER_H_
