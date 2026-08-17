/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_ANALYZING_VIDEO_SINKS_HELPER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_ANALYZING_VIDEO_SINKS_HELPER_H_

#include <list>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/video/video_frame_writer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Registry of known video configs and video writers.
// This class is thread safe.
class AnalyzingVideoSinksHelper {
 public:
  // Adds config in the registry. If config with such stream label was
  // registered before, the new value will override the old one.
  void AddConfig(absl::string_view sender_peer_name, VideoConfig config);
  std::optional<std::pair<std::string, VideoConfig>> GetPeerAndConfig(
      absl::string_view stream_label);
  // Removes video config for specified stream label. If there are no know video
  // config for such stream label - does nothing.
  void RemoveConfig(absl::string_view stream_label);

  // Takes ownership of the provided video writer. All video writers owned by
  // this class will be closed during `AnalyzingVideoSinksHelper` destruction
  // and guaranteed to be alive either until explicitly removed by
  // `CloseAndRemoveVideoWriters` or until `AnalyzingVideoSinksHelper` is
  // destroyed.
  //
  // Returns pointer to the added writer. Ownership is maintained by
  // `AnalyzingVideoSinksHelper`.
  test::VideoFrameWriter* AddVideoWriter(
      std::unique_ptr<test::VideoFrameWriter> video_writer);
  // For each provided `writers_to_close`, if it is known, will close and
  // destroy it, otherwise does nothing with it.
  void CloseAndRemoveVideoWriters(
      std::set<test::VideoFrameWriter*> writers_to_close);

  // Removes all added configs and close and removes all added writers.
  void Clear();

 private:
  Mutex mutex_;
  std::map<std::string, std::pair<std::string, VideoConfig>> video_configs_
      RTC_GUARDED_BY(mutex_);
  std::list<std::unique_ptr<test::VideoFrameWriter>> video_writers_
      RTC_GUARDED_BY(mutex_);
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_ANALYZING_VIDEO_SINKS_HELPER_H_
