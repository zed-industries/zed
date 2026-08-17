/*
 *  Copyright 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_DATA_CHANNEL_CONTROLLER_H_
#define PC_TEST_FAKE_DATA_CHANNEL_CONTROLLER_H_

#include <cstddef>
#include <set>
#include <string>
#include <utility>

#include "absl/algorithm/container.h"
#include "absl/strings/string_view.h"
#include "api/data_channel_interface.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/transport/data_channel_transport_interface.h"
#include "pc/sctp_data_channel.h"
#include "pc/sctp_utils.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

class FakeDataChannelController
    : public webrtc::SctpDataChannelControllerInterface {
 public:
  explicit FakeDataChannelController(webrtc::Thread* network_thread)
      : signaling_thread_(webrtc::Thread::Current()),
        network_thread_(network_thread),
        send_blocked_(false),
        transport_available_(false),
        ready_to_send_(false),
        transport_error_(false) {}

  ~FakeDataChannelController() override {
    network_thread_->BlockingCall([&] {
      RTC_DCHECK_RUN_ON(network_thread_);
      weak_factory_.InvalidateWeakPtrs();
    });
  }

  webrtc::WeakPtr<FakeDataChannelController> weak_ptr() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return weak_factory_.GetWeakPtr();
  }

  webrtc::scoped_refptr<webrtc::SctpDataChannel> CreateDataChannel(
      absl::string_view label,
      webrtc::InternalDataChannelInit init) {
    webrtc::scoped_refptr<webrtc::SctpDataChannel> channel =
        network_thread_->BlockingCall([&]() {
          RTC_DCHECK_RUN_ON(network_thread_);
          webrtc::WeakPtr<FakeDataChannelController> my_weak_ptr = weak_ptr();
          // Explicitly associate the weak ptr instance with the current thread
          // to catch early any inappropriate referencing of it on the network
          // thread.
          RTC_CHECK(my_weak_ptr);

          webrtc::scoped_refptr<webrtc::SctpDataChannel> channel =
              webrtc::SctpDataChannel::Create(
                  std::move(my_weak_ptr), std::string(label),
                  transport_available_, init, signaling_thread_,
                  network_thread_);
          if (transport_available_ && channel->sid_n().has_value()) {
            AddSctpDataStream(*channel->sid_n(), channel->priority());
          }
          if (ready_to_send_) {
            network_thread_->PostTask([channel = channel] {
              if (channel->state() !=
                  webrtc::DataChannelInterface::DataState::kClosed) {
                channel->OnTransportReady();
              }
            });
          }
          connected_channels_.insert(channel.get());
          return channel;
        });
    return channel;
  }

  webrtc::RTCError SendData(webrtc::StreamId sid,
                            const webrtc::SendDataParams& params,
                            const webrtc::CopyOnWriteBuffer& payload) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    RTC_CHECK(ready_to_send_);
    RTC_CHECK(transport_available_);
    if (send_blocked_) {
      return webrtc::RTCError(webrtc::RTCErrorType::RESOURCE_EXHAUSTED);
    }

    if (transport_error_) {
      return webrtc::RTCError(webrtc::RTCErrorType::INTERNAL_ERROR);
    }

    last_sid_ = sid;
    last_send_data_params_ = params;
    return webrtc::RTCError::OK();
  }

  void AddSctpDataStream(webrtc::StreamId sid,
                         webrtc::PriorityValue priority) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (!transport_available_) {
      return;
    }
    known_stream_ids_.insert(sid);
  }

  void RemoveSctpDataStream(webrtc::StreamId sid) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    known_stream_ids_.erase(sid);
    // Unlike the real SCTP transport, act like the closing procedure finished
    // instantly.
    auto it = absl::c_find_if(connected_channels_,
                              [&](const auto* c) { return c->sid_n() == sid; });
    // This path mimics the DCC's OnChannelClosed handler since the FDCC
    // (this class) doesn't have a transport that would do that.
    if (it != connected_channels_.end())
      (*it)->OnClosingProcedureComplete();
  }

  void OnChannelStateChanged(
      webrtc::SctpDataChannel* data_channel,
      webrtc::DataChannelInterface::DataState state) override {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (state == webrtc::DataChannelInterface::DataState::kOpen) {
      ++channels_opened_;
    } else if (state == webrtc::DataChannelInterface::DataState::kClosed) {
      ++channels_closed_;
      connected_channels_.erase(data_channel);
    }
  }

  size_t buffered_amount(webrtc::StreamId sid) const override { return 0; }
  size_t buffered_amount_low_threshold(webrtc::StreamId sid) const override {
    return 0;
  }
  void SetBufferedAmountLowThreshold(webrtc::StreamId sid,
                                     size_t bytes) override {}

  // Set true to emulate the SCTP stream being blocked by congestion control.
  void set_send_blocked(bool blocked) {
    network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      send_blocked_ = blocked;
      if (!blocked) {
        RTC_CHECK(transport_available_);
        // Make a copy since `connected_channels_` may change while
        // OnTransportReady is called.
        auto copy = connected_channels_;
        for (webrtc::SctpDataChannel* ch : copy) {
          ch->OnTransportReady();
        }
      }
    });
  }

  // Set true to emulate the transport channel creation, e.g. after
  // setLocalDescription/setRemoteDescription called with data content.
  void set_transport_available(bool available) {
    network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      transport_available_ = available;
    });
  }

  // Set true to emulate the transport OnTransportReady signal when the
  // transport becomes writable for the first time.
  void set_ready_to_send(bool ready) {
    network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      RTC_CHECK(transport_available_);
      ready_to_send_ = ready;
      if (ready) {
        std::set<webrtc::SctpDataChannel*>::iterator it;
        for (it = connected_channels_.begin(); it != connected_channels_.end();
             ++it) {
          (*it)->OnTransportReady();
        }
      }
    });
  }

  void set_transport_error() {
    network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      transport_error_ = true;
    });
  }

  int last_sid() const {
    return network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      return last_sid_.stream_id_int();
    });
  }

  webrtc::SendDataParams last_send_data_params() const {
    return network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      return last_send_data_params_;
    });
  }

  bool IsConnected(webrtc::SctpDataChannel* data_channel) const {
    return network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      return connected_channels_.find(data_channel) !=
             connected_channels_.end();
    });
  }

  bool IsStreamAdded(webrtc::StreamId id) const {
    return network_thread_->BlockingCall([&]() {
      RTC_DCHECK_RUN_ON(network_thread_);
      return known_stream_ids_.find(id) != known_stream_ids_.end();
    });
  }

  int channels_opened() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return channels_opened_;
  }
  int channels_closed() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return channels_closed_;
  }

 private:
  webrtc::Thread* const signaling_thread_;
  webrtc::Thread* const network_thread_;
  webrtc::StreamId last_sid_ RTC_GUARDED_BY(network_thread_);
  webrtc::SendDataParams last_send_data_params_ RTC_GUARDED_BY(network_thread_);
  bool send_blocked_ RTC_GUARDED_BY(network_thread_);
  bool transport_available_ RTC_GUARDED_BY(network_thread_);
  bool ready_to_send_ RTC_GUARDED_BY(network_thread_);
  bool transport_error_ RTC_GUARDED_BY(network_thread_);
  int channels_closed_ RTC_GUARDED_BY(network_thread_) = 0;
  int channels_opened_ RTC_GUARDED_BY(network_thread_) = 0;
  std::set<webrtc::SctpDataChannel*> connected_channels_
      RTC_GUARDED_BY(network_thread_);
  std::set<webrtc::StreamId> known_stream_ids_ RTC_GUARDED_BY(network_thread_);
  webrtc::WeakPtrFactory<FakeDataChannelController> weak_factory_
      RTC_GUARDED_BY(network_thread_){this};
};
#endif  // PC_TEST_FAKE_DATA_CHANNEL_CONTROLLER_H_
