/* Extended cpio format from POSIX.1.
   This file is part of the GNU C Library.
   Copyright (C) 1992-2020 Free Software Foundation, Inc.
   NOTE: The canonical source of this file is maintained with the GNU cpio.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _CPIO_H
#define _CPIO_H 1

/* A cpio archive consists of a sequence of files.
   Each file has a 76 byte header,
   a variable length, NUL terminated filename,
   and variable length file data.
   A header for a filename "TRAILER!!!" indicates the end of the archive.  */

/* All the fields in the header are ISO 646 (approximately ASCII) strings
   of octal numbers, left padded, not NUL terminated.

   Field Name	Length in Bytes	Notes
   c_magic	6		must be "070707"
   c_dev	6
   c_ino	6
   c_mode	6		see below for value
   c_uid	6
   c_gid	6
   c_nlink	6
   c_rdev	6		only valid for chr and blk special files
   c_mtime	11
   c_namesize	6		count includes terminating NUL in pathname
   c_filesize	11		must be 0 for FIFOs and directories  */

/* Value for the field `c_magic'.  */
#define MAGIC	"070707"

/* Values for c_mode, OR'd together: */

#define C_IRUSR		000400
#define C_IWUSR		000200
#define C_IXUSR		000100
#define C_IRGRP		000040
#define C_IWGRP		000020
#define C_IXGRP		000010
#define C_IROTH		000004
#define C_IWOTH		000002
#define C_IXOTH		000001

#define C_ISUID		004000
#define C_ISGID		002000
#define C_ISVTX		001000

#define C_ISBLK		060000
#define C_ISCHR		020000
#define C_ISDIR		040000
#define C_ISFIFO	010000
#define C_ISSOCK	0140000
#define C_ISLNK		0120000
#define C_ISCTG		0110000
#define C_ISREG		0100000

#endif /* cpio.h */
