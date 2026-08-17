/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TX_OUTSTANDING_DATA_H_
#define NET_DCSCTP_TX_OUTSTANDING_DATA_H_

#include <deque>
#include <map>
#include <optional>
#include <set>
#include <utility>
#include <vector>

#include "api/units/timestamp.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/forward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/iforward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/sack_chunk.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/public/types.h"
#include "rtc_base/containers/flat_set.h"

namespace dcsctp {

// This class keeps track of outstanding data chunks (sent, not yet acked) and
// handles acking, nacking, rescheduling and abandoning.
//
// Items are added to this queue as they are sent and will be removed when the
// peer acks them using the cumulative TSN ack.
class OutstandingData {
 public:
  // State for DATA chunks (message fragments) in the queue - used in tests.
  enum class State {
    // The chunk has been sent but not received yet (from the sender's point of
    // view, as no SACK has been received yet that reference this chunk).
    kInFlight,
    // A SACK has been received which explicitly marked this chunk as missing -
    // it's now NACKED and may be retransmitted if NACKED enough times.
    kNacked,
    // A chunk that will be retransmitted when possible.
    kToBeRetransmitted,
    // A SACK has been received which explicitly marked this chunk as received.
    kAcked,
    // A chunk whose message has expired or has been retransmitted too many
    // times (RFC3758). It will not be retransmitted anymore.
    kAbandoned,
  };

  // Contains variables scoped to a processing of an incoming SACK.
  struct AckInfo {
    explicit AckInfo(UnwrappedTSN cumulative_tsn_ack)
        : highest_tsn_acked(cumulative_tsn_ack) {}

    // Bytes acked by increasing cumulative_tsn_ack and gap_ack_blocks.
    size_t bytes_acked = 0;

    // Indicates if this SACK indicates that packet loss has occurred. Just
    // because a packet is missing in the SACK doesn't necessarily mean that
    // there is packet loss as that packet might be in-flight and received
    // out-of-order. But when it has been reported missing consecutive times, it
    // will eventually be considered "lost" and this will be set.
    bool has_packet_loss = false;

    // Highest TSN Newly Acknowledged, an SCTP variable.
    UnwrappedTSN highest_tsn_acked;

    // The set of lifecycle IDs that were acked using cumulative_tsn_ack.
    std::vector<LifecycleId> acked_lifecycle_ids;
    // The set of lifecycle IDs that were acked, but had been abandoned.
    std::vector<LifecycleId> abandoned_lifecycle_ids;
  };

  OutstandingData(
      size_t data_chunk_header_size,
      UnwrappedTSN last_cumulative_tsn_ack,
      std::function<bool(StreamID, OutgoingMessageId)> discard_from_send_queue)
      : data_chunk_header_size_(data_chunk_header_size),
        last_cumulative_tsn_ack_(last_cumulative_tsn_ack),
        discard_from_send_queue_(std::move(discard_from_send_queue)) {}

  AckInfo HandleSack(
      UnwrappedTSN cumulative_tsn_ack,
      webrtc::ArrayView<const SackChunk::GapAckBlock> gap_ack_blocks,
      bool is_in_fast_recovery);

  // Returns as many of the chunks that are eligible for fast retransmissions
  // and that would fit in a single packet of `max_size`. The eligible chunks
  // that didn't fit will be marked for (normal) retransmission and will not be
  // returned if this method is called again.
  std::vector<std::pair<TSN, Data>> GetChunksToBeFastRetransmitted(
      size_t max_size);

  // Given `max_size` of space left in a packet, which chunks can be added to
  // it?
  std::vector<std::pair<TSN, Data>> GetChunksToBeRetransmitted(size_t max_size);

  // How many inflight bytes there are, as sent on the wire as packets.
  size_t unacked_packet_bytes() const { return unacked_packet_bytes_; }

  // How many inflight bytes there are, counting only the payload.
  size_t unacked_payload_bytes() const { return unacked_payload_bytes_; }

  // Returns the number of DATA chunks that are in-flight (not acked or nacked).
  size_t unacked_items() const { return unacked_items_; }

  // Given the current time `now_ms`, expire and abandon outstanding (sent at
  // least once) chunks that have a limited lifetime.
  void ExpireOutstandingChunks(webrtc::Timestamp now);

  bool empty() const { return outstanding_data_.empty(); }

  bool has_data_to_be_fast_retransmitted() const {
    return !to_be_fast_retransmitted_.empty();
  }

  bool has_data_to_be_retransmitted() const {
    return !to_be_retransmitted_.empty() || !to_be_fast_retransmitted_.empty();
  }

  UnwrappedTSN last_cumulative_tsn_ack() const {
    return last_cumulative_tsn_ack_;
  }

  UnwrappedTSN next_tsn() const {
    return highest_outstanding_tsn().next_value();
  }

  UnwrappedTSN highest_outstanding_tsn() const;

  // Schedules `data` to be sent, with the provided partial reliability
  // parameters. Returns the TSN if the item was actually added and scheduled to
  // be sent, and std::nullopt if it shouldn't be sent.
  std::optional<UnwrappedTSN> Insert(
      OutgoingMessageId message_id,
      const Data& data,
      webrtc::Timestamp time_sent,
      MaxRetransmits max_retransmissions = MaxRetransmits::NoLimit(),
      webrtc::Timestamp expires_at = webrtc::Timestamp::PlusInfinity(),
      LifecycleId lifecycle_id = LifecycleId::NotSet());

  // Nacks all outstanding data.
  void NackAll();

  // Creates a FORWARD-TSN chunk.
  ForwardTsnChunk CreateForwardTsn() const;

  // Creates an I-FORWARD-TSN chunk.
  IForwardTsnChunk CreateIForwardTsn() const;

  // Given the current time and a TSN, it returns the measured RTT between when
  // the chunk was sent and now. It takes into acccount Karn's algorithm, so if
  // the chunk has ever been retransmitted, it will return `PlusInfinity()`.
  webrtc::TimeDelta MeasureRTT(webrtc::Timestamp now, UnwrappedTSN tsn) const;

  // Returns the internal state of all queued chunks. This is only used in
  // unit-tests.
  std::vector<std::pair<TSN, State>> GetChunkStatesForTesting() const;

  // Returns true if the next chunk that is not acked by the peer has been
  // abandoned, which means that a FORWARD-TSN should be sent.
  bool ShouldSendForwardTsn() const;

  // Sets the next TSN to be used. This is used in handover.
  void ResetSequenceNumbers(UnwrappedTSN last_cumulative_tsn);

  // Called when an outgoing stream reset is sent, marking the last assigned TSN
  // as a breakpoint that a FORWARD-TSN shouldn't cross.
  void BeginResetStreams();

 private:
  // A fragmented message's DATA chunk while in the retransmission queue, and
  // its associated metadata.
  class Item {
   public:
    enum class NackAction {
      kNothing,
      kRetransmit,
      kAbandon,
    };

    Item(OutgoingMessageId message_id,
         Data data,
         webrtc::Timestamp time_sent,
         MaxRetransmits max_retransmissions,
         webrtc::Timestamp expires_at,
         LifecycleId lifecycle_id)
        : message_id_(message_id),
          time_sent_(time_sent),
          max_retransmissions_(max_retransmissions),
          expires_at_(expires_at),
          lifecycle_id_(lifecycle_id),
          data_(std::move(data)) {}

    Item(const Item&) = delete;
    Item& operator=(const Item&) = delete;

    OutgoingMessageId message_id() const { return message_id_; }

    webrtc::Timestamp time_sent() const { return time_sent_; }

    const Data& data() const { return data_; }

    // Acks an item.
    void Ack();

    // Nacks an item. If it has been nacked enough times, or if `retransmit_now`
    // is set, it might be marked for retransmission. If the item has reached
    // its max retransmission value, it will instead be abandoned. The action
    // performed is indicated as return value.
    NackAction Nack(bool retransmit_now);

    // Prepares the item to be retransmitted. Sets it as outstanding and
    // clears all nack counters.
    void MarkAsRetransmitted();

    // Marks this item as abandoned.
    void Abandon();

    bool is_outstanding() const { return ack_state_ == AckState::kUnacked; }
    bool is_acked() const { return ack_state_ == AckState::kAcked; }
    bool is_nacked() const { return ack_state_ == AckState::kNacked; }
    bool is_abandoned() const { return lifecycle_ == Lifecycle::kAbandoned; }

    // Indicates if this chunk should be retransmitted.
    bool should_be_retransmitted() const {
      return lifecycle_ == Lifecycle::kToBeRetransmitted;
    }
    // Indicates if this chunk has ever been retransmitted.
    bool has_been_retransmitted() const { return num_retransmissions_ > 0; }

    // Given the current time, and the current state of this DATA chunk, it will
    // indicate if it has expired (SCTP Partial Reliability Extension).
    bool has_expired(webrtc::Timestamp now) const;

    LifecycleId lifecycle_id() const { return lifecycle_id_; }

   private:
    enum class Lifecycle : uint8_t {
      // The chunk is alive (sent, received, etc)
      kActive,
      // The chunk is scheduled to be retransmitted, and will then transition to
      // become active.
      kToBeRetransmitted,
      // The chunk has been abandoned. This is a terminal state.
      kAbandoned
    };
    enum class AckState : uint8_t {
      // The chunk is in-flight.
      kUnacked,
      // The chunk has been received and acknowledged.
      kAcked,
      // The chunk has been nacked and is possibly lost.
      kNacked
    };

    // NOTE: This data structure has been optimized for size, by ordering fields
    // to avoid unnecessary padding.

    const OutgoingMessageId message_id_;

    // When the packet was sent, and placed in this queue.
    const webrtc::Timestamp time_sent_;
    // If the message was sent with a maximum number of retransmissions, this is
    // set to that number. The value zero (0) means that it will never be
    // retransmitted.
    const MaxRetransmits max_retransmissions_;

    // Indicates the life cycle status of this chunk.
    Lifecycle lifecycle_ = Lifecycle::kActive;
    // Indicates the presence of this chunk, if it's in flight (Unacked), has
    // been received (Acked) or is possibly lost (Nacked).
    AckState ack_state_ = AckState::kUnacked;

    // The number of times the DATA chunk has been nacked (by having received a
    // SACK which doesn't include it). Will be cleared on retransmissions.
    uint8_t nack_count_ = 0;
    // The number of times the DATA chunk has been retransmitted.
    uint16_t num_retransmissions_ = 0;

    // At this exact millisecond, the item is considered expired. If the message
    // is not to be expired, this is set to the infinite future.
    const webrtc::Timestamp expires_at_;

    // An optional lifecycle id, which may only be set for the last fragment.
    const LifecycleId lifecycle_id_;

    // The actual data to send/retransmit.
    const Data data_;
  };

  // Returns how large a chunk will be, serialized, carrying the data
  size_t GetSerializedChunkSize(const Data& data) const;

  Item& GetItem(UnwrappedTSN tsn);
  const Item& GetItem(UnwrappedTSN tsn) const;

  // Given a `cumulative_tsn_ack` from an incoming SACK, will remove those items
  // in the retransmission queue up until this value and will update `ack_info`
  // by setting `bytes_acked_by_cumulative_tsn_ack`.
  void RemoveAcked(UnwrappedTSN cumulative_tsn_ack, AckInfo& ack_info);

  // Will mark the chunks covered by the `gap_ack_blocks` from an incoming SACK
  // as "acked" and update `ack_info` by adding new TSNs to `added_tsns`.
  void AckGapBlocks(
      UnwrappedTSN cumulative_tsn_ack,
      webrtc::ArrayView<const SackChunk::GapAckBlock> gap_ack_blocks,
      AckInfo& ack_info);

  // Mark chunks reported as "missing", as "nacked" or "to be retransmitted"
  // depending how many times this has happened. Only packets up until
  // `ack_info.highest_tsn_acked` (highest TSN newly acknowledged) are
  // nacked/retransmitted. The method will set `ack_info.has_packet_loss`.
  void NackBetweenAckBlocks(
      UnwrappedTSN cumulative_tsn_ack,
      webrtc::ArrayView<const SackChunk::GapAckBlock> gap_ack_blocks,
      bool is_in_fast_recovery,
      OutstandingData::AckInfo& ack_info);

  // Process the acknowledgement of the chunk referenced by `iter` and updates
  // state in `ack_info` and the object's state.
  void AckChunk(AckInfo& ack_info, UnwrappedTSN tsn, Item& item);

  // Helper method to process an incoming nack of an item and perform the
  // correct operations given the action indicated when nacking an item (e.g.
  // retransmitting or abandoning). The return value indicate if an action was
  // performed, meaning that packet loss was detected and acted upon. If
  // `do_fast_retransmit` is set and if the item has been nacked sufficiently
  // many times so that it should be retransmitted, this will schedule it to be
  // "fast retransmitted". This is only done just before going into fast
  // recovery.
  //
  // Note that since nacking an item may result in it becoming abandoned, which
  // in turn could alter `outstanding_data_`, any iterators are invalidated
  // after having called this method.
  bool NackItem(UnwrappedTSN tsn, bool retransmit_now, bool do_fast_retransmit);

  // Given that a message fragment, `item` has been abandoned, abandon all other
  // fragments that share the same message - both never-before-sent fragments
  // that are still in the SendQueue and outstanding chunks.
  void AbandonAllFor(const OutstandingData::Item& item);

  std::vector<std::pair<TSN, Data>> ExtractChunksThatCanFit(
      std::set<UnwrappedTSN>& chunks,
      size_t max_size);

  bool IsConsistent() const;

  // The size of the data chunk (DATA/I-DATA) header that is used.
  const size_t data_chunk_header_size_;
  // The last cumulative TSN ack number.
  UnwrappedTSN last_cumulative_tsn_ack_;
  // Callback when to discard items from the send queue.
  std::function<bool(StreamID, OutgoingMessageId)> discard_from_send_queue_;

  // Outstanding items. If non-empty, the first element has
  // `TSN=last_cumulative_tsn_ack_ + 1` and the following items are in strict
  // increasing TSN order. The last item has `TSN=highest_outstanding_tsn()`.
  std::deque<Item> outstanding_data_;
  // The number of bytes that are in-flight, counting only the payload.
  size_t unacked_payload_bytes_ = 0;
  // The number of bytes that are in-flight, as sent on the wire (as packets).
  size_t unacked_packet_bytes_ = 0;
  // The number of DATA chunks that are in-flight (sent but not yet acked or
  // nacked).
  size_t unacked_items_ = 0;
  // Data chunks that are eligible for fast retransmission.
  std::set<UnwrappedTSN> to_be_fast_retransmitted_;
  // Data chunks that are to be retransmitted.
  std::set<UnwrappedTSN> to_be_retransmitted_;
  // Wben a stream reset has begun, the "next TSN to assign" is added to this
  // set, and removed when the cum-ack TSN reaches it. This is used to limit a
  // FORWARD-TSN to reset streams past a "stream reset last assigned TSN".
  webrtc::flat_set<UnwrappedTSN> stream_reset_breakpoint_tsns_;
};
}  // namespace dcsctp
#endif  // NET_DCSCTP_TX_OUTSTANDING_DATA_H_
