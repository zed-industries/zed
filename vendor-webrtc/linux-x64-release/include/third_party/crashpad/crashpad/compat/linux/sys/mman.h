// Copyright 2019 The Crashpad Authors
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

#ifndef CRASHPAD_COMPAT_LINUX_SYS_MMAN_H_
#define CRASHPAD_COMPAT_LINUX_SYS_MMAN_H_

#include_next <sys/mman.h>

#include <features.h>

// There's no memfd_create() wrapper before glibc 2.27.
// This can't select for glibc < 2.27 because linux-chromeos-rel bots build this
// code using a sysroot which has glibc 2.27, but then run it on Ubuntu 16.04,
// which doesn't.
#if defined(__GLIBC__)

#ifdef __cplusplus
extern "C" {
#endif

int memfd_create(const char* name, unsigned int flags) __THROW;

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // __GLIBC__

#endif  // CRASHPAD_COMPAT_LINUX_SYS_MMAN_H_
