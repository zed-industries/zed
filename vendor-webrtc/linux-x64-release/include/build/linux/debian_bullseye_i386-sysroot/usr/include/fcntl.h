/* Copyright (C) 1991-2020 Free Software Foundation, Inc.
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
 *	POSIX Standard: 6.5 File Control Operations	<fcntl.h>
 */

#ifndef	_FCNTL_H
#define	_FCNTL_H	1

#include <features.h>

/* This must be early so <bits/fcntl.h> can define types winningly.  */
__BEGIN_DECLS

/* Get __mode_t, __dev_t and __off_t  .*/
#include <bits/types.h>

/* Get the definitions of O_*, F_*, FD_*: all the
   numbers and flag bits for `open', `fcntl', et al.  */
#include <bits/fcntl.h>

/* Detect if open needs mode as a third argument (or for openat as a fourth
   argument).  */
#ifdef __O_TMPFILE
# define __OPEN_NEEDS_MODE(oflag) \
  (((oflag) & O_CREAT) != 0 || ((oflag) & __O_TMPFILE) == __O_TMPFILE)
#else
# define __OPEN_NEEDS_MODE(oflag) (((oflag) & O_CREAT) != 0)
#endif

/* POSIX.1-2001 specifies that these types are defined by <fcntl.h>.
   Earlier POSIX standards permitted any type ending in `_t' to be defined
   by any POSIX header, so we don't conditionalize the definitions here.  */
#ifndef __mode_t_defined
typedef __mode_t mode_t;
# define __mode_t_defined
#endif

#ifndef __off_t_defined
# ifndef __USE_FILE_OFFSET64
typedef __off_t off_t;
# else
typedef __off64_t off_t;
# endif
# define __off_t_defined
#endif

#if defined __USE_LARGEFILE64 && !defined __off64_t_defined
typedef __off64_t off64_t;
# define __off64_t_defined
#endif

#ifndef __pid_t_defined
typedef __pid_t pid_t;
# define __pid_t_defined
#endif

/* For XPG all symbols from <sys/stat.h> should also be available.  */
#ifdef __USE_XOPEN2K8
# include <bits/types/struct_timespec.h>
#endif
#if defined __USE_XOPEN || defined __USE_XOPEN2K8
# include <bits/stat.h>

# define S_IFMT		__S_IFMT
# define S_IFDIR	__S_IFDIR
# define S_IFCHR	__S_IFCHR
# define S_IFBLK	__S_IFBLK
# define S_IFREG	__S_IFREG
# ifdef __S_IFIFO
#  define S_IFIFO	__S_IFIFO
# endif
# ifdef __S_IFLNK
#  define S_IFLNK	__S_IFLNK
# endif
# if (defined __USE_UNIX98 || defined __USE_XOPEN2K8) && defined __S_IFSOCK
#  define S_IFSOCK	__S_IFSOCK
# endif

/* Protection bits.  */

# define S_ISUID	__S_ISUID       /* Set user ID on execution.  */
# define S_ISGID	__S_ISGID       /* Set group ID on execution.  */

# if defined __USE_MISC || defined __USE_XOPEN
/* Save swapped text after use (sticky bit).  This is pretty well obsolete.  */
#  define S_ISVTX	__S_ISVTX
# endif

# define S_IRUSR	__S_IREAD       /* Read by owner.  */
# define S_IWUSR	__S_IWRITE      /* Write by owner.  */
# define S_IXUSR	__S_IEXEC       /* Execute by owner.  */
/* Read, write, and execute by owner.  */
# define S_IRWXU	(__S_IREAD|__S_IWRITE|__S_IEXEC)

# define S_IRGRP	(S_IRUSR >> 3)  /* Read by group.  */
# define S_IWGRP	(S_IWUSR >> 3)  /* Write by group.  */
# define S_IXGRP	(S_IXUSR >> 3)  /* Execute by group.  */
/* Read, write, and execute by group.  */
# define S_IRWXG	(S_IRWXU >> 3)

# define S_IROTH	(S_IRGRP >> 3)  /* Read by others.  */
# define S_IWOTH	(S_IWGRP >> 3)  /* Write by others.  */
# define S_IXOTH	(S_IXGRP >> 3)  /* Execute by others.  */
/* Read, write, and execute by others.  */
# define S_IRWXO	(S_IRWXG >> 3)
#endif

#ifdef	__USE_MISC
# ifndef R_OK			/* Verbatim from <unistd.h>.  Ugh.  */
/* Values for the second argument to access.
   These may be OR'd together.  */
#  define R_OK	4		/* Test for read permission.  */
#  define W_OK	2		/* Test for write permission.  */
#  define X_OK	1		/* Test for execute permission.  */
#  define F_OK	0		/* Test for existence.  */
# endif
#endif /* Use misc.  */

/* XPG wants the following symbols.   <stdio.h> has the same definitions.  */
#if defined __USE_XOPEN || defined __USE_XOPEN2K8
# define SEEK_SET	0	/* Seek from beginning of file.  */
# define SEEK_CUR	1	/* Seek from current position.  */
# define SEEK_END	2	/* Seek from end of file.  */
#endif	/* XPG */

/* Do the file control operation described by CMD on FD.
   The remaining arguments are interpreted depending on CMD.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
#if 1
extern int fcntl (int __fd, int __cmd, ...);
#else
# ifdef __REDIRECT
extern int __REDIRECT (fcntl, (int __fd, int __cmd, ...), fcntl64);
# else
#  define fcntl fcntl64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int fcntl64 (int __fd, int __cmd, ...);
#endif

/* Open FILE and return a new file descriptor for it, or -1 on error.
   OFLAG determines the type of access used.  If O_CREAT or O_TMPFILE is set
   in OFLAG, the third argument is taken as a `mode_t', the mode of the
   created file.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
#ifndef __USE_FILE_OFFSET64
extern int open (const char *__file, int __oflag, ...) __nonnull ((1));
#else
# ifdef __REDIRECT
extern int __REDIRECT (open, (const char *__file, int __oflag, ...), open64)
     __nonnull ((1));
# else
#  define open open64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int open64 (const char *__file, int __oflag, ...) __nonnull ((1));
#endif

#ifdef __USE_ATFILE
/* Similar to `open' but a relative path name is interpreted relative to
   the directory for which FD is a descriptor.

   NOTE: some other `openat' implementation support additional functionality
   through this interface, especially using the O_XATTR flag.  This is not
   yet supported here.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
# ifndef __USE_FILE_OFFSET64
extern int openat (int __fd, const char *__file, int __oflag, ...)
     __nonnull ((2));
# else
#  ifdef __REDIRECT
extern int __REDIRECT (openat, (int __fd, const char *__file, int __oflag,
				...), openat64) __nonnull ((2));
#  else
#   define openat openat64
#  endif
# endif
# ifdef __USE_LARGEFILE64
extern int openat64 (int __fd, const char *__file, int __oflag, ...)
     __nonnull ((2));
# endif
#endif

/* Create and open FILE, with mode MODE.  This takes an `int' MODE
   argument because that is what `mode_t' will be widened to.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
#ifndef __USE_FILE_OFFSET64
extern int creat (const char *__file, mode_t __mode) __nonnull ((1));
#else
# ifdef __REDIRECT
extern int __REDIRECT (creat, (const char *__file, mode_t __mode),
		       creat64) __nonnull ((1));
# else
#  define creat creat64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int creat64 (const char *__file, mode_t __mode) __nonnull ((1));
#endif

#if !defined F_LOCK && (defined __USE_MISC || (defined __USE_XOPEN_EXTENDED \
					       && !defined __USE_POSIX))
/* NOTE: These declarations also appear in <unistd.h>; be sure to keep both
   files consistent.  Some systems have them there and some here, and some
   software depends on the macros being defined without including both.  */

/* `lockf' is a simpler interface to the locking facilities of `fcntl'.
   LEN is always relative to the current file position.
   The CMD argument is one of the following.  */

# define F_ULOCK 0	/* Unlock a previously locked region.  */
# define F_LOCK  1	/* Lock a region for exclusive use.  */
# define F_TLOCK 2	/* Test and lock a region for exclusive use.  */
# define F_TEST  3	/* Test a region for other processes locks.  */

# ifndef __USE_FILE_OFFSET64
extern int lockf (int __fd, int __cmd, off_t __len);
# else
#  ifdef __REDIRECT
extern int __REDIRECT (lockf, (int __fd, int __cmd, __off64_t __len), lockf64);
#  else
#   define lockf lockf64
#  endif
# endif
# ifdef __USE_LARGEFILE64
extern int lockf64 (int __fd, int __cmd, off64_t __len);
# endif
#endif

#ifdef __USE_XOPEN2K
/* Advice the system about the expected behaviour of the application with
   respect to the file associated with FD.  */
# ifndef __USE_FILE_OFFSET64
extern int posix_fadvise (int __fd, off_t __offset, off_t __len,
			  int __advise) __THROW;
# else
 # ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (posix_fadvise, (int __fd, __off64_t __offset,
					   __off64_t __len, int __advise),
			   posix_fadvise64);
#  else
#   define posix_fadvise posix_fadvise64
#  endif
# endif
# ifdef __USE_LARGEFILE64
extern int posix_fadvise64 (int __fd, off64_t __offset, off64_t __len,
			    int __advise) __THROW;
# endif


/* Reserve storage for the data of the file associated with FD.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
# ifndef __USE_FILE_OFFSET64
extern int posix_fallocate (int __fd, off_t __offset, off_t __len);
# else
 # ifdef __REDIRECT
extern int __REDIRECT (posix_fallocate, (int __fd, __off64_t __offset,
					 __off64_t __len),
		       posix_fallocate64);
#  else
#   define posix_fallocate posix_fallocate64
#  endif
# endif
# ifdef __USE_LARGEFILE64
extern int posix_fallocate64 (int __fd, off64_t __offset, off64_t __len);
# endif
#endif


/* Define some inlines helping to catch common problems.  */
#if __USE_FORTIFY_LEVEL > 0 && defined __fortify_function \
    && defined __va_arg_pack_len
# include <bits/fcntl2.h>
#endif

__END_DECLS

#endif /* fcntl.h  */
