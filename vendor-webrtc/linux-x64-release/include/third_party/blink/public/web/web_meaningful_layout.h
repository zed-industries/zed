// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEANINGFUL_LAYOUT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEANINGFUL_LAYOUT_H_

namespace blink {

enum class WebMeaningfulLayout {
  // Signifies that one of the following things were involved during the layout:
  // * > 200 text characters
  // * > 1024 image pixels
  // * a plugin
  // * a canvas
  // An approximation for first layout that resulted in pixels on screen.
  // Not the best heuristic, and we should replace it with something better.
  kVisuallyNonEmpty,
  // First layout of a frame immediately after the parsing finished.
  kFinishedParsing,
  // First layout of a frame immediately after the loading finished.
  kFinishedLoading
};
}

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEANINGFUL_LAYOUT_H_
