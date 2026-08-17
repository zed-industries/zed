/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_TEST_PACKETLOSSTEST_H_
#define MODULES_AUDIO_CODING_TEST_PACKETLOSSTEST_H_

#include <string>

#include "absl/strings/string_view.h"
#include "modules/audio_coding/test/EncodeDecodeTest.h"

namespace webrtc {

class ReceiverWithPacketLoss : public Receiver {
 public:
  ReceiverWithPacketLoss();
  void Setup(NetEq* neteq,
             RTPStream* rtpStream,
             absl::string_view out_file_name,
             int channels,
             int file_num,
             int loss_rate,
             int burst_length);
  bool IncomingPacket() override;

 protected:
  bool PacketLost();
  int loss_rate_;
  int burst_length_;
  int packet_counter_;
  int lost_packet_counter_;
  int burst_lost_counter_;
};

class SenderWithFEC : public Sender {
 public:
  SenderWithFEC();
  void Setup(const Environment& env,
             AudioCodingModule* acm,
             RTPStream* rtpStream,
             absl::string_view in_file_name,
             int payload_type,
             SdpAudioFormat format,
             int expected_loss_rate);
  bool SetPacketLossRate(int expected_loss_rate);
  bool SetFEC(bool enable_fec);

 protected:
  int expected_loss_rate_;
};

class PacketLossTest {
 public:
  PacketLossTest(int channels,
                 int expected_loss_rate_,
                 int actual_loss_rate,
                 int burst_length);
  void Perform();

 protected:
  int channels_;
  std::string in_file_name_;
  int sample_rate_hz_;
  int expected_loss_rate_;
  int actual_loss_rate_;
  int burst_length_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_TEST_PACKETLOSSTEST_H_
