/*
 * Copyright (c) 2019 Andrew G. Morgan <morgan@kernel.org>
 *
 * This header, and the -lpsx library, provide a number of things to
 * support POSIX semantics for syscalls associated with the pthread
 * library. Linking this code is tricky and is done as follows:
 *
 *     ld ... -lpsx -lpthread --wrap=pthread_create
 * or, gcc ... -lpsx -lpthread -Wl,-wrap,pthread_create
 *
 * glibc provides a subset of this functionality natively through the
 * nptl:setxid mechanism and could implement psx_syscall() directly
 * using that style of functionality but, as of 2019-11-30, the setxid
 * mechanism is limited to 9 specific set*() syscalls that do not
 * support the syscall6 API (needed for prctl functions and the ambient
 * capabilities set for example).
 */

#ifndef _SYS_PSX_SYSCALL_H
#define _SYS_PSX_SYSCALL_H

#ifdef __cplusplus
extern "C" {
#endif

#include <pthread.h>

/*
 * This function is actually provided by the linker trick:
 *
 *   gcc ... -lpsx -lpthread -Wl,-wrap,pthread_create
 */
int __real_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
			  void *(*start_routine) (void *), void *arg);

/*
 * psx_syscall performs the specified syscall on all psx registered
 * threads. The mechanism by which this occurs is much less efficient
 * than a standard system call on Linux, so it should only be used
 * when POSIX semantics are required to change process relevant
 * security state.
 *
 * Glibc has native support for POSIX semantics on setgroups() and the
 * 8 set*[gu]id() functions. So, there is no need to use psx_syscall()
 * for these calls. This call exists for all the other system calls
 * that need to maintain parity on all pthreads of a program.
 *
 * Some macrology is used to allow the caller to provide only as many
 * arguments as needed, thus psx_syscall() cannot be used as a
 * function pointer. For those situations, we define psx_syscall3()
 * and psx_syscall6().
 */
#define psx_syscall(syscall_nr, ...) \
    __psx_syscall(syscall_nr, __VA_ARGS__, 6, 5, 4, 3, 2, 1, 0)
long int __psx_syscall(long int syscall_nr, ...);
long int psx_syscall3(long int syscall_nr,
		      long int arg1, long int arg2, long int arg3);
long int psx_syscall6(long int syscall_nr,
		      long int arg1, long int arg2, long int arg3,
		      long int arg4, long int arg5, long int arg6);

/*
 * psx_pthread_create() wraps the -lpthread pthread_create() function
 * call and registers the generated thread with the psx_syscall
 * infrastructure.
 *
 * Note, to transparently redirect all the pthread_create() calls in
 * your binary to psx_pthread_create(), link with:
 *
 *   gcc ... -lpsx -lpthread -Wl,-wrap,pthread_create
 *
 * [That is, libpsx contains an internal definition for the
 * __wrap_pthread_create function to invoke psx_pthread_create
 * functionality instead.]
 */
int psx_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
		       void *(*start_routine) (void *), void *arg);

/*
 * This function should be used by systems to obtain pointers to the
 * two syscall functions provided by the PSX library. A linkage trick
 * is to define this function as weak in a library that can optionally
 * use libpsx and then, should the caller link -lpsx, that library can
 * implicitly use these POSIX semantics syscalls. See libcap for an
 * example of this useage.
 */
void psx_load_syscalls(long int (**syscall_fn)(long int,
					       long int, long int, long int),
		       long int (**syscall6_fn)(long int,
						long int, long int, long int,
						long int, long int, long int));

#ifdef __cplusplus
}
#endif

#endif /* _SYS_PSX_SYSCALL_H */
