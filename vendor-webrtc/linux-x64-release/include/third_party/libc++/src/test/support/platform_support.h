//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

// Define a bunch of macros that can be used in the tests instead of
//  implementation defined assumptions:
//   - locale names
//   - floating point number string output

#ifndef PLATFORM_SUPPORT_H
#define PLATFORM_SUPPORT_H

#include "test_macros.h"

// locale names
#define LOCALE_en_US_UTF_8     "en_US.UTF-8"
#define LOCALE_fr_FR_UTF_8     "fr_FR.UTF-8"
#ifdef __linux__
#    define LOCALE_fr_CA_ISO8859_1 "fr_CA.ISO-8859-1"
#    define LOCALE_cs_CZ_ISO8859_2 "cs_CZ.ISO-8859-2"
#elif defined(_WIN32)
#    define LOCALE_fr_CA_ISO8859_1 "fr-CA"
#    define LOCALE_cs_CZ_ISO8859_2 "cs-CZ"
#else
#    define LOCALE_fr_CA_ISO8859_1 "fr_CA.ISO8859-1"
#    define LOCALE_cs_CZ_ISO8859_2 "cs_CZ.ISO8859-2"
#endif
#define LOCALE_ja_JP_UTF_8     "ja_JP.UTF-8"
#define LOCALE_ru_RU_UTF_8     "ru_RU.UTF-8"
#define LOCALE_zh_CN_UTF_8     "zh_CN.UTF-8"

#include <stdio.h>
#include <stdlib.h>
#include <string>
#if defined(_WIN32)
#   include <io.h> // _mktemp_s
#   include <fcntl.h> // _O_EXCL, ...
#   include <sys/stat.h> // _S_IREAD, ...
#elif __has_include(<unistd.h>)
#  include <unistd.h> // close
#endif

#if defined(_CS_GNU_LIBC_VERSION)
# include <string.h> // strverscmp
#endif

#if defined(_NEWLIB_VERSION) && defined(__STRICT_ANSI__)
// Newlib provides this, but in the header it's under __STRICT_ANSI__
extern "C" {
  int mkstemp(char*);
}
#endif

inline std::string get_temp_file_name() {
#if defined(_WIN32)
  while (true) {
    char Name[] = "libcxx.XXXXXX";
    if (_mktemp_s(Name, sizeof(Name)) != 0)
      abort();
    int fd = _open(Name, _O_RDWR | _O_CREAT | _O_EXCL, _S_IREAD | _S_IWRITE);
    if (fd != -1) {
      _close(fd);
      return Name;
    }
    if (errno == EEXIST)
      continue;
    abort();
  }
#elif !__has_include(<unistd.h>)
  // Without `unistd.h` we cannot guarantee that the file is unused, however we
  // can simply generate a good guess in the temporary folder and create it.
  constexpr char chars[] = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
  char Name[]            = "/tmp/libcxx.XXXXXX";
  for (std::size_t i = 0; i < sizeof(Name); ++i)
    if (Name[i] == 'X')
      Name[i] = chars[rand() % strlen(chars)];
  FILE* file = fopen(Name, "w");
  if (!file)
    abort();
  if (fclose(file) == EOF)
    abort();
  return std::string(Name);
#else
  std::string Name = "libcxx.XXXXXX";
  int FD           = mkstemp(&Name[0]);
  if (FD == -1) {
    perror("mkstemp");
    abort();
  }
  close(FD);
  return Name;
#endif
}

#if defined(_CS_GNU_LIBC_VERSION)
inline bool glibc_version_less_than(char const* version) {
  std::string test_version = std::string("glibc ") + version;

  std::size_t n = confstr(_CS_GNU_LIBC_VERSION, nullptr, (size_t)0);
  char *current_version = new char[n];
  confstr(_CS_GNU_LIBC_VERSION, current_version, n);

  bool result = strverscmp(current_version, test_version.c_str()) < 0;
  delete[] current_version;
  return result;
}
#endif // _CS_GNU_LIBC_VERSION

#endif // PLATFORM_SUPPORT_H
