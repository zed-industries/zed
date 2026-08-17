/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_DEPRECATED_STREAM_GENERATOR_H_
#define MODULES_VIDEO_CODING_DEPRECATED_STREAM_GENERATOR_H_

#include <stdint.h>

#include <list>

#include "api/video/video_frame_type.h"
#include "modules/video_coding/deprecated/packet.h"

namespace webrtc {

const unsigned int kDefaultBitrateKbps = 1000;
const unsigned int kDefaultFrameRate = 25;
const unsigned int kMaxPacketSize = 1500;
const unsigned int kFrameSize =
    (kDefaultBitrateKbps + kDefaultFrameRate * 4) / (kDefaultFrameRate * 8);
const int kDefaultFramePeriodMs = 1000 / kDefaultFrameRate;

class StreamGenerator {
 public:
  StreamGenerator(uint16_t start_seq_num, int64_t current_time);

  StreamGenerator(const StreamGenerator&) = delete;
  StreamGenerator& operator=(const StreamGenerator&) = delete;

  void Init(uint16_t start_seq_num, int64_t current_time);

  // `time_ms` denotes the timestamp you want to put on the frame, and the unit
  // is millisecond. GenerateFrame will translate `time_ms` into a 90kHz
  // timestamp and put it on the frame.
  void GenerateFrame(VideoFrameType type,
                     int num_media_packets,
                     int num_empty_packets,
                     int64_t time_ms);

  bool PopPacket(VCMPacket* packet, int index);
  void DropLastPacket();

  bool GetPacket(VCMPacket* packet, int index);

  bool NextPacket(VCMPacket* packet);

  uint16_t NextSequenceNumber() const;

  int PacketsRemaining() const;

 private:
  VCMPacket GeneratePacket(uint16_t sequence_number,
                           uint32_t timestamp,
                           unsigned int size,
                           bool first_packet,
                           bool marker_bit,
                           VideoFrameType type);

  std::list<VCMPacket>::iterator GetPacketIterator(int index);

  std::list<VCMPacket> packets_;
  uint16_t sequence_number_;
  int64_t start_time_;
  uint8_t packet_buffer_[kMaxPacketSize];
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_STREAM_GENERATOR_H_
