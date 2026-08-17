// Copyright 2022 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_POSIX_TIME_H
#define OPENSSL_HEADER_POSIX_TIME_H

#include <openssl/base.h>   // IWYU pragma: export

#include <time.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Time functions.


// OPENSSL_posix_to_tm converts a int64_t POSIX time value in |time|, which must
// be in the range of year 0000 to 9999, to a broken out time value in |tm|. It
// returns one on success and zero on error.
OPENSSL_EXPORT int OPENSSL_posix_to_tm(int64_t time, struct tm *out_tm);

// OPENSSL_tm_to_posix converts a time value between the years 0 and 9999 in
// |tm| to a POSIX time value in |out|. One is returned on success, zero is
// returned on failure. It is a failure if |tm| contains out of range values.
OPENSSL_EXPORT int OPENSSL_tm_to_posix(const struct tm *tm, int64_t *out);

// OPENSSL_timegm converts a time value between the years 0 and 9999 in |tm| to
// a time_t value in |out|. One is returned on success, zero is returned on
// failure. It is a failure if the converted time can not be represented in a
// time_t, or if the tm contains out of range values.
OPENSSL_EXPORT int OPENSSL_timegm(const struct tm *tm, time_t *out);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_POSIX_TIME_H
