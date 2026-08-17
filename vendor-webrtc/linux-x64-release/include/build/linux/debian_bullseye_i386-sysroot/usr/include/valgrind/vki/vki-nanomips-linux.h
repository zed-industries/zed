
/*--------------------------------------------------------------------*/
/*-- nanoMIPS/Linux-specific kernel interface  vki-nanomips-linux.h --*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2018 RT-RK
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
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
   02111-1307, USA.

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef __VKI_NANOMIPS_LINUX_H
#define __VKI_NANOMIPS_LINUX_H

#include <config.h>

#if defined (_MIPSEL)
#define VKI_LITTLE_ENDIAN    1
#elif defined (_MIPSEB)
#define VKI_BIG_ENDIAN       1
#endif

#define VKI_PAGE_MASK        (~(VKI_PAGE_SIZE - 1))
#define VKI_MAX_PAGE_SHIFT   16
#define VKI_MAX_PAGE_SIZE    (1UL << VKI_MAX_PAGE_SHIFT)

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
#define VKI_SIGUSR1          10
#define VKI_SIGSEGV          11
#define VKI_SIGUSR2          12
#define VKI_SIGPIPE          13
#define VKI_SIGALRM          14
#define VKI_SIGTERM          15
#define VKI_SIGSTKFLT        16
#define VKI_SIGCHLD          17
#define VKI_SIGCONT          18
#define VKI_SIGSTOP          19
#define VKI_SIGTSTP          20
#define VKI_SIGTTIN          21
#define VKI_SIGTTOU          22
#define VKI_SIGURG           23
#define VKI_SIGXCPU          24
#define VKI_SIGXFSZ          25
#define VKI_SIGVTALRM        26
#define VKI_SIGPROF          27
#define VKI_SIGWINCH         28
#define VKI_SIGIO            29
#define VKI_SIGPWR           30
#define VKI_SIGSYS           31
#define VKI_SIGRTMIN         32

#define VKI_SIG_BLOCK        0    /* for blocking signals */
#define VKI_SIG_UNBLOCK      1    /* for unblocking signals */
#define VKI_SIG_SETMASK      2    /* for setting the signal mask */

/* default signal handling */
#define VKI_SIG_DFL  ((__vki_sighandler_t)0)
/* ignore signal */
#define VKI_SIG_IGN  ((__vki_sighandler_t)1)

#define VKI_SIGRTMAX         _VKI_NSIG
#define VKI_MINSIGSTKSZ      2048
#define _VKI_NSIG            64
#define _VKI_NSIG_BPW        32
#define _VKI_NSIG_WORDS (_VKI_NSIG / _VKI_NSIG_BPW)

#define VKI_SA_NOCLDSTOP     0x00000001
#define VKI_SA_NOCLDWAIT     0x00000002
#define VKI_SA_SIGINFO       0x00000004
#define VKI_SA_ONSTACK       0x08000000
#define VKI_SA_RESTART       0x10000000
#define VKI_SA_NODEFER       0x40000000
#define VKI_SA_RESETHAND     0x80000000
#define VKI_SA_RESTORER      0x04000000
#define VKI_SA_NOMASK        VKI_SA_NODEFER
#define VKI_SA_ONESHOT       VKI_SA_RESETHAND

#define VKI_SS_ONSTACK       1
#define VKI_SS_DISABLE       2

#define VKI_PROT_NONE        0x0       /* No page permissions */
#define VKI_PROT_READ        0x1       /* page can be read */
#define VKI_PROT_WRITE       0x2       /* page can be written */
#define VKI_PROT_EXEC        0x4       /* page can be executed */
#define VKI_PROT_GROWSDOWN   0x1000000 /* mprotect flag: extend change to start
                                                         of growsdown vma */
#define VKI_PROT_GROWSUP     0x2000000 /* mprotect flag: extend change to end
                                                         of growsup vma */
#define VKI_MAP_SHARED       0x01      /* Share changes */
#define VKI_MAP_PRIVATE      0x02      /* Changes are private */
#define VKI_MAP_TYPE         0x0f      /* Mask for type of mapping */
#define VKI_MAP_FIXED        0x10      /* Interpret addr exactly */
#define VKI_MAP_ANONYMOUS    0x0800    /* don't use a file */
#define VKI_MAP_GROWSDOWN    0x1000    /* stack-like segment */
#define VKI_MAP_DENYWRITE    0x2000    /* ETXTBSY */
#define VKI_MAP_EXECUTABLE   0x4000    /* mark it as an executable */
#define VKI_MAP_LOCKED       0x8000    /* pages are locked */
#define VKI_MAP_NORESERVE    0x0400    /* don't check for reservations */
#define VKI_MAP_POPULATE     0x10000   /* populate (prefault) pagetables */
#define VKI_MAP_NONBLOCK     0x20000   /* do not block on IO */

#define VKI_O_ACCMODE        03
#define VKI_O_RDONLY         00
#define VKI_O_WRONLY         01
#define VKI_O_RDWR           02
#define VKI_O_APPEND         0x000400
#define VKI_O_DSYNC          0x001000
#define VKI_O_NONBLOCK       0x000800
#define VKI_O_CREAT          0x000040
#define VKI_O_TRUNC          0x000200
#define VKI_O_EXCL           0x000080
#define VKI_O_NOCTTY         0x000100
#define VKI_FASYNC           0x002000
#define VKI_O_LARGEFILE      0x008000
#define __VKI_O_SYNC         0x101000
#define VKI_O_DIRECT         0x004000
#define VKI_O_CLOEXEC        0x080000

#define VKI_AT_FDCWD         -100
#define VKI_AT_EMPTY_PATH    0x1000

#define VKI_F_DUPFD          0   /* dup */
#define VKI_F_GETFD          1   /* get close_on_exec */
#define VKI_F_SETFD          2   /* set/clear close_on_exec */
#define VKI_F_GETFL          3   /* get file->f_flags */
#define VKI_F_SETFL          4   /* set file->f_flags */
#define VKI_F_GETLK          5
#define VKI_F_SETLK          6
#define VKI_F_SETLKW         7
#define VKI_F_SETOWN         8   /*  for sockets. */
#define VKI_F_GETOWN         9   /*  for sockets. */
#define VKI_F_SETSIG         10  /*  for sockets. */
#define VKI_F_GETSIG         11  /*  for sockets. */
#define VKI_F_GETLK64        12
#define VKI_F_SETLK64        13
#define VKI_F_SETLKW64       14
#define VKI_F_SETOWN_EX      15
#define VKI_F_GETOWN_EX      16
#define VKI_F_OFD_GETLK      36
#define VKI_F_OFD_SETLK      37
#define VKI_F_OFD_SETLKW     38
#define VKI_F_LINUX_SPECIFIC_BASE 1024

#define VKI_FD_CLOEXEC       1

#define VKI_RLIMIT_DATA      2   /* max data size */
#define VKI_RLIMIT_STACK     3   /* max stack size */
#define VKI_RLIMIT_CORE      4   /* max core file size */
#define VKI_RLIMIT_NOFILE    7   /* max number of open files */

#define VKI_SOL_SOCKET       0xffff

#define VKI_SO_TYPE          0x1008

#define _VKI_IOC_NRBITS      8
#define _VKI_IOC_TYPEBITS    8
#define _VKI_IOC_SIZEBITS    13
#define _VKI_IOC_DIRBITS     3

#define _VKI_IOC_NRMASK      ((1 << _VKI_IOC_NRBITS)-1)
#define _VKI_IOC_TYPEMASK    ((1 << _VKI_IOC_TYPEBITS)-1)
#define _VKI_IOC_SIZEMASK    ((1 << _VKI_IOC_SIZEBITS)-1)
#define _VKI_IOC_DIRMASK     ((1 << _VKI_IOC_DIRBITS)-1)

#define _VKI_IOC_NRSHIFT     0
#define _VKI_IOC_TYPESHIFT   (_VKI_IOC_NRSHIFT+_VKI_IOC_NRBITS)
#define _VKI_IOC_SIZESHIFT   (_VKI_IOC_TYPESHIFT+_VKI_IOC_TYPEBITS)
#define _VKI_IOC_DIRSHIFT    (_VKI_IOC_SIZESHIFT+_VKI_IOC_SIZEBITS)

#define _VKI_IOC_NONE        1U
#define _VKI_IOC_READ        2U
#define _VKI_IOC_WRITE       4U

#define _VKI_IOC(a,b,c,d)    (((a)<<29) | ((b)<<8) | (c) | ((d)<<16))

#define _VKI_IO(type,nr)        _VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)  _VKI_IOC(_VKI_IOC_READ,(type),(nr), \
                                        (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOW(type,nr,size)  _VKI_IOC(_VKI_IOC_WRITE,(type),(nr), \
                                        (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOWR(type,nr,size) _VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE,(type), \
                                        (nr),(_VKI_IOC_TYPECHECK(size)))

#define _VKI_IOC_DIR(nr)   (((nr) >> _VKI_IOC_DIRSHIFT) & _VKI_IOC_DIRMASK)
#define _VKI_IOC_TYPE(nr)  (((nr) >> _VKI_IOC_TYPESHIFT) & _VKI_IOC_TYPEMASK)
#define _VKI_IOC_NR(nr)    (((nr) >> _VKI_IOC_NRSHIFT) & _VKI_IOC_NRMASK)
#define _VKI_IOC_SIZE(nr)  (((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

#define VKI_TCGETA           0x5401
#define VKI_TCSETA           0x5402
#define VKI_TCSETAW          0x5403
#define VKI_TCSETAF          0x5404
#define VKI_TCSBRK           0x5405
#define VKI_TCXONC           0x5406
#define VKI_TCFLSH           0x5407
#define VKI_TCGETS           0x540D
#define VKI_TCSETS           0x540E
#define VKI_TCSETSW          0x540F
#define VKI_TCSETSF          0x5410

#define VKI_FIONREAD         0x467F
#define VKI_FIOCLEX          0x6601
#define VKI_FIONCLEX         0x6602
#define VKI_FIOASYNC         0x667D
#define VKI_FIONBIO          0x667E
#define VKI_FIOQSIZE         0x667F

#define VKI_TIOCSBRK         0x5427 /* BSD compatibility */
#define VKI_TIOCCBRK         0x5428 /* BSD compatibility */
#define VKI_TIOCPKT          0x5470 /* pty: set/clear packet mode */
#define VKI_TIOCNOTTY        0x5471
#define VKI_TIOCSTI          0x5472 /* simulate terminal input */
#define VKI_TIOCSCTTY        0x5480 /* become controlling tty */
#define VKI_TIOCGSOFTCAR     0x5481
#define VKI_TIOCSSOFTCAR     0x5482
#define VKI_TIOCLINUX        0x5483
#define VKI_TIOCGSERIAL      0x5484
#define VKI_TIOCSSERIAL      0x5485
#define VKI_TCSBRKP          0x5486 /* Needed for POSIX tcsendbreak() */
#define VKI_TIOCSERCONFIG    0x5488
#define VKI_TIOCSERGWILD     0x5489
#define VKI_TIOCSERSWILD     0x548a
#define VKI_TIOCGLCKTRMIOS   0x548b
#define VKI_TIOCSLCKTRMIOS   0x548c
#define VKI_TIOCSERGSTRUCT   0x548d /* For debugging only */
#define VKI_TIOCSERGETLSR    0x548e /* Get line status register */
#define VKI_TIOCSERGETMULTI  0x548f /* Get multiport config  */
#define VKI_TIOCSERSETMULTI  0x5490 /* Set multiport config */
#define VKI_TIOCMIWAIT       0x5491 /* wait for a change on serial input line(s) */
#define VKI_TIOCGICOUNT      0x5492 /* read serial port inline interrupt counts */
#define VKI_TIOCGHAYESESP    0x5493 /* Get Hayes ESP configuration */
#define VKI_TIOCSHAYESESP    0x5494 /* Set Hayes ESP configuration */
#define VKI_TIOCGETD         0x7400
#define VKI_TIOCSETD         0x7401
#define VKI_TIOCGETP         0x7408
#define VKI_TIOCSETP         0x7409
#define VKI_TIOCSETN         0x740A /* TIOCSETP wo flush */
#define VKI_TIOCEXCL         0x740D /* set exclusive use of tty */
#define VKI_TIOCNXCL         0x740E /* reset exclusive use of tty */
#define VKI_TIOCGSID         0x7416 /* Return the session ID of FD */
#define VKI_TIOCMSET         0x741A /* set all modem bits */
#define VKI_TIOCMBIS         0x741B /* bis modem bits */
#define VKI_TIOCMBIC         0x741C /* bic modem bits */
#define VKI_TIOCMGET         0x741D /* get all modem bits */
#define VKI_TIOCOUTQ         0x7472 /* output queue size */
#define VKI_TIOCGLTC         0x7474
#define VKI_TIOCSLTC         0x7475
#define VKI_TIOCINQ          VKI_FIONREAD

#define VKI_TIOCSWINSZ  _VKI_IOW('t', 103, struct vki_winsize)
#define VKI_TIOCGWINSZ  _VKI_IOR('t', 104, struct vki_winsize)
#define VKI_TIOCSPGRP   _VKI_IOW('t', 118, int)
#define VKI_TIOCGPGRP   _VKI_IOR('t', 119, int)
#define VKI_TIOCCONS    _VKI_IOW('t', 120, int)
#define VKI_TIOCGPTN    _VKI_IOR('T',0x30, unsigned int)
#define VKI_TIOCSPTLCK  _VKI_IOW('T',0x31, int)

#define VKI_TIOCPKT_DATA       0x00    /* data packet */
#define VKI_TIOCPKT_FLUSHREAD  0x01    /* flush packet */
#define VKI_TIOCPKT_FLUSHWRITE 0x02    /* flush packet */
#define VKI_TIOCPKT_STOP       0x04    /* stop output */
#define VKI_TIOCPKT_START      0x08    /* start output */
#define VKI_TIOCPKT_NOSTOP     0x10    /* no more ^S, ^Q */
#define VKI_TIOCPKT_DOSTOP     0x20    /* now do ^S ^Q */

#define NCC                  8
#define NCCS                 23

#define VKI_SO_ATTACH_FILTER 26

#define VKI_SIOCATMARK  _VKI_IOR('s', 7, int)
#define VKI_SIOCSPGRP   _VKI_IOW('s', 8, vki_pid_t)
#define VKI_SIOCGPGRP   _VKI_IOR('s', 9, vki_pid_t)
#define VKI_SIOCGSTAMP       0x8906
#define VKI_SIOCGSTAMPNS     0x8907

#define VKI_POLLIN           0x0001

#define VKI_SEMOP            1
#define VKI_SEMGET           2
#define VKI_SEMCTL           3
#define VKI_SEMTIMEDOP       4
#define VKI_MSGSND           11
#define VKI_MSGRCV           12
#define VKI_MSGGET           13
#define VKI_MSGCTL           14
#define VKI_SHMAT            21
#define VKI_SHMDT            22
#define VKI_SHMGET           23
#define VKI_SHMCTL           24
#define VKI_SHMLBA           0x40000

#define VKI_EF_NANOMIPS_ABI     0x0000F000
#define VKI_EF_NANOMIPS_ABI_P32 0x00001000

#define VKI_PTRACE_GETREGS   12
#define VKI_PTRACE_SETREGS   13
#define VKI_PTRACE_GETFPREGS 14
#define VKI_PTRACE_SETFPREGS 15

#define VKI_MIPS32_EF_R0     6
#define VKI_MIPS32_EF_R1     7
#define VKI_MIPS32_EF_R2     8
#define VKI_MIPS32_EF_R3     9
#define VKI_MIPS32_EF_R4     10
#define VKI_MIPS32_EF_R5     11
#define VKI_MIPS32_EF_R6     12
#define VKI_MIPS32_EF_R7     13
#define VKI_MIPS32_EF_R8     14
#define VKI_MIPS32_EF_R9     15
#define VKI_MIPS32_EF_R10    16
#define VKI_MIPS32_EF_R11    17
#define VKI_MIPS32_EF_R12    18
#define VKI_MIPS32_EF_R13    19
#define VKI_MIPS32_EF_R14    20
#define VKI_MIPS32_EF_R15    21
#define VKI_MIPS32_EF_R16    22
#define VKI_MIPS32_EF_R17    23
#define VKI_MIPS32_EF_R18    24
#define VKI_MIPS32_EF_R19    25
#define VKI_MIPS32_EF_R20    26
#define VKI_MIPS32_EF_R21    27
#define VKI_MIPS32_EF_R22    28
#define VKI_MIPS32_EF_R23    29
#define VKI_MIPS32_EF_R24    30
#define VKI_MIPS32_EF_R25    31
#define VKI_MIPS32_EF_R26    32
#define VKI_MIPS32_EF_R27    33
#define VKI_MIPS32_EF_R28    34
#define VKI_MIPS32_EF_R29    35
#define VKI_MIPS32_EF_R30    36
#define VKI_MIPS32_EF_R31    37

#define VKI_MIPS32_EF_CP0_EPC      40
#define VKI_MIPS32_EF_CP0_BADVADDR 41
#define VKI_MIPS32_EF_CP0_STATUS   42
#define VKI_MIPS32_EF_CP0_CAUSE    43
#define VKI_MIPS32_EF_UNUSED0      44

#define VKI_ELF_NGREG        45
#define VKI_ELF_NFPREG       1 /* Not used. Just to satisfy compiler. */

#define VKI_AT_SYSINFO       32

#ifndef __VKI_ARCH_SI_PREAMBLE_SIZE
#define __VKI_ARCH_SI_PREAMBLE_SIZE (3 * sizeof(int))
#endif

#define VKI_SI_MAX_SIZE 128

#ifndef VKI_SI_PAD_SIZE
#define VKI_SI_PAD_SIZE ((VKI_SI_MAX_SIZE - __VKI_ARCH_SI_PREAMBLE_SIZE) \
                         / sizeof(int))
#endif

#ifndef __VKI_ARCH_SI_UID_T
#define __VKI_ARCH_SI_UID_T   vki_uid_t
#endif

#ifndef __VKI_ARCH_SI_BAND_T
#define __VKI_ARCH_SI_BAND_T long
#endif

#define VKI_BRK_OVERFLOW     6   /* Overflow check */
#define VKI_BRK_DIVZERO      7   /* Divide by zero check */

#define VKI_ELIBBAD          84  /* Accessing a corrupted shared library */
#define VKI_EOPNOTSUPP       122 /* Operation not supported on transport
                                    endpoint */
#define VKI_ENOSYS           38  /* Function not implemented */
#define VKI_EOVERFLOW        75  /* Value too large for defined data type */

#define ARCH_HAS_SOCKET_TYPES 1
#define HAVE_ARCH_SIGINFO_T   1

#define VKI_RLIM_INFINITY 0x7fffffffUL

typedef __signed__ char __vki_s8;
typedef unsigned   char __vki_u8;

typedef __signed__ short __vki_s16;
typedef unsigned   short __vki_u16;

typedef __signed__ int __vki_s32;
typedef unsigned   int __vki_u32;

typedef __signed__ long long __vki_s64;
typedef unsigned   long long __vki_u64;

typedef __signed char vki_s8;
typedef unsigned char vki_u8;

typedef __signed short vki_s16;
typedef unsigned short vki_u16;

typedef __signed int vki_s32;
typedef unsigned int vki_u32;

typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

typedef unsigned long vki_old_sigset_t;

typedef struct {
   unsigned long sig[_VKI_NSIG_WORDS];
} vki_sigset_t;

struct vki_old_sigaction {
   __vki_sighandler_t ksa_handler;
   unsigned int sa_flags;
   vki_old_sigset_t sa_mask;
   __vki_sigrestore_t sa_restorer;
};

struct vki_sigaction_base {
   __vki_sighandler_t ksa_handler;
   unsigned int sa_flags;
   vki_sigset_t sa_mask;
   __vki_sigrestore_t sa_restorer;
};

// On Linux we use the same type for passing sigactions to
// and from the kernel.  Hence: */
typedef  struct vki_sigaction_base  vki_sigaction_toK_t;
typedef  struct vki_sigaction_base  vki_sigaction_fromK_t;

typedef struct vki_sigaltstack {
   void __user *ss_sp;
   int ss_flags;
   vki_size_t ss_size;
} vki_stack_t;

struct vki_sigcontext {
   __vki_u64 sc_regs[32];
   __vki_u64 sc_pc;
   __vki_u32 sc_used_math;
   __vki_u32 sc_reserved;
};

struct vki_winsize {
   unsigned short ws_row;
   unsigned short ws_col;
   unsigned short ws_xpixel;
   unsigned short ws_ypixel;
};

struct vki_termio {
   unsigned short c_iflag;   /* input mode flags */
   unsigned short c_oflag;   /* output mode flags */
   unsigned short c_cflag;   /* control mode flags */
   unsigned short c_lflag;   /* local mode flags */
   char c_line;              /* line discipline */
   unsigned char c_cc[NCCS]; /* control characters */
};

typedef unsigned char   vki_cc_t;
typedef unsigned long   vki_speed_t;
typedef unsigned long   vki_tcflag_t;

struct vki_termios {
   vki_tcflag_t c_iflag;     /* input mode flags */
   vki_tcflag_t c_oflag;     /* output mode flags */
   vki_tcflag_t c_cflag;     /* control mode flags */
   vki_tcflag_t c_lflag;     /* local mode flags */
   vki_cc_t c_line;          /* line discipline */
   vki_cc_t c_cc[NCCS];      /* control characters */
};

struct vki_f_owner_ex {
   int type;
   __vki_kernel_pid_t pid;
};

struct vki_pollfd {
   int fd;
   short events;
   short revents;
};

struct vki_ucontext {
   unsigned long uc_flags;
   struct vki_ucontext *uc_link;
   vki_stack_t uc_stack;
   struct vki_sigcontext uc_mcontext;
   vki_sigset_t uc_sigmask; /* mask last for extensibility */
};

typedef char vki_modify_ldt_t;

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

struct vki_semid64_ds {
   struct vki_ipc64_perm sem_perm;             /* permissions .. see ipc.h */
   __vki_kernel_time_t sem_otime;              /* last semop time */
   __vki_kernel_time_t sem_ctime;              /* last change time */
   unsigned long sem_nsems;              /* no. of semaphores in array */
   unsigned long __unused1;
   unsigned long __unused2;
};

struct vki_msqid64_ds {
   struct vki_ipc64_perm msg_perm;
   __vki_kernel_time_t msg_stime;   /* last msgsnd time */
   unsigned long  __unused1;
   __vki_kernel_time_t msg_rtime;   /* last msgrcv time */
   unsigned long  __unused2;
   __vki_kernel_time_t msg_ctime;   /* last change time */
   unsigned long  __unused3;
   unsigned long  msg_cbytes; /* current number of bytes on queue */
   unsigned long  msg_qnum;   /* number of messages in queue */
   unsigned long  msg_qbytes; /* max number of bytes on queue */
   __vki_kernel_pid_t msg_lspid; /* pid of last msgsnd */
   __vki_kernel_pid_t msg_lrpid; /* last receive pid */
   unsigned long  __unused4;
   unsigned long  __unused5;
};

struct vki_ipc_kludge {
   struct vki_msgbuf __user *msgp;
   long msgtyp;
};

struct vki_shmid64_ds {
   struct vki_ipc64_perm       shm_perm;       /* operation perms */
   vki_size_t                  shm_segsz;      /* size of segment (bytes) */
   __vki_kernel_time_t         shm_atime;      /* last attach time */
   __vki_kernel_time_t         shm_dtime;      /* last detach time */
   __vki_kernel_time_t         shm_ctime;      /* last change time */
   __vki_kernel_pid_t          shm_cpid;       /* pid of creator */
   __vki_kernel_pid_t          shm_lpid;       /* pid of last operator */
   unsigned long           shm_nattch;     /* no. of current attaches */
   unsigned long           __unused1;
   unsigned long           __unused2;
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

typedef unsigned long vki_elf_greg_t;

typedef vki_elf_greg_t vki_elf_gregset_t[VKI_ELF_NGREG];

typedef double vki_elf_fpreg_t;

typedef vki_elf_fpreg_t vki_elf_fpregset_t[VKI_ELF_NFPREG];

typedef struct vki_user_fxsr_struct vki_elf_fpxregset_t;

typedef union vki_sigval {
   int sival_int;
   void __user *sival_ptr;
} vki_sigval_t;

typedef struct vki_siginfo {
   int si_signo;
   int si_errno;
   int si_code;
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

enum vki_sock_type {
   VKI_SOCK_STREAM = 2,
};

/* nanoMIPS uses runtime pagesize detection */
extern UWord VKI_PAGE_SHIFT;
extern UWord VKI_PAGE_SIZE;

#endif

/*--------------------------------------------------------------------*/
/*--- end                                     vki-nanomips-linux.h ---*/
/*--------------------------------------------------------------------*/
