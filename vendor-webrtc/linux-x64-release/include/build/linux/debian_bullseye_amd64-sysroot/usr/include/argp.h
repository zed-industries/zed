/* Hierarchial argument parsing, layered over getopt.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Written by Miles Bader <miles@gnu.ai.mit.edu>.

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

#ifndef _ARGP_H
#define _ARGP_H

#include <stdio.h>
#include <ctype.h>
#include <getopt.h>
#include <limits.h>
#include <errno.h>

__BEGIN_DECLS

/* error_t may or may not be available from errno.h, depending on the
   operating system.  */
#ifndef __error_t_defined
# define __error_t_defined 1
typedef int error_t;
#endif

/* A description of a particular option.  A pointer to an array of
   these is passed in the OPTIONS field of an argp structure.  Each option
   entry can correspond to one long option and/or one short option; more
   names for the same option can be added by following an entry in an option
   array with options having the OPTION_ALIAS flag set.  */
struct argp_option
{
  /* The long option name.  For more than one name for the same option, you
     can use following options with the OPTION_ALIAS flag set.  */
  const char *name;

  /* What key is returned for this option.  If > 0 and printable, then it's
     also accepted as a short option.  */
  int key;

  /* If non-NULL, this is the name of the argument associated with this
     option, which is required unless the OPTION_ARG_OPTIONAL flag is set. */
  const char *arg;

  /* OPTION_ flags.  */
  int flags;

  /* The doc string for this option.  If both NAME and KEY are 0, This string
     will be printed outdented from the normal option column, making it
     useful as a group header (it will be the first thing printed in its
     group); in this usage, it's conventional to end the string with a `:'.  */
  const char *doc;

  /* The group this option is in.  In a long help message, options are sorted
     alphabetically within each group, and the groups presented in the order
     0, 1, 2, ..., n, -m, ..., -2, -1.  Every entry in an options array with
     if this field 0 will inherit the group number of the previous entry, or
     zero if it's the first one, unless its a group header (NAME and KEY both
     0), in which case, the previous entry + 1 is the default.  Automagic
     options such as --help are put into group -1.  */
  int group;
};

/* The argument associated with this option is optional.  */
#define OPTION_ARG_OPTIONAL	0x1

/* This option isn't displayed in any help messages.  */
#define OPTION_HIDDEN	       	0x2

/* This option is an alias for the closest previous non-alias option.  This
   means that it will be displayed in the same help entry, and will inherit
   fields other than NAME and KEY from the aliased option.  */
#define OPTION_ALIAS		0x4

/* This option isn't actually an option (and so should be ignored by the
   actual option parser), but rather an arbitrary piece of documentation that
   should be displayed in much the same manner as the options.  If this flag
   is set, then the option NAME field is displayed unmodified (e.g., no `--'
   prefix is added) at the left-margin (where a *short* option would normally
   be displayed), and the documentation string in the normal place.  For
   purposes of sorting, any leading whitespace and punctuation is ignored,
   except that if the first non-whitespace character is not `-', this entry
   is displayed after all options (and OPTION_DOC entries with a leading `-')
   in the same group.  */
#define OPTION_DOC		0x8

/* This option shouldn't be included in `long' usage messages (but is still
   included in help messages).  This is mainly intended for options that are
   completely documented in an argp's ARGS_DOC field, in which case including
   the option in the generic usage list would be redundant.  For instance,
   if ARGS_DOC is "FOO BAR\n-x BLAH", and the `-x' option's purpose is to
   distinguish these two cases, -x should probably be marked
   OPTION_NO_USAGE.  */
#define OPTION_NO_USAGE		0x10

struct argp;			/* fwd declare this type */
struct argp_state;		/* " */
struct argp_child;		/* " */

/* The type of a pointer to an argp parsing function.  */
typedef error_t (*argp_parser_t) (int __key, char *__arg,
				  struct argp_state *__state);

/* What to return for unrecognized keys.  For special ARGP_KEY_ keys, such
   returns will simply be ignored.  For user keys, this error will be turned
   into EINVAL (if the call to argp_parse is such that errors are propagated
   back to the user instead of exiting); returning EINVAL itself would result
   in an immediate stop to parsing in *all* cases.  */
#define ARGP_ERR_UNKNOWN	E2BIG /* Hurd should never need E2BIG.  XXX */

/* Special values for the KEY argument to an argument parsing function.
   ARGP_ERR_UNKNOWN should be returned if they aren't understood.

   The sequence of keys to a parsing function is either (where each
   uppercased word should be prefixed by `ARGP_KEY_' and opt is a user key):

       INIT opt... NO_ARGS END SUCCESS  -- No non-option arguments at all
   or  INIT (opt | ARG)... END SUCCESS  -- All non-option args parsed
   or  INIT (opt | ARG)... SUCCESS      -- Some non-option arg unrecognized

   The third case is where every parser returned ARGP_KEY_UNKNOWN for an
   argument, in which case parsing stops at that argument (returning the
   unparsed arguments to the caller of argp_parse if requested, or stopping
   with an error message if not).

   If an error occurs (either detected by argp, or because the parsing
   function returned an error value), then the parser is called with
   ARGP_KEY_ERROR, and no further calls are made.  */

/* This is not an option at all, but rather a command line argument.  If a
   parser receiving this key returns success, the fact is recorded, and the
   ARGP_KEY_NO_ARGS case won't be used.  HOWEVER, if while processing the
   argument, a parser function decrements the NEXT field of the state it's
   passed, the option won't be considered processed; this is to allow you to
   actually modify the argument (perhaps into an option), and have it
   processed again.  */
#define ARGP_KEY_ARG		0
/* There are remaining arguments not parsed by any parser, which may be found
   starting at (STATE->argv + STATE->next).  If success is returned, but
   STATE->next left untouched, it's assumed that all arguments were consume,
   otherwise, the parser should adjust STATE->next to reflect any arguments
   consumed.  */
#define ARGP_KEY_ARGS		0x1000006
/* There are no more command line arguments at all.  */
#define ARGP_KEY_END		0x1000001
/* Because it's common to want to do some special processing if there aren't
   any non-option args, user parsers are called with this key if they didn't
   successfully process any non-option arguments.  Called just before
   ARGP_KEY_END (where more general validity checks on previously parsed
   arguments can take place).  */
#define ARGP_KEY_NO_ARGS	0x1000002
/* Passed in before any parsing is done.  Afterwards, the values of each
   element of the CHILD_INPUT field, if any, in the state structure is
   copied to each child's state to be the initial value of the INPUT field.  */
#define ARGP_KEY_INIT		0x1000003
/* Use after all other keys, including SUCCESS & END.  */
#define ARGP_KEY_FINI		0x1000007
/* Passed in when parsing has successfully been completed (even if there are
   still arguments remaining).  */
#define ARGP_KEY_SUCCESS	0x1000004
/* Passed in if an error occurs.  */
#define ARGP_KEY_ERROR		0x1000005

/* An argp structure contains a set of options declarations, a function to
   deal with parsing one, documentation string, a possible vector of child
   argp's, and perhaps a function to filter help output.  When actually
   parsing options, getopt is called with the union of all the argp
   structures chained together through their CHILD pointers, with conflicts
   being resolved in favor of the first occurrence in the chain.  */
struct argp
{
  /* An array of argp_option structures, terminated by an entry with both
     NAME and KEY having a value of 0.  */
  const struct argp_option *options;

  /* What to do with an option from this structure.  KEY is the key
     associated with the option, and ARG is any associated argument (NULL if
     none was supplied).  If KEY isn't understood, ARGP_ERR_UNKNOWN should be
     returned.  If a non-zero, non-ARGP_ERR_UNKNOWN value is returned, then
     parsing is stopped immediately, and that value is returned from
     argp_parse().  For special (non-user-supplied) values of KEY, see the
     ARGP_KEY_ definitions below.  */
  argp_parser_t parser;

  /* A string describing what other arguments are wanted by this program.  It
     is only used by argp_usage to print the `Usage:' message.  If it
     contains newlines, the strings separated by them are considered
     alternative usage patterns, and printed on separate lines (lines after
     the first are prefix by `  or: ' instead of `Usage:').  */
  const char *args_doc;

  /* If non-NULL, a string containing extra text to be printed before and
     after the options in a long help message (separated by a vertical tab
     `\v' character).  */
  const char *doc;

  /* A vector of argp_children structures, terminated by a member with a 0
     argp field, pointing to child argps should be parsed with this one.  Any
     conflicts are resolved in favor of this argp, or early argps in the
     CHILDREN list.  This field is useful if you use libraries that supply
     their own argp structure, which you want to use in conjunction with your
     own.  */
  const struct argp_child *children;

  /* If non-zero, this should be a function to filter the output of help
     messages.  KEY is either a key from an option, in which case TEXT is
     that option's help text, or a special key from the ARGP_KEY_HELP_
     defines, below, describing which other help text TEXT is.  The function
     should return either TEXT, if it should be used as-is, a replacement
     string, which should be malloced, and will be freed by argp, or NULL,
     meaning `print nothing'.  The value for TEXT is *after* any translation
     has been done, so if any of the replacement text also needs translation,
     that should be done by the filter function.  INPUT is either the input
     supplied to argp_parse, or NULL, if argp_help was called directly.  */
  char *(*help_filter) (int __key, const char *__text, void *__input);

  /* If non-zero the strings used in the argp library are translated using
     the domain described by this string.  Otherwise the currently installed
     default domain is used.  */
  const char *argp_domain;
};

/* Possible KEY arguments to a help filter function.  */
#define ARGP_KEY_HELP_PRE_DOC	0x2000001 /* Help text preceeding options. */
#define ARGP_KEY_HELP_POST_DOC	0x2000002 /* Help text following options. */
#define ARGP_KEY_HELP_HEADER	0x2000003 /* Option header string. */
#define ARGP_KEY_HELP_EXTRA	0x2000004 /* After all other documentation;
					     TEXT is NULL for this key.  */
/* Explanatory note emitted when duplicate option arguments have been
   suppressed.  */
#define ARGP_KEY_HELP_DUP_ARGS_NOTE 0x2000005
#define ARGP_KEY_HELP_ARGS_DOC	0x2000006 /* Argument doc string.  */

/* When an argp has a non-zero CHILDREN field, it should point to a vector of
   argp_child structures, each of which describes a subsidiary argp.  */
struct argp_child
{
  /* The child parser.  */
  const struct argp *argp;

  /* Flags for this child.  */
  int flags;

  /* If non-zero, an optional header to be printed in help output before the
     child options.  As a side-effect, a non-zero value forces the child
     options to be grouped together; to achieve this effect without actually
     printing a header string, use a value of "".  */
  const char *header;

  /* Where to group the child options relative to the other (`consolidated')
     options in the parent argp; the values are the same as the GROUP field
     in argp_option structs, but all child-groupings follow parent options at
     a particular group level.  If both this field and HEADER are zero, then
     they aren't grouped at all, but rather merged with the parent options
     (merging the child's grouping levels with the parents).  */
  int group;
};

/* Parsing state.  This is provided to parsing functions called by argp,
   which may examine and, as noted, modify fields.  */
struct argp_state
{
  /* The top level ARGP being parsed.  */
  const struct argp *root_argp;

  /* The argument vector being parsed.  May be modified.  */
  int argc;
  char **argv;

  /* The index in ARGV of the next arg that to be parsed.  May be modified. */
  int next;

  /* The flags supplied to argp_parse.  May be modified.  */
  unsigned flags;

  /* While calling a parsing function with a key of ARGP_KEY_ARG, this is the
     number of the current arg, starting at zero, and incremented after each
     such call returns.  At all other times, this is the number of such
     arguments that have been processed.  */
  unsigned arg_num;

  /* If non-zero, the index in ARGV of the first argument following a special
     `--' argument (which prevents anything following being interpreted as an
     option).  Only set once argument parsing has proceeded past this point. */
  int quoted;

  /* An arbitrary pointer passed in from the user.  */
  void *input;
  /* Values to pass to child parsers.  This vector will be the same length as
     the number of children for the current parser.  */
  void **child_inputs;

  /* For the parser's use.  Initialized to 0.  */
  void *hook;

  /* The name used when printing messages.  This is initialized to ARGV[0],
     or PROGRAM_INVOCATION_NAME if that is unavailable.  */
  char *name;

  /* Streams used when argp prints something.  */
  FILE *err_stream;		/* For errors; initialized to stderr. */
  FILE *out_stream;		/* For information; initialized to stdout. */

  void *pstate;			/* Private, for use by argp.  */
};

/* Flags for argp_parse (note that the defaults are those that are
   convenient for program command line parsing): */

/* Don't ignore the first element of ARGV.  Normally (and always unless
   ARGP_NO_ERRS is set) the first element of the argument vector is
   skipped for option parsing purposes, as it corresponds to the program name
   in a command line.  */
#define ARGP_PARSE_ARGV0  0x01

/* Don't print error messages for unknown options to stderr; unless this flag
   is set, ARGP_PARSE_ARGV0 is ignored, as ARGV[0] is used as the program
   name in the error messages.  This flag implies ARGP_NO_EXIT (on the
   assumption that silent exiting upon errors is bad behaviour).  */
#define ARGP_NO_ERRS	0x02

/* Don't parse any non-option args.  Normally non-option args are parsed by
   calling the parse functions with a key of ARGP_KEY_ARG, and the actual arg
   as the value.  Since it's impossible to know which parse function wants to
   handle it, each one is called in turn, until one returns 0 or an error
   other than ARGP_ERR_UNKNOWN; if an argument is handled by no one, the
   argp_parse returns prematurely (but with a return value of 0).  If all
   args have been parsed without error, all parsing functions are called one
   last time with a key of ARGP_KEY_END.  This flag needn't normally be set,
   as the normal behavior is to stop parsing as soon as some argument can't
   be handled.  */
#define ARGP_NO_ARGS	0x04

/* Parse options and arguments in the same order they occur on the command
   line -- normally they're rearranged so that all options come first. */
#define ARGP_IN_ORDER	0x08

/* Don't provide the standard long option --help, which causes usage and
      option help information to be output to stdout, and exit (0) called. */
#define ARGP_NO_HELP	0x10

/* Don't exit on errors (they may still result in error messages).  */
#define ARGP_NO_EXIT	0x20

/* Use the gnu getopt `long-only' rules for parsing arguments.  */
#define ARGP_LONG_ONLY	0x40

/* Turns off any message-printing/exiting options.  */
#define ARGP_SILENT    (ARGP_NO_EXIT | ARGP_NO_ERRS | ARGP_NO_HELP)

/* Parse the options strings in ARGC & ARGV according to the options in ARGP.
   FLAGS is one of the ARGP_ flags above.  If ARG_INDEX is non-NULL, the
   index in ARGV of the first unparsed option is returned in it.  If an
   unknown option is present, ARGP_ERR_UNKNOWN is returned; if some parser
   routine returned a non-zero value, it is returned; otherwise 0 is
   returned.  This function may also call exit unless the ARGP_NO_HELP flag
   is set.  INPUT is a pointer to a value to be passed in to the parser.  */
extern error_t argp_parse (const struct argp *__restrict __argp,
			   int __argc, char **__restrict __argv,
			   unsigned __flags, int *__restrict __arg_index,
			   void *__restrict __input);
extern error_t __argp_parse (const struct argp *__restrict __argp,
			     int __argc, char **__restrict __argv,
			     unsigned __flags, int *__restrict __arg_index,
			     void *__restrict __input);

/* Global variables.  */

/* If defined or set by the user program to a non-zero value, then a default
   option --version is added (unless the ARGP_NO_HELP flag is used), which
   will print this string followed by a newline and exit (unless the
   ARGP_NO_EXIT flag is used).  Overridden by ARGP_PROGRAM_VERSION_HOOK.  */
extern const char *argp_program_version;

/* If defined or set by the user program to a non-zero value, then a default
   option --version is added (unless the ARGP_NO_HELP flag is used), which
   calls this function with a stream to print the version to and a pointer to
   the current parsing state, and then exits (unless the ARGP_NO_EXIT flag is
   used).  This variable takes precedent over ARGP_PROGRAM_VERSION.  */
extern void (*argp_program_version_hook) (FILE *__restrict __stream,
					  struct argp_state *__restrict
					  __state);

/* If defined or set by the user program, it should point to string that is
   the bug-reporting address for the program.  It will be printed by
   argp_help if the ARGP_HELP_BUG_ADDR flag is set (as it is by various
   standard help messages), embedded in a sentence that says something like
   `Report bugs to ADDR.'.  */
extern const char *argp_program_bug_address;

/* The exit status that argp will use when exiting due to a parsing error.
   If not defined or set by the user program, this defaults to EX_USAGE from
   <sysexits.h>.  */
extern error_t argp_err_exit_status;

/* Flags for argp_help.  */
#define ARGP_HELP_USAGE		0x01 /* a Usage: message. */
#define ARGP_HELP_SHORT_USAGE	0x02 /*  " but don't actually print options. */
#define ARGP_HELP_SEE		0x04 /* a `Try ... for more help' message. */
#define ARGP_HELP_LONG		0x08 /* a long help message. */
#define ARGP_HELP_PRE_DOC	0x10 /* doc string preceding long help.  */
#define ARGP_HELP_POST_DOC	0x20 /* doc string following long help.  */
#define ARGP_HELP_DOC		(ARGP_HELP_PRE_DOC | ARGP_HELP_POST_DOC)
#define ARGP_HELP_BUG_ADDR	0x40 /* bug report address */
#define ARGP_HELP_LONG_ONLY	0x80 /* modify output appropriately to
					reflect ARGP_LONG_ONLY mode.  */

/* These ARGP_HELP flags are only understood by argp_state_help.  */
#define ARGP_HELP_EXIT_ERR	0x100 /* Call exit(1) instead of returning.  */
#define ARGP_HELP_EXIT_OK	0x200 /* Call exit(0) instead of returning.  */

/* The standard thing to do after a program command line parsing error, if an
   error message has already been printed.  */
#define ARGP_HELP_STD_ERR \
  (ARGP_HELP_SEE | ARGP_HELP_EXIT_ERR)
/* The standard thing to do after a program command line parsing error, if no
   more specific error message has been printed.  */
#define ARGP_HELP_STD_USAGE \
  (ARGP_HELP_SHORT_USAGE | ARGP_HELP_SEE | ARGP_HELP_EXIT_ERR)
/* The standard thing to do in response to a --help option.  */
#define ARGP_HELP_STD_HELP \
  (ARGP_HELP_SHORT_USAGE | ARGP_HELP_LONG | ARGP_HELP_EXIT_OK \
   | ARGP_HELP_DOC | ARGP_HELP_BUG_ADDR)

/* Output a usage message for ARGP to STREAM.  FLAGS are from the set
   ARGP_HELP_*.  */
extern void argp_help (const struct argp *__restrict __argp,
		       FILE *__restrict __stream,
		       unsigned __flags, char *__restrict __name);
extern void __argp_help (const struct argp *__restrict __argp,
			 FILE *__restrict __stream, unsigned __flags,
			 char *__name);

/* The following routines are intended to be called from within an argp
   parsing routine (thus taking an argp_state structure as the first
   argument).  They may or may not print an error message and exit, depending
   on the flags in STATE -- in any case, the caller should be prepared for
   them *not* to exit, and should return an appropiate error after calling
   them.  [argp_usage & argp_error should probably be called argp_state_...,
   but they're used often enough that they should be short]  */

/* Output, if appropriate, a usage message for STATE to STREAM.  FLAGS are
   from the set ARGP_HELP_*.  */
extern void argp_state_help (const struct argp_state *__restrict __state,
			     FILE *__restrict __stream,
			     unsigned int __flags);
extern void __argp_state_help (const struct argp_state *__restrict __state,
			       FILE *__restrict __stream,
			       unsigned int __flags);

/* Possibly output the standard usage message for ARGP to stderr and exit.  */
extern void argp_usage (const struct argp_state *__state);
extern void __argp_usage (const struct argp_state *__state);

/* If appropriate, print the printf string FMT and following args, preceded
   by the program name and `:', to stderr, and followed by a `Try ... --help'
   message, then exit (1).  */
extern void argp_error (const struct argp_state *__restrict __state,
			const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 2, 3)));
extern void __argp_error (const struct argp_state *__restrict __state,
			  const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 2, 3)));

/* Similar to the standard gnu error-reporting function error(), but will
   respect the ARGP_NO_EXIT and ARGP_NO_ERRS flags in STATE, and will print
   to STATE->err_stream.  This is useful for argument parsing code that is
   shared between program startup (when exiting is desired) and runtime
   option parsing (when typically an error code is returned instead).  The
   difference between this function and argp_error is that the latter is for
   *parsing errors*, and the former is for other problems that occur during
   parsing but don't reflect a (syntactic) problem with the input.  */
extern void argp_failure (const struct argp_state *__restrict __state,
			  int __status, int __errnum,
			  const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 4, 5)));
extern void __argp_failure (const struct argp_state *__restrict __state,
			    int __status, int __errnum,
			    const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 4, 5)));

/* Returns true if the option OPT is a valid short option.  */
extern int _option_is_short (const struct argp_option *__opt) __THROW;
extern int __option_is_short (const struct argp_option *__opt) __THROW;

/* Returns true if the option OPT is in fact the last (unused) entry in an
   options array.  */
extern int _option_is_end (const struct argp_option *__opt) __THROW;
extern int __option_is_end (const struct argp_option *__opt) __THROW;

/* Return the input field for ARGP in the parser corresponding to STATE; used
   by the help routines.  */
extern void *_argp_input (const struct argp *__restrict __argp,
			  const struct argp_state *__restrict __state)
     __THROW;
extern void *__argp_input (const struct argp *__restrict __argp,
			   const struct argp_state *__restrict __state)
     __THROW;

#ifdef __USE_EXTERN_INLINES

# if !(defined _LIBC && _LIBC)
#  define __argp_usage argp_usage
#  define __argp_state_help argp_state_help
#  define __option_is_short _option_is_short
#  define __option_is_end _option_is_end
# endif

# ifndef ARGP_EI
#  define ARGP_EI __extern_inline
# endif

ARGP_EI void
__argp_usage (const struct argp_state *__state)
{
  __argp_state_help (__state, stderr, ARGP_HELP_STD_USAGE);
}

ARGP_EI int
__NTH (__option_is_short (const struct argp_option *__opt))
{
  if (__opt->flags & OPTION_DOC)
    return 0;
  else
    {
      int __key = __opt->key;
      return __key > 0 && __key <= UCHAR_MAX && isprint (__key);
    }
}

ARGP_EI int
__NTH (__option_is_end (const struct argp_option *__opt))
{
  return !__opt->key && !__opt->name && !__opt->doc && !__opt->group;
}

# if !(defined _LIBC && _LIBC)
#  undef __argp_usage
#  undef __argp_state_help
#  undef __option_is_short
#  undef __option_is_end
# endif
#endif /* Use extern inlines.  */

#ifdef __LDBL_COMPAT
# include <bits/argp-ldbl.h>
#endif

__END_DECLS

#endif /* argp.h */
