/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains structures for describing SSRCs from a media source such
// as a MediaStreamTrack when it is sent across an RTP session. Multiple media
// sources may be sent across the same RTP session, each of them will be
// described by one StreamParams object
// SsrcGroup is used to describe the relationship between the SSRCs that
// are used for this media source.
// E.x: Consider a source that is sent as 3 simulcast streams
// Let the simulcast elements have SSRC 10, 20, 30.
// Let each simulcast element use FEC and let the protection packets have
// SSRC 11,21,31.
// To describe this 4 SsrcGroups are needed,
// StreamParams would then contain ssrc = {10,11,20,21,30,31} and
// ssrc_groups = {{SIM,{10,20,30}, {FEC,{10,11}, {FEC, {20,21}, {FEC {30,31}}}
// Please see RFC 5576.
// A spec-compliant way to achieve this is to use RIDs and Simulcast attribute
// instead of the ssrc-group. In this method, the StreamParam object will
// have multiple RidDescriptions, each corresponding to a simulcast layer
// and the media section will have a simulcast attribute that indicates
// that these layers are for the same source. This also removes the extra
// lines for redundancy streams, as the same RIDs appear in the redundancy
// packets.
// Note: in the spec compliant simulcast scenario, some of the RIDs might be
// alternatives for one another (such as different encodings for same data).
// In the context of the StreamParams class, the notion of alternatives does
// not exist and all the RIDs will describe different layers of the same source.
// When the StreamParams class is used to configure the media engine, simulcast
// considerations will be used to remove the alternative layers outside of this
// class.
// As an example, let the simulcast layers have RID 10, 20, 30.
// StreamParams would contain rid = { 10, 20, 30 }.
// MediaSection would contain SimulcastDescription specifying these rids.
// a=simulcast:send 10;20;30 (or a=simulcast:send 10,20;30 or similar).
// See https://tools.ietf.org/html/draft-ietf-mmusic-sdp-simulcast-13
// and https://tools.ietf.org/html/draft-ietf-mmusic-rid-15.

#ifndef MEDIA_BASE_STREAM_PARAMS_H_
#define MEDIA_BASE_STREAM_PARAMS_H_

#include <stddef.h>

#include <cstdint>
#include <string>
#include <vector>

#include "absl/algorithm/container.h"
#include "media/base/rid_description.h"
#include "rtc_base/unique_id_generator.h"

namespace webrtc {

extern const char kFecSsrcGroupSemantics[];
extern const char kFecFrSsrcGroupSemantics[];
extern const char kFidSsrcGroupSemantics[];
extern const char kSimSsrcGroupSemantics[];

struct SsrcGroup {
  SsrcGroup(const std::string& usage, const std::vector<uint32_t>& ssrcs);
  SsrcGroup(const SsrcGroup&);
  SsrcGroup(SsrcGroup&&);
  ~SsrcGroup();
  SsrcGroup& operator=(const SsrcGroup&);
  SsrcGroup& operator=(SsrcGroup&&);

  bool operator==(const SsrcGroup& other) const {
    return (semantics == other.semantics && ssrcs == other.ssrcs);
  }
  bool operator!=(const SsrcGroup& other) const { return !(*this == other); }

  bool has_semantics(const std::string& semantics) const;

  std::string ToString() const;

  std::string semantics;        // e.g FID, FEC-FR, SIM.
  std::vector<uint32_t> ssrcs;  // SSRCs of this type.
};

// StreamParams is used to represent a sender/track in a SessionDescription.
// In Plan B, this means that multiple StreamParams can exist within one
// MediaContentDescription, while in UnifiedPlan this means that there is one
// StreamParams per MediaContentDescription.
struct StreamParams {
  StreamParams();
  StreamParams(const StreamParams&);
  StreamParams(StreamParams&&);
  ~StreamParams();
  StreamParams& operator=(const StreamParams&);
  StreamParams& operator=(StreamParams&&);

  static StreamParams CreateLegacy(uint32_t ssrc) {
    StreamParams stream;
    stream.ssrcs.push_back(ssrc);
    return stream;
  }

  bool operator==(const StreamParams& other) const;
  bool operator!=(const StreamParams& other) const { return !(*this == other); }

  uint32_t first_ssrc() const {
    if (ssrcs.empty()) {
      return 0;
    }

    return ssrcs[0];
  }
  bool has_ssrcs() const { return !ssrcs.empty(); }
  bool has_ssrc(uint32_t ssrc) const {
    return absl::c_linear_search(ssrcs, ssrc);
  }
  void add_ssrc(uint32_t ssrc) { ssrcs.push_back(ssrc); }
  bool has_ssrc_groups() const { return !ssrc_groups.empty(); }
  bool has_ssrc_group(const std::string& semantics) const {
    return (get_ssrc_group(semantics) != NULL);
  }
  const SsrcGroup* get_ssrc_group(const std::string& semantics) const {
    for (const SsrcGroup& ssrc_group : ssrc_groups) {
      if (ssrc_group.has_semantics(semantics)) {
        return &ssrc_group;
      }
    }
    return NULL;
  }

  // Convenience function to add an FID ssrc for a primary_ssrc
  // that's already been added.
  bool AddFidSsrc(uint32_t primary_ssrc, uint32_t fid_ssrc) {
    return AddSecondarySsrc(kFidSsrcGroupSemantics, primary_ssrc, fid_ssrc);
  }

  // Convenience function to lookup the FID ssrc for a primary_ssrc.
  // Returns false if primary_ssrc not found or FID not defined for it.
  bool GetFidSsrc(uint32_t primary_ssrc, uint32_t* fid_ssrc) const {
    return GetSecondarySsrc(kFidSsrcGroupSemantics, primary_ssrc, fid_ssrc);
  }

  // Convenience function to add an FEC-FR ssrc for a primary_ssrc
  // that's already been added.
  bool AddFecFrSsrc(uint32_t primary_ssrc, uint32_t fecfr_ssrc) {
    return AddSecondarySsrc(kFecFrSsrcGroupSemantics, primary_ssrc, fecfr_ssrc);
  }

  // Convenience function to lookup the FEC-FR ssrc for a primary_ssrc.
  // Returns false if primary_ssrc not found or FEC-FR not defined for it.
  bool GetFecFrSsrc(uint32_t primary_ssrc, uint32_t* fecfr_ssrc) const {
    return GetSecondarySsrc(kFecFrSsrcGroupSemantics, primary_ssrc, fecfr_ssrc);
  }

  // Convenience function to populate the StreamParams with the requested number
  // of SSRCs along with accompanying FID and FEC-FR ssrcs if requested.
  // SSRCs are generated using the given generator.
  void GenerateSsrcs(int num_layers,
                     bool generate_fid,
                     bool generate_fec_fr,
                     UniqueRandomIdGenerator* ssrc_generator);

  // Convenience to get all the SIM SSRCs if there are SIM ssrcs, or
  // the first SSRC otherwise.
  void GetPrimarySsrcs(std::vector<uint32_t>* primary_ssrcs) const;

  // Convenience to get all the secondary SSRCs for the given primary ssrcs
  // of a particular semantic.
  // If a given primary SSRC does not have a secondary SSRC, the list of
  // secondary SSRCS will be smaller than the list of primary SSRCs.
  void GetSecondarySsrcs(const std::string& semantic,
                         const std::vector<uint32_t>& primary_ssrcs,
                         std::vector<uint32_t>* fid_ssrcs) const;

  // Convenience to get all the FID SSRCs for the given primary ssrcs.
  // If a given primary SSRC does not have a FID SSRC, the list of FID
  // SSRCS will be smaller than the list of primary SSRCs.
  void GetFidSsrcs(const std::vector<uint32_t>& primary_ssrcs,
                   std::vector<uint32_t>* fid_ssrcs) const;

  // Stream ids serialized to SDP.
  std::vector<std::string> stream_ids() const;
  void set_stream_ids(const std::vector<std::string>& stream_ids);

  // Returns the first stream id or "" if none exist. This method exists only
  // as temporary backwards compatibility with the old sync_label.
  std::string first_stream_id() const;

  std::string ToString() const;

  // A unique identifier of the StreamParams object. When the SDP is created,
  // this comes from the track ID of the sender that the StreamParams object
  // is associated with.
  std::string id;
  // There may be no SSRCs stored in unsignaled case when stream_ids are
  // signaled with a=msid lines.
  std::vector<uint32_t> ssrcs;         // All SSRCs for this source
  std::vector<SsrcGroup> ssrc_groups;  // e.g. FID, FEC, SIM
  std::string cname;                   // RTCP CNAME

  // RID functionality according to
  // https://tools.ietf.org/html/draft-ietf-mmusic-rid-15
  // Each layer can be represented by a RID identifier and can also have
  // restrictions (such as max-width, max-height, etc.)
  // If the track has multiple layers (ex. Simulcast), each layer will be
  // represented by a RID.
  bool has_rids() const { return !rids_.empty(); }
  const std::vector<RidDescription>& rids() const { return rids_; }
  void set_rids(const std::vector<RidDescription>& rids) { rids_ = rids; }

 private:
  bool AddSecondarySsrc(const std::string& semantics,
                        uint32_t primary_ssrc,
                        uint32_t secondary_ssrc);
  bool GetSecondarySsrc(const std::string& semantics,
                        uint32_t primary_ssrc,
                        uint32_t* secondary_ssrc) const;

  // The stream IDs of the sender that the StreamParams object is associated
  // with. In Plan B this should always be size of 1, while in Unified Plan this
  // could be none or multiple stream IDs.
  std::vector<std::string> stream_ids_;

  std::vector<RidDescription> rids_;
};

// A Stream can be selected by either id or ssrc.
struct StreamSelector {
  explicit StreamSelector(uint32_t ssrc) : ssrc(ssrc) {}

  explicit StreamSelector(const std::string& streamid)
      : ssrc(0), streamid(streamid) {}

  bool Matches(const StreamParams& stream) const {
    if (ssrc == 0) {
      return stream.id == streamid;
    } else {
      return stream.has_ssrc(ssrc);
    }
  }

  uint32_t ssrc;
  std::string streamid;
};

typedef std::vector<StreamParams> StreamParamsVec;

template <class Condition>
const StreamParams* GetStream(const StreamParamsVec& streams,
                              Condition condition) {
  auto found = absl::c_find_if(streams, condition);
  return found == streams.end() ? nullptr : &(*found);
}

template <class Condition>
StreamParams* GetStream(StreamParamsVec& streams, Condition condition) {
  auto found = absl::c_find_if(streams, condition);
  return found == streams.end() ? nullptr : &(*found);
}

inline bool HasStreamWithNoSsrcs(const StreamParamsVec& streams) {
  return GetStream(streams,
                   [](const StreamParams& sp) { return !sp.has_ssrcs(); });
}

inline const StreamParams* GetStreamBySsrc(const StreamParamsVec& streams,
                                           uint32_t ssrc) {
  return GetStream(
      streams, [&ssrc](const StreamParams& sp) { return sp.has_ssrc(ssrc); });
}

inline const StreamParams* GetStreamByIds(const StreamParamsVec& streams,
                                          const std::string& id) {
  return GetStream(streams,
                   [&id](const StreamParams& sp) { return sp.id == id; });
}

inline StreamParams* GetStreamByIds(StreamParamsVec& streams,
                                    const std::string& id) {
  return GetStream(streams,
                   [&id](const StreamParams& sp) { return sp.id == id; });
}

inline const StreamParams* GetStream(const StreamParamsVec& streams,
                                     const StreamSelector& selector) {
  return GetStream(streams, [&selector](const StreamParams& sp) {
    return selector.Matches(sp);
  });
}

template <class Condition>
bool RemoveStream(StreamParamsVec* streams, Condition condition) {
  auto iter(std::remove_if(streams->begin(), streams->end(), condition));
  if (iter == streams->end())
    return false;
  streams->erase(iter, streams->end());
  return true;
}

// Removes the stream from streams. Returns true if a stream is
// found and removed.
inline bool RemoveStream(StreamParamsVec* streams,
                         const StreamSelector& selector) {
  return RemoveStream(streams, [&selector](const StreamParams& sp) {
    return selector.Matches(sp);
  });
}
inline bool RemoveStreamBySsrc(StreamParamsVec* streams, uint32_t ssrc) {
  return RemoveStream(
      streams, [&ssrc](const StreamParams& sp) { return sp.has_ssrc(ssrc); });
}
inline bool RemoveStreamByIds(StreamParamsVec* streams, const std::string& id) {
  return RemoveStream(streams,
                      [&id](const StreamParams& sp) { return sp.id == id; });
}

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::GetStream;
using ::webrtc::GetStreamByIds;
using ::webrtc::GetStreamBySsrc;
using ::webrtc::HasStreamWithNoSsrcs;
using ::webrtc::kFecFrSsrcGroupSemantics;
using ::webrtc::kFecSsrcGroupSemantics;
using ::webrtc::kFidSsrcGroupSemantics;
using ::webrtc::kSimSsrcGroupSemantics;
using ::webrtc::RemoveStream;
using ::webrtc::RemoveStreamByIds;
using ::webrtc::RemoveStreamBySsrc;
using ::webrtc::SsrcGroup;
using ::webrtc::StreamParams;
using ::webrtc::StreamParamsVec;
using ::webrtc::StreamSelector;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_STREAM_PARAMS_H_
