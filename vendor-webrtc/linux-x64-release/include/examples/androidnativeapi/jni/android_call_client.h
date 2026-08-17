/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef EXAMPLES_ANDROIDNATIVEAPI_JNI_ANDROID_CALL_CLIENT_H_
#define EXAMPLES_ANDROIDNATIVEAPI_JNI_ANDROID_CALL_CLIENT_H_

#include <jni.h>

#include <memory>
#include <string>

#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "rtc_base/synchronization/mutex.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"
#include "sdk/android/native_api/video/video_source.h"

namespace webrtc_examples {

class AndroidCallClient {
 public:
  AndroidCallClient();
  ~AndroidCallClient();

  void Call(JNIEnv* env,
            const webrtc::JavaRef<jobject>& local_sink,
            const webrtc::JavaRef<jobject>& remote_sink);
  void Hangup(JNIEnv* env);
  // A helper method for Java code to delete this object. Calls delete this.
  void Delete(JNIEnv* env);

  webrtc::ScopedJavaLocalRef<jobject> GetJavaVideoCapturerObserver(JNIEnv* env);

 private:
  class PCObserver;

  void CreatePeerConnectionFactory() RTC_RUN_ON(thread_checker_);
  void CreatePeerConnection() RTC_RUN_ON(thread_checker_);
  void Connect() RTC_RUN_ON(thread_checker_);

  webrtc::SequenceChecker thread_checker_;

  bool call_started_ RTC_GUARDED_BY(thread_checker_);

  const std::unique_ptr<PCObserver> pc_observer_;

  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> pcf_
      RTC_GUARDED_BY(thread_checker_);
  std::unique_ptr<webrtc::Thread> network_thread_
      RTC_GUARDED_BY(thread_checker_);
  std::unique_ptr<webrtc::Thread> worker_thread_
      RTC_GUARDED_BY(thread_checker_);
  std::unique_ptr<webrtc::Thread> signaling_thread_
      RTC_GUARDED_BY(thread_checker_);

  std::unique_ptr<webrtc::VideoSinkInterface<webrtc::VideoFrame>> local_sink_
      RTC_GUARDED_BY(thread_checker_);
  std::unique_ptr<webrtc::VideoSinkInterface<webrtc::VideoFrame>> remote_sink_
      RTC_GUARDED_BY(thread_checker_);
  webrtc::scoped_refptr<webrtc::JavaVideoTrackSourceInterface> video_source_
      RTC_GUARDED_BY(thread_checker_);

  webrtc::Mutex pc_mutex_;
  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc_
      RTC_GUARDED_BY(pc_mutex_);
};

}  // namespace webrtc_examples

#endif  // EXAMPLES_ANDROIDNATIVEAPI_JNI_ANDROID_CALL_CLIENT_H_
