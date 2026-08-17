/* Callback interface for libthread_db, functions users must define.
   Copyright (C) 1999-2020 Free Software Foundation, Inc.
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

#ifndef _PROC_SERVICE_H
#define _PROC_SERVICE_H 1

/* The definitions in this file must correspond to those in the debugger.  */
#include <sys/procfs.h>

__BEGIN_DECLS

/* Functions in this interface return one of these status codes.  */
typedef enum
{
  PS_OK,		/* Generic "call succeeded".  */
  PS_ERR,		/* Generic error. */
  PS_BADPID,		/* Bad process handle.  */
  PS_BADLID,		/* Bad LWP identifier.  */
  PS_BADADDR,		/* Bad address.  */
  PS_NOSYM,		/* Could not find given symbol.  */
  PS_NOFREGS		/* FPU register set not available for given LWP.  */
} ps_err_e;


/* This type is opaque in this interface.
   It's defined by the user of libthread_db.  */
struct ps_prochandle;


/* Read or write process memory at the given address.  */
extern ps_err_e ps_pdread (struct ps_prochandle *,
			   psaddr_t, void *, size_t);
extern ps_err_e ps_pdwrite (struct ps_prochandle *,
			    psaddr_t, const void *, size_t);
extern ps_err_e ps_ptread (struct ps_prochandle *,
			   psaddr_t, void *, size_t);
extern ps_err_e ps_ptwrite (struct ps_prochandle *,
			    psaddr_t, const void *, size_t);


/* Get and set the given LWP's general or FPU register set.  */
extern ps_err_e ps_lgetregs (struct ps_prochandle *,
			     lwpid_t, prgregset_t);
extern ps_err_e ps_lsetregs (struct ps_prochandle *,
			     lwpid_t, const prgregset_t);
extern ps_err_e ps_lgetfpregs (struct ps_prochandle *,
			       lwpid_t, prfpregset_t *);
extern ps_err_e ps_lsetfpregs (struct ps_prochandle *,
			       lwpid_t, const prfpregset_t *);

/* Return the PID of the process.  */
extern pid_t ps_getpid (struct ps_prochandle *);

/* Fetch the special per-thread address associated with the given LWP.
   This call is only used on a few platforms (most use a normal register).
   The meaning of the `int' parameter is machine-dependent.  */
extern ps_err_e ps_get_thread_area (struct ps_prochandle *,
				    lwpid_t, int, psaddr_t *);


/* Look up the named symbol in the named DSO in the symbol tables
   associated with the process being debugged, filling in *SYM_ADDR
   with the corresponding run-time address.  */
extern ps_err_e ps_pglobal_lookup (struct ps_prochandle *,
				   const char *object_name,
				   const char *sym_name,
				   psaddr_t *sym_addr);


/* Stop or continue the entire process.  */
extern ps_err_e ps_pstop (struct ps_prochandle *);
extern ps_err_e ps_pcontinue (struct ps_prochandle *);

/* Stop or continue the given LWP alone.  */
extern ps_err_e ps_lstop (struct ps_prochandle *, lwpid_t);
extern ps_err_e ps_lcontinue (struct ps_prochandle *, lwpid_t);

__END_DECLS

#endif /* proc_service.h */
