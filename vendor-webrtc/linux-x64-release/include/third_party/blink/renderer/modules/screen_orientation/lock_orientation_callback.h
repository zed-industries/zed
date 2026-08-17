// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_LOCK_ORIENTATION_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_LOCK_ORIENTATION_CALLBACK_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/screen_orientation/web_lock_orientation_callback.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

// LockOrientationCallback is an implementation of WebLockOrientationCallback
// that will resolve the underlying promise depending on the result passed to
// the callback.
class LockOrientationCallback final : public WebLockOrientationCallback {
  USING_FAST_MALLOC(LockOrientationCallback);

 public:
  explicit LockOrientationCallback(ScriptPromiseResolver<IDLUndefined>*);

  LockOrientationCallback(const LockOrientationCallback&) = delete;
  LockOrientationCallback& operator=(const LockOrientationCallback&) = delete;

  ~LockOrientationCallback() override;

  void OnSuccess() override;
  void OnError(WebLockOrientationError) override;

 private:
  Persistent<ScriptPromiseResolver<IDLUndefined>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_LOCK_ORIENTATION_CALLBACK_H_
