/*
 * Copyright (c) 2011 Collabora Ltd.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *     * Redistributions of source code must retain the above
 *       copyright notice, this list of conditions and the
 *       following disclaimer.
 *     * Redistributions in binary form must reproduce the
 *       above copyright notice, this list of conditions and
 *       the following disclaimer in the documentation and/or
 *       other materials provided with the distribution.
 *     * The names of contributors to this software may not be
 *       used to endorse or promote products derived from this
 *       software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 * OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED
 * AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * Author: Stef Walter <stefw@collabora.co.uk>
 */

#ifndef __P11_KIT_H__
#define __P11_KIT_H__

#include "p11-kit/pkcs11.h"

/*
 * If the caller is using the PKCS#11 GNU calling convention, then we cater
 * to that here.
 */
#ifdef CRYPTOKI_GNU
typedef ck_rv_t CK_RV;
typedef struct ck_function_list* CK_FUNCTION_LIST_PTR;
typedef struct ck_function_list CK_FUNCTION_LIST;
#endif

#include "p11-kit/deprecated.h"

#ifdef __cplusplus
extern "C" {
#endif

enum {
	P11_KIT_MODULE_UNMANAGED = 1 << 0,
	P11_KIT_MODULE_CRITICAL = 1 << 1,
	P11_KIT_MODULE_TRUSTED = 1 << 2,
	P11_KIT_MODULE_VERBOSE = 1 << 3,
	P11_KIT_MODULE_MASK = (1 << 4) - 1
};

typedef void        (* p11_kit_destroyer)                   (void *data);

CK_FUNCTION_LIST **    p11_kit_modules_load                 (const char *reserved,
                                                             int flags);

CK_RV                  p11_kit_modules_initialize           (CK_FUNCTION_LIST **modules,
                                                             p11_kit_destroyer failure_callback);

CK_FUNCTION_LIST **    p11_kit_modules_load_and_initialize  (int flags);

CK_RV                  p11_kit_modules_finalize             (CK_FUNCTION_LIST **modules);

void                   p11_kit_modules_release              (CK_FUNCTION_LIST **modules);

void                   p11_kit_modules_finalize_and_release (CK_FUNCTION_LIST **modules);

CK_FUNCTION_LIST *     p11_kit_module_for_name              (CK_FUNCTION_LIST **modules,
                                                             const char *name);

char *                 p11_kit_module_get_filename          (CK_FUNCTION_LIST *module);
char *                 p11_kit_module_get_name              (CK_FUNCTION_LIST *module);

int                    p11_kit_module_get_flags             (CK_FUNCTION_LIST *module);

CK_FUNCTION_LIST *     p11_kit_module_load                  (const char *module_path,
                                                             int flags);

CK_RV                  p11_kit_module_initialize            (CK_FUNCTION_LIST *module);

CK_RV                  p11_kit_module_finalize              (CK_FUNCTION_LIST *module);

void                   p11_kit_module_release               (CK_FUNCTION_LIST *module);

char *                 p11_kit_config_option                (CK_FUNCTION_LIST *module,
                                                             const char *option);

const char*            p11_kit_strerror                     (CK_RV rv);

size_t                 p11_kit_space_strlen                 (const unsigned char *string,
                                                             size_t max_length);

char*                  p11_kit_space_strdup                 (const unsigned char *string,
                                                             size_t max_length);

void                   p11_kit_be_quiet                     (void);

void                   p11_kit_be_loud                      (void);

#ifdef P11_KIT_FUTURE_UNSTABLE_API

void                   p11_kit_set_progname                 (const char *progname);

void                   p11_kit_override_system_files        (const char *system_conf,
                                                             const char *user_conf,
                                                             const char *package_modules,
                                                             const char *system_modules,
						             const char *user_modules);
#endif

const char *           p11_kit_message                      (void);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __P11_KIT_H__ */
