/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_SCTP_DCSCTP_TRANSPORT_H_
#define MEDIA_SCTP_DCSCTP_TRANSPORT_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/dtls_transport_interface.h"
#include "api/environment/environment.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/sctp_transport_interface.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/data_channel_transport_interface.h"
#include "media/sctp/sctp_transport_internal.h"
#include "net/dcsctp/public/dcsctp_message.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/public/dcsctp_socket_factory.h"
#include "net/dcsctp/public/timeout.h"
#include "net/dcsctp/public/types.h"
#include "net/dcsctp/timer/task_queue_timeout.h"
#include "p2p/base/packet_transport_internal.h"
#include "p2p/dtls/dtls_transport_internal.h"
#include "rtc_base/containers/flat_map.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/random.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class DcSctpTransport : public SctpTransportInternal,
                        public dcsctp::DcSctpSocketCallbacks,
                        public sigslot::has_slots<> {
 public:
  DcSctpTransport(const Environment& env,
                  Thread* network_thread,
                  DtlsTransportInternal* transport);
  DcSctpTransport(const Environment& env,
                  Thread* network_thread,
                  DtlsTransportInternal* transport,
                  std::unique_ptr<dcsctp::DcSctpSocketFactory> socket_factory);
  ~DcSctpTransport() override;

  // webrtc::SctpTransportInternal
  void SetOnConnectedCallback(std::function<void()> callback) override;
  void SetDataChannelSink(DataChannelSink* sink) override;
  void SetDtlsTransport(DtlsTransportInternal* transport) override;
  bool Start(const SctpOptions& options) override;
  bool OpenStream(int sid, PriorityValue priority) override;
  bool ResetStream(int sid) override;
  RTCError SendData(int sid,
                    const SendDataParams& params,
                    const CopyOnWriteBuffer& payload) override;
  bool ReadyToSendData() override;
  int max_message_size() const override;
  std::optional<int> max_outbound_streams() const override;
  std::optional<int> max_inbound_streams() const override;
  size_t buffered_amount(int sid) const override;
  size_t buffered_amount_low_threshold(int sid) const override;
  void SetBufferedAmountLowThreshold(int sid, size_t bytes) override;
  void set_debug_name_for_testing(const char* debug_name) override;

 private:
  // dcsctp::DcSctpSocketCallbacks
  dcsctp::SendPacketStatus SendPacketWithStatus(
      ArrayView<const uint8_t> data) override;
  std::unique_ptr<dcsctp::Timeout> CreateTimeout(
      TaskQueueBase::DelayPrecision precision) override;
  dcsctp::TimeMs TimeMillis() override;
  uint32_t GetRandomInt(uint32_t low, uint32_t high) override;
  void OnTotalBufferedAmountLow() override;
  void OnBufferedAmountLow(dcsctp::StreamID stream_id) override;
  void OnMessageReceived(dcsctp::DcSctpMessage message) override;
  void OnError(dcsctp::ErrorKind error, absl::string_view message) override;
  void OnAborted(dcsctp::ErrorKind error, absl::string_view message) override;
  void OnConnected() override;
  void OnClosed() override;
  void OnConnectionRestarted() override;
  void OnStreamsResetFailed(ArrayView<const dcsctp::StreamID> outgoing_streams,
                            absl::string_view reason) override;
  void OnStreamsResetPerformed(
      ArrayView<const dcsctp::StreamID> outgoing_streams) override;
  void OnIncomingStreamsReset(
      ArrayView<const dcsctp::StreamID> incoming_streams) override;

  // Transport callbacks
  void ConnectTransportSignals();
  void DisconnectTransportSignals();
  void OnTransportWritableState(PacketTransportInternal* transport);
  void OnTransportReadPacket(PacketTransportInternal* transport,
                             const ReceivedIpPacket& packet);
  void OnDtlsTransportState(DtlsTransportInternal* transport,
                            webrtc::DtlsTransportState);
  void MaybeConnectSocket();

  Thread* network_thread_;
  DtlsTransportInternal* transport_;
  Environment env_;
  Random random_;

  std::unique_ptr<dcsctp::DcSctpSocketFactory> socket_factory_;
  dcsctp::TaskQueueTimeoutFactory task_queue_timeout_factory_;
  std::unique_ptr<dcsctp::DcSctpSocketInterface> socket_;
  std::string debug_name_ = "DcSctpTransport";
  CopyOnWriteBuffer receive_buffer_;

  // Used to keep track of the state of data channels.
  // Reset needs to happen both ways before signaling the transport
  // is closed.
  struct StreamState {
    // True when the local connection has initiated the reset.
    // If a connection receives a reset for a stream that isn't
    // already being reset locally, it needs to fire the signal
    // SignalClosingProcedureStartedRemotely.
    bool closure_initiated = false;
    // True when the local connection received OnIncomingStreamsReset
    bool incoming_reset_done = false;
    // True when the local connection received OnStreamsResetPerformed
    bool outgoing_reset_done = false;
    // Priority of the stream according to RFC 8831, section 6.4
    dcsctp::StreamPriority priority =
        dcsctp::StreamPriority(PriorityValue(webrtc::Priority::kLow).value());
  };

  // Map of all currently open or closing data channels
  flat_map<dcsctp::StreamID, StreamState> stream_states_
      RTC_GUARDED_BY(network_thread_);
  bool ready_to_send_data_ RTC_GUARDED_BY(network_thread_) = false;
  std::function<void()> on_connected_callback_ RTC_GUARDED_BY(network_thread_);
  DataChannelSink* data_channel_sink_ RTC_GUARDED_BY(network_thread_) = nullptr;
};

}  // namespace webrtc

#endif  // MEDIA_SCTP_DCSCTP_TRANSPORT_H_
