/*-
 * Copyright (c) 1982, 1986, 1992, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)gmon.h	8.2 (Berkeley) 1/4/94
 */

#ifndef	_SYS_GMON_H
#define	_SYS_GMON_H	1

#include <features.h>

#include <sys/types.h>

/*
 * See gmon_out.h for gmon.out format.
 */

/* structure emitted by "gcc -a".  This must match struct bb in
   gcc/libgcc2.c.  It is OK for gcc to declare a longer structure as
   long as the members below are present.  */
struct __bb
{
  long			zero_word;
  const char		*filename;
  long			*counts;
  long			ncounts;
  struct __bb		*next;
  const unsigned long	*addresses;
};

extern struct __bb *__bb_head;

/*
 * histogram counters are unsigned shorts (according to the kernel).
 */
#define	HISTCOUNTER	unsigned short

/*
 * fraction of text space to allocate for histogram counters here, 1/2
 */
#define	HISTFRACTION	2

/*
 * Fraction of text space to allocate for from hash buckets.
 * The value of HASHFRACTION is based on the minimum number of bytes
 * of separation between two subroutine call points in the object code.
 * Given MIN_SUBR_SEPARATION bytes of separation the value of
 * HASHFRACTION is calculated as:
 *
 *	HASHFRACTION = MIN_SUBR_SEPARATION / (2 * sizeof(short) - 1);
 *
 * For example, on the VAX, the shortest two call sequence is:
 *
 *	calls	$0,(r0)
 *	calls	$0,(r0)
 *
 * which is separated by only three bytes, thus HASHFRACTION is
 * calculated as:
 *
 *	HASHFRACTION = 3 / (2 * 2 - 1) = 1
 *
 * Note that the division above rounds down, thus if MIN_SUBR_FRACTION
 * is less than three, this algorithm will not work!
 *
 * In practice, however, call instructions are rarely at a minimal
 * distance.  Hence, we will define HASHFRACTION to be 2 across all
 * architectures.  This saves a reasonable amount of space for
 * profiling data structures without (in practice) sacrificing
 * any granularity.
 */
#define	HASHFRACTION	2

/*
 * Percent of text space to allocate for tostructs.
 * This is a heuristic; we will fail with a warning when profiling programs
 * with a very large number of very small functions, but that's
 * normally OK.
 * 2 is probably still a good value for normal programs.
 * Profiling a test case with 64000 small functions will work if
 * you raise this value to 3 and link statically (which bloats the
 * text size, thus raising the number of arcs expected by the heuristic).
 */
#define ARCDENSITY	3

/*
 * Always allocate at least this many tostructs.  This
 * hides the inadequacy of the ARCDENSITY heuristic, at least
 * for small programs.
 */
#define MINARCS		50

/*
 * The type used to represent indices into gmonparam.tos[].
 */
#define	ARCINDEX	unsigned long

/*
 * Maximum number of arcs we want to allow.
 * Used to be max representable value of ARCINDEX minus 2, but now
 * that ARCINDEX is a long, that's too large; we don't really want
 * to allow a 48 gigabyte table.
 * The old value of 1<<16 wasn't high enough in practice for large C++
 * programs; will 1<<20 be adequate for long?  FIXME
 */
#define MAXARCS		(1 << 20)

struct tostruct {
	unsigned long	selfpc;
	long		count;
	ARCINDEX	link;
};

/*
 * a raw arc, with pointers to the calling site and
 * the called site and a count.
 */
struct rawarc {
	unsigned long	raw_frompc;
	unsigned long	raw_selfpc;
	long		raw_count;
};

/*
 * general rounding functions.
 */
#define ROUNDDOWN(x,y)	(((x)/(y))*(y))
#define ROUNDUP(x,y)	((((x)+(y)-1)/(y))*(y))

/*
 * The profiling data structures are housed in this structure.
 */
struct gmonparam {
	long int	state;
	unsigned short	*kcount;
	unsigned long	kcountsize;
	ARCINDEX	*froms;
	unsigned long	fromssize;
	struct tostruct	*tos;
	unsigned long	tossize;
	long		tolimit;
	unsigned long	lowpc;
	unsigned long	highpc;
	unsigned long	textsize;
	unsigned long	hashfraction;
	long		log_hashfraction;
};

/*
 * Possible states of profiling.
 */
#define	GMON_PROF_ON	0
#define	GMON_PROF_BUSY	1
#define	GMON_PROF_ERROR	2
#define	GMON_PROF_OFF	3

/*
 * Sysctl definitions for extracting profiling information from the kernel.
 */
#define	GPROF_STATE	0	/* int: profiling enabling variable */
#define	GPROF_COUNT	1	/* struct: profile tick count buffer */
#define	GPROF_FROMS	2	/* struct: from location hash bucket */
#define	GPROF_TOS	3	/* struct: destination/count structure */
#define	GPROF_GMONPARAM	4	/* struct: profiling parameters (see above) */

__BEGIN_DECLS

/* Set up data structures and start profiling.  */
extern void __monstartup (unsigned long __lowpc, unsigned long __highpc) __THROW;
extern void monstartup (unsigned long __lowpc, unsigned long __highpc) __THROW;

/* Clean up profiling and write out gmon.out.  */
extern void _mcleanup (void) __THROW;

__END_DECLS

#endif /* sys/gmon.h */
