/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_FORWARD_ERROR_CORRECTION_H_
#define MODULES_RTP_RTCP_SOURCE_FORWARD_ERROR_CORRECTION_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <memory>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "api/array_view.h"
#include "api/scoped_refptr.h"
#include "modules/include/module_fec_types.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/source/forward_error_correction_internal.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

class FecHeaderReader;
class FecHeaderWriter;

// Performs codec-independent forward error correction (FEC), based on RFC 5109.
// Option exists to enable unequal protection (UEP) across packets.
// This is not to be confused with protection within packets
// (referred to as uneven level protection (ULP) in RFC 5109).
// TODO(brandtr): Split this class into a separate encoder
// and a separate decoder.
class ForwardErrorCorrection {
 public:
  // TODO(holmer): As a next step all these struct-like packet classes should be
  // refactored into proper classes, and their members should be made private.
  // This will require parts of the functionality in forward_error_correction.cc
  // and receiver_fec.cc to be refactored into the packet classes.
  class Packet {
   public:
    Packet();
    virtual ~Packet();

    // Add a reference.
    virtual int32_t AddRef();

    // Release a reference. Will delete the object if the reference count
    // reaches zero.
    virtual int32_t Release();

    CopyOnWriteBuffer data;  // Packet data.

   private:
    int32_t ref_count_;  // Counts the number of references to a packet.
  };

  // TODO(holmer): Refactor into a proper class.
  class SortablePacket {
   public:
    // Functor which returns true if the sequence number of `first`
    // is < the sequence number of `second`. Should only ever be called for
    // packets belonging to the same SSRC.
    struct LessThan {
      template <typename S, typename T>
      bool operator()(const S& first, const T& second);
    };

    uint32_t ssrc;
    uint16_t seq_num;
  };

  // Used for the input to DecodeFec().
  class ReceivedPacket : public SortablePacket {
   public:
    ReceivedPacket();
    ~ReceivedPacket();

    bool is_fec;  // Set to true if this is an FEC packet and false
                  // otherwise.
    bool is_recovered;
    RtpHeaderExtensionMap extensions;
    scoped_refptr<Packet> pkt;  // Pointer to the packet storage.
  };

  // The recovered list parameter of DecodeFec() references structs of
  // this type.
  // TODO(holmer): Refactor into a proper class.
  class RecoveredPacket : public SortablePacket {
   public:
    RecoveredPacket();
    ~RecoveredPacket();

    bool was_recovered;  // Will be true if this packet was recovered by
                         // the FEC. Otherwise it was a media packet passed in
                         // through the received packet list.
    bool returned;  // True when the packet already has been returned to the
                    // caller through the callback.
    scoped_refptr<Packet> pkt;  // Pointer to the packet storage.
  };

  // Used to link media packets to their protecting FEC packets.
  //
  // TODO(holmer): Refactor into a proper class.
  class ProtectedPacket : public SortablePacket {
   public:
    ProtectedPacket();
    ~ProtectedPacket();

    scoped_refptr<ForwardErrorCorrection::Packet> pkt;
  };

  using ProtectedPacketList = std::list<std::unique_ptr<ProtectedPacket>>;

  struct ProtectedStream {
    uint32_t ssrc = 0;
    uint16_t seq_num_base = 0;
    size_t packet_mask_offset = 0;  // Relative start of FEC header.
    size_t packet_mask_size = 0;
  };

  // Used for internal storage of received FEC packets in a list.
  //
  // TODO(holmer): Refactor into a proper class.
  class ReceivedFecPacket : public SortablePacket {
   public:
    // SSRC count is limited by 4 bits of CSRC count in RTP header (max 15).
    // Since most of the time number of SSRCs will be low (probably 1 most of
    // the time) setting this value to 4 for optimization.
    static constexpr size_t kInlinedSsrcsVectorSize = 4;

    ReceivedFecPacket();
    ~ReceivedFecPacket();

    // List of media packets that this FEC packet protects.
    ProtectedPacketList protected_packets;
    // RTP header fields.
    uint32_t ssrc;
    // FEC header fields.
    size_t fec_header_size;
    absl::InlinedVector<ProtectedStream, kInlinedSsrcsVectorSize>
        protected_streams;
    size_t protection_length;
    // Raw data.
    scoped_refptr<ForwardErrorCorrection::Packet> pkt;
  };

  using PacketList = std::list<std::unique_ptr<Packet>>;
  using RecoveredPacketList = std::list<std::unique_ptr<RecoveredPacket>>;
  using ReceivedFecPacketList = std::list<std::unique_ptr<ReceivedFecPacket>>;

  ~ForwardErrorCorrection();

  // Creates a ForwardErrorCorrection tailored for a specific FEC scheme.
  static std::unique_ptr<ForwardErrorCorrection> CreateUlpfec(uint32_t ssrc);
  static std::unique_ptr<ForwardErrorCorrection> CreateFlexfec(
      uint32_t ssrc,
      uint32_t protected_media_ssrc);

  // Generates a list of FEC packets from supplied media packets.
  //
  // Input:  media_packets          List of media packets to protect, of type
  //                                Packet. All packets must belong to the
  //                                same frame and the list must not be empty.
  // Input:  protection_factor      FEC protection overhead in the [0, 255]
  //                                domain. To obtain 100% overhead, or an
  //                                equal number of FEC packets as
  //                                media packets, use 255.
  // Input:  num_important_packets  The number of "important" packets in the
  //                                frame. These packets may receive greater
  //                                protection than the remaining packets.
  //                                The important packets must be located at the
  //                                start of the media packet list. For codecs
  //                                with data partitioning, the important
  //                                packets may correspond to first partition
  //                                packets.
  // Input:  use_unequal_protection Parameter to enable/disable unequal
  //                                protection (UEP) across packets. Enabling
  //                                UEP will allocate more protection to the
  //                                num_important_packets from the start of the
  //                                media_packets.
  // Input:  fec_mask_type          The type of packet mask used in the FEC.
  //                                Random or bursty type may be selected. The
  //                                bursty type is only defined up to 12 media
  //                                packets. If the number of media packets is
  //                                above 12, the packet masks from the random
  //                                table will be selected.
  // Output: fec_packets            List of pointers to generated FEC packets,
  //                                of type Packet. Must be empty on entry.
  //                                The memory available through the list will
  //                                be valid until the next call to
  //                                EncodeFec().
  //
  // Returns 0 on success, -1 on failure.
  //
  int EncodeFec(const PacketList& media_packets,
                uint8_t protection_factor,
                int num_important_packets,
                bool use_unequal_protection,
                FecMaskType fec_mask_type,
                std::list<Packet*>* fec_packets);

  // Decodes a list of received media and FEC packets. It will parse the
  // `received_packets`, storing FEC packets internally, and move
  // media packets to `recovered_packets`. The recovered list will be
  // sorted by ascending sequence number and have duplicates removed.
  // The function should be called as new packets arrive, and
  // `recovered_packets` will be progressively assembled with each call.
  // When the function returns, `received_packets` will be empty.
  //
  // The caller will allocate packets submitted through `received_packets`.
  // The function will handle allocation of recovered packets.
  //
  // Input:  received_packets   List of new received packets, of type
  //                            ReceivedPacket, belonging to a single
  //                            frame. At output the list will be empty,
  //                            with packets either stored internally,
  //                            or accessible through the recovered list.
  // Output: recovered_packets  List of recovered media packets, of type
  //                            RecoveredPacket, belonging to a single
  //                            frame. The memory available through the
  //                            list will be valid until the next call to
  //                            DecodeFec().
  //
  struct DecodeFecResult {
    // Number of recovered media packets using FEC.
    size_t num_recovered_packets = 0;
  };

  DecodeFecResult DecodeFec(const ReceivedPacket& received_packet,
                            RecoveredPacketList* recovered_packets);

  // Get the number of generated FEC packets, given the number of media packets
  // and the protection factor.
  static int NumFecPackets(int num_media_packets, int protection_factor);

  // Gets the maximum size of the FEC headers in bytes, which must be
  // accounted for as packet overhead.
  size_t MaxPacketOverhead() const;

  // Reset internal states from last frame and clear `recovered_packets`.
  // Frees all memory allocated by this class.
  void ResetState(RecoveredPacketList* recovered_packets);

  // TODO(brandtr): Remove these functions when the Packet classes
  // have been refactored.
  static uint16_t ParseSequenceNumber(const uint8_t* packet);
  static uint32_t ParseSsrc(const uint8_t* packet);

 protected:
  ForwardErrorCorrection(std::unique_ptr<FecHeaderReader> fec_header_reader,
                         std::unique_ptr<FecHeaderWriter> fec_header_writer,
                         uint32_t ssrc,
                         uint32_t protected_media_ssrc);

 private:
  // Analyzes `media_packets` for holes in the sequence and inserts zero columns
  // into the `packet_mask` where those holes are found. Zero columns means that
  // those packets will have no protection.
  // Returns the number of bits used for one row of the new packet mask.
  // Requires that `packet_mask` has at least 6 * `num_fec_packets` bytes
  // allocated.
  int InsertZerosInPacketMasks(const PacketList& media_packets,
                               size_t num_fec_packets);

  // Writes FEC payloads and some recovery fields in the FEC headers.
  void GenerateFecPayloads(const PacketList& media_packets,
                           size_t num_fec_packets);

  // Writes the FEC header fields that are not written by GenerateFecPayloads.
  // This includes writing the packet masks.
  void FinalizeFecHeaders(size_t num_fec_packets,
                          uint32_t media_ssrc,
                          uint16_t seq_num_base);

  // Inserts the `received_packet` into the internal received FEC packet list
  // or into `recovered_packets`.
  void InsertPacket(const ReceivedPacket& received_packet,
                    RecoveredPacketList* recovered_packets);

  // Inserts the `received_packet` into `recovered_packets`. Deletes duplicates.
  void InsertMediaPacket(RecoveredPacketList* recovered_packets,
                         const ReceivedPacket& received_packet);

  // Assigns pointers to the recovered packet from all FEC packets which cover
  // it.
  // Note: This reduces the complexity when we want to try to recover a packet
  // since we don't have to find the intersection between recovered packets and
  // packets covered by the FEC packet.
  void UpdateCoveringFecPackets(const RecoveredPacket& packet);

  // Insert `received_packet` into internal FEC list. Deletes duplicates.
  void InsertFecPacket(const RecoveredPacketList& recovered_packets,
                       const ReceivedPacket& received_packet);

  // Assigns pointers to already recovered packets covered by `fec_packet`.
  static void AssignRecoveredPackets(
      const RecoveredPacketList& recovered_packets,
      ReceivedFecPacket* fec_packet);

  // Attempt to recover missing packets, using the internally stored
  // received FEC packets.
  size_t AttemptRecovery(RecoveredPacketList* recovered_packets);

  // Initializes headers and payload before the XOR operation
  // that recovers a packet.
  static bool StartPacketRecovery(const ReceivedFecPacket& fec_packet,
                                  RecoveredPacket* recovered_packet);

  // Performs XOR between the first 8 bytes of `src` and `dst` and stores
  // the result in `dst`. The 3rd and 4th bytes are used for storing
  // the length recovery field.
  static void XorHeaders(const Packet& src, Packet* dst);

  // Performs XOR between the payloads of `src` and `dst` and stores the result
  // in `dst`. The parameter `dst_offset` determines at  what byte the
  // XOR operation starts in `dst`. In total, `payload_length` bytes are XORed.
  static void XorPayloads(const Packet& src,
                          size_t payload_length,
                          size_t dst_offset,
                          Packet* dst);

  // Finalizes recovery of packet by setting RTP header fields.
  // This is not specific to the FEC scheme used.
  static bool FinishPacketRecovery(const ReceivedFecPacket& fec_packet,
                                   RecoveredPacket* recovered_packet);

  // Recover a missing packet.
  static bool RecoverPacket(const ReceivedFecPacket& fec_packet,
                            RecoveredPacket* recovered_packet);

  // Get the number of missing media packets which are covered by `fec_packet`.
  // An FEC packet can recover at most one packet, and if zero packets are
  // missing the FEC packet can be discarded. This function returns 2 when two
  // or more packets are missing.
  static int NumCoveredPacketsMissing(const ReceivedFecPacket& fec_packet);

  // Discards old packets in `recovered_packets`, which are no longer relevant
  // for recovering lost packets.
  void DiscardOldRecoveredPackets(RecoveredPacketList* recovered_packets);

  // Checks if the FEC packet is old enough and no longer relevant for
  // recovering lost media packets.
  bool IsOldFecPacket(const ReceivedFecPacket& fec_packet,
                      const RecoveredPacketList* recovered_packets);

  // These SSRCs are only used by the decoder.
  const uint32_t ssrc_;
  const uint32_t protected_media_ssrc_;

  std::unique_ptr<FecHeaderReader> fec_header_reader_;
  std::unique_ptr<FecHeaderWriter> fec_header_writer_;

  std::vector<Packet> generated_fec_packets_;
  ReceivedFecPacketList received_fec_packets_;

  // Arrays used to avoid dynamically allocating memory when generating
  // the packet masks.
  // (There are never more than `kUlpfecMaxMediaPackets` FEC packets generated.)
  uint8_t packet_masks_[kUlpfecMaxMediaPackets * kUlpfecMaxPacketMaskSize];
  uint8_t tmp_packet_masks_[kUlpfecMaxMediaPackets * kUlpfecMaxPacketMaskSize];
  size_t packet_mask_size_;
};

// Classes derived from FecHeader{Reader,Writer} encapsulate the
// specifics of reading and writing FEC header for, e.g., ULPFEC
// and FlexFEC.
class FecHeaderReader {
 public:
  virtual ~FecHeaderReader();

  // The maximum number of media packets that can be covered by one FEC packet.
  size_t MaxMediaPackets() const;

  // The maximum number of FEC packets that is supported, per call
  // to ForwardErrorCorrection::EncodeFec().
  size_t MaxFecPackets() const;

  // Parses FEC header and stores information in ReceivedFecPacket members.
  virtual bool ReadFecHeader(
      ForwardErrorCorrection::ReceivedFecPacket* fec_packet) const = 0;

 protected:
  FecHeaderReader(size_t max_media_packets, size_t max_fec_packets);

  const size_t max_media_packets_;
  const size_t max_fec_packets_;
};

class FecHeaderWriter {
 public:
  struct ProtectedStream {
    uint32_t ssrc = 0;
    uint16_t seq_num_base = 0;
    ArrayView<const uint8_t> packet_mask;
  };

  virtual ~FecHeaderWriter();

  // The maximum number of media packets that can be covered by one FEC packet.
  size_t MaxMediaPackets() const;

  // The maximum number of FEC packets that is supported, per call
  // to ForwardErrorCorrection::EncodeFec().
  size_t MaxFecPackets() const;

  // The maximum overhead (in bytes) per packet, due to FEC headers.
  size_t MaxPacketOverhead() const;

  // Calculates the minimum packet mask size needed (in bytes),
  // given the discrete options of the ULPFEC masks and the bits
  // set in the current packet mask.
  virtual size_t MinPacketMaskSize(const uint8_t* packet_mask,
                                   size_t packet_mask_size) const = 0;

  // The header size (in bytes), given the packet mask size.
  virtual size_t FecHeaderSize(size_t packet_mask_size) const = 0;

  // Writes FEC header.
  virtual void FinalizeFecHeader(
      ArrayView<const ProtectedStream> protected_streams,
      ForwardErrorCorrection::Packet& fec_packet) const = 0;

 protected:
  FecHeaderWriter(size_t max_media_packets,
                  size_t max_fec_packets,
                  size_t max_packet_overhead);

  const size_t max_media_packets_;
  const size_t max_fec_packets_;
  const size_t max_packet_overhead_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_FORWARD_ERROR_CORRECTION_H_
