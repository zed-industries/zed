/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYNCHRONIZATION_MUTEX_PTHREAD_H_
#define RTC_BASE_SYNCHRONIZATION_MUTEX_PTHREAD_H_

#if defined(WEBRTC_POSIX)

#include <pthread.h>
#if defined(WEBRTC_MAC)
#include <pthread_spis.h>
#endif

#include "absl/base/attributes.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class RTC_LOCKABLE MutexImpl final {
 public:
  MutexImpl() {
    pthread_mutexattr_t mutex_attribute;
    pthread_mutexattr_init(&mutex_attribute);
#if defined(WEBRTC_MAC)
    pthread_mutexattr_setpolicy_np(&mutex_attribute,
                                   _PTHREAD_MUTEX_POLICY_FIRSTFIT);
#endif
    pthread_mutex_init(&mutex_, &mutex_attribute);
    pthread_mutexattr_destroy(&mutex_attribute);
  }
  MutexImpl(const MutexImpl&) = delete;
  MutexImpl& operator=(const MutexImpl&) = delete;
  ~MutexImpl() { pthread_mutex_destroy(&mutex_); }

  void Lock() RTC_EXCLUSIVE_LOCK_FUNCTION() {
    pthread_mutex_lock(&mutex_);
    owner_.SetOwner();
  }
  ABSL_MUST_USE_RESULT bool TryLock() RTC_EXCLUSIVE_TRYLOCK_FUNCTION(true) {
    if (pthread_mutex_trylock(&mutex_) != 0) {
      return false;
    }
    owner_.SetOwner();
    return true;
  }
  void AssertHeld() const RTC_ASSERT_EXCLUSIVE_LOCK() { owner_.AssertOwned(); }
  void Unlock() RTC_UNLOCK_FUNCTION() {
    owner_.ClearOwner();
    pthread_mutex_unlock(&mutex_);
  }

 private:
  class OwnerRecord {
   public:
#if !RTC_DCHECK_IS_ON
    void SetOwner() {}
    void ClearOwner() {}
    void AssertOwned() const {}
#else
    void SetOwner() {
      latest_owner_ = pthread_self();
      is_owned_ = true;
    }
    void ClearOwner() { is_owned_ = false; }
    void AssertOwned() const {
      RTC_CHECK(is_owned_);
      RTC_CHECK(pthread_equal(latest_owner_, pthread_self()));
    }

   private:
    // Use two separate primitive types, rather than std::optional, since the
    // data race described below might invalidate std::optional invariants.
    bool is_owned_ = false;
    pthread_t latest_owner_ = pthread_self();
#endif
  };

  pthread_mutex_t mutex_;
  // This record is modified only with the mutex held, and hence, calls to
  // AssertHeld where mutex is held are race-free and will always succeed.
  //
  // The failure case is more subtle: If AssertHeld is called from some thread
  // not holding the mutex, and RTC_DCHECK_IS_ON==1, we have a data race. It is
  // highly likely that the calling thread will see `is_owned_` false or
  // `latest_owner_` different from itself, and crash. But it may fail to crash,
  // and invoke some other undefined behavior (still, this race can happen only
  // when RTC_DCHECK_IS_ON==1).
  RTC_NO_UNIQUE_ADDRESS OwnerRecord owner_;
};

}  // namespace webrtc
#endif  // #if defined(WEBRTC_POSIX)
#endif  // RTC_BASE_SYNCHRONIZATION_MUTEX_PTHREAD_H_
