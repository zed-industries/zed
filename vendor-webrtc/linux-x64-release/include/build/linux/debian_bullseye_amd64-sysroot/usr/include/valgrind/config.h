/* config.h.  Generated from config.h.in by configure.  */
/* config.h.in.  Generated from configure.ac by autoheader.  */

/* Define to 1 if you're using Bionic */
/* #undef BIONIC_LIBC */

/* DARWIN_VERS value for Mac OS X 10.10 */
/* #undef DARWIN_10_10 */

/* DARWIN_VERS value for Mac OS X 10.11 */
/* #undef DARWIN_10_11 */

/* DARWIN_VERS value for macOS 10.12 */
/* #undef DARWIN_10_12 */

/* DARWIN_VERS value for macOS 10.13 */
/* #undef DARWIN_10_13 */

/* DARWIN_VERS value for Mac OS X 10.5 */
/* #undef DARWIN_10_5 */

/* DARWIN_VERS value for Mac OS X 10.6 */
/* #undef DARWIN_10_6 */

/* DARWIN_VERS value for Mac OS X 10.7 */
/* #undef DARWIN_10_7 */

/* DARWIN_VERS value for Mac OS X 10.8 */
/* #undef DARWIN_10_8 */

/* DARWIN_VERS value for Mac OS X 10.9 */
/* #undef DARWIN_10_9 */

/* Define to 1 if you're using Darwin */
/* #undef DARWIN_LIBC */

/* Darwin / Mac OS X version */
/* #undef DARWIN_VERS */

/* Disable intercept pthread_spin_lock() on MIPS32, MIPS64 and nanoMIPS. */
/* #undef DISABLE_PTHREAD_SPINLOCK_INTERCEPT */

/* configured to run as an inner Valgrind */
/* #undef ENABLE_INNER */

/* configured to build with lto link time optimisation */
/* #undef ENABLE_LTO */

/* path to GDB */
#define GDB_PATH "/usr/bin/gdb"

/* Define to 1 if index() and strlen() have been optimized heavily (x86 glibc
   >= 2.12) */
#define GLIBC_MANDATORY_INDEX_AND_STRLEN_REDIRECT 1

/* Define to 1 if strlen() has been optimized heavily (amd64 glibc >= 2.10) */
#define GLIBC_MANDATORY_STRLEN_REDIRECT 1

/* Define to 1 if you have the <asm/unistd.h> header file. */
#define HAVE_ASM_UNISTD_H 1

/* Define to 1 if as supports fxsave64/fxrstor64. */
#define HAVE_AS_AMD64_FXSAVE64 1

/* Define to 1 if as supports floating point phased out category. */
/* #undef HAVE_AS_PPC_FPPO */

/* Define to 1 if as supports mtocrf/mfocrf. */
/* #undef HAVE_AS_PPC_MFTOCRF */

/* Define to 1 if gcc supports __sync_bool_compare_and_swap() and
   __sync_add_and_fetch() for the primary target */
#define HAVE_BUILTIN_ATOMIC 1

/* Define to 1 if g++ supports __sync_bool_compare_and_swap() and
   __sync_add_and_fetch() */
#define HAVE_BUILTIN_ATOMIC_CXX 1

/* Define to 1 if compiler provides __builtin_clz(). */
#define HAVE_BUILTIN_CLZ 1

/* Define to 1 if compiler provides __builtin_ctz(). */
#define HAVE_BUILTIN_CTZ 1

/* Define to 1 if compiler provides __builtin_popcount(). */
#define HAVE_BUILTIN_POPCOUT 1

/* Define to 1 if you have the `clock_gettime' function. */
#define HAVE_CLOCK_GETTIME 1

/* Define to 1 if you have the `CLOCK_MONOTONIC' constant. */
#define HAVE_CLOCK_MONOTONIC 1

/* Define to 1 if you have the `copy_file_range' function. */
#define HAVE_COPY_FILE_RANGE 1

/* Define to 1 if you have a dlinfo that can do RTLD_DI_TLS_MODID. */
#define HAVE_DLINFO_RTLD_DI_TLS_MODID 1

/* Define to 1 if the system has the type `Elf32_Chdr'. */
#define HAVE_ELF32_CHDR 1

/* Define to 1 if the system has the type `Elf64_Chdr'. */
#define HAVE_ELF64_CHDR 1

/* Define to 1 if you have the <endian.h> header file. */
#define HAVE_ENDIAN_H 1

/* Define to 1 if you have the `epoll_create' function. */
#define HAVE_EPOLL_CREATE 1

/* Define to 1 if you have the `epoll_pwait' function. */
#define HAVE_EPOLL_PWAIT 1

/* Define to 1 if you have the `eventfd' function. */
#define HAVE_EVENTFD 1

/* Define to 1 if you have the `eventfd_read' function. */
#define HAVE_EVENTFD_READ 1

/* Define to 1 if you have the `getpagesize' function. */
#define HAVE_GETPAGESIZE 1

/* Define to 1 if you have the <inttypes.h> header file. */
#define HAVE_INTTYPES_H 1

/* Define to 1 if you have the `klogctl' function. */
#define HAVE_KLOGCTL 1

/* Define to 1 if you have the `pthread' library (-lpthread). */
#define HAVE_LIBPTHREAD 1

/* Define to 1 if you have the `rt' library (-lrt). */
#define HAVE_LIBRT 1

/* Define to 1 if you have the `scf' library (-lscf). */
/* #undef HAVE_LIBSCF */

/* Define to 1 if you have the `mallinfo' function. */
#define HAVE_MALLINFO 1

/* Define to 1 if you have the `memchr' function. */
#define HAVE_MEMCHR 1

/* Define to 1 if you have the <memory.h> header file. */
#define HAVE_MEMORY_H 1

/* Define to 1 if you have the `memset' function. */
#define HAVE_MEMSET 1

/* Define to 1 if you have the `mkdir' function. */
#define HAVE_MKDIR 1

/* Define to 1 if you have a working `mmap' system call. */
#define HAVE_MMAP 1

/* Define to 1 if you have the <mqueue.h> header file. */
#define HAVE_MQUEUE_H 1

/* Define to 1 if you have the `mremap' function. */
#define HAVE_MREMAP 1

/* Define to 1 if you have the `ppoll' function. */
#define HAVE_PPOLL 1

/* Define to 1 if you have the `preadv' function. */
#define HAVE_PREADV 1

/* Define to 1 if you have the `preadv2' function. */
#define HAVE_PREADV2 1

/* Define to 1 if you have the `process_vm_readv' function. */
#define HAVE_PROCESS_VM_READV 1

/* Define to 1 if you have the `process_vm_writev' function. */
#define HAVE_PROCESS_VM_WRITEV 1

/* Define to 1 if you have the `pthread_barrier_init' function. */
#define HAVE_PTHREAD_BARRIER_INIT 1

/* Define to 1 if you have the `pthread_condattr_setclock' function. */
#define HAVE_PTHREAD_CONDATTR_SETCLOCK 1

/* Define to 1 if you have the `pthread_create@glibc2.0' function. */
/* #undef HAVE_PTHREAD_CREATE_GLIBC_2_0 */

/* Define to 1 if you have the `PTHREAD_MUTEX_ADAPTIVE_NP' constant. */
#define HAVE_PTHREAD_MUTEX_ADAPTIVE_NP 1

/* Define to 1 if you have the `PTHREAD_MUTEX_ERRORCHECK_NP' constant. */
#define HAVE_PTHREAD_MUTEX_ERRORCHECK_NP 1

/* Define to 1 if you have the `PTHREAD_MUTEX_RECURSIVE_NP' constant. */
#define HAVE_PTHREAD_MUTEX_RECURSIVE_NP 1

/* Define to 1 if you have the `pthread_mutex_timedlock' function. */
#define HAVE_PTHREAD_MUTEX_TIMEDLOCK 1

/* Define to 1 if pthread_mutex_t has a member __data.__kind. */
#define HAVE_PTHREAD_MUTEX_T__DATA__KIND 1

/* Define to 1 if pthread_mutex_t has a member called __m_kind. */
/* #undef HAVE_PTHREAD_MUTEX_T__M_KIND */

/* Define to 1 if you have the `PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP'
   constant. */
#define HAVE_PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP 1

/* Define to 1 if you have the `pthread_rwlock_t' type. */
#define HAVE_PTHREAD_RWLOCK_T 1

/* Define to 1 if you have the `pthread_rwlock_timedrdlock' function. */
#define HAVE_PTHREAD_RWLOCK_TIMEDRDLOCK 1

/* Define to 1 if you have the `pthread_rwlock_timedwrlock' function. */
#define HAVE_PTHREAD_RWLOCK_TIMEDWRLOCK 1

/* Define to 1 if you have the `pthread_setname_np' function. */
#define HAVE_PTHREAD_SETNAME_NP 1

/* Define to 1 if you have the `pthread_spin_lock' function. */
#define HAVE_PTHREAD_SPIN_LOCK 1

/* Define to 1 if you have the `pthread_yield' function. */
#define HAVE_PTHREAD_YIELD 1

/* Define to 1 if you have the `PTRACE_GETREGS' ptrace request. */
#define HAVE_PTRACE_GETREGS 1

/* Define to 1 if you have the `pwritev' function. */
#define HAVE_PWRITEV 1

/* Define to 1 if you have the `pwritev2' function. */
#define HAVE_PWRITEV2 1

/* Define to 1 if you have the `readlinkat' function. */
#define HAVE_READLINKAT 1

/* Define to 1 if you have the `semtimedop' function. */
#define HAVE_SEMTIMEDOP 1

/* Define to 1 if libstd++ supports annotating shared pointers */
#define HAVE_SHARED_POINTER_ANNOTATION 1

/* Define to 1 if you have the `signalfd' function. */
#define HAVE_SIGNALFD 1

/* Define to 1 if you have the `sigwaitinfo' function. */
#define HAVE_SIGWAITINFO 1

/* Define to 1 if you have the <stdint.h> header file. */
#define HAVE_STDINT_H 1

/* Define to 1 if you have the <stdlib.h> header file. */
#define HAVE_STDLIB_H 1

/* Define to 1 if you have the `strchr' function. */
#define HAVE_STRCHR 1

/* Define to 1 if you have the `strdup' function. */
#define HAVE_STRDUP 1

/* Define to 1 if you have the <strings.h> header file. */
#define HAVE_STRINGS_H 1

/* Define to 1 if you have the <string.h> header file. */
#define HAVE_STRING_H 1

/* Define to 1 if you have the `strpbrk' function. */
#define HAVE_STRPBRK 1

/* Define to 1 if you have the `strrchr' function. */
#define HAVE_STRRCHR 1

/* Define to 1 if you have the `strstr' function. */
#define HAVE_STRSTR 1

/* Define to 1 if you have the `syscall' function. */
#define HAVE_SYSCALL 1

/* Define to 1 if you have the <sys/endian.h> header file. */
/* #undef HAVE_SYS_ENDIAN_H */

/* Define to 1 if you have the <sys/epoll.h> header file. */
#define HAVE_SYS_EPOLL_H 1

/* Define to 1 if you have the <sys/eventfd.h> header file. */
#define HAVE_SYS_EVENTFD_H 1

/* Define to 1 if you have the <sys/klog.h> header file. */
#define HAVE_SYS_KLOG_H 1

/* Define to 1 if you have the <sys/lgrp_user_impl.h> header file. */
/* #undef HAVE_SYS_LGRP_USER_IMPL_H */

/* Define to 1 if you have the <sys/param.h> header file. */
#define HAVE_SYS_PARAM_H 1

/* Define to 1 if you have the <sys/poll.h> header file. */
#define HAVE_SYS_POLL_H 1

/* Define to 1 if you have the <sys/prctl.h> header file. */
#define HAVE_SYS_PRCTL_H 1

/* Define to 1 if you have the <sys/signalfd.h> header file. */
#define HAVE_SYS_SIGNALFD_H 1

/* Define to 1 if you have the <sys/signal.h> header file. */
#define HAVE_SYS_SIGNAL_H 1

/* Define to 1 if you have the <sys/stat.h> header file. */
#define HAVE_SYS_STAT_H 1

/* Define to 1 if you have the <sys/syscall.h> header file. */
#define HAVE_SYS_SYSCALL_H 1

/* Define to 1 if you have the <sys/sysnvl.h> header file. */
/* #undef HAVE_SYS_SYSNVL_H */

/* Define to 1 if you have the <sys/time.h> header file. */
#define HAVE_SYS_TIME_H 1

/* Define to 1 if you have the <sys/types.h> header file. */
#define HAVE_SYS_TYPES_H 1

/* Define to 1 if <sys/user.h> defines struct user_regs_struct */
#define HAVE_SYS_USER_REGS 1

/* can use __thread to define thread-local variables */
#define HAVE_TLS 1

/* Define to 1 if you have the <unistd.h> header file. */
#define HAVE_UNISTD_H 1

/* Define to 1 if you have a usable <linux/futex.h> header file. */
#define HAVE_USABLE_LINUX_FUTEX_H 1

/* Define to 1 if you have the `utimensat' function. */
#define HAVE_UTIMENSAT 1

/* Define to 1 if you're using Musl libc */
/* #undef MUSL_LIBC */

/* Name of package */
#define PACKAGE "valgrind"

/* Define to the address where bug reports for this package should be sent. */
#define PACKAGE_BUGREPORT "valgrind-users@lists.sourceforge.net"

/* Define to the full name of this package. */
#define PACKAGE_NAME "Valgrind"

/* Define to the full name and version of this package. */
#define PACKAGE_STRING "Valgrind 3.16.1"

/* Define to the one symbol short name of this package. */
#define PACKAGE_TARNAME "valgrind"

/* Define to the home page for this package. */
#define PACKAGE_URL ""

/* Define to the version of this package. */
#define PACKAGE_VERSION "3.16.1"

/* Define to 1 if you have the `A_GETSTAT' and `A_SETSTAT' constants. */
/* #undef SOLARIS_AUDITON_STAT */

/* Define to 1 if you have the new `execve' syscall which accepts flags. */
/* #undef SOLARIS_EXECVE_SYSCALL_TAKES_FLAGS */

/* Define to 1 if fpregset_t defines struct _fpchip_state */
/* #undef SOLARIS_FPCHIP_STATE_TAKES_UNDERSCORE */

/* Define to 1 if you have the new `frealpathat' syscall. */
/* #undef SOLARIS_FREALPATHAT_SYSCALL */

/* Define to 1 if you have the new `gethrt' fasttrap. */
/* #undef SOLARIS_GETHRT_FASTTRAP */

/* Define to 1 if you have the new `getrandom' syscall. */
/* #undef SOLARIS_GETRANDOM_SYSCALL */

/* Define to 1 if you have the new `get_zone_offset' fasttrap. */
/* #undef SOLARIS_GETZONEOFFSET_FASTTRAP */

/* Default platform for Valgrind launcher. */
/* #undef SOLARIS_LAUNCHER_DEFAULT_PLATFORM */

/* Define to 1 if you have the new `lwp_name' syscall. */
/* #undef SOLARIS_LWP_NAME_SYSCALL */

/* Define to 1 if you have the new `lwp_sigqueue' syscall. */
/* #undef SOLARIS_LWP_SIGQUEUE_SYSCALL */

/* Define to 1 if you have the new `lwp_sigqueue' syscall which accepts pid.
   */
/* #undef SOLARIS_LWP_SIGQUEUE_SYSCALL_TAKES_PID */

/* Define to 1 if you have the `MODNVL_CTRLMAP' through `MODDEVINFO_CACHE_TS'
   constants. */
/* #undef SOLARIS_MODCTL_MODNVL */

/* Define to 1 if you have the new `accept' syscall. */
/* #undef SOLARIS_NEW_ACCEPT_SYSCALL */

/* Define to 1 if you have the new `pipe' syscall. */
/* #undef SOLARIS_NEW_PIPE_SYSCALL */

/* Define to 1 if nscd attaches to /system/volatile/name_service_door. */
/* #undef SOLARIS_NSCD_DOOR_SYSTEM_VOLATILE */

/* Define to 1 if you have the old Solaris syscalls. */
/* #undef SOLARIS_OLD_SYSCALLS */

/* Define to 1 if you have /proc/self/cmdline. */
/* #undef SOLARIS_PROC_CMDLINE */

/* Define to 1 if you have the `prxregset_t' type. */
/* #undef SOLARIS_PRXREGSET_T */

/* Define to 1 if you have the `PSET_GET_NAME' constants. */
/* #undef SOLARIS_PSET_GET_NAME */

/* Define to 1 if PT_SUNWDTRACE program header provides just an initial thread
   pointer for libc. */
/* #undef SOLARIS_PT_SUNDWTRACE_THRP */

/* Version number of the repository door cache protocol. */
/* #undef SOLARIS_REPCACHE_PROTOCOL_VERSION */

/* Define to 1 if you have the new `sysstat' segment reservation. */
/* #undef SOLARIS_RESERVE_SYSSTAT_ADDR */

/* Define to 1 if you have the new `sysstat_zone' segment reservation. */
/* #undef SOLARIS_RESERVE_SYSSTAT_ZONE_ADDR */

/* Define to 1 if you have the schedctl page executable. */
/* #undef SOLARIS_SCHEDCTL_PAGE_EXEC */

/* Define to 1 if you have the `IPC_XSTAT64', `SHMADV', `SHM_ADV_GET',
   `SHM_ADV_SET' and `SHMGET_OSM' constants. */
/* #undef SOLARIS_SHM_NEW */

/* Define to 1 if you have the `spawn' syscall. */
/* #undef SOLARIS_SPAWN_SYSCALL */

/* Define to 1 if you have the `system_stats' syscall. */
/* #undef SOLARIS_SYSTEM_STATS_SYSCALL */

/* Define to 1 if you have the `TNDB_GET_TNIP' constant. */
/* #undef SOLARIS_TNDB_GET_TNIP */

/* Define to 1 if you have the `TSOL_GETCLEARANCE' and `TSOL_SETCLEARANCE'
   constants. */
/* #undef SOLARIS_TSOL_CLEARANCE */

/* Define to 1 if you have the `utimensat' syscall. */
/* #undef SOLARIS_UTIMENSAT_SYSCALL */

/* Define to 1 if you have the `utimesys' syscall. */
/* #undef SOLARIS_UTIMESYS_SYSCALL */

/* Define to 1 if you have the new `uuidsys' syscall. */
/* #undef SOLARIS_UUIDSYS_SYSCALL */

/* Define to 1 if you have the `ZONE_LIST_DEFUNCT' and `ZONE_GETATTR_DEFUNC'
   constants. */
/* #undef SOLARIS_ZONE_DEFUNCT */

/* Define to 1 if you have the ANSI C header files. */
#define STDC_HEADERS 1

/* Define to 1 if you can safely include both <sys/time.h> and <time.h>. */
#define TIME_WITH_SYS_TIME 1

/* Version number of package */
#define VERSION "3.16.1"

/* Temporary files directory */
#define VG_TMPDIR "/tmp"

/* xcode sdk include directory */
/* #undef XCODE_DIR */

/* Define to `int' if <sys/types.h> doesn't define. */
/* #undef gid_t */

/* Define to `long int' if <sys/types.h> does not define. */
/* #undef off_t */

/* Define to `unsigned int' if <sys/types.h> does not define. */
/* #undef size_t */

/* Define to `int' if <sys/types.h> doesn't define. */
/* #undef uid_t */
