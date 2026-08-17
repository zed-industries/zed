/* Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This is an experimental interface and is subject to change without notice.

#ifndef API_TRANSPORT_DATA_CHANNEL_TRANSPORT_INTERFACE_H_
#define API_TRANSPORT_DATA_CHANNEL_TRANSPORT_INTERFACE_H_

#include <cstddef>
#include <optional>

#include "api/priority.h"
#include "api/rtc_error.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

// Supported types of application data messages.
enum class DataMessageType {
  // Application data buffer with the binary bit unset.
  kText,

  // Application data buffer with the binary bit set.
  kBinary,

  // Transport-agnostic control messages, such as open or open-ack messages.
  kControl,
};

// Parameters for sending data.  The parameters may change from message to
// message, even within a single channel.  For example, control messages may be
// sent reliably and in-order, even if the data channel is configured for
// unreliable delivery.
struct SendDataParams {
  DataMessageType type = DataMessageType::kText;

  // Whether to deliver the message in order with respect to other ordered
  // messages with the same channel_id.
  bool ordered = false;

  // If set, the maximum number of times this message may be
  // retransmitted by the transport before it is dropped.
  // Setting this value to zero disables retransmission.
  // Valid values are in the range [0-UINT16_MAX].
  // `max_rtx_count` and `max_rtx_ms` may not be set simultaneously.
  std::optional<int> max_rtx_count;

  // If set, the maximum number of milliseconds for which the transport
  // may retransmit this message before it is dropped.
  // Setting this value to zero disables retransmission.
  // Valid values are in the range [0-UINT16_MAX].
  // `max_rtx_count` and `max_rtx_ms` may not be set simultaneously.
  std::optional<int> max_rtx_ms;
};

// Sink for callbacks related to a data channel.
class DataChannelSink {
 public:
  virtual ~DataChannelSink() = default;

  // Callback issued when data is received by the transport.
  virtual void OnDataReceived(int channel_id,
                              DataMessageType type,
                              const CopyOnWriteBuffer& buffer) = 0;

  // Callback issued when a remote data channel begins the closing procedure.
  // Messages sent after the closing procedure begins will not be transmitted.
  virtual void OnChannelClosing(int channel_id) = 0;

  // Callback issued when a (remote or local) data channel completes the closing
  // procedure.  Closing channels become closed after all pending data has been
  // transmitted.
  virtual void OnChannelClosed(int channel_id) = 0;

  // Callback issued when the data channel becomes ready to send.
  // This callback will be issued immediately when the data channel sink is
  // registered if the transport is ready at that time.  This callback may be
  // invoked again following send errors (eg. due to the transport being
  // temporarily blocked or unavailable).
  virtual void OnReadyToSend() = 0;

  // Callback issued when the data channel becomes unusable (closed).
  // TODO(https://crbug.com/webrtc/10360): Make pure virtual when all
  // consumers updated.
  virtual void OnTransportClosed(RTCError /* error */) {}

  // The data channel's buffered_amount has fallen to or below the threshold
  // set when calling `SetBufferedAmountLowThreshold`
  virtual void OnBufferedAmountLow(int channel_id) = 0;
};

// Transport for data channels.
class DataChannelTransportInterface {
 public:
  virtual ~DataChannelTransportInterface() = default;

  // Opens a data `channel_id` for sending.  May return an error if the
  // specified `channel_id` is unusable.  Must be called before `SendData`.
  virtual RTCError OpenChannel(int channel_id, PriorityValue priority) = 0;

  // Sends a data buffer to the remote endpoint using the given send parameters.
  // `buffer` may not be larger than 256 KiB. Returns an error if the send
  // fails.
  virtual RTCError SendData(int channel_id,
                            const SendDataParams& params,
                            const CopyOnWriteBuffer& buffer) = 0;

  // Closes `channel_id` gracefully.  Returns an error if `channel_id` is not
  // open.  Data sent after the closing procedure begins will not be
  // transmitted. The channel becomes closed after pending data is transmitted.
  virtual RTCError CloseChannel(int channel_id) = 0;

  // Sets a sink for data messages and channel state callbacks. Before media
  // transport is destroyed, the sink must be unregistered by setting it to
  // nullptr.
  virtual void SetDataSink(DataChannelSink* sink) = 0;

  // Returns whether this data channel transport is ready to send.
  // Note: the default implementation always returns false (as it assumes no one
  // has implemented the interface).  This default implementation is temporary.
  virtual bool IsReadyToSend() const = 0;

  virtual size_t buffered_amount(int channel_id) const = 0;
  virtual size_t buffered_amount_low_threshold(int channel_id) const = 0;
  virtual void SetBufferedAmountLowThreshold(int channel_id, size_t bytes) = 0;
};

}  // namespace webrtc

#endif  // API_TRANSPORT_DATA_CHANNEL_TRANSPORT_INTERFACE_H_
