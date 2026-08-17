/* Architecture-specific adjustments to siginfo_t.  x86 version.  */
#ifndef _BITS_SIGINFO_ARCH_H
#define _BITS_SIGINFO_ARCH_H 1

#if defined __x86_64__ && __WORDSIZE == 32
/* si_utime and si_stime must be 4 byte aligned for x32 to match the
   kernel.  We align siginfo_t to 8 bytes so that si_utime and
   si_stime are actually aligned to 8 bytes since their offsets are
   multiple of 8 bytes.  Note: with some compilers, the alignment
   attribute would be ignored if it were put in __SI_CLOCK_T instead
   of encapsulated in a typedef.  */
typedef __clock_t __attribute__ ((__aligned__ (4))) __sigchld_clock_t;
# define __SI_ALIGNMENT __attribute__ ((__aligned__ (8)))
# define __SI_CLOCK_T __sigchld_clock_t
#endif

#endif
