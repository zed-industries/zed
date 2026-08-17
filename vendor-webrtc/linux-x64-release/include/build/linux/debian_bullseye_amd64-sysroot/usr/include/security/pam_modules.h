/*
 * <security/pam_modules.h>
 *
 * This header file collects definitions for the PAM API --- that is,
 * public interface between the PAM library and PAM modules.
 *
 * Note, the copyright information is at end of file.
 */

#ifndef _SECURITY_PAM_MODULES_H
#define _SECURITY_PAM_MODULES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <security/_pam_types.h>      /* Linux-PAM common defined types */

/* -------------- The Linux-PAM Module PI ------------- */

extern int PAM_NONNULL((1,2))
pam_set_data(pam_handle_t *pamh, const char *module_data_name, void *data,
	     void (*cleanup)(pam_handle_t *pamh, void *data,
			     int error_status));

extern int PAM_NONNULL((1,2,3))
pam_get_data(const pam_handle_t *pamh, const char *module_data_name,
	     const void **data);

extern int PAM_NONNULL((1,2))
pam_get_user(pam_handle_t *pamh, const char **user, const char *prompt);

/* Authentication API's */
int pam_sm_authenticate(pam_handle_t *pamh, int flags,
                        int argc, const char **argv);
int pam_sm_setcred(pam_handle_t *pamh, int flags,
		   int argc, const char **argv);

/* Account Management API's */
int pam_sm_acct_mgmt(pam_handle_t *pamh, int flags,
		     int argc, const char **argv);

/* Session Management API's */
int pam_sm_open_session(pam_handle_t *pamh, int flags,
			int argc, const char **argv);

int pam_sm_close_session(pam_handle_t *pamh, int flags,
			 int argc, const char **argv);

/* Password Management API's */
int pam_sm_chauthtok(pam_handle_t *pamh, int flags,
		     int argc, const char **argv);

/* The following two flags are for use across the Linux-PAM/module
 * interface only. The Application is not permitted to use these
 * tokens.
 *
 * The password service should only perform preliminary checks.  No
 * passwords should be updated. */
#define PAM_PRELIM_CHECK		0x4000

/* The password service should update passwords Note: PAM_PRELIM_CHECK
 * and PAM_UPDATE_AUTHTOK cannot both be set simultaneously! */
#define PAM_UPDATE_AUTHTOK		0x2000


/*
 * here are some proposed error status definitions for the
 * 'error_status' argument used by the cleanup function associated
 * with data items they should be logically OR'd with the error_status
 * of the latest return from libpam -- new with .52 and positive
 * impression from Sun although not official as of 1996/9/4 there are
 * others in _pam_types.h -- they are for common module/app use.
 */

#define PAM_DATA_REPLACE   0x20000000     /* used when replacing a data item */

/* PAM_EXTERN isn't needed anymore, but don't remove it to not break
   lot of external code using it. */
#define PAM_EXTERN extern

/* take care of any compatibility issues */
#include <security/_pam_compat.h>

#ifdef __cplusplus
}
#endif

/* Copyright (C) Theodore Ts'o, 1996.
 * Copyright (C) Andrew Morgan, 1996-8.
 *                                                All rights reserved.
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
 * the GNU General Public License, in which case the provisions of the
 * GNU GPL are required INSTEAD OF the above restrictions.  (This
 * clause is necessary due to a potential bad interaction between the
 * GNU GPL and the restrictions contained in a BSD-style copyright.)
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
 * OF THE POSSIBILITY OF SUCH DAMAGE.  */

#endif /* _SECURITY_PAM_MODULES_H */
