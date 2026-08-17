/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_RX_DATA_TRACKER_H_
#define NET_DCSCTP_RX_DATA_TRACKER_H_

#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "net/dcsctp/common/sequence_numbers.h"
#include "net/dcsctp/packet/chunk/data_common.h"
#include "net/dcsctp/packet/chunk/sack_chunk.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/public/dcsctp_handover_state.h"
#include "net/dcsctp/timer/timer.h"

namespace dcsctp {

// Keeps track of received DATA chunks and handles all logic for _when_ to
// create SACKs and also _how_ to generate them.
//
// It only uses TSNs to track delivery and doesn't need to be aware of streams.
//
// SACKs are optimally sent every second packet on connections with no packet
// loss. When packet loss is detected, it's sent for every packet. When SACKs
// are not sent directly, a timer is used to send a SACK delayed (by RTO/2, or
// 200ms, whatever is smallest).
class DataTracker {
 public:
  // The maximum number of duplicate TSNs that will be reported in a SACK.
  static constexpr size_t kMaxDuplicateTsnReported = 20;
  // The maximum number of gap-ack-blocks that will be reported in a SACK.
  static constexpr size_t kMaxGapAckBlocksReported = 20;

  // The maximum number of accepted in-flight DATA chunks. This indicates the
  // maximum difference from this buffer's last cumulative ack TSN, and any
  // received data. Data received beyond this limit will be dropped, which will
  // force the transmitter to send data that actually increases the last
  // cumulative acked TSN.
  static constexpr uint32_t kMaxAcceptedOutstandingFragments = 100000;

  DataTracker(absl::string_view log_prefix,
              Timer* delayed_ack_timer,
              TSN peer_initial_tsn)
      : log_prefix_(log_prefix),
        seen_packet_(false),
        delayed_ack_timer_(*delayed_ack_timer),
        last_cumulative_acked_tsn_(
            tsn_unwrapper_.Unwrap(TSN(*peer_initial_tsn - 1))) {}

  // Indicates if the provided TSN is valid. If this return false, the data
  // should be dropped and not added to any other buffers, which essentially
  // means that there is intentional packet loss.
  bool IsTSNValid(TSN tsn) const;

  // Call for every incoming data chunk. Returns `true` if `tsn` was seen for
  // the first time, and `false` if it has been seen before (a duplicate `tsn`).
  bool Observe(TSN tsn,
               AnyDataChunk::ImmediateAckFlag immediate_ack =
                   AnyDataChunk::ImmediateAckFlag(false));
  // Called at the end of processing an SCTP packet.
  void ObservePacketEnd();

  // Called for incoming FORWARD-TSN/I-FORWARD-TSN chunks. Indicates if the
  // chunk had any effect.
  bool HandleForwardTsn(TSN new_cumulative_ack);

  // Indicates if a SACK should be sent. There may be other reasons to send a
  // SACK, but if this function indicates so, it should be sent as soon as
  // possible. Calling this function will make it clear a flag so that if it's
  // called again, it will probably return false.
  //
  // If the delayed ack timer is running, this method will return false _unless_
  // `also_if_delayed` is set to true. Then it will return true as well.
  bool ShouldSendAck(bool also_if_delayed = false);

  // Returns the last cumulative ack TSN - the last seen data chunk's TSN
  // value before any packet loss was detected.
  TSN last_cumulative_acked_tsn() const {
    return TSN(last_cumulative_acked_tsn_.Wrap());
  }

  bool IsLaterThanCumulativeAckedTsn(TSN tsn) const {
    return tsn_unwrapper_.PeekUnwrap(tsn) > last_cumulative_acked_tsn_;
  }

  // Returns true if the received `tsn` would increase the cumulative ack TSN.
  bool will_increase_cum_ack_tsn(TSN tsn) const;

  // Forces `ShouldSendSack` to return true.
  void ForceImmediateSack();

  // Note that this will clear `duplicates_`, so every SackChunk that is
  // consumed must be sent.
  SackChunk CreateSelectiveAck(size_t a_rwnd);

  void HandleDelayedAckTimerExpiry();

  HandoverReadinessStatus GetHandoverReadiness() const;

  void AddHandoverState(DcSctpSocketHandoverState& state);
  void RestoreFromState(const DcSctpSocketHandoverState& state);

 private:
  enum class AckState {
    // No need to send an ACK.
    kIdle,

    // Has received data chunks (but not yet end of packet).
    kBecomingDelayed,

    // Has received data chunks and the end of a packet. Delayed ack timer is
    // running and a SACK will be sent on expiry, or if DATA is sent, or after
    // next packet with data.
    kDelayed,

    // Send a SACK immediately after handling this packet.
    kImmediate,
  };

  // Represents ranges of TSNs that have been received that are not directly
  // following the last cumulative acked TSN. This information is returned to
  // the sender in the "gap ack blocks" in the SACK chunk. The blocks are always
  // non-overlapping and non-adjacent.
  class AdditionalTsnBlocks {
   public:
    // Represents an inclusive range of received TSNs, i.e. [first, last].
    struct TsnRange {
      TsnRange(UnwrappedTSN first, UnwrappedTSN last)
          : first(first), last(last) {}
      UnwrappedTSN first;
      UnwrappedTSN last;
    };

    // Adds a TSN to the set. This will try to expand any existing block and
    // might merge blocks to ensure that all blocks are non-adjacent. If a
    // current block can't be expanded, a new block is created.
    //
    // The return value indicates if `tsn` was added. If false is returned, the
    // `tsn` was already represented in one of the blocks.
    bool Add(UnwrappedTSN tsn);

    // Erases all TSNs up to, and including `tsn`. This will remove all blocks
    // that are completely below `tsn` and may truncate a block where `tsn` is
    // within that block. In that case, the frontmost block's start TSN will be
    // the next following tsn after `tsn`.
    void EraseTo(UnwrappedTSN tsn);

    // Removes the first block. Must not be called on an empty set.
    void PopFront();

    const std::vector<TsnRange>& blocks() const { return blocks_; }

    bool empty() const { return blocks_.empty(); }

    const TsnRange& front() const { return blocks_.front(); }

   private:
    // A sorted vector of non-overlapping and non-adjacent blocks.
    std::vector<TsnRange> blocks_;
  };

  std::vector<SackChunk::GapAckBlock> CreateGapAckBlocks() const;
  void UpdateAckState(AckState new_state, absl::string_view reason);
  static absl::string_view ToString(AckState ack_state);

  const absl::string_view log_prefix_;
  // If a packet has ever been seen.
  bool seen_packet_;
  Timer& delayed_ack_timer_;
  AckState ack_state_ = AckState::kIdle;
  UnwrappedTSN::Unwrapper tsn_unwrapper_;

  // All TSNs up until (and including) this value have been seen.
  UnwrappedTSN last_cumulative_acked_tsn_;
  // Received TSNs that are not directly following `last_cumulative_acked_tsn_`.
  AdditionalTsnBlocks additional_tsn_blocks_;
  std::set<TSN> duplicate_tsns_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_RX_DATA_TRACKER_H_
