/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_HELPER_H_
#define MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_HELPER_H_

#include <memory>

#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/desktop_region.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// ScreenCapturerHelper is intended to be used by an implementation of the
// ScreenCapturer interface. It maintains a thread-safe invalid region, and
// the size of the most recently captured screen, on behalf of the
// ScreenCapturer that owns it.
class ScreenCapturerHelper {
 public:
  ScreenCapturerHelper() = default;
  ~ScreenCapturerHelper() = default;

  ScreenCapturerHelper(const ScreenCapturerHelper&) = delete;
  ScreenCapturerHelper& operator=(const ScreenCapturerHelper&) = delete;

  // Clear out the invalid region.
  void ClearInvalidRegion();

  // Invalidate the specified region.
  void InvalidateRegion(const DesktopRegion& invalid_region);

  // Invalidate the entire screen, of a given size.
  void InvalidateScreen(const DesktopSize& size);

  // Copies current invalid region to `invalid_region` clears invalid region
  // storage for the next frame.
  void TakeInvalidRegion(DesktopRegion* invalid_region);

  // Access the size of the most recently captured screen.
  const DesktopSize& size_most_recent() const;
  void set_size_most_recent(const DesktopSize& size);

  // Lossy compression can result in color values leaking between pixels in one
  // block. If part of a block changes, then unchanged parts of that block can
  // be changed in the compressed output. So we need to re-render an entire
  // block whenever part of the block changes.
  //
  // If `log_grid_size` is >= 1, then this function makes TakeInvalidRegion()
  // produce an invalid region expanded so that its vertices lie on a grid of
  // size 2 ^ `log_grid_size`. The expanded region is then clipped to the size
  // of the most recently captured screen, as previously set by
  // set_size_most_recent().
  // If `log_grid_size` is <= 0, then the invalid region is not expanded.
  void SetLogGridSize(int log_grid_size);

  // Expands a region so that its vertices all lie on a grid.
  // The grid size must be >= 2, so `log_grid_size` must be >= 1.
  static void ExpandToGrid(const DesktopRegion& region,
                           int log_grid_size,
                           DesktopRegion* result);

 private:
  // A region that has been manually invalidated (through InvalidateRegion).
  // These will be returned as dirty_region in the capture data during the next
  // capture.
  DesktopRegion invalid_region_ RTC_GUARDED_BY(invalid_region_mutex_);

  // A lock protecting `invalid_region_` across threads.
  Mutex invalid_region_mutex_;

  // The size of the most recently captured screen.
  DesktopSize size_most_recent_;

  // The log (base 2) of the size of the grid to which the invalid region is
  // expanded.
  // If the value is <= 0, then the invalid region is not expanded to a grid.
  int log_grid_size_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_HELPER_H_
