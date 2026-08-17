/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RECEIVE_STREAM_H_
#define CALL_RECEIVE_STREAM_H_

#include <cstdint>
#include <vector>

#include "api/crypto/frame_decryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"

namespace webrtc {

// Common base interface for MediaReceiveStreamInterface based classes and
// FlexfecReceiveStream.
class ReceiveStreamInterface {
 public:
  // Receive-stream specific RTP settings.
  // TODO(tommi): This struct isn't needed at this level anymore. Move it closer
  // to where it's used.
  struct ReceiveStreamRtpConfig {
    // Synchronization source (stream identifier) to be received.
    // This member will not change mid-stream and can be assumed to be const
    // post initialization.
    uint32_t remote_ssrc = 0;

    // Sender SSRC used for sending RTCP (such as receiver reports).
    // This value may change mid-stream and must be done on the same thread
    // that the value is read on (i.e. packet delivery).
    uint32_t local_ssrc = 0;
  };

 protected:
  virtual ~ReceiveStreamInterface() {}
};

// Either an audio or video receive stream.
class MediaReceiveStreamInterface : public ReceiveStreamInterface {
 public:
  // Starts stream activity.
  // When a stream is active, it can receive, process and deliver packets.
  virtual void Start() = 0;

  // Stops stream activity. Must be called to match with a previous call to
  // `Start()`. When a stream has been stopped, it won't receive, decode,
  // process or deliver packets to downstream objects such as callback pointers
  // set in the config struct.
  virtual void Stop() = 0;

  virtual void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer) = 0;

  virtual void SetFrameDecryptor(
      scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor) = 0;

  virtual std::vector<RtpSource> GetSources() const = 0;

  virtual void SetRtcpMode(RtcpMode mode) = 0;
};

}  // namespace webrtc

#endif  // CALL_RECEIVE_STREAM_H_
