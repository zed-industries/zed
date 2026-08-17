/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_MEDIA_MEDIA_HELPER_H_
#define TEST_PC_E2E_MEDIA_MEDIA_HELPER_H_

#include <memory>
#include <vector>

#include "api/test/frame_generator_interface.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/pclf/peer_configurer.h"
#include "test/pc/e2e/analyzer/video/video_quality_analyzer_injection_helper.h"
#include "test/pc/e2e/media/test_video_capturer_video_track_source.h"
#include "test/pc/e2e/test_peer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

class MediaHelper {
 public:
  MediaHelper(VideoQualityAnalyzerInjectionHelper*
                  video_quality_analyzer_injection_helper,
              TaskQueueFactory* task_queue_factory,
              Clock* clock)
      : clock_(clock),
        task_queue_factory_(task_queue_factory),
        video_quality_analyzer_injection_helper_(
            video_quality_analyzer_injection_helper) {}

  void MaybeAddAudio(TestPeer* peer);

  std::vector<scoped_refptr<TestVideoCapturerVideoTrackSource>> MaybeAddVideo(
      TestPeer* peer);

 private:
  std::unique_ptr<test::TestVideoCapturer> CreateVideoCapturer(
      const VideoConfig& video_config,
      PeerConfigurer::VideoSource source,
      std::unique_ptr<test::TestVideoCapturer::FramePreprocessor>
          frame_preprocessor);

  Clock* const clock_;
  TaskQueueFactory* const task_queue_factory_;
  VideoQualityAnalyzerInjectionHelper* video_quality_analyzer_injection_helper_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_MEDIA_MEDIA_HELPER_H_
