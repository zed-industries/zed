// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_MACH_PORT_RENDEZVOUS_H_
#define BASE_APPLE_MACH_PORT_RENDEZVOUS_H_

#include <dispatch/dispatch.h>
#include <mach/mach.h>
#include <stdint.h>
#include <sys/types.h>

#include <map>
#include <memory>
#include <vector>

#include "base/apple/dispatch_source.h"
#include "base/apple/scoped_mach_port.h"
#include "base/base_export.h"
#include "base/containers/buffer_iterator.h"
#include "base/feature_list.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "build/ios_buildflags.h"

namespace base {

// Mach Port Rendezvous is a technique to exchange Mach port rights across
// child process creation. macOS does not provide a way to inherit Mach port
// rights, unlike what is possible with file descriptors. Port rendezvous
// enables a parent process to register Mach port rights for a nascent child,
// which the child can then retrieve using Mach IPC by looking up the endpoint
// in launchd's bootstrap namespace.
//
// The same mechanism is used on iOS but the Mach IPC endpoint is not found
// via launchd's bootstrap namespace but via an initial XPC connection.
//
// When launching a child process, the parent process' rendezvous server lets
// calling code register a collection of ports for the new child. In order to
// acquire the ports, a child looks up the rendezvous server in the bootstrap
// namespace, and it sends an IPC message to the server, the reply to which
// contains the registered ports.
//
// Port rendezvous is only permitted between a parent and its direct child
// process descendants.

// A MachRendezvousPort contains a single Mach port to pass to the child
// process. The associated disposition controls how the reference count will
// be manipulated.
class BASE_EXPORT MachRendezvousPort {
 public:
  MachRendezvousPort() = default;
  // Creates a rendezvous port that allows specifying the specific disposition.
  MachRendezvousPort(mach_port_t name, mach_msg_type_name_t disposition);
  // Creates a rendezvous port for MACH_MSG_TYPE_MOVE_SEND.
  explicit MachRendezvousPort(apple::ScopedMachSendRight send_right);
  // Creates a rendezvous port for MACH_MSG_TYPE_MOVE_RECEIVE.
  explicit MachRendezvousPort(apple::ScopedMachReceiveRight receive_right);

  // Note that the destructor does not call Destroy() explicitly.
  // To avoid leaking ports, either use dispositions that create rights during
  // transit (MAKE or COPY), or use base::LaunchProcess, which will destroy
  // rights on failure.
  ~MachRendezvousPort();

  // Destroys the Mach port right type conveyed |disposition| named by |name|.
  void Destroy();

  mach_port_t name() const { return name_; }

  mach_msg_type_name_t disposition() const { return disposition_; }

 private:
  mach_port_t name_ = MACH_PORT_NULL;
  mach_msg_type_name_t disposition_ = 0;
  // Copy and assign allowed.
};

// The collection of ports to pass to a child process. There are no restrictions
// regarding the keys of the map. Clients are responsible for avoiding
// collisions with other clients.
using MachPortsForRendezvous = std::map<uint32_t, MachRendezvousPort>;

// Base class that runs a Mach message server, listening to requests on
// mach server port.
class BASE_EXPORT MachPortRendezvousServerBase {
 protected:
  MachPortRendezvousServerBase();
  virtual ~MachPortRendezvousServerBase();

  // The Mach receive right for the server. A send right to this is port is
  // registered in the bootstrap server.
  apple::ScopedMachReceiveRight server_port_;

  // Mach message dispatch source for |server_port_|.
  std::unique_ptr<apple::DispatchSource> dispatch_source_;

  // Ask for the associated ports associated with `audit_token`.
  // Return `std::nullopt` if the client is not authorized to
  // retrieve ports.
  virtual std::optional<MachPortsForRendezvous> PortsForClient(
      audit_token_t audit_token) = 0;

  // Return whether `msg_id` should be accepted along with the known
  // message IDs. Platform-specific subclasses may return additional data
  // based on the `msg_id` within `AdditionalDataForReply`.
  virtual bool IsValidAdditionalMessageId(mach_msg_id_t msg_id) const = 0;

  // Return additional data to be attached to a reply for `request`.
  virtual std::vector<uint8_t> AdditionalDataForReply(
      mach_msg_id_t request) const = 0;

  // The server-side Mach message handler. Called by |dispatch_source_| when a
  // message is received.
  void HandleRequest();

  // Returns a buffer containing a well-formed Mach message, destined for
  // `reply_port` containing descriptors for the specified `ports` and
  // `additional_data`.
  std::unique_ptr<uint8_t[]> CreateReplyMessage(
      mach_port_t reply_port,
      const MachPortsForRendezvous& ports,
      std::vector<uint8_t> additional_data);
};

// Client class for accessing the memory object exposed by the
// MachPortRendezvousServer.
class BASE_EXPORT MachPortRendezvousClient {
 public:
  MachPortRendezvousClient(const MachPortRendezvousClient&) = delete;
  MachPortRendezvousClient& operator=(const MachPortRendezvousClient&) = delete;

  // Connects to the MachPortRendezvousServer and requests any registered Mach
  // ports. This only performs the rendezvous once. Subsequent calls to this
  // method return the same instance. If the rendezvous fails, which can happen
  // if the server is not available or if the server fails the code signature
  // validation and requirement check, this returns null. Acquiring zero ports
  // from the exchange is not considered a failure.
  static MachPortRendezvousClient* GetInstance();

  // Returns the Mach send right that was registered with |key|. If no such
  // right exists, or it was already taken, returns an invalid right. Safe to
  // call from any thread. DCHECKs if the right referenced by |key| is not a
  // send or send-once right.
  apple::ScopedMachSendRight TakeSendRight(
      MachPortsForRendezvous::key_type key);

  // Returns the Mach receive right that was registered with |key|. If no such
  // right exists, or it was already taken, returns an invalid right. Safe to
  // call from any thread. DCHECKs if the right referenced by |key| is not a
  // receive right.
  apple::ScopedMachReceiveRight TakeReceiveRight(
      MachPortsForRendezvous::key_type key);

  // Returns the number of ports in the client. After PerformRendezvous(), this
  // reflects the number of ports acquired. But as rights are taken, this
  // only reflects the number of remaining rights.
  size_t GetPortCount();

 protected:
  MachPortRendezvousClient();
  virtual ~MachPortRendezvousClient();

  // Perform platform-specific validation on a received message and the peer
  // that sent it.
  virtual bool ValidateMessage(mach_msg_base_t* message,
                               BufferIterator<uint8_t> body) = 0;

  // Sends the actual IPC message to |server_port| and parses the reply.
  bool SendRequest(apple::ScopedMachSendRight server_port,
                   mach_msg_id_t request_id,
                   size_t additional_response_data_size = 0)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns a MachRendezvousPort for a given key and removes it from the
  // |ports_| map. If an entry does not exist for that key, then a
  // MachRendezvousPort with MACH_PORT_NULL is returned.
  MachRendezvousPort PortForKey(MachPortsForRendezvous::key_type key);

  Lock lock_;
  // The collection of ports that was acquired.
  MachPortsForRendezvous ports_ GUARDED_BY(lock_);
};

namespace internal {

// This limit is arbitrary and can be safely increased in the future.
inline constexpr size_t kMaximumRendezvousPorts = 6;

enum MachRendezvousMsgId : mach_msg_id_t {
  kMachRendezvousMsgIdRequest = 'mrzv',
  kMachRendezvousMsgIdResponse = 'MRZV',

#if BUILDFLAG(IS_MAC)
  // When MachPortRendezvousClientMac has a `ProcessRequirement` that requests
  // dynamic-only validation, it will request that the server provide a copy of
  // its Info.plist data in the rendezvous response. Dynamic-only validation
  // validates the running process without enforcing that it matches its on-disk
  // representation. This is necessary when validating applications such as
  // Chrome that may be updated on disk while the application is running.
  //
  // The Info.plist data ends up passed to `SecCodeCopyGuestWithAttributes`,
  // where it is validated against the hash stored within the code signature
  // before using it to evaluate any requirements involving Info.plist data.
  kMachRendezvousMsgIdRequestWithInfoPlistData = 'mrzV',
#endif  // BUILDFLAG(IS_MAC)
};

}  // namespace internal

}  // namespace base

#endif  // BASE_APPLE_MACH_PORT_RENDEZVOUS_H_
