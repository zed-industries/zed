// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#if defined(__linux__)
#undef _POSIX_C_SOURCE
#define _POSIX_C_SOURCE 200112L
#endif

#include <openssl/bio.h>
#include <openssl/err.h>

#if !defined(OPENSSL_NO_SOCK)

#include <time.h>
#include <fcntl.h>
#include <string.h>
#include <sys/types.h>

#if !defined(OPENSSL_WINDOWS)
#include <netdb.h>
#include <unistd.h>
#else
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
#include <ws2tcpip.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

#include "internal.h"
#include "../internal.h"


int bio_ip_and_port_to_socket_and_addr(int *out_sock,
                                       struct sockaddr_storage *out_addr,
                                       socklen_t *out_addr_length,
                                       const char *hostname,
                                       const char *port_str) {
  struct addrinfo hint, *result, *cur;
  int ret;

  *out_sock = -1;

  OPENSSL_cleanse(&hint,sizeof(hint));
  hint.ai_family = AF_UNSPEC;
  hint.ai_socktype = SOCK_STREAM;

  ret = getaddrinfo(hostname, port_str, &hint, &result);
  if (ret != 0) {
    OPENSSL_PUT_ERROR(SYS, 0);
#if defined(OPENSSL_WINDOWS)
    ERR_add_error_data(1, gai_strerrorA(ret));
#else
    ERR_add_error_data(1, gai_strerror(ret));
#endif
    return 0;
  }

  ret = 0;

  for (cur = result; cur; cur = cur->ai_next) {
    if ((size_t) cur->ai_addrlen > sizeof(struct sockaddr_storage)) {
      continue;
    }
    OPENSSL_cleanse(out_addr, sizeof(struct sockaddr_storage));
    OPENSSL_memcpy(out_addr, cur->ai_addr, cur->ai_addrlen);
    *out_addr_length = cur->ai_addrlen;

    *out_sock = socket(cur->ai_family, cur->ai_socktype, cur->ai_protocol);
    if (*out_sock < 0) {
      OPENSSL_PUT_SYSTEM_ERROR();
      goto out;
    }

    ret = 1;
    break;
  }

out:
  freeaddrinfo(result);
  return ret;
}

int bio_socket_nbio(int sock, int on) {
#if defined(OPENSSL_WINDOWS)
  u_long arg = on;

  return 0 == ioctlsocket(sock, FIONBIO, &arg);
#else
  int flags = fcntl(sock, F_GETFL, 0);
  if (flags < 0) {
    return 0;
  }
  if (!on) {
    flags &= ~O_NONBLOCK;
  } else {
    flags |= O_NONBLOCK;
  }
  return fcntl(sock, F_SETFL, flags) == 0;
#endif
}

void bio_clear_socket_error(int sock) {
  bio_sock_error_get_and_clear(sock);
}

int bio_sock_error_get_and_clear(int sock) {
  int error;
  socklen_t error_size = sizeof(error);
  // Get and clear the pending socket error. The SO_ERROR option is read-only.
  if (getsockopt(sock, SOL_SOCKET, SO_ERROR, (char *)&error, &error_size) < 0) {
    return 1;
  }
  return error;
}

int bio_socket_should_retry(int return_value) {
#if defined(OPENSSL_WINDOWS)
  return return_value == -1 && (WSAGetLastError() == WSAEWOULDBLOCK);
#else
  // On POSIX platforms, sockets and fds are the same.
  return bio_errno_should_retry(return_value);
#endif
}

#endif  // OPENSSL_NO_SOCK
