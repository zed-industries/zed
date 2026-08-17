/* PipeWire
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef PIPEWIRE_VERSION_H
#define PIPEWIRE_VERSION_H

/* WARNING: Make sure to edit the real source file version.h.in! */

#ifdef __cplusplus
extern "C" {
#endif

/** Return the version of the header files. Keep in mind that this is
a macro and not a function, so it is impossible to get the pointer of
it. */
#define pw_get_headers_version() ("0.3.56")

/** Return the version of the library the current application is
 * linked to. */
const char* pw_get_library_version(void);

/** The current API version. Versions prior to 0.2.0 have
 * PW_API_VERSION undefined. Please note that this is only ever
 * increased on incompatible API changes!  */
#define PW_API_VERSION "0.3"

/** The major version of PipeWire. \since 0.2.0 */
#define PW_MAJOR 0

/** The minor version of PipeWire. \since 0.2.0 */
#define PW_MINOR 3

/** The micro version of PipeWire. \since 0.2.0 */
#define PW_MICRO 56

/** Evaluates to TRUE if the PipeWire library version is equal or
 * newer than the specified. \since 0.2.0 */
#define PW_CHECK_VERSION(major,minor,micro)                             \
    ((PW_MAJOR > (major)) ||                                            \
     (PW_MAJOR == (major) && PW_MINOR > (minor)) ||                     \
     (PW_MAJOR == (major) && PW_MINOR == (minor) && PW_MICRO >= (micro)))

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_VERSION_H */
