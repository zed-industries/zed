/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_TEST_PEER_H_
#define TEST_PC_E2E_TEST_PEER_H_

#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/data_channel_interface.h"
#include "api/jsep.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/stats/rtc_stats_report.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/pclf/media_quality_test_params.h"
#include "api/test/pclf/peer_configurer.h"
#include "pc/peer_connection_wrapper.h"
#include "pc/test/mock_peer_connection_observers.h"
#include "rtc_base/checks.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "test/pc/e2e/stats_provider.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Describes a single participant in the call.
class TestPeer final : public StatsProvider {
 public:
  ~TestPeer() override = default;

  const Params& params() const { return params_; }

  ConfigurableParams configurable_params() const;
  void AddVideoConfig(VideoConfig config);
  // Removes video config with specified name. Crashes if the config with
  // specified name isn't found.
  void RemoveVideoConfig(absl::string_view stream_label);
  void SetVideoSubscription(VideoSubscription subscription);

  void GetStats(RTCStatsCollectorCallback* callback) override;

  PeerConfigurer::VideoSource ReleaseVideoSource(size_t i) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return std::move(video_sources_[i]);
  }

  PeerConnectionFactoryInterface* pc_factory() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->pc_factory();
  }
  PeerConnectionInterface* pc() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->pc();
  }
  MockPeerConnectionObserver* observer() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->observer();
  }

  // Tell underlying `PeerConnection` to create an Offer.
  // `observer` will be invoked on the signaling thread when offer is created.
  void CreateOffer(scoped_refptr<CreateSessionDescriptionObserver> observer) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    pc()->CreateOffer(observer.get(), params_.rtc_offer_answer_options);
  }
  std::unique_ptr<SessionDescriptionInterface> CreateOffer() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->CreateOffer(params_.rtc_offer_answer_options);
  }

  std::unique_ptr<SessionDescriptionInterface> CreateAnswer() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->CreateAnswer();
  }

  bool SetLocalDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                           std::string* error_out = nullptr) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->SetLocalDescription(std::move(desc), error_out);
  }

  // `error_out` will be set only if returned value is false.
  bool SetRemoteDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                            std::string* error_out = nullptr);

  scoped_refptr<RtpTransceiverInterface> AddTransceiver(
      webrtc::MediaType media_type,
      const RtpTransceiverInit& init) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->AddTransceiver(media_type, init);
  }

  scoped_refptr<RtpSenderInterface> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids = {}) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->AddTrack(track, stream_ids);
  }

  scoped_refptr<DataChannelInterface> CreateDataChannel(
      const std::string& label,
      const std::optional<DataChannelInit>& config = std::nullopt) {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->CreateDataChannel(label, config);
  }

  PeerConnectionInterface::SignalingState signaling_state() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->signaling_state();
  }

  bool IsIceGatheringDone() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->IsIceGatheringDone();
  }

  bool IsIceConnected() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->IsIceConnected();
  }

  scoped_refptr<const RTCStatsReport> GetStats() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    return wrapper_->GetStats();
  }

  void DetachAecDump() {
    RTC_CHECK(wrapper_) << "TestPeer is already closed";
    wrapper_->pc_factory()->StopAecDump();
  }

  // Adds provided `candidates` to the owned peer connection.
  bool AddIceCandidates(
      std::vector<std::unique_ptr<IceCandidateInterface>> candidates);

  // Closes underlying peer connection and destroys all related objects freeing
  // up related resources.
  void Close();

 protected:
  friend class TestPeerFactory;
  TestPeer(scoped_refptr<PeerConnectionFactoryInterface> pc_factory,
           scoped_refptr<PeerConnectionInterface> pc,
           std::unique_ptr<MockPeerConnectionObserver> observer,
           Params params,
           ConfigurableParams configurable_params,
           std::vector<PeerConfigurer::VideoSource> video_sources,
           std::unique_ptr<Thread> worker_thread);

 private:
  const Params params_;

  mutable Mutex mutex_;
  ConfigurableParams configurable_params_ RTC_GUARDED_BY(mutex_);

  // Safety flag to protect all tasks posted on the signaling thread to not be
  // executed after `wrapper_` object is destructed.
  scoped_refptr<PendingTaskSafetyFlag> signaling_thread_task_safety_ = nullptr;

  // Keeps ownership of worker thread. It has to be destroyed after `wrapper_`.
  // `worker_thread_`can be null if the Peer use only one thread as both the
  // worker thread and network thread.
  std::unique_ptr<Thread> worker_thread_;
  std::unique_ptr<PeerConnectionWrapper> wrapper_;
  std::vector<PeerConfigurer::VideoSource> video_sources_;

  std::vector<std::unique_ptr<IceCandidateInterface>> remote_ice_candidates_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_TEST_PEER_H_
