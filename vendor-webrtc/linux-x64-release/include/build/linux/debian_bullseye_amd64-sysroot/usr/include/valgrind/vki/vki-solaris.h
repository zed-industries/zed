
/*--------------------------------------------------------------------*/
/*--- Solaris-specific kernel interface.             vki-solaris.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2011-2017 Petr Pavlu
      setup@dagobah.cz

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

/* Copyright 2013-2017, Ivo Raisr <ivosh@ivosh.net> */

/* Copyright 2013, OmniTI Computer Consulting, Inc. All rights reserved. */

/* The purpose of this file is described in vki-linux.h.

   To avoid any copyright issues, vki-solaris.h follows the same approach as
   vki-darwin.h (not copying anything from kernel header files but instead
   just including them).
 */

#ifndef __VKI_SOLARIS_H
#define __VKI_SOLARIS_H

#include "config.h"

/* _XOPEN_SOURCE equal to at least '500' is required so that various system
   structures have all necessary attributes (for example struct msghdr). */
#if !defined(_XOPEN_SOURCE)
/* Compiler versions c99 and higher require _XOPEN_SOURCE at least '600'. */
#   if (__STDC_VERSION__ - 0 >= 199901L)
#      define _XOPEN_SOURCE 600
#   else
#      define _XOPEN_SOURCE 500
#   endif
#elif (_XOPEN_SOURCE - 0 != 500) && (_XOPEN_SOURCE - 0 != 600) && (_XOPEN_SOURCE - 0 != 700)
#   error "Compiler or options invalid for including this header file."
#endif /* _XOPEN_SOURCE */

#define __EXTENSIONS__ 1
/* assert _FILE_OFFSET_BITS == 32 */

#define VKI_PAGE_SHIFT 12
#define VKI_PAGE_SIZE (1UL << VKI_PAGE_SHIFT)
#define VKI_PAGEMASK (~VKI_PAGEOFFSET)
#define VKI_PAGEOFFSET (VKI_PAGE_SIZE - 1)
#define VKI_MAX_PAGE_SHIFT VKI_PAGE_SHIFT
#define VKI_MAX_PAGE_SIZE VKI_PAGE_SIZE


#include <sys/types.h>
#define VKI_UINT_MAX UINT_MAX
#define VKI_UINTPTR_MAX UINTPTR_MAX
#define vki_boolean_t boolean_t
#define vki_datalink_id_t datalink_id_t
#define vki_uint_t uint_t
#define vki_uint32_t uint32_t
#define vki_uint64_t uint64_t
#define vki_ulong_t ulong_t
#define vki_caddr_t caddr_t
#define vki_dev_t dev_t
#define vki_off_t off_t
#define vki_id_t id_t
#define vki_key_t key_t
#define vki_mode_t mode_t
#define vki_o_dev_t o_dev_t
#define vki_projid_t projid_t
#define vki_uid_t uid_t
#define vki_gid_t gid_t
#define vki_pid_t pid_t
#define vki_size_t size_t
#define vki_time_t time_t
#define vki_timer_t timer_t
#define vki_uchar_t uchar_t

typedef uint32_t vki_u32;


#include <sys/types32.h>
#define vki_size32_t size32_t


#include <fcntl.h>
#define VKI_SEEK_SET SEEK_SET


#include <limits.h>
#define VKI_NGROUPS_MAX NGROUPS_MAX
#define VKI_PATH_MAX PATH_MAX
/* Used in launcher-linux.c which we share with Linux port. */
#define VKI_BINPRM_BUF_SIZE VKI_PATH_MAX


#include <ucred.h>
#define vki_ucred_t ucred_t


#include <unistd.h>
#define VKI_R_OK R_OK
#define VKI_W_OK W_OK
#define VKI_X_OK X_OK


#include <bsm/audit.h>
#define VKI_A_GETAMASK A_GETAMASK
#define VKI_A_GETCAR A_GETCAR
#define VKI_A_GETCLASS A_GETCLASS
#define VKI_A_GETCOND A_GETCOND
#define VKI_A_GETCWD A_GETCWD
#define VKI_A_GETKAUDIT A_GETKAUDIT
#define VKI_A_GETKMASK A_GETKMASK
#define VKI_A_GETPINFO A_GETPINFO
#define VKI_A_GETPINFO_ADDR A_GETPINFO_ADDR
#define VKI_A_GETPOLICY A_GETPOLICY
#define VKI_A_GETQCTRL A_GETQCTRL
#if defined(SOLARIS_AUDITON_STAT)
#define VKI_A_GETSTAT A_GETSTAT
#define VKI_A_SETSTAT A_SETSTAT
#endif /* SOLARIS_AUDITON_STAT */
#define VKI_A_SETAMASK A_SETAMASK
#define VKI_A_SETCLASS A_SETCLASS
#define VKI_A_SETCOND A_SETCOND
#define VKI_A_SETKAUDIT A_SETKAUDIT
#define VKI_A_SETKMASK A_SETKMASK
#define VKI_A_SETPMASK A_SETPMASK
#define VKI_A_SETPOLICY A_SETPOLICY
#define VKI_A_SETQCTRL A_SETQCTRL
#define VKI_A_SETSMASK A_SETSMASK
#define VKI_A_SETUMASK A_SETUMASK
#define VKI_BSM_AUDIT BSM_AUDIT
#define VKI_BSM_AUDITCTL BSM_AUDITCTL
#define VKI_BSM_AUDITDOOR BSM_AUDITDOOR
#define VKI_BSM_GETAUDIT BSM_GETAUDIT
#define VKI_BSM_GETAUDIT_ADDR BSM_GETAUDIT_ADDR
#define VKI_BSM_GETAUID BSM_GETAUID
#define VKI_BSM_SETAUDIT BSM_SETAUDIT
#define VKI_BSM_SETAUDIT_ADDR BSM_SETAUDIT_ADDR
#define VKI_BSM_SETAUID BSM_SETAUID
#define vki_au_evclass_map_t au_evclass_map_t
#define vki_au_id_t au_id_t
#define vki_au_mask_t au_mask_t
#define vki_au_qctrl au_qctrl
#define vki_auditinfo_t auditinfo_t
#define vki_auditinfo_addr_t auditinfo_addr_t
#define vki_auditpinfo auditpinfo
#define vki_auditpinfo_addr auditpinfo_addr
#if defined(SOLARIS_AUDITON_STAT)
#define vki_au_stat_t au_stat_t
#endif /* SOLARIS_AUDITON_STAT */


#include <sys/psw.h>
#define VKI_PSL_USER PSL_USER


#include <ia32/sys/trap.h>
#define VKI_T_BPTFLT T_BPTFLT


/* From <libc/inc/libc_int.h> which is consolidation private. */
#define VKI_CI_BIND_GUARD 4
#define VKI_CI_BIND_CLEAR 5
#define VKI_THR_FLG_RTLD 0x01

typedef struct {
   int ci_tag;
   union {
      int (*ci_func)(int);
      long ci_val;
      char *ci_ptr;
   } vki_ci_un;
} vki_Lc_interface;


/* From <libc/port/gen/getxby_door.h> which is consolidation private. */
#if defined(SOLARIS_NSCD_DOOR_SYSTEM_VOLATILE)
#define VKI_NAME_SERVICE_DOOR "/system/volatile/name_service_door"
#else
#define VKI_NAME_SERVICE_DOOR "/var/run/name_service_door"
#endif /* SOLARIS_NSCD_DOOR_SYSTEM_VOLATILE */


#include <nfs/nfs.h>
#include <nfs/nfssys.h>
#define VKI_NFS_REVAUTH NFS_REVAUTH
#define vki_nfs_revauth_args nfs_revauth_args


#include <net/if.h>
#define vki_ifconf ifconf
#define vki_ifreq ifreq
#define vki_lifconf lifconf
#define vki_lifnum lifnum
#define vki_lifreq lifreq


#include <netinet/in.h>
#define VKI_IPPROTO_TCP IPPROTO_TCP
#define vki_in_addr in_addr
#define vki_sockaddr_in sockaddr_in
#define vki_sockaddr_in6 sockaddr_in6

#include <netinet/tcp.h>
#define VKI_TCP_NODELAY TCP_NODELAY


/* Do not include nss_dbdefs.h if a C++ compiler is used to build a file
   which includes vki-solaris.h. This cannot be done because the nss_dbdefs.h
   header file uses 'delete' keyword as a method name. */
#if !defined(__cplusplus)
#include <nss_dbdefs.h>
#define VKI_NSCD_CALLCAT_APP NSCD_CALLCAT_APP
#define VKI_NSCDV2CATMASK NSCDV2CATMASK
#define vki_nss_dbd_t nss_dbd_t
#define vki_nss_pheader_t nss_pheader_t
#endif /* !__cplusplus */


/* From <repcache_protocol.h> which is consolidation private. */
#include "vki-solaris-repcache.h"

#include <sys/acl.h>
#define vki_aclent_t aclent_t
#define vki_ace_t ace_t
#define VKI_GETACL GETACL
#define VKI_SETACL SETACL
#define VKI_GETACLCNT GETACLCNT
#define VKI_ACE_GETACL ACE_GETACL
#define VKI_ACE_SETACL ACE_SETACL
#define VKI_ACE_GETACLCNT ACE_GETACLCNT


#include <sys/auxv.h>
#define vki_auxv_t auxv_t
#define VKI_AT_NULL AT_NULL
#define VKI_AT_PHDR AT_PHDR
#define VKI_AT_PAGESZ AT_PAGESZ
#define VKI_AT_BASE AT_BASE
#define VKI_AT_FLAGS AT_FLAGS
#define VKI_AT_ENTRY AT_ENTRY
#define VKI_AT_SUN_PLATFORM AT_SUN_PLATFORM
#define VKI_AT_SUN_HWCAP AT_SUN_HWCAP
#define VKI_AT_SUN_EXECNAME AT_SUN_EXECNAME
#define VKI_AT_SUN_AUXFLAGS AT_SUN_AUXFLAGS
#if defined(SOLARIS_RESERVE_SYSSTAT_ADDR)
#define VKI_AT_SUN_SYSSTAT_ADDR AT_SUN_SYSSTAT_ADDR
#endif
#if defined(SOLARIS_RESERVE_SYSSTAT_ZONE_ADDR)
#define VKI_AT_SUN_SYSSTAT_ZONE_ADDR AT_SUN_SYSSTAT_ZONE_ADDR
#endif

#define VKI_AF_SUN_HWCAPVERIFY AF_SUN_HWCAPVERIFY


#include <sys/auxv_386.h>
#define VKI_AV_386_FPU AV_386_FPU
#define VKI_AV_386_TSC AV_386_TSC
#define VKI_AV_386_CX8 AV_386_CX8
#define VKI_AV_386_SEP AV_386_SEP
#define VKI_AV_386_AMD_SYSC AV_386_AMD_SYSC
#define VKI_AV_386_CMOV AV_386_CMOV
#define VKI_AV_386_MMX AV_386_MMX
#define VKI_AV_386_FXSR AV_386_FXSR
#define VKI_AV_386_SSE AV_386_SSE
#define VKI_AV_386_SSE2 AV_386_SSE2
#define VKI_AV_386_SSE3 AV_386_SSE3
#define VKI_AV_386_CX16 AV_386_CX16
#define VKI_AV_386_AHF AV_386_AHF
#define VKI_AV_386_TSCP AV_386_TSCP
#define VKI_AV_386_POPCNT AV_386_POPCNT
#define VKI_AV_386_AMD_LZCNT AV_386_AMD_LZCNT
#define VKI_AV_386_SSSE3 AV_386_SSSE3
#define VKI_AV_386_SSE4_1 AV_386_SSE4_1
#define VKI_AV_386_SSE4_2 AV_386_SSE4_2
#define VKI_AV_386_AES AV_386_AES
#define VKI_AV_386_PCLMULQDQ AV_386_PCLMULQDQ
#define VKI_AV_386_XSAVE AV_386_XSAVE


#include <sys/corectl.h>
#define VKI_CC_CONTENT_ANON CC_CONTENT_ANON
#define VKI_CC_CONTENT_DATA CC_CONTENT_DATA
#define VKI_CC_CONTENT_DISM CC_CONTENT_DISM
#define VKI_CC_CONTENT_HEAP CC_CONTENT_HEAP
#define VKI_CC_CONTENT_ISM CC_CONTENT_ISM
#define VKI_CC_CONTENT_RODATA CC_CONTENT_RODATA
#define VKI_CC_CONTENT_SHANON CC_CONTENT_SHANON
#define VKI_CC_CONTENT_SHM CC_CONTENT_SHM
#define VKI_CC_CONTENT_STACK CC_CONTENT_STACK
#define VKI_CC_CONTENT_TEXT CC_CONTENT_TEXT
#define vki_core_content_t core_content_t


/* From <sys/crypto/elfsign.h> which is consolidation private. */
#define VKI__PATH_KCFD_DOOR "/system/volatile/kcfd_door"
typedef enum vki_ELFsign_status_e {
   VKI_ELFSIGN_UNKNOWN,
   VKI_ELFSIGN_SUCCESS,
   VKI_ELFSIGN_FAILED,
   VKI_ELFSIGN_NOTSIGNED,
   VKI_ELFSIGN_INVALID_CERTPATH,
   VKI_ELFSIGN_INVALID_ELFOBJ,
   VKI_ELFSIGN_UNAVAILABLE
} vki_ELFsign_status_t;
typedef struct vki_kcf_door_arg_s {
   short         da_version;
   vki_boolean_t da_iskernel;
   union {
      char filename[MAXPATHLEN];	/* For request */

      struct vki_kcf_door_result_s {	/* For response */
         vki_ELFsign_status_t status;
         vki_uint32_t         siglen;
         vki_uchar_t          signature[1];
      } result;
   } vki_da_u;
} vki_kcf_door_arg_t;


#include <sys/crypto/ioctl.h>
#define VKI_CRYPTO_SUCCESS CRYPTO_SUCCESS
#define VKI_CRYPTO_GET_PROVIDER_LIST CRYPTO_GET_PROVIDER_LIST
#define vki_crypto_provider_id_t crypto_provider_id_t
#define vki_crypto_provider_entry_t crypto_provider_entry_t
#define vki_crypto_get_provider_list_t crypto_get_provider_list_t


#include <sys/dditypes.h>
#include <sys/devinfo_impl.h>
#define VKI_DINFOUSRLD DINFOUSRLD
#define VKI_DINFOIDENT DINFOIDENT


#include <sys/dirent.h>
#define VKI_MAXGETDENTS_SIZE MAXGETDENTS_SIZE
#define vki_dirent dirent
#define vki_dirent64 dirent64


#include <sys/door.h>
#define vki_door_desc_t door_desc_t
#define vki_door_info_t door_info_t
#define vki_door_arg_t door_arg_t
#define vki_door_results door_results
#define vki_door_return_desc_t door_return_desc_t

#define VKI_DOOR_CREATE DOOR_CREATE
#define VKI_DOOR_REVOKE DOOR_REVOKE
#define VKI_DOOR_INFO DOOR_INFO
#define VKI_DOOR_CALL DOOR_CALL
#define VKI_DOOR_BIND DOOR_BIND
#define VKI_DOOR_UNBIND DOOR_UNBIND
#define VKI_DOOR_UNREFSYS DOOR_UNREFSYS
#define VKI_DOOR_UCRED DOOR_UCRED
#define VKI_DOOR_RETURN DOOR_RETURN
#define VKI_DOOR_GETPARAM DOOR_GETPARAM
#define VKI_DOOR_SETPARAM DOOR_SETPARAM


#include <sys/dtrace.h>
#define VKI_DTRACEHIOC_REMOVE DTRACEHIOC_REMOVE
#define VKI_DTRACEHIOC_ADDDOF DTRACEHIOC_ADDDOF
#define vki_dof_helper_t dof_helper_t


#include <sys/elf.h>
#define VKI_EI_CLASS EI_CLASS
#define VKI_EI_DATA EI_DATA
#define VKI_EI_MAG0 EI_MAG0
#define VKI_EI_MAG1 EI_MAG1
#define VKI_EI_MAG2 EI_MAG2
#define VKI_EI_MAG3 EI_MAG3
#define VKI_EI_VERSION EI_VERSION
#define VKI_ELFMAG ELFMAG
#define VKI_ELFMAG ELFMAG
#define VKI_ELFMAG0 ELFMAG0
#define VKI_ELFMAG1 ELFMAG1
#define VKI_ELFMAG2 ELFMAG2
#define VKI_ELFMAG3 ELFMAG3
#define VKI_ET_CORE ET_CORE
#define VKI_ET_DYN ET_DYN
#define VKI_ET_EXEC ET_EXEC
#define VKI_EV_CURRENT EV_CURRENT
#define VKI_NT_AUXV NT_AUXV
#define VKI_NT_CONTENT NT_CONTENT
#define VKI_NT_LWPSINFO NT_LWPSINFO
#define VKI_NT_LWPSTATUS NT_LWPSTATUS
#define VKI_NT_PLATFORM NT_PLATFORM
#define VKI_NT_PRCRED NT_PRCRED
#define VKI_NT_PRFPREG NT_PRFPREG
#define VKI_NT_PRPRIV NT_PRPRIV
#define VKI_NT_PRPRIVINFO NT_PRPRIVINFO
#define VKI_NT_PRPSINFO NT_PRPSINFO
#define VKI_NT_PRSTATUS NT_PRSTATUS
#define VKI_NT_PRXREG NT_PRXREG
#define VKI_NT_PSINFO NT_PSINFO
#define VKI_NT_PSTATUS NT_PSTATUS
#define VKI_NT_UTSNAME NT_UTSNAME
#define VKI_NT_ZONENAME NT_ZONENAME
#define VKI_PF_R PF_R
#define VKI_PF_W PF_W
#define VKI_PF_X PF_X
#define VKI_PN_XNUM PN_XNUM
#define VKI_PT_LOAD PT_LOAD
#define VKI_PT_SUNWBSS PT_SUNWBSS
#define VKI_SELFMAG SELFMAG

#if	VG_WORDSIZE == 8
#define VKI_ESZ(x) Elf64_##x
#elif	VG_WORDSIZE == 4
#define VKI_ESZ(x) Elf32_##x
#else
#error VG_WORDSIZE needs to ==4 or ==8
#endif


#include <sys/errno.h>
#define VKI_EPERM EPERM
#define VKI_ENOENT ENOENT
#define VKI_ESRCH ESRCH
#define VKI_EINTR EINTR
#define VKI_EIO EIO
#define VKI_ENXIO ENXIO
#define VKI_E2BIG E2BIG
#define VKI_EBADF EBADF
#define VKI_ECHILD ECHILD
#define VKI_ENOEXEC ENOEXEC
#define VKI_EAGAIN EAGAIN
#define VKI_ENOMEM ENOMEM
#define VKI_EACCES EACCES
#define VKI_EFAULT EFAULT
#define VKI_ENOTBLK ENOTBLK
#define VKI_EBUSY EBUSY
#define VKI_EEXIST EEXIST
#define VKI_EXDEV EXDEV
#define VKI_ENODEV ENODEV
#define VKI_ENOTDIR ENOTDIR
#define VKI_EISDIR EISDIR
#define VKI_EINVAL EINVAL
#define VKI_ENFILE ENFILE
#define VKI_EMFILE EMFILE
#define VKI_ENOTTY ENOTTY
#define VKI_ETXTBSY ETXTBSY
#define VKI_EFBIG EFBIG
#define VKI_ENOSPC ENOSPC
#define VKI_ESPIPE ESPIPE
#define VKI_EROFS EROFS
#define VKI_EMLINK EMLINK
#define VKI_EPIPE EPIPE
#define VKI_EDOM EDOM
#define VKI_ERANGE ERANGE
#define VKI_ENOTSUP ENOTSUP
#define VKI_ENODATA ENODATA
#define VKI_EOVERFLOW EOVERFLOW
#define VKI_ENOSYS ENOSYS
#define VKI_ERESTART ERESTART
#define VKI_EADDRINUSE EADDRINUSE


#if defined(SOLARIS_EXECVE_SYSCALL_TAKES_FLAGS)
#include <sys/execx.h>
#define VKI_EXEC_DESCRIPTOR EXEC_DESCRIPTOR
#endif /* SOLARIS_EXECVE_SYSCALL_TAKES_FLAGS */


#include <sys/fasttrap.h>
#define VKI_PT_SUNWDTRACE_SIZE PT_SUNWDTRACE_SIZE


#include <sys/fcntl.h>
#define VKI_O_RDONLY O_RDONLY
#define VKI_O_WRONLY O_WRONLY
#define VKI_O_RDWR O_RDWR
#define VKI_O_APPEND O_APPEND
#define VKI_O_NONBLOCK O_NONBLOCK

#define VKI_O_CREAT O_CREAT
#define VKI_O_TRUNC O_TRUNC
#define VKI_O_EXCL O_EXCL
#define VKI_O_LARGEFILE O_LARGEFILE

#define VKI_F_DUPFD F_DUPFD
#define VKI_F_DUPFD_CLOEXEC F_DUPFD_CLOEXEC
#define VKI_F_GETFD F_GETFD
#define VKI_F_SETFD F_SETFD
#define VKI_F_GETFL F_GETFL
#define VKI_F_GETXFL F_GETXFL
#define VKI_F_SETFL F_SETFL

/* SVR3 rfs compatibility const, declared only if _KERNEL or _KMEMUSER is
   defined. */
#if 0
#define VKI_F_O_GETLK F_O_GETLK
#endif // 0

#define VKI_F_DUP2FD F_DUP2FD

/* Mostly unused and kernel-unimplemented commands. In case of F_GETOWN and
   F_GETOWN, they are translated by libc in __fcntl() into other syscalls,
   that means these two values are never passed to the fcntl handler in the
   kernel. F_HASREMOTELOCKS is also special, the fcntl kernel handler doesn't
   know about it but it's used inside the kernel. */
#if 0
#define VKI_F_ISSTREAM F_ISSTREAM
#define VKI_F_PRIV F_PRIV
#define VKI_F_NPRIV F_NPRIV
#define VKI_F_QUATACTL F_QUOTACTL
#define VKI_F_BLOCKS F_BLOCKS
#define VKI_F_BLKSIZE F_BLKSIZE
#define VKI_F_GETOWN F_GETOWN
#define VKI_F_SETOWN F_SETOWN
#define VKI_F_REVOKE F_REVOKE
#define VKI_F_HASREMOTELOCKS F_HASREMOTELOCKS
#endif // 0

#define VKI_F_SETLK F_SETLK
#define VKI_F_SETLKW F_SETLKW
#define VKI_F_ALLOCSP F_ALLOCSP
#define VKI_F_FREESP F_FREESP
#define VKI_F_GETLK F_GETLK
#define VKI_F_SETLK_NBMAND F_SETLK_NBMAND
#if defined(VGP_x86_solaris)
#define VKI_F_SETLK64 F_SETLK64
#define VKI_F_SETLKW64 F_SETLKW64
#define VKI_F_ALLOCSP64 F_ALLOCSP64
#define VKI_F_FREESP64 F_FREESP64
#define VKI_F_GETLK64 F_GETLK64
#define VKI_F_SETLK64_NBMAND F_SETLK64_NBMAND
#endif // defined(VGP_x86_solaris)

#define VKI_F_SHARE F_SHARE
#define VKI_F_UNSHARE F_UNSHARE
#define VKI_F_SHARE_NBMAND F_SHARE_NBMAND

#define VKI_F_BADFD F_BADFD

#define vki_flock flock
#if defined(VGP_x86_solaris)
#define vki_flock64 flock64
#endif // defined(VGP_x86_solaris)

#define VKI_FD_CLOEXEC FD_CLOEXEC

#define vki_fshare fshare

#define VKI_AT_FDCWD AT_FDCWD


#include <sys/filio.h>
#define VKI_FIOSETOWN FIOSETOWN
#define VKI_FIOGETOWN FIOGETOWN


#include <sys/fs/namenode.h>
#define vki_namefd namefd


#include <sys/fstyp.h>
#define VKI_FSTYPSZ FSTYPSZ
#define VKI_GETFSIND GETFSIND
#define VKI_GETFSTYP GETFSTYP
#define VKI_GETNFSTYP GETNFSTYP


#include <sys/ioccom.h>
#define _VKI_IOC_DIR(x) ((x) & (IOC_VOID | IOC_OUT | IOC_IN))
#define _VKI_IOC_SIZE(x) (((x) >> 16) & IOCPARM_MASK)
#define _VKI_IOC_NONE 0
#define _VKI_IOC_READ IOC_OUT
#define _VKI_IOC_WRITE IOC_IN


#include <sys/ipc.h>
#include <sys/ipc_impl.h>
#define VKI_IPC_RMID IPC_RMID
#define VKI_IPC_SET IPC_SET
#define VKI_IPC_SET64 IPC_SET64
#define VKI_IPC_STAT IPC_STAT
#define VKI_IPC_STAT64 IPC_STAT64
#if defined(SOLARIS_SHM_NEW)
#define VKI_IPC_XSTAT64 IPC_XSTAT64
#endif /* SOLARIS_SHM_NEW */

#define vki_semid64_ds semid_ds64


#include <sys/lgrp_user.h>
#if defined(HAVE_SYS_LGRP_USER_IMPL_H)
/* Include implementation specific header file on newer Solaris. */
#include <sys/lgrp_user_impl.h>
#endif /* HAVE_SYS_LGRP_USER_IMPL_H */
#define VKI_LGRP_SYS_MEMINFO LGRP_SYS_MEMINFO
#define VKI_LGRP_SYS_GENERATION LGRP_SYS_GENERATION
#define VKI_LGRP_SYS_VERSION LGRP_SYS_VERSION
#define VKI_LGRP_SYS_SNAPSHOT LGRP_SYS_SNAPSHOT
#define VKI_LGRP_SYS_AFFINITY_GET LGRP_SYS_AFFINITY_GET
#define VKI_LGRP_SYS_AFFINITY_SET LGRP_SYS_AFFINITY_SET
#define VKI_LGRP_SYS_LATENCY LGRP_SYS_LATENCY
#define VKI_LGRP_SYS_HOME LGRP_SYS_HOME
#define VKI_LGRP_SYS_AFF_INHERIT_GET LGRP_SYS_AFF_INHERIT_GET
#define VKI_LGRP_SYS_AFF_INHERIT_SET LGRP_SYS_AFF_INHERIT_SET
#define VKI_LGRP_SYS_DEVICE_LGRPS LGRP_SYS_DEVICE_LGRPS
#define VKI_LGRP_SYS_MAXSOCKETS_GET LGRP_SYS_MAXSOCKETS_GET
#define vki_lgrp_view_t lgrp_view_t


#include <sys/loadavg.h>
#define VKI_LOADAVG_NSTATS LOADAVG_NSTATS


#include <sys/lwp.h>
#define VKI_LWP_DAEMON LWP_DAEMON
#define VKI_LWP_FSBASE _LWP_FSBASE
#define VKI_LWP_GSBASE _LWP_GSBASE
#define VKI_LWP_SETPRIVATE _LWP_SETPRIVATE
#define VKI_LWP_GETPRIVATE _LWP_GETPRIVATE


#include <sys/mman.h>
#define VKI_PROT_READ PROT_READ
#define VKI_PROT_WRITE PROT_WRITE
#define VKI_PROT_EXEC PROT_EXEC
#define VKI_PROT_NONE PROT_NONE

#define VKI_MAP_SHARED MAP_SHARED
#define VKI_MAP_PRIVATE MAP_PRIVATE
#define VKI_MAP_FIXED MAP_FIXED
#define VKI_MAP_ANONYMOUS MAP_ANONYMOUS
#define VKI_MAP_ALIGN MAP_ALIGN
#define VKI_MAP_TEXT MAP_TEXT
#define VKI_MAP_INITDATA MAP_INITDATA

#define VKI_MMOBJ_ALL_FLAGS MMOBJ_ALL_FLAGS
#define VKI_MMOBJ_INTERPRET MMOBJ_INTERPRET
#define VKI_MMOBJ_PADDING MMOBJ_PADDING
#define VKI_MR_PADDING MR_PADDING
#define VKI_MR_HDR_ELF MR_HDR_ELF
#define VKI_MR_GET_TYPE(val) MR_GET_TYPE(val)
#define vki_mmapobj_result_t mmapobj_result_t

#define vki_memcntl_mha memcntl_mha
#define VKI_MC_LOCKAS MC_LOCKAS
#define VKI_MC_UNLOCKAS MC_UNLOCKAS
#define VKI_MC_HAT_ADVISE MC_HAT_ADVISE

#define vki_meminfo_t meminfo_t


#include <sys/mntio.h>
#define VKI_MNTIOC_GETEXTMNTENT MNTIOC_GETEXTMNTENT
#define VKI_MNTIOC_GETMNTANY MNTIOC_GETMNTANY


#include <sys/mnttab.h>
#define vki_extmnttab extmnttab
#define vki_mntentbuf mntentbuf
#define vki_mnttab mnttab


#include <sys/modctl.h>
#define VKI_MODLOAD MODLOAD
#define VKI_MODUNLOAD MODUNLOAD
#define VKI_MODINFO MODINFO

#if defined(SOLARIS_MODCTL_MODNVL)
#define VKI_MODNVL_DEVLINKSYNC MODNVL_DEVLINKSYNC
#define VKI_MODDEVINFO_CACHE_TS MODDEVINFO_CACHE_TS
#if !defined(HAVE_SYS_SYSNVL_H)
#define VKI_MODCTL_NVL_OP_GET MODCTL_NVL_OP_GET
#define VKI_MODCTL_NVL_OP_UPDATE MODCTL_NVL_OP_UPDATE
#endif /* !HAVE_SYS_SYSNVL_H */
#endif /* SOLARIS_MODCTL_MODNVL */

#define vki_modid_t int
#define vki_modinfo modinfo


#include <sys/mount.h>
#define	VKI_MS_DATA MS_DATA
#define	VKI_MS_OPTIONSTR MS_OPTIONSTR


#include <sys/poll.h>
#define vki_pollfd pollfd
#define vki_pollfd_t pollfd_t
#define vki_nfds_t nfds_t


#include <sys/pool_impl.h>
#define VKI_POOL_STATUSQ POOL_STATUSQ
#define vki_pool_status_t pool_status_t


#include <sys/port.h>
#include <sys/port_impl.h>
#define VKI_PORT_SOURCE_FD PORT_SOURCE_FD
#define VKI_PORT_SOURCE_FILE PORT_SOURCE_FILE

#define vki_port_event_t port_event_t
#define vki_port_notify_t port_notify_t
#define vki_file_obj file_obj

#define VKI_PORT_CREATE PORT_CREATE
#define VKI_PORT_ASSOCIATE PORT_ASSOCIATE
#define VKI_PORT_DISSOCIATE PORT_DISSOCIATE
#define VKI_PORT_SEND PORT_SEND
#define VKI_PORT_SENDN PORT_SENDN
#define VKI_PORT_GET PORT_GET
#define VKI_PORT_GETN PORT_GETN
#define VKI_PORT_ALERT PORT_ALERT
#define VKI_PORT_DISPATCH PORT_DISPATCH

#define VKI_PORT_SYS_NOPORT PORT_SYS_NOPORT
#define VKI_PORT_CODE_MASK PORT_CODE_MASK


#include <sys/priocntl.h>
#include <sys/rtpriocntl.h>
#include <sys/tspriocntl.h>
#include <sys/iapriocntl.h>
#include <sys/fsspriocntl.h>
#include <sys/fxpriocntl.h>
#define VKI_PC_GETCID PC_GETCID
#define VKI_PC_GETCLINFO PC_GETCLINFO
#define VKI_PC_SETPARMS PC_SETPARMS
#define VKI_PC_GETPARMS PC_GETPARMS
#define VKI_PC_ADMIN PC_ADMIN
#define VKI_PC_GETPRIRANGE PC_GETPRIRANGE
#define VKI_PC_DONICE PC_DONICE
#define VKI_PC_SETXPARMS PC_SETXPARMS
#define VKI_PC_GETXPARMS PC_GETXPARMS
#define VKI_PC_SETDFLCL PC_SETDFLCL
#define VKI_PC_GETDFLCL PC_GETDFLCL
#define VKI_PC_DOPRIO PC_DOPRIO

#define VKI_PC_CLNMSZ PC_CLNMSZ

#define VKI_PC_GETNICE PC_GETNICE
#define VKI_PC_SETNICE PC_SETNICE

#define VKI_PC_GETPRIO PC_GETPRIO
#define VKI_PC_SETPRIO PC_SETPRIO

#define vki_pcinfo_t pcinfo_t
#define vki_rtinfo_t rtinfo_t
#define vki_tsinfo_t tsinfo_t
#define vki_iainfo_t iainfo_t
#define vki_fssinfo_t fssinfo_t
#define vki_fxinfo_t fxinfo_t

#define vki_pcparms_t pcparms_t
#define vki_pcnice_t pcnice_t
#define vki_pcprio_t pcprio_t
#define vki_pc_vaparm_t pc_vaparm_t
#define vki_pc_vaparms_t pc_vaparms_t
#define vki_pcpri_t pcpri_t

#define VKI_PC_KY_CLNAME PC_KY_CLNAME
#define VKI_RT_KY_PRI RT_KY_PRI
#define VKI_RT_KY_TQSECS RT_KY_TQSECS
#define VKI_RT_KY_TQNSECS RT_KY_TQNSECS
#define VKI_RT_KY_TQSIG RT_KY_TQSIG
#define VKI_TS_KY_UPRILIM TS_KY_UPRILIM
#define VKI_TS_KY_UPRI TS_KY_UPRI
#define VKI_IA_KY_UPRILIM IA_KY_UPRILIM
#define VKI_IA_KY_UPRI IA_KY_UPRI
#define VKI_IA_KY_MODE IA_KY_MODE
#define VKI_FSS_KY_UPRILIM FSS_KY_UPRILIM
#define VKI_FSS_KY_UPRI FSS_KY_UPRI
#define VKI_FX_KY_UPRILIM FX_KY_UPRILIM
#define VKI_FX_KY_UPRI FX_KY_UPRI
#define VKI_FX_KY_TQSECS FX_KY_TQSECS
#define VKI_FX_KY_TQNSECS FX_KY_TQNSECS


#include <sys/priv.h>
#define vki_priv_impl_info_t priv_impl_info_t


#include <sys/proc.h>
#define VKI_SRUN SRUN
#define VKI_SSLEEP SSLEEP
#define VKI_SZOMB SZOMB


#include <sys/processor.h>
#define vki_processorid_t processorid_t


/* We want the new /proc definitions. */
#define _STRUCTURED_PROC 1
#include <sys/procfs.h>
#define VKI_MA_READ MA_READ
#define VKI_MA_WRITE MA_WRITE
#define VKI_MA_EXEC MA_EXEC
#define VKI_PRNODEV PRNODEV
#define VKI_PR_PCINVAL PR_PCINVAL
#define vki_lwpsinfo_t lwpsinfo_t
#define vki_lwpstatus_t lwpstatus_t
#define vki_prcred_t prcred_t
#define vki_prmap_t prmap_t
#define vki_prpriv_t prpriv_t
#define vki_prxmap_t prxmap_t
#define vki_pstatus_t pstatus_t
#define vki_psinfo_t psinfo_t


#include <sys/procfs_isa.h>
#if defined(SOLARIS_PRXREGSET_T)
#define vki_prxregset_t prxregset_t
#endif /* SOLARIS_PRXREGSET_T */


#include <sys/procset.h>
#define vki_idtype_t idtype_t
#define VKI_P_PID P_PID
#define VKI_P_PGID P_PGID
#define VKI_P_ALL P_ALL
#define VKI_POP_AND POP_AND
#define vki_procset_t procset_t


#include <sys/pset.h>
#define VKI_PSET_CREATE PSET_CREATE
#define VKI_PSET_DESTROY PSET_DESTROY
#define VKI_PSET_ASSIGN PSET_ASSIGN
#define VKI_PSET_INFO PSET_INFO
#define VKI_PSET_BIND PSET_BIND
#define VKI_PSET_GETLOADAVG PSET_GETLOADAVG
#define VKI_PSET_LIST PSET_LIST
#define VKI_PSET_SETATTR PSET_SETATTR
#define VKI_PSET_GETATTR PSET_GETATTR
#define VKI_PSET_ASSIGN_FORCED PSET_ASSIGN_FORCED
#define VKI_PSET_BIND_LWP PSET_BIND_LWP
#if defined(SOLARIS_PSET_GET_NAME)
#define VKI_PSET_GET_NAME PSET_GET_NAME
#endif /* SOLARIS_PSET_GET_NAME */
#define vki_psetid_t psetid_t


#include <sys/regset.h>
#define vki_prgregset_t prgregset_t


#include <sys/resource.h>
#define VKI_RLIMIT_DATA RLIMIT_DATA
#define VKI_RLIMIT_STACK RLIMIT_STACK
#define VKI_RLIMIT_CORE RLIMIT_CORE
#define VKI_RLIMIT_NOFILE RLIMIT_NOFILE
#define VKI__RUSAGESYS_GETRUSAGE _RUSAGESYS_GETRUSAGE
#define VKI__RUSAGESYS_GETRUSAGE_CHLD _RUSAGESYS_GETRUSAGE_CHLD
#define VKI__RUSAGESYS_GETRUSAGE_LWP _RUSAGESYS_GETRUSAGE_LWP
#define VKI__RUSAGESYS_GETVMUSAGE _RUSAGESYS_GETVMUSAGE
#define vki_rlimit rlimit
#define vki_rlimit64 rlimit64
#define vki_rusage rusage


#include <sys/schedctl.h>
#define vki_sc_shared sc_shared


#include <sys/segments.h>
#define VKI_GDT_LWPGS GDT_LWPGS
#if defined(VGP_amd64_solaris)
/* Values VKI_UCS_SEL/VKI_UDS_SEL can be used only on amd64. On x86, correct
   %cs/%ds values for a client need to be obtained from the host registers
   because they are different depending on the running kernel (x86 or amd64).
 */
#define VKI_UCS_SEL UCS_SEL
#define VKI_UDS_SEL UDS_SEL
#endif
#define VKI_LWPGS_SEL LWPGS_SEL


#include <sys/select.h>
#define vki_fd_set fd_set


#include <sys/priv.h>
/* Define _KMEMUSER so priv_set is pulled in. */
#define _KMEMUSER
#include <sys/priv_impl.h>
#undef _KMEMUSER
#define vki_priv_set_t priv_set_t
#define vki_priv_ptype_t priv_ptype_t
#define vki_priv_op_t priv_op_t

#define VKI_PRIVSYS_SETPPRIV PRIVSYS_SETPPRIV
#define VKI_PRIVSYS_GETPPRIV PRIVSYS_GETPPRIV
#define VKI_PRIVSYS_GETIMPLINFO PRIVSYS_GETIMPLINFO
#define VKI_PRIVSYS_SETPFLAGS PRIVSYS_SETPFLAGS
#define VKI_PRIVSYS_GETPFLAGS PRIVSYS_GETPFLAGS
#define VKI_PRIVSYS_ISSETUGID PRIVSYS_ISSETUGID
#define VKI_PRIVSYS_PFEXEC_REG PRIVSYS_PFEXEC_REG
#define VKI_PRIVSYS_PFEXEC_UNREG PRIVSYS_PFEXEC_UNREG

#define vki_priv_impl_info_t priv_impl_info_t


#include <sys/sem.h>
#include <sys/sem_impl.h>
#define VKI_GETALL GETALL
#define VKI_GETPID GETPID
#define VKI_GETNCNT GETNCNT
#define VKI_GETZCNT GETZCNT
#define VKI_GETVAL GETVAL
#define VKI_SEMCTL SEMCTL
#define VKI_SEMGET SEMGET
#define VKI_SEMIDS SEMIDS
#define VKI_SEMOP SEMOP
#define VKI_SEMTIMEDOP SEMTIMEDOP
#define VKI_SETALL SETALL
#define VKI_SETVAL SETVAL

#define vki_semid_ds semid_ds
#define vki_sembuf sembuf

/* The semun union has to be explicitly declared by the application program
   (see semctl(2)). */
union vki_semun {
   int val;
   struct semid_ds *buf;
   ushort_t *array;
};


#include <sys/sendfile.h>
#define VKI_SENDFILEV SENDFILEV
#define VKI_SENDFILEV64 SENDFILEV64
#define VKI_SFV_FD_SELF SFV_FD_SELF
#define vki_sendfilevec sendfilevec
#define vki_sendfilevec64 sendfilevec64


#include <sys/shm.h>
#include <sys/shm_impl.h>
#define VKI_SHMAT SHMAT
#define VKI_SHMCTL SHMCTL
#define VKI_SHMDT SHMDT
#define VKI_SHMGET SHMGET
#define VKI_SHMIDS SHMIDS
#if defined(SOLARIS_SHM_NEW)
#define VKI_SHMADV SHMADV
#define VKI_SHMGET_OSM SHMGET_OSM
#define VKI_SHM_ADV_GET SHM_ADV_GET
#define VKI_SHM_ADV_SET SHM_ADV_SET
#endif /* SOLARIS_SHM_NEW */
#define VKI_SHM_LOCK SHM_LOCK
#define VKI_SHM_RDONLY SHM_RDONLY
#define VKI_SHM_UNLOCK SHM_UNLOCK
/* Should be correct, but not really neat. */
#define VKI_SHMLBA VKI_PAGE_SIZE

#define vki_shmid_ds shmid_ds
#define vki_shmid_ds64 shmid_ds64
#define vki_shmid_xds64 shmid_xds64


#include <sys/siginfo.h>
/* This section also contains items defined in sys/machsig.h, this file
   is directly included in sys/siginfo.h. */
#define vki_sigevent sigevent
#define vki_siginfo_t siginfo_t

#define VKI_SI_LWP SI_LWP
#define VKI_SI_USER SI_USER
#define VKI_SIGEV_PORT SIGEV_PORT
#define VKI_SIGEV_THREAD SIGEV_THREAD

/* SIGTRAP signal codes */
#define VKI_TRAP_BRKPT TRAP_BRKPT

/* SIGCLD signal codes */
#define VKI_CLD_EXITED CLD_EXITED
#define VKI_CLD_KILLED CLD_KILLED
#define VKI_CLD_DUMPED CLD_DUMPED
#define VKI_CLD_TRAPPED CLD_TRAPPED
#define VKI_CLD_STOPPED CLD_STOPPED
#define VKI_CLD_CONTINUED CLD_CONTINUED

/* SIGILL signal codes */
#define VKI_ILL_ILLOPC ILL_ILLOPC
#define VKI_ILL_ILLOPN ILL_ILLOPN
#define VKI_ILL_ILLADR ILL_ILLADR
#define VKI_ILL_ILLTRP ILL_ILLTRP
#define VKI_ILL_PRVOPC ILL_PRVOPC
#define VKI_ILL_PRVREG ILL_PRVREG
#define VKI_ILL_COPROC ILL_COPROC
#define VKI_ILL_BADSTK ILL_BADSTK

/* SIGFPE signal codes */
#define VKI_FPE_INTDIV FPE_INTDIV
#define VKI_FPE_INTOVF FPE_INTOVF
#define VKI_FPE_FLTDIV FPE_FLTDIV
#define VKI_FPE_FLTOVF FPE_FLTOVF
#define VKI_FPE_FLTUND FPE_FLTUND
#define VKI_FPE_FLTRES FPE_FLTRES
#define VKI_FPE_FLTINV FPE_FLTINV
#define VKI_FPE_FLTSUB FPE_FLTSUB
#define VKI_FPE_FLTDEN FPE_FLTDEN

/* SIGSEV signal codes */
#define VKI_SEGV_MAPERR SEGV_MAPERR
#define VKI_SEGV_ACCERR SEGV_ACCERR

/* SIGBUS signal codes */
#define VKI_BUS_ADRALN BUS_ADRALN
#define VKI_BUS_ADRERR BUS_ADRERR
#define VKI_BUS_OBJERR BUS_OBJERR


#include <sys/signal.h>
/* This section also contains items defined in sys/iso/signal_iso.h, this file
   is directly included in sys/signal.h. */

/* Next three constants describe the internal representation of sigset_t,
   there are checks in coregrind/m_vki.c to make sure they are correct. */
#define _VKI_NSIG 128
#define _VKI_MAXSIG MAXSIG
#define _VKI_NSIG_BPW 32
#define _VKI_NSIG_WORDS (_VKI_NSIG / _VKI_NSIG_BPW)
#define vki_sigset_t sigset_t
#define vki_sigaltstack sigaltstack
/* sigset_t accessor */
#define sig __sigbits

/* On Solaris we use the same type for passing sigactions to
   and from the kernel. Hence: */
typedef struct sigaction vki_sigaction_toK_t;
typedef struct sigaction vki_sigaction_fromK_t;
/* sigaction_t accessor */
#define ksa_handler sa_handler

#define VKI_SA_ONSTACK SA_ONSTACK
#define VKI_SA_ONESHOT SA_RESETHAND
#define VKI_SA_NOMASK SA_NODEFER

#define VKI_MINSIGSTKSZ MINSIGSTKSZ

#define VKI_SS_ONSTACK SS_ONSTACK
#define VKI_SS_DISABLE SS_DISABLE

#define vki_stack_t stack_t

#define VKI_SA_NOCLDSTOP SA_NOCLDSTOP
#define VKI_SA_RESTART SA_RESTART
#define VKI_SA_SIGINFO SA_SIGINFO
#define VKI_SA_NOCLDWAIT SA_NOCLDWAIT
#define VKI_SA_RESTORER 0 /* Solaris doesn't have this */

#define VKI_SIGHUP SIGHUP               /*  1 */
#define VKI_SIGINT SIGINT               /*  2 */
#define VKI_SIGQUIT SIGQUIT             /*  3 */
#define VKI_SIGILL SIGILL               /*  4 */
#define VKI_SIGTRAP SIGTRAP             /*  5 */
#define VKI_SIGABRT SIGABRT             /*  6 */
#define VKI_SIGEMT SIGEMT               /*  7 */
#define VKI_SIGFPE SIGFPE               /*  8 */
#define VKI_SIGKILL SIGKILL             /*  9 */
#define VKI_SIGBUS SIGBUS               /* 10 */
#define VKI_SIGSEGV SIGSEGV             /* 11 */
#define VKI_SIGSYS SIGSYS               /* 12 */
#define VKI_SIGPIPE SIGPIPE             /* 13 */
#define VKI_SIGALRM SIGALRM             /* 14 */
#define VKI_SIGTERM SIGTERM             /* 15 */
#define VKI_SIGUSR1 SIGUSR1             /* 16 */
#define VKI_SIGUSR2 SIGUSR2             /* 17 */
#define VKI_SIGCHLD SIGCHLD             /* 18 */
#define VKI_SIGPWR SIGPWR               /* 19 */
#define VKI_SIGWINCH SIGWINCH           /* 20 */
#define VKI_SIGURG SIGURG               /* 21 */
#define VKI_SIGIO SIGIO                 /* 22 */
#define VKI_SIGSTOP SIGSTOP             /* 23 */
#define VKI_SIGTSTP SIGTSTP             /* 24 */
#define VKI_SIGCONT SIGCONT             /* 25 */
#define VKI_SIGTTIN SIGTTIN             /* 26 */
#define VKI_SIGTTOU SIGTTOU             /* 27 */
#define VKI_SIGVTALRM SIGVTALRM         /* 28 */
#define VKI_SIGPROF SIGPROF             /* 29 */
#define VKI_SIGXCPU SIGXCPU             /* 30 */
#define VKI_SIGXFSZ SIGXFSZ             /* 31 */
#define VKI_SIGWAITING SIGWAITING       /* 32 */
#define VKI_SIGLWP SIGLWP               /* 33 */
#define VKI_SIGFREEZE SIGFREEZE         /* 34 */
#define VKI_SIGTHAW SIGTHAW             /* 35 */
#define VKI_SIGCANCEL SIGCANCEL         /* 36 */
#define VKI_SIGLOST SIGLOST             /* 37 */
#define VKI_SIGXRES SIGXRES             /* 38 */
#define VKI_SIGJVM1 SIGJVM1             /* 39 */
#define VKI_SIGJVM2 SIGJVM2             /* 40 */
/* Note that SIGRTMIN and SIGRTMAX are actually macros calling into
   libc's sysconf() which in turn calls into kernel. And it returns
   these _SIGRTM* values. So we are safe until someone rebuilds Solaris
   kernel with different values... */
#define VKI_SIGRTMIN _SIGRTMIN          /* 41 */
#define VKI_SIGRTMAX _SIGRTMAX          /* 72 */

#define VKI_SIG_DFL SIG_DFL
#define VKI_SIG_IGN SIG_IGN

#define VKI_SIG_BLOCK SIG_BLOCK
#define VKI_SIG_UNBLOCK SIG_UNBLOCK
#define VKI_SIG_SETMASK SIG_SETMASK


#include <sys/socket.h>
#define vki_sa_family_t sa_family_t
#define vki_sockaddr sockaddr

#define vki_socklen_t socklen_t

#define VKI_SOCK_STREAM SOCK_STREAM

#define VKI_SO_TYPE SO_TYPE
#define VKI_SCM_RIGHTS SCM_RIGHTS
#define VKI_SOL_SOCKET SOL_SOCKET

#define VKI_AF_UNIX AF_UNIX
#define VKI_AF_INET AF_INET
#define VKI_AF_INET6 AF_INET6

#define vki_msghdr msghdr
#define vki_cmsghdr cmsghdr

#define VKI_CMSG_ALIGN(a) _CMSG_DATA_ALIGN(a)
#define VKI_CMSG_DATA(cmsg) CMSG_DATA(cmsg)
#define VKI_CMSG_FIRSTHDR(mhdr) CMSG_FIRSTHDR(mhdr)
#define VKI_CMSG_NXTHDR(mhdr, cmsg) CMSG_NXTHDR(mhdr, cmsg)


#include <sys/socketvar.h>
#define VKI_SOV_DEFAULT SOV_DEFAULT


#include <sys/sockio.h>
#define VKI_SIOCGIFCONF SIOCGIFCONF
#define VKI_SIOCGIFFLAGS SIOCGIFFLAGS
#define VKI_SIOCGIFNETMASK SIOCGIFNETMASK
#define VKI_SIOCGIFNUM SIOCGIFNUM
#define VKI_SIOCGLIFBRDADDR SIOCGLIFBRDADDR
#define VKI_SIOCGLIFCONF SIOCGLIFCONF
#define VKI_SIOCGLIFFLAGS SIOCGLIFFLAGS
#define VKI_SIOCGLIFNETMASK SIOCGLIFNETMASK
#define VKI_SIOCGLIFNUM SIOCGLIFNUM


#if defined(SOLARIS_SPAWN_SYSCALL)
#include <sys/spawn_impl.h>
#define VKI_FA_CHDIR FA_CHDIR
#define VKI_FA_CLOSE FA_CLOSE
#define VKI_FA_CLOSEFROM FA_CLOSEFROM
#define VKI_FA_DUP2 FA_DUP2
#define VKI_FA_OPEN FA_OPEN
#define VKI_POSIX_SPAWN_NOEXECERR_NP POSIX_SPAWN_NOEXECERR_NP
#define VKI_POSIX_SPAWN_NOSIGCHLD_NP POSIX_SPAWN_NOSIGCHLD_NP
#define VKI_POSIX_SPAWN_RESETIDS POSIX_SPAWN_RESETIDS
#define VKI_POSIX_SPAWN_SETPGROUP POSIX_SPAWN_SETPGROUP
#define VKI_POSIX_SPAWN_SETSCHEDPARAM POSIX_SPAWN_SETSCHEDPARAM
#define VKI_POSIX_SPAWN_SETSCHEDULER POSIX_SPAWN_SETSCHEDULER
#define VKI_POSIX_SPAWN_SETSID_NP POSIX_SPAWN_SETSID_NP
#define VKI_POSIX_SPAWN_SETSIGDEF POSIX_SPAWN_SETSIGDEF
#define VKI_POSIX_SPAWN_SETSIGIGN_NP POSIX_SPAWN_SETSIGIGN_NP
#define VKI_POSIX_SPAWN_SETSIGMASK POSIX_SPAWN_SETSIGMASK
#define VKI_POSIX_SPAWN_SETVAMASK_NP POSIX_SPAWN_SETVAMASK_NP
#define VKI_POSIX_SPAWN_WAITPID_NP POSIX_SPAWN_WAITPID_NP
#define VKI_SPAWN_VERSION SPAWN_VERSION
#define vki_kfile_attr_t kfile_attr_t
#define vki_kspawn_attr_t kspawn_attr_t
#define vki_spawn_attr_t spawn_attr_t
#endif /* SOLARIS_SPAWN_SYSCALL */


#include <sys/stat.h>
#define vki_stat stat
#define vki_stat64 stat64

#define st_atime_nsec st_atim.tv_nsec
#define st_mtime_nsec st_mtim.tv_nsec
#define st_ctime_nsec st_ctim.tv_nsec

#define VKI_S_IFIFO S_IFIFO
#define VKI_S_ISUID S_ISUID
#define VKI_S_ISGID S_ISGID

#define VKI_S_IRUSR S_IRUSR
#define VKI_S_IWUSR S_IWUSR
#define VKI_S_IXUSR S_IXUSR
#define VKI_S_IRGRP S_IRGRP
#define VKI_S_IWGRP S_IWGRP
#define VKI_S_IXGRP S_IXGRP
#define VKI_S_IROTH S_IROTH
#define VKI_S_IWOTH S_IWOTH
#define VKI_S_IXOTH S_IXOTH

#define VKI_S_ISCHR S_ISCHR
#define VKI_S_ISDIR S_ISDIR
#define VKI_S_ISBLK S_ISBLK
#define VKI_S_ISREG S_ISREG
#define VKI_S_ISLNK S_ISLNK


#include <sys/statfs.h>
#define vki_statfs statfs


#include <sys/statvfs.h>
#define vki_statvfs statvfs
#define vki_statvfs64 statvfs64


#include <sys/stropts.h>
#define VKI_I_CANPUT I_CANPUT
#define VKI_I_FIND I_FIND
#define VKI_I_FLUSH I_FLUSH
#define VKI_I_PEEK I_PEEK
#define VKI_I_PUSH I_PUSH
#define VKI_I_STR I_STR
#define vki_strbuf strbuf
#define vki_strioctl strioctl
#define vki_strpeek strpeek


#include <sys/synch.h>
#define vki_lwp_mutex_t lwp_mutex_t
#define vki_lwp_cond_t lwp_cond_t
#define vki_lwp_sema_t lwp_sema_t
#define vki_lwp_rwlock_t lwp_rwlock_t

/* Defines from the private sys/synch32.h header. */
#define vki_mutex_flag flags.flag1
#define vki_mutex_type flags.mbcp_type_un.mtype_rcount.count_type1
#define vki_mutex_rcount flags.mbcp_type_un.mtype_rcount.count_type2
#define vki_mutex_owner data
#define vki_mutex_lockw lock.lock64.pad[7]
#define vki_mutex_waiters lock.lock64.pad[6]
#define vki_mutex_ownerpid lock.lock32.ownerpid

#define vki_cond_type flags.type
#define vki_cond_waiters_kernel flags.flag[3]

#define vki_sema_count count
#define vki_sema_type type
#define vki_sema_waiters flags[7]

#define vki_rwlock_readers readers
#define vki_rwlock_type type
#define vki_rwlock_owner readercv.data
#define vki_rwlock_ownerpid writercv.data


#include <sys/sysconfig.h>
#define VKI_CONFIG_OPEN_FILES _CONFIG_OPEN_FILES


#include <sys/sysi86.h>
#define VKI_SI86FPSTART SI86FPSTART


#if defined(HAVE_SYS_SYSNVL_H)
#include <sys/sysnvl.h>
#define VKI_SYSNVL_OP_GET SYSNVL_OP_GET
#define VKI_SYSNVL_OP_UPDATE SYSNVL_OP_UPDATE
#endif /* HAVE_SYS_SYSNVL_H */


#include <sys/systeminfo.h>
#define VKI_SI_SYSNAME SI_SYSNAME
#define VKI_SI_HOSTNAME SI_HOSTNAME
#define VKI_SI_RELEASE SI_RELEASE
#define VKI_SI_VERSION SI_VERSION
#define VKI_SI_MACHINE SI_MACHINE
#define VKI_SI_ARCHITECTURE SI_ARCHITECTURE
#define VKI_SI_HW_SERIAL SI_HW_SERIAL
#define VKI_SI_HW_PROVIDER SI_HW_PROVIDER
#define VKI_SI_SRPC_DOMAIN SI_SRPC_DOMAIN

#define VKI_SI_SET_HOSTNAME SI_SET_HOSTNAME
#define VKI_SI_SET_SRCP_DOMAIN SI_SET_SRPC_DOMAIN

#define VKI_SI_PLATFORM SI_PLATFORM
#define VKI_SI_ISALIST SI_ISALIST
#define VKI_SI_DHCP_CACHE SI_DHCP_CACHE
#define VKI_SI_ARCHITECTURE_32 SI_ARCHITECTURE_32
#define VKI_SI_ARCHITECTURE_64 SI_ARCHITECTURE_64
#define VKI_SI_ARCHITECTURE_K SI_ARCHITECTURE_K
#define VKI_SI_ARCHITECTURE_NATIVE SI_ARCHITECTURE_NATIVE


#include <sys/termio.h>
#define vki_termio termio


#include <sys/termios.h>
#define vki_termios termios
#define VKI_TCGETA TCGETA
#define VKI_TCGETS TCGETS
#define VKI_TCSETS TCSETS
#define VKI_TCSETSF TCSETSF
#define VKI_TCSETSW TCSETSW
#define VKI_TIOCGPGRP TIOCGPGRP
#define VKI_TIOCGSID TIOCGSID
#define VKI_TIOCGWINSZ TIOCGWINSZ
#define VKI_TIOCNOTTY TIOCNOTTY
#define VKI_TIOCSCTTY TIOCSCTTY
#define VKI_TIOCSPGRP TIOCSPGRP
#define VKI_TIOCSWINSZ TIOCSWINSZ
#define vki_winsize winsize


#include <sys/time.h>
#define VKI_CLOCK_MONOTONIC CLOCK_MONOTONIC
#define VKI_CLOCK_THREAD_CPUTIME_ID CLOCK_THREAD_CPUTIME_ID

#define vki_clockid_t clockid_t
#define vki_timespec timespec
#define vki_timespec_t timespec_t
#define vki_timeval timeval
#define vki_timezone timezone
#define vki_itimerspec itimerspec
#define vki_itimerval itimerval


#include <sys/times.h>
#define vki_tms tms


#include <sys/tsol/label_macro.h>
#define vki_bslabel_t bslabel_t


/* Do not include sys/tsol/tndb.h if a C++ compiler is used to build a file
   which includes vki-solaris.h.  This cannot be done because the tndb.h
   header file uses the template keyword as a member name (on illumos). */
#if !defined(__cplusplus)
#include <sys/tsol/tndb.h>
#define VKI_TNDB_DELETE TNDB_DELETE
#define VKI_TNDB_FLUSH TNDB_FLUSH
#define VKI_TNDB_GET TNDB_GET
#if defined(SOLARIS_TNDB_GET_TNIP)
#define VKI_TNDB_GET_TNIP TNDB_GET_TNIP
#endif /* SOLARIS_TNDB_GET_TNIP */
#define VKI_TNDB_LOAD TNDB_LOAD
#define vki_tsol_mlpent_t tsol_mlpent_t
#define vki_tsol_rhent_t tsol_rhent_t
#define vki_tsol_tpent_t tsol_tpent_t
#endif /* !__cplusplus */


#include <sys/tsol/tsyscall.h>
#define VKI_TSOL_FGETLABEL TSOL_FGETLABEL
#define VKI_TSOL_GETLABEL TSOL_GETLABEL
#define VKI_TSOL_TNMLP TSOL_TNMLP
#define VKI_TSOL_TNRH TSOL_TNRH
#define VKI_TSOL_TNRHTP TSOL_TNRHTP
#define VKI_TSOL_SYSLABELING TSOL_SYSLABELING
#if defined(SOLARIS_TSOL_CLEARANCE)
#define VKI_TSOL_GETCLEARANCE TSOL_GETCLEARANCE
#define VKI_TSOL_SETCLEARANCE TSOL_SETCLEARANCE
#endif /* SOLARIS_TSOL_CLEARANCE */


#include <sys/ttold.h>
#define vki_sgttyb sgttyb


#include <sys/ucontext.h>
/* This section also contains items defined in sys/regset.h, this file
   is directly included in sys/ucontext.h. */
#if defined(VGP_x86_solaris)
#define VKI_SS SS
#define VKI_UESP UESP
#define VKI_EFL EFL
#define VKI_CS CS
#define VKI_EIP EIP
#define VKI_ERR 13 /* ERR */
#define VKI_TRAPNO TRAPNO
#define VKI_EAX EAX
#define VKI_ECX ECX
#define VKI_EDX EDX
#define VKI_EBX EBX
#define VKI_ESP ESP
#define VKI_EBP EBP
#define VKI_ESI ESI
#define VKI_EDI EDI
#define VKI_DS DS
#define VKI_ES ES
#define VKI_FS FS
#define VKI_GS GS

/* Definitions for compatibility with amd64-solaris. */
#define VKI_REG_ERR VKI_ERR
#define VKI_REG_TRAPNO VKI_TRAPNO

#define VKI_EFLAGS_ID_BIT (1 << 21)

#elif defined(VGP_amd64_solaris)
#define VKI_REG_GSBASE REG_GSBASE
#define VKI_REG_FSBASE REG_FSBASE
#define VKI_REG_DS REG_DS
#define VKI_REG_ES REG_ES
#define VKI_REG_GS REG_GS
#define VKI_REG_FS REG_FS
#define VKI_REG_SS REG_SS
#define VKI_REG_RSP REG_RSP
#define VKI_REG_RFL REG_RFL
#define VKI_REG_CS REG_CS
#define VKI_REG_RIP REG_RIP
#define VKI_REG_ERR REG_ERR
#define VKI_REG_TRAPNO REG_TRAPNO
#define VKI_REG_RAX REG_RAX
#define VKI_REG_RCX REG_RCX
#define VKI_REG_RDX REG_RDX
#define VKI_REG_RBX REG_RBX
#define VKI_REG_RBP REG_RBP
#define VKI_REG_RSI REG_RSI
#define VKI_REG_RDI REG_RDI
#define VKI_REG_R8 REG_R8
#define VKI_REG_R9 REG_R9
#define VKI_REG_R10 REG_R10
#define VKI_REG_R11 REG_R11
#define VKI_REG_R12 REG_R12
#define VKI_REG_R13 REG_R13
#define VKI_REG_R14 REG_R14
#define VKI_REG_R15 REG_R15

#define VKI_RFLAGS_ID_BIT (1 << 21)

#else
#error "Unknown platform"
#endif

#define vki_fpregset_t fpregset_t

/* Don't polute global namespace so much. */
#undef r_r0
#undef r_r1
#undef r_fp
#undef r_sp
#undef r_pc
#undef r_ps
#undef ERR

#if defined(VGP_x86_solaris)
/* The ucontext structure as defined in the SYSV ABI for Intel386. Illumos
   contains exactly this definition. Solaris 11 utilizes two uc_filler values
   -> "xrs_t uc_xrs; long uc_filler[3];". The xrs_t structure is used for the
   AVX support. We define our own ucontext structure because all five
   uc_filler values need to be available in VG_(save_context). Note that
   Valgrind doesn't support AVX on the x86 platform. */
typedef struct sysv_ucontext sysv_ucontext_t;
struct sysv_ucontext {
   unsigned long uc_flags;
   sysv_ucontext_t *uc_link;
   sigset_t uc_sigmask;
   stack_t uc_stack;
   mcontext_t uc_mcontext;
   long uc_filler[5];
};
#define VKI_UC_GUEST_CC_OP(uc) (*(UWord*)&(uc)->uc_filler[0])
#define VKI_UC_GUEST_CC_NDEP(uc) (*(UWord*)&(uc)->uc_filler[1])
#define VKI_UC_GUEST_CC_DEP1(uc) (*(UWord*)&(uc)->uc_filler[2])
#define VKI_UC_GUEST_CC_DEP2(uc) (*(UWord*)&(uc)->uc_filler[3])
#define VKI_UC_GUEST_EFLAGS_NEG(uc) \
   (*(UWord*)&(uc)->uc_mcontext.fpregs.fp_reg_set.fpchip_state.__pad[0])
#define VKI_UC_GUEST_EFLAGS_CHECKSUM(uc) \
   (*(UWord*)&(uc)->uc_mcontext.fpregs.fp_reg_set.fpchip_state.__pad[1])
#define VKI_UC_SIGNO(uc) (*(UWord*)&(uc)->uc_filler[4])
#define VKI_UC_SIGNO_CONST(uc) (*(const UWord*)&(uc)->uc_filler[4])

#define vki_ucontext_t sysv_ucontext_t
#define vki_ucontext sysv_ucontext

#elif defined(VGP_amd64_solaris)
/* The ucontext structure on Solaris has only 3 elements available in uc_filler
   which is not enough to store all required information. Therefore padding
   area in mcontext's FPU regset is used. */
#define vki_ucontext ucontext
#define vki_ucontext_t ucontext_t
#define VKI_UC_MC_FP_FX_IGN2(uc) \
    (uc)->uc_mcontext.fpregs.fp_reg_set.fpchip_state.__fx_ign2
#define VKI_UC_GUEST_CC_OP(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[0])
#define VKI_UC_GUEST_CC_NDEP(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[1])
#define VKI_UC_GUEST_CC_DEP1(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[2])
#define VKI_UC_GUEST_CC_DEP2(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[3])
#define VKI_UC_GUEST_RFLAGS_NEG(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[4])
#define VKI_UC_GUEST_RFLAGS_CHECKSUM(uc) \
   (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[5])
#define VKI_UC_SIGNO(uc) (*(UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[6])
#define VKI_UC_SIGNO_CONST(uc) (*(const UWord *) &VKI_UC_MC_FP_FX_IGN2(uc)[6])

#else
#error "Unknown platform"
#endif

#if defined(SOLARIS_FPCHIP_STATE_TAKES_UNDERSCORE)
#define vki_fpchip_state _fpchip_state
#else
#define vki_fpchip_state fpchip_state
#endif /* SOLARIS_FPCHIP_STATE_TAKES_UNDERSCORE */

#define VKI_GETCONTEXT GETCONTEXT
#define VKI_SETCONTEXT SETCONTEXT
#define VKI_GETUSTACK GETUSTACK
#define VKI_SETUSTACK SETUSTACK

#define VKI_UC_SIGMASK UC_SIGMASK
#define VKI_UC_STACK UC_STACK
#define VKI_UC_CPU UC_CPU
#define VKI_UC_FPU UC_FPU
#define VKI_UC_ALL UC_ALL

#include <sys/uio.h>
#define vki_iovec iovec


#include <sys/un.h>
#define vki_sockaddr_un sockaddr_un


#if defined(SOLARIS_UUIDSYS_SYSCALL)
#include <sys/uuid.h>
#define vki_uuid uuid
#endif /* SOLARIS_UUIDSYS_SYSCALL */


#include <sys/utsname.h>
#define vki_utsname utsname
/* Add another alias for utsname, used in syswrap-generic.c. */
#define vki_new_utsname utsname


#include <sys/vm_usage.h>
#define vki_vmusage_t vmusage_t


#include <sys/wait.h>
#define VKI_WEXITED WEXITED
#define VKI_WTRAPPED WTRAPPED

#define VKI_WSTOPFLG WSTOPFLG
#define VKI_WCONTFLG WCONTFLG
#define VKI_WCOREFLG WCOREFLG


#include <sys/zone.h>
#define VKI_ZONE_ADD_DATALINK ZONE_ADD_DATALINK
#define VKI_ZONE_ATTR_NAME ZONE_ATTR_NAME
#define VKI_ZONE_BOOT ZONE_BOOT
#define VKI_ZONE_CHECK_DATALINK ZONE_CHECK_DATALINK
#define VKI_ZONE_CREATE ZONE_CREATE
#define VKI_ZONE_DEL_DATALINK ZONE_DEL_DATALINK
#define VKI_ZONE_DESTROY ZONE_DESTROY
#define VKI_ZONE_ENTER ZONE_ENTER
#define VKI_ZONE_GETATTR ZONE_GETATTR
#define VKI_ZONE_LIST ZONE_LIST
#define VKI_ZONE_LIST_DATALINK ZONE_LIST_DATALINK
#define VKI_ZONE_LOOKUP ZONE_LOOKUP
#define VKI_ZONE_SETATTR ZONE_SETATTR
#define VKI_ZONE_SHUTDOWN ZONE_SHUTDOWN
#if defined(SOLARIS_ZONE_DEFUNCT)
#define VKI_ZONE_GETATTR_DEFUNCT ZONE_GETATTR_DEFUNCT
#define VKI_ZONE_LIST_DEFUNCT ZONE_LIST_DEFUNCT
#endif /* SOLARIS_ZONE_DEFUNCT */
#define VKI_ZONENAME_MAX ZONENAME_MAX
#define vki_zone_def zone_def
#define vki_zoneid_t zoneid_t


/* from <sys/ucred.h> which is consolidation private */
#define VKI_UCREDSYS_UCREDGET 0
#define VKI_UCREDSYS_GETPEERUCRED 1
struct ucred_s {
   vki_uint32_t uc_size;     /* Size of the full structure */
   vki_uint32_t uc_credoff;  /* Credential offset */
   vki_uint32_t uc_privoff;  /* Privilege offset */
   vki_pid_t    uc_pid;      /* Process id */
   vki_uint32_t uc_audoff;   /* Audit info offset */
   vki_zoneid_t uc_zoneid;   /* Zone id */
   vki_projid_t uc_projid;   /* Project id */
   vki_uint32_t uc_labeloff; /* label offset */
};


/* from sys/old_procfs.h which clashes with sys/procfs.h */

#define VKI_ELF_OLD_PR_PCINVAL 0x0080

typedef struct vki_elf_prpsinfo {
   char		pr_state;	/* numeric process state (see pr_sname) */
   char		pr_sname;	/* printable character representing pr_state */
   char		pr_zomb;	/* !=0: process terminated but not waited for */
   char		pr_nice;	/* nice for cpu usage */
   vki_uint_t	pr_flag;	/* process flags */
   vki_uid_t	pr_uid;		/* real user id */
   vki_gid_t	pr_gid;		/* real group id */
   vki_pid_t	pr_pid;		/* unique process id */
   vki_pid_t	pr_ppid;	/* process id of parent */
   vki_pid_t	pr_pgrp;	/* pid of process group leader */
   vki_pid_t	pr_sid;		/* session id */
   vki_caddr_t	pr_addr;	/* physical address of process */
   vki_size_t	pr_size;	/* size of process image in pages */
   vki_size_t	pr_rssize;	/* resident set size in pages */
   vki_caddr_t	pr_wchan;	/* wait addr for sleeping process */
   vki_timespec_t pr_start;	/* process start time, sec+nsec since epoch */
   vki_timespec_t pr_time;	/* usr+sys cpu time for this process */
   int		pr_pri;		/* priority, high value is high priority */
   char		pr_oldpri;	/* pre-SVR4, low value is high priority */
   char		pr_cpu;		/* pre-SVR4, cpu usage for scheduling */
   vki_o_dev_t	pr_ottydev;	/* short tty device number */
   vki_dev_t	pr_lttydev;	/* controlling tty device (PRNODEV if none) */
   char		pr_clname[8];	/* scheduling class name */
   char		pr_fname[16];	/* last component of execed pathname */
   char		pr_psargs[80];	/* initial characters of arg list */
   short	pr_syscall;	/* system call number (if in syscall) */
   short	pr_fill;
   vki_timespec_t pr_ctime;	/* usr+sys cpu time for reaped children */
   vki_size_t	pr_bysize;	/* size of process image in bytes */
   vki_size_t	pr_byrssize;	/* resident set size in bytes */
   int		pr_argc;	/* initial argument count */
   char		**pr_argv;	/* initial argument vector */
   char		**pr_envp;	/* initial environment vector */
   int		pr_wstat;	/* if zombie, the wait() status */
                	        /* The following percent numbers are 16-bit binary */
                	        /* fractions [0 .. 1] with the binary point to the */
                	        /* right of the high-order bit (one == 0x8000) */
   ushort_t 	pr_pctcpu;	/* % of recent cpu time, one or all lwps */
   ushort_t 	pr_pctmem;	/* % of of system memory used by the process */
   vki_uid_t	pr_euid;	/* effective user id */
   vki_gid_t	pr_egid;	/* effective group id */
   vki_id_t	pr_aslwpid;	/* historical; now always zero */
   char		pr_dmodel;	/* data model of the process */
   char		pr_pad[3];
   int		pr_filler[6];	/* for future expansion */
} vki_elf_prpsinfo_t;

typedef struct vki_elf_prstatus {
   int		pr_flags;	/* Flags (see below) */
   short	pr_why;		/* Reason for process stop (if stopped) */
   short	pr_what;	/* More detailed reason */
   vki_siginfo_t pr_info;	/* Info associated with signal or fault */
   short	pr_cursig;	/* Current signal */
   ushort_t 	pr_nlwp;	/* Number of lwps in the process */
   vki_sigset_t pr_sigpend;	/* Set of signals pending to the process */
   vki_sigset_t pr_sighold;	/* Set of signals held (blocked) by the lwp */
   struct vki_sigaltstack pr_altstack; /* Alternate signal stack info */
   struct sigaction pr_action;	/* Signal action for current signal */
   vki_pid_t	pr_pid;		/* Process id */
   vki_pid_t	pr_ppid;	/* Parent process id */
   vki_pid_t	pr_pgrp;	/* Process group id */
   vki_pid_t	pr_sid;		/* Session id */
   vki_timespec_t pr_utime;	/* Process user cpu time */
   vki_timespec_t pr_stime;	/* Process system cpu time */
   vki_timespec_t pr_cutime;	/* Sum of children's user times */
   vki_timespec_t pr_cstime;	/* Sum of children's system times */
   char	pr_clname[PRCLSZ]; 	/* Scheduling class name */
   short	pr_syscall;	/* System call number (if in syscall) */
   short	pr_nsysarg;	/* Number of arguments to this syscall */
   long	pr_sysarg[PRSYSARGS];	/* Arguments to this syscall */
   vki_id_t	pr_who;		/* Specific lwp identifier */
   vki_sigset_t pr_lwppend;	/* Set of signals pending to the lwp */
   struct vki_ucontext *pr_oldcontext; /* Address of previous ucontext */
   vki_caddr_t	pr_brkbase;	/* Address of the process heap */
   vki_size_t	pr_brksize;	/* Size of the process heap, in bytes */
   vki_caddr_t	pr_stkbase;	/* Address of the process stack */
   vki_size_t	pr_stksize;	/* Size of the process stack, in bytes */
   short	pr_processor;	/* processor which last ran this LWP */
   short	pr_bind;	/* processor LWP bound to or PBIND_NONE */
   long		pr_instr;	/* Current instruction */
   vki_prgregset_t pr_reg;	/* General registers */
} vki_elf_prstatus_t;


/* Signal frames. */
#if defined(VGP_x86_solaris)
struct vki_sigframe {
   /* First four words look like a call to a 3-arg x86 function. */
   void *return_addr;
   int a1_signo;
   vki_siginfo_t *a2_siginfo;
   vki_ucontext_t *a3_ucontext;
   /* Saved ucontext and siginfo. */
   vki_ucontext_t ucontext;
   vki_siginfo_t siginfo;
};

#elif defined(VGP_amd64_solaris)
struct vki_sigframe {
   void *return_addr;
   long a1_signo;
   vki_siginfo_t *a2_siginfo;
   /* Saved ucontext and siginfo. */
   vki_ucontext_t ucontext;
   vki_siginfo_t siginfo;
};

#else
#error "Unknown platform"
#endif
typedef struct vki_sigframe vki_sigframe_t;

#endif // __VKI_SOLARIS_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
