/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_PEERCONNECTION_QUALITY_TEST_FIXTURE_H_
#define API_TEST_PEERCONNECTION_QUALITY_TEST_FIXTURE_H_

#include <stddef.h>

#include <functional>
#include <memory>

#include "absl/strings/string_view.h"
#include "api/test/pclf/media_quality_test_params.h"
#include "api/test/pclf/peer_configurer.h"
#include "api/test/stats_observer_interface.h"
#include "api/test/track_id_stream_info_map.h"
#include "api/units/time_delta.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// API is in development. Can be changed/removed without notice.
class PeerConnectionE2EQualityTestFixture {
 public:
  // Represent an entity that will report quality metrics after test.
  class QualityMetricsReporter : public StatsObserverInterface {
   public:
    virtual ~QualityMetricsReporter() = default;

    // Invoked by framework after peer connection factory and peer connection
    // itself will be created but before offer/answer exchange will be started.
    // `test_case_name` is name of test case, that should be used to report all
    // metrics.
    // `reporter_helper` is a pointer to a class that will allow track_id to
    // stream_id matching. The caller is responsible for ensuring the
    // TrackIdStreamInfoMap will be valid from Start() to
    // StopAndReportResults().
    virtual void Start(absl::string_view test_case_name,
                       const TrackIdStreamInfoMap* reporter_helper) = 0;

    // Invoked by framework after call is ended and peer connection factory and
    // peer connection are destroyed.
    virtual void StopAndReportResults() = 0;
  };

  // Represents single participant in call and can be used to perform different
  // in-call actions. Might be extended in future.
  class PeerHandle {
   public:
    virtual ~PeerHandle() = default;
  };

  virtual ~PeerConnectionE2EQualityTestFixture() = default;

  // Add activity that will be executed on the best effort at least after
  // `target_time_since_start` after call will be set up (after offer/answer
  // exchange, ICE gathering will be done and ICE candidates will passed to
  // remote side). `func` param is amount of time spent from the call set up.
  virtual void ExecuteAt(TimeDelta target_time_since_start,
                         std::function<void(TimeDelta)> func) = 0;
  // Add activity that will be executed every `interval` with first execution
  // on the best effort at least after `initial_delay_since_start` after call
  // will be set up (after all participants will be connected). `func` param is
  // amount of time spent from the call set up.
  virtual void ExecuteEvery(TimeDelta initial_delay_since_start,
                            TimeDelta interval,
                            std::function<void(TimeDelta)> func) = 0;

  // Add stats reporter entity to observe the test.
  virtual void AddQualityMetricsReporter(
      std::unique_ptr<QualityMetricsReporter> quality_metrics_reporter) = 0;

  // Add a new peer to the call and return an object through which caller
  // can configure peer's behavior.
  // `network_dependencies` are used to provide networking for peer's peer
  // connection. Members must be non-null.
  // `configurer` function will be used to configure peer in the call.
  virtual PeerHandle* AddPeer(std::unique_ptr<PeerConfigurer> configurer) = 0;

  // Runs the media quality test, which includes setting up the call with
  // configured participants, running it according to provided `run_params` and
  // terminating it properly at the end. During call duration media quality
  // metrics are gathered, which are then reported to stdout and (if configured)
  // to the json/protobuf output file through the WebRTC perf test results
  // reporting system.
  virtual void Run(RunParams run_params) = 0;

  // Returns real test duration - the time of test execution measured during
  // test. Client must call this method only after test is finished (after
  // Run(...) method returned). Test execution time is time from end of call
  // setup (offer/answer, ICE candidates exchange done and ICE connected) to
  // start of call tear down (PeerConnection closed).
  virtual TimeDelta GetRealTestDuration() const = 0;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_PEERCONNECTION_QUALITY_TEST_FIXTURE_H_
