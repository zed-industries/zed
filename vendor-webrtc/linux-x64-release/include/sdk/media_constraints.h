/*
 *  Copyright 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Implementation of the w3c constraints spec is the responsibility of the
// browser. Chrome no longer uses the constraints api declared here, and it will
// be removed from WebRTC.
// https://bugs.chromium.org/p/webrtc/issues/detail?id=9239

#ifndef SDK_MEDIA_CONSTRAINTS_H_
#define SDK_MEDIA_CONSTRAINTS_H_

#include <stddef.h>

#include <string>
#include <utility>
#include <vector>

#include "api/audio_options.h"
#include "api/peer_connection_interface.h"

namespace webrtc {

// Class representing constraints, as used by the android and objc apis.
//
// Constraints may be either "mandatory", which means that unless satisfied,
// the method taking the constraints should fail, or "optional", which means
// they may not be satisfied..
class MediaConstraints {
 public:
  struct Constraint {
    Constraint() {}
    Constraint(const std::string& key, const std::string value)
        : key(key), value(value) {}
    std::string key;
    std::string value;
  };

  class Constraints : public std::vector<Constraint> {
   public:
    Constraints() = default;
    Constraints(std::initializer_list<Constraint> l)
        : std::vector<Constraint>(l) {}

    bool FindFirst(const std::string& key, std::string* value) const;
  };

  MediaConstraints() = default;
  MediaConstraints(Constraints mandatory, Constraints optional)
      : mandatory_(std::move(mandatory)), optional_(std::move(optional)) {}

  // Constraint keys used by a local audio source.

  // These keys are google specific.
  static const char kGoogEchoCancellation[];  // googEchoCancellation

  static const char kAutoGainControl[];   // googAutoGainControl
  static const char kNoiseSuppression[];  // googNoiseSuppression
  static const char kHighpassFilter[];    // googHighpassFilter
  static const char kAudioMirroring[];    // googAudioMirroring
  static const char
      kAudioNetworkAdaptorConfig[];  // googAudioNetworkAdaptorConfig
  static const char kInitAudioRecordingOnSend[];  // InitAudioRecordingOnSend;

  // Constraint keys for CreateOffer / CreateAnswer
  // Specified by the W3C PeerConnection spec
  static const char kOfferToReceiveVideo[];     // OfferToReceiveVideo
  static const char kOfferToReceiveAudio[];     // OfferToReceiveAudio
  static const char kVoiceActivityDetection[];  // VoiceActivityDetection
  static const char kIceRestart[];              // IceRestart
  // These keys are google specific.
  static const char kUseRtpMux[];  // googUseRtpMUX

  // Constraints values.
  static const char kValueTrue[];   // true
  static const char kValueFalse[];  // false

  // PeerConnection constraint keys.
  // Google-specific constraint keys.
  // Temporary pseudo-constraint for enabling DSCP through JS.
  static const char kEnableDscp[];  // googDscp
  // Constraint to enable IPv6 through JS.
  static const char kEnableIPv6[];  // googIPv6
  // Temporary constraint to enable suspend below min bitrate feature.
  static const char kEnableVideoSuspendBelowMinBitrate[];
  static const char kScreencastMinBitrate[];  // googScreencastMinBitrate
  static const char kCpuOveruseDetection[];   // googCpuOveruseDetection

  // Constraint to enable negotiating raw RTP packetization using attribute
  // "a=packetization:<payload_type> raw" in the SDP for all video payload.
  static const char kRawPacketizationForVideoEnabled[];

  // Specifies number of simulcast layers for all video tracks
  // with a Plan B offer/answer
  // (see RTCOfferAnswerOptions::num_simulcast_layers).
  static const char kNumSimulcastLayers[];

  ~MediaConstraints() = default;

  const Constraints& GetMandatory() const { return mandatory_; }
  const Constraints& GetOptional() const { return optional_; }

 private:
  const Constraints mandatory_ = {};
  const Constraints optional_ = {};
};

// Copy all relevant constraints into an RTCConfiguration object.
void CopyConstraintsIntoRtcConfiguration(
    const MediaConstraints* constraints,
    PeerConnectionInterface::RTCConfiguration* configuration);

// Copy all relevant constraints into an AudioOptions object.
void CopyConstraintsIntoAudioOptions(const MediaConstraints* constraints,
                                     AudioOptions* options);

bool CopyConstraintsIntoOfferAnswerOptions(
    const MediaConstraints* constraints,
    PeerConnectionInterface::RTCOfferAnswerOptions* offer_answer_options);

}  // namespace webrtc

#endif  // SDK_MEDIA_CONSTRAINTS_H_
