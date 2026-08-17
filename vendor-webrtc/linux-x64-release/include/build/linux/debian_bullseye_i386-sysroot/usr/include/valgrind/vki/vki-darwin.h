
/*--------------------------------------------------------------------*/
/*--- Darwin-specific kernel interface.               vki-darwin.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2007-2017 Apple Inc.
      Greg Parker  gparker@apple.com

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

/* Unlike vki-linux, this Darwin kernel interface includes system headers
   directly, to avoid copyright complexity. */

#ifndef __VKI_DARWIN_H
#define __VKI_DARWIN_H

/* struct __darwin_ucontext isn't fully declared without
 * this definition.  It's crazy but there it is.  */
#ifndef _XOPEN_SOURCE
#define _XOPEN_SOURCE 0500
#endif

#include <stdint.h>

#define vki_int8_t int8_t
#define vki_uint8_t uint8_t
#define vki_int16_t int16_t
#define vki_uint16_t uint16_t
#define vki_int32_t int32_t
#define vki_uint32_t uint32_t
#define vki_int64_t int64_t
#define vki_uint64_t uint64_t
#define vki_intptr_t intptr_t
#define vki_uintptr_t uintptr_t

#include <sys/types.h>

#define vki_dev_t dev_t
#define vki_mode_t mode_t
#define vki_ino_t ino_t
#define vki_ino64_t ino64_t
#define vki_nlink_t nlink_t
#define vki_uid_t uid_t
#define vki_gid_t gid_t
#define vki_time_t time_t
#define vki_off_t off_t
#define vki_blkcnt_t blkcnt_t
#define vki_blksize_t blksize_t
#define vki_size_t size_t
#define vki_ssize_t ssize_t
#define vki_pid_t pid_t
#define vki_socklen_t socklen_t
#define vki_suseconds_t suseconds_t
#define vki_caddr_t caddr_t
#define vki_u_long u_long
#define vki_u_short u_short
#define vki_clock_t clock_t
#define vki_u_int32_t u_int32_t
#define vki_u_int16_t u_int16_t
#define vki_pthread_t pthread_t


// valgrind special

// magic mmap() flags
#define	VKI_MAP_ANONYMOUS MAP_ANON	// linux synonym

// fds for mmap(MAP_ANON), displayed by vmmap
#define VM_TAG_VALGRIND VM_MAKE_TAG(239)  // SkAnonV

// page sizes
#define VKI_MAX_PAGE_SHIFT VKI_PAGE_SHIFT
#define VKI_MAX_PAGE_SIZE VKI_PAGE_SIZE

// types
typedef uint32_t vki_u32;

// linux-like ioctl flags
#define _VKI_IOC_DIR(x)		((x) & IOC_DIRMASK)
#define _VKI_IOC_SIZE(x)	IOCPARM_LEN(x)
#define _VKI_IOC_NONE		IOC_VOID  /* GrP fixme correct? */
#define _VKI_IOC_READ		IOC_OUT
#define _VKI_IOC_WRITE		IOC_IN


#include <malloc/malloc.h>

#define vki_malloc_zone_t malloc_zone_t


#include <sys/time.h>

#define vki_timeval timeval
#define vki_timeval32 timeval32
#define vki_tv_sec tv_sec
#define vki_tv_usec tv_usec
#define vki_timespec timespec
#define vki_itimerval itimerval
#define vki_timezone timezone


#include <sys/stat.h>

#define	VKI_S_ISBLK(m)	S_ISBLK(m)
#define	VKI_S_ISCHR(m)	S_ISCHR(m)
#define	VKI_S_ISDIR(m)	S_ISDIR(m)
#define	VKI_S_ISFIFO(m)	S_ISFIFO(m)
#define	VKI_S_ISREG(m)	S_ISREG(m)
#define	VKI_S_ISLNK(m)	S_ISLNK(m)
#define	VKI_S_ISSOCK(m)	S_ISSOCK(m)
#define	VKI_S_ISWHT(m)	S_ISWHT(m)
#define VKI_S_ISXATTR(m) S_ISXATTR(m)

#define	VKI_S_IRWXU	S_IRWXU
#define	VKI_S_IRUSR	S_IRUSR
#define	VKI_S_IWUSR	S_IWUSR
#define	VKI_S_IXUSR	S_IXUSR
#define	VKI_S_IRWXG	S_IRWXG
#define	VKI_S_IRGRP	S_IRGRP
#define	VKI_S_IWGRP	S_IWGRP
#define	VKI_S_IXGRP	S_IXGRP
#define	VKI_S_IRWXO	S_IRWXO
#define	VKI_S_IROTH	S_IROTH
#define	VKI_S_IWOTH	S_IWOTH
#define	VKI_S_IXOTH	S_IXOTH
#define	VKI_S_ISUID	S_ISUID
#define	VKI_S_ISGID	S_ISGID
#define	VKI_S_ISVTX	S_ISVTX

#define vki_stat stat
#define vki_stat64 stat64

#define st_atime      st_atimespec.tv_sec
#define st_atime_nsec st_atimespec.tv_nsec
#define st_mtime      st_mtimespec.tv_sec
#define st_mtime_nsec st_mtimespec.tv_nsec
#define st_ctime      st_ctimespec.tv_sec
#define st_ctime_nsec st_ctimespec.tv_nsec


#include <sys/dirent.h>

#define VKI_MAXNAMLEN MAXNAMLEN
#define vki_dirent dirent


#include <sys/socket.h>
#define	VKI_SOCK_STREAM	SOCK_STREAM
#define	VKI_SOCK_DGRAM	SOCK_DGRAM
#define	VKI_SOCK_RAW	SOCK_RAW

#define	VKI_AF_UNIX	AF_UNIX
#define	VKI_AF_INET	AF_INET
#define	VKI_AF_INET6	AF_INET6

#define	VKI_SOL_SOCKET	SOL_SOCKET

#define	VKI_SO_REUSEADDR SO_REUSEADDR

#define VKI_SO_SNDBUF	SO_SNDBUF
#define VKI_SO_RCVBUF	SO_RCVBUF
#define VKI_SO_SNDLOWAT	SO_SNDLOWAT
#define VKI_SO_RCVLOWAT	SO_RCVLOWAT
#define VKI_SO_SNDTIMEO	SO_SNDTIMEO
#define VKI_SO_RCVTIMEO	SO_RCVTIMEO
#define	VKI_SO_ERROR	SO_ERROR
#define	VKI_SO_TYPE	SO_TYPE
#define VKI_SO_NREAD	SO_NREAD
#define VKI_SO_NKE	SO_NKE
#define VKI_SO_NOSIGPIPE	SO_NOSIGPIPE
#define VKI_SO_NOADDRERR	SO_NOADDRERR
#define VKI_SO_NWRITE	SO_NWRITE
#define VKI_SO_LINGER_SEC	SO_LINGER_SEC

#define vki_sa_family_t sa_family_t
#define vki_sockaddr sockaddr
#define vki_iovec iovec
#define vki_msghdr msghdr
#define vki_cmsghdr cmsghdr


#define VKI_CMSG_ALIGN(a) 	ALIGN(a)
#define	VKI_CMSG_DATA(cmsg)	CMSG_DATA(cmsg)
#define	VKI_CMSG_FIRSTHDR(mhdr)	CMSG_FIRSTHDR(mhdr)
#define	VKI_CMSG_NXTHDR(mhdr, cmsg)	CMSG_NXTHDR(mhdr, cmsg)

#define	VKI_SCM_RIGHTS		SCM_RIGHTS
#define	VKI_SCM_TIMESTAMP	SCM_TIMESTAMP
#define	VKI_SCM_CREDS		SCM_CREDS


#include <sys/un.h>

#define vki_sockaddr_un sockaddr_un


#include <netinet/in.h>

#define vki_in_addr_t in_addr_t
#define vki_in_port_t in_port_t
#define vki_in_addr in_addr
#define vki_sockaddr_in sockaddr_in

#define	VKI_INADDR_LOOPBACK	INADDR_LOOPBACK


// #include <netinet6/in6.h>

#define vki_in6_addr in6_addr
#define vki_sockaddr_in6 sockaddr_in6


#include <net/if.h>

#define	VKI_IFNAMSIZ	IFNAMSIZ

#define vki_ifdevmtu ifdevmtu
#define vki_ifreq ifreq
#define vki_ifr_name 	ifr_name
#define	vki_ifr_addr	ifr_addr
#define	vki_ifr_dstaddr	ifr_dstaddr
#define	vki_ifr_broadaddr	ifr_broadaddr
#define	vki_ifr_flags	ifr_flags
#define	vki_ifr_metric	ifr_metric
#define	vki_ifr_mtu	ifr_mtu
#define vki_ifr_phys	ifr_phys
#define vki_ifr_media	ifr_media
#define	vki_ifr_data	ifr_data
#define vki_ifr_devmtu	ifr_devmtu
#define vki_ifr_intval	ifr_intval

#define vki_ifconf ifconf
#define vki_ifc_buf 	ifc_buf
#define vki_ifc_req 	ifc_req


#include <sys/fcntl.h>

#define	VKI_SEEK_SET	SEEK_SET
#define	VKI_SEEK_CUR	SEEK_CUR
#define	VKI_SEEK_END	SEEK_END

#define	VKI_O_RDONLY	O_RDONLY
#define	VKI_O_WRONLY	O_WRONLY
#define	VKI_O_RDWR	O_RDWR
#define	VKI_O_ACCMODE	O_ACCMODE
#define	VKI_O_NONBLOCK	O_NONBLOCK
#define	VKI_O_APPEND	O_APPEND
#define	VKI_O_SYNC	O_SYN
#define	VKI_O_SHLOCK	O_SHLOCK
#define	VKI_O_EXLOCK	O_EXLOCK
#define	VKI_O_ASYNC	O_ASYNC
#define VKI_O_NOFOLLOW  O_NOFOLLOW
#define	VKI_O_CREAT	O_CREAT
#define	VKI_O_TRUNC	O_TRUNC
#define	VKI_O_EXCL	O_EXCL
#define	VKI_O_EVTONLY	O_EVTONLY

#define	VKI_F_DUPFD	F_DUPFD
#define	VKI_F_GETFD	F_GETFD
#define	VKI_F_SETFD	F_SETFD
#define	VKI_F_GETFL	F_GETFL
#define	VKI_F_SETFL	F_SETFL
#define	VKI_F_GETOWN	F_GETOWN
#define VKI_F_SETOWN	F_SETOWN
#define	VKI_F_GETLK	F_GETLK
#define	VKI_F_SETLK	F_SETLK
#define	VKI_F_SETLKW	F_SETLKW
#if DARWIN_VERS >= DARWIN_10_10
#define	VKI_F_SETLKWTIMEOUT F_SETLKWTIMEOUT
#endif

#define VKI_F_CHKCLEAN	F_CHKCLEAN
#define VKI_F_PREALLOCATE	F_PREALLOCATE
#define VKI_F_SETSIZE	F_SETSIZE
#define VKI_F_RDADVISE	F_RDADVISE
#define VKI_F_RDAHEAD	F_RDAHEAD
#define VKI_F_READBOOTSTRAP	F_READBOOTSTRAP
#define VKI_F_WRITEBOOTSTRAP	F_WRITEBOOTSTRAP
#define VKI_F_NOCACHE	F_NOCACHE
#define VKI_F_LOG2PHYS	F_LOG2PHYS
#define VKI_F_GETPATH	F_GETPATH
#define VKI_F_ADDSIGS	F_ADDSIGS
#if DARWIN_VERS >= DARWIN_10_9
# define VKI_F_ADDFILESIGS  F_ADDFILESIGS
#endif
#if DARWIN_VERS >= DARWIN_10_11
# define VKI_F_ADDFILESIGS_FOR_DYLD_SIM  F_ADDFILESIGS_FOR_DYLD_SIM
# define VKI_F_BARRIERFSYNC              F_BARRIERFSYNC
# define VKI_F_ADDFILESIGS_RETURN        F_ADDFILESIGS_RETURN
#endif
#define VKI_F_FULLFSYNC	F_FULLFSYNC
#define VKI_F_PATHPKG_CHECK	F_PATHPKG_CHECK
#define VKI_F_FREEZE_FS	F_FREEZE_FS
#define VKI_F_THAW_FS	F_THAW_FS
#define	VKI_F_GLOBAL_NOCACHE	F_GLOBAL_NOCACHE

#define VKI_FD_CLOEXEC	FD_CLOEXEC

#define vki_radvisory radvisory
#define vki_fstore fstore
#define vki_fbootstraptransfer fbootstraptransfer
#define vki_log2phys log2phys
#define vki_fsignatures_t fsignatures_t

// These constants aren't in a standard header, they are from the kernel code:
// xnu-1228.3.13/bsd/sys/codesign.h
// Mac OS X 10.5.6 - Darwin 9.6
#define VKI_CS_OPS_STATUS           0       /* return status */
#define VKI_CS_OPS_MARKINVALID      1       /* invalidate process */
#define VKI_CS_OPS_MARKHARD         2       /* set HARD flag */
#define VKI_CS_OPS_MARKKILL         3       /* set KILL flag (sticky) */
#define VKI_CS_OPS_PIDPATH          4       /* get executable's pathname */
#define VKI_CS_OPS_CDHASH           5       /* get code directory hash */

#include <sys/mman.h>

#define	VKI_PROT_NONE	PROT_NONE
#define	VKI_PROT_READ	PROT_READ
#define	VKI_PROT_WRITE	PROT_WRITE
#define	VKI_PROT_EXEC	PROT_EXEC

#define	VKI_MAP_SHARED	MAP_SHARED
#define	VKI_MAP_PRIVATE	MAP_PRIVATE
#define	VKI_MAP_FIXED	MAP_FIXED
#define	VKI_MAP_RENAME	MAP_RENAME
#define	VKI_MAP_NORESERVE	MAP_NORESERVE
#define	VKI_MAP_RESERVED0080	MAP_RESERVED0080
#define	VKI_MAP_NOEXTEND	MAP_NOEXTEND
#define	VKI_MAP_HASSEMAPHORE	MAP_HASSEMAPHORE
#define	VKI_MAP_FILE	MAP_FILE
#define	VKI_MAP_ANON	MAP_ANON
#define VKI_MAP_FAILED	MAP_FAILED


#include <mach/vm_param.h>

#define VKI_PAGE_SHIFT PAGE_SHIFT
#define VKI_PAGE_SIZE PAGE_SIZE
#define VKI_PAGE_MASK PAGE_MASK


#include <sys/vmparam.h>

#define VKI_USRSTACK USRSTACK
#define VKI_USRSTACK64 USRSTACK64


#include <mach/mach_time.h>

#define vki_mach_timebase_info mach_timebase_info


#include <sys/syslimits.h>

#define VKI_PATH_MAX PATH_MAX


#include <sys/param.h>

#define VKI_MAXPATHLEN MAXPATHLEN


#include <sys/signal.h>

/* While we fully intend to make 'vki_sigset_t' match the native
   Darwin 'sigset_t', we can't just clone the Darwin sigset_t type,
   because it isn't an array, and the VG_(sigfillset) etc functions
   assume it is.  So instead define another isomorphic type, and check
   in VG_(vki_do_initial_consistency_checks) that it really is
   correct. */
/* #define vki_sigset_t sigset_t */
#define _VKI_NSIG_BPW   32
#define _VKI_NSIG       32
#define _VKI_NSIG_WORDS (_VKI_NSIG / _VKI_NSIG_BPW)
typedef struct {
   UInt sig[_VKI_NSIG_WORDS];
} vki_sigset_t;
/* and now let VG_(vki_do_initial_consistency_checks) make sure it
   matches 'sigset_t'. */


#define VKI_SS_ONSTACK	SS_ONSTACK
#define	VKI_SS_DISABLE	SS_DISABLE
#define	VKI_MINSIGSTKSZ	MINSIGSTKSZ
#define	VKI_SIGSTKSZ	SIGSTKSZ

#define vki_stack_t        stack_t
#define vki_siginfo_t      siginfo_t

/* There are two versions of this.  'struct __sigaction' is used for
   passing sigactions to the kernel interface, and has the added
   complexity of requiring an extra pointer to a new demultiplexing
   function to be run in user space.  'struct sigaction' is used for
   receiving old sigactions from the kernel, and lacks this
   demux-function pointer.  So the type of the second and third
   parameters in Darwin's sys_sigaction appear to be different,
   respectively 'struct __sigaction*' and 'struct sigaction*'.
*/
//#define vki_sigaction      __sigaction
//#define vki_user_sigaction sigaction
//#define vki_sigaltstack    sigaltstack
//#define vki_sigval         sigval
//#define vki_sigaction_u    sigaction_u
//#define vki_sigaction     sigaction

//typedef  struct __sigaction  vki_sigaction_toK_t;
//typedef  struct sigaction    vki_sigaction_fromK_t;

typedef
   struct {
      void* ksa_handler;
      void (*sa_tramp)(void*,UWord,UWord,void*,void*);
      vki_sigset_t sa_mask;
      int sa_flags;
   }
   vki_sigaction_toK_t;

typedef
   struct {
      void* ksa_handler;
      vki_sigset_t sa_mask;
      int sa_flags;
   }
   vki_sigaction_fromK_t;



/* and /usr/include/sys/signal.c in turn defines 'sa_handler' to
   be '__sigaction_u.__sa_handler' */
//#define	ksa_handler      sa_handler

//#define	vki_sa_sigaction sa_sigaction

#define VKI_SA_ONSTACK	SA_ONSTACK
#define VKI_SA_RESTART	SA_RESTART
#define	VKI_SA_DISABLE	SA_DISABLE
#define	VKI_SA_RESETHAND	SA_RESETHAND
#define VKI_SA_NOCLDSTOP	SA_NOCLDSTOP
#define	VKI_SA_NODEFER	SA_NODEFER
#define	VKI_SA_NOCLDWAIT	SA_NOCLDWAIT
#define	VKI_SA_SIGINFO	SA_SIGINFO
#define	VKI_SA_USERTRAMP	SA_USERTRAMP
#define	VKI_SA_64REGSET	SA_64REGSET
#define VKI_SA_RESTORER  0 /* Darwin doesn't have this */

#define	VKI_SIG_BLOCK	SIG_BLOCK
#define	VKI_SIG_UNBLOCK	SIG_UNBLOCK
#define	VKI_SIG_SETMASK	SIG_SETMASK

#define	VKI_SIGHUP	SIGHUP
#define	VKI_SIGINT	SIGINT
#define	VKI_SIGQUIT	SIGQUIT
#define	VKI_SIGILL	SIGILL
#define	VKI_SIGTRAP	SIGTRAP
#define	VKI_SIGABRT	SIGABRT
#define	VKI_SIGPOLL	SIGPOLL
#define	VKI_SIGFPE	SIGFPE
#define	VKI_SIGKILL	SIGKILL
#define	VKI_SIGBUS	SIGBUS
#define	VKI_SIGSEGV	SIGSEGV
#define	VKI_SIGSYS	SIGSYS
#define	VKI_SIGPIPE	SIGPIPE
#define	VKI_SIGALRM	SIGALRM
#define	VKI_SIGTERM	SIGTERM
#define	VKI_SIGURG	SIGURG
#define	VKI_SIGSTOP	SIGSTOP
#define	VKI_SIGTSTP	SIGTSTP
#define	VKI_SIGCONT	SIGCONT
#define	VKI_SIGCHLD	SIGCHLD
#define	VKI_SIGTTIN	SIGTTIN
#define	VKI_SIGTTOU	SIGTTOU
#define	VKI_SIGIO	SIGIO
#define	VKI_SIGXCPU	SIGXCPU
#define	VKI_SIGXFSZ	SIGXFSZ
#define	VKI_SIGVTALRM	SIGVTALRM
#define	VKI_SIGPROF	SIGPROF
#define VKI_SIGWINCH	SIGWINCH
#define VKI_SIGINFO	SIGINFO
#define VKI_SIGUSR1	SIGUSR1
#define VKI_SIGUSR2	SIGUSR2

#define VKI_SIG_DFL     SIG_DFL
#define VKI_SIG_IGN     SIG_IGN


#define VKI_SI_USER      SI_USER
#define VKI_SEGV_MAPERR  SEGV_MAPERR
#define VKI_SEGV_ACCERR  SEGV_ACCERR
#define VKI_ILL_ILLOPC   ILL_ILLOPC
#define VKI_ILL_ILLOPN   ILL_ILLOPN
#define VKI_ILL_ILLADR   ILL_ILLADR
#define VKI_ILL_ILLTRP   ILL_ILLTRP
#define VKI_ILL_PRVOPC   ILL_PRVOPC
#define VKI_ILL_PRVREG   ILL_PRVREG
#define VKI_ILL_COPROC   ILL_COPROC
#define VKI_ILL_BADSTK   ILL_BADSTK
#define VKI_FPE_INTDIV   FPE_INTDIV
#define VKI_FPE_INTOVF   FPE_INTOVF
#define VKI_FPE_FLTDIV   FPE_FLTDIV
#define VKI_FPE_FLTOVF   FPE_FLTOVF
#define VKI_FPE_FLTUND   FPE_FLTUND
#define VKI_FPE_FLTRES   FPE_FLTRES
#define VKI_FPE_FLTINV   FPE_FLTINV
#define VKI_FPE_FLTSUB   FPE_FLTSUB
#define VKI_BUS_ADRALN   BUS_ADRALN
#define VKI_BUS_ADRERR   BUS_ADRERR
#define VKI_BUS_OBJERR   BUS_OBJERR
#define VKI_TRAP_BRKPT   TRAP_BRKPT

/* JRS: not 100% sure, but I think these two are correct */
#define VKI_SA_ONESHOT   SA_RESETHAND
#define VKI_SA_NOMASK    SA_NODEFER

#define VKI_UC_SET_ALT_STACK   0x40000000
#define VKI_UC_RESET_ALT_STACK 0x80000000


#include <sys/errno.h>

#define VKI_EPERM		EPERM
#define VKI_ENOENT		ENOENT
#define VKI_ESRCH		ESRCH
#define VKI_EINTR		EINTR
#define VKI_EIO			EIO
#define VKI_ENXIO		ENXIO
#define VKI_E2BIG		E2BIG
#define VKI_ENOEXEC		ENOEXEC
#define VKI_EBADF		EBADF
#define VKI_ECHILD		ECHILD
#define VKI_EDEADLK		EDEADLK
#define VKI_ENOMEM		ENOMEM
#define VKI_EACCES		EACCES
#define VKI_EFAULT		EFAULT
#define VKI_ENOTBLK		ENOTBLK
#define VKI_EBUSY		EBUSY
#define VKI_EEXIST		EEXIST
#define VKI_EXDEV		EXDEV
#define VKI_ENODEV		ENODEV
#define VKI_ENOTDIR		ENOTDIR
#define VKI_EISDIR		EISDIR
#define VKI_EINVAL		EINVAL
#define VKI_ENFILE		ENFILE
#define VKI_EMFILE		EMFILE
#define VKI_ENOTTY		ENOTTY
#define VKI_ETXTBSY		ETXTBSY
#define VKI_EFBIG		EFBIG
#define VKI_ENOSPC		ENOSPC
#define VKI_ESPIPE		ESPIPE
#define VKI_EROFS		EROFS
#define VKI_EMLINK		EMLINK
#define VKI_EPIPE		EPIPE
#define VKI_EDOM		EDOM
#define VKI_ERANGE		ERANGE
#define VKI_EAGAIN		EAGAIN
#define VKI_EWOULDBLOCK		EAGAIN
#define VKI_EINPROGRESS		EINPROGRESS
#define VKI_EALREADY		EALREADY
#define VKI_ENOTSOCK		ENOTSOCK
#define VKI_EDESTADDRREQ	EDESTADDRREQ
#define VKI_EMSGSIZE		EMSGSIZE
#define VKI_EPROTOTYPE		EPROTOTYPE
#define VKI_ENOPROTOOPT		ENOPROTOOPT
#define VKI_EPROTONOSUPPORT	EPROTONOSUPPORT
#define VKI_ESOCKTNOSUPPORT	ESOCKTNOSUPPORT
#define VKI_ENOTSUP		ENOTSUP
#define VKI_EPFNOSUPPORT	EPFNOSUPPORT
#define VKI_EAFNOSUPPORT	EAFNOSUPPORT
#define VKI_EADDRINUSE		EADDRINUSE
#define VKI_EADDRNOTAVAIL	EADDRNOTAVAIL
#define VKI_ENETDOWN		ENETDOWN
#define VKI_ENETUNREACH		ENETUNREACH
#define VKI_ENETRESET		ENETRESET
#define VKI_ECONNABORTED	ECONNABORTED
#define VKI_ECONNRESET		ECONNRESET
#define VKI_ENOBUFS		ENOBUFS
#define VKI_EISCONN		EISCONN
#define VKI_ENOTCONN		ENOTCONN
#define VKI_ESHUTDOWN		ESHUTDOWN
#define VKI_ETOOMANYREFS	ETOOMANYREFS
#define VKI_ETIMEDOUT		ETIMEDOUT
#define VKI_ECONNREFUSED	ECONNREFUSED
#define VKI_ELOOP		ELOOP
#define VKI_ENAMETOOLONG	ENAMETOOLONG
#define VKI_EHOSTDOWN		EHOSTDOWN
#define VKI_EHOSTUNREACH	EHOSTUNREACH
#define VKI_ENOTEMPTY		ENOTEMPTY
#define VKI_EPROCLIM		EPROCLIM
#define VKI_EUSERS		EUSERS
#define VKI_EDQUOT		EDQUOT
#define VKI_ESTALE		ESTALE
#define VKI_EREMOTE		EREMOTE
#define VKI_EBADRPC		EBADRPC
#define VKI_ERPCMISMATCH	ERPCMISMATCH
#define VKI_EPROGUNAVAIL	EPROGUNAVAIL
#define VKI_EPROGMISMATCH	EPROGMISMATCH
#define VKI_EPROCUNAVAIL	EPROCUNAVAIL
#define VKI_ENOLCK		ENOLCK
#define VKI_ENOSYS		ENOSYS
#define VKI_EFTYPE		EFTYPE
#define VKI_EAUTH		EAUTH
#define VKI_ENEEDAUTH		ENEEDAUTH
#define VKI_EPWROFF		EPWROFF
#define VKI_EDEVERR		EDEVERR
#define VKI_EOVERFLOW		EOVERFLOW
#define VKI_EBADEXEC		EBADEXEC
#define VKI_EBADARCH		EBADARCH
#define VKI_ESHLIBVERS		ESHLIBVERS
#define VKI_EBADMACHO		EBADMACHO
#define VKI_ECANCELED		ECANCELED
#define VKI_EIDRM		EIDRM
#define VKI_ENOMSG		ENOMSG
#define VKI_EILSEQ		EILSEQ
#define VKI_ENOATTR		ENOATTR
#define VKI_EBADMSG		EBADMSG
#define VKI_EMULTIHOP		EMULTIHOP
#define VKI_ENODATA		ENODATA
#define VKI_ENOLINK		ENOLINK
#define VKI_ENOSR		ENOSR
#define VKI_ENOSTR		ENOSTR
#define VKI_EPROTO		EPROTO
#define VKI_ETIME		ETIME
#define VKI_EOPNOTSUPP		EOPNOTSUPP
#define VKI_ELAST		ELAST


#include <sys/resource.h>

#define	VKI_RLIMIT_CPU		RLIMIT_CPU
#define	VKI_RLIMIT_FSIZE	RLIMIT_FSIZE
#define	VKI_RLIMIT_DATA		RLIMIT_DATA
#define	VKI_RLIMIT_STACK	RLIMIT_STACK
#define	VKI_RLIMIT_CORE		RLIMIT_CORE
#define	VKI_RLIMIT_AS		RLIMIT_AD
#define	VKI_RLIMIT_RSS		RLIMIT_AS
#define	VKI_RLIMIT_MEMLOCK	RLIMIT_MEMLOCK
#define	VKI_RLIMIT_NPROC	RLIMIT_NPROC
#define	VKI_RLIMIT_NOFILE	RLIMIT_NOFILE
#define	VKI_RLIM_NLIMITS	RLIM_NLIMITS

#define vki_rlim_t rlim_t
#define vki_rlimit rlimit
#define vki_rusage rusage


#include <sys/poll.h>

#define vki_pollfd pollfd


#include <sys/ipc.h>

#define	VKI_IPC_RMID	IPC_RMID
#define	VKI_IPC_SET	IPC_SET
#define	VKI_IPC_STAT	IPC_STAT

#define vki_key_t key_t
#define vki_ipc_perm ipc_perm


#include <sys/sem.h>

#define VKI_GETNCNT	GETNCNT
#define VKI_GETPID	GETPID
#define VKI_GETVAL	GETVAL
#define VKI_GETALL	GETALL
#define VKI_GETZCNT	GETZCNT
#define VKI_SETVAL	SETVAL
#define VKI_SETALL	SETALL

#define vki_sembuf sembuf
#define vki_semid_ds semid_ds
#define vki_semun semun


#include <sys/semaphore.h>

#define vki_sem_t sem_t


#include <sys/mount.h>

#define	VKI_MFSNAMELEN	MFSNAMELEN
#define	VKI_MNAMELEN	MNAMELEN

#define vki_fsid fsid
#define vki_fsid_t fsid_t
#define vki_statfs statfs
#define vki_statfs64 statfs64


#include <sys/select.h>

#define vki_fd_set fd_set


#include <sys/msgbuf.h>

#define	VKI_MSG_BSIZE	MSG_BSIZE
#define VKI_MSG_MAGIC	MSG_MAGIC
#define vki_msgbuf msgbuf


#include <sys/shm.h>

#define VKI_SHM_RDONLY	SHM_RDONLY
#define VKI_SHM_RND	SHM_RND
#define VKI_SHMLBA	SHMLBA

#define vki_shmid_ds shmid_ds


#include <sys/times.h>

#define vki_tms tms


#include <sys/utsname.h>

#define	_VKI_SYS_NAMELEN	_SYS_NAMELEN
#define vki_new_utsname utsname


#include <sys/unistd.h>

#define	VKI_F_OK	F_OK
#define	VKI_X_OK	X_OK
#define	VKI_W_OK	W_OK
#define	VKI_R_OK	R_OK

#define vki_accessx_descriptor         accessx_descriptor
#define VKI_ACCESSX_MAX_DESCRIPTORS    ACCESSX_MAX_DESCRIPTORS

#include <sys/sysctl.h>

#define VKI_CTL_MAXNAME		CTL_MAXNAME

#define	VKI_CTL_UNSPEC		CTL_UNSPEC
#define	VKI_CTL_KERN		CTL_KERN
#define	VKI_CTL_VM		CTL_VM
#define	VKI_CTL_VFS		CTL_VFS
#define	VKI_CTL_NET		CTL_NET
#define	VKI_CTL_DEBUG		CTL_DEBUG
#define	VKI_CTL_HW		CTL_HW
#define	VKI_CTL_MACHDEP		CTL_MACHDEP
#define	VKI_CTL_USER		CTL_USER
#define	VKI_CTL_MAXID		CTL_MAXID

#define	VKI_HW_MACHINE		HW_MACHINE
#define	VKI_HW_MODEL		HW_MODEL
#define	VKI_HW_NCPU		HW_NCPU
#define	VKI_HW_BYTEORDER	HW_BYTEORDER
#define	VKI_HW_PHYSMEM		HW_PHYSMEM
#define	VKI_HW_USERMEM		HW_USERMEM
#define	VKI_HW_PAGESIZE		HW_PAGESIZE
#define	VKI_HW_DISKNAMES	HW_DISKNAMES
#define	VKI_HW_DISKSTATS	HW_DISKSTATS
#define	VKI_HW_EPOCH  		HW_EPOCH
#define VKI_HW_FLOATINGPT	HW_FLOATINGPT
#define VKI_HW_MACHINE_ARCH	HW_MACHINE_ARCH
#define VKI_HW_VECTORUNIT	HW_VECTORUNIT
#define VKI_HW_BUS_FREQ		HW_BUS_FREQ
#define VKI_HW_CPU_FREQ		HW_CPU_FREQ
#define VKI_HW_CACHELINE	HW_CACHELINE
#define VKI_HW_L1ICACHESIZE	HW_L1ICACHESIZE
#define VKI_HW_L1DCACHESIZE	HW_L1DCACHESIZE
#define VKI_HW_L2SETTINGS	HW_L2SETTINGS
#define VKI_HW_L2CACHESIZE	HW_L2CACHESIZE
#define VKI_HW_L3SETTINGS	HW_L3SETTINGS
#define VKI_HW_L3CACHESIZE	HW_L3CACHESIZE
#define VKI_HW_TB_FREQ		HW_TB_FREQ
#define VKI_HW_MEMSIZE		HW_MEMSIZE
#define VKI_HW_AVAILCPU		MW_AVAILCPU
#define	VKI_HW_MAXID		MW_MAXID

#define	VKI_KERN_USRSTACK32	KERN_USRSTACK32
#define	VKI_KERN_USRSTACK64	KERN_USRSTACK64


#include <sys/attr.h>

#define vki_attrlist attrlist


#include <sys/event.h>

#define vki_kevent kevent
#define vki_kevent64 kevent64_s

// xnu_root/bsd/sys/event.h

struct vki_kevent_qos_s {
    uint64_t    ident;      /* identifier for this event */
    int16_t     filter;     /* filter for event */
    uint16_t    flags;      /* general flags */
    int32_t     qos;        /* quality of service */
    uint64_t    udata;      /* opaque user data identifier */
    uint32_t    fflags;     /* filter-specific flags */
    uint32_t    xflags;     /* extra filter-specific flags */
    int64_t     data;       /* filter-specific data */
    uint64_t    ext[4];     /* filter-specific extensions */
};

#include <sys/ev.h>

typedef struct eventreq vki_eventreq;


#include <sys/acl.h>

#define vki_kauth_filesec kauth_filesec


#include <sys/ptrace.h>

#define VKI_PTRACE_TRACEME   PT_TRACE_ME
#define VKI_PTRACE_DETACH    PT_DETACH


// sqlite/src/os_unix.c

struct ByteRangeLockPB2
{
    unsigned long long offset;        /* offset to first byte to lock */
    unsigned long long length;        /* nbr of bytes to lock */
    unsigned long long retRangeStart; /* nbr of 1st byte locked if successful */
    unsigned char unLockFlag;         /* 1 = unlock, 0 = lock */
    unsigned char startEndFlag;       /* 1=rel to end of fork, 0=rel to start */
    int fd;                           /* file desc to assoc this lock with */
};

#define afpfsByteRangeLock2FSCTL _IOWR('z', 23, struct ByteRangeLockPB2)

#define vki_ByteRangeLockPB2 ByteRangeLockPB2
#define VKI_afpfsByteRangeLock2FSCTL afpfsByteRangeLock2FSCTL


// xnu/bsd/sys/fsctl.h

#define VKI_FSIOC_SYNC_VOLUME        _IOW('A', 1, uint32_t)


// libpthread/kern/workqueue_internal.h

#define VKI_WQOPS_QUEUE_ADD                    1
#define VKI_WQOPS_QUEUE_REMOVE                 2
#define VKI_WQOPS_THREAD_RETURN                4  /* parks the thread back into the kernel */
#define VKI_WQOPS_THREAD_SETCONC               8
#define VKI_WQOPS_QUEUE_NEWSPISUPP            16  /* check for newer SPI support */
#define VKI_WQOPS_QUEUE_REQTHREADS            32  /* request number of threads of a prio */
#define VKI_WQOPS_QUEUE_REQTHREADS2           48  /* request a number of threads in a given priority bucket */
#define VKI_WQOPS_THREAD_KEVENT_RETURN        64  /* parks the thread after delivering the passed kevent array */
#define VKI_WQOPS_SET_EVENT_MANAGER_PRIORITY 128  /* max() in the provided priority in the the priority of the event manager */
#define VKI_WQOPS_THREAD_WORKLOOP_RETURN     256  /* parks the thread after delivering the passed kevent array */
#define VKI_WQOPS_SHOULD_NARROW              512  /* checks whether we should narrow our concurrency */


#include <sys/ttycom.h>

#define vki_winsize winsize

#define	VKI_TIOCMODG	TIOCMODG
#define	VKI_TIOCMODS	TIOCMODS
#define	VKI_TIOCEXCL	TIOCEXCL
#define	VKI_TIOCNXCL	TIOCNXCL
#define	VKI_TIOCFLUSH	TIOCFLUSH
#define	VKI_TIOCGETA	TIOCGETA
#define	VKI_TIOCSETA	TIOCSETA
#define	VKI_TIOCSETAW	TIOCSETAW
#define	VKI_TIOCSETAF	TIOCSETAF
#define	VKI_TIOCGETD	TIOCGETD
#define	VKI_TIOCSETD	TIOCSETD
#define	VKI_TIOCSBRK	TIOCSBRK
#define	VKI_TIOCCBRK	TIOCCBRK
#define	VKI_TIOCSDTR	TIOCSDTR
#define	VKI_TIOCCDTR	TIOCCDTR
#define	VKI_TIOCGPGRP	TIOCGPGRP
#define	VKI_TIOCSPGRP	TIOCSPGRP
#define	VKI_TIOCOUTQ	TIOCOUTQ
#define	VKI_TIOCSTI	TIOCSTI
#define	VKI_TIOCNOTTY	TIOCNOTTY
#define	VKI_TIOCPKT	TIOCPKT
#define	VKI_TIOCSTOP	TIOCSTOP
#define	VKI_TIOCSTART	TIOCSTART
#define	VKI_TIOCMSET	TIOCMSET
#define	VKI_TIOCMBIS	TIOCMBIS
#define	VKI_TIOCMBIC	TIOCMBIC
#define	VKI_TIOCMGET	TIOCMGET
#define	VKI_TIOCREMOTE	TIOCREMOTE
#define	VKI_TIOCGWINSZ	TIOCGWINSZ
#define	VKI_TIOCSWINSZ	TIOCSWINSZ
#define	VKI_TIOCUCNTL	TIOCUCNTL
#define	VKI_TIOCSTAT	TIOCSTAT
#define	VKI_UIOCCMD(n)	UIOCCMD(n)
#define	VKI_TIOCSCONS	TIOCSCONS
#define	VKI_TIOCCONS	TIOCCONS
#define	VKI_TIOCSCTTY	TIOCSCTTY
#define	VKI_TIOCEXT	TIOCEXT
#define	VKI_TIOCSIG	TIOCSIG
#define	VKI_TIOCDRAIN	TIOCDRAIN
#define	VKI_TIOCMSDTRWAIT	TIOCMSDTRWAIT
#define	VKI_TIOCMGDTRWAIT	TIOCMGDTRWAIT
#define	VKI_TIOCTIMESTAMP	TIOCTIMESTAMP
#define	VKI_TIOCDCDTIMESTAMP	TIOCDCDTIMESTAMP
#define	VKI_TIOCSDRAINWAIT	TIOCSDRAINWAIT
#define	VKI_TIOCGDRAINWAIT	TIOCGDRAINWAIT
#define	VKI_TIOCDSIMICROCODE	TIOCDSIMICROCODE
#define VKI_TIOCPTYGRANT	TIOCPTYGRANT
#define VKI_TIOCPTYGNAME	TIOCPTYGNAME
#define VKI_TIOCPTYUNLK	TIOCPTYUNLK


#include <sys/filio.h>

#define	VKI_FIOCLEX	FIOCLEX
#define	VKI_FIONCLEX	FIONCLEX
#define	VKI_FIONREAD	FIONREAD
#define	VKI_FIONBIO	FIONBIO
#define	VKI_FIOASYNC	FIOASYNC
#define	VKI_FIOSETOWN	FIOSETOWN
#define	VKI_FIOGETOWN	FIOGETOWN
#define	VKI_FIODTYPE	FIODTYPE


#include <sys/sockio.h>

#define	VKI_SIOCSHIWAT	SIOCSHIWAT
#define	VKI_SIOCGHIWAT	SIOCGHIWAT
#define	VKI_SIOCSLOWAT	SIOCSLOWAT
#define	VKI_SIOCGLOWAT	SIOCGLOWAT
#define	VKI_SIOCATMARK	SIOCATMARK
#define	VKI_SIOCSPGRP	SIOCSPGRP
#define	VKI_SIOCGPGRP	SIOCGPGRP

#define	VKI_SIOCSIFADDR		SIOCSIFADDR
#define	VKI_OSIOCGIFADDR	OSIOCGIFADDR
#define	VKI_SIOCSIFDSTADDR	SIOCSIFDSTADDR
#define	VKI_OSIOCGIFDSTADDR	OSIOCGIFDSTADDR
#define	VKI_SIOCSIFFLAGS	SIOCSIFFLAGS
#define	VKI_SIOCGIFFLAGS	SIOCGIFFLAGS
#define	VKI_OSIOCGIFBRDADDR	OSIOCGIFBRDADDR
#define	VKI_SIOCSIFBRDADDR	SIOCSIFBRDADDR
#define	VKI_OSIOCGIFCONF	OSIOCGIFCONF
#define	VKI_OSIOCGIFNETMASK	OSIOCGIFNETMASK
#define	VKI_SIOCSIFNETMASK	SIOCSIFNETMASK
#define	VKI_SIOCGIFMETRIC	SIOCGIFMETRIC
#define	VKI_SIOCSIFMETRIC	SIOCSIFMETRIC
#define	VKI_SIOCDIFADDR		SIOCDIFADDR
#define	VKI_SIOCAIFADDR		SIOCAIFADDR
#define	VKI_SIOCGETVIFCNT	SIOCGETVIFCNT
#define	VKI_SIOCGETSGCNT	SIOCGETSGCNT
#define VKI_SIOCALIFADDR	SIOCALIFADDR
#define VKI_SIOCGLIFADDR	SIOCGLIFADDR
#define VKI_SIOCDLIFADDR	SIOCDLIFADDR

#define	VKI_SIOCGIFADDR		SIOCGIFADDR
#define	VKI_SIOCGIFDSTADDR	SIOCGIFDSTADDR
#define	VKI_SIOCGIFBRDADDR	SIOCGIFBRDADDR
#define	VKI_SIOCGIFCONF		SIOCGIFCONF
#define	VKI_SIOCGIFNETMASK	SIOCGIFNETMASK
#define VKI_SIOCAUTOADDR	SIOCAUTOADDR
#define VKI_SIOCAUTONETMASK	SIOCAUTONETMASK
#define VKI_SIOCARPIPLL		SIOCARPIPLL

#define	VKI_SIOCADDMULTI	SIOCADDMULTI
#define	VKI_SIOCDELMULTI	SIOCDELMULTI
#define	VKI_SIOCGIFMTU		SIOCGIFMTU
#define	VKI_SIOCSIFMTU	 	SIOCSIFMTU
#define	VKI_SIOCGIFPHYS		SIOCGIFPHYS
#define	VKI_SIOCSIFPHYS	 	SIOCSIFPHYS
#define	VKI_SIOCSIFMEDIA	SIOCSIFMEDIA
#define	VKI_SIOCGIFMEDIA	SIOCGIFMEDIA
#define	VKI_SIOCSIFGENERIC	SIOCSIFGENERIC
#define	VKI_SIOCGIFGENERIC	SIOCGIFGENERIC
#define VKI_SIOCRSLVMULTI   	SIOCRSLVMULTI

#define	VKI_SIOCSIFLLADDR	SIOCSIFLLADDR
#define	VKI_SIOCGIFSTATUS	SIOCGIFSTATUS
#define	VKI_SIOCSIFPHYADDR   	SIOCSIFPHYADDR
#define	VKI_SIOCGIFPSRCADDR	SIOCGIFPSRCADDR
#define	VKI_SIOCGIFPDSTADDR	SIOCGIFPDSTADDR
#define	VKI_SIOCDIFPHYADDR	SIOCDIFPHYADDR
#define	VKI_SIOCSLIFPHYADDR	SIOCSLIFPHYADDR
#define	VKI_SIOCGLIFPHYADDR	SIOCGLIFPHYADDR

#define	VKI_SIOCGIFDEVMTU	SIOCGIFDEVMTU
#define	VKI_SIOCSIFALTMTU	SIOCSIFALTMTU
#define VKI_SIOCGIFALTMTU	SIOCGIFALTMTU
#define VKI_SIOCSIFBOND	 	SIOCSIFBOND
#define VKI_SIOCGIFBOND		SIOCGIFBOND
#define	VKI_SIOCIFCREATE	SIOCIFCREATE
#define	VKI_SIOCIFDESTROY	SIOCIFDESTROY
#define	VKI_SIOCSIFVLAN	 	SIOCSIFVLAN
#define	VKI_SIOCGIFVLAN		SIOCGIFVLAN

#define	VKI_SIOCSETVLAN		SIOCSIFVLAN
#define	VKI_SIOCGETVLAN		SIOCGIFVLAN

#define	VKI_SIOCGIFASYNCMAP 	SIOCGIFASYNCMAP
#define	VKI_SIOCSIFASYNCMAP 	SIOCSIGASYNCMAP


#include <sys/dtrace.h>

#define VKI_DTRACEHIOC_REMOVE   DTRACEHIOC_REMOVE
#define VKI_DTRACEHIOC_ADDDOF   DTRACEHIOC_ADDDOF


#include <net/bpf.h>

#define vki_bpf_program bpf_program
#define vki_bf_len bf_len
#define vki_bf_insns bf_insns
#define vki_bpf_dltlist bpf_dltlist
#define vki_bfl_len bfl_len
#define vki_bfl_list bfl_list

#define VKI_BIOCSETF        BIOCSETF
#define VKI_BIOCFLUSH       BIOCFLUSH
#define VKI_BIOCPROMISC     BIOCPROMISC
#define VKI_BIOCSETIF       BIOCSETIF
#define VKI_BIOCSRTIMEOUT   BIOCSRTIMEOUT
#define VKI_BIOCGDLTLIST    BIOCGDLTLIST


#include <sys/ucontext.h>

/* quite why sys/ucontext.h provides a 'struct __darwin_ucontext'
   but no 'struct ucontext' beats me. -- JRS */
#define vki_ucontext __darwin_ucontext


#include <sys/termios.h>

#define vki_termios termios


#include <uuid/uuid.h>

#define vki_uuid_t uuid_t


#include <bsm/audit.h>

#define	VKI_A_GETPOLICY	A_GETPOLICY	
#define	VKI_A_SETPOLICY	A_SETPOLICY	
#define	VKI_A_GETKMASK	A_GETKMASK	
#define	VKI_A_SETKMASK	A_SETKMASK	
#define	VKI_A_GETQCTRL	A_GETQCTRL	
#define	VKI_A_SETQCTRL	A_SETQCTRL	
#define	VKI_A_GETCWD	A_GETCWD	
#define	VKI_A_GETCAR	A_GETCAR	
#define	VKI_A_GETSTAT	A_GETSTAT	
#define	VKI_A_SETSTAT	A_SETSTAT	
#define	VKI_A_SETUMASK	A_SETUMASK	
#define	VKI_A_SETSMASK	A_SETSMASK	
#define	VKI_A_GETCOND	A_GETCOND	
#define	VKI_A_SETCOND	A_SETCOND	
#define	VKI_A_GETCLASS	A_GETCLASS	
#define	VKI_A_SETCLASS	A_SETCLASS	
#define	VKI_A_GETPINFO	A_GETPINFO	
#define	VKI_A_SETPMASK	A_SETPMASK	
#define	VKI_A_SETFSIZE	A_SETFSIZE	
#define	VKI_A_GETFSIZE	A_GETFSIZE	
#define	VKI_A_GETPINFO_ADDR	A_GETPINFO_ADDR	
#define	VKI_A_GETKAUDIT	A_GETKAUDIT	
#define	VKI_A_SETKAUDIT	A_SETKAUDIT	
#if DARWIN_VERS >= DARWIN_10_6
#define VKI_A_SENDTRIGGER A_SENDTRIGGER
#define VKI_A_GETSINFO_ADDR A_GETSINFO_ADDR
#endif


#include <sys/aio.h>

#define vki_aiocb aiocb


#include <netinet/tcp.h>

#define VKI_TCP_NODELAY  TCP_NODELAY


#include <netinet/in.h>

#define VKI_IPPROTO_TCP  IPPROTO_TCP


// XXX: for some reason when I #include <sys/kernel_types.h> I get a syntax
// error.  Hmm.  So just define things ourselves.
//#include <sys/kernel_types.h>

//#define vki_errno_t
typedef int vki_errno_t;


/* necp stuff.  This doesn't appear to exist in any user space include
   file. */
#if DARWIN_VERS == DARWIN_10_10
struct vki_necp_aggregate_result {
   vki_u_int32_t field1;
   unsigned int  field2;
   vki_u_int32_t field3;
   vki_u_int32_t field4;
   vki_uuid_t    field5;
   u_int32_t     field6;
   u_int32_t     field7;
};
#endif /* DARWIN_VERS == DARWIN_10_10 */

#if DARWIN_VERS >= DARWIN_10_12
// ulock_wake & ulock_wait operations
#define VKI_UL_OPCODE_MASK      0x000000FF
#define VKI_UL_FLAGS_MASK       0xFFFFFF00
#define VKI_UL_COMPARE_AND_WAIT 1
#define VKI_UL_UNFAIR_LOCK      2
// ulock_wake & ulock_wait flags
#define ULF_NO_ERRNO            0x01000000

// ulock_wait flags
#define WKI_ULF_WAIT_WORKQ_DATA_CONTENTION	0x00010000
#endif /* DARWIN_VERS >= DARWIN_10_12 */

#endif
