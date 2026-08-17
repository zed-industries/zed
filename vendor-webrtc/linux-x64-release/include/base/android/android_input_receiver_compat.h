// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_ANDROID_INPUT_RECEIVER_COMPAT_H_
#define BASE_ANDROID_ANDROID_INPUT_RECEIVER_COMPAT_H_

#include <android/input.h>
#include <jni.h>

#include "base/base_export.h"

extern "C" {
typedef struct AChoreographer AChoreographer;
typedef struct ALooper ALooper;
typedef struct ASurfaceControl ASurfaceControl;
typedef struct AInputReceiverCallbacks AInputReceiverCallbacks;
typedef struct AInputTransferToken AInputTransferToken;
typedef struct AInputReceiver AInputReceiver;
typedef bool (*AInputReceiver_onMotionEvent)(void* context,
                                             AInputEvent* motionEvent);

using pAInputTransferToken_fromJava = AInputTransferToken* (*)(JNIEnv*,
                                                               jobject);
using pAInputTransferToken_toJava = jobject (*)(JNIEnv*,
                                                const AInputTransferToken*);
using pAInputTransferToken_release =
    void (*)(AInputTransferToken* aInputTransferToken);
using pAInputEvent_toJava = jobject (*)(JNIEnv*, const AInputEvent*);
using pAInputReceiverCallbacks_create =
    AInputReceiverCallbacks* (*)(void* context);
using pAInputReceiverCallbacks_release =
    void (*)(AInputReceiverCallbacks* callbacks);
using pAInputReceiverCallbacks_setMotionEventCallback =
    void (*)(AInputReceiverCallbacks*, AInputReceiver_onMotionEvent);
using pAInputReceiver_createBatchedInputReceiver =
    AInputReceiver* (*)(AChoreographer*,
                        const AInputTransferToken*,
                        const ASurfaceControl*,
                        AInputReceiverCallbacks*);
using pAInputReceiver_createUnbatchedInputReceiver =
    AInputReceiver* (*)(ALooper*,
                        const AInputTransferToken*,
                        const ASurfaceControl*,
                        AInputReceiverCallbacks*);
using pAInputReceiver_getInputTransferToken =
    AInputTransferToken* (*)(AInputReceiver*);
using pAInputReceiver_release = void (*)(AInputReceiver*);

}  // extern "C"

namespace base {

// This class provides runtime support for using surface control input receiver
// methods.
// Don't call GetInstance() unless IsSupportAvailable() returns true.
class BASE_EXPORT AndroidInputReceiverCompat {
 public:
  static bool IsSupportAvailable();
  static const AndroidInputReceiverCompat& GetInstance();

  AndroidInputReceiverCompat(const AndroidInputReceiverCompat&) = delete;
  AndroidInputReceiverCompat& operator=(const AndroidInputReceiverCompat&) =
      delete;

  pAInputTransferToken_fromJava AInputTransferToken_fromJavaFn;
  pAInputTransferToken_toJava AInputTransferToken_toJavaFn;
  pAInputTransferToken_release AInputTransferToken_releaseFn;
  pAInputEvent_toJava AInputEvent_toJavaFn;
  pAInputReceiverCallbacks_create AInputReceiverCallbacks_createFn;
  pAInputReceiverCallbacks_release AInputReceiverCallbacks_releaseFn;
  pAInputReceiverCallbacks_setMotionEventCallback
      AInputReceiverCallbacks_setMotionEventCallbackFn;
  pAInputReceiver_createUnbatchedInputReceiver
      AInputReceiver_createUnbatchedInputReceiverFn;
  pAInputReceiver_createBatchedInputReceiver
      AInputReceiver_createBatchedInputReceiverFn;
  pAInputReceiver_getInputTransferToken AInputReceiver_getInputTransferTokenFn;
  pAInputReceiver_release AInputReceiver_releaseFn;

 private:
  AndroidInputReceiverCompat();
};

}  // namespace base

#endif  // BASE_ANDROID_ANDROID_INPUT_RECEIVER_COMPAT_H_
