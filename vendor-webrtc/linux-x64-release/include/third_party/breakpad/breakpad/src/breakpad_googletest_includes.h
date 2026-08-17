// Copyright 2009 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef BREAKPAD_GOOGLETEST_INCLUDES_H__
#define BREAKPAD_GOOGLETEST_INCLUDES_H__

#include "gtest/gtest.h"
#include "gmock/gmock.h"

// If AddressSanitizer is used, NULL pointer dereferences generate SIGILL
// (illegal instruction) instead of SIGSEGV (segmentation fault).  Also,
// the number of memory regions differs, so there is no point in running
// this test if AddressSanitizer is used.
//
// Ideally we'd use this attribute to disable ASAN on a per-func basis,
// but this doesn't seem to actually work, and it's changed names over
// time.  So just stick with disabling the actual tests.
// http://crbug.com/304575
//#define NO_ASAN __attribute__((no_sanitize_address))
#if defined(__clang__) && defined(__has_feature)
// Have to keep this check sep from above as newer gcc will barf on it.
# if __has_feature(address_sanitizer)
#  define ADDRESS_SANITIZER
# endif
#elif defined(__GNUC__) && defined(__SANITIZE_ADDRESS__)
# define ADDRESS_SANITIZER
#else
# undef ADDRESS_SANITIZER
#endif

#endif  // BREAKPAD_GOOGLETEST_INCLUDES_H__
