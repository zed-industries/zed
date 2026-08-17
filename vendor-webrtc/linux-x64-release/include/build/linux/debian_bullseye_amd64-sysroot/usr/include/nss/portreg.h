/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * shexp.h: Defines and prototypes for shell exp. match routines
 *
 * This routine will match a string with a shell expression. The expressions
 * accepted are based loosely on the expressions accepted by zsh.
 *
 * o * matches anything
 * o ? matches one character
 * o \ will escape a special character
 * o $ matches the end of the string
 * Bracketed expressions:
 * o [abc] matches one occurence of a, b, or c.
 * o [^abc] matches any character except a, b, or c.
 *     To be matched between [ and ], these characters must be escaped: \ ]
 *     No other characters need be escaped between brackets.
 *     Unnecessary escaping is permitted.
 * o [a-z] matches any character between a and z, inclusive.
 *     The two range-definition characters must be alphanumeric ASCII.
 *     If one is upper case and the other is lower case, then the ASCII
 *     non-alphanumeric characters between Z and a will also be in range.
 * o [^a-z] matches any character except those between a and z, inclusive.
 *     These forms cannot be combined, e.g [a-gp-z] does not work.
 * o Exclusions:
 *   As a top level, outter-most expression only, the expression
 *   foo~bar will match the expression foo, provided it does not also
 *     match the expression bar.  Either expression or both may be a union.
 *     Except between brackets, any unescaped ~ is an exclusion.
 *     At most one exclusion is permitted.
 *     Exclusions cannot be nested (contain other exclusions).
 *     example: *~abc will match any string except abc
 * o Unions:
 *   (foo|bar) will match either the expression foo, or the expression bar.
 *     At least one '|' separator is required.  More are permitted.
 *     Expressions inside unions may not include unions or exclusions.
 *     Inside a union, to be matched and not treated as a special character,
 *     these characters must be escaped: \ ( | ) [ ~ except when they occur
 *     inside a bracketed expression, where only \ and ] require escaping.
 *
 * The public interface to these routines is documented below.
 *
 */

#ifndef SHEXP_H
#define SHEXP_H

#include "utilrename.h"
/*
 * Requires that the macro MALLOC be set to a "safe" malloc that will
 * exit if no memory is available.
 */

/* --------------------------- Public routines ---------------------------- */

/*
 * shexp_valid takes a shell expression exp as input. It returns:
 *
 *  NON_SXP      if exp is a standard string
 *  INVALID_SXP  if exp is a shell expression, but invalid
 *  VALID_SXP    if exp is a valid shell expression
 */

#define NON_SXP -1
#define INVALID_SXP -2
#define VALID_SXP 1

SEC_BEGIN_PROTOS

extern int PORT_RegExpValid(const char *exp);

extern int PORT_RegExpSearch(const char *str, const char *exp);

/* same as above but uses case insensitive search */
extern int PORT_RegExpCaseSearch(const char *str, const char *exp);

SEC_END_PROTOS

#endif
