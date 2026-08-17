/* File tree traversal functions declarations.
   Copyright (C) 1994-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

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

/*
 * Copyright (c) 1989, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)fts.h	8.3 (Berkeley) 8/14/94
 */

#ifndef	_FTS_H
#define	_FTS_H 1

#include <features.h>
#include <sys/types.h>


typedef struct {
	struct _ftsent *fts_cur;	/* current node */
	struct _ftsent *fts_child;	/* linked list of children */
	struct _ftsent **fts_array;	/* sort array */
	dev_t fts_dev;			/* starting device # */
	char *fts_path;			/* path for this descent */
	int fts_rfd;			/* fd for root */
	int fts_pathlen;		/* sizeof(path) */
	int fts_nitems;			/* elements in the sort array */
	int (*fts_compar) (const void *, const void *); /* compare fn */

#define	FTS_COMFOLLOW	0x0001		/* follow command line symlinks */
#define	FTS_LOGICAL	0x0002		/* logical walk */
#define	FTS_NOCHDIR	0x0004		/* don't change directories */
#define	FTS_NOSTAT	0x0008		/* don't get stat info */
#define	FTS_PHYSICAL	0x0010		/* physical walk */
#define	FTS_SEEDOT	0x0020		/* return dot and dot-dot */
#define	FTS_XDEV	0x0040		/* don't cross devices */
#define FTS_WHITEOUT	0x0080		/* return whiteout information */
#define	FTS_OPTIONMASK	0x00ff		/* valid user option mask */

#define	FTS_NAMEONLY	0x0100		/* (private) child names only */
#define	FTS_STOP	0x0200		/* (private) unrecoverable error */
	int fts_options;		/* fts_open options, global flags */
} FTS;

#ifdef __USE_LARGEFILE64
typedef struct {
	struct _ftsent64 *fts_cur;	/* current node */
	struct _ftsent64 *fts_child;	/* linked list of children */
	struct _ftsent64 **fts_array;	/* sort array */
	dev_t fts_dev;			/* starting device # */
	char *fts_path;			/* path for this descent */
	int fts_rfd;			/* fd for root */
	int fts_pathlen;		/* sizeof(path) */
	int fts_nitems;			/* elements in the sort array */
	int (*fts_compar) (const void *, const void *); /* compare fn */
	int fts_options;		/* fts_open options, global flags */
} FTS64;
#endif

typedef struct _ftsent {
	struct _ftsent *fts_cycle;	/* cycle node */
	struct _ftsent *fts_parent;	/* parent directory */
	struct _ftsent *fts_link;	/* next file in directory */
	long fts_number;	        /* local numeric value */
	void *fts_pointer;	        /* local address value */
	char *fts_accpath;		/* access path */
	char *fts_path;			/* root path */
	int fts_errno;			/* errno for this node */
	int fts_symfd;			/* fd for symlink */
	unsigned short fts_pathlen;	/* strlen(fts_path) */
	unsigned short fts_namelen;	/* strlen(fts_name) */

	ino_t fts_ino;			/* inode */
	dev_t fts_dev;			/* device */
	nlink_t fts_nlink;		/* link count */

#define	FTS_ROOTPARENTLEVEL	-1
#define	FTS_ROOTLEVEL		 0
	short fts_level;		/* depth (-1 to N) */

#define	FTS_D		 1		/* preorder directory */
#define	FTS_DC		 2		/* directory that causes cycles */
#define	FTS_DEFAULT	 3		/* none of the above */
#define	FTS_DNR		 4		/* unreadable directory */
#define	FTS_DOT		 5		/* dot or dot-dot */
#define	FTS_DP		 6		/* postorder directory */
#define	FTS_ERR		 7		/* error; errno is set */
#define	FTS_F		 8		/* regular file */
#define	FTS_INIT	 9		/* initialized only */
#define	FTS_NS		10		/* stat(2) failed */
#define	FTS_NSOK	11		/* no stat(2) requested */
#define	FTS_SL		12		/* symbolic link */
#define	FTS_SLNONE	13		/* symbolic link without target */
#define FTS_W		14		/* whiteout object */
	unsigned short fts_info;	/* user flags for FTSENT structure */

#define	FTS_DONTCHDIR	 0x01		/* don't chdir .. to the parent */
#define	FTS_SYMFOLLOW	 0x02		/* followed a symlink to get here */
	unsigned short fts_flags;	/* private flags for FTSENT structure */

#define	FTS_AGAIN	 1		/* read node again */
#define	FTS_FOLLOW	 2		/* follow symbolic link */
#define	FTS_NOINSTR	 3		/* no instructions */
#define	FTS_SKIP	 4		/* discard node */
	unsigned short fts_instr;	/* fts_set() instructions */

	struct stat *fts_statp;		/* stat(2) information */
	char fts_name[1];		/* file name */
} FTSENT;

#ifdef __USE_LARGEFILE64
typedef struct _ftsent64 {
	struct _ftsent64 *fts_cycle;	/* cycle node */
	struct _ftsent64 *fts_parent;	/* parent directory */
	struct _ftsent64 *fts_link;	/* next file in directory */
	long fts_number;	        /* local numeric value */
	void *fts_pointer;	        /* local address value */
	char *fts_accpath;		/* access path */
	char *fts_path;			/* root path */
	int fts_errno;			/* errno for this node */
	int fts_symfd;			/* fd for symlink */
	unsigned short fts_pathlen;		/* strlen(fts_path) */
	unsigned short fts_namelen;		/* strlen(fts_name) */

	ino64_t fts_ino;		/* inode */
	dev_t fts_dev;			/* device */
	nlink_t fts_nlink;		/* link count */

	short fts_level;		/* depth (-1 to N) */

	unsigned short fts_info;	/* user flags for FTSENT structure */

	unsigned short fts_flags;	/* private flags for FTSENT structure */

	unsigned short fts_instr;	/* fts_set() instructions */

	struct stat64 *fts_statp;	/* stat(2) information */
	char fts_name[1];		/* file name */
} FTSENT64;
#endif

__BEGIN_DECLS
#ifndef __USE_FILE_OFFSET64
FTSENT	*fts_children (FTS *, int);
int	 fts_close (FTS *);
FTS	*fts_open (char * const *, int,
		   int (*)(const FTSENT **, const FTSENT **));
FTSENT	*fts_read (FTS *);
int	 fts_set (FTS *, FTSENT *, int) __THROW;
#else
# ifdef __REDIRECT
FTSENT	*__REDIRECT (fts_children, (FTS *, int), fts64_children);
int	 __REDIRECT (fts_close, (FTS *), fts64_close);
FTS	*__REDIRECT (fts_open, (char * const *, int,
				int (*)(const FTSENT **, const FTSENT **)),
		     fts64_open);
FTSENT	*__REDIRECT (fts_read, (FTS *), fts64_read);
int	 __REDIRECT_NTH (fts_set, (FTS *, FTSENT *, int), fts64_set);
# else
#  define fts_children fts64_children
#  define fts_close fts64_close
#  define fts_open fts64_open
#  define fts_read fts64_read
#  define fts_set fts64_set
# endif
#endif
#ifdef __USE_LARGEFILE64
FTSENT64 *fts64_children (FTS64 *, int);
int	  fts64_close (FTS64 *);
FTS64	 *fts64_open (char * const *, int,
		      int (*)(const FTSENT64 **, const FTSENT64 **));
FTSENT64 *fts64_read (FTS64 *);
int	 fts64_set (FTS64 *, FTSENT64 *, int) __THROW;
#endif
__END_DECLS

#endif /* fts.h */
