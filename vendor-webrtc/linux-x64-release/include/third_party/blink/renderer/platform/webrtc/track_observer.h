// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_TRACK_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_TRACK_OBSERVER_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace blink {

class PLATFORM_EXPORT TrackObserver {
 public:
  TrackObserver(const scoped_refptr<base::SingleThreadTaskRunner>& main_thread,
                const scoped_refptr<webrtc::MediaStreamTrackInterface>& track);
  TrackObserver(const TrackObserver&) = delete;
  TrackObserver& operator=(const TrackObserver&) = delete;
  ~TrackObserver();

  using OnChangedCallback = base::RepeatingCallback<void(
      webrtc::MediaStreamTrackInterface::TrackState)>;
  void SetCallback(const OnChangedCallback& callback);

  const scoped_refptr<webrtc::MediaStreamTrackInterface>& track() const;

 private:
  class TrackObserverImpl;
  const scoped_refptr<TrackObserverImpl> observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_TRACK_OBSERVER_H_
