/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains mock implementations of observers used in PeerConnection.
// TODO(steveanton): These aren't really mocks and should be renamed.

#ifndef PC_TEST_MOCK_PEER_CONNECTION_OBSERVERS_H_
#define PC_TEST_MOCK_PEER_CONNECTION_OBSERVERS_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "api/candidate.h"
#include "api/data_channel_interface.h"
#include "api/jsep.h"
#include "api/jsep_ice_candidate.h"
#include "api/legacy_stats_types.h"
#include "api/make_ref_counted.h"
#include "api/media_stream_interface.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/stats/rtc_stats_report.h"
#include "pc/stream_collection.h"
#include "rtc_base/checks.h"
#include "rtc_base/string_encode.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class MockPeerConnectionObserver : public PeerConnectionObserver {
 public:
  struct AddTrackEvent {
    explicit AddTrackEvent(
        scoped_refptr<RtpReceiverInterface> event_receiver,
        std::vector<scoped_refptr<MediaStreamInterface>> event_streams)
        : receiver(std::move(event_receiver)),
          streams(std::move(event_streams)) {
      for (auto stream : streams) {
        std::vector<scoped_refptr<MediaStreamTrackInterface>> tracks;
        for (auto audio_track : stream->GetAudioTracks()) {
          tracks.push_back(audio_track);
        }
        for (auto video_track : stream->GetVideoTracks()) {
          tracks.push_back(video_track);
        }
        snapshotted_stream_tracks[stream] = tracks;
      }
    }

    scoped_refptr<RtpReceiverInterface> receiver;
    std::vector<scoped_refptr<MediaStreamInterface>> streams;
    // This map records the tracks present in each stream at the time the
    // OnAddTrack callback was issued.
    std::map<scoped_refptr<MediaStreamInterface>,
             std::vector<scoped_refptr<MediaStreamTrackInterface>>>
        snapshotted_stream_tracks;
  };

  MockPeerConnectionObserver() : remote_streams_(StreamCollection::Create()) {}
  virtual ~MockPeerConnectionObserver() {}
  void SetPeerConnectionInterface(PeerConnectionInterface* pc) {
    pc_ = pc;
    if (pc) {
      state_ = pc_->signaling_state();
    }
  }
  void OnSignalingChange(
      PeerConnectionInterface::SignalingState new_state) override {
    RTC_DCHECK(pc_);
    RTC_DCHECK(pc_->signaling_state() == new_state);
    state_ = new_state;
  }

  MediaStreamInterface* RemoteStream(const std::string& label) {
    return remote_streams_->find(label);
  }
  StreamCollectionInterface* remote_streams() const {
    return remote_streams_.get();
  }
  void OnAddStream(scoped_refptr<MediaStreamInterface> stream) override {
    last_added_stream_ = stream;
    remote_streams_->AddStream(stream);
  }
  void OnRemoveStream(scoped_refptr<MediaStreamInterface> stream) override {
    last_removed_stream_ = stream;
    remote_streams_->RemoveStream(stream.get());
  }
  void OnRenegotiationNeeded() override { renegotiation_needed_ = true; }
  void OnNegotiationNeededEvent(uint32_t event_id) override {
    latest_negotiation_needed_event_ = event_id;
  }
  void OnDataChannel(
      scoped_refptr<DataChannelInterface> data_channel) override {
    last_datachannel_ = data_channel;
  }

  void OnIceConnectionChange(
      PeerConnectionInterface::IceConnectionState new_state) override {
    RTC_DCHECK(pc_);
    RTC_DCHECK(pc_->ice_connection_state() == new_state);
    // When ICE is finished, the caller will get to a kIceConnectionCompleted
    // state, because it has the ICE controlling role, while the callee
    // will get to a kIceConnectionConnected state. This means that both ICE
    // and DTLS are connected.
    ice_connected_ =
        (new_state == PeerConnectionInterface::kIceConnectionConnected) ||
        (new_state == PeerConnectionInterface::kIceConnectionCompleted);
    callback_triggered_ = true;
  }
  void OnIceGatheringChange(
      PeerConnectionInterface::IceGatheringState new_state) override {
    RTC_DCHECK(pc_);
    RTC_DCHECK(pc_->ice_gathering_state() == new_state);
    ice_gathering_complete_ =
        new_state == PeerConnectionInterface::kIceGatheringComplete;
    callback_triggered_ = true;
  }
  void OnIceCandidate(const IceCandidateInterface* candidate) override {
    RTC_DCHECK(pc_);
    candidates_.push_back(std::make_unique<JsepIceCandidate>(
        candidate->sdp_mid(), candidate->sdp_mline_index(),
        candidate->candidate()));
    callback_triggered_ = true;
  }

  void OnIceCandidatesRemoved(
      const std::vector<Candidate>& candidates) override {
    num_candidates_removed_++;
    callback_triggered_ = true;
  }

  void OnIceConnectionReceivingChange(bool receiving) override {
    callback_triggered_ = true;
  }

  void OnAddTrack(scoped_refptr<RtpReceiverInterface> receiver,
                  const std::vector<scoped_refptr<MediaStreamInterface>>&
                      streams) override {
    RTC_DCHECK(receiver);
    num_added_tracks_++;
    last_added_track_label_ = receiver->id();
    add_track_events_.push_back(AddTrackEvent(receiver, streams));
  }

  void OnTrack(scoped_refptr<RtpTransceiverInterface> transceiver) override {
    on_track_transceivers_.push_back(transceiver);
  }

  void OnRemoveTrack(scoped_refptr<RtpReceiverInterface> receiver) override {
    remove_track_events_.push_back(receiver);
  }

  std::vector<scoped_refptr<RtpReceiverInterface>> GetAddTrackReceivers() {
    std::vector<scoped_refptr<RtpReceiverInterface>> receivers;
    for (const AddTrackEvent& event : add_track_events_) {
      receivers.push_back(event.receiver);
    }
    return receivers;
  }

  int CountAddTrackEventsForStream(const std::string& stream_id) {
    int found_tracks = 0;
    for (const AddTrackEvent& event : add_track_events_) {
      bool has_stream_id = false;
      for (auto stream : event.streams) {
        if (stream->id() == stream_id) {
          has_stream_id = true;
          break;
        }
      }
      if (has_stream_id) {
        ++found_tracks;
      }
    }
    return found_tracks;
  }

  // Returns the id of the last added stream.
  // Empty string if no stream have been added.
  std::string GetLastAddedStreamId() {
    if (last_added_stream_.get())
      return last_added_stream_->id();
    return "";
  }
  std::string GetLastRemovedStreamId() {
    if (last_removed_stream_.get())
      return last_removed_stream_->id();
    return "";
  }

  IceCandidateInterface* last_candidate() {
    if (candidates_.empty()) {
      return nullptr;
    } else {
      return candidates_.back().get();
    }
  }

  std::vector<const IceCandidateInterface*> GetAllCandidates() {
    std::vector<const IceCandidateInterface*> candidates;
    for (const auto& candidate : candidates_) {
      candidates.push_back(candidate.get());
    }
    return candidates;
  }

  std::vector<IceCandidateInterface*> GetCandidatesByMline(int mline_index) {
    std::vector<IceCandidateInterface*> candidates;
    for (const auto& candidate : candidates_) {
      if (candidate->sdp_mline_index() == mline_index) {
        candidates.push_back(candidate.get());
      }
    }
    return candidates;
  }

  bool legacy_renegotiation_needed() const { return renegotiation_needed_; }
  void clear_legacy_renegotiation_needed() { renegotiation_needed_ = false; }

  bool has_negotiation_needed_event() {
    return latest_negotiation_needed_event_.has_value();
  }
  uint32_t latest_negotiation_needed_event() {
    return latest_negotiation_needed_event_.value_or(0u);
  }
  void clear_latest_negotiation_needed_event() {
    latest_negotiation_needed_event_ = std::nullopt;
  }

  scoped_refptr<PeerConnectionInterface> pc_;
  PeerConnectionInterface::SignalingState state_;
  std::vector<std::unique_ptr<IceCandidateInterface>> candidates_;
  scoped_refptr<DataChannelInterface> last_datachannel_;
  scoped_refptr<StreamCollection> remote_streams_;
  bool renegotiation_needed_ = false;
  std::optional<uint32_t> latest_negotiation_needed_event_;
  bool ice_gathering_complete_ = false;
  bool ice_connected_ = false;
  bool callback_triggered_ = false;
  int num_added_tracks_ = 0;
  std::string last_added_track_label_;
  std::vector<AddTrackEvent> add_track_events_;
  std::vector<scoped_refptr<RtpReceiverInterface>> remove_track_events_;
  std::vector<scoped_refptr<RtpTransceiverInterface>> on_track_transceivers_;
  int num_candidates_removed_ = 0;

 private:
  scoped_refptr<MediaStreamInterface> last_added_stream_;
  scoped_refptr<MediaStreamInterface> last_removed_stream_;
};

class MockCreateSessionDescriptionObserver
    : public CreateSessionDescriptionObserver {
 public:
  MockCreateSessionDescriptionObserver()
      : called_(false),
        error_("MockCreateSessionDescriptionObserver not called") {}
  virtual ~MockCreateSessionDescriptionObserver() {}
  void OnSuccess(SessionDescriptionInterface* desc) override {
    MutexLock lock(&mutex_);
    called_ = true;
    error_ = "";
    desc_.reset(desc);
  }
  void OnFailure(RTCError error) override {
    MutexLock lock(&mutex_);
    called_ = true;
    error_ = error.message();
  }
  bool called() const {
    MutexLock lock(&mutex_);
    return called_;
  }
  bool result() const {
    MutexLock lock(&mutex_);
    return error_.empty();
  }
  const std::string& error() const {
    MutexLock lock(&mutex_);
    return error_;
  }
  std::unique_ptr<SessionDescriptionInterface> MoveDescription() {
    MutexLock lock(&mutex_);
    return std::move(desc_);
  }

 private:
  mutable Mutex mutex_;
  bool called_ RTC_GUARDED_BY(mutex_);
  std::string error_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<SessionDescriptionInterface> desc_ RTC_GUARDED_BY(mutex_);
};

class MockSetSessionDescriptionObserver : public SetSessionDescriptionObserver {
 public:
  static scoped_refptr<MockSetSessionDescriptionObserver> Create() {
    return make_ref_counted<MockSetSessionDescriptionObserver>();
  }

  MockSetSessionDescriptionObserver()
      : called_(false),
        error_("MockSetSessionDescriptionObserver not called") {}
  ~MockSetSessionDescriptionObserver() override {}
  void OnSuccess() override {
    MutexLock lock(&mutex_);

    called_ = true;
    error_ = "";
  }
  void OnFailure(RTCError error) override {
    MutexLock lock(&mutex_);
    called_ = true;
    error_ = error.message();
  }

  bool called() const {
    MutexLock lock(&mutex_);
    return called_;
  }
  bool result() const {
    MutexLock lock(&mutex_);
    return error_.empty();
  }
  const std::string& error() const {
    MutexLock lock(&mutex_);
    return error_;
  }

 private:
  mutable Mutex mutex_;
  bool called_;
  std::string error_;
};

class FakeSetLocalDescriptionObserver
    : public SetLocalDescriptionObserverInterface {
 public:
  bool called() const { return error_.has_value(); }
  RTCError& error() {
    RTC_DCHECK(error_.has_value());
    return *error_;
  }

  // SetLocalDescriptionObserverInterface implementation.
  void OnSetLocalDescriptionComplete(RTCError error) override {
    error_ = std::move(error);
  }

 private:
  // Set on complete, on success this is set to an RTCError::OK() error.
  std::optional<RTCError> error_;
};

class FakeSetRemoteDescriptionObserver
    : public SetRemoteDescriptionObserverInterface {
 public:
  bool called() const { return error_.has_value(); }
  RTCError& error() {
    RTC_DCHECK(error_.has_value());
    return *error_;
  }

  // SetRemoteDescriptionObserverInterface implementation.
  void OnSetRemoteDescriptionComplete(RTCError error) override {
    error_ = std::move(error);
  }

 private:
  // Set on complete, on success this is set to an RTCError::OK() error.
  std::optional<RTCError> error_;
};

class MockDataChannelObserver : public DataChannelObserver {
 public:
  struct Message {
    std::string data;
    bool binary;
  };

  explicit MockDataChannelObserver(DataChannelInterface* channel)
      : channel_(channel) {
    channel_->RegisterObserver(this);
    states_.push_back(channel_->state());
  }
  virtual ~MockDataChannelObserver() { channel_->UnregisterObserver(); }

  void OnBufferedAmountChange(uint64_t previous_amount) override {}

  void OnStateChange() override {
    states_.push_back(channel_->state());
    if (state_change_callback_) {
      state_change_callback_(states_.back());
    }
  }

  void OnMessage(const DataBuffer& buffer) override {
    messages_.push_back(
        {std::string(buffer.data.data<char>(), buffer.data.size()),
         buffer.binary});
  }

  bool IsOpen() const { return state() == DataChannelInterface::kOpen; }
  std::vector<Message> messages() const { return messages_; }
  std::string last_message() const {
    if (messages_.empty())
      return {};

    return messages_.back().data;
  }
  bool last_message_is_binary() const {
    if (messages_.empty())
      return false;
    return messages_.back().binary;
  }
  size_t received_message_count() const { return messages_.size(); }

  DataChannelInterface::DataState state() const { return states_.back(); }
  const std::vector<DataChannelInterface::DataState>& states() const {
    return states_;
  }

  void set_state_change_callback(
      std::function<void(DataChannelInterface::DataState)> func) {
    state_change_callback_ = std::move(func);
  }

 private:
  scoped_refptr<DataChannelInterface> channel_;
  std::vector<DataChannelInterface::DataState> states_;
  std::vector<Message> messages_;
  std::function<void(DataChannelInterface::DataState)> state_change_callback_;
};

class MockStatsObserver : public StatsObserver {
 public:
  MockStatsObserver() : called_(false), stats_() {}
  virtual ~MockStatsObserver() {}

  virtual void OnComplete(const StatsReports& reports) {
    RTC_CHECK(!called_);
    called_ = true;
    stats_.Clear();
    stats_.number_of_reports = reports.size();
    for (const auto* r : reports) {
      if (r->type() == StatsReport::kStatsReportTypeSsrc) {
        stats_.timestamp = r->timestamp();
        GetIntValue(r, StatsReport::kStatsValueNameAudioOutputLevel,
                    &stats_.audio_output_level);
        GetIntValue(r, StatsReport::kStatsValueNameAudioInputLevel,
                    &stats_.audio_input_level);
        GetIntValue(r, StatsReport::kStatsValueNameBytesReceived,
                    &stats_.bytes_received);
        GetIntValue(r, StatsReport::kStatsValueNameBytesSent,
                    &stats_.bytes_sent);
        GetInt64Value(r, StatsReport::kStatsValueNameCaptureStartNtpTimeMs,
                      &stats_.capture_start_ntp_time);
        stats_.track_ids.emplace_back();
        GetStringValue(r, StatsReport::kStatsValueNameTrackId,
                       &stats_.track_ids.back());
      } else if (r->type() == StatsReport::kStatsReportTypeBwe) {
        stats_.timestamp = r->timestamp();
        GetIntValue(r, StatsReport::kStatsValueNameAvailableReceiveBandwidth,
                    &stats_.available_receive_bandwidth);
      } else if (r->type() == StatsReport::kStatsReportTypeComponent) {
        stats_.timestamp = r->timestamp();
        GetStringValue(r, StatsReport::kStatsValueNameDtlsCipher,
                       &stats_.dtls_cipher);
        GetStringValue(r, StatsReport::kStatsValueNameSrtpCipher,
                       &stats_.srtp_cipher);
      }
    }
  }

  bool called() const { return called_; }
  size_t number_of_reports() const { return stats_.number_of_reports; }
  double timestamp() const { return stats_.timestamp; }

  int AudioOutputLevel() const {
    RTC_CHECK(called_);
    return stats_.audio_output_level;
  }

  int AudioInputLevel() const {
    RTC_CHECK(called_);
    return stats_.audio_input_level;
  }

  int BytesReceived() const {
    RTC_CHECK(called_);
    return stats_.bytes_received;
  }

  int BytesSent() const {
    RTC_CHECK(called_);
    return stats_.bytes_sent;
  }

  int64_t CaptureStartNtpTime() const {
    RTC_CHECK(called_);
    return stats_.capture_start_ntp_time;
  }

  int AvailableReceiveBandwidth() const {
    RTC_CHECK(called_);
    return stats_.available_receive_bandwidth;
  }

  std::string DtlsCipher() const {
    RTC_CHECK(called_);
    return stats_.dtls_cipher;
  }

  std::string SrtpCipher() const {
    RTC_CHECK(called_);
    return stats_.srtp_cipher;
  }

  std::vector<std::string> TrackIds() const {
    RTC_CHECK(called_);
    return stats_.track_ids;
  }

 private:
  bool GetIntValue(const StatsReport* report,
                   StatsReport::StatsValueName name,
                   int* value) {
    const StatsReport::Value* v = report->FindValue(name);
    if (v) {
      // TODO(tommi): We should really just be using an int here :-/
      *value = FromString<int>(v->ToString());
    }
    return v != nullptr;
  }

  bool GetInt64Value(const StatsReport* report,
                     StatsReport::StatsValueName name,
                     int64_t* value) {
    const StatsReport::Value* v = report->FindValue(name);
    if (v) {
      // TODO(tommi): We should really just be using an int here :-/
      *value = FromString<int64_t>(v->ToString());
    }
    return v != nullptr;
  }

  bool GetStringValue(const StatsReport* report,
                      StatsReport::StatsValueName name,
                      std::string* value) {
    const StatsReport::Value* v = report->FindValue(name);
    if (v)
      *value = v->ToString();
    return v != nullptr;
  }

  bool called_;
  struct {
    void Clear() {
      number_of_reports = 0;
      timestamp = 0;
      audio_output_level = 0;
      audio_input_level = 0;
      bytes_received = 0;
      bytes_sent = 0;
      capture_start_ntp_time = 0;
      available_receive_bandwidth = 0;
      dtls_cipher.clear();
      srtp_cipher.clear();
      track_ids.clear();
    }

    size_t number_of_reports;
    double timestamp;
    int audio_output_level;
    int audio_input_level;
    int bytes_received;
    int bytes_sent;
    int64_t capture_start_ntp_time;
    int available_receive_bandwidth;
    std::string dtls_cipher;
    std::string srtp_cipher;
    std::vector<std::string> track_ids;
  } stats_;
};

// Helper class that just stores the report from the callback.
class MockRTCStatsCollectorCallback : public RTCStatsCollectorCallback {
 public:
  scoped_refptr<const RTCStatsReport> report() { return report_; }

  bool called() const { return called_; }

 protected:
  void OnStatsDelivered(
      const scoped_refptr<const RTCStatsReport>& report) override {
    report_ = report;
    called_ = true;
  }

 private:
  bool called_ = false;
  scoped_refptr<const RTCStatsReport> report_;
};

}  // namespace webrtc

#endif  // PC_TEST_MOCK_PEER_CONNECTION_OBSERVERS_H_
