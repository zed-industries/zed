// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_DELEGATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_DELEGATE_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_media_player.h"

namespace media {
enum class MediaContentType;
}  // namespace media

namespace blink {

enum class WebFullscreenVideoStatus;
class WebMediaPlayer;

// An interface to collect WebMediaPlayer state changes and to fan out commands
// from the browser.
class BLINK_PLATFORM_EXPORT WebMediaPlayerDelegate {
 public:
  // Note: WebMediaPlayerDelegate implementations should not call an Observer
  // method on a stack that includes a call from the player.
  // Note: It is likely that players will call WebMediaPlayerDelegate methods
  // from within Observer callbacks.
  class Observer {
   public:
    // Called when the page containing the host frame is hidden (usually by tab
    // switching or by minimizing the current browser window). Note:
    // OnPageHidden() is not called when the frame is closed, even though
    // IsPageHidden() will start returning true.
    virtual void OnPageHidden() = 0;

    // Called when the page containing the host frame is shown (usually by tab
    // switching).
    virtual void OnPageShown() = 0;

    // Called when the host frame is not rendered on the host page. This should
    // be called whenever the host frame's:
    // - 'display' property is set to 'none';
    // - 'visibility' property is set to 'hidden';
    virtual void OnFrameHidden() = 0;

    // Called when the host frame is rendered on the host page.
    virtual void OnFrameShown() = 0;

    // Called when an idle player has become stale, usually interpreted to mean
    // that it is unlikely to be interacted with in the near future.
    //
    // Players should typically respond by releasing resources, for example by
    // discarding their decoders.
    virtual void OnIdleTimeout() = 0;
  };

  // Returns true if the hosting page (WebView) is hidden or closed.
  virtual bool IsPageHidden() = 0;

  // Returns true if the WebMediaPlayer's host frame is hidden or destroyed in
  // the document. Returns false otherwise.
  virtual bool IsFrameHidden() = 0;

  // Subscribe to observer callbacks. A player must use the returned |player_id|
  // for the rest of the calls below.
  virtual int AddObserver(Observer* observer) = 0;

  // Unsubscribe from observer callbacks.
  virtual void RemoveObserver(int player_id) = 0;

  // Notify about the kind of tracks the media player has, and the type of
  // content.
  virtual void DidMediaMetadataChange(
      int player_id,
      bool has_audio,
      bool has_video,
      media::MediaContentType media_content_type) = 0;

  // Notify playback started. This will request appropriate wake locks and, if
  // applicable, show a pause button in external controls.
  //
  // DidPlay() should not be called for remote playback.
  virtual void DidPlay(int player_id) = 0;

  // Notify that playback is paused. This will drop wake locks and, if
  // applicable, show a play button in external controls.
  // TODO(sandersd): It may be helpful to get |has_audio| and |has_video| here,
  // so that we can do the right thing with media that starts paused.
  virtual void DidPause(int player_id, bool reached_end_of_stream) = 0;

  // Notify that playback is stopped. This will drop wake locks and remove any
  // external controls.
  //
  // Clients must still call RemoveObserver() to unsubscribe from observer
  // callbacks.
  virtual void PlayerGone(int player_id) = 0;

  // Set the player's idle state. While idle, a player may recieve an
  // OnIdleTimeout() callback.
  // TODO(sandersd): Merge this into DidPlay()/DidPause()/PlayerGone().
  virtual void SetIdle(int player_id, bool is_idle) = 0;

  // Get the player's idle state. A stale player is considered idle.
  // TODO(sandersd): Remove this. It is only used in tests and in one special
  // case in WMPI.
  virtual bool IsIdle(int player_id) = 0;

  // Returns a stale player to an idle state, and resumes OnIdleTimeout() calls
  // without an additional idle timeout.
  // TODO(sandersd): This exists only to support WMPI's didLoadingProgress()
  // workaround. A better option may be to take a 'minimum idle' duration in
  // SetIdle().
  virtual void ClearStaleFlag(int player_id) = 0;

  // Returns |true| if the player is stale; that is that OnIdleTimeout() was
  // called and returned |true|.
  virtual bool IsStale(int player_id) = 0;

 protected:
  WebMediaPlayerDelegate() = default;
  virtual ~WebMediaPlayerDelegate() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_DELEGATE_H_
