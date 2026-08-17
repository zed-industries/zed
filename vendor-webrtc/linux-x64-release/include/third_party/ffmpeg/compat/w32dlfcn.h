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

#ifndef COMPAT_W32DLFCN_H
#define COMPAT_W32DLFCN_H

#ifdef _WIN32
#include <stdint.h>

#include <windows.h>

#include "config.h"
#include "libavutil/macros.h"
#include "libavutil/mem.h"
#include "libavutil/wchar_filename.h"

static inline wchar_t *get_module_filename(HMODULE module)
{
    wchar_t *path = NULL, *new_path;
    DWORD path_size = 0, path_len;

    do {
        path_size = path_size ? FFMIN(2 * path_size, INT16_MAX + 1) : MAX_PATH;
        new_path = av_realloc_array(path, path_size, sizeof *path);
        if (!new_path) {
            av_free(path);
            return NULL;
        }
        path = new_path;
        // Returns path_size in case of insufficient buffer.
        // Whether the error is set or not and whether the output
        // is null-terminated or not depends on the version of Windows.
        path_len = GetModuleFileNameW(module, path, path_size);
    } while (path_len && path_size <= INT16_MAX && path_size <= path_len);

    if (!path_len) {
        av_free(path);
        return NULL;
    }
    return path;
}

/**
 * Safe function used to open dynamic libs. This attempts to improve program security
 * by removing the current directory from the dll search path. Only dll's found in the
 * executable or system directory are allowed to be loaded.
 * @param name  The dynamic lib name.
 * @return A handle to the opened lib.
 */
static inline HMODULE win32_dlopen(const char *name)
{
    wchar_t *name_w;
    HMODULE module = NULL;
    if (utf8towchar(name, &name_w))
        name_w = NULL;
#if _WIN32_WINNT < 0x0602
    // On Win7 and earlier we check if KB2533623 is available
    if (!GetProcAddress(GetModuleHandleW(L"kernel32.dll"), "SetDefaultDllDirectories")) {
        wchar_t *path = NULL, *new_path;
        DWORD pathlen, pathsize, namelen;
        if (!name_w)
            goto exit;
        namelen = wcslen(name_w);
        // Try local directory first
        path = get_module_filename(NULL);
        if (!path)
            goto exit;
        new_path = wcsrchr(path, '\\');
        if (!new_path)
            goto exit;
        pathlen = new_path - path;
        pathsize = pathlen + namelen + 2;
        new_path = av_realloc_array(path, pathsize, sizeof *path);
        if (!new_path)
            goto exit;
        path = new_path;
        wcscpy(path + pathlen + 1, name_w);
        module = LoadLibraryExW(path, NULL, LOAD_WITH_ALTERED_SEARCH_PATH);
        if (module == NULL) {
            // Next try System32 directory
            pathlen = GetSystemDirectoryW(path, pathsize);
            if (!pathlen)
                goto exit;
            // Buffer is not enough in two cases:
            // 1. system directory + \ + module name
            // 2. system directory even without the module name.
            if (pathlen + namelen + 2 > pathsize) {
                pathsize = pathlen + namelen + 2;
                new_path = av_realloc_array(path, pathsize, sizeof *path);
                if (!new_path)
                    goto exit;
                path = new_path;
                // Query again to handle the case #2.
                pathlen = GetSystemDirectoryW(path, pathsize);
                if (!pathlen)
                    goto exit;
            }
            path[pathlen] = L'\\';
            wcscpy(path + pathlen + 1, name_w);
            module = LoadLibraryExW(path, NULL, LOAD_WITH_ALTERED_SEARCH_PATH);
        }
exit:
        av_free(path);
        av_free(name_w);
        return module;
    }
#endif
#ifndef LOAD_LIBRARY_SEARCH_APPLICATION_DIR
#   define LOAD_LIBRARY_SEARCH_APPLICATION_DIR 0x00000200
#endif
#ifndef LOAD_LIBRARY_SEARCH_SYSTEM32
#   define LOAD_LIBRARY_SEARCH_SYSTEM32        0x00000800
#endif
#if HAVE_WINRT
    if (!name_w)
        return NULL;
    module = LoadPackagedLibrary(name_w, 0);
#else
#define LOAD_FLAGS (LOAD_LIBRARY_SEARCH_APPLICATION_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32)
    /* filename may be be in CP_ACP */
    if (!name_w)
        return LoadLibraryExA(name, NULL, LOAD_FLAGS);
    module = LoadLibraryExW(name_w, NULL, LOAD_FLAGS);
#undef LOAD_FLAGS
#endif
    av_free(name_w);
    return module;
}
#define dlopen(name, flags) win32_dlopen(name)
#define dlclose FreeLibrary
#define dlsym GetProcAddress
#else
#include <dlfcn.h>
#endif

#endif /* COMPAT_W32DLFCN_H */
