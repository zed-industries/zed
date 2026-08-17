// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_MACH_PORT_RENDEZVOUS_IOS_H_
#define BASE_APPLE_MACH_PORT_RENDEZVOUS_IOS_H_

#include <dispatch/dispatch.h>
#include <mach/mach.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "base/apple/dispatch_source.h"
#include "base/apple/mach_port_rendezvous.h"
#include "base/apple/scoped_mach_port.h"
#include "base/base_export.h"
#include "base/containers/buffer_iterator.h"

namespace base {

// An implementation class that works for a single process. It is intended
// that each process spawned will create a corresponding instance and the
// mach send right of this server will be sent using XPC to the process.
class BASE_EXPORT MachPortRendezvousServerIOS final
    : public MachPortRendezvousServerBase {
 public:
  MachPortRendezvousServerIOS(const MachPortsForRendezvous& ports);
  ~MachPortRendezvousServerIOS() override;
  MachPortRendezvousServerIOS(const MachPortRendezvousServerIOS&) = delete;
  MachPortRendezvousServerIOS& operator=(const MachPortRendezvousServerIOS&) =
      delete;

  // Retrieve the send right to be sent to the process.
  apple::ScopedMachSendRight GetMachSendRight();

 protected:
  std::optional<MachPortsForRendezvous> PortsForClient(audit_token_t) override;
  bool IsValidAdditionalMessageId(mach_msg_id_t) const override;
  std::vector<uint8_t> AdditionalDataForReply(mach_msg_id_t) const override;

 private:
  apple::ScopedMachSendRight send_right_;
  MachPortsForRendezvous ports_;
};

class BASE_EXPORT MachPortRendezvousClientIOS final
    : public MachPortRendezvousClient {
 public:
  // Initialize the MacPortRendezvousClient using `server_port`.
  static bool Initialize(apple::ScopedMachSendRight server_port);

 protected:
  bool ValidateMessage(mach_msg_base_t* message,
                       BufferIterator<uint8_t> body) override;

 private:
  MachPortRendezvousClientIOS();
  ~MachPortRendezvousClientIOS() override;

  // Helper method to look up the server in the bootstrap namespace and send
  // the acquisition request message.
  bool AcquirePorts(apple::ScopedMachSendRight server_port);
};

}  // namespace base

#endif  // BASE_APPLE_MACH_PORT_RENDEZVOUS_IOS_H_
