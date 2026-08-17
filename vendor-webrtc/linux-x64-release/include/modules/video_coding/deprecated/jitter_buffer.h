/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_H_
#define MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_H_

#include <cstddef>
#include <cstdint>
#include <list>
#include <map>
#include <memory>
#include <set>
#include <vector>

#include "api/field_trials_view.h"
#include "modules/include/module_common_types_public.h"
#include "modules/video_coding/deprecated/decoding_state.h"
#include "modules/video_coding/deprecated/event_wrapper.h"
#include "modules/video_coding/deprecated/jitter_buffer_common.h"
#include "modules/video_coding/timing/inter_frame_delay_variation_calculator.h"
#include "modules/video_coding/timing/jitter_estimator.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// forward declarations
class Clock;
class VCMFrameBuffer;
class VCMPacket;
class VCMEncodedFrame;

typedef std::list<VCMFrameBuffer*> UnorderedFrameList;

struct VCMJitterSample {
  VCMJitterSample() : timestamp(0), frame_size(0), latest_packet_time(-1) {}
  uint32_t timestamp;
  uint32_t frame_size;
  int64_t latest_packet_time;
};

class TimestampLessThan {
 public:
  bool operator()(uint32_t timestamp1, uint32_t timestamp2) const {
    return IsNewerTimestamp(timestamp2, timestamp1);
  }
};

class FrameList
    : public std::map<uint32_t, VCMFrameBuffer*, TimestampLessThan> {
 public:
  void InsertFrame(VCMFrameBuffer* frame);
  VCMFrameBuffer* PopFrame(uint32_t timestamp);
  VCMFrameBuffer* Front() const;
  VCMFrameBuffer* Back() const;
  int RecycleFramesUntilKeyFrame(FrameList::iterator* key_frame_it,
                                 UnorderedFrameList* free_frames);
  void CleanUpOldOrEmptyFrames(VCMDecodingState* decoding_state,
                               UnorderedFrameList* free_frames);
  void Reset(UnorderedFrameList* free_frames);
};

class VCMJitterBuffer {
 public:
  VCMJitterBuffer(Clock* clock,
                  std::unique_ptr<EventWrapper> event,
                  const FieldTrialsView& field_trials);

  ~VCMJitterBuffer();

  VCMJitterBuffer(const VCMJitterBuffer&) = delete;
  VCMJitterBuffer& operator=(const VCMJitterBuffer&) = delete;

  // Initializes and starts jitter buffer.
  void Start() RTC_LOCKS_EXCLUDED(mutex_);

  // Signals all internal events and stops the jitter buffer.
  void Stop() RTC_LOCKS_EXCLUDED(mutex_);

  // Returns true if the jitter buffer is running.
  bool Running() const RTC_LOCKS_EXCLUDED(mutex_);

  // Empty the jitter buffer of all its data.
  void Flush() RTC_LOCKS_EXCLUDED(mutex_);

  // Gets number of packets received.
  int num_packets() const RTC_LOCKS_EXCLUDED(mutex_);

  // Gets number of duplicated packets received.
  int num_duplicated_packets() const RTC_LOCKS_EXCLUDED(mutex_);

  // Wait `max_wait_time_ms` for a complete frame to arrive.
  // If found, a pointer to the frame is returned. Returns nullptr otherwise.
  VCMEncodedFrame* NextCompleteFrame(uint32_t max_wait_time_ms)
      RTC_LOCKS_EXCLUDED(mutex_);

  // Extract frame corresponding to input timestamp.
  // Frame will be set to a decoding state.
  VCMEncodedFrame* ExtractAndSetDecode(uint32_t timestamp)
      RTC_LOCKS_EXCLUDED(mutex_);

  // Releases a frame returned from the jitter buffer, should be called when
  // done with decoding.
  void ReleaseFrame(VCMEncodedFrame* frame) RTC_LOCKS_EXCLUDED(mutex_);

  // Returns the time in ms when the latest packet was inserted into the frame.
  // Retransmitted is set to true if any of the packets belonging to the frame
  // has been retransmitted.
  int64_t LastPacketTime(const VCMEncodedFrame* frame,
                         bool* retransmitted) const RTC_LOCKS_EXCLUDED(mutex_);

  // Inserts a packet into a frame returned from GetFrame().
  // If the return value is <= 0, `frame` is invalidated and the pointer must
  // be dropped after this function returns.
  VCMFrameBufferEnum InsertPacket(const VCMPacket& packet, bool* retransmitted)
      RTC_LOCKS_EXCLUDED(mutex_);

  // Returns the estimated jitter in milliseconds.
  uint32_t EstimatedJitterMs() RTC_LOCKS_EXCLUDED(mutex_);

  void SetNackSettings(size_t max_nack_list_size,
                       int max_packet_age_to_nack,
                       int max_incomplete_time_ms) RTC_LOCKS_EXCLUDED(mutex_);

  // Returns a list of the sequence numbers currently missing.
  std::vector<uint16_t> GetNackList(bool* request_key_frame)
      RTC_LOCKS_EXCLUDED(mutex_);

 private:
  class SequenceNumberLessThan {
   public:
    bool operator()(const uint16_t& sequence_number1,
                    const uint16_t& sequence_number2) const {
      return IsNewerSequenceNumber(sequence_number2, sequence_number1);
    }
  };
  typedef std::set<uint16_t, SequenceNumberLessThan> SequenceNumberSet;

  // Gets the frame assigned to the timestamp of the packet. May recycle
  // existing frames if no free frames are available. Returns an error code if
  // failing, or kNoError on success. `frame_list` contains which list the
  // packet was in, or NULL if it was not in a FrameList (a new frame).
  VCMFrameBufferEnum GetFrame(const VCMPacket& packet,
                              VCMFrameBuffer** frame,
                              FrameList** frame_list)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Returns true if `frame` is continuous in `decoding_state`, not taking
  // decodable frames into account.
  bool IsContinuousInState(const VCMFrameBuffer& frame,
                           const VCMDecodingState& decoding_state) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Returns true if `frame` is continuous in the `last_decoded_state_`, taking
  // all decodable frames into account.
  bool IsContinuous(const VCMFrameBuffer& frame) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Looks for frames in `incomplete_frames_` which are continuous in the
  // provided `decoded_state`. Starts the search from the timestamp of
  // `decoded_state`.
  void FindAndInsertContinuousFramesWithState(
      const VCMDecodingState& decoded_state)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Looks for frames in `incomplete_frames_` which are continuous in
  // `last_decoded_state_` taking all decodable frames into account. Starts
  // the search from `new_frame`.
  void FindAndInsertContinuousFrames(const VCMFrameBuffer& new_frame)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  VCMFrameBuffer* NextFrame() const RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Returns true if the NACK list was updated to cover sequence numbers up to
  // `sequence_number`. If false a key frame is needed to get into a state where
  // we can continue decoding.
  bool UpdateNackList(uint16_t sequence_number)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  bool TooLargeNackList() const;
  // Returns true if the NACK list was reduced without problem. If false a key
  // frame is needed to get into a state where we can continue decoding.
  bool HandleTooLargeNackList() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  bool MissingTooOldPacket(uint16_t latest_sequence_number) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Returns true if the too old packets was successfully removed from the NACK
  // list. If false, a key frame is needed to get into a state where we can
  // continue decoding.
  bool HandleTooOldPackets(uint16_t latest_sequence_number)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Drops all packets in the NACK list up until `last_decoded_sequence_number`.
  void DropPacketsFromNackList(uint16_t last_decoded_sequence_number);

  // Gets an empty frame, creating a new frame if necessary (i.e. increases
  // jitter buffer size).
  VCMFrameBuffer* GetEmptyFrame() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Attempts to increase the size of the jitter buffer. Returns true on
  // success, false otherwise.
  bool TryToIncreaseJitterBufferSize() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Recycles oldest frames until a key frame is found. Used if jitter buffer is
  // completely full. Returns true if a key frame was found.
  bool RecycleFramesUntilKeyFrame() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Update rolling average of packets per frame.
  void UpdateAveragePacketsPerFrame(int current_number_packets_)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Cleans the frame list in the JB from old/empty frames.
  // Should only be called prior to actual use.
  void CleanUpOldOrEmptyFrames() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Returns true if `packet` is likely to have been retransmitted.
  bool IsPacketRetransmitted(const VCMPacket& packet) const;

  // The following three functions update the jitter estimate with the
  // payload size, receive time and RTP timestamp of a frame.
  void UpdateJitterEstimate(const VCMJitterSample& sample,
                            bool incomplete_frame);
  void UpdateJitterEstimate(const VCMFrameBuffer& frame, bool incomplete_frame);
  void UpdateJitterEstimate(int64_t latest_packet_time_ms,
                            uint32_t timestamp,
                            unsigned int frame_size,
                            bool incomplete_frame);

  int NonContinuousOrIncompleteDuration() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  uint16_t EstimatedLowSequenceNumber(const VCMFrameBuffer& frame) const;

  // Reset frame buffer and return it to free_frames_.
  void RecycleFrameBuffer(VCMFrameBuffer* frame)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Empty the jitter buffer of all its data.
  void FlushInternal() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  Clock* clock_;
  // If we are running (have started) or not.
  bool running_;
  mutable Mutex mutex_;
  // Event to signal when we have a frame ready for decoder.
  std::unique_ptr<EventWrapper> frame_event_;
  // Number of allocated frames.
  int max_number_of_frames_;
  UnorderedFrameList free_frames_ RTC_GUARDED_BY(mutex_);
  FrameList decodable_frames_ RTC_GUARDED_BY(mutex_);
  FrameList incomplete_frames_ RTC_GUARDED_BY(mutex_);
  VCMDecodingState last_decoded_state_ RTC_GUARDED_BY(mutex_);
  bool first_packet_since_reset_;

  // Number of packets in a row that have been too old.
  int num_consecutive_old_packets_;
  // Number of packets received.
  int num_packets_ RTC_GUARDED_BY(mutex_);
  // Number of duplicated packets received.
  int num_duplicated_packets_ RTC_GUARDED_BY(mutex_);

  // Jitter estimation.
  // Filter for estimating jitter.
  JitterEstimator jitter_estimate_;
  // Calculates network delays used for jitter calculations.
  InterFrameDelayVariationCalculator inter_frame_delay_;
  VCMJitterSample waiting_for_completion_;

  // Holds the internal NACK list (the missing sequence numbers).
  SequenceNumberSet missing_sequence_numbers_;
  uint16_t latest_received_sequence_number_;
  size_t max_nack_list_size_;
  int max_packet_age_to_nack_;  // Measured in sequence numbers.
  int max_incomplete_time_ms_;

  // Estimated rolling average of packets per frame
  float average_packets_per_frame_;
  // average_packets_per_frame converges fast if we have fewer than this many
  // frames.
  int frame_counter_;
};
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_H_
