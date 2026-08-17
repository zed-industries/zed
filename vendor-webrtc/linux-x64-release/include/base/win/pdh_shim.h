// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_PDH_SHIM_H_
#define BASE_WIN_PDH_SHIM_H_

// Any Chromium headers which want to `#include <pdh.h>` should instead #include
// this header.

#include <windows.h>

// To prevent conflicts arising from redefining `HLOG`, `<pdh.h>` should only
// ever be #included after `<lm.h>`; see
// https://devblogs.microsoft.com/oldnewthing/20150724-00/?p=90831. If these are
// only ever #included directly, then Google style (alphabetical order) would
// normally prevent problems. If they are conditionally or indirectly included,
// however, the resulting problem is hard to diagnose. To avoid this, any
// Chromium headers which want to #include <pdh.h> should instead include this
// header.
#include <lm.h>
#include <pdh.h>

#endif  // BASE_WIN_PDH_SHIM_H_
