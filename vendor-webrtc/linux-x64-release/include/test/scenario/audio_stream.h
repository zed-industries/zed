/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_AUDIO_STREAM_H_
#define TEST_SCENARIO_AUDIO_STREAM_H_
#include <memory>
#include <string>
#include <vector>

#include "test/scenario/call_client.h"
#include "test/scenario/column_printer.h"
#include "test/scenario/network_node.h"
#include "test/scenario/scenario_config.h"

namespace webrtc {
namespace test {

// SendAudioStream represents sending of audio. It can be used for starting the
// stream if neccessary.
class SendAudioStream {
 public:
  ~SendAudioStream();

  SendAudioStream(const SendAudioStream&) = delete;
  SendAudioStream& operator=(const SendAudioStream&) = delete;

  void Start();
  void Stop();
  void SetMuted(bool mute);
  ColumnPrinter StatsPrinter();

 private:
  friend class Scenario;
  friend class AudioStreamPair;
  friend class ReceiveAudioStream;
  SendAudioStream(CallClient* sender,
                  AudioStreamConfig config,
                  scoped_refptr<AudioEncoderFactory> encoder_factory,
                  Transport* send_transport);
  AudioSendStream* send_stream_ = nullptr;
  CallClient* const sender_;
  const AudioStreamConfig config_;
  uint32_t ssrc_;
};

// ReceiveAudioStream represents an audio receiver. It can't be used directly.
class ReceiveAudioStream {
 public:
  ~ReceiveAudioStream();

  ReceiveAudioStream(const ReceiveAudioStream&) = delete;
  ReceiveAudioStream& operator=(const ReceiveAudioStream&) = delete;

  void Start();
  void Stop();
  AudioReceiveStreamInterface::Stats GetStats() const;

 private:
  friend class Scenario;
  friend class AudioStreamPair;
  ReceiveAudioStream(CallClient* receiver,
                     AudioStreamConfig config,
                     SendAudioStream* send_stream,
                     scoped_refptr<AudioDecoderFactory> decoder_factory,
                     Transport* feedback_transport);
  AudioReceiveStreamInterface* receive_stream_ = nullptr;
  CallClient* const receiver_;
  const AudioStreamConfig config_;
};

// AudioStreamPair represents an audio streaming session. It can be used to
// access underlying send and receive classes. It can also be used in calls to
// the Scenario class.
class AudioStreamPair {
 public:
  ~AudioStreamPair();

  AudioStreamPair(const AudioStreamPair&) = delete;
  AudioStreamPair& operator=(const AudioStreamPair&) = delete;

  SendAudioStream* send() { return &send_stream_; }
  ReceiveAudioStream* receive() { return &receive_stream_; }

 private:
  friend class Scenario;
  AudioStreamPair(CallClient* sender,
                  scoped_refptr<AudioEncoderFactory> encoder_factory,
                  CallClient* receiver,
                  scoped_refptr<AudioDecoderFactory> decoder_factory,
                  AudioStreamConfig config);

 private:
  const AudioStreamConfig config_;
  SendAudioStream send_stream_;
  ReceiveAudioStream receive_stream_;
};

std::vector<RtpExtension> GetAudioRtpExtensions(
    const AudioStreamConfig& config);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCENARIO_AUDIO_STREAM_H_
