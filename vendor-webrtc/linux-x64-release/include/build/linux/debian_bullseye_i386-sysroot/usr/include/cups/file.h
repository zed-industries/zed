/*
 * Public file definitions for CUPS.
 *
 * Since stdio files max out at 256 files on many systems, we have to
 * write similar functions without this limit.  At the same time, using
 * our own file functions allows us to provide transparent support of
 * different line endings, gzip'd print files, PPD files, etc.
 *
 * Copyright © 2007-2018 by Apple Inc.
 * Copyright © 1997-2007 by Easy Software Products, all rights reserved.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_FILE_H_
#  define _CUPS_FILE_H_


/*
 * Include necessary headers...
 */

#  include "versioning.h"
#  include <stddef.h>
#  include <sys/types.h>
#  if defined(_WIN32) && !defined(__CUPS_SSIZE_T_DEFINED)
#    define __CUPS_SSIZE_T_DEFINED
/* Windows does not support the ssize_t type, so map it to off_t... */
typedef off_t ssize_t;			/* @private@ */
#  endif /* _WIN32 && !__CUPS_SSIZE_T_DEFINED */


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * CUPS file definitions...
 */

#  define CUPS_FILE_NONE	0	/* No compression */
#  define CUPS_FILE_GZIP	1	/* GZIP compression */


/*
 * Types and structures...
 */

typedef struct _cups_file_s cups_file_t;/**** CUPS file type ****/


/*
 * Prototypes...
 */

extern int		cupsFileClose(cups_file_t *fp) _CUPS_API_1_2;
extern int		cupsFileCompression(cups_file_t *fp) _CUPS_API_1_2;
extern int		cupsFileEOF(cups_file_t *fp) _CUPS_API_1_2;
extern const char	*cupsFileFind(const char *filename, const char *path, int executable, char *buffer, int bufsize) _CUPS_API_1_2;
extern int		cupsFileFlush(cups_file_t *fp) _CUPS_API_1_2;
extern int		cupsFileGetChar(cups_file_t *fp) _CUPS_API_1_2;
extern char		*cupsFileGetConf(cups_file_t *fp, char *buf, size_t buflen, char **value, int *linenum) _CUPS_API_1_2;
extern size_t		cupsFileGetLine(cups_file_t *fp, char *buf, size_t buflen) _CUPS_API_1_2;
extern char		*cupsFileGets(cups_file_t *fp, char *buf, size_t buflen) _CUPS_API_1_2;
extern int		cupsFileLock(cups_file_t *fp, int block) _CUPS_API_1_2;
extern int		cupsFileNumber(cups_file_t *fp) _CUPS_API_1_2;
extern cups_file_t	*cupsFileOpen(const char *filename, const char *mode) _CUPS_API_1_2;
extern cups_file_t	*cupsFileOpenFd(int fd, const char *mode) _CUPS_API_1_2;
extern int		cupsFilePeekChar(cups_file_t *fp) _CUPS_API_1_2;
extern int		cupsFilePrintf(cups_file_t *fp, const char *format, ...) _CUPS_FORMAT(2, 3) _CUPS_API_1_2;
extern int		cupsFilePutChar(cups_file_t *fp, int c) _CUPS_API_1_2;
extern ssize_t		cupsFilePutConf(cups_file_t *fp, const char *directive, const char *value) _CUPS_API_1_4;
extern int		cupsFilePuts(cups_file_t *fp, const char *s) _CUPS_API_1_2;
extern ssize_t		cupsFileRead(cups_file_t *fp, char *buf, size_t bytes) _CUPS_API_1_2;
extern off_t		cupsFileRewind(cups_file_t *fp) _CUPS_API_1_2;
extern off_t		cupsFileSeek(cups_file_t *fp, off_t pos) _CUPS_API_1_2;
extern cups_file_t	*cupsFileStderr(void) _CUPS_API_1_2;
extern cups_file_t	*cupsFileStdin(void) _CUPS_API_1_2;
extern cups_file_t	*cupsFileStdout(void) _CUPS_API_1_2;
extern off_t		cupsFileTell(cups_file_t *fp) _CUPS_API_1_2;
extern int		cupsFileUnlock(cups_file_t *fp) _CUPS_API_1_2;
extern ssize_t		cupsFileWrite(cups_file_t *fp, const char *buf, size_t bytes) _CUPS_API_1_2;


#  ifdef __cplusplus
}
#  endif /* __cplusplus */
#endif /* !_CUPS_FILE_H_ */
