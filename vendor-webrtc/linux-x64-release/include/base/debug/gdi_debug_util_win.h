// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_GDI_DEBUG_UTIL_WIN_H_
#define BASE_DEBUG_GDI_DEBUG_UTIL_WIN_H_

#include <windows.h>

#include "base/base_export.h"

namespace base {
namespace debug {

struct BASE_EXPORT GdiHandleCounts {
  int dcs = 0;
  int regions = 0;
  int bitmaps = 0;
  int palettes = 0;
  int fonts = 0;
  int brushes = 0;
  int pens = 0;
  int unknown = 0;
  int total_tracked = 0;
};

// Crashes the process, using base::debug::Alias to leave valuable debugging
// information in the crash dump. Pass values for |header| and |shared_section|
// in the event of a bitmap allocation failure, to gather information about
// those as well.
BASE_EXPORT void CollectGDIUsageAndDie(BITMAPINFOHEADER* header = nullptr,
                                       HANDLE shared_section = nullptr);

BASE_EXPORT GdiHandleCounts GetGDIHandleCountsInCurrentProcessForTesting();

}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_GDI_DEBUG_UTIL_WIN_H_
