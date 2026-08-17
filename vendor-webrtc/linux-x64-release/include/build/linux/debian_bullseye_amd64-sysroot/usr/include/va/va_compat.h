/*
 * Copyright (c) 2007-2011 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/**
 * \file va_compat.h
 * \brief The Compatibility API
 *
 * This file contains the \ref api_compat "Compatibility API".
 */

#ifndef VA_COMPAT_H
#define VA_COMPAT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_compat Compatibility API
 *
 * The Compatibility API allows older programs that are not ported to
 * the current API to still build and run correctly. In particular,
 * this exposes older API to allow for backwards source compatibility.
 *
 * @{
 */

/**
 * Makes a string literal out of the macro argument
 */
#define VA_CPP_HELPER_STRINGIFY(x) \
    VA_CPP_HELPER_STRINGIFY_(x)
#define VA_CPP_HELPER_STRINGIFY_(x) \
    #x

/**
 * Concatenates two macro arguments at preprocessing time.
 */
#define VA_CPP_HELPER_CONCAT(a, b) \
    VA_CPP_HELPER_CONCAT_(a, b)
#define VA_CPP_HELPER_CONCAT_(a, b) \
    a ## b

/**
 * Generates the number of macro arguments at preprocessing time.
 * <http://groups.google.com/group/comp.std.c/browse_thread/thread/77ee8c8f92e4a3fb/346fc464319b1ee5>
 *
 * Note: this doesn't work for macros with no arguments
 */
#define VA_CPP_HELPER_N_ARGS(...) \
    VA_CPP_HELPER_N_ARGS_(__VA_ARGS__, VA_CPP_HELPER_N_ARGS_LIST_REV())
#define VA_CPP_HELPER_N_ARGS_(...) \
    VA_CPP_HELPER_N_ARGS_LIST(__VA_ARGS__)
#define VA_CPP_HELPER_N_ARGS_LIST(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a12, a13, a14, a15, a16, N, ...) N
#define VA_CPP_HELPER_N_ARGS_LIST_REV() \
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

/**
 * Generates a versioned function alias.
 *
 * VA_CPP_HELPER_ALIAS(vaSomeFunction, 0,32,0) will generate
 *   .symber vaSomeFunction_0_32_0, vaSomeFunction@VA_API_0.32.0
 */
#define VA_CPP_HELPER_ALIAS(func, major, minor, micro) \
    VA_CPP_HELPER_ALIAS_(func, major, minor, micro, "@")
#define VA_CPP_HELPER_ALIAS_DEFAULT(func, major, minor, micro) \
    VA_CPP_HELPER_ALIAS_(func, major, minor, micro, "@@")
#define VA_CPP_HELPER_ALIAS_(func, major, minor, micro, binding)        \
    asm(".symver " #func "_" #major "_" #minor "_" #micro ", "          \
        #func binding "VA_API_" #major "." #minor "." #micro)

/* vaCreateSurfaces() */

#ifndef VA_COMPAT_DISABLED
#define vaCreateSurfaces(dpy, ...)                                      \
    VA_CPP_HELPER_CONCAT(vaCreateSurfaces,                              \
                         VA_CPP_HELPER_N_ARGS(dpy, __VA_ARGS__))        \
    (dpy, __VA_ARGS__)
#endif

#define vaCreateSurfaces6(dpy, width, height, format, num_surfaces, surfaces) \
    (vaCreateSurfaces)(dpy, format, width, height, surfaces, num_surfaces, \
                       NULL, 0)

#define vaCreateSurfaces8(dpy, format, width, height, surfaces, num_surfaces, attribs, num_attribs) \
    (vaCreateSurfaces)(dpy, format, width, height, surfaces, num_surfaces, \
                       attribs, num_attribs)

/*@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_COMPAT_H */
