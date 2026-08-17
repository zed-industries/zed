/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_VIDEO_TRACK_SOURCE_PROXY_H_
#define PC_VIDEO_TRACK_SOURCE_PROXY_H_

#include <optional>

#include "api/media_stream_interface.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "api/video_track_source_constraints.h"
#include "pc/proxy.h"

namespace webrtc {

// Makes sure the real VideoTrackSourceInterface implementation is destroyed on
// the signaling thread and marshals all method calls to the signaling thread.
// TODO(deadbeef): Move this to .cc file. What threads methods are called on is
// an implementation detail.
BEGIN_PROXY_MAP(VideoTrackSource)

PROXY_PRIMARY_THREAD_DESTRUCTOR()
PROXY_CONSTMETHOD0(SourceState, state)
BYPASS_PROXY_CONSTMETHOD0(bool, remote)
BYPASS_PROXY_CONSTMETHOD0(bool, is_screencast)
PROXY_CONSTMETHOD0(std::optional<bool>, needs_denoising)
PROXY_METHOD1(bool, GetStats, Stats*)
PROXY_SECONDARY_METHOD2(void,
                        AddOrUpdateSink,
                        VideoSinkInterface<VideoFrame>*,
                        const VideoSinkWants&)
PROXY_SECONDARY_METHOD1(void, RemoveSink, VideoSinkInterface<VideoFrame>*)
PROXY_SECONDARY_METHOD0(void, RequestRefreshFrame)
PROXY_METHOD1(void, RegisterObserver, ObserverInterface*)
PROXY_METHOD1(void, UnregisterObserver, ObserverInterface*)
PROXY_CONSTMETHOD0(bool, SupportsEncodedOutput)
PROXY_SECONDARY_METHOD0(void, GenerateKeyFrame)
PROXY_SECONDARY_METHOD1(void,
                        AddEncodedSink,
                        VideoSinkInterface<RecordableEncodedFrame>*)
PROXY_SECONDARY_METHOD1(void,
                        RemoveEncodedSink,
                        VideoSinkInterface<RecordableEncodedFrame>*)
PROXY_SECONDARY_METHOD1(void,
                        ProcessConstraints,
                        const VideoTrackSourceConstraints&)
END_PROXY_MAP(VideoTrackSource)

}  // namespace webrtc

#endif  // PC_VIDEO_TRACK_SOURCE_PROXY_H_
