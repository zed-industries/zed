/* O_*, F_*, FD_* bit values for Linux.
   Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

#ifndef	_FCNTL_H
# error "Never use <bits/fcntl-linux.h> directly; include <fcntl.h> instead."
#endif

/* This file contains shared definitions between Linux architectures
   and is included by <bits/fcntl.h> to declare them.  The various
   #ifndef cases allow the architecture specific file to define those
   values with different values.

   A minimal <bits/fcntl.h> contains just:

   struct flock {...}
   #ifdef __USE_LARGEFILE64
   struct flock64 {...}
   #endif
   #include <bits/fcntl-linux.h>
*/

#ifdef __USE_GNU
# include <bits/types/struct_iovec.h>
#endif

/* open/fcntl.  */
#define O_ACCMODE	   0003
#define O_RDONLY	     00
#define O_WRONLY	     01
#define O_RDWR		     02
#ifndef O_CREAT
# define O_CREAT	   0100	/* Not fcntl.  */
#endif
#ifndef O_EXCL
# define O_EXCL		   0200	/* Not fcntl.  */
#endif
#ifndef O_NOCTTY
# define O_NOCTTY	   0400	/* Not fcntl.  */
#endif
#ifndef O_TRUNC
# define O_TRUNC	  01000	/* Not fcntl.  */
#endif
#ifndef O_APPEND
# define O_APPEND	  02000
#endif
#ifndef O_NONBLOCK
# define O_NONBLOCK	  04000
#endif
#ifndef O_NDELAY
# define O_NDELAY	O_NONBLOCK
#endif
#ifndef O_SYNC
# define O_SYNC	       04010000
#endif
#define O_FSYNC		O_SYNC
#ifndef O_ASYNC
# define O_ASYNC	 020000
#endif
#ifndef __O_LARGEFILE
# define __O_LARGEFILE	0100000
#endif

#ifndef __O_DIRECTORY
# define __O_DIRECTORY	0200000
#endif
#ifndef __O_NOFOLLOW
# define __O_NOFOLLOW	0400000
#endif
#ifndef __O_CLOEXEC
# define __O_CLOEXEC   02000000
#endif
#ifndef __O_DIRECT
# define __O_DIRECT	 040000
#endif
#ifndef __O_NOATIME
# define __O_NOATIME   01000000
#endif
#ifndef __O_PATH
# define __O_PATH     010000000
#endif
#ifndef __O_DSYNC
# define __O_DSYNC	 010000
#endif
#ifndef __O_TMPFILE
# define __O_TMPFILE   (020000000 | __O_DIRECTORY)
#endif

#ifndef F_GETLK
# ifndef __USE_FILE_OFFSET64
#  define F_GETLK	5	/* Get record locking info.  */
#  define F_SETLK	6	/* Set record locking info (non-blocking).  */
#  define F_SETLKW	7	/* Set record locking info (blocking).  */
# else
#  define F_GETLK	F_GETLK64  /* Get record locking info.  */
#  define F_SETLK	F_SETLK64  /* Set record locking info (non-blocking).*/
#  define F_SETLKW	F_SETLKW64 /* Set record locking info (blocking).  */
# endif
#endif
#ifndef F_GETLK64
# define F_GETLK64	12	/* Get record locking info.  */
# define F_SETLK64	13	/* Set record locking info (non-blocking).  */
# define F_SETLKW64	14	/* Set record locking info (blocking).  */
#endif

/* open file description locks.

   Usually record locks held by a process are released on *any* close and are
   not inherited across a fork.

   These cmd values will set locks that conflict with process-associated record
   locks, but are "owned" by the opened file description, not the process.
   This means that they are inherited across fork or clone with CLONE_FILES
   like BSD (flock) locks, and they are only released automatically when the
   last reference to the the file description against which they were acquired
   is put. */
#ifdef __USE_GNU
# define F_OFD_GETLK	36
# define F_OFD_SETLK	37
# define F_OFD_SETLKW	38
#endif

#ifdef __USE_LARGEFILE64
# define O_LARGEFILE __O_LARGEFILE
#endif

#ifdef __USE_XOPEN2K8
# define O_DIRECTORY	__O_DIRECTORY	/* Must be a directory.  */
# define O_NOFOLLOW	__O_NOFOLLOW	/* Do not follow links.  */
# define O_CLOEXEC	__O_CLOEXEC	/* Set close_on_exec.  */
#endif

#ifdef __USE_GNU
# define O_DIRECT	__O_DIRECT	/* Direct disk access.  */
# define O_NOATIME	__O_NOATIME	/* Do not set atime.  */
# define O_PATH		__O_PATH	/* Resolve pathname but do not open file.  */
# define O_TMPFILE	__O_TMPFILE	/* Atomically create nameless file.  */
#endif

/* For now, Linux has no separate synchronicity options for read
   operations.  We define O_RSYNC therefore as the same as O_SYNC
   since this is a superset.  */
#if defined __USE_POSIX199309 || defined __USE_UNIX98
# define O_DSYNC	__O_DSYNC	/* Synchronize data.  */
# if defined __O_RSYNC
#  define O_RSYNC	__O_RSYNC	/* Synchronize read operations.  */
# else
#  define O_RSYNC	O_SYNC		/* Synchronize read operations.  */
# endif
#endif

/* Values for the second argument to `fcntl'.  */
#define F_DUPFD		0	/* Duplicate file descriptor.  */
#define F_GETFD		1	/* Get file descriptor flags.  */
#define F_SETFD		2	/* Set file descriptor flags.  */
#define F_GETFL		3	/* Get file status flags.  */
#define F_SETFL		4	/* Set file status flags.  */

#ifndef __F_SETOWN
# define __F_SETOWN	8
# define __F_GETOWN	9
#endif

#if defined __USE_UNIX98 || defined __USE_XOPEN2K8
# define F_SETOWN	__F_SETOWN /* Get owner (process receiving SIGIO).  */
# define F_GETOWN	__F_GETOWN /* Set owner (process receiving SIGIO).  */
#endif

#ifndef __F_SETSIG
# define __F_SETSIG	10	/* Set number of signal to be sent.  */
# define __F_GETSIG	11	/* Get number of signal to be sent.  */
#endif
#ifndef __F_SETOWN_EX
# define __F_SETOWN_EX	15	/* Get owner (thread receiving SIGIO).  */
# define __F_GETOWN_EX	16	/* Set owner (thread receiving SIGIO).  */
#endif

#ifdef __USE_GNU
# define F_SETSIG	__F_SETSIG	/* Set number of signal to be sent.  */
# define F_GETSIG	__F_GETSIG	/* Get number of signal to be sent.  */
# define F_SETOWN_EX	__F_SETOWN_EX	/* Get owner (thread receiving SIGIO).  */
# define F_GETOWN_EX	__F_GETOWN_EX	/* Set owner (thread receiving SIGIO).  */
#endif

#ifdef __USE_GNU
# define F_SETLEASE	1024	/* Set a lease.  */
# define F_GETLEASE	1025	/* Enquire what lease is active.  */
# define F_NOTIFY	1026	/* Request notifications on a directory.  */
# define F_SETPIPE_SZ	1031	/* Set pipe page size array.  */
# define F_GETPIPE_SZ	1032	/* Set pipe page size array.  */
# define F_ADD_SEALS	1033	/* Add seals to file.  */
# define F_GET_SEALS	1034	/* Get seals for file.  */
/* Set / get write life time hints.  */
# define F_GET_RW_HINT	1035
# define F_SET_RW_HINT	1036
# define F_GET_FILE_RW_HINT	1037
# define F_SET_FILE_RW_HINT	1038
#endif
#ifdef __USE_XOPEN2K8
# define F_DUPFD_CLOEXEC 1030	/* Duplicate file descriptor with
				   close-on-exit set.  */
#endif

/* For F_[GET|SET]FD.  */
#define FD_CLOEXEC	1	/* Actually anything with low bit set goes */

#ifndef F_RDLCK
/* For posix fcntl() and `l_type' field of a `struct flock' for lockf().  */
# define F_RDLCK		0	/* Read lock.  */
# define F_WRLCK		1	/* Write lock.  */
# define F_UNLCK		2	/* Remove lock.  */
#endif


/* For old implementation of BSD flock.  */
#ifndef F_EXLCK
# define F_EXLCK		4	/* or 3 */
# define F_SHLCK		8	/* or 4 */
#endif

#ifdef __USE_MISC
/* Operations for BSD flock, also used by the kernel implementation.  */
# define LOCK_SH	1	/* Shared lock.  */
# define LOCK_EX	2	/* Exclusive lock.  */
# define LOCK_NB	4	/* Or'd with one of the above to prevent
				   blocking.  */
# define LOCK_UN	8	/* Remove lock.  */
#endif

#ifdef __USE_GNU
# define LOCK_MAND	32	/* This is a mandatory flock:  */
# define LOCK_READ	64	/* ... which allows concurrent read operations.  */
# define LOCK_WRITE	128	/* ... which allows concurrent write operations.  */
# define LOCK_RW	192	/* ... Which allows concurrent read & write operations.  */
#endif

#ifdef __USE_GNU
/* Types of directory notifications that may be requested with F_NOTIFY.  */
# define DN_ACCESS	0x00000001	/* File accessed.  */
# define DN_MODIFY	0x00000002	/* File modified.  */
# define DN_CREATE	0x00000004	/* File created.  */
# define DN_DELETE	0x00000008	/* File removed.  */
# define DN_RENAME	0x00000010	/* File renamed.  */
# define DN_ATTRIB	0x00000020	/* File changed attributes.  */
# define DN_MULTISHOT	0x80000000	/* Don't remove notifier.  */
#endif


#ifdef __USE_GNU
/* Owner types.  */
enum __pid_type
  {
    F_OWNER_TID = 0,		/* Kernel thread.  */
    F_OWNER_PID,		/* Process.  */
    F_OWNER_PGRP,		/* Process group.  */
    F_OWNER_GID = F_OWNER_PGRP	/* Alternative, obsolete name.  */
  };

/* Structure to use with F_GETOWN_EX and F_SETOWN_EX.  */
struct f_owner_ex
  {
    enum __pid_type type;	/* Owner type of ID.  */
    __pid_t pid;		/* ID of owner.  */
  };
#endif

#ifdef __USE_GNU
/* Types of seals.  */
# define F_SEAL_SEAL	0x0001	/* Prevent further seals from being set.  */
# define F_SEAL_SHRINK	0x0002	/* Prevent file from shrinking.  */
# define F_SEAL_GROW	0x0004	/* Prevent file from growing.  */
# define F_SEAL_WRITE	0x0008	/* Prevent writes.  */
# define F_SEAL_FUTURE_WRITE	0x0010	/* Prevent future writes while
					   mapped.  */
#endif

#ifdef __USE_GNU
/* Hint values for F_{GET,SET}_RW_HINT.  */
# define RWF_WRITE_LIFE_NOT_SET	0
# define RWH_WRITE_LIFE_NONE	1
# define RWH_WRITE_LIFE_SHORT	2
# define RWH_WRITE_LIFE_MEDIUM	3
# define RWH_WRITE_LIFE_LONG	4
# define RWH_WRITE_LIFE_EXTREME	5
#endif

/* Define some more compatibility macros to be backward compatible with
   BSD systems which did not managed to hide these kernel macros.  */
#ifdef	__USE_MISC
# define FAPPEND	O_APPEND
# define FFSYNC		O_FSYNC
# define FASYNC		O_ASYNC
# define FNONBLOCK	O_NONBLOCK
# define FNDELAY	O_NDELAY
#endif /* Use misc.  */

#ifndef __POSIX_FADV_DONTNEED
#  define __POSIX_FADV_DONTNEED	4
#  define __POSIX_FADV_NOREUSE	5
#endif
/* Advise to `posix_fadvise'.  */
#ifdef __USE_XOPEN2K
# define POSIX_FADV_NORMAL	0 /* No further special treatment.  */
# define POSIX_FADV_RANDOM	1 /* Expect random page references.  */
# define POSIX_FADV_SEQUENTIAL	2 /* Expect sequential page references.  */
# define POSIX_FADV_WILLNEED	3 /* Will need these pages.  */
# define POSIX_FADV_DONTNEED	__POSIX_FADV_DONTNEED /* Don't need these pages.  */
# define POSIX_FADV_NOREUSE	__POSIX_FADV_NOREUSE /* Data will be accessed once.  */
#endif


#ifdef __USE_GNU
/* Flags for SYNC_FILE_RANGE.  */
# define SYNC_FILE_RANGE_WAIT_BEFORE	1 /* Wait upon writeout of all pages
					     in the range before performing the
					     write.  */
# define SYNC_FILE_RANGE_WRITE		2 /* Initiate writeout of all those
					     dirty pages in the range which are
					     not presently under writeback.  */
# define SYNC_FILE_RANGE_WAIT_AFTER	4 /* Wait upon writeout of all pages in
					     the range after performing the
					     write.  */
/* SYNC_FILE_RANGE_WRITE_AND_WAIT ensures all pages in the range are
   written to disk before returning.  */
# define SYNC_FILE_RANGE_WRITE_AND_WAIT	(SYNC_FILE_RANGE_WRITE		\
					 | SYNC_FILE_RANGE_WAIT_BEFORE	\
					 | SYNC_FILE_RANGE_WAIT_AFTER)

/* Flags for SPLICE and VMSPLICE.  */
# define SPLICE_F_MOVE		1	/* Move pages instead of copying.  */
# define SPLICE_F_NONBLOCK	2	/* Don't block on the pipe splicing
					   (but we may still block on the fd
					   we splice from/to).  */
# define SPLICE_F_MORE		4	/* Expect more data.  */
# define SPLICE_F_GIFT		8	/* Pages passed in are a gift.  */


/* Flags for fallocate.  */
# include <linux/falloc.h>


/* File handle structure.  */
struct file_handle
{
  unsigned int handle_bytes;
  int handle_type;
  /* File identifier.  */
  unsigned char f_handle[0];
};

/* Maximum handle size (for now).  */
# define MAX_HANDLE_SZ	128
#endif

/* Values for `*at' functions.  */
#ifdef __USE_ATFILE
# define AT_FDCWD		-100	/* Special value used to indicate
					   the *at functions should use the
					   current working directory. */
# define AT_SYMLINK_NOFOLLOW	0x100	/* Do not follow symbolic links.  */
# define AT_REMOVEDIR		0x200	/* Remove directory instead of
					   unlinking file.  */
# define AT_SYMLINK_FOLLOW	0x400	/* Follow symbolic links.  */
# ifdef __USE_GNU
#  define AT_NO_AUTOMOUNT	0x800	/* Suppress terminal automount
					   traversal.  */
#  define AT_EMPTY_PATH		0x1000	/* Allow empty relative pathname.  */
#  define AT_STATX_SYNC_TYPE	0x6000
#  define AT_STATX_SYNC_AS_STAT	0x0000
#  define AT_STATX_FORCE_SYNC	0x2000
#  define AT_STATX_DONT_SYNC	0x4000
#  define AT_RECURSIVE		0x8000	/* Apply to the entire subtree.  */
# endif
# define AT_EACCESS		0x200	/* Test access permitted for
					   effective IDs, not real IDs.  */
#endif

__BEGIN_DECLS

#ifdef __USE_GNU

/* Provide kernel hint to read ahead.  */
extern __ssize_t readahead (int __fd, __off64_t __offset, size_t __count)
    __THROW;


/* Selective file content synch'ing.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int sync_file_range (int __fd, __off64_t __offset, __off64_t __count,
			    unsigned int __flags);


/* Splice address range into a pipe.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern __ssize_t vmsplice (int __fdout, const struct iovec *__iov,
			   size_t __count, unsigned int __flags);

/* Splice two files together.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern __ssize_t splice (int __fdin, __off64_t *__offin, int __fdout,
			 __off64_t *__offout, size_t __len,
			 unsigned int __flags);

/* In-kernel implementation of tee for pipe buffers.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern __ssize_t tee (int __fdin, int __fdout, size_t __len,
		      unsigned int __flags);

/* Reserve storage for the data of the file associated with FD.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
# ifndef __USE_FILE_OFFSET64
extern int fallocate (int __fd, int __mode, __off_t __offset, __off_t __len);
# else
#  ifdef __REDIRECT
extern int __REDIRECT (fallocate, (int __fd, int __mode, __off64_t __offset,
				   __off64_t __len),
		       fallocate64);
#  else
#   define fallocate fallocate64
#  endif
# endif
# ifdef __USE_LARGEFILE64
extern int fallocate64 (int __fd, int __mode, __off64_t __offset,
			__off64_t __len);
# endif


/* Map file name to file handle.  */
extern int name_to_handle_at (int __dfd, const char *__name,
			      struct file_handle *__handle, int *__mnt_id,
			      int __flags) __THROW;

/* Open file using the file handle.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int open_by_handle_at (int __mountdirfd, struct file_handle *__handle,
			      int __flags);

#endif	/* use GNU */

__END_DECLS
