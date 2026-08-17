// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DOWNLOAD_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DOWNLOAD_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_input_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlDownloadButtonElement final
    : public MediaControlInputElement {
 public:
  explicit MediaControlDownloadButtonElement(MediaControlsImpl&);

  // Returns true if the download button should be shown. We should
  // show the button for only non-MSE, non-EME, and non-MediaStream content.
  bool ShouldDisplayDownloadButton() const;

  // MediaControlInputElement overrides.
  // TODO(mlamouri): add WillRespondToMouseClickEvents
  int GetOverflowStringId() const final;
  bool HasOverflowButton() const final;
  bool IsControlPanelButton() const final;

  void Trace(Visitor*) const override;

 protected:
  const char* GetNameForHistograms() const final;

 private:
  // This is used for UMA histogram (Media.Controls.Download). New values should
  // be appended only and must be added before |Count|.
  enum class DownloadActionMetrics {
    kShown = 0,
    kClicked,
    kCount  // Keep last.
  };

  void DefaultEventHandler(Event&) final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DOWNLOAD_BUTTON_ELEMENT_H_
