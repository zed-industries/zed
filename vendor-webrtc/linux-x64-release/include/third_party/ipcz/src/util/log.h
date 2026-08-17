// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_LOG_H_
#define IPCZ_SRC_UTIL_LOG_H_

#if defined(IPCZ_STANDALONE)
#include "standalone/base/logging.h"  // nogncheck
#else
#include "base/logging.h"  // nogncheck
#endif

#endif  // IPCZ_SRC_UTIL_LOG_H_
