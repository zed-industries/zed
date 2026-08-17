
/*--------------------------------------------------------------------*/
/*--- Linux-specific kernel interface.                 vki-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2000-2017 Julian Seward 
      jseward@acm.org

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

/* This file defines types and constants for the kernel interface, and to
   make that clear everything is prefixed VKI_/vki_.

   All code is copied verbatim from kernel source files, except that:
   - VKI_/vki_ prefixes are added
   - some extra explanatory comments are included;  they are all within
     "[[ ]]"
   - for some types, we only care about the size;  for a few of them (big
     ones that are painful to fully drag in here), a VKI_SIZEOF_* constant
     is used.
   
   The files the code is taken from is indicated.

   Note especially that the types are not the glibc versions, many of which
   are different to those in here. 

   Also note that this file contains all the generic header info, ie. that
   from linux/include/linux/ *.h.  The arch-specific header info, eg. that
   from linux/include/asm-i386/ *.h, is in vki-$PLATFORM.h and
   vki_posixtypes-$PLATFORM.h.  (Two files are required to avoid
   circular dependencies between the generic VKI header and the
   arch-specific VKI header.  It's possible in the future, as more stuff
   gets pulled in, that we might have to split files up some more to avoid
   further circular dependencies.)
   
   Finally, note that it is assumed that __KERNEL__ is set for all these
   definitions, which affects some of them.
*/

/* The structure is (aiui, jrs 20060504):

     #include plat-specific posix types (vki-posixtypes-ARCH-linux.h)

     Lots more types, structs, consts, in this file

     #include other plat-specific stuff (vki-ARCH-linux.h)

     Even more types, structs, consts, in this file

   The system call numbers are dealt with by
   pub_{core,tool}_vkiscnums.h, not via pub_{core,tool}_vki.h, which
   is what this file is part of.
*/

#ifndef __VKI_LINUX_H
#define __VKI_LINUX_H

//----------------------------------------------------------------------
// Arch-specific POSIX types
//----------------------------------------------------------------------

#if defined(VGA_x86)
#  include "vki-posixtypes-x86-linux.h"
#elif defined(VGA_amd64)
#  include "vki-posixtypes-amd64-linux.h"
#elif defined(VGA_ppc32)
#  include "vki-posixtypes-ppc32-linux.h"
#elif defined(VGA_ppc64be) || defined(VGA_ppc64le)
#  include "vki-posixtypes-ppc64-linux.h"
#elif defined(VGA_arm)
#  include "vki-posixtypes-arm-linux.h"
#elif defined(VGA_arm64)
#  include "vki-posixtypes-arm64-linux.h"
#elif defined(VGA_s390x)
#  include "vki-posixtypes-s390x-linux.h"
#elif defined(VGA_mips32)
#  include "vki-posixtypes-mips32-linux.h"
#elif defined(VGA_mips64)
#  include "vki-posixtypes-mips64-linux.h"
#elif defined(VGA_nanomips)
#  include "vki-posixtypes-nanomips-linux.h"
#else
#  error Unknown platform
#endif

//----------------------------------------------------------------------
// VKI_STATIC_ASSERT(). Inspired by BUILD_BUG_ON() from
// linux-2.6.34/include/linux/kernel.h
//----------------------------------------------------------------------

/*
 * Evaluates to zero if 'expr' is true and forces a compilation error if
 * 'expr' is false. Can be used in a context where no comma expressions
 * are allowed.
 */
#ifdef __cplusplus
template <bool b> struct vki_static_assert { int m_bitfield:(2*b-1); };
#define VKI_STATIC_ASSERT(expr)                         \
    (sizeof(vki_static_assert<(expr)>) - sizeof(int))
#else
#define VKI_STATIC_ASSERT(expr) (sizeof(struct { int:-!(expr); }))
#endif

//----------------------------------------------------------------------
// Based on _IOC_TYPECHECK() from linux-2.6.34/asm-generic/ioctl.h
//----------------------------------------------------------------------

/* provoke compile error for invalid uses of size argument */
#define _VKI_IOC_TYPECHECK(t)                                           \
    (VKI_STATIC_ASSERT((sizeof(t) == sizeof(t[1])                       \
                        && sizeof(t) < (1 << _VKI_IOC_SIZEBITS)))       \
     + sizeof(t))

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/compiler.h
//----------------------------------------------------------------------

# define __user

//----------------------------------------------------------------------
// From linux/include/linux/compiler-gcc.h
//----------------------------------------------------------------------

#ifdef __GNUC__
#define __vki_packed			__attribute__((packed))
#endif

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/posix_types.h
//----------------------------------------------------------------------

#undef __VKI_NFDBITS
#define __VKI_NFDBITS	(8 * sizeof(unsigned long))

#undef __VKI_FD_SETSIZE
#define __VKI_FD_SETSIZE	1024

#undef __VKI_FDSET_LONGS
#define __VKI_FDSET_LONGS	(__VKI_FD_SETSIZE/__VKI_NFDBITS)

#undef __VKI_FDELT
#define	__VKI_FDELT(d)	((d) / __VKI_NFDBITS)

#undef __VKI_FDMASK
#define	__VKI_FDMASK(d)	(1UL << ((d) % __VKI_NFDBITS))

typedef struct {
	unsigned long fds_bits [__VKI_FDSET_LONGS];
} __vki_kernel_fd_set;

typedef int __vki_kernel_key_t;
typedef int __vki_kernel_mqd_t;

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/types.h
//----------------------------------------------------------------------

typedef __vki_kernel_fd_set	vki_fd_set;
typedef __vki_kernel_mode_t	vki_mode_t;
typedef __vki_kernel_off_t	vki_off_t;
typedef __vki_kernel_pid_t	vki_pid_t;
typedef __vki_kernel_key_t	vki_key_t;
typedef __vki_kernel_suseconds_t	vki_suseconds_t;
typedef __vki_kernel_timer_t	vki_timer_t;
typedef __vki_kernel_clockid_t	vki_clockid_t;
typedef __vki_kernel_mqd_t	vki_mqd_t;

// [[Nb: it's a bit unclear due to a #ifdef, but I think this is right. --njn]]
typedef __vki_kernel_uid32_t	vki_uid_t;
typedef __vki_kernel_gid32_t	vki_gid_t;

typedef __vki_kernel_old_uid_t	vki_old_uid_t;
typedef __vki_kernel_old_gid_t	vki_old_gid_t;

typedef __vki_kernel_loff_t	vki_loff_t;

typedef __vki_kernel_size_t	vki_size_t;
typedef __vki_kernel_time_t	vki_time_t;
typedef __vki_kernel_clock_t	vki_clock_t;
typedef __vki_kernel_caddr_t	vki_caddr_t;

typedef unsigned long           vki_u_long;

typedef unsigned int	        vki_uint;

//----------------------------------------------------------------------
// Now the rest of the arch-specific stuff
//----------------------------------------------------------------------

#if defined(VGA_x86)
#  include "vki-x86-linux.h"
#elif defined(VGA_amd64)
#  include "vki-amd64-linux.h"
#elif defined(VGA_ppc32)
#  include "vki-ppc32-linux.h"
#elif defined(VGA_ppc64be) || defined(VGA_ppc64le)
#  include "vki-ppc64-linux.h"
#elif defined(VGA_arm)
#  include "vki-arm-linux.h"
#elif defined(VGA_arm64)
#  include "vki-arm64-linux.h"
#elif defined(VGA_s390x)
#  include "vki-s390x-linux.h"
#elif defined(VGA_mips32)
#  include "vki-mips32-linux.h"
#elif defined(VGA_mips64)
#  include "vki-mips64-linux.h"
#elif defined(VGA_nanomips)
#  include "vki-nanomips-linux.h"
#else
#  error Unknown platform
#endif

//----------------------------------------------------------------------
// From linux-2.6.20.1/include/linux/types.h
//----------------------------------------------------------------------

typedef		__vki_s32	vki_int32_t;
typedef		__vki_s16	vki_int16_t;
typedef		__vki_s64	vki_int64_t;

typedef		__vki_u8	vki_uint8_t;
typedef		__vki_u16	vki_uint16_t;
typedef		__vki_u32	vki_uint32_t;
typedef		__vki_u64	vki_uint64_t;

typedef		__vki_u16	__vki_le16;

#define __vki_aligned_u64 __vki_u64 __attribute__((aligned(8)))

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/limits.h
//----------------------------------------------------------------------

#define VKI_PATH_MAX       4096	/* # chars in a path name including nul */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/kernel.h
//----------------------------------------------------------------------

struct vki_sysinfo {
	long uptime;			/* Seconds since boot */
	unsigned long loads[3];		/* 1, 5, and 15 minute load averages */
	unsigned long totalram;		/* Total usable main memory size */
	unsigned long freeram;		/* Available memory size */
	unsigned long sharedram;	/* Amount of shared memory */
	unsigned long bufferram;	/* Memory used by buffers */
	unsigned long totalswap;	/* Total swap space size */
	unsigned long freeswap;		/* swap space still available */
	unsigned short procs;		/* Number of current processes */
	unsigned short pad;		/* explicit padding for m68k */
	unsigned long totalhigh;	/* Total high memory size */
	unsigned long freehigh;		/* Available high memory size */
	unsigned int mem_unit;		/* Memory unit size in bytes */
	char _f[20-2*sizeof(long)-sizeof(int)];	/* Padding: libc5 uses this.. */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/time.h
//----------------------------------------------------------------------

#define VKI_CLOCK_REALTIME            0
#define VKI_CLOCK_MONOTONIC           1
#define VKI_CLOCK_PROCESS_CPUTIME_ID  2
#define VKI_CLOCK_THREAD_CPUTIME_ID   3

struct vki_timespec {
	vki_time_t	tv_sec;		/* seconds */
	long		tv_nsec;	/* nanoseconds */
};

/* Special values for vki_timespec.tv_nsec when used with utimensat.  */
#define VKI_UTIME_NOW  ((1l << 30) - 1l)
#define VKI_UTIME_OMIT ((1l << 30) - 2l)

struct vki_timeval {
	vki_time_t	tv_sec;		/* seconds */
	vki_suseconds_t	tv_usec;	/* microseconds */
};

struct vki_timezone {
	int	tz_minuteswest;	/* minutes west of Greenwich */
	int	tz_dsttime;	/* type of dst correction */
};

struct  vki_itimerspec {
        struct  vki_timespec it_interval;    /* timer period */
        struct  vki_timespec it_value;       /* timer expiration */
};

struct	vki_itimerval {
	struct	vki_timeval it_interval;	/* timer interval */
	struct	vki_timeval it_value;	/* current value */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/timex.h
//----------------------------------------------------------------------

struct vki_timex {
	unsigned int modes;	/* mode selector */
	long offset;		/* time offset (usec) */
	long freq;		/* frequency offset (scaled ppm) */
	long maxerror;		/* maximum error (usec) */
	long esterror;		/* estimated error (usec) */
	int status;		/* clock command/status */
	long constant;		/* pll time constant */
	long precision;		/* clock precision (usec) (read only) */
	long tolerance;		/* clock frequency tolerance (ppm)
				 * (read only)
				 */
	struct vki_timeval time;	/* (read only) */
	long tick;		/* (modified) usecs between clock ticks */

	long ppsfreq;           /* pps frequency (scaled ppm) (ro) */
	long jitter;            /* pps jitter (us) (ro) */
	int shift;              /* interval duration (s) (shift) (ro) */
	long stabil;            /* pps stability (scaled ppm) (ro) */
	long jitcnt;            /* jitter limit exceeded (ro) */
	long calcnt;            /* calibration intervals (ro) */
	long errcnt;            /* calibration errors (ro) */
	long stbcnt;            /* stability limit exceeded (ro) */

	int  :32; int  :32; int  :32; int  :32;
	int  :32; int  :32; int  :32; int  :32;
	int  :32; int  :32; int  :32; int  :32;
};

#define VKI_ADJ_OFFSET			0x0001	/* time offset */
#define VKI_ADJ_FREQUENCY		0x0002	/* frequency offset */
#define VKI_ADJ_MAXERROR		0x0004	/* maximum time error */
#define VKI_ADJ_ESTERROR		0x0008	/* estimated time error */
#define VKI_ADJ_STATUS			0x0010	/* clock status */
#define VKI_ADJ_TIMECONST		0x0020	/* pll time constant */
#define VKI_ADJ_TAI			0x0080	/* set TAI offset */
#define VKI_ADJ_TICK			0x4000	/* tick value */
#define VKI_ADJ_ADJTIME			0x8000	/* switch between adjtime/adjtimex modes */
//#define VKI_ADJ_OFFSET_SINGLESHOT	0x8001	/* old-fashioned adjtime */
#define VKI_ADJ_OFFSET_READONLY		0x2000	/* read-only adjtime */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/times.h
//----------------------------------------------------------------------

struct vki_tms {
	vki_clock_t tms_utime;
	vki_clock_t tms_stime;
	vki_clock_t tms_cutime;
	vki_clock_t tms_cstime;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/utime.h
//----------------------------------------------------------------------

struct vki_utimbuf {
	vki_time_t actime;
	vki_time_t modtime;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/sched.h
//----------------------------------------------------------------------

#define VKI_CSIGNAL			0x000000ff	/* signal mask to be sent at exit */
#define VKI_CLONE_VM			0x00000100	/* set if VM shared between processes */
#define VKI_CLONE_FS			0x00000200	/* set if fs info shared between processes */
#define VKI_CLONE_FILES			0x00000400	/* set if open files shared between processes */
#define VKI_CLONE_SIGHAND		0x00000800	/* set if signal handlers and blocked signals shared */
#define VKI_CLONE_PIDFD			0x00001000	/* set if a pidfd should be placed in parent */
#define VKI_CLONE_PTRACE		0x00002000	/* set if we want to let tracing continue on the child too */
#define VKI_CLONE_VFORK			0x00004000	/* set if the parent wants the child to wake it up on mm_release */
#define VKI_CLONE_PARENT		0x00008000	/* set if we want to have the same parent as the cloner */
#define VKI_CLONE_THREAD		0x00010000	/* Same thread group? */
#define VKI_CLONE_NEWNS			0x00020000	/* New mount namespace group */
#define VKI_CLONE_SYSVSEM		0x00040000	/* share system V SEM_UNDO semantics */
#define VKI_CLONE_SETTLS		0x00080000	/* create a new TLS for the child */
#define VKI_CLONE_PARENT_SETTID		0x00100000	/* set the TID in the parent */
#define VKI_CLONE_CHILD_CLEARTID	0x00200000	/* clear the TID in the child */
#define VKI_CLONE_DETACHED		0x00400000	/* Unused, ignored */
#define VKI_CLONE_UNTRACED		0x00800000	/* set if the tracing process can't force CLONE_PTRACE on this clone */
#define VKI_CLONE_CHILD_SETTID		0x01000000	/* set the TID in the child */
#define VKI_CLONE_NEWCGROUP		0x02000000	/* New cgroup namespace */
#define VKI_CLONE_NEWUTS		0x04000000	/* New utsname namespace */
#define VKI_CLONE_NEWIPC		0x08000000	/* New ipc namespace */
#define VKI_CLONE_NEWUSER		0x10000000	/* New user namespace */
#define VKI_CLONE_NEWPID		0x20000000	/* New pid namespace */
#define VKI_CLONE_NEWNET		0x40000000	/* New network namespace */
#define VKI_CLONE_IO			0x80000000	/* Clone io context */

struct vki_sched_param {
	int sched_priority;
};

#define VKI_TASK_COMM_LEN 16

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-generic/siginfo.h
//----------------------------------------------------------------------

// Some archs, such as MIPS, have non-standard vki_siginfo.
#ifndef HAVE_ARCH_SIGINFO_T
typedef union vki_sigval {
	int sival_int;
	void __user *sival_ptr;
} vki_sigval_t;

#ifndef __VKI_ARCH_SI_PREAMBLE_SIZE
#define __VKI_ARCH_SI_PREAMBLE_SIZE	(3 * sizeof(int))
#endif

#define VKI_SI_MAX_SIZE	128

#ifndef VKI_SI_PAD_SIZE
#define VKI_SI_PAD_SIZE	((VKI_SI_MAX_SIZE - __VKI_ARCH_SI_PREAMBLE_SIZE) / sizeof(int))
#endif

#ifndef __VKI_ARCH_SI_UID_T
#define __VKI_ARCH_SI_UID_T	vki_uid_t
#endif

#ifndef __VKI_ARCH_SI_BAND_T
#define __VKI_ARCH_SI_BAND_T long
#endif

#ifndef __VKI_ARCH_SI_CLOCK_T
#define __VKI_ARCH_SI_CLOCK_T vki_clock_t
#endif

#ifndef __VKI_ARCH_SI_ATTRIBUTES
#define __VKI_ARCH_SI_ATTRIBUTES
#endif

// [[Nb: this type changed between 2.4 and 2.6, but not in a way that
// affects Valgrind.]]
typedef struct vki_siginfo {
	int si_signo;
	int si_errno;
	int si_code;

	union {
		int _pad[VKI_SI_PAD_SIZE];

		/* kill() */
		struct {
			vki_pid_t _pid;		/* sender's pid */
			__VKI_ARCH_SI_UID_T _uid;	/* sender's uid */
		} _kill;

		/* POSIX.1b timers */
		struct {
			vki_timer_t _tid;		/* timer id */
			int _overrun;		/* overrun count */
			char _pad[sizeof( __VKI_ARCH_SI_UID_T) - sizeof(int)];
			vki_sigval_t _sigval;	/* same as below */
			int _sys_private;       /* not to be passed to user */
		} _timer;

		/* POSIX.1b signals */
		struct {
			vki_pid_t _pid;		/* sender's pid */
			__VKI_ARCH_SI_UID_T _uid;	/* sender's uid */
			vki_sigval_t _sigval;
		} _rt;

		/* SIGCHLD */
		struct {
			vki_pid_t _pid;		/* which child */
			__VKI_ARCH_SI_UID_T _uid;	/* sender's uid */
			int _status;		/* exit code */
			__VKI_ARCH_SI_CLOCK_T _utime;
			__VKI_ARCH_SI_CLOCK_T _stime;
		} _sigchld;

		/* SIGILL, SIGFPE, SIGSEGV, SIGBUS */
		struct {
			void __user *_addr; /* faulting insn/memory ref. */
#ifdef __ARCH_SI_TRAPNO
			int _trapno;	/* TRAP # which caused the signal */
#endif
		} _sigfault;

		/* SIGPOLL */
		struct {
			__VKI_ARCH_SI_BAND_T _band;	/* POLL_IN, POLL_OUT, POLL_MSG */
			int _fd;
		} _sigpoll;
	} _sifields;
} __VKI_ARCH_SI_ATTRIBUTES vki_siginfo_t;
#endif

#define __VKI_SI_FAULT	0

/*
 * si_code values
 * Digital reserves positive values for kernel-generated signals.
 */
#define VKI_SI_USER	0		/* sent by kill, sigsend, raise */
#define VKI_SI_TKILL	-6		/* sent by tkill system call */

/*
 * SIGILL si_codes
 */
#define VKI_ILL_ILLOPC	(__VKI_SI_FAULT|1)	/* illegal opcode */
#define VKI_ILL_ILLOPN	(__VKI_SI_FAULT|2)	/* illegal operand */
#define VKI_ILL_ILLADR	(__VKI_SI_FAULT|3)	/* illegal addressing mode */
#define VKI_ILL_ILLTRP	(__VKI_SI_FAULT|4)	/* illegal trap */
#define VKI_ILL_PRVOPC	(__VKI_SI_FAULT|5)	/* privileged opcode */
#define VKI_ILL_PRVREG	(__VKI_SI_FAULT|6)	/* privileged register */
#define VKI_ILL_COPROC	(__VKI_SI_FAULT|7)	/* coprocessor error */
#define VKI_ILL_BADSTK	(__VKI_SI_FAULT|8)	/* internal stack error */

/*
 * SIGFPE si_codes
 */
#define VKI_FPE_INTDIV	(__VKI_SI_FAULT|1)	/* integer divide by zero */
#define VKI_FPE_INTOVF	(__VKI_SI_FAULT|2)	/* integer overflow */
#define VKI_FPE_FLTDIV	(__VKI_SI_FAULT|3)	/* floating point divide by zero */
#define VKI_FPE_FLTOVF	(__VKI_SI_FAULT|4)	/* floating point overflow */
#define VKI_FPE_FLTUND	(__VKI_SI_FAULT|5)	/* floating point underflow */
#define VKI_FPE_FLTRES	(__VKI_SI_FAULT|6)	/* floating point inexact result */
#define VKI_FPE_FLTINV	(__VKI_SI_FAULT|7)	/* floating point invalid operation */
#define VKI_FPE_FLTSUB	(__VKI_SI_FAULT|8)	/* subscript out of range */

/*
 * SIGSEGV si_codes
 */
#define VKI_SEGV_MAPERR	(__VKI_SI_FAULT|1)	/* address not mapped to object */
#define VKI_SEGV_ACCERR	(__VKI_SI_FAULT|2)	/* invalid permissions for mapped object */

/*
 * SIGBUS si_codes
 */
#define VKI_BUS_ADRALN	(__VKI_SI_FAULT|1)	/* invalid address alignment */
#define VKI_BUS_ADRERR	(__VKI_SI_FAULT|2)	/* non-existent physical address */
#define VKI_BUS_OBJERR	(__VKI_SI_FAULT|3)	/* object specific hardware error */

/*
 * SIGTRAP si_codes
 */
#define VKI_TRAP_BRKPT      (__VKI_SI_FAULT|1)  /* process breakpoint */
#define VKI_TRAP_TRACE      (__VKI_SI_FAULT|2)  /* process trace trap */

/*
 * This works because the alignment is ok on all current architectures
 * but we leave open this being overridden in the future
 */
#ifndef VKI___ARCH_SIGEV_PREAMBLE_SIZE
#define VKI___ARCH_SIGEV_PREAMBLE_SIZE	(sizeof(int) * 2 + sizeof(vki_sigval_t))
#endif

#define VKI_SIGEV_MAX_SIZE	64
#define VKI_SIGEV_PAD_SIZE	((VKI_SIGEV_MAX_SIZE - VKI___ARCH_SIGEV_PREAMBLE_SIZE) \
		/ sizeof(int))

/* This is the flag the kernel handles, userspace/glibc handles SEGEV_THEAD. */
#define VKI_SIGEV_THREAD_ID	4

typedef struct vki_sigevent {
	vki_sigval_t sigev_value;
	int sigev_signo;
	int sigev_notify;
	union {
		int _pad[VKI_SIGEV_PAD_SIZE];
		 int _tid;

		struct {
			void (*_function)(vki_sigval_t);
			void *_attribute;	/* really pthread_attr_t */
		} _sigev_thread;
	} _sigev_un;
} vki_sigevent_t;

#define vki_sigev_notify_thread_id	_sigev_un._tid

//----------------------------------------------------------------------
// From elsewhere...
//----------------------------------------------------------------------

// [[The kernel actually uses the numbers 0,1,2 directly here, believe it or
// not.  So we introduce our own constants, based on the glibc ones.]]
#define VKI_SEEK_SET              0
#define VKI_SEEK_CUR              1
#define VKI_SEEK_END              2

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/net.h
//----------------------------------------------------------------------

#define VKI_SYS_SOCKET		1	/* sys_socket(2)		*/
#define VKI_SYS_BIND		2	/* sys_bind(2)			*/
#define VKI_SYS_CONNECT		3	/* sys_connect(2)		*/
#define VKI_SYS_LISTEN		4	/* sys_listen(2)		*/
#define VKI_SYS_ACCEPT		5	/* sys_accept(2)		*/
#define VKI_SYS_GETSOCKNAME	6	/* sys_getsockname(2)		*/
#define VKI_SYS_GETPEERNAME	7	/* sys_getpeername(2)		*/
#define VKI_SYS_SOCKETPAIR	8	/* sys_socketpair(2)		*/
#define VKI_SYS_SEND		9	/* sys_send(2)			*/
#define VKI_SYS_RECV		10	/* sys_recv(2)			*/
#define VKI_SYS_SENDTO		11	/* sys_sendto(2)		*/
#define VKI_SYS_RECVFROM	12	/* sys_recvfrom(2)		*/
#define VKI_SYS_SHUTDOWN	13	/* sys_shutdown(2)		*/
#define VKI_SYS_SETSOCKOPT	14	/* sys_setsockopt(2)		*/
#define VKI_SYS_GETSOCKOPT	15	/* sys_getsockopt(2)		*/
#define VKI_SYS_SENDMSG		16	/* sys_sendmsg(2)		*/
#define VKI_SYS_RECVMSG		17	/* sys_recvmsg(2)		*/
#define VKI_SYS_ACCEPT4		18	/* sys_accept4(2)		*/
#define VKI_SYS_RECVMMSG	19	/* sys_recvmmsg(2)              */
#define VKI_SYS_SENDMMSG	20	/* sys_sendmmsg(2)              */

#ifndef ARCH_HAS_SOCKET_TYPES
enum vki_sock_type {
	VKI_SOCK_STREAM	= 1,
	// [[others omitted]]
};
#endif /* ARCH_HAS_SOCKET_TYPES */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/uio.h
//----------------------------------------------------------------------

struct vki_iovec
{
	void __user *iov_base;	/* BSD uses caddr_t (1003.1g requires void *) */
	__vki_kernel_size_t iov_len; /* Must be size_t (1003.1g) */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/socket.h
//----------------------------------------------------------------------

// [[Resolved arbitrarily;  doesn't really matter whether it's '__inline__'
//   or 'inline']]
#define __KINLINE static __inline__

typedef unsigned short	vki_sa_family_t;

struct vki_sockaddr {
	vki_sa_family_t	sa_family;	/* address family, AF_xxx	*/
	char		sa_data[14];	/* 14 bytes of protocol address	*/
};

struct vki_msghdr {
	void	*	msg_name;	/* Socket name			*/
	int		msg_namelen;	/* Length of name		*/
	struct vki_iovec *	msg_iov;	/* Data blocks			*/
	__vki_kernel_size_t	msg_iovlen;	/* Number of blocks		*/
	void 	*	msg_control;	/* Per protocol magic (eg BSD file descriptor passing) */
	__vki_kernel_size_t	msg_controllen;	/* Length of cmsg list */
	unsigned	msg_flags;
};

struct vki_mmsghdr {
	struct vki_msghdr   msg_hdr;
	unsigned        msg_len;
};

struct vki_cmsghdr {
	__vki_kernel_size_t	cmsg_len;	/* data byte count, including hdr */
        int		cmsg_level;	/* originating protocol */
        int		cmsg_type;	/* protocol-specific type */
};

#define __VKI_CMSG_NXTHDR(ctl, len, cmsg) __vki_cmsg_nxthdr((ctl),(len),(cmsg))
#define VKI_CMSG_NXTHDR(mhdr, cmsg) vki_cmsg_nxthdr((mhdr), (cmsg))

#define VKI_CMSG_ALIGN(len) ( ((len)+sizeof(long)-1) & ~(sizeof(long)-1) )

#define VKI_CMSG_DATA(cmsg)	((void *)((char *)(cmsg) + VKI_CMSG_ALIGN(sizeof(struct vki_cmsghdr))))

#define __VKI_CMSG_FIRSTHDR(ctl,len) ((len) >= sizeof(struct vki_cmsghdr) ? \
				  (struct vki_cmsghdr *)(ctl) : \
				  (struct vki_cmsghdr *)NULL)
#define VKI_CMSG_FIRSTHDR(msg)	__VKI_CMSG_FIRSTHDR((msg)->msg_control, (msg)->msg_controllen)

// [[Urgh, this is revolting...]
__KINLINE struct vki_cmsghdr * __vki_cmsg_nxthdr(void *__ctl, __vki_kernel_size_t __size,
					       struct vki_cmsghdr *__cmsg)
{
	struct vki_cmsghdr * __ptr;

	__ptr = ASSUME_ALIGNED(struct vki_cmsghdr *,
        	((unsigned char *) __cmsg) +  VKI_CMSG_ALIGN(__cmsg->cmsg_len));
	if ((unsigned long)((char*)(__ptr+1) - (char *) __ctl) > __size)
		return (struct vki_cmsghdr *)0;

	return __ptr;
}

__KINLINE struct vki_cmsghdr * vki_cmsg_nxthdr (struct vki_msghdr *__msg, struct vki_cmsghdr *__cmsg)
{
	return __vki_cmsg_nxthdr(__msg->msg_control, __msg->msg_controllen, __cmsg);
}

#define	VKI_SCM_RIGHTS	0x01		/* rw: access rights (array of int) */

#define VKI_AF_UNSPEC   0
#define VKI_AF_UNIX	1	/* Unix domain sockets 		*/
#define VKI_AF_INET	2	/* Internet IP Protocol		*/
#define VKI_AF_INET6	10	/* IP version 6			*/
#define VKI_AF_NETLINK  16
#define VKI_AF_BLUETOOTH 31	/* Bluetooth sockets		*/

#define VKI_MSG_NOSIGNAL	0x4000	/* Do not generate SIGPIPE */

#define VKI_SOL_SCTP	132

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/in.h
//----------------------------------------------------------------------

struct vki_in_addr {
	__vki_u32	s_addr;
};

/* Structure describing an Internet (IP) socket address. */
#define __VKI_SOCK_SIZE__	16	/* sizeof(struct sockaddr)	*/
struct vki_sockaddr_in {
  vki_sa_family_t	sin_family;	/* Address family		*/
  unsigned short int	sin_port;	/* Port number			*/
  struct vki_in_addr	sin_addr;	/* Internet address		*/

  /* Pad to size of `struct sockaddr'. */
  unsigned char		__pad[__VKI_SOCK_SIZE__ - sizeof(short int) -
			sizeof(unsigned short int) - sizeof(struct vki_in_addr)];
};

#define VKI_IPPROTO_TCP 6       /* Transmission Control Protocol        */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/in6.h
//----------------------------------------------------------------------

struct vki_in6_addr
{
	union 
	{
		__vki_u8	u6_addr8[16];
		__vki_u16	u6_addr16[8];
		__vki_u32	u6_addr32[4];
	} vki_in6_u;
#define vki_s6_addr		vki_in6_u.u6_addr8
#define vki_s6_addr16		vki_in6_u.u6_addr16
#define vki_s6_addr32		vki_in6_u.u6_addr32
};

struct vki_sockaddr_in6 {
	unsigned short int	sin6_family;    /* AF_INET6 */
	__vki_u16		sin6_port;      /* Transport layer port # */
	__vki_u32		sin6_flowinfo;  /* IPv6 flow information */
	struct vki_in6_addr	sin6_addr;      /* IPv6 address */
	__vki_u32		sin6_scope_id;  /* scope id (new in RFC2553) */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/tcp.h
//----------------------------------------------------------------------

#define VKI_TCP_NODELAY    1       /* Turn off Nagle's algorithm. */


//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/un.h
//----------------------------------------------------------------------

#define VKI_UNIX_PATH_MAX	108

struct vki_sockaddr_un {
	vki_sa_family_t sun_family;	/* AF_UNIX */
	char sun_path[VKI_UNIX_PATH_MAX];	/* pathname */
};

//----------------------------------------------------------------------
// From linux-3.15.8/include/uapi/linux/netlink.h
//----------------------------------------------------------------------

struct vki_sockaddr_nl {
        vki_sa_family_t    nl_family;      /* AF_NETLINK   */
        unsigned short     nl_pad;         /* zero         */
        __vki_u32          nl_pid;         /* port ID      */
        __vki_u32          nl_groups;      /* multicast groups mask */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/if.h
//----------------------------------------------------------------------

#define	VKI_IFNAMSIZ	16

struct vki_ifmap 
{
	unsigned long mem_start;
	unsigned long mem_end;
	unsigned short base_addr; 
	unsigned char irq;
	unsigned char dma;
	unsigned char port;
	/* 3 bytes spare */
};

struct vki_if_settings
{
	unsigned int type;	/* Type of physical device or protocol */
	unsigned int size;	/* Size of the data allocated by the caller */
	union {
                // [[Nb: converted these all to void* to avoid pulling in
                //   unnecessary headers]]]
		/* {atm/eth/dsl}_settings anyone ? */
		void /*raw_hdlc_proto		*/__user *raw_hdlc;
		void /*cisco_proto		*/__user *cisco;
		void /*fr_proto			*/__user *fr;
		void /*fr_proto_pvc		*/__user *fr_pvc;
		void /*fr_proto_pvc_info	*/__user *fr_pvc_info;

		/* interface settings */
		void /*sync_serial_settings	*/__user *sync;
		void /*te1_settings		*/__user *te1;
	} ifs_ifsu;
};

struct vki_ifreq 
{
#define VKI_IFHWADDRLEN	6
	union
	{
		char	ifrn_name[VKI_IFNAMSIZ];		/* if name, e.g. "en0" */
	} ifr_ifrn;
	
	union {
		struct	vki_sockaddr ifru_addr;
		struct	vki_sockaddr ifru_dstaddr;
		struct	vki_sockaddr ifru_broadaddr;
		struct	vki_sockaddr ifru_netmask;
		struct  vki_sockaddr ifru_hwaddr;
		short	ifru_flags;
		int	ifru_ivalue;
		int	ifru_mtu;
		struct  vki_ifmap ifru_map;
		char	ifru_slave[VKI_IFNAMSIZ];	/* Just fits the size */
		char	ifru_newname[VKI_IFNAMSIZ];
		void __user *	ifru_data;
		struct	vki_if_settings ifru_settings;
	} ifr_ifru;
};

#define vki_ifr_name	ifr_ifrn.ifrn_name	/* interface name 	*/
#define vki_ifr_hwaddr	ifr_ifru.ifru_hwaddr	/* MAC address 		*/
#define	vki_ifr_addr	ifr_ifru.ifru_addr	/* address		*/
#define	vki_ifr_dstaddr	ifr_ifru.ifru_dstaddr	/* other end of p-p lnk	*/
#define	vki_ifr_broadaddr ifr_ifru.ifru_broadaddr /* broadcast address	*/
#define	vki_ifr_netmask	ifr_ifru.ifru_netmask	/* interface net mask	*/
#define	vki_ifr_flags	ifr_ifru.ifru_flags	/* flags		*/
#define	vki_ifr_metric	ifr_ifru.ifru_ivalue	/* metric		*/
#define	vki_ifr_mtu	ifr_ifru.ifru_mtu	/* mtu			*/
#define vki_ifr_map	ifr_ifru.ifru_map	/* device map		*/
#define vki_ifr_slave	ifr_ifru.ifru_slave	/* slave device		*/
#define	vki_ifr_data	ifr_ifru.ifru_data	/* for use by interface	*/
#define vki_ifr_ifindex	ifr_ifru.ifru_ivalue	/* interface index	*/
#define vki_ifr_bandwidth ifr_ifru.ifru_ivalue  /* link bandwidth	*/
#define vki_ifr_qlen	ifr_ifru.ifru_ivalue	/* Queue length 	*/
#define vki_ifr_newname	ifr_ifru.ifru_newname	/* New name		*/
#define vki_ifr_settings ifr_ifru.ifru_settings	/* Device/proto settings*/

struct vki_ifconf 
{
	int	ifc_len;			/* size of buffer	*/
	union 
	{
		char __user *ifcu_buf;
		struct vki_ifreq __user *ifcu_req;
	} ifc_ifcu;
};
#define	vki_ifc_buf	ifc_ifcu.ifcu_buf	/* buffer address	*/

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/if_arp.h
//----------------------------------------------------------------------

struct vki_arpreq {
  struct vki_sockaddr	arp_pa;		/* protocol address		*/
  struct vki_sockaddr	arp_ha;		/* hardware address		*/
  int			arp_flags;	/* flags			*/
  struct vki_sockaddr   arp_netmask;    /* netmask (only for proxy arps) */
  char			arp_dev[16];
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/route.h
//----------------------------------------------------------------------

struct vki_rtentry 
{
	unsigned long	rt_pad1;
	struct vki_sockaddr	rt_dst;		/* target address		*/
	struct vki_sockaddr	rt_gateway;	/* gateway addr (RTF_GATEWAY)	*/
	struct vki_sockaddr	rt_genmask;	/* target network mask (IP)	*/
	unsigned short	rt_flags;
	short		rt_pad2;
	unsigned long	rt_pad3;
	void		*rt_pad4;
	short		rt_metric;	/* +1 for binary compatibility!	*/
	char __user	*rt_dev;	/* forcing the device at add	*/
	unsigned long	rt_mtu;		/* per route MTU/Window 	*/
// [[Not important for Valgrind]]
//#ifndef __KERNEL__
//#define rt_mss	rt_mtu		/* Compatibility :-(            */
//#endif
	unsigned long	rt_window;	/* Window clamping 		*/
	unsigned short	rt_irtt;	/* Initial RTT			*/
};

//----------------------------------------------------------------------
// From linux-2.6.13-rc5/include/net/sctp/user.h
//----------------------------------------------------------------------

typedef __vki_s32 vki_sctp_assoc_t;

enum vki_sctp_optname {
	VKI_SCTP_RTOINFO,
#define VKI_SCTP_RTOINFO VKI_SCTP_RTOINFO
	VKI_SCTP_ASSOCINFO,
#define VKI_SCTP_ASSOCINFO VKI_SCTP_ASSOCINFO
	VKI_SCTP_INITMSG,
#define VKI_SCTP_INITMSG VKI_SCTP_INITMSG
	VKI_SCTP_NODELAY, 	/* Get/set nodelay option. */
#define VKI_SCTP_NODELAY	VKI_SCTP_NODELAY
	VKI_SCTP_AUTOCLOSE,
#define VKI_SCTP_AUTOCLOSE VKI_SCTP_AUTOCLOSE
	VKI_SCTP_SET_PEER_PRIMARY_ADDR, 
#define VKI_SCTP_SET_PEER_PRIMARY_ADDR VKI_SCTP_SET_PEER_PRIMARY_ADDR
	VKI_SCTP_PRIMARY_ADDR,
#define VKI_SCTP_PRIMARY_ADDR VKI_SCTP_PRIMARY_ADDR
	VKI_SCTP_ADAPTION_LAYER,      
#define VKI_SCTP_ADAPTION_LAYER VKI_SCTP_ADAPTION_LAYER
	VKI_SCTP_DISABLE_FRAGMENTS,
#define VKI_SCTP_DISABLE_FRAGMENTS VKI_SCTP_DISABLE_FRAGMENTS
	VKI_SCTP_PEER_ADDR_PARAMS,
#define VKI_SCTP_PEER_ADDR_PARAMS VKI_SCTP_PEER_ADDR_PARAMS
	VKI_SCTP_DEFAULT_SEND_PARAM,
#define VKI_SCTP_DEFAULT_SEND_PARAM VKI_SCTP_DEFAULT_SEND_PARAM
	VKI_SCTP_EVENTS,
#define VKI_SCTP_EVENTS VKI_SCTP_EVENTS
	VKI_SCTP_I_WANT_MAPPED_V4_ADDR,  /* Turn on/off mapped v4 addresses  */
#define VKI_SCTP_I_WANT_MAPPED_V4_ADDR VKI_SCTP_I_WANT_MAPPED_V4_ADDR
	VKI_SCTP_MAXSEG, 	/* Get/set maximum fragment. */
#define VKI_SCTP_MAXSEG 	VKI_SCTP_MAXSEG
	VKI_SCTP_STATUS,
#define VKI_SCTP_STATUS VKI_SCTP_STATUS
	VKI_SCTP_GET_PEER_ADDR_INFO,
#define VKI_SCTP_GET_PEER_ADDR_INFO VKI_SCTP_GET_PEER_ADDR_INFO

	/* Internal Socket Options. Some of the sctp library functions are 
	 * implemented using these socket options.
	 */
	VKI_SCTP_SOCKOPT_BINDX_ADD = 100,/* BINDX requests for adding addresses. */
#define VKI_SCTP_SOCKOPT_BINDX_ADD	VKI_SCTP_SOCKOPT_BINDX_ADD
	VKI_SCTP_SOCKOPT_BINDX_REM, /* BINDX requests for removing addresses. */
#define VKI_SCTP_SOCKOPT_BINDX_REM	VKI_SCTP_SOCKOPT_BINDX_REM
	VKI_SCTP_SOCKOPT_PEELOFF, 	/* peel off association. */
#define VKI_SCTP_SOCKOPT_PEELOFF	VKI_SCTP_SOCKOPT_PEELOFF
	VKI_SCTP_GET_PEER_ADDRS_NUM, 	/* Get number of peer addresss. */
#define VKI_SCTP_GET_PEER_ADDRS_NUM	VKI_SCTP_GET_PEER_ADDRS_NUM
	VKI_SCTP_GET_PEER_ADDRS, 	/* Get all peer addresss. */
#define VKI_SCTP_GET_PEER_ADDRS	VKI_SCTP_GET_PEER_ADDRS
	VKI_SCTP_GET_LOCAL_ADDRS_NUM, 	/* Get number of local addresss. */
#define VKI_SCTP_GET_LOCAL_ADDRS_NUM	VKI_SCTP_GET_LOCAL_ADDRS_NUM
	VKI_SCTP_GET_LOCAL_ADDRS, 	/* Get all local addresss. */
#define VKI_SCTP_GET_LOCAL_ADDRS	VKI_SCTP_GET_LOCAL_ADDRS
	VKI_SCTP_SOCKOPT_CONNECTX, /* CONNECTX requests. */
#define VKI_SCTP_SOCKOPT_CONNECTX	VKI_SCTP_SOCKOPT_CONNECTX
};

struct vki_sctp_getaddrs {
	vki_sctp_assoc_t        assoc_id;
	int			addr_num;
	struct vki_sockaddr	*addrs;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/resource.h
//----------------------------------------------------------------------

#define VKI_RUSAGE_SELF     0
#define VKI_RUSAGE_CHILDREN (-1)
#define VKI_RUSAGE_BOTH     (-2)        /* sys_wait4() uses this */
#define VKI_RUSAGE_THREAD   1           /* only the calling thread */

struct	vki_rusage {
	struct vki_timeval ru_utime;	/* user time used */
	struct vki_timeval ru_stime;	/* system time used */
	long	ru_maxrss;		/* maximum resident set size */
	long	ru_ixrss;		/* integral shared memory size */
	long	ru_idrss;		/* integral unshared data size */
	long	ru_isrss;		/* integral unshared stack size */
	long	ru_minflt;		/* page reclaims */
	long	ru_majflt;		/* page faults */
	long	ru_nswap;		/* swaps */
	long	ru_inblock;		/* block input operations */
	long	ru_oublock;		/* block output operations */
	long	ru_msgsnd;		/* messages sent */
	long	ru_msgrcv;		/* messages received */
	long	ru_nsignals;		/* signals received */
	long	ru_nvcsw;		/* voluntary context switches */
	long	ru_nivcsw;		/* involuntary " */
};

struct vki_rlimit {
	unsigned long	rlim_cur;
	unsigned long	rlim_max;
};

struct vki_rlimit64 {
	__vki_u64 rlim_cur;
	__vki_u64 rlim_max;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/elfcore.h
//----------------------------------------------------------------------

struct vki_elf_siginfo
{
	int	si_signo;			/* signal number */
	int	si_code;			/* extra code */
	int	si_errno;			/* errno */
};

// [[Removed some commented out lines here]]
struct vki_elf_prstatus
{
	struct vki_elf_siginfo pr_info;	/* Info associated with signal */
	short	pr_cursig;		/* Current signal */
	unsigned long pr_sigpend;	/* Set of pending signals */
	unsigned long pr_sighold;	/* Set of held signals */
	vki_pid_t	pr_pid;
	vki_pid_t	pr_ppid;
	vki_pid_t	pr_pgrp;
	vki_pid_t	pr_sid;
	struct vki_timeval pr_utime;	/* User time */
	struct vki_timeval pr_stime;	/* System time */
	struct vki_timeval pr_cutime;	/* Cumulative user time */
	struct vki_timeval pr_cstime;	/* Cumulative system time */
	vki_elf_gregset_t pr_reg;	/* GP registers */
	int pr_fpvalid;		/* True if math co-processor being used.  */
};

#define VKI_ELF_PRARGSZ	(80)	/* Number of chars for args */

struct vki_elf_prpsinfo
{
	char	pr_state;	/* numeric process state */
	char	pr_sname;	/* char for pr_state */
	char	pr_zomb;	/* zombie */
	char	pr_nice;	/* nice val */
	unsigned long pr_flag;	/* flags */
	__vki_kernel_uid_t	pr_uid;
	__vki_kernel_gid_t	pr_gid;
	vki_pid_t	pr_pid, pr_ppid, pr_pgrp, pr_sid;
	/* Lots missing */
	char	pr_fname[16];	/* filename of executable */
	char	pr_psargs[VKI_ELF_PRARGSZ];	/* initial part of arg list */
};

//----------------------------------------------------------------------
// From linux-2.6.12.1/include/linux/eventpoll.h
//----------------------------------------------------------------------

/* Valid opcodes to issue to sys_epoll_ctl() */
#define VKI_EPOLL_CTL_ADD 1
#define VKI_EPOLL_CTL_DEL 2
#define VKI_EPOLL_CTL_MOD 3

#ifdef __x86_64__
#define VKI_EPOLL_PACKED __attribute__((packed))
#else
#define VKI_EPOLL_PACKED
#endif

struct vki_epoll_event {
	__vki_u32 events;
	__vki_u64 data;
} VKI_EPOLL_PACKED;


//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/mqueue.h
//----------------------------------------------------------------------

struct vki_mq_attr {
	long	mq_flags;	/* message queue flags			*/
	long	mq_maxmsg;	/* maximum number of messages		*/
	long	mq_msgsize;	/* maximum message size			*/
	long	mq_curmsgs;	/* number of messages currently queued	*/
	long	__reserved[4];	/* ignored for input, zeroed for output */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/utsname.h
//----------------------------------------------------------------------

struct vki_new_utsname {
	char sysname[65];
	char nodename[65];
	char release[65];
	char version[65];
	char machine[65];
	char domainname[65];
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/mii.h
//----------------------------------------------------------------------

/* This structure is used in all SIOCxMIIxxx ioctl calls */
struct vki_mii_ioctl_data {
	vki_u16		phy_id;
	vki_u16		reg_num;
	vki_u16		val_in;
	vki_u16		val_out;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/capability.h
//----------------------------------------------------------------------

// [[capget()/capset() man page says this, ominously:
//
//   The kernel API is likely to change and use of these functions  (in
//   particular the format of the cap_user_*_t types) is subject to
//   change with each kernel revision.
//
// However, the format hasn't changed since at least Linux 2.4.6.]]

typedef struct __vki_user_cap_header_struct {
	__vki_u32 version;
	int pid;
} __user *vki_cap_user_header_t;
 
typedef struct __vki_user_cap_data_struct {
        __vki_u32 effective;
        __vki_u32 permitted;
        __vki_u32 inheritable;
} __user *vki_cap_user_data_t;
  

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/module.h
//----------------------------------------------------------------------

// [[We do a VKI_SIZEOF_* here because this type is so big, and its size
//   depends on the word size, so see vki_arch.h]]

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/ipc.h
//----------------------------------------------------------------------

/* Obsolete, used only for backwards compatibility and libc5 compiles */
struct vki_ipc_perm
{
	__vki_kernel_key_t	key;
	__vki_kernel_uid_t	uid;
	__vki_kernel_gid_t	gid;
	__vki_kernel_uid_t	cuid;
	__vki_kernel_gid_t	cgid;
	__vki_kernel_mode_t	mode; 
	unsigned short	seq;
};

#define VKI_IPC_CREAT  00001000   /* create if key is nonexistent */
#define VKI_IPC_EXCL   00002000   /* fail if key exists */
#define VKI_IPC_NOWAIT 00004000   /* return error on wait */

//#define VKI_IPC_RMID 0     /* remove resource */
#define VKI_IPC_SET  1     /* set ipc_perm options */
#define VKI_IPC_STAT 2     /* get ipc_perm options */
#define VKI_IPC_INFO 3     /* see ipcs */

#define VKI_IPC_64  0x0100  /* New version (support 32-bit UIDs, bigger
			       message sizes, etc. */
// From /usr/include/bits/shm.h
# define VKI_SHM_HUGETLB   04000


//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/sem.h
//----------------------------------------------------------------------

#define VKI_GETALL  13       /* get all semval's */
#define VKI_SETVAL  16       /* set semval */
#define VKI_SETALL  17       /* set all semval's */

#define VKI_SEM_STAT 18
#define VKI_SEM_INFO 19

/* Obsolete, used only for backwards compatibility and libc5 compiles */
struct vki_semid_ds {
	struct vki_ipc_perm	sem_perm;		/* permissions .. see ipc.h */
	__vki_kernel_time_t	sem_otime;		/* last semop time */
	__vki_kernel_time_t	sem_ctime;		/* last change time */
        // [[Use void* to avoid excess header copying]]
	void/*struct sem	*/*sem_base;		/* ptr to first semaphore in array */
	void/*struct sem_queue */*sem_pending;		/* pending operations to be processed */
	void/*struct sem_queue */**sem_pending_last;	/* last pending operation */
	void/*struct sem_undo	*/*undo;			/* undo requests on this array */
	unsigned short	sem_nsems;		/* no. of semaphores in array */
};

struct vki_sembuf {
	unsigned short  sem_num;	/* semaphore index in array */
	short		sem_op;		/* semaphore operation */
	short		sem_flg;	/* operation flags */
};

union vki_semun {
	int val;			/* value for SETVAL */
	struct vki_semid_ds __user *buf;	/* buffer for IPC_STAT & IPC_SET */
	struct vki_semid64_ds __user *buf64;	/* buffer for IPC_STAT & IPC_SET */
	unsigned short __user *array;	/* array for GETALL & SETALL */
	struct vki_seminfo __user *__buf;	/* buffer for IPC_INFO */
	void __user *__pad;
};

struct  vki_seminfo {
	int semmap;
	int semmni;
	int semmns;
	int semmnu;
	int semmsl;
	int semopm;
	int semume;
	int semusz;
	int semvmx;
	int semaem;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-generic/errno-base.h
//----------------------------------------------------------------------

#define	VKI_EPERM		 1	/* Operation not permitted */
#define	VKI_ENOENT		 2	/* No such file or directory */
#define	VKI_ESRCH		 3	/* No such process */
#define	VKI_EINTR		 4	/* Interrupted system call */
#define	VKI_EIO			 5	/* I/O error */
#define	VKI_ENXIO		 6	/* No such device or address */
#define	VKI_E2BIG		 7	/* Argument list too long */
#define	VKI_ENOEXEC		 8	/* Exec format error */
#define	VKI_EBADF		 9	/* Bad file number */
#define	VKI_ECHILD		10	/* No child processes */
#define	VKI_EAGAIN		11	/* Try again */
#define	VKI_ENOMEM		12	/* Out of memory */
#define	VKI_EACCES		13	/* Permission denied */
#define	VKI_EFAULT		14	/* Bad address */
#define	VKI_ENOTBLK		15	/* Block device required */
#define	VKI_EBUSY		16	/* Device or resource busy */
#define	VKI_EEXIST		17	/* File exists */
#define	VKI_EXDEV		18	/* Cross-device link */
#define	VKI_ENODEV		19	/* No such device */
#define	VKI_ENOTDIR		20	/* Not a directory */
#define	VKI_EISDIR		21	/* Is a directory */
#define	VKI_EINVAL		22	/* Invalid argument */
#define	VKI_ENFILE		23	/* File table overflow */
#define	VKI_EMFILE		24	/* Too many open files */
#define	VKI_ENOTTY		25	/* Not a typewriter */
#define	VKI_ETXTBSY		26	/* Text file busy */
#define	VKI_EFBIG		27	/* File too large */
#define	VKI_ENOSPC		28	/* No space left on device */
#define	VKI_ESPIPE		29	/* Illegal seek */
#define	VKI_EROFS		30	/* Read-only file system */
#define	VKI_EMLINK		31	/* Too many links */
#define	VKI_EPIPE		32	/* Broken pipe */
#define	VKI_EDOM		33	/* Math argument out of domain of func */
#define	VKI_ERANGE		34	/* Math result not representable */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-generic/errno.h
//----------------------------------------------------------------------

#define VKI_EWOULDBLOCK		VKI_EAGAIN

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/wait.h
//----------------------------------------------------------------------

#define VKI_WNOHANG	0x00000001

#define __VKI_WALL	0x40000000	/* Wait on all children, regardless of type */
#define __VKI_WCLONE	0x80000000	/* Wait only on non-SIGCHLD children */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/mman.h
//----------------------------------------------------------------------

#define VKI_MREMAP_MAYMOVE	1
#define VKI_MREMAP_FIXED	2

//----------------------------------------------------------------------
// From linux-2.6.31-rc4/include/linux/futex.h
//----------------------------------------------------------------------

#define VKI_FUTEX_WAIT (0)
#define VKI_FUTEX_WAKE (1)
#define VKI_FUTEX_FD (2)
#define VKI_FUTEX_REQUEUE (3)
#define VKI_FUTEX_CMP_REQUEUE (4)
#define VKI_FUTEX_WAKE_OP (5)
#define VKI_FUTEX_LOCK_PI (6)
#define VKI_FUTEX_UNLOCK_PI (7)
#define VKI_FUTEX_TRYLOCK_PI (8)
#define VKI_FUTEX_WAIT_BITSET (9)
#define VKI_FUTEX_WAKE_BITSET (10)
#define VKI_FUTEX_WAIT_REQUEUE_PI (11)
#define VKI_FUTEX_CMP_REQUEUE_PI (12)
#define VKI_FUTEX_PRIVATE_FLAG (128)
#define VKI_FUTEX_CLOCK_REALTIME (256)

struct vki_robust_list {
	struct vki_robust_list __user *next;
};

struct vki_robust_list_head {
	/*
	 * The head of the list. Points back to itself if empty:
	 */
	struct vki_robust_list list;

	/*
	 * This relative offset is set by user-space, it gives the kernel
	 * the relative position of the futex field to examine. This way
	 * we keep userspace flexible, to freely shape its data-structure,
	 * without hardcoding any particular offset into the kernel:
	 */
	long futex_offset;

	/*
	 * The death of the thread may race with userspace setting
	 * up a lock's links. So to handle this race, userspace first
	 * sets this field to the address of the to-be-taken lock,
	 * then does the lock acquire, and then adds itself to the
	 * list, and then clears this field. Hence the kernel will
	 * always have full knowledge of all locks that the thread
	 * _might_ have taken. We check the owner TID in any case,
	 * so only truly owned locks will be handled.
	 */
	struct vki_robust_list __user *list_op_pending;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/errno.h
//----------------------------------------------------------------------

#define VKI_ERESTARTSYS	512

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/stat.h
//----------------------------------------------------------------------

#define VKI_S_IFMT  00170000
#define VKI_S_IFSOCK 0140000
#define VKI_S_IFLNK  0120000
#define VKI_S_IFREG  0100000
#define VKI_S_IFBLK  0060000
#define VKI_S_IFDIR  0040000
#define VKI_S_IFCHR  0020000
#define VKI_S_IFIFO  0010000
#define VKI_S_ISUID  0004000
#define VKI_S_ISGID  0002000
#define VKI_S_ISVTX  0001000

#define VKI_S_ISLNK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFLNK)
#define VKI_S_ISREG(m)	(((m) & VKI_S_IFMT) == VKI_S_IFREG)
#define VKI_S_ISDIR(m)	(((m) & VKI_S_IFMT) == VKI_S_IFDIR)
#define VKI_S_ISCHR(m)	(((m) & VKI_S_IFMT) == VKI_S_IFCHR)
#define VKI_S_ISBLK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFBLK)
#define VKI_S_ISFIFO(m)	(((m) & VKI_S_IFMT) == VKI_S_IFIFO)
#define VKI_S_ISSOCK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFSOCK)

#define VKI_S_IRWXU 00700
#define VKI_S_IRUSR 00400
#define VKI_S_IWUSR 00200
#define VKI_S_IXUSR 00100

#define VKI_S_IRWXG 00070
#define VKI_S_IRGRP 00040
#define VKI_S_IWGRP 00020
#define VKI_S_IXGRP 00010

#define VKI_S_IRWXO 00007
#define VKI_S_IROTH 00004
#define VKI_S_IWOTH 00002
#define VKI_S_IXOTH 00001

#define VKI_STATX_ALL 0x00000FFFU

struct vki_statx_timestamp {
        __vki_s64   tv_sec;
        __vki_u32   tv_nsec;
        __vki_s32   __reserved;
};

struct vki_statx {
        /* 0x00 */
        __vki_u32   stx_mask;       /* What results were written [uncond] */
        __vki_u32   stx_blksize;    /* Preferred general I/O size [uncond] */
        __vki_u64   stx_attributes; /* Flags conveying information about the file [uncond] */
        /* 0x10 */
        __vki_u32   stx_nlink;      /* Number of hard links */
        __vki_u32   stx_uid;        /* User ID of owner */
        __vki_u32   stx_gid;        /* Group ID of owner */
        __vki_u16   stx_mode;       /* File mode */
        __vki_u16   __spare0[1];
        /* 0x20 */
        __vki_u64   stx_ino;        /* Inode number */
        __vki_u64   stx_size;       /* File size */
        __vki_u64   stx_blocks;     /* Number of 512-byte blocks allocated */
        __vki_u64   stx_attributes_mask; /* Mask to show what's supported in stx_attributes */
        /* 0x40 */
        struct vki_statx_timestamp  stx_atime;      /* Last access time */
        struct vki_statx_timestamp  stx_btime;      /* File creation time */
        struct vki_statx_timestamp  stx_ctime;      /* Last attribute change time */
        struct vki_statx_timestamp  stx_mtime;      /* Last data modification time */
        /* 0x80 */
        __vki_u32   stx_rdev_major; /* Device ID of special file [if bdev/cdev] */
        __vki_u32   stx_rdev_minor;
        __vki_u32   stx_dev_major;  /* ID of device containing file [uncond] */
        __vki_u32   stx_dev_minor;
        /* 0x90 */
        __vki_u64   __spare2[14];   /* Spare space for future expansion */
        /* 0x100 */
};


//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/dirent.h
//----------------------------------------------------------------------

/* This is the old compat structure to use with the old dirent syscall. */
struct vki_dirent {
	long		d_ino;
	__vki_kernel_off_t	d_off;
	unsigned short	d_reclen;
	char		d_name[256]; /* We must not include limits.h! */
};

/* This is the new structure to use with the dirent64 syscall. */
struct vki_dirent64 {
	__vki_u64 d_ino;
	__vki_s64 d_off;
	unsigned short d_reclen;
	unsigned char d_type;
	char d_name[256]; /* Note we hard code a max file length here. */
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/fcntl.h
//----------------------------------------------------------------------

#define VKI_F_SETLEASE      (VKI_F_LINUX_SPECIFIC_BASE + 0)
#define VKI_F_GETLEASE      (VKI_F_LINUX_SPECIFIC_BASE + 1)

#define VKI_F_CANCELLK      (VKI_F_LINUX_SPECIFIC_BASE + 5)

#define VKI_F_DUPFD_CLOEXEC (VKI_F_LINUX_SPECIFIC_BASE + 6)

#define VKI_F_NOTIFY        (VKI_F_LINUX_SPECIFIC_BASE + 2)

#define VKI_F_SETPIPE_SZ    (VKI_F_LINUX_SPECIFIC_BASE + 7)
#define VKI_F_GETPIPE_SZ    (VKI_F_LINUX_SPECIFIC_BASE + 8)

struct vki_flock {
	short			l_type;
	short			l_whence;
	__vki_kernel_off_t	l_start;
	__vki_kernel_off_t	l_len;
	__vki_kernel_pid_t	l_pid;
};

struct vki_flock64 {
	short			l_type;
	short			l_whence;
	__vki_kernel_loff_t	l_start;
	__vki_kernel_loff_t	l_len;
	__vki_kernel_pid_t	l_pid;
};

#define VKI_AT_EMPTY_PATH       0x1000  /* Allow empty relative pathname */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/sysctl.h
//----------------------------------------------------------------------

struct __vki_sysctl_args {
	int __user *name;
	int nlen;
	void __user *oldval;
	vki_size_t __user *oldlenp;
	void __user *newval;
	vki_size_t newlen;
	unsigned long __unused0[4];
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/aio_abi.h
//----------------------------------------------------------------------

typedef unsigned long	vki_aio_context_t;

enum {
	VKI_IOCB_CMD_PREAD = 0,
	VKI_IOCB_CMD_PWRITE = 1,
	VKI_IOCB_CMD_FSYNC = 2,
	VKI_IOCB_CMD_FDSYNC = 3,
	VKI_IOCB_CMD_PREADV = 7,
	VKI_IOCB_CMD_PWRITEV = 8,
};

/* read() from /dev/aio returns these structures. */
struct vki_io_event {
	__vki_u64	data;		/* the data field from the iocb */
	__vki_u64	obj;		/* what iocb this event came from */
        // [[Nb: These fields renamed from 'res' and 'res2' because 'res' is
        //   a macro in vg_syscalls.c!]]
	__vki_s64	result;		/* result code for this event */
	__vki_s64	result2;	/* secondary result */
};

#if defined(VKI_LITTLE_ENDIAN)
#  define VKI_PADDED(x,y)	x, y
#elif defined(VKI_BIG_ENDIAN)
#  define VKI_PADDED(x,y)	y, x
#else
#error edit for your odd byteorder.
#endif

struct vki_iocb {
	/* these are internal to the kernel/libc. */
	__vki_u64	aio_data;	/* data to be returned in event's data */
	__vki_u32	VKI_PADDED(aio_key, aio_reserved1);
				/* the kernel sets aio_key to the req # */

	/* common fields */
	__vki_u16	aio_lio_opcode;	/* see IOCB_CMD_ above */
	__vki_s16	aio_reqprio;
	__vki_u32	aio_fildes;

	__vki_u64	aio_buf;
	__vki_u64	aio_nbytes;
	__vki_s64	aio_offset;

	/* extra parameters */
	__vki_u64	aio_reserved2;	/* TODO: use this for a (struct sigevent *) */
	__vki_u64	aio_reserved3;
}; /* 64 bytes */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/aio.h
//----------------------------------------------------------------------

struct vki_aio_ring {
	unsigned	id;	/* kernel internal index number */
	unsigned	nr;	/* number of io_events */
	unsigned	head;
	unsigned	tail;

	unsigned	magic;
	unsigned	compat_features;
	unsigned	incompat_features;
	unsigned	header_length;	/* size of aio_ring */

	struct vki_io_event		io_events[0];
}; /* 128 bytes + ring size */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/msg.h
//----------------------------------------------------------------------

#define VKI_MSG_STAT 11
#define VKI_MSG_INFO 12

struct vki_msqid_ds {
	struct vki_ipc_perm msg_perm;
	struct vki_msg *msg_first;		/* first message on queue,unused  */
	struct vki_msg *msg_last;		/* last message in queue,unused */
	__vki_kernel_time_t msg_stime;	/* last msgsnd time */
	__vki_kernel_time_t msg_rtime;	/* last msgrcv time */
	__vki_kernel_time_t msg_ctime;	/* last change time */
	unsigned long  msg_lcbytes;	/* Reuse junk fields for 32 bit */
	unsigned long  msg_lqbytes;	/* ditto */
	unsigned short msg_cbytes;	/* current number of bytes on queue */
	unsigned short msg_qnum;	/* number of messages in queue */
	unsigned short msg_qbytes;	/* max number of bytes on queue */
	__vki_kernel_ipc_pid_t msg_lspid;	/* pid of last msgsnd */
	__vki_kernel_ipc_pid_t msg_lrpid;	/* last receive pid */
};

struct vki_msgbuf {
	long mtype;         /* type of message */
	char mtext[1];      /* message text */
};

struct vki_msginfo {
	int msgpool;
	int msgmap; 
	int msgmax; 
	int msgmnb; 
	int msgmni; 
	int msgssz; 
	int msgtql; 
	unsigned short  msgseg; 
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/shm.h
//----------------------------------------------------------------------

struct vki_shmid_ds {
	struct vki_ipc_perm		shm_perm;	/* operation perms */
	int			shm_segsz;	/* size of segment (bytes) */
	__vki_kernel_time_t		shm_atime;	/* last attach time */
	__vki_kernel_time_t		shm_dtime;	/* last detach time */
	__vki_kernel_time_t		shm_ctime;	/* last change time */
	__vki_kernel_ipc_pid_t	shm_cpid;	/* pid of creator */
	__vki_kernel_ipc_pid_t	shm_lpid;	/* pid of last operator */
	unsigned short		shm_nattch;	/* no. of current attaches */
	unsigned short 		shm_unused;	/* compatibility */
	void 			*shm_unused2;	/* ditto - used by DIPC */
	void			*shm_unused3;	/* unused */
};

#define VKI_SHM_RDONLY  010000  /* read-only access */
#define VKI_SHM_RND     020000  /* round attach address to SHMLBA boundary */

#define VKI_SHM_STAT 	13
#define VKI_SHM_INFO 	14

/* Obsolete, used only for backwards compatibility */
struct	vki_shminfo {
	int shmmax;
	int shmmin;
	int shmmni;
	int shmseg;
	int shmall;
};

struct vki_shm_info {
	int used_ids;
	unsigned long shm_tot;	/* total allocated shm */
	unsigned long shm_rss;	/* total resident shm */
	unsigned long shm_swp;	/* total swapped shm */
	unsigned long swap_attempts;
	unsigned long swap_successes;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/rtc.h
//----------------------------------------------------------------------

struct vki_rtc_time {
	int tm_sec;
	int tm_min;
	int tm_hour;
	int tm_mday;
	int tm_mon;
	int tm_year;
	int tm_wday;
	int tm_yday;
	int tm_isdst;
};

#define VKI_RTC_AIE_ON	_VKI_IO('p', 0x01)	/* Alarm int. enable on	*/
#define VKI_RTC_AIE_OFF	_VKI_IO('p', 0x02)	/* ... off		*/
#define VKI_RTC_UIE_ON	_VKI_IO('p', 0x03)	/* Update int. enable on*/
#define VKI_RTC_UIE_OFF	_VKI_IO('p', 0x04)	/* ... off		*/
#define VKI_RTC_PIE_ON	_VKI_IO('p', 0x05)	/* Periodic int. enable on*/
#define VKI_RTC_PIE_OFF	_VKI_IO('p', 0x06)	/* ... off		*/

#define VKI_RTC_ALM_SET		_VKI_IOW('p', 0x07, struct vki_rtc_time) /* Set alarm time  */
#define VKI_RTC_ALM_READ	_VKI_IOR('p', 0x08, struct vki_rtc_time) /* Read alarm time */
#define VKI_RTC_RD_TIME		_VKI_IOR('p', 0x09, struct vki_rtc_time) /* Read RTC time   */
//#define RTC_SET_TIME	_IOW('p', 0x0a, struct rtc_time) /* Set RTC time    */
#define VKI_RTC_IRQP_READ	_VKI_IOR('p', 0x0b, unsigned long)	 /* Read IRQ rate   */
#define VKI_RTC_IRQP_SET	_VKI_IOW('p', 0x0c, unsigned long)	 /* Set IRQ rate    */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/isdn.h
//----------------------------------------------------------------------

// [[Nb: Resolved this for the common case where CONFIG_COBALT_MICRO_SERVER
//   is not defined]]
#define VKI_ISDN_MAX_CHANNELS   64

#define VKI_IIOCGETCPS  _VKI_IO('I',21)

#define VKI_IIOCNETGPN  _VKI_IO('I',34)

#define VKI_ISDN_MSNLEN          32

typedef struct {
  char name[10];
  char phone[VKI_ISDN_MSNLEN];
  int  outgoing;
} vki_isdn_net_ioctl_phone;

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCOUTQ		VKI_TIOCOUTQ

#define VKI_SIOCADDRT		0x890B	/* add routing table entry	*/
#define VKI_SIOCDELRT		0x890C	/* delete routing table entry	*/

#define VKI_SIOCGIFNAME		0x8910	/* get iface name		*/
#define VKI_SIOCGIFCONF		0x8912	/* get iface list		*/
#define VKI_SIOCGIFFLAGS	0x8913	/* get flags			*/
#define VKI_SIOCSIFFLAGS	0x8914	/* set flags			*/
#define VKI_SIOCGIFADDR		0x8915	/* get PA address		*/
#define VKI_SIOCSIFADDR		0x8916	/* set PA address		*/
#define VKI_SIOCGIFDSTADDR	0x8917	/* get remote PA address	*/
#define VKI_SIOCSIFDSTADDR	0x8918	/* set remote PA address	*/
#define VKI_SIOCGIFBRDADDR	0x8919	/* get broadcast PA address	*/
#define VKI_SIOCSIFBRDADDR	0x891a	/* set broadcast PA address	*/
#define VKI_SIOCGIFNETMASK	0x891b	/* get network PA mask		*/
#define VKI_SIOCSIFNETMASK	0x891c	/* set network PA mask		*/
#define VKI_SIOCGIFMETRIC	0x891d	/* get metric			*/
#define VKI_SIOCSIFMETRIC	0x891e	/* set metric			*/
#define VKI_SIOCGIFMTU		0x8921	/* get MTU size			*/
#define VKI_SIOCSIFMTU		0x8922	/* set MTU size			*/
#define	VKI_SIOCSIFHWADDR	0x8924	/* set hardware address 	*/
#define VKI_SIOCGIFHWADDR	0x8927	/* Get hardware address		*/
#define VKI_SIOCGIFINDEX	0x8933	/* name -> if_index mapping	*/

#define VKI_SIOCGIFTXQLEN	0x8942	/* Get the tx queue length	*/
#define VKI_SIOCSIFTXQLEN	0x8943	/* Set the tx queue length 	*/

#define VKI_SIOCETHTOOL		0x8946	/* Ethtool interface		*/

#define VKI_SIOCGMIIPHY		0x8947	/* Get address of MII PHY in use. */
#define VKI_SIOCGMIIREG		0x8948	/* Read MII PHY register.	*/
#define VKI_SIOCSMIIREG		0x8949	/* Write MII PHY register.	*/

#define VKI_SIOCDARP		0x8953	/* delete ARP table entry	*/
#define VKI_SIOCGARP		0x8954	/* get ARP table entry		*/
#define VKI_SIOCSARP		0x8955	/* set ARP table entry		*/

#define VKI_SIOCDRARP		0x8960	/* delete RARP table entry	*/
#define VKI_SIOCGRARP		0x8961	/* get RARP table entry		*/
#define VKI_SIOCSRARP		0x8962	/* set RARP table entry		*/

#define VKI_SIOCGIFMAP		0x8970	/* Get device parameters	*/
#define VKI_SIOCSIFMAP		0x8971	/* Set device parameters	*/

#define VKI_SIOCSHWTSTAMP	0x89B0	/* Set hardware time stamping */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/ppdev.h
//----------------------------------------------------------------------

#define VKI_PP_MAJOR	99

#define VKI_PP_IOCTL	'p'

/* Set mode for read/write (e.g. IEEE1284_MODE_EPP) */
#define VKI_PPSETMODE	_VKI_IOW(VKI_PP_IOCTL, 0x80, int)

/* Read status */
#define VKI_PPRSTATUS	_VKI_IOR(VKI_PP_IOCTL, 0x81, unsigned char)
//#define PPWSTATUS	OBSOLETE__IOW(PP_IOCTL, 0x82, unsigned char)

/* Read/write control */
#define VKI_PPRCONTROL	_VKI_IOR(VKI_PP_IOCTL, 0x83, unsigned char)
#define VKI_PPWCONTROL	_VKI_IOW(VKI_PP_IOCTL, 0x84, unsigned char)

struct vki_ppdev_frob_struct {
	unsigned char mask;
	unsigned char val;
};
#define VKI_PPFCONTROL      _VKI_IOW(VKI_PP_IOCTL, 0x8e, struct vki_ppdev_frob_struct)

/* Read/write data */
#define VKI_PPRDATA		_VKI_IOR(VKI_PP_IOCTL, 0x85, unsigned char)
#define VKI_PPWDATA		_VKI_IOW(VKI_PP_IOCTL, 0x86, unsigned char)

/* Claim the port to start using it */
#define VKI_PPCLAIM		_VKI_IO(VKI_PP_IOCTL, 0x8b)

/* Release the port when you aren't using it */
#define VKI_PPRELEASE	_VKI_IO(VKI_PP_IOCTL, 0x8c)

/* Yield the port (release it if another driver is waiting,
 * then reclaim) */
#define VKI_PPYIELD		_VKI_IO(VKI_PP_IOCTL, 0x8d)

/* Register device exclusively (must be before PPCLAIM). */
#define VKI_PPEXCL		_VKI_IO(VKI_PP_IOCTL, 0x8f)

/* Data line direction: non-zero for input mode. */
#define VKI_PPDATADIR	_VKI_IOW(VKI_PP_IOCTL, 0x90, int)

/* Negotiate a particular IEEE 1284 mode. */
#define VKI_PPNEGOT	_VKI_IOW(VKI_PP_IOCTL, 0x91, int)

/* Set control lines when an interrupt occurs. */
#define VKI_PPWCTLONIRQ	_VKI_IOW(VKI_PP_IOCTL, 0x92, unsigned char)

/* Clear (and return) interrupt count. */
#define VKI_PPCLRIRQ	_VKI_IOR(VKI_PP_IOCTL, 0x93, int)

/* Set the IEEE 1284 phase that we're in (e.g. IEEE1284_PH_FWD_IDLE) */
#define VKI_PPSETPHASE	_VKI_IOW(VKI_PP_IOCTL, 0x94, int)

/* Set and get port timeout (struct timeval's) */
#define VKI_PPGETTIME	_VKI_IOR(VKI_PP_IOCTL, 0x95, struct vki_timeval)
#define VKI_PPSETTIME	_VKI_IOW(VKI_PP_IOCTL, 0x96, struct vki_timeval)

#define VKI_PPGETMODES	_VKI_IOR(VKI_PP_IOCTL, 0x97, unsigned int)

#define VKI_PPGETMODE	_VKI_IOR(VKI_PP_IOCTL, 0x98, int)
#define VKI_PPGETPHASE	_VKI_IOR(VKI_PP_IOCTL, 0x99, int)

#define VKI_PPGETFLAGS	_VKI_IOR(VKI_PP_IOCTL, 0x9a, int)
#define VKI_PPSETFLAGS	_VKI_IOW(VKI_PP_IOCTL, 0x9b, int)

//----------------------------------------------------------------------
// From linux-5.2.5/include/uapi/linux/fs.h
//----------------------------------------------------------------------

#define VKI_BLKROSET   _VKI_IO(0x12,93)	/* set device read-only (0 = read-write) */
#define VKI_BLKROGET   _VKI_IO(0x12,94)	/* get read-only status (0 = read_write) */
#define VKI_BLKRRPART  _VKI_IO(0x12,95) /* re-read partition table */
#define VKI_BLKGETSIZE _VKI_IO(0x12,96) /* return device size /512 (long *arg) */
#define VKI_BLKFLSBUF  _VKI_IO(0x12,97) /* flush buffer cache */
#define VKI_BLKRASET   _VKI_IO(0x12,98)	/* set read ahead for block device */
#define VKI_BLKRAGET   _VKI_IO(0x12,99)	/* get current read ahead setting */
#define VKI_BLKFRASET  _VKI_IO(0x12,100)/* set filesystem (mm/filemap.c) read-ahead */
#define VKI_BLKFRAGET  _VKI_IO(0x12,101)/* get filesystem (mm/filemap.c) read-ahead */
#define VKI_BLKSECTSET _VKI_IO(0x12,102)/* set max sectors per request (ll_rw_blk.c) */
#define VKI_BLKSECTGET _VKI_IO(0x12,103)/* get max sectors per request (ll_rw_blk.c) */
#define VKI_BLKSSZGET  _VKI_IO(0x12,104)/* get block device sector size */
#define VKI_BLKBSZGET  _VKI_IOR(0x12,112,vki_size_t)
#define VKI_BLKBSZSET  _VKI_IOW(0x12,113,vki_size_t)
#define VKI_BLKGETSIZE64 _VKI_IOR(0x12,114,vki_size_t) /* return device size in bytes (u64 *arg) */
#define VKI_BLKDISCARD _VKI_IO(0x12,119)
#define VKI_BLKIOMIN _VKI_IO(0x12,120)
#define VKI_BLKIOOPT _VKI_IO(0x12,121)
#define VKI_BLKALIGNOFF _VKI_IO(0x12,122)
#define VKI_BLKPBSZGET _VKI_IO(0x12,123)
#define VKI_BLKDISCARDZEROES _VKI_IO(0x12,124)
#define VKI_BLKZEROOUT _VKI_IO(0x12,127)

#define VKI_FIBMAP	_VKI_IO(0x00,1)	/* bmap access */
#define VKI_FIGETBSZ    _VKI_IO(0x00,2)	/* get the block size used for bmap */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/scsi/sg.h
//----------------------------------------------------------------------

typedef struct vki_sg_io_hdr
{
    int interface_id;           /* [i] 'S' for SCSI generic (required) */
    int dxfer_direction;        /* [i] data transfer direction  */
    unsigned char cmd_len;      /* [i] SCSI command length ( <= 16 bytes) */
    unsigned char mx_sb_len;    /* [i] max length to write to sbp */
    unsigned short iovec_count; /* [i] 0 implies no scatter gather */
    unsigned int dxfer_len;     /* [i] byte count of data transfer */
    void __user *dxferp;	/* [i], [*io] points to data transfer memory
					      or scatter gather list */
    unsigned char __user *cmdp; /* [i], [*i] points to command to perform */
    void __user *sbp;		/* [i], [*o] points to sense_buffer memory */
    unsigned int timeout;       /* [i] MAX_UINT->no timeout (unit: millisec) */
    unsigned int flags;         /* [i] 0 -> default, see SG_FLAG... */
    int pack_id;                /* [i->o] unused internally (normally) */
    void __user * usr_ptr;      /* [i->o] unused internally */
    unsigned char status;       /* [o] scsi status */
    unsigned char masked_status;/* [o] shifted, masked scsi status */
    unsigned char msg_status;   /* [o] messaging level data (optional) */
    unsigned char sb_len_wr;    /* [o] byte count actually written to sbp */
    unsigned short host_status; /* [o] errors from host adapter */
    unsigned short driver_status;/* [o] errors from software driver */
    int resid;                  /* [o] dxfer_len - actual_transferred */
    unsigned int duration;      /* [o] time taken by cmd (unit: millisec) */
    unsigned int info;          /* [o] auxiliary information */
} vki_sg_io_hdr_t;  /* 64 bytes long (on i386) */

#define VKI_SG_DXFER_NONE -1        /* e.g. a SCSI Test Unit Ready command */
#define VKI_SG_DXFER_TO_DEV -2      /* e.g. a SCSI WRITE command */
#define VKI_SG_DXFER_FROM_DEV -3    /* e.g. a SCSI READ command */
#define VKI_SG_DXFER_TO_FROM_DEV -4 /* treated like SG_DXFER_FROM_DEV with the
				   additional property than during indirect
				   IO the user buffer is copied into the
				   kernel buffers before the transfer */

typedef struct vki_sg_scsi_id { /* used by SG_GET_SCSI_ID ioctl() */
    int host_no;        /* as in "scsi<n>" where 'n' is one of 0, 1, 2 etc */
    int channel;
    int scsi_id;        /* scsi id of target device */
    int lun;
    int scsi_type;      /* TYPE_... defined in scsi/scsi.h */
    short h_cmd_per_lun;/* host (adapter) maximum commands per lun */
    short d_queue_depth;/* device (or adapter) maximum queue length */
    int unused[2];      /* probably find a good use, set 0 for now */
} vki_sg_scsi_id_t; /* 32 bytes long on i386 */

#define VKI_SG_EMULATED_HOST 0x2203 /* true for emulated host adapter (ATAPI) */

#define VKI_SG_SET_RESERVED_SIZE 0x2275  /* request a new reserved buffer size */
#define VKI_SG_GET_RESERVED_SIZE 0x2272  /* actual size of reserved buffer */

#define VKI_SG_GET_SCSI_ID 0x2276   /* Yields fd's bus, chan, dev, lun + type */

#define VKI_SG_GET_SG_TABLESIZE 0x227F  /* 0 implies can't do scatter gather */

#define VKI_SG_GET_VERSION_NUM 0x2282 /* Example: version 2.1.34 yields 20134 */

#define VKI_SG_IO 0x2285   /* similar effect as write() followed by read() */

#define VKI_SG_SET_TIMEOUT 0x2201  /* unit: jiffies (10ms on i386) */
#define VKI_SG_GET_TIMEOUT 0x2202  /* yield timeout as _return_ value */

//#define SG_GET_COMMAND_Q 0x2270   /* Yields 0 (queuing off) or 1 (on) */
#define VKI_SG_SET_COMMAND_Q 0x2271   /* Change queuing state with 0 or 1 */

//----------------------------------------------------------------------
// From linux-2.6.34/include/scsi/scsi.h and scsi/scsi_ioctl.h
//----------------------------------------------------------------------

#define VKI_SCSI_IOCTL_DOORLOCK		0x5380 /* Lock the eject mechanism.  */
#define VKI_SCSI_IOCTL_DOORUNLOCK	0x5381 /* Unlock the mechanism.  */
#define VKI_SCSI_IOCTL_GET_IDLUN	0x5382
#define VKI_SCSI_IOCTL_GET_BUS_NUMBER	0x5386

struct vki_scsi_idlun {
	__vki_u32 dev_id;
	__vki_u32 host_unique_id;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/cdrom.h
//----------------------------------------------------------------------

#define VKI_CDROMPLAYMSF		0x5303 /* Play Audio MSF (struct cdrom_msf) */
#define VKI_CDROMREADTOCHDR		0x5305 /* Read TOC header 
                                	           (struct cdrom_tochdr) */
#define VKI_CDROMREADTOCENTRY		0x5306 /* Read TOC entry 
                                	           (struct cdrom_tocentry) */
#define VKI_CDROMSTOP			0x5307 /* Stop the cdrom drive */
#define VKI_CDROMSUBCHNL		0x530b /* Read subchannel data 
                                	           (struct cdrom_subchnl) */
#define VKI_CDROMREADMODE2		0x530c /* Read CDROM mode 2 data (2336 Bytes) 
                                	           (struct cdrom_read) */
#define VKI_CDROMREADMODE1		0x530d /* Read CDROM mode 1 data (2048 Bytes)
                                                   (struct cdrom_read) */
#define VKI_CDROMREADAUDIO		0x530e /* (struct cdrom_read_audio) */
#define VKI_CDROMMULTISESSION		0x5310 /* Obtain the start-of-last-session 
                                	           address of multi session disks 
                                	           (struct cdrom_multisession) */
#define VKI_CDROM_GET_MCN		0x5311 /* Obtain the "Universal Product Code" 
                                	           if available (struct cdrom_mcn) */
#define VKI_CDROMVOLREAD		0x5313 /* Get the drive's volume setting
                                	          (struct cdrom_volctrl) */
#define VKI_CDROMREADRAW		0x5314	/* read data in raw mode (2352 Bytes)
                                	           (struct cdrom_read) */
#define VKI_CDROM_CLEAR_OPTIONS		0x5321  /* Clear behavior options */
#define VKI_CDROM_DRIVE_STATUS		0x5326  /* Get tray position, etc. */
#define VKI_CDROM_DISC_STATUS		0x5327	/* get CD type information */
#define VKI_CDROM_GET_CAPABILITY	0x5331	/* get capabilities */

#define VKI_DVD_READ_STRUCT		0x5390  /* read structure */
#define VKI_CDROM_SEND_PACKET		0x5393	/* send a packet to the drive */

struct vki_cdrom_msf0		
{
	__vki_u8	minute;
	__vki_u8	second;
	__vki_u8	frame;
};

union vki_cdrom_addr		
{
	struct vki_cdrom_msf0	msf;
	int			lba;
};

struct vki_cdrom_msf 
{
	__vki_u8	cdmsf_min0;	/* start minute */
	__vki_u8	cdmsf_sec0;	/* start second */
	__vki_u8	cdmsf_frame0;	/* start frame */
	__vki_u8	cdmsf_min1;	/* end minute */
	__vki_u8	cdmsf_sec1;	/* end second */
	__vki_u8	cdmsf_frame1;	/* end frame */
};

struct vki_cdrom_tochdr 	
{
	__vki_u8	cdth_trk0;	/* start track */
	__vki_u8	cdth_trk1;	/* end track */
};

struct vki_cdrom_volctrl
{
	__vki_u8	channel0;
	__vki_u8	channel1;
	__vki_u8	channel2;
	__vki_u8	channel3;
};

struct vki_cdrom_subchnl 
{
	__vki_u8	cdsc_format;
	__vki_u8	cdsc_audiostatus;
	__vki_u8	cdsc_adr:	4;
	__vki_u8	cdsc_ctrl:	4;
	__vki_u8	cdsc_trk;
	__vki_u8	cdsc_ind;
	union vki_cdrom_addr cdsc_absaddr;
	union vki_cdrom_addr cdsc_reladdr;
};

struct vki_cdrom_tocentry 
{
	__vki_u8	cdte_track;
	__vki_u8	cdte_adr	:4;
	__vki_u8	cdte_ctrl	:4;
	__vki_u8	cdte_format;
	union vki_cdrom_addr cdte_addr;
	__vki_u8	cdte_datamode;
};

struct vki_cdrom_read      
{
	int	cdread_lba;
	char 	*cdread_bufaddr;
	int	cdread_buflen;
};

struct vki_cdrom_read_audio
{
	union vki_cdrom_addr addr; /* frame address */
	__vki_u8 addr_format;      /* CDROM_LBA or CDROM_MSF */
	int nframes;           /* number of 2352-byte-frames to read at once */
	__vki_u8 __user *buf;      /* frame buffer (size: nframes*2352 bytes) */
};

struct vki_cdrom_multisession
{
	union vki_cdrom_addr addr; /* frame address: start-of-last-session 
	                           (not the new "frame 16"!).  Only valid
	                           if the "xa_flag" is true. */
	__vki_u8 xa_flag;        /* 1: "is XA disk" */
	__vki_u8 addr_format;    /* CDROM_LBA or CDROM_MSF */
};

struct vki_cdrom_mcn 
{
  __vki_u8 medium_catalog_number[14]; /* 13 ASCII digits, null-terminated */
};

#define VKI_CDROM_PACKET_SIZE	12

struct vki_cdrom_generic_command
{
	unsigned char 		cmd[VKI_CDROM_PACKET_SIZE];
	unsigned char		__user *buffer;
	unsigned int 		buflen;
	int			stat;
        // [[replace with void* to reduce inclusion amounts]]
	void/*struct vki_request_sense	*/__user *sense;
	unsigned char		data_direction;
	int			quiet;
	int			timeout;
	void			__user *reserved[1];	/* unused, actually */
};

#define VKI_CD_SYNC_SIZE         12 /* 12 sync bytes per raw data frame */
#define VKI_CD_HEAD_SIZE          4 /* header (address) bytes per raw data frame */
#define VKI_CD_FRAMESIZE_RAW   2352 /* bytes per frame, "raw" mode */
#define VKI_CD_FRAMESIZE_RAW0 (VKI_CD_FRAMESIZE_RAW-VKI_CD_SYNC_SIZE-VKI_CD_HEAD_SIZE) /*2336*/
#define VKI_CD_FRAMESIZE_RAW1  2048 /* bytes per frame, mode 1*/

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/soundcard.h
//----------------------------------------------------------------------

#ifndef _VKI_SIOWR
#if defined(_VKI_IOWR) && (defined(_AIX) || (!defined(sun) && !defined(sparc) && !defined(__sparc__) && !defined(__INCioctlh) && !defined(__Lynx__)))
/* Use already defined ioctl defines if they exist (except with Sun or Sparc) */
#define	_VKI_SIO		_VKI_IO
#define	_VKI_SIOR		_VKI_IOR
#define	_VKI_SIOW		_VKI_IOW
#define	_VKI_SIOWR		_VKI_IOWR
#else
// [[Valgrind: Install this case if/when necessary]
#error Valgrind: Cannot handle sparc/sun case yet...
#  endif /* _IOWR */
#endif  /* !_VKI_SIOWR */

#define VKI_SNDCTL_SEQ_CTRLRATE		_VKI_SIOWR('Q', 3, int)	/* Set/get timer resolution (HZ) */
#define VKI_SNDCTL_SEQ_GETOUTCOUNT	_VKI_SIOR ('Q', 4, int)
#define VKI_SNDCTL_SEQ_GETINCOUNT	_VKI_SIOR ('Q', 5, int)
#define VKI_SNDCTL_SEQ_PERCMODE		_VKI_SIOW ('Q', 6, int)
#define VKI_SNDCTL_SEQ_TESTMIDI		_VKI_SIOW ('Q', 8, int)
#define VKI_SNDCTL_SEQ_RESETSAMPLES	_VKI_SIOW ('Q', 9, int)
#define VKI_SNDCTL_SEQ_NRSYNTHS		_VKI_SIOR ('Q',10, int)
#define VKI_SNDCTL_SEQ_NRMIDIS		_VKI_SIOR ('Q',11, int)
#define VKI_SNDCTL_SEQ_GETTIME		_VKI_SIOR ('Q',19, int)

#define VKI_SNDCTL_TMR_TIMEBASE		_VKI_SIOWR('T', 1, int)
#define VKI_SNDCTL_TMR_TEMPO		_VKI_SIOWR('T', 5, int)
#define VKI_SNDCTL_TMR_SOURCE		_VKI_SIOWR('T', 6, int)

#define VKI_SNDCTL_MIDI_PRETIME		_VKI_SIOWR('m', 0, int)
#define VKI_SNDCTL_MIDI_MPUMODE		_VKI_SIOWR('m', 1, int)

#define VKI_SNDCTL_DSP_RESET		_VKI_SIO  ('P', 0)
#define VKI_SNDCTL_DSP_SYNC		_VKI_SIO  ('P', 1)
#define VKI_SNDCTL_DSP_SPEED		_VKI_SIOWR('P', 2, int)
#define VKI_SNDCTL_DSP_STEREO		_VKI_SIOWR('P', 3, int)
#define VKI_SNDCTL_DSP_GETBLKSIZE	_VKI_SIOWR('P', 4, int)
#define VKI_SNDCTL_DSP_CHANNELS		_VKI_SIOWR('P', 6, int)
#define VKI_SOUND_PCM_WRITE_FILTER	_VKI_SIOWR('P', 7, int)
#define VKI_SNDCTL_DSP_POST		_VKI_SIO  ('P', 8)
#define VKI_SNDCTL_DSP_SUBDIVIDE	_VKI_SIOWR('P', 9, int)
#define VKI_SNDCTL_DSP_SETFRAGMENT	_VKI_SIOWR('P',10, int)

#define VKI_SNDCTL_DSP_GETFMTS		_VKI_SIOR ('P',11, int) /* Returns a mask */
#define VKI_SNDCTL_DSP_SETFMT		_VKI_SIOWR('P', 5, int) /* Selects ONE fmt */

typedef struct vki_audio_buf_info {
			int fragments;	/* # of available fragments (partially usend ones not counted) */
			int fragstotal;	/* Total # of fragments allocated */
			int fragsize;	/* Size of a fragment in bytes */

			int bytes;	/* Available space in bytes (includes partially used fragments) */
			/* Note! 'bytes' could be more than fragments*fragsize */
		} vki_audio_buf_info;

#define VKI_SNDCTL_DSP_GETOSPACE	_VKI_SIOR ('P',12, vki_audio_buf_info)
#define VKI_SNDCTL_DSP_GETISPACE	_VKI_SIOR ('P',13, vki_audio_buf_info)
#define VKI_SNDCTL_DSP_NONBLOCK		_VKI_SIO  ('P',14)
#define VKI_SNDCTL_DSP_GETCAPS		_VKI_SIOR ('P',15, int)

#define VKI_SNDCTL_DSP_GETTRIGGER	_VKI_SIOR ('P',16, int)
#define VKI_SNDCTL_DSP_SETTRIGGER	_VKI_SIOW ('P',16, int)

#define VKI_SNDCTL_DSP_SETSYNCRO	_VKI_SIO  ('P', 21)
#define VKI_SNDCTL_DSP_SETDUPLEX	_VKI_SIO  ('P', 22)
#define VKI_SNDCTL_DSP_GETODELAY	_VKI_SIOR ('P', 23, int)

#define VKI_SNDCTL_DSP_GETCHANNELMASK	_VKI_SIOWR('P', 64, int)
#define VKI_SNDCTL_DSP_BIND_CHANNEL	_VKI_SIOWR('P', 65, int)

#define VKI_SNDCTL_DSP_SETSPDIF		_VKI_SIOW ('P', 66, int)
#define VKI_SNDCTL_DSP_GETSPDIF		_VKI_SIOR ('P', 67, int)

#define VKI_SOUND_PCM_READ_RATE		_VKI_SIOR ('P', 2, int)
#define VKI_SOUND_PCM_READ_CHANNELS	_VKI_SIOR ('P', 6, int)
#define VKI_SOUND_PCM_READ_BITS		_VKI_SIOR ('P', 5, int)
#define VKI_SOUND_PCM_READ_FILTER	_VKI_SIOR ('P', 7, int)


//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/hdreg.h
//----------------------------------------------------------------------

struct vki_hd_geometry {
      unsigned char heads;
      unsigned char sectors;
      unsigned short cylinders;
      unsigned long start;
};

#define VKI_HDIO_GETGEO		0x0301	/* get device geometry */
#define VKI_HDIO_GET_DMA	0x030b	/* get use-dma flag */
#define VKI_HDIO_GET_IDENTITY	0x030d	/* get IDE identification info */

// [[Nb: done like this because the original type is a huge struct that will
//   always be the same size.]]
#define VKI_SIZEOF_STRUCT_HD_DRIVEID   512

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/linux/fb.h
//----------------------------------------------------------------------

#define VKI_FBIOGET_VSCREENINFO	0x4600
#define VKI_FBIOPUT_VSCREENINFO	0x4601
#define VKI_FBIOGET_FSCREENINFO	0x4602
#define VKI_FBIOPAN_DISPLAY	0x4606

struct vki_fb_fix_screeninfo {
	char id[16];			/* identification string eg "TT Builtin" */
	unsigned long smem_start;	/* Start of frame buffer mem */
					/* (physical address) */
	__vki_u32 smem_len;			/* Length of frame buffer mem */
	__vki_u32 type;			/* see FB_TYPE_*		*/
	__vki_u32 type_aux;		/* Interleave for interleaved Planes */
	__vki_u32 visual;		/* see FB_VISUAL_*		*/ 
	__vki_u16 xpanstep;		/* zero if no hardware panning  */
	__vki_u16 ypanstep;		/* zero if no hardware panning  */
	__vki_u16 ywrapstep;		/* zero if no hardware ywrap    */
	__vki_u32 line_length;		/* length of a line in bytes    */
	unsigned long mmio_start;	/* Start of Memory Mapped I/O   */
					/* (physical address) */
	__vki_u32 mmio_len;		/* Length of Memory Mapped I/O  */
	__vki_u32 accel;		/* Indicate to driver which	*/
					/*  specific chip/card we have	*/
	__vki_u16 reserved[3];		/* Reserved for future compatibility */
};

struct vki_fb_bitfield {
	__vki_u32 offset;		/* beginning of bitfield	*/
	__vki_u32 length;		/* length of bitfield		*/
	__vki_u32 msb_right;		/* != 0 : Most significant bit is */ 
					/* right */ 
};

struct vki_fb_var_screeninfo {
	__vki_u32 xres;			/* visible resolution		*/
	__vki_u32 yres;
	__vki_u32 xres_virtual;		/* virtual resolution		*/
	__vki_u32 yres_virtual;
	__vki_u32 xoffset;		/* offset from virtual to visible */
	__vki_u32 yoffset;		/* resolution			*/

	__vki_u32 bits_per_pixel;	/* guess what			*/
	__vki_u32 grayscale;		/* != 0 Graylevels instead of colors */

	struct vki_fb_bitfield red;	/* bitfield in fb mem if true color, */
	struct vki_fb_bitfield green;	/* else only length is significant */
	struct vki_fb_bitfield blue;
	struct vki_fb_bitfield transp;	/* transparency			*/	

	__vki_u32 nonstd;		/* != 0 Non standard pixel format */

	__vki_u32 activate;		/* see FB_ACTIVATE_*		*/

	__vki_u32 height;		/* height of picture in mm    */
	__vki_u32 width;		/* width of picture in mm     */

	__vki_u32 accel_flags;		/* (OBSOLETE) see fb_info.flags */

	/* Timing: All values in pixclocks, except pixclock (of course) */
	__vki_u32 pixclock;		/* pixel clock in ps (pico seconds) */
	__vki_u32 left_margin;		/* time from sync to picture	*/
	__vki_u32 right_margin;		/* time from picture to sync	*/
	__vki_u32 upper_margin;		/* time from sync to picture	*/
	__vki_u32 lower_margin;
	__vki_u32 hsync_len;		/* length of horizontal sync	*/
	__vki_u32 vsync_len;		/* length of vertical sync	*/
	__vki_u32 sync;			/* see FB_SYNC_*		*/
	__vki_u32 vmode;		/* see FB_VMODE_*		*/
	__vki_u32 rotate;		/* angle we rotate counter clockwise */
	__vki_u32 reserved[5];		/* Reserved for future compatibility */
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/linux/kd.h
//----------------------------------------------------------------------

#define VKI_GIO_FONT       0x4B60  /* gets font in expanded form */
#define VKI_PIO_FONT       0x4B61  /* use font in expanded form */

#define VKI_GIO_FONTX      0x4B6B  /* get font using struct consolefontdesc */
#define VKI_PIO_FONTX      0x4B6C  /* set font using struct consolefontdesc */
struct vki_consolefontdesc {
	unsigned short charcount;	/* characters in font (256 or 512) */
	unsigned short charheight;	/* scan lines per character (1-32) */
	char __user *chardata;		/* font data in expanded form */
};

#define VKI_PIO_FONTRESET  0x4B6D  /* reset to default font */

#define VKI_GIO_CMAP       0x4B70  /* gets colour palette on VGA+ */
#define VKI_PIO_CMAP       0x4B71  /* sets colour palette on VGA+ */

#define VKI_KIOCSOUND      0x4B2F  /* start sound generation (0 for off) */
#define VKI_KDMKTONE       0x4B30  /* generate tone */

#define VKI_KDGETLED       0x4B31  /* return current led state */
#define VKI_KDSETLED       0x4B32  /* set led state [lights, not flags] */

#define VKI_KDGKBTYPE      0x4B33  /* get keyboard type */

#define VKI_KDADDIO        0x4B34  /* add i/o port as valid */
#define VKI_KDDELIO        0x4B35  /* del i/o port as valid */
#define VKI_KDENABIO       0x4B36  /* enable i/o to video board */
#define VKI_KDDISABIO      0x4B37  /* disable i/o to video board */

#define VKI_KDSETMODE      0x4B3A  /* set text/graphics mode */
#define VKI_KDGETMODE      0x4B3B  /* get current mode */

#define VKI_KDMAPDISP      0x4B3C  /* map display into address space */
#define VKI_KDUNMAPDISP    0x4B3D  /* unmap display from address space */

#define		VKI_E_TABSZ		256
#define VKI_GIO_SCRNMAP    0x4B40  /* get screen mapping from kernel */
#define VKI_PIO_SCRNMAP	   0x4B41  /* put screen mapping table in kernel */
#define VKI_GIO_UNISCRNMAP 0x4B69  /* get full Unicode screen mapping */
#define VKI_PIO_UNISCRNMAP 0x4B6A  /* set full Unicode screen mapping */

#define VKI_GIO_UNIMAP     0x4B66  /* get unicode-to-font mapping from kernel */
struct vki_unipair {
	unsigned short unicode;
	unsigned short fontpos;
};
struct vki_unimapdesc {
	unsigned short entry_ct;
	struct vki_unipair __user *entries;
};
#define VKI_PIO_UNIMAP     0x4B67  /* put unicode-to-font mapping in kernel */
#define VKI_PIO_UNIMAPCLR  0x4B68  /* clear table, possibly advise hash algorithm */
struct vki_unimapinit {
	unsigned short advised_hashsize;  /* 0 if no opinion */
	unsigned short advised_hashstep;  /* 0 if no opinion */
	unsigned short advised_hashlevel; /* 0 if no opinion */
};

#define VKI_KDGKBMODE      0x4B44  /* gets current keyboard mode */
#define VKI_KDSKBMODE      0x4B45  /* sets current keyboard mode */

#define VKI_KDGKBMETA      0x4B62  /* gets meta key handling mode */
#define VKI_KDSKBMETA      0x4B63  /* sets meta key handling mode */

#define VKI_KDGKBLED       0x4B64  /* get led flags (not lights) */
#define VKI_KDSKBLED       0x4B65  /* set led flags (not lights) */

struct vki_kbentry {
	unsigned char kb_table;
	unsigned char kb_index;
	unsigned short kb_value;
};
#define VKI_KDGKBENT       0x4B46  /* gets one entry in translation table */
#define VKI_KDSKBENT       0x4B47  /* sets one entry in translation table */

struct vki_kbsentry {
	unsigned char kb_func;
	unsigned char kb_string[512];
};
#define VKI_KDGKBSENT      0x4B48  /* gets one function key string entry */
#define VKI_KDSKBSENT      0x4B49  /* sets one function key string entry */

struct vki_kbdiacr {
        unsigned char diacr, base, result;
};
struct vki_kbdiacrs {
        unsigned int kb_cnt;    /* number of entries in following array */
	struct vki_kbdiacr kbdiacr[256];    /* MAX_DIACR from keyboard.h */
};
#define VKI_KDGKBDIACR     0x4B4A  /* read kernel accent table */
#define VKI_KDSKBDIACR     0x4B4B  /* write kernel accent table */

struct vki_kbkeycode {
	unsigned int scancode, keycode;
};
#define VKI_KDGETKEYCODE   0x4B4C  /* read kernel keycode table entry */
#define VKI_KDSETKEYCODE   0x4B4D  /* write kernel keycode table entry */

#define VKI_KDSIGACCEPT    0x4B4E  /* accept kbd generated signals */

struct vki_kbd_repeat {
	int delay;	/* in msec; <= 0: don't change */
	int period;	/* in msec; <= 0: don't change */
			/* earlier this field was misnamed "rate" */
};
#define VKI_KDKBDREP       0x4B52  /* set keyboard delay/repeat rate;
                                    * actually used values are returned */

#define VKI_KDFONTOP       0x4B72  /* font operations */

struct vki_console_font_op {
	unsigned int op;	/* operation code KD_FONT_OP_* */
	unsigned int flags;	/* KD_FONT_FLAG_* */
	unsigned int width, height;	/* font size */
	unsigned int charcount;
	unsigned char __user *data;	/* font data with height fixed to 32 */
};

#define VKI_KD_FONT_OP_SET		0	/* Set font */
#define VKI_KD_FONT_OP_GET		1	/* Get font */
#define VKI_KD_FONT_OP_SET_DEFAULT	2	/* Set font to default, data points to name / NULL */
#define VKI_KD_FONT_OP_COPY		3	/* Copy from another console */

//----------------------------------------------------------------------
// From linux-2.6.9/include/linux/kb.h
//----------------------------------------------------------------------

typedef __vki_kernel_uid32_t vki_qid_t; /* Type in which we store ids in memory */

//----------------------------------------------------------------------
// From linux-2.6.20.1/include/linux/ptrace.h
//----------------------------------------------------------------------

#define VKI_PTRACE_TRACEME         0
#define VKI_PTRACE_PEEKTEXT	   1
#define VKI_PTRACE_PEEKDATA	   2
#define VKI_PTRACE_PEEKUSR	   3
#define VKI_PTRACE_POKEUSR	   6

#define VKI_PTRACE_DETACH         17

#define VKI_PTRACE_GETEVENTMSG	0x4201
#define VKI_PTRACE_GETSIGINFO	0x4202
#define VKI_PTRACE_SETSIGINFO	0x4203
#define VKI_PTRACE_GETREGSET	0x4204
#define VKI_PTRACE_SETREGSET	0x4205

#define VKI_PT_PTRACED 0x00000001

//----------------------------------------------------------------------
// From linux-2.6.14/include/sound/asound.h
//----------------------------------------------------------------------

enum {
	VKI_SNDRV_PCM_IOCTL_HW_FREE = _VKI_IO('A', 0x12),
	VKI_SNDRV_PCM_IOCTL_HWSYNC = _VKI_IO('A', 0x22),
	VKI_SNDRV_PCM_IOCTL_PREPARE = _VKI_IO('A', 0x40),
	VKI_SNDRV_PCM_IOCTL_RESET = _VKI_IO('A', 0x41),
	VKI_SNDRV_PCM_IOCTL_START = _VKI_IO('A', 0x42),
	VKI_SNDRV_PCM_IOCTL_DROP = _VKI_IO('A', 0x43),
	VKI_SNDRV_PCM_IOCTL_DRAIN = _VKI_IO('A', 0x44),
	VKI_SNDRV_PCM_IOCTL_PAUSE = _VKI_IOW('A', 0x45, int),
	VKI_SNDRV_PCM_IOCTL_RESUME = _VKI_IO('A', 0x47),
	VKI_SNDRV_PCM_IOCTL_XRUN = _VKI_IO('A', 0x48),
	VKI_SNDRV_PCM_IOCTL_LINK = _VKI_IOW('A', 0x60, int),
	VKI_SNDRV_PCM_IOCTL_UNLINK = _VKI_IO('A', 0x61),
};

enum {
	VKI_SNDRV_TIMER_IOCTL_START = _VKI_IO('T', 0xa0),
	VKI_SNDRV_TIMER_IOCTL_STOP = _VKI_IO('T', 0xa1),
	VKI_SNDRV_TIMER_IOCTL_CONTINUE = _VKI_IO('T', 0xa2),
	VKI_SNDRV_TIMER_IOCTL_PAUSE = _VKI_IO('T', 0xa3),
};

struct vki_snd_ctl_card_info {
	int card;			/* card number */
	int pad;			/* reserved for future (was type) */
	unsigned char id[16];		/* ID of card (user selectable) */
	unsigned char driver[16];	/* Driver name */
	unsigned char name[32];		/* Short name of soundcard */
	unsigned char longname[80];	/* name + info text about soundcard */
	unsigned char reserved_[16];	/* reserved for future (was ID of mixer) */
	unsigned char mixername[80];	/* visual mixer identification */
	unsigned char components[128];	/* card components / fine identification, delimited with one space (AC97 etc..) */
};

typedef int vki_snd_ctl_elem_iface_t;
#define	VKI_SNDRV_CTL_ELEM_IFACE_CARD		((vki_snd_ctl_elem_iface_t) 0) /* global control */
#define	VKI_SNDRV_CTL_ELEM_IFACE_HWDEP		((vki_snd_ctl_elem_iface_t) 1) /* hardware dependent device */
#define	VKI_SNDRV_CTL_ELEM_IFACE_MIXER		((vki_snd_ctl_elem_iface_t) 2) /* virtual mixer device */
#define	VKI_SNDRV_CTL_ELEM_IFACE_PCM		((vki_snd_ctl_elem_iface_t) 3) /* PCM device */
#define	VKI_SNDRV_CTL_ELEM_IFACE_RAWMIDI	((vki_snd_ctl_elem_iface_t) 4) /* RawMidi device */
#define	VKI_SNDRV_CTL_ELEM_IFACE_TIMER		((vki_snd_ctl_elem_iface_t) 5) /* timer device */
#define	VKI_SNDRV_CTL_ELEM_IFACE_SEQUENCER	((vki_snd_ctl_elem_iface_t) 6) /* sequencer client */
#define	VKI_SNDRV_CTL_ELEM_IFACE_LAST		VKI_SNDRV_CTL_ELEM_IFACE_SEQUENCER

struct vki_snd_ctl_elem_id {
	unsigned int numid;		/* numeric identifier, zero = invalid */
	vki_snd_ctl_elem_iface_t iface;	/* interface identifier */
	unsigned int device;		/* device/client number */
	unsigned int subdevice;		/* subdevice (substream) number */
	unsigned char name[44];		/* ASCII name of item */
	unsigned int index;		/* index of item */
};

struct vki_snd_ctl_elem_list {
	unsigned int offset;		/* W: first element ID to get */
	unsigned int space;		/* W: count of element IDs to get */
	unsigned int used;		/* R: count of element IDs set */
	unsigned int count;		/* R: count of all elements */
	struct vki_snd_ctl_elem_id __user *pids; /* R: IDs */
	unsigned char reserved[50];
};

struct vki_snd_ctl_tlv {
    unsigned int numid;	/* control element numeric identification */
    unsigned int length;	/* in bytes aligned to 4 */
    unsigned int tlv[0];	/* first TLV */
};

#define VKI_SNDRV_CTL_IOCTL_PVERSION	_VKI_IOR('U', 0x00, int)
#define VKI_SNDRV_CTL_IOCTL_CARD_INFO	_VKI_IOR('U', 0x01, struct vki_snd_ctl_card_info)
#define VKI_SNDRV_CTL_IOCTL_ELEM_LIST	_VKI_IOWR('U', 0x10, struct vki_snd_ctl_elem_list)
#define VKI_SNDRV_CTL_IOCTL_TLV_READ	_VKI_IOWR('U', 0x1a, struct vki_snd_ctl_tlv)
#define VKI_SNDRV_CTL_IOCTL_TLV_WRITE	_VKI_IOWR('U', 0x1b, struct vki_snd_ctl_tlv)
#define VKI_SNDRV_CTL_IOCTL_TLV_COMMAND	_VKI_IOWR('U', 0x1c, struct vki_snd_ctl_tlv)

//----------------------------------------------------------------------
// From linux-2.6.15.4/include/linux/serial.h
//----------------------------------------------------------------------

struct vki_serial_icounter_struct {
	int cts, dsr, rng, dcd;
	int rx, tx;
	int frame, overrun, parity, brk;
	int buf_overrun;
	int reserved[9];
};

//----------------------------------------------------------------------
// From linux-2.6.16/include/linux/vt.h
//----------------------------------------------------------------------

#define VKI_VT_OPENQRY	0x5600	/* find available vt */

struct vki_vt_mode {
	char mode;		/* vt mode */
	char waitv;		/* if set, hang on writes if not active */
	short relsig;		/* signal to raise on release req */
	short acqsig;		/* signal to raise on acquisition */
	short frsig;		/* unused (set to 0) */
};
#define VKI_VT_GETMODE	0x5601	/* get mode of active vt */
#define VKI_VT_SETMODE	0x5602	/* set mode of active vt */

struct vki_vt_stat {
	unsigned short v_active;	/* active vt */
	unsigned short v_signal;	/* signal to send */
	unsigned short v_state;		/* vt bitmask */
};
#define VKI_VT_GETSTATE	0x5603	/* get global vt state info */
#define VKI_VT_SENDSIG	0x5604	/* signal to send to bitmask of vts */

#define VKI_VT_RELDISP	0x5605	/* release display */

#define VKI_VT_ACTIVATE	0x5606	/* make vt active */
#define VKI_VT_WAITACTIVE	0x5607	/* wait for vt active */
#define VKI_VT_DISALLOCATE	0x5608  /* free memory associated to vt */

struct vki_vt_sizes {
	unsigned short v_rows;		/* number of rows */
	unsigned short v_cols;		/* number of columns */
	unsigned short v_scrollsize;	/* number of lines of scrollback */
};
#define VKI_VT_RESIZE	0x5609	/* set kernel's idea of screensize */

struct vki_vt_consize {
	unsigned short v_rows;	/* number of rows */
	unsigned short v_cols;	/* number of columns */
	unsigned short v_vlin;	/* number of pixel rows on screen */
	unsigned short v_clin;	/* number of pixel rows per character */
	unsigned short v_vcol;	/* number of pixel columns on screen */
	unsigned short v_ccol;	/* number of pixel columns per character */
};
#define VKI_VT_RESIZEX      0x560A  /* set kernel's idea of screensize + more */
#define VKI_VT_LOCKSWITCH   0x560B  /* disallow vt switching */
#define VKI_VT_UNLOCKSWITCH 0x560C  /* allow vt switching */

//----------------------------------------------------------------------
// From linux-2.6.19/include/linux/prctl.h
//----------------------------------------------------------------------

#define VKI_PR_SET_PDEATHSIG  1  /* Second arg is a signal */
#define VKI_PR_GET_PDEATHSIG  2  /* Second arg is a ptr to return the signal */

#define VKI_PR_GET_DUMPABLE   3
#define VKI_PR_SET_DUMPABLE   4

#define VKI_PR_GET_UNALIGN	  5
#define VKI_PR_SET_UNALIGN	  6
# define VKI_PR_UNALIGN_NOPRINT	1	/* silently fix up unaligned user accesses */
# define VKI_PR_UNALIGN_SIGBUS	2	/* generate SIGBUS on unaligned user access */

#define VKI_PR_GET_KEEPCAPS   7
#define VKI_PR_SET_KEEPCAPS   8

#define VKI_PR_GET_FPEMU  9
#define VKI_PR_SET_FPEMU 10
# define VKI_PR_FPEMU_NOPRINT	1	/* silently emulate fp operations accesses */
# define VKI_PR_FPEMU_SIGFPE	2	/* don't emulate fp operations, send SIGFPE instead */

#define VKI_PR_GET_FPEXC	11
#define VKI_PR_SET_FPEXC	12
# define VKI_PR_FP_EXC_SW_ENABLE	0x80	/* Use FPEXC for FP exception enables */
# define VKI_PR_FP_EXC_DIV		0x010000	/* floating point divide by zero */
# define VKI_PR_FP_EXC_OVF		0x020000	/* floating point overflow */
# define VKI_PR_FP_EXC_UND		0x040000	/* floating point underflow */
# define VKI_PR_FP_EXC_RES		0x080000	/* floating point inexact result */
# define VKI_PR_FP_EXC_INV		0x100000	/* floating point invalid operation */
# define VKI_PR_FP_EXC_DISABLED	0	/* FP exceptions disabled */
# define VKI_PR_FP_EXC_NONRECOV	1	/* async non-recoverable exc. mode */
# define VKI_PR_FP_EXC_ASYNC	2	/* async recoverable exception mode */
# define VKI_PR_FP_EXC_PRECISE	3	/* precise exception mode */

#define VKI_PR_GET_TIMING   13
#define VKI_PR_SET_TIMING   14
# define VKI_PR_TIMING_STATISTICAL  0       /* Normal, traditional,
                                                   statistical process timing */
# define VKI_PR_TIMING_TIMESTAMP    1       /* Accurate timestamp based
                                                   process timing */

#define VKI_PR_SET_NAME    15		/* Set process name */
#define VKI_PR_GET_NAME    16		/* Get process name */

#define VKI_PR_GET_ENDIAN	19
#define VKI_PR_SET_ENDIAN	20
# define VKI_PR_ENDIAN_BIG		0
# define VKI_PR_ENDIAN_LITTLE	1	/* True little endian mode */
# define VKI_PR_ENDIAN_PPC_LITTLE	2	/* "PowerPC" pseudo little endian */

#define VKI_PR_SET_SECCOMP 22

#define VKI_PR_CAPBSET_READ 23
#define VKI_PR_CAPBSET_DROP 24

#define VKI_PR_GET_TSC 25
#define VKI_PR_SET_TSC 26

#define VKI_PR_GET_SECUREBITS 27
#define VKI_PR_SET_SECUREBITS 28

#define VKI_PR_SET_TIMERSLACK 29
#define VKI_PR_GET_TIMERSLACK 30

#define VKI_PR_TASK_PERF_EVENTS_DISABLE		31
#define VKI_PR_TASK_PERF_EVENTS_ENABLE		32

#define VKI_PR_MCE_KILL	33
#define VKI_PR_MCE_KILL_GET 34

#define VKI_PR_SET_PTRACER 0x59616d61

#define VKI_PR_SET_CHILD_SUBREAPER	36
#define VKI_PR_GET_CHILD_SUBREAPER	37

#define VKI_PR_SET_NO_NEW_PRIVS	38
#define VKI_PR_GET_NO_NEW_PRIVS	39

#define VKI_PR_GET_TID_ADDRESS	40

#define VKI_PR_SET_THP_DISABLE	41
#define VKI_PR_GET_THP_DISABLE	42

#define VKI_PR_MPX_ENABLE_MANAGEMENT  43
#define VKI_PR_MPX_DISABLE_MANAGEMENT 44

#define VKI_PR_SET_FP_MODE		45
#define VKI_PR_GET_FP_MODE		46

#define VKI_PR_CAP_AMBIENT		47

#define VKI_PR_SVE_SET_VL		50
#define VKI_PR_SVE_GET_VL		51
#define VKI_PR_GET_SPECULATION_CTRL	52
#define VKI_PR_SET_SPECULATION_CTRL	53
#define VKI_PR_PAC_RESET_KEYS		54
#define VKI_PR_SET_TAGGED_ADDR_CTRL	55
#define VKI_PR_GET_TAGGED_ADDR_CTRL	56

//----------------------------------------------------------------------
// From linux-2.6.19/include/linux/usbdevice_fs.h
//----------------------------------------------------------------------

struct vki_usbdevfs_ctrltransfer {
	__vki_u8 bRequestType;
	__vki_u8 bRequest;
	__vki_u16 wValue;
	__vki_u16 wIndex;
	__vki_u16 wLength;
	__vki_u32 timeout;  /* in milliseconds */
 	void __user *data;
};

struct vki_usbdevfs_bulktransfer {
	unsigned int ep;
	unsigned int len;
	unsigned int timeout; /* in milliseconds */
	void __user *data;
};

#define VKI_USBDEVFS_MAXDRIVERNAME 255

struct vki_usbdevfs_getdriver {
	unsigned int interface;
	char driver[VKI_USBDEVFS_MAXDRIVERNAME + 1];
};

struct vki_usbdevfs_connectinfo {
	unsigned int devnum;
	unsigned char slow;
};

struct vki_usbdevfs_iso_packet_desc {
	unsigned int length;
	unsigned int actual_length;
	unsigned int status;
};

struct vki_usbdevfs_urb {
	unsigned char type;
	unsigned char endpoint;
	int status;
	unsigned int flags;
	void __user *buffer;
	int buffer_length;
	int actual_length;
	int start_frame;
	int number_of_packets;
	int error_count;
	unsigned int signr;  /* signal to be sent on error, -1 if none should be sent */
	void *usercontext;
	struct vki_usbdevfs_iso_packet_desc iso_frame_desc[0];
};

struct vki_usbdevfs_ioctl {
	int	ifno;		/* interface 0..N ; negative numbers reserved */
	int	ioctl_code;	/* MUST encode size + direction of data so the
				 * macros in <asm/ioctl.h> give correct values */
	void __user *data;	/* param buffer (in, or out) */
};

#define VKI_USBDEVFS_CONTROL           _VKI_IOWR('U', 0, struct vki_usbdevfs_ctrltransfer)
#define VKI_USBDEVFS_BULK              _VKI_IOWR('U', 2, struct vki_usbdevfs_bulktransfer)
#define VKI_USBDEVFS_GETDRIVER         _VKI_IOW('U', 8, struct vki_usbdevfs_getdriver)
#define VKI_USBDEVFS_SUBMITURB         _VKI_IOR('U', 10, struct vki_usbdevfs_urb)
#define VKI_USBDEVFS_DISCARDURB        _VKI_IO('U', 11)
#define VKI_USBDEVFS_REAPURB           _VKI_IOW('U', 12, void *)
#define VKI_USBDEVFS_REAPURBNDELAY     _VKI_IOW('U', 13, void *)
#define VKI_USBDEVFS_CONNECTINFO       _VKI_IOW('U', 17, struct vki_usbdevfs_connectinfo)
#define VKI_USBDEVFS_IOCTL             _VKI_IOWR('U', 18, struct vki_usbdevfs_ioctl)
#define VKI_USBDEVFS_RESET             _VKI_IO('U', 20)

#define VKI_USBDEVFS_URB_TYPE_ISO              0
#define VKI_USBDEVFS_URB_TYPE_INTERRUPT        1
#define VKI_USBDEVFS_URB_TYPE_CONTROL          2
#define VKI_USBDEVFS_URB_TYPE_BULK             3

// [[this is missing in usbdevice_fs.h]]
struct vki_usbdevfs_setuppacket {
       __vki_u8 bRequestType;
       __vki_u8 bRequest;
       __vki_u16 wValue;
       __vki_u16 wIndex;
       __vki_u16 wLength;
};

//----------------------------------------------------------------------
// From linux-2.6.20.1/include/linux/i2c.h
//----------------------------------------------------------------------

#define VKI_I2C_SMBUS_QUICK             0
#define VKI_I2C_SMBUS_BYTE              1
#define VKI_I2C_SMBUS_BYTE_DATA         2
#define VKI_I2C_SMBUS_WORD_DATA         3
#define VKI_I2C_SMBUS_PROC_CALL         4
#define VKI_I2C_SMBUS_BLOCK_DATA        5
#define VKI_I2C_SMBUS_I2C_BLOCK_BROKEN  6
#define VKI_I2C_SMBUS_BLOCK_PROC_CALL   7           /* SMBus 2.0 */
#define VKI_I2C_SMBUS_I2C_BLOCK_DATA    8

/* smbus_access read or write markers */
#define VKI_I2C_SMBUS_READ  1
#define VKI_I2C_SMBUS_WRITE 0

#define VKI_I2C_SLAVE        0x0703  /* Change slave address                 */
                                     /* Attn.: Slave address is 7 or 10 bits */
#define VKI_I2C_SLAVE_FORCE  0x0706  /* Change slave address                 */
                                     /* Attn.: Slave address is 7 or 10 bits */
                                     /* This changes the address, even if it */
                                     /* is already taken!                    */
#define VKI_I2C_TENBIT       0x0704  /* 0 for 7 bit addrs, != 0 for 10 bit   */
#define VKI_I2C_FUNCS        0x0705  /* Get the adapter functionality */
#define VKI_I2C_RDWR         0x0707  /* Combined R/W transfer (one STOP only) */
#define VKI_I2C_PEC          0x0708  /* != 0 for SMBus PEC                   */
#define VKI_I2C_SMBUS        0x0720  /* SMBus transfer */

#define VKI_I2C_SMBUS_BLOCK_MAX  32  /* As specified in SMBus standard */
union vki_i2c_smbus_data {
        __vki_u8 byte;
        __vki_u16 word;
        __vki_u8 block[VKI_I2C_SMBUS_BLOCK_MAX + 2];
                 /* block[0] is used for length */
                 /* and one more for PEC */
};

/* This is the structure as used in the I2C_SMBUS ioctl call */
struct vki_i2c_smbus_ioctl_data {
        __vki_u8 read_write;
        __vki_u8 command;
        __vki_u32 size;
        union vki_i2c_smbus_data __user *data;
};

struct vki_i2c_msg {
	__vki_u16 addr;		/* slave address			*/
	__vki_u16 flags;
#define VKI_I2C_M_TEN		0x0010	/* this is a ten bit chip address */
#define VKI_I2C_M_RD		0x0001	/* read data, from slave to master */
#define VKI_I2C_M_NOSTART	0x4000	/* if I2C_FUNC_PROTOCOL_MANGLING */
#define VKI_I2C_M_REV_DIR_ADDR	0x2000	/* if I2C_FUNC_PROTOCOL_MANGLING */
#define VKI_I2C_M_IGNORE_NAK	0x1000	/* if I2C_FUNC_PROTOCOL_MANGLING */
#define VKI_I2C_M_NO_RD_ACK	0x0800	/* if I2C_FUNC_PROTOCOL_MANGLING */
#define VKI_I2C_M_RECV_LEN	0x0400	/* length will be first received byte */
	__vki_u16 len;		/* msg length				*/
	__vki_u8 *buf;		/* pointer to msg data			*/
};

struct vki_i2c_rdwr_ioctl_data {
	struct vki_i2c_msg *msgs;	/* pointers to i2c_msgs */
	__vki_u32 nmsgs;		/* number of i2c_msgs */
};

//----------------------------------------------------------------------
// From linux-2.6.20.1/include/linux/keyctl.h
//----------------------------------------------------------------------

/* keyctl commands */
#define VKI_KEYCTL_GET_KEYRING_ID	0	/* ask for a keyring's ID */
#define VKI_KEYCTL_JOIN_SESSION_KEYRING	1	/* join or start named session keyring */
#define VKI_KEYCTL_UPDATE		2	/* update a key */
#define VKI_KEYCTL_REVOKE		3	/* revoke a key */
#define VKI_KEYCTL_CHOWN		4	/* set ownership of a key */
#define VKI_KEYCTL_SETPERM		5	/* set perms on a key */
#define VKI_KEYCTL_DESCRIBE		6	/* describe a key */
#define VKI_KEYCTL_CLEAR		7	/* clear contents of a keyring */
#define VKI_KEYCTL_LINK			8	/* link a key into a keyring */
#define VKI_KEYCTL_UNLINK		9	/* unlink a key from a keyring */
#define VKI_KEYCTL_SEARCH		10	/* search for a key in a keyring */
#define VKI_KEYCTL_READ			11	/* read a key or keyring's contents */
#define VKI_KEYCTL_INSTANTIATE		12	/* instantiate a partially constructed key */
#define VKI_KEYCTL_NEGATE		13	/* negate a partially constructed key */
#define VKI_KEYCTL_SET_REQKEY_KEYRING	14	/* set default request-key keyring */
#define VKI_KEYCTL_SET_TIMEOUT		15	/* set key timeout */
#define VKI_KEYCTL_ASSUME_AUTHORITY	16	/* assume request_key() authorisation */

/*--------------------------------------------------------------------*/
// From linux-2.6.20.1/include/linux/key.h
/*--------------------------------------------------------------------*/

/* key handle serial number */
typedef vki_int32_t vki_key_serial_t;

/* key handle permissions mask */
typedef vki_uint32_t vki_key_perm_t;

//----------------------------------------------------------------------
// From linux-2.6.24.7/include/linux/wireless.h
// (wireless extensions version 22, 2007-03-16)
//----------------------------------------------------------------------

/*
 * [[Wireless extensions ioctls.]]
 */

/* Wireless Identification */
#define VKI_SIOCSIWCOMMIT	0x8B00	/* Commit pending changes to driver */
#define VKI_SIOCGIWNAME		0x8B01	/* get name == wireless protocol */

/* Basic operations */
#define VKI_SIOCSIWNWID		0x8B02	/* set network id (pre-802.11) */
#define VKI_SIOCGIWNWID		0x8B03	/* get network id (the cell) */
#define VKI_SIOCSIWFREQ		0x8B04	/* set channel/frequency (Hz) */
#define VKI_SIOCGIWFREQ		0x8B05	/* get channel/frequency (Hz) */
#define VKI_SIOCSIWMODE		0x8B06	/* set operation mode */
#define VKI_SIOCGIWMODE		0x8B07	/* get operation mode */
#define VKI_SIOCSIWSENS		0x8B08	/* set sensitivity (dBm) */
#define VKI_SIOCGIWSENS		0x8B09	/* get sensitivity (dBm) */

/* Informative stuff */
#define VKI_SIOCSIWRANGE	0x8B0A	/* Unused */
#define VKI_SIOCGIWRANGE	0x8B0B	/* Get range of parameters */
#define VKI_SIOCSIWPRIV		0x8B0C	/* Unused */
#define VKI_SIOCGIWPRIV		0x8B0D	/* get private ioctl interface info */
#define VKI_SIOCSIWSTATS	0x8B0E	/* Unused */
#define VKI_SIOCGIWSTATS	0x8B0F	/* Get /proc/net/wireless stats */

/* Spy support (statistics per MAC address - used for Mobile IP support) */
#define VKI_SIOCSIWSPY		0x8B10	/* set spy addresses */
#define VKI_SIOCGIWSPY		0x8B11	/* get spy info (quality of link) */
#define VKI_SIOCSIWTHRSPY	0x8B12	/* set spy threshold (spy event) */
#define VKI_SIOCGIWTHRSPY	0x8B13	/* get spy threshold */

/* Access Point manipulation */
#define VKI_SIOCSIWAP		0x8B14	/* set access point MAC addresses */
#define VKI_SIOCGIWAP		0x8B15	/* get access point MAC addresses */
#define VKI_SIOCGIWAPLIST	0x8B17	/* Deprecated in favor of scanning */
#define VKI_SIOCSIWSCAN         0x8B18	/* trigger scanning (list cells) */
#define VKI_SIOCGIWSCAN         0x8B19	/* get scanning results */

/* 802.11 specific support */
#define VKI_SIOCSIWESSID	0x8B1A	/* set ESSID (network name) */
#define VKI_SIOCGIWESSID	0x8B1B	/* get ESSID */
#define VKI_SIOCSIWNICKN	0x8B1C	/* set node name/nickname */
#define VKI_SIOCGIWNICKN	0x8B1D	/* get node name/nickname */

/* Other parameters useful in 802.11 and some other devices */
#define VKI_SIOCSIWRATE		0x8B20	/* set default bit rate (bps) */
#define VKI_SIOCGIWRATE		0x8B21	/* get default bit rate (bps) */
#define VKI_SIOCSIWRTS		0x8B22	/* set RTS/CTS threshold (bytes) */
#define VKI_SIOCGIWRTS		0x8B23	/* get RTS/CTS threshold (bytes) */
#define VKI_SIOCSIWFRAG		0x8B24	/* set fragmentation thr (bytes) */
#define VKI_SIOCGIWFRAG		0x8B25	/* get fragmentation thr (bytes) */
#define VKI_SIOCSIWTXPOW	0x8B26	/* set transmit power (dBm) */
#define VKI_SIOCGIWTXPOW	0x8B27	/* get transmit power (dBm) */
#define VKI_SIOCSIWRETRY	0x8B28	/* set retry limits and lifetime */
#define VKI_SIOCGIWRETRY	0x8B29	/* get retry limits and lifetime */

/* Encoding stuff (scrambling, hardware security, WEP...) */
#define VKI_SIOCSIWENCODE	0x8B2A	/* set encoding token & mode */
#define VKI_SIOCGIWENCODE	0x8B2B	/* get encoding token & mode */

/* Power saving stuff (power management, unicast and multicast) */
#define VKI_SIOCSIWPOWER	0x8B2C	/* set Power Management settings */
#define VKI_SIOCGIWPOWER	0x8B2D	/* get Power Management settings */

/* WPA : Generic IEEE 802.11 information element (e.g., for WPA/RSN/WMM). */
#define VKI_SIOCSIWGENIE	0x8B30		/* set generic IE */
#define VKI_SIOCGIWGENIE	0x8B31		/* get generic IE */

/* WPA : IEEE 802.11 MLME requests */
#define VKI_SIOCSIWMLME		0x8B16	/* request MLME operation; uses
					 * struct iw_mlme */
/* WPA : Authentication mode parameters */
#define VKI_SIOCSIWAUTH		0x8B32	/* set authentication mode params */
#define VKI_SIOCGIWAUTH		0x8B33	/* get authentication mode params */

/* WPA : Extended version of encoding configuration */
#define VKI_SIOCSIWENCODEEXT	0x8B34	/* set encoding token & mode */
#define VKI_SIOCGIWENCODEEXT	0x8B35	/* get encoding token & mode */

/* WPA2 : PMKSA cache management */
#define VKI_SIOCSIWPMKSA	0x8B36	/* PMKSA cache operation */

/*
 * [[Payload for the wireless extensions ioctls.]]
 */

struct	vki_iw_param
{
  __vki_s32	value;		/* The value of the parameter itself */
  __vki_u8	fixed;		/* Hardware should not use auto select */
  __vki_u8	disabled;	/* Disable the feature */
  __vki_u16	flags;		/* Various specific flags (if any) */
};

struct	vki_iw_point
{
  void __user	*pointer;	/* Pointer to the data  (in user space) */
  __vki_u16	length;		/* number of fields or size in bytes */
  __vki_u16	flags;		/* Optional params */
};

struct	vki_iw_freq
{
	__vki_s32	m;		/* Mantissa */
	__vki_s16	e;		/* Exponent */
	__vki_u8	i;		/* List index (when in range struct) */
	__vki_u8	flags;		/* Flags (fixed/auto) */
};

struct	vki_iw_quality
{
	__vki_u8	qual;		/* link quality (%retries, SNR,
					   %missed beacons or better...) */
	__vki_u8	level;		/* signal level (dBm) */
	__vki_u8	noise;		/* noise level (dBm) */
	__vki_u8	updated;	/* Flags to know if updated */
};

union	vki_iwreq_data
{
	/* Config - generic */
	char		name[VKI_IFNAMSIZ];
	/* Name : used to verify the presence of  wireless extensions.
	 * Name of the protocol/provider... */

	struct vki_iw_point	essid;	/* Extended network name */
	struct vki_iw_param	nwid;	/* network id (or domain - the cell) */
	struct vki_iw_freq	freq;	/* frequency or channel :
					 * 0-1000 = channel
					 * > 1000 = frequency in Hz */

	struct vki_iw_param	sens;	/* signal level threshold */
	struct vki_iw_param	bitrate;/* default bit rate */
	struct vki_iw_param	txpower;/* default transmit power */
	struct vki_iw_param	rts;	/* RTS threshold threshold */
	struct vki_iw_param	frag;	/* Fragmentation threshold */
	__vki_u32		mode;	/* Operation mode */
	struct vki_iw_param	retry;	/* Retry limits & lifetime */

	struct vki_iw_point	encoding; /* Encoding stuff : tokens */
	struct vki_iw_param	power;	/* PM duration/timeout */
	struct vki_iw_quality	qual;	/* Quality part of statistics */

	struct vki_sockaddr ap_addr;	/* Access point address */
	struct vki_sockaddr addr;	/* Destination address (hw/mac) */

	struct vki_iw_param	param;	/* Other small parameters */
	struct vki_iw_point	data;	/* Other large parameters */
};

struct	vki_iwreq 
{
	union
	{
		char ifrn_name[VKI_IFNAMSIZ];	/* if name, e.g. "eth0" */
	} ifr_ifrn;

	/* Data part (defined just above) */
	union	vki_iwreq_data	u;
};

/*--------------------------------------------------------------------*/
// From linux-2.6.31.5/include/linux/perf_event.h
/*--------------------------------------------------------------------*/

struct vki_perf_event_attr {

	/*
	 * Major type: hardware/software/tracepoint/etc.
	 */
	__vki_u32			type;

	/*
	 * Size of the attr structure, for fwd/bwd compat.
	 */
	__vki_u32			size;

	/*
	 * Type specific configuration information.
	 */
	__vki_u64			config;

	union {
		__vki_u64		sample_period;
		__vki_u64		sample_freq;
	};

	__vki_u64			sample_type;
	__vki_u64			read_format;

	__vki_u64			disabled       :  1, /* off by default        */
					inherit	       :  1, /* children inherit it   */
					pinned	       :  1, /* must always be on PMU */
					exclusive      :  1, /* only group on PMU     */
					exclude_user   :  1, /* don't count user      */
					exclude_kernel :  1, /* ditto kernel          */
					exclude_hv     :  1, /* ditto hypervisor      */
					exclude_idle   :  1, /* don't count when idle */
					mmap           :  1, /* include mmap data     */
					comm	       :  1, /* include comm data     */
					freq           :  1, /* use freq, not period  */
					inherit_stat   :  1, /* per task counts       */
					enable_on_exec :  1, /* next exec enables     */
					task           :  1, /* trace fork/exit       */
					watermark      :  1, /* wakeup_watermark      */
					/*
					 * precise_ip:
					 *
					 *  0 - SAMPLE_IP can have arbitrary skid
					 *  1 - SAMPLE_IP must have constant skid
					 *  2 - SAMPLE_IP requested to have 0 skid
					 *  3 - SAMPLE_IP must have 0 skid
					 *
					 *  See also PERF_RECORD_MISC_EXACT_IP
					 */
					precise_ip     :  2, /* skid constraint       */
					mmap_data      :  1, /* non-exec mmap data    */
					sample_id_all  :  1, /* sample_type all events */

					__reserved_1   : 45;

	union {
		__vki_u32		wakeup_events;	  /* wakeup every n events */
		__vki_u32		wakeup_watermark; /* bytes before wakeup   */
	};

	__vki_u32			bp_type;
	union {
		__vki_u64		bp_addr;
		__vki_u64		config1; /* extension of config */
	};
	union {
		__vki_u64		bp_len;
		__vki_u64		config2; /* extension of config1 */
	};
};

#define VKI_PERF_EVENT_IOC_ENABLE       _VKI_IO ('$', 0)
#define VKI_PERF_EVENT_IOC_DISABLE      _VKI_IO ('$', 1)
#define VKI_PERF_EVENT_IOC_REFRESH      _VKI_IO ('$', 2)
#define VKI_PERF_EVENT_IOC_RESET        _VKI_IO ('$', 3)
#define VKI_PERF_EVENT_IOC_PERIOD       _VKI_IOW('$', 4, __vki_u64)
#define VKI_PERF_EVENT_IOC_SET_OUTPUT   _VKI_IO ('$', 5)
#define VKI_PERF_EVENT_IOC_SET_FILTER   _VKI_IOW('$', 6, char *)
#define VKI_PERF_EVENT_IOC_ID           _VKI_IOR('$', 7, __vki_u64 *)
#define VKI_PERF_EVENT_IOC_SET_BPF      _VKI_IOW('$', 8, __vki_u32)

/*--------------------------------------------------------------------*/
// From linux-2.6.32.4/include/linux/getcpu.h
/*--------------------------------------------------------------------*/

struct vki_getcpu_cache {
	unsigned long blob[128 / sizeof(long)];
};

//----------------------------------------------------------------------
// From linux-2.6.33.3/include/linux/input.h
//----------------------------------------------------------------------

/*
 * IOCTLs (0x00 - 0x7f)
 */

#define VKI_EVIOCGNAME(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x06, len)		/* get device name */
#define VKI_EVIOCGPHYS(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x07, len)		/* get physical location */
#define VKI_EVIOCGUNIQ(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x08, len)		/* get unique identifier */

#define VKI_EVIOCGKEY(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x18, len)		/* get global keystate */
#define VKI_EVIOCGLED(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x19, len)		/* get all LEDs */
#define VKI_EVIOCGSND(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x1a, len)		/* get all sounds status */
#define VKI_EVIOCGSW(len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x1b, len)		/* get all switch states */

#define VKI_EVIOCGBIT(ev,len)	_VKI_IOC(_VKI_IOC_READ, 'E', 0x20 + ev, len)	/* get event bits */

/*
 * Event types
 */

#define VKI_EV_SYN		0x00
#define VKI_EV_KEY		0x01
#define VKI_EV_REL		0x02
#define VKI_EV_ABS		0x03
#define VKI_EV_MSC		0x04
#define VKI_EV_SW		0x05
#define VKI_EV_LED		0x11
#define VKI_EV_SND		0x12
#define VKI_EV_REP		0x14
#define VKI_EV_FF		0x15
#define VKI_EV_PWR		0x16
#define VKI_EV_FF_STATUS	0x17
#define VKI_EV_MAX		0x1f
#define VKI_EV_CNT		(VKI_EV_MAX+1)

//----------------------------------------------------------------------
// From linux-2.6.39-rc2/include/asm_generic/ioctls.h
//----------------------------------------------------------------------

#ifndef VKI_FIOQSIZE
#define VKI_FIOQSIZE 0x5460     /* Value differs on some platforms */
#endif

#ifndef VKI_TIOCSIG
#define VKI_TIOCSIG _VKI_IOW('T', 0x36, int) /* Value differs on some platforms */
#endif

//----------------------------------------------------------------------
// From kernel/common/include/linux/ashmem.h
//----------------------------------------------------------------------

#if defined(VGPV_arm_linux_android) || defined(VGPV_x86_linux_android) \
    || defined(VGPV_mips32_linux_android) \
    || defined(VGPV_arm64_linux_android)

#define VKI_ASHMEM_NAME_LEN 256

#define VKI_ASHMEM_NAME_DEF "dev/ashmem"

#define VKI_ASHMEM_NOT_PURGED 0
#define VKI_ASHMEM_WAS_PURGED 1

#define VKI_ASHMEM_IS_UNPINNED 0
#define VKI_ASHMEM_IS_PINNED 1

struct vki_ashmem_pin {
   vki_uint32_t offset;
   vki_uint32_t len;
};

#define __VKI_ASHMEMIOC 0x77

#define VKI_ASHMEM_SET_NAME _VKI_IOW(__VKI_ASHMEMIOC, 1, char[VKI_ASHMEM_NAME_LEN])
#define VKI_ASHMEM_GET_NAME _VKI_IOR(__VKI_ASHMEMIOC, 2, char[VKI_ASHMEM_NAME_LEN])
#define VKI_ASHMEM_SET_SIZE _VKI_IOW(__VKI_ASHMEMIOC, 3, vki_size_t)
#define VKI_ASHMEM_GET_SIZE _VKI_IO(__VKI_ASHMEMIOC, 4)
#define VKI_ASHMEM_SET_PROT_MASK _VKI_IOW(__VKI_ASHMEMIOC, 5, unsigned long)
#define VKI_ASHMEM_GET_PROT_MASK _VKI_IO(__VKI_ASHMEMIOC, 6)
#define VKI_ASHMEM_PIN _VKI_IOW(__VKI_ASHMEMIOC, 7, struct vki_ashmem_pin)
#define VKI_ASHMEM_UNPIN _VKI_IOW(__VKI_ASHMEMIOC, 8, struct vki_ashmem_pin)
#define VKI_ASHMEM_GET_PIN_STATUS _VKI_IO(__VKI_ASHMEMIOC, 9)
#define VKI_ASHMEM_PURGE_ALL_CACHES _VKI_IO(__VKI_ASHMEMIOC, 10)

//----------------------------------------------------------------------
// From kernel/common/include/linux/binder.h
//----------------------------------------------------------------------

struct vki_binder_write_read {
 signed long write_size;
 signed long write_consumed;
 unsigned long write_buffer;
 signed long read_size;
 signed long read_consumed;
 unsigned long read_buffer;
};

struct vki_binder_version {
 signed long protocol_version;
};

#define VKI_BINDER_WRITE_READ _VKI_IOWR('b', 1, struct vki_binder_write_read)
#define VKI_BINDER_SET_IDLE_TIMEOUT _VKI_IOW('b', 3, vki_int64_t)
#define VKI_BINDER_SET_MAX_THREADS _VKI_IOW('b', 5, vki_size_t)
#define VKI_BINDER_SET_IDLE_PRIORITY _VKI_IOW('b', 6, int)
#define VKI_BINDER_SET_CONTEXT_MGR _VKI_IOW('b', 7, int)
#define VKI_BINDER_THREAD_EXIT _VKI_IOW('b', 8, int)
#define VKI_BINDER_VERSION _VKI_IOWR('b', 9, struct vki_binder_version)

#endif /* defined(VGPV_*_linux_android) */

//----------------------------------------------------------------------
// From linux-3.0.4/include/net/bluetooth/bluetooth.h
//----------------------------------------------------------------------

typedef struct {
   __vki_u8 b[6];
} __vki_packed vki_bdaddr_t;

//----------------------------------------------------------------------
// From linux-3.0.4/include/net/bluetooth/hci.h
//----------------------------------------------------------------------

#define VKI_HCIDEVUP        _VKI_IOW('H', 201, int)
#define VKI_HCIDEVDOWN      _VKI_IOW('H', 202, int)
#define VKI_HCIDEVRESET     _VKI_IOW('H', 203, int)
#define VKI_HCIDEVRESTAT    _VKI_IOW('H', 204, int)

#define VKI_HCIGETDEVLIST   _VKI_IOR('H', 210, int)

struct vki_hci_dev_req {
	__vki_u16  dev_id;
	__vki_u32  dev_opt;
};

struct vki_hci_dev_list_req {
	__vki_u16  dev_num;
	struct vki_hci_dev_req dev_req[0];	/* hci_dev_req structures */
};

#define VKI_HCIGETDEVINFO   _VKI_IOR('H', 211, int)
#define VKI_HCIGETCONNLIST  _VKI_IOR('H', 212, int)
#define VKI_HCIGETCONNINFO  _VKI_IOR('H', 213, int)
#define VKI_HCIGETAUTHINFO  _VKI_IOR('H', 215, int)

#define VKI_HCISETRAW       _VKI_IOW('H', 220, int)
#define VKI_HCISETSCAN      _VKI_IOW('H', 221, int)
#define VKI_HCISETAUTH      _VKI_IOW('H', 222, int)
#define VKI_HCISETENCRYPT   _VKI_IOW('H', 223, int)
#define VKI_HCISETPTYPE     _VKI_IOW('H', 224, int)
#define VKI_HCISETLINKPOL   _VKI_IOW('H', 225, int)
#define VKI_HCISETLINKMODE  _VKI_IOW('H', 226, int)
#define VKI_HCISETACLMTU    _VKI_IOW('H', 227, int)
#define VKI_HCISETSCOMTU    _VKI_IOW('H', 228, int)

#define VKI_HCIBLOCKADDR    _VKI_IOW('H', 230, int)
#define VKI_HCIUNBLOCKADDR  _VKI_IOW('H', 231, int)

#define VKI_HCIINQUIRY      _VKI_IOR('H', 240, int)

struct vki_inquiry_info {
   vki_bdaddr_t bdaddr;
   __vki_u8     pscan_rep_mode;
   __vki_u8     pscan_period_mode;
   __vki_u8     pscan_mode;
   __vki_u8     dev_class[3];
   __vki_le16   clock_offset;
} __vki_packed;

struct vki_hci_inquiry_req {
   __vki_u16 dev_id;
   __vki_u16 flags;
   __vki_u8  lap[3];
   __vki_u8  length;
   __vki_u8  num_rsp;
};

//----------------------------------------------------------------------
// From linux-3.9.2/include/net/bluetooth/rfcomm.h
//----------------------------------------------------------------------

struct vki_sockaddr_rc {
        vki_sa_family_t     rc_family;
        vki_bdaddr_t        rc_bdaddr;
        __vki_u8            rc_channel;
};

//----------------------------------------------------------------------
// From linux-3.4/include/linux/kvm.h
//----------------------------------------------------------------------
#define KVMIO 0xAE

#define VKI_KVM_GET_API_VERSION       _VKI_IO(KVMIO,   0x00)
#define VKI_KVM_CREATE_VM             _VKI_IO(KVMIO,   0x01) /* returns a VM fd */
#define VKI_KVM_CHECK_EXTENSION       _VKI_IO(KVMIO,   0x03)
#define VKI_KVM_GET_VCPU_MMAP_SIZE    _VKI_IO(KVMIO,   0x04) /* in bytes */
#define VKI_KVM_S390_ENABLE_SIE       _VKI_IO(KVMIO,   0x06)
#define VKI_KVM_CREATE_VCPU           _VKI_IO(KVMIO,   0x41)
#define VKI_KVM_SET_NR_MMU_PAGES      _VKI_IO(KVMIO,   0x44)
#define VKI_KVM_GET_NR_MMU_PAGES      _VKI_IO(KVMIO,   0x45)
#define VKI_KVM_SET_TSS_ADDR          _VKI_IO(KVMIO,   0x47)
#define VKI_KVM_CREATE_IRQCHIP        _VKI_IO(KVMIO,   0x60)
#define VKI_KVM_CREATE_PIT            _VKI_IO(KVMIO,   0x64)
#define VKI_KVM_REINJECT_CONTROL      _VKI_IO(KVMIO,   0x71)
#define VKI_KVM_SET_BOOT_CPU_ID       _VKI_IO(KVMIO,   0x78)
#define VKI_KVM_SET_TSC_KHZ           _VKI_IO(KVMIO,  0xa2)
#define VKI_KVM_GET_TSC_KHZ           _VKI_IO(KVMIO,  0xa3)
#define VKI_KVM_RUN                   _VKI_IO(KVMIO,   0x80)
#define VKI_KVM_S390_INITIAL_RESET    _VKI_IO(KVMIO,   0x97)
#define VKI_KVM_NMI                   _VKI_IO(KVMIO,   0x9a)
#define VKI_KVM_KVMCLOCK_CTRL         _VKI_IO(KVMIO,   0xad)

struct vki_kvm_s390_mem_op {
        /* in */
        __vki_u64 gaddr;            /* the guest address */
        __vki_u64 flags;            /* flags */
        __vki_u32 size;             /* amount of bytes */
        __vki_u32 op;               /* type of operation */
        __vki_u64 buf;              /* buffer in userspace */
        __vki_u8 ar;                /* the access register number */
        __vki_u8 reserved[31];      /* should be set to 0 */
};

#define VKI_KVM_S390_MEMOP_LOGICAL_READ		0
#define VKI_KVM_S390_MEMOP_LOGICAL_WRITE	1
#define VKI_KVM_S390_MEMOP_F_CHECK_ONLY		(1ULL << 0)
#define VKI_KVM_S390_MEMOP_F_INJECT_EXCEPTION	(1ULL << 1)

#define VKI_KVM_S390_MEM_OP           _VKI_IOW(KVMIO,  0xb1, struct vki_kvm_s390_mem_op)

//----------------------------------------------------------------------
// From linux-2.6/include/linux/net_stamp.h
//----------------------------------------------------------------------

struct vki_hwtstamp_config {
	int flags;
	int tx_type;
	int rx_filter;
};

//----------------------------------------------------------------------
// From linux-2.6.12-rc2/include/linux/uinput.h
//----------------------------------------------------------------------

#define VKI_UINPUT_IOCTL_BASE       'U'
#define VKI_UI_DEV_CREATE		_VKI_IO(VKI_UINPUT_IOCTL_BASE, 1)
#define VKI_UI_DEV_DESTROY		_VKI_IO(VKI_UINPUT_IOCTL_BASE, 2)

#define VKI_UI_SET_EVBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 100, int)
#define VKI_UI_SET_KEYBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 101, int)
#define VKI_UI_SET_RELBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 102, int)
#define VKI_UI_SET_ABSBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 103, int)
#define VKI_UI_SET_MSCBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 104, int)
#define VKI_UI_SET_LEDBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 105, int)
#define VKI_UI_SET_SNDBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 106, int)
#define VKI_UI_SET_FFBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 107, int)
#define VKI_UI_SET_SWBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 109, int)
#define VKI_UI_SET_PROPBIT		_VKI_IOW(VKI_UINPUT_IOCTL_BASE, 110, int)

//----------------------------------------------------------------------
// From linux-2.6/include/uapi/rdma/ib_user_mad.h
//----------------------------------------------------------------------

#define VKI_IB_IOCTL_MAGIC          0x1b

#define VKI_IB_USER_MAD_REGISTER_AGENT    _VKI_IOWR(VKI_IB_IOCTL_MAGIC, 1, \
                                              struct ib_user_mad_reg_req)

#define VKI_IB_USER_MAD_UNREGISTER_AGENT  _VKI_IOW(VKI_IB_IOCTL_MAGIC, 2, __u32)

#define VKI_IB_USER_MAD_ENABLE_PKEY       _VKI_IO(VKI_IB_IOCTL_MAGIC, 3)

//----------------------------------------------------------------------
// From linux-3.8/include/uapi/linux/if_tun.h
//----------------------------------------------------------------------

#define VKI_TUNSETNOCSUM  _VKI_IOW('T', 200, int) 
#define VKI_TUNSETDEBUG   _VKI_IOW('T', 201, int) 
#define VKI_TUNSETIFF     _VKI_IOW('T', 202, int) 
#define VKI_TUNSETPERSIST _VKI_IOW('T', 203, int) 
#define VKI_TUNSETOWNER   _VKI_IOW('T', 204, int)
#define VKI_TUNSETLINK    _VKI_IOW('T', 205, int)
#define VKI_TUNSETGROUP   _VKI_IOW('T', 206, int)
#define VKI_TUNGETFEATURES _VKI_IOR('T', 207, unsigned int)
#define VKI_TUNSETOFFLOAD  _VKI_IOW('T', 208, unsigned int)
#define VKI_TUNSETTXFILTER _VKI_IOW('T', 209, unsigned int)
#define VKI_TUNGETIFF      _VKI_IOR('T', 210, unsigned int)
#define VKI_TUNGETSNDBUF   _VKI_IOR('T', 211, int)
#define VKI_TUNSETSNDBUF   _VKI_IOW('T', 212, int)
//#define VKI_TUNATTACHFILTER _VKI_IOW('T', 213, struct sock_fprog)
//#define VKI_TUNDETACHFILTER _VKI_IOW('T', 214, struct sock_fprog)
#define VKI_TUNGETVNETHDRSZ _VKI_IOR('T', 215, int)
#define VKI_TUNSETVNETHDRSZ _VKI_IOW('T', 216, int)
#define VKI_TUNSETQUEUE  _VKI_IOW('T', 217, int)
#define VKI_TUNSETIFINDEX	_VKI_IOW('T', 218, unsigned int)
//#define VKI_TUNGETFILTER _VKI_IOR('T', 219, struct sock_fprog)

//----------------------------------------------------------------------
// From linux-3.8/include/uapi/linux/vhost.h
//----------------------------------------------------------------------

#define VKI_VHOST_VIRTIO 0xAF
#define VKI_VHOST_SET_OWNER _VKI_IO(VKI_VHOST_VIRTIO, 0x01)
#define VKI_VHOST_RESET_OWNER _VKI_IO(VKI_VHOST_VIRTIO, 0x02)

//----------------------------------------------------------------------
// Xen privcmd IOCTL
//----------------------------------------------------------------------

typedef unsigned long __vki_xen_pfn_t;

struct vki_xen_privcmd_hypercall {
       __vki_u64 op;
       __vki_u64 arg[5];
};

struct vki_xen_privcmd_mmap_entry {
        __vki_u64 va;
        __vki_u64 mfn;
        __vki_u64 npages;
};

struct vki_xen_privcmd_mmap {
        int num;
        __vki_u16 dom; /* target domain */
        struct vki_xen_privcmd_mmap_entry *entry;
};

struct vki_xen_privcmd_mmapbatch {
        int num;     /* number of pages to populate */
        __vki_u16 dom; /* target domain */
        __vki_u64 addr;  /* virtual address */
        __vki_xen_pfn_t *arr; /* array of mfns - top nibble set on err */
};

struct vki_xen_privcmd_mmapbatch_v2 {
        unsigned int num; /* number of pages to populate */
        __vki_u16 dom;      /* target domain */
        __vki_u64 addr;       /* virtual address */
        const __vki_xen_pfn_t *arr; /* array of mfns */
        int __user *err;  /* array of error codes */
};

#define VKI_XEN_IOCTL_PRIVCMD_HYPERCALL    _VKI_IOC(_VKI_IOC_NONE, 'P', 0, sizeof(struct vki_xen_privcmd_hypercall))
#define VKI_XEN_IOCTL_PRIVCMD_MMAP         _VKI_IOC(_VKI_IOC_NONE, 'P', 2, sizeof(struct vki_xen_privcmd_mmap))

#define VKI_XEN_IOCTL_PRIVCMD_MMAPBATCH    _VKI_IOC(_VKI_IOC_NONE, 'P', 3, sizeof(struct vki_xen_privcmd_mmapbatch))
#define VKI_XEN_IOCTL_PRIVCMD_MMAPBATCH_V2 _VKI_IOC(_VKI_IOC_NONE, 'P', 4, sizeof(struct vki_xen_privcmd_mmapbatch_v2))

//----------------------------------------------------------------------
// Xen evtchn IOCTL
//----------------------------------------------------------------------

#define VKI_XEN_IOCTL_EVTCHN_BIND_VIRQ				\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 0, sizeof(struct vki_xen_ioctl_evtchn_bind_virq))
struct vki_xen_ioctl_evtchn_bind_virq {
	vki_uint32_t virq;
};

#define VKI_XEN_IOCTL_EVTCHN_BIND_INTERDOMAIN			\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 1, sizeof(struct vki_xen_ioctl_evtchn_bind_interdomain))
struct vki_xen_ioctl_evtchn_bind_interdomain {
	vki_uint32_t remote_domain;
	vki_uint32_t remote_port;
};

#define VKI_XEN_IOCTL_EVTCHN_BIND_UNBOUND_PORT			\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 2, sizeof(struct vki_xen_ioctl_evtchn_bind_unbound_port))
struct vki_xen_ioctl_evtchn_bind_unbound_port {
	vki_uint32_t remote_domain;
};

#define VKI_XEN_IOCTL_EVTCHN_UNBIND				\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 3, sizeof(struct vki_xen_ioctl_evtchn_unbind))
struct vki_xen_ioctl_evtchn_unbind {
	vki_uint32_t port;
};

#define VKI_XEN_IOCTL_EVTCHN_NOTIFY				\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 4, sizeof(struct vki_xen_ioctl_evtchn_notify))
struct vki_xen_ioctl_evtchn_notify {
	vki_uint32_t port;
};

#define VKI_XEN_IOCTL_EVTCHN_RESET				\
	_VKI_IOC(_VKI_IOC_NONE, 'E', 5, 0)


//----------------------------------------------------------------------
// From linux-3.4.0/include/linux/fs.h
//----------------------------------------------------------------------

struct vki_file_handle {
   __vki_u32 handle_bytes;
   int handle_type;
   /* file identifier */
   unsigned char f_handle[0];
};

//----------------------------------------------------------------------
// From linux-3.2.0/include/linux/filter.h
//----------------------------------------------------------------------

struct vki_sock_filter {
	__vki_u16 code; /* Actual filter code */
	__vki_u8 jt;    /* Jump true */
	__vki_u8 jf;    /* Jump false */
	__vki_u32 k;    /* Generic multiuse field */
};

struct vki_sock_fprog {
	__vki_u16 len;  /* actually unsigned short */
	struct vki_sock_filter *filter;
};
   
//----------------------------------------------------------------------
// From linux/include/uapi/linux/ethtool.h
//----------------------------------------------------------------------

struct vki_ethtool_cmd {
	__vki_u32	cmd;
	__vki_u32	supported;
	__vki_u32	advertising;
	__vki_u16	speed;
	__vki_u8	duplex;
	__vki_u8	port;
	__vki_u8	phy_address;
	__vki_u8	transceiver;
	__vki_u8	autoneg;
	__vki_u8	mdio_support;
	__vki_u32	maxtxpkt;
	__vki_u32	maxrxpkt;
	__vki_u16	speed_hi;
	__vki_u8	eth_tp_mdix;
	__vki_u8	eth_tp_mdix_ctrl;
	__vki_u32	lp_advertising;
	__vki_u32	reserved[2];
};

#define VKI_ETHTOOL_FWVERS_LEN	32
#define VKI_ETHTOOL_BUSINFO_LEN	32

struct vki_ethtool_drvinfo {
	__vki_u32	cmd;
	char	driver[32];
	char	version[32];
	char	fw_version[VKI_ETHTOOL_FWVERS_LEN];
	char	bus_info[VKI_ETHTOOL_BUSINFO_LEN];
	char	reserved1[32];
	char	reserved2[12];
	__vki_u32	n_priv_flags;
	__vki_u32	n_stats;
	__vki_u32	testinfo_len;
	__vki_u32	eedump_len;
	__vki_u32	regdump_len;
};

#define VKI_SOPASS_MAX	6

struct vki_ethtool_wolinfo {
	__vki_u32	cmd;
	__vki_u32	supported;
	__vki_u32	wolopts;
	__vki_u8	sopass[VKI_SOPASS_MAX];
};

struct vki_ethtool_value {
	__vki_u32	cmd;
	__vki_u32	data;
};

struct vki_ethtool_regs {
	__vki_u32	cmd;
	__vki_u32	version;
	__vki_u32	len;
	__vki_u8	data[0];
};

struct vki_ethtool_ringparam {
	__vki_u32	cmd;
	__vki_u32	rx_max_pending;
	__vki_u32	rx_mini_max_pending;
	__vki_u32	rx_jumbo_max_pending;
	__vki_u32	tx_max_pending;
	__vki_u32	rx_pending;
	__vki_u32	rx_mini_pending;
	__vki_u32	rx_jumbo_pending;
	__vki_u32	tx_pending;
};

struct vki_ethtool_channels {
	__vki_u32	cmd;
	__vki_u32	max_rx;
	__vki_u32	max_tx;
	__vki_u32	max_other;
	__vki_u32	max_combined;
	__vki_u32	rx_count;
	__vki_u32	tx_count;
	__vki_u32	other_count;
	__vki_u32	combined_count;
};

struct vki_ethtool_sset_info {
	__vki_u32	cmd;
	__vki_u32	reserved;
	__vki_u64	sset_mask;
	__vki_u32	data[0];
};

struct vki_ethtool_test {
	__vki_u32	cmd;
	__vki_u32	flags;
	__vki_u32	reserved;
	__vki_u32	len;
	__vki_u64	data[0];
};

struct vki_ethtool_perm_addr {
	__vki_u32	cmd;
	__vki_u32	size;
	__vki_u8	data[0];
};

struct vki_ethtool_get_features_block {
	__vki_u32	available;
	__vki_u32	requested;
	__vki_u32	active;
	__vki_u32	never_changed;
};

struct vki_ethtool_gfeatures {
	__vki_u32	cmd;
	__vki_u32	size;
	struct vki_ethtool_get_features_block features[0];
};

struct vki_ethtool_set_features_block {
	__vki_u32	valid;
	__vki_u32	requested;
};

struct vki_ethtool_sfeatures {
	__vki_u32	cmd;
	__vki_u32	size;
	struct vki_ethtool_set_features_block features[0];
};

struct vki_ethtool_ts_info {
	__vki_u32	cmd;
	__vki_u32	so_timestamping;
	__vki_s32	phc_index;
	__vki_u32	tx_types;
	__vki_u32	tx_reserved[3];
	__vki_u32	rx_filters;
	__vki_u32	rx_reserved[3];
};

#define VKI_ETHTOOL_GSET	0x00000001 /* Get settings. */
#define VKI_ETHTOOL_SSET	0x00000002 /* Set settings. */
#define VKI_ETHTOOL_GDRVINFO	0x00000003 /* Get driver info. */
#define VKI_ETHTOOL_GREGS	0x00000004 /* Get NIC registers. */
#define VKI_ETHTOOL_GWOL	0x00000005 /* Get wake-on-lan options. */
#define VKI_ETHTOOL_SWOL	0x00000006 /* Set wake-on-lan options. */
#define VKI_ETHTOOL_GMSGLVL	0x00000007 /* Get driver message level */
#define VKI_ETHTOOL_SMSGLVL	0x00000008 /* Set driver msg level. */
#define VKI_ETHTOOL_NWAY_RST	0x00000009 /* Restart autonegotiation. */
#define VKI_ETHTOOL_GLINK	0x0000000a
#define VKI_ETHTOOL_GRINGPARAM	0x00000010 /* Get ring parameters */
#define VKI_ETHTOOL_SRINGPARAM	0x00000011 /* Set ring parameters. */
#define VKI_ETHTOOL_GRXCSUM	0x00000014 /* Get RX hw csum enable (ethtool_value) */
#define VKI_ETHTOOL_SRXCSUM	0x00000015 /* Set RX hw csum enable (ethtool_value) */
#define VKI_ETHTOOL_GTXCSUM	0x00000016 /* Get TX hw csum enable (ethtool_value) */
#define VKI_ETHTOOL_STXCSUM	0x00000017 /* Set TX hw csum enable (ethtool_value) */
#define VKI_ETHTOOL_GSG		0x00000018 /* Get scatter-gather enable
					    * (ethtool_value) */
#define VKI_ETHTOOL_SSG		0x00000019 /* Set scatter-gather enable
					    * (ethtool_value). */
#define VKI_ETHTOOL_TEST	0x0000001a /* execute NIC self-test. */
#define VKI_ETHTOOL_PHYS_ID	0x0000001c /* identify the NIC */
#define VKI_ETHTOOL_GTSO	0x0000001e /* Get TSO enable (ethtool_value) */
#define VKI_ETHTOOL_STSO	0x0000001f /* Set TSO enable (ethtool_value) */
#define VKI_ETHTOOL_GPERMADDR	0x00000020 /* Get permanent hardware address */
#define VKI_ETHTOOL_GUFO	0x00000021 /* Get UFO enable (ethtool_value) */
#define VKI_ETHTOOL_SUFO	0x00000022 /* Set UFO enable (ethtool_value) */
#define VKI_ETHTOOL_GGSO	0x00000023 /* Get GSO enable (ethtool_value) */
#define VKI_ETHTOOL_SGSO	0x00000024 /* Set GSO enable (ethtool_value) */
#define VKI_ETHTOOL_GFLAGS	0x00000025 /* Get flags bitmap(ethtool_value) */
#define VKI_ETHTOOL_SFLAGS	0x00000026 /* Set flags bitmap(ethtool_value) */
#define VKI_ETHTOOL_GGRO	0x0000002b /* Get GRO enable (ethtool_value) */
#define VKI_ETHTOOL_SGRO	0x0000002c /* Set GRO enable (ethtool_value) */
#define VKI_ETHTOOL_RESET	0x00000034 /* Reset hardware */
#define VKI_ETHTOOL_GSSET_INFO	0x00000037 /* Get string set info */
#define VKI_ETHTOOL_GFEATURES	0x0000003a /* Get device offload settings */
#define VKI_ETHTOOL_SFEATURES	0x0000003b /* Change device offload settings */
#define VKI_ETHTOOL_GCHANNELS	0x0000003c /* Get no of channels */
#define VKI_ETHTOOL_SCHANNELS	0x0000003d /* Set no of channels */
#define VKI_ETHTOOL_GET_TS_INFO	0x00000041 /* Get time stamping and PHC info */

//----------------------------------------------------------------------
// From linux-3.15.8/drivers/staging/android/uapi/ion.h
//----------------------------------------------------------------------

typedef int vki_ion_user_handle_t;

struct vki_ion_allocation_data {
        vki_size_t len;
        vki_size_t align;
        unsigned int heap_id_mask;
        unsigned int flags;
        vki_ion_user_handle_t handle;
};

struct vki_ion_fd_data {
        vki_ion_user_handle_t handle;
        int fd;
};

struct vki_ion_handle_data {
        vki_ion_user_handle_t handle;
};

struct vki_ion_custom_data {
        unsigned int cmd;
        unsigned long arg;
};

#define VKI_ION_IOC_MAGIC   'I'

#define VKI_ION_IOC_ALLOC \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 0, struct vki_ion_allocation_data)

#define VKI_ION_IOC_FREE \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 1, struct vki_ion_handle_data)

#define VKI_ION_IOC_MAP \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 2, struct vki_ion_fd_data)

#define VKI_ION_IOC_SHARE \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 4, struct vki_ion_fd_data)

#define VKI_ION_IOC_IMPORT \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 5, struct vki_ion_fd_data)

#define VKI_ION_IOC_SYNC \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 7, struct vki_ion_fd_data)

#define VKI_ION_IOC_CUSTOM \
   _VKI_IOWR(VKI_ION_IOC_MAGIC, 6, struct vki_ion_custom_data)

//----------------------------------------------------------------------
// From linux-3.19-rc5/drivers/staging/android/uapi/sync.h
//----------------------------------------------------------------------

struct vki_sync_merge_data {
        __vki_s32 fd2;
        char      name[32];
        __vki_s32 fence;
};

struct vki_sync_pt_info {
        __vki_u32 len;
        char      obj_name[32];
        char      driver_name[32];
        __vki_s32 status;
        __vki_u64 timestamp_ns;
        __vki_u8  driver_data[0];
};

struct vki_sync_fence_info_data {
        __vki_u32 len;
        char      name[32];
        __vki_s32 status;
        __vki_u8  pt_info[0];
};

#define VKI_SYNC_IOC_MAGIC   '>'

#define VKI_SYNC_IOC_WAIT \
   _VKI_IOW(VKI_SYNC_IOC_MAGIC, 0, __vki_s32)

#define VKI_SYNC_IOC_MERGE \
   _VKI_IOWR(VKI_SYNC_IOC_MAGIC, 1, struct vki_sync_merge_data)

#define VKI_SYNC_IOC_FENCE_INFO \
   _VKI_IOWR(VKI_SYNC_IOC_MAGIC, 2, struct vki_sync_fence_info_data)

//----------------------------------------------------------------------
// From drivers/staging/lustre/lustre/include/lustre/lustre_user.h
//----------------------------------------------------------------------

struct vki_lu_fid {
	__vki_u64	f_seq;
	__vki_u32	f_oid;
	__vki_u32	f_ver;
};

//----------------------------------------------------------------------
// From drivers/staging/lustre/lustre/include/lustre/lustre_idl.h
//----------------------------------------------------------------------

struct vki_getinfo_fid2path {
	struct vki_lu_fid	gf_fid;
	__vki_u64		gf_recno;
	__vki_u32		gf_linkno;
	__vki_u32		gf_pathlen;
	char			gf_path[0];
} __attribute__((packed));

//----------------------------------------------------------------------
// From drivers/staging/lustre/lustre/include/linux/lustre_lib.h
//----------------------------------------------------------------------

#define VKI_OBD_IOC_DATA_TYPE               long

//----------------------------------------------------------------------
// From drivers/staging/lustre/lustre/include/lustre_lib.h
//----------------------------------------------------------------------

#define VKI_OBD_IOC_FID2PATH \
           _VKI_IOWR ('f', 150, VKI_OBD_IOC_DATA_TYPE)
#define VKI_LL_IOC_PATH2FID \
           _VKI_IOR ('f', 173, long)

//----------------------------------------------------------------------
// From lustre/include/lustre/lustre_idl.h
//----------------------------------------------------------------------

struct vki_getparent {
    struct vki_lu_fid   gp_fid;
    __vki_u32       gp_linkno;
    __vki_u32       gp_name_size;
    char            gp_name[0];
} __attribute__((packed));

//----------------------------------------------------------------------
// From Lustre's lustre/include/lustre/lustre_user.h
//----------------------------------------------------------------------
#define VKI_LL_IOC_GROUP_LOCK \
           _VKI_IOW('f', 158, long)
#define VKI_LL_IOC_GROUP_UNLOCK \
           _VKI_IOW('f', 159, long)
#define VKI_LL_IOC_GETPARENT \
           _VKI_IOWR('f', 249, struct vki_getparent)


struct vki_v4l2_rect {
	__vki_s32   left;
	__vki_s32   top;
	__vki_u32   width;
	__vki_u32   height;
};

struct vki_v4l2_fract {
	__vki_u32   numerator;
	__vki_u32   denominator;
};

struct vki_v4l2_capability {
	__vki_u8	driver[16];
	__vki_u8	card[32];
	__vki_u8	bus_info[32];
	__vki_u32   version;
	__vki_u32	capabilities;
	__vki_u32	device_caps;
	__vki_u32	reserved[3];
};

struct vki_v4l2_pix_format {
	__vki_u32         		width;
	__vki_u32			height;
	__vki_u32			pixelformat;
	__vki_u32			field;		/* enum vki_v4l2_field */
	__vki_u32            	bytesperline;	/* for padding, zero if unused */
	__vki_u32          		sizeimage;
	__vki_u32			colorspace;	/* enum vki_v4l2_colorspace */
	__vki_u32			priv;		/* private data, depends on pixelformat */
	__vki_u32			flags;		/* format flags (VKI_V4L2_PIX_FMT_FLAG_*) */
	__vki_u32			ycbcr_enc;
	__vki_u32			quantization;
};

struct vki_v4l2_fmtdesc {
	__vki_u32		    index;             /* Format number      */
	__vki_u32		    type;              /* enum vki_v4l2_buf_type */
	__vki_u32               flags;
	__vki_u8		    description[32];   /* Description string */
	__vki_u32		    pixelformat;       /* Format fourcc      */
	__vki_u32		    reserved[4];
};

struct vki_v4l2_frmsize_discrete {
	__vki_u32			width;		/* Frame width [pixel] */
	__vki_u32			height;		/* Frame height [pixel] */
};

struct vki_v4l2_frmsize_stepwise {
	__vki_u32			min_width;	/* Minimum frame width [pixel] */
	__vki_u32			max_width;	/* Maximum frame width [pixel] */
	__vki_u32			step_width;	/* Frame width step size [pixel] */
	__vki_u32			min_height;	/* Minimum frame height [pixel] */
	__vki_u32			max_height;	/* Maximum frame height [pixel] */
	__vki_u32			step_height;	/* Frame height step size [pixel] */
};

struct vki_v4l2_frmsizeenum {
	__vki_u32			index;		/* Frame size number */
	__vki_u32			pixel_format;	/* Pixel format */
	__vki_u32			type;		/* Frame size type the device supports. */

	union {					/* Frame size */
		struct vki_v4l2_frmsize_discrete	discrete;
		struct vki_v4l2_frmsize_stepwise	stepwise;
	};

	__vki_u32   reserved[2];			/* Reserved space for future use */
};

struct vki_v4l2_frmival_stepwise {
	struct vki_v4l2_fract	min;
	struct vki_v4l2_fract	max;
	struct vki_v4l2_fract	step;
};

struct vki_v4l2_frmivalenum {
	__vki_u32			index;
	__vki_u32			pixel_format;
	__vki_u32			width;
	__vki_u32			height;
	__vki_u32			type;

	union {
		struct vki_v4l2_fract		discrete;
		struct vki_v4l2_frmival_stepwise	stepwise;
	};

	__vki_u32	reserved[2];
};

struct vki_v4l2_timecode {
	__vki_u32	type;
	__vki_u32	flags;
	__vki_u8	frames;
	__vki_u8	seconds;
	__vki_u8	minutes;
	__vki_u8	hours;
	__vki_u8	userbits[4];
};

struct vki_v4l2_jpegcompression {
	int quality;
	int  APPn;
	int  APP_len;
	char APP_data[60];
	int  COM_len;
	char COM_data[60];
	__vki_u32 jpeg_markers;
};

struct vki_v4l2_requestbuffers {
	__vki_u32			count;
	__vki_u32			type;
	__vki_u32			memory;
	__vki_u32			reserved[2];
};

struct vki_v4l2_plane {
	__vki_u32			bytesused;
	__vki_u32			length;
	union {
		__vki_u32		mem_offset;
		unsigned long	userptr;
		__vki_s32		fd;
	} m;
	__vki_u32			data_offset;
	__vki_u32			reserved[11];
};

#define VKI_V4L2_MEMORY_MMAP             1
#define VKI_V4L2_MEMORY_DMABUF           4
#define VKI_V4L2_BUF_FLAG_TIMESTAMP_MASK		0x0000e000
#define VKI_V4L2_BUF_FLAG_TIMESTAMP_COPY		0x00004000
struct vki_v4l2_buffer {
	__vki_u32			index;
	__vki_u32			type;
	__vki_u32			bytesused;
	__vki_u32			flags;
	__vki_u32			field;
	struct vki_timeval		timestamp;
	struct vki_v4l2_timecode	timecode;
	__vki_u32			sequence;

	/* memory location */
	__vki_u32			memory;
	union {
		__vki_u32           offset;
		unsigned long   userptr;
		struct vki_v4l2_plane *planes;
		__vki_s32		fd;
	} m;
	__vki_u32			length;
	__vki_u32			reserved2;
	__vki_u32			reserved;
};

struct vki_v4l2_exportbuffer {
	__vki_u32		type; /* enum vki_v4l2_buf_type */
	__vki_u32		index;
	__vki_u32		plane;
	__vki_u32		flags;
	__vki_s32		fd;
	__vki_u32		reserved[11];
};

struct vki_v4l2_framebuffer {
	__vki_u32			capability;
	__vki_u32			flags;
	void                    *base;
	struct {
		__vki_u32		width;
		__vki_u32		height;
		__vki_u32		pixelformat;
		__vki_u32		field;		/* enum vki_v4l2_field */
		__vki_u32		bytesperline;	/* for padding, zero if unused */
		__vki_u32		sizeimage;
		__vki_u32		colorspace;	/* enum vki_v4l2_colorspace */
		__vki_u32		priv;		/* reserved field, set to 0 */
	} fmt;
};

struct vki_v4l2_clip {
	struct vki_v4l2_rect        c;
	struct vki_v4l2_clip	__user *next;
};

struct vki_v4l2_window {
	struct vki_v4l2_rect        w;
	__vki_u32			field;	 /* enum vki_v4l2_field */
	__vki_u32			chromakey;
	struct vki_v4l2_clip	__user *clips;
	__vki_u32			clipcount;
	void			__user *bitmap;
	__vki_u8                    global_alpha;
};

struct vki_v4l2_captureparm {
	__vki_u32		   capability;	  /*  Supported modes */
	__vki_u32		   capturemode;	  /*  Current mode */
	struct vki_v4l2_fract  timeperframe;  /*  Time per frame in seconds */
	__vki_u32		   extendedmode;  /*  Driver-specific extensions */
	__vki_u32              readbuffers;   /*  # of buffers for read */
	__vki_u32		   reserved[4];
};

struct vki_v4l2_outputparm {
	__vki_u32		   capability;	 /*  Supported modes */
	__vki_u32		   outputmode;	 /*  Current mode */
	struct vki_v4l2_fract  timeperframe; /*  Time per frame in seconds */
	__vki_u32		   extendedmode; /*  Driver-specific extensions */
	__vki_u32              writebuffers; /*  # of buffers for write */
	__vki_u32		   reserved[4];
};

struct vki_v4l2_cropcap {
	__vki_u32			type;	/* enum vki_v4l2_buf_type */
	struct vki_v4l2_rect        bounds;
	struct vki_v4l2_rect        defrect;
	struct vki_v4l2_fract       pixelaspect;
};

struct vki_v4l2_crop {
	__vki_u32			type;	/* enum vki_v4l2_buf_type */
	struct vki_v4l2_rect        c;
};

struct vki_v4l2_selection {
	__vki_u32			type;
	__vki_u32			target;
	__vki_u32                   flags;
	struct vki_v4l2_rect        r;
	__vki_u32                   reserved[9];
};

typedef __vki_u64 vki_v4l2_std_id;

struct vki_v4l2_standard {
	__vki_u32		     index;
	vki_v4l2_std_id          id;
	__vki_u8		     name[24];
	struct vki_v4l2_fract    frameperiod; /* Frames, not fields */
	__vki_u32		     framelines;
	__vki_u32		     reserved[4];
};

struct vki_v4l2_bt_timings {
	__vki_u32	width;
	__vki_u32	height;
	__vki_u32	interlaced;
	__vki_u32	polarities;
	__vki_u64	pixelclock;
	__vki_u32	hfrontporch;
	__vki_u32	hsync;
	__vki_u32	hbackporch;
	__vki_u32	vfrontporch;
	__vki_u32	vsync;
	__vki_u32	vbackporch;
	__vki_u32	il_vfrontporch;
	__vki_u32	il_vsync;
	__vki_u32	il_vbackporch;
	__vki_u32	standards;
	__vki_u32	flags;
	__vki_u32	reserved[14];
} __attribute__ ((packed));

struct vki_v4l2_dv_timings {
	__vki_u32 type;
	union {
		struct vki_v4l2_bt_timings	bt;
		__vki_u32	reserved[32];
	};
} __attribute__ ((packed));

struct vki_v4l2_enum_dv_timings {
	__vki_u32 index;
	__vki_u32 pad;
	__vki_u32 reserved[2];
	struct vki_v4l2_dv_timings timings;
};

struct vki_v4l2_bt_timings_cap {
	__vki_u32	min_width;
	__vki_u32	max_width;
	__vki_u32	min_height;
	__vki_u32	max_height;
	__vki_u64	min_pixelclock;
	__vki_u64	max_pixelclock;
	__vki_u32	standards;
	__vki_u32	capabilities;
	__vki_u32	reserved[16];
} __attribute__ ((packed));

struct vki_v4l2_dv_timings_cap {
	__vki_u32 type;
	__vki_u32 pad;
	__vki_u32 reserved[2];
	union {
		struct vki_v4l2_bt_timings_cap bt;
		__vki_u32 raw_data[32];
	};
};

struct vki_v4l2_input {
	__vki_u32	     index;		/*  Which input */
	__vki_u8	     name[32];		/*  Label */
	__vki_u32	     type;		/*  Type of input */
	__vki_u32	     audioset;		/*  Associated audios (bitfield) */
	__vki_u32        tuner;             /*  enum vki_v4l2_tuner_type */
	vki_v4l2_std_id  std;
	__vki_u32	     status;
	__vki_u32	     capabilities;
	__vki_u32	     reserved[3];
};

struct vki_v4l2_output {
	__vki_u32	     index;		/*  Which output */
	__vki_u8	     name[32];		/*  Label */
	__vki_u32	     type;		/*  Type of output */
	__vki_u32	     audioset;		/*  Associated audios (bitfield) */
	__vki_u32	     modulator;         /*  Associated modulator */
	vki_v4l2_std_id  std;
	__vki_u32	     capabilities;
	__vki_u32	     reserved[3];
};

struct vki_v4l2_control {
	__vki_u32		     id;
	__vki_s32		     value;
};

struct vki_v4l2_ext_control {
	__vki_u32 id;
	__vki_u32 size;
	__vki_u32 reserved2[1];
	union {
		__vki_s32 value;
		__vki_s64 value64;
		char *string;
		__vki_u8 *p_u8;
		__vki_u16 *p_u16;
		__vki_u32 *p_u32;
		void *ptr;
	};
} __attribute__ ((packed));

struct vki_v4l2_ext_controls {
	__vki_u32 ctrl_class;
	__vki_u32 count;
	__vki_u32 error_idx;
	__vki_u32 reserved[2];
	struct vki_v4l2_ext_control *controls;
};

struct vki_v4l2_queryctrl {
	__vki_u32		     id;
	__vki_u32		     type;	/* enum vki_v4l2_ctrl_type */
	__vki_u8		     name[32];	/* Whatever */
	__vki_s32		     minimum;	/* Note signedness */
	__vki_s32		     maximum;
	__vki_s32		     step;
	__vki_s32		     default_value;
	__vki_u32                flags;
	__vki_u32		     reserved[2];
};

#define VKI_V4L2_CTRL_MAX_DIMS	  (4)
struct vki_v4l2_query_ext_ctrl {
	__vki_u32		     id;
	__vki_u32		     type;
	char		     name[32];
	__vki_s64		     minimum;
	__vki_s64		     maximum;
	__vki_u64		     step;
	__vki_s64		     default_value;
	__vki_u32                flags;
	__vki_u32                elem_size;
	__vki_u32                elems;
	__vki_u32                nr_of_dims;
	__vki_u32                dims[VKI_V4L2_CTRL_MAX_DIMS];
	__vki_u32		     reserved[32];
};

struct vki_v4l2_querymenu {
	__vki_u32		id;
	__vki_u32		index;
	union {
		__vki_u8	name[32];	/* Whatever */
		__vki_s64	value;
	};
	__vki_u32		reserved;
} __attribute__ ((packed));

struct vki_v4l2_tuner {
	__vki_u32                   index;
	__vki_u8			name[32];
	__vki_u32			type;	/* enum vki_v4l2_tuner_type */
	__vki_u32			capability;
	__vki_u32			rangelow;
	__vki_u32			rangehigh;
	__vki_u32			rxsubchans;
	__vki_u32			audmode;
	__vki_s32			signal;
	__vki_s32			afc;
	__vki_u32			reserved[4];
};

struct vki_v4l2_modulator {
	__vki_u32			index;
	__vki_u8			name[32];
	__vki_u32			capability;
	__vki_u32			rangelow;
	__vki_u32			rangehigh;
	__vki_u32			txsubchans;
	__vki_u32			reserved[4];
};

struct vki_v4l2_frequency {
	__vki_u32	tuner;
	__vki_u32	type;	/* enum vki_v4l2_tuner_type */
	__vki_u32	frequency;
	__vki_u32	reserved[8];
};

struct vki_v4l2_frequency_band {
	__vki_u32	tuner;
	__vki_u32	type;	/* enum vki_v4l2_tuner_type */
	__vki_u32	index;
	__vki_u32	capability;
	__vki_u32	rangelow;
	__vki_u32	rangehigh;
	__vki_u32	modulation;
	__vki_u32	reserved[9];
};

struct vki_v4l2_hw_freq_seek {
	__vki_u32	tuner;
	__vki_u32	type;	/* enum vki_v4l2_tuner_type */
	__vki_u32	seek_upward;
	__vki_u32	wrap_around;
	__vki_u32	spacing;
	__vki_u32	rangelow;
	__vki_u32	rangehigh;
	__vki_u32	reserved[5];
};

struct vki_v4l2_audio {
	__vki_u32	index;
	__vki_u8	name[32];
	__vki_u32	capability;
	__vki_u32	mode;
	__vki_u32	reserved[2];
};

struct vki_v4l2_audioout {
	__vki_u32	index;
	__vki_u8	name[32];
	__vki_u32	capability;
	__vki_u32	mode;
	__vki_u32	reserved[2];
};

struct vki_v4l2_enc_idx_entry {
	__vki_u64 offset;
	__vki_u64 pts;
	__vki_u32 length;
	__vki_u32 flags;
	__vki_u32 reserved[2];
};

#define VKI_V4L2_ENC_IDX_ENTRIES (64)
struct vki_v4l2_enc_idx {
	__vki_u32 entries;
	__vki_u32 entries_cap;
	__vki_u32 reserved[4];
	struct vki_v4l2_enc_idx_entry entry[VKI_V4L2_ENC_IDX_ENTRIES];
};

struct vki_v4l2_encoder_cmd {
	__vki_u32 cmd;
	__vki_u32 flags;
	union {
		struct {
			__vki_u32 data[8];
		} raw;
	};
};

struct vki_v4l2_decoder_cmd {
	__vki_u32 cmd;
	__vki_u32 flags;
	union {
		struct {
			__vki_u64 pts;
		} stop;

		struct {
			__vki_s32 speed;
			__vki_u32 format;
		} start;

		struct {
			__vki_u32 data[16];
		} raw;
	};
};

struct vki_v4l2_vbi_format {
	__vki_u32	sampling_rate;		/* in 1 Hz */
	__vki_u32	offset;
	__vki_u32	samples_per_line;
	__vki_u32	sample_format;		/* VKI_V4L2_PIX_FMT_* */
	__vki_s32	start[2];
	__vki_u32	count[2];
	__vki_u32	flags;			/* VKI_V4L2_VBI_* */
	__vki_u32	reserved[2];		/* must be zero */
};

struct vki_v4l2_sliced_vbi_format {
	__vki_u16   service_set;
	__vki_u16   service_lines[2][24];
	__vki_u32   io_size;
	__vki_u32   reserved[2];            /* must be zero */
};

struct vki_v4l2_sliced_vbi_cap {
	__vki_u16   service_set;
	__vki_u16   service_lines[2][24];
	__vki_u32	type;		/* enum vki_v4l2_buf_type */
	__vki_u32   reserved[3];    /* must be 0 */
};

struct vki_v4l2_sliced_vbi_data {
	__vki_u32   id;
	__vki_u32   field;          /* 0: first field, 1: second field */
	__vki_u32   line;           /* 1-23 */
	__vki_u32   reserved;       /* must be 0 */
	__vki_u8    data[48];
};

struct vki_v4l2_plane_pix_format {
	__vki_u32		sizeimage;
	__vki_u32		bytesperline;
	__vki_u16		reserved[6];
} __attribute__ ((packed));

#define VKI_VIDEO_MAX_PLANES               8

struct vki_v4l2_pix_format_mplane {
	__vki_u32				width;
	__vki_u32				height;
	__vki_u32				pixelformat;
	__vki_u32				field;
	__vki_u32				colorspace;

	struct vki_v4l2_plane_pix_format	plane_fmt[VKI_VIDEO_MAX_PLANES];
	__vki_u8				num_planes;
	__vki_u8				flags;
	__vki_u8				ycbcr_enc;
	__vki_u8				quantization;
	__vki_u8				reserved[8];
} __attribute__ ((packed));

struct vki_v4l2_sdr_format {
	__vki_u32				pixelformat;
	__vki_u32				buffersize;
	__vki_u8				reserved[24];
} __attribute__ ((packed));

enum vki_v4l2_buf_type {
	VKI_V4L2_BUF_TYPE_VIDEO_CAPTURE        = 1,
	VKI_V4L2_BUF_TYPE_VIDEO_OUTPUT         = 2,
	VKI_V4L2_BUF_TYPE_VIDEO_OVERLAY        = 3,
	VKI_V4L2_BUF_TYPE_VBI_CAPTURE          = 4,
	VKI_V4L2_BUF_TYPE_VBI_OUTPUT           = 5,
	VKI_V4L2_BUF_TYPE_SLICED_VBI_CAPTURE   = 6,
	VKI_V4L2_BUF_TYPE_SLICED_VBI_OUTPUT    = 7,
	VKI_V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY = 8,
	VKI_V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE = 9,
	VKI_V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE  = 10,
	VKI_V4L2_BUF_TYPE_SDR_CAPTURE          = 11,
};

struct vki_v4l2_format {
	__vki_u32	 type;
	union {
		struct vki_v4l2_pix_format		pix;
		struct vki_v4l2_pix_format_mplane	pix_mp;
		struct vki_v4l2_window		win;
		struct vki_v4l2_vbi_format		vbi;
		struct vki_v4l2_sliced_vbi_format	sliced;
		struct vki_v4l2_sdr_format		sdr;
		__vki_u8	raw_data[200];
	} fmt;
};

struct vki_v4l2_streamparm {
	__vki_u32	 type;
	union {
		struct vki_v4l2_captureparm	capture;
		struct vki_v4l2_outputparm	output;
		__vki_u8	raw_data[200];  /* user-defined */
	} parm;
};

struct vki_v4l2_event_vsync {
	__vki_u8 field;
} __attribute__ ((packed));

struct vki_v4l2_event_ctrl {
	__vki_u32 changes;
	__vki_u32 type;
	union {
		__vki_s32 value;
		__vki_s64 value64;
	};
	__vki_u32 flags;
	__vki_s32 minimum;
	__vki_s32 maximum;
	__vki_s32 step;
	__vki_s32 default_value;
};

struct vki_v4l2_event_frame_sync {
	__vki_u32 frame_sequence;
};

struct vki_v4l2_event_src_change {
	__vki_u32 changes;
};

struct vki_v4l2_event_motion_det {
	__vki_u32 flags;
	__vki_u32 frame_sequence;
	__vki_u32 region_mask;
};

struct vki_v4l2_event {
	__vki_u32				type;
	union {
		struct vki_v4l2_event_vsync		vsync;
		struct vki_v4l2_event_ctrl		ctrl;
		struct vki_v4l2_event_frame_sync	frame_sync;
		struct vki_v4l2_event_src_change	src_change;
		struct vki_v4l2_event_motion_det	motion_det;
		__vki_u8				data[64];
	} u;
	__vki_u32				pending;
	__vki_u32				sequence;
	struct vki_timespec			timestamp;
	__vki_u32				id;
	__vki_u32				reserved[8];
};

struct vki_v4l2_event_subscription {
	__vki_u32				type;
	__vki_u32				id;
	__vki_u32				flags;
	__vki_u32				reserved[5];
};

struct vki_v4l2_dbg_match {
	__vki_u32 type; /* Match type */
	union {     /* Match this chip, meaning determined by type */
		__vki_u32 addr;
		char name[32];
	};
} __attribute__ ((packed));

struct vki_v4l2_dbg_register {
	struct vki_v4l2_dbg_match match;
	__vki_u32 size;	/* register size in bytes */
	__vki_u64 reg;
	__vki_u64 val;
} __attribute__ ((packed));

struct vki_v4l2_dbg_chip_info {
	struct vki_v4l2_dbg_match match;
	char name[32];
	__vki_u32 flags;
	__vki_u32 reserved[32];
} __attribute__ ((packed));

struct vki_v4l2_create_buffers {
	__vki_u32			index;
	__vki_u32			count;
	__vki_u32			memory;
	struct vki_v4l2_format	format;
	__vki_u32			reserved[8];
};

struct vki_v4l2_edid {
	__vki_u32 pad;
	__vki_u32 start_block;
	__vki_u32 blocks;
	__vki_u32 reserved[5];
	__vki_u8  *edid;
};

#define VKI_V4L2_QUERYCAP		_VKI_IOR('V',  0, struct vki_v4l2_capability)
#define VKI_V4L2_ENUM_FMT		_VKI_IOWR('V',  2, struct vki_v4l2_fmtdesc)
#define VKI_V4L2_G_FMT			_VKI_IOWR('V',  4, struct vki_v4l2_format)
#define VKI_V4L2_S_FMT			_VKI_IOWR('V',  5, struct vki_v4l2_format)
#define VKI_V4L2_REQBUFS		_VKI_IOWR('V',  8, struct vki_v4l2_requestbuffers)
#define VKI_V4L2_QUERYBUF		_VKI_IOWR('V',  9, struct vki_v4l2_buffer)
#define VKI_V4L2_G_FBUF		 	_VKI_IOR('V', 10, struct vki_v4l2_framebuffer)
#define VKI_V4L2_S_FBUF		 	_VKI_IOW('V', 11, struct vki_v4l2_framebuffer)
#define VKI_V4L2_OVERLAY	 	_VKI_IOW('V', 14, int)
#define VKI_V4L2_QBUF			_VKI_IOWR('V', 15, struct vki_v4l2_buffer)
#define VKI_V4L2_EXPBUF			_VKI_IOWR('V', 16, struct vki_v4l2_exportbuffer)
#define VKI_V4L2_DQBUF			_VKI_IOWR('V', 17, struct vki_v4l2_buffer)
#define VKI_V4L2_STREAMON	 	_VKI_IOW('V', 18, int)
#define VKI_V4L2_STREAMOFF	 	_VKI_IOW('V', 19, int)
#define VKI_V4L2_G_PARM			_VKI_IOWR('V', 21, struct vki_v4l2_streamparm)
#define VKI_V4L2_S_PARM			_VKI_IOWR('V', 22, struct vki_v4l2_streamparm)
#define VKI_V4L2_G_STD			_VKI_IOR('V', 23, vki_v4l2_std_id)
#define VKI_V4L2_S_STD			_VKI_IOW('V', 24, vki_v4l2_std_id)
#define VKI_V4L2_ENUMSTD		_VKI_IOWR('V', 25, struct vki_v4l2_standard)
#define VKI_V4L2_ENUMINPUT		_VKI_IOWR('V', 26, struct vki_v4l2_input)
#define VKI_V4L2_G_CTRL			_VKI_IOWR('V', 27, struct vki_v4l2_control)
#define VKI_V4L2_S_CTRL			_VKI_IOWR('V', 28, struct vki_v4l2_control)
#define VKI_V4L2_G_TUNER		_VKI_IOWR('V', 29, struct vki_v4l2_tuner)
#define VKI_V4L2_S_TUNER		_VKI_IOW('V', 30, struct vki_v4l2_tuner)
#define VKI_V4L2_G_AUDIO		_VKI_IOR('V', 33, struct vki_v4l2_audio)
#define VKI_V4L2_S_AUDIO		_VKI_IOW('V', 34, struct vki_v4l2_audio)
#define VKI_V4L2_QUERYCTRL		_VKI_IOWR('V', 36, struct vki_v4l2_queryctrl)
#define VKI_V4L2_QUERYMENU		_VKI_IOWR('V', 37, struct vki_v4l2_querymenu)
#define VKI_V4L2_G_INPUT		_VKI_IOR('V', 38, int)
#define VKI_V4L2_S_INPUT		_VKI_IOWR('V', 39, int)
#define VKI_V4L2_G_EDID			_VKI_IOWR('V', 40, struct vki_v4l2_edid)
#define VKI_V4L2_S_EDID			_VKI_IOWR('V', 41, struct vki_v4l2_edid)
#define VKI_V4L2_G_OUTPUT		_VKI_IOR('V', 46, int)
#define VKI_V4L2_S_OUTPUT		_VKI_IOWR('V', 47, int)
#define VKI_V4L2_ENUMOUTPUT		_VKI_IOWR('V', 48, struct vki_v4l2_output)
#define VKI_V4L2_G_AUDOUT		_VKI_IOR('V', 49, struct vki_v4l2_audioout)
#define VKI_V4L2_S_AUDOUT		_VKI_IOW('V', 50, struct vki_v4l2_audioout)
#define VKI_V4L2_G_MODULATOR		_VKI_IOWR('V', 54, struct vki_v4l2_modulator)
#define VKI_V4L2_S_MODULATOR		_VKI_IOW('V', 55, struct vki_v4l2_modulator)
#define VKI_V4L2_G_FREQUENCY		_VKI_IOWR('V', 56, struct vki_v4l2_frequency)
#define VKI_V4L2_S_FREQUENCY		_VKI_IOW('V', 57, struct vki_v4l2_frequency)
#define VKI_V4L2_CROPCAP		_VKI_IOWR('V', 58, struct vki_v4l2_cropcap)
#define VKI_V4L2_G_CROP			_VKI_IOWR('V', 59, struct vki_v4l2_crop)
#define VKI_V4L2_S_CROP			_VKI_IOW('V', 60, struct vki_v4l2_crop)
#define VKI_V4L2_G_JPEGCOMP		_VKI_IOR('V', 61, struct vki_v4l2_jpegcompression)
#define VKI_V4L2_S_JPEGCOMP		_VKI_IOW('V', 62, struct vki_v4l2_jpegcompression)
#define VKI_V4L2_QUERYSTD      		_VKI_IOR('V', 63, vki_v4l2_std_id)
#define VKI_V4L2_TRY_FMT      		_VKI_IOWR('V', 64, struct vki_v4l2_format)
#define VKI_V4L2_ENUMAUDIO		_VKI_IOWR('V', 65, struct vki_v4l2_audio)
#define VKI_V4L2_ENUMAUDOUT		_VKI_IOWR('V', 66, struct vki_v4l2_audioout)
#define VKI_V4L2_G_PRIORITY		_VKI_IOR('V', 67, __vki_u32)
#define VKI_V4L2_S_PRIORITY		_VKI_IOW('V', 68, __vki_u32)
#define VKI_V4L2_G_SLICED_VBI_CAP 	_VKI_IOWR('V', 69, struct vki_v4l2_sliced_vbi_cap)
#define VKI_V4L2_LOG_STATUS     	_VKI_IO('V', 70)
#define VKI_V4L2_G_EXT_CTRLS		_VKI_IOWR('V', 71, struct vki_v4l2_ext_controls)
#define VKI_V4L2_S_EXT_CTRLS		_VKI_IOWR('V', 72, struct vki_v4l2_ext_controls)
#define VKI_V4L2_TRY_EXT_CTRLS		_VKI_IOWR('V', 73, struct vki_v4l2_ext_controls)
#define VKI_V4L2_ENUM_FRAMESIZES	_VKI_IOWR('V', 74, struct vki_v4l2_frmsizeenum)
#define VKI_V4L2_ENUM_FRAMEINTERVALS 	_VKI_IOWR('V', 75, struct vki_v4l2_frmivalenum)
#define VKI_V4L2_G_ENC_INDEX    	_VKI_IOR('V', 76, struct vki_v4l2_enc_idx)
#define VKI_V4L2_ENCODER_CMD    	_VKI_IOWR('V', 77, struct vki_v4l2_encoder_cmd)
#define VKI_V4L2_TRY_ENCODER_CMD 	_VKI_IOWR('V', 78, struct vki_v4l2_encoder_cmd)
#define	VKI_V4L2_DBG_S_REGISTER 	_VKI_IOW('V', 79, struct vki_v4l2_dbg_register)
#define	VKI_V4L2_DBG_G_REGISTER 	_VKI_IOWR('V', 80, struct vki_v4l2_dbg_register)
#define VKI_V4L2_S_HW_FREQ_SEEK		_VKI_IOW('V', 82, struct vki_v4l2_hw_freq_seek)
#define	VKI_V4L2_S_DV_TIMINGS		_VKI_IOWR('V', 87, struct vki_v4l2_dv_timings)
#define	VKI_V4L2_G_DV_TIMINGS		_VKI_IOWR('V', 88, struct vki_v4l2_dv_timings)
#define	VKI_V4L2_DQEVENT		_VKI_IOR('V', 89, struct vki_v4l2_event)
#define	VKI_V4L2_SUBSCRIBE_EVENT	_VKI_IOW('V', 90, struct vki_v4l2_event_subscription)
#define	VKI_V4L2_UNSUBSCRIBE_EVENT 	_VKI_IOW('V', 91, struct vki_v4l2_event_subscription)
#define VKI_V4L2_CREATE_BUFS		_VKI_IOWR('V', 92, struct vki_v4l2_create_buffers)
#define VKI_V4L2_PREPARE_BUF		_VKI_IOWR('V', 93, struct vki_v4l2_buffer)
#define VKI_V4L2_G_SELECTION		_VKI_IOWR('V', 94, struct vki_v4l2_selection)
#define VKI_V4L2_S_SELECTION		_VKI_IOWR('V', 95, struct vki_v4l2_selection)
#define VKI_V4L2_DECODER_CMD		_VKI_IOWR('V', 96, struct vki_v4l2_decoder_cmd)
#define VKI_V4L2_TRY_DECODER_CMD	_VKI_IOWR('V', 97, struct vki_v4l2_decoder_cmd)
#define VKI_V4L2_ENUM_DV_TIMINGS 	_VKI_IOWR('V', 98, struct vki_v4l2_enum_dv_timings)
#define VKI_V4L2_QUERY_DV_TIMINGS 	_VKI_IOR('V', 99, struct vki_v4l2_dv_timings)
#define VKI_V4L2_DV_TIMINGS_CAP   	_VKI_IOWR('V', 100, struct vki_v4l2_dv_timings_cap)
#define VKI_V4L2_ENUM_FREQ_BANDS	_VKI_IOWR('V', 101, struct vki_v4l2_frequency_band)
#define VKI_V4L2_DBG_G_CHIP_INFO 	_VKI_IOWR('V', 102, struct vki_v4l2_dbg_chip_info)
#define VKI_V4L2_QUERY_EXT_CTRL		_VKI_IOWR('V', 103, struct vki_v4l2_query_ext_ctrl)

struct vki_v4l2_mbus_framefmt {
	__vki_u32			width;
	__vki_u32			height;
	__vki_u32			code;
	__vki_u32			field;
	__vki_u32			colorspace;
	__vki_u16			ycbcr_enc;
	__vki_u16			quantization;
	__vki_u32			reserved[6];
};

struct vki_v4l2_subdev_format {
	__vki_u32 which;
	__vki_u32 pad;
	struct vki_v4l2_mbus_framefmt format;
	__vki_u32 reserved[8];
};

struct vki_v4l2_subdev_crop {
	__vki_u32 which;
	__vki_u32 pad;
	struct vki_v4l2_rect rect;
	__vki_u32 reserved[8];
};

struct vki_v4l2_subdev_mbus_code_enum {
	__vki_u32 pad;
	__vki_u32 index;
	__vki_u32 code;
	__vki_u32 which;
	__vki_u32 reserved[8];
};

struct vki_v4l2_subdev_frame_size_enum {
	__vki_u32 index;
	__vki_u32 pad;
	__vki_u32 code;
	__vki_u32 min_width;
	__vki_u32 max_width;
	__vki_u32 min_height;
	__vki_u32 max_height;
	__vki_u32 which;
	__vki_u32 reserved[8];
};

struct vki_v4l2_subdev_frame_interval {
	__vki_u32 pad;
	struct vki_v4l2_fract interval;
	__vki_u32 reserved[9];
};

struct vki_v4l2_subdev_frame_interval_enum {
	__vki_u32 index;
	__vki_u32 pad;
	__vki_u32 code;
	__vki_u32 width;
	__vki_u32 height;
	struct vki_v4l2_fract interval;
	__vki_u32 which;
	__vki_u32 reserved[8];
};

struct vki_v4l2_subdev_selection {
	__vki_u32 which;
	__vki_u32 pad;
	__vki_u32 target;
	__vki_u32 flags;
	struct vki_v4l2_rect r;
	__vki_u32 reserved[8];
};

#define VKI_V4L2_SUBDEV_G_FMT			_VKI_IOWR('V',  4, struct vki_v4l2_subdev_format)
#define VKI_V4L2_SUBDEV_S_FMT			_VKI_IOWR('V',  5, struct vki_v4l2_subdev_format)
#define VKI_V4L2_SUBDEV_G_FRAME_INTERVAL	_VKI_IOWR('V', 21, struct vki_v4l2_subdev_frame_interval)
#define VKI_V4L2_SUBDEV_S_FRAME_INTERVAL	_VKI_IOWR('V', 22, struct vki_v4l2_subdev_frame_interval)
#define VKI_V4L2_SUBDEV_ENUM_MBUS_CODE		_VKI_IOWR('V',  2, struct vki_v4l2_subdev_mbus_code_enum)
#define VKI_V4L2_SUBDEV_ENUM_FRAME_SIZE		_VKI_IOWR('V', 74, struct vki_v4l2_subdev_frame_size_enum)
#define VKI_V4L2_SUBDEV_ENUM_FRAME_INTERVAL	_VKI_IOWR('V', 75, struct vki_v4l2_subdev_frame_interval_enum)
#define VKI_V4L2_SUBDEV_G_CROP			_VKI_IOWR('V', 59, struct vki_v4l2_subdev_crop)
#define VKI_V4L2_SUBDEV_S_CROP			_VKI_IOWR('V', 60, struct vki_v4l2_subdev_crop)
#define VKI_V4L2_SUBDEV_G_SELECTION		_VKI_IOWR('V', 61, struct vki_v4l2_subdev_selection)
#define VKI_V4L2_SUBDEV_S_SELECTION		_VKI_IOWR('V', 62, struct vki_v4l2_subdev_selection)

struct vki_media_device_info {
	char driver[16];
	char model[32];
	char serial[40];
	char bus_info[32];
	__vki_u32 media_version;
	__vki_u32 hw_revision;
	__vki_u32 driver_version;
	__vki_u32 reserved[31];
};

struct vki_media_entity_desc {
	__vki_u32 id;
	char name[32];
	__vki_u32 type;
	__vki_u32 revision;
	__vki_u32 flags;
	__vki_u32 group_id;
	__vki_u16 pads;
	__vki_u16 links;

	__vki_u32 reserved[4];

	union {
		/* Node specifications */
		struct {
			__vki_u32 major;
			__vki_u32 minor;
		} v4l;
		struct {
			__vki_u32 major;
			__vki_u32 minor;
		} fb;
		struct {
			__vki_u32 card;
			__vki_u32 device;
			__vki_u32 subdevice;
		} alsa;
		int dvb;

		/* Sub-device specifications */
		/* Nothing needed yet */
		__vki_u8 raw[184];
	};
};

struct vki_media_pad_desc {
	__vki_u32 entity;		/* entity ID */
	__vki_u16 index;		/* pad index */
	__vki_u32 flags;		/* pad flags */
	__vki_u32 reserved[2];
};

struct vki_media_link_desc {
	struct vki_media_pad_desc source;
	struct vki_media_pad_desc sink;
	__vki_u32 flags;
	__vki_u32 reserved[2];
};

struct vki_media_links_enum {
	__vki_u32 entity;
	struct vki_media_pad_desc __user *pads;
	struct vki_media_link_desc __user *links;
	__vki_u32 reserved[4];
};

#define VKI_MEDIA_IOC_DEVICE_INFO		_VKI_IOWR('|', 0x00, struct vki_media_device_info)
#define VKI_MEDIA_IOC_ENUM_ENTITIES		_VKI_IOWR('|', 0x01, struct vki_media_entity_desc)
#define VKI_MEDIA_IOC_ENUM_LINKS		_VKI_IOWR('|', 0x02, struct vki_media_links_enum)
#define VKI_MEDIA_IOC_SETUP_LINK		_VKI_IOWR('|', 0x03, struct vki_media_link_desc)

/* DVB demux API */
#define	VKI_DMX_STOP	_VKI_IO('o', 42)

/* Comparison type */
enum vki_kcmp_type {
   VKI_KCMP_FILE,
   VKI_KCMP_VM,
   VKI_KCMP_FILES,
   VKI_KCMP_FS,
   VKI_KCMP_SIGHAND,
   VKI_KCMP_IO,
   VKI_KCMP_SYSVSEM,

   VKI_KCMP_TYPES
};

//----------------------------------------------------------------------
// From linux-3.19-rc5/include/uapi/linux/seccomp.h
//----------------------------------------------------------------------

#define VKI_SECCOMP_MODE_FILTER 2

//----------------------------------------------------------------------
// From linux-3.19.3/include/uapi/linux/binfmts.h
//----------------------------------------------------------------------
#define VKI_BINPRM_BUF_SIZE 128

//----------------------------------------------------------------------
// From linux-3.19.0/include/linux/serial.h
//----------------------------------------------------------------------

struct vki_serial_struct {
	int	type;
	int	line;
	unsigned int	port;
	int	irq;
	int	flags;
	int	xmit_fifo_size;
	int	custom_divisor;
	int	baud_base;
	unsigned short	close_delay;
	char	io_type;
	char	reserved_char[1];
	int	hub6;
	unsigned short	closing_wait; /* time to wait before closing */
	unsigned short	closing_wait2; /* no longer used... */
	unsigned char	*iomem_base;
	unsigned short	iomem_reg_shift;
	unsigned int	port_high;
	unsigned long	iomap_base;	/* cookie passed into ioremap */
};

//----------------------------------------------------------------------
// From linux-3.19.0/fs/binfmt_elf.c
//----------------------------------------------------------------------

#if !defined(VKI_INIT_ARCH_ELF_STATE)
   /* This structure is used to preserve architecture specific data during
      the loading of an ELF file, throughout the checking of architecture
      specific ELF headers & through to the point where the ELF load is
      known to be proceeding. This implementation is a dummy for
      architectures which require no specific state. */
   struct vki_arch_elf_state {
   };

#  define VKI_INIT_ARCH_ELF_STATE { }

#endif

//----------------------------------------------------------------------
// From linux-4.0/include/uapi/linux/prctl.h
//----------------------------------------------------------------------

#define VKI_PR_SET_FP_MODE          45
#define VKI_PR_GET_FP_MODE          46
# define VKI_PR_FP_MODE_FR          (1 << 0)     /* 64b FP registers  */
# define VKI_PR_FP_MODE_FRE         (1 << 1)     /* 32b compatibility */

#endif // __VKI_LINUX_H

//----------------------------------------------------------------------
// From linux-4.10/include/uapi/linux/blkzoned.h
//----------------------------------------------------------------------

struct vki_blk_zone {
	__vki_u64	start;
	__vki_u64	len;
	__vki_u64	wp;
	__vki_u8	type;
	__vki_u8	cond;
	__vki_u8	non_seq;
	__vki_u8	reset;
	__vki_u8	reserved[36];
};

struct vki_blk_zone_report {
	__vki_u64		sector;
	__vki_u32		nr_zones;
	__vki_u8		reserved[4];
	struct vki_blk_zone	zones[0];
};

struct vki_blk_zone_range {
	__vki_u64		sector;
	__vki_u64		nr_sectors;
};

#define VKI_BLKREPORTZONE	_VKI_IOWR(0x12, 130, struct vki_blk_zone_report)
#define VKI_BLKRESETZONE	_VKI_IOW(0x12, 131, struct vki_blk_zone_range)

//----------------------------------------------------------------------
// From linux-4.18/include/uapi/linux/bpf.h
//----------------------------------------------------------------------

struct vki_bpf_insn {
	__vki_u8	code;		/* opcode */
	__vki_u8	dst_reg:4;	/* dest register */
	__vki_u8	src_reg:4;	/* source register */
	__vki_s16	off;		/* signed offset */
	__vki_s32	imm;		/* signed immediate constant */
};

enum vki_bpf_cmd {
	VKI_BPF_MAP_CREATE,
	VKI_BPF_MAP_LOOKUP_ELEM,
	VKI_BPF_MAP_UPDATE_ELEM,
	VKI_BPF_MAP_DELETE_ELEM,
	VKI_BPF_MAP_GET_NEXT_KEY,
	VKI_BPF_PROG_LOAD,
	VKI_BPF_OBJ_PIN,
	VKI_BPF_OBJ_GET,
	VKI_BPF_PROG_ATTACH,
	VKI_BPF_PROG_DETACH,
	VKI_BPF_PROG_TEST_RUN,
	VKI_BPF_PROG_GET_NEXT_ID,
	VKI_BPF_MAP_GET_NEXT_ID,
	VKI_BPF_PROG_GET_FD_BY_ID,
	VKI_BPF_MAP_GET_FD_BY_ID,
	VKI_BPF_OBJ_GET_INFO_BY_FD,
	VKI_BPF_PROG_QUERY,
	VKI_BPF_RAW_TRACEPOINT_OPEN,
	VKI_BPF_BTF_LOAD,
	VKI_BPF_BTF_GET_FD_BY_ID,
	VKI_BPF_TASK_FD_QUERY,
};

enum vki_bpf_map_type {
	VKI_BPF_MAP_TYPE_UNSPEC,
	VKI_BPF_MAP_TYPE_HASH,
	VKI_BPF_MAP_TYPE_ARRAY,
	VKI_BPF_MAP_TYPE_PROG_ARRAY,
	VKI_BPF_MAP_TYPE_PERF_EVENT_ARRAY,
	VKI_BPF_MAP_TYPE_PERCPU_HASH,
	VKI_BPF_MAP_TYPE_PERCPU_ARRAY,
	VKI_BPF_MAP_TYPE_STACK_TRACE,
	VKI_BPF_MAP_TYPE_CGROUP_ARRAY,
	VKI_BPF_MAP_TYPE_LRU_HASH,
	VKI_BPF_MAP_TYPE_LRU_PERCPU_HASH,
	VKI_BPF_MAP_TYPE_LPM_TRIE,
	VKI_BPF_MAP_TYPE_ARRAY_OF_MAPS,
	VKI_BPF_MAP_TYPE_HASH_OF_MAPS,
	VKI_BPF_MAP_TYPE_DEVMAP,
	VKI_BPF_MAP_TYPE_SOCKMAP,
	VKI_BPF_MAP_TYPE_CPUMAP,
	VKI_BPF_MAP_TYPE_XSKMAP,
	VKI_BPF_MAP_TYPE_SOCKHASH,
};

enum vki_bpf_prog_type {
	VKI_BPF_PROG_TYPE_UNSPEC,
	VKI_BPF_PROG_TYPE_SOCKET_FILTER,
	VKI_BPF_PROG_TYPE_KPROBE,
	VKI_BPF_PROG_TYPE_SCHED_CLS,
	VKI_BPF_PROG_TYPE_SCHED_ACT,
	VKI_BPF_PROG_TYPE_TRACEPOINT,
	VKI_BPF_PROG_TYPE_XDP,
	VKI_BPF_PROG_TYPE_PERF_EVENT,
	VKI_BPF_PROG_TYPE_CGROUP_SKB,
	VKI_BPF_PROG_TYPE_CGROUP_SOCK,
	VKI_BPF_PROG_TYPE_LWT_IN,
	VKI_BPF_PROG_TYPE_LWT_OUT,
	VKI_BPF_PROG_TYPE_LWT_XMIT,
	VKI_BPF_PROG_TYPE_SOCK_OPS,
	VKI_BPF_PROG_TYPE_SK_SKB,
	VKI_BPF_PROG_TYPE_CGROUP_DEVICE,
	VKI_BPF_PROG_TYPE_SK_MSG,
	VKI_BPF_PROG_TYPE_RAW_TRACEPOINT,
	VKI_BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
	VKI_BPF_PROG_TYPE_LWT_SEG6LOCAL,
	VKI_BPF_PROG_TYPE_LIRC_MODE2,
};

enum vki_bpf_attach_type {
	VKI_BPF_CGROUP_INET_INGRESS,
	VKI_BPF_CGROUP_INET_EGRESS,
	VKI_BPF_CGROUP_INET_SOCK_CREATE,
	VKI_BPF_CGROUP_SOCK_OPS,
	VKI_BPF_SK_SKB_STREAM_PARSER,
	VKI_BPF_SK_SKB_STREAM_VERDICT,
	VKI_BPF_CGROUP_DEVICE,
	VKI_BPF_SK_MSG_VERDICT,
	VKI_BPF_CGROUP_INET4_BIND,
	VKI_BPF_CGROUP_INET6_BIND,
	VKI_BPF_CGROUP_INET4_CONNECT,
	VKI_BPF_CGROUP_INET6_CONNECT,
	VKI_BPF_CGROUP_INET4_POST_BIND,
	VKI_BPF_CGROUP_INET6_POST_BIND,
	VKI_BPF_CGROUP_UDP4_SENDMSG,
	VKI_BPF_CGROUP_UDP6_SENDMSG,
	VKI_BPF_LIRC_MODE2,
	__VKI_MAX_BPF_ATTACH_TYPE
};

/* Specify numa node during map creation */
#define VKI_BPF_F_NUMA_NODE		(1U << 2)

#define VKI_BPF_OBJ_NAME_LEN 16U

union vki_bpf_attr {
	struct { /* anonymous struct used by BPF_MAP_CREATE command */
		__vki_u32	map_type;	/* one of enum bpf_map_type */
		__vki_u32	key_size;	/* size of key in bytes */
		__vki_u32	value_size;	/* size of value in bytes */
		__vki_u32	max_entries;	/* max number of entries in a map */
		__vki_u32	map_flags;	/* BPF_MAP_CREATE related
					 * flags defined above.
					 */
		__vki_u32	inner_map_fd;	/* fd pointing to the inner map */
		__vki_u32	numa_node;	/* numa node (effective only if
					 * BPF_F_NUMA_NODE is set).
					 */
		char	map_name[VKI_BPF_OBJ_NAME_LEN];
		__vki_u32	map_ifindex;	/* ifindex of netdev to create on */
		__vki_u32	btf_fd;		/* fd pointing to a BTF type data */
		__vki_u32	btf_key_type_id;	/* BTF type_id of the key */
		__vki_u32	btf_value_type_id;	/* BTF type_id of the value */
	};

	struct { /* anonymous struct used by BPF_MAP_*_ELEM commands */
		__vki_u32		map_fd;
		__vki_aligned_u64	key;
		union {
			__vki_aligned_u64 value;
			__vki_aligned_u64 next_key;
		};
		__vki_u64		flags;
	};

	struct { /* anonymous struct used by BPF_PROG_LOAD command */
		__vki_u32		prog_type;	/* one of enum bpf_prog_type */
		__vki_u32		insn_cnt;
		__vki_aligned_u64	insns;
		__vki_aligned_u64	license;
		__vki_u32		log_level;	/* verbosity level of verifier */
		__vki_u32		log_size;	/* size of user buffer */
		__vki_aligned_u64	log_buf;	/* user supplied buffer */
		__vki_u32		kern_version;	/* checked when prog_type=kprobe */
		__vki_u32		prog_flags;
		char		prog_name[VKI_BPF_OBJ_NAME_LEN];
		__vki_u32		prog_ifindex;	/* ifindex of netdev to prep for */
		/* For some prog types expected attach type must be known at
		 * load time to verify attach type specific parts of prog
		 * (context accesses, allowed helpers, etc).
		 */
		__vki_u32		expected_attach_type;
	};

	struct { /* anonymous struct used by BPF_OBJ_* commands */
		__vki_aligned_u64	pathname;
		__vki_u32		bpf_fd;
		__vki_u32		file_flags;
	};

	struct { /* anonymous struct used by BPF_PROG_ATTACH/DETACH commands */
		__vki_u32		target_fd;	/* container object to attach to */
		__vki_u32		attach_bpf_fd;	/* eBPF program to attach */
		__vki_u32		attach_type;
		__vki_u32		attach_flags;
	};

	struct { /* anonymous struct used by BPF_PROG_TEST_RUN command */
		__vki_u32		prog_fd;
		__vki_u32		retval;
		__vki_u32		data_size_in;
		__vki_u32		data_size_out;
		__vki_aligned_u64	data_in;
		__vki_aligned_u64	data_out;
		__vki_u32		repeat;
		__vki_u32		duration;
	} test;

	struct { /* anonymous struct used by BPF_*_GET_*_ID */
		union {
			__vki_u32		start_id;
			__vki_u32		prog_id;
			__vki_u32		map_id;
			__vki_u32		btf_id;
		};
		__vki_u32		next_id;
		__vki_u32		open_flags;
	};

	struct { /* anonymous struct used by BPF_OBJ_GET_INFO_BY_FD */
		__vki_u32		bpf_fd;
		__vki_u32		info_len;
		__vki_aligned_u64	info;
	} info;

	struct { /* anonymous struct used by BPF_PROG_QUERY command */
		__vki_u32		target_fd;	/* container object to query */
		__vki_u32		attach_type;
		__vki_u32		query_flags;
		__vki_u32		attach_flags;
		__vki_aligned_u64	prog_ids;
		__vki_u32		prog_cnt;
	} query;

	struct {
		__vki_u64 name;
		__vki_u32 prog_fd;
	} raw_tracepoint;

	struct { /* anonymous struct for BPF_BTF_LOAD */
		__vki_aligned_u64	btf;
		__vki_aligned_u64	btf_log_buf;
		__vki_u32		btf_size;
		__vki_u32		btf_log_size;
		__vki_u32		btf_log_level;
	};

	struct {
		__vki_u32		pid;		/* input: pid */
		__vki_u32		fd;		/* input: fd */
		__vki_u32		flags;		/* input: flags */
		__vki_u32		buf_len;	/* input/output: buf len */
		__vki_aligned_u64	buf;		/* input/output:
						 *   tp_name for tracepoint
						 *   symbol for kprobe
						 *   filename for uprobe
						 */
		__vki_u32		prog_id;	/* output: prod_id */
		__vki_u32		fd_type;	/* output: BPF_FD_TYPE_* */
		__vki_u64		probe_offset;	/* output: probe_offset */
		__vki_u64		probe_addr;	/* output: probe_addr */
	} task_fd_query;
} __attribute__((aligned(8)));

#define VKI_XDP_PACKET_HEADROOM 256

#define VKI_BPF_TAG_SIZE	8

struct vki_bpf_prog_info {
	__vki_u32 type;
	__vki_u32 id;
	__vki_u8  tag[VKI_BPF_TAG_SIZE];
	__vki_u32 jited_prog_len;
	__vki_u32 xlated_prog_len;
	__vki_aligned_u64 jited_prog_insns;
	__vki_aligned_u64 xlated_prog_insns;
	__vki_u64 load_time;	/* ns since boottime */
	__vki_u32 created_by_uid;
	__vki_u32 nr_map_ids;
	__vki_aligned_u64 map_ids;
	char name[VKI_BPF_OBJ_NAME_LEN];
	__vki_u32 ifindex;
	__vki_u32 gpl_compatible:1;
	__vki_u64 netns_dev;
	__vki_u64 netns_ino;
	__vki_u32 nr_jited_ksyms;
	__vki_u32 nr_jited_func_lens;
	__vki_aligned_u64 jited_ksyms;
	__vki_aligned_u64 jited_func_lens;
} __attribute__((aligned(8)));

struct vki_bpf_map_info {
	__vki_u32 type;
	__vki_u32 id;
	__vki_u32 key_size;
	__vki_u32 value_size;
	__vki_u32 max_entries;
	__vki_u32 map_flags;
	char  name[VKI_BPF_OBJ_NAME_LEN];
	__vki_u32 ifindex;
	__vki_u32 :32;
	__vki_u64 netns_dev;
	__vki_u64 netns_ino;
	__vki_u32 btf_id;
	__vki_u32 btf_key_type_id;
	__vki_u32 btf_value_type_id;
} __attribute__((aligned(8)));

struct vki_bpf_btf_info {
	__vki_aligned_u64 btf;
	__vki_u32 btf_size;
	__vki_u32 id;
} __attribute__((aligned(8)));

//----------------------------------------------------------------------
// From linux-5.1/include/uapi/linux/pps.h
//----------------------------------------------------------------------

struct vki_pps_ktime {
	__vki_s64 sec;
	__vki_s32 nsec;
	__vki_u32 flags;
};

struct vki_pps_kinfo {
	__vki_u32 assert_sequence;
	__vki_u32 clear_sequence;
	struct vki_pps_ktime assert_tu;
	struct vki_pps_ktime clear_tu;
	int current_mode;
};

struct vki_pps_kparams {
	int api_version;
	int mode;
	struct vki_pps_ktime assert_off_tu;
	struct vki_pps_ktime clear_off_tu;
};

struct vki_pps_fdata {
	struct vki_pps_kinfo info;
	struct vki_pps_ktime timeout;
};

struct vki_pps_bind_args {
	int tsformat;
	int edge;
	int consumer;
};

#define VKI_PPS_GETPARAMS	_VKI_IOR('p', 0xa1, struct vki_pps_kparams *)
#define VKI_PPS_SETPARAMS	_VKI_IOW('p', 0xa2, struct vki_pps_kparams *)
#define VKI_PPS_GETCAP		_VKI_IOR('p', 0xa3, int *)
#define VKI_PPS_FETCH		_VKI_IOWR('p', 0xa4, struct vki_pps_fdata *)
#define VKI_PPS_KC_BIND		_VKI_IOW('p', 0xa5, struct vki_pps_bind_args *)

//----------------------------------------------------------------------
// From linux-5.1/include/uapi/linux/ptp_clock.h
//----------------------------------------------------------------------

struct vki_ptp_clock_time {
	__vki_s64 sec;
	__vki_u32 nsec;
	__vki_u32 reserved;
};

struct vki_ptp_clock_caps {
	int max_adj;
	int n_alarm;
	int n_ext_ts;
	int n_per_out;
	int pps;
	int n_pins;
	int cross_timestamping;
	int rsv[13];
};

struct vki_ptp_extts_request {
	unsigned int index;
	unsigned int flags;
	unsigned int rsv[2];
};

struct vki_ptp_perout_request {
	struct vki_ptp_clock_time start;
	struct vki_ptp_clock_time period;
	unsigned int index;
	unsigned int flags;
	unsigned int rsv[4];
};

#define VKI_PTP_MAX_SAMPLES 25

struct vki_ptp_sys_offset {
	unsigned int n_samples;
	unsigned int rsv[3];
	struct vki_ptp_clock_time ts[2 * VKI_PTP_MAX_SAMPLES + 1];
};

struct vki_ptp_sys_offset_extended {
	unsigned int n_samples;
	unsigned int rsv[3];
	struct vki_ptp_clock_time ts[VKI_PTP_MAX_SAMPLES][3];
};

struct vki_ptp_sys_offset_precise {
	struct vki_ptp_clock_time device;
	struct vki_ptp_clock_time sys_realtime;
	struct vki_ptp_clock_time sys_monoraw;
	unsigned int rsv[4];
};

struct vki_ptp_pin_desc {
	char name[64];
	unsigned int index;
	unsigned int func;
	unsigned int chan;
	unsigned int rsv[5];
};

#define VKI_PTP_CLOCK_GETCAPS  _VKI_IOR('=', 1, struct vki_ptp_clock_caps)
#define VKI_PTP_EXTTS_REQUEST  _VKI_IOW('=', 2, struct vki_ptp_extts_request)
#define VKI_PTP_PEROUT_REQUEST _VKI_IOW('=', 3, struct vki_ptp_perout_request)
#define VKI_PTP_ENABLE_PPS     _VKI_IOW('=', 4, int)
#define VKI_PTP_SYS_OFFSET     _VKI_IOW('=', 5, struct vki_ptp_sys_offset)
#define VKI_PTP_PIN_GETFUNC    _VKI_IOWR('=', 6, struct vki_ptp_pin_desc)
#define VKI_PTP_PIN_SETFUNC    _VKI_IOW('=', 7, struct vki_ptp_pin_desc)
#define VKI_PTP_SYS_OFFSET_PRECISE \
	_VKI_IOWR('=', 8, struct vki_ptp_sys_offset_precise)
#define VKI_PTP_SYS_OFFSET_EXTENDED \
	_VKI_IOWR('=', 9, struct vki_ptp_sys_offset_extended)

/* Needed for 64bit time_t on 32bit arches.  */

typedef vki_int64_t vki_time64_t;

/* Note that this is the padding used by glibc, the kernel uses
   a 64-bit signed int, but is ignoring the upper 32 bits of the
   tv_nsec field.  It does always write the full struct though.
   So this is only needed for PRE_MEM_READ. See pre_read_timespec64. */
struct vki_timespec64 {
   vki_time64_t tv_sec;
#if defined(VKI_BIG_ENDIAN)
   vki_int32_t tv_pad;
   vki_int32_t tv_nsec;
#elif defined(VKI_LITTLE_ENDIAN)
   vki_int32_t tv_nsec;
   vki_int32_t tv_pad;
#else
#error edit for your odd byteorder.
#endif
};

struct vki_itimerspec64 {
   struct vki_timespec it_interval;
   struct vki_timespec it_value;
};

#ifndef VKI_RLIM_INFINITY
#define VKI_RLIM_INFINITY (~0UL)
#endif

#define VKI_RLIM64_INFINITY (~0ULL)

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
