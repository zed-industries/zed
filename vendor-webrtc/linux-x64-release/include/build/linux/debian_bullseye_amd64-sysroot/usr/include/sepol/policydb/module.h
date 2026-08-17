/* Author: Karl MacMillan <kmacmillan@tresys.com>
 *
 * Copyright (C) 2004-2005 Tresys Technology, LLC
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

#ifndef _SEPOL_POLICYDB_MODULE_H_
#define _SEPOL_POLICYDB_MODULE_H_

#include <stdlib.h>
#include <stddef.h>

#include <sepol/module.h>

#include <sepol/policydb/policydb.h>
#include <sepol/policydb/conditional.h>

#define SEPOL_MODULE_PACKAGE_MAGIC 0xf97cff8f

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_module_package {
	sepol_policydb_t *policy;
	uint32_t version;
	char *file_contexts;
	size_t file_contexts_len;
	char *seusers;
	size_t seusers_len;
	char *user_extra;
	size_t user_extra_len;
	char *netfilter_contexts;
	size_t netfilter_contexts_len;
};

extern int sepol_module_package_init(sepol_module_package_t * p);

#ifdef __cplusplus
}
#endif

#endif
