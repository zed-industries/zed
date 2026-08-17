/*
 *  Copyright 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_FACTORY_PROXY_H_
#define PC_PEER_CONNECTION_FACTORY_PROXY_H_

#include <cstdint>
#include <cstdio>
#include <string>

#include "absl/strings/string_view.h"
#include "api/audio_options.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "pc/proxy.h"

namespace webrtc {

// TODO(deadbeef): Move this to .cc file. What threads methods are called on is
// an implementation detail.
BEGIN_PROXY_MAP(PeerConnectionFactory)
PROXY_PRIMARY_THREAD_DESTRUCTOR()
PROXY_METHOD1(void, SetOptions, const Options&)
PROXY_METHOD2(RTCErrorOr<webrtc::scoped_refptr<PeerConnectionInterface>>,
              CreatePeerConnectionOrError,
              const PeerConnectionInterface::RTCConfiguration&,
              PeerConnectionDependencies)
PROXY_CONSTMETHOD1(RtpCapabilities, GetRtpSenderCapabilities, webrtc::MediaType)
PROXY_CONSTMETHOD1(RtpCapabilities,
                   GetRtpReceiverCapabilities,
                   webrtc::MediaType)
PROXY_METHOD1(scoped_refptr<MediaStreamInterface>,
              CreateLocalMediaStream,
              const std::string&)
PROXY_METHOD1(scoped_refptr<AudioSourceInterface>,
              CreateAudioSource,
              const AudioOptions&)
PROXY_METHOD2(scoped_refptr<VideoTrackInterface>,
              CreateVideoTrack,
              scoped_refptr<VideoTrackSourceInterface>,
              absl::string_view)
PROXY_METHOD2(scoped_refptr<AudioTrackInterface>,
              CreateAudioTrack,
              const std::string&,
              AudioSourceInterface*)
PROXY_SECONDARY_METHOD2(bool, StartAecDump, FILE*, int64_t)
PROXY_SECONDARY_METHOD0(void, StopAecDump)
END_PROXY_MAP(PeerConnectionFactory)

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_FACTORY_PROXY_H_
