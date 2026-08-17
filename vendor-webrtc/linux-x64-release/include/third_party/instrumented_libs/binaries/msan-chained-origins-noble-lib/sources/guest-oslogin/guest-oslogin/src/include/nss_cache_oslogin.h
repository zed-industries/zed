// Copyright 2018 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <errno.h>
#include <grp.h>
#include <nss.h>
#include <pwd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <syslog.h>
#include <sys/param.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#ifndef NSS_CACHE_OSLOGIN_H
#define NSS_CACHE_OSLOGIN_H

#ifdef DEBUG
#undef DEBUG
#define DEBUG(fmt, ...)                                                        \
  do {                                                                         \
      openlog("nss_cache_oslogin", LOG_PID|LOG_PERROR, LOG_DAEMON);            \
      syslog(LOG_ERR, fmt, ##__VA_ARGS__);                                     \
      closelog();                                                              \
  } while (0)
#else
#define DEBUG(fmt, ...)                                                        \
  do {                                                                         \
  } while (0)
#endif /* DEBUG */

// why isn't this in compat.h ?
#define NSS_CACHE_OSLOGIN_PATH_LENGTH 255

#endif /* NSS_CACHE_OSLOGIN_H */
