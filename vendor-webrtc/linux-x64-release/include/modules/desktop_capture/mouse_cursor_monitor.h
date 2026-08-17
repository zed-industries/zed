/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_MOUSE_CURSOR_MONITOR_H_
#define MODULES_DESKTOP_CAPTURE_MOUSE_CURSOR_MONITOR_H_

#include <memory>

#include "modules/desktop_capture/desktop_capture_types.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class DesktopCaptureOptions;
class DesktopFrame;
class MouseCursor;

// Captures mouse shape and position.
class MouseCursorMonitor {
 public:
  // Deprecated: CursorState will not be provided.
  enum CursorState {
    // Cursor on top of the window including window decorations.
    INSIDE,

    // Cursor is outside of the window.
    OUTSIDE,
  };

  enum Mode {
    // Capture only shape of the mouse cursor, but not position.
    SHAPE_ONLY,

    // Capture both, mouse cursor shape and position.
    SHAPE_AND_POSITION,
  };

  // Callback interface used to pass current mouse cursor position and shape.
  class Callback {
   public:
    // Called in response to Capture() when the cursor shape has changed. Must
    // take ownership of `cursor`.
    virtual void OnMouseCursor(MouseCursor* cursor) = 0;

    // Called in response to Capture(). `position` indicates cursor position
    // relative to the `window` specified in the constructor.
    // Deprecated: use the following overload instead.
    virtual void OnMouseCursorPosition(CursorState /* state */,
                                       const DesktopVector& /* position */) {}

    // Called in response to Capture(). `position` indicates cursor absolute
    // position on the system in fullscreen coordinate, i.e. the top-left
    // monitor always starts from (0, 0).
    // The coordinates of the position is controlled by OS, but it's always
    // consistent with DesktopFrame.rect().top_left().
    // TODO(zijiehe): Ensure all implementations return the absolute position.
    // TODO(zijiehe): Current this overload works correctly only when capturing
    // mouse cursor against fullscreen.
    virtual void OnMouseCursorPosition(const DesktopVector& /* position */) {}

   protected:
    virtual ~Callback() {}
  };

  virtual ~MouseCursorMonitor() {}

  // Creates a capturer that notifies of mouse cursor events while the cursor is
  // over the specified window.
  //
  // Deprecated: use Create() function.
  static MouseCursorMonitor* CreateForWindow(
      const DesktopCaptureOptions& options,
      WindowId window);

  // Creates a capturer that monitors the mouse cursor shape and position over
  // the specified screen.
  //
  // Deprecated: use Create() function.
  static RTC_EXPORT MouseCursorMonitor* CreateForScreen(
      const DesktopCaptureOptions& options,
      ScreenId screen);

  // Creates a capturer that monitors the mouse cursor shape and position across
  // the entire desktop. The capturer ensures that the top-left monitor starts
  // from (0, 0).
  static RTC_EXPORT std::unique_ptr<MouseCursorMonitor> Create(
      const DesktopCaptureOptions& options);

  // Initializes the monitor with the `callback`, which must remain valid until
  // capturer is destroyed.
  virtual void Init(Callback* callback, Mode mode) = 0;

  // Captures current cursor shape and position (depending on the `mode` passed
  // to Init()). Calls Callback::OnMouseCursor() if cursor shape has
  // changed since the last call (or when Capture() is called for the first
  // time) and then Callback::OnMouseCursorPosition() if mode is set to
  // SHAPE_AND_POSITION.
  virtual void Capture() = 0;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_MOUSE_CURSOR_MONITOR_H_
