// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_REQUEST_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_REQUEST_TYPE_H_

#include "base/check.h"
#include "base/dcheck_is_on.h"

#if DCHECK_IS_ON()
#include <string>
#endif

namespace blink {

// This enum class represents Implementation-internal details for the fullscreen
// request, these are in addition to API options from fullscreen_options.idl
// that are passed separately.
//
// The integer values are powers of two for use as a flag bitmap. The class
// provides minimal operators for combining flags and checking if a specific
// flag is set.
enum class FullscreenRequestType {
  // No bits set, equivalent to unprefixed with no other properties
  kNull = 0,

  // False for Element.requestFullscreen(), true for
  // Element.webkitRequestFullscreen()/webkitRequestFullScreen() and
  // HTMLVideoElement.webkitEnterFullscreen()/webkitEnterFullScreen()
  kPrefixed = 1,

  // For WebRemoteFrameImpl to notify that a cross-process descendant frame
  // has requested and is about to enter fullscreen.
  kForCrossProcessDescendant = 2,

  // For WebXR DOM Overlay, in this mode the element and parent iframes use a
  // transparent background.
  kForXrOverlay = 4,

  // For WebXR Immersive AR sessions with access to the camera, in this mode,
  // the status bar should stay visible.
  kForXrArWithCamera = 8,

  // Convenience value for "no flags".
  kUnprefixed = kNull,
};

inline FullscreenRequestType operator|(FullscreenRequestType lhs,
                                       FullscreenRequestType rhs) {
  return static_cast<FullscreenRequestType>(static_cast<int>(lhs) |
                                            static_cast<int>(rhs));
}

// Returns true if lhs and rhs have at least one flag bit in common.
inline bool operator&(FullscreenRequestType lhs, FullscreenRequestType rhs) {
  return static_cast<int>(lhs) & static_cast<int>(rhs);
}

#if DCHECK_IS_ON()
std::string FullscreenRequestTypeToDebugString(FullscreenRequestType req);
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_REQUEST_TYPE_H_
