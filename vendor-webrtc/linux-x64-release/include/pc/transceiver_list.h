/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TRANSCEIVER_LIST_H_
#define PC_TRANSCEIVER_LIST_H_

#include <stddef.h>

#include <algorithm>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "pc/rtp_transceiver.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

typedef scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
    RtpTransceiverProxyRefPtr;

// Captures partial state to be used for rollback. Applicable only in
// Unified Plan.
class TransceiverStableState {
 public:
  TransceiverStableState() {}
  void set_newly_created();
  void SetMSectionIfUnset(std::optional<std::string> mid,
                          std::optional<size_t> mline_index);
  void SetRemoteStreamIds(const std::vector<std::string>& ids);
  void SetInitSendEncodings(
      const std::vector<RtpEncodingParameters>& encodings);
  void SetFiredDirection(
      std::optional<RtpTransceiverDirection> fired_direction) {
    fired_direction_ = fired_direction;
  }
  std::optional<std::string> mid() const { return mid_; }
  std::optional<size_t> mline_index() const { return mline_index_; }
  std::optional<std::vector<std::string>> remote_stream_ids() const {
    return remote_stream_ids_;
  }
  std::optional<std::vector<RtpEncodingParameters>> init_send_encodings()
      const {
    return init_send_encodings_;
  }
  bool has_m_section() const { return has_m_section_; }
  bool newly_created() const { return newly_created_; }
  bool did_set_fired_direction() const { return fired_direction_.has_value(); }
  // Because fired_direction() is nullable, did_set_fired_direction() is used to
  // distinguish beteen "no value" and "null value".
  std::optional<RtpTransceiverDirection> fired_direction() const {
    RTC_DCHECK(did_set_fired_direction());
    return fired_direction_.value();
  }

 private:
  std::optional<std::string> mid_;
  std::optional<size_t> mline_index_;
  std::optional<std::vector<std::string>> remote_stream_ids_;
  std::optional<std::vector<RtpEncodingParameters>> init_send_encodings_;
  // Indicates that mid value from stable state has been captured and
  // that rollback has to restore the transceiver. Also protects against
  // subsequent overwrites.
  bool has_m_section_ = false;
  // Indicates that the transceiver was created as part of applying a
  // description to track potential need for removing transceiver during
  // rollback.
  bool newly_created_ = false;
  // `fired_direction_` is nullable, so an optional of an optional is used to
  // distinguish between null and not set (sorry if this hurts your eyes).
  std::optional<std::optional<RtpTransceiverDirection>> fired_direction_;
};

// This class encapsulates the active list of transceivers on a
// PeerConnection, and offers convenient functions on that list.
// It is a single-thread class; all operations must be performed
// on the same thread.
class TransceiverList {
 public:
  // Returns a copy of the currently active list of transceivers. The
  // list consists of webrtc::scoped_refptrs, which will keep the transceivers
  // from being deallocated, even if they are removed from the TransceiverList.
  std::vector<RtpTransceiverProxyRefPtr> List() const {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return transceivers_;
  }
  // As above, but does not check thread ownership. Unsafe.
  // TODO(bugs.webrtc.org/12692): Refactor and remove
  std::vector<RtpTransceiverProxyRefPtr> UnsafeList() const {
    return transceivers_;
  }

  // Returns a list of the internal() pointers of the currently active list
  // of transceivers. These raw pointers are not thread-safe, so need to
  // be consumed on the same thread.
  std::vector<RtpTransceiver*> ListInternal() const;

  void Add(RtpTransceiverProxyRefPtr transceiver) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    transceivers_.push_back(transceiver);
  }
  void Remove(RtpTransceiverProxyRefPtr transceiver) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    transceivers_.erase(
        std::remove(transceivers_.begin(), transceivers_.end(), transceiver),
        transceivers_.end());
  }
  RtpTransceiverProxyRefPtr FindBySender(
      scoped_refptr<RtpSenderInterface> sender) const;
  RtpTransceiverProxyRefPtr FindByMid(const std::string& mid) const;
  RtpTransceiverProxyRefPtr FindByMLineIndex(size_t mline_index) const;

  // Find or create the stable state for a transceiver.
  TransceiverStableState* StableState(RtpTransceiverProxyRefPtr transceiver) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return &(transceiver_stable_states_by_transceivers_[transceiver]);
  }

  void DiscardStableStates() {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    transceiver_stable_states_by_transceivers_.clear();
  }

  std::map<RtpTransceiverProxyRefPtr, TransceiverStableState>& StableStates() {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return transceiver_stable_states_by_transceivers_;
  }

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  std::vector<RtpTransceiverProxyRefPtr> transceivers_;
  // TODO(bugs.webrtc.org/12692): Add RTC_GUARDED_BY(sequence_checker_);

  // Holds changes made to transceivers during applying descriptors for
  // potential rollback. Gets cleared once signaling state goes to stable.
  std::map<RtpTransceiverProxyRefPtr, TransceiverStableState>
      transceiver_stable_states_by_transceivers_
          RTC_GUARDED_BY(sequence_checker_);
  // Holds remote stream ids for transceivers from stable state.
  std::map<RtpTransceiverProxyRefPtr, std::vector<std::string>>
      remote_stream_ids_by_transceivers_ RTC_GUARDED_BY(sequence_checker_);
};

}  // namespace webrtc

#endif  // PC_TRANSCEIVER_LIST_H_
