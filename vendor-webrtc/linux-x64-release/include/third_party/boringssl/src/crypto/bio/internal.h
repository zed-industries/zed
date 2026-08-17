// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_BIO_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_BIO_INTERNAL_H

#include <openssl/base.h>

#if !defined(OPENSSL_NO_SOCK)
#if !defined(OPENSSL_WINDOWS)
#if defined(OPENSSL_PNACL)
// newlib uses u_short in socket.h without defining it.
typedef unsigned short u_short;
#endif
#include <sys/types.h>
#include <sys/socket.h>
#else
#include <winsock2.h>
typedef int socklen_t;
#endif
#endif  // !OPENSSL_NO_SOCK

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(OPENSSL_NO_SOCK)

// bio_ip_and_port_to_socket_and_addr creates a socket and fills in |*out_addr|
// and |*out_addr_length| with the correct values for connecting to |hostname|
// on |port_str|. It returns one on success or zero on error.
int bio_ip_and_port_to_socket_and_addr(int *out_sock,
                                       struct sockaddr_storage *out_addr,
                                       socklen_t *out_addr_length,
                                       const char *hostname,
                                       const char *port_str);

// bio_socket_nbio sets whether |sock| is non-blocking. It returns one on
// success and zero otherwise.
int bio_socket_nbio(int sock, int on);

// bio_clear_socket_error clears the last system socket error.
//
// TODO(fork): remove all callers of this.
void bio_clear_socket_error(void);

// bio_sock_error returns the last socket error on |sock|.
int bio_sock_error(int sock);

// bio_socket_should_retry returns non-zero if |return_value| indicates an error
// and the last socket error indicates that it's non-fatal.
int bio_socket_should_retry(int return_value);

#endif  // !OPENSSL_NO_SOCK

// bio_errno_should_retry returns non-zero if |return_value| indicates an error
// and |errno| indicates that it's non-fatal.
int bio_errno_should_retry(int return_value);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_BIO_INTERNAL_H
