/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_NATIVE_API_PEERCONNECTION_PEER_CONNECTION_FACTORY_H_
#define SDK_ANDROID_NATIVE_API_PEERCONNECTION_PEER_CONNECTION_FACTORY_H_

#include <jni.h>

#include <memory>

#include "api/peer_connection_interface.h"
#include "rtc_base/thread.h"

namespace webrtc {

// Creates java PeerConnectionFactory with specified `pcf`.
jobject NativeToJavaPeerConnectionFactory(
    JNIEnv* jni,
    scoped_refptr<webrtc::PeerConnectionFactoryInterface> pcf,
    std::unique_ptr<SocketFactory> socket_factory,
    std::unique_ptr<Thread> network_thread,
    std::unique_ptr<Thread> worker_thread,
    std::unique_ptr<Thread> signaling_thread);

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_PEERCONNECTION_PEER_CONNECTION_FACTORY_H_
