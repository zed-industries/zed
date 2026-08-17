/*
 * Work around broken floating point limits on some systems.
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

#include_next <float.h>

#ifdef FLT_MAX
#undef  FLT_MAX
#define FLT_MAX 3.40282346638528859812e+38F

#undef  FLT_MIN
#define FLT_MIN 1.17549435082228750797e-38F

#undef  DBL_MAX
#define DBL_MAX ((double)1.79769313486231570815e+308L)

#undef  DBL_MIN
#define DBL_MIN ((double)2.22507385850720138309e-308L)
#endif
