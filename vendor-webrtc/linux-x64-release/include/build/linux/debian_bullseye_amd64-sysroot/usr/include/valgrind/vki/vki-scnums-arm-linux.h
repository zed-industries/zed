
/*--------------------------------------------------------------------*/
/*--- System call numbers for arm-linux.                           ---*/
/*---                                       vki-scnums-arm-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2008-2017 Evan Geller
      gaze@bea.ms

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

#ifndef __VKI_SCNUMS_ARM_LINUX_H
#define __VKI_SCNUMS_ARM_LINUX_H

// From linux-2.6/arch/arm/tools/syscall.tbl

#define __NR_restart_syscall		  0
#define __NR_exit			  1
#define __NR_fork			  2
#define __NR_read			  3
#define __NR_write			  4
#define __NR_open			  5
#define __NR_close			  6
					/* 7 was sys_waitpid */
#define __NR_creat			  8
#define __NR_link			  9
#define __NR_unlink			 10
#define __NR_execve			 11
#define __NR_chdir			 12
#define __NR_time			 13
#define __NR_mknod			 14
#define __NR_chmod			 15
#define __NR_lchown			 16
					/* 17 was sys_break */
					/* 18 was sys_stat */
#define __NR_lseek			 19
#define __NR_getpid			 20
#define __NR_mount			 21
#define __NR_umount			 22
#define __NR_setuid			 23
#define __NR_getuid			 24
#define __NR_stime			 25
#define __NR_ptrace			 26
#define __NR_alarm			 27
					/* 28 was sys_fstat */
#define __NR_pause			 29
#define __NR_utime			 30
					/* 31 was sys_stty */
					/* 32 was sys_gtty */
#define __NR_access			 33
#define __NR_nice			 34
					/* 35 was sys_ftime */
#define __NR_sync			 36
#define __NR_kill			 37
#define __NR_rename			 38
#define __NR_mkdir			 39
#define __NR_rmdir			 40
#define __NR_dup			 41
#define __NR_pipe			 42
#define __NR_times			 43
					/* 44 was sys_prof */
#define __NR_brk			 45
#define __NR_setgid			 46
#define __NR_getgid			 47
					/* 48 was sys_signal */
#define __NR_geteuid			 49
#define __NR_getegid			 50
#define __NR_acct			 51
#define __NR_umount2			 52
					/* 53 was sys_lock */
#define __NR_ioctl			 54
#define __NR_fcntl			 55
					/* 56 was sys_mpx */
#define __NR_setpgid			 57
					/* 58 was sys_ulimit */
					/* 59 was sys_olduname */
#define __NR_umask			 60
#define __NR_chroot			 61
#define __NR_ustat			 62
#define __NR_dup2			 63
#define __NR_getppid			 64
#define __NR_getpgrp			 65
#define __NR_setsid			 66
#define __NR_sigaction			 67
					/* 68 was sys_sgetmask */
					/* 69 was sys_ssetmask */
#define __NR_setreuid			 70
#define __NR_setregid			 71
#define __NR_sigsuspend			 72
#define __NR_sigpending			 73
#define __NR_sethostname		 74
#define __NR_setrlimit			 75
#define __NR_getrlimit			 76	/* Back compat 2GB limited rlimit */
#define __NR_getrusage			 77
#define __NR_gettimeofday		 78
#define __NR_settimeofday		 79
#define __NR_getgroups			 80
#define __NR_setgroups			 81
#define __NR_select			 82
#define __NR_symlink			 83
					/* 84 was sys_lstat */
#define __NR_readlink			 85
#define __NR_uselib			 86
#define __NR_swapon			 87
#define __NR_reboot			 88
#define __NR_readdir			 89
#define __NR_mmap			 90
#define __NR_munmap			 91
#define __NR_truncate			 92
#define __NR_ftruncate			 93
#define __NR_fchmod			 94
#define __NR_fchown			 95
#define __NR_getpriority		 96
#define __NR_setpriority		 97
					/* 98 was sys_profil */
#define __NR_statfs			 99
#define __NR_fstatfs			100
					/* 101 was sys_ioperm */
#define __NR_socketcall			102
#define __NR_syslog			103
#define __NR_setitimer			104
#define __NR_getitimer			105
#define __NR_stat			106
#define __NR_lstat			107
#define __NR_fstat			108
					/* 109 was sys_uname */
					/* 110 was sys_iopl */
#define __NR_vhangup			111
					/* 112 was sys_idle */
#define __NR_syscall			113 /* syscall to call a syscall! */
#define __NR_wait4			114
#define __NR_swapoff			115
#define __NR_sysinfo			116
#define __NR_ipc			117
#define __NR_fsync			118
#define __NR_sigreturn			119
#define __NR_clone			120
#define __NR_setdomainname		121
#define __NR_uname			122
					/* 123 was sys_modify_ldt */
#define __NR_adjtimex			124
#define __NR_mprotect			125
#define __NR_sigprocmask		126
					/* 127 was sys_create_module */
#define __NR_init_module		128
#define __NR_delete_module		129
					/* 130 was sys_get_kernel_syms */
#define __NR_quotactl			131
#define __NR_getpgid			132
#define __NR_fchdir			133
#define __NR_bdflush			134
#define __NR_sysfs			135
#define __NR_personality		136
					/* 137 was sys_afs_syscall */
#define __NR_setfsuid			138
#define __NR_setfsgid			139
#define __NR__llseek			140
#define __NR_getdents			141
#define __NR__newselect			142
#define __NR_flock			143
#define __NR_msync			144
#define __NR_readv			145
#define __NR_writev			146
#define __NR_getsid			147
#define __NR_fdatasync			148
#define __NR__sysctl			149
#define __NR_mlock			150
#define __NR_munlock			151
#define __NR_mlockall			152
#define __NR_munlockall			153
#define __NR_sched_setparam		154
#define __NR_sched_getparam		155
#define __NR_sched_setscheduler		156
#define __NR_sched_getscheduler		157
#define __NR_sched_yield		158
#define __NR_sched_get_priority_max	159
#define __NR_sched_get_priority_min	160
#define __NR_sched_rr_get_interval	161
#define __NR_nanosleep			162
#define __NR_mremap			163
#define __NR_setresuid			164
#define __NR_getresuid			165
					/* 166 was sys_vm86 */
					/* 167 was sys_query_module */
#define __NR_poll			168
#define __NR_nfsservctl			169
#define __NR_setresgid			170
#define __NR_getresgid			171
#define __NR_prctl			172
#define __NR_rt_sigreturn		173
#define __NR_rt_sigaction		174
#define __NR_rt_sigprocmask		175
#define __NR_rt_sigpending		176
#define __NR_rt_sigtimedwait		177
#define __NR_rt_sigqueueinfo		178
#define __NR_rt_sigsuspend		179
#define __NR_pread64			180
#define __NR_pwrite64			181
#define __NR_chown			182
#define __NR_getcwd			183
#define __NR_capget			184
#define __NR_capset			185
#define __NR_sigaltstack		186
#define __NR_sendfile			187
					/* 188 reserved */
					/* 189 reserved */
#define __NR_vfork			190
#define __NR_ugetrlimit			191	/* SuS compliant getrlimit */
#define __NR_mmap2			192
#define __NR_truncate64			193
#define __NR_ftruncate64		194
#define __NR_stat64			195
#define __NR_lstat64			196
#define __NR_fstat64			197
#define __NR_lchown32			198
#define __NR_getuid32			199
#define __NR_getgid32			200
#define __NR_geteuid32			201
#define __NR_getegid32			202
#define __NR_setreuid32			203
#define __NR_setregid32			204
#define __NR_getgroups32		205
#define __NR_setgroups32		206
#define __NR_fchown32			207
#define __NR_setresuid32		208
#define __NR_getresuid32		209
#define __NR_setresgid32		210
#define __NR_getresgid32		211
#define __NR_chown32			212
#define __NR_setuid32			213
#define __NR_setgid32			214
#define __NR_setfsuid32			215
#define __NR_setfsgid32			216
#define __NR_getdents64			217
#define __NR_pivot_root			218
#define __NR_mincore			219
#define __NR_madvise			220
#define __NR_fcntl64			221
					/* 222 for tux */
					/* 223 is unused */
#define __NR_gettid			224
#define __NR_readahead			225
#define __NR_setxattr			226
#define __NR_lsetxattr			227
#define __NR_fsetxattr			228
#define __NR_getxattr			229
#define __NR_lgetxattr			230
#define __NR_fgetxattr			231
#define __NR_listxattr			232
#define __NR_llistxattr			233
#define __NR_flistxattr			234
#define __NR_removexattr		235
#define __NR_lremovexattr		236
#define __NR_fremovexattr		237
#define __NR_tkill			238
#define __NR_sendfile64			239
#define __NR_futex			240
#define __NR_sched_setaffinity		241
#define __NR_sched_getaffinity		242
#define __NR_io_setup			243
#define __NR_io_destroy			244
#define __NR_io_getevents		245
#define __NR_io_submit			246
#define __NR_io_cancel			247
#define __NR_exit_group			248
#define __NR_lookup_dcookie		249
#define __NR_epoll_create		250
#define __NR_epoll_ctl			251
#define __NR_epoll_wait			252
#define __NR_remap_file_pages		253
					/* 254 for set_thread_area */
					/* 255 for get_thread_area */
#define __NR_set_tid_address		256
#define __NR_timer_create		257
#define __NR_timer_settime		258
#define __NR_timer_gettime		259
#define __NR_timer_getoverrun		260
#define __NR_timer_delete		261
#define __NR_clock_settime		262
#define __NR_clock_gettime		263
#define __NR_clock_getres		264
#define __NR_clock_nanosleep		265
#define __NR_statfs64			266
#define __NR_fstatfs64			267
#define __NR_tgkill			268
#define __NR_utimes			269
#define __NR_arm_fadvise64_64		270
#define __NR_fadvise64          270 //Added by Johan, 2008-10-11, not sure why it's called _arm_.. otherwise.
#define __NR_pciconfig_iobase		271
#define __NR_pciconfig_read		272
#define __NR_pciconfig_write		273
#define __NR_mq_open			274
#define __NR_mq_unlink			275
#define __NR_mq_timedsend		276
#define __NR_mq_timedreceive		277
#define __NR_mq_notify			278
#define __NR_mq_getsetattr		279
#define __NR_waitid			280
#define __NR_socket			281
#define __NR_bind			282
#define __NR_connect			283
#define __NR_listen			284
#define __NR_accept			285
#define __NR_getsockname		286
#define __NR_getpeername		287
#define __NR_socketpair			288
#define __NR_send			289
#define __NR_sendto			290
#define __NR_recv			291
#define __NR_recvfrom			292
#define __NR_shutdown			293
#define __NR_setsockopt			294
#define __NR_getsockopt			295
#define __NR_sendmsg			296
#define __NR_recvmsg			297
#define __NR_semop			298
#define __NR_semget			299
#define __NR_semctl			300
#define __NR_msgsnd			301
#define __NR_msgrcv			302
#define __NR_msgget			303
#define __NR_msgctl			304
#define __NR_shmat			305
#define __NR_shmdt			306
#define __NR_shmget			307
#define __NR_shmctl			308
#define __NR_add_key			309
#define __NR_request_key		310
#define __NR_keyctl			311
#define __NR_semtimedop			312
#define __NR_vserver			313
#define __NR_ioprio_set			314
#define __NR_ioprio_get			315
#define __NR_inotify_init		316
#define __NR_inotify_add_watch		317
#define __NR_inotify_rm_watch		318
#define __NR_mbind			319
#define __NR_get_mempolicy		320
#define __NR_set_mempolicy		321
#define __NR_openat			322
#define __NR_mkdirat			323
#define __NR_mknodat			324
#define __NR_fchownat			325
#define __NR_futimesat			326
#define __NR_fstatat64			327
#define __NR_unlinkat			328
#define __NR_renameat			329
#define __NR_linkat			330
#define __NR_symlinkat			331
#define __NR_readlinkat			332
#define __NR_fchmodat			333
#define __NR_faccessat			334
#define __NR_pselect6			335 /* JRS 20100812: is this correct? */
#define __NR_ppoll	       		336
#define __NR_unshare			337
#define __NR_set_robust_list		338
#define __NR_get_robust_list		339
#define __NR_splice			340
#define __NR_arm_sync_file_range	341
#define __NR_sync_file_range2		__NR_arm_sync_file_range
#define __NR_tee			342
#define __NR_vmsplice			343
#define __NR_move_pages			344
#define __NR_getcpu			345
#define __NR_epoll_pwait		346
#define __NR_kexec_load			347
#define __NR_utimensat			348
#define __NR_signalfd			349
#define __NR_timerfd_create		350
#define __NR_eventfd			351
#define __NR_fallocate			352
#define __NR_timerfd_settime		353
#define __NR_timerfd_gettime		354
#define __NR_signalfd4                  355
#define __NR_eventfd2                   356
#define __NR_epoll_create1              357
#define __NR_dup3                       358
#define __NR_pipe2                      359
#define __NR_inotify_init1              360
#define __NR_preadv			361
#define __NR_pwritev			362
#define __NR_rt_tgsigqueueinfo		363
#define __NR_perf_event_open		364
#define __NR_recvmmsg			365
#define __NR_accept4			366
#define __NR_fanotify_init		367
#define __NR_fanotify_mark		368
#define __NR_prlimit64			369
#define __NR_name_to_handle_at		370
#define __NR_open_by_handle_at		371
#define __NR_clock_adjtime		372
#define __NR_syncfs			373
#define __NR_sendmmsg			374
#define __NR_setns			375
#define __NR_process_vm_readv		376
#define __NR_process_vm_writev		377
#define __NR_kcmp                       378
#define __NR_finit_module               379
#define __NR_sched_setattr              380
#define __NR_sched_getattr              381
#define __NR_renameat2                  382
#define __NR_seccomp                    383
#define __NR_getrandom                  384
#define __NR_memfd_create               385
#define __NR_bpf                        386
#define __NR_execveat                   387
#define __NR_userfaultfd                388
#define __NR_membarrier                 389
#define __NR_mlock2                     390
#define __NR_copy_file_range            391
#define __NR_preadv2                    392
#define __NR_pwritev2                   393
#define __NR_pkey_mprotect              394
#define __NR_pkey_alloc                 395
#define __NR_pkey_free                  396
#define __NR_statx                      397



#define __NR_ARM_BASE                   (0x0f0000)
#define __NR_ARM_breakpoint             (__NR_ARM_BASE+1)
#define __NR_ARM_cacheflush             (__NR_ARM_BASE+2)
#define __NR_ARM_usr26                  (__NR_ARM_BASE+3)
#define __NR_ARM_usr32                  (__NR_ARM_BASE+4)
#define __NR_ARM_set_tls                (__NR_ARM_BASE+5)


#endif /* __VKI_SCNUMS_ARM_LINUX_H */

/*--------------------------------------------------------------------*/
/*--- end                                   vki-scnums-arm-linux.h ---*/
/*--------------------------------------------------------------------*/
