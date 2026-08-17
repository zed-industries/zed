/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_VIDEO_TRACK_H_
#define PC_VIDEO_TRACK_H_

#include <optional>
#include <string>

#include "api/media_stream_interface.h"
#include "api/media_stream_track.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "media/base/video_source_base.h"
#include "pc/video_track_source_proxy.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// TODO(tommi): Instead of inheriting from `MediaStreamTrack<>`, implement the
// properties directly in this class. `MediaStreamTrack` doesn't guard against
// conflicting access, so we'd need to override those methods anyway in this
// class in order to make sure things are correctly checked.
class VideoTrack : public MediaStreamTrack<VideoTrackInterface>,
                   public VideoSourceBaseGuarded,
                   public ObserverInterface {
 public:
  static scoped_refptr<VideoTrack> Create(
      absl::string_view label,
      scoped_refptr<VideoTrackSourceInterface> source,
      Thread* worker_thread);

  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;
  void RequestRefreshFrame() override;
  VideoTrackSourceInterface* GetSource() const override;

  void set_should_receive(bool should_receive) override;
  bool should_receive() const override;

  ContentHint content_hint() const override;
  void set_content_hint(ContentHint hint) override;
  bool set_enabled(bool enable) override;
  bool enabled() const override;
  MediaStreamTrackInterface::TrackState state() const override;
  std::string kind() const override;

  // Direct access to the non-proxied source object for internal implementation.
  VideoTrackSourceInterface* GetSourceInternal() const;

 protected:
  VideoTrack(
      absl::string_view id,
      scoped_refptr<
          VideoTrackSourceProxyWithInternal<VideoTrackSourceInterface>> source,
      Thread* worker_thread);
  ~VideoTrack();

 private:
  // Implements ObserverInterface. Observes `video_source_` state.
  void OnChanged() override;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker signaling_thread_;
  Thread* const worker_thread_;
  const scoped_refptr<
      VideoTrackSourceProxyWithInternal<VideoTrackSourceInterface>>
      video_source_;
  ContentHint content_hint_ RTC_GUARDED_BY(&signaling_thread_);
  // Cached `enabled` state for the worker thread. This is kept in sync with
  // the state maintained on the signaling thread via set_enabled() but can
  // be queried without blocking on the worker thread by callers that don't
  // use an api proxy to call the `enabled()` method.
  bool enabled_w_ RTC_GUARDED_BY(worker_thread_) = true;
  bool should_receive_ RTC_GUARDED_BY(signaling_thread_) = true;
};

}  // namespace webrtc

#endif  // PC_VIDEO_TRACK_H_
