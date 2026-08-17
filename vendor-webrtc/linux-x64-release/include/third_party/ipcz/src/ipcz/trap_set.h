// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_TRAP_SET_H_
#define IPCZ_SRC_IPCZ_TRAP_SET_H_

#include <cstdint>

#include "ipcz/ipcz.h"
#include "ipcz/parcel_queue.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

namespace ipcz {

class TrapEventDispatcher;

// A set of traps installed on a portal.
class TrapSet {
 public:
  TrapSet();
  TrapSet(const TrapSet&) = delete;
  TrapSet& operator=(const TrapSet&) = delete;

  // NOTE: A TrapSet must be empty before it can be destroyed.
  ~TrapSet();

  bool empty() const { return traps_.empty(); }

  // Attempts to install a new trap in the set. This effectively implements
  // the ipcz Trap() API. `status_flags`, `num_local_parcels`, and
  // `num_local_bytes` convey the current status of the portal. If `conditions`
  // are already met, returns IPCZ_RESULT_FAILED_PRECONDITION and populates
  // `satisfied_condition_flags` and/or `status` if non-null.
  IpczResult Add(const IpczTrapConditions& conditions,
                 IpczTrapEventHandler handler,
                 uintptr_t context,
                 IpczPortalStatusFlags status_flags,
                 ParcelQueue& inbound_parcel_queue,
                 IpczTrapConditionFlags* satisfied_condition_flags,
                 IpczPortalStatus* status);

  // Notifies the TrapSet that a new local parcel has arrived on its portal.
  // Any trap interested in this is removed from the set, and its event handler
  // invocation is appended to `dispatcher`. `status_flags`, `num_local_parcels`
  // and `num_local_bytes` convey the new status of the portal.
  void NotifyNewLocalParcel(IpczPortalStatusFlags status_flags,
                            ParcelQueue& inbound_parcel_queue,
                            TrapEventDispatcher& dispatcher);

  // Notifies the TrapSet that a local parcel has been consumed from its portal.
  // Any trap interested in this is removed from the set, and its event handler
  // invocation is appended to `dispatcher`. `status_flags`, `num_local_parcels`
  // and `num_local_bytes` convey the new status of the portal.
  void NotifyLocalParcelConsumed(IpczPortalStatusFlags status_flags,
                                 ParcelQueue& inbound_parcel_queue,
                                 TrapEventDispatcher& dispatcher);

  // Notifies the TrapSet that its portal's peer has been closed. Any trap
  // interested in this is removed from the set, and its event handler
  // invocation is appended to `dispatcher`. `status_flags` conveys the new
  // status of the portal.
  void NotifyPeerClosed(IpczPortalStatusFlags status_flags,
                        ParcelQueue& inbound_parcel_queue,
                        TrapEventDispatcher& dispatcher);

  // Immediately removes all traps from the set. Every trap present appends an
  // IPCZ_TRAP_REMOVED event to `dispatcher` before removal.
  void RemoveAll(TrapEventDispatcher& dispatcher);

 private:
  struct Trap {
    Trap(IpczTrapConditions conditions,
         IpczTrapEventHandler handler,
         uintptr_t context);
    ~Trap();

    IpczTrapConditions conditions;
    IpczTrapEventHandler handler;
    uintptr_t context;
  };

  // The reason for each status update when something happens that might
  // interest a trap.
  enum class UpdateReason {
    // A new trap is being installed and this is an initial state query.
    kInstallTrap,

    // A new inbound parcel has arrived for retrieval.
    kNewLocalParcel,

    // We just discovered that the remote portal is gone.
    kPeerClosed,

    // A previously queued inbound parcel has been fully or partially retrieved
    // by the application.
    kLocalParcelConsumed,
  };

  // Determines which trap condition flags would be set if an event fired for
  // a trap watching the given `conditions`, given the most recent state change.
  IpczTrapConditionFlags GetSatisfiedConditionsForUpdate(
      const IpczTrapConditions& conditions,
      IpczPortalStatusFlags status_flags,
      ParcelQueue& inbound_parcel_queue,
      UpdateReason reason);

  // Helper used by Notify* methods to carry out common update work.
  void UpdatePortalStatus(IpczPortalStatusFlags status_flags,
                          ParcelQueue& inbound_parcel_queue,
                          UpdateReason reason,
                          TrapEventDispatcher& dispatcher);

  using TrapList = std::vector<Trap>;
  TrapList traps_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_TRAP_SET_H_
