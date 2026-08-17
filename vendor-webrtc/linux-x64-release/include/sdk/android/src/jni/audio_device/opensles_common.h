/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_COMMON_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_COMMON_H_

#include <SLES/OpenSLES.h>
#include <stddef.h>

#include "api/ref_counted_base.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"

namespace webrtc {

namespace jni {

// Returns a string representation given an integer SL_RESULT_XXX code.
// The mapping can be found in <SLES/OpenSLES.h>.
const char* GetSLErrorString(size_t code);

// Configures an SL_DATAFORMAT_PCM structure based on native audio parameters.
SLDataFormat_PCM CreatePCMConfiguration(size_t channels,
                                        int sample_rate,
                                        size_t bits_per_sample);

// Helper class for using SLObjectItf interfaces.
template <typename SLType, typename SLDerefType>
class ScopedSLObject {
 public:
  ScopedSLObject() : obj_(nullptr) {}

  ~ScopedSLObject() { Reset(); }

  SLType* Receive() {
    RTC_DCHECK(!obj_);
    return &obj_;
  }

  SLDerefType operator->() { return *obj_; }

  SLType Get() const { return obj_; }

  void Reset() {
    if (obj_) {
      (*obj_)->Destroy(obj_);
      obj_ = nullptr;
    }
  }

 private:
  SLType obj_;
};

typedef ScopedSLObject<SLObjectItf, const SLObjectItf_*> ScopedSLObjectItf;

// Creates and realizes the main (global) Open SL engine object and returns
// a reference to it. The engine object is only created at the first call
// since OpenSL ES for Android only supports a single engine per application.
// Subsequent calls returns the already created engine.
// Note: This class must be used single threaded and this is enforced by a
// thread checker.
class OpenSLEngineManager
    : public webrtc::RefCountedNonVirtual<OpenSLEngineManager> {
 public:
  OpenSLEngineManager();
  ~OpenSLEngineManager() = default;
  SLObjectItf GetOpenSLEngine();

 private:
  SequenceChecker thread_checker_;
  // This object is the global entry point of the OpenSL ES API.
  // After creating the engine object, the application can obtain this object‘s
  // SLEngineItf interface. This interface contains creation methods for all
  // the other object types in the API. None of these interface are realized
  // by this class. It only provides access to the global engine object.
  ScopedSLObjectItf engine_object_;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_COMMON_H_
