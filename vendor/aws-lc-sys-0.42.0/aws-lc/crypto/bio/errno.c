// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bio.h>

#include <errno.h>

#include "internal.h"


int bio_errno_should_retry(int return_value) {
  if (return_value != -1) {
    return 0;
  }

  return
#ifdef EWOULDBLOCK
      errno == EWOULDBLOCK ||
#endif
#ifdef ENOTCONN
      errno == ENOTCONN ||
#endif
#ifdef EINTR
      errno == EINTR ||
#endif
#ifdef EAGAIN
      errno == EAGAIN ||
#endif
#ifdef EPROTO
      errno == EPROTO ||
#endif
#ifdef EINPROGRESS
      errno == EINPROGRESS ||
#endif
#ifdef EALREADY
      errno == EALREADY ||
#endif
      0;
}
