/*
#             (C) 2008 Hans de Goede <hdegoede@redhat.com>

# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation; either version 2.1 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Suite 500, Boston, MA  02110-1335  USA
*/

#ifndef __LIBV4L2_H
#define __LIBV4L2_H

#include <stdio.h>
#include <unistd.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#if HAVE_VISIBILITY
#define LIBV4L_PUBLIC __attribute__ ((visibility("default")))
#else
#define LIBV4L_PUBLIC
#endif

/* Point this to a FILE opened for writing when you want to log error and
   status messages to a file, when NULL errors will get send to stderr */
LIBV4L_PUBLIC extern FILE *v4l2_log_file;

/* Just like your regular open/close/etc, except that format conversion is
   done if necessary when capturing. That is if you (try to) set a capture
   format which is not supported by the cam, but is supported by libv4lconvert,
   then the try_fmt / set_fmt will succeed as if the cam supports the format
   and on dqbuf / read the data will be converted for you and returned in
   the request format. enum_fmt will also report support for the formats to
   which conversion is possible.

   Another difference is that you can make v4l2_read() calls even on devices
   which do not support the regular read() method.

   Note the device name passed to v4l2_open must be of a video4linux2 device,
   if it is anything else (including a video4linux1 device), v4l2_open will
   fail.

   Note that the argument to v4l2_ioctl after the request must be a valid
   memory address of structure of the appropriate type for the request (for
   v4l2 requests which expect a structure address). Passing in NULL or an
   invalid memory address will not lead to failure with errno being EFAULT,
   as it would with a real ioctl, but will cause libv4l2 to break, and you
   get to keep both pieces.
*/

LIBV4L_PUBLIC int v4l2_open(const char *file, int oflag, ...);
LIBV4L_PUBLIC int v4l2_close(int fd);
LIBV4L_PUBLIC int v4l2_dup(int fd);
LIBV4L_PUBLIC int v4l2_ioctl(int fd, unsigned long int request, ...);
LIBV4L_PUBLIC ssize_t v4l2_read(int fd, void *buffer, size_t n);
LIBV4L_PUBLIC ssize_t v4l2_write(int fd, const void *buffer, size_t n);
LIBV4L_PUBLIC void *v4l2_mmap(void *start, size_t length, int prot, int flags,
		int fd, int64_t offset);
LIBV4L_PUBLIC int v4l2_munmap(void *_start, size_t length);


/* Misc utility functions */

/* This function takes a value of 0 - 65535, and then scales that range to
   the actual range of the given v4l control id, and then if the cid exists
   and is not locked sets the cid to the scaled value.

   Normally returns 0, even if the cid did not exist or was locked, returns
   non 0 when an other error occured. */
LIBV4L_PUBLIC int v4l2_set_control(int fd, int cid, int value);

/* This function returns a value of 0 - 65535, scaled to from the actual range
   of the given v4l control id. When the cid does not exist, or could not be
   accessed -1 is returned. */
LIBV4L_PUBLIC int v4l2_get_control(int fd, int cid);


/* "low level" access functions, these functions allow somewhat lower level
   access to libv4l2 (currently there only is v4l2_fd_open here) */

/* Flags for v4l2_fd_open's v4l2_flags argument */

/* Disable all format conversion done by libv4l2, this includes the software
   whitebalance, gamma correction, flipping, etc. libv4lconvert does. Use this
   if you want raw frame data, but still want the additional error checks and
   the read() emulation libv4l2 offers. */
#define V4L2_DISABLE_CONVERSION 0x01
/* This flag is *OBSOLETE*, since version 0.5.98 libv4l *always* reports
   emulated formats to ENUM_FMT, except when conversion is disabled. */
#define V4L2_ENABLE_ENUM_FMT_EMULATION 0x02

/* v4l2_fd_open: open an already opened fd for further use through
   v4l2lib and possibly modify libv4l2's default behavior through the
   v4l2_flags argument.

   Returns fd on success, -1 if the fd is not suitable for use through libv4l2
   (note the fd is left open in this case). */
LIBV4L_PUBLIC int v4l2_fd_open(int fd, int v4l2_flags);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif
