// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_VIDEO_WEBRTC_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_VIDEO_WEBRTC_SINK_H_

#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {
class MediaStreamVideoTrack;
class PeerConnectionDependencyFactory;
class WebRtcVideoTrackSource;

// MediaStreamVideoWebRtcSink is an adapter between a
// MediaStreamVideoTrack object and a webrtc VideoTrack that is
// currently sent on a PeerConnection.
// The responsibility of the class is to create and own a representation of a
// webrtc VideoTrack that can be added and removed from a RTCPeerConnection. An
// instance of MediaStreamVideoWebRtcSink is created when a VideoTrack is added
// to an RTCPeerConnection object.
// Instances of this class is owned by the WebRtcMediaStreamAdapter object that
// created it.
class MODULES_EXPORT MediaStreamVideoWebRtcSink : public MediaStreamVideoSink {
 public:
  MediaStreamVideoWebRtcSink(
      MediaStreamComponent* component,
      PeerConnectionDependencyFactory* factory,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  MediaStreamVideoWebRtcSink(const MediaStreamVideoWebRtcSink&) = delete;
  MediaStreamVideoWebRtcSink& operator=(const MediaStreamVideoWebRtcSink&) =
      delete;

  ~MediaStreamVideoWebRtcSink() override;

  webrtc::VideoTrackInterface* webrtc_video_track() {
    return video_track_.get();
  }

  std::optional<bool> SourceNeedsDenoisingForTesting() const;

  double GetRequiredMinFramesPerSec() const override { return 1; }

 protected:
  // Implementation of MediaStreamSink.
  void OnEnabledChanged(bool enabled) override;
  void OnContentHintChanged(
      WebMediaStreamTrack::ContentHintType content_hint) override;
  void OnVideoConstraintsChanged(std::optional<double> min_fps,
                                 std::optional<double> max_fps) override;

 private:
  FRIEND_TEST_ALL_PREFIXES(
      MediaStreamVideoWebRtcSinkTest,
      ForwardsConstraintsChangeToWebRtcVideoTrackSourceProxy);

  // Used to DCHECK that we are called on the correct thread.
  SEQUENCE_CHECKER(sequence_checker_);

  // |video_source_| and |video_source_proxy_| are held as
  // references to outlive |video_track_| since the interfaces between them
  // don't use reference counting.
  scoped_refptr<WebRtcVideoTrackSource> video_source_;
  scoped_refptr<webrtc::VideoTrackSourceInterface> video_source_proxy_;
  scoped_refptr<webrtc::VideoTrackInterface> video_track_;

  class WebRtcVideoSourceAdapter;
  scoped_refptr<WebRtcVideoSourceAdapter> source_adapter_;

  // Provides WebRtcVideoSourceAdapter a weak reference to
  // MediaStreamVideoWebRtcSink in order to allow it to request refresh frames.
  // See comments in media_stream_video_webrtc_sink.cc.
  //
  // TODO(crbug.com/787254): Make this object Oilpan-able, and get
  // rid of this weak prt factory use.
  base::WeakPtrFactory<MediaStreamVideoWebRtcSink> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_VIDEO_WEBRTC_SINK_H_
