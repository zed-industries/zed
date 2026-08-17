// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_API_H_
#define IPCZ_SRC_API_H_

#include "ipcz/ipcz.h"

#if defined(IPCZ_SHARED_LIBRARY)
#if defined(WIN32)
#define IPCZ_EXPORT __declspec(dllexport)
#else
#define IPCZ_EXPORT __attribute__((visibility("default")))
#endif
#else
#define IPCZ_EXPORT
#endif

extern "C" {

IPCZ_EXPORT IpczResult IPCZ_API IpczGetAPI(IpczAPI* api);

}  // namespace "C"

#endif  // IPCZ_SRC_API_H_
