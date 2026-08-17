#include <limits.h>
#if defined(__linux__) && !defined(__GLIBC__) && !defined(__WORDSIZE)
// include __WORDSIZE headers for musl
#  include <bits/reg.h>
#endif
#if __WORDSIZE == 64
#  define BACKTRACE_ELF_SIZE 64
#else
#  define BACKTRACE_ELF_SIZE 32
#endif

#define HAVE_DLFCN_H 1
#define HAVE_FCNTL 1
#define HAVE_INTTYPES_H 1
#define HAVE_LSTAT 1
#define HAVE_READLINK 1
#define HAVE_DL_ITERATE_PHDR 1
#define HAVE_ATOMIC_FUNCTIONS 1
#define HAVE_DECL_STRNLEN 1

#ifdef __APPLE__
#  define HAVE_MACH_O_DYLD_H 1
#elif defined BSD
#  define HAVE_KERN_PROC 1
#  define HAVE_KERN_PROC_ARGS 1
#endif
