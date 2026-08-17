/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_MAC_SCK_PICKER_HANDLE_H_
#define MODULES_DESKTOP_CAPTURE_MAC_SCK_PICKER_HANDLE_H_

#include <os/availability.h>
#include <cstddef>
#include <memory>
#include "modules/desktop_capture/desktop_capturer.h"

@class SCContentSharingPicker;
@class SCStream;

namespace webrtc {

// Helper class to manage multiple users of SCContentSharingPicker.
//
// The `active` and `maximumStreamCount` properties are automatically managed on
// `SCContentSharingPicker.sharedPicker`, which is what is returned from
// GetPicker().
//
// When using this class, for stream limits to work, only create one stream per
// handle.
//
// Designed for single thread use.
class API_AVAILABLE(macos(14.0)) SckPickerHandleInterface {
 public:
  virtual ~SckPickerHandleInterface() = default;
  // Effectively identical to `SCContentSharingPicker.sharedPicker`.
  virtual SCContentSharingPicker* GetPicker() const = 0;
  // A SourceId unique to this handle.
  virtual DesktopCapturer::SourceId Source() const = 0;
};

// Returns a newly created picker handle if the stream count limit has not been
// reached, null otherwise.
std::unique_ptr<SckPickerHandleInterface> API_AVAILABLE(macos(14.0))
    CreateSckPickerHandle();

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_MAC_SCK_PICKER_HANDLE_H_
