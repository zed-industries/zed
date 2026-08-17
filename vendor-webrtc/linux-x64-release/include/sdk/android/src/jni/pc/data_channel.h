/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_DATA_CHANNEL_H_
#define SDK_ANDROID_SRC_JNI_PC_DATA_CHANNEL_H_

#include "api/data_channel_interface.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

DataChannelInit JavaToNativeDataChannelInit(JNIEnv* env,
                                            const JavaRef<jobject>& j_init);

ScopedJavaLocalRef<jobject> WrapNativeDataChannel(
    JNIEnv* env,
    scoped_refptr<DataChannelInterface> channel);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_DATA_CHANNEL_H_
