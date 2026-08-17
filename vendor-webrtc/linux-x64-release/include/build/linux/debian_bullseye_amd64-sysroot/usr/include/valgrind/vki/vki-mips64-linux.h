
/*--------------------------------------------------------------------*/
/*--- mips/Linux-specific kernel interface.     vki-mips64-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2010-2017 RT-RK
      mips-valgrind@rt-rk.com

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
*/

#ifndef __VKI_MIPS64_LINUX_H
#define __VKI_MIPS64_LINUX_H

#include <config.h>

// mips endian
#if defined (_MIPSEL)
#define VKI_LITTLE_ENDIAN  1
#elif defined (_MIPSEB)
#define VKI_BIG_ENDIAN  1
#endif

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/cachectl.h
//----------------------------------------------------------------------

#define VKI_ICACHE  (1<<0)          /* flush instruction cache        */
#define VKI_DCACHE  (1<<1)          /* writeback and flush data cache */
#define VKI_BCACHE  (VKI_ICACHE | VKI_DCACHE) /* flush both caches    */

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/mips-mips/types.h
//----------------------------------------------------------------------

typedef __signed__ char __vki_s8;
typedef unsigned   char __vki_u8;

typedef __signed__ short __vki_s16;
typedef unsigned   short __vki_u16;

typedef __signed__ int __vki_s32;
typedef unsigned   int __vki_u32;

typedef __signed char vki_s8;
typedef unsigned char vki_u8;

typedef __signed short vki_s16;
typedef unsigned short vki_u16;

typedef __signed int vki_s32;
typedef unsigned int vki_u32;

#if (_MIPS_SZLONG == 64)
typedef __signed__ long __vki_s64;
typedef unsigned   long __vki_u64;
#else
typedef __signed__ long long __vki_s64;
typedef unsigned   long long __vki_u64;
#endif

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/page.h
//----------------------------------------------------------------------

/* MIPS64 uses runtime pagesize detection */
extern UWord VKI_PAGE_SHIFT;
extern UWord VKI_PAGE_SIZE;
#define VKI_PAGE_MASK           (~(VKI_PAGE_SIZE-1))
#define VKI_MAX_PAGE_SHIFT      16
#define VKI_MAX_PAGE_SIZE       (1UL << VKI_MAX_PAGE_SHIFT)

//----------------------------------------------------------------------
// From linux-2.6.35.9/arch/mips/include/bits/shm.h
//----------------------------------------------------------------------

#define VKI_SHMLBA  0x40000

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/signal.h
//----------------------------------------------------------------------

#define VKI_MINSIGSTKSZ 2048

#define VKI_SIG_BLOCK   1  /* for blocking signals */
#define VKI_SIG_UNBLOCK 2  /* for unblocking signals */
#define VKI_SIG_SETMASK 3  /* for setting the signal mask */

/* Type of a signal handler.  */
typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

#define VKI_SIG_DFL     ((__vki_sighandler_t)0)   /* default signal handling */
#define VKI_SIG_IGN     ((__vki_sighandler_t)1)   /* ignore signal */
#define VKI_SIG_ERR     ((__vki_sighandler_t)-1)  /* error return from signal */

#define _VKI_NSIG       128
#define _VKI_NSIG_BPW   (__SIZEOF_LONG__ * 8)
#define _VKI_NSIG_WORDS (_VKI_NSIG / _VKI_NSIG_BPW)

typedef unsigned long vki_old_sigset_t;  /* at least 32 bits */

typedef struct {
        unsigned long sig[_VKI_NSIG_WORDS];
} vki_sigset_t;

#define VKI_SIGHUP           1          /* Hangup (POSIX).                    */
#define VKI_SIGINT           2          /* Interrupt (ANSI).                  */
#define VKI_SIGQUIT          3          /* Quit (POSIX).                      */
#define VKI_SIGILL           4          /* Illegal instruction (ANSI).        */
#define VKI_SIGTRAP          5          /* Trace trap (POSIX).                */
#define VKI_SIGIOT           6          /* IOT trap (4.2 BSD).                */
#define VKI_SIGABRT          VKI_SIGIOT /* Abort (ANSI).                      */
#define VKI_SIGEMT           7
#define VKI_SIGFPE           8          /* Floating-point exception (ANSI).   */
#define VKI_SIGKILL          9          /* Kill, unblockable (POSIX).         */
#define VKI_SIGBUS          10          /* BUS error (4.2 BSD).               */
#define VKI_SIGSEGV         11          /* Segmentation violation (ANSI).     */
#define VKI_SIGSYS          12
#define VKI_SIGPIPE         13          /* Broken pipe (POSIX).               */
#define VKI_SIGALRM         14          /* Alarm clock (POSIX).               */
#define VKI_SIGTERM         15          /* Termination (ANSI).                */
#define VKI_SIGUSR1         16          /* User-defined signal 1 (POSIX).     */
#define VKI_SIGUSR2         17          /* User-defined signal 2 (POSIX).     */
#define VKI_SIGCHLD         18          /* Child status has changed (POSIX).  */
#define VKI_SIGCLD          VKI_SIGCHLD /* Same as SIGCHLD (System V).        */
#define VKI_SIGPWR          19          /* Power failure restart (System V).  */
#define VKI_SIGWINCH        20          /* Window size change (4.3 BSD, Sun). */
#define VKI_SIGURG          21          /* Urgent condition on socket.        */
#define VKI_SIGIO           22          /* I/O now possible (4.2 BSD).        */
#define VKI_SIGPOLL         VKI_SIGIO   /* Pollable event occurred (System V).*/
#define VKI_SIGSTOP         23          /* Stop, unblockable (POSIX).         */
#define VKI_SIGTSTP         24          /* Keyboard stop (POSIX).             */
#define VKI_SIGCONT         25          /* Continue (POSIX).                  */
#define VKI_SIGTTIN         26          /* Background read from tty (POSIX).  */
#define VKI_SIGTTOU         27          /* Background write to tty (POSIX).   */
#define VKI_SIGVTALRM       28          /* Virtual alarm clock (4.2 BSD).     */
#define VKI_SIGPROF         29          /* Profiling alarm clock (4.2 BSD).   */
#define VKI_SIGXCPU         30          /* CPU limit exceeded (4.2 BSD).      */
#define VKI_SIGXFSZ         31          /* File size limit exceeded (4.2 BSD).*/

/* These should not be considered constants from userland.  */
#define VKI_SIGRTMIN    32
// [[This was (_NSIG-1) in 2.4.X... not sure if it matters.]]
#define VKI_SIGRTMAX    (_VKI_NSIG - 1)

#define VKI_SA_ONSTACK      0x08000000u
#define VKI_SA_RESETHAND    0x80000000u
#define VKI_SA_RESTART      0x10000000u
#define VKI_SA_SIGINFO      0x00000008u
#define VKI_SA_NODEFER      0x40000000u
#define VKI_SA_NOCLDWAIT    0x00010000u
#define VKI_SA_NOCLDSTOP    0x00000001u

#define VKI_SA_NOMASK           VKI_SA_NODEFER
#define VKI_SA_ONESHOT          VKI_SA_RESETHAND
//#define VKI_SA_INTERRUPT      0x20000000  /* dummy -- ignored */

#define VKI_SA_RESTORER         0x04000000

#define VKI_SS_ONSTACK          1
#define VKI_SS_DISABLE          2

struct vki_old_sigaction {
       // [[Nb: a 'k' prefix is added to "sa_handler" because
       // bits/sigaction.h (which gets dragged in somehow via signal.h)
       // #defines it as something else.  Since that is done for glibc's
       // purposes, which we don't care about here, we use our own name.]]
       unsigned long sa_flags;
       __vki_sighandler_t ksa_handler;
       vki_old_sigset_t sa_mask;
       __vki_sigrestore_t sa_restorer;
};

struct vki_sigaction {
       unsigned int    sa_flags;
       __vki_sighandler_t  sa_handler;
       vki_sigset_t        sa_mask;
};


struct vki_sigaction_base {
       // [[See comment about extra 'k' above]]
       unsigned int sa_flags;
       __vki_sighandler_t ksa_handler;
       vki_sigset_t sa_mask;           // mask last for extensibility
       __vki_sigrestore_t sa_restorer;
};

/* On Linux we use the same type for passing sigactions to
   and from the kernel.  Hence: */
typedef  struct vki_sigaction_base  vki_sigaction_toK_t;
typedef  struct vki_sigaction_base  vki_sigaction_fromK_t;

typedef struct vki_sigaltstack {
        void __user *ss_sp;
        vki_size_t ss_size;
        int ss_flags;
} vki_stack_t;

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/sigcontext.h
//----------------------------------------------------------------------

struct _vki_fpreg {
       unsigned short significand[4];
       unsigned short exponent;
};

struct _vki_fpxreg {
       unsigned short significand[4];
       unsigned short exponent;
       unsigned short padding[3];
};

struct _vki_xmmreg {
       unsigned long element[4];
};

struct _vki_fpstate {
       /* Regular FPU environment */
       unsigned long    cw;
       unsigned long    sw;
       unsigned long    tag;
       unsigned long    ipoff;
       unsigned long    cssel;
       unsigned long    dataoff;
       unsigned long    datasel;
       struct _vki_fpreg    _st[8];
       unsigned short   status;
       unsigned short   magic;           /* 0xffff = regular FPU data only */

       /* FXSR FPU environment */
       unsigned long    _fxsr_env[6];    /* FXSR FPU env is ignored */
       unsigned long    mxcsr;
       unsigned long    reserved;
       struct _vki_fpxreg   _fxsr_st[8]; /* FXSR FPU reg data is ignored */
       struct _vki_xmmreg   _xmm[8];
       unsigned long    padding[56];
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/sigcontext.h
//----------------------------------------------------------------------
struct vki_sigcontext {
       __vki_u64   sc_regs[32];
       __vki_u64   sc_fpregs[32];
       __vki_u64   sc_mdhi;
       __vki_u64   sc_hi1;
       __vki_u64   sc_hi2;
       __vki_u64   sc_hi3;
       __vki_u64   sc_mdlo;
       __vki_u64   sc_lo1;
       __vki_u64   sc_lo2;
       __vki_u64   sc_lo3;
       __vki_u64   sc_pc;
       __vki_u64   sc_fpc_csr;
       __vki_u64   sc_used_math;
       __vki_u64   sc_dsp;
       __vki_u64   sc_reserved;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_NONE       0x0      /* No page permissions */
#define VKI_PROT_READ       0x1      /* page can be read */
#define VKI_PROT_WRITE      0x2      /* page can be written */
#define VKI_PROT_EXEC       0x4      /* page can be executed */
#define VKI_PROT_GROWSDOWN  0x01000000  /* mprotect flag: extend change to start
                                           of growsdown vma */
#define VKI_PROT_GROWSUP    0x02000000  /* mprotect flag: extend change to end
                                           of growsup vma */

#define VKI_MAP_SHARED      0x001     /* Share changes */
#define VKI_MAP_PRIVATE     0x002     /* Changes are private */
//#define VKI_MAP_TYPE      0x0f        /* Mask for type of mapping */
#define VKI_MAP_FIXED       0x010     /* Interpret addr exactly */

#define VKI_MAP_NORESERVE   0x0400   /* don't reserve swap pages */

/* These are linux-specific */
#define VKI_MAP_NORESERVE   0x0400          /* don't check for reservations */
#define VKI_MAP_ANONYMOUS   0x0800          /* don't use a file */
#define VKI_MAP_GROWSDOWN   0x1000          /* stack-like segment */
#define VKI_MAP_DENYWRITE   0x2000          /* ETXTBSY */
#define VKI_MAP_EXECUTABLE  0x4000          /* mark it as an executable */
#define VKI_MAP_LOCKED      0x8000          /* pages are locked */
#define VKI_MAP_POPULATE    0x10000         /* populate (prefault) pagetables */
#define VKI_MAP_NONBLOCK    0x20000         /* do not block on IO */

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/fcntl.h
//----------------------------------------------------------------------

#define VKI_O_RDONLY        00
#define VKI_O_WRONLY        01
#define VKI_O_RDWR          02
#define VKI_O_ACCMODE       03

#define VKI_O_CREAT         0x0100      /* not fcntl */
#define VKI_O_EXCL          0x0400      /* not fcntl */

#define VKI_O_TRUNC         0x0200      /* not fcntl */

#define VKI_O_APPEND        0x0008
#define VKI_O_NONBLOCK      0x0080
#define VKI_O_LARGEFILE     0x2000

#define VKI_AT_FDCWD        -100

#define VKI_F_DUPFD         0           /* dup */
#define VKI_F_GETFD         1           /* get close_on_exec */
#define VKI_F_SETFD         2           /* set/clear close_on_exec */
#define VKI_F_GETFL         3           /* get file->f_flags */
#define VKI_F_SETFL         4           /* set file->f_flags */

#define VKI_F_GETLK         14
#define VKI_F_SETLK         6
#define VKI_F_SETLKW        7

#define VKI_F_SETOWN        24          /*  for sockets. */
#define VKI_F_GETOWN        23          /*  for sockets. */
#define VKI_F_SETSIG        10          /*  for sockets. */
#define VKI_F_GETSIG        11          /*  for sockets. */

#define VKI_F_SETOWN_EX     15
#define VKI_F_GETOWN_EX     16

#define VKI_F_OFD_GETLK     36
#define VKI_F_OFD_SETLK     37
#define VKI_F_OFD_SETLKW    38

#define VKI_F_GETLK64       33          /*  using 'struct flock64' */
#define VKI_F_SETLK64       34
#define VKI_F_SETLKW64      35

/* for F_[GET|SET]FL */
#define VKI_FD_CLOEXEC      1      /* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE 1024

struct vki_f_owner_ex {
       int type;
       __vki_kernel_pid_t pid;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/resource.h
//----------------------------------------------------------------------

#define VKI_RLIMIT_DATA     2   /* max data size */
#define VKI_RLIMIT_STACK    3   /* max stack size */
#define VKI_RLIMIT_CORE     4   /* max core file size */
#define VKI_RLIMIT_NOFILE   5   /* max number of open files */

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET   0xffff

#define VKI_SO_TYPE      0x1008

#define VKI_SO_ATTACH_FILTER	26

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-i386/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCATMARK          _VKI_IOR('s', 7, int)
#define VKI_SIOCSPGRP           _VKI_IOW('s', 8, vki_pid_t)
#define VKI_SIOCGPGRP           _VKI_IOR('s', 9, vki_pid_t)
#define VKI_SIOCGSTAMP          0x8906      /* Get stamp (timeval) */
#define VKI_SIOCGSTAMPNS        0x8907      /* Get stamp (timespec) */

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/stat.h
//----------------------------------------------------------------------

/* Size of kernel long is different from Valgrind MIPS n32 long size, so we have to
use long long instead long type. */
struct vki_stat {
        unsigned int    st_dev;
        unsigned int    st_pad0[3];     /* Reserved for st_dev expansion  */

#if defined(VGABI_N32)
        unsigned long long st_ino;
#else
        unsigned long   st_ino;
#endif

        int             st_mode;
        unsigned int    st_nlink;

        unsigned int    st_uid;
        unsigned int    st_gid;

        unsigned int    st_rdev;
        unsigned int    st_pad1[3];     /* Reserved for st_rdev expansion  */

#if defined(VGABI_N32)
        long long       st_size;
#else
        long            st_size;
#endif

        /*
         * Actually this should be timestruc_t st_atime, st_mtime and st_ctime
         * but we don't have it under Linux.
         */
        unsigned int    st_atime;
        unsigned int    st_atime_nsec;  /* Reserved for st_atime expansion  */

        unsigned int    st_mtime;
        unsigned int    st_mtime_nsec;  /* Reserved for st_mtime expansion  */

        unsigned int    st_ctime;
        unsigned int    st_ctime_nsec;  /* Reserved for st_ctime expansion  */

        unsigned int    st_blksize;
        unsigned int    st_pad2;

        long long       st_blocks;
};

struct vki_stat64 {
        unsigned long   st_dev;
        unsigned long   st_pad0[3];     /* Reserved for st_dev expansion  */

        unsigned long long      st_ino;

        int             st_mode;
        unsigned int    st_nlink;

        unsigned int    st_uid;
        unsigned int    st_gid;

        unsigned long   st_rdev;
        unsigned long   st_pad1[3];     /* Reserved for st_rdev expansion  */

        long long       st_size;

        /*
         * Actually this should be timestruc_t st_atime, st_mtime and st_ctime
         * but we don't have it under Linux.
         */
        unsigned long   st_atime;
        unsigned long   st_atime_nsec;  /* Reserved for st_atime expansion  */

        unsigned long   st_mtime;
        unsigned long   st_mtime_nsec;  /* Reserved for st_mtime expansion  */

        unsigned long   st_ctime;
        unsigned long   st_ctime_nsec;  /* Reserved for st_ctime expansion  */

        unsigned long   st_blksize;
        unsigned long   st_pad2;

        long long       st_blocks;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/statfs.h
//----------------------------------------------------------------------

struct vki_statfs {
       __vki_u32 f_type;
#define f_fstyp f_type
       __vki_u32 f_bsize;
       __vki_u32 f_frsize;
       __vki_u32 f_blocks;
       __vki_u32 f_bfree;
       __vki_u32 f_files;
       __vki_u32 f_ffree;
       __vki_u32 f_bavail;
       __vki_kernel_fsid_t f_fsid;
       __vki_u32 f_namelen;
       __vki_u32 f_spare[6];
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/termios.h
//----------------------------------------------------------------------

struct vki_winsize {
       unsigned short ws_row;
       unsigned short ws_col;
       unsigned short ws_xpixel;
       unsigned short ws_ypixel;
};

#define NCC     8
#define NCCS    23
struct vki_termio {
       unsigned short c_iflag;      /* input mode flags */
       unsigned short c_oflag;      /* output mode flags */
       unsigned short c_cflag;      /* control mode flags */
       unsigned short c_lflag;      /* local mode flags */
       char           c_line;       /* line discipline */
       unsigned char  c_cc[NCCS];   /* control characters */
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/termbits.h
//----------------------------------------------------------------------

typedef unsigned char   vki_cc_t;
typedef unsigned long   vki_speed_t;
typedef unsigned long   vki_tcflag_t;

struct vki_termios {
       vki_tcflag_t c_iflag;        /* input mode flags */
       vki_tcflag_t c_oflag;        /* output mode flags */
       vki_tcflag_t c_cflag;        /* control mode flags */
       vki_tcflag_t c_lflag;        /* local mode flags */
       vki_cc_t c_line;             /* line discipline */
       vki_cc_t c_cc[NCCS];         /* control characters */
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ioctl.h
//----------------------------------------------------------------------

#define _VKI_IOC_NRBITS     8
#define _VKI_IOC_TYPEBITS   8
#define _VKI_IOC_SIZEBITS   13
#define _VKI_IOC_DIRBITS    3

#define _VKI_IOC_NRMASK     ((1 << _VKI_IOC_NRBITS)-1)
#define _VKI_IOC_TYPEMASK   ((1 << _VKI_IOC_TYPEBITS)-1)
#define _VKI_IOC_SIZEMASK   ((1 << _VKI_IOC_SIZEBITS)-1)
#define _VKI_IOC_DIRMASK    ((1 << _VKI_IOC_DIRBITS)-1)

#define _VKI_IOC_NRSHIFT    0
#define _VKI_IOC_TYPESHIFT  (_VKI_IOC_NRSHIFT+_VKI_IOC_NRBITS)
#define _VKI_IOC_SIZESHIFT  (_VKI_IOC_TYPESHIFT+_VKI_IOC_TYPEBITS)
#define _VKI_IOC_DIRSHIFT   (_VKI_IOC_SIZESHIFT+_VKI_IOC_SIZEBITS)

#define _VKI_IOC_NONE   1U
#define _VKI_IOC_READ   2U
#define _VKI_IOC_WRITE  4U

#define _VKI_IOC(dir,type,nr,size) \
                (((dir)  << _VKI_IOC_DIRSHIFT) | \
                ((type) << _VKI_IOC_TYPESHIFT) | \
                ((nr)   << _VKI_IOC_NRSHIFT) | \
                ((size) << _VKI_IOC_SIZESHIFT))

/* provoke compile error for invalid uses of size argument */
extern unsigned int __VKI_invalid_size_argument_for_IOC;
/* used to create numbers */
#define _VKI_IO(type,nr)        _VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)  _VKI_IOC(_VKI_IOC_READ,(type),(nr), \
                                (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOW(type,nr,size)  _VKI_IOC(_VKI_IOC_WRITE,(type),(nr), \
                                (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOWR(type,nr,size) _VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE,(type), \
                                (nr),(_VKI_IOC_TYPECHECK(size)))

/* used to decode ioctl numbers.. */
#define _VKI_IOC_DIR(nr)    (((nr) >> _VKI_IOC_DIRSHIFT) & _VKI_IOC_DIRMASK)
#define _VKI_IOC_TYPE(nr)   (((nr) >> _VKI_IOC_TYPESHIFT) & _VKI_IOC_TYPEMASK)
#define _VKI_IOC_NR(nr)     (((nr) >> _VKI_IOC_NRSHIFT) & _VKI_IOC_NRMASK)
#define _VKI_IOC_SIZE(nr)   (((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ioctls.h
//----------------------------------------------------------------------

#define VKI_TCGETA              0x5401
#define VKI_TCSETA              0x5402 /* Clashes with SNDCTL_TMR_START
                                          sound ioctl */
#define VKI_TCSETAW             0x5403
#define VKI_TCSETAF             0x5404

#define VKI_TCSBRK              0x5405
#define VKI_TCXONC              0x5406
#define VKI_TCFLSH              0x5407

#define VKI_TCGETS              0x540d
#define VKI_TCSETS              0x540e
#define VKI_TCSETSW             0x540f
#define VKI_TCSETSF             0x5410

#define VKI_TIOCEXCL            0x740d  /* set exclusive use of tty */
#define VKI_TIOCNXCL            0x740e  /* reset exclusive use of tty */
#define VKI_TIOCOUTQ            0x7472  /* output queue size */
#define VKI_TIOCSTI             0x5472  /* simulate terminal input */
#define VKI_TIOCMGET            0x741d  /* get all modem bits */
#define VKI_TIOCMBIS            0x741b  /* bis modem bits */
#define VKI_TIOCMBIC            0x741c  /* bic modem bits */
#define VKI_TIOCMSET            0x741a  /* set all modem bits */
#define VKI_TIOCPKT             0x5470  /* pty: set/clear packet mode */
#define VKI_TIOCPKT_DATA        0x00    /* data packet */
#define VKI_TIOCPKT_FLUSHREAD   0x01    /* flush packet */
#define VKI_TIOCPKT_FLUSHWRITE  0x02    /* flush packet */
#define VKI_TIOCPKT_STOP        0x04    /* stop output */
#define VKI_TIOCPKT_START       0x08    /* start output */
#define VKI_TIOCPKT_NOSTOP      0x10    /* no more ^S, ^Q */
#define VKI_TIOCPKT_DOSTOP      0x20    /* now do ^S ^Q */

/* set window size */
#define VKI_TIOCSWINSZ          _VKI_IOW('t', 103, struct vki_winsize)
/* get window size */
#define VKI_TIOCGWINSZ          _VKI_IOR('t', 104, struct vki_winsize)
#define VKI_TIOCNOTTY           0x5471 /* void tty association */
#define VKI_TIOCSETD            0x7401
#define VKI_TIOCGETD            0x7400

#define VKI_FIOCLEX             0x6601
#define VKI_FIONCLEX            0x6602
#define VKI_FIOASYNC            0x667d
#define VKI_FIONBIO             0x667e
#define VKI_FIOQSIZE            0x667f

#define VKI_TIOCGLTC            0x7474 /* get special local chars */
#define VKI_TIOCSLTC            0x7475 /* set special local chars */
#define VKI_TIOCSPGRP           _VKI_IOW('t', 118, int) /* set pgrp of tty */
#define VKI_TIOCGPGRP           _VKI_IOR('t', 119, int) /* get pgrp of tty */
#define VKI_TIOCCONS            _VKI_IOW('t', 120, int) /* become virtual
                                                           console */

#define VKI_FIONREAD            0x467f
#define VKI_TIOCINQ             FIONREAD

#define VKI_TIOCGETP            0x7408
#define VKI_TIOCSETP            0x7409
#define VKI_TIOCSETN            0x740a /* TIOCSETP wo flush */

#define VKI_TIOCSBRK            0x5427 /* BSD compatibility */
#define VKI_TIOCCBRK            0x5428 /* BSD compatibility */
#define VKI_TIOCGSID            0x7416 /* Return the session ID of FD */
#define VKI_TIOCGPTN            _VKI_IOR('T',0x30, unsigned int) /* Get Pty
                                                   Number (of pty-mux device) */
#define VKI_TIOCSPTLCK          _VKI_IOW('T',0x31, int) /* Lock/unlock Pty */

/* I hope the range from 0x5480 on is free ... */
#define VKI_TIOCSCTTY           0x5480 /* become controlling tty */
#define VKI_TIOCGSOFTCAR        0x5481
#define VKI_TIOCSSOFTCAR        0x5482
#define VKI_TIOCLINUX           0x5483
#define VKI_TIOCGSERIAL         0x5484
#define VKI_TIOCSSERIAL         0x5485
#define VKI_TCSBRKP             0x5486 /* Needed for POSIX tcsendbreak() */
#define VKI_TIOCSERCONFIG       0x5488
#define VKI_TIOCSERGWILD        0x5489
#define VKI_TIOCSERSWILD        0x548a
#define VKI_TIOCGLCKTRMIOS      0x548b
#define VKI_TIOCSLCKTRMIOS      0x548c
#define VKI_TIOCSERGSTRUCT      0x548d /* For debugging only */
#define VKI_TIOCSERGETLSR       0x548e /* Get line status register */
#define VKI_TIOCSERGETMULTI     0x548f /* Get multiport config  */
#define VKI_TIOCSERSETMULTI     0x5490 /* Set multiport config */
#define VKI_TIOCMIWAIT          0x5491 /* wait for a change on serial input
                                          line(s) */
#define VKI_TIOCGICOUNT         0x5492 /* read serial port inline interrupt
                                          counts */
#define VKI_TIOCGHAYESESP       0x5493 /* Get Hayes ESP configuration */
#define VKI_TIOCSHAYESESP       0x5494 /* Set Hayes ESP configuration */

//----------------------------------------------------------------------
// From asm-generic/poll.h
//----------------------------------------------------------------------

/* These are specified by iBCS2 */
#define VKI_POLLIN              0x0001

struct vki_pollfd {
       int fd;
       short events;
       short revents;
};
//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/elf.h
//----------------------------------------------------------------------

#define VKI_ELF_NGREG           45  /* includes nip, msr, lr, etc. */
#define VKI_ELF_NFPREG          33  /* includes fpscr */

typedef unsigned long vki_elf_greg_t;
typedef vki_elf_greg_t vki_elf_gregset_t[VKI_ELF_NGREG];

typedef double vki_elf_fpreg_t;
typedef vki_elf_fpreg_t vki_elf_fpregset_t[VKI_ELF_NFPREG];

typedef struct vki_user_fxsr_struct vki_elf_fpxregset_t;

#define VKI_AT_SYSINFO          32
//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ucontext.h
//----------------------------------------------------------------------

struct vki_ucontext {
       unsigned long          uc_flags;
       struct vki_ucontext    *uc_link;
       vki_stack_t            uc_stack;
       struct vki_sigcontext  uc_mcontext;
       vki_sigset_t           uc_sigmask;  /* mask last for extensibility */
};

typedef char vki_modify_ldt_t;
//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ipcbuf.h
//----------------------------------------------------------------------

struct vki_ipc64_perm {
       __vki_kernel_key_t  key;
       __vki_kernel_uid_t  uid;
       __vki_kernel_gid_t  gid;
       __vki_kernel_uid_t  cuid;
       __vki_kernel_gid_t  cgid;
       __vki_kernel_mode_t mode;
       unsigned short  seq;
       unsigned short  __pad1;
       unsigned long   __unused1;
       unsigned long   __unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/sembuf.h
//----------------------------------------------------------------------

struct vki_semid64_ds {
       struct vki_ipc64_perm sem_perm;         /* permissions .. see ipc.h */
       __vki_kernel_time_t sem_otime;          /* last semop time */
       __vki_kernel_time_t sem_ctime;          /* last change time */
       unsigned long   sem_nsems;              /* no. of semaphores in array */
       unsigned long   __unused1;
       unsigned long   __unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
       struct vki_ipc64_perm msg_perm;
       __vki_kernel_time_t msg_stime;     /* last msgsnd time */
       __vki_kernel_time_t msg_rtime;     /* last msgrcv time */
       __vki_kernel_time_t msg_ctime;     /* last change time */
       unsigned long  msg_cbytes;         /* current number of bytes on queue */
       unsigned long  msg_qnum;           /* number of messages in queue */
       unsigned long  msg_qbytes;         /* max number of bytes on queue */
       __vki_kernel_pid_t msg_lspid;      /* pid of last msgsnd */
       __vki_kernel_pid_t msg_lrpid;      /* last receive pid */
       unsigned long  __unused4;
       unsigned long  __unused5;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ipc.h
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
// From linux-2.6.35.9/include/asm-mips/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
       struct vki_ipc64_perm       shm_perm;       /* operation perms */
       vki_size_t                  shm_segsz;      /* size of segment (bytes) */
       __vki_kernel_time_t         shm_atime;      /* last attach time */
       __vki_kernel_time_t         shm_dtime;      /* last detach time */
       __vki_kernel_time_t         shm_ctime;      /* last change time */
       __vki_kernel_pid_t          shm_cpid;       /* pid of creator */
       __vki_kernel_pid_t          shm_lpid;       /* pid of last operator */
       unsigned long               shm_nattch;     /* no. of current attaches */
       unsigned long               __unused1;
       unsigned long               __unused2;
};

struct vki_shminfo64 {
       unsigned long   shmmax;
       unsigned long   shmmin;
       unsigned long   shmmni;
       unsigned long   shmseg;
       unsigned long   shmall;
       unsigned long   __unused1;
       unsigned long   __unused2;
       unsigned long   __unused3;
       unsigned long   __unused4;
};

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-mips/ptrace.h
//----------------------------------------------------------------------

struct vki_pt_regs {
#ifdef CONFIG_32BIT
        /* Pad bytes for argument save space on the stack. */
       unsigned long pad0[6];
#endif

       /* Saved main processor registers. */
       unsigned long regs[32];

       /* Saved special registers. */
       unsigned long cp0_status;
       unsigned long hi;
       unsigned long lo;
#ifdef CONFIG_CPU_HAS_SMARTMIPS
       unsigned long acx;
#endif
       unsigned long cp0_badvaddr;
       unsigned long cp0_cause;
       unsigned long cp0_epc;
#ifdef CONFIG_MIPS_MT_SMTC
       unsigned long cp0_tcstatus;
#endif /* CONFIG_MIPS_MT_SMTC */
#ifdef CONFIG_CPU_CAVIUM_OCTEON
       unsigned long long mpl[3];        /* MTM{0,1,2} */
       unsigned long long mtp[3];        /* MTP{0,1,2} */
#endif
} __attribute__ ((aligned (8)));

//----------------------------------------------------------------------
// From linux-4.5/arch/mips/include/uapi/asm/reg.h
//----------------------------------------------------------------------

#define VKI_MIPS64_EF_R0             0
#define VKI_MIPS64_EF_R1             1
#define VKI_MIPS64_EF_R2             2
#define VKI_MIPS64_EF_R3             3
#define VKI_MIPS64_EF_R4             4
#define VKI_MIPS64_EF_R5             5
#define VKI_MIPS64_EF_R6             6
#define VKI_MIPS64_EF_R7             7
#define VKI_MIPS64_EF_R8             8
#define VKI_MIPS64_EF_R9             9
#define VKI_MIPS64_EF_R10           10
#define VKI_MIPS64_EF_R11           11
#define VKI_MIPS64_EF_R12           12
#define VKI_MIPS64_EF_R13           13
#define VKI_MIPS64_EF_R14           14
#define VKI_MIPS64_EF_R15           15
#define VKI_MIPS64_EF_R16           16
#define VKI_MIPS64_EF_R17           17
#define VKI_MIPS64_EF_R18           18
#define VKI_MIPS64_EF_R19           19
#define VKI_MIPS64_EF_R20           20
#define VKI_MIPS64_EF_R21           21
#define VKI_MIPS64_EF_R22           22
#define VKI_MIPS64_EF_R23           23
#define VKI_MIPS64_EF_R24           24
#define VKI_MIPS64_EF_R25           25
#define VKI_MIPS64_EF_R26           26
#define VKI_MIPS64_EF_R27           27
#define VKI_MIPS64_EF_R28           28
#define VKI_MIPS64_EF_R29           29
#define VKI_MIPS64_EF_R30           30
#define VKI_MIPS64_EF_R31           31
#define VKI_MIPS64_EF_LO            32
#define VKI_MIPS64_EF_HI            33
#define VKI_MIPS64_EF_CP0_EPC       34
#define VKI_MIPS64_EF_CP0_BADVADDR  35
#define VKI_MIPS64_EF_CP0_STATUS    36
#define VKI_MIPS64_EF_CP0_CAUSE     37

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-i386/ptrace.h
//----------------------------------------------------------------------

#define VKI_PTRACE_GETREGS            12
#define VKI_PTRACE_SETREGS            13
#define VKI_PTRACE_GETFPREGS          14
#define VKI_PTRACE_SETFPREGS          15
#define VKI_PTRACE_GETFPXREGS         18
#define VKI_PTRACE_SETFPXREGS         19

/* Calls to trace a 64bit program from a 32bit program.  */
#define VKI_PTRACE_PEEKTEXT_3264    0xc0
#define VKI_PTRACE_PEEKDATA_3264    0xc1
#define VKI_PTRACE_POKETEXT_3264    0xc2
#define VKI_PTRACE_POKEDATA_3264    0xc3
#define VKI_PTRACE_GET_THREAD_AREA_3264     0xc4s

//----------------------------------------------------------------------
// From linux-2.6.35.9/include/asm-generic/siginfo.h
//----------------------------------------------------------------------

#define HAVE_ARCH_SIGINFO_T
typedef union vki_sigval {
        int sival_int;
        void __user *sival_ptr;
} vki_sigval_t;

#ifndef __VKI_ARCH_SI_PREAMBLE_SIZE
#if defined(VGABI_64)
 #define __VKI_ARCH_SI_PREAMBLE_SIZE (4 * sizeof(int))
#elif defined(VGABI_N32)
#define __VKI_ARCH_SI_PREAMBLE_SIZE (3 * sizeof(int))
#else
#error unknown mips64 abi
#endif
#endif

#define VKI_SI_MAX_SIZE 128

#ifndef VKI_SI_PAD_SIZE
#define VKI_SI_PAD_SIZE ((VKI_SI_MAX_SIZE - __VKI_ARCH_SI_PREAMBLE_SIZE) / sizeof(int))
#endif

#ifndef __VKI_ARCH_SI_UID_T
#define __VKI_ARCH_SI_UID_T vki_uid_t
#endif

#ifndef __VKI_ARCH_SI_BAND_T
#define __VKI_ARCH_SI_BAND_T long
#endif

typedef struct vki_siginfo {
        int si_signo;
        int si_code;
        int si_errno;
        int __pad0[VKI_SI_MAX_SIZE / sizeof(int) - VKI_SI_PAD_SIZE - 3];

        union {
                int _pad[VKI_SI_PAD_SIZE];

                /* kill() */
                struct {
                        vki_pid_t _pid;             /* sender's pid */
                        __VKI_ARCH_SI_UID_T _uid;   /* sender's uid */
                } _kill;

                /* POSIX.1b timers */
                struct {
                        vki_timer_t _tid;           /* timer id */
                        int _overrun;           /* overrun count */
                        char _pad[sizeof( __VKI_ARCH_SI_UID_T) - sizeof(int)];
                        vki_sigval_t _sigval;       /* same as below */
                        int _sys_private;       /* not to be passed to user */
                } _timer;

                /* POSIX.1b signals */
                struct {
                        vki_pid_t _pid;             /* sender's pid */
                        __VKI_ARCH_SI_UID_T _uid;   /* sender's uid */
                        vki_sigval_t _sigval;
                } _rt;

                /* SIGCHLD */
                struct {
                        vki_pid_t _pid;             /* which child */
                        __VKI_ARCH_SI_UID_T _uid;   /* sender's uid */
                        int _status;            /* exit code */
                        vki_clock_t _utime;
                        vki_clock_t _stime;
                } _sigchld;

                /* IRIX SIGCHLD */
                struct {
                        vki_pid_t _pid;             /* which child */
                        vki_clock_t _utime;
                        int _status;            /* exit code */
                        vki_clock_t _stime;
                } _irix_sigchld;

                /* SIGILL, SIGFPE, SIGSEGV, SIGBUS */
                struct {
                        void __user *_addr; /* faulting insn/memory ref. */
#ifdef __ARCH_SI_TRAPNO
                        int _trapno;    /* TRAP # which caused the signal */
#endif
                } _sigfault;

                /* SIGPOLL, SIGXFSZ (To do ...)  */
                struct {
                        __VKI_ARCH_SI_BAND_T _band; /* POLL_IN, POLL_OUT, POLL_MSG */
                        int _fd;
                } _sigpoll;
        } _sifields;
} vki_siginfo_t;

//----------------------------------------------------------------------
// From linux-2.6.35.5/include/asm/break.h
//----------------------------------------------------------------------
#define VKI_BRK_OVERFLOW 6  /* Overflow check */
#define VKI_BRK_DIVZERO  7  /* Divide by zero check */

//----------------------------------------------------------------------
// From linux-3.6.35.5/arch/mips/include/socket.h
//----------------------------------------------------------------------
enum vki_sock_type {
        VKI_SOCK_STREAM = 2,
        // [[others omitted]]
};
#define ARCH_HAS_SOCKET_TYPES 1

//----------------------------------------------------------------------
// From linux-3.13.0/include/asm/errno.h
//----------------------------------------------------------------------

#define	VKI_ENOSYS       89  /* Function not implemented */
#define	VKI_EOVERFLOW    79  /* Value too large for defined data type */

//----------------------------------------------------------------------
// From linux-3.7.0/arch/mips/include/uapi/asm/errno.h
//----------------------------------------------------------------------

#define VKI_EOPNOTSUPP   122 /* Operation not supported on transport
                                endpoint */

#endif // __VKI_MIPS64_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                       vki-mips64-linux.h ---*/
/*--------------------------------------------------------------------*/
