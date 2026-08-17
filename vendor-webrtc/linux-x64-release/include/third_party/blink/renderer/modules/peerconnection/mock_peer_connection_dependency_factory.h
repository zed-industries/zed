// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_PEER_CONNECTION_DEPENDENCY_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_PEER_CONNECTION_DEPENDENCY_FACTORY_H_

#include <string>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/modules/peerconnection/peer_connection_dependency_factory.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/api/metronome/metronome.h"
#include "third_party/webrtc/rtc_base/ref_counted_object.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

using ObserverSet = HashSet<webrtc::ObserverInterface*>;

class MockWebRtcAudioSource : public webrtc::AudioSourceInterface {
 public:
  MockWebRtcAudioSource(bool is_remote);
  void RegisterObserver(webrtc::ObserverInterface* observer) override;
  void UnregisterObserver(webrtc::ObserverInterface* observer) override;

  SourceState state() const override;
  bool remote() const override;

 private:
  const bool is_remote_;
};

class MockWebRtcAudioTrack : public webrtc::AudioTrackInterface {
 public:
  static scoped_refptr<MockWebRtcAudioTrack> Create(const std::string& id);

  void AddSink(webrtc::AudioTrackSinkInterface* sink) override {}
  void RemoveSink(webrtc::AudioTrackSinkInterface* sink) override {}
  webrtc::AudioSourceInterface* GetSource() const override;

  std::string kind() const override;
  std::string id() const override;
  bool enabled() const override;
  webrtc::MediaStreamTrackInterface::TrackState state() const override;
  bool set_enabled(bool enable) override;

  void RegisterObserver(webrtc::ObserverInterface* observer) override;
  void UnregisterObserver(webrtc::ObserverInterface* observer) override;

  void SetEnded();

 protected:
  MockWebRtcAudioTrack(const std::string& id);
  ~MockWebRtcAudioTrack() override;

 private:
  std::string id_;
  scoped_refptr<webrtc::AudioSourceInterface> source_;
  bool enabled_;
  TrackState state_;
  ObserverSet observers_;
};

class MockWebRtcVideoTrackSource
    : public webrtc::RefCountedObject<webrtc::VideoTrackSourceInterface> {
 public:
  static scoped_refptr<MockWebRtcVideoTrackSource> Create(
      bool supports_encoded_output);
  MockWebRtcVideoTrackSource(bool supports_encoded_output);
  void RegisterObserver(webrtc::ObserverInterface* observer) override;
  void UnregisterObserver(webrtc::ObserverInterface* observer) override;
  SourceState state() const override;
  bool remote() const override;
  void AddOrUpdateSink(webrtc::VideoSinkInterface<webrtc::VideoFrame>* sink,
                       const webrtc::VideoSinkWants& wants) override;
  void RemoveSink(
      webrtc::VideoSinkInterface<webrtc::VideoFrame>* sink) override;
  bool is_screencast() const override;
  std::optional<bool> needs_denoising() const override;
  bool GetStats(Stats* stats) override;
  bool SupportsEncodedOutput() const override;
  void GenerateKeyFrame() override;
  void AddEncodedSink(
      webrtc::VideoSinkInterface<webrtc::RecordableEncodedFrame>* sink)
      override;
  void RemoveEncodedSink(
      webrtc::VideoSinkInterface<webrtc::RecordableEncodedFrame>* sink)
      override;

 private:
  bool supports_encoded_output_;
};

class MockWebRtcVideoTrack
    : public webrtc::RefCountedObject<webrtc::VideoTrackInterface> {
 public:
  static scoped_refptr<MockWebRtcVideoTrack> Create(
      const std::string& id,
      scoped_refptr<webrtc::VideoTrackSourceInterface> source = nullptr);
  MockWebRtcVideoTrack(const std::string& id,
                       webrtc::VideoTrackSourceInterface* source);
  void AddOrUpdateSink(webrtc::VideoSinkInterface<webrtc::VideoFrame>* sink,
                       const webrtc::VideoSinkWants& wants) override;
  void RemoveSink(
      webrtc::VideoSinkInterface<webrtc::VideoFrame>* sink) override;
  webrtc::VideoTrackSourceInterface* GetSource() const override;

  std::string kind() const override;
  std::string id() const override;
  bool enabled() const override;
  webrtc::MediaStreamTrackInterface::TrackState state() const override;
  bool set_enabled(bool enable) override;

  void RegisterObserver(webrtc::ObserverInterface* observer) override;
  void UnregisterObserver(webrtc::ObserverInterface* observer) override;

  void SetEnded();

 protected:
  ~MockWebRtcVideoTrack() override;

 private:
  std::string id_;
  scoped_refptr<webrtc::VideoTrackSourceInterface> source_;
  bool enabled_;
  TrackState state_;
  ObserverSet observers_;
  raw_ptr<webrtc::VideoSinkInterface<webrtc::VideoFrame>> sink_;
};

class MockMediaStream : public webrtc::MediaStreamInterface {
 public:
  explicit MockMediaStream(const std::string& id);

  bool AddTrack(
      webrtc::scoped_refptr<webrtc::AudioTrackInterface> track) override;
  bool AddTrack(
      webrtc::scoped_refptr<webrtc::VideoTrackInterface> track) override;
  bool RemoveTrack(
      webrtc::scoped_refptr<webrtc::AudioTrackInterface> track) override;
  bool RemoveTrack(
      webrtc::scoped_refptr<webrtc::VideoTrackInterface> track) override;
  std::string id() const override;
  webrtc::AudioTrackVector GetAudioTracks() override;
  webrtc::VideoTrackVector GetVideoTracks() override;
  webrtc::scoped_refptr<webrtc::AudioTrackInterface> FindAudioTrack(
      const std::string& track_id) override;
  webrtc::scoped_refptr<webrtc::VideoTrackInterface> FindVideoTrack(
      const std::string& track_id) override;
  void RegisterObserver(webrtc::ObserverInterface* observer) override;
  void UnregisterObserver(webrtc::ObserverInterface* observer) override;

 protected:
  ~MockMediaStream() override;

 private:
  void NotifyObservers();

  std::string id_;
  webrtc::AudioTrackVector audio_track_vector_;
  webrtc::VideoTrackVector video_track_vector_;

  ObserverSet observers_;
};

// A mock factory for creating different objects for
// RTC PeerConnections.
class MockPeerConnectionDependencyFactory
    : public blink::PeerConnectionDependencyFactory {
 public:
  MockPeerConnectionDependencyFactory();

  MockPeerConnectionDependencyFactory(
      const MockPeerConnectionDependencyFactory&) = delete;
  MockPeerConnectionDependencyFactory& operator=(
      const MockPeerConnectionDependencyFactory&) = delete;

  ~MockPeerConnectionDependencyFactory() override;

  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> CreatePeerConnection(
      const webrtc::PeerConnectionInterface::RTCConfiguration& config,
      blink::WebLocalFrame* frame,
      webrtc::PeerConnectionObserver* observer,
      ExceptionState& exception_state,
      RTCRtpTransport* rtp_transport) override;
  scoped_refptr<webrtc::VideoTrackSourceInterface> CreateVideoTrackSourceProxy(
      webrtc::VideoTrackSourceInterface* source) override;
  scoped_refptr<webrtc::MediaStreamInterface> CreateLocalMediaStream(
      const String& label) override;
  scoped_refptr<webrtc::VideoTrackInterface> CreateLocalVideoTrack(
      const String& id,
      webrtc::VideoTrackSourceInterface* source) override;
  webrtc::IceCandidateInterface* CreateIceCandidate(const String& sdp_mid,
                                                    int sdp_mline_index,
                                                    const String& sdp) override;

  scoped_refptr<base::SingleThreadTaskRunner> GetWebRtcNetworkTaskRunner()
      override;
  scoped_refptr<base::SingleThreadTaskRunner> GetWebRtcSignalingTaskRunner()
      override;

  std::unique_ptr<webrtc::Metronome> CreateDecodeMetronome() override;

  // If |fail| is true, subsequent calls to CreateSessionDescription will
  // return nullptr. This can be used to fake a blob of SDP that fails to be
  // parsed.
  void SetFailToCreateSessionDescription(bool fail);

 private:
  // TODO(crbug.com/787254): Replace with the appropriate Blink class.
  base::Thread thread_;
  bool fail_to_create_session_description_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_PEER_CONNECTION_DEPENDENCY_FACTORY_H_
