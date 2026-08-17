// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TRANSCEIVER_STATE_SURFACER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TRANSCEIVER_STATE_SURFACER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_transceiver_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_peer_connection_handler_client.h"
#include "third_party/webrtc/api/rtp_transceiver_interface.h"
#include "third_party/webrtc/api/sctp_transport_interface.h"
#include "third_party/webrtc/rtc_base/ref_count.h"
#include "third_party/webrtc/rtc_base/ref_counted_object.h"

namespace blink {

// Takes care of creating and initializing transceiver states (including sender
// and receiver states). This is usable for both blocking and non-blocking calls
// to the webrtc signaling thread.
//
// The surfacer is initialized on the signaling thread and states are obtained
// on the main thread. It is designed to be initialized and handed off; it is
// not thread safe for concurrent thread usage.
class MODULES_EXPORT TransceiverStateSurfacer {
 public:
  TransceiverStateSurfacer(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner);
  TransceiverStateSurfacer(TransceiverStateSurfacer&&);
  TransceiverStateSurfacer(const TransceiverStateSurfacer&) = delete;
  ~TransceiverStateSurfacer();

  TransceiverStateSurfacer& operator=(TransceiverStateSurfacer&&) = delete;
  TransceiverStateSurfacer& operator=(const TransceiverStateSurfacer&) = delete;

  bool is_initialized() const { return is_initialized_; }

  // Must be invoked on the signaling thread.
  void Initialize(
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface>
          native_peer_connection,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      std::vector<webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>>
          transceivers);

  // Must be invoked on the main thread.
  blink::WebRTCSctpTransportSnapshot SctpTransportSnapshot();
  std::vector<blink::RtpTransceiverState> ObtainStates();

 private:
  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;
  bool is_initialized_;
  blink::WebRTCSctpTransportSnapshot sctp_transport_snapshot_;
  std::vector<blink::RtpTransceiverState> transceiver_states_
      ALLOW_DISCOURAGED_TYPE(
          "Avoids conversions when passed from/to webrtc API");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TRANSCEIVER_STATE_SURFACER_H_
