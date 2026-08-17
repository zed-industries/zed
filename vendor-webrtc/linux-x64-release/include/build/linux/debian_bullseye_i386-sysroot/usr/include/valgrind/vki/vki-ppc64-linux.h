
/*--------------------------------------------------------------------*/
/*--- PPC64/Linux-specific kernel interface.     vki-ppc64-linux.h ---*/
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

#ifndef __VKI_PPC64_LINUX_H
#define __VKI_PPC64_LINUX_H

#if defined(VGP_ppc32_linux) || defined(VGP_ppc64be_linux)
#define VKI_BIG_ENDIAN  1
#elif defined(VGP_ppc64le_linux)
#define VKI_LITTLE_ENDIAN  1
#endif
//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/types.h
//----------------------------------------------------------------------

typedef __signed__ char __vki_s8;
typedef unsigned char __vki_u8;

typedef __signed__ short __vki_s16;
typedef unsigned short __vki_u16;

typedef __signed__ int __vki_s32;
typedef unsigned int __vki_u32;

typedef __signed__ long __vki_s64;
typedef unsigned long __vki_u64;

typedef struct {
  __vki_u32 u[4];
} __attribute((aligned(16))) __vki_vector128;

typedef unsigned short vki_u16;

typedef unsigned int vki_u32;

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/page.h
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
// From linux-2.6.13/include/asm-ppc64/signal.h
//----------------------------------------------------------------------

#define VKI_MINSIGSTKSZ     2048

/* Next 9 non-blank lines asm-generic/signal.h */
#define VKI_SIG_BLOCK          0 /* for blocking signals */
#define VKI_SIG_UNBLOCK        1 /* for unblocking signals */
#define VKI_SIG_SETMASK        2 /* for setting the signal mask */

typedef void __vki_signalfn_t(int);
typedef __vki_signalfn_t __user *__vki_sighandler_t;

typedef void __vki_restorefn_t(void);
typedef __vki_restorefn_t __user *__vki_sigrestore_t;

#define VKI_SIG_DFL ((__vki_sighandler_t)0)     /* default signal handling */
#define VKI_SIG_IGN ((__vki_sighandler_t)1)     /* ignore signal */

/* Back in asm-ppc64/signal.h */
#define _VKI_NSIG           64
#define _VKI_NSIG_BPW       64
#define _VKI_NSIG_WORDS     (_VKI_NSIG / _VKI_NSIG_BPW)

typedef unsigned long vki_old_sigset_t;             /* at least 32 bits */

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
#define VKI_SIGPWR          30
#define VKI_SIGSYS          31
#define VKI_SIGUNUSED       31

/* These should not be considered constants from userland.  */
#define VKI_SIGRTMIN        32
#define VKI_SIGRTMAX        _VKI_NSIG

#define VKI_SA_NOCLDSTOP    0x00000001u
#define VKI_SA_NOCLDWAIT    0x00000002u
#define VKI_SA_SIGINFO      0x00000004u
#define VKI_SA_ONSTACK      0x08000000u
#define VKI_SA_RESTART      0x10000000u
#define VKI_SA_NODEFER      0x40000000u
#define VKI_SA_RESETHAND    0x80000000u

#define VKI_SA_NOMASK       VKI_SA_NODEFER
#define VKI_SA_ONESHOT      VKI_SA_RESETHAND
//#define VKI_SA_INTERRUPT    0x20000000u /* dummy -- ignored */

#define VKI_SA_RESTORER     0x04000000u

#define VKI_SS_ONSTACK      1
#define VKI_SS_DISABLE      2

// See comments on corresponding decls in vki-x86-linux.h re ksa_handler
struct vki_old_sigaction {
  __vki_sighandler_t ksa_handler;
  vki_old_sigset_t sa_mask;
  unsigned long sa_flags;
  __vki_sigrestore_t sa_restorer;
};

struct vki_sigaction_base {
  __vki_sighandler_t ksa_handler;
  unsigned long sa_flags;
  __vki_sigrestore_t sa_restorer;
  vki_sigset_t sa_mask;               /* mask last for extensibility */
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
// From linux-2.6.13/include/asm-ppc64/ptrace.h
//----------------------------------------------------------------------

#define VKI_PPC_REG unsigned long
struct vki_pt_regs {
  VKI_PPC_REG gpr[32];
  VKI_PPC_REG nip;
  VKI_PPC_REG msr;
  VKI_PPC_REG orig_gpr3;      /* Used for restarting system calls */
  VKI_PPC_REG ctr;
  VKI_PPC_REG link;
  VKI_PPC_REG xer;
  VKI_PPC_REG ccr;
  VKI_PPC_REG softe;          /* Soft enabled/disabled */
  VKI_PPC_REG trap;           /* Reason for being here */
  VKI_PPC_REG dar;            /* Fault registers */
  VKI_PPC_REG dsisr;
  VKI_PPC_REG result;         /* Result of a system call */

  /* Not in kernel's definition, but apparently needed to stop
     assertion at coredump-elf.c:267 firing.  These padding words make
     the struct have the same size as a 'vki_elf_greg_t'.  See message
     from Ghassan Hammouri on valgrind-developers on 6 April 06, and
     also the analogous kludge for ppc32-linux (svn r5852 and bug
     #121617). */
  unsigned long pad[4];
};

/* Kludge?  I don't know where this came from or if it is right. */
#define vki_user_regs_struct vki_pt_regs

#define VKI_PT_R0   0
#define VKI_PT_R1   1
#define VKI_PT_R2   2
#define VKI_PT_R3   3
#define VKI_PT_R4   4
#define VKI_PT_R5   5
#define VKI_PT_R6   6
#define VKI_PT_R7   7
#define VKI_PT_R8   8
#define VKI_PT_R9   9
#define VKI_PT_R10  10
#define VKI_PT_R11  11
#define VKI_PT_R12  12
#define VKI_PT_R13  13
#define VKI_PT_R14  14
#define VKI_PT_R15  15
#define VKI_PT_R16  16
#define VKI_PT_R17  17
#define VKI_PT_R18  18
#define VKI_PT_R19  19
#define VKI_PT_R20  20
#define VKI_PT_R21  21
#define VKI_PT_R22  22
#define VKI_PT_R23  23
#define VKI_PT_R24  24
#define VKI_PT_R25  25
#define VKI_PT_R26  26
#define VKI_PT_R27  27
#define VKI_PT_R28  28
#define VKI_PT_R29  29
#define VKI_PT_R30  30
#define VKI_PT_R31  31
#define VKI_PT_NIP  32
#define VKI_PT_MSR  33
#define VKI_PT_ORIG_R3 34
#define VKI_PT_CTR  35
#define VKI_PT_LNK  36
#define VKI_PT_XER  37
#define VKI_PT_CCR  38
#define VKI_PT_SOFTE 39
#define VKI_PT_RESULT 43

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/elf.h
//----------------------------------------------------------------------

#define VKI_ELF_NGREG       48      /* includes nip, msr, lr, etc. */
#define VKI_ELF_NFPREG      33      /* includes fpscr */
#define VKI_ELF_NVRREG      34      /* includes vscr & vrsave in split vectors */

typedef unsigned long vki_elf_greg_t64;
typedef vki_elf_greg_t64 vki_elf_gregset_t64[VKI_ELF_NGREG];

typedef vki_elf_gregset_t64 vki_elf_gregset_t;

typedef double vki_elf_fpreg_t;
typedef vki_elf_fpreg_t vki_elf_fpregset_t[VKI_ELF_NFPREG];

/* Altivec registers */
/*
 * The entries with indexes 0-31 contain the corresponding vector registers.
 * The entry with index 32 contains the vscr as the last word (offset 12)
 * within the quadword.  This allows the vscr to be stored as either a
 * quadword (since it must be copied via a vector register to/from storage)
 * or as a word.  The entry with index 33 contains the vrsave as the first
 * word (offset 0) within the quadword.
 *
 * This definition of the VMX state is compatible with the current PPC32
 * ptrace interface.  This allows signal handling and ptrace to use the same
 * structures.  This also simplifies the implementation of a bi-arch
 * (combined (32- and 64-bit) gdb.
 *
 * Note that it's _not_ compatible with 32 bits ucontext which stuffs the
 * vrsave along with vscr and so only uses 33 vectors for the register set
 */
typedef __vki_vector128 vki_elf_vrreg_t;
typedef vki_elf_vrreg_t vki_elf_vrregset_t[VKI_ELF_NVRREG];

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/sigcontext.h
//----------------------------------------------------------------------

struct vki_sigcontext {
  unsigned long       _unused[4];
  int                 signal;
  int                 _pad0;
  unsigned long       handler;
  unsigned long       oldmask;
  struct vki_pt_regs  __user *regs;
  vki_elf_gregset_t   gp_regs;
  vki_elf_fpregset_t  fp_regs;
  /*
   * To maintain compatibility with current implementations the sigcontext is
   * extended by appending a pointer (v_regs) to a quadword type (elf_vrreg_t)
   * followed by an unstructured (vmx_reserve) field of 69 doublewords.  This
   * allows the array of vector registers to be quadword aligned independent of
   * the alignment of the containing sigcontext or ucontext. It is the
   * responsibility of the code setting the sigcontext to set this pointer to
   * either NULL (if this processor does not support the VMX feature) or the
   * address of the first quadword within the allocated (vmx_reserve) area.
   *
   * The pointer (v_regs) of vector type (elf_vrreg_t) is type compatible with
   * an array of 34 quadword entries (elf_vrregset_t).  The entries with
   * indexes 0-31 contain the corresponding vector registers.  The entry with
   * index 32 contains the vscr as the last word (offset 12) within the
   * quadword.  This allows the vscr to be stored as either a quadword (since
   * it must be copied via a vector register to/from storage) or as a word.
   * The entry with index 33 contains the vrsave as the first word (offset 0)
   * within the quadword.
   */
  vki_elf_vrreg_t  __user *v_regs;
  long             vmx_reserve[VKI_ELF_NVRREG+VKI_ELF_NVRREG+1];
};

//----------------------------------------------------------------------
// From linux-5.0.0/arch/powerpc/include/uapi/asm/siginfo.h
//----------------------------------------------------------------------

#ifdef __powerpc64__
# define __VKI_ARCH_SI_PREAMBLE_SIZE     (4 * sizeof(int))
#endif

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/mman.h
//----------------------------------------------------------------------

#define VKI_PROT_NONE       0x0             /* page can not be accessed */
#define VKI_PROT_READ       0x1             /* page can be read */
#define VKI_PROT_WRITE      0x2             /* page can be written */
#define VKI_PROT_EXEC       0x4             /* page can be executed */
#define VKI_PROT_GROWSDOWN  0x01000000      /* mprotect flag: extend
					       change to start of
					       growsdown vma */
#define VKI_PROT_GROWSUP    0x02000000      /* mprotect flag: extend
					       change to end of
					       growsup vma */

#define VKI_MAP_SHARED      0x01            /* Share changes */
#define VKI_MAP_PRIVATE     0x02            /* Changes are private */
#define VKI_MAP_FIXED       0x10            /* Interpret addr exactly */
#define VKI_MAP_ANONYMOUS   0x20            /* don't use a file */
#define VKI_MAP_NORESERVE   0x40            /* don't reserve swap pages */

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/fcntl.h
//----------------------------------------------------------------------

#define VKI_O_ACCMODE	         03
#define VKI_O_RDONLY             00
#define VKI_O_WRONLY             01
#define VKI_O_RDWR               02
#define VKI_O_CREAT            0100 /* not fcntl */
#define VKI_O_EXCL             0200 /* not fcntl */
#define VKI_O_TRUNC           01000 /* not fcntl */
#define VKI_O_APPEND          02000
#define VKI_O_NONBLOCK        04000
#define VKI_O_LARGEFILE     0200000

#define VKI_AT_FDCWD            -100

#define VKI_F_DUPFD         0       /* dup */
#define VKI_F_GETFD         1       /* get close_on_exec */
#define VKI_F_SETFD         2       /* set/clear close_on_exec */
#define VKI_F_GETFL         3       /* get file->f_flags */
#define VKI_F_SETFL         4       /* set file->f_flags */
#define VKI_F_GETLK         5
#define VKI_F_SETLK         6
#define VKI_F_SETLKW        7

#define VKI_F_SETOWN        8       /*  for sockets. */
#define VKI_F_GETOWN        9       /*  for sockets. */
#define VKI_F_SETSIG        10      /*  for sockets. */
#define VKI_F_GETSIG        11      /*  for sockets. */

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
#define VKI_FD_CLOEXEC  1  /* actually anything with low bit set goes */

#define VKI_F_LINUX_SPECIFIC_BASE   1024

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/resource.h
//----------------------------------------------------------------------

// which just does #include <asm-generic/resource.h>

#define VKI_RLIMIT_DATA             2       /* max data size */
#define VKI_RLIMIT_STACK            3       /* max stack size */
#define VKI_RLIMIT_CORE             4       /* max core file size */
#define VKI_RLIMIT_NOFILE           7       /* max number of open files */

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/socket.h
//----------------------------------------------------------------------

#define VKI_SOL_SOCKET      1

#define VKI_SO_TYPE         3

#define VKI_SO_ATTACH_FILTER	26

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/sockios.h
//----------------------------------------------------------------------

#define VKI_SIOCSPGRP       0x8902
#define VKI_SIOCGPGRP       0x8904
#define VKI_SIOCATMARK      0x8905
#define VKI_SIOCGSTAMP      0x8906          /* Get stamp (timeval) */
#define VKI_SIOCGSTAMPNS    0x8907          /* Get stamp (timespec) */

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/stat.h
//----------------------------------------------------------------------

struct vki_stat {
  unsigned long   st_dev;
  unsigned long   st_ino;
  unsigned long   st_nlink;
  unsigned int    st_mode;
  unsigned int    st_uid;
  unsigned int    st_gid;
  unsigned long   st_rdev;
  long            st_size;
  unsigned long   st_blksize;
  unsigned long   st_blocks;
  unsigned long   st_atime;
  unsigned long   st_atime_nsec;
  unsigned long   st_mtime;
  unsigned long   st_mtime_nsec;
  unsigned long   st_ctime;
  unsigned long   st_ctime_nsec;
  unsigned long   __unused4;
  unsigned long   __unused5;
  unsigned long   __unused6;
};

#define VKI_STAT_HAVE_NSEC 1

/* This matches struct stat64 in glibc2.1. Only used for 32 bit. */
struct vki_stat64 {
  unsigned long st_dev;           /* Device.  */
  unsigned long st_ino;           /* File serial number.  */
  unsigned int st_mode;           /* File mode.  */
  unsigned int st_nlink;          /* Link count.  */
  unsigned int st_uid;            /* User ID of the file's owner.  */
  unsigned int st_gid;            /* Group ID of the file's group. */
  unsigned long st_rdev;          /* Device number, if device.  */
  unsigned short __pad2;
  long st_size;                   /* Size of file, in bytes.  */
  int  st_blksize;                /* Optimal block size for I/O.  */

  long st_blocks;                 /* Number 512-byte blocks allocated. */
  int   st_atime;                 /* Time of last access.  */
  int   st_atime_nsec;
  int   st_mtime;                 /* Time of last modification.  */
  int   st_mtime_nsec;
  int   st_ctime;                 /* Time of last status change.  */
  int   st_ctime_nsec;
  unsigned int   __unused4;
  unsigned int   __unused5;
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/statfs.h
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
// From linux-2.6.13/include/asm-ppc64/termios.h
//----------------------------------------------------------------------

struct vki_winsize {
  unsigned short ws_row;
  unsigned short ws_col;
  unsigned short ws_xpixel;
  unsigned short ws_ypixel;
};

#define VKI_NCC 10
struct vki_termio {
  unsigned short c_iflag;         /* input mode flags */
  unsigned short c_oflag;         /* output mode flags */
  unsigned short c_cflag;         /* control mode flags */
  unsigned short c_lflag;         /* local mode flags */
  unsigned char c_line;           /* line discipline */
  unsigned char c_cc[VKI_NCC];    /* control characters */
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/termbits.h
//----------------------------------------------------------------------

typedef unsigned char   vki_cc_t;
typedef unsigned int    vki_speed_t;
typedef unsigned int    vki_tcflag_t;

#define VKI_NCCS 19
struct vki_termios {
  vki_tcflag_t c_iflag;               /* input mode flags */
  vki_tcflag_t c_oflag;               /* output mode flags */
  vki_tcflag_t c_cflag;               /* control mode flags */
  vki_tcflag_t c_lflag;               /* local mode flags */
  vki_cc_t c_cc[VKI_NCCS];            /* control characters */
  vki_cc_t c_line;                    /* line discipline (== c_cc[19]) */
  vki_speed_t c_ispeed;               /* input speed */
  vki_speed_t c_ospeed;               /* output speed */
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/ioctl.h
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

/*
 * Direction bits _IOC_NONE could be 0, but OSF/1 gives it a bit.
 * And this turns out useful to catch old ioctl numbers in header
 * files for us.
 */
#define _VKI_IOC_NONE       1U
#define _VKI_IOC_READ       2U
#define _VKI_IOC_WRITE      4U

#define _VKI_IOC(dir,type,nr,size) \
        (((dir)  << _VKI_IOC_DIRSHIFT) | \
         ((type) << _VKI_IOC_TYPESHIFT) | \
         ((nr)   << _VKI_IOC_NRSHIFT) | \
         ((size) << _VKI_IOC_SIZESHIFT))

/* used to create numbers */
#define _VKI_IO(type,nr)            _VKI_IOC(_VKI_IOC_NONE,(type),(nr),0)
#define _VKI_IOR(type,nr,size)      _VKI_IOC(_VKI_IOC_READ,(type),(nr), \
                                       (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOW(type,nr,size)      _VKI_IOC(_VKI_IOC_WRITE,(type),(nr), \
                                       (_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOWR(type,nr,size)     _VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE, \
                                       (type),(nr),(_VKI_IOC_TYPECHECK(size)))
#define _VKI_IOR_BAD(type,nr,size)  _VKI_IOC(_VKI_IOC_READ,(type),(nr), \
                                       sizeof(size))
#define _VKI_IOW_BAD(type,nr,size)  _VKI_IOC(_VKI_IOC_WRITE,(type),(nr), \
                                       sizeof(size))
#define _VKI_IOWR_BAD(type,nr,size) _VKI_IOC(_VKI_IOC_READ|_VKI_IOC_WRITE, \
                                       (type),(nr),sizeof(size))

/* used to decode them.. */
#define _VKI_IOC_DIR(nr)        (((nr) >> _VKI_IOC_DIRSHIFT) & _VKI_IOC_DIRMASK)
#define _VKI_IOC_TYPE(nr)       (((nr) >> _VKI_IOC_TYPESHIFT) & _VKI_IOC_TYPEMASK)
#define _VKI_IOC_NR(nr)         (((nr) >> _VKI_IOC_NRSHIFT) & _VKI_IOC_NRMASK)
#define _VKI_IOC_SIZE(nr)       (((nr) >> _VKI_IOC_SIZESHIFT) & _VKI_IOC_SIZEMASK)

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/ioctls.h
//----------------------------------------------------------------------

#define VKI_FIOCLEX         _VKI_IO('f', 1)
#define VKI_FIONCLEX        _VKI_IO('f', 2)

#define VKI_TCGETS          _VKI_IOR('t', 19, struct vki_termios)
#define VKI_TCSETS          _VKI_IOW('t', 20, struct vki_termios)
#define VKI_TCSETSW         _VKI_IOW('t', 21, struct vki_termios)
#define VKI_TCSETSF         _VKI_IOW('t', 22, struct vki_termios)
#define VKI_TCGETA          _VKI_IOR('t', 23, struct vki_termio)
#define VKI_TCSETA          _VKI_IOW('t', 24, struct vki_termio)
#define VKI_TCSETAW         _VKI_IOW('t', 25, struct vki_termio)
#define VKI_TCSETAF         _VKI_IOW('t', 28, struct vki_termio)
#define VKI_TCSBRK          _VKI_IO('t', 29)
#define VKI_TCXONC          _VKI_IO('t', 30)
#define VKI_TCFLSH          _VKI_IO('t', 31)
#define VKI_TIOCSCTTY       0x540E
#define VKI_TIOCGPGRP       _VKI_IOR('t', 119, int)
#define VKI_TIOCSPGRP       _VKI_IOW('t', 118, int)
#define VKI_TIOCOUTQ        _VKI_IOR('t', 115, int)     /* output queue size */
#define VKI_TIOCGWINSZ      _VKI_IOR('t', 104, struct vki_winsize)
#define VKI_TIOCSWINSZ      _VKI_IOW('t', 103, struct vki_winsize)
#define VKI_TIOCMGET        0x5415
#define VKI_TIOCMBIS        0x5416
#define VKI_TIOCMBIC        0x5417
#define VKI_TIOCMSET        0x5418
#define VKI_FIONREAD        _VKI_IOR('f', 127, int)
#define VKI_TIOCLINUX       0x541C
#define VKI_FIONBIO         _VKI_IOW('f', 126, int)
#define VKI_TIOCNOTTY       0x5422
#define VKI_TCSBRKP         0x5425  /* Needed for POSIX tcsendbreak() */
#define VKI_TIOCGPTN        _VKI_IOR('T',0x30, unsigned int) 
                            /* Get Pty Number (of pty-mux device) */
#define VKI_TIOCSPTLCK      _VKI_IOW('T',0x31, int)  /* Lock/unlock Pty */
#define VKI_FIOASYNC        _VKI_IOW('f', 125, int)
#define VKI_TIOCSERGETLSR   0x5459 /* Get line status register */
#define VKI_TIOCGICOUNT	    0x545D /* read serial port inline interrupt counts */
#define VKI_FIOQSIZE        _VKI_IOR('f', 128, vki_loff_t)

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/poll.h
//----------------------------------------------------------------------

#define VKI_POLLIN          0x0001

struct vki_pollfd {
  int fd;
  short events;
  short revents;
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/user.h
//----------------------------------------------------------------------

// Not sure what's needed from here

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/elf.h
//----------------------------------------------------------------------

// Not sure what's needed from here

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/ucontext.h
//----------------------------------------------------------------------

struct vki_ucontext {
  unsigned long         uc_flags;
  struct vki_ucontext  *uc_link;
  vki_stack_t           uc_stack;
  vki_sigset_t          uc_sigmask;
  vki_sigset_t          __unused0[15]; /* Allow for uc_sigmask growth */
  struct vki_sigcontext uc_mcontext;  /* last for extensibility */
};

// CAB: TODO
typedef char vki_modify_ldt_t;

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/ipcbuf.h
//----------------------------------------------------------------------

struct vki_ipc64_perm
{
  __vki_kernel_key_t  key;
  __vki_kernel_uid_t  uid;
  __vki_kernel_gid_t  gid;
  __vki_kernel_uid_t  cuid;
  __vki_kernel_gid_t  cgid;
  __vki_kernel_mode_t mode;
  unsigned int        seq;
  unsigned int        __pad1;
  unsigned long       __unused1;
  unsigned long       __unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/sembuf.h
//----------------------------------------------------------------------

struct vki_semid64_ds {
  struct vki_ipc64_perm sem_perm;     /* permissions .. see ipc.h */
  __vki_kernel_time_t   sem_otime;      /* last semop time */
  __vki_kernel_time_t   sem_ctime;      /* last change time */
  unsigned long         sem_nsems;      /* no. of semaphores in array */
  unsigned long         __unused1;
  unsigned long         __unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/msgbuf.h
//----------------------------------------------------------------------

struct vki_msqid64_ds {
  struct vki_ipc64_perm msg_perm;
  __vki_kernel_time_t   msg_stime;      /* last msgsnd time */
  __vki_kernel_time_t   msg_rtime;      /* last msgrcv time */
  __vki_kernel_time_t   msg_ctime;      /* last change time */
  unsigned long         msg_cbytes;      /* current number of bytes on queue */
  unsigned long         msg_qnum;        /* number of messages in queue */
  unsigned long         msg_qbytes;      /* max number of bytes on queue */
  __vki_kernel_pid_t    msg_lspid;       /* pid of last msgsnd */
  __vki_kernel_pid_t    msg_lrpid;       /* last receive pid */
  unsigned long         __unused1;
  unsigned long         __unused2;
};

//----------------------------------------------------------------------
// From linux-2.6.13/include/asm-ppc64/ipc.h
//----------------------------------------------------------------------

// this just does #include <asm-generic/ipc.h>

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
// From linux-2.6.13/include/asm-ppc64/shmbuf.h
//----------------------------------------------------------------------

struct vki_shmid64_ds {
  struct vki_ipc64_perm       shm_perm;       /* operation perms */
  __vki_kernel_time_t         shm_atime;      /* last attach time */
  __vki_kernel_time_t         shm_dtime;      /* last detach time */
  __vki_kernel_time_t         shm_ctime;      /* last change time */
  vki_size_t                  shm_segsz;      /* size of segment (bytes) */
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
// From linux-2.6.8.1/include/asm-generic/errno.h
//----------------------------------------------------------------------

#define	VKI_ENOSYS       38  /* Function not implemented */
#define	VKI_EOVERFLOW    75  /* Value too large for defined data type */

//----------------------------------------------------------------------
// From linux-3.19.0/arch/powerpc/include/uapi/asm/ioctls.h
//----------------------------------------------------------------------

#define VKI_TIOCGSERIAL     0x541E
#define VKI_TIOCSSERIAL     0x541F

//----------------------------------------------------------------------
// end
//----------------------------------------------------------------------

#endif // __VKI_PPC64_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
