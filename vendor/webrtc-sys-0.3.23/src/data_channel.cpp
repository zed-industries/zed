/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/data_channel.h"

#include <utility>

#include "rtc_base/synchronization/mutex.h"
#include "webrtc-sys/src/data_channel.rs.h"

namespace livekit_ffi {

webrtc::DataChannelInit to_native_data_channel_init(DataChannelInit init) {
  webrtc::DataChannelInit rtc_init{};
  rtc_init.id = init.id;
  rtc_init.negotiated = init.negotiated;
  rtc_init.ordered = init.ordered;
  rtc_init.protocol = init.protocol.c_str();

  if (init.has_max_retransmit_time)
    rtc_init.maxRetransmitTime = init.max_retransmit_time;

  if (init.has_max_retransmits)
    rtc_init.maxRetransmits = init.max_retransmits;

  if (init.has_priority)
    rtc_init.priority = webrtc::PriorityValue(static_cast<webrtc::Priority>(init.priority));

  return rtc_init;
}

DataChannel::DataChannel(
    std::shared_ptr<RtcRuntime> rtc_runtime,
    webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
    : rtc_runtime_(rtc_runtime), data_channel_(std::move(data_channel)) {
  RTC_LOG(LS_VERBOSE) << "DataChannel::DataChannel()";
}

DataChannel::~DataChannel() {
  RTC_LOG(LS_VERBOSE) << "DataChannel::~DataChannel()";
  unregister_observer();
}

void DataChannel::register_observer(
    rust::Box<DataChannelObserverWrapper> observer) const {
  webrtc::MutexLock lock(&mutex_);

  data_channel_->UnregisterObserver();

  observer_ =
      std::make_unique<NativeDataChannelObserver>(std::move(observer), this);
  data_channel_->RegisterObserver(observer_.get());
}

void DataChannel::unregister_observer() const {
  webrtc::MutexLock lock(&mutex_);
  data_channel_->UnregisterObserver();
  observer_ = nullptr;
}

bool DataChannel::send(const DataBuffer& buffer) const {
  return data_channel_->Send(webrtc::DataBuffer{
      webrtc::CopyOnWriteBuffer(buffer.ptr, buffer.len), buffer.binary});
}

int DataChannel::id() const {
  return data_channel_->id();
}

rust::String DataChannel::label() const {
  return data_channel_->label();
}

DataState DataChannel::state() const {
  return static_cast<DataState>(data_channel_->state());
}

void DataChannel::close() const {
  return data_channel_->Close();
}

uint64_t DataChannel::buffered_amount() const {
  return data_channel_->buffered_amount();
}

NativeDataChannelObserver::NativeDataChannelObserver(
    rust::Box<DataChannelObserverWrapper> observer,
    const DataChannel* dc)
    : observer_(std::move(observer)), dc_(dc) {}

void NativeDataChannelObserver::OnStateChange() {
  observer_->on_state_change(dc_->state());
}

void NativeDataChannelObserver::OnMessage(const webrtc::DataBuffer& buffer) {
  DataBuffer data{};
  data.ptr = buffer.data.data();
  data.len = buffer.data.size();
  data.binary = buffer.binary;
  observer_->on_message(data);
}

void NativeDataChannelObserver::OnBufferedAmountChange(
    uint64_t sent_data_size) {
  observer_->on_buffered_amount_change(sent_data_size);
}

}  // namespace livekit_ffi
