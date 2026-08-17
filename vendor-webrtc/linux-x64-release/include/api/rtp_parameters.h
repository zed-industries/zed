/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTP_PARAMETERS_H_
#define API_RTP_PARAMETERS_H_

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "api/media_types.h"
#include "api/priority.h"
#include "api/rtp_transceiver_direction.h"
#include "api/video/resolution.h"
#include "api/video_codecs/scalability_mode.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

using CodecParameterMap = std::map<std::string, std::string>;

// These structures are intended to mirror those defined by:
// http://draft.ortc.org/#rtcrtpdictionaries*
// Contains everything specified as of 2017 Jan 24.
//
// They are used when retrieving or modifying the parameters of an
// RtpSender/RtpReceiver, or retrieving capabilities.
//
// Note on conventions: Where ORTC may use "octet", "short" and "unsigned"
// types, we typically use "int", in keeping with our style guidelines. The
// parameter's actual valid range will be enforced when the parameters are set,
// rather than when the parameters struct is built. An exception is made for
// SSRCs, since they use the full unsigned 32-bit range, and aren't expected to
// be used for any numeric comparisons/operations.
//
// Additionally, where ORTC uses strings, we may use enums for things that have
// a fixed number of supported values. However, for things that can be extended
// (such as codecs, by providing an external encoder factory), a string
// identifier is used.

enum class FecMechanism {
  RED,
  RED_AND_ULPFEC,
  FLEXFEC,
};

// Used in RtcpFeedback struct.
enum class RtcpFeedbackType {
  CCM,
  LNTF,  // "goog-lntf"
  NACK,
  REMB,  // "goog-remb"
  TRANSPORT_CC,
};

// Used in RtcpFeedback struct when type is NACK or CCM.
enum class RtcpFeedbackMessageType {
  // Equivalent to {type: "nack", parameter: undefined} in ORTC.
  GENERIC_NACK,
  PLI,  // Usable with NACK.
  FIR,  // Usable with CCM.
};

enum class DtxStatus {
  DISABLED,
  ENABLED,
};

// Based on the spec in
// https://w3c.github.io/webrtc-pc/#idl-def-rtcdegradationpreference.
// These options are enforced on a best-effort basis. For instance, all of
// these options may suffer some frame drops in order to avoid queuing.
// TODO(sprang): Look into possibility of more strictly enforcing the
// maintain-framerate option.
// TODO(deadbeef): Default to "balanced", as the spec indicates?
enum class DegradationPreference {
  // Don't take any actions based on over-utilization signals. Not part of the
  // web API.
  DISABLED,
  // On over-use, request lower resolution, possibly causing down-scaling.
  MAINTAIN_FRAMERATE,
  // On over-use, request lower frame rate, possibly causing frame drops.
  MAINTAIN_RESOLUTION,
  // Try to strike a "pleasing" balance between frame rate or resolution.
  BALANCED,
};

RTC_EXPORT const char* DegradationPreferenceToString(
    DegradationPreference degradation_preference);

RTC_EXPORT extern const double kDefaultBitratePriority;

struct RTC_EXPORT RtcpFeedback {
  RtcpFeedbackType type = RtcpFeedbackType::CCM;

  // Equivalent to ORTC "parameter" field with slight differences:
  // 1. It's an enum instead of a string.
  // 2. Generic NACK feedback is represented by a GENERIC_NACK message type,
  //    rather than an unset "parameter" value.
  std::optional<RtcpFeedbackMessageType> message_type;

  // Constructors for convenience.
  RtcpFeedback();
  explicit RtcpFeedback(RtcpFeedbackType type);
  RtcpFeedback(RtcpFeedbackType type, RtcpFeedbackMessageType message_type);
  RtcpFeedback(const RtcpFeedback&);
  ~RtcpFeedback();

  bool operator==(const RtcpFeedback& o) const {
    return type == o.type && message_type == o.message_type;
  }
  bool operator!=(const RtcpFeedback& o) const { return !(*this == o); }
};

struct RTC_EXPORT RtpCodec {
  RtpCodec();
  RtpCodec(const RtpCodec&);
  virtual ~RtpCodec();

  // Build MIME "type/subtype" string from `name` and `kind`.
  std::string mime_type() const { return MediaTypeToString(kind) + "/" + name; }

  // Used to identify the codec. Equivalent to MIME subtype.
  std::string name;

  // The media type of this codec. Equivalent to MIME top-level type.
  webrtc::MediaType kind = webrtc::MediaType::AUDIO;

  // If unset, the implementation default is used.
  std::optional<int> clock_rate;

  // The number of audio channels used. Unset for video codecs. If unset for
  // audio, the implementation default is used.
  // TODO(deadbeef): The "implementation default" part isn't fully implemented.
  // Only defaults to 1, even though some codecs (such as opus) should really
  // default to 2.
  std::optional<int> num_channels;

  // Feedback mechanisms to be used for this codec.
  // TODO(deadbeef): Not implemented with PeerConnection senders/receivers.
  std::vector<RtcpFeedback> rtcp_feedback;

  // Codec-specific parameters that must be signaled to the remote party.
  //
  // Corresponds to "a=fmtp" parameters in SDP.
  //
  // Contrary to ORTC, these parameters are named using all lowercase strings.
  // This helps make the mapping to SDP simpler, if an application is using SDP.
  // Boolean values are represented by the string "1".
  std::map<std::string, std::string> parameters;

  bool operator==(const RtpCodec& o) const {
    return name == o.name && kind == o.kind && clock_rate == o.clock_rate &&
           num_channels == o.num_channels && rtcp_feedback == o.rtcp_feedback &&
           parameters == o.parameters;
  }
  bool operator!=(const RtpCodec& o) const { return !(*this == o); }
  bool IsResiliencyCodec() const;
  bool IsMediaCodec() const;
};

// RtpCodecCapability is to RtpCodecParameters as RtpCapabilities is to
// RtpParameters. This represents the static capabilities of an endpoint's
// implementation of a codec.
struct RTC_EXPORT RtpCodecCapability : public RtpCodec {
  RtpCodecCapability();
  virtual ~RtpCodecCapability();

  // Default payload type for this codec. Mainly needed for codecs that have
  // statically assigned payload types.
  std::optional<int> preferred_payload_type;

  // List of scalability modes supported by the video codec.
  absl::InlinedVector<ScalabilityMode, kScalabilityModeCount> scalability_modes;

  bool operator==(const RtpCodecCapability& o) const {
    return RtpCodec::operator==(o) &&
           preferred_payload_type == o.preferred_payload_type &&
           scalability_modes == o.scalability_modes;
  }
  bool operator!=(const RtpCodecCapability& o) const { return !(*this == o); }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const RtpCodecCapability& cap) {
    if (cap.kind == webrtc::MediaType::AUDIO) {
      absl::Format(&sink, "[audio/%s/%d/%d]", cap.name,
                   cap.clock_rate.value_or(0), cap.num_channels.value_or(1));
    } else {
      absl::Format(&sink, "[video/%s]", cap.name);
    }
  }
};

// Used in RtpCapabilities and RtpTransceiverInterface's header extensions query
// and setup methods; represents the capabilities/preferences of an
// implementation for a header extension.
//
// Just called "RtpHeaderExtension" in ORTC, but the "Capability" suffix was
// added here for consistency and to avoid confusion with
// RtpHeaderExtensionParameters.
//
// Note that ORTC includes a "kind" field, but we omit this because it's
// redundant; if you call
// "RtpReceiver::GetCapabilities(webrtc::MediaType::AUDIO)", you know you're
// getting audio capabilities.
struct RTC_EXPORT RtpHeaderExtensionCapability {
  // URI of this extension, as defined in RFC8285.
  std::string uri;

  // Preferred value of ID that goes in the packet.
  std::optional<int> preferred_id;

  // If true, it's preferred that the value in the header is encrypted.
  bool preferred_encrypt = false;

  // The direction of the extension. The kStopped value is only used with
  // RtpTransceiverInterface::SetHeaderExtensionsToNegotiate() and
  // SetHeaderExtensionsToNegotiate().
  RtpTransceiverDirection direction = RtpTransceiverDirection::kSendRecv;

  // Constructors for convenience.
  RtpHeaderExtensionCapability();
  explicit RtpHeaderExtensionCapability(absl::string_view uri);
  RtpHeaderExtensionCapability(absl::string_view uri, int preferred_id);
  RtpHeaderExtensionCapability(absl::string_view uri,
                               int preferred_id,
                               RtpTransceiverDirection direction);
  RtpHeaderExtensionCapability(absl::string_view uri,
                               int preferred_id,
                               bool preferred_encrypt,
                               RtpTransceiverDirection direction);
  ~RtpHeaderExtensionCapability();

  bool operator==(const RtpHeaderExtensionCapability& o) const {
    return uri == o.uri && preferred_id == o.preferred_id &&
           preferred_encrypt == o.preferred_encrypt && direction == o.direction;
  }
  bool operator!=(const RtpHeaderExtensionCapability& o) const {
    return !(*this == o);
  }
};

// RTP header extension, see RFC8285.
struct RTC_EXPORT RtpExtension {
  enum Filter {
    // Encrypted extensions will be ignored and only non-encrypted extensions
    // will be considered.
    kDiscardEncryptedExtension,
    // Encrypted extensions will be preferred but will fall back to
    // non-encrypted extensions if necessary.
    kPreferEncryptedExtension,
    // Encrypted extensions will be required, so any non-encrypted extensions
    // will be discarded.
    kRequireEncryptedExtension,
  };

  RtpExtension();
  RtpExtension(absl::string_view uri, int id);
  RtpExtension(absl::string_view uri, int id, bool encrypt);
  ~RtpExtension();

  std::string ToString() const;
  bool operator==(const RtpExtension& rhs) const {
    return uri == rhs.uri && id == rhs.id && encrypt == rhs.encrypt;
  }
  static bool IsSupportedForAudio(absl::string_view uri);
  static bool IsSupportedForVideo(absl::string_view uri);
  // Return "true" if the given RTP header extension URI may be encrypted.
  static bool IsEncryptionSupported(absl::string_view uri);

  // Returns the header extension with the given URI or nullptr if not found.
  static const RtpExtension* FindHeaderExtensionByUri(
      const std::vector<RtpExtension>& extensions,
      absl::string_view uri,
      Filter filter);

  // Returns the header extension with the given URI and encrypt parameter,
  // if found, otherwise nullptr.
  static const RtpExtension* FindHeaderExtensionByUriAndEncryption(
      const std::vector<RtpExtension>& extensions,
      absl::string_view uri,
      bool encrypt);

  // Returns a list of extensions where any extension URI is unique.
  // The returned list will be sorted by uri first, then encrypt and id last.
  // Having the list sorted allows the caller fo compare filtered lists for
  // equality to detect when changes have been made.
  static const std::vector<RtpExtension> DeduplicateHeaderExtensions(
      const std::vector<RtpExtension>& extensions,
      Filter filter);

  // Encryption of Header Extensions, see RFC 6904 for details:
  // https://tools.ietf.org/html/rfc6904
  static constexpr char kEncryptHeaderExtensionsUri[] =
      "urn:ietf:params:rtp-hdrext:encrypt";

  // Header extension for audio levels, as defined in:
  // https://tools.ietf.org/html/rfc6464
  static constexpr char kAudioLevelUri[] =
      "urn:ietf:params:rtp-hdrext:ssrc-audio-level";

  // Header extension for RTP timestamp offset, see RFC 5450 for details:
  // http://tools.ietf.org/html/rfc5450
  static constexpr char kTimestampOffsetUri[] =
      "urn:ietf:params:rtp-hdrext:toffset";

  // Header extension for absolute send time, see url for details:
  // http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
  static constexpr char kAbsSendTimeUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time";

  // Header extension for absolute capture time, see url for details:
  // http://www.webrtc.org/experiments/rtp-hdrext/abs-capture-time
  static constexpr char kAbsoluteCaptureTimeUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/abs-capture-time";

  // Header extension for coordination of video orientation, see url for
  // details:
  // http://www.etsi.org/deliver/etsi_ts/126100_126199/126114/12.07.00_60/ts_126114v120700p.pdf
  static constexpr char kVideoRotationUri[] = "urn:3gpp:video-orientation";

  // Header extension for video content type. E.g. default or screenshare.
  static constexpr char kVideoContentTypeUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/video-content-type";

  // Header extension for video timing.
  static constexpr char kVideoTimingUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/video-timing";

  // Experimental codec agnostic frame descriptor.
  static constexpr char kGenericFrameDescriptorUri00[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/"
      "generic-frame-descriptor-00";
  static constexpr char kDependencyDescriptorUri[] =
      "https://aomediacodec.github.io/av1-rtp-spec/"
      "#dependency-descriptor-rtp-header-extension";

  // Experimental extension for signalling target bitrate per layer.
  static constexpr char kVideoLayersAllocationUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/video-layers-allocation00";

  // Header extension for transport sequence number, see url for details:
  // http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions
  static constexpr char kTransportSequenceNumberUri[] =
      "http://www.ietf.org/id/"
      "draft-holmer-rmcat-transport-wide-cc-extensions-01";
  static constexpr char kTransportSequenceNumberV2Uri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/transport-wide-cc-02";

  // This extension allows applications to adaptively limit the playout delay
  // on frames as per the current needs. For example, a gaming application
  // has very different needs on end-to-end delay compared to a video-conference
  // application.
  static constexpr char kPlayoutDelayUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/playout-delay";

  // Header extension for color space information.
  static constexpr char kColorSpaceUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/color-space";

  // Header extension for identifying media section within a transport.
  // https://tools.ietf.org/html/draft-ietf-mmusic-sdp-bundle-negotiation-49#section-15
  static constexpr char kMidUri[] = "urn:ietf:params:rtp-hdrext:sdes:mid";

  // Header extension for RIDs and Repaired RIDs
  // https://tools.ietf.org/html/draft-ietf-avtext-rid-09
  // https://tools.ietf.org/html/draft-ietf-mmusic-rid-15
  static constexpr char kRidUri[] =
      "urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id";
  static constexpr char kRepairedRidUri[] =
      "urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id";

  // Header extension to propagate webrtc::VideoFrame id field
  static constexpr char kVideoFrameTrackingIdUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/video-frame-tracking-id";

  // Header extension for Mixer-to-Client audio levels per CSRC as defined in
  // https://tools.ietf.org/html/rfc6465
  static constexpr char kCsrcAudioLevelsUri[] =
      "urn:ietf:params:rtp-hdrext:csrc-audio-level";

  // Header extension for automatic corruption detection.
  static constexpr char kCorruptionDetectionUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/corruption-detection";

  // Inclusive min and max IDs for two-byte header extensions and one-byte
  // header extensions, per RFC8285 Section 4.2-4.3.
  static constexpr int kMinId = 1;
  static constexpr int kMaxId = 255;
  static constexpr int kMaxValueSize = 255;
  static constexpr int kOneByteHeaderExtensionMaxId = 14;
  static constexpr int kOneByteHeaderExtensionMaxValueSize = 16;

  std::string uri;
  int id = 0;
  bool encrypt = false;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const RtpExtension& extension) {
    if (extension.encrypt) {
      absl::Format(&sink, "[%d %s (encrypted)]", extension.id, extension.uri);
    } else {
      absl::Format(&sink, "[%d %s]", extension.id, extension.uri);
    }
  }
};

struct RTC_EXPORT RtpFecParameters {
  // If unset, a value is chosen by the implementation.
  // Works just like RtpEncodingParameters::ssrc.
  std::optional<uint32_t> ssrc;

  FecMechanism mechanism = FecMechanism::RED;

  // Constructors for convenience.
  RtpFecParameters();
  explicit RtpFecParameters(FecMechanism mechanism);
  RtpFecParameters(FecMechanism mechanism, uint32_t ssrc);
  RtpFecParameters(const RtpFecParameters&);
  ~RtpFecParameters();

  bool operator==(const RtpFecParameters& o) const {
    return ssrc == o.ssrc && mechanism == o.mechanism;
  }
  bool operator!=(const RtpFecParameters& o) const { return !(*this == o); }
};

struct RTC_EXPORT RtpRtxParameters {
  // If unset, a value is chosen by the implementation.
  // Works just like RtpEncodingParameters::ssrc.
  std::optional<uint32_t> ssrc;

  // Constructors for convenience.
  RtpRtxParameters();
  explicit RtpRtxParameters(uint32_t ssrc);
  RtpRtxParameters(const RtpRtxParameters&);
  ~RtpRtxParameters();

  bool operator==(const RtpRtxParameters& o) const { return ssrc == o.ssrc; }
  bool operator!=(const RtpRtxParameters& o) const { return !(*this == o); }
};

struct RTC_EXPORT RtpEncodingParameters {
  RtpEncodingParameters();
  RtpEncodingParameters(const RtpEncodingParameters&);
  ~RtpEncodingParameters();

  // If unset, a value is chosen by the implementation.
  //
  // Note that the chosen value is NOT returned by GetParameters, because it
  // may change due to an SSRC conflict, in which case the conflict is handled
  // internally without any event. Another way of looking at this is that an
  // unset SSRC acts as a "wildcard" SSRC.
  std::optional<uint32_t> ssrc;

  // The relative bitrate priority of this encoding. Currently this is
  // implemented for the entire rtp sender by using the value of the first
  // encoding parameter.
  // See: https://w3c.github.io/webrtc-priority/#enumdef-rtcprioritytype
  // "very-low" = 0.5
  // "low" = 1.0
  // "medium" = 2.0
  // "high" = 4.0
  // TODO(webrtc.bugs.org/8630): Implement this per encoding parameter.
  // Currently there is logic for how bitrate is distributed per simulcast layer
  // in the VideoBitrateAllocator. This must be updated to incorporate relative
  // bitrate priority.
  double bitrate_priority = kDefaultBitratePriority;

  // The relative DiffServ Code Point priority for this encoding, allowing
  // packets to be marked relatively higher or lower without affecting
  // bandwidth allocations. See https://w3c.github.io/webrtc-dscp-exp/ .
  // TODO(http://crbug.com/webrtc/8630): Implement this per encoding parameter.
  // TODO(http://crbug.com/webrtc/11379): TCP connections should use a single
  // DSCP value even if shared by multiple senders; this is not implemented.
  Priority network_priority = Priority::kLow;

  // If set, this represents the Transport Independent Application Specific
  // maximum bandwidth defined in RFC3890. If unset, there is no maximum
  // bitrate. Currently this is implemented for the entire rtp sender by using
  // the value of the first encoding parameter.
  //
  // Just called "maxBitrate" in ORTC spec.
  //
  // TODO(deadbeef): With ORTC RtpSenders, this currently sets the total
  // bandwidth for the entire bandwidth estimator (audio and video). This is
  // just always how "b=AS" was handled, but it's not correct and should be
  // fixed.
  std::optional<int> max_bitrate_bps;

  // Specifies the minimum bitrate in bps for video.
  std::optional<int> min_bitrate_bps;

  // Specifies the maximum framerate in fps for video.
  std::optional<double> max_framerate;

  // Specifies the number of temporal layers for video (if the feature is
  // supported by the codec implementation).
  // Screencast support is experimental.
  std::optional<int> num_temporal_layers;

  // For video, scale the resolution down by this factor.
  std::optional<double> scale_resolution_down_by;

  // https://w3c.github.io/webrtc-svc/#rtcrtpencodingparameters
  std::optional<std::string> scalability_mode;

  // This is an alternative API to `scale_resolution_down_by` but expressed in
  // absolute terms (max width and max height) as opposed to relative terms (a
  // scaling factor that is relative to the input frame size).
  //
  // If both `scale_resolution_down_by` and `scale_resolution_down_to` are
  // specified, the "scale by" value is ignored.
  //
  // See spec:
  // https://w3c.github.io/webrtc-extensions/#dom-rtcrtpencodingparameters-scaleresolutiondownto
  std::optional<Resolution> scale_resolution_down_to;

  // For an RtpSender, set to true to cause this encoding to be encoded and
  // sent, and false for it not to be encoded and sent. This allows control
  // across multiple encodings of a sender for turning simulcast layers on and
  // off.
  // TODO(webrtc.bugs.org/8807): Updating this parameter will trigger an encoder
  // reset, but this isn't necessarily required.
  bool active = true;

  // Value to use for RID RTP header extension.
  // Called "encodingId" in ORTC.
  std::string rid;
  bool request_key_frame = false;

  // Allow dynamic frame length changes for audio:
  // https://w3c.github.io/webrtc-extensions/#dom-rtcrtpencodingparameters-adaptiveptime
  bool adaptive_ptime = false;

  // Allow changing the used codec for this encoding.
  std::optional<RtpCodec> codec;

  bool operator==(const RtpEncodingParameters& o) const {
    return ssrc == o.ssrc && bitrate_priority == o.bitrate_priority &&
           network_priority == o.network_priority &&
           max_bitrate_bps == o.max_bitrate_bps &&
           min_bitrate_bps == o.min_bitrate_bps &&
           max_framerate == o.max_framerate &&
           num_temporal_layers == o.num_temporal_layers &&
           scale_resolution_down_by == o.scale_resolution_down_by &&
           active == o.active && rid == o.rid &&
           adaptive_ptime == o.adaptive_ptime &&
           scale_resolution_down_to == o.scale_resolution_down_to &&
           codec == o.codec;
  }
  bool operator!=(const RtpEncodingParameters& o) const {
    return !(*this == o);
  }
};

struct RTC_EXPORT RtpCodecParameters : public RtpCodec {
  RtpCodecParameters();
  RtpCodecParameters(const RtpCodecParameters&);
  virtual ~RtpCodecParameters();

  // Payload type used to identify this codec in RTP packets.
  // This must always be present, and must be unique across all codecs using
  // the same transport.
  int payload_type = 0;

  bool operator==(const RtpCodecParameters& o) const {
    return RtpCodec::operator==(o) && payload_type == o.payload_type;
  }
  bool operator!=(const RtpCodecParameters& o) const { return !(*this == o); }
};

// RtpCapabilities is used to represent the static capabilities of an endpoint.
// An application can use these capabilities to construct an RtpParameters.
struct RTC_EXPORT RtpCapabilities {
  RtpCapabilities();
  ~RtpCapabilities();

  // Supported codecs.
  std::vector<RtpCodecCapability> codecs;

  // Supported RTP header extensions.
  std::vector<RtpHeaderExtensionCapability> header_extensions;

  // Supported Forward Error Correction (FEC) mechanisms. Note that the RED,
  // ulpfec and flexfec codecs used by these mechanisms will still appear in
  // `codecs`.
  std::vector<FecMechanism> fec;

  bool operator==(const RtpCapabilities& o) const {
    return codecs == o.codecs && header_extensions == o.header_extensions &&
           fec == o.fec;
  }
  bool operator!=(const RtpCapabilities& o) const { return !(*this == o); }
};

struct RtcpParameters final {
  RtcpParameters();
  RtcpParameters(const RtcpParameters&);
  ~RtcpParameters();

  // The SSRC to be used in the "SSRC of packet sender" field. If not set, one
  // will be chosen by the implementation.
  // TODO(deadbeef): Not implemented.
  std::optional<uint32_t> ssrc;

  // The Canonical Name (CNAME) used by RTCP (e.g. in SDES messages).
  //
  // If empty in the construction of the RtpTransport, one will be generated by
  // the implementation, and returned in GetRtcpParameters. Multiple
  // RtpTransports created by the same OrtcFactory will use the same generated
  // CNAME.
  //
  // If empty when passed into SetParameters, the CNAME simply won't be
  // modified.
  std::string cname;

  // Send reduced-size RTCP?
  bool reduced_size = false;

  // Send RTCP multiplexed on the RTP transport?
  // Not used with PeerConnection senders/receivers
  bool mux = true;

  bool operator==(const RtcpParameters& o) const {
    return ssrc == o.ssrc && cname == o.cname &&
           reduced_size == o.reduced_size && mux == o.mux;
  }
  bool operator!=(const RtcpParameters& o) const { return !(*this == o); }
};

struct RTC_EXPORT RtpParameters {
  RtpParameters();
  RtpParameters(const RtpParameters&);
  ~RtpParameters();

  // Used when calling getParameters/setParameters with a PeerConnection
  // RtpSender, to ensure that outdated parameters are not unintentionally
  // applied successfully.
  std::string transaction_id;

  // Value to use for MID RTP header extension.
  // Called "muxId" in ORTC.
  // TODO(deadbeef): Not implemented.
  std::string mid;

  std::vector<RtpCodecParameters> codecs;

  std::vector<RtpExtension> header_extensions;

  std::vector<RtpEncodingParameters> encodings;

  // Only available with a Peerconnection RtpSender.
  // In ORTC, our API includes an additional "RtpTransport"
  // abstraction on which RTCP parameters are set.
  RtcpParameters rtcp;

  // When bandwidth is constrained and the RtpSender needs to choose between
  // degrading resolution or degrading framerate, degradationPreference
  // indicates which is preferred. Only for video tracks.
  std::optional<DegradationPreference> degradation_preference;

  bool operator==(const RtpParameters& o) const {
    return mid == o.mid && codecs == o.codecs &&
           header_extensions == o.header_extensions &&
           encodings == o.encodings && rtcp == o.rtcp &&
           degradation_preference == o.degradation_preference;
  }
  bool operator!=(const RtpParameters& o) const { return !(*this == o); }
};

}  // namespace webrtc

#endif  // API_RTP_PARAMETERS_H_
