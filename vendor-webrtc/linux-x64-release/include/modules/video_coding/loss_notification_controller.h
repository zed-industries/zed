/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_LOSS_NOTIFICATION_CONTROLLER_H_
#define MODULES_VIDEO_CODING_LOSS_NOTIFICATION_CONTROLLER_H_

#include <stdint.h>

#include <optional>
#include <set>

#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "modules/include/module_common_types.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class LossNotificationController {
 public:
  struct FrameDetails {
    bool is_keyframe;
    int64_t frame_id;
    ArrayView<const int64_t> frame_dependencies;
  };

  LossNotificationController(KeyFrameRequestSender* key_frame_request_sender,
                             LossNotificationSender* loss_notification_sender);
  ~LossNotificationController();

  // An RTP packet was received from the network.
  // `frame` is non-null iff the packet is the first packet in the frame.
  void OnReceivedPacket(uint16_t rtp_seq_num, const FrameDetails* frame);

  // A frame was assembled from packets previously received.
  // (Should be called even if the frame was composed of a single packet.)
  void OnAssembledFrame(uint16_t first_seq_num,
                        int64_t frame_id,
                        bool discardable,
                        ArrayView<const int64_t> frame_dependencies);

 private:
  void DiscardOldInformation();

  bool AllDependenciesDecodable(
      ArrayView<const int64_t> frame_dependencies) const;

  // When the loss of a packet or the non-decodability of a frame is detected,
  // produces a key frame request or a loss notification.
  // 1. `last_received_seq_num` is the last received sequence number.
  // 2. `decodability_flag` refers to the frame associated with the last packet.
  //    It is set to `true` if and only if all of that frame's dependencies are
  //    known to be decodable, and the frame itself is not yet known to be
  //    unassemblable (i.e. no earlier parts of it were lost).
  //    Clarifications:
  //    a. In a multi-packet frame, the first packet reveals the frame's
  //       dependencies, but it is not yet known whether all parts of the
  //       current frame will be received.
  //    b. In a multi-packet frame, if the first packet is missed, the
  //       dependencies are unknown, but it is known that the frame itself
  //       is unassemblable.
  void HandleLoss(uint16_t last_received_seq_num, bool decodability_flag);

  KeyFrameRequestSender* const key_frame_request_sender_
      RTC_GUARDED_BY(sequence_checker_);

  LossNotificationSender* const loss_notification_sender_
      RTC_GUARDED_BY(sequence_checker_);

  // Tracked to avoid processing repeated frames (buggy/malicious remote).
  std::optional<int64_t> last_received_frame_id_
      RTC_GUARDED_BY(sequence_checker_);

  // Tracked to avoid processing repeated packets.
  std::optional<uint16_t> last_received_seq_num_
      RTC_GUARDED_BY(sequence_checker_);

  // Tracked in order to correctly report the potential-decodability of
  // multi-packet frames.
  bool current_frame_potentially_decodable_ RTC_GUARDED_BY(sequence_checker_);

  // Loss notifications contain the sequence number of the first packet of
  // the last decodable-and-non-discardable frame. Since this is a bit of
  // a mouthful, last_decodable_non_discardable_.first_seq_num is used,
  // which hopefully is a bit easier for human beings to parse
  // than `first_seq_num_of_last_decodable_non_discardable_`.
  struct FrameInfo {
    explicit FrameInfo(uint16_t first_seq_num) : first_seq_num(first_seq_num) {}
    uint16_t first_seq_num;
  };
  std::optional<FrameInfo> last_decodable_non_discardable_
      RTC_GUARDED_BY(sequence_checker_);

  // Track which frames are decodable. Later frames are also decodable if
  // all of their dependencies can be found in this container.
  // (Naturally, later frames must also be assemblable to be decodable.)
  std::set<int64_t> decodable_frame_ids_ RTC_GUARDED_BY(sequence_checker_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_LOSS_NOTIFICATION_CONTROLLER_H_
