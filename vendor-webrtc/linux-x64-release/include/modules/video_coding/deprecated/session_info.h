/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_DEPRECATED_SESSION_INFO_H_
#define MODULES_VIDEO_CODING_DEPRECATED_SESSION_INFO_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <vector>

#include "api/video/video_frame_type.h"
#include "modules/video_coding/codecs/h264/include/h264_globals.h"
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "modules/video_coding/deprecated/packet.h"

namespace webrtc {
// Used to pass data from jitter buffer to session info.
// This data is then used in determining whether a frame is decodable.
struct FrameData {
  int64_t rtt_ms;
  float rolling_average_packets_per_frame;
};

class VCMSessionInfo {
 public:
  VCMSessionInfo();
  ~VCMSessionInfo();

  void UpdateDataPointers(const uint8_t* old_base_ptr,
                          const uint8_t* new_base_ptr);
  void Reset();
  int InsertPacket(const VCMPacket& packet,
                   uint8_t* frame_buffer,
                   const FrameData& frame_data);
  bool complete() const;

  // Makes the frame decodable. I.e., only contain decodable NALUs. All
  // non-decodable NALUs will be deleted and packets will be moved to in
  // memory to remove any empty space.
  // Returns the number of bytes deleted from the session.
  size_t MakeDecodable();

  size_t SessionLength() const;
  int NumPackets() const;
  bool HaveFirstPacket() const;
  bool HaveLastPacket() const;
  webrtc::VideoFrameType FrameType() const { return frame_type_; }
  int LowSequenceNumber() const;

  // Returns highest sequence number, media or empty.
  int HighSequenceNumber() const;
  int PictureId() const;
  int TemporalId() const;
  bool LayerSync() const;
  int Tl0PicId() const;

  std::vector<NaluInfo> GetNaluInfos() const;

  void SetGofInfo(const GofInfoVP9& gof_info, size_t idx);

 private:
  enum { kMaxVP8Partitions = 9 };

  typedef std::list<VCMPacket> PacketList;
  typedef PacketList::iterator PacketIterator;
  typedef PacketList::const_iterator PacketIteratorConst;
  typedef PacketList::reverse_iterator ReversePacketIterator;

  void InformOfEmptyPacket(uint16_t seq_num);

  // Finds the packet of the beginning of the next VP8 partition. If
  // none is found the returned iterator points to `packets_.end()`.
  // `it` is expected to point to the last packet of the previous partition,
  // or to the first packet of the frame. `packets_skipped` is incremented
  // for each packet found which doesn't have the beginning bit set.
  PacketIterator FindNextPartitionBeginning(PacketIterator it) const;

  // Returns an iterator pointing to the last packet of the partition pointed to
  // by `it`.
  PacketIterator FindPartitionEnd(PacketIterator it) const;
  static bool InSequence(const PacketIterator& it,
                         const PacketIterator& prev_it);
  size_t InsertBuffer(uint8_t* frame_buffer, PacketIterator packetIterator);
  size_t Insert(const uint8_t* buffer,
                size_t length,
                bool insert_start_code,
                uint8_t* frame_buffer);
  void ShiftSubsequentPackets(PacketIterator it, int steps_to_shift);
  PacketIterator FindNaluEnd(PacketIterator packet_iter) const;
  // Deletes the data of all packets between `start` and `end`, inclusively.
  // Note that this function doesn't delete the actual packets.
  size_t DeletePacketData(PacketIterator start, PacketIterator end);
  void UpdateCompleteSession();

  bool complete_;
  webrtc::VideoFrameType frame_type_;
  // Packets in this frame.
  PacketList packets_;
  int empty_seq_num_low_;
  int empty_seq_num_high_;

  // The following two variables correspond to the first and last media packets
  // in a session defined by the first packet flag and the marker bit.
  // They are not necessarily equal to the front and back packets, as packets
  // may enter out of order.
  // TODO(mikhal): Refactor the list to use a map.
  int first_packet_seq_num_;
  int last_packet_seq_num_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_SESSION_INFO_H_
