/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTP_TRANSCEIVER_INTERFACE_H_
#define API_RTP_TRANSCEIVER_INTERFACE_H_

#include <optional>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "api/array_view.h"
#include "api/media_types.h"
#include "api/ref_count.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_direction.h"
#include "api/scoped_refptr.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Structure for initializing an RtpTransceiver in a call to
// PeerConnectionInterface::AddTransceiver.
// https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiverinit
struct RTC_EXPORT RtpTransceiverInit final {
  RtpTransceiverInit();
  RtpTransceiverInit(const RtpTransceiverInit&);
  ~RtpTransceiverInit();
  // Direction of the RtpTransceiver. See RtpTransceiverInterface::direction().
  RtpTransceiverDirection direction = RtpTransceiverDirection::kSendRecv;

  // The added RtpTransceiver will be added to these streams.
  std::vector<std::string> stream_ids;

  std::vector<RtpEncodingParameters> send_encodings;
};

// The RtpTransceiverInterface maps to the RTCRtpTransceiver defined by the
// WebRTC specification. A transceiver represents a combination of an RtpSender
// and an RtpReceiver than share a common mid. As defined in JSEP, an
// RtpTransceiver is said to be associated with a media description if its mid
// property is non-null; otherwise, it is said to be disassociated.
// JSEP: https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24
//
// Note that RtpTransceivers are only supported when using PeerConnection with
// Unified Plan SDP.
//
// This class is thread-safe.
//
// WebRTC specification for RTCRtpTransceiver, the JavaScript analog:
// https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver
class RTC_EXPORT RtpTransceiverInterface : public webrtc::RefCountInterface {
 public:
  // Media type of the transceiver. Any sender(s)/receiver(s) will have this
  // type as well.
  virtual webrtc::MediaType media_type() const = 0;

  // The mid attribute is the mid negotiated and present in the local and
  // remote descriptions. Before negotiation is complete, the mid value may be
  // null. After rollbacks, the value may change from a non-null value to null.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-mid
  virtual std::optional<std::string> mid() const = 0;

  // The sender attribute exposes the RtpSender corresponding to the RTP media
  // that may be sent with the transceiver's mid. The sender is always present,
  // regardless of the direction of media.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-sender
  virtual scoped_refptr<RtpSenderInterface> sender() const = 0;

  // The receiver attribute exposes the RtpReceiver corresponding to the RTP
  // media that may be received with the transceiver's mid. The receiver is
  // always present, regardless of the direction of media.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-receiver
  virtual scoped_refptr<RtpReceiverInterface> receiver() const = 0;

  // The stopped attribute indicates that the sender of this transceiver will no
  // longer send, and that the receiver will no longer receive. It is true if
  // either stop has been called or if setting the local or remote description
  // has caused the RtpTransceiver to be stopped.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-stopped
  virtual bool stopped() const = 0;

  // The stopping attribute indicates that the user has indicated that the
  // sender of this transceiver will stop sending, and that the receiver will
  // no longer receive. It is always true if stopped() is true.
  // If stopping() is true and stopped() is false, it means that the
  // transceiver's stop() method has been called, but the negotiation with
  // the other end for shutting down the transceiver is not yet done.
  // https://w3c.github.io/webrtc-pc/#dfn-stopping-0
  virtual bool stopping() const = 0;

  // The direction attribute indicates the preferred direction of this
  // transceiver, which will be used in calls to CreateOffer and CreateAnswer.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-direction
  virtual RtpTransceiverDirection direction() const = 0;

  // Sets the preferred direction of this transceiver. An update of
  // directionality does not take effect immediately. Instead, future calls to
  // CreateOffer and CreateAnswer mark the corresponding media descriptions as
  // sendrecv, sendonly, recvonly, or inactive.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-direction
  // TODO(hta): Deprecate SetDirection without error and rename
  // SetDirectionWithError to SetDirection, remove default implementations.
  ABSL_DEPRECATED("Use SetDirectionWithError instead")
  virtual void SetDirection(RtpTransceiverDirection new_direction);
  virtual RTCError SetDirectionWithError(RtpTransceiverDirection new_direction);

  // The current_direction attribute indicates the current direction negotiated
  // for this transceiver. If this transceiver has never been represented in an
  // offer/answer exchange, or if the transceiver is stopped, the value is null.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-currentdirection
  virtual std::optional<RtpTransceiverDirection> current_direction() const = 0;

  // An internal slot designating for which direction the relevant
  // PeerConnection events have been fired. This is to ensure that events like
  // OnAddTrack only get fired once even if the same session description is
  // applied again.
  // Exposed in the public interface for use by Chromium.
  virtual std::optional<RtpTransceiverDirection> fired_direction() const;

  // Initiates a stop of the transceiver.
  // The stop is complete when stopped() returns true.
  // A stopped transceiver can be reused for a different track.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-stop
  // TODO(hta): Rename to Stop() when users of the non-standard Stop() are
  // updated.
  virtual RTCError StopStandard();

  // Stops a transceiver immediately, without waiting for signalling.
  // This is an internal function, and is exposed for historical reasons.
  // https://w3c.github.io/webrtc-pc/#dfn-stop-the-rtcrtptransceiver
  virtual void StopInternal();
  ABSL_DEPRECATED("Use StopStandard instead") virtual void Stop();

  // The SetCodecPreferences method overrides the default codec preferences used
  // by WebRTC for this transceiver.
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtptransceiver-setcodecpreferences
  virtual RTCError SetCodecPreferences(
      ArrayView<RtpCodecCapability> codecs) = 0;
  virtual std::vector<RtpCodecCapability> codec_preferences() const = 0;

  // Returns the set of header extensions that was set
  // with SetHeaderExtensionsToNegotiate, or a default set if it has not been
  // called.
  // https://w3c.github.io/webrtc-extensions/#rtcrtptransceiver-interface
  virtual std::vector<RtpHeaderExtensionCapability>
  GetHeaderExtensionsToNegotiate() const = 0;

  // Returns either the empty set if negotation has not yet
  // happened, or a vector of the negotiated header extensions.
  // https://w3c.github.io/webrtc-extensions/#rtcrtptransceiver-interface
  virtual std::vector<RtpHeaderExtensionCapability>
  GetNegotiatedHeaderExtensions() const = 0;

  // The SetHeaderExtensionsToNegotiate method modifies the next SDP negotiation
  // so that it negotiates use of header extensions which are not kStopped.
  // https://w3c.github.io/webrtc-extensions/#rtcrtptransceiver-interface
  virtual webrtc::RTCError SetHeaderExtensionsToNegotiate(
      ArrayView<const RtpHeaderExtensionCapability> header_extensions) = 0;

 protected:
  ~RtpTransceiverInterface() override = default;
};

}  // namespace webrtc

#endif  // API_RTP_TRANSCEIVER_INTERFACE_H_
