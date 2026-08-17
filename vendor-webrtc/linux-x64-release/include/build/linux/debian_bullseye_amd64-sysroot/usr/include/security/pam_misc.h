/* $Id$ */

#ifndef __PAMMISC_H
#define __PAMMISC_H

#include <security/pam_appl.h>
#include <security/pam_client.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* include some useful macros */

#include <security/_pam_macros.h>

/* functions defined in pam_misc.* libraries */

extern int misc_conv(int num_msg, const struct pam_message **msgm,
		     struct pam_response **response, void *appdata_ptr);

#include <time.h>

extern time_t pam_misc_conv_warn_time; /* time that we should warn user */
extern time_t pam_misc_conv_die_time;         /* cut-off time for input */
extern const char *pam_misc_conv_warn_line;           /* warning notice */
extern const char *pam_misc_conv_die_line;            /* cut-off remark */
extern int pam_misc_conv_died;      /* 1 = cut-off time reached (0 not) */
extern int (*pam_binary_handler_fn)(void *appdata, pamc_bp_t *prompt_p);
extern void (*pam_binary_handler_free)(void *appdata, pamc_bp_t *prompt_p);
/*
 * Environment helper functions
 */

/* transcribe given environment (to pam) */
extern int pam_misc_paste_env(pam_handle_t *pamh
			      , const char * const * user_env);

/* delete environment as obtained from (pam_getenvlist) */
extern char **pam_misc_drop_env(char **env);

/* provide something like the POSIX setenv function for the (Linux-)PAM
 * environment. */

extern int pam_misc_setenv(pam_handle_t *pamh, const char *name
			   , const char *value, int readonly);

#ifdef __cplusplus
}
#endif /* def __cplusplus */

#endif /* ndef __PAMMISC_H */
