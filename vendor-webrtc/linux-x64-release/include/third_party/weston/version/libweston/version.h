/*
 * Copyright © 2013 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#ifndef WESTON_VERSION_H
#define WESTON_VERSION_H

#define WESTON_VERSION_MAJOR 12
#define WESTON_VERSION_MINOR 0
#define WESTON_VERSION_MICRO 1
#define WESTON_VERSION "12.0.1"

/* This macro may not do what you expect.  Weston doesn't guarantee
 * a stable API between 1.X and 1.Y, and thus this macro will return
 * FALSE on any WESTON_VERSION_AT_LEAST(1,X,0) if the actual version
 * is 1.Y.0 and X != Y).  In particular, it fails if X < Y, that is,
 * 1.3.0 is considered to not be "at least" 1.4.0.
 *
 * If you want to test for the version number being 1.3.0 or above or
 * maybe in a range (eg 1.2.0 to 1.4.0), just use the WESTON_VERSION_*
 * defines above directly.
 */

#define WESTON_VERSION_AT_LEAST(major, minor, micro) \
        (WESTON_VERSION_MAJOR == (major) && \
         WESTON_VERSION_MINOR == (minor) && \
         WESTON_VERSION_MICRO >= (micro))

#endif
