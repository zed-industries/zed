/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains the implementation of MediaStreamInterface interface.

#ifndef PC_MEDIA_STREAM_H_
#define PC_MEDIA_STREAM_H_

#include <string>

#include "api/media_stream_interface.h"
#include "api/notifier.h"
#include "api/scoped_refptr.h"

namespace webrtc {

class MediaStream : public Notifier<MediaStreamInterface> {
 public:
  static scoped_refptr<MediaStream> Create(const std::string& id);

  std::string id() const override { return id_; }

  bool AddTrack(scoped_refptr<AudioTrackInterface> track) override;
  bool AddTrack(scoped_refptr<VideoTrackInterface> track) override;
  bool RemoveTrack(scoped_refptr<AudioTrackInterface> track) override;
  bool RemoveTrack(scoped_refptr<VideoTrackInterface> track) override;
  scoped_refptr<AudioTrackInterface> FindAudioTrack(
      const std::string& track_id) override;
  scoped_refptr<VideoTrackInterface> FindVideoTrack(
      const std::string& track_id) override;

  AudioTrackVector GetAudioTracks() override { return audio_tracks_; }
  VideoTrackVector GetVideoTracks() override { return video_tracks_; }

 protected:
  explicit MediaStream(const std::string& id);

 private:
  template <typename TrackVector, typename Track>
  bool AddTrack(TrackVector* Tracks, scoped_refptr<Track> track);
  template <typename TrackVector>
  bool RemoveTrack(TrackVector* Tracks,
                   scoped_refptr<MediaStreamTrackInterface> track);

  const std::string id_;
  AudioTrackVector audio_tracks_;
  VideoTrackVector video_tracks_;
};

}  // namespace webrtc

#endif  // PC_MEDIA_STREAM_H_
