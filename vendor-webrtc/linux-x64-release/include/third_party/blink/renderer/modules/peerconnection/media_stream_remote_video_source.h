// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_REMOTE_VIDEO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_REMOTE_VIDEO_SOURCE_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace blink {

class TrackObserver;

// MediaStreamRemoteVideoSource implements the MediaStreamVideoSource
// interface for video tracks received on a PeerConnection. The purpose of the
// class is to make sure there is no difference between a video track where the
// source is a local source and a video track where the source is a remote video
// track.
class MODULES_EXPORT MediaStreamRemoteVideoSource
    : public MediaStreamVideoSource {
 public:
  explicit MediaStreamRemoteVideoSource(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      std::unique_ptr<TrackObserver> observer);

  MediaStreamRemoteVideoSource(const MediaStreamRemoteVideoSource&) = delete;
  MediaStreamRemoteVideoSource& operator=(const MediaStreamRemoteVideoSource&) =
      delete;

  ~MediaStreamRemoteVideoSource() override;

  // Should be called when the remote video track this source originates from is
  // no longer received on a PeerConnection. This cleans up the references to
  // the webrtc::MediaStreamTrackInterface instance held by |observer_|.
  void OnSourceTerminated();

  // MediaStreamVideoSource overrides.
  bool SupportsEncodedOutput() const override;
  void RequestKeyFrame() override;
  base::WeakPtr<MediaStreamVideoSource> GetWeakPtr() override;

 protected:
  // Implements MediaStreamVideoSource.
  void StartSourceImpl(
      MediaStreamVideoSourceCallbacks media_stream_callbacks) override;
  void StopSourceImpl() override;
  void OnEncodedSinkEnabled() override;
  void OnEncodedSinkDisabled() override;

  // Used by tests to test that a frame can be received and that the
  // MediaStreamRemoteVideoSource behaves as expected.
  webrtc::VideoSinkInterface<webrtc::VideoFrame>* SinkInterfaceForTesting();
  webrtc::VideoSinkInterface<webrtc::RecordableEncodedFrame>*
  EncodedSinkInterfaceForTesting();

 private:
  void OnChanged(webrtc::MediaStreamTrackInterface::TrackState state);

  // Internal class used for receiving frames from the webrtc track on a
  // libjingle thread and forward it to the IO-thread.
  class RemoteVideoSourceDelegate;
  scoped_refptr<RemoteVideoSourceDelegate> delegate_;
  std::unique_ptr<TrackObserver> observer_;

  base::WeakPtrFactory<MediaStreamVideoSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_REMOTE_VIDEO_SOURCE_H_
