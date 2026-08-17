/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains interfaces for DataChannels
// http://dev.w3.org/2011/webrtc/editor/webrtc.html#rtcdatachannel

#ifndef API_DATA_CHANNEL_INTERFACE_H_
#define API_DATA_CHANNEL_INTERFACE_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>

#include "absl/functional/any_invocable.h"
#include "api/priority.h"
#include "api/ref_count.h"
#include "api/rtc_error.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// C++ version of: https://www.w3.org/TR/webrtc/#idl-def-rtcdatachannelinit
// TODO(deadbeef): Use std::optional for the "-1 if unset" things.
struct DataChannelInit {
  // Deprecated. Reliability is assumed, and channel will be unreliable if
  // maxRetransmitTime or MaxRetransmits is set.
  bool reliable = false;

  // True if ordered delivery is required.
  bool ordered = true;

  // The max period of time in milliseconds in which retransmissions will be
  // sent. After this time, no more retransmissions will be sent.
  //
  // Cannot be set along with `maxRetransmits`.
  // This is called `maxPacketLifeTime` in the WebRTC JS API.
  // Negative values are ignored, and positive values are clamped to [0-65535]
  std::optional<int> maxRetransmitTime;

  // The max number of retransmissions.
  //
  // Cannot be set along with `maxRetransmitTime`.
  // Negative values are ignored, and positive values are clamped to [0-65535]
  std::optional<int> maxRetransmits;

  // This is set by the application and opaque to the WebRTC implementation.
  std::string protocol;

  // True if the channel has been externally negotiated and we do not send an
  // in-band signalling in the form of an "open" message. If this is true, `id`
  // below must be set; otherwise it should be unset and will be negotiated
  // in-band.
  bool negotiated = false;

  // The stream id, or SID, for SCTP data channels. -1 if unset (see above).
  int id = -1;

  // https://w3c.github.io/webrtc-priority/#new-rtcdatachannelinit-member
  std::optional<PriorityValue> priority;
};

// At the JavaScript level, data can be passed in as a string or a blob, so
// this structure's `binary` flag tells whether the data should be interpreted
// as binary or text.
struct DataBuffer {
  DataBuffer(const CopyOnWriteBuffer& data, bool binary)
      : data(data), binary(binary) {}
  // For convenience for unit tests.
  explicit DataBuffer(const std::string& text)
      : data(text.data(), text.length()), binary(false) {}
  size_t size() const { return data.size(); }

  CopyOnWriteBuffer data;
  // Indicates if the received data contains UTF-8 or binary data.
  // Note that the upper layers are left to verify the UTF-8 encoding.
  // TODO(jiayl): prefer to use an enum instead of a bool.
  bool binary;
};

// Used to implement RTCDataChannel events.
//
// The code responding to these callbacks should unwind the stack before
// using any other webrtc APIs; re-entrancy is not supported.
class DataChannelObserver {
 public:
  // The data channel state have changed.
  virtual void OnStateChange() = 0;
  //  A data buffer was successfully received.
  virtual void OnMessage(const DataBuffer& buffer) = 0;
  // The data channel's buffered_amount has changed.
  virtual void OnBufferedAmountChange(uint64_t /* sent_data_size */) {}

  // Override this to get callbacks directly on the network thread.
  // An implementation that does that must not block the network thread
  // but rather only use the callback to trigger asynchronous processing
  // elsewhere as a result of the notification.
  // The default return value, `false`, means that notifications will be
  // delivered on the signaling thread associated with the peerconnection
  // instance.
  // TODO(webrtc:11547): Eventually all DataChannelObserver implementations
  // should be called on the network thread and this method removed.
  virtual bool IsOkToCallOnTheNetworkThread() { return false; }

 protected:
  virtual ~DataChannelObserver() = default;
};

class RTC_EXPORT DataChannelInterface : public RefCountInterface {
 public:
  // C++ version of: https://www.w3.org/TR/webrtc/#idl-def-rtcdatachannelstate
  // Unlikely to change, but keep in sync with DataChannel.java:State and
  // RTCDataChannel.h:RTCDataChannelState.
  enum DataState {
    kConnecting,
    kOpen,  // The DataChannel is ready to send data.
    kClosing,
    kClosed
  };

  static const char* DataStateString(DataState state) {
    switch (state) {
      case kConnecting:
        return "connecting";
      case kOpen:
        return "open";
      case kClosing:
        return "closing";
      case kClosed:
        return "closed";
    }
    RTC_CHECK(false) << "Unknown DataChannel state: " << state;
    return "";
  }

  // Used to receive events from the data channel. Only one observer can be
  // registered at a time. UnregisterObserver should be called before the
  // observer object is destroyed.
  virtual void RegisterObserver(DataChannelObserver* observer) = 0;
  virtual void UnregisterObserver() = 0;

  // The label attribute represents a label that can be used to distinguish this
  // DataChannel object from other DataChannel objects.
  virtual std::string label() const = 0;

  // The accessors below simply return the properties from the DataChannelInit
  // the data channel was constructed with.
  virtual bool reliable() const = 0;
  // TODO(deadbeef): Remove these dummy implementations when all classes have
  // implemented these APIs. They should all just return the values the
  // DataChannel was created with.
  virtual bool ordered() const;
  virtual std::optional<int> maxRetransmitsOpt() const;
  virtual std::optional<int> maxPacketLifeTime() const;
  virtual std::string protocol() const;
  virtual bool negotiated() const;

  // Returns the ID from the DataChannelInit, if it was negotiated out-of-band.
  // If negotiated in-band, this ID will be populated once the DTLS role is
  // determined, and until then this will return -1.
  virtual int id() const = 0;
  virtual PriorityValue priority() const;
  virtual DataState state() const = 0;
  // When state is kClosed, and the DataChannel was not closed using
  // the closing procedure, returns the error information about the closing.
  // The default implementation returns "no error".
  virtual RTCError error() const { return RTCError(); }
  virtual uint32_t messages_sent() const = 0;
  virtual uint64_t bytes_sent() const = 0;
  virtual uint32_t messages_received() const = 0;
  virtual uint64_t bytes_received() const = 0;

  // Returns the number of bytes of application data (UTF-8 text and binary
  // data) that have been queued using Send but have not yet been processed at
  // the SCTP level. See comment above Send below.
  // Values are less or equal to MaxSendQueueSize().
  virtual uint64_t buffered_amount() const = 0;

  // Begins the graceful data channel closing procedure. See:
  // https://tools.ietf.org/html/draft-ietf-rtcweb-data-channel-13#section-6.7
  virtual void Close() = 0;

  // Sends `data` to the remote peer. If the data can't be sent at the SCTP
  // level (due to congestion control), it's buffered at the data channel level,
  // up to a maximum of MaxSendQueueSize().
  // Returns false if the data channel is not in open state or if the send
  // buffer is full.
  // TODO(webrtc:13289): Return an RTCError with information about the failure.
  // TODO(tommi): Remove this method once downstream implementations don't refer
  // to it.
  virtual bool Send(const DataBuffer& buffer);

  // Queues up an asynchronus send operation to run on a network thread.
  // Once the operation has completed the `on_complete` callback is invoked,
  // on the thread the send operation was done on. It's important that
  // `on_complete` implementations do not block the current thread but rather
  // post any expensive operations to other worker threads.
  // TODO(tommi): Make pure virtual after updating mock class in Chromium.
  // Deprecate `Send` in favor of this variant since the return value of `Send`
  // is limiting for a fully async implementation (yet in practice is ignored).
  virtual void SendAsync(DataBuffer buffer,
                         absl::AnyInvocable<void(RTCError) &&> on_complete);

  // Amount of bytes that can be queued for sending on the data channel.
  // Those are bytes that have not yet been processed at the SCTP level.
  static uint64_t MaxSendQueueSize();

 protected:
  ~DataChannelInterface() override = default;
};

}  // namespace webrtc

#endif  // API_DATA_CHANNEL_INTERFACE_H_
