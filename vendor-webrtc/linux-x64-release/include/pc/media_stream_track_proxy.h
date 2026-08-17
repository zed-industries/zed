/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file includes proxy classes for tracks. The purpose is
// to make sure tracks are only accessed from the signaling thread.

#ifndef PC_MEDIA_STREAM_TRACK_PROXY_H_
#define PC_MEDIA_STREAM_TRACK_PROXY_H_

#include <string>

#include "api/media_stream_interface.h"
#include "pc/proxy.h"

namespace webrtc {

// TODO(deadbeef): Move this to .cc file. What threads methods are called on is
// an implementation detail.
BEGIN_PRIMARY_PROXY_MAP(AudioTrack)
PROXY_PRIMARY_THREAD_DESTRUCTOR()
BYPASS_PROXY_CONSTMETHOD0(std::string, kind)
BYPASS_PROXY_CONSTMETHOD0(std::string, id)
PROXY_CONSTMETHOD0(TrackState, state)
PROXY_CONSTMETHOD0(bool, enabled)
BYPASS_PROXY_CONSTMETHOD0(AudioSourceInterface*, GetSource)
PROXY_METHOD1(void, AddSink, AudioTrackSinkInterface*)
PROXY_METHOD1(void, RemoveSink, AudioTrackSinkInterface*)
PROXY_METHOD1(bool, GetSignalLevel, int*)
PROXY_METHOD0(scoped_refptr<AudioProcessorInterface>, GetAudioProcessor)
PROXY_METHOD1(bool, set_enabled, bool)
PROXY_METHOD1(void, RegisterObserver, ObserverInterface*)
PROXY_METHOD1(void, UnregisterObserver, ObserverInterface*)
END_PROXY_MAP(AudioTrack)

BEGIN_PROXY_MAP(VideoTrack)
PROXY_PRIMARY_THREAD_DESTRUCTOR()
BYPASS_PROXY_CONSTMETHOD0(std::string, kind)
BYPASS_PROXY_CONSTMETHOD0(std::string, id)
PROXY_SECONDARY_CONSTMETHOD0(TrackState, state)
PROXY_CONSTMETHOD0(bool, enabled)
PROXY_METHOD1(bool, set_enabled, bool)
PROXY_CONSTMETHOD0(ContentHint, content_hint)
PROXY_METHOD1(void, set_content_hint, ContentHint)
PROXY_SECONDARY_METHOD2(void,
                        AddOrUpdateSink,
                        VideoSinkInterface<VideoFrame>*,
                        const VideoSinkWants&)
PROXY_SECONDARY_METHOD1(void, RemoveSink, VideoSinkInterface<VideoFrame>*)
PROXY_SECONDARY_METHOD0(void, RequestRefreshFrame)
BYPASS_PROXY_CONSTMETHOD0(VideoTrackSourceInterface*, GetSource)
PROXY_CONSTMETHOD0(bool, should_receive)
PROXY_METHOD1(void, set_should_receive, bool)

PROXY_METHOD1(void, RegisterObserver, ObserverInterface*)
PROXY_METHOD1(void, UnregisterObserver, ObserverInterface*)
END_PROXY_MAP(VideoTrack)

}  // namespace webrtc

#endif  // PC_MEDIA_STREAM_TRACK_PROXY_H_
