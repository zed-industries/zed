/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2013 Apple Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

// no-include-guard-because-pch-file
// no-std-usage-because-pch-file

/* This prefix file is for use on Mac OS X only. This prefix file should contain
 * only files to precompile for faster builds. The project should be able to
 * build without this header, although we rarely test that.
 */

#ifdef THIRD_PARTY_BLINK_RENDERER_BUILD_MAC_PREFIX_H_
#error You shouldn't include the precompiled header file more than once.
#endif

#define THIRD_PARTY_BLINK_RENDERER_BUILD_MAC_PREFIX_H_

#include <pthread.h>
#include <sys/types.h>
#include <fcntl.h>
#include <regex.h>
#include <setjmp.h>
#include <signal.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#ifdef __cplusplus
#include <algorithm>
#include <cstddef>
#include <new>
#endif  // __cplusplus

#include <sys/param.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <CoreFoundation/CoreFoundation.h>

#ifdef __OBJC__
#import <Cocoa/Cocoa.h>
#endif
