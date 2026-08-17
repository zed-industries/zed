/* GIO - GLib Input, Output and Streaming Library
 *
 * Copyright (C) 2008-2011 Red Hat, Inc.
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __G_NETWORKING_H__
#define __G_NETWORKING_H__

#include <gio/gio-visibility.h>
#include <glib.h>

#ifdef G_OS_WIN32
#include <winsock2.h>
#include <ws2tcpip.h>

#include <iphlpapi.h>
#include <mswsock.h>
#include <windns.h>
#include <wspiapi.h>
#undef interface

#else /* !G_OS_WIN32 */

#include <arpa/inet.h>
#include <arpa/nameser.h>
#include <net/if.h>
#include <netdb.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <resolv.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/un.h>

#ifndef __GI_SCANNER__

#ifndef T_SRV
#define T_SRV 33
#endif

#ifndef _PATH_RESCONF
#define _PATH_RESCONF "/etc/resolv.conf"
#endif

#ifndef CMSG_LEN
/* CMSG_LEN and CMSG_SPACE are defined by RFC 2292, but missing on
 * some older platforms.
 */
#define CMSG_LEN(len) ((size_t)CMSG_DATA((struct cmsghdr*)NULL) + (len))

/* CMSG_SPACE must add at least as much padding as CMSG_NXTHDR()
 * adds. We overestimate here.
 */
#define GLIB_ALIGN_TO_SIZEOF(len, obj) \
  (((len) + sizeof(obj) - 1) & ~(sizeof(obj) - 1))
#define CMSG_SPACE(len) GLIB_ALIGN_TO_SIZEOF(CMSG_LEN(len), struct cmsghdr)
#endif
#endif

#endif /* !__GI_SCANNER__ */

G_BEGIN_DECLS

GIO_AVAILABLE_IN_2_36
void g_networking_init(void);

G_END_DECLS

#endif /* __G_NETWORKING_H__ */
