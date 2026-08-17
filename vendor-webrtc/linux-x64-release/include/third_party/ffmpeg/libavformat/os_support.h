/*
 * various OS-feature replacement utilities
 * copyright (c) 2000, 2001, 2002 Fabrice Bellard
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

#ifndef AVFORMAT_OS_SUPPORT_H
#define AVFORMAT_OS_SUPPORT_H

/**
 * @file
 * miscellaneous OS support macros and functions.
 */

#include "config.h"

#include <sys/stat.h>

#ifdef _WIN32
#if HAVE_DIRECT_H
#include <direct.h>
#endif
#if HAVE_IO_H
#include <io.h>
#endif
#endif

#ifdef _WIN32
#  include <fcntl.h>
#  include <stdint.h>
#  ifdef lseek
#   undef lseek
#  endif
#  define lseek(f,p,w) _lseeki64((f), (p), (w))
#  ifdef stat
#   undef stat
#  endif

#  define stat win32_stat

    /*
     * The POSIX definition for the stat() function uses a struct of the
     * same name (struct stat), that why it takes this extra effort  for
     * redirecting/replacing the stat() function with our own one which
     * is capable to handle long path names on Windows.
     * The struct below roughly follows the POSIX definition. Time values
     * are 64bit, but in cases when _USE_32BIT_TIME_T is defined, they
     * will be set to values no larger than INT32_MAX which corresponds
     * to file times up to the year 2038.
     */
    struct win32_stat
    {
        _dev_t         st_dev;     /* ID of device containing file */
        _ino_t         st_ino;     /* inode number */
        unsigned short st_mode;    /* protection */
        short          st_nlink;   /* number of hard links */
        short          st_uid;     /* user ID of owner */
        short          st_gid;     /* group ID of owner */
        _dev_t         st_rdev;    /* device ID (if special file) */
        int64_t        st_size;    /* total size, in bytes */
        int64_t        st_atime;   /* time of last access */
        int64_t        st_mtime;   /* time of last modification */
        int64_t        st_ctime;   /* time of last status change */
    };

#  ifdef fstat
#   undef fstat
#  endif
#  define fstat win32_fstat
#endif /* defined(_WIN32) */


#ifdef __ANDROID__
#  if HAVE_UNISTD_H
#    include <unistd.h>
#  endif
#  ifdef lseek
#   undef lseek
#  endif
#  define lseek(f,p,w) lseek64((f), (p), (w))
#endif

static inline int is_dos_path(const char *path)
{
#if HAVE_DOS_PATHS
    if (path[0] && path[1] == ':')
        return 1;
#endif
    return 0;
}

#if defined(_WIN32)
#ifndef S_IRUSR
#define S_IRUSR S_IREAD
#endif
#ifndef S_IWUSR
#define S_IWUSR S_IWRITE
#endif
#endif

#if CONFIG_NETWORK
#if defined(_WIN32)
#define SHUT_RD SD_RECEIVE
#define SHUT_WR SD_SEND
#define SHUT_RDWR SD_BOTH
#else
#include <sys/socket.h>
#if !defined(SHUT_RD) /* OS/2, DJGPP */
#define SHUT_RD 0
#define SHUT_WR 1
#define SHUT_RDWR 2
#endif
#endif

#if !HAVE_SOCKLEN_T
typedef int socklen_t;
#endif

/* most of the time closing a socket is just closing an fd */
#if !HAVE_CLOSESOCKET
#define closesocket close
#endif

#if !HAVE_POLL_H
typedef unsigned long nfds_t;

#if HAVE_WINSOCK2_H
#include <winsock2.h>
#endif
#if !HAVE_STRUCT_POLLFD
struct pollfd {
    int fd;
    short events;  /* events to look for */
    short revents; /* events that occurred */
};

/* events & revents */
#define POLLIN     0x0001  /* any readable data available */
#define POLLOUT    0x0002  /* file descriptor is writeable */
#define POLLRDNORM POLLIN
#define POLLWRNORM POLLOUT
#define POLLRDBAND 0x0008  /* priority readable data */
#define POLLWRBAND 0x0010  /* priority data can be written */
#define POLLPRI    0x0020  /* high priority readable data */

/* revents only */
#define POLLERR    0x0004  /* errors pending */
#define POLLHUP    0x0080  /* disconnected */
#define POLLNVAL   0x1000  /* invalid file descriptor */
#endif


int ff_poll(struct pollfd *fds, nfds_t numfds, int timeout);
#define poll ff_poll
#endif /* HAVE_POLL_H */
#endif /* CONFIG_NETWORK */

#ifdef _WIN32
#include <stdio.h>
#include <windows.h>
#include "libavutil/mem.h"
#include "libavutil/wchar_filename.h"

#define DEF_FS_FUNCTION(name, wfunc, afunc)               \
static inline int win32_##name(const char *filename_utf8) \
{                                                         \
    wchar_t *filename_w;                                  \
    int ret;                                              \
                                                          \
    if (get_extended_win32_path(filename_utf8, &filename_w)) \
        return -1;                                        \
    if (!filename_w)                                      \
        goto fallback;                                    \
                                                          \
    ret = wfunc(filename_w);                              \
    av_free(filename_w);                                  \
    return ret;                                           \
                                                          \
fallback:                                                 \
    /* filename may be be in CP_ACP */                    \
    return afunc(filename_utf8);                          \
}

DEF_FS_FUNCTION(unlink, _wunlink, _unlink)
DEF_FS_FUNCTION(mkdir,  _wmkdir,  _mkdir)
DEF_FS_FUNCTION(rmdir,  _wrmdir , _rmdir)

static inline int win32_access(const char *filename_utf8, int mode)
{
    wchar_t *filename_w;
    int ret;
    if (get_extended_win32_path(filename_utf8, &filename_w))
        return -1;
    if (!filename_w)
        goto fallback;
    ret = _waccess(filename_w, mode);
    av_free(filename_w);
    return ret;
fallback:
    return _access(filename_utf8, mode);
}

static inline void copy_stat(struct _stat64 *crtstat, struct win32_stat *buf)
{
    buf->st_dev   = crtstat->st_dev;
    buf->st_ino   = crtstat->st_ino;
    buf->st_mode  = crtstat->st_mode;
    buf->st_nlink = crtstat->st_nlink;
    buf->st_uid   = crtstat->st_uid;
    buf->st_gid   = crtstat->st_gid;
    buf->st_rdev  = crtstat->st_rdev;
    buf->st_size  = crtstat->st_size;
    buf->st_atime = crtstat->st_atime;
    buf->st_mtime = crtstat->st_mtime;
    buf->st_ctime = crtstat->st_ctime;
}

static inline int win32_stat(const char *filename_utf8, struct win32_stat *buf)
{
    struct _stat64 crtstat = { 0 };
    wchar_t *filename_w;
    int ret;

    if (get_extended_win32_path(filename_utf8, &filename_w))
        return -1;

    if (filename_w) {
        ret = _wstat64(filename_w, &crtstat);
        av_free(filename_w);
    } else
        ret = _stat64(filename_utf8, &crtstat);

    copy_stat(&crtstat, buf);

    return ret;
}

static inline int win32_fstat(int fd, struct win32_stat *buf)
{
    struct _stat64 crtstat = { 0 };
    int ret;

    ret = _fstat64(fd, &crtstat);

    copy_stat(&crtstat, buf);

    return ret;
}

static inline int win32_rename(const char *src_utf8, const char *dest_utf8)
{
    wchar_t *src_w, *dest_w;
    int ret;

    if (get_extended_win32_path(src_utf8, &src_w))
        return -1;
    if (get_extended_win32_path(dest_utf8, &dest_w)) {
        av_free(src_w);
        return -1;
    }
    if (!src_w || !dest_w) {
        av_free(src_w);
        av_free(dest_w);
        goto fallback;
    }

    ret = MoveFileExW(src_w, dest_w, MOVEFILE_REPLACE_EXISTING);
    av_free(src_w);
    av_free(dest_w);
    // Lacking proper mapping from GetLastError() error codes to errno codes
    if (ret)
        errno = EPERM;
    return ret;

fallback:
    /* filename may be be in CP_ACP */
#if !HAVE_UWP
    ret = MoveFileExA(src_utf8, dest_utf8, MOVEFILE_REPLACE_EXISTING);
    if (ret)
        errno = EPERM;
#else
    /* Windows Phone doesn't have MoveFileExA, and for Windows Store apps,
     * it is available but not allowed by the app certification kit. However,
     * it's unlikely that anybody would input filenames in CP_ACP there, so this
     * fallback is kept mostly for completeness. Alternatively we could
     * do MultiByteToWideChar(CP_ACP) and use MoveFileExW, but doing
     * explicit conversions with CP_ACP is allegedly forbidden in windows
     * store apps (or windows phone), and the notion of a native code page
     * doesn't make much sense there. */
    ret = rename(src_utf8, dest_utf8);
#endif
    return ret;
}

#define mkdir(a, b) win32_mkdir(a)
#define rename      win32_rename
#define rmdir       win32_rmdir
#define unlink      win32_unlink
#define access      win32_access

#endif

#endif /* AVFORMAT_OS_SUPPORT_H */
