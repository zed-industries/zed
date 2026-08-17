/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PC_E2E_PEER_CONNECTION_QUALITY_TEST_H_
#define TEST_PC_E2E_PEER_CONNECTION_QUALITY_TEST_H_

#include <memory>
#include <queue>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/test/audio_quality_analyzer_interface.h"
#include "api/test/metrics/metrics_logger.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/pclf/media_quality_test_params.h"
#include "api/test/pclf/peer_configurer.h"
#include "api/test/peerconnection_quality_test_fixture.h"
#include "api/test/time_controller.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/task_queue_for_test.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "test/pc/e2e/analyzer/video/single_process_encoded_image_data_injector.h"
#include "test/pc/e2e/analyzer/video/video_quality_analyzer_injection_helper.h"
#include "test/pc/e2e/analyzer_helper.h"
#include "test/pc/e2e/media/media_helper.h"
#include "test/pc/e2e/sdp/sdp_changer.h"
#include "test/pc/e2e/test_activities_executor.h"
#include "test/pc/e2e/test_peer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

class PeerConnectionE2EQualityTest
    : public PeerConnectionE2EQualityTestFixture {
 public:
  using QualityMetricsReporter =
      PeerConnectionE2EQualityTestFixture::QualityMetricsReporter;

  PeerConnectionE2EQualityTest(
      std::string test_case_name,
      TimeController& time_controller,
      std::unique_ptr<AudioQualityAnalyzerInterface> audio_quality_analyzer,
      std::unique_ptr<VideoQualityAnalyzerInterface> video_quality_analyzer);
  PeerConnectionE2EQualityTest(
      std::string test_case_name,
      TimeController& time_controller,
      std::unique_ptr<AudioQualityAnalyzerInterface> audio_quality_analyzer,
      std::unique_ptr<VideoQualityAnalyzerInterface> video_quality_analyzer,
      test::MetricsLogger* metrics_logger);

  ~PeerConnectionE2EQualityTest() override = default;

  void ExecuteAt(TimeDelta target_time_since_start,
                 std::function<void(TimeDelta)> func) override;
  void ExecuteEvery(TimeDelta initial_delay_since_start,
                    TimeDelta interval,
                    std::function<void(TimeDelta)> func) override;

  void AddQualityMetricsReporter(std::unique_ptr<QualityMetricsReporter>
                                     quality_metrics_reporter) override;

  PeerHandle* AddPeer(std::unique_ptr<PeerConfigurer> configurer) override;
  void Run(RunParams run_params) override;

  TimeDelta GetRealTestDuration() const override {
    MutexLock lock(&lock_);
    RTC_CHECK_NE(real_test_duration_, TimeDelta::Zero());
    return real_test_duration_;
  }

 private:
  class PeerHandleImpl : public PeerHandle {
   public:
    ~PeerHandleImpl() override = default;
  };

  // For some functionality some field trials have to be enabled, they will be
  // enabled in Run().
  std::string GetFieldTrials(const RunParams& run_params);
  void OnTrackCallback(absl::string_view peer_name,
                       VideoSubscription peer_subscription,
                       scoped_refptr<RtpTransceiverInterface> transceiver,
                       std::vector<VideoConfig> remote_video_configs);
  // Have to be run on the signaling thread.
  void SetupCallOnSignalingThread(const RunParams& run_params);
  void TearDownCallOnSignalingThread();
  void SetPeerCodecPreferences(TestPeer* peer);
  std::unique_ptr<SignalingInterceptor> CreateSignalingInterceptor(
      const RunParams& run_params);
  void WaitUntilIceCandidatesGathered(Thread* signaling_thread);
  void WaitUntilPeersAreConnected(Thread* signaling_thread);
  void ExchangeOfferAnswer(SignalingInterceptor* signaling_interceptor);
  void ExchangeIceCandidates(SignalingInterceptor* signaling_interceptor);
  void StartVideo(
      const std::vector<scoped_refptr<TestVideoCapturerVideoTrackSource>>&
          sources);
  void TearDownCall();
  void ReportGeneralTestResults();
  Timestamp Now() const;

  TimeController& time_controller_;
  const std::unique_ptr<TaskQueueFactory> task_queue_factory_;
  std::string test_case_name_;
  std::unique_ptr<VideoQualityAnalyzerInjectionHelper>
      video_quality_analyzer_injection_helper_;
  std::unique_ptr<MediaHelper> media_helper_;
  std::unique_ptr<EncodedImageDataPropagator> encoded_image_data_propagator_;
  std::unique_ptr<AudioQualityAnalyzerInterface> audio_quality_analyzer_;
  std::unique_ptr<TestActivitiesExecutor> executor_;
  test::MetricsLogger* const metrics_logger_;

  std::vector<std::unique_ptr<PeerConfigurer>> peer_configurations_;
  std::vector<PeerHandleImpl> peer_handles_;

  std::unique_ptr<TestPeer> alice_;
  std::unique_ptr<TestPeer> bob_;
  std::vector<std::unique_ptr<QualityMetricsReporter>>
      quality_metrics_reporters_;

  std::vector<scoped_refptr<TestVideoCapturerVideoTrackSource>>
      alice_video_sources_;
  std::vector<scoped_refptr<TestVideoCapturerVideoTrackSource>>
      bob_video_sources_;
  std::vector<std::unique_ptr<VideoSinkInterface<VideoFrame>>>
      output_video_sinks_;
  AnalyzerHelper analyzer_helper_;

  mutable Mutex lock_;
  TimeDelta real_test_duration_ RTC_GUARDED_BY(lock_) = TimeDelta::Zero();

  // Task queue, that is used for running activities during test call.
  // This task queue will be created before call set up and will be destroyed
  // immediately before call tear down.
  std::unique_ptr<TaskQueueForTest> task_queue_;

  bool alice_connected_ = false;
  bool bob_connected_ = false;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_PEER_CONNECTION_QUALITY_TEST_H_
