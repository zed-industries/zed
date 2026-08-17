/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_PACKET_BUFFER_H_
#define MODULES_AUDIO_CODING_NETEQ_PACKET_BUFFER_H_

#include <optional>

#include "modules/audio_coding/neteq/decoder_database.h"
#include "modules/audio_coding/neteq/packet.h"
#include "modules/include/module_common_types_public.h"  // IsNewerTimestamp

namespace webrtc {

class DecoderDatabase;
class StatisticsCalculator;
class TickTimer;

// This is the actual buffer holding the packets before decoding.
class PacketBuffer {
 public:
  enum BufferReturnCodes {
    kOK = 0,
    kFlushed,
    kNotFound,
    kBufferEmpty,
    kInvalidPacket,
    kInvalidPointer
  };

  // Constructor creates a buffer which can hold a maximum of
  // `max_number_of_packets` packets.
  PacketBuffer(size_t max_number_of_packets,
               const TickTimer* tick_timer,
               StatisticsCalculator* stats);

  // Deletes all packets in the buffer before destroying the buffer.
  virtual ~PacketBuffer();

  PacketBuffer(const PacketBuffer&) = delete;
  PacketBuffer& operator=(const PacketBuffer&) = delete;

  // Flushes the buffer and deletes all packets in it.
  virtual void Flush();

  // Returns true for an empty buffer.
  virtual bool Empty() const;

  // Inserts `packet` into the buffer. The buffer will take over ownership of
  // the packet object.
  // Returns PacketBuffer::kOK on success, PacketBuffer::kFlushed if the buffer
  // was flushed due to overfilling.
  virtual int InsertPacket(Packet&& packet);

  // Gets the timestamp for the first packet in the buffer and writes it to the
  // output variable `next_timestamp`.
  // Returns PacketBuffer::kBufferEmpty if the buffer is empty,
  // PacketBuffer::kOK otherwise.
  virtual int NextTimestamp(uint32_t* next_timestamp) const;

  // Gets the timestamp for the first packet in the buffer with a timestamp no
  // lower than the input limit `timestamp`. The result is written to the output
  // variable `next_timestamp`.
  // Returns PacketBuffer::kBufferEmpty if the buffer is empty,
  // PacketBuffer::kOK otherwise.
  virtual int NextHigherTimestamp(uint32_t timestamp,
                                  uint32_t* next_timestamp) const;

  // Returns a (constant) pointer to the first packet in the buffer. Returns
  // NULL if the buffer is empty.
  virtual const Packet* PeekNextPacket() const;

  // Extracts the first packet in the buffer and returns it.
  // Returns an empty optional if the buffer is empty.
  virtual std::optional<Packet> GetNextPacket();

  // Discards the first packet in the buffer. The packet is deleted.
  // Returns PacketBuffer::kBufferEmpty if the buffer is empty,
  // PacketBuffer::kOK otherwise.
  virtual int DiscardNextPacket();

  // Discards all packets that are (strictly) older than timestamp_limit,
  // but newer than timestamp_limit - horizon_samples. Setting horizon_samples
  // to zero implies that the horizon is set to half the timestamp range. That
  // is, if a packet is more than 2^31 timestamps into the future compared with
  // timestamp_limit (including wrap-around), it is considered old.
  virtual void DiscardOldPackets(uint32_t timestamp_limit,
                                 uint32_t horizon_samples);

  // Discards all packets that are (strictly) older than timestamp_limit.
  virtual void DiscardAllOldPackets(uint32_t timestamp_limit);

  // Removes all packets with a specific payload type from the buffer.
  virtual void DiscardPacketsWithPayloadType(uint8_t payload_type);

  // Returns the number of packets in the buffer, including duplicates and
  // redundant packets.
  virtual size_t NumPacketsInBuffer() const;

  // Returns the number of samples in the buffer, including samples carried in
  // duplicate and redundant packets.
  virtual size_t NumSamplesInBuffer(size_t last_decoded_length) const;

  // Returns the total duration in samples that the packets in the buffer spans
  // across.
  virtual size_t GetSpanSamples(size_t last_decoded_length,
                                size_t sample_rate,
                                bool count_waiting_time) const;

  // Returns true if the packet buffer contains any DTX or CNG packets.
  virtual bool ContainsDtxOrCngPacket(
      const DecoderDatabase* decoder_database) const;

  // Static method returning true if `timestamp` is older than `timestamp_limit`
  // but less than `horizon_samples` behind `timestamp_limit`. For instance,
  // with timestamp_limit = 100 and horizon_samples = 10, a timestamp in the
  // range (90, 100) is considered obsolete, and will yield true.
  // Setting `horizon_samples` to 0 is the same as setting it to 2^31, i.e.,
  // half the 32-bit timestamp range.
  static bool IsObsoleteTimestamp(uint32_t timestamp,
                                  uint32_t timestamp_limit,
                                  uint32_t horizon_samples) {
    return IsNewerTimestamp(timestamp_limit, timestamp) &&
           (horizon_samples == 0 ||
            IsNewerTimestamp(timestamp, timestamp_limit - horizon_samples));
  }

 private:
  void LogPacketDiscarded(int codec_level);

  size_t max_number_of_packets_;
  PacketList buffer_;
  const TickTimer* tick_timer_;
  StatisticsCalculator* stats_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_PACKET_BUFFER_H_
