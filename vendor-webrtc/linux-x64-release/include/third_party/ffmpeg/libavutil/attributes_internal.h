/*
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

#ifndef AVUTIL_ATTRIBUTES_INTERNAL_H
#define AVUTIL_ATTRIBUTES_INTERNAL_H

#include "attributes.h"

#if (AV_GCC_VERSION_AT_LEAST(4,0) || defined(__clang__)) && (defined(__ELF__) || defined(__MACH__))
#    define attribute_visibility_hidden __attribute__((visibility("hidden")))
#    define FF_VISIBILITY_PUSH_HIDDEN   _Pragma("GCC visibility push(hidden)")
#    define FF_VISIBILITY_POP_HIDDEN    _Pragma("GCC visibility pop")
#else
#    define attribute_visibility_hidden
#    define FF_VISIBILITY_PUSH_HIDDEN
#    define FF_VISIBILITY_POP_HIDDEN
#endif

#define EXTERN extern attribute_visibility_hidden

#endif /* AVUTIL_ATTRIBUTES_INTERNAL_H */
