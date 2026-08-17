/* Definitions for POSIX spawn interface.
   Copyright (C) 2000-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef	_SPAWN_H
#define	_SPAWN_H	1

#include <features.h>
#include <sched.h>
#include <sys/types.h>
#include <bits/types/sigset_t.h>


/* Data structure to contain attributes for thread creation.  */
typedef struct
{
  short int __flags;
  pid_t __pgrp;
  sigset_t __sd;
  sigset_t __ss;
  struct sched_param __sp;
  int __policy;
  int __pad[16];
} posix_spawnattr_t;


/* Data structure to contain information about the actions to be
   performed in the new process with respect to file descriptors.  */
typedef struct
{
  int __allocated;
  int __used;
  struct __spawn_action *__actions;
  int __pad[16];
} posix_spawn_file_actions_t;


/* Flags to be set in the `posix_spawnattr_t'.  */
#define POSIX_SPAWN_RESETIDS		0x01
#define POSIX_SPAWN_SETPGROUP		0x02
#define POSIX_SPAWN_SETSIGDEF		0x04
#define POSIX_SPAWN_SETSIGMASK		0x08
#define POSIX_SPAWN_SETSCHEDPARAM	0x10
#define POSIX_SPAWN_SETSCHEDULER	0x20
#ifdef __USE_GNU
# define POSIX_SPAWN_USEVFORK		0x40
# define POSIX_SPAWN_SETSID		0x80
#endif


__BEGIN_DECLS

/* Spawn a new process executing PATH with the attributes describes in *ATTRP.
   Before running the process perform the actions described in FILE-ACTIONS.

   This function is a possible cancellation point and therefore not
   marked with __THROW. */
extern int posix_spawn (pid_t *__restrict __pid,
			const char *__restrict __path,
			const posix_spawn_file_actions_t *__restrict
			__file_actions,
			const posix_spawnattr_t *__restrict __attrp,
			char *const __argv[__restrict_arr],
			char *const __envp[__restrict_arr])
    __nonnull ((2, 5));

/* Similar to `posix_spawn' but search for FILE in the PATH.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int posix_spawnp (pid_t *__pid, const char *__file,
			 const posix_spawn_file_actions_t *__file_actions,
			 const posix_spawnattr_t *__attrp,
			 char *const __argv[], char *const __envp[])
    __nonnull ((2, 5));


/* Initialize data structure with attributes for `spawn' to default values.  */
extern int posix_spawnattr_init (posix_spawnattr_t *__attr)
    __THROW __nonnull ((1));

/* Free resources associated with ATTR.  */
extern int posix_spawnattr_destroy (posix_spawnattr_t *__attr)
    __THROW __nonnull ((1));

/* Store signal mask for signals with default handling from ATTR in
   SIGDEFAULT.  */
extern int posix_spawnattr_getsigdefault (const posix_spawnattr_t *
					  __restrict __attr,
					  sigset_t *__restrict __sigdefault)
     __THROW __nonnull ((1, 2));

/* Set signal mask for signals with default handling in ATTR to SIGDEFAULT.  */
extern int posix_spawnattr_setsigdefault (posix_spawnattr_t *__restrict __attr,
					  const sigset_t *__restrict
					  __sigdefault)
     __THROW __nonnull ((1, 2));

/* Store signal mask for the new process from ATTR in SIGMASK.  */
extern int posix_spawnattr_getsigmask (const posix_spawnattr_t *__restrict
				       __attr,
				       sigset_t *__restrict __sigmask)
    __THROW __nonnull ((1, 2));

/* Set signal mask for the new process in ATTR to SIGMASK.  */
extern int posix_spawnattr_setsigmask (posix_spawnattr_t *__restrict __attr,
				       const sigset_t *__restrict __sigmask)
     __THROW __nonnull ((1, 2));

/* Get flag word from the attribute structure.  */
extern int posix_spawnattr_getflags (const posix_spawnattr_t *__restrict
				     __attr,
				     short int *__restrict __flags)
     __THROW __nonnull ((1, 2));

/* Store flags in the attribute structure.  */
extern int posix_spawnattr_setflags (posix_spawnattr_t *_attr,
				     short int __flags)
     __THROW __nonnull ((1));

/* Get process group ID from the attribute structure.  */
extern int posix_spawnattr_getpgroup (const posix_spawnattr_t *__restrict
				      __attr, pid_t *__restrict __pgroup)
     __THROW __nonnull ((1, 2));

/* Store process group ID in the attribute structure.  */
extern int posix_spawnattr_setpgroup (posix_spawnattr_t *__attr,
				      pid_t __pgroup)
     __THROW __nonnull ((1));

/* Get scheduling policy from the attribute structure.  */
extern int posix_spawnattr_getschedpolicy (const posix_spawnattr_t *
					   __restrict __attr,
					   int *__restrict __schedpolicy)
     __THROW __nonnull ((1, 2));

/* Store scheduling policy in the attribute structure.  */
extern int posix_spawnattr_setschedpolicy (posix_spawnattr_t *__attr,
					   int __schedpolicy)
     __THROW __nonnull ((1));

/* Get scheduling parameters from the attribute structure.  */
extern int posix_spawnattr_getschedparam (const posix_spawnattr_t *
					  __restrict __attr,
					  struct sched_param *__restrict
					  __schedparam)
     __THROW __nonnull ((1, 2));

/* Store scheduling parameters in the attribute structure.  */
extern int posix_spawnattr_setschedparam (posix_spawnattr_t *__restrict __attr,
					  const struct sched_param *
					  __restrict __schedparam)
     __THROW __nonnull ((1, 2));


/* Initialize data structure for file attribute for `spawn' call.  */
extern int posix_spawn_file_actions_init (posix_spawn_file_actions_t *
					  __file_actions)
     __THROW __nonnull ((1));

/* Free resources associated with FILE-ACTIONS.  */
extern int posix_spawn_file_actions_destroy (posix_spawn_file_actions_t *
					     __file_actions)
     __THROW __nonnull ((1));

/* Add an action to FILE-ACTIONS which tells the implementation to call
   `open' for the given file during the `spawn' call.  */
extern int posix_spawn_file_actions_addopen (posix_spawn_file_actions_t *
					     __restrict __file_actions,
					     int __fd,
					     const char *__restrict __path,
					     int __oflag, mode_t __mode)
     __THROW __nonnull ((1, 3));

/* Add an action to FILE-ACTIONS which tells the implementation to call
   `close' for the given file descriptor during the `spawn' call.  */
extern int posix_spawn_file_actions_addclose (posix_spawn_file_actions_t *
					      __file_actions, int __fd)
     __THROW __nonnull ((1));

/* Add an action to FILE-ACTIONS which tells the implementation to call
   `dup2' for the given file descriptors during the `spawn' call.  */
extern int posix_spawn_file_actions_adddup2 (posix_spawn_file_actions_t *
					     __file_actions,
					     int __fd, int __newfd)
     __THROW __nonnull ((1));

#ifdef __USE_GNU
/* Add an action changing the directory to PATH during spawn.  This
   affects the subsequent file actions.  */
extern int posix_spawn_file_actions_addchdir_np (posix_spawn_file_actions_t *
						 __restrict __actions,
						 const char *__restrict __path)
     __THROW __nonnull ((1, 2));

/* Add an action changing the directory to FD during spawn.  This
   affects the subsequent file actions.  FD is not duplicated and must
   be open when the file action is executed.  */
extern int posix_spawn_file_actions_addfchdir_np (posix_spawn_file_actions_t *,
						  int __fd)
     __THROW __nonnull ((1));
#endif

__END_DECLS

#endif /* spawn.h */
