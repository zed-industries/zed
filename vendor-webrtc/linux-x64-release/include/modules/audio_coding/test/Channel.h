/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_TEST_CHANNEL_H_
#define MODULES_AUDIO_CODING_TEST_CHANNEL_H_

#include <stdio.h>

#include "api/neteq/neteq.h"
#include "modules/audio_coding/include/audio_coding_module.h"
#include "modules/include/module_common_types.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

#define MAX_NUM_PAYLOADS 50
#define MAX_NUM_FRAMESIZES 6

// TODO(turajs): Write constructor for this structure.
struct ACMTestFrameSizeStats {
  uint16_t frameSizeSample;
  size_t maxPayloadLen;
  uint32_t numPackets;
  uint64_t totalPayloadLenByte;
  uint64_t totalEncodedSamples;
  double rateBitPerSec;
  double usageLenSec;
};

// TODO(turajs): Write constructor for this structure.
struct ACMTestPayloadStats {
  bool newPacket;
  int16_t payloadType;
  size_t lastPayloadLenByte;
  uint32_t lastTimestamp;
  ACMTestFrameSizeStats frameSizeStats[MAX_NUM_FRAMESIZES];
};

class Channel : public AudioPacketizationCallback {
 public:
  Channel(int16_t chID = -1);
  ~Channel() override;

  int32_t SendData(AudioFrameType frameType,
                   uint8_t payloadType,
                   uint32_t timeStamp,
                   const uint8_t* payloadData,
                   size_t payloadSize,
                   int64_t absolute_capture_timestamp_ms) override;

  void RegisterReceiverNetEq(NetEq* neteq);

  void ResetStats();

  void SetIsStereo(bool isStereo) { _isStereo = isStereo; }

  uint32_t LastInTimestamp();

  void SetFECTestWithPacketLoss(bool usePacketLoss) {
    _useFECTestWithPacketLoss = usePacketLoss;
  }

  double BitRate();

  void set_send_timestamp(uint32_t new_send_ts) {
    external_send_timestamp_ = new_send_ts;
  }

  void set_sequence_number(uint16_t new_sequence_number) {
    external_sequence_number_ = new_sequence_number;
  }

  void set_num_packets_to_drop(int new_num_packets_to_drop) {
    num_packets_to_drop_ = new_num_packets_to_drop;
  }

 private:
  void CalcStatistics(const RTPHeader& rtp_header, size_t payloadSize);

  NetEq* _neteq;
  uint16_t _seqNo;
  // 60msec * 32 sample(max)/msec * 2 description (maybe) * 2 bytes/sample
  uint8_t _payloadData[60 * 32 * 2 * 2];

  Mutex _channelCritSect;
  FILE* _bitStreamFile;
  bool _saveBitStream;
  int16_t _lastPayloadType;
  ACMTestPayloadStats _payloadStats[MAX_NUM_PAYLOADS];
  bool _isStereo;
  RTPHeader _rtp_header;
  bool _leftChannel;
  uint32_t _lastInTimestamp;
  bool _useLastFrameSize;
  uint32_t _lastFrameSizeSample;
  // FEC Test variables
  int16_t _packetLoss;
  bool _useFECTestWithPacketLoss;
  uint64_t _beginTime;
  uint64_t _totalBytes;

  // External timing info, defaulted to -1. Only used if they are
  // non-negative.
  int64_t external_send_timestamp_;
  int32_t external_sequence_number_;
  int num_packets_to_drop_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_TEST_CHANNEL_H_
