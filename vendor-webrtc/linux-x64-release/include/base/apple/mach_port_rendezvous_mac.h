// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_MACH_PORT_RENDEZVOUS_MAC_H_
#define BASE_APPLE_MACH_PORT_RENDEZVOUS_MAC_H_

#include <dispatch/dispatch.h>
#include <mach/mach.h>
#include <stdint.h>
#include <sys/types.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "base/apple/dispatch_source.h"
#include "base/apple/mach_port_rendezvous.h"
#include "base/apple/scoped_mach_port.h"
#include "base/base_export.h"
#include "base/containers/buffer_iterator.h"
#include "base/environment.h"
#include "base/feature_list.h"
#include "base/mac/process_requirement.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"

namespace base {

// An implementation class that uses bootstrap to register ports to many
// processes.
class BASE_EXPORT MachPortRendezvousServerMac final
    : public MachPortRendezvousServerBase {
 public:
  // Returns the instance of the server. Upon the first call to this method,
  // the server is created, which registers an endpoint in the Mach bootstrap
  // namespace.
  static MachPortRendezvousServerMac* GetInstance();

  // Add feature state to an environment variable that will be used when
  // launching a child process. MachPortRendezvousClient is used during
  // feature list initialization so any state it uses must be passed
  // via a side channel.
  // TODO(crbug.com/362302761): Remove once enforcement is enabled by default.
  static void AddFeatureStateToEnvironment(EnvironmentMap& environment);

  MachPortRendezvousServerMac(const MachPortRendezvousServerMac&) = delete;
  MachPortRendezvousServerMac& operator=(const MachPortRendezvousServerMac&) =
      delete;

  // Registers a collection of Mach ports |ports| to be acquirable by the
  // process known by |pid|. This cannot be called again for the same |pid|
  // until the process known by |pid| has either acquired the ports or died.
  //
  // This must be called with the lock from GetLock() held.
  void RegisterPortsForPid(pid_t pid, const MachPortsForRendezvous& ports)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());

  // Sets the process requirement that `pid` must match before it
  // can acquire any Mach ports. This cannot be called again for the same `pid`
  // until the process known by `pid` has acquired the ports or died.
  //
  // This must be called with the lock from GetLock() held.
  void SetProcessRequirementForPid(pid_t pid,
                                   mac::ProcessRequirement requirement)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());

  // Returns a lock on the internal port registration map. The parent process
  // should hold this lock for the duration of launching a process, including
  // after calling RegisterPortsForPid(). This ensures that a child process
  // cannot race acquiring ports before they are registered. The lock should
  // be released after the child process is launched and the ports are
  // registered.
  Lock& GetLock() LOCK_RETURNED(lock_) { return lock_; }

  void ClearClientDataForTesting() EXCLUSIVE_LOCKS_REQUIRED(GetLock());

 protected:
  // Returns the registered collection of ports for the specified `audit_token`.
  // `std::nullopt` indicates that the client is not authorized to retrieve the
  // ports. This claims the collection of ports and removes the entry from
  // `client_data_`.
  std::optional<MachPortsForRendezvous> PortsForClient(
      audit_token_t audit_token) override;

  bool IsValidAdditionalMessageId(mach_msg_id_t) const override;
  std::vector<uint8_t> AdditionalDataForReply(
      mach_msg_id_t request) const override;

 private:
  friend class MachPortRendezvousServerTest;
  friend struct MachPortRendezvousFuzzer;

  MachPortRendezvousServerMac();
  ~MachPortRendezvousServerMac() override;

  struct ClientData;

  // Returns the `ClientData` for `pid`, creating it if necessary.
  // It will be cleaned up automatically when `pid` exits.
  ClientData& ClientDataForPid(int pid) EXCLUSIVE_LOCKS_REQUIRED(GetLock());

  // Called by the ClientData::exit_watcher dispatch sources when a process
  // for which ports have been registered exits. This releases port rights
  // that are strongly owned, in the event that the child has not claimed them.
  void OnClientExited(pid_t pid);

  Lock lock_;
  // Association of pid-to-ports.
  std::map<pid_t, ClientData> client_data_ GUARDED_BY(lock_);
};

// Describes how the `ProcessRequirement` should be used during Mach port
// rendezvous. The active policy is derived from the feature flags in the
// browser process and is passed via an environment variable to child processes.
// TODO(crbug.com/362302761): Remove this policy once enforcement is enabled by
// default.
enum class MachPortRendezvousPeerValidationPolicy {
  // Do not validate the peer against a process requirement.
  kNoValidation,

  // Validate the peer against a process requirement, if specified, but do not
  // abort rendezvous if validation fails. Used to gather success metrics during
  // experiment rollout.
  kValidateOnly,

  // Validate the peer against a process requirement, if specified, and abort
  // rendezvous if the validation fails.
  kEnforce,
};

class BASE_EXPORT MachPortRendezvousClientMac final
    : public MachPortRendezvousClient {
 public:
  // Set a ProcessRequirement that the server should be validated
  // against before accepting any Mach ports from it.
  //
  // Must be called before `GetInstance` or this will have no effect.
  static void SetServerProcessRequirement(mac::ProcessRequirement requirement);

  // Get the peer validation policy that was derived from feature flags.
  static MachPortRendezvousPeerValidationPolicy
  PeerValidationPolicyForTesting();

 protected:
  // Validate the server against a process requirement if one was set via
  // `SetServerProcessRequirement`.
  bool ValidateMessage(mach_msg_base_t* message,
                       BufferIterator<uint8_t> body) override;

 private:
  friend class MachPortRendezvousClient;

  MachPortRendezvousClientMac();
  ~MachPortRendezvousClientMac() override;

  // Returns the name of the server to look up in the bootstrap namespace.
  static std::string GetBootstrapName();

  // Helper method to look up the server in the bootstrap namespace and send
  // the acquisition request message.
  bool AcquirePorts();

  // Take ownership of the server process requirement, if any.
  static std::optional<mac::ProcessRequirement>
  TakeServerCodeSigningRequirement();

  // Whether Info.plist data is needed from the server in order
  // to validate `server_requirement_`.
  bool NeedsInfoPlistData() const;

  std::optional<mac::ProcessRequirement> server_requirement_;
};

// Whether any peer process requirements should be validated.
BASE_EXPORT BASE_DECLARE_FEATURE(kMachPortRendezvousValidatePeerRequirements);

// Whether a failure to validate a peer process against a requirement
// should result in aborting the rendezvous.
BASE_EXPORT BASE_DECLARE_FEATURE(kMachPortRendezvousEnforcePeerRequirements);

}  // namespace base

#endif  // BASE_APPLE_MACH_PORT_RENDEZVOUS_MAC_H_
