/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_LINUX_X11_WINDOW_LIST_UTILS_H_
#define MODULES_DESKTOP_CAPTURE_LINUX_X11_WINDOW_LIST_UTILS_H_

#include <X11/X.h>
#include <X11/Xlib.h>
#include <stdint.h>

#include "api/function_view.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/linux/x11/x_atom_cache.h"

namespace webrtc {

// Synchronously iterates all on-screen windows in `cache`.display() in
// decreasing z-order and sends them one-by-one to `on_window` function before
// GetWindowList() returns. If `on_window` returns false, this function ignores
// other windows and returns immediately. GetWindowList() returns false if
// native APIs failed. If multiple screens are attached to the `display`, this
// function returns false only when native APIs failed on all screens. Menus,
// panels and minimized windows will be ignored.
bool GetWindowList(XAtomCache* cache, FunctionView<bool(::Window)> on_window);

// Returns WM_STATE property of the `window`. This function returns
// WithdrawnState if the `window` is missing.
int32_t GetWindowState(XAtomCache* cache, ::Window window);

// Returns the rectangle of the `window` in the coordinates of `display`. This
// function returns false if native APIs failed. If `attributes` is provided, it
// will be filled with the attributes of `window`. The `rect` is in system
// coordinate, i.e. the primary monitor always starts from (0, 0).
bool GetWindowRect(::Display* display,
                   ::Window window,
                   DesktopRect* rect,
                   XWindowAttributes* attributes = nullptr);

// Creates a DesktopRect from `attributes`.
template <typename T>
DesktopRect DesktopRectFromXAttributes(const T& attributes) {
  return DesktopRect::MakeXYWH(attributes.x, attributes.y, attributes.width,
                               attributes.height);
}

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_LINUX_X11_WINDOW_LIST_UTILS_H_
