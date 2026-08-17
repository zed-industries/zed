/* Authors: Jason Tang <jtang@tresys.com>
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

#ifndef _SEPOL_AVRULE_BLOCK_H_
#define _SEPOL_AVRULE_BLOCK_H_

#include <sepol/policydb/policydb.h>

#ifdef __cplusplus
extern "C" {
#endif

extern avrule_block_t *avrule_block_create(void);
extern void avrule_block_destroy(avrule_block_t * x);
extern avrule_decl_t *avrule_decl_create(uint32_t decl_id);
extern void avrule_decl_destroy(avrule_decl_t * x);
extern void avrule_block_list_destroy(avrule_block_t * x);
extern avrule_decl_t *get_avrule_decl(policydb_t * p, uint32_t decl_id);
extern cond_list_t *get_decl_cond_list(policydb_t * p,
				       avrule_decl_t * decl,
				       cond_list_t * cond);
extern int is_id_enabled(char *id, policydb_t * p, int symbol_table);
extern int is_perm_enabled(char *class_id, char *perm_id, policydb_t * p);

#ifdef __cplusplus
}
#endif

#endif
