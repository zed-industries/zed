
/*--------------------------------------------------------------------*/
/*--- Command line options.                     pub_tool_options.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2000-2017 Julian Seward
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

#ifndef __PUB_TOOL_OPTIONS_H
#define __PUB_TOOL_OPTIONS_H

#include "pub_tool_basics.h"     // for VG_ macro
#include "libvex.h"              // for VexControl

// Command line option parsing happens in the following modes:
//   cloE : Early processing, used by coregrind m_main.c to parse the
//      command line  options that must be handled early on.
//   cloP : Processing,  used by coregrind and tools during startup, when
//      doing command line options Processing.
//   clodD : Dynamic, used to dynamically change options after startup.
//      A subset of the command line options can be changed dynamically
//      after startup.
//   cloH : Help, special mode to produce the list of dynamically changeable
//      options for --help-dyn-options.
typedef
   enum {
      cloE = 1,
      cloP = 2,
      cloD = 4,
      cloH = 8
   } Clo_Mode;

// Defines often used mode sets, e.g. for options used in several modes.
#define cloEP (cloE | cloP)
#define cloED (cloE | cloD)
#define cloPD (cloP | cloD)

// Sets and gets the current option parsing mode.
// VG_(set_Clo_Mode) also resets the value of VG_(Clo_Recognised) to False.
void VG_(set_Clo_Mode) (Clo_Mode mode);

Clo_Mode VG_(Clo_Mode) (void);

// This is called by the various macros below to indicate that
// the currently parsed option has been recognised.
void VG_(set_Clo_Recognised) (void);
Bool VG_(Clo_Recognised) (void);


/* Once the system is started up, the dynamic options can be changed
   (mode CloD) or listed (mode cloH) using the below.  */
void VG_(process_dynamic_option) (Clo_Mode mode, HChar *value);

// Macros below are calling VG_(check_clom) to handle an option according
// to the current Clo_Mode.
// If recognised, it marks the option as recognised, and then returns True
// if the current mode matches the modes allowed for this option,
// False if option should not be processed according to current mode
// and qq_mode.
// It produces a warning if mode is cloD and cloD is not allowed by
// modes.  If current mode is cloH, CHECK_CLOM calls VG_(list_clo) if cloD
// is allowed by modes.
Bool VG_(check_clom) (Clo_Mode modes, const HChar* arg, const HChar* option,
                      Bool recognised);

// Higher-level command-line option recognisers;  use in if/else chains. 
// Note that they assign a value to the 'qq_var' argument.  So often they
// can be used like this:
//
//   if VG_STR_CLO(arg, "--foo", clo_foo) { }
//
// But if you want to do further checking or processing, you can do this:
//
//   if VG_STR_CLO(arg, "--foo", clo_foo) { <checking or processing> }
//
// The recognisers above are only modifying their argument in the relevant
// parsing mode (by default, only during cloP mode).
// If an option is handled during other modes than cloP, use the *M
// variants of the recognisers to specify the mode.
//
// They use GNU statement expressions to do the qq_var assignment within a
// conditional expression.

// Used to produce the list of dynamically changeable options.
extern void VG_(list_clo)(const HChar *qq_option);

// True if current option parsing mode matches qq_mode
// and the first qq_len characters of qq_arg match qq_option.
#define VG_STREQN_CLOM(qq_mode, qq_len, qq_arg, qq_option) \
   (VG_(check_clom)                                        \
    (qq_mode, qq_arg, qq_option,                           \
     VG_STREQN(qq_len, qq_arg, qq_option)))

// True if current option parsing mode matches qq_mode
// and qq_arg match qq_option.
#define VG_STREQ_CLOM(qq_mode, qq_arg, qq_option)       \
   (VG_(check_clom)                                     \
    (qq_mode, qq_arg, qq_option,                        \
     VG_STREQ(qq_arg, qq_option)))

// String argument, eg. --foo=yes or --foo=no
#define VG_BOOL_CLOM(qq_mode, qq_arg, qq_option, qq_var)        \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
      const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      if      VG_STREQ(val, "yes") (qq_var) = True;                     \
      else if VG_STREQ(val, "no")  (qq_var) = False;                    \
      else {VG_(fmsg_bad_option)(qq_arg, "Invalid boolean value '%s'"   \
                                        " (should be 'yes' or 'no')\n", \
       /* gcc 10 (20200119) complains that |val| could be null here. */ \
       /* I think it is wrong, but anyway, to placate it .. */          \
                                        (val ? val : "(null)"));        \
         res = False; }                                                 \
      res; }))

#define VG_BOOL_CLO(qq_arg, qq_option, qq_var) \
   VG_BOOL_CLOM(cloP, qq_arg, qq_option, qq_var)

// String argument, eg. --foo=bar
#define VG_STR_CLOM(qq_mode, qq_arg, qq_option, qq_var)                 \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      (qq_var) = val;                                                   \
      True; }))

#define VG_STR_CLO(qq_arg, qq_option, qq_var) \
   VG_STR_CLOM(cloP, qq_arg, qq_option, qq_var)

// UInt enum set arg, eg. --foo=fubar,bar,baz or --foo=none
// or --foo=all  (if qq_all is True)
#define VG_USETGEN_CLOM(qq_mode, qq_arg, qq_option, qq_vals, qq_var, qq_all) \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
       const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];        \
      if (!VG_(parse_enum_set)(qq_vals,                                 \
                               qq_all,/*allow_all*/                     \
                               val,                                     \
                               &(qq_var))) {                            \
         VG_(fmsg_bad_option)(qq_arg, "%s is an invalid %s set\n",      \
                              val, qq_option+2);                        \
         res = False; }                                                 \
      res; }))

// UInt enum set arg, eg. --foo=fubar,bar,baz or --foo=none or --foo=all
#define VG_USET_CLO(qq_arg, qq_option, qq_vals, qq_var) \
   VG_USETGEN_CLOM(cloP, (qq_arg), qq_option, (qq_vals), (qq_var), True)
#define VG_USET_CLOM(qq_mode, qq_arg, qq_option, qq_vals, qq_var) \
   VG_USETGEN_CLOM(qq_mode, (qq_arg), qq_option, (qq_vals), (qq_var), True)

/* Same as VG_USET_CLO but not allowing --foo=all.
   To be used when some or all of the enum set are mutually eXclusive. */
#define VG_USETX_CLO(qq_arg, qq_option, qq_vals, qq_var) \
   VG_USETGEN_CLOM(cloP, (qq_arg), qq_option, (qq_vals), (qq_var), False)
#define VG_USETX_CLOM(qq_mode, qq_arg, qq_option, qq_vals, qq_var) \
   VG_USETGEN_CLOM(qq_mode, (qq_arg), qq_option, (qq_vals), (qq_var), False)

// Unbounded integer arg, eg. --foo=10
#define VG_INT_CLOM(qq_mode, qq_arg, qq_option, qq_var)                 \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
      const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      HChar* s;                                                         \
      Long n = VG_(strtoll10)( val, &s );                               \
      (qq_var) = n;                                                     \
      /* Check for non-numeralness, or overflow. */                     \
      if ('\0' != s[0] || (qq_var) != n) {                              \
         VG_(fmsg_bad_option)(qq_arg,                                   \
                              "Invalid integer value '%s'\n", val);     \
         res = False; }                                                 \
      res; }))

#define VG_INT_CLO(qq_arg, qq_option, qq_var) \
   VG_INT_CLOM(cloP, qq_arg, qq_option, qq_var)

// Bounded integer arg, eg. --foo=10 ;  if the value exceeds the bounds it
// causes an abort.  'qq_base' can be 10 or 16.
#define VG_BINTN_CLOM(qq_mode, qq_base, qq_arg, qq_option, qq_var, qq_lo, qq_hi) \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
      const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      HChar* s;                                                         \
      Long n = VG_(strtoll##qq_base)( val, &s );                        \
      (qq_var) = n;                                                     \
      /* MMM: separate the two cases, and explain the problem;  likewise */ \
      /* for all the other macros in this file. */                      \
      /* Check for non-numeralness, or overflow. */                     \
      /* Nb: it will overflow if qq_var is unsigned and qq_val is negative! */ \
      if ('\0' != s[0] || (qq_var) != n) {                              \
         VG_(fmsg_bad_option)(qq_arg,                                   \
                              "Invalid integer value '%s'\n", val);     \
         res = False; }                                                 \
      /* Check bounds. */                                               \
      if ((qq_var) < (qq_lo) || (qq_var) > (qq_hi)) {                   \
         VG_(fmsg_bad_option)(qq_arg,                                   \
            "'%s' argument must be between %lld and %lld\n",            \
                              (qq_option), (Long)(qq_lo), (Long)(qq_hi)); \
         res = False;                                                  \
      }                                                                \
      res;}))

// Bounded decimal integer arg, eg. --foo=100
#define VG_BINT_CLO(qq_arg, qq_option, qq_var, qq_lo, qq_hi) \
   VG_BINTN_CLOM(cloP, 10, (qq_arg), qq_option, (qq_var), (qq_lo), (qq_hi))
#define VG_BINT_CLOM(qq_mode, qq_arg, qq_option, qq_var, qq_lo, qq_hi) \
   VG_BINTN_CLOM(qq_mode, 10, (qq_arg), qq_option, (qq_var), (qq_lo), (qq_hi))

// Bounded hexadecimal integer arg, eg. --foo=0x1fa8
#define VG_BHEX_CLO(qq_arg, qq_option, qq_var, qq_lo, qq_hi) \
   VG_BINTN_CLOM(cloP, 16, (qq_arg), qq_option, (qq_var), (qq_lo), (qq_hi))

// Double (decimal) arg, eg. --foo=4.6
// XXX: there's not VG_BDBL_CLO because we don't have a good way of printing
// floats at the moment!
#define VG_DBL_CLOM(qq_mode, qq_arg, qq_option, qq_var) \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
      const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      HChar* s;                                                         \
      double n = VG_(strtod)( val, &s );                                \
      (qq_var) = n;                                                     \
      /* Check for non-numeralness */                                   \
      if ('\0' != s[0]) {                                               \
         VG_(fmsg_bad_option)(qq_arg,                                   \
                              "Invalid floating point value '%s'\n",val); \
         res = False; }                                                 \
      res;}))

#define VG_DBL_CLO( qq_arg, qq_option, qq_var) \
   VG_DBL_CLOM(cloP, qq_arg, qq_option, qq_var)

// Arg whose value is denoted by the exact presence of the given string;
// if it matches, qq_var is assigned the value in qq_val.
#define VG_XACT_CLOM(qq_mode, qq_arg, qq_option, qq_var, qq_val) \
   (VG_(check_clom) \
    (qq_mode, qq_arg, qq_option,                                \
     VG_STREQ((qq_arg), (qq_option))) &&                        \
    ({(qq_var) = (qq_val);                                      \
       True; }))

#define VG_XACT_CLO(qq_arg, qq_option, qq_var, qq_val) \
   VG_XACT_CLOM(cloP, qq_arg, qq_option, qq_var, qq_val)

// Arg that can be one of a set of strings, as specified in an NULL
// terminated array.  Returns the index of the string in |qq_ix|, or
// aborts if not found.
#define VG_STRINDEX_CLOM(qq_mode, qq_arg, qq_option, qq_strings, qq_ix) \
   (VG_(check_clom)                                                     \
    (qq_mode, qq_arg, qq_option,                                        \
     VG_STREQN(VG_(strlen)(qq_option)+1, qq_arg, qq_option"=")) &&      \
    ({Bool res = True;                                                  \
      const HChar* val = &(qq_arg)[ VG_(strlen)(qq_option)+1 ];         \
      for (qq_ix = 0; (qq_strings)[qq_ix]; qq_ix++) {                   \
         if (VG_STREQ(val, (qq_strings)[qq_ix]))                        \
            break;                                                      \
      }                                                                 \
      if ((qq_strings)[qq_ix] == NULL) {                                \
         VG_(fmsg_bad_option)(qq_arg,                                   \
                              "Invalid string '%s' in '%s'\n", val, qq_arg); \
         res = False;                                                   \
      } \
      res; }))

#define VG_STRINDEX_CLO(qq_arg, qq_option, qq_strings, qq_ix) \
   VG_STRINDEX_CLOM(cloP, qq_arg, qq_option, qq_strings, qq_ix)

/* Verbosity level: 0 = silent, 1 (default), > 1 = more verbose. */
extern Int  VG_(clo_verbosity);

/* Show tool and core statistics */
extern Bool VG_(clo_stats);

/* wait for vgdb/gdb after reporting that amount of error.
   Note that this value can be changed dynamically. */
extern Int VG_(clo_vgdb_error);

/* If user has provided the --vgdb-prefix command line option,
   VG_(arg_vgdb_prefix) points at the provided argument (including the
   '--vgdb-prefix=' string).
   Otherwise, it is NULL.
   Typically, this is used by tools to produce user message with the
   expected vgdb prefix argument, if the user has changed the default. */
extern const HChar *VG_(arg_vgdb_prefix);

/* Emit all messages as XML? default: NO */
/* If clo_xml is set, various other options are set in a non-default
   way.  See vg_main.c and mc_main.c. */
extern Bool VG_(clo_xml);

/* An arbitrary user-supplied string which is copied into the
   XML output, in between <usercomment> tags. */
extern const HChar* VG_(clo_xml_user_comment);

/* Vex iropt control.  Tool-visible so tools can make Vex optimise
   less aggressively if that is needed (callgrind needs this). */
extern VexControl VG_(clo_vex_control);
extern VexRegisterUpdates VG_(clo_px_file_backed);

extern Int VG_(clo_redzone_size);

typedef
   enum {
      Vg_XTMemory_None,   // Do not do any xtree memory profiling.
      Vg_XTMemory_Allocs, // Currently allocated size xtree memory profiling
      Vg_XTMemory_Full,   // Full profiling : Current allocated size, total
      // allocated size, nr of blocks, total freed size, ...
   }
   VgXTMemory;
// Tools that replace malloc can optionally implement memory profiling
// following the value of VG_(clo_xtree_profile_memory) to produce a report
// at the end of execution.
extern VgXTMemory VG_(clo_xtree_memory);
/* Holds the filename to use for xtree memory profiling output, before expansion
   of %p and %q templates. */
extern const HChar* VG_(clo_xtree_memory_file);
/* Compress strings in xtree dumps. */
extern Bool VG_(clo_xtree_compress_strings);

/* Number of parents of a backtrace.  Default: 12  */
extern Int   VG_(clo_backtrace_size);

/* Continue stack traces below main()?  Default: NO */
extern Bool VG_(clo_show_below_main);

/* Keep symbols (and all other debuginfo) for code that is unloaded (dlclose
   or similar) so that stack traces can still give line/file info for
   previously captured stack traces.  e.g. ... showing where a block was
   allocated e.g. leaks of or accesses just outside a block. */
extern Bool VG_(clo_keep_debuginfo);


/* Used to expand file names.  "option_name" is the option name, eg.
   "--log-file".  'format' is what follows, eg. "cachegrind.out.%p".  In
   'format':
   - "%p" is replaced with PID.
   - "%q{QUAL}" is replaced with the environment variable $QUAL.  If $QUAL
     isn't set, we abort.  If the "{QUAL}" part is malformed, we abort.
   - "%%" is replaced with "%".
   Anything else after '%' causes an abort.
   If the format specifies a relative file name, it's put in the program's
   initial working directory.  If it specifies an absolute file name (ie.
   starts with '/') then it is put there.

   Note that "option_name" has no effect on the returned string: the
   returned string depends only on "format" and the PIDs and
   environment variables that it references (if any). "option_name" is
   merely used in printing error messages, if an error message needs
   to be printed due to malformedness of the "format" argument.
*/
extern HChar* VG_(expand_file_name)(const HChar* option_name,
                                    const HChar* format);

#endif   // __PUB_TOOL_OPTIONS_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
