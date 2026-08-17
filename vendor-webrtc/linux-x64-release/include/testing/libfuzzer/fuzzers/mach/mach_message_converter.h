// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_FUZZERS_MACH_MACH_MESSAGE_CONVERTER_H_
#define TESTING_LIBFUZZER_FUZZERS_MACH_MACH_MESSAGE_CONVERTER_H_

#include <mach/mach.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "base/apple/scoped_mach_port.h"
#include "base/memory/raw_ptr.h"
#include "testing/libfuzzer/fuzzers/mach/mach_message.pb.h"

namespace mach_fuzzer {

// Container for a Mach port right that will be sent in a message.
struct SendablePort {
  mach_port_t name = MACH_PORT_NULL;
  mach_msg_type_name_t disposition = 0;
  MachPortType proto_type = static_cast<MachPortType>(-1);

  base::apple::ScopedMachSendRight send_right;
  base::apple::ScopedMachReceiveRight receive_right;
};

// Holds the buffer allocation and port references for a message to be sent.
struct SendableMessage {
  // The message buffer.
  std::unique_ptr<uint8_t[]> buffer;

  // The |ports| are also encoded into the body of the message, but they are
  // accessible here to allow for further manipulation.
  std::vector<SendablePort> ports;

  // Pointer to the header of the message stored in |buffer|.
  raw_ptr<mach_msg_header_t> header = nullptr;
};

// Converts the given protobuf message into a live Mach message, including port
// rights.
SendableMessage ConvertProtoToMachMessage(const MachMessage& proto);

// Takes the protobuf |proto|, converts it to a Mach message using
// ConvertProtoToMachMessage(), and then sends it via |local_port|. The port
// named by |local_port| must have a send right, which will be copied.
struct SendResult {
  // The return value from mach_msg_send().
  kern_return_t kr;

  // The message that was sent, including its descriptors. This allows callers
  // to control the lifetimes of any Mach rights after the message has been
  // sent.
  SendableMessage message;
};
SendResult SendMessage(mach_port_t local_port, const MachMessage& proto);

}  // namespace mach_fuzzer

#endif  // TESTING_LIBFUZZER_FUZZERS_MACH_MACH_MESSAGE_CONVERTER_H_
