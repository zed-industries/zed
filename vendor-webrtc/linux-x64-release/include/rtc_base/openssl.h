/*
 *  Copyright 2013 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_OPENSSL_H_
#define RTC_BASE_OPENSSL_H_

#if defined(WEBRTC_WIN)
// Must be included first before openssl headers.
#include "rtc_base/win32.h"  // NOLINT
#endif                       // WEBRTC_WIN

#include <openssl/ssl.h>  // IWYU pragma: export

#if (OPENSSL_VERSION_NUMBER < 0x10100000L)
#error OpenSSL is older than 1.1.0, which is the minimum supported version.
#endif

#endif  // RTC_BASE_OPENSSL_H_
