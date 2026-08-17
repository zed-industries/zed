
/*--------------------------------------------------------------------*/
/*--- s390x/Linux-specific kernel interface.     vki-s390x-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright IBM Corp. 2010-2017

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

/* Contributed by Florian Krohm and Volker Sameske */

#ifndef __VKI_S390X_LINUX_H
#define __VKI_S390X_LINUX_H

#define __force

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/types.h
//----------------------------------------------------------------------

typedef __signed__ char __vki_s8;
typedef unsigned char __vki_u8;

typedef __signed__ short __vki_s16;
typedef unsigned short __vki_u16;

typedef __signed__ int __vki_s32;
typedef unsigned int __vki_u32;

typedef __signed__ long __vki_s64;
typedef unsigned long __vki_u64;

typedef unsigned short vki_u16;

typedef unsigned int vki_u32;

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/page.h
//----------------------------------------------------------------------

/* PAGE_SHIFT determines the page size */
#define VKI_PAGE_SHIFT  12
#define VKI_PAGE_SIZE   (1UL << VKI_PAGE_SHIFT)

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/siginfo.h
//----------------------------------------------------------------------

/* We need that to ensure that sizeof(siginfo) == 128. */
#ifdef __s390x__
#define __VKI_ARCH_SI_PREAMBLE_SIZE (4 * sizeof(int))
#endif

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/sigcontext.h
//----------------------------------------------------------------------

#define __VKI_NUM_GPRS 16
#define __VKI_NUM_FPRS 16
#define __VKI_NUM_ACRS 16

#ifndef VGA_s390x

/* Has to be at least _NSIG_WORDS from asm/signal.h */
#define _VKI_SIGCONTEXT_NSIG	64
#define _VKI_SIGCONTEXT_NSIG_BPW	32
/* Size of stack frame allocated when calling signal handler. */
#define __VKI_SIGNAL_FRAMESIZE	96

#else /* VGA_s390x */

/* Has to be at least _NSIG_WORDS from asm/signal.h */
#define _VKI_SIGCONTEXT_NSIG	64
#define _VKI_SIGCONTEXT_NSIG_BPW	64
/* Size of stack frame allocated when calling signal handler. */
#define __VKI_SIGNAL_FRAMESIZE	160

#endif /* VGA_s390x */


#define _VKI_SIGCONTEXT_NSIG_WORDS	(_VKI_SIGCONTEXT_NSIG / _VKI_SIGCONTEXT_NSIG_BPW)
#define _VKI_SIGMASK_COPY_SIZE	(sizeof(unsigned long)*_VKI_SIGCONTEXT_NSIG_WORDS)

typedef struct
{
	unsigned long mask;
	unsigned long addr;
} __attribute__ ((aligned(8))) _vki_psw_t;

typedef struct
{
	_vki_psw_t psw;
	unsigned long gprs[__VKI_NUM_GPRS];
	unsigned int  acrs[__VKI_NUM_ACRS];
} _vki_s390_regs_common;

typedef struct
{
	unsigned int fpc;
	double   fprs[__VKI_NUM_FPRS];
} _vki_s390_fp_regs;

typedef struct
{
	_vki_s390_regs_common regs;
	_vki_s390_fp_regs     fpregs;
} _vki_sigregs;


struct vki_sigcontext
{
	unsigned long   oldmask[_VKI_SIGCONTEXT_NSIG_WORDS];
	_vki_sigregs    __user *sregs;
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/signal.h
//----------------------------------------------------------------------

#define _VKI_NSIG           _VKI_SIGCONTEXT_NSIG
#define _VKI_NSIG_BPW       _VKI_SIGCONTEXT_NSIG_BPW
#define _VKI_NSIG_WORDS     _VKI_SIGCONTEXT_NSIG_WORDS

typedef unsigned long vki_old_sigset_t;

typedef struct {
	unsigned long sig[_VKI_NSIG_WORDS];
} vki_sigset_t;

#define VKI_SIGHUP           1
#define VKI_SIGINT           2
#define VKI_SIGQUIT          3
#define VKI_SIGILL           4
#define VKI_SIGTRAP          5
#define VKI_SIGABRT          6
#define VKI_SIGIOT           6
#define VKI_SIGBUS           7
#define VKI_SIGFPE           8
#define VKI_SIGKILL          9
#define VKI_SIGUSR1         10
#define VKI_SIGSEGV         11
#define VKI_SIGUSR2         12
#define VKI_SIGPIPE         13
#define VKI_SIGALRM         14
#define VKI_SIGTERM         15
#define VKI_SIGSTKFLT       16
#define VKI_SIGCHLD         17
#define VKI_SIGCONT         18
#define VKI_SIGSTOP         19
#define VKI_SIGTSTP         20
#define VKI_SIGTTIN         21
#define VKI_SIGTTOU         22
#define VKI_SIGURG          23
#define VKI_SIGXCPU         24
#define VKI_SIGXFSZ         25
#define VKI_SIGVTALRM       26
#define VKI_SIGPROF         27
#define VKI_SIGWINCH        28
#define VKI_SIGIO           29
#define VKI_SIGPOLL         VKI_SIGIO
/*
#define VKI_SIGLOST         29
*/
#define VKI_SIGPWR          30
#define VKI_SIGSYS	    31
#define VKI_SIGUNUSED       31

/* These should not be considered constants from userland.  */
#define VKI_SIGRTMIN        32
#define VKI_SIGRTMAX        _VKI_NSIG

/*
 * SA_FLAGS values:
 *
 * SA_ONSTACK indicates that a registered stack_t will be used.
 * SA_INTERRUPT is a no-op, but left due to historical reasons. Use the
 * SA_RESTART flag to get restarting signals (which were the default long ago)
 * SA_NOCLDSTOP flag to turn off SIGCHLD when children stop.
 * SA_RESETHAND clears the handler when the signal is delivered.
 * SA_NOCLDWAIT flag on SIGCHLD to inhibit zombies.
 * SA_NODEFER prevents the current signal from being masked in the handler.
 *
 * SA_ONESHOT and SA_NOMASK are the historical Linux names for the Single
 * Unix names RESETHAND and NODEFER respectively.
 */
#define VKI_SA_NOCLDSTOP    0x00000001
#define VKI_SA_NOCLDWAIT    0x00000002
#define VKI_SA_SIGINFO      0x00000004
#define VKI_SA_ONSTACK      0x08000000
#define VKI_SA_RESTART      0x10000000
#define VKI_SA_NODEFER      0x40000000
#define VKI_SA_RESETHAND    0x80000000

#define VKI_SA_NOMASK       VKI_SA_NODEFER
#define VKI_SA_ONESHOT      VKI_SA_RESETHAND
#define VKI_SA_INTERRUPT    0x20000000 /* dummy -- ignored */

#define VKI_SA_RESTORER     0x04000000

/*
 * sigaltstack controls
 */
#define VKI_SS_ONSTACK      1
#define VKI_SS_DISABLE      2

#define VKI_MINSIGSTKSZ     2048
#define VKI_SIGSTKSZ        8192


/* Next lines asm-generic/signal.h */
#define VKI_SIG_BLOCK          0 /* for blocking signals */
#define VKI_SIG_UNBLOCK        1 /* for unblocking signals */
#define VKI_SIG_SETMASK        2 /* for setting the signal mask */

typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

/* default signal handling */
#define VKI_SIG_DFL ((__force __vki_sighandler_t)0)
/* ignore signal */
#define VKI_SIG_IGN ((__force __vki_sighandler_t)1)
/* error return from signal */
#define VKI_SIG_ERR ((__force __vki_sighandler_t)-1)
/* Back to asm-s390/signal.h */

struct vki_old_sigaction {
        // [[Nb: a 'k' prefix is added to "sa_handler" because
        // bits/sigaction.h (which gets dragged in somehow via signal.h)
        // #defines it as something else.  Since that is done for glibc's
        // purposes, which we don't care about here, we use our own name.]]
        __vki_sighandler_t ksa_handler;
        vki_old_sigset_t sa_mask;
        unsigned long sa_flags;
        void (*sa_restorer)(void);
};

struct vki_sigaction {
        // [[See comment about extra 'k' above]]
        __vki_sighandler_t ksa_handler;
        // Yes, the reserved field is really glibc specific. The kernel
        // doesn't have it and uses an unsigned long for sa_flags.
        // The glibc and the kernel agreed this is fine and the
        // __glibc_reserved0 field can be undefined.
        // See https://sourceware.org/ml/libc-alpha/2014-09/msg00161.html
        int __glibc_reserved0;
        int sa_flags;
        void (*sa_restorer)(void);
        vki_sigset_t sa_mask;               /* mask last for extensibility */
};

struct vki_k_sigaction {
        struct vki_sigaction sa;
};


/* On Linux we use the same type for passing sigactions to
   and from the kernel.  Hence: */
typedef  struct vki_sigaction  vki_sigaction_toK_t;
typedef  struct vki_sigaction  vki_sigaction_fromK_t;


typedef struct vki_sigaltstack {
	void __user *ss_sp;
	int ss_flags;
	vki_size_t ss_size;
} vki_stack_t;


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_NONE   0x0             /* No page permissions */
#define VKI_PROT_READ   0x1             /* page can be read */
#define VKI_PROT_WRITE  0x2             /* page can be written */
#define VKI_PROT_EXEC   0x4             /* page can be executed */
#define VKI_PROT_GROWSDOWN 0x01000000   /* mprotect flag: extend
					   change to start of
					   growsdown vma */
#define VKI_PROT_GROWSUP   0x02000000   /* mprotect flag:
					   extend change to end
					   of growsup vma */

#define VKI_MAP_SHARED		0x0001  /* Share changes */
#define VKI_MAP_PRIVATE 	0x0002	/*  */
#define VKI_MAP_FIXED   	0x0010	/*  */
#define VKI_MAP_ANONYMOUS	0x0020	/*  */


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/fcntl.h
//----------------------------------------------------------------------

#define VKI_O_RDONLY        00000000
#define VKI_O_WRONLY        00000001
#define VKI_O_RDWR          00000002
#define VKI_O_ACCMODE       00000003
#define VKI_O_CREAT         00000100        /* not fcntl */
#define VKI_O_EXCL          00000200        /* not fcntl */
#define VKI_O_NOCTTY        00000400        /* not fcntl */
#define VKI_O_TRUNC         00001000        /* not fcntl */
#define VKI_O_APPEND        00002000
#define VKI_O_NONBLOCK      00004000

#define VKI_AT_FDCWD            -100

#define VKI_F_DUPFD	0	/* dup */
#define VKI_F_GETFD	1	/* get close_on_exec */
#define VKI_F_SETFD	2	/* set/clear close_on_exec */
#define VKI_F_GETFL	3	/* get file->f_flags */
#define VKI_F_SETFL	4	/* set file->f_flags */
#define VKI_F_GETLK	5
#define VKI_F_SETLK	6
#define VKI_F_SETLKW	7
#define VKI_F_SETOWN	8	/* for sockets. */
#define VKI_F_GETOWN	9	/* for sockets. */
#define VKI_F_SETSIG	10	/* for sockets. */
#define VKI_F_GETSIG	11	/* for sockets. */

#define VKI_F_SETOWN_EX		15
#define VKI_F_GETOWN_EX		16

#define VKI_F_OFD_GETLK		36
#define VKI_F_OFD_SETLK		37
#define VKI_F_OFD_SETLKW	38

#define VKI_F_OWNER_TID		0
#define VKI_F_OWNER_PID		1
#define VKI_F_OWNER_PGRP	2

struct vki_f_owner_ex {
	int	type;
	__vki_kernel_pid_t	pid;
};

#define VKI_FD_CLOEXEC  1  /* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE   1024


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390x/resource.h
//----------------------------------------------------------------------

// which just does #include <asm-generic/resource.h>

#define VKI_RLIMIT_DATA             2       /* max data size */
#define VKI_RLIMIT_STACK            3       /* max stack size */
#define VKI_RLIMIT_CORE             4       /* max core file size */
#define VKI_RLIMIT_NOFILE           7       /* max number of open files */


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET      1

#define VKI_SO_TYPE         3

#define VKI_SO_ATTACH_FILTER        26

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCSPGRP       0x8902
#define VKI_SIOCGPGRP       0x8904
#define VKI_SIOCATMARK      0x8905
#define VKI_SIOCGSTAMP      0x8906          /* Get stamp (timeval) */
/* since 2.6.22 */
#define VKI_SIOCGSTAMPNS    0x8907          /* Get stamp (timespec) */


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/stat.h
//----------------------------------------------------------------------

#ifndef VGA_s390x
struct vki_stat {
        unsigned short st_dev;
        unsigned short __pad1;
        unsigned long  st_ino;
        unsigned short st_mode;
        unsigned short st_nlink;
        unsigned short st_uid;
        unsigned short st_gid;
        unsigned short st_rdev;
        unsigned short __pad2;
        unsigned long  st_size;
        unsigned long  st_blksize;
        unsigned long  st_blocks;
        unsigned long  st_atime;
        unsigned long  st_atime_nsec;
        unsigned long  st_mtime;
        unsigned long  st_mtime_nsec;
        unsigned long  st_ctime;
        unsigned long  st_ctime_nsec;
        unsigned long  __unused4;
        unsigned long  __unused5;
};

/* This matches struct stat64 in glibc2.1, hence the absolutely
 * insane amounts of padding around dev_t's.
 */
struct vki_stat64 {
        unsigned long long	st_dev;
        unsigned int    __pad1;
        unsigned long   __st_ino;
        unsigned int    st_mode;
        unsigned int    st_nlink;
        unsigned long   st_uid;
        unsigned long   st_gid;
        unsigned long long	st_rdev;
        unsigned int    __pad3;
        long long	st_size;
        unsigned long   st_blksize;
        unsigned char   __pad4[4];
        unsigned long   __pad5;     /* future possible st_blocks high bits */
        unsigned long   st_blocks;  /* Number 512-byte blocks allocated. */
        unsigned long   st_atime;
        unsigned long   st_atime_nsec;
        unsigned long   st_mtime;
        unsigned long   st_mtime_nsec;
        unsigned long   st_ctime;
        unsigned long   st_ctime_nsec;  /* will be high 32 bits of ctime someday */
        unsigned long long	st_ino;
};

#else

struct vki_stat {
        unsigned long  st_dev;
        unsigned long  st_ino;
        unsigned long  st_nlink;
        unsigned int   st_mode;
        unsigned int   st_uid;
        unsigned int   st_gid;
        unsigned int   __pad1;
        unsigned long  st_rdev;
        unsigned long  st_size;
        unsigned long  st_atime;
	unsigned long  st_atime_nsec;
        unsigned long  st_mtime;
	unsigned long  st_mtime_nsec;
        unsigned long  st_ctime;
	unsigned long  st_ctime_nsec;
        unsigned long  st_blksize;
        long           st_blocks;
        unsigned long  __unused0[3];
};

#endif /* VGA_s390x */


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/statfs.h
//----------------------------------------------------------------------

struct vki_statfs {
        int  f_type;
        int  f_bsize;
        long f_blocks;
        long f_bfree;
        long f_bavail;
        long f_files;
        long f_ffree;
        __vki_kernel_fsid_t f_fsid;
        int  f_namelen;
        int  f_frsize;
        int  f_spare[5];
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/termios.h
//----------------------------------------------------------------------

struct vki_winsize {
	unsigned short ws_row;
	unsigned short ws_col;
	unsigned short ws_xpixel;
	unsigned short ws_ypixel;
};

#define VKI_NCC 8
struct vki_termio {
	unsigned short c_iflag;		/* input mode flags */
	unsigned short c_oflag;		/* output mode flags */
	unsigned short c_cflag;		/* control mode flags */
	unsigned short c_lflag;		/* local mode flags */
	unsigned char c_line;		/* line discipline */
	unsigned char c_cc[VKI_NCC];	/* control characters */
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/termbits.h
//----------------------------------------------------------------------

typedef unsigned char   vki_cc_t;
typedef unsigned int    vki_tcflag_t;

#define VKI_NCCS 19
struct vki_termios {
	vki_tcflag_t c_iflag;		/* input mode flags */
	vki_tcflag_t c_oflag;		/* output mode flags */
	vki_tcflag_t c_cflag;		/* control mode flags */
	vki_tcflag_t c_lflag;		/* local mode flags */
	vki_cc_t c_line;		/* line discipline */
	vki_cc_t c_cc[VKI_NCCS];	/* control characters */
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ioctl.h
//----------------------------------------------------------------------

#define _VKI_IOC_NRBITS		8
#define _VKI_IOC_TYPEBITS	8
#define _VKI_IOC_SIZEBITS	14
#define _VKI_IOC_DIRBITS	2

#define _VKI_IOC_NRMASK		((1 << _VKI_IOC_NRBITS)-1)
#define _VKI_IOC_TYPEMASK	((1 << _VKI_IOC_TYPEBITS)-1)
#define _VKI_IOC_SIZEMASK	((1 << _VKI_IOC_SIZEBITS)-1)
#define _VKI_IOC_DIRMASK	((1 << _VKI_IOC_DIRBITS)-1)

#define _VKI_IOC_NRSHIFT	0
#define _VKI_IOC_TYPESHIFT	(_VKI_IOC_NRSHIFT+_VKI_IOC_NRBITS)
#define _VKI_IOC_SIZESHIFT	(_VKI_IOC_TYPESHIFT+_VKI_IOC_TYPEBITS)
#define _VKI_IOC_DIRSHIFT	(_VKI_IOC_SIZESHIFT+_VKI_IOC_SIZEBITS)

#define _VKI_IOC_NONE	0U
#define _VKI_IOC_WRITE	1U
#define _VKI_IOC_READ	2U

#define _VKI_IOC(dir,type,nr,size) \
	(((dir)  << _VKI_IOC_DIRSHIFT) | \
	 ((type) << _VKI_IOC_TYPESHIFT) | \
	 ((nr)   << _VKI_IOC_NRSHIFT) | \
	 ((size) << _VKI_IOC_SIZESHIFT))

/* used to create numbers */
#define _VKI_IO(type,nr)	_VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ,(type),(nr),(_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOW(type,nr,size)	_VKI_IOC(_VKI_IOC_WRITE,(type),(nr),(_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOWR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE,(type),(nr),(_VKI_IOC_TYPECHECK(size)))

/* used to decode ioctl numbers.. */
#define _VKI_IOC_DIR(nr)	(((nr) >> _VKI_IOC_DIRSHIFT) & _VKI_IOC_DIRMASK)
#define _VKI_IOC_TYPE(nr)	(((nr) >> _VKI_IOC_TYPESHIFT) & _VKI_IOC_TYPEMASK)
#define _VKI_IOC_NR(nr)		(((nr) >> _VKI_IOC_NRSHIFT) & _VKI_IOC_NRMASK)
#define _VKI_IOC_SIZE(nr)	(((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ioctls.h
//----------------------------------------------------------------------

/* 0x54 is just a magic number to make these relatively unique ('T') */

#define VKI_TCGETS	0x5401
#define VKI_TCSETS	0x5402
#define VKI_TCSETSW	0x5403
#define VKI_TCSETSF	0x5404
#define VKI_TCGETA	0x5405
#define VKI_TCSETA	0x5406
#define VKI_TCSETAW	0x5407
#define VKI_TCSETAF	0x5408
#define VKI_TCSBRK	0x5409
#define VKI_TCXONC	0x540A
#define VKI_TCFLSH	0x540B

#define VKI_TIOCSCTTY	0x540E
#define VKI_TIOCGPGRP	0x540F
#define VKI_TIOCSPGRP	0x5410
#define VKI_TIOCOUTQ	0x5411

#define VKI_TIOCGWINSZ	0x5413
#define VKI_TIOCSWINSZ	0x5414
#define VKI_TIOCMGET	0x5415
#define VKI_TIOCMBIS	0x5416
#define VKI_TIOCMBIC	0x5417
#define VKI_TIOCMSET	0x5418

#define VKI_FIONREAD	0x541B
#define VKI_TIOCLINUX	0x541C

#define VKI_FIONBIO	0x5421
#define VKI_TIOCNOTTY	0x5422

#define VKI_TCSBRKP	0x5425	/* Needed for POSIX tcsendbreak() */

#define VKI_TIOCGPTN	_VKI_IOR('T',0x30, unsigned int) /* Get Pty Number (of pty-mux device) */
#define VKI_TIOCSPTLCK	_VKI_IOW('T',0x31, int)  /* Lock/unlock Pty */

#define VKI_FIONCLEX	0x5450
#define VKI_FIOCLEX	0x5451
#define VKI_FIOASYNC	0x5452

#define VKI_TIOCSERGETLSR       0x5459 /* Get line status register */

#define VKI_TIOCGICOUNT	0x545D	/* read serial port inline interrupt counts */

//----------------------------------------------------------------------
// From linux-2.6.39-rc2/arch/s390/include/asm/ioctls.h
//----------------------------------------------------------------------

#define VKI_FIOQSIZE 0x545E

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/poll.h
//----------------------------------------------------------------------

struct vki_pollfd {
	int fd;
	short events;
	short revents;
};

#define VKI_POLLIN          0x0001

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ptrace.h
//----------------------------------------------------------------------
#define VKI_NUM_GPRS	16
#define VKI_NUM_FPRS	16
#define VKI_NUM_CRS	16
#define VKI_NUM_ACRS	16

typedef union
{
	float   f;
	double  d;
        __vki_u64   ui;
	struct
	{
		__vki_u32 hi;
		__vki_u32 lo;
	} fp;
} vki_freg_t;

typedef struct
{
	__vki_u32   fpc;
	vki_freg_t  fprs[VKI_NUM_FPRS];
} vki_s390_fp_regs;

typedef struct
{
        unsigned long mask;
        unsigned long addr;
} __attribute__ ((aligned(8))) vki_psw_t;

typedef struct
{
	vki_psw_t psw;
	unsigned long gprs[VKI_NUM_GPRS];
	unsigned int  acrs[VKI_NUM_ACRS];
	unsigned long orig_gpr2;
} vki_s390_regs;

/*
 * Now for the program event recording (trace) definitions.
 */
typedef struct
{
	unsigned long cr[3];
} vki_per_cr_words;

typedef	struct
{
#ifdef VGA_s390x
	unsigned                       : 32;
#endif /* VGA_s390x */
	unsigned em_branching          : 1;
	unsigned em_instruction_fetch  : 1;
	/*
	 * Switching on storage alteration automatically fixes
	 * the storage alteration event bit in the users std.
	 */
	unsigned em_storage_alteration : 1;
	unsigned em_gpr_alt_unused     : 1;
	unsigned em_store_real_address : 1;
	unsigned                       : 3;
	unsigned branch_addr_ctl       : 1;
	unsigned                       : 1;
	unsigned storage_alt_space_ctl : 1;
	unsigned                       : 21;
	unsigned long starting_addr;
	unsigned long ending_addr;
} vki_per_cr_bits;

typedef struct
{
	unsigned short perc_atmid;
	unsigned long address;
	unsigned char access_id;
} vki_per_lowcore_words;

typedef struct
{
	unsigned perc_branching          : 1;
	unsigned perc_instruction_fetch  : 1;
	unsigned perc_storage_alteration : 1;
	unsigned perc_gpr_alt_unused     : 1;
	unsigned perc_store_real_address : 1;
	unsigned                         : 3;
	unsigned atmid_psw_bit_31        : 1;
	unsigned atmid_validity_bit      : 1;
	unsigned atmid_psw_bit_32        : 1;
	unsigned atmid_psw_bit_5         : 1;
	unsigned atmid_psw_bit_16        : 1;
	unsigned atmid_psw_bit_17        : 1;
	unsigned si                      : 2;
	unsigned long address;
	unsigned                         : 4;
	unsigned access_id               : 4;
} vki_per_lowcore_bits;

typedef struct
{
	union {
		vki_per_cr_words   words;
		vki_per_cr_bits    bits;
	} control_regs;
	/*
	 * Use these flags instead of setting em_instruction_fetch
	 * directly they are used so that single stepping can be
	 * switched on & off while not affecting other tracing
	 */
	unsigned  single_step       : 1;
	unsigned  instruction_fetch : 1;
	unsigned                    : 30;
	/*
	 * These addresses are copied into cr10 & cr11 if single
	 * stepping is switched off
	 */
	unsigned long starting_addr;
	unsigned long ending_addr;
	union {
		vki_per_lowcore_words words;
		vki_per_lowcore_bits  bits;
	} lowcore;
} vki_per_struct;

/*
 * The user_regs_struct defines the way the user registers are
 * store on the stack for signal handling.
 */
struct vki_user_regs_struct
{
	vki_psw_t psw;
	unsigned long gprs[VKI_NUM_GPRS];
	unsigned int  acrs[VKI_NUM_ACRS];
	unsigned long orig_gpr2;
	vki_s390_fp_regs fp_regs;
	/*
	 * These per registers are in here so that gdb can modify them
	 * itself as there is no "official" ptrace interface for hardware
	 * watchpoints. This is the way intel does it.
	 */
	vki_per_struct per_info;
	unsigned long ieee_instruction_pointer;
	/* Used to give failing instruction back to user for ieee exceptions */
};

typedef struct
{
	unsigned int  vki_len;
	unsigned long vki_kernel_addr;
	unsigned long vki_process_addr;
} vki_ptrace_area;

/*
 * S/390 specific non posix ptrace requests
 */
#define VKI_PTRACE_PEEKUSR_AREA       0x5000
#define VKI_PTRACE_POKEUSR_AREA       0x5001

//----------------------------------------------------------------------
// From linux-3.18/include/asm-s390/elf.h
//----------------------------------------------------------------------

typedef vki_s390_fp_regs vki_elf_fpregset_t;
typedef vki_s390_regs vki_elf_gregset_t;

#define VKI_HWCAP_S390_TE           1024
#define VKI_HWCAP_S390_VXRS         2048


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ucontext.h
//----------------------------------------------------------------------

struct vki_ucontext {
	unsigned long	      uc_flags;
	struct vki_ucontext  *uc_link;
	vki_stack_t	      uc_stack;
	_vki_sigregs          uc_mcontext;
	vki_sigset_t	      uc_sigmask; /* mask last for extensibility */
};

typedef char vki_modify_ldt_t;

//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ipcbuf.h
//----------------------------------------------------------------------

struct vki_ipc64_perm
{
	__vki_kernel_key_t	key;
	__vki_kernel_uid32_t	uid;
	__vki_kernel_gid32_t	gid;
	__vki_kernel_uid32_t	cuid;
	__vki_kernel_gid32_t	cgid;
	__vki_kernel_mode_t	mode;
	unsigned short		__pad1;
	unsigned short		seq;
#ifndef VGA_s390x
	unsigned short		__pad2;
#endif /* ! VGA_s390x */
	unsigned long		__unused1;
	unsigned long		__unused2;
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/sembuf.h
//----------------------------------------------------------------------

struct vki_semid64_ds {
	struct vki_ipc64_perm sem_perm;		/* permissions .. see ipc.h */
	__vki_kernel_time_t   sem_otime;	/* last semop time */
#ifndef VGA_s390x
	unsigned long	__unused1;
#endif /* ! VGA_s390x */
	__vki_kernel_time_t   sem_ctime;	/* last change time */
#ifndef VGA_s390x
	unsigned long	__unused2;
#endif /* ! VGA_s390x */
	unsigned long	sem_nsems;		/* no. of semaphores in array */
	unsigned long	__unused3;
	unsigned long	__unused4;
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
	struct vki_ipc64_perm msg_perm;
	__vki_kernel_time_t msg_stime;	/* last msgsnd time */
#ifndef VGA_s390x
	unsigned long	__unused1;
#endif /* ! VGA_s390x */
	__vki_kernel_time_t msg_rtime;	/* last msgrcv time */
#ifndef VGA_s390x
	unsigned long	__unused2;
#endif /* ! VGA_s390x */
	__vki_kernel_time_t msg_ctime;	/* last change time */
#ifndef VGA_s390x
	unsigned long	__unused3;
#endif /* ! VGA_s390x */
	unsigned long  msg_cbytes;	/* current number of bytes on queue */
	unsigned long  msg_qnum;	/* number of messages in queue */
	unsigned long  msg_qbytes;	/* max number of bytes on queue */
	__vki_kernel_pid_t msg_lspid;	/* pid of last msgsnd */
	__vki_kernel_pid_t msg_lrpid;	/* last receive pid */
	unsigned long  __unused4;
	unsigned long  __unused5;
};


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/ipc.h
//----------------------------------------------------------------------

struct vki_ipc_kludge {
	struct vki_msgbuf __user *msgp;
	long msgtyp;
};

#define VKI_SEMOP	 1
#define VKI_SEMGET	 2
#define VKI_SEMCTL	 3
#define VKI_SEMTIMEDOP	 4
#define VKI_MSGSND	11
#define VKI_MSGRCV	12
#define VKI_MSGGET	13
#define VKI_MSGCTL	14
#define VKI_SHMAT	21
#define VKI_SHMDT	22
#define VKI_SHMGET	23
#define VKI_SHMCTL	24


//----------------------------------------------------------------------
// From linux-2.6.16.60/include/asm-s390/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
	struct vki_ipc64_perm	shm_perm;	/* operation perms */
	vki_size_t		shm_segsz;	/* size of segment (bytes) */
	__vki_kernel_time_t	shm_atime;	/* last attach time */
#ifndef VGA_s390x
	unsigned long		__unused1;
#endif /* ! VGA_s390x */
	__vki_kernel_time_t	shm_dtime;	/* last detach time */
#ifndef VGA_s390x
	unsigned long		__unused2;
#endif /* ! VGA_s390x */
	__vki_kernel_time_t	shm_ctime;	/* last change time */
#ifndef VGA_s390x
	unsigned long		__unused3;
#endif /* ! VGA_s390x */
	__vki_kernel_pid_t	shm_cpid;	/* pid of creator */
	__vki_kernel_pid_t	shm_lpid;	/* pid of last operator */
	unsigned long		shm_nattch;	/* no. of current attaches */
	unsigned long		__unused4;
	unsigned long		__unused5;
};

struct vki_shminfo64 {
	unsigned long	shmmax;
	unsigned long	shmmin;
	unsigned long	shmmni;
	unsigned long	shmseg;
	unsigned long	shmall;
	unsigned long	__unused1;
	unsigned long	__unused2;
	unsigned long	__unused3;
	unsigned long	__unused4;
};


//----------------------------------------------------------------------
// The following are defined in the VKI namespace but are nowhere found
// in the linux headers.
//----------------------------------------------------------------------
#define VKI_BIG_ENDIAN      1
#define VKI_MAX_PAGE_SHIFT  VKI_PAGE_SHIFT
#define VKI_MAX_PAGE_SIZE   VKI_PAGE_SIZE

//----------------------------------------------------------------------
// From linux-2.6.35.4/arch/s390x/include/asm/shmparam.h
//----------------------------------------------------------------------

#define VKI_SHMLBA  VKI_PAGE_SIZE

/* If a system call returns a value >= VKI_MAX_ERRNO then that is considered
   an error condition. I.e. the system call failed. */
#define VKI_MAX_ERRNO       -125

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-generic/errno.h
//----------------------------------------------------------------------

#define	VKI_ENOSYS       38  /* Function not implemented */
#define	VKI_EOVERFLOW    75  /* Value too large for defined data type */

//----------------------------------------------------------------------
// From linux-3.19.0/include/uapi/asm-generic/ioctls.h
//----------------------------------------------------------------------

#define VKI_TIOCGSERIAL     0x541E
#define VKI_TIOCSSERIAL     0x541F

#endif // __VKI_S390X_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
