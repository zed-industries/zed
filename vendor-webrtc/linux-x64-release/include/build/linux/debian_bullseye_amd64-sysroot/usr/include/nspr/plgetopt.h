/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:          plgetopt.h
** Description:   utilities to parse argc/argv
*/

#if defined(PLGETOPT_H_)
#else
#define PLGETOPT_H_

#include "prtypes.h"

PR_BEGIN_EXTERN_C

typedef struct PLOptionInternal PLOptionInternal;

typedef enum
{
    PL_OPT_OK,              /* all's well with the option */
    PL_OPT_EOL,             /* end of options list */
    PL_OPT_BAD              /* invalid option (and value) */
} PLOptStatus;

typedef struct PLLongOpt
{
    const char * longOptName;   /* long option name string                  */
    PRIntn       longOption;    /* value put in PLOptState for this option. */
    PRBool       valueRequired; /* If option name not followed by '=',      */
    /* value is the next argument from argv.    */
} PLLongOpt;

typedef struct PLOptState
{
    char option;                /* the name of the option */
    const char *value;          /* the value of that option | NULL */

    PLOptionInternal *internal; /* private processing state */

    PRIntn   longOption;        /* value from PLLongOpt put here */
    PRIntn   longOptIndex;      /* index into caller's array of PLLongOpts */
} PLOptState;

/*
 * PL_CreateOptState
 *
 * The argument "options" points to a string of single-character option
 * names.  Option names that may have an option argument value must be
 * followed immediately by a ':' character.
 */
PR_EXTERN(PLOptState*) PL_CreateOptState(
    PRIntn argc, char **argv, const char *options);

/*
 * PL_CreateLongOptState
 *
 * Alternative to PL_CreateOptState.
 * Allows caller to specify BOTH a string of single-character option names,
 * AND an array of structures describing "long" (keyword) option names.
 * The array is terminated by a structure in which longOptName is NULL.
 * Long option values (arguments) may always be given as "--name=value".
 * If PLLongOpt.valueRequired is not PR_FALSE, and the option name was not
 * followed by '=' then the next argument from argv is taken as the value.
 */
PR_EXTERN(PLOptState*) PL_CreateLongOptState(
    PRIntn argc, char **argv, const char *options,
    const PLLongOpt *longOpts);
/*
 * PL_DestroyOptState
 *
 * Call this to destroy the PLOptState returned from PL_CreateOptState or
 * PL_CreateLongOptState.
 */
PR_EXTERN(void) PL_DestroyOptState(PLOptState *opt);

/*
 * PL_GetNextOpt
 *
 * When this function returns PL_OPT_OK,
 * - opt->option will hold the single-character option name that was parsed,
 *   or zero.
 * When opt->option is zero, the token parsed was either a "long" (keyword)
 *   option or a positional parameter.
 * For a positional parameter,
 * - opt->longOptIndex will contain -1, and
 * - opt->value will point to the positional parameter string.
 * For a long option name,
 * - opt->longOptIndex will contain the non-negative index of the
 *   PLLongOpt structure in the caller's array of PLLongOpt structures
 *   corresponding to the long option name, and
 * For a single-character or long option,
 * - opt->longOption will contain the value of the single-character option
 *   name, or the value of the longOption from the PLLongOpt structure
 *   for that long option.  See notes below.
 * - opt->value will point to the argument option string, or will
 *   be NULL if option does not require argument.  If option requires
 *   argument but it is not provided, PL_OPT_BAD is returned.
 * When opt->option is non-zero,
 * - opt->longOptIndex will be -1
 * When this function returns PL_OPT_EOL, or PL_OPT_BAD, the contents of
 *   opt are undefined.
 *
 * Notes: It is possible to ignore opt->option, and always look at
 *   opt->longOption instead.  opt->longOption will contain the same value
 *   as opt->option for single-character option names, and will contain the
 *   value of longOption from the PLLongOpt structure for long option names.
 * This means that it is possible to equivalence long option names to
 *   single character names by giving the longOption in the PLLongOpt struct
 *   the same value as the single-character option name.
 * For long options that are NOT intended to be equivalent to any single-
 *   character option, the longOption value should be chosen to not match
 *   any possible single character name.  It might be advisable to choose
 *   longOption values greater than 0xff for such long options.
 */
PR_EXTERN(PLOptStatus) PL_GetNextOpt(PLOptState *opt);

PR_END_EXTERN_C

#endif /* defined(PLGETOPT_H_) */

/* plgetopt.h */

