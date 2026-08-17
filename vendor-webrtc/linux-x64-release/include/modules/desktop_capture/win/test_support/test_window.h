/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_DESKTOP_CAPTURE_WIN_TEST_SUPPORT_TEST_WINDOW_H_
#define MODULES_DESKTOP_CAPTURE_WIN_TEST_SUPPORT_TEST_WINDOW_H_

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>

namespace webrtc {

typedef unsigned char uint8_t;

// Define an arbitrary color for the test window with unique R, G, and B values
// so consumers can verify captured content in tests.
const uint8_t kTestWindowRValue = 191;
const uint8_t kTestWindowGValue = 99;
const uint8_t kTestWindowBValue = 12;

struct WindowInfo {
  HWND hwnd;
  HINSTANCE window_instance;
  ATOM window_class;
};

WindowInfo CreateTestWindow(const WCHAR* window_title,
                            int height = 0,
                            int width = 0,
                            LONG extended_styles = 0);

void ResizeTestWindow(HWND hwnd, int width, int height);

void MoveTestWindow(HWND hwnd, int x, int y);

void MinimizeTestWindow(HWND hwnd);

void UnminimizeTestWindow(HWND hwnd);

void DestroyTestWindow(WindowInfo info);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_TEST_SUPPORT_TEST_WINDOW_H_
