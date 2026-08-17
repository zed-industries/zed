
/*--------------------------------------------------------------------*/
/*--- PPC32/Linux-specific kernel interface.     vki-ppc32-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2005-2017 Julian Seward
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

#ifndef __VKI_PPC32_LINUX_H
#define __VKI_PPC32_LINUX_H

// ppc32 is big-endian.
#define VKI_BIG_ENDIAN  1

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/types.h
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

typedef struct {
        __vki_u32 u[4];
} __vki_vector128;

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/page.h
//----------------------------------------------------------------------

/* PAGE_SHIFT determines the page size, unfortunately
   page size might vary between 32-bit and 64-bit ppc kernels */
extern UWord VKI_PAGE_SHIFT;
extern UWord VKI_PAGE_SIZE;
#define VKI_MAX_PAGE_SHIFT	16
#define VKI_MAX_PAGE_SIZE	(1UL << VKI_MAX_PAGE_SHIFT)

//----------------------------------------------------------------------
// From linux-2.6.35.4/arch/powerpc/include/asm/shmparam.h
//----------------------------------------------------------------------

#define VKI_SHMLBA  VKI_PAGE_SIZE

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/signal.h
//----------------------------------------------------------------------

#define VKI_MINSIGSTKSZ	2048

#define VKI_SIG_BLOCK         0    /* for blocking signals */
#define VKI_SIG_UNBLOCK       1    /* for unblocking signals */
#define VKI_SIG_SETMASK       2    /* for setting the signal mask */

/* Type of a signal handler.  */
typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

#define VKI_SIG_DFL     ((__vki_sighandler_t)0)     /* default signal handling */
#define VKI_SIG_IGN     ((__vki_sighandler_t)1)     /* ignore signal */

#define _VKI_NSIG       64
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
#define VKI_SIGUNUSED		31

/* These should not be considered constants from userland.  */
#define VKI_SIGRTMIN    32
// [[This was (_NSIG-1) in 2.4.X... not sure if it matters.]]
#define VKI_SIGRTMAX    _VKI_NSIG

#define VKI_SA_NOCLDSTOP	0x00000001
#define VKI_SA_NOCLDWAIT	0x00000002
#define VKI_SA_SIGINFO		0x00000004
#define VKI_SA_ONSTACK		0x08000000
#define VKI_SA_RESTART		0x10000000
#define VKI_SA_NODEFER		0x40000000
#define VKI_SA_RESETHAND	0x80000000

#define VKI_SA_NOMASK		VKI_SA_NODEFER
#define VKI_SA_ONESHOT		VKI_SA_RESETHAND
//#define VKI_SA_INTERRUPT	0x20000000 /* dummy -- ignored */

#define VKI_SA_RESTORER		0x04000000

#define VKI_SS_ONSTACK		1
#define VKI_SS_DISABLE		2

/* These are 'legacy' sigactions in which the size of sa_mask is fixed
   (cannot be expanded at any future point) because it is sandwiched
   between two other fields.
   (there is identical kludgery in vki-x86-linux.h) */
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
// From linux-2.6.9/include/asm-ppc/ptrace.h
//----------------------------------------------------------------------

struct vki_pt_regs {
        unsigned long gpr[32];
        unsigned long nip;
        unsigned long msr;
        unsigned long orig_gpr3;        /* Used for restarting system calls */
        unsigned long ctr;
        unsigned long link;
        unsigned long xer;
        unsigned long ccr;
        unsigned long mq;               /* 601 only (not used at present) */
                                        /* Used on APUS to hold IPL value. */
        unsigned long trap;             /* Reason for being here */
        /* N.B. for critical exceptions on 4xx, the dar and dsisr
           fields are overloaded to hold srr0 and srr1. */
        unsigned long dar;              /* Fault registers */
        unsigned long dsisr;            /* on 4xx/Book-E used for ESR */
        unsigned long result;           /* Result of a system call */

        /* Not in kernel's definition, but apparently needed to stop
           assertion at coredump-elf.c:267 firing.  These padding
           words make the struct have the same size as a
           'vki_elf_greg_t'.  See message from Ghassan Hammouri on
           valgrind-developers on 6 April 06. */
        unsigned long pad[4];
};

#define vki_user_regs_struct vki_pt_regs

#define VKI_PT_R0		0
#define VKI_PT_R1		1
#define VKI_PT_R2		2
#define VKI_PT_R3		3
#define VKI_PT_R4		4
#define VKI_PT_R5		5
#define VKI_PT_R6		6
#define VKI_PT_R7		7
#define VKI_PT_R8		8
#define VKI_PT_R9		9
#define VKI_PT_R10		10
#define VKI_PT_R11		11
#define VKI_PT_R12		12
#define VKI_PT_R13		13
#define VKI_PT_R14		14
#define VKI_PT_R15		15
#define VKI_PT_R16		16
#define VKI_PT_R17		17
#define VKI_PT_R18		18
#define VKI_PT_R19		19
#define VKI_PT_R20		20
#define VKI_PT_R21		21
#define VKI_PT_R22		22
#define VKI_PT_R23		23
#define VKI_PT_R24		24
#define VKI_PT_R25		25
#define VKI_PT_R26		26
#define VKI_PT_R27		27
#define VKI_PT_R28		28
#define VKI_PT_R29		29
#define VKI_PT_R30		30
#define VKI_PT_R31		31
#define VKI_PT_NIP		32
#define VKI_PT_MSR		33
#define VKI_PT_ORIG_R3		34
#define VKI_PT_CTR		35
#define VKI_PT_LNK		36
#define VKI_PT_XER		37
#define VKI_PT_CCR		38
#define VKI_PT_MQ		39
#define VKI_PT_TRAP		40
#define VKI_PT_DAR		41
#define VKI_PT_DSISR		42
#define VKI_PT_RESULT		43

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/sigcontext.h
//----------------------------------------------------------------------

struct vki_sigcontext {
        unsigned long      _unused[4];
        int                signal;
        unsigned long      handler;
        unsigned long      oldmask;
        struct vki_pt_regs *regs;
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_NONE		0x0      /* No page permissions */
#define VKI_PROT_READ		0x1      /* page can be read */
#define VKI_PROT_WRITE		0x2      /* page can be written */
#define VKI_PROT_EXEC		0x4      /* page can be executed */
#define VKI_PROT_GROWSDOWN	0x01000000	/* mprotect flag: extend change to start of growsdown vma */
#define VKI_PROT_GROWSUP	0x02000000	/* mprotect flag: extend change to end of growsup vma */

#define VKI_MAP_SHARED		0x01     /* Share changes */
#define VKI_MAP_PRIVATE		0x02     /* Changes are private */
//#define VKI_MAP_TYPE		0x0f     /* Mask for type of mapping */
#define VKI_MAP_FIXED		0x10     /* Interpret addr exactly */
#define VKI_MAP_ANONYMOUS	0x20     /* don't use a file */
#define VKI_MAP_NORESERVE	0x40     /* don't reserve swap pages */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/fcntl.h
//----------------------------------------------------------------------

#define VKI_O_ACCMODE		   03
#define VKI_O_RDONLY		   00
#define VKI_O_WRONLY		   01
#define VKI_O_RDWR		   02
#define VKI_O_CREAT		 0100		/* not fcntl */
#define VKI_O_EXCL		 0200		/* not fcntl */
#define VKI_O_TRUNC		01000		/* not fcntl */
#define VKI_O_APPEND		02000
#define VKI_O_NONBLOCK		04000
#define VKI_O_LARGEFILE     0200000

#define VKI_AT_FDCWD            -100

#define VKI_F_DUPFD		 0			/* dup */
#define VKI_F_GETFD		 1			/* get close_on_exec */
#define VKI_F_SETFD		 2			/* set/clear close_on_exec */
#define VKI_F_GETFL		 3			/* get file->f_flags */
#define VKI_F_SETFL		 4			/* set file->f_flags */
#define VKI_F_GETLK		 5
#define VKI_F_SETLK		 6
#define VKI_F_SETLKW		 7

#define VKI_F_SETOWN		 8			/*  for sockets. */
#define VKI_F_GETOWN		 9			/*  for sockets. */
#define VKI_F_SETSIG		10			/*  for sockets. */
#define VKI_F_GETSIG		11			/*  for sockets. */

#define VKI_F_GETLK64		12			/*  using 'struct flock64' */
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
#define VKI_FD_CLOEXEC	 1		/* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE	1024

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/resource.h
//----------------------------------------------------------------------

#define VKI_RLIMIT_DATA		2   /* max data size */
#define VKI_RLIMIT_STACK	3   /* max stack size */
#define VKI_RLIMIT_CORE		4   /* max core file size */
#define VKI_RLIMIT_NOFILE	7   /* max number of open files */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET	1

#define VKI_SO_TYPE	3

#define VKI_SO_ATTACH_FILTER	26

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-ppc/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCSPGRP		0x8902
#define VKI_SIOCGPGRP		0x8904
#define VKI_SIOCATMARK		0x8905
#define VKI_SIOCGSTAMP		0x8906          /* Get stamp (timeval) */
#define VKI_SIOCGSTAMPNS	0x8907          /* Get stamp (timespec) */

//----------------------------------------------------------------------
// From linux-2.6.10/include/asm-ppc/stat.h
//----------------------------------------------------------------------

//.. #define VKI_S_IFMT		00170000
//.. #define VKI_S_IFSOCK	 0140000
//.. #define VKI_S_IFLNK	 0120000
//.. #define VKI_S_IFREG	 0100000
//.. #define VKI_S_IFBLK	 0060000
//.. #define VKI_S_IFDIR	 0040000
//.. #define VKI_S_IFCHR	 0020000
//.. #define VKI_S_IFIFO	 0010000
//.. #define VKI_S_ISUID	 0004000
//.. #define VKI_S_ISGID	 0002000
//.. #define VKI_S_ISVTX	 0001000
//.. 
//.. #define VKI_S_ISLNK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFLNK)
//.. #define VKI_S_ISREG(m)	(((m) & VKI_S_IFMT) == VKI_S_IFREG)
//.. #define VKI_S_ISDIR(m)	(((m) & VKI_S_IFMT) == VKI_S_IFDIR)
//.. #define VKI_S_ISCHR(m)	(((m) & VKI_S_IFMT) == VKI_S_IFCHR)
//.. #define VKI_S_ISBLK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFBLK)
//.. #define VKI_S_ISFIFO(m)	(((m) & VKI_S_IFMT) == VKI_S_IFIFO)
//.. #define VKI_S_ISSOCK(m)	(((m) & VKI_S_IFMT) == VKI_S_IFSOCK)

struct vki_stat {
   unsigned		st_dev;
   unsigned long	st_ino;
   unsigned int		st_mode;
   unsigned short	st_nlink;
   unsigned int		st_uid;
   unsigned int		st_gid;
   unsigned		st_rdev;
   long			st_size;
   unsigned long	st_blksize;
   unsigned long	st_blocks;
   unsigned long	st_atime;
   unsigned long	st_atime_nsec;
   unsigned long	st_mtime;
   unsigned long	st_mtime_nsec;
   unsigned long	st_ctime;
   unsigned long	st_ctime_nsec;
   unsigned long	__unused4;
   unsigned long	__unused5;
};

struct vki_stat64 {
   unsigned long long   st_dev;
   unsigned long long   st_ino;
   unsigned int         st_mode;
   unsigned int         st_nlink;
   unsigned int         st_uid;
   unsigned int         st_gid;
   unsigned long long   st_rdev;
   unsigned short int   __pad2;
   long long            st_size;
   long                 st_blksize;

   long long            st_blocks;
   long                 st_atime;
   unsigned long        st_atime_nsec;
   long                 st_mtime;
   unsigned long int    st_mtime_nsec;
   long                 st_ctime;
   unsigned long int    st_ctime_nsec;
   unsigned long int    __unused4;
   unsigned long int    __unused5;
};


//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/statfs.h
//----------------------------------------------------------------------

// [[Nb: asm-ppc/statfs.h just #include asm-generic/statfs.h directly]]
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
// From linux-2.6.9/include/asm-ppc/termios.h
//----------------------------------------------------------------------

struct vki_winsize {
   unsigned short ws_row;
   unsigned short ws_col;
   unsigned short ws_xpixel;
   unsigned short ws_ypixel;
};

#define NCC 10
struct vki_termio {
   unsigned short	c_iflag;		/* input mode flags */
   unsigned short	c_oflag;		/* output mode flags */
   unsigned short	c_cflag;		/* control mode flags */
   unsigned short	c_lflag;		/* local mode flags */
   unsigned char	c_line;			/* line discipline */
   unsigned char	c_cc[NCC];		/* control characters */
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/termbits.h
//----------------------------------------------------------------------

typedef unsigned char   vki_cc_t;
typedef unsigned int    vki_speed_t;
typedef unsigned int    vki_tcflag_t;

#define NCCS 19
struct vki_termios {
        vki_tcflag_t	c_iflag;		/* input mode flags */
        vki_tcflag_t	c_oflag;		/* output mode flags */
        vki_tcflag_t	c_cflag;		/* control mode flags */
        vki_tcflag_t	c_lflag;		/* local mode flags */
        vki_cc_t	c_cc[NCCS];		/* control characters */
        vki_cc_t	c_line;			/* line discipline (== c_cc[19]) */
        vki_speed_t	c_ispeed;		/* input speed */
        vki_speed_t	c_ospeed;		/* output speed */
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/ioctl.h
//----------------------------------------------------------------------

#define _VKI_IOC_NRBITS		 8
#define _VKI_IOC_TYPEBITS	 8
#define _VKI_IOC_SIZEBITS	13
#define _VKI_IOC_DIRBITS	 3

#define _VKI_IOC_NRMASK		((1 << _VKI_IOC_NRBITS)-1)
#define _VKI_IOC_TYPEMASK	((1 << _VKI_IOC_TYPEBITS)-1)
#define _VKI_IOC_SIZEMASK	((1 << _VKI_IOC_SIZEBITS)-1)
#define _VKI_IOC_DIRMASK	((1 << _VKI_IOC_DIRBITS)-1)

#define _VKI_IOC_NRSHIFT	0
#define _VKI_IOC_TYPESHIFT	(_VKI_IOC_NRSHIFT+_VKI_IOC_NRBITS)
#define _VKI_IOC_SIZESHIFT	(_VKI_IOC_TYPESHIFT+_VKI_IOC_TYPEBITS)
#define _VKI_IOC_DIRSHIFT	(_VKI_IOC_SIZESHIFT+_VKI_IOC_SIZEBITS)

#define _VKI_IOC_NONE	1U
#define _VKI_IOC_READ	2U
#define _VKI_IOC_WRITE	4U

#define _VKI_IOC(dir,type,nr,size) \
        (((dir)  << _VKI_IOC_DIRSHIFT) | \
         ((type) << _VKI_IOC_TYPESHIFT) | \
         ((nr)   << _VKI_IOC_NRSHIFT) | \
         ((size) << _VKI_IOC_SIZESHIFT))

/* used to create numbers */
#define _VKI_IO(type,nr)			_VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ,(type),(nr),(_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOW(type,nr,size)	_VKI_IOC(_VKI_IOC_WRITE,(type),(nr),(_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOWR(type,nr,size)	_VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE,(type),(nr),(_VKI_IOC_TYPECHECK(size)))

/* used to decode them.. */
#define _VKI_IOC_DIR(nr)		(((nr) >> _VKI_IOC_DIRSHIFT)  & _VKI_IOC_DIRMASK)
//.. #define _VKI_IOC_TYPE(nr)		(((nr) >> _VKI_IOC_TYPESHIFT) & _VKI_IOC_TYPEMASK)
//.. #define _VKI_IOC_NR(nr)		(((nr) >> _VKI_IOC_NRSHIFT)   & _VKI_IOC_NRMASK)
#define _VKI_IOC_SIZE(nr)		(((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/ioctls.h
//----------------------------------------------------------------------

#define VKI_FIOCLEX		_VKI_IO('f', 1)
#define VKI_FIONCLEX		_VKI_IO('f', 2)
#define VKI_FIOASYNC		_VKI_IOW('f', 125, int)
#define VKI_FIONBIO		_VKI_IOW('f', 126, int)
#define VKI_FIONREAD		_VKI_IOR('f', 127, int)
//#define VKI_TIOCINQ		VKI_FIONREAD
#define VKI_FIOQSIZE		_VKI_IOR('f', 128, vki_loff_t)

//#define VKI_TIOCGETP		_VKI_IOR('t', 8, struct vki_sgttyb)
//#define VKI_TIOCSETP		_VKI_IOW('t', 9, struct vki_sgttyb)
//#define VKI_TIOCSETN		_VKI_IOW('t', 10, struct vki_sgttyb)    /* TIOCSETP wo flush */

//#define VKI_TIOCSETC		_VKI_IOW('t', 17, struct vki_tchars)
//#define VKI_TIOCGETC		_VKI_IOR('t', 18, struct vki_tchars)
#define VKI_TCGETS		_VKI_IOR('t', 19, struct vki_termios)
#define VKI_TCSETS		_VKI_IOW('t', 20, struct vki_termios)
#define VKI_TCSETSW		_VKI_IOW('t', 21, struct vki_termios)
#define VKI_TCSETSF		_VKI_IOW('t', 22, struct vki_termios)

#define VKI_TCGETA		_VKI_IOR('t', 23, struct vki_termio)
#define VKI_TCSETA		_VKI_IOW('t', 24, struct vki_termio)
#define VKI_TCSETAW		_VKI_IOW('t', 25, struct vki_termio)
#define VKI_TCSETAF		_VKI_IOW('t', 28, struct vki_termio)

#define VKI_TCSBRK		_VKI_IO('t', 29)
#define VKI_TCXONC		_VKI_IO('t', 30)
#define VKI_TCFLSH		_VKI_IO('t', 31)

#define VKI_TIOCSWINSZ		_VKI_IOW('t', 103, struct vki_winsize)
#define VKI_TIOCGWINSZ		_VKI_IOR('t', 104, struct vki_winsize)
//#define VKI_TIOCSTART		_VKI_IO('t', 110)	   /* start output, like ^Q */
//#define VKI_TIOCSTOP		_VKI_IO('t', 111)	   /* stop output, like ^S */
#define VKI_TIOCOUTQ		_VKI_IOR('t', 115, int)	   /* output queue size */

//#define VKI_TIOCGLTC		_VKI_IOR('t', 116, struct vki_ltchars)
//#define VKI_TIOCSLTC		_VKI_IOW('t', 117, struct vki_ltchars)
#define VKI_TIOCSPGRP		_VKI_IOW('t', 118, int)
#define VKI_TIOCGPGRP		_VKI_IOR('t', 119, int)

//#define VKI_TIOCEXCL		0x540C
//#define VKI_TIOCNXCL		0x540D
#define VKI_TIOCSCTTY		0x540E

//#define VKI_TIOCSTI		0x5412
#define VKI_TIOCMGET		0x5415
#define VKI_TIOCMBIS		0x5416
#define VKI_TIOCMBIC		0x5417
#define VKI_TIOCMSET		0x5418
//# define VKI_TIOCM_LE		0x001
//# define VKI_TIOCM_DTR	0x002
//# define VKI_TIOCM_RTS	0x004
//# define VKI_TIOCM_ST		0x008
//# define VKI_TIOCM_SR		0x010
//# define VKI_TIOCM_CTS	0x020
//# define VKI_TIOCM_CAR	0x040
//# define VKI_TIOCM_RNG	0x080
//# define VKI_TIOCM_DSR	0x100
//# define VKI_TIOCM_CD		VKI_TIOCM_CAR
//# define VKI_TIOCM_RI		VKI_TIOCM_RNG

//#define VKI_TIOCGSOFTCAR	0x5419
//#define VKI_TIOCSSOFTCAR	0x541A
#define VKI_TIOCLINUX		0x541C
//#define VKI_TIOCCONS		0x541D
#define VKI_TIOCGSERIAL	0x541E
#define VKI_TIOCSSERIAL	0x541F
//#define VKI_TIOCPKT		0x5420
//# define VKI_TIOCPKT_DATA		 0
//# define VKI_TIOCPKT_FLUSHREAD	 1
//# define VKI_TIOCPKT_FLUSHWRITE	 2
//# define VKI_TIOCPKT_STOP		 4
//# define VKI_TIOCPKT_START		 8
//# define VKI_TIOCPKT_NOSTOP		16
//# define VKI_TIOCPKT_DOSTOP		32

#define VKI_TIOCNOTTY		0x5422
//#define VKI_TIOCSETD		0x5423
//#define VKI_TIOCGETD		0x5424
#define VKI_TCSBRKP		0x5425  /* Needed for POSIX tcsendbreak() */
//#define VKI_TIOCSBRK		0x5427  /* BSD compatibility */
//#define VKI_TIOCCBRK		0x5428  /* BSD compatibility */
//#define VKI_TIOCGSID		0x5429  /* Return the session ID of FD */
#define VKI_TIOCGPTN		_VKI_IOR('T',0x30, unsigned int) /* Get Pty Number (of pty-mux device) */
#define VKI_TIOCSPTLCK		_VKI_IOW('T',0x31, int)  /* Lock/unlock Pty */

//#define VKI_TIOCSERCONFIG	0x5453
//#define VKI_TIOCSERGWILD	0x5454
//#define VKI_TIOCSERSWILD  	0x5455
//#define VKI_TIOCGLCKTRMIOS	0x5456
//#define VKI_TIOCSLCKTRMIOS	0x5457
//#define VKI_TIOCSERGSTRUCT	0x5458 /* For debugging only */
#define VKI_TIOCSERGETLSR	0x5459 /* Get line status register */
  /* ioctl (fd, VKI_TIOCSERGETLSR, &result) where result may be as below */
//# define VKI_TIOCSER_TEMT	0x01   /* Transmitter physically empty */
//#define VKI_TIOCSERGETMULTI	0x545A /* Get multiport config  */
//#define VKI_TIOCSERSETMULTI	0x545B /* Set multiport config */

//#define VKI_TIOCMIWAIT	0x545C  /* wait for a change on serial input line(s) */
#define VKI_TIOCGICOUNT		0x545D  /* read serial port inline interrupt counts */

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/poll.h
//----------------------------------------------------------------------

//.. /* These are specified by iBCS2 */
//.. #define VKI_POLLIN		0x0001

struct vki_pollfd {
	int fd;
	short events;
	short revents;
};

//.. //----------------------------------------------------------------------
//.. // From linux-2.6.8.1/include/asm-i386/user.h
//.. //----------------------------------------------------------------------
//.. 
//.. struct vki_user_i387_struct {
//.. 	long	cwd;
//.. 	long	swd;
//.. 	long	twd;
//.. 	long	fip;
//.. 	long	fcs;
//.. 	long	foo;
//.. 	long	fos;
//.. 	long	st_space[20];	/* 8*10 bytes for each FP-reg = 80 bytes */
//.. };
//.. 
//.. struct vki_user_fxsr_struct {
//.. 	unsigned short	cwd;
//.. 	unsigned short	swd;
//.. 	unsigned short	twd;
//.. 	unsigned short	fop;
//.. 	long	fip;
//.. 	long	fcs;
//.. 	long	foo;
//.. 	long	fos;
//.. 	long	mxcsr;
//.. 	long	reserved;
//.. 	long	st_space[32];	/* 8*16 bytes for each FP-reg = 128 bytes */
//.. 	long	xmm_space[32];	/* 8*16 bytes for each XMM-reg = 128 bytes */
//.. 	long	padding[56];
//.. };
//.. 
//.. /*
//..  * This is the old layout of "struct pt_regs", and
//..  * is still the layout used by user mode (the new
//..  * pt_regs doesn't have all registers as the kernel
//..  * doesn't use the extra segment registers)
//..  */
//.. struct vki_user_regs_struct {
//.. 	long ebx, ecx, edx, esi, edi, ebp, eax;
//.. 	unsigned short ds, __ds, es, __es;
//.. 	unsigned short fs, __fs, gs, __gs;
//.. 	long orig_eax, eip;
//.. 	unsigned short cs, __cs;
//.. 	long eflags, esp;
//.. 	unsigned short ss, __ss;
//.. };

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/elf.h
//----------------------------------------------------------------------

#define VKI_ELF_NGREG			48	/* includes nip, msr, lr, etc. */
#define VKI_ELF_NFPREG			33	/* includes fpscr */
#define VKI_ELF_NVRREG			33	/* includes vscr */

/* General registers */
typedef unsigned long vki_elf_greg_t;
typedef vki_elf_greg_t vki_elf_gregset_t[VKI_ELF_NGREG];

/* Floating point registers */
typedef double vki_elf_fpreg_t;
typedef vki_elf_fpreg_t vki_elf_fpregset_t[VKI_ELF_NFPREG];

/* Altivec registers */
typedef __vki_vector128 vki_elf_vrreg_t;
typedef vki_elf_vrreg_t vki_elf_vrregset_t[VKI_ELF_NVRREG];

#define VKI_AT_DCACHEBSIZE		19
#define VKI_AT_ICACHEBSIZE		20
#define VKI_AT_UCACHEBSIZE		21
/* A special ignored type value for PPC, for glibc compatibility.  */
#define VKI_AT_IGNOREPPC	  	22

/* CAB: Do we want these? */
//#define VKI_AT_SYSINFO		32
//#define VKI_AT_SYSINFO_EHDR  		33

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/ucontext.h
//----------------------------------------------------------------------

struct vki_mcontext {
        vki_elf_gregset_t	mc_gregs;
        vki_elf_fpregset_t	mc_fregs;
        unsigned long		mc_pad[2];
        vki_elf_vrregset_t	mc_vregs __attribute__((__aligned__(16)));
};

struct vki_ucontext {
        unsigned long		uc_flags;
        struct vki_ucontext	__user *uc_link;
        vki_stack_t		uc_stack;
        int			uc_pad[7];
        struct vki_mcontext	__user *uc_regs;		/* points to uc_mcontext field */
        vki_sigset_t		uc_sigmask;
        /* glibc has 1024-bit signal masks, ours are 64-bit */
        int			uc_maskext[30];
        int			uc_pad2[3];
        struct vki_mcontext	uc_mcontext;
};

//.. //----------------------------------------------------------------------
//.. // From linux-2.6.8.1/include/asm-i386/segment.h
//.. //----------------------------------------------------------------------
//.. 
//.. #define VKI_GDT_ENTRY_TLS_ENTRIES	3
//.. #define VKI_GDT_ENTRY_TLS_MIN	6
//.. #define VKI_GDT_ENTRY_TLS_MAX 	(VKI_GDT_ENTRY_TLS_MIN + VKI_GDT_ENTRY_TLS_ENTRIES - 1)

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/ldt.h
//----------------------------------------------------------------------

//.. /* [[Nb: This is the structure passed to the modify_ldt syscall.  Just so as
//..    to confuse and annoy everyone, this is _not_ the same as an
//..    VgLdtEntry and has to be translated into such.  The logic for doing
//..    so, in vg_ldt.c, is copied from the kernel sources.]] */
//.. struct vki_user_desc {
//.. 	unsigned int	entry_number;
//.. 	unsigned long	base_addr;
//.. 	unsigned int	limit;
//.. 	unsigned int	seg_32bit:1;
//.. 	unsigned int	contents:2;
//.. 	unsigned int	read_exec_only:1;
//.. 	unsigned int	limit_in_pages:1;
//.. 	unsigned int	seg_not_present:1;
//.. 	unsigned int	useable:1;
//..         // [[Nb: this field is not in the kernel sources, but it has always
//..         // been in the Valgrind sources so I will keep it there in case it's
//..         // important... this is an x86-defined data structure so who
//..         // knows;  maybe it's important to set this field to zero at some
//..         // point.  --njn]]
//.. 	unsigned int	reserved:25;
//.. };
//.. 
//.. // [[Nb: for our convenience within Valgrind, use a more specific name]]

// CAB: TODO
typedef char vki_modify_ldt_t;


//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/ipcbuf.h
//----------------------------------------------------------------------

struct vki_ipc64_perm
{
   __vki_kernel_key_t	key;
   __vki_kernel_uid_t	uid;
   __vki_kernel_gid_t	gid;
   __vki_kernel_uid_t	cuid;
   __vki_kernel_gid_t	cgid;
   __vki_kernel_mode_t	mode;
   unsigned long	seq;
   unsigned int		__pad2;
   unsigned long long	__unused1;
   unsigned long long	__unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/sembuf.h
//----------------------------------------------------------------------

struct vki_semid64_ds {
   struct vki_ipc64_perm	sem_perm;		/* permissions .. see ipc.h */
   unsigned int			__unused1;
   __vki_kernel_time_t		sem_otime;		/* last semop time */
   unsigned int			__unused2;
   __vki_kernel_time_t		sem_ctime;		/* last change time */
   unsigned long		sem_nsems;		/* no. of semaphores in array */
   unsigned long		__unused3;
   unsigned long		__unused4;
};

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
   struct vki_ipc64_perm	msg_perm;
   unsigned int			__unused1;
   __vki_kernel_time_t		msg_stime;		/* last msgsnd time */
   unsigned int			__unused2;
   __vki_kernel_time_t		msg_rtime;		/* last msgrcv time */
   unsigned int			__unused3;
   __vki_kernel_time_t		msg_ctime;		/* last change time */
   unsigned long		msg_cbytes;		/* current number of bytes on queue */
   unsigned long		msg_qnum;		/* number of messages in queue */
   unsigned long		msg_qbytes;		/* max number of bytes on queue */
   __vki_kernel_pid_t		msg_lspid;		/* pid of last msgsnd */
   __vki_kernel_pid_t		msg_lrpid;		/* last receive pid */
   unsigned long		__unused4;
   unsigned long		__unused5;
};

//----------------------------------------------------------------------
// From linux-2.6.8.1/include/asm-ppc/ipc.h
//----------------------------------------------------------------------

struct vki_ipc_kludge {
        struct vki_msgbuf __user *msgp;
        long msgtyp;
};

#define VKI_SEMOP            1
#define VKI_SEMGET           2
#define VKI_SEMCTL           3
#define VKI_SEMTIMEDOP       4
#define VKI_MSGSND          11
#define VKI_MSGRCV          12
#define VKI_MSGGET          13
#define VKI_MSGCTL          14
#define VKI_SHMAT           21
#define VKI_SHMDT           22
#define VKI_SHMGET          23
#define VKI_SHMCTL          24

//----------------------------------------------------------------------
// From linux-2.6.9/include/asm-ppc/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
   struct vki_ipc64_perm	shm_perm;		/* operation perms */
   unsigned int			__unused1;
   __vki_kernel_time_t		shm_atime;		/* last attach time */
   unsigned int			__unused2;
   __vki_kernel_time_t		shm_dtime;		/* last detach time */
   unsigned int			__unused3;
   __vki_kernel_time_t		shm_ctime;		/* last change time */
   unsigned int			__unused4;
   vki_size_t			shm_segsz;		/* size of segment (bytes) */
   __vki_kernel_pid_t		shm_cpid;		/* pid of creator */
   __vki_kernel_pid_t		shm_lpid;		/* pid of last operator */
   unsigned long		shm_nattch;		/* no. of current attaches */
   unsigned long		__unused5;
   unsigned long		__unused6;
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
// From linux-2.6.8.1/include/asm-generic/errno.h
//----------------------------------------------------------------------

#define	VKI_ENOSYS       38  /* Function not implemented */
#define	VKI_EOVERFLOW    75  /* Value too large for defined data type */

//.. //----------------------------------------------------------------------
//.. // DRM ioctls
//.. //----------------------------------------------------------------------
//.. 
//.. // jrs 20050207: where did all this stuff come from?  Is it really
//.. // i386 specific, or should it go into the linux-generic category?
//.. //struct vki_drm_buf_pub {
//.. //	Int		  idx;	       /**< Index into the master buffer list */
//.. //	Int		  total;       /**< Buffer size */
//.. //	Int		  used;	       /**< Amount of buffer in use (for DMA) */
//.. //	void	  __user *address;     /**< Address of buffer */
//.. //};
//.. //
//.. //struct vki_drm_buf_map {
//.. //	Int	      count;		/**< Length of the buffer list */
//.. //	void	      __user *virtual;	/**< Mmap'd area in user-virtual */
//.. //	struct vki_drm_buf_pub __user *list;	/**< Buffer information */
//.. //};
//.. //
//.. ///* We need to pay attention to this, because it mmaps memory */
//.. //#define VKI_DRM_IOCTL_MAP_BUFS		_VKI_IOWR('d', 0x19, struct vki_drm_buf_map)

//----------------------------------------------------------------------
// And that's it!
//----------------------------------------------------------------------

#endif // __VKI_PPC32_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
