/*
 * C99-compatible snprintf() and vsnprintf() implementations
 * Copyright (c) 2012 Ronald S. Bultje <rsbultje@gmail.com>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef COMPAT_MSVCRT_SNPRINTF_H
#define COMPAT_MSVCRT_SNPRINTF_H

#include <stdarg.h>
#include <stdio.h>

extern int avpriv_snprintf(char *s, size_t n, const char *fmt, ...);
extern int avpriv_vsnprintf(char *s, size_t n, const char *fmt, va_list ap);

#endif /* COMPAT_MSVCRT_SNPRINTF_H */
