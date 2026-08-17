/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_SDP_SDP_CHANGER_H_
#define TEST_PC_E2E_SDP_SDP_CHANGER_H_

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/jsep.h"
#include "api/rtp_parameters.h"
#include "api/test/pclf/media_configuration.h"
#include "media/base/rid_description.h"
#include "pc/session_description.h"
#include "pc/simulcast_description.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Creates list of capabilities, which can be set on RtpTransceiverInterface via
// RtpTransceiverInterface::SetCodecPreferences(...) to negotiate use of codecs
// from list of `supported_codecs` which will match `video_codecs`. If flags
// `ulpfec` or `flexfec` set to true corresponding FEC codec will be added.
// FEC and RTX codecs will be added after required codecs.
//
// All codecs will be added only if they exists in the list of
// `supported_codecs`. If multiple codecs from this list will match
// `video_codecs`, then all of them will be added to the output
// vector and they will be added in the same order, as they were in
// `supported_codecs`.
std::vector<RtpCodecCapability> FilterVideoCodecCapabilities(
    ArrayView<const VideoCodecConfig> video_codecs,
    bool use_rtx,
    bool use_ulpfec,
    bool use_flexfec,
    ArrayView<const RtpCodecCapability> supported_codecs);

struct LocalAndRemoteSdp {
  LocalAndRemoteSdp(std::unique_ptr<SessionDescriptionInterface> local_sdp,
                    std::unique_ptr<SessionDescriptionInterface> remote_sdp)
      : local_sdp(std::move(local_sdp)), remote_sdp(std::move(remote_sdp)) {}

  // Sdp, that should be as local description on the peer, that created it.
  std::unique_ptr<SessionDescriptionInterface> local_sdp;
  // Sdp, that should be set as remote description on the peer opposite to the
  // one, who created it.
  std::unique_ptr<SessionDescriptionInterface> remote_sdp;
};

struct PatchingParams {
  PatchingParams(
      bool use_conference_mode,
      std::map<std::string, int> stream_label_to_simulcast_streams_count)
      : use_conference_mode(use_conference_mode),
        stream_label_to_simulcast_streams_count(
            stream_label_to_simulcast_streams_count) {}

  bool use_conference_mode;
  std::map<std::string, int> stream_label_to_simulcast_streams_count;
};

class SignalingInterceptor {
 public:
  explicit SignalingInterceptor(PatchingParams params) : params_(params) {}

  LocalAndRemoteSdp PatchOffer(
      std::unique_ptr<SessionDescriptionInterface> offer,
      const VideoCodecConfig& first_codec);
  LocalAndRemoteSdp PatchAnswer(
      std::unique_ptr<SessionDescriptionInterface> answer,
      const VideoCodecConfig& first_codec);

  std::vector<std::unique_ptr<IceCandidateInterface>> PatchOffererIceCandidates(
      ArrayView<const IceCandidateInterface* const> candidates);
  std::vector<std::unique_ptr<IceCandidateInterface>>
  PatchAnswererIceCandidates(
      ArrayView<const IceCandidateInterface* const> candidates);

 private:
  // Contains information about simulcast section, that is required to perform
  // modified offer/answer and ice candidates exchange.
  struct SimulcastSectionInfo {
    SimulcastSectionInfo(const std::string& mid,
                         MediaProtocolType media_protocol_type,
                         const std::vector<RidDescription>& rids_desc);

    const std::string mid;
    const MediaProtocolType media_protocol_type;
    std::vector<std::string> rids;
    SimulcastDescription simulcast_description;
    webrtc::RtpExtension mid_extension;
    webrtc::RtpExtension rid_extension;
    webrtc::RtpExtension rrid_extension;
    TransportDescription transport_description;
  };

  struct SignalingContext {
    SignalingContext() = default;
    // SignalingContext is not copyable and movable.
    SignalingContext(SignalingContext&) = delete;
    SignalingContext& operator=(SignalingContext&) = delete;
    SignalingContext(SignalingContext&&) = delete;
    SignalingContext& operator=(SignalingContext&&) = delete;

    void AddSimulcastInfo(const SimulcastSectionInfo& info);
    bool HasSimulcast() const { return !simulcast_infos.empty(); }

    std::vector<SimulcastSectionInfo> simulcast_infos;
    std::map<std::string, SimulcastSectionInfo*> simulcast_infos_by_mid;
    std::map<std::string, SimulcastSectionInfo*> simulcast_infos_by_rid;

    std::vector<std::string> mids_order;
  };

  LocalAndRemoteSdp PatchVp8Offer(
      std::unique_ptr<SessionDescriptionInterface> offer);
  LocalAndRemoteSdp PatchVp9Offer(
      std::unique_ptr<SessionDescriptionInterface> offer);
  LocalAndRemoteSdp PatchVp8Answer(
      std::unique_ptr<SessionDescriptionInterface> answer);
  LocalAndRemoteSdp PatchVp9Answer(
      std::unique_ptr<SessionDescriptionInterface> answer);

  void FillSimulcastContext(SessionDescriptionInterface* offer);
  std::unique_ptr<SessionDescription> RestoreMediaSectionsOrder(
      std::unique_ptr<SessionDescription> source);

  PatchingParams params_;
  SignalingContext context_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_SDP_SDP_CHANGER_H_
