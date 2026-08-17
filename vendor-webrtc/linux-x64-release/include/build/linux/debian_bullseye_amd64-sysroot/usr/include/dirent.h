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
 *	POSIX Standard: 5.1.2 Directory Operations	<dirent.h>
 */

#ifndef	_DIRENT_H
#define	_DIRENT_H	1

#include <features.h>

__BEGIN_DECLS

#include <bits/types.h>

#ifdef __USE_XOPEN
# ifndef __ino_t_defined
#  ifndef __USE_FILE_OFFSET64
typedef __ino_t ino_t;
#  else
typedef __ino64_t ino_t;
#  endif
#  define __ino_t_defined
# endif
# if defined __USE_LARGEFILE64 && !defined __ino64_t_defined
typedef __ino64_t ino64_t;
#  define __ino64_t_defined
# endif
#endif

/* This file defines `struct dirent'.

   It defines the macro `_DIRENT_HAVE_D_NAMLEN' iff there is a `d_namlen'
   member that gives the length of `d_name'.

   It defines the macro `_DIRENT_HAVE_D_RECLEN' iff there is a `d_reclen'
   member that gives the size of the entire directory entry.

   It defines the macro `_DIRENT_HAVE_D_OFF' iff there is a `d_off'
   member that gives the file offset of the next directory entry.

   It defines the macro `_DIRENT_HAVE_D_TYPE' iff there is a `d_type'
   member that gives the type of the file.
 */

#include <bits/dirent.h>

#if defined __USE_MISC && !defined d_fileno
# define d_ino	d_fileno		 /* Backward compatibility.  */
#endif

/* These macros extract size information from a `struct dirent *'.
   They may evaluate their argument multiple times, so it must not
   have side effects.  Each of these may involve a relatively costly
   call to `strlen' on some systems, so these values should be cached.

   _D_EXACT_NAMLEN (DP)	returns the length of DP->d_name, not including
   its terminating null character.

   _D_ALLOC_NAMLEN (DP)	returns a size at least (_D_EXACT_NAMLEN (DP) + 1);
   that is, the allocation size needed to hold the DP->d_name string.
   Use this macro when you don't need the exact length, just an upper bound.
   This macro is less likely to require calling `strlen' than _D_EXACT_NAMLEN.
   */

#ifdef _DIRENT_HAVE_D_NAMLEN
# define _D_EXACT_NAMLEN(d) ((d)->d_namlen)
# define _D_ALLOC_NAMLEN(d) (_D_EXACT_NAMLEN (d) + 1)
#else
# define _D_EXACT_NAMLEN(d) (strlen ((d)->d_name))
# ifdef _DIRENT_HAVE_D_RECLEN
#  define _D_ALLOC_NAMLEN(d) (((char *) (d) + (d)->d_reclen) - &(d)->d_name[0])
# else
#  define _D_ALLOC_NAMLEN(d) (sizeof (d)->d_name > 1 ? sizeof (d)->d_name \
			      : _D_EXACT_NAMLEN (d) + 1)
# endif
#endif


#ifdef __USE_MISC
/* File types for `d_type'.  */
enum
  {
    DT_UNKNOWN = 0,
# define DT_UNKNOWN	DT_UNKNOWN
    DT_FIFO = 1,
# define DT_FIFO	DT_FIFO
    DT_CHR = 2,
# define DT_CHR		DT_CHR
    DT_DIR = 4,
# define DT_DIR		DT_DIR
    DT_BLK = 6,
# define DT_BLK		DT_BLK
    DT_REG = 8,
# define DT_REG		DT_REG
    DT_LNK = 10,
# define DT_LNK		DT_LNK
    DT_SOCK = 12,
# define DT_SOCK	DT_SOCK
    DT_WHT = 14
# define DT_WHT		DT_WHT
  };

/* Convert between stat structure types and directory types.  */
# define IFTODT(mode)	(((mode) & 0170000) >> 12)
# define DTTOIF(dirtype)	((dirtype) << 12)
#endif


/* This is the data type of directory stream objects.
   The actual structure is opaque to users.  */
typedef struct __dirstream DIR;

/* Open a directory stream on NAME.
   Return a DIR stream on the directory, or NULL if it could not be opened.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern DIR *opendir (const char *__name) __nonnull ((1));

#ifdef __USE_XOPEN2K8
/* Same as opendir, but open the stream on the file descriptor FD.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern DIR *fdopendir (int __fd);
#endif

/* Close the directory stream DIRP.
   Return 0 if successful, -1 if not.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int closedir (DIR *__dirp) __nonnull ((1));

/* Read a directory entry from DIRP.  Return a pointer to a `struct
   dirent' describing the entry, or NULL for EOF or error.  The
   storage returned may be overwritten by a later readdir call on the
   same DIR stream.

   If the Large File Support API is selected we have to use the
   appropriate interface.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
#ifndef __USE_FILE_OFFSET64
extern struct dirent *readdir (DIR *__dirp) __nonnull ((1));
#else
# ifdef __REDIRECT
extern struct dirent *__REDIRECT (readdir, (DIR *__dirp), readdir64)
     __nonnull ((1));
# else
#  define readdir readdir64
# endif
#endif

#ifdef __USE_LARGEFILE64
extern struct dirent64 *readdir64 (DIR *__dirp) __nonnull ((1));
#endif

#ifdef __USE_POSIX
/* Reentrant version of `readdir'.  Return in RESULT a pointer to the
   next entry.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
# ifndef __USE_FILE_OFFSET64
extern int readdir_r (DIR *__restrict __dirp,
		      struct dirent *__restrict __entry,
		      struct dirent **__restrict __result)
     __nonnull ((1, 2, 3)) __attribute_deprecated__;
# else
#  ifdef __REDIRECT
extern int __REDIRECT (readdir_r,
		       (DIR *__restrict __dirp,
			struct dirent *__restrict __entry,
			struct dirent **__restrict __result),
		       readdir64_r)
  __nonnull ((1, 2, 3)) __attribute_deprecated__;
#  else
#   define readdir_r readdir64_r
#  endif
# endif

# ifdef __USE_LARGEFILE64
extern int readdir64_r (DIR *__restrict __dirp,
			struct dirent64 *__restrict __entry,
			struct dirent64 **__restrict __result)
  __nonnull ((1, 2, 3)) __attribute_deprecated__;
# endif
#endif	/* POSIX or misc */

/* Rewind DIRP to the beginning of the directory.  */
extern void rewinddir (DIR *__dirp) __THROW __nonnull ((1));

#if defined __USE_MISC || defined __USE_XOPEN
# include <bits/types.h>

/* Seek to position POS on DIRP.  */
extern void seekdir (DIR *__dirp, long int __pos) __THROW __nonnull ((1));

/* Return the current position of DIRP.  */
extern long int telldir (DIR *__dirp) __THROW __nonnull ((1));
#endif

#ifdef __USE_XOPEN2K8

/* Return the file descriptor used by DIRP.  */
extern int dirfd (DIR *__dirp) __THROW __nonnull ((1));

# if defined __OPTIMIZE__ && defined _DIR_dirfd
#  define dirfd(dirp)	_DIR_dirfd (dirp)
# endif

# ifdef __USE_MISC
#  ifndef MAXNAMLEN
/* Get the definitions of the POSIX.1 limits.  */
#  include <bits/posix1_lim.h>

/* `MAXNAMLEN' is the BSD name for what POSIX calls `NAME_MAX'.  */
#   ifdef NAME_MAX
#    define MAXNAMLEN	NAME_MAX
#   else
#    define MAXNAMLEN	255
#   endif
#  endif
# endif

# define __need_size_t
# include <stddef.h>

/* Scan the directory DIR, calling SELECTOR on each directory entry.
   Entries for which SELECT returns nonzero are individually malloc'd,
   sorted using qsort with CMP, and collected in a malloc'd array in
   *NAMELIST.  Returns the number of entries selected, or -1 on error.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
# ifndef __USE_FILE_OFFSET64
extern int scandir (const char *__restrict __dir,
		    struct dirent ***__restrict __namelist,
		    int (*__selector) (const struct dirent *),
		    int (*__cmp) (const struct dirent **,
				  const struct dirent **))
     __nonnull ((1, 2));
# else
#  ifdef __REDIRECT
extern int __REDIRECT (scandir,
		       (const char *__restrict __dir,
			struct dirent ***__restrict __namelist,
			int (*__selector) (const struct dirent *),
			int (*__cmp) (const struct dirent **,
				      const struct dirent **)),
		       scandir64) __nonnull ((1, 2));
#  else
#   define scandir scandir64
#  endif
# endif

# if defined __USE_GNU && defined __USE_LARGEFILE64
/* This function is like `scandir' but it uses the 64bit dirent structure.
   Please note that the CMP function must now work with struct dirent64 **.  */
extern int scandir64 (const char *__restrict __dir,
		      struct dirent64 ***__restrict __namelist,
		      int (*__selector) (const struct dirent64 *),
		      int (*__cmp) (const struct dirent64 **,
				    const struct dirent64 **))
     __nonnull ((1, 2));
# endif

# ifdef __USE_GNU
/* Similar to `scandir' but a relative DIR name is interpreted relative
   to the directory for which DFD is a descriptor.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
#  ifndef __USE_FILE_OFFSET64
extern int scandirat (int __dfd, const char *__restrict __dir,
		      struct dirent ***__restrict __namelist,
		      int (*__selector) (const struct dirent *),
		      int (*__cmp) (const struct dirent **,
				    const struct dirent **))
     __nonnull ((2, 3));
#  else
#   ifdef __REDIRECT
extern int __REDIRECT (scandirat,
		       (int __dfd, const char *__restrict __dir,
			struct dirent ***__restrict __namelist,
			int (*__selector) (const struct dirent *),
			int (*__cmp) (const struct dirent **,
				      const struct dirent **)),
		       scandirat64) __nonnull ((2, 3));
#   else
#    define scandirat scandirat64
#   endif
#  endif

/* This function is like `scandir' but it uses the 64bit dirent structure.
   Please note that the CMP function must now work with struct dirent64 **.  */
extern int scandirat64 (int __dfd, const char *__restrict __dir,
			struct dirent64 ***__restrict __namelist,
			int (*__selector) (const struct dirent64 *),
			int (*__cmp) (const struct dirent64 **,
				      const struct dirent64 **))
     __nonnull ((2, 3));
# endif

/* Function to compare two `struct dirent's alphabetically.  */
# ifndef __USE_FILE_OFFSET64
extern int alphasort (const struct dirent **__e1,
		      const struct dirent **__e2)
     __THROW __attribute_pure__ __nonnull ((1, 2));
# else
#  ifdef __REDIRECT
extern int __REDIRECT_NTH (alphasort,
			   (const struct dirent **__e1,
			    const struct dirent **__e2),
			   alphasort64) __attribute_pure__ __nonnull ((1, 2));
#  else
#   define alphasort alphasort64
#  endif
# endif

# if defined __USE_GNU && defined __USE_LARGEFILE64
extern int alphasort64 (const struct dirent64 **__e1,
			const struct dirent64 **__e2)
     __THROW __attribute_pure__ __nonnull ((1, 2));
# endif
#endif /* Use XPG7.  */


#ifdef __USE_MISC
/* Read directory entries from FD into BUF, reading at most NBYTES.
   Reading starts at offset *BASEP, and *BASEP is updated with the new
   position after reading.  Returns the number of bytes read; zero when at
   end of directory; or -1 for errors.  */
# ifndef __USE_FILE_OFFSET64
extern __ssize_t getdirentries (int __fd, char *__restrict __buf,
				size_t __nbytes,
				__off_t *__restrict __basep)
     __THROW __nonnull ((2, 4));
# else
#  ifdef __REDIRECT
extern __ssize_t __REDIRECT_NTH (getdirentries,
				 (int __fd, char *__restrict __buf,
				  size_t __nbytes,
				  __off64_t *__restrict __basep),
				 getdirentries64) __nonnull ((2, 4));
#  else
#   define getdirentries getdirentries64
#  endif
# endif

# ifdef __USE_LARGEFILE64
extern __ssize_t getdirentries64 (int __fd, char *__restrict __buf,
				  size_t __nbytes,
				  __off64_t *__restrict __basep)
     __THROW __nonnull ((2, 4));
# endif
#endif /* Use misc.  */

#ifdef __USE_GNU
/* Function to compare two `struct dirent's by name & version.  */
# ifndef __USE_FILE_OFFSET64
extern int versionsort (const struct dirent **__e1,
			const struct dirent **__e2)
     __THROW __attribute_pure__ __nonnull ((1, 2));
# else
#  ifdef __REDIRECT
extern int __REDIRECT_NTH (versionsort,
			   (const struct dirent **__e1,
			    const struct dirent **__e2),
			   versionsort64)
     __attribute_pure__ __nonnull ((1, 2));
#  else
#   define versionsort versionsort64
#  endif
# endif

# ifdef __USE_LARGEFILE64
extern int versionsort64 (const struct dirent64 **__e1,
			  const struct dirent64 **__e2)
     __THROW __attribute_pure__ __nonnull ((1, 2));
# endif
#endif /* Use GNU.  */

__END_DECLS

#include <bits/dirent_ext.h>

#endif /* dirent.h  */
