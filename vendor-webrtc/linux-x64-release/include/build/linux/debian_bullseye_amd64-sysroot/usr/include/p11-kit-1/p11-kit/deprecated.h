/*
 * Copyright (c) 2013 Red Hat Inc.
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
 * Author: Stef Walter <stefw@redhat.com>
 */

#ifndef __P11_KIT_DEPRECATED_H__
#define __P11_KIT_DEPRECATED_H__

#ifndef __P11_KIT_H__
#error "Please include <p11-kit/p11-kit.h> instead of this file."
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifndef P11_KIT_NO_DEPRECATIONS
#if __GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 5)
#define P11_KIT_DEPRECATED_FOR(f) __attribute__((deprecated("Use " #f " instead")))
#elif __GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 1)
#define P11_KIT_DEPRECATED_FOR(f) __attribute__((__deprecated__))
#endif
#endif

#ifndef P11_KIT_DEPRECATED_FOR
#define P11_KIT_DEPRECATED_FOR(f)
#endif

#ifndef P11_KIT_DISABLE_DEPRECATED

P11_KIT_DEPRECATED_FOR (p11_kit_modules_load)
CK_RV                    p11_kit_initialize_registered     (void);

P11_KIT_DEPRECATED_FOR (p11_kit_modules_release)
CK_RV                    p11_kit_finalize_registered       (void);

P11_KIT_DEPRECATED_FOR (p11_kit_modules_release)
CK_FUNCTION_LIST_PTR *   p11_kit_registered_modules        (void);

P11_KIT_DEPRECATED_FOR (p11_kit_module_for_name)
CK_FUNCTION_LIST_PTR     p11_kit_registered_name_to_module (const char *name);

P11_KIT_DEPRECATED_FOR (p11_kit_module_get_name)
char *                   p11_kit_registered_module_to_name (CK_FUNCTION_LIST_PTR module);

P11_KIT_DEPRECATED_FOR (p11_kit_config_option)
char *                   p11_kit_registered_option         (CK_FUNCTION_LIST_PTR module,
                                                            const char *field);

P11_KIT_DEPRECATED_FOR (module->C_Initialize)
CK_RV                    p11_kit_initialize_module         (CK_FUNCTION_LIST_PTR module);

P11_KIT_DEPRECATED_FOR (module->C_Finalize)
CK_RV                    p11_kit_finalize_module           (CK_FUNCTION_LIST_PTR module);

P11_KIT_DEPRECATED_FOR (p11_kit_module_load)
CK_RV                    p11_kit_load_initialize_module    (const char *module_path,
                                                            CK_FUNCTION_LIST_PTR *module);

#endif /* P11_KIT_DISABLE_DEPRECATED */

#undef P11_KIT_DEPRECATED_FOR

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __P11_KIT_DEPRECATED_H__ */
