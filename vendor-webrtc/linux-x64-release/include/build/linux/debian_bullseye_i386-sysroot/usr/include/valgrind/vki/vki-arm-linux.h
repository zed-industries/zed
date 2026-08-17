
/*--------------------------------------------------------------------*/
/*--- arm/Linux-specific kernel interface.         vki-arm-linux.h ---*/
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

#ifndef __VKI_ARM_LINUX_H
#define __VKI_ARM_LINUX_H

// arm is little-endian.
#define VKI_LITTLE_ENDIAN  1

// The various comments below indicating i386-ness should be regarded
// with great skepticism -- they are quite possibly wrong.  But see
// also bug 269079 comment 0.

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/types.h
//----------------------------------------------------------------------

typedef unsigned char __vki_u8;

typedef __signed__ short __vki_s16;
typedef unsigned short __vki_u16;

typedef __signed__ int __vki_s32;
typedef unsigned int __vki_u32;

typedef __signed__ long long __vki_s64;
typedef unsigned long long __vki_u64;

typedef unsigned short vki_u16;

typedef unsigned int vki_u32;

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/page.h
//----------------------------------------------------------------------

/* PAGE_SHIFT determines the page size */
#define VKI_PAGE_SHIFT	12
#define VKI_PAGE_SIZE	(1UL << VKI_PAGE_SHIFT)
#define VKI_MAX_PAGE_SHIFT	VKI_PAGE_SHIFT
#define VKI_MAX_PAGE_SIZE	VKI_PAGE_SIZE

//----------------------------------------------------------------------
// From linux-2.6.35.4/arch/arm/include/asm/shmparam.h
//----------------------------------------------------------------------

#define VKI_SHMLBA  (4 * VKI_PAGE_SIZE)

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/signal.h
//----------------------------------------------------------------------

#define VKI_MINSIGSTKSZ	2048

#define VKI_SIG_BLOCK          0	/* for blocking signals */
#define VKI_SIG_UNBLOCK        1	/* for unblocking signals */
#define VKI_SIG_SETMASK        2	/* for setting the signal mask */

/* Type of a signal handler.  */
typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

#define VKI_SIG_DFL	((__vki_sighandler_t)0)	/* default signal handling */
#define VKI_SIG_IGN	((__vki_sighandler_t)1)	/* ignore signal */

#define _VKI_NSIG	64
#define _VKI_NSIG_BPW	32
#define _VKI_NSIG_WORDS	(_VKI_NSIG / _VKI_NSIG_BPW)

typedef unsigned long vki_old_sigset_t;		/* at least 32 bits */

typedef struct {
	unsigned long sig[_VKI_NSIG_WORDS];
} vki_sigset_t;

#define VKI_SIGHUP		 1
#define VKI_SIGINT		 2
#define VKI_SIGQUIT		 3
#define VKI_SIGILL		 4
#define VKI_SIGTRAP		 5
#define VKI_SIGABRT		 6
//#define VKI_SIGIOT		 6
#define VKI_SIGBUS		 7
#define VKI_SIGFPE		 8
#define VKI_SIGKILL		 9
#define VKI_SIGUSR1		10
#define VKI_SIGSEGV		11
#define VKI_SIGUSR2		12
#define VKI_SIGPIPE		13
#define VKI_SIGALRM		14
#define VKI_SIGTERM		15
#define VKI_SIGSTKFLT		16
#define VKI_SIGCHLD		17
#define VKI_SIGCONT		18
#define VKI_SIGSTOP		19
#define VKI_SIGTSTP		20
#define VKI_SIGTTIN		21
#define VKI_SIGTTOU		22
#define VKI_SIGURG		23
#define VKI_SIGXCPU		24
#define VKI_SIGXFSZ		25
#define VKI_SIGVTALRM		26
#define VKI_SIGPROF		27
#define VKI_SIGWINCH		28
#define VKI_SIGIO		29
#define VKI_SIGPWR		30
#define VKI_SIGSYS		31
#define	VKI_SIGUNUSED		31

/* These should not be considered constants from userland.  */
#define VKI_SIGRTMIN	32
// [[This was (_NSIG-1) in 2.4.X... not sure if it matters.]]
#define VKI_SIGRTMAX	_VKI_NSIG

#define VKI_SA_NOCLDSTOP	0x00000001u
#define VKI_SA_NOCLDWAIT	0x00000002u
#define VKI_SA_SIGINFO		0x00000004u
#define VKI_SA_ONSTACK		0x08000000u
#define VKI_SA_RESTART		0x10000000u
#define VKI_SA_NODEFER		0x40000000u
#define VKI_SA_RESETHAND	0x80000000u

#define VKI_SA_NOMASK		VKI_SA_NODEFER
#define VKI_SA_ONESHOT		VKI_SA_RESETHAND
//#define VKI_SA_INTERRUPT	0x20000000 /* dummy -- ignored */

#define VKI_SA_RESTORER		0x04000000

#define VKI_SS_ONSTACK	1
#define VKI_SS_DISABLE	2

struct vki_old_sigaction {
        // [[Nb: a 'k' prefix is added to "sa_handler" because
        // bits/sigaction.h (which gets dragged in somehow via signal.h)
        // #defines it as something else.  Since that is done for glibc's
        // purposes, which we don't care about here, we use our own name.]]
        __vki_sighandler_t ksa_handler;
        vki_old_sigset_t sa_mask;
        unsigned long sa_flags;
        __vki_sigrestore_t sa_restorer;
};

struct vki_sigaction_base {
        // [[See comment about extra 'k' above]]
	__vki_sighandler_t ksa_handler;
	unsigned long sa_flags;
	__vki_sigrestore_t sa_restorer;
	vki_sigset_t sa_mask;		/* mask last for extensibility */
};

/* On Linux we use the same type for passing sigactions to
   and from the kernel.  Hence: */
typedef  struct vki_sigaction_base  vki_sigaction_toK_t;
typedef  struct vki_sigaction_base  vki_sigaction_fromK_t;


typedef struct vki_sigaltstack {
	void __user *ss_sp;
	int ss_flags;
	vki_size_t ss_size;
} vki_stack_t;

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/sigcontext.h
//----------------------------------------------------------------------

struct vki_sigcontext {
		unsigned long trap_no;
		unsigned long error_code;
		unsigned long oldmask;
		unsigned long arm_r0;
		unsigned long arm_r1;
		unsigned long arm_r2;
		unsigned long arm_r3;
		unsigned long arm_r4;
		unsigned long arm_r5;
		unsigned long arm_r6;
		unsigned long arm_r7;
		unsigned long arm_r8;
		unsigned long arm_r9;
		unsigned long arm_r10;
		unsigned long arm_fp;
		unsigned long arm_ip;
		unsigned long arm_sp;
		unsigned long arm_lr;
		unsigned long arm_pc;
		unsigned long arm_cpsr;
		unsigned long fault_address;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_NONE	0x0		/* No page permissions */
#define VKI_PROT_READ	0x1		/* page can be read */
#define VKI_PROT_WRITE	0x2		/* page can be written */
#define VKI_PROT_EXEC	0x4		/* page can be executed */
#define VKI_PROT_GROWSDOWN	0x01000000	/* mprotect flag: extend change to start of growsdown vma */
#define VKI_PROT_GROWSUP	0x02000000	/* mprotect flag: extend change to end of growsup vma */

#define VKI_MAP_SHARED	0x01		/* Share changes */
#define VKI_MAP_PRIVATE	0x02		/* Changes are private */
//#define VKI_MAP_TYPE	0x0f		/* Mask for type of mapping */
#define VKI_MAP_FIXED	0x10		/* Interpret addr exactly */
#define VKI_MAP_ANONYMOUS	0x20	/* don't use a file */
#define VKI_MAP_NORESERVE	0x4000		/* don't check for reservations */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/fcntl.h
//----------------------------------------------------------------------

#define VKI_O_ACCMODE	     03
#define VKI_O_RDONLY	     00
#define VKI_O_WRONLY	     01
#define VKI_O_RDWR	     02
#define VKI_O_CREAT	   0100	/* not fcntl */
#define VKI_O_EXCL	   0200	/* not fcntl */
#define VKI_O_TRUNC	  01000	/* not fcntl */
#define VKI_O_APPEND	  02000
#define VKI_O_NONBLOCK	  04000
#define VKI_O_LARGEFILE	0100000

#define VKI_AT_FDCWD            -100

#define VKI_F_DUPFD		0	/* dup */
#define VKI_F_GETFD		1	/* get close_on_exec */
#define VKI_F_SETFD		2	/* set/clear close_on_exec */
#define VKI_F_GETFL		3	/* get file->f_flags */
#define VKI_F_SETFL		4	/* set file->f_flags */
#define VKI_F_GETLK		5
#define VKI_F_SETLK		6
#define VKI_F_SETLKW		7

#define VKI_F_SETOWN		8	/*  for sockets. */
#define VKI_F_GETOWN		9	/*  for sockets. */
#define VKI_F_SETSIG		10	/*  for sockets. */
#define VKI_F_GETSIG		11	/*  for sockets. */

#define VKI_F_GETLK64		12	/*  using 'struct flock64' */
#define VKI_F_SETLK64		13
#define VKI_F_SETLKW64		14

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

/* for F_[GET|SET]FL */
#define VKI_FD_CLOEXEC	1	/* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE	1024

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/resource.h
//----------------------------------------------------------------------

#define VKI_RLIMIT_DATA		2	/* max data size */
#define VKI_RLIMIT_STACK	3	/* max stack size */
#define VKI_RLIMIT_CORE		4	/* max core file size */
#define VKI_RLIMIT_NOFILE	7	/* max number of open files */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET	1

#define VKI_SO_TYPE	3

#define VKI_SO_ATTACH_FILTER	26

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCSPGRP           0x8902
#define VKI_SIOCGPGRP           0x8904
#define VKI_SIOCATMARK          0x8905
#define VKI_SIOCGSTAMP          0x8906      /* Get stamp (timeval) */
#define VKI_SIOCGSTAMPNS        0x8907      /* Get stamp (timespec) */

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/stat.h
//----------------------------------------------------------------------

struct vki_stat {
	unsigned long  st_dev;
	unsigned long  st_ino;
	unsigned short st_mode;
	unsigned short st_nlink;
	unsigned short st_uid;
	unsigned short st_gid;
	unsigned long  st_rdev;
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

struct vki_stat64 {
	unsigned long long	st_dev;
	unsigned char	__pad0[4];

#define STAT64_HAS_BROKEN_ST_INO	1
	unsigned long	__st_ino;

	unsigned int	st_mode;
	unsigned int	st_nlink;

	unsigned long	st_uid;
	unsigned long	st_gid;

	unsigned long long	st_rdev;
	unsigned char	__pad3[4];

	long long	st_size;
	unsigned long	st_blksize;

	unsigned long	st_blocks;	/* Number 512-byte blocks allocated. */
	unsigned long	__pad4;		/* future possible st_blocks high bits */

	unsigned long	st_atime;
	unsigned long	st_atime_nsec;

	unsigned long	st_mtime;
	unsigned int	st_mtime_nsec;

	unsigned long	st_ctime;
	unsigned long	st_ctime_nsec;

	unsigned long long	st_ino;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/statfs.h
//----------------------------------------------------------------------

// [[Nb: asm-i386/statfs.h just #include asm-generic/statfs.h directly]]
struct vki_statfs {
	__vki_u32 f_type;
	__vki_u32 f_bsize;
	__vki_u32 f_blocks;
	__vki_u32 f_bfree;
	__vki_u32 f_bavail;
	__vki_u32 f_files;
	__vki_u32 f_ffree;
	__vki_kernel_fsid_t f_fsid;
	__vki_u32 f_namelen;
	__vki_u32 f_frsize;
	__vki_u32 f_spare[5];
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/termios.h
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
// From linux-2.6.8.1/include/asm-i386/termbits.h
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
// From linux-2.6.8.1/include/asm-i386/ioctl.h
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
// From linux-2.6.8.1/include/asm-i386/ioctls.h
//----------------------------------------------------------------------

#define VKI_TCGETS	0x5401
#define VKI_TCSETS	0x5402 /* Clashes with SNDCTL_TMR_START sound ioctl */
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
#define VKI_TIOCSERGETLSR   0x5459 /* Get line status register */

#define VKI_TIOCGICOUNT	0x545D	/* read serial port inline interrupt counts */

//----------------------------------------------------------------------
// From linux-2.6.39-rc2/arch/arm/include/asm/ioctls.h
//----------------------------------------------------------------------

#define VKI_FIOQSIZE 0x545E

//----------------------------------------------------------------------
// From asm-generic/poll.h
//----------------------------------------------------------------------

/* These are specified by iBCS2 */
#define VKI_POLLIN		0x0001

struct vki_pollfd {
	int fd;
	short events;
	short revents;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/user.h
//----------------------------------------------------------------------

struct vki_user_fp {
	struct vki_fp_reg {
		unsigned int sign1:1;
		unsigned int unused:15;
		unsigned int sign2:1;
		unsigned int exponent:14;
		unsigned int j:1;
		unsigned int mantissa1:31;
		unsigned int mantissa0:32;
	} fpregs[8];
	unsigned int fpsr:32;
	unsigned int fpcr:32;
	unsigned char ftype[8];
	unsigned int init_flag;
};

struct vki_user_vfp {
	unsigned long long fpregs[32];
	unsigned long fpscr;
};

#define VKI_IWMMXT_SIZE 0x98

struct vki_iwmmxt_struct {
	unsigned int save[VKI_IWMMXT_SIZE / sizeof(unsigned int)];
};

struct vki_crunch_state {
	unsigned int    mvdx[16][2];
	unsigned int    mvax[4][3];
	unsigned int    dspsc[2];
};

#define VKI_CRUNCH_SIZE sizeof(struct vki_crunch_state)

struct vki_user_regs_struct {
    long uregs[18];
};
#define ARM_cpsr	uregs[16]
#define ARM_pc		uregs[15]
#define ARM_lr		uregs[14]
#define ARM_sp		uregs[13]
#define ARM_ip		uregs[12]
#define ARM_fp		uregs[11]
#define ARM_r10		uregs[10]
#define ARM_r9		uregs[9]
#define ARM_r8		uregs[8]
#define ARM_r7		uregs[7]
#define ARM_r6		uregs[6]
#define ARM_r5		uregs[5]
#define ARM_r4		uregs[4]
#define ARM_r3		uregs[3]
#define ARM_r2		uregs[2]
#define ARM_r1		uregs[1]
#define ARM_r0		uregs[0]
#define ARM_ORIG_r0	uregs[17]
//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/elf.h
//----------------------------------------------------------------------

typedef unsigned long vki_elf_greg_t;

#define VKI_ELF_NGREG (sizeof (struct vki_user_regs_struct) / sizeof(vki_elf_greg_t))
typedef vki_elf_greg_t vki_elf_gregset_t[VKI_ELF_NGREG];

typedef struct vki_user_fp vki_elf_fpregset_t;

#define VKI_AT_SYSINFO		32

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/ucontext.h
//----------------------------------------------------------------------

struct vki_ucontext {
	unsigned long		uc_flags;
	struct vki_ucontext    *uc_link;
	vki_stack_t		uc_stack;
	struct vki_sigcontext	uc_mcontext;
	vki_sigset_t		uc_sigmask;	/* mask last for extensibility */
	int              __unused0[32 - (sizeof (vki_sigset_t) / sizeof (int))];
	unsigned long     uc_regspace[128] __attribute__((__aligned__(8)));

};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/segment.h
//----------------------------------------------------------------------

#define VKI_GDT_ENTRY_TLS_ENTRIES	3
#define VKI_GDT_ENTRY_TLS_MIN	6
#define VKI_GDT_ENTRY_TLS_MAX 	(VKI_GDT_ENTRY_TLS_MIN + VKI_GDT_ENTRY_TLS_ENTRIES - 1)

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/ldt.h
//----------------------------------------------------------------------

/* [[Nb: This is the structure passed to the modify_ldt syscall.  Just so as
   to confuse and annoy everyone, this is _not_ the same as an
   VgLdtEntry and has to be translated into such.  The logic for doing
   so, in vg_ldt.c, is copied from the kernel sources.]] */
struct vki_user_desc {
	unsigned int  entry_number;
	unsigned long base_addr;
	unsigned int  limit;
	unsigned int  seg_32bit:1;
	unsigned int  contents:2;
	unsigned int  read_exec_only:1;
	unsigned int  limit_in_pages:1;
	unsigned int  seg_not_present:1;
	unsigned int  useable:1;
        // [[Nb: this field is not in the kernel sources, but it has always
        // been in the Valgrind sources so I will keep it there in case it's
        // important... this is an x86-defined data structure so who
        // knows;  maybe it's important to set this field to zero at some
        // point.  --njn]]
	unsigned int  reserved:25;
};

// [[Nb: for our convenience within Valgrind, use a more specific name]]
typedef struct vki_user_desc vki_modify_ldt_t;

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/ipcbuf.h
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
	unsigned short		__pad2;
	unsigned long		__unused1;
	unsigned long		__unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/sembuf.h
//----------------------------------------------------------------------

struct vki_semid64_ds {
	struct vki_ipc64_perm sem_perm;		/* permissions .. see ipc.h */
	__vki_kernel_time_t	sem_otime;		/* last semop time */
	unsigned long	__unused1;
	__vki_kernel_time_t	sem_ctime;		/* last change time */
	unsigned long	__unused2;
	unsigned long	sem_nsems;		/* no. of semaphores in array */
	unsigned long	__unused3;
	unsigned long	__unused4;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
	struct vki_ipc64_perm msg_perm;
	__vki_kernel_time_t msg_stime;	/* last msgsnd time */
	unsigned long	__unused1;
	__vki_kernel_time_t msg_rtime;	/* last msgrcv time */
	unsigned long	__unused2;
	__vki_kernel_time_t msg_ctime;	/* last change time */
	unsigned long	__unused3;
	unsigned long  msg_cbytes;	/* current number of bytes on queue */
	unsigned long  msg_qnum;	/* number of messages in queue */
	unsigned long  msg_qbytes;	/* max number of bytes on queue */
	__vki_kernel_pid_t msg_lspid;	/* pid of last msgsnd */
	__vki_kernel_pid_t msg_lrpid;	/* last receive pid */
	unsigned long  __unused4;
	unsigned long  __unused5;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/ipc.h
//----------------------------------------------------------------------

struct vki_ipc_kludge {
	struct vki_msgbuf __user *msgp;
	long msgtyp;
};

#define VKI_SEMOP		 1
#define VKI_SEMGET		 2
#define VKI_SEMCTL		 3
#define VKI_SEMTIMEDOP	 	 4
#define VKI_MSGSND		11
#define VKI_MSGRCV		12
#define VKI_MSGGET		13
#define VKI_MSGCTL		14
#define VKI_SHMAT		21
#define VKI_SHMDT		22
#define VKI_SHMGET		23
#define VKI_SHMCTL		24

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-i386/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
	struct vki_ipc64_perm	shm_perm;	/* operation perms */
	vki_size_t		shm_segsz;	/* size of segment (bytes) */
	__vki_kernel_time_t	shm_atime;	/* last attach time */
	unsigned long		__unused1;
	__vki_kernel_time_t	shm_dtime;	/* last detach time */
	unsigned long		__unused2;
	__vki_kernel_time_t	shm_ctime;	/* last change time */
	unsigned long		__unused3;
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
// DRM ioctls
//----------------------------------------------------------------------

// jrs 20050207: where did all this stuff come from?  Is it really
// i386 specific, or should it go into the linux-generic category?
//struct vki_drm_buf_pub {
//	Int		  idx;	       /**< Index into the master buffer list */
//	Int		  total;       /**< Buffer size */
//	Int		  used;	       /**< Amount of buffer in use (for DMA) */
//	void	  __user *address;     /**< Address of buffer */
//};
//
//struct vki_drm_buf_map {
//	Int	      count;		/**< Length of the buffer list */
//	void	      __user *virtual;	/**< Mmap'd area in user-virtual */
//	struct vki_drm_buf_pub __user *list;	/**< Buffer information */
//};
//
///* We need to pay attention to this, because it mmaps memory */
//#define VKI_DRM_IOCTL_MAP_BUFS		_VKI_IOWR('d', 0x19, struct vki_drm_buf_map)

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-i386/ptrace.h
//----------------------------------------------------------------------

#define VKI_PTRACE_GETREGS            12
#define VKI_PTRACE_SETREGS            13
#define VKI_PTRACE_GETFPREGS          14
#define VKI_PTRACE_SETFPREGS          15
#define VKI_PTRACE_GETWMMXREGS        18
#define VKI_PTRACE_SETWMMXREGS        19
#define VKI_PTRACE_GET_THREAD_AREA    22
#define VKI_PTRACE_SET_SYSCALL        23
#define VKI_PTRACE_GETCRUNCHREGS      25
#define VKI_PTRACE_SETCRUNCHREGS      26
#define VKI_PTRACE_GETVFPREGS         27
#define VKI_PTRACE_SETVFPREGS         28
#define VKI_PTRACE_GETHBPREGS         29
#define VKI_PTRACE_SETHBPREGS         30

//----------------------------------------------------------------------
// From linux-2.6.15.4/include/asm-i386/vm86.h
//----------------------------------------------------------------------

#define VKI_VM86_PLUS_INSTALL_CHECK	0
#define VKI_VM86_ENTER			1
#define VKI_VM86_ENTER_NO_BYPASS	2
#define	VKI_VM86_REQUEST_IRQ		3
#define VKI_VM86_FREE_IRQ		4
#define VKI_VM86_GET_IRQ_BITS		5
#define VKI_VM86_GET_AND_RESET_IRQ	6

struct vki_vm86_regs {
/*
 * normal regs, with special meaning for the segment descriptors..
 */
	long ebx;
	long ecx;
	long edx;
	long esi;
	long edi;
	long ebp;
	long eax;
	long __null_ds;
	long __null_es;
	long __null_fs;
	long __null_gs;
	long orig_eax;
	long eip;
	unsigned short cs, __csh;
	long eflags;
	long esp;
	unsigned short ss, __ssh;
/*
 * these are specific to v86 mode:
 */
	unsigned short es, __esh;
	unsigned short ds, __dsh;
	unsigned short fs, __fsh;
	unsigned short gs, __gsh;
};

struct vki_revectored_struct {
	unsigned long __map[8];			/* 256 bits */
};

struct vki_vm86_struct {
	struct vki_vm86_regs regs;
	unsigned long flags;
	unsigned long screen_bitmap;
	unsigned long cpu_type;
	struct vki_revectored_struct int_revectored;
	struct vki_revectored_struct int21_revectored;
};

struct vki_vm86plus_info_struct {
	unsigned long force_return_for_pic:1;
	unsigned long vm86dbg_active:1;       /* for debugger */
	unsigned long vm86dbg_TFpendig:1;     /* for debugger */
	unsigned long unused:28;
	unsigned long is_vm86pus:1;	      /* for vm86 internal use */
	unsigned char vm86dbg_intxxtab[32];   /* for debugger */
};

struct vki_vm86plus_struct {
	struct vki_vm86_regs regs;
	unsigned long flags;
	unsigned long screen_bitmap;
	unsigned long cpu_type;
	struct vki_revectored_struct int_revectored;
	struct vki_revectored_struct int21_revectored;
	struct vki_vm86plus_info_struct vm86plus;
};

//----------------------------------------------------------------------
// From linux-2.6.35.4/arch/arm/include/asm/hwcap.h
//----------------------------------------------------------------------

#define VKI_HWCAP_NEON      4096

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

//----------------------------------------------------------------------
// And that's it!
//----------------------------------------------------------------------

#endif // __VKI_ARM_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                          vki-arm-linux.h ---*/
/*--------------------------------------------------------------------*/
