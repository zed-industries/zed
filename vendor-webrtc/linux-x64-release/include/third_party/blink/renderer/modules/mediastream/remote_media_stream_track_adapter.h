// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_REMOTE_MEDIA_STREAM_TRACK_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_REMOTE_MEDIA_STREAM_TRACK_ADAPTER_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/web/web_frame.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/peer_connection_dependency_factory.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component_impl.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class TrackObserver;

// Base class used for mapping between webrtc and blink MediaStream tracks.
// RemoteMediaStreamImpl has a RemoteMediaStreamTrackAdapter per remote audio
// (RemoteAudioTrackAdapter) and video (RemoteVideoTrackAdapter) track.
template <typename WebRtcMediaStreamTrackType>
class MODULES_EXPORT RemoteMediaStreamTrackAdapter
    : public WTF::ThreadSafeRefCounted<
          RemoteMediaStreamTrackAdapter<WebRtcMediaStreamTrackType>> {
 public:
  RemoteMediaStreamTrackAdapter(
      const scoped_refptr<base::SingleThreadTaskRunner>& main_thread,
      WebRtcMediaStreamTrackType* webrtc_track,
      ExecutionContext* track_execution_context)
      : main_thread_(main_thread),
        webrtc_track_(webrtc_track),
        track_execution_context_(track_execution_context),
        id_(String::FromUTF8(webrtc_track->id())) {}

  RemoteMediaStreamTrackAdapter(const RemoteMediaStreamTrackAdapter&) = delete;
  RemoteMediaStreamTrackAdapter& operator=(
      const RemoteMediaStreamTrackAdapter&) = delete;

  const scoped_refptr<WebRtcMediaStreamTrackType>& observed_track() {
    return webrtc_track_;
  }

  MediaStreamComponent* track() {
    DCHECK(main_thread_->BelongsToCurrentThread());
    DCHECK(component_);
    return component_;
  }

  String id() const { return id_; }

  bool initialized() const {
    DCHECK(main_thread_->BelongsToCurrentThread());
    return !!component_;
  }

  void Initialize() {
    DCHECK(main_thread_->BelongsToCurrentThread());
    DCHECK(!initialized());
    std::move(web_initialize_).Run();
    DCHECK(initialized());
  }

 protected:
  friend class WTF::ThreadSafeRefCounted<
      RemoteMediaStreamTrackAdapter<WebRtcMediaStreamTrackType>>;

  virtual ~RemoteMediaStreamTrackAdapter() {
    DCHECK(main_thread_->BelongsToCurrentThread());
  }

  void InitializeTrack(
      MediaStreamSource::StreamType type,
      std::unique_ptr<WebPlatformMediaStreamSource> platform_source,
      std::unique_ptr<MediaStreamTrackPlatform> platform_track) {
    DCHECK(main_thread_->BelongsToCurrentThread());
    DCHECK(!component_);

    auto* source = MakeGarbageCollected<MediaStreamSource>(
        id_, type, id_, true /*remote*/, std::move(platform_source));
    component_ = MakeGarbageCollected<MediaStreamComponentImpl>(
        id_, source, std::move(platform_track));
    // If we have a reference to a window frame where the track was created,
    // store it on the component. This allows other code to use the correct
    // per-frame object for the track, such as the audio device for playout.
    if (track_execution_context_ && track_execution_context_->IsWindow() &&
        To<LocalDOMWindow>(track_execution_context_.Get())->GetFrame()) {
      // IsWindow() being true means that the ExecutionContext is a
      // LocalDOMWindow, so these casts should be safe.
      component_->SetCreationFrameGetter(WTF::BindRepeating(
          [](LocalFrame* local_frame) {
            return local_frame
                       ? WebFrame::FromCoreFrame(local_frame)->ToWebLocalFrame()
                       : nullptr;
          },
          WrapWeakPersistent(
              To<LocalDOMWindow>(track_execution_context_.Get())->GetFrame())));
    }
    DCHECK(component_);
  }

  const scoped_refptr<base::SingleThreadTaskRunner> main_thread_;
  // This callback will be run when Initialize() is called and then freed.
  // The callback is used by derived classes to bind objects that need to be
  // instantiated and initialized on the signaling thread but then moved to
  // and used on the main thread when initializing the web object(s).
  CrossThreadOnceClosure web_initialize_;

 private:
  const scoped_refptr<WebRtcMediaStreamTrackType> webrtc_track_;
  CrossThreadPersistent<MediaStreamComponent> component_;
  CrossThreadWeakPersistent<ExecutionContext> track_execution_context_;
  // const copy of the webrtc track id that allows us to check it from both the
  // main and signaling threads without incurring a synchronous thread hop.
  const String id_;
};

class MODULES_EXPORT RemoteVideoTrackAdapter
    : public RemoteMediaStreamTrackAdapter<webrtc::VideoTrackInterface> {
 public:
  // Called on the signaling thread.
  RemoteVideoTrackAdapter(
      const scoped_refptr<base::SingleThreadTaskRunner>& main_thread,
      webrtc::VideoTrackInterface* webrtc_track,
      ExecutionContext* execution_context);

 protected:
  ~RemoteVideoTrackAdapter() override;

 private:
  void InitializeWebVideoTrack(std::unique_ptr<TrackObserver> observer,
                               bool enabled);
};

// RemoteAudioTrackAdapter is responsible for listening on state
// change notifications on a remote webrtc audio MediaStreamTracks and notify
// Blink.
class MODULES_EXPORT RemoteAudioTrackAdapter
    : public RemoteMediaStreamTrackAdapter<webrtc::AudioTrackInterface>,
      public webrtc::ObserverInterface {
 public:
  // Called on the signaling thread.
  RemoteAudioTrackAdapter(
      const scoped_refptr<base::SingleThreadTaskRunner>& main_thread,
      webrtc::AudioTrackInterface* webrtc_track,
      ExecutionContext* execution_context);

  RemoteAudioTrackAdapter(const RemoteAudioTrackAdapter&) = delete;
  RemoteAudioTrackAdapter& operator=(const RemoteAudioTrackAdapter&) = delete;

  void Unregister();

 protected:
  ~RemoteAudioTrackAdapter() override;

 private:
  void InitializeWebAudioTrack(
      const scoped_refptr<base::SingleThreadTaskRunner>& main_thread);

  // webrtc::ObserverInterface implementation.
  void OnChanged() override;

  void OnChangedOnMainThread(
      webrtc::MediaStreamTrackInterface::TrackState state);

#if DCHECK_IS_ON()
  bool unregistered_;
#endif

  webrtc::MediaStreamTrackInterface::TrackState state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_REMOTE_MEDIA_STREAM_TRACK_ADAPTER_H_
