
/*--------------------------------------------------------------------*/
/*--- AMD64/Linux-specific kernel interface.     vki-amd64-linux.h ---*/
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

#ifndef __VKI_AMD64_LINUX_H
#define __VKI_AMD64_LINUX_H

// AMD64 is little-endian.
#define VKI_LITTLE_ENDIAN  1

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/types.h
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
// From linux-2.6.9/include/asm-x86_64/page.h
//----------------------------------------------------------------------

#define VKI_PAGE_SHIFT	12
#define VKI_PAGE_SIZE	(1UL << VKI_PAGE_SHIFT)
#define VKI_MAX_PAGE_SHIFT	VKI_PAGE_SHIFT
#define VKI_MAX_PAGE_SIZE	VKI_PAGE_SIZE

//----------------------------------------------------------------------
// From linux-2.6.35.4/arch/x86/include/asm/shmparam.h
//----------------------------------------------------------------------

#define VKI_SHMLBA  VKI_PAGE_SIZE

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/signal.h
//----------------------------------------------------------------------

#define _VKI_NSIG	64
#define _VKI_NSIG_BPW	64
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

#define VKI_SIGRTMIN		32
#define VKI_SIGRTMAX		_VKI_NSIG

#define VKI_SA_NOCLDSTOP	0x00000001
#define VKI_SA_NOCLDWAIT	0x00000002
#define VKI_SA_SIGINFO		0x00000004
#define VKI_SA_ONSTACK		0x08000000
#define VKI_SA_RESTART		0x10000000
#define VKI_SA_NODEFER		0x40000000
#define VKI_SA_RESETHAND	0x80000000

#define VKI_SA_NOMASK	VKI_SA_NODEFER
#define VKI_SA_ONESHOT	VKI_SA_RESETHAND

#define VKI_SA_RESTORER	0x04000000

#define VKI_SS_ONSTACK	1
#define VKI_SS_DISABLE	2

#define VKI_MINSIGSTKSZ	2048

#define VKI_SIG_BLOCK          0	/* for blocking signals */
#define VKI_SIG_UNBLOCK        1	/* for unblocking signals */
#define VKI_SIG_SETMASK        2	/* for setting the signal mask */

typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

#define VKI_SIG_DFL	((__vki_sighandler_t)0)	/* default signal handling */
#define VKI_SIG_IGN	((__vki_sighandler_t)1)	/* ignore signal */

struct vki_sigaction_base {
        // [[Nb: a 'k' prefix is added to "sa_handler" because
        // bits/sigaction.h (which gets dragged in somehow via signal.h)
        // #defines it as something else.  Since that is done for glibc's
        // purposes, which we don't care about here, we use our own name.]]
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
// From linux-2.6.9/include/asm-x86_64/sigcontext.h
//----------------------------------------------------------------------

struct _vki_fpstate {
	__vki_u16	cwd;
	__vki_u16	swd;
	__vki_u16	twd;	/* Note this is not the same as the 32bit/x87/FSAVE twd */
	__vki_u16	fop;
	__vki_u64	rip;
	__vki_u64	rdp; 
	__vki_u32	mxcsr;
	__vki_u32	mxcsr_mask;
	__vki_u32	st_space[32];	/* 8*16 bytes for each FP-reg */
	__vki_u32	xmm_space[64];	/* 16*16 bytes for each XMM-reg  */
	__vki_u32	reserved2[24];
};

struct vki_sigcontext { 
	unsigned long r8;
	unsigned long r9;
	unsigned long r10;
	unsigned long r11;
	unsigned long r12;
	unsigned long r13;
	unsigned long r14;
	unsigned long r15;
	unsigned long rdi;
	unsigned long rsi;
	unsigned long rbp;
	unsigned long rbx;
	unsigned long rdx;
	unsigned long rax;
	unsigned long rcx;
	unsigned long rsp;
	unsigned long rip;
	unsigned long eflags;		/* RFLAGS */
	unsigned short cs;
	unsigned short gs;
	unsigned short fs;
	unsigned short __pad0; 
	unsigned long err;
	unsigned long trapno;
	unsigned long oldmask;
	unsigned long cr2;
	struct _vki_fpstate __user *fpstate;	/* zero when no FPU context */
	unsigned long reserved1[8];
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_READ	0x1		/* page can be read */
#define VKI_PROT_WRITE	0x2		/* page can be written */
#define VKI_PROT_EXEC	0x4		/* page can be executed */
#define VKI_PROT_NONE	0x0		/* page can not be accessed */
#define VKI_PROT_GROWSDOWN	0x01000000	/* mprotect flag: extend change to start of growsdown vma */
#define VKI_PROT_GROWSUP	0x02000000	/* mprotect flag: extend change to end of growsup vma */

#define VKI_MAP_SHARED	0x01		/* Share changes */
#define VKI_MAP_PRIVATE	0x02		/* Changes are private */
#define VKI_MAP_FIXED	0x10		/* Interpret addr exactly */
#define VKI_MAP_ANONYMOUS	0x20	/* don't use a file */
#define VKI_MAP_32BIT	0x40		/* only give out 32bit addresses */
#define VKI_MAP_NORESERVE       0x4000  /* don't check for reservations */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/fcntl.h
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

#define VKI_FD_CLOEXEC	1	/* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE	1024

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/resource.h
//----------------------------------------------------------------------

#define VKI_RLIMIT_DATA		2	/* max data size */
#define VKI_RLIMIT_STACK	3	/* max stack size */
#define VKI_RLIMIT_CORE		4	/* max core file size */
#define VKI_RLIMIT_NOFILE	7	/* max number of open files */

//----------------------------------------------------------------------
// From linux-5.0.0/arch/x86/include/uapi/asm/siginfo.h
//----------------------------------------------------------------------

/* We need that to ensure that sizeof(siginfo) == 128. */
#ifdef __x86_64__
# ifdef __ILP32__
typedef long long __vki_kernel_si_clock_t __attribute__((aligned(4)));
#  define __VKI_ARCH_SI_CLOCK_T             __vki_kernel_si_clock_t
#  define __VKI_ARCH_SI_ATTRIBUTES          __attribute__((aligned(8)))
# else
#  define __VKI_ARCH_SI_PREAMBLE_SIZE (4 * sizeof(int))
# endif
#endif

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET	1

#define VKI_SO_TYPE	3

#define VKI_SO_ATTACH_FILTER	26

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCSPGRP		0x8902
#define VKI_SIOCGPGRP		0x8904
#define VKI_SIOCATMARK		0x8905
#define VKI_SIOCGSTAMP		0x8906		/* Get stamp (timeval) */
#define VKI_SIOCGSTAMPNS	0x8907		/* Get stamp (timespec) */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/stat.h
//----------------------------------------------------------------------

struct vki_stat {
	unsigned long	st_dev;
	unsigned long	st_ino;
	unsigned long	st_nlink;

	unsigned int	st_mode;
	unsigned int	st_uid;
	unsigned int	st_gid;
	unsigned int	__pad0;
	unsigned long	st_rdev;
	long		st_size;
	long		st_blksize;
	long		st_blocks;	/* Number 512-byte blocks allocated. */

	unsigned long	st_atime;
	unsigned long 	st_atime_nsec; 
	unsigned long	st_mtime;
	unsigned long	st_mtime_nsec;
	unsigned long	st_ctime;
	unsigned long   st_ctime_nsec;
  	long		__unused0[3];
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/statfs.h
//----------------------------------------------------------------------

struct vki_statfs {
	long f_type;
	long f_bsize;
	long f_blocks;
	long f_bfree;
	long f_bavail;
	long f_files;
	long f_ffree;
	__vki_kernel_fsid_t f_fsid;
	long f_namelen;
	long f_frsize;
	long f_spare[5];
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/termios.h
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
// From linux-2.6.9/include/asm-x86_64/termbits.h
//----------------------------------------------------------------------

typedef unsigned char	vki_cc_t;
typedef unsigned int	vki_tcflag_t;

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
// From linux-2.6.9/include/asm-x86_64/ioctl.h
//----------------------------------------------------------------------

#define _VKI_IOC_NRBITS		8
#define _VKI_IOC_TYPEBITS	8
#define _VKI_IOC_SIZEBITS	14
#define _VKI_IOC_DIRBITS	2

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

#define _VKI_IO(type,nr)		_VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ,(type),(nr),sizeof(size))
#define _VKI_IOW(type,nr,size)	_VKI_IOC(_VKI_IOC_WRITE,(type),(nr),sizeof(size))
#define _VKI_IOWR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE,(type),(nr),sizeof(size))

#define _VKI_IOC_DIR(nr)		(((nr) >> _VKI_IOC_DIRSHIFT) & _VKI_IOC_DIRMASK)
#define _VKI_IOC_SIZE(nr)		(((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/ioctls.h
//----------------------------------------------------------------------

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

#define VKI_FIONCLEX    0x5450
#define VKI_FIOCLEX     0x5451
#define VKI_FIOASYNC	0x5452
#define VKI_TIOCSERGETLSR   0x5459 /* Get line status register */

#define VKI_TIOCGICOUNT	0x545D	/* read serial port inline interrupt counts */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/poll.h
//----------------------------------------------------------------------

#define VKI_POLLIN		0x0001

struct vki_pollfd {
	int fd;
	short events;
	short revents;
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/user.h
//----------------------------------------------------------------------

struct vki_user_i387_struct {
	unsigned short	cwd;
	unsigned short	swd;
	unsigned short	twd; /* Note this is not the same as the 32bit/x87/FSAVE twd */
	unsigned short	fop;
	__vki_u64	rip;
	__vki_u64	rdp;
	__vki_u32	mxcsr;
	__vki_u32	mxcsr_mask;
	__vki_u32	st_space[32];	/* 8*16 bytes for each FP-reg = 128 bytes */
	__vki_u32	xmm_space[64];	/* 16*16 bytes for each XMM-reg = 256 bytes */
	__vki_u32	padding[24];
};

struct vki_user_regs_struct {
	unsigned long r15,r14,r13,r12,rbp,rbx,r11,r10;
	unsigned long r9,r8,rax,rcx,rdx,rsi,rdi,orig_rax;
	unsigned long rip,cs,eflags;
	unsigned long rsp,ss;
  	unsigned long fs_base, gs_base;
	unsigned long ds,es,fs,gs; 
}; 

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/elf.h
//----------------------------------------------------------------------

typedef unsigned long vki_elf_greg_t;

#define VKI_ELF_NGREG (sizeof (struct vki_user_regs_struct) / sizeof(vki_elf_greg_t))
typedef vki_elf_greg_t vki_elf_gregset_t[VKI_ELF_NGREG];

typedef struct vki_user_i387_struct vki_elf_fpregset_t;

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/ucontext.h
//----------------------------------------------------------------------

struct vki_ucontext {
	unsigned long		uc_flags;
	struct vki_ucontext    *uc_link;
	vki_stack_t		uc_stack;
	struct vki_sigcontext	uc_mcontext;
	vki_sigset_t		uc_sigmask;	/* mask last for extensibility */
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-x86_64/segment.h
//----------------------------------------------------------------------

#define VKI_GDT_ENTRY_TLS_ENTRIES 3

#define VKI_GDT_ENTRY_TLS_MIN 11
#define VKI_GDT_ENTRY_TLS_MAX 13

//----------------------------------------------------------------------
// From linux-2.6.11.9/include/asm-x86_64/prctl.h
//----------------------------------------------------------------------

#define VKI_ARCH_SET_GS 0x1001
#define VKI_ARCH_SET_FS 0x1002
#define VKI_ARCH_GET_FS 0x1003
#define VKI_ARCH_GET_GS 0x1004

//----------------------------------------------------------------------
// Originally from linux-2.6.9/include/asm-x86_64/ldt.h
//----------------------------------------------------------------------

// Note that the type here is very slightly different to the
// type for x86 (the final 'lm' field is added).
/* The explanation is: the final bit is not present in 32 bit code running
   on 64 bits kernel. The kernel has to assume this value is 0 whenever
   user_desc arrives from a 32-bit program.
   See /usr/include/asm/ldt.h. */

/* [[Nb: This is the structure passed to the modify_ldt syscall.  Just so as
   to confuse and annoy everyone, this is _not_ the same as an
   VgLdtEntry and has to be translated into such.  The logic for doing
   so, in vg_ldt.c, is copied from the kernel sources.]] */
/* Note also that a comment in ldt.h indicates that the below
   contains several fields ignored on 64bit, and that modify_ldt
   is rather for 32bit. */
struct vki_user_desc {
	unsigned int  entry_number;
	unsigned int  base_addr;
	unsigned int  limit;
	unsigned int  seg_32bit:1;
	unsigned int  contents:2;
	unsigned int  read_exec_only:1;
	unsigned int  limit_in_pages:1;
	unsigned int  seg_not_present:1;
	unsigned int  useable:1;
        unsigned int  lm:1;
};

// [[Nb: for our convenience within Valgrind, use a more specific name]]
typedef struct vki_user_desc vki_modify_ldt_t;

//----------------------------------------------------------------------
// From linux-2.6.11.2/include/asm-x86_64/ipcbuf.h
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
// From linux-2.6.11.2/include/asm-x86_64/sembuf.h
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
// From linux-2.6.11.2/include/asm-x86_64/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
	struct vki_ipc64_perm msg_perm;
	__vki_kernel_time_t msg_stime;	/* last msgsnd time */
	__vki_kernel_time_t msg_rtime;	/* last msgrcv time */
	__vki_kernel_time_t msg_ctime;	/* last change time */
	unsigned long  msg_cbytes;	/* current number of bytes on queue */
	unsigned long  msg_qnum;	/* number of messages in queue */
	unsigned long  msg_qbytes;	/* max number of bytes on queue */
	__vki_kernel_pid_t msg_lspid;	/* pid of last msgsnd */
	__vki_kernel_pid_t msg_lrpid;	/* last receive pid */
	unsigned long  __unused4;
	unsigned long  __unused5;
};

//----------------------------------------------------------------------
// From linux-2.6.11.2/include/asm-x86_64/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
	struct vki_ipc64_perm	shm_perm;	/* operation perms */
	vki_size_t		shm_segsz;	/* size of segment (bytes) */
	__vki_kernel_time_t	shm_atime;	/* last attach time */
	__vki_kernel_time_t	shm_dtime;	/* last detach time */
	__vki_kernel_time_t	shm_ctime;	/* last change time */
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
// From linux-2.6.12.2/include/asm-x86_64/ptrace.h
//----------------------------------------------------------------------

#define VKI_PTRACE_GETREGS            12
#define VKI_PTRACE_SETREGS            13
#define VKI_PTRACE_GETFPREGS          14
#define VKI_PTRACE_SETFPREGS          15

// From /usr/include/asm/ptrace-abit.h
/* only useful for access 32bit programs / kernels */
#define VKI_PTRACE_GET_THREAD_AREA    25
#define VKI_PTRACE_SET_THREAD_AREA    26

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

#endif // __VKI_AMD64_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
