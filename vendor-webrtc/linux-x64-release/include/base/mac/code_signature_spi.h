// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_CODE_SIGNATURE_SPI_H_
#define BASE_MAC_CODE_SIGNATURE_SPI_H_

#include <unistd.h>

extern "C" {

// From
// https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/codesign.h

#define CS_OPS_STATUS 0               /* return status */
#define CS_OPS_TEAMID 14              /* get team id */
#define CS_OPS_VALIDATION_CATEGORY 17 /* get process validation category */

#define CS_MAX_TEAMID_LEN 64

int csops(pid_t pid, unsigned int ops, void* useraddr, size_t usersize);

}  // extern "C"

#endif  // BASE_MAC_CODE_SIGNATURE_SPI_H_
