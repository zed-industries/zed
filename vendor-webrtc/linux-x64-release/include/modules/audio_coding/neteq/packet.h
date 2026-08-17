/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_PACKET_H_
#define MODULES_AUDIO_CODING_NETEQ_PACKET_H_

#include <stdint.h>

#include <list>
#include <memory>

#include "api/audio_codecs/audio_decoder.h"
#include "api/neteq/tick_timer.h"
#include "api/rtp_packet_info.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Struct for holding RTP packets.
struct Packet {
  struct Priority {
    Priority() : codec_level(0), red_level(0) {}
    Priority(int codec_level, int red_level)
        : codec_level(codec_level), red_level(red_level) {
      CheckInvariant();
    }

    int codec_level;
    int red_level;

    // Priorities are sorted low-to-high, first on the level the codec
    // prioritizes it, then on the level of RED packet it is; i.e. if it is a
    // primary or secondary payload of a RED packet. For example: with Opus, an
    // Fec packet (which the decoder prioritizes lower than a regular packet)
    // will not be used if there is _any_ RED payload for the same
    // timeframe. The highest priority packet will have levels {0, 0}. Negative
    // priorities are not allowed.
    bool operator<(const Priority& b) const {
      CheckInvariant();
      b.CheckInvariant();
      if (codec_level == b.codec_level)
        return red_level < b.red_level;

      return codec_level < b.codec_level;
    }
    bool operator==(const Priority& b) const {
      CheckInvariant();
      b.CheckInvariant();
      return codec_level == b.codec_level && red_level == b.red_level;
    }
    bool operator!=(const Priority& b) const { return !(*this == b); }
    bool operator>(const Priority& b) const { return b < *this; }
    bool operator<=(const Priority& b) const { return !(b > *this); }
    bool operator>=(const Priority& b) const { return !(b < *this); }

   private:
    void CheckInvariant() const {
      RTC_DCHECK_GE(codec_level, 0);
      RTC_DCHECK_GE(red_level, 0);
    }
  };

  uint32_t timestamp;
  uint16_t sequence_number;
  uint8_t payload_type;
  // Datagram excluding RTP header and header extension.
  Buffer payload;
  Priority priority;
  std::optional<RtpPacketInfo> packet_info;
  std::unique_ptr<TickTimer::Stopwatch> waiting_time;
  std::unique_ptr<AudioDecoder::EncodedAudioFrame> frame;

  Packet();
  Packet(Packet&& b);
  ~Packet();

  // Packets should generally be moved around but sometimes it's useful to make
  // a copy, for example for testing purposes. NOTE: Will only work for
  // un-parsed packets, i.e. `frame` must be unset. The payload will, however,
  // be copied. `waiting_time` will also not be copied.
  Packet Clone() const;

  Packet& operator=(Packet&& b);

  // Comparison operators. Establish a packet ordering based on (1) timestamp,
  // (2) sequence number and (3) redundancy.
  // Timestamp and sequence numbers are compared taking wrap-around into
  // account. For two packets with the same sequence number and timestamp a
  // primary payload is considered "smaller" than a secondary.
  bool operator==(const Packet& rhs) const {
    return (this->timestamp == rhs.timestamp &&
            this->sequence_number == rhs.sequence_number &&
            this->priority == rhs.priority);
  }
  bool operator!=(const Packet& rhs) const { return !operator==(rhs); }
  bool operator<(const Packet& rhs) const {
    if (this->timestamp == rhs.timestamp) {
      if (this->sequence_number == rhs.sequence_number) {
        // Timestamp and sequence numbers are identical - deem the left hand
        // side to be "smaller" (i.e., "earlier") if it has higher priority.
        return this->priority < rhs.priority;
      }
      return (static_cast<uint16_t>(rhs.sequence_number -
                                    this->sequence_number) < 0xFFFF / 2);
    }
    return (static_cast<uint32_t>(rhs.timestamp - this->timestamp) <
            0xFFFFFFFF / 2);
  }
  bool operator>(const Packet& rhs) const { return rhs.operator<(*this); }
  bool operator<=(const Packet& rhs) const { return !operator>(rhs); }
  bool operator>=(const Packet& rhs) const { return !operator<(rhs); }

  bool empty() const { return !frame && payload.empty(); }
};

// A list of packets.
typedef std::list<Packet> PacketList;

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_PACKET_H_
