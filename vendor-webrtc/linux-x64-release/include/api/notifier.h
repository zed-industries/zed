/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NOTIFIER_H_
#define API_NOTIFIER_H_

#include <list>

#include "api/media_stream_interface.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Implements a template version of a notifier.
// TODO(deadbeef): This is an implementation detail; move out of api/.
template <class T>
class Notifier : public T {
 public:
  Notifier() = default;

  virtual void RegisterObserver(ObserverInterface* observer) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    RTC_DCHECK(observer != nullptr);
    observers_.push_back(observer);
  }

  virtual void UnregisterObserver(ObserverInterface* observer) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    for (std::list<ObserverInterface*>::iterator it = observers_.begin();
         it != observers_.end(); it++) {
      if (*it == observer) {
        observers_.erase(it);
        break;
      }
    }
  }

  void FireOnChanged() {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    // Copy the list of observers to avoid a crash if the observer object
    // unregisters as a result of the OnChanged() call. If the same list is used
    // UnregisterObserver will affect the list make the iterator invalid.
    std::list<ObserverInterface*> observers = observers_;
    for (std::list<ObserverInterface*>::iterator it = observers.begin();
         it != observers.end(); ++it) {
      (*it)->OnChanged();
    }
  }

 protected:
  std::list<ObserverInterface*> observers_ RTC_GUARDED_BY(sequence_checker_);

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_{
      SequenceChecker::kDetached};
};

}  // namespace webrtc

#endif  // API_NOTIFIER_H_
