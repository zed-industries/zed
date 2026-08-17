/*
 * Copyright (C) 2011, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2011, 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_IMPL_H_

#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/renderer/core/geometry/dom_rect_read_only.h"
#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/core/html/media/media_controls.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_track_selector_list_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class Event;
class HTMLVideoElement;
class MediaControlsMediaEventListener;
class MediaControlsDisplayCutoutDelegate;
class MediaControlsOrientationLockDelegate;
class MediaControlsRotateToFullscreenDelegate;
class MediaControlsWindowEventListener;
class MediaControlAnimatedArrowContainerElement;
class MediaControlButtonPanelElement;
class MediaControlCastButtonElement;
class MediaControlCurrentTimeDisplayElement;
class MediaControlDownloadButtonElement;
class MediaControlFullscreenButtonElement;
class MediaControlLoadingPanelElement;
class MediaControlMuteButtonElement;
class MediaControlDisplayCutoutFullscreenButtonElement;
class MediaControlOverflowMenuButtonElement;
class MediaControlOverflowMenuListElement;
class MediaControlOverlayEnclosureElement;
class MediaControlOverlayPlayButtonElement;
class MediaControlPanelElement;
class MediaControlPanelEnclosureElement;
class MediaControlPictureInPictureButtonElement;
class MediaControlPlaybackSpeedButtonElement;
class MediaControlPlaybackSpeedListElement;
class MediaControlPlayButtonElement;
class MediaControlRemainingTimeDisplayElement;
class MediaControlScrubbingMessageElement;
class MediaControlTextTrackListElement;
class MediaControlsTextTrackManager;
class MediaControlTimelineElement;
class MediaControlTrackSelectorMenuButtonElement;
class MediaControlToggleClosedCaptionsButtonElement;
class MediaControlVolumeControlContainerElement;
class MediaControlVolumeSliderElement;
class ShadowRoot;
class TextTrack;

// Default implementation of the core/ MediaControls interface used by
// HTMLMediaElement.
class MODULES_EXPORT MediaControlsImpl final : public HTMLDivElement,
                                               public MediaControls {
 public:
  static MediaControlsImpl* Create(HTMLMediaElement&, ShadowRoot&);

  explicit MediaControlsImpl(HTMLMediaElement&);

  MediaControlsImpl(const MediaControlsImpl&) = delete;
  MediaControlsImpl& operator=(const MediaControlsImpl&) = delete;

  ~MediaControlsImpl() override = default;

  // Returns whether the event is considered a touch event.
  static bool IsTouchEvent(Event*);

  // Node override.
  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  // MediaControls implementation.
  void MaybeShow() override;
  void Hide() override;
  void Reset() override;
  void OnControlsListUpdated() override;
  // TODO(mlamouri): this is temporary to notify the controls that an
  // HTMLTrackElement failed to load because there is no web exposed way to
  // be notified on the TextTrack object. See https://crbug.com/669977
  void OnTrackElementFailedToLoad() override { OnTextTracksAddedOrRemoved(); }
  // Notify us that the media element's network state has changed.
  void NetworkStateChanged() override;
  LayoutObject* PanelLayoutObject() override;
  LayoutObject* TimelineLayoutObject() override;
  LayoutObject* ButtonPanelLayoutObject() override;
  LayoutObject* ContainerLayoutObject() override;
  void SetTestMode(bool) override;
  // Return the internal elements, which is used by registering clicking
  // EventHandlers from MediaControlsWindowEventListener.
  HTMLDivElement* PanelElement() override;
  HTMLDivElement* ButtonPanelElement();
  // TODO(mlamouri): this method is needed in order to notify the controls that
  // the `MediaControlsEnabled` setting has changed.
  void OnMediaControlsEnabledChange() override {
    // There is no update because only the overlay is expected to change.
    RefreshCastButtonVisibilityWithoutUpdate();
  }

  // Called by the fullscreen buttons to toggle fulllscreen on/off.
  void EnterFullscreen();
  void ExitFullscreen();

  // Called by the MediaControlOverlayPlayButtonElement to check if toggling
  // fullscreen is allowed.
  bool IsFullscreenEnabled() const;

  // Text track related methods exposed to components handling closed captions.
  void ToggleTextTrackList();
  bool TextTrackListIsWanted();
  MediaControlsTextTrackManager& GetTextTrackManager();

  // Methods related to the playback speed menu.
  void TogglePlaybackSpeedList();
  bool PlaybackSpeedListIsWanted();

  // Methods related to the track selection menu.
  void ToggleTrackSelectionList(WebMediaPlayer::TrackType);

  // Methods related to the overflow menu.
  void OpenOverflowMenu();
  void CloseOverflowMenu();
  bool OverflowMenuIsWanted();

  void ToggleOverflowMenu();
  bool OverflowMenuVisible();

  void VolumeSliderWantedTimerFired(TimerBase*);
  void OpenVolumeSliderIfNecessary();
  void CloseVolumeSliderIfNecessary();

  void ShowOverlayCastButtonIfNeeded();

  // Methods call by the scrubber.
  void BeginScrubbing(bool);
  void EndScrubbing();
  void UpdateCurrentTimeDisplay();

  // Methods used for Download In-product help.
  const MediaControlOverflowMenuButtonElement& OverflowButton() const;
  MediaControlOverflowMenuButtonElement& OverflowButton();

  // Accessors for UI elements.
  const MediaControlCurrentTimeDisplayElement& CurrentTimeDisplay() const;
  const MediaControlRemainingTimeDisplayElement& RemainingTimeDisplay() const;
  MediaControlToggleClosedCaptionsButtonElement& ToggleClosedCaptions();

  void Trace(Visitor*) const override;

  // Track the state of the controls.
  enum ControlsState {
    // There is no video source.
    kNoSource,

    // Metadata has not been loaded.
    kNotLoaded,

    // Metadata is being loaded and the media will not play once it's loaded.
    kLoadingMetadataPaused,

    // Metadata is being loaded and the media will play once it's loaded.
    kLoadingMetadataPlaying,

    // Metadata is loaded and the media is ready to play. This can be when the
    // media is paused, when it has ended or before the media has started
    // playing.
    kStopped,

    // The media is playing.
    kPlaying,

    // Playback has stopped to buffer.
    kBuffering,

    // The media is being scrubbed.
    kScrubbing,
  };
  ControlsState State() const;

  void MaybeToggleControlsFromTap();

  // Called by accessibility code to notify that the controls was focused by an
  // accessibility tool. This is meant to be replaced by AOM when the event will
  // be exposed to the platform.
  void OnAccessibleFocus();
  void OnAccessibleBlur();

  // Returns true/false based on which set of controls to display.
  bool ShouldShowAudioControls() const;

 private:
  // MediaControlsMediaEventListener is a component that is listening to events
  // and calling the appropriate callback on MediaControlsImpl. The object is
  // split from MedaiControlsImpl to reduce boilerplate and ease reading. In
  // order to not expose accessors only for this component, a friendship is
  // declared.
  friend class MediaControlsMediaEventListener;
  // Same as above but handles the menus hiding when the window is interacted
  // with, allowing MediaControlsImpl to not have the boilerplate.
  friend class MediaControlsWindowEventListener;

  // For tests.
  friend class MediaControlsDisplayCutoutDelegateTest;
  friend class MediaControlsOrientationLockDelegateTest;
  friend class MediaControlsOrientationLockAndRotateToFullscreenDelegateTest;
  friend class MediaControlsRotateToFullscreenDelegateTest;
  friend class MediaControlsImplTest;
  friend class MediaControlsImplInProductHelpTest;
  friend class MediaControlDisplayCutoutFullscreenButtonElementTest;
  friend class MediaControlPopupMenuElementTest;
  friend class MediaControlTimelineElementTest;

  // Need to be members of MediaControls for private member access.
  class BatchedControlUpdate;
  class MediaControlsResizeObserverDelegate;
  class MediaElementMutationCallback;

  // Notify us that our controls enclosure has changed size.
  void NotifyElementSizeChanged(DOMRectReadOnly* new_size);

  // Update the CSS class when we think the state has updated.
  void UpdateCSSClassFromState();

  // Sets/removes a CSS class from this element based on |should_have_class|.
  void SetClass(const String& class_name, bool should_have_class);

  // Get the HTMLVideoElement that the controls are attached to. The caller must
  // check that the element is a video element first.
  HTMLVideoElement& VideoElement();

  void InitializeControls();
  void PopulatePanel();

  // Attach hover background div to buttons
  void AttachHoverBackground(Element*);

  void MakeOpaque();
  void MakeOpaqueFromPointerEvent();
  void MakeTransparent();
  bool IsVisible() const;

  // If the overlay play button is present then make sure it is displayed.
  void MaybeShowOverlayPlayButton();

  void UpdatePlayState();

  enum HideBehaviorFlags {
    kIgnoreNone = 0,
    kIgnoreVideoHover = 1 << 0,
    kIgnoreFocus = 1 << 1,
    kIgnoreControlsHover = 1 << 2,
    kIgnoreWaitForTimer = 1 << 3,
  };

  bool ShouldHideMediaControls(unsigned behavior_flags = 0) const;
  bool AreVideoControlsHovered() const;
  void HideMediaControlsTimerFired(TimerBase*);
  void StartHideMediaControlsIfNecessary();
  void StartHideMediaControlsTimer();
  void StopHideMediaControlsTimer();
  void ResetHideMediaControlsTimer();
  void HideCursor();
  void ShowCursor();

  bool ShouldOpenVolumeSlider() const;
  bool ShouldCloseVolumeSlider() const;

  void ElementSizeChangedTimerFired(TimerBase*);

  // Update any visible indicators of the current time.
  void UpdateTimeIndicators(bool suppress_aria = false);

  // Hide elements that don't fit, and show those things that we want which
  // do fit.  This requires that m_effectiveWidth and m_effectiveHeight are
  // current.
  void ComputeWhichControlsFit();

  void HidePopupMenu();
  void UpdateOverflowMenuWanted() const;
  void UpdateOverflowMenuItemCSSClass() const;
  void UpdateScrubbingMessageFits() const;
  void UpdateSizingCSSClass();
  void MaybeRecordElementsDisplayed() const;

  // Takes a popup menu (caption, overflow) and position on the screen. This is
  // used because these menus use a fixed position in order to appear over all
  // content.
  void PositionPopupMenu(Element*);

  // When a video element has an audio track but no video track, we modify the
  // controls to display like audio controls.
  bool ShouldActAsAudioControls() const;
  void StartActingAsAudioControls();
  void StopActingAsAudioControls();
  void UpdateActingAsAudioControls();

  // Returns true/false based on which set of controls to display.
  bool ShouldShowVideoControls() const;

  // Returns true/false based on whether this player is showing live content,
  // and should have no seek bar or timestamp.
  bool IsLivePlayback() const;

  // Node
  bool IsMediaControls() const override { return true; }
  bool WillRespondToMouseMoveEvents() const override { return true; }
  void DefaultEventHandler(Event&) override;
  bool ContainsRelatedTarget(Event*);

  void HandlePointerEvent(Event*);
  void HandleClickEvent(Event*);
  void HandleTouchEvent(Event*);

  void EnsureAnimatedArrowContainer();
  void MaybeJump(int);
  bool IsOnLeftSide(Event*);
  void TapTimerFired(TimerBase*);

  // Internal cast related methods.
  void RemotePlaybackStateChanged();
  void RefreshCastButtonVisibility();
  void RefreshCastButtonVisibilityWithoutUpdate();

  // Methods called by MediaControlsMediaEventListener.
  void OnVolumeChange();
  void OnFocusIn();
  void OnTimeUpdate();
  void OnDurationChange();
  void OnPlay();
  void OnPlaying();
  void OnPause();
  void OnSeeking();
  void OnSeeked();
  void OnTextTracksAddedOrRemoved();
  void OnTextTracksChanged();
  void OnError();
  void OnLoadedMetadata();
  void OnEnteredFullscreen();
  void OnExitedFullscreen();
  void OnPictureInPictureChanged();
  void OnPanelKeypress();
  void OnMediaKeyboardEvent(Event* event) { DefaultEventHandler(*event); }
  void OnWaiting();
  void OnLoadingProgress();
  void OnLoadedData();

  // Media control elements.
  Member<MediaControlOverlayEnclosureElement> overlay_enclosure_;
  Member<MediaControlOverlayPlayButtonElement> overlay_play_button_;
  Member<MediaControlCastButtonElement> overlay_cast_button_;
  Member<MediaControlPanelEnclosureElement> enclosure_;
  Member<MediaControlPanelElement> panel_;
  Member<MediaControlPlayButtonElement> play_button_;
  Member<MediaControlTimelineElement> timeline_;
  Member<MediaControlScrubbingMessageElement> scrubbing_message_;
  Member<MediaControlCurrentTimeDisplayElement> current_time_display_;
  Member<MediaControlRemainingTimeDisplayElement> duration_display_;
  Member<MediaControlMuteButtonElement> mute_button_;
  Member<MediaControlVolumeSliderElement> volume_slider_;
  Member<MediaControlVolumeControlContainerElement> volume_control_container_;
  Member<MediaControlToggleClosedCaptionsButtonElement>
      toggle_closed_captions_button_;
  Member<MediaControlTextTrackListElement> text_track_list_;
  Member<MediaControlTrackSelectorMenuButtonElement>
      audio_track_selector_button_;
  Member<MediaControlTrackSelectorMenuButtonElement>
      video_track_selector_button_;
  Member<MediaControlTrackSelectorListElement> video_track_selector_list_;
  Member<MediaControlTrackSelectorListElement> audio_track_selector_list_;

  Member<MediaControlPlaybackSpeedButtonElement> playback_speed_button_;
  Member<MediaControlPlaybackSpeedListElement> playback_speed_list_;
  Member<MediaControlOverflowMenuButtonElement> overflow_menu_;
  Member<MediaControlOverflowMenuListElement> overflow_list_;
  Member<MediaControlButtonPanelElement> media_button_panel_;
  Member<MediaControlLoadingPanelElement> loading_panel_;
  Member<MediaControlPictureInPictureButtonElement> picture_in_picture_button_;
  Member<MediaControlAnimatedArrowContainerElement>
      animated_arrow_container_element_;

  Member<MediaControlCastButtonElement> cast_button_;
  Member<MediaControlFullscreenButtonElement> fullscreen_button_;
  Member<MediaControlDisplayCutoutFullscreenButtonElement>
      display_cutout_fullscreen_button_;
  Member<MediaControlDownloadButtonElement> download_button_;

  Member<MediaControlsMediaEventListener> media_event_listener_;
  Member<MediaControlsOrientationLockDelegate> orientation_lock_delegate_;
  Member<MediaControlsRotateToFullscreenDelegate>
      rotate_to_fullscreen_delegate_;
  Member<MediaControlsDisplayCutoutDelegate> display_cutout_delegate_;

  HeapTaskRunnerTimer<MediaControlsImpl> hide_media_controls_timer_;
  unsigned hide_timer_behavior_flags_;
  bool is_mouse_over_controls_ : 1;
  bool is_paused_for_scrubbing_ : 1;
  bool is_scrubbing_ = false;

  // When controls are hidden, we defer CSS updates on them in order to avoid
  // unnecessary style calculation. When controls transition from shown to
  // hidden, we set this flag to true to ensure that one final style update
  // takes place in order to eliminate states such as scrubbing.
  bool is_hiding_controls_ = false;

  // Watches the video element for resize and updates media controls as
  // necessary.
  Member<ResizeObserver> resize_observer_;

  // Watches the media element for attribute changes and updates media controls
  // as necessary.
  Member<MediaElementMutationCallback> element_mutation_callback_;

  HeapTaskRunnerTimer<MediaControlsImpl> element_size_changed_timer_;
  gfx::Size size_;

  bool keep_showing_until_timer_fires_ : 1;

  bool is_acting_as_audio_controls_ = false;

  // Our best guess on whether the user is interacting with the controls via
  // touch (as opposed to mouse). This is used to determine how to handle
  // certain pointer events. In particular, when the user is interacting via
  // touch events, we want to ignore pointerover/pointerout/pointermove events.
  bool is_touch_interaction_ = false;

  // Timer for distinguishing double-taps.
  HeapTaskRunnerTimer<MediaControlsImpl> tap_timer_;
  bool is_paused_for_double_tap_ = false;

  // Timer to delay showing the volume slider to avoid accidental triggering
  // of the slider
  HeapTaskRunnerTimer<MediaControlsImpl> volume_slider_wanted_timer_;

  Member<MediaControlsTextTrackManager> text_track_manager_;

  bool is_test_mode_ = false;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_IMPL_H_
