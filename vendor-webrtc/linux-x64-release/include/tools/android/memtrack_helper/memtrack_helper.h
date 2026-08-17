/*
 * Copyright 2015 The Chromium Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#ifndef TOOLS_ANDROID_MEMTRACK_HELPER_H_
#define TOOLS_ANDROID_MEMTRACK_HELPER_H_

#include <android/log.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>

static const char* const kLogTag = "memtrack_helper";

static inline void exit_with_failure(const char* reason) {
  perror(reason);
  __android_log_write(ANDROID_LOG_ERROR, kLogTag, reason);
  exit(EXIT_FAILURE);
}

static inline void init_memtrack_server_addr(struct sockaddr_un* addr) {
  const char* const kAbstractSocketName = "chrome_tracing_memtrack_helper";
  memset(addr, 0, sizeof(*addr));
  addr->sun_family = AF_UNIX;
  strncpy(&addr->sun_path[1], kAbstractSocketName, sizeof(addr->sun_path) - 2);
}

#endif  // TOOLS_ANDROID_MEMTRACK_HELPER_H_
