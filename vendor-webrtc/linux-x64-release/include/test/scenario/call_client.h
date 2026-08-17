/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_CALL_CLIENT_H_
#define TEST_SCENARIO_CALL_CLIENT_H_

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_device.h"
#include "api/environment/environment.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/rtp_parameters.h"
#include "api/test/time_controller.h"
#include "api/units/data_rate.h"
#include "call/call.h"
#include "modules/congestion_controller/goog_cc/test/goog_cc_printer.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "rtc_base/task_queue_for_test.h"
#include "test/logging/log_writer.h"
#include "test/network/network_emulation.h"
#include "test/scenario/column_printer.h"
#include "test/scenario/network_node.h"
#include "test/scenario/scenario_config.h"

namespace webrtc {

namespace test {
// Helper class to capture network controller state.
class NetworkControleUpdateCache : public NetworkControllerInterface {
 public:
  explicit NetworkControleUpdateCache(
      std::unique_ptr<NetworkControllerInterface> controller);

  NetworkControlUpdate OnNetworkAvailability(NetworkAvailability msg) override;
  NetworkControlUpdate OnNetworkRouteChange(NetworkRouteChange msg) override;
  NetworkControlUpdate OnProcessInterval(ProcessInterval msg) override;
  NetworkControlUpdate OnRemoteBitrateReport(RemoteBitrateReport msg) override;
  NetworkControlUpdate OnRoundTripTimeUpdate(RoundTripTimeUpdate msg) override;
  NetworkControlUpdate OnSentPacket(SentPacket msg) override;
  NetworkControlUpdate OnReceivedPacket(ReceivedPacket msg) override;
  NetworkControlUpdate OnStreamsConfig(StreamsConfig msg) override;
  NetworkControlUpdate OnTargetRateConstraints(
      TargetRateConstraints msg) override;
  NetworkControlUpdate OnTransportLossReport(TransportLossReport msg) override;
  NetworkControlUpdate OnTransportPacketsFeedback(
      TransportPacketsFeedback msg) override;
  NetworkControlUpdate OnNetworkStateEstimate(
      NetworkStateEstimate msg) override;

  NetworkControlUpdate update_state() const;

 private:
  NetworkControlUpdate Update(NetworkControlUpdate update);
  const std::unique_ptr<NetworkControllerInterface> controller_;
  NetworkControlUpdate update_state_;
};

class LoggingNetworkControllerFactory
    : public NetworkControllerFactoryInterface {
 public:
  LoggingNetworkControllerFactory(LogWriterFactoryInterface* log_writer_factory,
                                  TransportControllerConfig config);

  ~LoggingNetworkControllerFactory();

  LoggingNetworkControllerFactory(const LoggingNetworkControllerFactory&) =
      delete;
  LoggingNetworkControllerFactory& operator=(
      const LoggingNetworkControllerFactory&) = delete;

  std::unique_ptr<NetworkControllerInterface> Create(
      NetworkControllerConfig config) override;
  TimeDelta GetProcessInterval() const override;
  // TODO(srte): Consider using the Columnprinter interface for this.
  void LogCongestionControllerStats(Timestamp at_time);
  void SetRemoteBitrateEstimate(RemoteBitrateReport msg);

  NetworkControlUpdate GetUpdate() const;

 private:
  GoogCcDebugFactory goog_cc_factory_;
  NetworkControllerFactoryInterface* cc_factory_ = nullptr;
  bool print_cc_state_ = false;
  NetworkControleUpdateCache* last_controller_ = nullptr;
};

struct CallClientFakeAudio {
  scoped_refptr<AudioProcessing> apm;
  scoped_refptr<AudioDeviceModule> fake_audio_device;
  scoped_refptr<AudioState> audio_state;
};
// CallClient represents a participant in a call scenario. It is created by the
// Scenario class and is used as sender and receiver when setting up a media
// stream session.
class CallClient : public EmulatedNetworkReceiverInterface {
 public:
  CallClient(TimeController* time_controller,
             std::unique_ptr<LogWriterFactoryInterface> log_writer_factory,
             CallClientConfig config);

  ~CallClient();

  CallClient(const CallClient&) = delete;
  CallClient& operator=(const CallClient&) = delete;

  ColumnPrinter StatsPrinter();
  Call::Stats GetStats();
  DataRate send_bandwidth() {
    return DataRate::BitsPerSec(GetStats().send_bandwidth_bps);
  }
  DataRate target_rate() const;
  DataRate stable_target_rate() const;
  DataRate padding_rate() const;
  void UpdateBitrateConstraints(const BitrateConstraints& constraints);
  void SetRemoteBitrate(DataRate bitrate);

  void SetAudioReceiveRtpHeaderExtensions(ArrayView<RtpExtension> extensions);
  void SetVideoReceiveRtpHeaderExtensions(ArrayView<RtpExtension> extensions);

  // Sets the network adapter id used next time the network route changes.
  void UpdateNetworkAdapterId(int adapter_id);

  void OnPacketReceived(EmulatedIpPacket packet) override;
  std::unique_ptr<RtcEventLogOutput> GetLogWriter(std::string name);

  // Exposed publicly so that tests can execute tasks such as querying stats
  // for media streams in the expected runtime environment (essentially what
  // CallClient does internally for GetStats()).
  void SendTask(std::function<void()> task);

 private:
  friend class Scenario;
  friend class CallClientPair;
  friend class SendVideoStream;
  friend class VideoStreamPair;
  friend class ReceiveVideoStream;
  friend class SendAudioStream;
  friend class ReceiveAudioStream;
  friend class AudioStreamPair;
  friend class NetworkNodeTransport;
  uint32_t GetNextVideoSsrc();
  uint32_t GetNextVideoLocalSsrc();
  uint32_t GetNextAudioSsrc();
  uint32_t GetNextAudioLocalSsrc();
  uint32_t GetNextRtxSsrc();
  int16_t Bind(EmulatedEndpoint* endpoint);
  void UnBind();

  TimeController* const time_controller_;
  Environment env_;
  const std::unique_ptr<LogWriterFactoryInterface> log_writer_factory_;
  LoggingNetworkControllerFactory network_controller_factory_;
  CallClientFakeAudio fake_audio_setup_;
  std::unique_ptr<Call> call_;
  std::unique_ptr<NetworkNodeTransport> transport_;
  std::vector<std::pair<EmulatedEndpoint*, uint16_t>> endpoints_;
  RtpHeaderExtensionMap audio_extensions_;
  RtpHeaderExtensionMap video_extensions_;

  int next_video_ssrc_index_ = 0;
  int next_video_local_ssrc_index_ = 0;
  int next_rtx_ssrc_index_ = 0;
  int next_audio_ssrc_index_ = 0;
  int next_audio_local_ssrc_index_ = 0;
  std::map<uint32_t, MediaType> ssrc_media_types_;
  // Defined last so it's destroyed first.
  TaskQueueForTest task_queue_;
};

class CallClientPair {
 public:
  ~CallClientPair();

  CallClientPair(const CallClientPair&) = delete;
  CallClientPair& operator=(const CallClientPair&) = delete;

  CallClient* first() { return first_; }
  CallClient* second() { return second_; }
  std::pair<CallClient*, CallClient*> forward() { return {first(), second()}; }
  std::pair<CallClient*, CallClient*> reverse() { return {second(), first()}; }

 private:
  friend class Scenario;
  CallClientPair(CallClient* first, CallClient* second)
      : first_(first), second_(second) {}
  CallClient* const first_;
  CallClient* const second_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCENARIO_CALL_CLIENT_H_
