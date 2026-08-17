// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SPELL_CHECK_PANEL_HOST_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SPELL_CHECK_PANEL_HOST_CLIENT_H_

namespace blink {

class WebString;

class WebSpellCheckPanelHostClient {
 public:
  // Show or hide the spelling panel UI.
  virtual void ShowSpellingUI(bool show) = 0;

  // Returns true if the spelling panel UI is showing.
  virtual bool IsShowingSpellingUI() = 0;

  // Update the spelling panel UI with the given |word|. Pass the empty string
  // to indicate that there are no misspelled words left.
  virtual void UpdateSpellingUIWithMisspelledWord(const WebString& word) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SPELL_CHECK_PANEL_HOST_CLIENT_H_
