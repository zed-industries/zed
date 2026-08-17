/*
 * Copyright (C) 2016 Free Software Foundation, Inc.
 *
 * Author: Tim Ruehsen
 *
 * This file is part of GnuTLS.
 *
 * The GnuTLS is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 *
 */

/* This file contains socket related types, prototypes and includes.
 */

#ifndef GNUTLS_SOCKET_H
#define GNUTLS_SOCKET_H

#include <gnutls/gnutls.h>

/* Get socklen_t */
#include <sys/socket.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

void gnutls_transport_set_fastopen(gnutls_session_t session,
                                   int fd,
                                   struct sockaddr *connect_addr,
                                   socklen_t connect_addrlen,
                                   unsigned int flags);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_SOCKET_H */
