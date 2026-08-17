/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURE_UTILS_H_
#define MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURE_UTILS_H_

#include <shlobj.h>
#include <windows.h>
#include <wrl/client.h>

#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

// Outputs the window rect. The returned DesktopRect is in system coordinates,
// i.e. the primary monitor on the system always starts from (0, 0). This
// function returns false if native APIs fail.
bool GetWindowRect(HWND window, DesktopRect* result);

// Outputs the window rect, with the left/right/bottom frame border cropped if
// the window is maximized or has a transparent resize border.
// `avoid_cropping_border` may be set to true to avoid cropping the visible
// border when cropping any resize border.
// `cropped_rect` is the cropped rect relative to the
// desktop. `original_rect` is the original rect returned from GetWindowRect.
// Returns true if all API calls succeeded. The returned DesktopRect is in
// system coordinates, i.e. the primary monitor on the system always starts from
// (0, 0). `original_rect` can be nullptr.
//
// TODO(zijiehe): Move this function to CroppingWindowCapturerWin after it has
// been removed from MouseCursorMonitorWin.
// This function should only be used by CroppingWindowCapturerWin. Instead a
// DesktopRect CropWindowRect(const DesktopRect& rect)
// should be added as a utility function to help CroppingWindowCapturerWin and
// WindowCapturerWinGdi to crop out the borders or shadow according to their
// scenarios. But this function is too generic and easy to be misused.
bool GetCroppedWindowRect(HWND window,
                          bool avoid_cropping_border,
                          DesktopRect* cropped_rect,
                          DesktopRect* original_rect);

// Retrieves the rectangle of the content area of `window`. Usually it contains
// title bar and window client area, but borders or shadow are excluded. The
// returned DesktopRect is in system coordinates, i.e. the primary monitor on
// the system always starts from (0, 0). This function returns false if native
// APIs fail.
bool GetWindowContentRect(HWND window, DesktopRect* result);

// Returns the region type of the `window` and fill `rect` with the region of
// `window` if region type is SIMPLEREGION.
int GetWindowRegionTypeWithBoundary(HWND window, DesktopRect* result);

// Retrieves the size of the `hdc`. This function returns false if native APIs
// fail.
bool GetDcSize(HDC hdc, DesktopSize* size);

// Retrieves whether the `window` is maximized and stores in `result`. This
// function returns false if native APIs fail.
bool IsWindowMaximized(HWND window, bool* result);

// Checks that the HWND is for a valid window, that window's visibility state is
// visible, and that it is not minimized.
bool IsWindowValidAndVisible(HWND window);

// Checks if a window responds to a message within 50ms.
bool IsWindowResponding(HWND window);

enum GetWindowListFlags {
  kNone = 0x00,
  kIgnoreUntitled = 1 << 0,
  kIgnoreUnresponsive = 1 << 1,
  kIgnoreCurrentProcessWindows = 1 << 2,
};

// Retrieves the list of top-level windows on the screen.
// Some windows will be ignored:
// - Those that are invisible or minimized.
// - Program Manager & Start menu.
// - [with kIgnoreUntitled] windows with no title.
// - [with kIgnoreUnresponsive] windows that are unresponsive.
// - [with kIgnoreCurrentProcessWindows] windows owned by the current process.
// - Any windows with extended styles that match `ex_style_filters`.
// Returns false if native APIs failed.
bool GetWindowList(int flags,
                   DesktopCapturer::SourceList* windows,
                   LONG ex_style_filters = 0);

typedef HRESULT(WINAPI* DwmIsCompositionEnabledFunc)(BOOL* enabled);
typedef HRESULT(WINAPI* DwmGetWindowAttributeFunc)(HWND hwnd,
                                                   DWORD flag,
                                                   PVOID result_ptr,
                                                   DWORD result_size);
class WindowCaptureHelperWin {
 public:
  WindowCaptureHelperWin();
  ~WindowCaptureHelperWin();

  WindowCaptureHelperWin(const WindowCaptureHelperWin&) = delete;
  WindowCaptureHelperWin& operator=(const WindowCaptureHelperWin&) = delete;

  bool IsAeroEnabled();
  bool IsWindowChromeNotification(HWND hwnd);
  bool AreWindowsOverlapping(HWND hwnd,
                             HWND selected_hwnd,
                             const DesktopRect& selected_window_rect);
  bool IsWindowOnCurrentDesktop(HWND hwnd);
  bool IsWindowVisibleOnCurrentDesktop(HWND hwnd);
  bool IsWindowCloaked(HWND hwnd);

  // The optional `ex_style_filters` parameter allows callers to provide
  // extended window styles (e.g. WS_EX_TOOLWINDOW) and prevent windows that
  // match from being included in `results`.
  bool EnumerateCapturableWindows(DesktopCapturer::SourceList* results,
                                  bool enumerate_current_process_windows,
                                  LONG ex_style_filters = 0);

 private:
  HMODULE dwmapi_library_ = nullptr;
  DwmIsCompositionEnabledFunc func_ = nullptr;
  DwmGetWindowAttributeFunc dwm_get_window_attribute_func_ = nullptr;

  // Only used on Win10+.
  Microsoft::WRL::ComPtr<IVirtualDesktopManager> virtual_desktop_manager_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURE_UTILS_H_
