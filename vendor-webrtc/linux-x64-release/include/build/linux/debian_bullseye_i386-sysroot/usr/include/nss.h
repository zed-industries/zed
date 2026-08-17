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

/* Define interface to NSS.  This is meant for the interface functions
   and for implementors of new services.  */

#ifndef _NSS_H
#define _NSS_H	1

#include <features.h>
#include <stdint.h>


__BEGIN_DECLS

/* Possible results of lookup using a nss_* function.  */
enum nss_status
{
  NSS_STATUS_TRYAGAIN = -2,
  NSS_STATUS_UNAVAIL,
  NSS_STATUS_NOTFOUND,
  NSS_STATUS_SUCCESS,
  NSS_STATUS_RETURN
};


/* Data structure used for the 'gethostbyname4_r' function.  */
struct gaih_addrtuple
  {
    struct gaih_addrtuple *next;
    char *name;
    int family;
    uint32_t addr[4];
    uint32_t scopeid;
  };


/* Overwrite service selection for database DBNAME using specification
   in STRING.
   This function should only be used by system programs which have to
   work around non-existing services (e.e., while booting).
   Attention: Using this function repeatedly will slowly eat up the
   whole memory since previous selection data cannot be freed.  */
extern int __nss_configure_lookup (const char *__dbname,
				   const char *__string) __THROW;

__END_DECLS

#endif /* nss.h */
