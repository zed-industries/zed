/*
 * Public directory definitions for CUPS.
 *
 * This set of APIs abstracts enumeration of directory entries.
 *
 * Copyright 2007-2011 by Apple Inc.
 * Copyright 1997-2006 by Easy Software Products, all rights reserved.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more information.
 */

#ifndef _CUPS_DIR_H_
#  define _CUPS_DIR_H_


/*
 * Include necessary headers...
 */

#  include "versioning.h"
#  include <sys/stat.h>


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Data types...
 */

typedef struct _cups_dir_s cups_dir_t;	/**** Directory type ****/

typedef struct cups_dentry_s		/**** Directory entry type ****/
{
  char		filename[260];		/* File name */
  struct stat	fileinfo;		/* File information */
} cups_dentry_t;


/*
 * Prototypes...
 */

extern void		cupsDirClose(cups_dir_t *dp) _CUPS_API_1_2;
extern cups_dir_t	*cupsDirOpen(const char *directory) _CUPS_API_1_2;
extern cups_dentry_t	*cupsDirRead(cups_dir_t *dp) _CUPS_API_1_2;
extern void		cupsDirRewind(cups_dir_t *dp) _CUPS_API_1_2;


#  ifdef __cplusplus
}
#  endif /* __cplusplus */
#endif /* !_CUPS_DIR_H_ */
