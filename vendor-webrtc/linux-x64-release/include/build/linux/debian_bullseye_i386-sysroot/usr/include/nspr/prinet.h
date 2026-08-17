/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * File:        prinet.h
 * Description:
 *     Header file used to find the system header files for socket support[1].
 *     This file serves the following purposes:
 *     - A cross-platform, "get-everything" socket header file.  On
 *       Unix, socket support is scattered in several header files,
 *       while Windows has a "get-everything" socket header file[2].
 *     - NSPR needs the following macro definitions and function
 *       prototype declarations from these header files:
 *           AF_INET
 *           INADDR_ANY, INADDR_LOOPBACK, INADDR_BROADCAST
 *           ntohl(), ntohs(), htonl(), ntons().
 *       NSPR does not define its own versions of these macros and
 *       functions.  It simply uses the native versions, which have
 *       the same names on all supported platforms.
 *     This file is intended to be included by NSPR public header
 *     files, such as prio.h.  One should not include this file directly.
 *
 * Notes:
 *     1. This file should have been an internal header.  Please do not
 *        depend on it to pull in the system header files you need.
 *     2. WARNING: This file is no longer cross-platform as it is a no-op
 *        for WIN32!  See the comment in the WIN32 section for details.
 */

#ifndef prinet_h__
#define prinet_h__

#if defined(XP_UNIX) || defined(XP_OS2)
#include <sys/types.h>
#include <sys/socket.h>     /* AF_INET */
#include <netinet/in.h>         /* INADDR_ANY, ..., ntohl(), ... */
#ifdef XP_OS2
#include <sys/ioctl.h>
#endif
#ifdef XP_UNIX
#ifdef AIX
/*
 * On AIX 4.3, the header <arpa/inet.h> refers to struct
 * ether_addr and struct sockaddr_dl that are not declared.
 * The following struct declarations eliminate the compiler
 * warnings.
 */
struct ether_addr;
struct sockaddr_dl;
#endif /* AIX */
#include <arpa/inet.h>
#endif /* XP_UNIX */
#include <netdb.h>

#if defined(BSDI) || defined(QNX)
#include <rpc/types.h> /* the only place that defines INADDR_LOOPBACK */
#endif

/*
 * OS/2 hack.  For some reason INADDR_LOOPBACK is not defined in the
 * socket headers.
 */
#if defined(OS2) && !defined(INADDR_LOOPBACK)
#define INADDR_LOOPBACK 0x7f000001
#endif

/*
 * Prototypes of ntohl() etc. are declared in <machine/endian.h>
 * on these platforms.
 */
#if defined(BSDI)
#include <machine/endian.h>
#endif

/* On Android, ntohl() etc. are declared in <sys/endian.h>. */
#ifdef __ANDROID__
#include <sys/endian.h>
#endif

#elif defined(WIN32)

/*
 * Do not include any system header files.
 *
 * Originally we were including <windows.h>.  It slowed down the
 * compilation of files that included NSPR headers, so we removed
 * the <windows.h> inclusion at customer's request, which created
 * an unfortunate inconsistency with other platforms.
 */

#else

#error Unknown platform

#endif

#endif /* prinet_h__ */
