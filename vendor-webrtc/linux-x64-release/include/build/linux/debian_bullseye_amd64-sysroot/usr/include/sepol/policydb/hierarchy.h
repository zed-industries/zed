/* Authors: Jason Tang <jtang@tresys.com>
 *	    Joshua Brindle <jbrindle@tresys.com>
 *          Karl MacMillan <kmacmillan@tresys.com>
 *
 * A set of utility functions that aid policy decision when dealing
 * with hierarchal items.
 *
 * Copyright (C) 2005 Tresys Technology, LLC
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

#ifndef _SEPOL_POLICYDB_HIERARCHY_H_
#define _SEPOL_POLICYDB_HIERARCHY_H_

#include <sepol/policydb/avtab.h>
#include <sepol/policydb/policydb.h>

#ifdef __cplusplus
extern "C" {
#endif

extern int hierarchy_add_bounds(sepol_handle_t *handle, policydb_t *p);

extern void bounds_destroy_bad(avtab_ptr_t cur);
extern int bounds_check_type(sepol_handle_t *handle, policydb_t *p, uint32_t child,
			     uint32_t parent, avtab_ptr_t *bad, int *numbad);

extern int bounds_check_users(sepol_handle_t *handle, policydb_t *p);
extern int bounds_check_roles(sepol_handle_t *handle, policydb_t *p);
extern int bounds_check_types(sepol_handle_t *handle, policydb_t *p);

extern int hierarchy_check_constraints(sepol_handle_t * handle, policydb_t * p);

#ifdef __cplusplus
}
#endif

#endif
