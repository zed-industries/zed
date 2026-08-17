// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_ERROR_H_

namespace blink {

enum WebLockOrientationError {
  // If locking isn't available on the platform.
  kWebLockOrientationErrorNotAvailable,

  // If fullscreen is required to lock.
  kWebLockOrientationErrorFullscreenRequired,

  // If another lock/unlock got called before that one ended.
  kWebLockOrientationErrorCanceled,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_WEB_LOCK_ORIENTATION_ERROR_H_
