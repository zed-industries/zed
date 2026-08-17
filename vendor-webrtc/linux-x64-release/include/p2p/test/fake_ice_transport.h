/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_FAKE_ICE_TRANSPORT_H_
#define P2P_TEST_FAKE_ICE_TRANSPORT_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/algorithm/container.h"
#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/candidate.h"
#include "api/ice_transport_interface.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/enums.h"
#include "api/transport/stun.h"
#include "api/units/time_delta.h"
#include "p2p/base/candidate_pair_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/connection_info.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/port.h"
#include "p2p/base/transport_description.h"
#include "p2p/dtls/dtls_stun_piggyback_callbacks.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/byte_buffer.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/logging.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/socket.h"
#include "rtc_base/task_queue_for_test.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/time_utils.h"
#include "test/explicit_key_value_config.h"

namespace webrtc {
using ::webrtc::SafeTask;
using ::webrtc::TimeDelta;

// All methods must be called on the network thread (which is either the thread
// calling the constructor, or the separate thread explicitly passed to the
// constructor).
class FakeIceTransport : public IceTransportInternal {
 public:
  explicit FakeIceTransport(absl::string_view name,
                            int component,
                            Thread* network_thread = nullptr,
                            absl::string_view field_trials_string = "")
      : name_(name),
        component_(component),
        network_thread_(network_thread ? network_thread : Thread::Current()),
        field_trials_(field_trials_string) {
    RTC_DCHECK(network_thread_);
  }

  // Must be called either on the network thread, or after the network thread
  // has been shut down.
  ~FakeIceTransport() override {
    if (dest_ && dest_->dest_ == this) {
      dest_->dest_ = nullptr;
    }
  }

  // If async, will send packets by "Post"-ing to message queue instead of
  // synchronously "Send"-ing.
  void SetAsync(bool async) {
    RTC_DCHECK_RUN_ON(network_thread_);
    async_ = async;
  }
  void SetAsyncDelay(int delay_ms) {
    RTC_DCHECK_RUN_ON(network_thread_);
    async_delay_ms_ = delay_ms;
  }

  // SetWritable, SetReceiving and SetDestination are the main methods that can
  // be used for testing, to simulate connectivity or lack thereof.
  void SetWritable(bool writable) {
    RTC_DCHECK_RUN_ON(network_thread_);
    set_writable(writable);
  }
  void SetReceiving(bool receiving) {
    RTC_DCHECK_RUN_ON(network_thread_);
    set_receiving(receiving);
  }

  // Simulates the two transports connecting to each other.
  // If `asymmetric` is true this method only affects this FakeIceTransport.
  // If false, it affects `dest` as well.
  void SetDestination(FakeIceTransport* dest, bool asymmetric = false) {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (dest == dest_) {
      return;
    }
    RTC_DCHECK(!dest || !dest_)
        << "Changing fake destination from one to another is not supported.";
    if (dest) {
      // This simulates the delivery of candidates.
      dest_ = dest;
      set_writable(true);
      if (!asymmetric) {
        dest->SetDestination(this, true);
      }
    } else {
      // Simulates loss of connectivity, by asymmetrically forgetting dest_.
      dest_ = nullptr;
      set_writable(false);
    }
  }

  void SetDestinationNotWritable(FakeIceTransport* dest) {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (dest == dest_) {
      return;
    }
    RTC_DCHECK(!dest || !dest_)
        << "Changing fake destination from one to another is not supported.";

    if (dest) {
      RTC_DCHECK_RUN_ON(dest->network_thread_);
      dest->dest_ = this;
    } else if (dest_) {
      RTC_DCHECK_RUN_ON(dest_->network_thread_);
      dest_->dest_ = nullptr;
    }
    dest_ = dest;
  }

  void SetTransportState(IceTransportState state,
                         IceTransportStateInternal legacy_state) {
    RTC_DCHECK_RUN_ON(network_thread_);
    transport_state_ = state;
    legacy_transport_state_ = legacy_state;
    SignalIceTransportStateChanged(this);
  }

  void SetConnectionCount(size_t connection_count) {
    RTC_DCHECK_RUN_ON(network_thread_);
    size_t old_connection_count = connection_count_;
    connection_count_ = connection_count;
    if (connection_count) {
      had_connection_ = true;
    }
    // In this fake transport channel, `connection_count_` determines the
    // transport state.
    if (connection_count_ < old_connection_count) {
      SignalStateChanged(this);
    }
  }

  void SetCandidatesGatheringComplete() {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (gathering_state_ != webrtc::kIceGatheringComplete) {
      gathering_state_ = webrtc::kIceGatheringComplete;
      SendGatheringStateEvent();
    }
  }

  // Convenience functions for accessing ICE config and other things.
  int receiving_timeout() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return ice_config_.receiving_timeout_or_default();
  }
  bool gather_continually() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return ice_config_.gather_continually();
  }
  const Candidates& remote_candidates() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_candidates_;
  }

  // Fake IceTransportInternal implementation.
  const std::string& transport_name() const override { return name_; }
  int component() const override { return component_; }
  IceMode remote_ice_mode() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_ice_mode_;
  }
  const IceParameters* local_ice_parameters() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return &ice_parameters_;
  }
  const IceParameters* remote_ice_parameters() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return &remote_ice_parameters_;
  }

  IceTransportStateInternal GetState() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (legacy_transport_state_) {
      return *legacy_transport_state_;
    }

    if (connection_count_ == 0) {
      return had_connection_ ? IceTransportStateInternal::STATE_FAILED
                             : IceTransportStateInternal::STATE_INIT;
    }

    if (connection_count_ == 1) {
      return IceTransportStateInternal::STATE_COMPLETED;
    }

    return IceTransportStateInternal::STATE_CONNECTING;
  }

  IceTransportState GetIceTransportState() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (transport_state_) {
      return *transport_state_;
    }

    if (connection_count_ == 0) {
      return had_connection_ ? IceTransportState::kFailed
                             : IceTransportState::kNew;
    }

    if (connection_count_ == 1) {
      return IceTransportState::kCompleted;
    }

    return IceTransportState::kConnected;
  }

  void SetIceRole(IceRole role) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    role_ = role;
  }
  IceRole GetIceRole() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return role_;
  }
  void SetIceParameters(const IceParameters& ice_params) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    ice_parameters_ = ice_params;
  }
  void SetRemoteIceParameters(const IceParameters& params) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    remote_ice_parameters_ = params;
  }

  void SetRemoteIceMode(IceMode mode) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    remote_ice_mode_ = mode;
  }

  void MaybeStartGathering() override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (gathering_state_ == webrtc::kIceGatheringNew) {
      gathering_state_ = webrtc::kIceGatheringGathering;
      SendGatheringStateEvent();
    }
  }

  IceGatheringState gathering_state() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return gathering_state_;
  }

  void SetIceConfig(const IceConfig& config) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    ice_config_ = config;
  }

  const IceConfig& config() const override { return ice_config_; }

  void AddRemoteCandidate(const Candidate& candidate) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    remote_candidates_.push_back(candidate);
  }
  void RemoveRemoteCandidate(const Candidate& candidate) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    auto it = absl::c_find(remote_candidates_, candidate);
    if (it == remote_candidates_.end()) {
      RTC_LOG(LS_INFO) << "Trying to remove a candidate which doesn't exist.";
      return;
    }

    remote_candidates_.erase(it);
  }

  void RemoveAllRemoteCandidates() override {
    RTC_DCHECK_RUN_ON(network_thread_);
    remote_candidates_.clear();
  }

  bool GetStats(IceTransportStats* ice_transport_stats) override {
    CandidateStats candidate_stats;
    ConnectionInfo candidate_pair_stats;
    ice_transport_stats->candidate_stats_list.clear();
    ice_transport_stats->candidate_stats_list.push_back(candidate_stats);
    ice_transport_stats->connection_infos.clear();
    ice_transport_stats->connection_infos.push_back(candidate_pair_stats);
    return true;
  }

  std::optional<int> GetRttEstimate() override { return rtt_estimate_; }

  const Connection* selected_connection() const override { return nullptr; }
  std::optional<const CandidatePair> GetSelectedCandidatePair() const override {
    return std::nullopt;
  }

  // Fake PacketTransportInternal implementation.
  bool writable() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return writable_;
  }
  bool receiving() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return receiving_;
  }
  // If combine is enabled, every two consecutive packets to be sent with
  // "SendPacket" will be combined into one outgoing packet.
  void combine_outgoing_packets(bool combine) {
    RTC_DCHECK_RUN_ON(network_thread_);
    combine_outgoing_packets_ = combine;
  }
  int SendPacket(const char* data,
                 size_t len,
                 const AsyncSocketPacketOptions& options,
                 int flags) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (!dest_) {
      return -1;
    }

    send_packet_.AppendData(data, len);
    if (!combine_outgoing_packets_ || send_packet_.size() > len) {
      CopyOnWriteBuffer packet(std::move(send_packet_));
      if (!SendPacketInternal(packet, options, flags)) {
        return -1;
      }
    }

    SentPacketInfo sent_packet(options.packet_id, webrtc::TimeMillis());
    SignalSentPacket(this, sent_packet);
    return static_cast<int>(len);
  }

  int SetOption(Socket::Option opt, int value) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    socket_options_[opt] = value;
    return true;
  }
  bool GetOption(Socket::Option opt, int* value) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    auto it = socket_options_.find(opt);
    if (it != socket_options_.end()) {
      *value = it->second;
      return true;
    } else {
      return false;
    }
  }

  int GetError() override { return 0; }

  CopyOnWriteBuffer last_sent_packet() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return last_sent_packet_;
  }

  std::optional<NetworkRoute> network_route() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return network_route_;
  }
  void SetNetworkRoute(std::optional<NetworkRoute> network_route) {
    RTC_DCHECK_RUN_ON(network_thread_);
    network_route_ = network_route;
    SendTask(network_thread_, [this] {
      RTC_DCHECK_RUN_ON(network_thread_);
      SignalNetworkRouteChanged(network_route_);
    });
  }

  // If `func` return TRUE means that packet will be dropped.
  void set_packet_send_filter(
      absl::AnyInvocable<bool(const char* data,
                              size_t len,
                              const webrtc::AsyncSocketPacketOptions& options,
                              int /* flags */)> func) {
    RTC_DCHECK_RUN_ON(network_thread_);
    packet_send_filter_func_ = std::move(func);
  }

  // If `func` return TRUE means that packet will be dropped.
  void set_packet_recv_filter(
      absl::AnyInvocable<bool(const CopyOnWriteBuffer& packet,
                              uint32_t time_ms)> func) {
    RTC_DCHECK_RUN_ON(network_thread_);
    packet_recv_filter_func_ = std::move(func);
  }

  void set_rtt_estimate(std::optional<int> value, bool set_async = false) {
    rtt_estimate_ = value;
    if (value && set_async) {
      SetAsync(true);
      SetAsyncDelay(*value / 2);
    }
  }

  void ResetDtlsStunPiggybackCallbacks() override {
    dtls_stun_piggyback_callbacks_.reset();
  }
  void SetDtlsStunPiggybackCallbacks(
      DtlsStunPiggybackCallbacks&& callbacks) override {
    if (!callbacks.empty()) {
      RTC_LOG(LS_INFO) << name_ << ": SetDtlsStunPiggybackCallbacks";
    } else if (!dtls_stun_piggyback_callbacks_.empty()) {
      RTC_LOG(LS_INFO) << name_ << ": ResetDtlsStunPiggybackCallbacks";
    }
    dtls_stun_piggyback_callbacks_ = std::move(callbacks);
  }

  bool SendIcePing() {
    RTC_DCHECK_RUN_ON(network_thread_);
    RTC_DLOG(LS_INFO) << name_ << ": SendIcePing()";
    last_sent_ping_timestamp_ = webrtc::TimeMicros();
    auto msg = std::make_unique<IceMessage>(STUN_BINDING_REQUEST);
    MaybeAddDtlsPiggybackingAttributes(msg.get());
    msg->AddFingerprint();
    ByteBufferWriter buf;
    msg->Write(&buf);
    AsyncSocketPacketOptions options;
    options.info_signaled_after_sent.packet_type =
        PacketType::kIceConnectivityCheck;
    SendPacketInternal(CopyOnWriteBuffer(buf.DataView()), options, 0);
    return true;
  }

  void MaybeAddDtlsPiggybackingAttributes(StunMessage* msg) {
    if (dtls_stun_piggyback_callbacks_.empty()) {
      return;
    }

    const auto& [attr, ack] = dtls_stun_piggyback_callbacks_.send_data(
        static_cast<StunMessageType>(msg->type()));

    RTC_DLOG(LS_INFO) << name_ << ": Adding attr: " << attr.has_value()
                      << " ack: " << ack.has_value() << " to stun message: "
                      << StunMethodToString(msg->type());

    if (attr) {
      msg->AddAttribute(std::make_unique<StunByteStringAttribute>(
          STUN_ATTR_META_DTLS_IN_STUN, *attr));
    }
    if (ack) {
      msg->AddAttribute(std::make_unique<StunByteStringAttribute>(
          STUN_ATTR_META_DTLS_IN_STUN_ACK, *ack));
    }
  }

  bool SendIcePingConf() {
    RTC_DCHECK_RUN_ON(network_thread_);
    RTC_DLOG(LS_INFO) << name_ << ": SendIcePingConf()";
    auto msg = std::make_unique<IceMessage>(STUN_BINDING_RESPONSE);
    MaybeAddDtlsPiggybackingAttributes(msg.get());
    msg->AddFingerprint();
    ByteBufferWriter buf;
    msg->Write(&buf);
    AsyncSocketPacketOptions options;
    options.info_signaled_after_sent.packet_type =
        PacketType::kIceConnectivityCheckResponse;
    SendPacketInternal(CopyOnWriteBuffer(buf.DataView()), options, 0);
    return true;
  }

  int GetCountOfReceivedStunMessages(int type) {
    return received_stun_messages_per_type[type];
  }

  int GetCountOfReceivedPackets() { return received_packets_; }

  const FieldTrialsView* field_trials() const { return &field_trials_; }

  void set_drop_non_stun_unless_writable(bool value) {
    drop_non_stun_unless_writable_ = value;
  }

 private:
  void set_writable(bool writable)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(network_thread_) {
    if (writable_ == writable) {
      return;
    }
    RTC_LOG(LS_INFO) << "Change writable_ to " << writable;
    writable_ = writable;
    if (writable_) {
      SignalReadyToSend(this);
    }
    SignalWritableState(this);
  }

  void set_receiving(bool receiving)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(network_thread_) {
    if (receiving_ == receiving) {
      return;
    }
    receiving_ = receiving;
    SignalReceivingState(this);
  }

  bool SendPacketInternal(const CopyOnWriteBuffer& packet,
                          const AsyncSocketPacketOptions& options,
                          int flags)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(network_thread_) {
    last_sent_packet_ = packet;
    bool is_stun =
        StunMessage::ValidateFingerprint(packet.data<char>(), packet.size());
    if (packet_send_filter_func_ &&
        packet_send_filter_func_(packet.data<char>(), packet.size(), options,
                                 flags)) {
      RTC_LOG(LS_INFO) << name_ << ": dropping packet len=" << packet.size()
                       << ", data[0]: "
                       << static_cast<uint8_t>(packet.data()[0]);
      return false;
    }

    if (drop_non_stun_unless_writable_ && !writable_ && !is_stun) {
      RTC_LOG(LS_INFO) << name_
                       << ": dropping non stun packet len=" << packet.size()
                       << ", data[0]: "
                       << static_cast<uint8_t>(packet.data()[0]);
      return false;
    }
    if (async_) {
      network_thread_->PostDelayedTask(
          SafeTask(task_safety_.flag(),
                   [this, packet] {
                     RTC_DCHECK_RUN_ON(network_thread_);
                     if (dest_) {
                       dest_->ReceivePacketInternal(packet);
                     }
                   }),
          TimeDelta::Millis(async_delay_ms_));
    } else {
      if (dest_) {
        dest_->ReceivePacketInternal(packet);
      }
    }
    return true;
  }

  void ReceivePacketInternal(const CopyOnWriteBuffer& packet) {
    RTC_DCHECK_RUN_ON(network_thread_);
    auto now = webrtc::TimeMicros();
    if (auto msg = GetStunMessage(packet)) {
      RTC_LOG(LS_INFO) << name_ << ": RECV STUN message: "
                       << ", data[0]: "
                       << static_cast<uint8_t>(packet.data()[0]);

      const auto* dtls_piggyback_attr =
          msg->GetByteString(STUN_ATTR_META_DTLS_IN_STUN);
      const auto* dtls_piggyback_ack =
          msg->GetByteString(STUN_ATTR_META_DTLS_IN_STUN_ACK);
      RTC_DLOG(LS_INFO) << name_ << ": Got STUN message: "
                        << StunMethodToString(msg->type())
                        << " attr: " << (dtls_piggyback_attr != nullptr)
                        << " ack: " << (dtls_piggyback_ack != nullptr);
      if (!dtls_stun_piggyback_callbacks_.empty()) {
        dtls_stun_piggyback_callbacks_.recv_data(dtls_piggyback_attr,
                                                 dtls_piggyback_ack);
      }

      if (msg->type() == STUN_BINDING_RESPONSE) {
        if (!rtt_estimate_ && last_sent_ping_timestamp_) {
          rtt_estimate_ = (now - *last_sent_ping_timestamp_) / 1000;
        }
        set_writable(true);
      }

      received_stun_messages_per_type[msg->type()]++;
      return;
    }

    if (packet_recv_filter_func_ && packet_recv_filter_func_(packet, now)) {
      RTC_DLOG(LS_INFO) << name_
                        << ": dropping packet at receiver len=" << packet.size()
                        << ", data[0]: "
                        << static_cast<uint8_t>(packet.data()[0]);
    } else {
      received_packets_++;
      NotifyPacketReceived(ReceivedIpPacket::CreateFromLegacy(
          packet.data(), packet.size(), now));
    }
  }

  std::unique_ptr<IceMessage> GetStunMessage(const CopyOnWriteBuffer& packet) {
    if (!StunMessage::ValidateFingerprint(packet.data<char>(), packet.size())) {
      return nullptr;
    }

    std::unique_ptr<IceMessage> stun_msg(new IceMessage());
    ByteBufferReader buf(MakeArrayView(packet.data(), packet.size()));
    RTC_CHECK(stun_msg->Read(&buf));
    return stun_msg;
  }

  const std::string name_;
  const int component_;
  FakeIceTransport* dest_ RTC_GUARDED_BY(network_thread_) = nullptr;
  bool async_ RTC_GUARDED_BY(network_thread_) = false;
  int async_delay_ms_ RTC_GUARDED_BY(network_thread_) = 0;
  Candidates remote_candidates_ RTC_GUARDED_BY(network_thread_);
  IceConfig ice_config_ RTC_GUARDED_BY(network_thread_);
  IceRole role_ RTC_GUARDED_BY(network_thread_) = ICEROLE_UNKNOWN;
  IceParameters ice_parameters_ RTC_GUARDED_BY(network_thread_);
  IceParameters remote_ice_parameters_ RTC_GUARDED_BY(network_thread_);
  IceMode remote_ice_mode_ RTC_GUARDED_BY(network_thread_) = ICEMODE_FULL;
  size_t connection_count_ RTC_GUARDED_BY(network_thread_) = 0;
  std::optional<IceTransportState> transport_state_
      RTC_GUARDED_BY(network_thread_);
  std::optional<IceTransportStateInternal> legacy_transport_state_
      RTC_GUARDED_BY(network_thread_);
  IceGatheringState gathering_state_ RTC_GUARDED_BY(network_thread_) =
      webrtc::kIceGatheringNew;
  bool had_connection_ RTC_GUARDED_BY(network_thread_) = false;
  bool writable_ RTC_GUARDED_BY(network_thread_) = false;
  bool receiving_ RTC_GUARDED_BY(network_thread_) = false;
  bool combine_outgoing_packets_ RTC_GUARDED_BY(network_thread_) = false;
  CopyOnWriteBuffer send_packet_ RTC_GUARDED_BY(network_thread_);
  std::optional<NetworkRoute> network_route_ RTC_GUARDED_BY(network_thread_);
  std::map<Socket::Option, int> socket_options_ RTC_GUARDED_BY(network_thread_);
  CopyOnWriteBuffer last_sent_packet_ RTC_GUARDED_BY(network_thread_);
  Thread* const network_thread_;
  ScopedTaskSafetyDetached task_safety_;
  std::optional<int> rtt_estimate_;
  std::optional<int64_t> last_sent_ping_timestamp_;

  // If filter func return TRUE means that packet will be dropped.
  absl::AnyInvocable<
      // NOLINTNEXTLINE(readability/casting) - not a cast; false positive!
      bool(const char*, size_t, const AsyncSocketPacketOptions&, int)>
      packet_send_filter_func_ RTC_GUARDED_BY(network_thread_) = nullptr;
  absl::AnyInvocable<bool(const CopyOnWriteBuffer&, uint64_t)>
      packet_recv_filter_func_ RTC_GUARDED_BY(network_thread_) = nullptr;
  DtlsStunPiggybackCallbacks dtls_stun_piggyback_callbacks_;
  std::map<int, int> received_stun_messages_per_type;
  int received_packets_ = 0;
  test::ExplicitKeyValueConfig field_trials_;
  bool drop_non_stun_unless_writable_ = false;
};

class FakeIceTransportWrapper : public IceTransportInterface {
 public:
  explicit FakeIceTransportWrapper(std::unique_ptr<FakeIceTransport> internal)
      : internal_(std::move(internal)) {}

  IceTransportInternal* internal() override { return internal_.get(); }

 private:
  std::unique_ptr<FakeIceTransport> internal_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeIceTransport;
using ::webrtc::FakeIceTransportWrapper;
using ::webrtc::SafeTask;
using ::webrtc::TimeDelta;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_FAKE_ICE_TRANSPORT_H_
