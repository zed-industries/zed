/*
 *  Copyright 2021 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_JSEP_TRANSPORT_COLLECTION_H_
#define PC_JSEP_TRANSPORT_COLLECTION_H_

#include <functional>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/jsep.h"
#include "api/peer_connection_interface.h"
#include "api/sequence_checker.h"
#include "pc/jsep_transport.h"
#include "pc/session_description.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// This class manages information about RFC 8843 BUNDLE bundles
// in SDP descriptions.

// This is a work-in-progress. Planned steps:
// 1) Move all Bundle-related data structures from JsepTransport
//    into this class.
// 2) Move all Bundle-related functions into this class.
// 3) Move remaining Bundle-related logic into this class.
//    Make data members private.
// 4) Refine interface to have comprehensible semantics.
// 5) Add unit tests.
// 6) Change the logic to do what's right.
class BundleManager {
 public:
  explicit BundleManager(PeerConnectionInterface::BundlePolicy bundle_policy)
      : bundle_policy_(bundle_policy) {}
  const std::vector<std::unique_ptr<ContentGroup>>& bundle_groups() const {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return bundle_groups_;
  }
  // Lookup a bundle group by a member mid name.
  const ContentGroup* LookupGroupByMid(const std::string& mid) const;
  ContentGroup* LookupGroupByMid(const std::string& mid);
  // Returns true if the MID is the first item of a group, or if
  // the MID is not a member of a group.
  bool IsFirstMidInGroup(const std::string& mid) const;
  // Update the groups description. This completely replaces the group
  // description with the one from the SessionDescription.
  void Update(const SessionDescription* description, SdpType type);
  // Delete a MID from the group that contains it.
  void DeleteMid(const ContentGroup* bundle_group, const std::string& mid);
  // Delete a group.
  void DeleteGroup(const ContentGroup* bundle_group);
  // Roll back to previous stable state.
  void Rollback();
  // Commit current bundle groups.
  void Commit();

 private:
  // Recalculate established_bundle_groups_by_mid_ from bundle_groups_.
  void RefreshEstablishedBundleGroupsByMid() RTC_RUN_ON(sequence_checker_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_{
      SequenceChecker::kDetached};
  PeerConnectionInterface::BundlePolicy bundle_policy_;
  std::vector<std::unique_ptr<ContentGroup>> bundle_groups_
      RTC_GUARDED_BY(sequence_checker_);
  std::vector<std::unique_ptr<ContentGroup>> stable_bundle_groups_
      RTC_GUARDED_BY(sequence_checker_);
  std::map<std::string, ContentGroup*> established_bundle_groups_by_mid_;
};

// This class keeps the mapping of MIDs to transports.
// It is pulled out here because a lot of the code that deals with
// bundles end up modifying this map, and the two need to be consistent;
// the managers may merge.
class JsepTransportCollection {
 public:
  JsepTransportCollection(
      std::function<bool(const std::string& mid,
                         webrtc::JsepTransport* transport)> map_change_callback,
      std::function<void()> state_change_callback)
      : map_change_callback_(map_change_callback),
        state_change_callback_(state_change_callback) {}

  void RegisterTransport(const std::string& mid,
                         std::unique_ptr<JsepTransport> transport);
  // Returns all transports, including those not currently mapped to any MID
  // because they're being kept alive in case of rollback.
  std::vector<JsepTransport*> Transports();
  // Only returns transports currently mapped to a MID.
  std::vector<JsepTransport*> ActiveTransports();
  void DestroyAllTransports();
  // Lookup a JsepTransport by the MID that was used to register it.
  JsepTransport* GetTransportByName(const std::string& mid);
  const JsepTransport* GetTransportByName(const std::string& mid) const;
  // Lookup a JsepTransport by any MID that refers to it.
  JsepTransport* GetTransportForMid(const std::string& mid);
  const JsepTransport* GetTransportForMid(const std::string& mid) const;
  JsepTransport* GetTransportForMid(absl::string_view mid);
  const JsepTransport* GetTransportForMid(absl::string_view mid) const;
  // Set transport for a MID. This may destroy a transport if it is no
  // longer in use.
  bool SetTransportForMid(const std::string& mid,
                          JsepTransport* jsep_transport);
  // Remove a transport for a MID. This may destroy a transport if it is
  // no longer in use.
  void RemoveTransportForMid(const std::string& mid);
  // Roll back to previous stable mid-to-transport mappings.
  bool RollbackTransports();
  // Commit pending mid-transport mappings (rollback is no longer possible),
  // and destroy unused transports because we know now we'll never need them
  // again.
  void CommitTransports();

 private:
  // Returns true if any mid currently maps to this transport.
  bool TransportInUse(JsepTransport* jsep_transport) const;

  // Returns true if any mid in the last stable mapping maps to this transport,
  // meaning it should be kept alive in case of rollback.
  bool TransportNeededForRollback(JsepTransport* jsep_transport) const;

  // Destroy a transport if it's no longer in use. This includes whether it
  // will be needed in case of rollback.
  void MaybeDestroyJsepTransport(JsepTransport* transport);

  // Destroys all transports that are no longer in use.
  void DestroyUnusedTransports();

  bool IsConsistent();  // For testing only: Verify internal structure.

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_{
      SequenceChecker::kDetached};
  // This member owns the JSEP transports.
  std::map<std::string, std::unique_ptr<JsepTransport>> jsep_transports_by_name_
      RTC_GUARDED_BY(sequence_checker_);

  // This keeps track of the mapping between media section
  // (BaseChannel/SctpTransport) and the JsepTransport underneath.
  std::map<std::string, JsepTransport*> mid_to_transport_
      RTC_GUARDED_BY(sequence_checker_);
  // A snapshot of mid_to_transport_ at the last stable state. Used for
  // rollback.
  std::map<std::string, JsepTransport*> stable_mid_to_transport_
      RTC_GUARDED_BY(sequence_checker_);
  // Callback used to inform subscribers of altered transports.
  const std::function<bool(const std::string& mid,
                           webrtc::JsepTransport* transport)>
      map_change_callback_;
  // Callback used to inform subscribers of possibly altered state.
  const std::function<void()> state_change_callback_;
};

}  // namespace webrtc

#endif  // PC_JSEP_TRANSPORT_COLLECTION_H_
