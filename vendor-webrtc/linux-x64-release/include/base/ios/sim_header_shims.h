// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_IOS_SIM_HEADER_SHIMS_H_
#define BASE_IOS_SIM_HEADER_SHIMS_H_

#include "build/blink_buildflags.h"

#if !BUILDFLAG(USE_BLINK)
#error File can only be included when USE_BLINK is true
#endif

#include <mach/kern_return.h>
#include <mach/message.h>
#include <stdint.h>
#include <sys/param.h>

// This file includes the necessary headers that are not part of the
// iOS public SDK in order to support multiprocess and memory instrumentations
// on iOS.

__BEGIN_DECLS

#define BOOTSTRAP_MAX_NAME_LEN 128
typedef char name_t[BOOTSTRAP_MAX_NAME_LEN];
kern_return_t bootstrap_check_in(mach_port_t bp,
                                 const name_t service_name,
                                 mach_port_t* sp);
kern_return_t bootstrap_look_up(mach_port_t bp,
                                const name_t service_name,
                                mach_port_t* sp);
pid_t audit_token_to_pid(audit_token_t atoken);

const char* bootstrap_strerror(kern_return_t r);
#define BOOTSTRAP_SUCCESS 0
#define BOOTSTRAP_NOT_PRIVILEGED 1100
#define BOOTSTRAP_NAME_IN_USE 1101
#define BOOTSTRAP_UNKNOWN_SERVICE 1102
#define BOOTSTRAP_SERVICE_ACTIVE 1103
#define BOOTSTRAP_BAD_COUNT 1104
#define BOOTSTRAP_NO_MEMORY 1105
#define BOOTSTRAP_NO_CHILDREN 1106

// These values are copied from darwin-xnu/osfmk/mach/shared_region.h.
// https://github.com/apple/darwin-xnu/blob/8f02f2a044b9bb1ad951987ef5bab20ec9486310/osfmk/mach/shared_region.h#L86-L87
#define SHARED_REGION_BASE_ARM64 0x180000000ULL
#define SHARED_REGION_SIZE_ARM64 0x100000000ULL

int proc_pidinfo(int pid,
                 int flavor,
                 uint64_t arg,
                 void* buffer,
                 int buffersize);
int proc_pidpath(int pid, void* buffer, uint32_t buffersize);
int proc_regionfilename(int pid,
                        uint64_t address,
                        void* buffer,
                        uint32_t buffersize);

#define PROC_PIDPATHINFO_MAXSIZE (4 * MAXPATHLEN)

// These values are copied from xnu/xnu-4570.1.46/bsd/sys/proc_info.h.
// https://opensource.apple.com/source/xnu/xnu-4570.1.46/bsd/sys/proc_info.h#L697-L710
struct proc_fdinfo {
  int32_t proc_fd;
  uint32_t proc_fdtype;
};
#define PROC_PIDLISTFDS 1
#define PROC_PIDLISTFD_SIZE (sizeof(struct proc_fdinfo))

__END_DECLS

#endif  // BASE_IOS_SIM_HEADER_SHIMS_H_
