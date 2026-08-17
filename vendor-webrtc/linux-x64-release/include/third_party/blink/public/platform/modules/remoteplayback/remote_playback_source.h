// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_SOURCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_SOURCE_H_

namespace blink {

inline constexpr char kRemotePlaybackPresentationUrlScheme[] =
    "remote-playback";
// The path for RemotePlayback Urls on both Android and Desktop.
// Needs to be in sync with
// components/media_router/browser/android/java/src/org/chromium/components/media_router/caf/remoting/RemotingMediaSource.java
inline constexpr char kRemotePlaybackPresentationUrlPath[] =
    "remote-playback:media-element";

// The format for RemotePlayback Urls on desktop.
inline constexpr char kRemotePlaybackDesktopUrlFormat[] =
    "remote-playback:media-session?video_codec=%s&audio_codec=%s&tab_id=%d";

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_SOURCE_H_
