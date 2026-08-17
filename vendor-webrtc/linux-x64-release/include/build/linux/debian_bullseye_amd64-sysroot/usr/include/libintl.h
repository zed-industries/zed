/* Message catalogs for internationalization.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   This file is derived from the file libgettext.h in the GNU gettext package.

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

#ifndef _LIBINTL_H
#define _LIBINTL_H	1

#include <features.h>

/* We define an additional symbol to signal that we use the GNU
   implementation of gettext.  */
#define __USE_GNU_GETTEXT 1

/* Provide information about the supported file formats.  Returns the
   maximum minor revision number supported for a given major revision.  */
#define __GNU_GETTEXT_SUPPORTED_REVISION(major) \
  ((major) == 0 ? 1 : -1)

__BEGIN_DECLS

/* Look up MSGID in the current default message catalog for the current
   LC_MESSAGES locale.  If not found, returns MSGID itself (the default
   text).  */
extern char *gettext (const char *__msgid)
     __THROW __attribute_format_arg__ (1);

/* Look up MSGID in the DOMAINNAME message catalog for the current
   LC_MESSAGES locale.  */
extern char *dgettext (const char *__domainname, const char *__msgid)
     __THROW __attribute_format_arg__ (2);
extern char *__dgettext (const char *__domainname, const char *__msgid)
     __THROW __attribute_format_arg__ (2);

/* Look up MSGID in the DOMAINNAME message catalog for the current CATEGORY
   locale.  */
extern char *dcgettext (const char *__domainname,
			const char *__msgid, int __category)
     __THROW __attribute_format_arg__ (2);
extern char *__dcgettext (const char *__domainname,
			  const char *__msgid, int __category)
     __THROW __attribute_format_arg__ (2);


/* Similar to `gettext' but select the plural form corresponding to the
   number N.  */
extern char *ngettext (const char *__msgid1, const char *__msgid2,
		       unsigned long int __n)
     __THROW __attribute_format_arg__ (1) __attribute_format_arg__ (2);

/* Similar to `dgettext' but select the plural form corresponding to the
   number N.  */
extern char *dngettext (const char *__domainname, const char *__msgid1,
			const char *__msgid2, unsigned long int __n)
     __THROW __attribute_format_arg__ (2) __attribute_format_arg__ (3);

/* Similar to `dcgettext' but select the plural form corresponding to the
   number N.  */
extern char *dcngettext (const char *__domainname, const char *__msgid1,
			 const char *__msgid2, unsigned long int __n,
			 int __category)
     __THROW __attribute_format_arg__ (2) __attribute_format_arg__ (3);


/* Set the current default message catalog to DOMAINNAME.
   If DOMAINNAME is null, return the current default.
   If DOMAINNAME is "", reset to the default of "messages".  */
extern char *textdomain (const char *__domainname) __THROW;

/* Specify that the DOMAINNAME message catalog will be found
   in DIRNAME rather than in the system locale data base.  */
extern char *bindtextdomain (const char *__domainname,
			     const char *__dirname) __THROW;

/* Specify the character encoding in which the messages from the
   DOMAINNAME message catalog will be returned.  */
extern char *bind_textdomain_codeset (const char *__domainname,
				      const char *__codeset) __THROW;


/* Optimized version of the function above.  */
#if defined __OPTIMIZE__ && !defined __cplusplus

/* We need NULL for `gettext'.  */
# define __need_NULL
# include <stddef.h>

/* We need LC_MESSAGES for `dgettext'.  */
# include <locale.h>

/* These must be macros.  Inlined functions are useless because the
   `__builtin_constant_p' predicate in dcgettext would always return
   false.  */

# define gettext(msgid) dgettext (NULL, msgid)

# define dgettext(domainname, msgid) \
  dcgettext (domainname, msgid, LC_MESSAGES)

# define ngettext(msgid1, msgid2, n) dngettext (NULL, msgid1, msgid2, n)

# define dngettext(domainname, msgid1, msgid2, n) \
  dcngettext (domainname, msgid1, msgid2, n, LC_MESSAGES)

#endif	/* Optimizing.  */

__END_DECLS

#endif /* libintl.h */
