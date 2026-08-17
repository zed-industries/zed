/*
 * <sys/capability.h>
 *
 * Copyright (C) 1997   Aleph One
 * Copyright (C) 1997,8, 2008,19,20 Andrew G. Morgan <morgan@kernel.org>
 *
 * defunct POSIX.1e Standard: 25.2 Capabilities           <sys/capability.h>
 */

#ifndef _SYS_CAPABILITY_H
#define _SYS_CAPABILITY_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * This file complements the kernel file by providing prototype
 * information for the user library.
 */

#include <sys/types.h>
#include <stdint.h>
#include <linux/types.h>

#ifndef __user
#define __user
#endif
#include <linux/capability.h>

/*
 * POSIX capability types
 */

/*
 * Opaque capability handle (defined internally by libcap)
 * internal capability representation
 */
typedef struct _cap_struct *cap_t;

/* "external" capability representation is a (void *) */

/*
 * This is the type used to identify capabilities
 */

typedef int cap_value_t;

/*
 * libcap initialized first unnamed capability of the running kernel.
 * capsh includes a runtime test to flag when this is larger than
 * what is known to libcap... Time for a new libcap release!
 */
extern cap_value_t cap_max_bits(void);

/*
 * Set identifiers
 */
typedef enum {
    CAP_EFFECTIVE = 0,                 /* Specifies the effective flag */
    CAP_PERMITTED = 1,                 /* Specifies the permitted flag */
    CAP_INHERITABLE = 2                /* Specifies the inheritable flag */
} cap_flag_t;

typedef enum {
    CAP_IAB_INH = 2,
    CAP_IAB_AMB = 3,
    CAP_IAB_BOUND = 4
} cap_iab_vector_t;

/*
 * An opaque generalization of the inheritable bits that includes both
 * what ambient bits to raise and what bounding bits to *lower* (aka
 * drop).  None of these bits once set, using cap_iab_set(), affect
 * the running process but are consulted, through the execve() system
 * call, by the kernel. Note, the ambient bits ('A') of the running
 * process are fragile with respect to other aspects of the "posix"
 * (cap_t) operations: most importantly, 'A' cannot ever hold bits not
 * present in the intersection of 'pI' and 'pP'. The kernel
 * immediately drops all ambient caps whenever such a situation
 * arises. Typically, the ambient bits are used to support a naive
 * capability inheritance model - at odds with the POSIX (sic) model
 * of inheritance where inherited (pI) capabilities need to also be
 * wanted by the executed binary (fI) in order to become raised
 * through exec.
 */
typedef struct cap_iab_s *cap_iab_t;

/*
 * These are the states available to each capability
 */
typedef enum {
    CAP_CLEAR=0,                            /* The flag is cleared/disabled */
    CAP_SET=1                                    /* The flag is set/enabled */
} cap_flag_value_t;

/*
 * User-space capability manipulation routines
 */
typedef unsigned cap_mode_t;
#define CAP_MODE_UNCERTAIN    ((cap_mode_t) 0)
#define CAP_MODE_NOPRIV       ((cap_mode_t) 1)
#define CAP_MODE_PURE1E_INIT  ((cap_mode_t) 2)
#define CAP_MODE_PURE1E       ((cap_mode_t) 3)

/* libcap/cap_alloc.c */
extern cap_t      cap_dup(cap_t);
extern int        cap_free(void *);
extern cap_t      cap_init(void);
extern cap_iab_t  cap_iab_init(void);

/* libcap/cap_flag.c */
extern int     cap_get_flag(cap_t, cap_value_t, cap_flag_t, cap_flag_value_t *);
extern int     cap_set_flag(cap_t, cap_flag_t, int, const cap_value_t *,
			    cap_flag_value_t);
extern int     cap_clear(cap_t);
extern int     cap_clear_flag(cap_t, cap_flag_t);

extern cap_flag_value_t cap_iab_get_vector(cap_iab_t, cap_iab_vector_t,
					 cap_value_t);
extern int     cap_iab_set_vector(cap_iab_t, cap_iab_vector_t, cap_value_t,
				cap_flag_value_t);
extern int     cap_iab_fill(cap_iab_t, cap_iab_vector_t, cap_t, cap_flag_t);

/* libcap/cap_file.c */
extern cap_t   cap_get_fd(int);
extern cap_t   cap_get_file(const char *);
extern uid_t   cap_get_nsowner(cap_t);
extern int     cap_set_fd(int, cap_t);
extern int     cap_set_file(const char *, cap_t);
extern int     cap_set_nsowner(cap_t, uid_t);

/* libcap/cap_proc.c */
extern cap_t   cap_get_proc(void);
extern cap_t   cap_get_pid(pid_t);
extern int     cap_set_proc(cap_t);

extern int     cap_get_bound(cap_value_t);
extern int     cap_drop_bound(cap_value_t);
#define CAP_IS_SUPPORTED(cap)  (cap_get_bound(cap) >= 0)

extern int     cap_get_ambient(cap_value_t);
extern int     cap_set_ambient(cap_value_t, cap_flag_value_t);
extern int     cap_reset_ambient(void);
#define CAP_AMBIENT_SUPPORTED() (cap_get_ambient(CAP_CHOWN) >= 0)

/* libcap/cap_extint.c */
extern ssize_t cap_size(cap_t);
extern ssize_t cap_copy_ext(void *, cap_t, ssize_t);
extern cap_t   cap_copy_int(const void *);

/* libcap/cap_text.c */
extern cap_t   cap_from_text(const char *);
extern char *  cap_to_text(cap_t, ssize_t *);
extern int     cap_from_name(const char *, cap_value_t *);
extern char *  cap_to_name(cap_value_t);

extern char *     cap_iab_to_text(cap_iab_t iab);
extern cap_iab_t  cap_iab_from_text(const char *text);

#define CAP_DIFFERS(result, flag)  (((result) & (1 << (flag))) != 0)
extern int     cap_compare(cap_t, cap_t);

/* libcap/cap_proc.c */
extern void cap_set_syscall(long int (*new_syscall)(long int,
				long int, long int, long int),
			    long int (*new_syscall6)(long int,
				long int, long int, long int,
				long int, long int, long int));

extern int cap_set_mode(cap_mode_t flavor);
extern cap_mode_t cap_get_mode(void);
extern const char *cap_mode_name(cap_mode_t flavor);

extern unsigned cap_get_secbits(void);
extern int cap_set_secbits(unsigned bits);

extern int cap_setuid(uid_t uid);
extern int cap_setgroups(gid_t gid, size_t ngroups, const gid_t groups[]);

extern cap_iab_t cap_iab_get_proc(void);
extern int cap_iab_set_proc(cap_iab_t iab);

typedef struct cap_launch_s *cap_launch_t;

extern cap_launch_t cap_new_launcher(const char *arg0, const char * const *argv,
				     const char * const *envp);
extern void cap_launcher_callback(cap_launch_t attr,
				  int (callback_fn)(void *detail));
extern void cap_launcher_setuid(cap_launch_t attr, uid_t uid);
extern void cap_launcher_setgroups(cap_launch_t attr, gid_t gid,
				   int ngroups, const gid_t *groups);
extern void cap_launcher_set_mode(cap_launch_t attr, cap_mode_t flavor);
extern cap_iab_t cap_launcher_set_iab(cap_launch_t attr, cap_iab_t iab);
extern void cap_launcher_set_chroot(cap_launch_t attr, const char *chroot);
extern pid_t cap_launch(cap_launch_t attr, void *data);

/*
 * system calls - look to libc for function to system call
 * mapping. Note, libcap does not use capset directly, but permits the
 * cap_set_syscall() to redirect the system call function.
 */
extern int capget(cap_user_header_t header, cap_user_data_t data);
extern int capset(cap_user_header_t header, const cap_user_data_t data);

/* deprecated - use cap_get_pid() */
extern int capgetp(pid_t pid, cap_t cap_d);

/* not valid with filesystem capability support - use cap_set_proc() */
extern int capsetp(pid_t pid, cap_t cap_d);

#ifdef __cplusplus
}
#endif

#endif /* _SYS_CAPABILITY_H */
