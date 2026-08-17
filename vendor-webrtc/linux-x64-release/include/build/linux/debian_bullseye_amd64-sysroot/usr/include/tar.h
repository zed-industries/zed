/* Extended tar format from POSIX.1.
   Copyright (C) 1992-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Written by David J. MacKenzie.

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

#ifndef	_TAR_H
#define	_TAR_H	1

#include <features.h>


/* A tar archive consists of 512-byte blocks.
   Each file in the archive has a header block followed by 0+ data blocks.
   Two blocks of NUL bytes indicate the end of the archive.  */

/* The fields of header blocks:
   All strings are stored as ISO 646 (approximately ASCII) strings.

   Fields are numeric unless otherwise noted below; numbers are ISO 646
   representations of octal numbers, with leading zeros as needed.

   linkname is only valid when typeflag==LNKTYPE.  It doesn't use prefix;
   files that are links to pathnames >100 chars long can not be stored
   in a tar archive.

   If typeflag=={LNKTYPE,SYMTYPE,DIRTYPE} then size must be 0.

   devmajor and devminor are only valid for typeflag=={BLKTYPE,CHRTYPE}.

   chksum contains the sum of all 512 bytes in the header block,
   treating each byte as an 8-bit unsigned value and treating the
   8 bytes of chksum as blank characters.

   uname and gname are used in preference to uid and gid, if those
   names exist locally.

   Field Name	Byte Offset	Length in Bytes	Field Type
   name		0		100		NUL-terminated if NUL fits
   mode		100		8
   uid		108		8
   gid		116		8
   size		124		12
   mtime	136		12
   chksum	148		8
   typeflag	156		1		see below
   linkname	157		100		NUL-terminated if NUL fits
   magic	257		6		must be TMAGIC (NUL term.)
   version	263		2		must be TVERSION
   uname	265		32		NUL-terminated
   gname	297		32		NUL-terminated
   devmajor	329		8
   devminor	337		8
   prefix	345		155		NUL-terminated if NUL fits

   If the first character of prefix is '\0', the file name is name;
   otherwise, it is prefix/name.  Files whose pathnames don't fit in that
   length can not be stored in a tar archive.  */

/* The bits in mode: */
#define TSUID	04000
#define TSGID	02000
#if defined __USE_XOPEN || !defined __USE_XOPEN2K
# define TSVTX	01000
#endif
#define TUREAD	00400
#define TUWRITE	00200
#define TUEXEC	00100
#define TGREAD	00040
#define TGWRITE	00020
#define TGEXEC	00010
#define TOREAD	00004
#define TOWRITE	00002
#define TOEXEC	00001

/* The values for typeflag:
   Values 'A'-'Z' are reserved for custom implementations.
   All other values are reserved for future POSIX.1 revisions.  */

#define REGTYPE		'0'	/* Regular file (preferred code).  */
#define AREGTYPE	'\0'	/* Regular file (alternate code).  */
#define LNKTYPE		'1'	/* Hard link.  */
#define SYMTYPE		'2'	/* Symbolic link (hard if not supported).  */
#define CHRTYPE		'3'	/* Character special.  */
#define BLKTYPE		'4'	/* Block special.  */
#define DIRTYPE		'5'	/* Directory.  */
#define FIFOTYPE	'6'	/* Named pipe.  */
#define CONTTYPE	'7'	/* Contiguous file */
 /* (regular file if not supported).  */

/* Contents of magic field and its length.  */
#define TMAGIC	"ustar"
#define TMAGLEN	6

/* Contents of the version field and its length.  */
#define TVERSION	"00"
#define TVERSLEN	2

#endif /* tar.h */
