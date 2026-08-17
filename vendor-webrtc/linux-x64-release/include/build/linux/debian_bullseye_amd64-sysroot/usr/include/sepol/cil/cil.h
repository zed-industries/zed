/*
 * Copyright 2011 Tresys Technology, LLC. All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 * 
 *    1. Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 * 
 *    2. Redistributions in binary form must reproduce the above copyright notice,
 *       this list of conditions and the following disclaimer in the documentation
 *       and/or other materials provided with the distribution.
 * 
 * THIS SOFTWARE IS PROVIDED BY TRESYS TECHNOLOGY, LLC ``AS IS'' AND ANY EXPRESS
 * OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
 * EVENT SHALL TRESYS TECHNOLOGY, LLC OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * 
 * The views and conclusions contained in the software and documentation are those
 * of the authors and should not be interpreted as representing official policies,
 * either expressed or implied, of Tresys Technology, LLC.
 */

#ifndef CIL_H_
#define CIL_H_

#include <sepol/policydb/policydb.h>

#ifdef __cplusplus
extern "C" {
#endif

struct cil_db;
typedef struct cil_db cil_db_t;

extern void cil_db_init(cil_db_t **db);
extern void cil_db_destroy(cil_db_t **db);

extern int cil_add_file(cil_db_t *db, char *name, char *data, size_t size);

extern int cil_compile(cil_db_t *db);
extern int cil_build_policydb(cil_db_t *db, sepol_policydb_t **sepol_db);
extern int cil_userprefixes_to_string(cil_db_t *db, char **out, size_t *size);
extern int cil_selinuxusers_to_string(cil_db_t *db, char **out, size_t *size);
extern int cil_filecons_to_string(cil_db_t *db, char **out, size_t *size);
extern void cil_set_disable_dontaudit(cil_db_t *db, int disable_dontaudit);
extern void cil_set_multiple_decls(cil_db_t *db, int multiple_decls);
extern void cil_set_disable_neverallow(cil_db_t *db, int disable_neverallow);
extern void cil_set_preserve_tunables(cil_db_t *db, int preserve_tunables);
extern int cil_set_handle_unknown(cil_db_t *db, int handle_unknown);
extern void cil_set_mls(cil_db_t *db, int mls);
extern void cil_set_attrs_expand_generated(struct cil_db *db, int attrs_expand_generated);
extern void cil_set_attrs_expand_size(struct cil_db *db, unsigned attrs_expand_size);
extern void cil_set_target_platform(cil_db_t *db, int target_platform);
extern void cil_set_policy_version(cil_db_t *db, int policy_version);
extern void cil_write_policy_conf(FILE *out, struct cil_db *db);

enum cil_log_level {
	CIL_ERR = 1,
	CIL_WARN,
	CIL_INFO
};
extern void cil_set_log_level(enum cil_log_level lvl);
extern void cil_set_log_handler(void (*handler)(int lvl, char *msg));

#ifdef __GNUC__
__attribute__ ((format(printf, 2, 3)))
#endif
extern void cil_log(enum cil_log_level lvl, const char *msg, ...);

extern void cil_set_malloc_error_handler(void (*handler)(void));

#ifdef __cplusplus
}
#endif
#endif
