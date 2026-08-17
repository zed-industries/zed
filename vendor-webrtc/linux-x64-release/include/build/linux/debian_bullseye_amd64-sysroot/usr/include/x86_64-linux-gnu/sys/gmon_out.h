/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by David Mosberger <davidm@cs.arizona.edu>.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

/* This file specifies the format of gmon.out files.  It should have
   as few external dependencies as possible as it is going to be included
   in many different programs.  That is, minimize the number of #include's.

   A gmon.out file consists of a header (defined by gmon_hdr) followed by
   a sequence of records.  Each record starts with a one-byte tag
   identifying the type of records, followed by records specific data. */

#ifndef _SYS_GMON_OUT_H
#define _SYS_GMON_OUT_H	1

#include <features.h>

#define	GMON_MAGIC	"gmon"	/* magic cookie */
#define GMON_VERSION	1	/* version number */

/* For profiling shared object we need a new format.  */
#define GMON_SHOBJ_VERSION	0x1ffff

__BEGIN_DECLS

/*
 * Raw header as it appears on file (without padding).  This header
 * always comes first in gmon.out and is then followed by a series
 * records defined below.
 */
struct gmon_hdr
  {
    char cookie[4];
    char version[4];
    char spare[3 * 4];
  };

/* types of records in this file: */
typedef enum
  {
    GMON_TAG_TIME_HIST = 0,
    GMON_TAG_CG_ARC = 1,
    GMON_TAG_BB_COUNT = 2
  } GMON_Record_Tag;

struct gmon_hist_hdr
  {
    char low_pc[sizeof (char *)];	/* base pc address of sample buffer */
    char high_pc[sizeof (char *)];	/* max pc address of sampled buffer */
    char hist_size[4];			/* size of sample buffer */
    char prof_rate[4];			/* profiling clock rate */
    char dimen[15];			/* phys. dim., usually "seconds" */
    char dimen_abbrev;			/* usually 's' for "seconds" */
  };

struct gmon_cg_arc_record
  {
    char from_pc[sizeof (char *)];	/* address within caller's body */
    char self_pc[sizeof (char *)];	/* address within callee's body */
    char count[4];			/* number of arc traversals */
  };

__END_DECLS

#endif /* sys/gmon_out.h */
