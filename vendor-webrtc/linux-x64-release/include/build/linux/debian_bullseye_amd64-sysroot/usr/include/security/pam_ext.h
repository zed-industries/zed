/*
 * Copyright (C) 2005, 2006, 2008, 2009 Thorsten Kukuk.
 *
 * <security/pam_ext.h>
 *
 * This header file collects definitions for the extended PAM API.
 * This is a public interface of the PAM library for PAM modules,
 * which makes the life of PAM developers easier, but are not documented
 * in any standard and are not portable between different PAM
 * implementations.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, and the entire permission notice in its entirety,
 *    including the disclaimer of warranties.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote
 *    products derived from this software without specific prior
 *    written permission.
 *
 * ALTERNATIVELY, this product may be distributed under the terms of
 * the GNU Public License, in which case the provisions of the GPL are
 * required INSTEAD OF the above restrictions.  (This clause is
 * necessary due to a potential bad interaction between the GPL and
 * the restrictions contained in a BSD-style copyright.)
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef _SECURITY__PAM_EXT_H_
#define _SECURITY__PAM_EXT_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <security/_pam_types.h>
#include <stdarg.h>

extern void PAM_FORMAT((printf, 3, 0)) PAM_NONNULL((3))
pam_vsyslog (const pam_handle_t *pamh, int priority,
             const char *fmt, va_list args);

extern void PAM_FORMAT((printf, 3, 4)) PAM_NONNULL((3))
pam_syslog (const pam_handle_t *pamh, int priority, const char *fmt, ...);

extern int PAM_FORMAT((printf, 4, 0)) PAM_NONNULL((1,4))
pam_vprompt (pam_handle_t *pamh, int style, char **response,
	     const char *fmt, va_list args);

extern int PAM_FORMAT((printf, 4, 5)) PAM_NONNULL((1,4))
pam_prompt (pam_handle_t *pamh, int style, char **response,
	    const char *fmt, ...);

#define pam_error(pamh, fmt...) \
	pam_prompt(pamh, PAM_ERROR_MSG, NULL, fmt)
#define pam_verror(pamh, fmt, args) \
	pam_vprompt(pamh, PAM_ERROR_MSG, NULL, fmt, args)

#define pam_info(pamh, fmt...) pam_prompt(pamh, PAM_TEXT_INFO, NULL, fmt)
#define pam_vinfo(pamh, fmt, args) pam_vprompt(pamh, PAM_TEXT_INFO, NULL, fmt, args)

extern int PAM_NONNULL((1,3))
pam_get_authtok (pam_handle_t *pamh, int item, const char **authtok,
		 const char *prompt);
extern int PAM_NONNULL((1,2))
pam_get_authtok_noverify (pam_handle_t *pamh, const char **authtok,
			  const char *prompt);
extern int PAM_NONNULL((1,2))
pam_get_authtok_verify (pam_handle_t *pamh, const char **authtok,
			const char *prompt);

#ifdef __cplusplus
}
#endif

#endif
