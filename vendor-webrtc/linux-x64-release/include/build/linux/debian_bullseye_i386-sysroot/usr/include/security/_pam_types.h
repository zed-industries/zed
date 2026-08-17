/*
 * <security/_pam_types.h>
 *
 * This file defines all of the types common to the Linux-PAM library
 * applications and modules.
 *
 * Note, the copyright+license information is at end of file.
 */

#ifndef _SECURITY__PAM_TYPES_H
#define _SECURITY__PAM_TYPES_H

/* This is a blind structure; users aren't allowed to see inside a
 * pam_handle_t, so we don't define struct pam_handle here.  This is
 * defined in a file private to the PAM library.  (i.e., it's private
 * to PAM service modules, too!)  */

typedef struct pam_handle pam_handle_t;

/* ---------------- The Linux-PAM Version defines ----------------- */

/* Major and minor version number of the Linux-PAM package.  Use
   these macros to test for features in specific releases.  */
#define __LINUX_PAM__ 1
#define __LINUX_PAM_MINOR__ 0

/* ----------------- The Linux-PAM return values ------------------ */

#define PAM_SUCCESS 0		/* Successful function return */
#define PAM_OPEN_ERR 1		/* dlopen() failure when dynamically */
				/* loading a service module */
#define PAM_SYMBOL_ERR 2	/* Symbol not found */
#define PAM_SERVICE_ERR 3	/* Error in service module */
#define PAM_SYSTEM_ERR 4	/* System error */
#define PAM_BUF_ERR 5		/* Memory buffer error */
#define PAM_PERM_DENIED 6	/* Permission denied */
#define PAM_AUTH_ERR 7		/* Authentication failure */
#define PAM_CRED_INSUFFICIENT 8	/* Can not access authentication data */
				/* due to insufficient credentials */
#define PAM_AUTHINFO_UNAVAIL 9	/* Underlying authentication service */
				/* can not retrieve authentication */
				/* information  */
#define PAM_USER_UNKNOWN 10	/* User not known to the underlying */
				/* authentication module */
#define PAM_MAXTRIES 11		/* An authentication service has */
				/* maintained a retry count which has */
				/* been reached.  No further retries */
				/* should be attempted */
#define PAM_NEW_AUTHTOK_REQD 12	/* New authentication token required. */
				/* This is normally returned if the */
				/* machine security policies require */
				/* that the password should be changed */
				/* because the password is NULL or it */
				/* has aged */
#define PAM_ACCT_EXPIRED 13	/* User account has expired */
#define PAM_SESSION_ERR 14	/* Can not make/remove an entry for */
				/* the specified session */
#define PAM_CRED_UNAVAIL 15	/* Underlying authentication service */
				/* can not retrieve user credentials */
                                /* unavailable */
#define PAM_CRED_EXPIRED 16	/* User credentials expired */
#define PAM_CRED_ERR 17		/* Failure setting user credentials */
#define PAM_NO_MODULE_DATA 18	/* No module specific data is present */
#define PAM_CONV_ERR 19		/* Conversation error */
#define PAM_AUTHTOK_ERR 20	/* Authentication token manipulation error */
#define PAM_AUTHTOK_RECOVERY_ERR 21 /* Authentication information */
				    /* cannot be recovered */
#define PAM_AUTHTOK_LOCK_BUSY 22   /* Authentication token lock busy */
#define PAM_AUTHTOK_DISABLE_AGING 23 /* Authentication token aging disabled */
#define PAM_TRY_AGAIN 24	/* Preliminary check by password service */
#define PAM_IGNORE 25		/* Ignore underlying account module */
				/* regardless of whether the control */
				/* flag is required, optional, or sufficient */
#define PAM_ABORT 26            /* Critical error (?module fail now request) */
#define PAM_AUTHTOK_EXPIRED  27 /* user's authentication token has expired */
#define PAM_MODULE_UNKNOWN   28 /* module is not known */

#define PAM_BAD_ITEM         29 /* Bad item passed to pam_*_item() */
#define PAM_CONV_AGAIN       30 /* conversation function is event driven
				     and data is not available yet */
#define PAM_INCOMPLETE       31 /* please call this function again to
				   complete authentication stack. Before
				   calling again, verify that conversation
				   is completed */

/*
 * Add new #define's here - take care to also extend the libpam code:
 * pam_strerror() and "libpam/pam_tokens.h" .
 */

#define _PAM_RETURN_VALUES 32   /* this is the number of return values */


/* ---------------------- The Linux-PAM flags -------------------- */

/* Authentication service should not generate any messages */
#define PAM_SILENT			0x8000U

/* Note: these flags are used by pam_authenticate{,_secondary}() */

/* The authentication service should return PAM_AUTH_ERROR if the
 * user has a null authentication token */
#define PAM_DISALLOW_NULL_AUTHTOK	0x0001U

/* Note: these flags are used for pam_setcred() */

/* Set user credentials for an authentication service */
#define PAM_ESTABLISH_CRED              0x0002U

/* Delete user credentials associated with an authentication service */
#define PAM_DELETE_CRED                 0x0004U

/* Reinitialize user credentials */
#define PAM_REINITIALIZE_CRED           0x0008U

/* Extend lifetime of user credentials */
#define PAM_REFRESH_CRED                0x0010U

/* Note: these flags are used by pam_chauthtok */

/* The password service should only update those passwords that have
 * aged.  If this flag is not passed, the password service should
 * update all passwords. */
#define PAM_CHANGE_EXPIRED_AUTHTOK	0x0020U

/* ------------------ The Linux-PAM item types ------------------- */

/* These defines are used by pam_set_item() and pam_get_item().
   Please check the spec which are allowed for use by applications
   and which are only allowed for use by modules. */

#define PAM_SERVICE	   1	/* The service name */
#define PAM_USER           2	/* The user name */
#define PAM_TTY            3	/* The tty name */
#define PAM_RHOST          4	/* The remote host name */
#define PAM_CONV           5	/* The pam_conv structure */
#define PAM_AUTHTOK        6	/* The authentication token (password) */
#define PAM_OLDAUTHTOK     7	/* The old authentication token */
#define PAM_RUSER          8	/* The remote user name */
#define PAM_USER_PROMPT    9    /* the prompt for getting a username */
/* Linux-PAM extensions */
#define PAM_FAIL_DELAY     10   /* app supplied function to override failure
				   delays */
#define PAM_XDISPLAY       11   /* X display name */
#define PAM_XAUTHDATA      12   /* X server authentication data */
#define PAM_AUTHTOK_TYPE   13   /* The type for pam_get_authtok */

/* -------------- Special defines used by Linux-PAM -------------- */

#if defined(__GNUC__) && defined(__GNUC_MINOR__)
# define PAM_GNUC_PREREQ(maj, min) \
        ((__GNUC__ << 16) + __GNUC_MINOR__ >= ((maj) << 16) + (min))
#else
# define PAM_GNUC_PREREQ(maj, min) 0
#endif

#if PAM_GNUC_PREREQ(2,5)
# define PAM_FORMAT(params) __attribute__((__format__ params))
#else
# define PAM_FORMAT(params)
#endif

#if PAM_GNUC_PREREQ(3,3) && !defined(LIBPAM_COMPILE)
# define PAM_NONNULL(params) __attribute__((__nonnull__ params))
#else
# define PAM_NONNULL(params)
#endif

/* ---------- Common Linux-PAM application/module PI ----------- */

extern int PAM_NONNULL((1))
pam_set_item(pam_handle_t *pamh, int item_type, const void *item);

extern int PAM_NONNULL((1))
pam_get_item(const pam_handle_t *pamh, int item_type, const void **item);

extern const char *
pam_strerror(pam_handle_t *pamh, int errnum);

extern int PAM_NONNULL((1,2))
pam_putenv(pam_handle_t *pamh, const char *name_value);

extern const char * PAM_NONNULL((1,2))
pam_getenv(pam_handle_t *pamh, const char *name);

extern char ** PAM_NONNULL((1))
pam_getenvlist(pam_handle_t *pamh);

/* ---------- Common Linux-PAM application/module PI ----------- */

/*
 * here are some proposed error status definitions for the
 * 'error_status' argument used by the cleanup function associated
 * with data items they should be logically OR'd with the error_status
 * of the latest return from libpam -- new with .52 and positive
 * impression from Sun although not official as of 1996/9/4
 * [generally the other flags are to be found in pam_modules.h]
 */

#define PAM_DATA_SILENT    0x40000000     /* used to suppress messages... */

/*
 * here we define an externally (by apps or modules) callable function
 * that primes the libpam library to delay when a stacked set of
 * modules results in a failure. In the case of PAM_SUCCESS this delay
 * is ignored.
 *
 * Note, the pam_[gs]et_item(... PAM_FAIL_DELAY ...) can be used to set
 * a function pointer which can override the default fail-delay behavior.
 * This item was added to accommodate event driven programs that need to
 * manage delays more carefully.  The function prototype for this data
 * item is
 *     void (*fail_delay)(int status, unsigned int delay, void *appdata_ptr);
 */

#define HAVE_PAM_FAIL_DELAY
extern int pam_fail_delay(pam_handle_t *pamh, unsigned int musec_delay);

/* ------------ The Linux-PAM conversation structures ------------ */

/* Message styles */

#define PAM_PROMPT_ECHO_OFF	1
#define PAM_PROMPT_ECHO_ON	2
#define PAM_ERROR_MSG		3
#define PAM_TEXT_INFO		4

/* Linux-PAM specific types */

#define PAM_RADIO_TYPE          5        /* yes/no/maybe conditionals */

/* This is for server client non-human interaction.. these are NOT
   part of the X/Open PAM specification. */

#define PAM_BINARY_PROMPT       7

/* maximum size of messages/responses etc.. (these are mostly
   arbitrary so Linux-PAM should handle longer values). */

#define PAM_MAX_NUM_MSG       32
#define PAM_MAX_MSG_SIZE      512
#define PAM_MAX_RESP_SIZE     512

/* Used to pass prompting text, error messages, or other informatory
 * text to the user.  This structure is allocated and freed by the PAM
 * library (or loaded module).  */

struct pam_message {
    int msg_style;
    const char *msg;
};

/* if the pam_message.msg_style = PAM_BINARY_PROMPT
   the 'pam_message.msg' is a pointer to a 'const *' for the following
   pseudo-structure.  When used with a PAM_BINARY_PROMPT, the returned
   pam_response.resp pointer points to an object with the following
   structure:

   struct {
       u32 length;                         #  network byte order
       unsigned char type;
       unsigned char data[length-5];
   };

   The 'libpamc' library is designed around this flavor of
   message and should be used to handle this flavor of msg_style.
   */

/* Used to return the user's response to the PAM library.  This
   structure is allocated by the application program, and free()'d by
   the Linux-PAM library (or calling module).  */

struct pam_response {
    char *resp;
    int	resp_retcode;	/* currently un-used, zero expected */
};

/* The actual conversation structure itself */

struct pam_conv {
    int (*conv)(int num_msg, const struct pam_message **msg,
		struct pam_response **resp, void *appdata_ptr);
    void *appdata_ptr;
};

/* Used by the PAM_XAUTHDATA pam item.  Contains X authentication
   data used by modules to connect to the user's X display.  Note:
   this structure is intentionally compatible with xcb_auth_info_t. */

struct pam_xauth_data {
    int namelen;
    char *name;
    int datalen;
    char *data;
};

/* ... adapted from the pam_appl.h file created by Theodore Ts'o and
 *
 * Copyright Theodore Ts'o, 1996.  All rights reserved.
 * Copyright (c) Andrew G. Morgan <morgan@linux.kernel.org>, 1996-8
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
 * OF THE POSSIBILITY OF SUCH DAMAGE.  */

#endif /* _SECURITY__PAM_TYPES_H */
