/* Message display handling.
   Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

#ifndef __FMTMSG_H
#define __FMTMSG_H	1

#include <features.h>


__BEGIN_DECLS

/* Values to control `fmtmsg' function.  */
enum
{
  MM_HARD = 0x001,	/* Source of the condition is hardware.  */
#define MM_HARD MM_HARD
  MM_SOFT = 0x002,	/* Source of the condition is software.  */
#define MM_SOFT MM_SOFT
  MM_FIRM = 0x004,	/* Source of the condition is firmware.  */
#define MM_FIRM MM_FIRM
  MM_APPL = 0x008,	/* Condition detected by application.  */
#define MM_APPL MM_APPL
  MM_UTIL = 0x010,	/* Condition detected by utility.  */
#define MM_UTIL MM_UTIL
  MM_OPSYS = 0x020,	/* Condition detected by operating system.  */
#define MM_OPSYS MM_OPSYS
  MM_RECOVER = 0x040,	/* Recoverable error.  */
#define MM_RECOVER MM_RECOVER
  MM_NRECOV = 0x080,	/* Non-recoverable error.  */
#define MM_NRECOV MM_NRECOV
  MM_PRINT = 0x100,	/* Display message in standard error.  */
#define MM_PRINT MM_PRINT
  MM_CONSOLE = 0x200	/* Display message on system console.  */
#define MM_CONSOLE MM_CONSOLE
};

/* Values to be for SEVERITY parameter of `fmtmsg'.  */
enum
{
  MM_NOSEV = 0,		/* No severity level provided for the message.  */
#define MM_NOSEV MM_NOSEV
  MM_HALT,		/* Error causing application to halt.  */
#define MM_HALT MM_HALT
  MM_ERROR,		/* Application has encountered a non-fatal fault.  */
#define MM_ERROR MM_ERROR
  MM_WARNING,		/* Application has detected unusual non-error
			   condition.  */
#define MM_WARNING MM_WARNING
  MM_INFO		/* Informative message.  */
#define MM_INFO MM_INFO
};


/* Macros which can be used as null values for the arguments of `fmtmsg'.  */
#define MM_NULLLBL	((char *) 0)
#define MM_NULLSEV	0
#define MM_NULLMC	((long int) 0)
#define MM_NULLTXT	((char *) 0)
#define MM_NULLACT	((char *) 0)
#define MM_NULLTAG	((char *) 0)


/* Possible return values of `fmtmsg'.  */
enum
{
  MM_NOTOK = -1,
#define MM_NOTOK MM_NOTOK
  MM_OK = 0,
#define MM_OK MM_OK
  MM_NOMSG = 1,
#define MM_NOMSG MM_NOMSG
  MM_NOCON = 4
#define MM_NOCON MM_NOCON
};


/* Print message with given CLASSIFICATION, LABEL, SEVERITY, TEXT, ACTION
   and TAG to console or standard error.  */
extern int fmtmsg (long int __classification, const char *__label,
		   int __severity, const char *__text,
		   const char *__action, const char *__tag);

#ifdef __USE_MISC
/* Add or remove severity level.  */
extern int addseverity (int __severity, const char *__string) __THROW;
#endif

__END_DECLS

#endif /* fmtmsg.h */
