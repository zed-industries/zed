// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bio.h>

#if !defined(OPENSSL_NO_POSIX_IO)

#include <errno.h>
#include <string.h>

#if !defined(OPENSSL_WINDOWS)
#include <unistd.h>
#else
#include <io.h>
#endif

#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../internal.h"


#if defined(OPENSSL_WINDOWS)
  #define BORINGSSL_CLOSE _close
  #define BORINGSSL_LSEEK _lseek
  #define BORINGSSL_READ _read
  #define BORINGSSL_WRITE _write
#else
  #define BORINGSSL_CLOSE close
  #define BORINGSSL_LSEEK lseek
  #define BORINGSSL_READ read
  #define BORINGSSL_WRITE write
#endif

BIO *BIO_new_fd(int fd, int close_flag) {
  BIO *ret = BIO_new(BIO_s_fd());
  if (ret == NULL) {
    return NULL;
  }
  BIO_set_fd(ret, fd, close_flag);
  return ret;
}

static int fd_new(BIO *bio) {
  // num is used to store the file descriptor.
  bio->num = -1;
  return 1;
}

static int fd_free(BIO *bio) {
  if (bio->shutdown) {
    if (bio->init) {
      BORINGSSL_CLOSE(bio->num);
    }
    bio->init = 0;
  }
  return 1;
}

static int fd_read(BIO *b, char *out, int outl) {
  int ret = 0;

  ret = (int)BORINGSSL_READ(b->num, out, outl);
  BIO_clear_retry_flags(b);
  if (ret <= 0) {
    if (bio_errno_should_retry(ret)) {
      BIO_set_retry_read(b);
    }
  }

  return ret;
}

static int fd_write(BIO *b, const char *in, int inl) {
  int ret = (int)BORINGSSL_WRITE(b->num, in, inl);
  BIO_clear_retry_flags(b);
  if (ret <= 0) {
    if (bio_errno_should_retry(ret)) {
      BIO_set_retry_write(b);
    }
  }

  return ret;
}

static long fd_ctrl(BIO *b, int cmd, long num, void *ptr) {
  long ret = 1;
  int *ip;

  switch (cmd) {
    case BIO_CTRL_RESET:
      num = 0;
      OPENSSL_FALLTHROUGH;
    case BIO_C_FILE_SEEK:
      ret = 0;
      if (b->init) {
        ret = (long)BORINGSSL_LSEEK(b->num, num, SEEK_SET);
      }
      break;
    case BIO_C_FILE_TELL:
    case BIO_CTRL_INFO:
      ret = 0;
      if (b->init) {
        ret = (long)BORINGSSL_LSEEK(b->num, 0, SEEK_CUR);
      }
      break;
    case BIO_C_SET_FD:
      fd_free(b);
      b->num = *((int *)ptr);
      b->shutdown = (int)num;
      b->init = 1;
      break;
    case BIO_C_GET_FD:
      if (b->init) {
        ip = (int *)ptr;
        if (ip != NULL) {
          *ip = b->num;
        }
        return b->num;
      } else {
        ret = -1;
      }
      break;
    case BIO_CTRL_GET_CLOSE:
      ret = b->shutdown;
      break;
    case BIO_CTRL_SET_CLOSE:
      b->shutdown = (int)num;
      break;
    case BIO_CTRL_PENDING:
    case BIO_CTRL_WPENDING:
      ret = 0;
      break;
    case BIO_CTRL_FLUSH:
      ret = 1;
      break;
    default:
      ret = 0;
      break;
  }

  return ret;
}

static int fd_gets(BIO *bp, char *buf, int size) {
  if (size <= 0) {
    return 0;
  }

  char *ptr = buf;
  char *end = buf + size - 1;
  while (ptr < end && fd_read(bp, ptr, 1) > 0) {
    char c = ptr[0];
    ptr++;
    if (c == '\n') {
      break;
    }
  }

  ptr[0] = '\0';

  // The output length is bounded by |size|.
  return (int)(ptr - buf);
}

static const BIO_METHOD methods_fdp = {
    BIO_TYPE_FD, "file descriptor", fd_write, fd_read, NULL /* puts */,
    fd_gets,     fd_ctrl,           fd_new,   fd_free, NULL /* callback_ctrl */,
};

const BIO_METHOD *BIO_s_fd(void) { return &methods_fdp; }

#endif  // OPENSSL_NO_POSIX_IO

int BIO_set_fd(BIO *bio, int fd, int close_flag) {
  return (int)BIO_int_ctrl(bio, BIO_C_SET_FD, close_flag, fd);
}

int BIO_get_fd(BIO *bio, int *out_fd) {
  return (int)BIO_ctrl(bio, BIO_C_GET_FD, 0, (char *) out_fd);
}
