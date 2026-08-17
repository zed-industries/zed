/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DELEGATED_SOURCE_LIST_CONTROLLER_H_
#define MODULES_DESKTOP_CAPTURE_DELEGATED_SOURCE_LIST_CONTROLLER_H_

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// A controller to be implemented and returned by
// GetDelegatedSourceListController in capturers that require showing their own
// source list and managing user selection there. Apart from ensuring the
// visibility of the source list, these capturers should largely be interacted
// with the same as a normal capturer, though there may be some caveats for
// some DesktopCapturer methods. See GetDelegatedSourceListController for more
// information.
class RTC_EXPORT DelegatedSourceListController {
 public:
  // Notifications that can be used to help drive any UI that the consumer may
  // want to show around this source list (e.g. if an consumer shows their own
  // UI in addition to the delegated source list).
  class Observer {
   public:
    // Called after the user has made a selection in the delegated source list.
    // Note that the consumer will still need to get the source out of the
    // capturer by calling GetSourceList.
    virtual void OnSelection() = 0;

    // Called when there is any user action that cancels the source selection.
    virtual void OnCancelled() = 0;

    // Called when there is a system error that cancels the source selection.
    virtual void OnError() = 0;

   protected:
    virtual ~Observer() {}
  };

  // Observer must remain valid until the owning DesktopCapturer is destroyed.
  // Only one Observer is allowed at a time, and may be cleared by passing
  // nullptr.
  virtual void Observe(Observer* observer) = 0;

  // Used to prompt the capturer to show the delegated source list. If the
  // source list is already visible, this will be a no-op. Must be called after
  // starting the DesktopCapturer.
  //
  // Note that any selection from a previous invocation of the source list may
  // be cleared when this method is called.
  virtual void EnsureVisible() = 0;

  // Used to prompt the capturer to hide the delegated source list. If the
  // source list is already hidden, this will be a no-op.
  virtual void EnsureHidden() = 0;

 protected:
  virtual ~DelegatedSourceListController() {}
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DELEGATED_SOURCE_LIST_CONTROLLER_H_
