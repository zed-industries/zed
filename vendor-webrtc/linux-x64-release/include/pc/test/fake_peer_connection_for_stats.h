/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_PEER_CONNECTION_FOR_STATS_H_
#define PC_TEST_FAKE_PEER_CONNECTION_FOR_STATS_H_

#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_device.h"
#include "api/audio_options.h"
#include "api/crypto/crypto_options.h"
#include "api/environment/environment_factory.h"
#include "api/make_ref_counted.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "call/call.h"
#include "call/payload_type_picker.h"
#include "media/base/fake_media_engine.h"
#include "media/base/media_channel.h"
#include "p2p/base/p2p_constants.h"
#include "p2p/base/port.h"
#include "pc/channel.h"
#include "pc/connection_context.h"
#include "pc/data_channel_utils.h"
#include "pc/rtp_receiver.h"
#include "pc/rtp_receiver_proxy.h"
#include "pc/rtp_sender.h"
#include "pc/rtp_sender_proxy.h"
#include "pc/rtp_transceiver.h"
#include "pc/sctp_data_channel.h"
#include "pc/stream_collection.h"
#include "pc/test/enable_fake_media.h"
#include "pc/test/fake_codec_lookup_helper.h"
#include "pc/test/fake_data_channel_controller.h"
#include "pc/test/fake_peer_connection_base.h"
#include "pc/transport_stats.h"
#include "rtc_base/checks.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/thread.h"
#include "rtc_base/unique_id_generator.h"

namespace webrtc {

// Fake VoiceMediaChannel where the result of GetStats can be configured.
class FakeVoiceMediaSendChannelForStats : public FakeVoiceMediaSendChannel {
 public:
  explicit FakeVoiceMediaSendChannelForStats(TaskQueueBase* network_thread)
      : FakeVoiceMediaSendChannel(AudioOptions(), network_thread) {}

  void SetStats(const VoiceMediaInfo& voice_info) {
    send_stats_ = VoiceMediaSendInfo();
    send_stats_->senders = voice_info.senders;
    send_stats_->send_codecs = voice_info.send_codecs;
  }

  // VoiceMediaChannel overrides.
  bool GetStats(VoiceMediaSendInfo* info) override {
    if (send_stats_) {
      *info = *send_stats_;
      return true;
    }
    return false;
  }

 private:
  std::optional<VoiceMediaSendInfo> send_stats_;
};

class FakeVoiceMediaReceiveChannelForStats
    : public FakeVoiceMediaReceiveChannel {
 public:
  explicit FakeVoiceMediaReceiveChannelForStats(TaskQueueBase* network_thread)
      : FakeVoiceMediaReceiveChannel(AudioOptions(), network_thread) {}

  void SetStats(const VoiceMediaInfo& voice_info) {
    receive_stats_ = VoiceMediaReceiveInfo();
    receive_stats_->receivers = voice_info.receivers;
    receive_stats_->receive_codecs = voice_info.receive_codecs;
    receive_stats_->device_underrun_count = voice_info.device_underrun_count;
  }

  // VoiceMediaChannel overrides.
  bool GetStats(VoiceMediaReceiveInfo* info,
                bool get_and_clear_legacy_stats) override {
    if (receive_stats_) {
      *info = *receive_stats_;
      return true;
    }
    return false;
  }

 private:
  std::optional<VoiceMediaReceiveInfo> receive_stats_;
};

// Fake VideoMediaChannel where the result of GetStats can be configured.
class FakeVideoMediaSendChannelForStats : public FakeVideoMediaSendChannel {
 public:
  explicit FakeVideoMediaSendChannelForStats(TaskQueueBase* network_thread)
      : FakeVideoMediaSendChannel(VideoOptions(), network_thread) {}

  void SetStats(const VideoMediaInfo& video_info) {
    send_stats_ = VideoMediaSendInfo();
    send_stats_->senders = video_info.senders;
    send_stats_->aggregated_senders = video_info.aggregated_senders;
    send_stats_->send_codecs = video_info.send_codecs;
  }

  // VideoMediaChannel overrides.
  bool GetStats(VideoMediaSendInfo* info) override {
    if (send_stats_) {
      *info = *send_stats_;
      return true;
    }
    return false;
  }

 private:
  std::optional<VideoMediaSendInfo> send_stats_;
};

class FakeVideoMediaReceiveChannelForStats
    : public FakeVideoMediaReceiveChannel {
 public:
  explicit FakeVideoMediaReceiveChannelForStats(TaskQueueBase* network_thread)
      : FakeVideoMediaReceiveChannel(VideoOptions(), network_thread) {}

  void SetStats(const VideoMediaInfo& video_info) {
    receive_stats_ = VideoMediaReceiveInfo();
    receive_stats_->receivers = video_info.receivers;
    receive_stats_->receive_codecs = video_info.receive_codecs;
  }

  // VideoMediaChannel overrides.
  bool GetStats(VideoMediaReceiveInfo* info) override {
    if (receive_stats_) {
      *info = *receive_stats_;
      return true;
    }
    return false;
  }

 private:
  std::optional<VideoMediaReceiveInfo> receive_stats_;
};

constexpr bool kDefaultRtcpMuxRequired = true;
constexpr bool kDefaultSrtpRequired = true;

class VoiceChannelForTesting : public VoiceChannel {
 public:
  VoiceChannelForTesting(
      Thread* worker_thread,
      Thread* network_thread,
      Thread* signaling_thread,
      std::unique_ptr<VoiceMediaSendChannelInterface> send_channel,
      std::unique_ptr<VoiceMediaReceiveChannelInterface> receive_channel,
      const std::string& content_name,
      bool srtp_required,
      CryptoOptions crypto_options,
      UniqueRandomIdGenerator* ssrc_generator,
      std::string transport_name)
      : VoiceChannel(worker_thread,
                     network_thread,
                     signaling_thread,
                     std::move(send_channel),
                     std::move(receive_channel),
                     content_name,
                     srtp_required,
                     std::move(crypto_options),
                     ssrc_generator),
        test_transport_name_(std::move(transport_name)) {}

 private:
  absl::string_view transport_name() const override {
    return test_transport_name_;
  }

  const std::string test_transport_name_;
};

class VideoChannelForTesting : public VideoChannel {
 public:
  VideoChannelForTesting(
      Thread* worker_thread,
      Thread* network_thread,
      Thread* signaling_thread,
      std::unique_ptr<VideoMediaSendChannelInterface> send_channel,
      std::unique_ptr<VideoMediaReceiveChannelInterface> receive_channel,
      const std::string& content_name,
      bool srtp_required,
      CryptoOptions crypto_options,
      UniqueRandomIdGenerator* ssrc_generator,
      std::string transport_name)
      : VideoChannel(worker_thread,
                     network_thread,
                     signaling_thread,
                     std::move(send_channel),
                     std::move(receive_channel),
                     content_name,
                     srtp_required,
                     std::move(crypto_options),
                     ssrc_generator),
        test_transport_name_(std::move(transport_name)) {}

 private:
  absl::string_view transport_name() const override {
    return test_transport_name_;
  }

  const std::string test_transport_name_;
};

// This class is intended to be fed into the StatsCollector and
// RTCStatsCollector so that the stats functionality can be unit tested.
// Individual tests can configure this fake as needed to simulate scenarios
// under which to test the stats collectors.
class FakePeerConnectionForStats : public FakePeerConnectionBase {
 public:
  // TODO(steveanton): Add support for specifying separate threads to test
  // multi-threading correctness.
  FakePeerConnectionForStats()
      : network_thread_(Thread::Current()),
        worker_thread_(Thread::Current()),
        signaling_thread_(Thread::Current()),
        // TODO(hta): remove separate thread variables and use context.
        dependencies_(MakeDependencies()),
        context_(
            ConnectionContext::Create(CreateEnvironment(), &dependencies_)),
        local_streams_(StreamCollection::Create()),
        remote_streams_(StreamCollection::Create()),
        data_channel_controller_(network_thread_),
        codec_lookup_helper_(context_.get()) {}

  ~FakePeerConnectionForStats() {
    for (auto transceiver : transceivers_) {
      transceiver->internal()->ClearChannel();
    }
  }

  static PeerConnectionFactoryDependencies MakeDependencies() {
    PeerConnectionFactoryDependencies dependencies;
    dependencies.network_thread = Thread::Current();
    dependencies.worker_thread = Thread::Current();
    dependencies.signaling_thread = Thread::Current();
    EnableFakeMedia(dependencies);
    return dependencies;
  }

  scoped_refptr<StreamCollection> mutable_local_streams() {
    return local_streams_;
  }

  scoped_refptr<StreamCollection> mutable_remote_streams() {
    return remote_streams_;
  }

  scoped_refptr<RtpSenderInterface> AddSender(
      scoped_refptr<RtpSenderInternal> sender) {
    // TODO(steveanton): Switch tests to use RtpTransceivers directly.
    auto sender_proxy = RtpSenderProxyWithInternal<RtpSenderInternal>::Create(
        signaling_thread_, sender);
    GetOrCreateFirstTransceiverOfType(sender->media_type())
        ->internal()
        ->AddSender(sender_proxy);
    return sender_proxy;
  }

  void RemoveSender(scoped_refptr<RtpSenderInterface> sender) {
    GetOrCreateFirstTransceiverOfType(sender->media_type())
        ->internal()
        ->RemoveSender(sender.get());
  }

  scoped_refptr<RtpReceiverInterface> AddReceiver(
      scoped_refptr<RtpReceiverInternal> receiver) {
    // TODO(steveanton): Switch tests to use RtpTransceivers directly.
    auto receiver_proxy =
        RtpReceiverProxyWithInternal<RtpReceiverInternal>::Create(
            signaling_thread_, worker_thread_, receiver);
    GetOrCreateFirstTransceiverOfType(receiver->media_type())
        ->internal()
        ->AddReceiver(receiver_proxy);
    return receiver_proxy;
  }

  void RemoveReceiver(scoped_refptr<RtpReceiverInterface> receiver) {
    GetOrCreateFirstTransceiverOfType(receiver->media_type())
        ->internal()
        ->RemoveReceiver(receiver.get());
  }

  std::pair<FakeVoiceMediaSendChannelForStats*,
            FakeVoiceMediaReceiveChannelForStats*>
  AddVoiceChannel(const std::string& mid,
                  const std::string& transport_name,
                  VoiceMediaInfo initial_stats = VoiceMediaInfo()) {
    auto voice_media_send_channel =
        std::make_unique<FakeVoiceMediaSendChannelForStats>(network_thread_);
    auto voice_media_receive_channel =
        std::make_unique<FakeVoiceMediaReceiveChannelForStats>(network_thread_);
    auto* voice_media_send_channel_ptr = voice_media_send_channel.get();
    auto* voice_media_receive_channel_ptr = voice_media_receive_channel.get();
    auto voice_channel = std::make_unique<VoiceChannelForTesting>(
        worker_thread_, network_thread_, signaling_thread_,
        std::move(voice_media_send_channel),
        std::move(voice_media_receive_channel), mid, kDefaultSrtpRequired,
        CryptoOptions(), context_->ssrc_generator(), transport_name);
    auto transceiver =
        GetOrCreateFirstTransceiverOfType(webrtc::MediaType::AUDIO)->internal();
    if (transceiver->channel()) {
      // This transceiver already has a channel, create a new one.
      transceiver =
          CreateTransceiverOfType(webrtc::MediaType::AUDIO)->internal();
    }
    RTC_DCHECK(!transceiver->channel());
    transceiver->SetChannel(std::move(voice_channel),
                            [](const std::string&) { return nullptr; });
    voice_media_send_channel_ptr->SetStats(initial_stats);
    voice_media_receive_channel_ptr->SetStats(initial_stats);
    return std::make_pair(voice_media_send_channel_ptr,
                          voice_media_receive_channel_ptr);
  }

  std::pair<FakeVideoMediaSendChannelForStats*,
            FakeVideoMediaReceiveChannelForStats*>
  AddVideoChannel(const std::string& mid,
                  const std::string& transport_name,
                  VideoMediaInfo initial_stats = VideoMediaInfo()) {
    auto video_media_send_channel =
        std::make_unique<FakeVideoMediaSendChannelForStats>(network_thread_);
    auto video_media_receive_channel =
        std::make_unique<FakeVideoMediaReceiveChannelForStats>(network_thread_);
    auto video_media_send_channel_ptr = video_media_send_channel.get();
    auto video_media_receive_channel_ptr = video_media_receive_channel.get();
    auto video_channel = std::make_unique<VideoChannelForTesting>(
        worker_thread_, network_thread_, signaling_thread_,
        std::move(video_media_send_channel),
        std::move(video_media_receive_channel), mid, kDefaultSrtpRequired,
        CryptoOptions(), context_->ssrc_generator(), transport_name);
    auto transceiver =
        GetOrCreateFirstTransceiverOfType(webrtc::MediaType::VIDEO)->internal();
    if (transceiver->channel()) {
      // This transceiver already has a channel, create a new one.
      transceiver =
          CreateTransceiverOfType(webrtc::MediaType::VIDEO)->internal();
    }
    RTC_DCHECK(!transceiver->channel());
    transceiver->SetChannel(std::move(video_channel),
                            [](const std::string&) { return nullptr; });
    video_media_send_channel_ptr->SetStats(initial_stats);
    video_media_receive_channel_ptr->SetStats(initial_stats);
    return std::make_pair(video_media_send_channel_ptr,
                          video_media_receive_channel_ptr);
  }

  void AddSctpDataChannel(const std::string& label) {
    AddSctpDataChannel(label, InternalDataChannelInit());
  }

  void AddSctpDataChannel(const std::string& label,
                          const InternalDataChannelInit& init) {
    // TODO(bugs.webrtc.org/11547): Supply a separate network thread.
    AddSctpDataChannel(SctpDataChannel::Create(
        data_channel_controller_.weak_ptr(), label, false, init,
        Thread::Current(), Thread::Current()));
  }

  void AddSctpDataChannel(scoped_refptr<SctpDataChannel> data_channel) {
    sctp_data_channels_.push_back(data_channel);
  }

  void SetTransportStats(const std::string& transport_name,
                         const TransportChannelStats& channel_stats) {
    SetTransportStats(transport_name,
                      std::vector<TransportChannelStats>{channel_stats});
  }

  void SetTransportStats(
      const std::string& transport_name,
      const std::vector<TransportChannelStats>& channel_stats_list) {
    TransportStats transport_stats;
    transport_stats.transport_name = transport_name;
    transport_stats.channel_stats = channel_stats_list;
    transport_stats_by_name_[transport_name] = transport_stats;
  }

  void SetCallStats(const Call::Stats& call_stats) { call_stats_ = call_stats; }

  void SetAudioDeviceStats(
      std::optional<AudioDeviceModule::Stats> audio_device_stats) {
    audio_device_stats_ = audio_device_stats;
  }

  void SetLocalCertificate(const std::string& transport_name,
                           scoped_refptr<RTCCertificate> certificate) {
    local_certificates_by_transport_[transport_name] = certificate;
  }

  void SetRemoteCertChain(const std::string& transport_name,
                          std::unique_ptr<SSLCertChain> chain) {
    remote_cert_chains_by_transport_[transport_name] = std::move(chain);
  }

  // PeerConnectionInterface overrides.

  scoped_refptr<StreamCollectionInterface> local_streams() override {
    return local_streams_;
  }

  scoped_refptr<StreamCollectionInterface> remote_streams() override {
    return remote_streams_;
  }

  std::vector<scoped_refptr<RtpSenderInterface>> GetSenders() const override {
    std::vector<scoped_refptr<RtpSenderInterface>> senders;
    for (auto transceiver : transceivers_) {
      for (auto sender : transceiver->internal()->senders()) {
        senders.push_back(sender);
      }
    }
    return senders;
  }

  std::vector<scoped_refptr<RtpReceiverInterface>> GetReceivers()
      const override {
    std::vector<scoped_refptr<RtpReceiverInterface>> receivers;
    for (auto transceiver : transceivers_) {
      for (auto receiver : transceiver->internal()->receivers()) {
        receivers.push_back(receiver);
      }
    }
    return receivers;
  }

  // PeerConnectionInternal overrides.

  Thread* network_thread() const override { return network_thread_; }

  Thread* worker_thread() const override { return worker_thread_; }

  Thread* signaling_thread() const override { return signaling_thread_; }

  std::vector<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  GetTransceiversInternal() const override {
    return transceivers_;
  }

  std::vector<DataChannelStats> GetDataChannelStats() const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    std::vector<DataChannelStats> stats;
    for (const auto& channel : sctp_data_channels_)
      stats.push_back(channel->GetStats());
    return stats;
  }

  CandidateStatsList GetPooledCandidateStats() const override { return {}; }

  std::map<std::string, TransportStats> GetTransportStatsByNames(
      const std::set<std::string>& transport_names) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    std::map<std::string, TransportStats> transport_stats_by_name;
    for (const std::string& transport_name : transport_names) {
      transport_stats_by_name[transport_name] =
          GetTransportStatsByName(transport_name);
    }
    return transport_stats_by_name;
  }

  Call::Stats GetCallStats() override { return call_stats_; }

  std::optional<AudioDeviceModule::Stats> GetAudioDeviceStats() override {
    return audio_device_stats_;
  }

  bool GetLocalCertificate(
      const std::string& transport_name,
      scoped_refptr<RTCCertificate>* certificate) override {
    auto it = local_certificates_by_transport_.find(transport_name);
    if (it != local_certificates_by_transport_.end()) {
      *certificate = it->second;
      return true;
    } else {
      return false;
    }
  }

  std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain(
      const std::string& transport_name) override {
    auto it = remote_cert_chains_by_transport_.find(transport_name);
    if (it != remote_cert_chains_by_transport_.end()) {
      return it->second->Clone();
    } else {
      return nullptr;
    }
  }
  PayloadTypePicker& payload_type_picker() { return payload_type_picker_; }

 private:
  TransportStats GetTransportStatsByName(const std::string& transport_name) {
    auto it = transport_stats_by_name_.find(transport_name);
    if (it != transport_stats_by_name_.end()) {
      // If specific transport stats have been specified, return those.
      return it->second;
    }
    // Otherwise, generate some dummy stats.
    TransportChannelStats channel_stats;
    channel_stats.component = ICE_CANDIDATE_COMPONENT_RTP;
    TransportStats transport_stats;
    transport_stats.transport_name = transport_name;
    transport_stats.channel_stats.push_back(channel_stats);
    return transport_stats;
  }

  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  GetOrCreateFirstTransceiverOfType(webrtc::MediaType media_type) {
    for (auto transceiver : transceivers_) {
      if (transceiver->internal()->media_type() == media_type) {
        return transceiver;
      }
    }
    return CreateTransceiverOfType(media_type);
  }

  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  CreateTransceiverOfType(webrtc::MediaType media_type) {
    auto transceiver = RtpTransceiverProxyWithInternal<RtpTransceiver>::Create(
        signaling_thread_,
        make_ref_counted<RtpTransceiver>(media_type, context_.get(),
                                         &codec_lookup_helper_));
    transceivers_.push_back(transceiver);
    return transceiver;
  }

  Thread* const network_thread_;
  Thread* const worker_thread_;
  Thread* const signaling_thread_;

  PeerConnectionFactoryDependencies dependencies_;
  scoped_refptr<ConnectionContext> context_;

  scoped_refptr<StreamCollection> local_streams_;
  scoped_refptr<StreamCollection> remote_streams_;

  std::vector<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
      transceivers_;

  FakeDataChannelController data_channel_controller_;

  std::vector<scoped_refptr<SctpDataChannel>> sctp_data_channels_;

  std::map<std::string, TransportStats> transport_stats_by_name_;

  Call::Stats call_stats_;

  std::optional<AudioDeviceModule::Stats> audio_device_stats_;

  std::map<std::string, scoped_refptr<RTCCertificate>>
      local_certificates_by_transport_;
  std::map<std::string, std::unique_ptr<SSLCertChain>>
      remote_cert_chains_by_transport_;
  PayloadTypePicker payload_type_picker_;
  FakeCodecLookupHelper codec_lookup_helper_;
};

}  // namespace webrtc

#endif  // PC_TEST_FAKE_PEER_CONNECTION_FOR_STATS_H_
