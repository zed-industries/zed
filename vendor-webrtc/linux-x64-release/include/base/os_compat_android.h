// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_OS_COMPAT_ANDROID_H_
#define BASE_OS_COMPAT_ANDROID_H_

#include <fcntl.h>
#include <sys/types.h>
#include <utime.h>

#if __ANDROID_API__ < 26
extern "C" int futimes(int fd, const struct timeval tv[2]);
#endif

#endif  // BASE_OS_COMPAT_ANDROID_H_
