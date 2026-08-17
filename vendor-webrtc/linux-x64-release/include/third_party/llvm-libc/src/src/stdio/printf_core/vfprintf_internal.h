//===-- Internal implementation header of vfprintf --------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_PRINTF_CORE_VFPRINTF_INTERNAL_H
#define LLVM_LIBC_SRC_STDIO_PRINTF_CORE_VFPRINTF_INTERNAL_H

#include "src/__support/File/file.h"
#include "src/__support/arg_list.h"
#include "src/__support/macros/attributes.h" // For LIBC_INLINE
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/stdio/printf_core/printf_main.h"
#include "src/stdio/printf_core/writer.h"

#include "hdr/types/FILE.h"

namespace LIBC_NAMESPACE_DECL {

namespace internal {
#ifndef LIBC_COPT_STDIO_USE_SYSTEM_FILE
LIBC_INLINE int ferror_unlocked(FILE *f) {
  return reinterpret_cast<LIBC_NAMESPACE::File *>(f)->error_unlocked();
}

LIBC_INLINE void flockfile(FILE *f) {
  reinterpret_cast<LIBC_NAMESPACE::File *>(f)->lock();
}

LIBC_INLINE void funlockfile(FILE *f) {
  reinterpret_cast<LIBC_NAMESPACE::File *>(f)->unlock();
}

LIBC_INLINE size_t fwrite_unlocked(const void *ptr, size_t size, size_t nmemb,
                                   FILE *f) {
  return reinterpret_cast<LIBC_NAMESPACE::File *>(f)->write_unlocked(
      ptr, size * nmemb);
}
#else  // defined(LIBC_COPT_STDIO_USE_SYSTEM_FILE)
LIBC_INLINE int ferror_unlocked(::FILE *f) { return ::ferror_unlocked(f); }

LIBC_INLINE void flockfile(::FILE *f) { ::flockfile(f); }

LIBC_INLINE void funlockfile(::FILE *f) { ::funlockfile(f); }

LIBC_INLINE size_t fwrite_unlocked(const void *ptr, size_t size, size_t nmemb,
                                   ::FILE *f) {
  return ::fwrite_unlocked(ptr, size, nmemb, f);
}
#endif // LIBC_COPT_STDIO_USE_SYSTEM_FILE
} // namespace internal

namespace printf_core {

LIBC_INLINE int file_write_hook(cpp::string_view new_str, void *fp) {
  ::FILE *target_file = reinterpret_cast<::FILE *>(fp);
  // Write new_str to the target file. The logic preventing a zero-length write
  // is in the writer, so we don't check here.
  size_t written = internal::fwrite_unlocked(new_str.data(), sizeof(char),
                                             new_str.size(), target_file);
  if (written != new_str.size() || internal::ferror_unlocked(target_file))
    return FILE_WRITE_ERROR;
  return WRITE_OK;
}

LIBC_INLINE int vfprintf_internal(::FILE *__restrict stream,
                                  const char *__restrict format,
                                  internal::ArgList &args) {
  constexpr size_t BUFF_SIZE = 1024;
  char buffer[BUFF_SIZE];
  printf_core::WriteBuffer<Mode<WriteMode::FLUSH_TO_STREAM>::value> wb(
      buffer, BUFF_SIZE, &file_write_hook, reinterpret_cast<void *>(stream));
  Writer writer(wb);
  internal::flockfile(stream);
  int retval = printf_main(&writer, format, args);
  int flushval = wb.overflow_write("");
  if (flushval != WRITE_OK)
    retval = flushval;
  internal::funlockfile(stream);
  return retval;
}

} // namespace printf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_PRINTF_CORE_VFPRINTF_INTERNAL_H
