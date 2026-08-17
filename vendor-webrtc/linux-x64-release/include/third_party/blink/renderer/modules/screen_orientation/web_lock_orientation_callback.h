// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_CALLBACK_H_

#include "third_party/blink/renderer/modules/screen_orientation/web_lock_orientation_error.h"

namespace blink {

// WebLockOrientationCallback is an interface to be used by the embedder in
// order to inform Blink when a screen lock operation has succeeded or failed.
// A success notification comes with the new orientation angle and orientation
// type and a failure notification comes with an information about the type of
// failure.
class WebLockOrientationCallback {
 public:
  virtual ~WebLockOrientationCallback() = default;

  virtual void OnSuccess() = 0;
  virtual void OnError(WebLockOrientationError) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_CALLBACK_H_
