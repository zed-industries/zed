/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_END_TO_END_TESTS_MULTI_STREAM_TESTER_H_
#define VIDEO_END_TO_END_TESTS_MULTI_STREAM_TESTER_H_

#include <map>
#include <memory>

#include "api/task_queue/task_queue_base.h"
#include "call/call.h"
#include "test/direct_transport.h"
#include "test/frame_generator_capturer.h"

namespace webrtc {
// Test sets up a Call multiple senders with different resolutions and SSRCs.
// Another is set up to receive all three of these with different renderers.
class MultiStreamTester {
 public:
  static constexpr size_t kNumStreams = 3;
  const uint8_t kVideoPayloadType = 124;
  const std::map<uint8_t, MediaType> payload_type_map_ = {
      {kVideoPayloadType, MediaType::VIDEO}};

  struct CodecSettings {
    uint32_t ssrc;
    int width;
    int height;
  } codec_settings[kNumStreams];

  MultiStreamTester();

  virtual ~MultiStreamTester();

  void RunTest();

 protected:
  virtual void Wait() = 0;
  // Note: frame_generator is a point-to-pointer, since the actual instance
  // hasn't been created at the time of this call. Only when packets/frames
  // start flowing should this be dereferenced.
  virtual void UpdateSendConfig(size_t stream_index,
                                VideoSendStream::Config* send_config,
                                VideoEncoderConfig* encoder_config,
                                test::FrameGeneratorCapturer** frame_generator);
  virtual void UpdateReceiveConfig(
      size_t stream_index,
      VideoReceiveStreamInterface::Config* receive_config);
  virtual std::unique_ptr<test::DirectTransport> CreateSendTransport(
      TaskQueueBase* task_queue,
      Call* sender_call);
  virtual std::unique_ptr<test::DirectTransport> CreateReceiveTransport(
      TaskQueueBase* task_queue,
      Call* receiver_call);
};
}  // namespace webrtc
#endif  // VIDEO_END_TO_END_TESTS_MULTI_STREAM_TESTER_H_
