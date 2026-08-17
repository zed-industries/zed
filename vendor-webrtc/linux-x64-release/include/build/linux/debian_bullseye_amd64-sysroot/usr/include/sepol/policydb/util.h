/* Authors: Karl MacMillan <kmacmillan@tresys.com>
 *
 * A set of utility functions that aid policy decision when dealing
 * with hierarchal namespaces.
 *
 * Copyright (C) 2006 Tresys Technology, LLC
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 2.1 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

#ifndef __SEPOL_UTIL_H__
#define __SEPOL_UTIL_H__

#ifdef __cplusplus
extern "C" {
#endif

extern int add_i_to_a(uint32_t i, uint32_t * cnt, uint32_t ** a);

extern char *sepol_av_to_string(policydb_t * policydbp, uint32_t tclass,
				sepol_access_vector_t av);

char *sepol_extended_perms_to_string(avtab_extended_perms_t *xperms);

/*
 * The tokenize function may be used to
 * replace sscanf
 */
extern int tokenize(char *line_buf, char delim, int num_args, ...);

#ifdef __cplusplus
}
#endif

#endif
