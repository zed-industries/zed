// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_HELPERMETHODS_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_HELPERMETHODS_H_

// Make sure we don't get min/max macros
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>

#include <wdf.h>

#include <iddcx.h>

namespace display::test {
void FillSignalInfo(DISPLAYCONFIG_VIDEO_SIGNAL_INFO&,
                    DWORD,
                    DWORD,
                    DWORD,
                    bool);
IDDCX_MONITOR_MODE CreateIddCxMonitorMode(DWORD,
                                          DWORD,
                                          DWORD,
                                          IDDCX_MONITOR_MODE_ORIGIN);
IDDCX_TARGET_MODE CreateIddCxTargetMode(DWORD, DWORD, DWORD);
}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_HELPERMETHODS_H_
