/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_MEDIA_STREAM_TRACK_H_
#define API_MEDIA_STREAM_TRACK_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/media_stream_interface.h"
#include "api/notifier.h"

namespace webrtc {

// MediaTrack implements the interface common to AudioTrackInterface and
// VideoTrackInterface.
template <typename T>
class MediaStreamTrack : public Notifier<T> {
 public:
  typedef typename T::TrackState TypedTrackState;

  std::string id() const override { return id_; }
  MediaStreamTrackInterface::TrackState state() const override {
    return state_;
  }
  bool enabled() const override { return enabled_; }
  bool set_enabled(bool enable) override {
    bool fire_on_change = (enable != enabled_);
    enabled_ = enable;
    if (fire_on_change) {
      Notifier<T>::FireOnChanged();
    }
    return fire_on_change;
  }
  void set_ended() { set_state(MediaStreamTrackInterface::TrackState::kEnded); }

 protected:
  explicit MediaStreamTrack(absl::string_view id)
      : enabled_(true), id_(id), state_(MediaStreamTrackInterface::kLive) {}

  bool set_state(MediaStreamTrackInterface::TrackState new_state) {
    bool fire_on_change = (state_ != new_state);
    state_ = new_state;
    if (fire_on_change)
      Notifier<T>::FireOnChanged();
    return true;
  }

 private:
  bool enabled_;
  const std::string id_;
  MediaStreamTrackInterface::TrackState state_;
};

}  // namespace webrtc

#endif  // API_MEDIA_STREAM_TRACK_H_
