/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_MEDIA_STREAM_PROXY_H_
#define PC_MEDIA_STREAM_PROXY_H_

#include <string>

#include "api/media_stream_interface.h"
#include "pc/proxy.h"

namespace webrtc {

// TODO(deadbeef): Move this to a .cc file. What threads methods are called on
// is an implementation detail.
BEGIN_PRIMARY_PROXY_MAP(MediaStream)
PROXY_PRIMARY_THREAD_DESTRUCTOR()
BYPASS_PROXY_CONSTMETHOD0(std::string, id)
PROXY_METHOD0(AudioTrackVector, GetAudioTracks)
PROXY_METHOD0(VideoTrackVector, GetVideoTracks)
PROXY_METHOD1(scoped_refptr<AudioTrackInterface>,
              FindAudioTrack,
              const std::string&)
PROXY_METHOD1(scoped_refptr<VideoTrackInterface>,
              FindVideoTrack,
              const std::string&)
PROXY_METHOD1(bool, AddTrack, scoped_refptr<AudioTrackInterface>)
PROXY_METHOD1(bool, AddTrack, scoped_refptr<VideoTrackInterface>)
PROXY_METHOD1(bool, RemoveTrack, scoped_refptr<AudioTrackInterface>)
PROXY_METHOD1(bool, RemoveTrack, scoped_refptr<VideoTrackInterface>)
PROXY_METHOD1(void, RegisterObserver, ObserverInterface*)
PROXY_METHOD1(void, UnregisterObserver, ObserverInterface*)
END_PROXY_MAP(MediaStream)

}  // namespace webrtc

#endif  // PC_MEDIA_STREAM_PROXY_H_
