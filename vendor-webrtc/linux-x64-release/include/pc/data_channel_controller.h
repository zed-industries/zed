/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_DATA_CHANNEL_CONTROLLER_H_
#define PC_DATA_CHANNEL_CONTROLLER_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/data_channel_event_observer_interface.h"
#include "api/data_channel_interface.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/data_channel_transport_interface.h"
#include "pc/data_channel_utils.h"
#include "pc/sctp_data_channel.h"
#include "pc/sctp_utils.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

class PeerConnectionInternal;

class DataChannelController : public SctpDataChannelControllerInterface,
                              public DataChannelSink {
 public:
  explicit DataChannelController(PeerConnectionInternal* pc) : pc_(pc) {}
  ~DataChannelController();

  // Not copyable or movable.
  DataChannelController(DataChannelController&) = delete;
  DataChannelController& operator=(const DataChannelController& other) = delete;
  DataChannelController(DataChannelController&&) = delete;
  DataChannelController& operator=(DataChannelController&& other) = delete;

  // Implements
  // SctpDataChannelProviderInterface.
  RTCError SendData(StreamId sid,
                    const SendDataParams& params,
                    const CopyOnWriteBuffer& payload) override;
  void AddSctpDataStream(StreamId sid, PriorityValue priority) override;
  void RemoveSctpDataStream(StreamId sid) override;
  void OnChannelStateChanged(SctpDataChannel* channel,
                             DataChannelInterface::DataState state) override;
  size_t buffered_amount(StreamId sid) const override;
  size_t buffered_amount_low_threshold(StreamId sid) const override;
  void SetBufferedAmountLowThreshold(StreamId sid, size_t bytes) override;

  // Implements DataChannelSink.
  void OnDataReceived(int channel_id,
                      DataMessageType type,
                      const CopyOnWriteBuffer& buffer) override;
  void OnChannelClosing(int channel_id) override;
  void OnChannelClosed(int channel_id) override;
  void OnReadyToSend() override;
  void OnTransportClosed(RTCError error) override;
  void OnBufferedAmountLow(int channel_id) override;

  // Called as part of destroying the owning PeerConnection.
  void PrepareForShutdown();

  // Called from PeerConnection::SetupDataChannelTransport_n
  void SetupDataChannelTransport_n(DataChannelTransportInterface* transport);
  // Called from PeerConnection::TeardownDataChannelTransport_n
  void TeardownDataChannelTransport_n(RTCError error);

  // Called from PeerConnection::OnTransportChanged
  // to make required changes to datachannels' transports.
  void OnTransportChanged(
      DataChannelTransportInterface* data_channel_transport);

  // Called from PeerConnection::GetDataChannelStats on the signaling thread.
  std::vector<DataChannelStats> GetDataChannelStats() const;

  // Creates channel and adds it to the collection of DataChannels that will
  // be offered in a SessionDescription, and wraps it in a proxy object.
  RTCErrorOr<scoped_refptr<DataChannelInterface>>
  InternalCreateDataChannelWithProxy(const std::string& label,
                                     const InternalDataChannelInit& config);
  void AllocateSctpSids(SSLRole role);

  // Check if data channels are currently tracked. Used to decide whether a
  // rejected m=application section should be reoffered.
  bool HasDataChannels() const;

  // At some point in time, a data channel has existed.
  bool HasUsedDataChannels() const;

  void SetEventObserver(
      std::unique_ptr<DataChannelEventObserverInterface> observer);

 protected:
  Thread* network_thread() const;
  Thread* signaling_thread() const;

 private:
  void OnSctpDataChannelClosed(SctpDataChannel* channel);

  // Creates a new SctpDataChannel object on the network thread.
  RTCErrorOr<scoped_refptr<SctpDataChannel>> CreateDataChannel(
      const std::string& label,
      InternalDataChannelInit& config) RTC_RUN_ON(network_thread());

  // Parses and handles open messages.  Returns true if the message is an open
  // message and should be considered to be handled, false otherwise.
  bool HandleOpenMessage_n(int channel_id,
                           DataMessageType type,
                           const CopyOnWriteBuffer& buffer)
      RTC_RUN_ON(network_thread());
  // Called when a valid data channel OPEN message is received.
  void OnDataChannelOpenMessage(scoped_refptr<SctpDataChannel> channel,
                                bool ready_to_send)
      RTC_RUN_ON(signaling_thread());

  // Accepts a `StreamId` which may be pre-negotiated or unassigned. For
  // pre-negotiated sids, attempts to reserve the sid in the allocation pool,
  // for unassigned sids attempts to generate a new sid if possible. Returns
  // RTCError::OK() if the sid is reserved (and may have been generated) or
  // if not enough information exists to generate a sid, in which case the sid
  // will still be unassigned upon return, but will be assigned later.
  // If the pool has been exhausted or a sid has already been reserved, an
  // error will be returned.
  RTCError ReserveOrAllocateSid(std::optional<StreamId>& sid,
                                std::optional<SSLRole> fallback_ssl_role)
      RTC_RUN_ON(network_thread());

  // Called when all data channels need to be notified of a transport channel
  // (calls OnTransportChannelCreated on the signaling thread).
  void NotifyDataChannelsOfTransportCreated();

  void set_data_channel_transport(DataChannelTransportInterface* transport);

  std::optional<DataChannelEventObserverInterface::Message>
  BuildObserverMessage(
      StreamId sid,
      DataMessageType type,
      ArrayView<const uint8_t> payload,
      DataChannelEventObserverInterface::Message::Direction direction) const
      RTC_RUN_ON(network_thread());

  // Plugin transport used for data channels.  Pointer may be accessed and
  // checked from any thread, but the object may only be touched on the
  // network thread.
  DataChannelTransportInterface* data_channel_transport_
      RTC_GUARDED_BY(network_thread()) = nullptr;
  SctpSidAllocator sid_allocator_ RTC_GUARDED_BY(network_thread());
  std::vector<scoped_refptr<SctpDataChannel>> sctp_data_channels_n_
      RTC_GUARDED_BY(network_thread());
  enum class DataChannelUsage : uint8_t {
    kNeverUsed = 0,
    kHaveBeenUsed,
    kInUse
  };
  DataChannelUsage channel_usage_ RTC_GUARDED_BY(signaling_thread()) =
      DataChannelUsage::kNeverUsed;

  std::unique_ptr<DataChannelEventObserverInterface> event_observer_;

  // Owning PeerConnection.
  PeerConnectionInternal* const pc_;
  // The weak pointers must be dereferenced and invalidated on the network
  // thread only.
  WeakPtrFactory<DataChannelController> weak_factory_
      RTC_GUARDED_BY(network_thread()){this};
  ScopedTaskSafety signaling_safety_;
};

}  // namespace webrtc

#endif  // PC_DATA_CHANNEL_CONTROLLER_H_
