/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_AUDIO_QUALITY_ANALYZER_INTERFACE_H_
#define API_TEST_AUDIO_QUALITY_ANALYZER_INTERFACE_H_

#include <string>

#include "api/test/stats_observer_interface.h"
#include "api/test/track_id_stream_info_map.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// API is in development. Can be changed/removed without notice.
class AudioQualityAnalyzerInterface : public StatsObserverInterface {
 public:
  ~AudioQualityAnalyzerInterface() override = default;

  // Will be called by the framework before the test.
  // `test_case_name` is name of test case, that should be used to report all
  // audio metrics.
  // `analyzer_helper` is a pointer to a class that will allow track_id to
  // stream_id matching. The caller is responsible for ensuring the
  // AnalyzerHelper outlives the instance of the AudioQualityAnalyzerInterface.
  virtual void Start(std::string test_case_name,
                     TrackIdStreamInfoMap* analyzer_helper) = 0;

  // Will be called by the framework at the end of the test. The analyzer
  // has to finalize all its stats and it should report them.
  virtual void Stop() = 0;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_AUDIO_QUALITY_ANALYZER_INTERFACE_H_
