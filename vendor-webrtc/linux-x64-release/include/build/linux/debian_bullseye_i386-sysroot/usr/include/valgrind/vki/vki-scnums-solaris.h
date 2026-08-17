
/*--------------------------------------------------------------------*/
/*--- System call numbers for Solaris.        vki-scnums-solaris.h ---*/
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

/* Copyright 2013-2017, Ivo Raisr <ivosh@ivosh.net>. */

/* Copyright 2013, OmniTI Computer Consulting, Inc. All rights reserved. */

#ifndef __VKI_SCNUMS_SOLARIS_H
#define __VKI_SCNUMS_SOLARIS_H

/* Note: Basic information about Solaris syscalls can be found in the kernel
   source file uts/common/os/sysent.c.
 */

/* Include sys/syscall.h to get SYS_* constants (and sys/trap.h to get T_*) to
   avoid any copyright issues connected with their potential copying out of
   the header file.
 */
#include <sys/syscall.h>
#include <sys/trap.h>

/* normal syscall (int $0x91) */
#define VG_SOLARIS_SYSCALL_CLASS_CLASSIC        0
/* fasttrap syscall (int $0xD2) */
#define VG_SOLARIS_SYSCALL_CLASS_FASTTRAP       1

#define VG_SOLARIS_SYSCALL_CLASS_SHIFT 24
#define VG_SOLARIS_SYSCALL_NUMBER_MASK 0x00FFFFFF

#define VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(sysno) \
   ((VG_SOLARIS_SYSCALL_CLASS_FASTTRAP << VG_SOLARIS_SYSCALL_CLASS_SHIFT) \
    | (sysno))
#define VG_SOLARIS_SYSNO_CLASS(sysno) \
   ((sysno) >> VG_SOLARIS_SYSCALL_CLASS_SHIFT)
#define VG_SOLARIS_SYSNO_INDEX(sysno) \
   ((sysno) & VG_SOLARIS_SYSCALL_NUMBER_MASK)

#define __NR_exit                       SYS_exit
#if defined(SOLARIS_SPAWN_SYSCALL)
#define __NR_spawn                      SYS_spawn
#endif /* SOLARIS_SPAWN_SYSCALL */
#define __NR_read                       SYS_read
#define __NR_write                      SYS_write
#define __NR_close                      SYS_close
#define __NR_linkat                     SYS_linkat
#define __NR_symlinkat                  SYS_symlinkat
#define __NR_chdir                      SYS_chdir
#define __NR_time                       SYS_time
#define __NR_brk                        SYS_brk
#define __NR_lseek                      SYS_lseek
#define __NR_getpid                     SYS_getpid
#define __NR_mount                      SYS_mount
#define __NR_readlinkat                 SYS_readlinkat
#define __NR_setuid                     SYS_setuid
#define __NR_getuid                     SYS_getuid
#define __NR_stime                      SYS_stime
//#define __NR_pcsample                   SYS_pcsample
#define __NR_alarm                      SYS_alarm
#define __NR_pause                      SYS_pause
#if defined(SOLARIS_FREALPATHAT_SYSCALL)
#define __NR_frealpathat                SYS_frealpathat
#endif /* SOLARIS_FREALPATHAT_SYSCALL */
#define __NR_stty                       SYS_stty
#define __NR_gtty                       SYS_gtty
//#define __NR_nice                       SYS_nice
//#define __NR_statfs                     SYS_statfs
//#define __NR_sync                       SYS_sync
#define __NR_kill                       SYS_kill
//#define __NR_fstatfs                    SYS_fstatfs
#define __NR_pgrpsys                    SYS_pgrpsys
//#define __NR_uucopystr                  SYS_uucopystr
#define __NR_pipe                       SYS_pipe
#define __NR_times                      SYS_times
//#define __NR_profil                     SYS_profil
#define __NR_faccessat                  SYS_faccessat
#define __NR_setgid                     SYS_setgid
#define __NR_getgid                     SYS_getgid
#define __NR_mknodat                    SYS_mknodat
//#define __NR_msgsys                     SYS_msgsys
#define __NR_sysi86                     SYS_sysi86
//#define __NR_acct                       SYS_acct
#define __NR_shmsys                     SYS_shmsys
#define __NR_semsys                     SYS_semsys
#define __NR_ioctl                      SYS_ioctl
//#define __NR_uadmin                     SYS_uadmin
#define __NR_fchownat                   SYS_fchownat
//#define __NR_utssys                     SYS_utssys
#define __NR_fdsync                     SYS_fdsync
#define __NR_execve                     SYS_execve
#define __NR_umask                      SYS_umask
#define __NR_chroot                     SYS_chroot
#define __NR_fcntl                      SYS_fcntl
//#define __NR_ulimit                     SYS_ulimit
#define __NR_renameat                   SYS_renameat
#define __NR_unlinkat                   SYS_unlinkat
#define __NR_fstatat                    SYS_fstatat
#define __NR_fstatat64                  SYS_fstatat64
#define __NR_openat                     SYS_openat
#define __NR_openat64                   SYS_openat64
#define __NR_tasksys                    SYS_tasksys
//#define __NR_acctctl                    SYS_acctctl
//#define __NR_exacctsys                  SYS_exacctsys
#define __NR_getpagesizes               SYS_getpagesizes
//#define __NR_rctlsys                    SYS_rctlsys
//#define __NR_sidsys                     SYS_sidsys
#define __NR_lwp_park                   SYS_lwp_park
#define __NR_sendfilev                  SYS_sendfilev
#if defined(SOLARIS_LWP_NAME_SYSCALL)
#define __NR_lwp_name                   SYS_lwp_name
#endif /* SOLARIS_LWP_NAME_SYSCALL */
#define __NR_getdents                   SYS_getdents
#define __NR_privsys                    SYS_privsys
#define __NR_ucredsys                   SYS_ucredsys
#define __NR_sysfs                      SYS_sysfs
#define __NR_getmsg                     SYS_getmsg
#define __NR_putmsg                     SYS_putmsg
#define __NR_setgroups                  SYS_setgroups
#define __NR_getgroups                  SYS_getgroups
#define __NR_sigprocmask                SYS_sigprocmask
#define __NR_sigsuspend                 SYS_sigsuspend
#define __NR_sigaltstack                SYS_sigaltstack
#define __NR_sigaction                  SYS_sigaction
#define __NR_sigpending                 SYS_sigpending
#define __NR_context                    SYS_context
#define __NR_fchmodat                   SYS_fchmodat
#define __NR_mkdirat                    SYS_mkdirat
#define __NR_statvfs                    SYS_statvfs
#define __NR_fstatvfs                   SYS_fstatvfs
//#define __NR_getloadavg                 SYS_getloadavg
#define __NR_nfssys                     SYS_nfssys
#define __NR_waitid                     SYS_waitid
#define __NR_waitsys                    SYS_waitsys /* = SYS_waitid (historical) */
#define __NR_sigsendsys                 SYS_sigsendsys
//#define __NR_hrtsys                     SYS_hrtsys
#if defined(SOLARIS_UTIMESYS_SYSCALL)
#define __NR_utimesys                   SYS_utimesys
#endif /* SOLARIS_UTIMESYS_SYSCALL */
#if defined(SOLARIS_UTIMENSAT_SYSCALL)
#define __NR_utimensat                  SYS_utimensat
#endif /* SOLARIS_UTIMENSAT_SYSCALL */
#define __NR_sigresend                  SYS_sigresend
#define __NR_priocntlsys                SYS_priocntlsys
#define __NR_pathconf                   SYS_pathconf
//#define __NR_mincore                    SYS_mincore
#define __NR_mmap                       SYS_mmap
#define __NR_mprotect                   SYS_mprotect
#define __NR_munmap                     SYS_munmap
//#define __NR_fpathconf                  SYS_fpathconf
//#define __NR_vfork                      SYS_vfork
#define __NR_fchdir                     SYS_fchdir
#define __NR_readv                      SYS_readv
#define __NR_writev                     SYS_writev
#if defined(SOLARIS_UUIDSYS_SYSCALL)
#define __NR_uuidsys                    SYS_uuidsys
#endif /* SOLARIS_UUIDSYS_SYSCALL */
#define __NR_mmapobj                    SYS_mmapobj
#define __NR_setrlimit                  SYS_setrlimit
#define __NR_getrlimit                  SYS_getrlimit
#define __NR_memcntl                    SYS_memcntl
#define __NR_getpmsg                    SYS_getpmsg
#define __NR_putpmsg                    SYS_putpmsg
#define __NR_uname                      SYS_uname
#define __NR_setegid                    SYS_setegid
#define __NR_sysconfig                  SYS_sysconfig
//#define __NR_adjtime                    SYS_adjtime
#define __NR_systeminfo                 SYS_systeminfo
//#define __NR_sharefs                    SYS_sharefs
#define __NR_seteuid                    SYS_seteuid
#define __NR_forksys                    SYS_forksys
#if defined(SOLARIS_GETRANDOM_SYSCALL)
#define __NR_getrandom                  SYS_getrandom
#endif /* SOLARIS_GETRANDOM_SYSCALL */
#define __NR_sigtimedwait               SYS_sigtimedwait
//#define __NR_lwp_info                   SYS_lwp_info
#define __NR_yield                      SYS_yield
#define __NR_lwp_sema_post              SYS_lwp_sema_post
#define __NR_lwp_sema_trywait           SYS_lwp_sema_trywait
#define __NR_lwp_detach                 SYS_lwp_detach
//#define __NR_corectl                    SYS_corectl
#define __NR_modctl                     SYS_modctl
#define __NR_fchroot                    SYS_fchroot
#if defined(SOLARIS_SYSTEM_STATS_SYSCALL)
#define __NR_system_stats               SYS_system_stats
#endif /* SOLARIS_SYSTEM_STATS_SYSCALL */
//#define __NR_vhangup                    SYS_vhangup
#define __NR_gettimeofday               SYS_gettimeofday
#define __NR_getitimer                  SYS_getitimer
#define __NR_setitimer                  SYS_setitimer
#define __NR_lwp_create                 SYS_lwp_create
#define __NR_lwp_exit                   SYS_lwp_exit
#define __NR_lwp_suspend                SYS_lwp_suspend
#define __NR_lwp_continue               SYS_lwp_continue
#if defined(SOLARIS_LWP_SIGQUEUE_SYSCALL)
#define __NR_lwp_sigqueue               SYS_lwp_sigqueue
#else
#define __NR_lwp_kill                   SYS_lwp_kill
#endif /* SOLARIS_LWP_SIGQUEUE_SYSCALL */
#define __NR_lwp_self                   SYS_lwp_self
#define __NR_lwp_sigmask                SYS_lwp_sigmask
#define __NR_lwp_private                SYS_lwp_private
#define __NR_lwp_wait                   SYS_lwp_wait
#define __NR_lwp_mutex_wakeup           SYS_lwp_mutex_wakeup
#define __NR_lwp_cond_wait              SYS_lwp_cond_wait
#define __NR_lwp_cond_signal            SYS_lwp_cond_signal
#define __NR_lwp_cond_broadcast         SYS_lwp_cond_broadcast
#define __NR_pread                      SYS_pread
#define __NR_pwrite                     SYS_pwrite
#define __NR_llseek                     SYS_llseek
//#define __NR_inst_sync                  SYS_inst_sync
//#define __NR_brand                      SYS_brand
//#define __NR_kaio                       SYS_kaio
//#define __NR_cpc                        SYS_cpc
#define __NR_lgrpsys                    SYS_lgrpsys
#define __NR_rusagesys                  SYS_rusagesys
#define __NR_port                       SYS_port
#define __NR_pollsys                    SYS_pollsys
#define __NR_labelsys                   SYS_labelsys
#define __NR_acl                        SYS_acl
#define __NR_auditsys                   SYS_auditsys
//#define __NR_processor_bind             SYS_processor_bind
//#define __NR_processor_info             SYS_processor_info
#define __NR_p_online                   SYS_p_online
#define __NR_sigqueue                   SYS_sigqueue
#define __NR_clock_gettime              SYS_clock_gettime
#define __NR_clock_settime              SYS_clock_settime
#define __NR_clock_getres               SYS_clock_getres
#define __NR_timer_create               SYS_timer_create
#define __NR_timer_delete               SYS_timer_delete
#define __NR_timer_settime              SYS_timer_settime
#define __NR_timer_gettime              SYS_timer_gettime
#define __NR_timer_getoverrun           SYS_timer_getoverrun
#define __NR_nanosleep                  SYS_nanosleep
#define __NR_facl                       SYS_facl
#define __NR_door                       SYS_door
#define __NR_setreuid                   SYS_setreuid
#define __NR_setregid                   SYS_setregid
//#define __NR_install_utrap              SYS_install_utrap
//#define __NR_signotify                  SYS_signotify
#define __NR_schedctl                   SYS_schedctl
#define __NR_pset                       SYS_pset
//#define SYS_sparc_utrap_install
#define __NR_resolvepath                SYS_resolvepath
#define __NR_lwp_mutex_timedlock        SYS_lwp_mutex_timedlock
#define __NR_lwp_sema_timedwait         SYS_lwp_sema_timedwait
#define __NR_lwp_rwlock_sys             SYS_lwp_rwlock_sys
#define __NR_getdents64                 SYS_getdents64
#define __NR_mmap64                     SYS_mmap64
#define __NR_statvfs64                  SYS_statvfs64
#define __NR_fstatvfs64                 SYS_fstatvfs64
#define __NR_setrlimit64                SYS_setrlimit64
#define __NR_getrlimit64                SYS_getrlimit64
#define __NR_pread64                    SYS_pread64
#define __NR_pwrite64                   SYS_pwrite64
//#define __NR_rpcsys                     SYS_rpcsys
#define __NR_zone                       SYS_zone
//#define __NR_autofssys                  SYS_autofssys
#define __NR_getcwd                     SYS_getcwd
#define __NR_so_socket                  SYS_so_socket
#define __NR_so_socketpair              SYS_so_socketpair
#define __NR_bind                       SYS_bind
#define __NR_listen                     SYS_listen
#define __NR_accept                     SYS_accept
#define __NR_connect                    SYS_connect
#define __NR_shutdown                   SYS_shutdown
#define __NR_recv                       SYS_recv
#define __NR_recvfrom                   SYS_recvfrom
#define __NR_recvmsg                    SYS_recvmsg
#define __NR_send                       SYS_send
#define __NR_sendmsg                    SYS_sendmsg
#define __NR_sendto                     SYS_sendto
#define __NR_getpeername                SYS_getpeername
#define __NR_getsockname                SYS_getsockname
#define __NR_getsockopt                 SYS_getsockopt
#define __NR_setsockopt                 SYS_setsockopt
//#define __NR_sockconfig                 SYS_sockconfig
//#define __NR_ntp_gettime                SYS_ntp_gettime
//#define __NR_ntp_adjtime                SYS_ntp_adjtime
//#define __NR_lwp_mutex_unlock           SYS_lwp_mutex_unlock
//#define __NR_lwp_mutex_trylock          SYS_lwp_mutex_trylock
#define __NR_lwp_mutex_register         SYS_lwp_mutex_register
//#define __NR_cladm                      SYS_cladm
#define __NR_uucopy                     SYS_uucopy
#define __NR_umount2                    SYS_umount2

/* The following syscalls were removed in Solaris 11 (see
   https://wikis.oracle.com/display/DTrace/syscall+Provider). Valgrind's core
   cannot use these syscalls but wrappers have to be provided for them because
   they are still in use on illumos.
*/
#if defined(SOLARIS_OLD_SYSCALLS)
#define __NR_open                       SYS_open
#define __NR_link                       SYS_link
#define __NR_unlink                     SYS_unlink
#define __NR_mknod                      SYS_mknod
#define __NR_chmod                      SYS_chmod
#define __NR_chown                      SYS_chown
#define __NR_stat                       SYS_stat
#define __NR_fstat                      SYS_fstat
#define __NR_access                     SYS_access
#define __NR_rmdir                      SYS_rmdir
#define __NR_mkdir                      SYS_mkdir
#define __NR_lstat                      SYS_lstat
#define __NR_symlink                    SYS_symlink
#define __NR_readlink                   SYS_readlink
#define __NR_fchmod                     SYS_fchmod
#define __NR_fchown                     SYS_fchown
#define __NR_lchown                     SYS_lchown
#define __NR_rename                     SYS_rename
#define __NR_stat64                     SYS_stat64
#define __NR_lstat64                    SYS_lstat64
#define __NR_fstat64                    SYS_fstat64
#define __NR_open64                     SYS_open64
#endif /* SOLARIS_OLD_SYSCALLS */

/*
#define __NR_null \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_FNULL)
#define __NR_fgetfp \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_FGETFP)
#define __NR_fsetfp \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_FSETFP)
*/
#define __NR_gethrtime \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETHRTIME)
#define __NR_gethrvtime \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETHRVTIME)
#define __NR_gethrestime \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETHRESTIME)
#define __NR_getlgrp \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETLGRP)
#if defined(SOLARIS_GETHRT_FASTTRAP)
#define __NR_gethrt \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETHRT)
#endif /* SOLARIS_GETHRT_FASTTRAP */
#if defined(SOLARIS_GETZONEOFFSET_FASTTRAP)
#define __NR_getzoneoffset \
   VG_SOLARIS_SYSCALL_CONSTRUCT_FASTTRAP(T_GETZONEOFFSET)
#endif /* SOLARIS_GETZONEOFFSET_FASTTRAP */

#endif /* __VKI_SCNUMS_SOLARIS_H */

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
