// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EMPTY_WEB_MEDIA_PLAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EMPTY_WEB_MEDIA_PLAYER_H_

#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "media/base/picture_in_picture_events_info.h"
#include "third_party/blink/public/platform/web_media_player.h"

namespace cc {
class PaintCanvas;
class PaintFlags;
}  // namespace cc

namespace blink {

// An empty WebMediaPlayer used only for tests. This class defines the methods
// of WebMediaPlayer so that mock WebMediaPlayers don't need to redefine them if
// they don't care their behavior.
class EmptyWebMediaPlayer : public WebMediaPlayer {
 public:
  ~EmptyWebMediaPlayer() override = default;

  LoadTiming Load(LoadType,
                  const WebMediaPlayerSource&,
                  CorsMode,
                  bool is_cache_disabled) override;
  void Play() override {}
  void Pause(PauseReason pause_reason) override {}
  void Seek(double seconds) override {}
  void SetRate(double) override {}
  void SetVolume(double) override {}
  void SetLatencyHint(double) override {}
  void SetPreservesPitch(bool) override {}
  void SetWasPlayedWithUserActivationAndHighMediaEngagement(bool) override {}
  void SetShouldPauseWhenFrameIsHidden(bool) override {}
  void OnRequestPictureInPicture() override {}
  WebTimeRanges Buffered() const override;
  WebTimeRanges Seekable() const override;
  void OnFrozen() override {}
  bool SetSinkId(const WebString& sink_id,
                 WebSetSinkIdCompleteCallback) override {
    return false;
  }
  bool HasVideo() const override { return false; }
  bool HasAudio() const override { return false; }
  gfx::Size NaturalSize() const override;
  gfx::Size VisibleSize() const override;
  bool Paused() const override { return false; }
  bool Seeking() const override { return false; }
  double Duration() const override { return 0.0; }
  double CurrentTime() const override { return 0.0; }
  bool IsEnded() const override { return false; }
  NetworkState GetNetworkState() const override { return kNetworkStateIdle; }
  ReadyState GetReadyState() const override { return kReadyStateHaveNothing; }
  WebString GetErrorMessage() const override;
  bool DidLoadingProgress() override { return false; }
  bool WouldTaintOrigin() const override { return false; }
  double MediaTimeForTimeValue(double time_value) const override {
    return time_value;
  }
  unsigned DecodedFrameCount() const override { return 0; }
  unsigned DroppedFrameCount() const override { return 0; }
  uint64_t AudioDecodedByteCount() const override { return 0; }
  uint64_t VideoDecodedByteCount() const override { return 0; }
  void SetVolumeMultiplier(double multiplier) override {}
  void SetPowerExperimentState(bool enabled) override {}
  void SuspendForFrameClosed() override {}
  void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) override {}
  void Paint(cc::PaintCanvas*, const gfx::Rect&, cc::PaintFlags&) override {}
  scoped_refptr<media::VideoFrame> GetCurrentFrameThenUpdate() override;
  std::optional<media::VideoFrame::ID> CurrentFrameId() const override;
  bool HasAvailableVideoFrame() const override { return false; }
  bool HasReadableVideoFrame() const override { return false; }
  base::WeakPtr<WebMediaPlayer> AsWeakPtr() override {
    return weak_ptr_factory_.GetWeakPtr();
  }
  void RegisterFrameSinkHierarchy() override {}
  void UnregisterFrameSinkHierarchy() override {}
  bool PassedTimingAllowOriginCheck() const override { return true; }

 private:
  base::WeakPtrFactory<EmptyWebMediaPlayer> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EMPTY_WEB_MEDIA_PLAYER_H_
