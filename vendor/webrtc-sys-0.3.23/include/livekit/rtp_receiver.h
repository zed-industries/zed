/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>

#include "api/peer_connection_interface.h"
#include "api/rtp_receiver_interface.h"
#include "api/scoped_refptr.h"
#include "livekit/helper.h"
#include "livekit/media_stream.h"
#include "livekit/rtp_parameters.h"
#include "livekit/webrtc.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class RtpReceiver;
}
#include "webrtc-sys/src/rtp_receiver.rs.h"
namespace livekit_ffi {

// TODO(theomonnom): Implement RtpReceiverObserverInterface?
// TODO(theomonnom): RtpSource
// TODO(theomonnom): FrameTransformer & FrameDecryptor interface
class RtpReceiver {
 public:
  RtpReceiver(
      std::shared_ptr<RtcRuntime> rtc_runtime,
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection);

  std::shared_ptr<MediaStreamTrack> track() const;

  void get_stats(
      rust::Box<ReceiverContext> ctx,
      rust::Fn<void(rust::Box<ReceiverContext>, rust::String)> on_stats) const;

  rust::Vec<rust::String> stream_ids() const;
  rust::Vec<MediaStreamPtr> streams() const;

  MediaType media_type() const;
  rust::String id() const;

  RtpParameters get_parameters() const;

  // bool set_parameters(RtpParameters parameters) const; // Seems unsupported

  void set_jitter_buffer_minimum_delay(bool is_some,
                                       double delay_seconds) const;

  webrtc::scoped_refptr<webrtc::RtpReceiverInterface> rtc_receiver() const {
    return receiver_;
  }

 private:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver_;
  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection_;
};

static std::shared_ptr<RtpReceiver> _shared_rtp_receiver() {
  return nullptr;
}

}  // namespace livekit_ffi
