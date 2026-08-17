/* -*- Mode: C; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/*
 * libdatrie - Double-Array Trie Library
 * Copyright (C) 2006  Theppitak Karoonboonyanan <theppitak@gmail.com>
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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

/*
 * typedefs.h - general types
 * Created : 11 Aug 2006
 * Author  : Theppitak Karoonboonyanan <theppitak@gmail.com>
 */

#ifndef __TYPEDEFS_H
#define __TYPEDEFS_H

#include <limits.h>

typedef enum { DA_FALSE = 0, DA_TRUE = 1 } Bool;
#ifndef FALSE
# define FALSE DA_FALSE
#endif
#ifndef TRUE
# define TRUE DA_TRUE
#endif

# if UCHAR_MAX == 0xff
#   ifndef UINT8_TYPEDEF
#     define UINT8_TYPEDEF
      typedef unsigned char  uint8;
#   endif /* UINT8_TYPEDEF */
# endif /* UCHAR_MAX */

# if SCHAR_MAX == 0x7f
#   ifndef INT8_TYPEDEF
#     define INT8_TYPEDEF
      typedef signed char    int8;
#   endif /* INT8_TYPEDEF */
# endif /* SCHAR_MAX */

# if UINT_MAX == 0xffff
#   ifndef UINT16_TYPEDEF
#     define UINT16_TYPEDEF
      typedef unsigned int   uint16;
#   endif /* UINT16_TYPEDEF */
# endif /* UINT_MAX */

# if INT_MAX == 0x7fff
#   ifndef INT16_TYPEDEF
#     define INT16_TYPEDEF
      typedef int            int16;
#   endif /* INT16_TYPEDEF */
# endif /* INT_MAX */

# if USHRT_MAX == 0xffff
#   ifndef UINT16_TYPEDEF
#     define UINT16_TYPEDEF
      typedef unsigned short uint16;
#   endif /* UINT16_TYPEDEF */
# endif /* USHRT_MAX */

# if SHRT_MAX == 0x7fff
#   ifndef INT16_TYPEDEF
#     define INT16_TYPEDEF
      typedef short          int16;
#   endif /* INT16_TYPEDEF */
# endif /* SHRT_MAX */

# if UINT_MAX == 0xffffffff
#   ifndef UINT32_TYPEDEF
#     define UINT32_TYPEDEF
      typedef unsigned int   uint32;
#   endif /* UINT32_TYPEDEF */
# endif /* UINT_MAX */

# if INT_MAX == 0x7fffffff
#   ifndef INT32_TYPEDEF
#     define INT32_TYPEDEF
      typedef int            int32;
#   endif /* INT32_TYPEDEF */
# endif /* INT_MAX */

# if ULONG_MAX == 0xffffffff
#   ifndef UINT32_TYPEDEF
#     define UINT32_TYPEDEF
      typedef unsigned long  uint32;
#   endif /* UINT32_TYPEDEF */
# endif /* ULONG_MAX */

# if LONG_MAX == 0x7fffffff
#   ifndef INT32_TYPEDEF
#     define INT32_TYPEDEF
      typedef long           int32;
#   endif /* INT32_TYPEDEF */
# endif /* LONG_MAX */

# ifndef UINT8_TYPEDEF
#   error "uint8 type is undefined!"
# endif
# ifndef INT8_TYPEDEF
#   error "int8 type is undefined!"
# endif
# ifndef UINT16_TYPEDEF
#   error "uint16 type is undefined!"
# endif
# ifndef INT16_TYPEDEF
#   error "int16 type is undefined!"
# endif
# ifndef UINT32_TYPEDEF
#   error "uint32 type is undefined!"
# endif
# ifndef INT32_TYPEDEF
#   error "int32 type is undefined!"
# endif


#endif /* __TYPEDEFS_H */

/*
vi:ts=4:ai:expandtab
*/
