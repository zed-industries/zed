/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_DEMUXER_H_
#define CALL_RTP_DEMUXER_H_

#include <cstdint>
#include <map>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "rtc_base/containers/flat_map.h"
#include "rtc_base/containers/flat_set.h"

namespace webrtc {

class RtpPacketReceived;
class RtpPacketSinkInterface;

// This struct describes the criteria that will be used to match packets to a
// specific sink.
class RtpDemuxerCriteria {
 public:
  explicit RtpDemuxerCriteria(absl::string_view mid,
                              absl::string_view rsid = absl::string_view());
  RtpDemuxerCriteria();
  ~RtpDemuxerCriteria();

  bool operator==(const RtpDemuxerCriteria& other) const;
  bool operator!=(const RtpDemuxerCriteria& other) const;

  // If not the empty string, will match packets with this MID.
  const std::string& mid() const { return mid_; }

  // Return string representation of demux criteria to facilitate logging
  std::string ToString() const;

  // If not the empty string, will match packets with this as their RTP stream
  // ID or repaired RTP stream ID.
  // Note that if both MID and RSID are specified, this will only match packets
  // that have both specified (either through RTP header extensions, SSRC
  // latching or RTCP).
  const std::string& rsid() const { return rsid_; }

  // The criteria will match packets with any of these SSRCs.
  const flat_set<uint32_t>& ssrcs() const { return ssrcs_; }

  // Writable accessor for directly modifying the list of ssrcs.
  flat_set<uint32_t>& ssrcs() { return ssrcs_; }

  // The criteria will match packets with any of these payload types.
  const flat_set<uint8_t>& payload_types() const { return payload_types_; }

  // Writable accessor for directly modifying the list of payload types.
  flat_set<uint8_t>& payload_types() { return payload_types_; }

 private:
  // Intentionally private member variables to encourage specifying them via the
  // constructor and consider them to be const as much as possible.
  const std::string mid_;
  const std::string rsid_;
  flat_set<uint32_t> ssrcs_;
  flat_set<uint8_t> payload_types_;
};

// This class represents the RTP demuxing, for a single RTP session (i.e., one
// SSRC space, see RFC 7656). It isn't thread aware, leaving responsibility of
// multithreading issues to the user of this class.
// The demuxing algorithm follows the sketch given in the BUNDLE draft:
// https://tools.ietf.org/html/draft-ietf-mmusic-sdp-bundle-negotiation-38#section-10.2
// with modifications to support RTP stream IDs also.
//
// When a packet is received, the RtpDemuxer will route according to the
// following rules:
// 1. If the packet contains the MID header extension, and no sink has been
//    added with that MID as a criteria, the packet is not routed.
// 2. If the packet has the MID header extension, but no RSID or RRID extension,
//    and the MID is bound to a sink, then bind its SSRC to the same sink and
//    forward the packet to that sink. Note that rebinding to the same sink is
//    not an error. (Later packets with that SSRC would therefore be forwarded
//    to the same sink, whether they have the MID header extension or not.)
// 3. If the packet has the MID header extension and either the RSID or RRID
//    extension, and the MID, RSID (or RRID) pair is bound to a sink, then bind
//    its SSRC to the same sink and forward the packet to that sink. Later
//    packets with that SSRC will be forwarded to the same sink.
// 4. If the packet has the RSID or RRID header extension, but no MID extension,
//    and the RSID or RRID is bound to an RSID sink, then bind its SSRC to the
//    same sink and forward the packet to that sink. Later packets with that
//    SSRC will be forwarded to the same sink.
// 5. If the packet's SSRC is bound to an SSRC through a previous call to
//    AddSink, then forward the packet to that sink. Note that the RtpDemuxer
//    will not verify the payload type even if included in the sink's criteria.
//    The sink is expected to do the check in its handler.
// 6. If the packet's payload type is bound to exactly one payload type sink
//    through an earlier call to AddSink, then forward the packet to that sink.
// 7. Otherwise, the packet is not routed.
//
// In summary, the routing algorithm will always try to first match MID and RSID
// (including through SSRC binding), match SSRC directly as needed, and use
// payload types only if all else fails.
class RtpDemuxer {
 public:
  // Maximum number of unique SSRC bindings allowed. This limit is to prevent
  // memory overuse attacks due to a malicious peer sending many packets with
  // different SSRCs.
  static constexpr int kMaxSsrcBindings = 1000;

  // Returns a string that contains all the attributes of the given packet
  // relevant for demuxing.
  static std::string DescribePacket(const RtpPacketReceived& packet);

  explicit RtpDemuxer(bool use_mid = true);
  ~RtpDemuxer();

  RtpDemuxer(const RtpDemuxer&) = delete;
  void operator=(const RtpDemuxer&) = delete;

  // Registers a sink that will be notified when RTP packets match its given
  // criteria according to the algorithm described in the class description.
  // Returns true if the sink was successfully added.
  // Returns false in the following situations:
  // - Only MID is specified and the MID is already registered.
  // - Only RSID is specified and the RSID is already registered.
  // - Both MID and RSID is specified and the (MID, RSID) pair is already
  //   registered.
  // - Any of the criteria SSRCs are already registered.
  // If false is returned, no changes are made to the demuxer state.
  bool AddSink(const RtpDemuxerCriteria& criteria,
               RtpPacketSinkInterface* sink);

  // Registers a sink. Multiple SSRCs may be mapped to the same sink, but
  // each SSRC may only be mapped to one sink. The return value reports
  // whether the association has been recorded or rejected. Rejection may occur
  // if the SSRC has already been associated with a sink. The previously added
  // sink is *not* forgotten.
  bool AddSink(uint32_t ssrc, RtpPacketSinkInterface* sink);

  // Registers a sink's association to an RSID. Only one sink may be associated
  // with a given RSID. Null pointer is not allowed.
  void AddSink(absl::string_view rsid, RtpPacketSinkInterface* sink);

  // Removes a sink. Return value reports if anything was actually removed.
  // Null pointer is not allowed.
  bool RemoveSink(const RtpPacketSinkInterface* sink);

  // Returns the set of SSRCs associated with a sink.
  flat_set<uint32_t> GetSsrcsForSink(const RtpPacketSinkInterface* sink) const;

  // Demuxes the given packet and forwards it to the chosen sink. Returns true
  // if the packet was forwarded and false if the packet was dropped.
  bool OnRtpPacket(const RtpPacketReceived& packet);

 private:
  // Returns true if adding a sink with the given criteria would cause conflicts
  // with the existing criteria and should be rejected.
  bool CriteriaWouldConflict(const RtpDemuxerCriteria& criteria) const;

  // Runs the demux algorithm on the given packet and returns the sink that
  // should receive the packet.
  // Will record any SSRC<->ID associations along the way.
  // If the packet should be dropped, this method returns null.
  RtpPacketSinkInterface* ResolveSink(const RtpPacketReceived& packet);

  // Used by the ResolveSink algorithm.
  RtpPacketSinkInterface* ResolveSinkByMid(absl::string_view mid,
                                           uint32_t ssrc);
  RtpPacketSinkInterface* ResolveSinkByMidRsid(absl::string_view mid,
                                               absl::string_view rsid,
                                               uint32_t ssrc);
  RtpPacketSinkInterface* ResolveSinkByRsid(absl::string_view rsid,
                                            uint32_t ssrc);
  RtpPacketSinkInterface* ResolveSinkByPayloadType(uint8_t payload_type,
                                                   uint32_t ssrc);

  // Regenerate the known_mids_ set from information in the sink_by_mid_ and
  // sink_by_mid_and_rsid_ maps.
  void RefreshKnownMids();

  // Map each sink by its component attributes to facilitate quick lookups.
  // Payload Type mapping is a multimap because if two sinks register for the
  // same payload type, both AddSinks succeed but we must know not to demux on
  // that attribute since it is ambiguous.
  // Note: Mappings are only modified by AddSink/RemoveSink (except for
  // SSRC mapping which receives all MID, payload type, or RSID to SSRC bindings
  // discovered when demuxing packets).
  flat_map<std::string, RtpPacketSinkInterface*> sink_by_mid_;
  flat_map<uint32_t, RtpPacketSinkInterface*> sink_by_ssrc_;
  std::multimap<uint8_t, RtpPacketSinkInterface*> sinks_by_pt_;
  flat_map<std::pair<std::string, std::string>, RtpPacketSinkInterface*>
      sink_by_mid_and_rsid_;
  flat_map<std::string, RtpPacketSinkInterface*> sink_by_rsid_;

  // Tracks all the MIDs that have been identified in added criteria. Used to
  // determine if a packet should be dropped right away because the MID is
  // unknown.
  flat_set<std::string> known_mids_;

  // Records learned mappings of MID --> SSRC and RSID --> SSRC as packets are
  // received.
  // This is stored separately from the sink mappings because if a sink is
  // removed we want to still remember these associations.
  flat_map<uint32_t, std::string> mid_by_ssrc_;
  flat_map<uint32_t, std::string> rsid_by_ssrc_;

  // Adds a binding from the SSRC to the given sink.
  void AddSsrcSinkBinding(uint32_t ssrc, RtpPacketSinkInterface* sink);

  const bool use_mid_;
};

}  // namespace webrtc

#endif  // CALL_RTP_DEMUXER_H_
