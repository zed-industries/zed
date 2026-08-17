/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_HELPER_H_
#define TEST_PC_E2E_ANALYZER_HELPER_H_

#include <map>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/sequence_checker.h"
#include "api/test/track_id_stream_info_map.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// This class is a utility that provides bookkeeping capabilities that
// are useful to associate stats reports track_ids to the remote stream info.
// The framework will populate an instance of this class and it will pass
// it to the Start method of Media Quality Analyzers.
// An instance of AnalyzerHelper must only be accessed from a single
// thread and since stats collection happens on the signaling thread,
// AddTrackToStreamMapping, GetStreamLabelFromTrackId and
// GetSyncGroupLabelFromTrackId must be invoked from the signaling thread. Get
// methods should be invoked only after all data is added. Mixing Get methods
// with adding new data may lead to undefined behavior.
class AnalyzerHelper : public TrackIdStreamInfoMap {
 public:
  AnalyzerHelper();

  void AddTrackToStreamMapping(absl::string_view track_id,
                               absl::string_view receiver_peer,
                               absl::string_view stream_label,
                               std::optional<std::string> sync_group);
  void AddTrackToStreamMapping(std::string track_id, std::string stream_label);
  void AddTrackToStreamMapping(std::string track_id,
                               std::string stream_label,
                               std::string sync_group);

  StreamInfo GetStreamInfoFromTrackId(
      absl::string_view track_id) const override;

 private:
  SequenceChecker signaling_sequence_checker_;
  std::map<std::string, StreamInfo> track_to_stream_map_
      RTC_GUARDED_BY(signaling_sequence_checker_);
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_HELPER_H_
