/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_MEDIA_STREAM_INTERFACE_H_
#define API_TEST_MOCK_MEDIA_STREAM_INTERFACE_H_

#include <string>
#include <type_traits>

#include "api/audio_options.h"
#include "api/media_stream_interface.h"
#include "api/scoped_refptr.h"
#include "rtc_base/ref_counted_object.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioSource : public RefCountedObject<AudioSourceInterface> {
 public:
  static scoped_refptr<MockAudioSource> Create() {
    return scoped_refptr<MockAudioSource>(new MockAudioSource());
  }

  MOCK_METHOD(void,
              RegisterObserver,
              (ObserverInterface * observer),
              (override));
  MOCK_METHOD(void,
              UnregisterObserver,
              (ObserverInterface * observer),
              (override));
  MOCK_METHOD(SourceState, state, (), (const, override));
  MOCK_METHOD(bool, remote, (), (const, override));
  MOCK_METHOD(void, SetVolume, (double volume), (override));
  MOCK_METHOD(void,
              RegisterAudioObserver,
              (AudioObserver * observer),
              (override));
  MOCK_METHOD(void,
              UnregisterAudioObserver,
              (AudioObserver * observer),
              (override));
  MOCK_METHOD(void, AddSink, (AudioTrackSinkInterface * sink), (override));
  MOCK_METHOD(void, RemoveSink, (AudioTrackSinkInterface * sink), (override));
  MOCK_METHOD(const AudioOptions, options, (), (const, override));

 private:
  MockAudioSource() = default;
};

class MockAudioTrack : public RefCountedObject<AudioTrackInterface> {
 public:
  static scoped_refptr<MockAudioTrack> Create() {
    return scoped_refptr<MockAudioTrack>(new MockAudioTrack());
  }

  MOCK_METHOD(void,
              RegisterObserver,
              (ObserverInterface * observer),
              (override));
  MOCK_METHOD(void,
              UnregisterObserver,
              (ObserverInterface * observer),
              (override));
  MOCK_METHOD(std::string, kind, (), (const, override));
  MOCK_METHOD(std::string, id, (), (const, override));
  MOCK_METHOD(bool, enabled, (), (const, override));
  MOCK_METHOD(bool, set_enabled, (bool enable), (override));
  MOCK_METHOD(TrackState, state, (), (const, override));
  MOCK_METHOD(AudioSourceInterface*, GetSource, (), (const, override));
  MOCK_METHOD(void, AddSink, (AudioTrackSinkInterface * sink), (override));
  MOCK_METHOD(void, RemoveSink, (AudioTrackSinkInterface * sink), (override));
  MOCK_METHOD(bool, GetSignalLevel, (int* level), (override));
  MOCK_METHOD(scoped_refptr<AudioProcessorInterface>,
              GetAudioProcessor,
              (),
              (override));

 private:
  MockAudioTrack() = default;
};

class MockMediaStream : public MediaStreamInterface {
 public:
  MOCK_METHOD(std::string, id, (), (const, override));
  MOCK_METHOD(AudioTrackVector, GetAudioTracks, (), (override));
  MOCK_METHOD(VideoTrackVector, GetVideoTracks, (), (override));
  MOCK_METHOD(scoped_refptr<AudioTrackInterface>,
              FindAudioTrack,
              (const std::string& track_id),
              (override));
  MOCK_METHOD(scoped_refptr<VideoTrackInterface>,
              FindVideoTrack,
              (const std::string& track_id),
              (override));
  MOCK_METHOD(bool,
              AddTrack,
              (webrtc::scoped_refptr<AudioTrackInterface> track),
              (override));
  MOCK_METHOD(bool,
              AddTrack,
              (webrtc::scoped_refptr<VideoTrackInterface> track),
              (override));
  MOCK_METHOD(bool,
              RemoveTrack,
              (webrtc::scoped_refptr<AudioTrackInterface> track),
              (override));
  MOCK_METHOD(bool,
              RemoveTrack,
              (webrtc::scoped_refptr<VideoTrackInterface> track),
              (override));
  MOCK_METHOD(void,
              RegisterObserver,
              (ObserverInterface * observer),
              (override));
  MOCK_METHOD(void,
              UnregisterObserver,
              (ObserverInterface * observer),
              (override));
};

static_assert(!std::is_abstract_v<webrtc::RefCountedObject<MockMediaStream>>,
              "");

}  // namespace webrtc

#endif  // API_TEST_MOCK_MEDIA_STREAM_INTERFACE_H_
