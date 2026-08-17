// Copyright 2023 Google Inc. All Rights Reserved.
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

#ifndef _OSLOGIN_SSHCA_H_
#define _OSLOGIN_SSHCA_H_ 1

#include <compat.h>
#include <ctype.h>
#include <security/pam_modules.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syslog.h>
#include <sys/types.h>

#define SKIP_BYTES(b, l, s)                     \
  {                                             \
    b = b + s;                                  \
    l = l - s;                                  \
  }                                             \

#define SKIP_UINT64(b, l)                       \
  SKIP_BYTES(b, l, 8)                           \

#define SKIP_UINT32(b, l)                       \
  SKIP_BYTES(b, l, 4)                           \

#define PEEK_U32(p)                                     \
  (((u_int32_t)(((const u_char *)(p))[0]) << 24) |      \
   ((u_int32_t)(((const u_char *)(p))[1]) << 16) |      \
   ((u_int32_t)(((const u_char *)(p))[2]) << 8) |       \
   (u_int32_t)(((const u_char *)(p))[3]))

namespace oslogin_sshca {
// The public interface - given a blob with a list of certificates we parse each of
// them until we find the first fingerprint.
int FingerPrintFromBlob(const char *blob, char **fingerprint);
}

#endif
