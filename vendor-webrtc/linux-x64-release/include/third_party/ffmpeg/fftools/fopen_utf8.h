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

#ifndef FFTOOLS_FOPEN_UTF8_H
#define FFTOOLS_FOPEN_UTF8_H

#include <stdio.h>

/* The fopen_utf8 function here is essentially equivalent to avpriv_fopen_utf8,
 * except that it doesn't set O_CLOEXEC, and that it isn't exported
 * from a different library. (On Windows, each DLL might use a different
 * CRT, and FILE* handles can't be shared across them.) */

#ifdef _WIN32
#include "libavutil/mem.h"
#include "libavutil/wchar_filename.h"

static inline FILE *fopen_utf8(const char *path_utf8, const char *mode)
{
    wchar_t *path_w, *mode_w;
    FILE *f;

    /* convert UTF-8 to wide chars */
    if (get_extended_win32_path(path_utf8, &path_w)) /* This sets errno on error. */
        return NULL;
    if (!path_w)
        goto fallback;

    if (utf8towchar(mode, &mode_w))
        return NULL;
    if (!mode_w) {
        /* If failing to interpret the mode string as utf8, it is an invalid
         * parameter. */
        av_freep(&path_w);
        errno = EINVAL;
        return NULL;
    }

    f = _wfopen(path_w, mode_w);
    av_freep(&path_w);
    av_freep(&mode_w);

    return f;
fallback:
    /* path may be in CP_ACP */
    return fopen(path_utf8, mode);
}

#else

static inline FILE *fopen_utf8(const char *path, const char *mode)
{
    return fopen(path, mode);
}
#endif

#endif /* FFTOOLS_FOPEN_UTF8_H */
