/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_CHANNEL_INTERFACE_H_
#define PC_CHANNEL_INTERFACE_H_

#include <functional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/jsep.h"
#include "api/media_types.h"
#include "media/base/media_channel.h"
#include "media/base/media_config.h"
#include "media/base/stream_params.h"
#include "pc/rtp_transport_internal.h"
#include "pc/session_description.h"

namespace webrtc {
class Call;
class VideoBitrateAllocatorFactory;
class VideoChannel;
class VoiceChannel;
}  // namespace webrtc

namespace webrtc {

// A Channel is a construct that groups media streams of the same type
// (audio or video), both outgoing and incoming.
// When the PeerConnection API is used, a Channel corresponds one to one
// to an RtpTransceiver.
// When Unified Plan is used, there can only be at most one outgoing and
// one incoming stream. With Plan B, there can be more than one.

// ChannelInterface contains methods common to voice and video channels.
// As more methods are added to BaseChannel, they should be included in the
// interface as well.
// TODO(bugs.webrtc.org/13931): Merge this class into RtpTransceiver.
class ChannelInterface {
 public:
  virtual ~ChannelInterface() = default;
  virtual MediaType media_type() const = 0;

  virtual VideoChannel* AsVideoChannel() = 0;
  virtual VoiceChannel* AsVoiceChannel() = 0;

  virtual MediaSendChannelInterface* media_send_channel() = 0;
  // Typecasts of media_channel(). Will cause an exception if the
  // channel is of the wrong type.
  virtual VideoMediaSendChannelInterface* video_media_send_channel() = 0;
  virtual VoiceMediaSendChannelInterface* voice_media_send_channel() = 0;
  virtual MediaReceiveChannelInterface* media_receive_channel() = 0;
  // Typecasts of media_channel(). Will cause an exception if the
  // channel is of the wrong type.
  virtual VideoMediaReceiveChannelInterface* video_media_receive_channel() = 0;
  virtual VoiceMediaReceiveChannelInterface* voice_media_receive_channel() = 0;

  // Returns a string view for the transport name. Fetching the transport name
  // must be done on the network thread only and note that the lifetime of
  // the returned object should be assumed to only be the calling scope.
  // TODO(deadbeef): This is redundant; remove this.
  virtual absl::string_view transport_name() const = 0;

  // TODO(tommi): Change return type to string_view.
  virtual const std::string& mid() const = 0;

  // Enables or disables this channel
  virtual void Enable(bool enable) = 0;

  // Used for latency measurements.
  virtual void SetFirstPacketReceivedCallback(
      std::function<void()> callback) = 0;
  virtual void SetFirstPacketSentCallback(std::function<void()> callback) = 0;

  // Channel control
  virtual bool SetLocalContent(const MediaContentDescription* content,
                               SdpType type,
                               std::string& error_desc) = 0;
  virtual bool SetRemoteContent(const MediaContentDescription* content,
                                SdpType type,
                                std::string& error_desc) = 0;
  virtual bool SetPayloadTypeDemuxingEnabled(bool enabled) = 0;

  // Access to the local and remote streams that were set on the channel.
  virtual const std::vector<StreamParams>& local_streams() const = 0;
  virtual const std::vector<StreamParams>& remote_streams() const = 0;

  // Set an RTP level transport.
  // Some examples:
  //   * An RtpTransport without encryption.
  //   * An SrtpTransport for SDES.
  //   * A DtlsSrtpTransport for DTLS-SRTP.
  virtual bool SetRtpTransport(RtpTransportInternal* rtp_transport) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::ChannelInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_CHANNEL_INTERFACE_H_
