/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_INCLUDE_AUDIO_CODING_MODULE_H_
#define MODULES_AUDIO_CODING_INCLUDE_AUDIO_CODING_MODULE_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "api/audio_codecs/audio_encoder.h"
#include "api/function_view.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"

namespace webrtc {

// forward declarations
class AudioDecoder;
class AudioEncoder;
class AudioFrame;
struct RTPHeader;

// Callback class used for sending data ready to be packetized
class AudioPacketizationCallback {
 public:
  virtual ~AudioPacketizationCallback() {}

  virtual int32_t SendData(AudioFrameType frame_type,
                           uint8_t payload_type,
                           uint32_t timestamp,
                           const uint8_t* payload_data,
                           size_t payload_len_bytes,
                           int64_t /* absolute_capture_timestamp_ms */) {
    // TODO(bugs.webrtc.org/10739): Deprecate the old SendData and make this one
    // pure virtual.
    return SendData(frame_type, payload_type, timestamp, payload_data,
                    payload_len_bytes);
  }
  virtual int32_t SendData(AudioFrameType /* frame_type */,
                           uint8_t /* payload_type */,
                           uint32_t /* timestamp */,
                           const uint8_t* /* payload_data */,
                           size_t /* payload_len_bytes */) {
    RTC_DCHECK_NOTREACHED() << "This method must be overridden, or not used.";
    return -1;
  }
};

class AudioCodingModule {
 protected:
  AudioCodingModule() {}

 public:
  static std::unique_ptr<AudioCodingModule> Create();
  virtual ~AudioCodingModule() = default;

  // `modifier` is called exactly once with one argument: a pointer to the
  // unique_ptr that holds the current encoder (which is null if there is no
  // current encoder). For the duration of the call, `modifier` has exclusive
  // access to the unique_ptr; it may call the encoder, steal the encoder and
  // replace it with another encoder or with nullptr, etc.
  virtual void ModifyEncoder(
      FunctionView<void(std::unique_ptr<AudioEncoder>*)> modifier) = 0;

  // Utility method for simply replacing the existing encoder with a new one.
  void SetEncoder(std::unique_ptr<AudioEncoder> new_encoder) {
    ModifyEncoder([&](std::unique_ptr<AudioEncoder>* encoder) {
      *encoder = std::move(new_encoder);
    });
  }

  // Reset encoder and audio coding module. This throws away any audio passed
  // and starts fresh.
  virtual void Reset() = 0;

  // int32_t RegisterTransportCallback()
  // Register a transport callback which will be called to deliver
  // the encoded buffers whenever Process() is called and a
  // bit-stream is ready.
  //
  // Input:
  //   -transport          : pointer to the callback class
  //                         transport->SendData() is called whenever
  //                         Process() is called and bit-stream is ready
  //                         to deliver.
  //
  // Return value:
  //   -1 if the transport callback could not be registered
  //    0 if registration is successful.
  //
  virtual int32_t RegisterTransportCallback(
      AudioPacketizationCallback* transport) = 0;

  ///////////////////////////////////////////////////////////////////////////
  // int32_t Add10MsData()
  // Add 10MS of raw (PCM) audio data and encode it. If the sampling
  // frequency of the audio does not match the sampling frequency of the
  // current encoder ACM will resample the audio. If an encoded packet was
  // produced, it will be delivered via the callback object registered using
  // RegisterTransportCallback, and the return value from this function will
  // be the number of bytes encoded.
  //
  // Input:
  //   -audio_frame        : the input audio frame, containing raw audio
  //                         sampling frequency etc.
  //
  // Return value:
  //   >= 0   number of bytes encoded.
  //     -1   some error occurred.
  //
  virtual int32_t Add10MsData(const AudioFrame& audio_frame) = 0;

  ///////////////////////////////////////////////////////////////////////////
  // int SetPacketLossRate()
  // Sets expected packet loss rate for encoding. Some encoders provide packet
  // loss gnostic encoding to make stream less sensitive to packet losses,
  // through e.g., FEC. No effects on codecs that do not provide such encoding.
  //
  // Input:
  //   -packet_loss_rate   : expected packet loss rate (0 -- 100 inclusive).
  //
  // Return value
  //   -1 if failed to set packet loss rate,
  //   0 if succeeded.
  //
  // This is only used in test code that rely on old ACM APIs.
  // TODO(minyue): Remove it when possible.
  virtual int SetPacketLossRate(int packet_loss_rate) = 0;

  ///////////////////////////////////////////////////////////////////////////
  //   statistics
  //

  virtual ANAStats GetANAStats() const = 0;

  virtual int GetTargetBitrate() const = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_INCLUDE_AUDIO_CODING_MODULE_H_
