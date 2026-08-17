//===-- Internal implementation header of vfscanf ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_VFSCANF_INTERNAL_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_VFSCANF_INTERNAL_H

#include "src/__support/File/file.h"
#include "src/__support/arg_list.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"
#include "src/stdio/scanf_core/reader.h"
#include "src/stdio/scanf_core/scanf_main.h"

#if defined(LIBC_TARGET_ARCH_IS_GPU)
#include "src/stdio/ferror.h"
#include "src/stdio/getc.h"
#include "src/stdio/ungetc.h"
#endif

#include "hdr/types/FILE.h"
#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {

namespace internal {

#if defined(LIBC_TARGET_ARCH_IS_GPU)
// The GPU build provides FILE access through the host operating system's
// library. So here we simply use the public entrypoints like in the SYSTEM_FILE
// interface. Entrypoints should normally not call others, this is an exception.
// FIXME: We do not acquire any locks here, so this is not thread safe.
LIBC_INLINE void flockfile(::FILE *) { return; }

LIBC_INLINE void funlockfile(::FILE *) { return; }

LIBC_INLINE int ferror_unlocked(::FILE *f) { return LIBC_NAMESPACE::ferror(f); }

LIBC_INLINE int getc(::FILE *f) { return LIBC_NAMESPACE::getc(f); }

LIBC_INLINE void ungetc(int c, ::FILE *f) { LIBC_NAMESPACE::ungetc(c, f); }

#elif !defined(LIBC_COPT_STDIO_USE_SYSTEM_FILE)

LIBC_INLINE void flockfile(FILE *f) {
  reinterpret_cast<LIBC_NAMESPACE::File *>(f)->lock();
}

LIBC_INLINE void funlockfile(FILE *f) {
  reinterpret_cast<LIBC_NAMESPACE::File *>(f)->unlock();
}

LIBC_INLINE int ferror_unlocked(FILE *f) {
  return reinterpret_cast<LIBC_NAMESPACE::File *>(f)->error_unlocked();
}

LIBC_INLINE int getc(FILE *f) {
  unsigned char c;
  auto result =
      reinterpret_cast<LIBC_NAMESPACE::File *>(f)->read_unlocked(&c, 1);
  size_t r = result.value;
  if (result.has_error() || r != 1)
    return '\0';

  return c;
}

LIBC_INLINE void ungetc(int c, FILE *f) {
  reinterpret_cast<LIBC_NAMESPACE::File *>(f)->ungetc_unlocked(c);
}

#else // defined(LIBC_COPT_STDIO_USE_SYSTEM_FILE)

// Since ungetc_unlocked isn't always available, we don't acquire the lock for
// system files.
LIBC_INLINE void flockfile(::FILE *) { return; }

LIBC_INLINE void funlockfile(::FILE *) { return; }

LIBC_INLINE int ferror_unlocked(::FILE *f) { return ::ferror(f); }

LIBC_INLINE int getc(::FILE *f) { return ::getc(f); }

LIBC_INLINE void ungetc(int c, ::FILE *f) { ::ungetc(c, f); }

#endif // LIBC_COPT_STDIO_USE_SYSTEM_FILE

} // namespace internal

namespace scanf_core {

class StreamReader : public Reader<StreamReader> {
  ::FILE *stream;

public:
  LIBC_INLINE StreamReader(::FILE *stream) : stream(stream) {}

  LIBC_INLINE char getc() {
    return static_cast<char>(internal::getc(static_cast<FILE *>(stream)));
  }
  LIBC_INLINE void ungetc(int c) {
    internal::ungetc(c, static_cast<FILE *>(stream));
  }
};

LIBC_INLINE int vfscanf_internal(::FILE *__restrict stream,
                                 const char *__restrict format,
                                 internal::ArgList &args) {
  internal::flockfile(stream);
  scanf_core::StreamReader reader(stream);
  int retval = scanf_core::scanf_main(&reader, format, args);
  if (retval == 0 && internal::ferror_unlocked(stream))
    retval = EOF;
  internal::funlockfile(stream);

  return retval;
}
} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_VFSCANF_INTERNAL_H
