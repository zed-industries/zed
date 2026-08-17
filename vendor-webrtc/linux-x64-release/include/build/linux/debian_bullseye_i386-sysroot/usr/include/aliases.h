/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef _ALIASES_H
#define _ALIASES_H	1

#include <features.h>

#include <sys/types.h>


__BEGIN_DECLS

/* Structure to represent one entry of the alias data base.  */
struct aliasent
  {
    char *alias_name;
    size_t alias_members_len;
    char **alias_members;
    int alias_local;
  };


/* Open alias data base files.  */
extern void setaliasent (void) __THROW;

/* Close alias data base files.  */
extern void endaliasent (void) __THROW;

/* Get the next entry from the alias data base.  */
extern struct aliasent *getaliasent (void) __THROW;

/* Get the next entry from the alias data base and put it in RESULT_BUF.  */
extern int getaliasent_r (struct aliasent *__restrict __result_buf,
			  char *__restrict __buffer, size_t __buflen,
			  struct aliasent **__restrict __result) __THROW;

/* Get alias entry corresponding to NAME.  */
extern struct aliasent *getaliasbyname (const char *__name) __THROW;

/* Get alias entry corresponding to NAME and put it in RESULT_BUF.  */
extern int getaliasbyname_r (const char *__restrict __name,
			     struct aliasent *__restrict __result_buf,
			     char *__restrict __buffer, size_t __buflen,
			     struct aliasent **__restrict __result) __THROW;

__END_DECLS

#endif /* aliases.h */
