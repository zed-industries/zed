// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/bio.h>

#if !defined(OPENSSL_NO_SOCK)

#include <openssl/mem.h>

#include "../internal.h"
#include "./internal.h"

#if !defined(OPENSSL_WINDOWS)
#include <sys/time.h>
static int closesocket(const int sock) { return close(sock); }
#else
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

typedef struct bio_dgram_data_st {
  BIO_ADDR peer;
  unsigned int connected;
  unsigned int _errno;
} bio_dgram_data;

static socklen_t BIO_ADDR_sockaddr_size(const BIO_ADDR *bap) {
  GUARD_PTR(bap);
  if (bap->sa.sa_family == AF_INET) {
    return sizeof(bap->s_in);
  }
#ifdef AF_INET6
  if (bap->sa.sa_family == AF_INET6) {
    return sizeof(bap->s_in6);
  }
#endif
#ifdef AWS_LC_HAS_AF_UNIX
  if (bap->sa.sa_family == AF_UNIX) {
    return sizeof(bap->s_un);
  }
#endif
  return sizeof(*bap);
}

static struct sockaddr *BIO_ADDR_sockaddr_noconst(BIO_ADDR *bap) {
  GUARD_PTR(bap);
  return &bap->sa;
}

static const struct sockaddr *BIO_ADDR_sockaddr(const BIO_ADDR *bap) {
  GUARD_PTR(bap);
  return &bap->sa;
}

static int dgram_get_errno(void) {
#if defined(OPENSSL_WINDOWS)
  return WSAGetLastError();
#else
  return errno;
#endif
}

static int dgram_write(BIO *bp, const char *in, const int in_len) {
  GUARD_PTR(bp);
  GUARD_PTR(in);

  ssize_t result;
  bio_dgram_data *data = bp->ptr;
  GUARD_PTR(data);

  if (in_len < 0) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_INVALID_ARGUMENT);
    return -1;
  }

  bio_clear_socket_error(bp->num);
  if (data->connected) {
    // With a zero flags argument, send() is equivalent to write(2).
    result = send(bp->num, in, in_len, 0);
  } else {
    // If a peer address has been pre-specified, sendto may return -1 and set
    // errno to[EISCONN].
    const socklen_t peerlen = BIO_ADDR_sockaddr_size(&data->peer);
    result =
        sendto(bp->num, in, in_len, 0, BIO_ADDR_sockaddr(&data->peer), peerlen);
  }

  if (result < INT_MIN || result > INT_MAX) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
    return -1;
  }
  const int ret = result;

  BIO_clear_retry_flags(bp);
  if (ret <= 0) {
    if (bio_socket_should_retry(ret)) {
      BIO_set_retry_write(bp);
    }
    data->_errno = dgram_get_errno();
  }
  return ret;
}

static int dgram_read(BIO *bp, char *out, const int out_len) {
  GUARD_PTR(bp);
  GUARD_PTR(out);
  GUARD_PTR(bp->ptr);

  BIO_ADDR peer;
  // Might be modified by call to `recvfrom`.
  socklen_t len = sizeof(peer);

  bio_dgram_data *data = bp->ptr;
  bio_clear_socket_error(bp->num);

  if (out_len < 0) {
    // out_len is cast to `size_t` below.
    OPENSSL_PUT_ERROR(BIO, BIO_R_INVALID_ARGUMENT);
    return -1;
  }

  // recvfrom may be used to receive data on a socket regardless of whether
  // it's connection-oriented.
  const ssize_t result = recvfrom(bp->num, out, out_len, 0,
                                  BIO_ADDR_sockaddr_noconst(&peer), &len);

  if (result < INT_MIN || result > INT_MAX) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
    return -1;
  }
  const int ret = result;

  if (!data->connected && ret >= 0) {
    if (1 != BIO_dgram_set_peer(bp, &peer)) {
      // The operation does not fail if peer not set.
    }
  }

  BIO_clear_retry_flags(bp);
  if (ret < 0) {
    if (bio_socket_should_retry(ret)) {
      BIO_set_retry_read(bp);
    }
    data->_errno = dgram_get_errno();
  }

  return ret;
}

static int dgram_puts(BIO *bp, const char *str) {
  GUARD_PTR(bp);
  GUARD_PTR(str);

  const int len = OPENSSL_strnlen(str, INT_MAX);
  return dgram_write(bp, str, len);
}

static int dgram_free(BIO *bp) {
  GUARD_PTR(bp);
  if (bp->shutdown && bp->init) {
    if (0 != closesocket(bp->num)) {
      // the show must go on
    }
  }
  bp->init = 0;
  bp->num = -1;
  bp->flags = 0;
  OPENSSL_free(bp->ptr);
  bp->ptr = NULL;
  return 1;
}

static int dgram_new(BIO *bio) {
  bio->init = 0;
  bio->num = -1;
  bio->ptr = OPENSSL_zalloc(sizeof(bio_dgram_data));
  return bio->ptr != NULL;
}

static long dgram_ctrl(BIO *bp, const int cmd, const long num, void *ptr) {
  GUARD_PTR(bp);
  bio_dgram_data *data = bp->ptr;

  long ret = 1;

  switch (cmd) {
    case BIO_C_SET_FD:
      GUARD_PTR(ptr);
      int fd = *(int *)ptr;
      if (fd < 0) {
        // file descriptors must be non-negative.
        OPENSSL_PUT_ERROR(BIO, BIO_R_INVALID_ARGUMENT);
        return 0;
      }
      dgram_free(bp);
      dgram_new(bp);
      bp->num = fd;
      bp->shutdown = (int)num;
      bp->init = 1;
      break;
    case BIO_C_GET_FD:
      if (bp->init) {
        int *ip = ptr;
        if (ip) {
          *ip = bp->num;
        }
        ret = bp->num;
      } else {
        ret = -1;
      }
      break;
    case BIO_CTRL_GET_CLOSE:
      ret = bp->shutdown;
      break;
    case BIO_CTRL_SET_CLOSE:
      bp->shutdown = num != 0;
      break;
    case BIO_CTRL_FLUSH:
      ret = 1;
      break;
    case BIO_CTRL_DGRAM_SET_CONNECTED:
      GUARD_PTR(data);
      if (ptr != NULL) {
        data->connected = 1;
        ret = BIO_ADDR_copy(&data->peer, ptr);
      } else {
        data->connected = 0;
        OPENSSL_cleanse(&data->peer, sizeof(data->peer));
        ret = 1;
      }
      break;
    case BIO_CTRL_DGRAM_GET_PEER: {
      GUARD_PTR(data);
      GUARD_PTR(ptr);
      const socklen_t size = BIO_ADDR_sockaddr_size(&data->peer);
      if (num == 0 || num >= (long)size) {
        OPENSSL_memcpy(ptr, &data->peer, size);
        ret = size;
      } else {
        ret = 0;
      }
      break;
    }
    case BIO_CTRL_DGRAM_CONNECT:
    case BIO_CTRL_DGRAM_SET_PEER:
      GUARD_PTR(data);
      GUARD_PTR(ptr);
      ret = BIO_ADDR_copy(&data->peer, ptr);
      break;
    case BIO_CTRL_DGRAM_GET_SEND_TIMER_EXP:
    case BIO_CTRL_DGRAM_GET_RECV_TIMER_EXP: {
      GUARD_PTR(data);
      int d_errno = 0;
#ifdef OPENSSL_WINDOWS
      d_errno = (data->_errno == WSAETIMEDOUT);
#else
      /*
       * if no data has been transferred and the timeout has been reached,
       * then -1 is returned with errno set to EAGAIN or EWOULDBLOCK,
       * or EINPROGRESS (for connect(2)) just as if the socket was specified
       * to be nonblocking.
       */
      d_errno = data->_errno == EAGAIN || data->_errno == EWOULDBLOCK ||
                data->_errno == EINPROGRESS;
#endif
      if (d_errno) {
        ret = 1;
        data->_errno = 0;
      } else {
        ret = 0;
      }
      break;
    }
#if defined(SO_RCVTIMEO)
    case BIO_CTRL_DGRAM_SET_RECV_TIMEOUT: {
      GUARD_PTR(ptr);
      struct timeval *tv = (struct timeval *)ptr;
      // Check for negative values in timeval
      if (tv->tv_sec < 0 || tv->tv_usec < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_INVALID_ARGUMENT);
        ret = -1;
        break;
      }
#ifdef OPENSSL_WINDOWS
      DWORD timeout = (DWORD)(tv->tv_sec * 1000 + tv->tv_usec / 1000);
      if (setsockopt(bp->num, SOL_SOCKET, SO_RCVTIMEO,
                     (void *)&timeout, sizeof(timeout)) != 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      }
#else
      if (setsockopt(bp->num, SOL_SOCKET, SO_RCVTIMEO, ptr,
                     sizeof(struct timeval)) < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      }
#endif
      break;
    }
    case BIO_CTRL_DGRAM_GET_RECV_TIMEOUT: {
      GUARD_PTR(ptr);
      union {
        size_t s;
        int i;
      } sz = {0};
#ifdef OPENSSL_WINDOWS
      int timeout;
      struct timeval *tv = (struct timeval *)ptr;

      sz.i = sizeof(timeout);
      if (getsockopt(bp->num, SOL_SOCKET, SO_RCVTIMEO,
                     (void *)&timeout, &sz.i) != 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      } else {
        tv->tv_sec = timeout / 1000;
        tv->tv_usec = (timeout % 1000) * 1000;
        ret = sizeof(*tv);
      }
#else
      sz.i = sizeof(struct timeval);
      if (getsockopt(bp->num, SOL_SOCKET, SO_RCVTIMEO,
                     ptr, (void *)&sz) < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      } else if (sizeof(sz.s) != sizeof(sz.i) && sz.i == 0) {
        ret = (int)sz.s;
      } else {
        ret = sz.i;
      }
#endif
      break;
    }
#endif
#if defined(SO_SNDTIMEO)
    case BIO_CTRL_DGRAM_SET_SEND_TIMEOUT: {
      GUARD_PTR(ptr);
      struct timeval *tv = (struct timeval *)ptr;
      // Check for negative values in timeval
      if (tv->tv_sec < 0 || tv->tv_usec < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_INVALID_ARGUMENT);
        ret = -1;
        break;
      }
#ifdef OPENSSL_WINDOWS
      DWORD timeout = (DWORD)(tv->tv_sec * 1000 + tv->tv_usec / 1000);
      if (setsockopt(bp->num, SOL_SOCKET, SO_SNDTIMEO,
                     (void *)&timeout, sizeof(timeout)) != 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      }
#else
      if (setsockopt(bp->num, SOL_SOCKET, SO_SNDTIMEO, ptr,
                     sizeof(struct timeval)) < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      }
#endif
      break;
    }
    case BIO_CTRL_DGRAM_GET_SEND_TIMEOUT: {
      GUARD_PTR(ptr);
      union {
        size_t s;
        int i;
      } sz = {0};
#ifdef OPENSSL_WINDOWS
      int timeout;
      struct timeval *tv = (struct timeval *)ptr;

      sz.i = sizeof(timeout);
      if (getsockopt(bp->num, SOL_SOCKET, SO_SNDTIMEO,
                     (void *)&timeout, &sz.i) != 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      } else {
        tv->tv_sec = timeout / 1000;
        tv->tv_usec = (timeout % 1000) * 1000;
        ret = sizeof(*tv);
      }
#else
      sz.i = sizeof(struct timeval);
      if (getsockopt(bp->num, SOL_SOCKET, SO_SNDTIMEO,
                     ptr, (void *)&sz) < 0) {
        OPENSSL_PUT_ERROR(BIO, BIO_R_SYS_LIB);
        ret = -1;
      } else if (sizeof(sz.s) != sizeof(sz.i) && sz.i == 0) {
        ret = (int)sz.s;
      } else {
        ret = sz.i;
      }
#endif
      break;
    }
#endif
    default:
      ret = 0;
      break;
  }
  return ret;
}



static const BIO_METHOD methods_dgramp = {
    .type = BIO_TYPE_DGRAM,
    .name = "datagram socket",
    .bwrite = dgram_write,
    .bread = dgram_read,
    .bputs = dgram_puts,
    .bgets = NULL,
    .ctrl = dgram_ctrl,
    .create = dgram_new,
    .destroy = dgram_free,
    .callback_ctrl = NULL,
};

const BIO_METHOD *BIO_s_datagram(void) { return &methods_dgramp; }

BIO *BIO_new_dgram(int fd, int close_flag) {
  BIO *ret = BIO_new(BIO_s_datagram());
  if (ret == NULL) {
    return NULL;
  }
  int result = BIO_set_fd(ret, fd, close_flag);
  if (result <= 0) {
    BIO_free(ret);
    return NULL;
  }
  return ret;
}

int BIO_ctrl_dgram_connect(BIO *bp, const BIO_ADDR *peer) {
  const long ret = BIO_ctrl(bp, BIO_CTRL_DGRAM_CONNECT, 0, (BIO_ADDR *)peer);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

int BIO_ctrl_set_connected(BIO *bp, const BIO_ADDR *peer) {
  const long ret =
      BIO_ctrl(bp, BIO_CTRL_DGRAM_SET_CONNECTED, 0, (BIO_ADDR *)peer);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

int BIO_dgram_recv_timedout(BIO *bp) {
  const long ret = BIO_ctrl(bp, BIO_CTRL_DGRAM_GET_RECV_TIMER_EXP, 0, NULL);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

int BIO_dgram_send_timedout(BIO *bp) {
  const long ret = BIO_ctrl(bp, BIO_CTRL_DGRAM_GET_SEND_TIMER_EXP, 0, NULL);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

int BIO_dgram_get_peer(BIO *bp, BIO_ADDR *peer) {
  const long ret = BIO_ctrl(bp, BIO_CTRL_DGRAM_GET_PEER, 0, peer);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

int BIO_dgram_set_peer(BIO *bp, const BIO_ADDR *peer) {
  const long ret = BIO_ctrl(bp, BIO_CTRL_DGRAM_SET_PEER, 0, (BIO_ADDR *)peer);
  if (ret < INT_MIN || ret > INT_MAX) {
    return 0;
  }
  return ret;
}

#endif
