//===-- Classes to capture properites of linux applications -----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_CONFIG_LINUX_APP_H
#define LLVM_LIBC_CONFIG_LINUX_APP_H

#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {

// Data structure to capture properties of the linux/ELF TLS image.
struct TLSImage {
  // The load address of the TLS.
  uintptr_t address;

  // The byte size of the TLS image consisting of both initialized and
  // uninitialized memory. In ELF executables, it is size of .tdata + size of
  // .tbss. Put in another way, it is the memsz field of the PT_TLS header.
  uintptr_t size;

  // The byte size of initialized memory in the TLS image. In ELF exectubles,
  // this is the size of .tdata. Put in another way, it is the filesz of the
  // PT_TLS header.
  uintptr_t init_size;

  // The alignment of the TLS layout. It assumed that the alignment
  // value is a power of 2.
  uintptr_t align;
};

// Linux manpage on `proc(5)` says that the aux vector is an array of
// unsigned long pairs.
// (see: https://man7.org/linux/man-pages/man5/proc.5.html)
using AuxEntryType = unsigned long;
// Using the naming convention from `proc(5)`.
// TODO: Would be nice to use the aux entry structure from elf.h when available.
struct AuxEntry {
  AuxEntryType id;
  AuxEntryType value;
};

struct Args {
  uintptr_t argc;

  // A flexible length array would be more suitable here, but C++ doesn't have
  // flexible arrays: P1039 proposes to fix this. So, for now we just fake it.
  // Even if argc is zero, "argv[argc] shall be a null pointer"
  // (ISO C 5.1.2.2.1) so one is fine. Also, length of 1 is not really wrong as
  // |argc| is guaranteed to be atleast 1, and there is an 8-byte null entry at
  // the end of the argv array.
  uintptr_t argv[1];
};

// Data structure which captures properties of a linux application.
struct AppProperties {
  // Page size used for the application.
  uintptr_t page_size;

  Args *args;

  // The properties of an application's TLS image.
  TLSImage tls;

  // Environment data.
  uintptr_t *env_ptr;

  // Auxiliary vector data.
  AuxEntry *auxv_ptr;
};

[[gnu::weak]] extern AppProperties app;

// The descriptor of a thread's TLS area.
struct TLSDescriptor {
  // The size of the TLS area.
  uintptr_t size = 0;

  // The address of the TLS area. This address can be passed to cleanup
  // functions like munmap.
  uintptr_t addr = 0;

  // The value the thread pointer register should be initialized to.
  // Note that, dependending the target architecture ABI, it can be the
  // same as |addr| or something else.
  uintptr_t tp = 0;

  constexpr TLSDescriptor() = default;
};

// Create and initialize the TLS area for the current thread. Should not
// be called before app.tls has been initialized.
void init_tls(TLSDescriptor &tls);

// Cleanup the TLS area as described in |tls_descriptor|.
void cleanup_tls(uintptr_t tls_addr, uintptr_t tls_size);

// Set the thread pointer for the current thread.
bool set_thread_ptr(uintptr_t val);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_CONFIG_LINUX_APP_H
