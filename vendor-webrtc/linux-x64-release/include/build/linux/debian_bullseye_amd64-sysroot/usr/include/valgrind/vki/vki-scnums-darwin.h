
/*--------------------------------------------------------------------*/
/*--- System call numbers for Darwin.          vki-scnums-darwin.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2007-2017 Apple Inc.
      Greg Parker  gparker@apple.com

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

#ifndef __VKI_SCNUMS_DARWIN_H
#define __VKI_SCNUMS_DARWIN_H


// need DARWIN_10_x definitions
#include "config.h"

// osfmk/mach/i386/syscall_sw.h

// There are two syscall number encodings in Darwin.
//
// The 64-bit encoding is that the top 8-bits are the syscall class.  The low
// 24 are the syscall number (index) within that class.
//
// The 32-bit encoding is that the syscall number (index) is stored as-is and
// the syscall class is encoded as the argument to the 'int' instruction used
// to trigger the syscall:
// - 0x80: Unix
// - 0x81: Mach
// - 0x82: Machine-dependent
// - 0x83: Diagnostic
// Furthermore, just to make life interesting, for Mach traps the number is
// negative.
//
// Within Valgrind we only use the 64-bit encoding -- on 32-bit systems, we
// convert any syscall number to 64-bit encoding when we receive it, and
// convert back with VG_DARWIN_SYSNO_FOR_KERNEL when passing any syscall
// number back to the kernel (__NR_something shouldn't be passed directly to
// the kernel).
//
// Hack: x86 `int $0x80` (unix, 64-bit result) are special.
// [I haven't worked out why... --njn]

#define VG_DARWIN_SYSCALL_CLASS_SHIFT     24
#define VG_DARWIN_SYSCALL_CLASS_MASK      (0xFF << VG_DARWIN_SYSCALL_CLASS_SHIFT)
#define VG_DARWIN_SYSCALL_NUMBER_MASK     (~VG_DARWIN_SYSCALL_CLASS_MASK)

#define VG_DARWIN_SYSCALL_CLASS_NONE      0       /* Invalid */
#define VG_DARWIN_SYSCALL_CLASS_MACH      1       /* Mach */      
#define VG_DARWIN_SYSCALL_CLASS_UNIX      2       /* Unix/BSD */
#define VG_DARWIN_SYSCALL_CLASS_MDEP      3       /* Machine-dependent */
#define VG_DARWIN_SYSCALL_CLASS_DIAG      4       /* Diagnostics */

// Macros for encoding syscall numbers in the 64-bit encoding scheme.
#define VG_DARWIN_SYSCALL_CONSTRUCT_MACH(syscall_number) \
    ((VG_DARWIN_SYSCALL_CLASS_MACH << VG_DARWIN_SYSCALL_CLASS_SHIFT) | \
     (VG_DARWIN_SYSCALL_NUMBER_MASK & (syscall_number)))

#define VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(syscall_number) \
    ((VG_DARWIN_SYSCALL_CLASS_UNIX << VG_DARWIN_SYSCALL_CLASS_SHIFT) | \
     (VG_DARWIN_SYSCALL_NUMBER_MASK & (syscall_number)))

#define VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(syscall_number) \
    ((VG_DARWIN_SYSCALL_CLASS_MDEP << VG_DARWIN_SYSCALL_CLASS_SHIFT) | \
     (VG_DARWIN_SYSCALL_NUMBER_MASK & (syscall_number)))

#define VG_DARWIN_SYSCALL_CONSTRUCT_DIAG(syscall_number) \
    ((VG_DARWIN_SYSCALL_CLASS_DIAG << VG_DARWIN_SYSCALL_CLASS_SHIFT) | \
     (VG_DARWIN_SYSCALL_NUMBER_MASK & (syscall_number)))


/* Macros for decoding syscall numbers from the 64-bit encoding scheme. */
#define VG_DARWIN_SYSNO_INDEX(sysno) ((sysno) & VG_DARWIN_SYSCALL_NUMBER_MASK)
#define VG_DARWIN_SYSNO_CLASS(sysno) ((sysno) >> VG_DARWIN_SYSCALL_CLASS_SHIFT)


/* Macros for converting syscall numbers to the form expected by the kernel.*/
#if defined(VGA_x86)
   // This converts the 64-bit syscall number encoding, which we use
   // throughout Valgrind, into the 32-bit syscall number encoding, which is
   // suitable for passing to the (32-bit) kernel.
#  define VG_DARWIN_SYSNO_FOR_KERNEL(sysno) \
    ((VG_DARWIN_SYSNO_CLASS(sysno) == VG_DARWIN_SYSCALL_CLASS_MACH) \
    ? -VG_DARWIN_SYSNO_INDEX(sysno) \
    :  VG_DARWIN_SYSNO_INDEX(sysno) \
    )

#elif defined(VGA_amd64)
   // For 64-bit systems, we don't need to do anything to the syscall number.
#  define VG_DARWIN_SYSNO_FOR_KERNEL(sysno) (sysno)

#else
#  error Unknown architecture
#endif


// mdep syscalls

#if defined(VGA_x86)

// osfmk/i386/machdep_call.c
// DDD: the last two are BSD_CALL instead of CALL...
//#define __NR_thread_get_cthread_self      VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(0)
//#define __NR_thread_set_cthread_self      VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(1)
// 2 is invalid
#define __NR_thread_fast_set_cthread_self VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(3)
//#define __NR_thread_set_user_ldt          VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(4)
//#define __NR_i386_set_ldt                 VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(5)
//#define __NR_i386_get_ldt                 VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(6)

#elif defined(VGA_amd64)

// osfmk/i386/machdep_call.c
// 0, 1, 2 are invalid
#define __NR_thread_fast_set_cthread_self VG_DARWIN_SYSCALL_CONSTRUCT_MDEP(3)
// 4, 5, 6 are invalid

#else
#  error unknown architecture
#endif


// osfmk/mach/syscall_sw.h

#define __NR_kernelrpc_mach_vm_allocate_trap         VG_DARWIN_SYSCALL_CONSTRUCT_MACH(10)

#define __NR_kernelrpc_mach_vm_deallocate_trap       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(12)

#define __NR_kernelrpc_mach_vm_protect_trap          VG_DARWIN_SYSCALL_CONSTRUCT_MACH(14)
#define __NR_kernelrpc_mach_vm_map_trap              VG_DARWIN_SYSCALL_CONSTRUCT_MACH(15)
#define __NR_kernelrpc_mach_port_allocate_trap       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(16)
#define __NR_kernelrpc_mach_port_destroy_trap        VG_DARWIN_SYSCALL_CONSTRUCT_MACH(17)
#define __NR_kernelrpc_mach_port_deallocate_trap     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(18)
#define __NR_kernelrpc_mach_port_mod_refs_trap       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(19)
#define __NR_kernelrpc_mach_port_move_member_trap    VG_DARWIN_SYSCALL_CONSTRUCT_MACH(20)
#define __NR_kernelrpc_mach_port_insert_right_trap   VG_DARWIN_SYSCALL_CONSTRUCT_MACH(21)
#define __NR_kernelrpc_mach_port_insert_member_trap  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(22)
#define __NR_kernelrpc_mach_port_extract_member_trap VG_DARWIN_SYSCALL_CONSTRUCT_MACH(23)
#define __NR_kernelrpc_mach_port_construct_trap      VG_DARWIN_SYSCALL_CONSTRUCT_MACH(24)
#define __NR_kernelrpc_mach_port_destruct_trap       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(25)


#define __NR_mach_reply_port                  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(26)
#define __NR_thread_self_trap                 VG_DARWIN_SYSCALL_CONSTRUCT_MACH(27)
#define __NR_task_self_trap                   VG_DARWIN_SYSCALL_CONSTRUCT_MACH(28)
#define __NR_host_self_trap                   VG_DARWIN_SYSCALL_CONSTRUCT_MACH(29)

#define __NR_mach_msg_trap                    VG_DARWIN_SYSCALL_CONSTRUCT_MACH(31)
#define __NR_mach_msg_overwrite_trap          VG_DARWIN_SYSCALL_CONSTRUCT_MACH(32)
#define __NR_semaphore_signal_trap            VG_DARWIN_SYSCALL_CONSTRUCT_MACH(33)
#define __NR_semaphore_signal_all_trap        VG_DARWIN_SYSCALL_CONSTRUCT_MACH(34)
#define __NR_semaphore_signal_thread_trap     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(35)
#define __NR_semaphore_wait_trap              VG_DARWIN_SYSCALL_CONSTRUCT_MACH(36)
#define __NR_semaphore_wait_signal_trap       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(37)
#define __NR_semaphore_timedwait_trap         VG_DARWIN_SYSCALL_CONSTRUCT_MACH(38)
#define __NR_semaphore_timedwait_signal_trap  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(39)

#if DARWIN_VERS >= DARWIN_10_9
#define __NR_kernelrpc_mach_port_guard_trap   VG_DARWIN_SYSCALL_CONSTRUCT_MACH(41)
#define __NR_kernelrpc_mach_port_unguard_trap VG_DARWIN_SYSCALL_CONSTRUCT_MACH(42)
#endif

#if DARWIN_VERS >= DARWIN_10_12
#define __NR_mach_generate_activity_id        VG_DARWIN_SYSCALL_CONSTRUCT_MACH(43)
#elif defined(VGA_x86) || DARWIN_VERS == DARWIN_10_9
#define __NR_map_fd                           VG_DARWIN_SYSCALL_CONSTRUCT_MACH(43)
#endif

#define __NR_task_name_for_pid                VG_DARWIN_SYSCALL_CONSTRUCT_MACH(44)
#define __NR_task_for_pid                     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(45)
#define __NR_pid_for_task                     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(46)

#if defined(VGA_x86)
#define __NR_macx_swapon                      VG_DARWIN_SYSCALL_CONSTRUCT_MACH(48)
#define __NR_macx_swapoff                     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(49)
#endif

#if DARWIN_VERS >= DARWIN_10_13
#define __NR_thread_get_special_reply_port    VG_DARWIN_SYSCALL_CONSTRUCT_MACH(50)
#endif /* DARWIN_VERS >= DARWIN_10_13 */

#if defined(VGA_x86)
#define __NR_macx_triggers                    VG_DARWIN_SYSCALL_CONSTRUCT_MACH(51)
#define __NR_macx_backing_store_suspend       VG_DARWIN_SYSCALL_CONSTRUCT_MACH(52)
#define __NR_macx_backing_store_recovery      VG_DARWIN_SYSCALL_CONSTRUCT_MACH(53)
#endif

#define __NR_swtch_pri                        VG_DARWIN_SYSCALL_CONSTRUCT_MACH(59)
#define __NR_swtch                            VG_DARWIN_SYSCALL_CONSTRUCT_MACH(60)
#define __NR_sched_yield  __NR_swtch  /* linux-alike name */
#define __NR_syscall_thread_switch            VG_DARWIN_SYSCALL_CONSTRUCT_MACH(61)
#define __NR_clock_sleep_trap                 VG_DARWIN_SYSCALL_CONSTRUCT_MACH(62)

#if DARWIN_VERS >= DARWIN_10_12
#define __NR_host_create_mach_voucher_trap    VG_DARWIN_SYSCALL_CONSTRUCT_MACH(70)
#endif

#define __NR_mach_timebase_info               VG_DARWIN_SYSCALL_CONSTRUCT_MACH(89)
#define __NR_mach_wait_until                  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(90)
#define __NR_mk_timer_create                  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(91)
#define __NR_mk_timer_destroy                 VG_DARWIN_SYSCALL_CONSTRUCT_MACH(92)
#define __NR_mk_timer_arm                     VG_DARWIN_SYSCALL_CONSTRUCT_MACH(93)
#define __NR_mk_timer_cancel                  VG_DARWIN_SYSCALL_CONSTRUCT_MACH(94)

#define __NR_iokit_user_client_trap           VG_DARWIN_SYSCALL_CONSTRUCT_MACH(100)


// bsd/sys/syscall.h
 
#define	__NR_syscall        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(0)
#define	__NR_exit           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(1)
#define	__NR_fork           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(2) // was UX64
#define	__NR_read           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(3)
#define	__NR_write          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(4)
#define	__NR_open           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(5)
#define	__NR_close          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(6)
#define	__NR_wait4          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(7)
			/* 8  old creat */
#define	__NR_link           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(9)
#define	__NR_unlink         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(10)
			/* 11  old execv */
#define	__NR_chdir          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(12)
#define	__NR_fchdir         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(13)
#define	__NR_mknod          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(14)
#define	__NR_chmod          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(15)
#define	__NR_chown          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(16)
			/* 17  old break */
#define	__NR_getfsstat      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(18)
			/* 19  old lseek */
#define	__NR_getpid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(20)
			/* 21  old mount */
			/* 22  old umount */
#define	__NR_setuid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(23)
#define	__NR_getuid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(24)
#define	__NR_geteuid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(25)
#define	__NR_ptrace         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(26)
#define	__NR_recvmsg        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(27)
#define	__NR_sendmsg        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(28)
#define	__NR_recvfrom       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(29)
#define	__NR_accept         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(30)
#define	__NR_getpeername    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(31)
#define	__NR_getsockname    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(32)
#define	__NR_access         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(33)
#define	__NR_chflags        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(34)
#define	__NR_fchflags       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(35)
#define	__NR_sync           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(36)
#define	__NR_kill           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(37)
			/* 38  old stat */
#define	__NR_getppid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(39)
			/* 40  old lstat */
#define	__NR_dup            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(41)
#define	__NR_pipe           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(42) // was UX64
#define	__NR_getegid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(43)
#define	__NR_profil         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(44)
			/* 45  old ktrace */
#define	__NR_sigaction      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(46)
#define	__NR_getgid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(47)
#define	__NR_sigprocmask    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(48)
#define	__NR_getlogin       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(49)
#define	__NR_setlogin       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(50)
#define	__NR_acct           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(51)
#define	__NR_sigpending     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(52)
#define	__NR_sigaltstack    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(53)
#define	__NR_ioctl          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(54)
#define	__NR_reboot         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(55)
#define	__NR_revoke         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(56)
#define	__NR_symlink        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(57)
#define	__NR_readlink       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(58)
#define	__NR_execve         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(59)
#define	__NR_umask          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(60)
#define	__NR_chroot         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(61)
			/* 62  old fstat */
			/* 63  used internally , reserved */
			/* 64  old getpagesize */
#define	__NR_msync          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(65)
#define	__NR_vfork          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(66)
			/* 67  old vread */
			/* 68  old vwrite */
			/* 69  old sbrk */
			/* 70  old sstk */
			/* 71  old mmap */
			/* 72  old vadvise */
#define	__NR_munmap         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(73)
#define	__NR_mprotect       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(74)
#define	__NR_madvise        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(75)
			/* 76  old vhangup */
			/* 77  old vlimit */
#define	__NR_mincore        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(78)
#define	__NR_getgroups      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(79)
#define	__NR_setgroups      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(80)
#define	__NR_getpgrp        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(81)
#define	__NR_setpgid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(82)
#define	__NR_setitimer      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(83)
			/* 84  old wait */
#define	__NR_swapon         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(85)
#define	__NR_getitimer      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(86)
			/* 87  old gethostname */
			/* 88  old sethostname */
#define	__NR_getdtablesize  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(89)
#define	__NR_dup2           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(90)
			/* 91  old getdopt */
#define	__NR_fcntl          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(92)
#define	__NR_select         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(93)
			/* 94  old setdopt */
#define	__NR_fsync          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(95)
#define	__NR_setpriority    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(96)
#define	__NR_socket         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(97)
#define	__NR_connect        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(98)
			/* 99  old accept */
#define	__NR_getpriority    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(100)
			/* 101  old send */
			/* 102  old recv */
			/* 103  old sigreturn */
#define	__NR_bind           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(104)
#define	__NR_setsockopt     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(105)
#define	__NR_listen         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(106)
			/* 107  old vtimes */
			/* 108  old sigvec */
			/* 109  old sigblock */
			/* 110  old sigsetmask */
#define	__NR_sigsuspend     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(111)
			/* 112  old sigstack */
			/* 113  old recvmsg */
			/* 114  old sendmsg */
			/* 115  old vtrace */
#define	__NR_gettimeofday   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(116)
#define	__NR_getrusage      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(117)
#define	__NR_getsockopt     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(118)
			/* 119  old resuba */
#define	__NR_readv          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(120)
#define	__NR_writev         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(121)
#define	__NR_settimeofday   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(122)
#define	__NR_fchown         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(123)
#define	__NR_fchmod         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(124)
			/* 125  old recvfrom */
#define	__NR_setreuid       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(126)
#define	__NR_setregid       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(127)
#define	__NR_rename         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(128)
			/* 129  old truncate */
			/* 130  old ftruncate */
#define	__NR_flock          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(131)
#define	__NR_mkfifo         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(132)
#define	__NR_sendto         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(133)
#define	__NR_shutdown       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(134)
#define	__NR_socketpair     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(135)
#define	__NR_mkdir          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(136)
#define	__NR_rmdir          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(137)
#define	__NR_utimes         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(138)
#define	__NR_futimes        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(139)
#define	__NR_adjtime        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(140)
			/* 141  old getpeername */
#define __NR_gethostuuid    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(142)
			/* 143  old sethostid */
			/* 144  old getrlimit */
			/* 145  old setrlimit */
			/* 146  old killpg */
#define	__NR_setsid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(147)
			/* 148  old setquota */
			/* 149  old qquota */
			/* 150  old getsockname */
#define	__NR_getpgid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(151)
#define	__NR_setprivexec    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(152)
#define	__NR_pread          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(153)
#define	__NR_pwrite         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(154)
#define __NR_nfssvc         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(155)
			/* 156  old getdirentries */
#define	__NR_statfs         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(157)
#define	__NR_fstatfs        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(158)
#define	__NR_unmount        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(159)
			/* 160  old async_daemon */
#define __NR_getfh          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(161)
			/* 162  old getdomainname */
			/* 163  old setdomainname */
			/* 164  */
#define	__NR_quotactl       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(165)
			/* 166  old exportfs */
#define	__NR_mount          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(167)
			/* 168  old ustat */
#define __NR_csops          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(169)
			/* 170  old table */
			/* 171  old wait3 */
			/* 172  old rpause */
#define	__NR_waitid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(173)
			/* 174  old getdents */
			/* 175  old gc_control */
#define	__NR_add_profil     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(176)

#if DARWIN_VERS >= DARWIN_10_12
#define __NR_kdebug_typefilter VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(177)
#endif /* DARWIN_VERS >= DARWIN_10_12 */
#if DARWIN_VERS >= DARWIN_10_11
#define __NR_kdebug_trace_string VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(178)
#endif /* DARWIN_VERS >= DARWIN_10_11 */
			/* 179  */
#define	__NR_kdebug_trace   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(180)
#define	__NR_setgid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(181)
#define	__NR_setegid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(182)
#define	__NR_seteuid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(183)
#define __NR_sigreturn      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(184)
#define __NR_chud           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(185)
#if DARWIN_VERS >= DARWIN_10_13
#define	__NR_thread_selfcounts      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(186)
#else
			/* 186  */
#endif /* DARWIN_VERS >= DARWIN_10_13 */
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_fdatasync      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(187)
#else
			/* 187  */
#endif
#define	__NR_stat           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(188)
#define	__NR_fstat          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(189)
#define	__NR_lstat          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(190)
#define	__NR_pathconf       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(191)
#define	__NR_fpathconf      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(192)
			/* 193 */
#define	__NR_getrlimit      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(194)
#define	__NR_setrlimit      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(195)
#define	__NR_getdirentries  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(196)
#define	__NR_mmap           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(197)
			/* 198  __syscall */
#define	__NR_lseek          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(199) // was UX64
#define	__NR_truncate       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(200)
#define	__NR_ftruncate      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(201)
#define	__NR___sysctl       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(202)
#define	__NR_mlock          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(203)
#define	__NR_munlock        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(204)
#define	__NR_undelete       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(205)
#define	__NR_ATsocket       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(206)
#define	__NR_ATgetmsg       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(207)
#define	__NR_ATputmsg       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(208)
#define	__NR_ATPsndreq      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(209)
#define	__NR_ATPsndrsp      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(210)
#define	__NR_ATPgetreq      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(211)
#define	__NR_ATPgetrsp      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(212)
			/* 213  Reserved for AppleTalk */
#if DARWIN_VERS >= DARWIN_10_6
                        /* 214  old kqueue_from_portset_np*/
                        /* 215  old kqueue_portset_np*/
#else
#define __NR_kqueue_from_portset_np VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(214)
#define __NR_kqueue_portset_np VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(215)
#endif
#define	__NR_mkcomplex      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(216)
#define	__NR_statv          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(217)
#define	__NR_lstatv         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(218)
#define	__NR_fstatv         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(219)
#define	__NR_getattrlist    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(220)
#define	__NR_setattrlist    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(221)
#define	__NR_getdirentriesattr VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(222)
#define	__NR_exchangedata   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(223)
			/* 224  old checkuseraccess */
#define	__NR_searchfs       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(225)
#define	__NR_delete         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(226)
#define	__NR_copyfile       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(227)
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_fgetattrlist   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(228)
#define __NR_fsetattrlist   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(229)
#else
			/* 228  */
			/* 229  */
#endif
#define	__NR_poll           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(230)
#define	__NR_watchevent     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(231)
#define	__NR_waitevent      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(232)
#define	__NR_modwatch       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(233)
#define	__NR_getxattr       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(234)
#define	__NR_fgetxattr      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(235)
#define	__NR_setxattr       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(236)
#define	__NR_fsetxattr      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(237)
#define	__NR_removexattr    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(238)
#define	__NR_fremovexattr   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(239)
#define	__NR_listxattr      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(240)
#define	__NR_flistxattr     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(241)
#define	__NR_fsctl          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(242)
#define	__NR_initgroups     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(243)
#define __NR_posix_spawn    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(244)
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_ffsctl         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(245)
#else
			/* 245  */
#endif
			/* 246  */
#define __NR_nfsclnt        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(247)
#define __NR_fhopen         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(248)
			/* 249  */
#define	__NR_minherit       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(250)
#define	__NR_semsys         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(251)
#define	__NR_msgsys         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(252)
#define	__NR_shmsys         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(253)
#define	__NR_semctl         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(254)
#define	__NR_semget         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(255)
#define	__NR_semop          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(256)
			/* 257  */
#define	__NR_msgctl         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(258)
#define	__NR_msgget         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(259)
#define	__NR_msgsnd         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(260)
#define	__NR_msgrcv         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(261)
#define	__NR_shmat          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(262)
#define	__NR_shmctl         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(263)
#define	__NR_shmdt          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(264)
#define	__NR_shmget         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(265)
#define	__NR_shm_open       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(266)
#define	__NR_shm_unlink     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(267)
#define	__NR_sem_open       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(268)
#define	__NR_sem_close      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(269)
#define	__NR_sem_unlink     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(270)
#define	__NR_sem_wait       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(271)
#define	__NR_sem_trywait    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(272)
#define	__NR_sem_post       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(273)

#if DARWIN_VERS < DARWIN_10_10
#define	__NR_sem_getvalue   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(274)
#elif DARWIN_VERS >= DARWIN_10_10
#define	__NR_sysctlbyname   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(274)
#endif

#define	__NR_sem_init       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(275)
#define	__NR_sem_destroy    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(276)
#define	__NR_open_extended  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(277)
#define	__NR_umask_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(278)
#define	__NR_stat_extended  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(279)
#define	__NR_lstat_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(280)
#define	__NR_fstat_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(281)
#define	__NR_chmod_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(282)
#define	__NR_fchmod_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(283)
#define	__NR_access_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(284)
#define	__NR_settid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(285)
#define	__NR_gettid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(286)
#define	__NR_setsgroups     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(287)
#define	__NR_getsgroups     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(288)
#define	__NR_setwgroups     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(289)
#define	__NR_getwgroups     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(290)
#define	__NR_mkfifo_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(291)
#define	__NR_mkdir_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(292)
#define	__NR_identitysvc    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(293)
#define	__NR_shared_region_check_np VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(294)
#define	__NR_shared_region_map_np   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(295)
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_vm_pressure_monitor    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(296)
#define __NR_psynch_rw_longrdlock   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(297)
#define __NR_psynch_rw_yieldwrlock  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(298)
#define __NR_psynch_rw_downgrade    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(299)
#define __NR_psynch_rw_upgrade      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(300)
#define __NR_psynch_mutexwait       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(301)
#define __NR_psynch_mutexdrop       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(302)
#define __NR_psynch_cvbroad         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(303)
#define __NR_psynch_cvsignal        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(304)
#define __NR_psynch_cvwait          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(305)
#define __NR_psynch_rw_rdlock       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(306)
#define __NR_psynch_rw_wrlock       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(307)
#define __NR_psynch_rw_unlock       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(308)
#define __NR_psynch_rw_unlock2      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(309)
#else
			/* 296  old load_shared_file */
			/* 297  old reset_shared_file */
			/* 298  old new_system_shared_regions */
			/* 299  old shared_region_map_file_np */
			/* 300  old shared_region_make_private_np */
#define __NR___pthread_mutex_destroy  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(301)
#define __NR___pthread_mutex_init     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(302)
#define __NR___pthread_mutex_lock     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(303)
#define __NR___pthread_mutex_trylock  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(304)
#define __NR___pthread_mutex_unlock   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(305)
#define __NR___pthread_cond_init      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(306)
#define __NR___pthread_cond_destroy   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(307)
#define __NR___pthread_cond_broadcast VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(308)
#define __NR___pthread_cond_signal    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(309)
#endif
#define	__NR_getsid         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(310)
#define	__NR_settid_with_pid VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(311)
#if DARWIN_VERS >= DARWIN_10_7
#define __NR_psynch_cvclrprepost VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(312)
#else
#define __NR___pthread_cond_timedwait VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(312)
#endif
#define	__NR_aio_fsync      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(313)
#define	__NR_aio_return     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(314)
#define	__NR_aio_suspend    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(315)
#define	__NR_aio_cancel     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(316)
#define	__NR_aio_error      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(317)
#define	__NR_aio_read       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(318)
#define	__NR_aio_write      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(319)
#define	__NR_lio_listio     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(320)
			/* 321 */
#define __NR_iopolicysys    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(322)
#define __NR_process_policy VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(323)
#define	__NR_mlockall       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(324)
#define	__NR_munlockall     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(325)
			/* 326  */
#define	__NR_issetugid      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(327)
#define	__NR___pthread_kill VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(328)
#define	__NR___pthread_sigmask VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(329)
#define	__NR___sigwait        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(330)
#define	__NR___disable_threadsignal VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(331)
#define	__NR___pthread_markcancel VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(332)
#define	__NR___pthread_canceled VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(333)
#define	__NR___semwait_signal VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(334)
			/* 335  old utrace */
#define __NR_proc_info      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(336)
#define __NR_sendfile       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(337)
#define __NR_stat64         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(338)
#define __NR_fstat64        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(339)
#define __NR_lstat64        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(340)
#define __NR_stat64_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(341)
#define __NR_lstat64_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(342)
#define __NR_fstat64_extended VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(343)
#define __NR_getdirentries64 VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(344)
#define __NR_statfs64       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(345)
#define __NR_fstatfs64      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(346)
#define __NR_getfsstat64    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(347)
#define __NR___pthread_chdir VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(348)
#define __NR___pthread_fchdir VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(349)

#define	__NR_audit          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(350)
#define	__NR_auditon        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(351)
			/* 352  */
#define	__NR_getauid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(353)
#define	__NR_setauid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(354)
#define	__NR_getaudit       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(355)
#define	__NR_setaudit       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(356)
#define	__NR_getaudit_addr  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(357)
#define	__NR_setaudit_addr  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(358)
#define	__NR_auditctl       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(359)
#define	__NR_bsdthread_create VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(360)
#define	__NR_bsdthread_terminate VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(361)
#define	__NR_kqueue         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(362)
#define	__NR_kevent         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(363)
#define	__NR_lchown         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(364)
#define __NR_stack_snapshot VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(365)
#define __NR_bsdthread_register VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(366)
#define __NR_workq_open     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(367)
#define __NR_workq_ops      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(368)
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_kevent64       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(369)
#else
			/* 369  */
#endif
			/* 370  */
			/* 371  */
#if DARWIN_VERS >= DARWIN_10_6
#define __NR___thread_selfid VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(372)  // was UX64
#else
			/* 372  */
#endif
			/* 373  */
#if DARWIN_VERS >= DARWIN_10_11
#define	__NR_kevent_qos             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(374)
#endif /* DARWIN_VERS >= DARWIN_10_11 */
#if DARWIN_VERS >= DARWIN_10_13
#define	__NR_kevent_id              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(375)
#else
			/* 375  */
#endif /* DARWIN_VERS >= DARWIN_10_13 */
			/* 376  */
			/* 377  */
			/* 378  */
			/* 379  */
#define __NR___mac_execve   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(380)
#define __NR___mac_syscall  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(381)
#define __NR___mac_get_file VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(382)
#define __NR___mac_set_file VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(383)
#define __NR___mac_get_link VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(384)
#define __NR___mac_set_link VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(385)
#define __NR___mac_get_proc VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(386)
#define __NR___mac_set_proc VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(387)
#define __NR___mac_get_fd   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(388)
#define __NR___mac_set_fd   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(389)
#define __NR___mac_get_pid  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(390)
#define __NR___mac_get_lcid VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(391)
#define __NR___mac_get_lctx VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(392)
#define __NR___mac_set_lctx VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(393)

#if DARWIN_VERS >= DARWIN_10_11
#define __NR_pselect        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(394)
#else
#define __NR_setlcid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(394)
#endif /* DARWIN_VERS >= DARWIN_10_11 */

#define __NR_getlcid        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(395)
#define __NR_read_nocancel          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(396)
#define __NR_write_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(397)
#define __NR_open_nocancel          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(398)
#define __NR_close_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(399)
#define __NR_wait4_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(400)
#define __NR_recvmsg_nocancel       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(401)
#define __NR_sendmsg_nocancel       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(402)
#define __NR_recvfrom_nocancel      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(403)
#define __NR_accept_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(404)
#define __NR_msync_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(405)
#define __NR_fcntl_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(406)
#define __NR_select_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(407)
#define __NR_fsync_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(408)
#define __NR_connect_nocancel       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(409)
#define __NR_sigsuspend_nocancel    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(410)
#define __NR_readv_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(411)
#define __NR_writev_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(412)
#define __NR_sendto_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(413)
#define __NR_pread_nocancel         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(414)
#define __NR_pwrite_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(415)
#define __NR_waitid_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(416)
#define __NR_poll_nocancel          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(417)
#define __NR_msgsnd_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(418)
#define __NR_msgrcv_nocancel        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(419)
#define __NR_sem_wait_nocancel      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(420)
#define __NR_aio_suspend_nocancel   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(421)
#define __NR___sigwait_nocancel     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(422)
#define __NR___semwait_signal_nocancel VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(423)
#define __NR___mac_mount            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(424)
#define __NR___mac_get_mount        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(425)
#define __NR___mac_getfsstat        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(426)
#if DARWIN_VERS >= DARWIN_10_6
#define __NR_fsgetpath              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(427)
#define __NR_audit_session_self     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(428)
#define __NR_audit_session_join     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(429)
#endif /* DARWIN_VERS >= DARWIN_10_6 */
#if DARWIN_VERS >= DARWIN_10_9
#define __NR_fileport_makeport      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(430)
#define __NR_fileport_makefd        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(431)
#define __NR_audit_session_port     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(432)
#define __NR_pid_suspend            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(433)
#define __NR_pid_resume             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(434)
			/* 435  */
			/* 436  */
			/* 437  */
#define __NR_shared_region_map_and_slide_np  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(438)
#define __NR_kas_info               VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(439)
#define __NR_memorystatus_control   VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(440)
#define __NR_guarded_open_np        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(441)
#define __NR_guarded_close_np       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(442)
#define __NR_guarded_kqueue_np      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(443)
#define __NR_change_fdguard_np      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(444)
			/* 445  */
#define __NR_proc_rlimit_control    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(446)
#define __NR_connectx               VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(447)
#define __NR_disconnectx            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(448)
#define __NR_peeloff                VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(449)
#define __NR_socket_delegate        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(450)
#define __NR_telemetry              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(451)
#define __NR_proc_uuid_policy       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(452)
#define __NR_memorystatus_get_level VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(453)
#define __NR_system_override        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(454)
#define __NR_vfs_purge              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(455)
#endif /* DARWIN_VERS >= DARWIN_10_9 */

#if DARWIN_VERS >= DARWIN_10_10
#define __NR_necp_match_policy      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(460)
#define __NR_getattrlistbulk        VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(461)
#endif /* DARWIN_VERS >= DARWIN_10_10 */

#if DARWIN_VERS >= DARWIN_10_12
#define __NR_clonefileat            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(462)
#endif /* DARWIN_VERS >= DARWIN_10_12 */

#if DARWIN_VERS >= DARWIN_10_10
#define __NR_faccessat              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(466)
#define __NR_fstatat64              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(470)
#define __NR_readlinkat             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(473)
#define __NR_bsdthread_ctl          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(478)
#define __NR_csrctl                 VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(483)
#define __NR_guarded_open_dprotected_np VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(484)
#define __NR_guarded_write_np       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(485)
#define __NR_guarded_pwrite_np      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(486)
#define __NR_guarded_writev_np      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(487)
#endif /* DARWIN_VERS >= DARWIN_10_10 */

#if DARWIN_VERS >= DARWIN_10_12
#define	__NR_renameatx_np           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(488)
#endif /* DARWIN_VERS >= DARWIN_10_12 */
			/* 489  */

#if DARWIN_VERS >= DARWIN_10_11
#define	__NR_netagent_trigger       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(490)
#define	__NR_stack_snapshot_with_config       VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(491)
#define	__NR_microstackshot         VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(492)
#define	__NR_grab_pgo_data          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(493)
#define	__NR_persona                VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(494)
			/* 495  */
			/* 496  */
			/* 497  */
			/* 498  */
#define	__NR_work_interval_ctl      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(499)
#endif /* DARWIN_VERS >= DARWIN_10_11 */

#if DARWIN_VERS >= DARWIN_10_12
#define	__NR_getentropy             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(500)
#define	__NR_necp_open              VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(501)
#define	__NR_necp_client_action     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(502)
			/* 503  */
			/* 504  */
			/* 505  */
			/* 506  */
			/* 507  */
			/* 508  */
			/* 509  */
			/* 510  */
			/* 511  */
			/* 512  */
			/* 513  */
			/* 514  */
#define	__NR_ulock_wait             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(515)
#define	__NR_ulock_wake             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(516)
#define	__NR_fclonefileat           VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(517)
#define	__NR_fs_snapshot            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(518)
			/* 519  */
#define	__NR_terminate_with_payload VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(520)
#define	__NR_abort_with_payload     VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(521)
#endif /* DARWIN_VERS >= DARWIN_10_12 */

#if DARWIN_VERS >= DARWIN_10_13
#define	__NR_necp_session_open      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(522)
#define	__NR_necp_session_action    VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(523)
#define	__NR_setattrlistat          VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(524)
#define	__NR_net_qos_guideline      VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(525)
#define	__NR_fmount                 VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(526)
#define	__NR_ntp_adjtime            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(527)
#define	__NR_ntp_gettime            VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(528)
#define	__NR_os_fault_with_payload  VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(529)
#endif /* DARWIN_VERS >= DARWIN_10_13 */

#if DARWIN_VERS < DARWIN_10_6
#define	__NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(427)
#elif DARWIN_VERS < DARWIN_10_7
#define	__NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(430)
#elif DARWIN_VERS < DARWIN_10_9
#define	__NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(430)
#elif DARWIN_VERS == DARWIN_10_9
#define	__NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(456)
#elif DARWIN_VERS == DARWIN_10_10
#define __NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(490)
#elif DARWIN_VERS == DARWIN_10_11
#define __NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(500)
#elif DARWIN_VERS == DARWIN_10_12
#define __NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(522)
#elif DARWIN_VERS == DARWIN_10_13
#define __NR_MAXSYSCALL             VG_DARWIN_SYSCALL_CONSTRUCT_UNIX(530)
#else
#error unknown darwin version
#endif

#define __NR_DARWIN_FAKE_SIGRETURN (1 + __NR_MAXSYSCALL)

#endif
