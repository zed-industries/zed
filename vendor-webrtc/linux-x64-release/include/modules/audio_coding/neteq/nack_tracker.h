/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_NACK_TRACKER_H_
#define MODULES_AUDIO_CODING_NETEQ_NACK_TRACKER_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <optional>
#include <vector>

#include "api/field_trials_view.h"
#include "modules/include/module_common_types_public.h"
#include "rtc_base/gtest_prod_util.h"

//
// The NackTracker class keeps track of the lost packets, an estimate of
// time-to-play for each packet is also given.
//
// Every time a packet is pushed into NetEq, LastReceivedPacket() has to be
// called to update the NACK list.
//
// Every time 10ms audio is pulled from NetEq LastDecodedPacket() should be
// called, and time-to-play is updated at that moment.
//
// If packet N is received, any packet prior to N which has not arrived is
// considered lost, and should be labeled as "missing" (the size of
// the list might be limited and older packet eliminated from the list).
//
// The NackTracker class has to know about the sample rate of the packets to
// compute time-to-play. So sample rate should be set as soon as the first
// packet is received. If there is a change in the receive codec (sender changes
// codec) then NackTracker should be reset. This is because NetEQ would flush
// its buffer and re-transmission is meaning less for old packet. Therefore, in
// that case, after reset the sampling rate has to be updated.
//
// Thread Safety
// =============
// Please note that this class in not thread safe. The class must be protected
// if different APIs are called from different threads.
//
namespace webrtc {

class NackTracker {
 public:
  // A limit for the size of the NACK list.
  static const size_t kNackListSizeLimit = 500;  // 10 seconds for 20 ms frame
                                                 // packets.
  explicit NackTracker(const FieldTrialsView& field_trials);
  ~NackTracker();

  // Set a maximum for the size of the NACK list. If the last received packet
  // has sequence number of N, then NACK list will not contain any element
  // with sequence number earlier than N - `max_nack_list_size`.
  //
  // The largest maximum size is defined by `kNackListSizeLimit`
  void SetMaxNackListSize(size_t max_nack_list_size);

  // Set the sampling rate.
  //
  // If associated sampling rate of the received packets is changed, call this
  // function to update sampling rate. Note that if there is any change in
  // received codec then NetEq will flush its buffer and NACK has to be reset.
  // After Reset() is called sampling rate has to be set.
  void UpdateSampleRate(int sample_rate_hz);

  // Update the sequence number and the timestamp of the last decoded RTP.
  void UpdateLastDecodedPacket(uint16_t sequence_number, uint32_t timestamp);

  // Update the sequence number and the timestamp of the last received RTP. This
  // API should be called every time a packet pushed into ACM.
  void UpdateLastReceivedPacket(uint16_t sequence_number, uint32_t timestamp);

  // Get a list of "missing" packets which have expected time-to-play larger
  // than the given round-trip-time (in milliseconds).
  // Note: Late packets are not included.
  // Calling this method multiple times may give different results, since the
  // internal nack list may get flushed if never_nack_multiple_times_ is true.
  std::vector<uint16_t> GetNackList(int64_t round_trip_time_ms);

  // Reset to default values. The NACK list is cleared.
  // `max_nack_list_size_` preserves its value.
  void Reset();

  // Returns the estimated packet loss rate in Q30, for testing only.
  uint32_t GetPacketLossRateForTest() { return packet_loss_rate_; }

 private:
  // This test need to access the private method GetNackList().
  FRIEND_TEST_ALL_PREFIXES(NackTrackerTest, EstimateTimestampAndTimeToPlay);

  // Options that can be configured via field trial.
  struct Config {
    explicit Config(const FieldTrialsView& field_trials);

    // The exponential decay factor used to estimate the packet loss rate.
    double packet_loss_forget_factor = 0.996;
    // How many additional ms we are willing to wait (at most) for nacked
    // packets for each additional percentage of packet loss.
    int ms_per_loss_percent = 20;
    // If true, never nack packets more than once.
    bool never_nack_multiple_times = false;
    // Only nack if the RTT is valid.
    bool require_valid_rtt = false;
    // Default RTT to use unless `require_valid_rtt` is set.
    int default_rtt_ms = 100;
    // Do not nack if the loss rate is above this value.
    double max_loss_rate = 1.0;
  };

  struct NackElement {
    NackElement(int64_t initial_time_to_play_ms, uint32_t initial_timestamp)
        : time_to_play_ms(initial_time_to_play_ms),
          estimated_timestamp(initial_timestamp) {}

    // Estimated time (ms) left for this packet to be decoded. This estimate is
    // updated every time jitter buffer decodes a packet.
    int64_t time_to_play_ms;

    // A guess about the timestamp of the missing packet, it is used for
    // estimation of `time_to_play_ms`. The estimate might be slightly wrong if
    // there has been frame-size change since the last received packet and the
    // missing packet. However, the risk of this is low, and in case of such
    // errors, there will be a minor misestimation in time-to-play of missing
    // packets. This will have a very minor effect on NACK performance.
    uint32_t estimated_timestamp;
  };

  class NackListCompare {
   public:
    bool operator()(uint16_t sequence_number_old,
                    uint16_t sequence_number_new) const {
      return IsNewerSequenceNumber(sequence_number_new, sequence_number_old);
    }
  };

  typedef std::map<uint16_t, NackElement, NackListCompare> NackList;

  // This API is used only for testing to assess whether time-to-play is
  // computed correctly.
  NackList GetNackList() const;

  // Returns a valid number of samples per packet given the current received
  // sequence number and timestamp or nullopt of none could be computed.
  std::optional<int> GetSamplesPerPacket(
      uint16_t sequence_number_current_received_rtp,
      uint32_t timestamp_current_received_rtp) const;

  // Given the `sequence_number_current_received_rtp` of currently received RTP
  // update the list. Packets that are older than the received packet are added
  // to the nack list.
  void UpdateList(uint16_t sequence_number_current_received_rtp,
                  uint32_t timestamp_current_received_rtp);

  // Packets which have sequence number older that
  // `sequence_num_last_received_rtp_` - `max_nack_list_size_` are removed
  // from the NACK list.
  void LimitNackListSize();

  // Estimate timestamp of a missing packet given its sequence number.
  uint32_t EstimateTimestamp(uint16_t sequence_number, int samples_per_packet);

  // Compute time-to-play given a timestamp.
  int64_t TimeToPlay(uint32_t timestamp) const;

  // Updates the estimated packet lost rate.
  void UpdatePacketLossRate(int packets_lost);

  const Config config_;

  // Valid if a packet is received.
  uint16_t sequence_num_last_received_rtp_;
  uint32_t timestamp_last_received_rtp_;
  bool any_rtp_received_;  // If any packet received.

  // Valid if a packet is decoded.
  uint16_t sequence_num_last_decoded_rtp_;
  uint32_t timestamp_last_decoded_rtp_;
  bool any_rtp_decoded_;  // If any packet decoded.

  int sample_rate_khz_;  // Sample rate in kHz.

  // A list of missing packets to be retransmitted. Components of the list
  // contain the sequence number of missing packets and the estimated time that
  // each pack is going to be played out.
  NackList nack_list_;

  // NACK list will not keep track of missing packets prior to
  // `sequence_num_last_received_rtp_` - `max_nack_list_size_`.
  size_t max_nack_list_size_;

  // Current estimate of the packet loss rate in Q30.
  uint32_t packet_loss_rate_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_NACK_TRACKER_H_
