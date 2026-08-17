/*    handy.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1999, 2000,
 *    2001, 2002, 2004, 2005, 2006, 2007, 2008, 2012 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/* IMPORTANT NOTE: Everything whose name begins with an underscore is for
 * internal core Perl use only. */

#ifndef PERL_HANDY_H_ /* Guard against nested #inclusion */
#define PERL_HANDY_H_

#ifndef PERL_CORE
#  define Null(type) ((type)NULL)

/*
=head1 Handy Values

=for apidoc AmnU||Nullch
Null character pointer.  (No longer available when C<PERL_CORE> is
defined.)

=for apidoc AmnU||Nullsv
Null SV pointer.  (No longer available when C<PERL_CORE> is defined.)

=cut
*/

#  define Nullch Null(char*)
#  define Nullfp Null(PerlIO*)
#  define Nullsv Null(SV*)
#endif

#ifdef TRUE
#undef TRUE
#endif
#ifdef FALSE
#undef FALSE
#endif
#define TRUE (1)
#define FALSE (0)

/* The MUTABLE_*() macros cast pointers to the types shown, in such a way
 * (compiler permitting) that casting away const-ness will give a warning;
 * e.g.:
 *
 * const SV *sv = ...;
 * AV *av1 = (AV*)sv;        <== BAD:  the const has been silently cast away
 * AV *av2 = MUTABLE_AV(sv); <== GOOD: it may warn
 */

#if defined(__GNUC__) && !defined(PERL_GCC_BRACE_GROUPS_FORBIDDEN)
#  define MUTABLE_PTR(p) ({ void *_p = (p); _p; })
#else
#  define MUTABLE_PTR(p) ((void *) (p))
#endif

#define MUTABLE_AV(p)	((AV *)MUTABLE_PTR(p))
#define MUTABLE_CV(p)	((CV *)MUTABLE_PTR(p))
#define MUTABLE_GV(p)	((GV *)MUTABLE_PTR(p))
#define MUTABLE_HV(p)	((HV *)MUTABLE_PTR(p))
#define MUTABLE_IO(p)	((IO *)MUTABLE_PTR(p))
#define MUTABLE_SV(p)	((SV *)MUTABLE_PTR(p))

#if defined(I_STDBOOL) && !defined(PERL_BOOL_AS_CHAR)
#  include <stdbool.h>
#  ifndef HAS_BOOL
#    define HAS_BOOL 1
#  endif
#endif

/* bool is built-in for g++-2.6.3 and later, which might be used
   for extensions.  <_G_config.h> defines _G_HAVE_BOOL, but we can't
   be sure _G_config.h will be included before this file.  _G_config.h
   also defines _G_HAVE_BOOL for both gcc and g++, but only g++
   actually has bool.  Hence, _G_HAVE_BOOL is pretty useless for us.
   g++ can be identified by __GNUG__.
   Andy Dougherty	February 2000
*/
#ifdef __GNUG__		/* GNU g++ has bool built-in */
# ifndef PERL_BOOL_AS_CHAR
#  ifndef HAS_BOOL
#    define HAS_BOOL 1
#  endif
# endif
#endif

#ifndef HAS_BOOL
# ifdef bool
#  undef bool
# endif
# define bool char
# define HAS_BOOL 1
#endif

/*
=for apidoc Am|bool|cBOOL|bool expr

Cast-to-bool.  A simple S<C<(bool) I<expr>>> cast may not do the right thing:
if C<bool> is defined as C<char>, for example, then the cast from C<int> is
implementation-defined.

C<(bool)!!(cbool)> in a ternary triggers a bug in xlc on AIX

=cut
*/
#define cBOOL(cbool) ((cbool) ? (bool)1 : (bool)0)

/* Try to figure out __func__ or __FUNCTION__ equivalent, if any.
 * XXX Should really be a Configure probe, with HAS__FUNCTION__
 *     and FUNCTION__ as results.
 * XXX Similarly, a Configure probe for __FILE__ and __LINE__ is needed. */
#if (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L) || (defined(__SUNPRO_C)) /* C99 or close enough. */
#  define FUNCTION__ __func__
#elif (defined(__DECC_VER)) /* Tru64 or VMS, and strict C89 being used, but not modern enough cc (in Tur64, -c99 not known, only -std1). */
#  define FUNCTION__ ""
#else
#  define FUNCTION__ __FUNCTION__ /* Common extension. */
#endif

/* XXX A note on the perl source internal type system.  The
   original intent was that I32 be *exactly* 32 bits.

   Currently, we only guarantee that I32 is *at least* 32 bits.
   Specifically, if int is 64 bits, then so is I32.  (This is the case
   for the Cray.)  This has the advantage of meshing nicely with
   standard library calls (where we pass an I32 and the library is
   expecting an int), but the disadvantage that an I32 is not 32 bits.
   Andy Dougherty	August 1996

   There is no guarantee that there is *any* integral type with
   exactly 32 bits.  It is perfectly legal for a system to have
   sizeof(short) == sizeof(int) == sizeof(long) == 8.

   Similarly, there is no guarantee that I16 and U16 have exactly 16
   bits.

   For dealing with issues that may arise from various 32/64-bit
   systems, we will ask Configure to check out

	SHORTSIZE == sizeof(short)
	INTSIZE == sizeof(int)
	LONGSIZE == sizeof(long)
	LONGLONGSIZE == sizeof(long long) (if HAS_LONG_LONG)
	PTRSIZE == sizeof(void *)
	DOUBLESIZE == sizeof(double)
	LONG_DOUBLESIZE == sizeof(long double) (if HAS_LONG_DOUBLE).

*/

#ifdef I_INTTYPES /* e.g. Linux has int64_t without <inttypes.h> */
#   include <inttypes.h>
#   ifdef INT32_MIN_BROKEN
#       undef  INT32_MIN
#       define INT32_MIN (-2147483647-1)
#   endif
#   ifdef INT64_MIN_BROKEN
#       undef  INT64_MIN
#       define INT64_MIN (-9223372036854775807LL-1)
#   endif
#endif

typedef I8TYPE I8;
typedef U8TYPE U8;
typedef I16TYPE I16;
typedef U16TYPE U16;
typedef I32TYPE I32;
typedef U32TYPE U32;

#ifdef QUADKIND
typedef I64TYPE I64;
typedef U64TYPE U64;
#endif

#if defined(UINT8_MAX) && defined(INT16_MAX) && defined(INT32_MAX)

/* I8_MAX and I8_MIN constants are not defined, as I8 is an ambiguous type.
   Please search CHAR_MAX in perl.h for further details. */
#define U8_MAX UINT8_MAX
#define U8_MIN UINT8_MIN

#define I16_MAX INT16_MAX
#define I16_MIN INT16_MIN
#define U16_MAX UINT16_MAX
#define U16_MIN UINT16_MIN

#define I32_MAX INT32_MAX
#define I32_MIN INT32_MIN
#ifndef UINT32_MAX_BROKEN /* e.g. HP-UX with gcc messes this up */
#  define U32_MAX UINT32_MAX
#else
#  define U32_MAX 4294967295U
#endif
#define U32_MIN UINT32_MIN

#else

/* I8_MAX and I8_MIN constants are not defined, as I8 is an ambiguous type.
   Please search CHAR_MAX in perl.h for further details. */
#define U8_MAX PERL_UCHAR_MAX
#define U8_MIN PERL_UCHAR_MIN

#define I16_MAX PERL_SHORT_MAX
#define I16_MIN PERL_SHORT_MIN
#define U16_MAX PERL_USHORT_MAX
#define U16_MIN PERL_USHORT_MIN

#if LONGSIZE > 4
# define I32_MAX PERL_INT_MAX
# define I32_MIN PERL_INT_MIN
# define U32_MAX PERL_UINT_MAX
# define U32_MIN PERL_UINT_MIN
#else
# define I32_MAX PERL_LONG_MAX
# define I32_MIN PERL_LONG_MIN
# define U32_MAX PERL_ULONG_MAX
# define U32_MIN PERL_ULONG_MIN
#endif

#endif

/* These C99 typedefs are useful sometimes for, say, loop variables whose
 * maximum values are small, but for which speed trumps size.  If we have a C99
 * compiler, use that.  Otherwise, a plain 'int' should be good enough.
 *
 * Restrict these to core for now until we are more certain this is a good
 * idea. */
#if defined(PERL_CORE) || defined(PERL_EXT)
#  ifdef I_STDINT
    typedef  int_fast8_t  PERL_INT_FAST8_T;
    typedef uint_fast8_t  PERL_UINT_FAST8_T;
    typedef  int_fast16_t PERL_INT_FAST16_T;
    typedef uint_fast16_t PERL_UINT_FAST16_T;
#  else
    typedef int           PERL_INT_FAST8_T;
    typedef unsigned int  PERL_UINT_FAST8_T;
    typedef int           PERL_INT_FAST16_T;
    typedef unsigned int  PERL_UINT_FAST16_T;
#  endif
#endif

/* log(2) (i.e., log base 10 of 2) is pretty close to 0.30103, just in case
 * anyone is grepping for it */
#define BIT_DIGITS(N)   (((N)*146)/485 + 1)  /* log10(2) =~ 146/485 */
#define TYPE_DIGITS(T)  BIT_DIGITS(sizeof(T) * 8)
#define TYPE_CHARS(T)   (TYPE_DIGITS(T) + 2) /* sign, NUL */

/* Unused by core; should be deprecated */
#define Ctl(ch) ((ch) & 037)

#if defined(PERL_CORE) || defined(PERL_EXT)
#  ifndef MIN
#    define MIN(a,b) ((a) < (b) ? (a) : (b))
#  endif
#  ifndef MAX
#    define MAX(a,b) ((a) > (b) ? (a) : (b))
#  endif
#endif

/* Returns a boolean as to whether the input unsigned number is a power of 2
 * (2**0, 2**1, etc).  In other words if it has just a single bit set.
 * If not, subtracting 1 would leave the uppermost bit set, so the & would
 * yield non-zero */
#if defined(PERL_CORE) || defined(PERL_EXT)
#  define isPOWER_OF_2(n) ((n) && ((n) & ((n)-1)) == 0)
#endif

/*
=for apidoc Am|void|__ASSERT_|bool expr

This is a helper macro to avoid preprocessor issues, replaced by nothing
unless under DEBUGGING, where it expands to an assert of its argument,
followed by a comma (hence the comma operator).  If we just used a straight
assert(), we would get a comma with nothing before it when not DEBUGGING.

=cut

We also use empty definition under Coverity since the __ASSERT__
checks often check for things that Really Cannot Happen, and Coverity
detects that and gets all excited. */

#if   defined(DEBUGGING) && !defined(__COVERITY__)                        \
 && ! defined(PERL_SMALL_MACRO_BUFFER)
#   define __ASSERT_(statement)  assert(statement),
#else
#   define __ASSERT_(statement)
#endif

/*
=head1 SV Manipulation Functions

=for apidoc Ama|SV*|newSVpvs|"literal string"
Like C<newSVpvn>, but takes a literal string instead of a
string/length pair.

=for apidoc Ama|SV*|newSVpvs_flags|"literal string"|U32 flags
Like C<newSVpvn_flags>, but takes a literal string instead of
a string/length pair.

=for apidoc Ama|SV*|newSVpvs_share|"literal string"
Like C<newSVpvn_share>, but takes a literal string instead of
a string/length pair and omits the hash parameter.

=for apidoc Am|void|sv_catpvs_flags|SV* sv|"literal string"|I32 flags
Like C<sv_catpvn_flags>, but takes a literal string instead
of a string/length pair.

=for apidoc Am|void|sv_catpvs_nomg|SV* sv|"literal string"
Like C<sv_catpvn_nomg>, but takes a literal string instead of
a string/length pair.

=for apidoc Am|void|sv_catpvs|SV* sv|"literal string"
Like C<sv_catpvn>, but takes a literal string instead of a
string/length pair.

=for apidoc Am|void|sv_catpvs_mg|SV* sv|"literal string"
Like C<sv_catpvn_mg>, but takes a literal string instead of a
string/length pair.

=for apidoc Am|void|sv_setpvs|SV* sv|"literal string"
Like C<sv_setpvn>, but takes a literal string instead of a
string/length pair.

=for apidoc Am|void|sv_setpvs_mg|SV* sv|"literal string"
Like C<sv_setpvn_mg>, but takes a literal string instead of a
string/length pair.

=for apidoc Am|SV *|sv_setref_pvs|SV *const rv|const char *const classname|"literal string"
Like C<sv_setref_pvn>, but takes a literal string instead of
a string/length pair.

=head1 Memory Management

=for apidoc Ama|char*|savepvs|"literal string"
Like C<savepvn>, but takes a literal string instead of a
string/length pair.

=for apidoc Ama|char*|savesharedpvs|"literal string"
A version of C<savepvs()> which allocates the duplicate string in memory
which is shared between threads.

=head1 GV Functions

=for apidoc Am|HV*|gv_stashpvs|"name"|I32 create
Like C<gv_stashpvn>, but takes a literal string instead of a
string/length pair.

=head1 Hash Manipulation Functions

=for apidoc Am|SV**|hv_fetchs|HV* tb|"key"|I32 lval
Like C<hv_fetch>, but takes a literal string instead of a
string/length pair.

=for apidoc Am|SV**|hv_stores|HV* tb|"key"|SV* val
Like C<hv_store>, but takes a literal string instead of a
string/length pair
and omits the hash parameter.

=head1 Lexer interface

=for apidoc Amx|void|lex_stuff_pvs|"pv"|U32 flags

Like L</lex_stuff_pvn>, but takes a literal string instead of
a string/length pair.

=cut
*/

/*
=head1 Handy Values

=for apidoc Amu|pair|STR_WITH_LEN|"literal string"

Returns two comma separated tokens of the input literal string, and its length.
This is convenience macro which helps out in some API calls.
Note that it can't be used as an argument to macros or functions that under
some configurations might be macros, which means that it requires the full
Perl_xxx(aTHX_ ...) form for any API calls where it's used.

=cut
*/


#define STR_WITH_LEN(s)  ("" s ""), (sizeof(s)-1)

/* STR_WITH_LEN() shortcuts */
#define newSVpvs(str) Perl_newSVpvn(aTHX_ STR_WITH_LEN(str))
#define newSVpvs_flags(str,flags)	\
    Perl_newSVpvn_flags(aTHX_ STR_WITH_LEN(str), flags)
#define newSVpvs_share(str) Perl_newSVpvn_share(aTHX_ STR_WITH_LEN(str), 0)
#define sv_catpvs_flags(sv, str, flags) \
    Perl_sv_catpvn_flags(aTHX_ sv, STR_WITH_LEN(str), flags)
#define sv_catpvs_nomg(sv, str) \
    Perl_sv_catpvn_flags(aTHX_ sv, STR_WITH_LEN(str), 0)
#define sv_catpvs(sv, str) \
    Perl_sv_catpvn_flags(aTHX_ sv, STR_WITH_LEN(str), SV_GMAGIC)
#define sv_catpvs_mg(sv, str) \
    Perl_sv_catpvn_flags(aTHX_ sv, STR_WITH_LEN(str), SV_GMAGIC|SV_SMAGIC)
#define sv_setpvs(sv, str) Perl_sv_setpvn(aTHX_ sv, STR_WITH_LEN(str))
#define sv_setpvs_mg(sv, str) Perl_sv_setpvn_mg(aTHX_ sv, STR_WITH_LEN(str))
#define sv_setref_pvs(rv, classname, str) \
    Perl_sv_setref_pvn(aTHX_ rv, classname, STR_WITH_LEN(str))
#define savepvs(str) Perl_savepvn(aTHX_ STR_WITH_LEN(str))
#define savesharedpvs(str) Perl_savesharedpvn(aTHX_ STR_WITH_LEN(str))
#define gv_stashpvs(str, create) \
    Perl_gv_stashpvn(aTHX_ STR_WITH_LEN(str), create)
#define gv_fetchpvs(namebeg, add, sv_type) \
    Perl_gv_fetchpvn_flags(aTHX_ STR_WITH_LEN(namebeg), add, sv_type)
#define gv_fetchpvn(namebeg, len, add, sv_type) \
    Perl_gv_fetchpvn_flags(aTHX_ namebeg, len, add, sv_type)
#define sv_catxmlpvs(dsv, str, utf8) \
    Perl_sv_catxmlpvn(aTHX_ dsv, STR_WITH_LEN(str), utf8)


#define lex_stuff_pvs(pv,flags) Perl_lex_stuff_pvn(aTHX_ STR_WITH_LEN(pv), flags)

#define get_cvs(str, flags)					\
	Perl_get_cvn_flags(aTHX_ STR_WITH_LEN(str), (flags))

/*
=head1 Miscellaneous Functions

=for apidoc Am|bool|strNE|char* s1|char* s2
Test two C<NUL>-terminated strings to see if they are different.  Returns true
or false.

=for apidoc Am|bool|strEQ|char* s1|char* s2
Test two C<NUL>-terminated strings to see if they are equal.  Returns true or
false.

=for apidoc Am|bool|strLT|char* s1|char* s2
Test two C<NUL>-terminated strings to see if the first, C<s1>, is less than the
second, C<s2>.  Returns true or false.

=for apidoc Am|bool|strLE|char* s1|char* s2
Test two C<NUL>-terminated strings to see if the first, C<s1>, is less than or
equal to the second, C<s2>.  Returns true or false.

=for apidoc Am|bool|strGT|char* s1|char* s2
Test two C<NUL>-terminated strings to see if the first, C<s1>, is greater than
the second, C<s2>.  Returns true or false.

=for apidoc Am|bool|strGE|char* s1|char* s2
Test two C<NUL>-terminated strings to see if the first, C<s1>, is greater than
or equal to the second, C<s2>.  Returns true or false.

=for apidoc Am|bool|strnNE|char* s1|char* s2|STRLEN len
Test two C<NUL>-terminated strings to see if they are different.  The C<len>
parameter indicates the number of bytes to compare.  Returns true or false.  (A
wrapper for C<strncmp>).

=for apidoc Am|bool|strnEQ|char* s1|char* s2|STRLEN len
Test two C<NUL>-terminated strings to see if they are equal.  The C<len>
parameter indicates the number of bytes to compare.  Returns true or false.  (A
wrapper for C<strncmp>).

=for apidoc Am|bool|memEQ|char* s1|char* s2|STRLEN len
Test two buffers (which may contain embedded C<NUL> characters, to see if they
are equal.  The C<len> parameter indicates the number of bytes to compare.
Returns zero if equal, or non-zero if non-equal.

=for apidoc Am|bool|memEQs|char* s1|STRLEN l1|"s2"
Like L</memEQ>, but the second string is a literal enclosed in double quotes,
C<l1> gives the number of bytes in C<s1>.
Returns zero if equal, or non-zero if non-equal.

=for apidoc Am|bool|memNE|char* s1|char* s2|STRLEN len
Test two buffers (which may contain embedded C<NUL> characters, to see if they
are not equal.  The C<len> parameter indicates the number of bytes to compare.
Returns zero if non-equal, or non-zero if equal.

=for apidoc Am|bool|memNEs|char* s1|STRLEN l1|"s2"
Like L</memNE>, but the second string is a literal enclosed in double quotes,
C<l1> gives the number of bytes in C<s1>.
Returns zero if non-equal, or zero if non-equal.

=for apidoc Am|bool|memCHRs|"list"|char c
Returns the position of the first occurence of the byte C<c> in the literal
string C<"list">, or NULL if C<c> doesn't appear in C<"list">.  All bytes are
treated as unsigned char.  Thus this macro can be used to determine if C<c> is
in a set of particular characters.  Unlike L<strchr(3)>, it works even if C<c>
is C<NUL> (and the set doesn't include C<NUL>).

=cut

New macros should use the following conventions for their names (which are
based on the underlying C library functions):

  (mem | str n? ) (EQ | NE | LT | GT | GE | (( BEGIN | END ) P? )) l? s?

  Each has two main parameters, string-like operands that are compared
  against each other, as specified by the macro name.  Some macros may
  additionally have one or potentially even two length parameters.  If a length
  parameter applies to both string parameters, it will be positioned third;
  otherwise any length parameter immediately follows the string parameter it
  applies to.

  If the prefix to the name is 'str', the string parameter is a pointer to a C
  language string.  Such a string does not contain embedded NUL bytes; its
  length may be unknown, but can be calculated by C<strlen()>, since it is
  terminated by a NUL, which isn't included in its length.

  The optional 'n' following 'str' means that there is a third parameter,
  giving the maximum number of bytes to look at in each string.  Even if both
  strings are longer than the length parameter, those extra bytes will be
  unexamined.

  The 's' suffix means that the 2nd byte string parameter is a literal C
  double-quoted string.  Its length will automatically be calculated by the
  macro, so no length parameter will ever be needed for it.

  If the prefix is 'mem', the string parameters don't have to be C strings;
  they may contain embedded NUL bytes, do not necessarily have a terminating
  NUL, and their lengths can be known only through other means, which in
  practice are additional parameter(s) passed to the function.  All 'mem'
  functions have at least one length parameter.  Barring any 'l' or 's' suffix,
  there is a single length parameter, in position 3, which applies to both
  string parameters.  The 's' suffix means, as described above, that the 2nd
  string is a literal double-quoted C string (hence its length is calculated by
  the macro, and the length parameter to the function applies just to the first
  string parameter, and hence is positioned just after it).  An 'l' suffix
  means that the 2nd string parameter has its own length parameter, and the
  signature will look like memFOOl(s1, l1, s2, l2).

  BEGIN (and END) are for testing if the 2nd string is an initial (or final)
  substring  of the 1st string.  'P' if present indicates that the substring
  must be a "proper" one in tha mathematical sense that the first one must be
  strictly larger than the 2nd.

*/


#define strNE(s1,s2) (strcmp(s1,s2) != 0)
#define strEQ(s1,s2) (strcmp(s1,s2) == 0)
#define strLT(s1,s2) (strcmp(s1,s2) < 0)
#define strLE(s1,s2) (strcmp(s1,s2) <= 0)
#define strGT(s1,s2) (strcmp(s1,s2) > 0)
#define strGE(s1,s2) (strcmp(s1,s2) >= 0)

#define strnNE(s1,s2,l) (strncmp(s1,s2,l) != 0)
#define strnEQ(s1,s2,l) (strncmp(s1,s2,l) == 0)

#define memEQ(s1,s2,l) (memcmp(((const void *) (s1)), ((const void *) (s2)), l) == 0)
#define memNE(s1,s2,l) (! memEQ(s1,s2,l))

/* memEQ and memNE where second comparand is a string constant */
#define memEQs(s1, l, s2) \
        (((sizeof(s2)-1) == (l)) && memEQ((s1), ("" s2 ""), (sizeof(s2)-1)))
#define memNEs(s1, l, s2) (! memEQs(s1, l, s2))

/* Keep these private until we decide it was a good idea */
#if defined(PERL_CORE) || defined(PERL_EXT) || defined(PERL_EXT_POSIX)

#define strBEGINs(s1,s2) (strncmp(s1,"" s2 "", sizeof(s2)-1) == 0)

#define memBEGINs(s1, l, s2)                                                \
            (   (Ptrdiff_t) (l) >= (Ptrdiff_t) sizeof(s2) - 1               \
             && memEQ(s1, "" s2 "", sizeof(s2)-1))
#define memBEGINPs(s1, l, s2)                                               \
            (   (Ptrdiff_t) (l) > (Ptrdiff_t) sizeof(s2) - 1                \
             && memEQ(s1, "" s2 "", sizeof(s2)-1))
#define memENDs(s1, l, s2)                                                  \
            (   (Ptrdiff_t) (l) >= (Ptrdiff_t) sizeof(s2) - 1               \
             && memEQ(s1 + (l) - (sizeof(s2) - 1), "" s2 "", sizeof(s2)-1))
#define memENDPs(s1, l, s2)                                                 \
            (   (Ptrdiff_t) (l) > (Ptrdiff_t) sizeof(s2)                    \
             && memEQ(s1 + (l) - (sizeof(s2) - 1), "" s2 "", sizeof(s2)-1))
#endif  /* End of making macros private */

#define memLT(s1,s2,l) (memcmp(s1,s2,l) < 0)
#define memLE(s1,s2,l) (memcmp(s1,s2,l) <= 0)
#define memGT(s1,s2,l) (memcmp(s1,s2,l) > 0)
#define memGE(s1,s2,l) (memcmp(s1,s2,l) >= 0)

#define memCHRs(s1,c) ((const char *) memchr("" s1 "" , c, sizeof(s1)-1))

/*
 * Character classes.
 *
 * Unfortunately, the introduction of locales means that we
 * can't trust isupper(), etc. to tell the truth.  And when
 * it comes to /\w+/ with tainting enabled, we *must* be able
 * to trust our character classes.
 *
 * Therefore, the default tests in the text of Perl will be
 * independent of locale.  Any code that wants to depend on
 * the current locale will use the tests that begin with "lc".
 */

#ifdef HAS_SETLOCALE  /* XXX Is there a better test for this? */
#  ifndef CTYPE256
#    define CTYPE256
#  endif
#endif

/*

=head1 Character classification
This section is about functions (really macros) that classify characters
into types, such as punctuation versus alphabetic, etc.  Most of these are
analogous to regular expression character classes.  (See
L<perlrecharclass/POSIX Character Classes>.)  There are several variants for
each class.  (Not all macros have all variants; each item below lists the
ones valid for it.)  None are affected by C<use bytes>, and only the ones
with C<LC> in the name are affected by the current locale.

The base function, e.g., C<isALPHA()>, takes any signed or unsigned value,
treating it as a code point, and returns a boolean as to whether or not the
character represented by it is (or on non-ASCII platforms, corresponds to) an
ASCII character in the named class based on platform, Unicode, and Perl rules.
If the input is a number that doesn't fit in an octet, FALSE is returned.

Variant C<isI<FOO>_A> (e.g., C<isALPHA_A()>) is identical to the base function
with no suffix C<"_A">.  This variant is used to emphasize by its name that
only ASCII-range characters can return TRUE.

Variant C<isI<FOO>_L1> imposes the Latin-1 (or EBCDIC equivalent) character set
onto the platform.  That is, the code points that are ASCII are unaffected,
since ASCII is a subset of Latin-1.  But the non-ASCII code points are treated
as if they are Latin-1 characters.  For example, C<isWORDCHAR_L1()> will return
true when called with the code point 0xDF, which is a word character in both
ASCII and EBCDIC (though it represents different characters in each).
If the input is a number that doesn't fit in an octet, FALSE is returned.
(Perl's documentation uses a colloquial definition of Latin-1, to include all
code points below 256.)

Variant C<isI<FOO>_uvchr> is exactly like the C<isI<FOO>_L1> variant, for
inputs below 256, but if the code point is larger than 255, Unicode rules are
used to determine if it is in the character class.  For example,
C<isWORDCHAR_uvchr(0x100)> returns TRUE, since 0x100 is LATIN CAPITAL LETTER A
WITH MACRON in Unicode, and is a word character.

Variants C<isI<FOO>_utf8> and C<isI<FOO>_utf8_safe> are like C<isI<FOO>_uvchr>,
but are used for UTF-8 encoded strings.  The two forms are different names for
the same thing.  Each call to one of these classifies the first character of
the string starting at C<p>.  The second parameter, C<e>, points to anywhere in
the string beyond the first character, up to one byte past the end of the
entire string.  Although both variants are identical, the suffix C<_safe> in
one name emphasizes that it will not attempt to read beyond S<C<e - 1>>,
provided that the constraint S<C<s E<lt> e>> is true (this is asserted for in
C<-DDEBUGGING> builds).  If the UTF-8 for the input character is malformed in
some way, the program may croak, or the function may return FALSE, at the
discretion of the implementation, and subject to change in future releases.

Variant C<isI<FOO>_LC> is like the C<isI<FOO>_A> and C<isI<FOO>_L1> variants,
but the result is based on the current locale, which is what C<LC> in the name
stands for.  If Perl can determine that the current locale is a UTF-8 locale,
it uses the published Unicode rules; otherwise, it uses the C library function
that gives the named classification.  For example, C<isDIGIT_LC()> when not in
a UTF-8 locale returns the result of calling C<isdigit()>.  FALSE is always
returned if the input won't fit into an octet.  On some platforms where the C
library function is known to be defective, Perl changes its result to follow
the POSIX standard's rules.

Variant C<isI<FOO>_LC_uvchr> acts exactly like C<isI<FOO>_LC> for inputs less
than 256, but for larger ones it returns the Unicode classification of the code
point.

Variants C<isI<FOO>_LC_utf8> and C<isI<FOO>_LC_utf8_safe> are like
C<isI<FOO>_LC_uvchr>, but are used for UTF-8 encoded strings.  The two forms
are different names for the same thing.  Each call to one of these classifies
the first character of the string starting at C<p>.  The second parameter,
C<e>, points to anywhere in the string beyond the first character, up to one
byte past the end of the entire string.  Although both variants are identical,
the suffix C<_safe> in one name emphasizes that it will not attempt to read
beyond S<C<e - 1>>, provided that the constraint S<C<s E<lt> e>> is true (this
is asserted for in C<-DDEBUGGING> builds).  If the UTF-8 for the input
character is malformed in some way, the program may croak, or the function may
return FALSE, at the discretion of the implementation, and subject to change in
future releases.

=for apidoc Am|bool|isALPHA|int ch
Returns a boolean indicating whether the specified input is one of C<[A-Za-z]>,
analogous to C<m/[[:alpha:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isALPHA_A>, C<isALPHA_L1>, C<isALPHA_uvchr>, C<isALPHA_utf8>,
C<isALPHA_utf8_safe>, C<isALPHA_LC>, C<isALPHA_LC_uvchr>, C<isALPHA_LC_utf8>,
and C<isALPHA_LC_utf8_safe>.

=cut

Here and below, we add the protoypes of these macros for downstream programs
that would be interested in them, such as Devel::PPPort

=for apidoc Amh|bool|isALPHA_A|int ch
=for apidoc Amh|bool|isALPHA_L1|int ch
=for apidoc Amh|bool|isALPHA_uvchr|int ch
=for apidoc Amh|bool|isALPHA_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isALPHA_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isALPHA_LC|int ch
=for apidoc Amh|bool|isALPHA_LC_uvchr|int ch
=for apidoc Amh|bool|isALPHA_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isALPHANUMERIC|int ch
Returns a boolean indicating whether the specified character is one of
C<[A-Za-z0-9]>, analogous to C<m/[[:alnum:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isALPHANUMERIC_A>, C<isALPHANUMERIC_L1>, C<isALPHANUMERIC_uvchr>,
C<isALPHANUMERIC_utf8>, C<isALPHANUMERIC_utf8_safe>, C<isALPHANUMERIC_LC>,
C<isALPHANUMERIC_LC_uvchr>, C<isALPHANUMERIC_LC_utf8>, and
C<isALPHANUMERIC_LC_utf8_safe>.

A (discouraged from use) synonym is C<isALNUMC> (where the C<C> suffix means
this corresponds to the C language alphanumeric definition).  Also
there are the variants
C<isALNUMC_A>, C<isALNUMC_L1>
C<isALNUMC_LC>, and C<isALNUMC_LC_uvchr>.

=for apidoc Amh|bool|isALPHANUMERIC_A|int ch
=for apidoc Amh|bool|isALPHANUMERIC_L1|int ch
=for apidoc Amh|bool|isALPHANUMERIC_uvchr|int ch
=for apidoc Amh|bool|isALPHANUMERIC_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isALPHANUMERIC_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isALPHANUMERIC_LC|int ch
=for apidoc Amh|bool|isALPHANUMERIC_LC_uvchr|int ch
=for apidoc Amh|bool|isALPHANUMERIC_LC_utf8_safe|U8 * s| U8 *end
=for apidoc Amh|bool|isALNUMC|int ch
=for apidoc Amh|bool|isALNUMC_A|int ch
=for apidoc Amh|bool|isALNUMC_L1|int ch
=for apidoc Amh|bool|isALNUMC_LC|int ch
=for apidoc Amh|bool|isALNUMC_LC_uvchr|int ch

=for apidoc Am|bool|isASCII|int ch
Returns a boolean indicating whether the specified character is one of the 128
characters in the ASCII character set, analogous to C<m/[[:ascii:]]/>.
On non-ASCII platforms, it returns TRUE iff this
character corresponds to an ASCII character.  Variants C<isASCII_A()> and
C<isASCII_L1()> are identical to C<isASCII()>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isASCII_uvchr>, C<isASCII_utf8>, C<isASCII_utf8_safe>, C<isASCII_LC>,
C<isASCII_LC_uvchr>, C<isASCII_LC_utf8>, and C<isASCII_LC_utf8_safe>.
Note, however, that some platforms do not have the C library routine
C<isascii()>.  In these cases, the variants whose names contain C<LC> are the
same as the corresponding ones without.

=for apidoc Amh|bool|isASCII_A|int ch
=for apidoc Amh|bool|isASCII_L1|int ch
=for apidoc Amh|bool|isASCII_uvchr|int ch
=for apidoc Amh|bool|isASCII_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isASCII_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isASCII_LC|int ch
=for apidoc Amh|bool|isASCII_LC_uvchr|int ch
=for apidoc Amh|bool|isASCII_LC_utf8_safe|U8 * s| U8 *end

Also note, that because all ASCII characters are UTF-8 invariant (meaning they
have the exact same representation (always a single byte) whether encoded in
UTF-8 or not), C<isASCII> will give the correct results when called with any
byte in any string encoded or not in UTF-8.  And similarly C<isASCII_utf8> and
C<isASCII_utf8_safe> will work properly on any string encoded or not in UTF-8.

=for apidoc Am|bool|isBLANK|char ch
Returns a boolean indicating whether the specified character is a
character considered to be a blank, analogous to C<m/[[:blank:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isBLANK_A>, C<isBLANK_L1>, C<isBLANK_uvchr>, C<isBLANK_utf8>,
C<isBLANK_utf8_safe>, C<isBLANK_LC>, C<isBLANK_LC_uvchr>, C<isBLANK_LC_utf8>,
and C<isBLANK_LC_utf8_safe>.  Note,
however, that some platforms do not have the C library routine
C<isblank()>.  In these cases, the variants whose names contain C<LC> are
the same as the corresponding ones without.

=for apidoc Amh|bool|isBLANK_A|int ch
=for apidoc Amh|bool|isBLANK_L1|int ch
=for apidoc Amh|bool|isBLANK_uvchr|int ch
=for apidoc Amh|bool|isBLANK_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isBLANK_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isBLANK_LC|int ch
=for apidoc Amh|bool|isBLANK_LC_uvchr|int ch
=for apidoc Amh|bool|isBLANK_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isCNTRL|char ch
Returns a boolean indicating whether the specified character is a
control character, analogous to C<m/[[:cntrl:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isCNTRL_A>, C<isCNTRL_L1>, C<isCNTRL_uvchr>, C<isCNTRL_utf8>,
C<isCNTRL_utf8_safe>, C<isCNTRL_LC>, C<isCNTRL_LC_uvchr>, C<isCNTRL_LC_utf8>
and C<isCNTRL_LC_utf8_safe>.  On EBCDIC
platforms, you almost always want to use the C<isCNTRL_L1> variant.

=for apidoc Amh|bool|isCNTRL_A|int ch
=for apidoc Amh|bool|isCNTRL_L1|int ch
=for apidoc Amh|bool|isCNTRL_uvchr|int ch
=for apidoc Amh|bool|isCNTRL_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isCNTRL_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isCNTRL_LC|int ch
=for apidoc Amh|bool|isCNTRL_LC_uvchr|int ch
=for apidoc Amh|bool|isCNTRL_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isDIGIT|char ch
Returns a boolean indicating whether the specified character is a
digit, analogous to C<m/[[:digit:]]/>.
Variants C<isDIGIT_A> and C<isDIGIT_L1> are identical to C<isDIGIT>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isDIGIT_uvchr>, C<isDIGIT_utf8>, C<isDIGIT_utf8_safe>, C<isDIGIT_LC>,
C<isDIGIT_LC_uvchr>, C<isDIGIT_LC_utf8>, and C<isDIGIT_LC_utf8_safe>.

=for apidoc Amh|bool|isDIGIT_A|int ch
=for apidoc Amh|bool|isDIGIT_L1|int ch
=for apidoc Amh|bool|isDIGIT_uvchr|int ch
=for apidoc Amh|bool|isDIGIT_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isDIGIT_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isDIGIT_LC|int ch
=for apidoc Amh|bool|isDIGIT_LC_uvchr|int ch
=for apidoc Amh|bool|isDIGIT_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isGRAPH|char ch
Returns a boolean indicating whether the specified character is a
graphic character, analogous to C<m/[[:graph:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants C<isGRAPH_A>, C<isGRAPH_L1>, C<isGRAPH_uvchr>, C<isGRAPH_utf8>,
C<isGRAPH_utf8_safe>, C<isGRAPH_LC>, C<isGRAPH_LC_uvchr>,
C<isGRAPH_LC_utf8_safe>, and C<isGRAPH_LC_utf8_safe>.

=for apidoc Amh|bool|isGRAPH_A|int ch
=for apidoc Amh|bool|isGRAPH_L1|int ch
=for apidoc Amh|bool|isGRAPH_uvchr|int ch
=for apidoc Amh|bool|isGRAPH_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isGRAPH_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isGRAPH_LC|int ch
=for apidoc Amh|bool|isGRAPH_LC_uvchr|int ch
=for apidoc Amh|bool|isGRAPH_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isLOWER|char ch
Returns a boolean indicating whether the specified character is a
lowercase character, analogous to C<m/[[:lower:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isLOWER_A>, C<isLOWER_L1>, C<isLOWER_uvchr>, C<isLOWER_utf8>,
C<isLOWER_utf8_safe>, C<isLOWER_LC>, C<isLOWER_LC_uvchr>, C<isLOWER_LC_utf8>,
and C<isLOWER_LC_utf8_safe>.

=for apidoc Amh|bool|isLOWER_A|int ch
=for apidoc Amh|bool|isLOWER_L1|int ch
=for apidoc Amh|bool|isLOWER_uvchr|int ch
=for apidoc Amh|bool|isLOWER_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isLOWER_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isLOWER_LC|int ch
=for apidoc Amh|bool|isLOWER_LC_uvchr|int ch
=for apidoc Amh|bool|isLOWER_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isOCTAL|char ch
Returns a boolean indicating whether the specified character is an
octal digit, [0-7].
The only two variants are C<isOCTAL_A> and C<isOCTAL_L1>; each is identical to
C<isOCTAL>.

=for apidoc Amh|bool|isOCTAL_A|int ch
=for apidoc Amh|bool|isOCTAL_L1|int ch

=for apidoc Am|bool|isPUNCT|char ch
Returns a boolean indicating whether the specified character is a
punctuation character, analogous to C<m/[[:punct:]]/>.
Note that the definition of what is punctuation isn't as
straightforward as one might desire.  See L<perlrecharclass/POSIX Character
Classes> for details.
See the L<top of this section|/Character classification> for an explanation of
variants C<isPUNCT_A>, C<isPUNCT_L1>, C<isPUNCT_uvchr>, C<isPUNCT_utf8>,
C<isPUNCT_utf8_safe>, C<isPUNCT_LC>, C<isPUNCT_LC_uvchr>, C<isPUNCT_LC_utf8>,
and C<isPUNCT_LC_utf8_safe>.

=for apidoc Amh|bool|isPUNCT_A|int ch
=for apidoc Amh|bool|isPUNCT_L1|int ch
=for apidoc Amh|bool|isPUNCT_uvchr|int ch
=for apidoc Amh|bool|isPUNCT_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isPUNCT_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isPUNCT_LC|int ch
=for apidoc Amh|bool|isPUNCT_LC_uvchr|int ch
=for apidoc Amh|bool|isPUNCT_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isSPACE|char ch
Returns a boolean indicating whether the specified character is a
whitespace character.  This is analogous
to what C<m/\s/> matches in a regular expression.  Starting in Perl 5.18
this also matches what C<m/[[:space:]]/> does.  Prior to 5.18, only the
locale forms of this macro (the ones with C<LC> in their names) matched
precisely what C<m/[[:space:]]/> does.  In those releases, the only difference,
in the non-locale variants, was that C<isSPACE()> did not match a vertical tab.
(See L</isPSXSPC> for a macro that matches a vertical tab in all releases.)
See the L<top of this section|/Character classification> for an explanation of
variants
C<isSPACE_A>, C<isSPACE_L1>, C<isSPACE_uvchr>, C<isSPACE_utf8>,
C<isSPACE_utf8_safe>, C<isSPACE_LC>, C<isSPACE_LC_uvchr>, C<isSPACE_LC_utf8>,
and C<isSPACE_LC_utf8_safe>.

=for apidoc Amh|bool|isSPACE_A|int ch
=for apidoc Amh|bool|isSPACE_L1|int ch
=for apidoc Amh|bool|isSPACE_uvchr|int ch
=for apidoc Amh|bool|isSPACE_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isSPACE_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isSPACE_LC|int ch
=for apidoc Amh|bool|isSPACE_LC_uvchr|int ch
=for apidoc Amh|bool|isSPACE_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isPSXSPC|char ch
(short for Posix Space)
Starting in 5.18, this is identical in all its forms to the
corresponding C<isSPACE()> macros.
The locale forms of this macro are identical to their corresponding
C<isSPACE()> forms in all Perl releases.  In releases prior to 5.18, the
non-locale forms differ from their C<isSPACE()> forms only in that the
C<isSPACE()> forms don't match a Vertical Tab, and the C<isPSXSPC()> forms do.
Otherwise they are identical.  Thus this macro is analogous to what
C<m/[[:space:]]/> matches in a regular expression.
See the L<top of this section|/Character classification> for an explanation of
variants C<isPSXSPC_A>, C<isPSXSPC_L1>, C<isPSXSPC_uvchr>, C<isPSXSPC_utf8>,
C<isPSXSPC_utf8_safe>, C<isPSXSPC_LC>, C<isPSXSPC_LC_uvchr>,
C<isPSXSPC_LC_utf8>, and C<isPSXSPC_LC_utf8_safe>.

=for apidoc Amh|bool|isPSXSPC_A|int ch
=for apidoc Amh|bool|isPSXSPC_L1|int ch
=for apidoc Amh|bool|isPSXSPC_uvchr|int ch
=for apidoc Amh|bool|isPSXSPC_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isPSXSPC_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isPSXSPC_LC|int ch
=for apidoc Amh|bool|isPSXSPC_LC_uvchr|int ch
=for apidoc Amh|bool|isPSXSPC_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isUPPER|char ch
Returns a boolean indicating whether the specified character is an
uppercase character, analogous to C<m/[[:upper:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants C<isUPPER_A>, C<isUPPER_L1>, C<isUPPER_uvchr>, C<isUPPER_utf8>,
C<isUPPER_utf8_safe>, C<isUPPER_LC>, C<isUPPER_LC_uvchr>, C<isUPPER_LC_utf8>,
and C<isUPPER_LC_utf8_safe>.

=for apidoc Amh|bool|isUPPER_A|int ch
=for apidoc Amh|bool|isUPPER_L1|int ch
=for apidoc Amh|bool|isUPPER_uvchr|int ch
=for apidoc Amh|bool|isUPPER_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isUPPER_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isUPPER_LC|int ch
=for apidoc Amh|bool|isUPPER_LC_uvchr|int ch
=for apidoc Amh|bool|isUPPER_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isPRINT|char ch
Returns a boolean indicating whether the specified character is a
printable character, analogous to C<m/[[:print:]]/>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isPRINT_A>, C<isPRINT_L1>, C<isPRINT_uvchr>, C<isPRINT_utf8>,
C<isPRINT_utf8_safe>, C<isPRINT_LC>, C<isPRINT_LC_uvchr>, C<isPRINT_LC_utf8>,
and C<isPRINT_LC_utf8_safe>.

=for apidoc Amh|bool|isPRINT_A|int ch
=for apidoc Amh|bool|isPRINT_L1|int ch
=for apidoc Amh|bool|isPRINT_uvchr|int ch
=for apidoc Amh|bool|isPRINT_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isPRINT_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isPRINT_LC|int ch
=for apidoc Amh|bool|isPRINT_LC_uvchr|int ch
=for apidoc Amh|bool|isPRINT_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isWORDCHAR|char ch
Returns a boolean indicating whether the specified character is a character
that is a word character, analogous to what C<m/\w/> and C<m/[[:word:]]/> match
in a regular expression.  A word character is an alphabetic character, a
decimal digit, a connecting punctuation character (such as an underscore), or
a "mark" character that attaches to one of those (like some sort of accent).
C<isALNUM()> is a synonym provided for backward compatibility, even though a
word character includes more than the standard C language meaning of
alphanumeric.
See the L<top of this section|/Character classification> for an explanation of
variants C<isWORDCHAR_A>, C<isWORDCHAR_L1>, C<isWORDCHAR_uvchr>,
C<isWORDCHAR_utf8>, and C<isWORDCHAR_utf8_safe>.  C<isWORDCHAR_LC>,
C<isWORDCHAR_LC_uvchr>, C<isWORDCHAR_LC_utf8>, and C<isWORDCHAR_LC_utf8_safe>
are also as described there, but additionally include the platform's native
underscore.

=for apidoc Amh|bool|isWORDCHAR_A|int ch
=for apidoc Amh|bool|isWORDCHAR_L1|int ch
=for apidoc Amh|bool|isWORDCHAR_uvchr|int ch
=for apidoc Amh|bool|isWORDCHAR_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isWORDCHAR_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isWORDCHAR_LC|int ch
=for apidoc Amh|bool|isWORDCHAR_LC_uvchr|int ch
=for apidoc Amh|bool|isWORDCHAR_LC_utf8_safe|U8 * s| U8 *end
=for apidoc Amh|bool|isALNUM|int ch
=for apidoc Amh|bool|isALNUM_A|int ch
=for apidoc Amh|bool|isALNUM_LC|int ch
=for apidoc Amh|bool|isALNUM_LC_uvchr|int ch

=for apidoc Am|bool|isXDIGIT|char ch
Returns a boolean indicating whether the specified character is a hexadecimal
digit.  In the ASCII range these are C<[0-9A-Fa-f]>.  Variants C<isXDIGIT_A()>
and C<isXDIGIT_L1()> are identical to C<isXDIGIT()>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isXDIGIT_uvchr>, C<isXDIGIT_utf8>, C<isXDIGIT_utf8_safe>, C<isXDIGIT_LC>,
C<isXDIGIT_LC_uvchr>, C<isXDIGIT_LC_utf8>, and C<isXDIGIT_LC_utf8_safe>.

=for apidoc Amh|bool|isXDIGIT_A|int ch
=for apidoc Amh|bool|isXDIGIT_L1|int ch
=for apidoc Amh|bool|isXDIGIT_uvchr|int ch
=for apidoc Amh|bool|isXDIGIT_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isXDIGIT_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isXDIGIT_LC|int ch
=for apidoc Amh|bool|isXDIGIT_LC_uvchr|int ch
=for apidoc Amh|bool|isXDIGIT_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isIDFIRST|char ch
Returns a boolean indicating whether the specified character can be the first
character of an identifier.  This is very close to, but not quite the same as
the official Unicode property C<XID_Start>.  The difference is that this
returns true only if the input character also matches L</isWORDCHAR>.
See the L<top of this section|/Character classification> for an explanation of
variants
C<isIDFIRST_A>, C<isIDFIRST_L1>, C<isIDFIRST_uvchr>, C<isIDFIRST_utf8>,
C<isIDFIRST_utf8_safe>, C<isIDFIRST_LC>, C<isIDFIRST_LC_uvchr>,
C<isIDFIRST_LC_utf8>, and C<isIDFIRST_LC_utf8_safe>.

=for apidoc Amh|bool|isIDFIRST_A|int ch
=for apidoc Amh|bool|isIDFIRST_L1|int ch
=for apidoc Amh|bool|isIDFIRST_uvchr|int ch
=for apidoc Amh|bool|isIDFIRST_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isIDFIRST_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isIDFIRST_LC|int ch
=for apidoc Amh|bool|isIDFIRST_LC_uvchr|int ch
=for apidoc Amh|bool|isIDFIRST_LC_utf8_safe|U8 * s| U8 *end

=for apidoc Am|bool|isIDCONT|char ch
Returns a boolean indicating whether the specified character can be the
second or succeeding character of an identifier.  This is very close to, but
not quite the same as the official Unicode property C<XID_Continue>.  The
difference is that this returns true only if the input character also matches
L</isWORDCHAR>.  See the L<top of this section|/Character classification> for
an explanation of variants C<isIDCONT_A>, C<isIDCONT_L1>, C<isIDCONT_uvchr>,
C<isIDCONT_utf8>, C<isIDCONT_utf8_safe>, C<isIDCONT_LC>, C<isIDCONT_LC_uvchr>,
C<isIDCONT_LC_utf8>, and C<isIDCONT_LC_utf8_safe>.

=for apidoc Amh|bool|isIDCONT_A|int ch
=for apidoc Amh|bool|isIDCONT_L1|int ch
=for apidoc Amh|bool|isIDCONT_uvchr|int ch
=for apidoc Amh|bool|isIDCONT_utf8_safe|U8 * s|U8 * end
=for apidoc Amh|bool|isIDCONT_utf8|U8 * s|U8 * end
=for apidoc Amh|bool|isIDCONT_LC|int ch
=for apidoc Amh|bool|isIDCONT_LC_uvchr|int ch
=for apidoc Amh|bool|isIDCONT_LC_utf8_safe|U8 * s| U8 *end

=head1 Miscellaneous Functions

=for apidoc Am|U8|READ_XDIGIT|char str*
Returns the value of an ASCII-range hex digit and advances the string pointer.
Behaviour is only well defined when isXDIGIT(*str) is true.

=head1 Character case changing
Perl uses "full" Unicode case mappings.  This means that converting a single
character to another case may result in a sequence of more than one character.
For example, the uppercase of C<E<223>> (LATIN SMALL LETTER SHARP S) is the two
character sequence C<SS>.  This presents some complications   The lowercase of
all characters in the range 0..255 is a single character, and thus
C<L</toLOWER_L1>> is furnished.  But, C<toUPPER_L1> can't exist, as it couldn't
return a valid result for all legal inputs.  Instead C<L</toUPPER_uvchr>> has
an API that does allow every possible legal result to be returned.)  Likewise
no other function that is crippled by not being able to give the correct
results for the full range of possible inputs has been implemented here.

=for apidoc Am|U8|toUPPER|int ch
Converts the specified character to uppercase.  If the input is anything but an
ASCII lowercase character, that input character itself is returned.  Variant
C<toUPPER_A> is equivalent.

=for apidoc Am|UV|toUPPER_uvchr|UV cp|U8* s|STRLEN* lenp
Converts the code point C<cp> to its uppercase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  The code
point is interpreted as native if less than 256; otherwise as Unicode.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the uppercase version may be longer than the original character.

The first code point of the uppercased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more.)

=for apidoc Am|UV|toUPPER_utf8|U8* p|U8* e|U8* s|STRLEN* lenp
Converts the first UTF-8 encoded character in the sequence starting at C<p> and
extending no further than S<C<e - 1>> to its uppercase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the uppercase version may be longer than the original character.

The first code point of the uppercased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

It will not attempt to read beyond S<C<e - 1>>, provided that the constraint
S<C<s E<lt> e>> is true (this is asserted for in C<-DDEBUGGING> builds).  If
the UTF-8 for the input character is malformed in some way, the program may
croak, or the function may return the REPLACEMENT CHARACTER, at the discretion
of the implementation, and subject to change in future releases.

=for apidoc Am|UV|toUPPER_utf8_safe|U8* p|U8* e|U8* s|STRLEN* lenp
Same as L</toUPPER_utf8>.

=for apidoc Am|U8|toFOLD|U8 ch
Converts the specified character to foldcase.  If the input is anything but an
ASCII uppercase character, that input character itself is returned.  Variant
C<toFOLD_A> is equivalent.  (There is no equivalent C<to_FOLD_L1> for the full
Latin1 range, as the full generality of L</toFOLD_uvchr> is needed there.)

=for apidoc Am|UV|toFOLD_uvchr|UV cp|U8* s|STRLEN* lenp
Converts the code point C<cp> to its foldcase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  The code
point is interpreted as native if less than 256; otherwise as Unicode.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the foldcase version may be longer than the original character.

The first code point of the foldcased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

=for apidoc Am|UV|toFOLD_utf8|U8* p|U8* e|U8* s|STRLEN* lenp
Converts the first UTF-8 encoded character in the sequence starting at C<p> and
extending no further than S<C<e - 1>> to its foldcase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the foldcase version may be longer than the original character.

The first code point of the foldcased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

It will not attempt
to read beyond S<C<e - 1>>, provided that the constraint S<C<s E<lt> e>> is
true (this is asserted for in C<-DDEBUGGING> builds).  If the UTF-8 for the
input character is malformed in some way, the program may croak, or the
function may return the REPLACEMENT CHARACTER, at the discretion of the
implementation, and subject to change in future releases.

=for apidoc Am|UV|toFOLD_utf8_safe|U8* p|U8* e|U8* s|STRLEN* lenp
Same as L</toFOLD_utf8>.

=for apidoc Am|U8|toLOWER|U8 ch
Converts the specified character to lowercase.  If the input is anything but an
ASCII uppercase character, that input character itself is returned.  Variant
C<toLOWER_A> is equivalent.

=for apidoc Am|U8|toLOWER_L1|U8 ch
Converts the specified Latin1 character to lowercase.  The results are
undefined if the input doesn't fit in a byte.

=for apidoc Am|U8|toLOWER_LC|U8 ch
Converts the specified character to lowercase using the current locale's rules,
if possible; otherwise returns the input character itself.

=for apidoc Am|UV|toLOWER_uvchr|UV cp|U8* s|STRLEN* lenp
Converts the code point C<cp> to its lowercase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  The code
point is interpreted as native if less than 256; otherwise as Unicode.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the lowercase version may be longer than the original character.

The first code point of the lowercased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

=for apidoc Am|UV|toLOWER_utf8|U8* p|U8* e|U8* s|STRLEN* lenp
Converts the first UTF-8 encoded character in the sequence starting at C<p> and
extending no further than S<C<e - 1>> to its lowercase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the lowercase version may be longer than the original character.

The first code point of the lowercased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).
It will not attempt to read beyond S<C<e - 1>>, provided that the constraint
S<C<s E<lt> e>> is true (this is asserted for in C<-DDEBUGGING> builds).  If
the UTF-8 for the input character is malformed in some way, the program may
croak, or the function may return the REPLACEMENT CHARACTER, at the discretion
of the implementation, and subject to change in future releases.

=for apidoc Am|UV|toLOWER_utf8_safe|U8* p|U8* e|U8* s|STRLEN* lenp
Same as L</toLOWER_utf8>.

=for apidoc Am|U8|toTITLE|U8 ch
Converts the specified character to titlecase.  If the input is anything but an
ASCII lowercase character, that input character itself is returned.  Variant
C<toTITLE_A> is equivalent.  (There is no C<toTITLE_L1> for the full Latin1
range, as the full generality of L</toTITLE_uvchr> is needed there.  Titlecase is
not a concept used in locale handling, so there is no functionality for that.)

=for apidoc Am|UV|toTITLE_uvchr|UV cp|U8* s|STRLEN* lenp
Converts the code point C<cp> to its titlecase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  The code
point is interpreted as native if less than 256; otherwise as Unicode.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the titlecase version may be longer than the original character.

The first code point of the titlecased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

=for apidoc Am|UV|toTITLE_utf8|U8* p|U8* e|U8* s|STRLEN* lenp
Converts the first UTF-8 encoded character in the sequence starting at C<p> and
extending no further than S<C<e - 1>> to its titlecase version, and
stores that in UTF-8 in C<s>, and its length in bytes in C<lenp>.  Note
that the buffer pointed to by C<s> needs to be at least C<UTF8_MAXBYTES_CASE+1>
bytes since the titlecase version may be longer than the original character.

The first code point of the titlecased version is returned
(but note, as explained at L<the top of this section|/Character case
changing>, that there may be more).

It will not attempt
to read beyond S<C<e - 1>>, provided that the constraint S<C<s E<lt> e>> is
true (this is asserted for in C<-DDEBUGGING> builds).  If the UTF-8 for the
input character is malformed in some way, the program may croak, or the
function may return the REPLACEMENT CHARACTER, at the discretion of the
implementation, and subject to change in future releases.

=for apidoc Am|UV|toTITLE_utf8_safe|U8* p|U8* e|U8* s|STRLEN* lenp
Same as L</toTITLE_utf8>.

=cut

XXX Still undocumented isVERTWS_uvchr and _utf8; it's unclear what their names
really should be.  Also toUPPER_LC and toFOLD_LC, which are subject to change,
and aren't general purpose as they don't work on U+DF, and assert against that.

Note that these macros are repeated in Devel::PPPort, so should also be
patched there.  The file as of this writing is cpan/Devel-PPPort/parts/inc/misc

*/

/*
   void below because that's the best fit, and works for Devel::PPPort
=for apidoc AmnU|void|WIDEST_UTYPE

Yields the widest unsigned integer type on the platform, currently either
C<U32> or C<64>.  This can be used in declarations such as

 WIDEST_UTYPE my_uv;

or casts

 my_uv = (WIDEST_UTYPE) val;

=cut

*/
#ifdef QUADKIND
#   define WIDEST_UTYPE U64
#else
#   define WIDEST_UTYPE U32
#endif

/* FITS_IN_8_BITS(c) returns true if c doesn't have  a bit set other than in
 * the lower 8.  It is designed to be hopefully bomb-proof, making sure that no
 * bits of information are lost even on a 64-bit machine, but to get the
 * compiler to optimize it out if possible.  This is because Configure makes
 * sure that the machine has an 8-bit byte, so if c is stored in a byte, the
 * sizeof() guarantees that this evaluates to a constant true at compile time.
 *
 * For Coverity, be always true, because otherwise Coverity thinks
 * it finds several expressions that are always true, independent
 * of operands.  Well, they are, but that is kind of the point.
 */
#ifndef __COVERITY__
  /* The '| 0' part ensures a compiler error if c is not integer (like e.g., a
   * pointer) */
#define FITS_IN_8_BITS(c) (   (sizeof(c) == 1)                      \
                           || !(((WIDEST_UTYPE)((c) | 0)) & ~0xFF))
#else
#define FITS_IN_8_BITS(c) (1)
#endif

/* Returns true if l <= c <= (l + n), where 'l' and 'n' are non-negative
 * Written this way so that after optimization, only one conditional test is
 * needed.  (The NV casts stop any warnings about comparison always being true
 * if called with an unsigned.  The cast preserves the sign, which is all we
 * care about.) */
#define withinCOUNT(c, l, n) (__ASSERT_((NV) (l) >= 0)                         \
                              __ASSERT_((NV) (n) >= 0)                         \
   (((WIDEST_UTYPE) (((c)) - ((l) | 0))) <= (((WIDEST_UTYPE) ((n) | 0)))))

/* Returns true if c is in the range l..u, where 'l' is non-negative
 * Written this way so that after optimization, only one conditional test is
 * needed. */
#define inRANGE(c, l, u) (__ASSERT_((u) >= (l))                                \
   (  (sizeof(c) == sizeof(U8))  ? withinCOUNT(((U8)  (c)), (l), ((u) - (l)))  \
    : (sizeof(c) == sizeof(U32)) ? withinCOUNT(((U32) (c)), (l), ((u) - (l)))  \
    : (__ASSERT_(sizeof(c) == sizeof(WIDEST_UTYPE))                            \
                          withinCOUNT(((WIDEST_UTYPE) (c)), (l), ((u) - (l))))))

#ifdef EBCDIC
#   ifndef _ALL_SOURCE
        /* The native libc isascii() et.al. functions return the wrong results
         * on at least z/OS unless this is defined. */
#       error   _ALL_SOURCE should probably be defined
#   endif
#else
    /* There is a simple definition of ASCII for ASCII platforms.  But the
     * EBCDIC one isn't so simple, so is defined using table look-up like the
     * other macros below.
     *
     * The cast here is used instead of '(c) >= 0', because some compilers emit
     * a warning that that test is always true when the parameter is an
     * unsigned type.  khw supposes that it could be written as
     *      && ((c) == '\0' || (c) > 0)
     * to avoid the message, but the cast will likely avoid extra branches even
     * with stupid compilers.
     *
     * The '| 0' part ensures a compiler error if c is not integer (like e.g.,
     * a pointer) */
#   define isASCII(c)    ((WIDEST_UTYPE)((c) | 0) < 128)
#endif

/* Take the eight possible bit patterns of the lower 3 bits and you get the
 * lower 3 bits of the 8 octal digits, in both ASCII and EBCDIC, so those bits
 * can be ignored.  If the rest match '0', we have an octal */
#define isOCTAL_A(c)  (((WIDEST_UTYPE)((c) | 0) & ~7) == '0')

#ifdef H_PERL       /* If have access to perl.h, lookup in its table */

/* Character class numbers.  For internal core Perl use only.  The ones less
 * than 32 are used in PL_charclass[] and the ones up through the one that
 * corresponds to <_HIGHEST_REGCOMP_DOT_H_SYNC> are used by regcomp.h and
 * related files.  PL_charclass ones use names used in l1_char_class_tab.h but
 * their actual definitions are here.  If that file has a name not used here,
 * it won't compile.
 *
 * The first group of these is ordered in what I (khw) estimate to be the
 * frequency of their use.  This gives a slight edge to exiting a loop earlier
 * (in reginclass() in regexec.c).  Except \v should be last, as it isn't a
 * real Posix character class, and some (small) inefficiencies in regular
 * expression handling would be introduced by putting it in the middle of those
 * that are.  Also, cntrl and ascii come after the others as it may be useful
 * to group these which have no members that match above Latin1, (or above
 * ASCII in the latter case) */

#  define _CC_WORDCHAR           0      /* \w and [:word:] */
#  define _CC_DIGIT              1      /* \d and [:digit:] */
#  define _CC_ALPHA              2      /* [:alpha:] */
#  define _CC_LOWER              3      /* [:lower:] */
#  define _CC_UPPER              4      /* [:upper:] */
#  define _CC_PUNCT              5      /* [:punct:] */
#  define _CC_PRINT              6      /* [:print:] */
#  define _CC_ALPHANUMERIC       7      /* [:alnum:] */
#  define _CC_GRAPH              8      /* [:graph:] */
#  define _CC_CASED              9      /* [:lower:] or [:upper:] under /i */
#  define _CC_SPACE             10      /* \s, [:space:] */
#  define _CC_BLANK             11      /* [:blank:] */
#  define _CC_XDIGIT            12      /* [:xdigit:] */
#  define _CC_CNTRL             13      /* [:cntrl:] */
#  define _CC_ASCII             14      /* [:ascii:] */
#  define _CC_VERTSPACE         15      /* \v */

#  define _HIGHEST_REGCOMP_DOT_H_SYNC _CC_VERTSPACE

/* The members of the third group below do not need to be coordinated with data
 * structures in regcomp.[ch] and regexec.c. */
#  define _CC_IDFIRST                  16
#  define _CC_CHARNAME_CONT            17
#  define _CC_NONLATIN1_FOLD           18
#  define _CC_NONLATIN1_SIMPLE_FOLD    19
#  define _CC_QUOTEMETA                20
#  define _CC_NON_FINAL_FOLD           21
#  define _CC_IS_IN_SOME_FOLD          22
#  define _CC_BINDIGIT                 23
#  define _CC_OCTDIGIT                 24
#  define _CC_MNEMONIC_CNTRL           25

/* This next group is only used on EBCDIC platforms, so theoretically could be
 * shared with something entirely different that's only on ASCII platforms */
#  define _CC_UTF8_START_BYTE_IS_FOR_AT_LEAST_SURROGATE 31
/* Unused: 24-30
 * If more bits are needed, one could add a second word for non-64bit
 * QUAD_IS_INT systems, using some #ifdefs to distinguish between having a 2nd
 * word or not.  The IS_IN_SOME_FOLD bit is the most easily expendable, as it
 * is used only for optimization (as of this writing), and differs in the
 * Latin1 range from the ALPHA bit only in two relatively unimportant
 * characters: the masculine and feminine ordinal indicators, so removing it
 * would just cause /i regexes which match them to run less efficiently.
 * Similarly the EBCDIC-only bits are used just for speed, and could be
 * replaced by other means */

#if defined(PERL_CORE) || defined(PERL_EXT)
/* An enum version of the character class numbers, to help compilers
 * optimize */
typedef enum {
    _CC_ENUM_ALPHA          = _CC_ALPHA,
    _CC_ENUM_ALPHANUMERIC   = _CC_ALPHANUMERIC,
    _CC_ENUM_ASCII          = _CC_ASCII,
    _CC_ENUM_BLANK          = _CC_BLANK,
    _CC_ENUM_CASED          = _CC_CASED,
    _CC_ENUM_CNTRL          = _CC_CNTRL,
    _CC_ENUM_DIGIT          = _CC_DIGIT,
    _CC_ENUM_GRAPH          = _CC_GRAPH,
    _CC_ENUM_LOWER          = _CC_LOWER,
    _CC_ENUM_PRINT          = _CC_PRINT,
    _CC_ENUM_PUNCT          = _CC_PUNCT,
    _CC_ENUM_SPACE          = _CC_SPACE,
    _CC_ENUM_UPPER          = _CC_UPPER,
    _CC_ENUM_VERTSPACE      = _CC_VERTSPACE,
    _CC_ENUM_WORDCHAR       = _CC_WORDCHAR,
    _CC_ENUM_XDIGIT         = _CC_XDIGIT
} _char_class_number;
#endif

#define POSIX_CC_COUNT    (_HIGHEST_REGCOMP_DOT_H_SYNC + 1)

START_EXTERN_C
#  ifdef DOINIT
EXTCONST  U32 PL_charclass[] = {
#    include "l1_char_class_tab.h"
};

#  else /* ! DOINIT */
EXTCONST U32 PL_charclass[];
#  endif
END_EXTERN_C

    /* The 1U keeps Solaris from griping when shifting sets the uppermost bit */
#   define _CC_mask(classnum) (1U << (classnum))

    /* For internal core Perl use only: the base macro for defining macros like
     * isALPHA */
#   define _generic_isCC(c, classnum) cBOOL(FITS_IN_8_BITS(c)    \
                && (PL_charclass[(U8) (c)] & _CC_mask(classnum)))

    /* The mask for the _A versions of the macros; it just adds in the bit for
     * ASCII. */
#   define _CC_mask_A(classnum) (_CC_mask(classnum) | _CC_mask(_CC_ASCII))

    /* For internal core Perl use only: the base macro for defining macros like
     * isALPHA_A.  The foo_A version makes sure that both the desired bit and
     * the ASCII bit are present */
#   define _generic_isCC_A(c, classnum) (FITS_IN_8_BITS(c)      \
        && ((PL_charclass[(U8) (c)] & _CC_mask_A(classnum))     \
                                   == _CC_mask_A(classnum)))

/* On ASCII platforms certain classes form a single range.  It's faster to
 * special case these.  isDIGIT is a single range on all platforms */
#   ifdef EBCDIC
#     define isALPHA_A(c)  _generic_isCC_A(c, _CC_ALPHA)
#     define isGRAPH_A(c)  _generic_isCC_A(c, _CC_GRAPH)
#     define isLOWER_A(c)  _generic_isCC_A(c, _CC_LOWER)
#     define isPRINT_A(c)  _generic_isCC_A(c, _CC_PRINT)
#     define isUPPER_A(c)  _generic_isCC_A(c, _CC_UPPER)
#   else
      /* By folding the upper and lowercase, we can use a single range */
#     define isALPHA_A(c)  inRANGE((~('A' ^ 'a') & (c)), 'A', 'Z')
#     define isGRAPH_A(c)  inRANGE(c, ' ' + 1, 0x7e)
#     define isLOWER_A(c)  inRANGE(c, 'a', 'z')
#     define isPRINT_A(c)  inRANGE(c, ' ', 0x7e)
#     define isUPPER_A(c)  inRANGE(c, 'A', 'Z')
#   endif
#   define isALPHANUMERIC_A(c) _generic_isCC_A(c, _CC_ALPHANUMERIC)
#   define isBLANK_A(c)  _generic_isCC_A(c, _CC_BLANK)
#   define isCNTRL_A(c)  _generic_isCC_A(c, _CC_CNTRL)
#   define isDIGIT_A(c)  inRANGE(c, '0', '9')
#   define isPUNCT_A(c)  _generic_isCC_A(c, _CC_PUNCT)
#   define isSPACE_A(c)  _generic_isCC_A(c, _CC_SPACE)
#   define isWORDCHAR_A(c) _generic_isCC_A(c, _CC_WORDCHAR)
#   define isXDIGIT_A(c)  _generic_isCC(c, _CC_XDIGIT) /* No non-ASCII xdigits
                                                        */
#   define isIDFIRST_A(c) _generic_isCC_A(c, _CC_IDFIRST)
#   define isALPHA_L1(c)  _generic_isCC(c, _CC_ALPHA)
#   define isALPHANUMERIC_L1(c) _generic_isCC(c, _CC_ALPHANUMERIC)
#   define isBLANK_L1(c)  _generic_isCC(c, _CC_BLANK)

    /* continuation character for legal NAME in \N{NAME} */
#   define isCHARNAME_CONT(c) _generic_isCC(c, _CC_CHARNAME_CONT)

#   define isCNTRL_L1(c)  _generic_isCC(c, _CC_CNTRL)
#   define isGRAPH_L1(c)  _generic_isCC(c, _CC_GRAPH)
#   define isLOWER_L1(c)  _generic_isCC(c, _CC_LOWER)
#   define isPRINT_L1(c)  _generic_isCC(c, _CC_PRINT)
#   define isPSXSPC_L1(c)  isSPACE_L1(c)
#   define isPUNCT_L1(c)  _generic_isCC(c, _CC_PUNCT)
#   define isSPACE_L1(c)  _generic_isCC(c, _CC_SPACE)
#   define isUPPER_L1(c)  _generic_isCC(c, _CC_UPPER)
#   define isWORDCHAR_L1(c) _generic_isCC(c, _CC_WORDCHAR)
#   define isIDFIRST_L1(c) _generic_isCC(c, _CC_IDFIRST)

#   ifdef EBCDIC
#       define isASCII(c) _generic_isCC(c, _CC_ASCII)
#   endif

    /* Participates in a single-character fold with a character above 255 */
#   define _HAS_NONLATIN1_SIMPLE_FOLD_CLOSURE_ONLY_FOR_USE_BY_REGCOMP_DOT_C_AND_REGEXEC_DOT_C(c) ((! cBOOL(FITS_IN_8_BITS(c))) || (PL_charclass[(U8) (c)] & _CC_mask(_CC_NONLATIN1_SIMPLE_FOLD)))

    /* Like the above, but also can be part of a multi-char fold */
#   define _HAS_NONLATIN1_FOLD_CLOSURE_ONLY_FOR_USE_BY_REGCOMP_DOT_C_AND_REGEXEC_DOT_C(c) ((! cBOOL(FITS_IN_8_BITS(c))) || (PL_charclass[(U8) (c)] & _CC_mask(_CC_NONLATIN1_FOLD)))

#   define _isQUOTEMETA(c) _generic_isCC(c, _CC_QUOTEMETA)
#   define _IS_NON_FINAL_FOLD_ONLY_FOR_USE_BY_REGCOMP_DOT_C(c) \
                                           _generic_isCC(c, _CC_NON_FINAL_FOLD)
#   define _IS_IN_SOME_FOLD_ONLY_FOR_USE_BY_REGCOMP_DOT_C(c) \
                                           _generic_isCC(c, _CC_IS_IN_SOME_FOLD)

/* is c a control character for which we have a mnemonic? */
#  if defined(PERL_CORE) || defined(PERL_EXT)
#     define isMNEMONIC_CNTRL(c) _generic_isCC(c, _CC_MNEMONIC_CNTRL)
#  endif
#else   /* else we don't have perl.h H_PERL */

    /* If we don't have perl.h, we are compiling a utility program.  Below we
     * hard-code various macro definitions that wouldn't otherwise be available
     * to it. Most are coded based on first principles.  These are written to
     * avoid EBCDIC vs. ASCII #ifdef's as much as possible. */
#   define isDIGIT_A(c)  inRANGE(c, '0', '9')
#   define isBLANK_A(c)  ((c) == ' ' || (c) == '\t')
#   define isSPACE_A(c)  (isBLANK_A(c)                                   \
                          || (c) == '\n'                                 \
                          || (c) == '\r'                                 \
                          || (c) == '\v'                                 \
                          || (c) == '\f')
    /* On EBCDIC, there are gaps between 'i' and 'j'; 'r' and 's'.  Same for
     * uppercase.  The tests for those aren't necessary on ASCII, but hurt only
     * performance (if optimization isn't on), and allow the same code to be
     * used for both platform types */
#   define isLOWER_A(c)  inRANGE((c), 'a', 'i')                         \
                      || inRANGE((c), 'j', 'r')                         \
                      || inRANGE((c), 's', 'z')
#   define isUPPER_A(c)  inRANGE((c), 'A', 'I')                         \
                      || inRANGE((c), 'J', 'R')                         \
                      || inRANGE((c), 'S', 'Z')
#   define isALPHA_A(c)  (isUPPER_A(c) || isLOWER_A(c))
#   define isALPHANUMERIC_A(c) (isALPHA_A(c) || isDIGIT_A(c))
#   define isWORDCHAR_A(c)   (isALPHANUMERIC_A(c) || (c) == '_')
#   define isIDFIRST_A(c)    (isALPHA_A(c) || (c) == '_')
#   define isXDIGIT_A(c) (   isDIGIT_A(c)                               \
                          || inRANGE((c), 'a', 'f')                     \
                          || inRANGE((c), 'A', 'F')
#   define isPUNCT_A(c)  ((c) == '-' || (c) == '!' || (c) == '"'        \
                       || (c) == '#' || (c) == '$' || (c) == '%'        \
                       || (c) == '&' || (c) == '\'' || (c) == '('       \
                       || (c) == ')' || (c) == '*' || (c) == '+'        \
                       || (c) == ',' || (c) == '.' || (c) == '/'        \
                       || (c) == ':' || (c) == ';' || (c) == '<'        \
                       || (c) == '=' || (c) == '>' || (c) == '?'        \
                       || (c) == '@' || (c) == '[' || (c) == '\\'       \
                       || (c) == ']' || (c) == '^' || (c) == '_'        \
                       || (c) == '`' || (c) == '{' || (c) == '|'        \
                       || (c) == '}' || (c) == '~')
#   define isGRAPH_A(c)  (isALPHANUMERIC_A(c) || isPUNCT_A(c))
#   define isPRINT_A(c)  (isGRAPH_A(c) || (c) == ' ')

#   ifdef EBCDIC
        /* The below is accurate for the 3 EBCDIC code pages traditionally
         * supported by perl.  The only difference between them in the controls
         * is the position of \n, and that is represented symbolically below */
#       define isCNTRL_A(c)  ((c) == '\0' || (c) == '\a' || (c) == '\b'     \
                          ||  (c) == '\f' || (c) == '\n' || (c) == '\r'     \
                          ||  (c) == '\t' || (c) == '\v'                    \
                          || inRANGE((c), 1, 3)     /* SOH, STX, ETX */     \
                          ||  (c) == 7F   /* U+7F DEL */                    \
                          || inRANGE((c), 0x0E, 0x13) /* SO SI DLE          \
                                                         DC[1-3] */         \
                          ||  (c) == 0x18 /* U+18 CAN */                    \
                          ||  (c) == 0x19 /* U+19 EOM */                    \
                          || inRANGE((c), 0x1C, 0x1F) /* [FGRU]S */         \
                          ||  (c) == 0x26 /* U+17 ETB */                    \
                          ||  (c) == 0x27 /* U+1B ESC */                    \
                          ||  (c) == 0x2D /* U+05 ENQ */                    \
                          ||  (c) == 0x2E /* U+06 ACK */                    \
                          ||  (c) == 0x32 /* U+16 SYN */                    \
                          ||  (c) == 0x37 /* U+04 EOT */                    \
                          ||  (c) == 0x3C /* U+14 DC4 */                    \
                          ||  (c) == 0x3D /* U+15 NAK */                    \
                          ||  (c) == 0x3F)/* U+1A SUB */
#       define isASCII(c)    (isCNTRL_A(c) || isPRINT_A(c))
#   else /* isASCII is already defined for ASCII platforms, so can use that to
            define isCNTRL */
#       define isCNTRL_A(c)  (isASCII(c) && ! isPRINT_A(c))
#   endif

    /* The _L1 macros may be unnecessary for the utilities; I (khw) added them
     * during debugging, and it seems best to keep them.  We may be called
     * without NATIVE_TO_LATIN1 being defined.  On ASCII platforms, it doesn't
     * do anything anyway, so make it not a problem */
#   if ! defined(EBCDIC) && ! defined(NATIVE_TO_LATIN1)
#       define NATIVE_TO_LATIN1(ch) (ch)
#   endif
#   define isALPHA_L1(c)     (isUPPER_L1(c) || isLOWER_L1(c))
#   define isALPHANUMERIC_L1(c) (isALPHA_L1(c) || isDIGIT_A(c))
#   define isBLANK_L1(c)     (isBLANK_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && NATIVE_TO_LATIN1((U8) c) == 0xA0))
#   define isCNTRL_L1(c)     (FITS_IN_8_BITS(c) && (! isPRINT_L1(c)))
#   define isGRAPH_L1(c)     (isPRINT_L1(c) && (! isBLANK_L1(c)))
#   define isLOWER_L1(c)     (isLOWER_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && ((   NATIVE_TO_LATIN1((U8) c) >= 0xDF   \
                                       && NATIVE_TO_LATIN1((U8) c) != 0xF7)  \
                                       || NATIVE_TO_LATIN1((U8) c) == 0xAA   \
                                       || NATIVE_TO_LATIN1((U8) c) == 0xBA   \
                                       || NATIVE_TO_LATIN1((U8) c) == 0xB5)))
#   define isPRINT_L1(c)     (isPRINT_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && NATIVE_TO_LATIN1((U8) c) >= 0xA0))
#   define isPUNCT_L1(c)     (isPUNCT_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && (   NATIVE_TO_LATIN1((U8) c) == 0xA1    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xA7    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xAB    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xB6    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xB7    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xBB    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xBF)))
#   define isSPACE_L1(c)     (isSPACE_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && (   NATIVE_TO_LATIN1((U8) c) == 0x85    \
                                      || NATIVE_TO_LATIN1((U8) c) == 0xA0)))
#   define isUPPER_L1(c)     (isUPPER_A(c)                                   \
                              || (FITS_IN_8_BITS(c)                          \
                                  && (   IN_RANGE(NATIVE_TO_LATIN1((U8) c),  \
                                                  0xC0, 0xDE)                \
                                      && NATIVE_TO_LATIN1((U8) c) != 0xD7)))
#   define isWORDCHAR_L1(c)  (isIDFIRST_L1(c) || isDIGIT_A(c))
#   define isIDFIRST_L1(c)   (isALPHA_L1(c) || NATIVE_TO_LATIN1(c) == '_')
#   define isCHARNAME_CONT(c) (isWORDCHAR_L1(c)                              \
                               || isBLANK_L1(c)                              \
                               || (c) == '-'                                 \
                               || (c) == '('                                 \
                               || (c) == ')')
    /* The following are not fully accurate in the above-ASCII range.  I (khw)
     * don't think it's necessary to be so for the purposes where this gets
     * compiled */
#   define _isQUOTEMETA(c)      (FITS_IN_8_BITS(c) && ! isWORDCHAR_L1(c))
#   define _IS_IN_SOME_FOLD_ONLY_FOR_USE_BY_REGCOMP_DOT_C(c) isALPHA_L1(c)

    /*  And these aren't accurate at all.  They are useful only for above
     *  Latin1, which utilities and bootstrapping don't deal with */
#   define _IS_NON_FINAL_FOLD_ONLY_FOR_USE_BY_REGCOMP_DOT_C(c) 0
#   define _HAS_NONLATIN1_SIMPLE_FOLD_CLOSURE_ONLY_FOR_USE_BY_REGCOMP_DOT_C_AND_REGEXEC_DOT_C(c) 0
#   define _HAS_NONLATIN1_FOLD_CLOSURE_ONLY_FOR_USE_BY_REGCOMP_DOT_C_AND_REGEXEC_DOT_C(c) 0

    /* Many of the macros later in this file are defined in terms of these.  By
     * implementing them with a function, which converts the class number into
     * a call to the desired macro, all of the later ones work.  However, that
     * function won't be actually defined when building a utility program (no
     * perl.h), and so a compiler error will be generated if one is attempted
     * to be used.  And the above-Latin1 code points require Unicode tables to
     * be present, something unlikely to be the case when bootstrapping */
#   define _generic_isCC(c, classnum)                                        \
         (FITS_IN_8_BITS(c) && S_bootstrap_ctype((U8) (c), (classnum), TRUE))
#   define _generic_isCC_A(c, classnum)                                      \
         (FITS_IN_8_BITS(c) && S_bootstrap_ctype((U8) (c), (classnum), FALSE))
#endif  /* End of no perl.h H_PERL */

#define isALPHANUMERIC(c)  isALPHANUMERIC_A(c)
#define isALPHA(c)   isALPHA_A(c)
#define isASCII_A(c)  isASCII(c)
#define isASCII_L1(c)  isASCII(c)
#define isBLANK(c)   isBLANK_A(c)
#define isCNTRL(c)   isCNTRL_A(c)
#define isDIGIT(c)   isDIGIT_A(c)
#define isGRAPH(c)   isGRAPH_A(c)
#define isIDFIRST(c) isIDFIRST_A(c)
#define isLOWER(c)   isLOWER_A(c)
#define isPRINT(c)   isPRINT_A(c)
#define isPSXSPC_A(c) isSPACE_A(c)
#define isPSXSPC(c)  isPSXSPC_A(c)
#define isPSXSPC_L1(c) isSPACE_L1(c)
#define isPUNCT(c)   isPUNCT_A(c)
#define isSPACE(c)   isSPACE_A(c)
#define isUPPER(c)   isUPPER_A(c)
#define isWORDCHAR(c) isWORDCHAR_A(c)
#define isXDIGIT(c)  isXDIGIT_A(c)

/* ASCII casing.  These could also be written as
    #define toLOWER(c) (isASCII(c) ? toLOWER_LATIN1(c) : (c))
    #define toUPPER(c) (isASCII(c) ? toUPPER_LATIN1_MOD(c) : (c))
   which uses table lookup and mask instead of subtraction.  (This would
   work because the _MOD does not apply in the ASCII range).

   These actually are UTF-8 invariant casing, not just ASCII, as any non-ASCII
   UTF-8 invariants are neither upper nor lower.  (Only on EBCDIC platforms are
   there non-ASCII invariants, and all of them are controls.) */
#define toLOWER(c)  (isUPPER(c) ? (U8)((c) + ('a' - 'A')) : (c))
#define toUPPER(c)  (isLOWER(c) ? (U8)((c) - ('a' - 'A')) : (c))

/* In the ASCII range, these are equivalent to what they're here defined to be.
 * But by creating these definitions, other code doesn't have to be aware of
 * this detail.  Actually this works for all UTF-8 invariants, not just the
 * ASCII range. (EBCDIC platforms can have non-ASCII invariants.) */
#define toFOLD(c)    toLOWER(c)
#define toTITLE(c)   toUPPER(c)

#define toLOWER_A(c) toLOWER(c)
#define toUPPER_A(c) toUPPER(c)
#define toFOLD_A(c)  toFOLD(c)
#define toTITLE_A(c) toTITLE(c)

/* Use table lookup for speed; returns the input itself if is out-of-range */
#define toLOWER_LATIN1(c)    ((! FITS_IN_8_BITS(c))                        \
                             ? (c)                                         \
                             : PL_latin1_lc[ (U8) (c) ])
#define toLOWER_L1(c)    toLOWER_LATIN1(c)  /* Synonym for consistency */

/* Modified uc.  Is correct uc except for three non-ascii chars which are
 * all mapped to one of them, and these need special handling; returns the
 * input itself if is out-of-range */
#define toUPPER_LATIN1_MOD(c) ((! FITS_IN_8_BITS(c))                       \
                               ? (c)                                       \
                               : PL_mod_latin1_uc[ (U8) (c) ])
#define IN_UTF8_CTYPE_LOCALE PL_in_utf8_CTYPE_locale

/* Use foo_LC_uvchr() instead  of these for beyond the Latin1 range */

/* For internal core Perl use only: the base macro for defining macros like
 * isALPHA_LC, which uses the current LC_CTYPE locale.  'c' is the code point
 * (0-255) to check.  In a UTF-8 locale, the result is the same as calling
 * isFOO_L1(); the 'utf8_locale_classnum' parameter is something like
 * _CC_UPPER, which gives the class number for doing this.  For non-UTF-8
 * locales, the code to actually do the test this is passed in 'non_utf8'.  If
 * 'c' is above 255, 0 is returned.  For accessing the full range of possible
 * code points under locale rules, use the macros based on _generic_LC_uvchr
 * instead of this. */
#define _generic_LC_base(c, utf8_locale_classnum, non_utf8)                    \
           (! FITS_IN_8_BITS(c)                                                \
           ? 0                                                                 \
           : IN_UTF8_CTYPE_LOCALE                                              \
             ? cBOOL(PL_charclass[(U8) (c)] & _CC_mask(utf8_locale_classnum))  \
             : cBOOL(non_utf8))

/* For internal core Perl use only: a helper macro for defining macros like
 * isALPHA_LC.  'c' is the code point (0-255) to check.  The function name to
 * actually do this test is passed in 'non_utf8_func', which is called on 'c',
 * casting 'c' to the macro _LC_CAST, which should not be parenthesized.  See
 * _generic_LC_base for more info */
#define _generic_LC(c, utf8_locale_classnum, non_utf8_func)                    \
                        _generic_LC_base(c,utf8_locale_classnum,               \
                                         non_utf8_func( (_LC_CAST) (c)))

/* For internal core Perl use only: like _generic_LC, but also returns TRUE if
 * 'c' is the platform's native underscore character */
#define _generic_LC_underscore(c,utf8_locale_classnum,non_utf8_func)           \
                        _generic_LC_base(c, utf8_locale_classnum,              \
                                         (non_utf8_func( (_LC_CAST) (c))       \
                                          || (char)(c) == '_'))

/* These next three are also for internal core Perl use only: case-change
 * helper macros.  The reason for using the PL_latin arrays is in case the
 * system function is defective; it ensures uniform results that conform to the
 * Unicod standard.   It does not handle the anomalies in UTF-8 Turkic locales */
#define _generic_toLOWER_LC(c, function, cast)  (! FITS_IN_8_BITS(c)           \
                                                ? (c)                          \
                                                : (IN_UTF8_CTYPE_LOCALE)       \
                                                  ? PL_latin1_lc[ (U8) (c) ]   \
                                                  : (cast)function((cast)(c)))

/* Note that the result can be larger than a byte in a UTF-8 locale.  It
 * returns a single value, so can't adequately return the upper case of LATIN
 * SMALL LETTER SHARP S in a UTF-8 locale (which should be a string of two
 * values "SS");  instead it asserts against that under DEBUGGING, and
 * otherwise returns its input.  It does not handle the anomalies in UTF-8
 * Turkic locales. */
#define _generic_toUPPER_LC(c, function, cast)                                 \
                    (! FITS_IN_8_BITS(c)                                       \
                    ? (c)                                                      \
                    : ((! IN_UTF8_CTYPE_LOCALE)                                \
                      ? (cast)function((cast)(c))                              \
                      : ((((U8)(c)) == MICRO_SIGN)                             \
                        ? GREEK_CAPITAL_LETTER_MU                              \
                        : ((((U8)(c)) == LATIN_SMALL_LETTER_Y_WITH_DIAERESIS)  \
                          ? LATIN_CAPITAL_LETTER_Y_WITH_DIAERESIS              \
                          : ((((U8)(c)) == LATIN_SMALL_LETTER_SHARP_S)         \
                            ? (__ASSERT_(0) (c))                               \
                            : PL_mod_latin1_uc[ (U8) (c) ])))))

/* Note that the result can be larger than a byte in a UTF-8 locale.  It
 * returns a single value, so can't adequately return the fold case of LATIN
 * SMALL LETTER SHARP S in a UTF-8 locale (which should be a string of two
 * values "ss"); instead it asserts against that under DEBUGGING, and
 * otherwise returns its input.  It does not handle the anomalies in UTF-8
 * Turkic locales */
#define _generic_toFOLD_LC(c, function, cast)                                  \
                    ((UNLIKELY((c) == MICRO_SIGN) && IN_UTF8_CTYPE_LOCALE)     \
                      ? GREEK_SMALL_LETTER_MU                                  \
                      : (__ASSERT_(! IN_UTF8_CTYPE_LOCALE                      \
                                   || (c) != LATIN_SMALL_LETTER_SHARP_S)       \
                         _generic_toLOWER_LC(c, function, cast)))

/* Use the libc versions for these if available. */
#if defined(HAS_ISASCII)
#   define isASCII_LC(c) (FITS_IN_8_BITS(c) && isascii( (U8) (c)))
#else
#   define isASCII_LC(c) isASCII(c)
#endif

#if defined(HAS_ISBLANK)
#   define isBLANK_LC(c) _generic_LC(c, _CC_BLANK, isblank)
#else /* Unlike isASCII, varies if in a UTF-8 locale */
#   define isBLANK_LC(c) ((IN_UTF8_CTYPE_LOCALE) ? isBLANK_L1(c) : isBLANK(c))
#endif

#define _LC_CAST U8

#ifdef WIN32
    /* The Windows functions don't bother to follow the POSIX standard, which
     * for example says that something can't both be a printable and a control.
     * But Windows treats the \t control as a printable, and does such things
     * as making superscripts into both digits and punctuation.  This tames
     * these flaws by assuming that the definitions of both controls and space
     * are correct, and then making sure that other definitions don't have
     * weirdnesses, by making sure that isalnum() isn't also ispunct(), etc.
     * Not all possible weirdnesses are checked for, just the ones that were
     * detected on actual Microsoft code pages */

#  define isCNTRL_LC(c)  _generic_LC(c, _CC_CNTRL, iscntrl)
#  define isSPACE_LC(c)  _generic_LC(c, _CC_SPACE, isspace)

#  define isALPHA_LC(c)  (_generic_LC(c, _CC_ALPHA, isalpha)                  \
                                                    && isALPHANUMERIC_LC(c))
#  define isALPHANUMERIC_LC(c)  (_generic_LC(c, _CC_ALPHANUMERIC, isalnum) && \
                                                              ! isPUNCT_LC(c))
#  define isDIGIT_LC(c)  (_generic_LC(c, _CC_DIGIT, isdigit) &&               \
                                                         isALPHANUMERIC_LC(c))
#  define isGRAPH_LC(c)  (_generic_LC(c, _CC_GRAPH, isgraph) && isPRINT_LC(c))
#  define isIDFIRST_LC(c) (((c) == '_')                                       \
                 || (_generic_LC(c, _CC_IDFIRST, isalpha) && ! isPUNCT_LC(c)))
#  define isLOWER_LC(c)  (_generic_LC(c, _CC_LOWER, islower) && isALPHA_LC(c))
#  define isPRINT_LC(c)  (_generic_LC(c, _CC_PRINT, isprint) && ! isCNTRL_LC(c))
#  define isPUNCT_LC(c)  (_generic_LC(c, _CC_PUNCT, ispunct) && ! isCNTRL_LC(c))
#  define isUPPER_LC(c)  (_generic_LC(c, _CC_UPPER, isupper) && isALPHA_LC(c))
#  define isWORDCHAR_LC(c) (((c) == '_') || isALPHANUMERIC_LC(c))
#  define isXDIGIT_LC(c) (_generic_LC(c, _CC_XDIGIT, isxdigit)                \
                                                    && isALPHANUMERIC_LC(c))

#  define toLOWER_LC(c) _generic_toLOWER_LC((c), tolower, U8)
#  define toUPPER_LC(c) _generic_toUPPER_LC((c), toupper, U8)
#  define toFOLD_LC(c)  _generic_toFOLD_LC((c), tolower, U8)

#elif defined(CTYPE256) || (!defined(isascii) && !defined(HAS_ISASCII))
    /* For most other platforms */

#  define isALPHA_LC(c)   _generic_LC(c, _CC_ALPHA, isalpha)
#  define isALPHANUMERIC_LC(c)  _generic_LC(c, _CC_ALPHANUMERIC, isalnum)
#  define isCNTRL_LC(c)    _generic_LC(c, _CC_CNTRL, iscntrl)
#  define isDIGIT_LC(c)    _generic_LC(c, _CC_DIGIT, isdigit)
#  define isGRAPH_LC(c)    _generic_LC(c, _CC_GRAPH, isgraph)
#  define isIDFIRST_LC(c)  _generic_LC_underscore(c, _CC_IDFIRST, isalpha)
#  define isLOWER_LC(c)    _generic_LC(c, _CC_LOWER, islower)
#  define isPRINT_LC(c)    _generic_LC(c, _CC_PRINT, isprint)
#  define isPUNCT_LC(c)    _generic_LC(c, _CC_PUNCT, ispunct)
#  define isSPACE_LC(c)    _generic_LC(c, _CC_SPACE, isspace)
#  define isUPPER_LC(c)    _generic_LC(c, _CC_UPPER, isupper)
#  define isWORDCHAR_LC(c) _generic_LC_underscore(c, _CC_WORDCHAR, isalnum)
#  define isXDIGIT_LC(c)   _generic_LC(c, _CC_XDIGIT, isxdigit)


#  define toLOWER_LC(c) _generic_toLOWER_LC((c), tolower, U8)
#  define toUPPER_LC(c) _generic_toUPPER_LC((c), toupper, U8)
#  define toFOLD_LC(c)  _generic_toFOLD_LC((c), tolower, U8)

#else  /* The final fallback position */

#  define isALPHA_LC(c)	        (isascii(c) && isalpha(c))
#  define isALPHANUMERIC_LC(c)  (isascii(c) && isalnum(c))
#  define isCNTRL_LC(c)	        (isascii(c) && iscntrl(c))
#  define isDIGIT_LC(c)	        (isascii(c) && isdigit(c))
#  define isGRAPH_LC(c)	        (isascii(c) && isgraph(c))
#  define isIDFIRST_LC(c)	(isascii(c) && (isalpha(c) || (c) == '_'))
#  define isLOWER_LC(c)	        (isascii(c) && islower(c))
#  define isPRINT_LC(c)	        (isascii(c) && isprint(c))
#  define isPUNCT_LC(c)	        (isascii(c) && ispunct(c))
#  define isSPACE_LC(c)	        (isascii(c) && isspace(c))
#  define isUPPER_LC(c)	        (isascii(c) && isupper(c))
#  define isWORDCHAR_LC(c)	(isascii(c) && (isalnum(c) || (c) == '_'))
#  define isXDIGIT_LC(c)        (isascii(c) && isxdigit(c))

#  define toLOWER_LC(c)	(isascii(c) ? tolower(c) : (c))
#  define toUPPER_LC(c)	(isascii(c) ? toupper(c) : (c))
#  define toFOLD_LC(c)	(isascii(c) ? tolower(c) : (c))

#endif

#define isIDCONT(c)             isWORDCHAR(c)
#define isIDCONT_A(c)           isWORDCHAR_A(c)
#define isIDCONT_L1(c)	        isWORDCHAR_L1(c)
#define isIDCONT_LC(c)	        isWORDCHAR_LC(c)
#define isPSXSPC_LC(c)		isSPACE_LC(c)

/* For internal core Perl use only: the base macros for defining macros like
 * isALPHA_uvchr.  'c' is the code point to check.  'classnum' is the POSIX class
 * number defined earlier in this file.  _generic_uvchr() is used for POSIX
 * classes where there is a macro or function 'above_latin1' that takes the
 * single argument 'c' and returns the desired value.  These exist for those
 * classes which have simple definitions, avoiding the overhead of an inversion
 * list binary search.  _generic_invlist_uvchr() can be used
 * for classes where that overhead is faster than a direct lookup.
 * _generic_uvchr() won't compile if 'c' isn't unsigned, as it won't match the
 * 'above_latin1' prototype. _generic_isCC() macro does bounds checking, so
 * have duplicate checks here, so could create versions of the macros that
 * don't, but experiments show that gcc optimizes them out anyway. */

/* Note that all ignore 'use bytes' */
#define _generic_uvchr(classnum, above_latin1, c) ((c) < 256                \
                                             ? _generic_isCC(c, classnum)   \
                                             : above_latin1(c))
#define _generic_invlist_uvchr(classnum, c) ((c) < 256                        \
                                             ? _generic_isCC(c, classnum)   \
                                             : _is_uni_FOO(classnum, c))
#define isALPHA_uvchr(c)      _generic_invlist_uvchr(_CC_ALPHA, c)
#define isALPHANUMERIC_uvchr(c) _generic_invlist_uvchr(_CC_ALPHANUMERIC, c)
#define isASCII_uvchr(c)      isASCII(c)
#define isBLANK_uvchr(c)      _generic_uvchr(_CC_BLANK, is_HORIZWS_cp_high, c)
#define isCNTRL_uvchr(c)      isCNTRL_L1(c) /* All controls are in Latin1 */
#define isDIGIT_uvchr(c)      _generic_invlist_uvchr(_CC_DIGIT, c)
#define isGRAPH_uvchr(c)      _generic_invlist_uvchr(_CC_GRAPH, c)
#define isIDCONT_uvchr(c)                                                   \
                    _generic_uvchr(_CC_WORDCHAR, _is_uni_perl_idcont, c)
#define isIDFIRST_uvchr(c)                                                  \
                    _generic_uvchr(_CC_IDFIRST, _is_uni_perl_idstart, c)
#define isLOWER_uvchr(c)      _generic_invlist_uvchr(_CC_LOWER, c)
#define isPRINT_uvchr(c)      _generic_invlist_uvchr(_CC_PRINT, c)

#define isPUNCT_uvchr(c)      _generic_invlist_uvchr(_CC_PUNCT, c)
#define isSPACE_uvchr(c)      _generic_uvchr(_CC_SPACE, is_XPERLSPACE_cp_high, c)
#define isPSXSPC_uvchr(c)     isSPACE_uvchr(c)

#define isUPPER_uvchr(c)      _generic_invlist_uvchr(_CC_UPPER, c)
#define isVERTWS_uvchr(c)     _generic_uvchr(_CC_VERTSPACE, is_VERTWS_cp_high, c)
#define isWORDCHAR_uvchr(c)   _generic_invlist_uvchr(_CC_WORDCHAR, c)
#define isXDIGIT_uvchr(c)     _generic_uvchr(_CC_XDIGIT, is_XDIGIT_cp_high, c)

#define toFOLD_uvchr(c,s,l)	to_uni_fold(c,s,l)
#define toLOWER_uvchr(c,s,l)	to_uni_lower(c,s,l)
#define toTITLE_uvchr(c,s,l)	to_uni_title(c,s,l)
#define toUPPER_uvchr(c,s,l)	to_uni_upper(c,s,l)

/* For backwards compatibility, even though '_uni' should mean official Unicode
 * code points, in Perl it means native for those below 256 */
#define isALPHA_uni(c)          isALPHA_uvchr(c)
#define isALPHANUMERIC_uni(c)   isALPHANUMERIC_uvchr(c)
#define isASCII_uni(c)          isASCII_uvchr(c)
#define isBLANK_uni(c)          isBLANK_uvchr(c)
#define isCNTRL_uni(c)          isCNTRL_uvchr(c)
#define isDIGIT_uni(c)          isDIGIT_uvchr(c)
#define isGRAPH_uni(c)          isGRAPH_uvchr(c)
#define isIDCONT_uni(c)         isIDCONT_uvchr(c)
#define isIDFIRST_uni(c)        isIDFIRST_uvchr(c)
#define isLOWER_uni(c)          isLOWER_uvchr(c)
#define isPRINT_uni(c)          isPRINT_uvchr(c)
#define isPUNCT_uni(c)          isPUNCT_uvchr(c)
#define isSPACE_uni(c)          isSPACE_uvchr(c)
#define isPSXSPC_uni(c)         isPSXSPC_uvchr(c)
#define isUPPER_uni(c)          isUPPER_uvchr(c)
#define isVERTWS_uni(c)         isVERTWS_uvchr(c)
#define isWORDCHAR_uni(c)       isWORDCHAR_uvchr(c)
#define isXDIGIT_uni(c)         isXDIGIT_uvchr(c)
#define toFOLD_uni(c,s,l)       toFOLD_uvchr(c,s,l)
#define toLOWER_uni(c,s,l)      toLOWER_uvchr(c,s,l)
#define toTITLE_uni(c,s,l)      toTITLE_uvchr(c,s,l)
#define toUPPER_uni(c,s,l)      toUPPER_uvchr(c,s,l)

/* For internal core Perl use only: the base macros for defining macros like
 * isALPHA_LC_uvchr.  These are like isALPHA_LC, but the input can be any code
 * point, not just 0-255.  Like _generic_uvchr, there are two versions, one for
 * simple class definitions; the other for more complex.  These are like
 * _generic_uvchr, so see it for more info. */
#define _generic_LC_uvchr(latin1, above_latin1, c)                            \
                                    (c < 256 ? latin1(c) : above_latin1(c))
#define _generic_LC_invlist_uvchr(latin1, classnum, c)                          \
                            (c < 256 ? latin1(c) : _is_uni_FOO(classnum, c))

#define isALPHA_LC_uvchr(c)  _generic_LC_invlist_uvchr(isALPHA_LC, _CC_ALPHA, c)
#define isALPHANUMERIC_LC_uvchr(c)  _generic_LC_invlist_uvchr(isALPHANUMERIC_LC, \
                                                         _CC_ALPHANUMERIC, c)
#define isASCII_LC_uvchr(c)   isASCII_LC(c)
#define isBLANK_LC_uvchr(c)  _generic_LC_uvchr(isBLANK_LC,                    \
                                                        is_HORIZWS_cp_high, c)
#define isCNTRL_LC_uvchr(c)  (c < 256 ? isCNTRL_LC(c) : 0)
#define isDIGIT_LC_uvchr(c)  _generic_LC_invlist_uvchr(isDIGIT_LC, _CC_DIGIT, c)
#define isGRAPH_LC_uvchr(c)  _generic_LC_invlist_uvchr(isGRAPH_LC, _CC_GRAPH, c)
#define isIDCONT_LC_uvchr(c) _generic_LC_uvchr(isIDCONT_LC,                   \
                                                  _is_uni_perl_idcont, c)
#define isIDFIRST_LC_uvchr(c) _generic_LC_uvchr(isIDFIRST_LC,                 \
                                                  _is_uni_perl_idstart, c)
#define isLOWER_LC_uvchr(c)  _generic_LC_invlist_uvchr(isLOWER_LC, _CC_LOWER, c)
#define isPRINT_LC_uvchr(c)  _generic_LC_invlist_uvchr(isPRINT_LC, _CC_PRINT, c)
#define isPSXSPC_LC_uvchr(c)  isSPACE_LC_uvchr(c)
#define isPUNCT_LC_uvchr(c)  _generic_LC_invlist_uvchr(isPUNCT_LC, _CC_PUNCT, c)
#define isSPACE_LC_uvchr(c)  _generic_LC_uvchr(isSPACE_LC,                    \
                                                    is_XPERLSPACE_cp_high, c)
#define isUPPER_LC_uvchr(c)  _generic_LC_invlist_uvchr(isUPPER_LC, _CC_UPPER, c)
#define isWORDCHAR_LC_uvchr(c) _generic_LC_invlist_uvchr(isWORDCHAR_LC,         \
                                                           _CC_WORDCHAR, c)
#define isXDIGIT_LC_uvchr(c) _generic_LC_uvchr(isXDIGIT_LC,                  \
                                                       is_XDIGIT_cp_high, c)

#define isBLANK_LC_uni(c)    isBLANK_LC_uvchr(UNI_TO_NATIVE(c))

/* The "_safe" macros make sure that we don't attempt to read beyond 'e', but
 * they don't otherwise go out of their way to look for malformed UTF-8.  If
 * they can return accurate results without knowing if the input is otherwise
 * malformed, they do so.  For example isASCII is accurate in spite of any
 * non-length malformations because it looks only at a single byte. Likewise
 * isDIGIT looks just at the first byte for code points 0-255, as all UTF-8
 * variant ones return FALSE.  But, if the input has to be well-formed in order
 * for the results to be accurate, the macros will test and if malformed will
 * call a routine to die
 *
 * Except for toke.c, the macros do assume that e > p, asserting that on
 * DEBUGGING builds.  Much code that calls these depends on this being true,
 * for other reasons.  toke.c is treated specially as using the regular
 * assertion breaks it in many ways.  All strings that these operate on there
 * are supposed to have an extra NUL character at the end,  so that *e = \0. A
 * bunch of code in toke.c assumes that this is true, so the assertion allows
 * for that */
#ifdef PERL_IN_TOKE_C
#  define _utf8_safe_assert(p,e) ((e) > (p) || ((e) == (p) && *(p) == '\0'))
#else
#  define _utf8_safe_assert(p,e) ((e) > (p))
#endif

#define _generic_utf8_safe(classnum, p, e, above_latin1)                    \
    ((! _utf8_safe_assert(p, e))                                            \
      ? (_force_out_malformed_utf8_message((U8 *) (p), (U8 *) (e), 0, 1), 0)\
      : (UTF8_IS_INVARIANT(*(p)))                                           \
          ? _generic_isCC(*(p), classnum)                                   \
          : (UTF8_IS_DOWNGRADEABLE_START(*(p))                              \
             ? ((LIKELY((e) - (p) > 1 && UTF8_IS_CONTINUATION(*((p)+1))))   \
                ? _generic_isCC(EIGHT_BIT_UTF8_TO_NATIVE(*(p), *((p)+1 )),  \
                                classnum)                                   \
                : (_force_out_malformed_utf8_message(                       \
                                        (U8 *) (p), (U8 *) (e), 0, 1), 0))  \
             : above_latin1))
/* Like the above, but calls 'above_latin1(p)' to get the utf8 value.
 * 'above_latin1' can be a macro */
#define _generic_func_utf8_safe(classnum, above_latin1, p, e)               \
                    _generic_utf8_safe(classnum, p, e, above_latin1(p, e))
#define _generic_non_invlist_utf8_safe(classnum, above_latin1, p, e)          \
          _generic_utf8_safe(classnum, p, e,                                \
                             (UNLIKELY((e) - (p) < UTF8SKIP(p))             \
                              ? (_force_out_malformed_utf8_message(         \
                                      (U8 *) (p), (U8 *) (e), 0, 1), 0)     \
                              : above_latin1(p)))
/* Like the above, but passes classnum to _isFOO_utf8(), instead of having an
 * 'above_latin1' parameter */
#define _generic_invlist_utf8_safe(classnum, p, e)                            \
            _generic_utf8_safe(classnum, p, e, _is_utf8_FOO(classnum, p, e))

/* Like the above, but should be used only when it is known that there are no
 * characters in the upper-Latin1 range (128-255 on ASCII platforms) which the
 * class is TRUE for.  Hence it can skip the tests for this range.
 * 'above_latin1' should include its arguments */
#define _generic_utf8_safe_no_upper_latin1(classnum, p, e, above_latin1)    \
         (__ASSERT_(_utf8_safe_assert(p, e))                                \
         (UTF8_IS_INVARIANT(*(p)))                                          \
          ? _generic_isCC(*(p), classnum)                                   \
          : (UTF8_IS_DOWNGRADEABLE_START(*(p)))                             \
             ? 0 /* Note that doesn't check validity for latin1 */          \
             : above_latin1)


#define isALPHA_utf8(p, e)         isALPHA_utf8_safe(p, e)
#define isALPHANUMERIC_utf8(p, e)  isALPHANUMERIC_utf8_safe(p, e)
#define isASCII_utf8(p, e)         isASCII_utf8_safe(p, e)
#define isBLANK_utf8(p, e)         isBLANK_utf8_safe(p, e)
#define isCNTRL_utf8(p, e)         isCNTRL_utf8_safe(p, e)
#define isDIGIT_utf8(p, e)         isDIGIT_utf8_safe(p, e)
#define isGRAPH_utf8(p, e)         isGRAPH_utf8_safe(p, e)
#define isIDCONT_utf8(p, e)        isIDCONT_utf8_safe(p, e)
#define isIDFIRST_utf8(p, e)       isIDFIRST_utf8_safe(p, e)
#define isLOWER_utf8(p, e)         isLOWER_utf8_safe(p, e)
#define isPRINT_utf8(p, e)         isPRINT_utf8_safe(p, e)
#define isPSXSPC_utf8(p, e)        isPSXSPC_utf8_safe(p, e)
#define isPUNCT_utf8(p, e)         isPUNCT_utf8_safe(p, e)
#define isSPACE_utf8(p, e)         isSPACE_utf8_safe(p, e)
#define isUPPER_utf8(p, e)         isUPPER_utf8_safe(p, e)
#define isVERTWS_utf8(p, e)        isVERTWS_utf8_safe(p, e)
#define isWORDCHAR_utf8(p, e)      isWORDCHAR_utf8_safe(p, e)
#define isXDIGIT_utf8(p, e)        isXDIGIT_utf8_safe(p, e)

#define isALPHA_utf8_safe(p, e)  _generic_invlist_utf8_safe(_CC_ALPHA, p, e)
#define isALPHANUMERIC_utf8_safe(p, e)                                      \
                        _generic_invlist_utf8_safe(_CC_ALPHANUMERIC, p, e)
#define isASCII_utf8_safe(p, e)                                             \
    /* Because ASCII is invariant under utf8, the non-utf8 macro            \
    * works */                                                              \
    (__ASSERT_(_utf8_safe_assert(p, e)) isASCII(*(p)))
#define isBLANK_utf8_safe(p, e)                                             \
        _generic_non_invlist_utf8_safe(_CC_BLANK, is_HORIZWS_high, p, e)

#ifdef EBCDIC
    /* Because all controls are UTF-8 invariants in EBCDIC, we can use this
     * more efficient macro instead of the more general one */
#   define isCNTRL_utf8_safe(p, e)                                          \
                    (__ASSERT_(_utf8_safe_assert(p, e)) isCNTRL_L1(*(p)))
#else
#   define isCNTRL_utf8_safe(p, e)  _generic_utf8_safe(_CC_CNTRL, p, e, 0)
#endif

#define isDIGIT_utf8_safe(p, e)                                             \
            _generic_utf8_safe_no_upper_latin1(_CC_DIGIT, p, e,             \
                                            _is_utf8_FOO(_CC_DIGIT, p, e))
#define isGRAPH_utf8_safe(p, e)    _generic_invlist_utf8_safe(_CC_GRAPH, p, e)
#define isIDCONT_utf8_safe(p, e)   _generic_func_utf8_safe(_CC_WORDCHAR,    \
                                                 _is_utf8_perl_idcont, p, e)

/* To prevent S_scan_word in toke.c from hanging, we have to make sure that
 * IDFIRST is an alnum.  See
 * https://github.com/Perl/perl5/issues/10275 for more detail than you
 * ever wanted to know about.  (In the ASCII range, there isn't a difference.)
 * This used to be not the XID version, but we decided to go with the more
 * modern Unicode definition */
#define isIDFIRST_utf8_safe(p, e)                                           \
    _generic_func_utf8_safe(_CC_IDFIRST,                                    \
                            _is_utf8_perl_idstart, (U8 *) (p), (U8 *) (e))

#define isLOWER_utf8_safe(p, e)     _generic_invlist_utf8_safe(_CC_LOWER, p, e)
#define isPRINT_utf8_safe(p, e)     _generic_invlist_utf8_safe(_CC_PRINT, p, e)
#define isPSXSPC_utf8_safe(p, e)     isSPACE_utf8_safe(p, e)
#define isPUNCT_utf8_safe(p, e)     _generic_invlist_utf8_safe(_CC_PUNCT, p, e)
#define isSPACE_utf8_safe(p, e)                                             \
    _generic_non_invlist_utf8_safe(_CC_SPACE, is_XPERLSPACE_high, p, e)
#define isUPPER_utf8_safe(p, e)  _generic_invlist_utf8_safe(_CC_UPPER, p, e)
#define isVERTWS_utf8_safe(p, e)                                            \
        _generic_non_invlist_utf8_safe(_CC_VERTSPACE, is_VERTWS_high, p, e)
#define isWORDCHAR_utf8_safe(p, e)                                          \
                             _generic_invlist_utf8_safe(_CC_WORDCHAR, p, e)
#define isXDIGIT_utf8_safe(p, e)                                            \
                   _generic_utf8_safe_no_upper_latin1(_CC_XDIGIT, p, e,     \
                             (UNLIKELY((e) - (p) < UTF8SKIP(p))             \
                              ? (_force_out_malformed_utf8_message(         \
                                      (U8 *) (p), (U8 *) (e), 0, 1), 0)     \
                              : is_XDIGIT_high(p)))

#define toFOLD_utf8(p,e,s,l)	toFOLD_utf8_safe(p,e,s,l)
#define toLOWER_utf8(p,e,s,l)	toLOWER_utf8_safe(p,e,s,l)
#define toTITLE_utf8(p,e,s,l)	toTITLE_utf8_safe(p,e,s,l)
#define toUPPER_utf8(p,e,s,l)	toUPPER_utf8_safe(p,e,s,l)

/* For internal core use only, subject to change */
#define _toFOLD_utf8_flags(p,e,s,l,f)  _to_utf8_fold_flags (p,e,s,l,f)
#define _toLOWER_utf8_flags(p,e,s,l,f) _to_utf8_lower_flags(p,e,s,l,f)
#define _toTITLE_utf8_flags(p,e,s,l,f) _to_utf8_title_flags(p,e,s,l,f)
#define _toUPPER_utf8_flags(p,e,s,l,f) _to_utf8_upper_flags(p,e,s,l,f)

#define toFOLD_utf8_safe(p,e,s,l)   _toFOLD_utf8_flags(p,e,s,l, FOLD_FLAGS_FULL)
#define toLOWER_utf8_safe(p,e,s,l)  _toLOWER_utf8_flags(p,e,s,l, 0)
#define toTITLE_utf8_safe(p,e,s,l)  _toTITLE_utf8_flags(p,e,s,l, 0)
#define toUPPER_utf8_safe(p,e,s,l)  _toUPPER_utf8_flags(p,e,s,l, 0)

#define isALPHA_LC_utf8(p, e)         isALPHA_LC_utf8_safe(p, e)
#define isALPHANUMERIC_LC_utf8(p, e)  isALPHANUMERIC_LC_utf8_safe(p, e)
#define isASCII_LC_utf8(p, e)         isASCII_LC_utf8_safe(p, e)
#define isBLANK_LC_utf8(p, e)         isBLANK_LC_utf8_safe(p, e)
#define isCNTRL_LC_utf8(p, e)         isCNTRL_LC_utf8_safe(p, e)
#define isDIGIT_LC_utf8(p, e)         isDIGIT_LC_utf8_safe(p, e)
#define isGRAPH_LC_utf8(p, e)         isGRAPH_LC_utf8_safe(p, e)
#define isIDCONT_LC_utf8(p, e)        isIDCONT_LC_utf8_safe(p, e)
#define isIDFIRST_LC_utf8(p, e)       isIDFIRST_LC_utf8_safe(p, e)
#define isLOWER_LC_utf8(p, e)         isLOWER_LC_utf8_safe(p, e)
#define isPRINT_LC_utf8(p, e)         isPRINT_LC_utf8_safe(p, e)
#define isPSXSPC_LC_utf8(p, e)        isPSXSPC_LC_utf8_safe(p, e)
#define isPUNCT_LC_utf8(p, e)         isPUNCT_LC_utf8_safe(p, e)
#define isSPACE_LC_utf8(p, e)         isSPACE_LC_utf8_safe(p, e)
#define isUPPER_LC_utf8(p, e)         isUPPER_LC_utf8_safe(p, e)
#define isWORDCHAR_LC_utf8(p, e)      isWORDCHAR_LC_utf8_safe(p, e)
#define isXDIGIT_LC_utf8(p, e)        isXDIGIT_LC_utf8_safe(p, e)

/* For internal core Perl use only: the base macros for defining macros like
 * isALPHA_LC_utf8_safe.  These are like _generic_utf8, but if the first code
 * point in 'p' is within the 0-255 range, it uses locale rules from the
 * passed-in 'macro' parameter */
#define _generic_LC_utf8_safe(macro, p, e, above_latin1)                    \
         (__ASSERT_(_utf8_safe_assert(p, e))                                \
         (UTF8_IS_INVARIANT(*(p)))                                          \
          ? macro(*(p))                                                     \
          : (UTF8_IS_DOWNGRADEABLE_START(*(p))                              \
             ? ((LIKELY((e) - (p) > 1 && UTF8_IS_CONTINUATION(*((p)+1))))   \
                ? macro(EIGHT_BIT_UTF8_TO_NATIVE(*(p), *((p)+1)))           \
                : (_force_out_malformed_utf8_message(                       \
                                        (U8 *) (p), (U8 *) (e), 0, 1), 0))  \
              : above_latin1))

#define _generic_LC_invlist_utf8_safe(macro, classnum, p, e)                  \
            _generic_LC_utf8_safe(macro, p, e,                              \
                                            _is_utf8_FOO(classnum, p, e))

#define _generic_LC_func_utf8_safe(macro, above_latin1, p, e)               \
            _generic_LC_utf8_safe(macro, p, e, above_latin1(p, e))

#define _generic_LC_non_invlist_utf8_safe(classnum, above_latin1, p, e)       \
          _generic_LC_utf8_safe(classnum, p, e,                             \
                             (UNLIKELY((e) - (p) < UTF8SKIP(p))             \
                              ? (_force_out_malformed_utf8_message(         \
                                      (U8 *) (p), (U8 *) (e), 0, 1), 0)     \
                              : above_latin1(p)))

#define isALPHANUMERIC_LC_utf8_safe(p, e)                                   \
            _generic_LC_invlist_utf8_safe(isALPHANUMERIC_LC,                  \
                                        _CC_ALPHANUMERIC, p, e)
#define isALPHA_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isALPHA_LC, _CC_ALPHA, p, e)
#define isASCII_LC_utf8_safe(p, e)                                          \
                    (__ASSERT_(_utf8_safe_assert(p, e)) isASCII_LC(*(p)))
#define isBLANK_LC_utf8_safe(p, e)                                          \
        _generic_LC_non_invlist_utf8_safe(isBLANK_LC, is_HORIZWS_high, p, e)
#define isCNTRL_LC_utf8_safe(p, e)                                          \
            _generic_LC_utf8_safe(isCNTRL_LC, p, e, 0)
#define isDIGIT_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isDIGIT_LC, _CC_DIGIT, p, e)
#define isGRAPH_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isGRAPH_LC, _CC_GRAPH, p, e)
#define isIDCONT_LC_utf8_safe(p, e)                                         \
            _generic_LC_func_utf8_safe(isIDCONT_LC,                         \
                                                _is_utf8_perl_idcont, p, e)
#define isIDFIRST_LC_utf8_safe(p, e)                                        \
            _generic_LC_func_utf8_safe(isIDFIRST_LC,                        \
                                               _is_utf8_perl_idstart, p, e)
#define isLOWER_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isLOWER_LC, _CC_LOWER, p, e)
#define isPRINT_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isPRINT_LC, _CC_PRINT, p, e)
#define isPSXSPC_LC_utf8_safe(p, e)    isSPACE_LC_utf8_safe(p, e)
#define isPUNCT_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isPUNCT_LC, _CC_PUNCT, p, e)
#define isSPACE_LC_utf8_safe(p, e)                                          \
    _generic_LC_non_invlist_utf8_safe(isSPACE_LC, is_XPERLSPACE_high, p, e)
#define isUPPER_LC_utf8_safe(p, e)                                          \
            _generic_LC_invlist_utf8_safe(isUPPER_LC, _CC_UPPER, p, e)
#define isWORDCHAR_LC_utf8_safe(p, e)                                       \
            _generic_LC_invlist_utf8_safe(isWORDCHAR_LC, _CC_WORDCHAR, p, e)
#define isXDIGIT_LC_utf8_safe(p, e)                                         \
        _generic_LC_non_invlist_utf8_safe(isXDIGIT_LC, is_XDIGIT_high, p, e)

/* Macros for backwards compatibility and for completeness when the ASCII and
 * Latin1 values are identical */
#define isALPHAU(c)         isALPHA_L1(c)
#define isDIGIT_L1(c)       isDIGIT_A(c)
#define isOCTAL(c)          isOCTAL_A(c)
#define isOCTAL_L1(c)       isOCTAL_A(c)
#define isXDIGIT_L1(c)      isXDIGIT_A(c)
#define isALNUM(c)          isWORDCHAR(c)
#define isALNUM_A(c)        isALNUM(c)
#define isALNUMU(c)         isWORDCHAR_L1(c)
#define isALNUM_LC(c)       isWORDCHAR_LC(c)
#define isALNUM_uni(c)      isWORDCHAR_uni(c)
#define isALNUM_LC_uvchr(c) isWORDCHAR_LC_uvchr(c)
#define isALNUM_utf8(p,e)   isWORDCHAR_utf8(p,e)
#define isALNUM_utf8_safe(p,e) isWORDCHAR_utf8_safe(p,e)
#define isALNUM_LC_utf8(p,e)isWORDCHAR_LC_utf8(p,e)
#define isALNUM_LC_utf8_safe(p,e)isWORDCHAR_LC_utf8_safe(p,e)
#define isALNUMC_A(c)       isALPHANUMERIC_A(c)      /* Mnemonic: "C's alnum" */
#define isALNUMC_L1(c)      isALPHANUMERIC_L1(c)
#define isALNUMC(c)	    isALPHANUMERIC(c)
#define isALNUMC_LC(c)	    isALPHANUMERIC_LC(c)
#define isALNUMC_uni(c)     isALPHANUMERIC_uni(c)
#define isALNUMC_LC_uvchr(c) isALPHANUMERIC_LC_uvchr(c)
#define isALNUMC_utf8(p,e)  isALPHANUMERIC_utf8(p,e)
#define isALNUMC_utf8_safe(p,e)  isALPHANUMERIC_utf8_safe(p,e)
#define isALNUMC_LC_utf8_safe(p,e) isALPHANUMERIC_LC_utf8_safe(p,e)

/* On EBCDIC platforms, CTRL-@ is 0, CTRL-A is 1, etc, just like on ASCII,
 * except that they don't necessarily mean the same characters, e.g. CTRL-D is
 * 4 on both systems, but that is EOT on ASCII;  ST on EBCDIC.
 * '?' is special-cased on EBCDIC to APC, which is the control there that is
 * the outlier from the block that contains the other controls, just like
 * toCTRL('?') on ASCII yields DEL, the control that is the outlier from the C0
 * block.  If it weren't special cased, it would yield a non-control.
 * The conversion works both ways, so toCTRL('D') is 4, and toCTRL(4) is D,
 * etc. */
#ifndef EBCDIC
#  define toCTRL(c)    (__ASSERT_(FITS_IN_8_BITS(c)) toUPPER(((U8)(c))) ^ 64)
#else
#  define toCTRL(c)   (__ASSERT_(FITS_IN_8_BITS(c))                     \
                      ((isPRINT_A(c))                                   \
                       ? (UNLIKELY((c) == '?')                          \
                         ? QUESTION_MARK_CTRL                           \
                         : (NATIVE_TO_LATIN1(toUPPER((U8) (c))) ^ 64))  \
                       : (UNLIKELY((c) == QUESTION_MARK_CTRL)           \
                         ? '?'                                          \
                         : (LATIN1_TO_NATIVE(((U8) (c)) ^ 64)))))
#endif

/* Line numbers are unsigned, 32 bits. */
typedef U32 line_t;
#define NOLINE ((line_t) 4294967295UL)  /* = FFFFFFFF */

/* Helpful alias for version prescan */
#define is_LAX_VERSION(a,b) \
	(a != Perl_prescan_version(aTHX_ a, FALSE, b, NULL, NULL, NULL, NULL))

#define is_STRICT_VERSION(a,b) \
	(a != Perl_prescan_version(aTHX_ a, TRUE, b, NULL, NULL, NULL, NULL))

#define BADVERSION(a,b,c) \
	if (b) { \
	    *b = c; \
	} \
	return a;

/* Converts a character KNOWN to represent a hexadecimal digit (0-9, A-F, or
 * a-f) to its numeric value without using any branches.  The input is
 * validated only by an assert() in DEBUGGING builds.
 *
 * It works by right shifting and isolating the bit that is 0 for the digits,
 * and 1 for at least the alphas A-F, a-f.  The bit is shifted to the ones
 * position, and then to the eights position.  Both are added together to form
 * 0 if the input is '0'-'9' and to form 9 if alpha.  This is added to the
 * final four bits of the input to form the correct value. */
#define XDIGIT_VALUE(c) (__ASSERT_(isXDIGIT(c))                             \
           ((NATIVE_TO_LATIN1(c) >> 6) & 1)  /* 1 if alpha; 0 if not */     \
         + ((NATIVE_TO_LATIN1(c) >> 3) & 8)  /* 8 if alpha; 0 if not */     \
         + ((c) & 0xF))   /* 0-9 if input valid hex digit */

/* The argument is a string pointer, which is advanced. */
#define READ_XDIGIT(s)  ((s)++, XDIGIT_VALUE(*((s) - 1)))

/* Converts a character known to represent an octal digit (0-7) to its numeric
 * value.  The input is validated only by an assert() in DEBUGGING builds.  In
 * both ASCII and EBCDIC the last 3 bits of the octal digits range from 0-7. */
#define OCTAL_VALUE(c) (__ASSERT_(isOCTAL(c)) (7 & (c)))

/* Efficiently returns a boolean as to if two native characters are equivalent
 * case-insenstively.  At least one of the characters must be one of [A-Za-z];
 * the ALPHA in the name is to remind you of that.  This is asserted() in
 * DEBUGGING builds.  Because [A-Za-z] are invariant under UTF-8, this macro
 * works (on valid input) for both non- and UTF-8-encoded bytes.
 *
 * When one of the inputs is a compile-time constant and gets folded by the
 * compiler, this reduces to an AND and a TEST.  On both EBCDIC and ASCII
 * machines, 'A' and 'a' differ by a single bit; the same with the upper and
 * lower case of all other ASCII-range alphabetics.  On ASCII platforms, they
 * are 32 apart; on EBCDIC, they are 64.  At compile time, this uses an
 * exclusive 'or' to find that bit and then inverts it to form a mask, with
 * just a single 0, in the bit position where the upper- and lowercase differ.
 * */
#define isALPHA_FOLD_EQ(c1, c2)                                         \
                      (__ASSERT_(isALPHA_A(c1) || isALPHA_A(c2))        \
                      ((c1) & ~('A' ^ 'a')) ==  ((c2) & ~('A' ^ 'a')))
#define isALPHA_FOLD_NE(c1, c2) (! isALPHA_FOLD_EQ((c1), (c2)))

/*
=head1 Memory Management

=for apidoc Am|void|Newx|void* ptr|int nitems|type
The XSUB-writer's interface to the C C<malloc> function.

Memory obtained by this should B<ONLY> be freed with L</"Safefree">.

In 5.9.3, Newx() and friends replace the older New() API, and drops
the first parameter, I<x>, a debug aid which allowed callers to identify
themselves.  This aid has been superseded by a new build option,
PERL_MEM_LOG (see L<perlhacktips/PERL_MEM_LOG>).  The older API is still
there for use in XS modules supporting older perls.

=for apidoc Am|void|Newxc|void* ptr|int nitems|type|cast
The XSUB-writer's interface to the C C<malloc> function, with
cast.  See also C<L</Newx>>.

Memory obtained by this should B<ONLY> be freed with L</"Safefree">.

=for apidoc Am|void|Newxz|void* ptr|int nitems|type
The XSUB-writer's interface to the C C<malloc> function.  The allocated
memory is zeroed with C<memzero>.  See also C<L</Newx>>.

Memory obtained by this should B<ONLY> be freed with L</"Safefree">.

=for apidoc Am|void|Renew|void* ptr|int nitems|type
The XSUB-writer's interface to the C C<realloc> function.

Memory obtained by this should B<ONLY> be freed with L</"Safefree">.

=for apidoc Am|void|Renewc|void* ptr|int nitems|type|cast
The XSUB-writer's interface to the C C<realloc> function, with
cast.

Memory obtained by this should B<ONLY> be freed with L</"Safefree">.

=for apidoc Am|void|Safefree|void* ptr
The XSUB-writer's interface to the C C<free> function.

This should B<ONLY> be used on memory obtained using L</"Newx"> and friends.

=for apidoc Am|void|Move|void* src|void* dest|int nitems|type
The XSUB-writer's interface to the C C<memmove> function.  The C<src> is the
source, C<dest> is the destination, C<nitems> is the number of items, and
C<type> is the type.  Can do overlapping moves.  See also C<L</Copy>>.

=for apidoc Am|void *|MoveD|void* src|void* dest|int nitems|type
Like C<Move> but returns C<dest>.  Useful
for encouraging compilers to tail-call
optimise.

=for apidoc Am|void|Copy|void* src|void* dest|int nitems|type
The XSUB-writer's interface to the C C<memcpy> function.  The C<src> is the
source, C<dest> is the destination, C<nitems> is the number of items, and
C<type> is the type.  May fail on overlapping copies.  See also C<L</Move>>.

=for apidoc Am|void *|CopyD|void* src|void* dest|int nitems|type

Like C<Copy> but returns C<dest>.  Useful
for encouraging compilers to tail-call
optimise.

=for apidoc Am|void|Zero|void* dest|int nitems|type

The XSUB-writer's interface to the C C<memzero> function.  The C<dest> is the
destination, C<nitems> is the number of items, and C<type> is the type.

=for apidoc Am|void *|ZeroD|void* dest|int nitems|type

Like C<Zero> but returns dest.  Useful
for encouraging compilers to tail-call
optimise.

=for apidoc Am|void|StructCopy|type *src|type *dest|type
This is an architecture-independent macro to copy one structure to another.

=for apidoc Am|void|PoisonWith|void* dest|int nitems|type|U8 byte

Fill up memory with a byte pattern (a byte repeated over and over
again) that hopefully catches attempts to access uninitialized memory.

=for apidoc Am|void|PoisonNew|void* dest|int nitems|type

PoisonWith(0xAB) for catching access to allocated but uninitialized memory.

=for apidoc Am|void|PoisonFree|void* dest|int nitems|type

PoisonWith(0xEF) for catching access to freed memory.

=for apidoc Am|void|Poison|void* dest|int nitems|type

PoisonWith(0xEF) for catching access to freed memory.

=cut */

/* Maintained for backwards-compatibility only. Use newSV() instead. */
#ifndef PERL_CORE
#define NEWSV(x,len)	newSV(len)
#endif

#define MEM_SIZE_MAX ((MEM_SIZE)-1)

#define _PERL_STRLEN_ROUNDUP_UNCHECKED(n) (((n) - 1 + PERL_STRLEN_ROUNDUP_QUANTUM) & ~((MEM_SIZE)PERL_STRLEN_ROUNDUP_QUANTUM - 1))

#ifdef PERL_MALLOC_WRAP

/* This expression will be constant-folded at compile time.  It checks
 * whether or not the type of the count n is so small (e.g. U8 or U16, or
 * U32 on 64-bit systems) that there's no way a wrap-around could occur.
 * As well as avoiding the need for a run-time check in some cases, it's
 * designed to avoid compiler warnings like:
 *     comparison is always false due to limited range of data type
 * It's mathematically equivalent to
 *    max(n) * sizeof(t) > MEM_SIZE_MAX
 */

#  define _MEM_WRAP_NEEDS_RUNTIME_CHECK(n,t) \
    (  sizeof(MEM_SIZE) < sizeof(n) \
    || sizeof(t) > ((MEM_SIZE)1 << 8*(sizeof(MEM_SIZE) - sizeof(n))))

/* This is written in a slightly odd way to avoid various spurious
 * compiler warnings. We *want* to write the expression as
 *    _MEM_WRAP_NEEDS_RUNTIME_CHECK(n,t) && (n > C)
 * (for some compile-time constant C), but even when the LHS
 * constant-folds to false at compile-time, g++ insists on emitting
 * warnings about the RHS (e.g. "comparison is always false"), so instead
 * we write it as
 *
 *    (cond ? n : X) > C
 *
 * where X is a constant with X > C always false. Choosing a value for X
 * is tricky. If 0, some compilers will complain about 0 > C always being
 * false; if 1, Coverity complains when n happens to be the constant value
 * '1', that cond ? 1 : 1 has the same value on both branches; so use C
 * for X and hope that nothing else whines.
 */

#  define _MEM_WRAP_WILL_WRAP(n,t) \
      ((_MEM_WRAP_NEEDS_RUNTIME_CHECK(n,t) ? (MEM_SIZE)(n) : \
            MEM_SIZE_MAX/sizeof(t)) > MEM_SIZE_MAX/sizeof(t))

#  define MEM_WRAP_CHECK(n,t) \
	(void)(UNLIKELY(_MEM_WRAP_WILL_WRAP(n,t)) \
        && (croak_memory_wrap(),0))

#  define MEM_WRAP_CHECK_1(n,t,a) \
	(void)(UNLIKELY(_MEM_WRAP_WILL_WRAP(n,t)) \
	&& (Perl_croak_nocontext("%s",(a)),0))

/* "a" arg must be a string literal */
#  define MEM_WRAP_CHECK_s(n,t,a) \
	(void)(UNLIKELY(_MEM_WRAP_WILL_WRAP(n,t)) \
	&& (Perl_croak_nocontext("" a ""),0))

#define MEM_WRAP_CHECK_(n,t) MEM_WRAP_CHECK(n,t),

#define PERL_STRLEN_ROUNDUP(n) ((void)(((n) > MEM_SIZE_MAX - 2 * PERL_STRLEN_ROUNDUP_QUANTUM) ? (croak_memory_wrap(),0) : 0), _PERL_STRLEN_ROUNDUP_UNCHECKED(n))
#else

#define MEM_WRAP_CHECK(n,t)
#define MEM_WRAP_CHECK_1(n,t,a)
#define MEM_WRAP_CHECK_s(n,t,a)
#define MEM_WRAP_CHECK_(n,t)

#define PERL_STRLEN_ROUNDUP(n) _PERL_STRLEN_ROUNDUP_UNCHECKED(n)

#endif

#ifdef PERL_MEM_LOG
/*
 * If PERL_MEM_LOG is defined, all Newx()s, Renew()s, and Safefree()s
 * go through functions, which are handy for debugging breakpoints, but
 * which more importantly get the immediate calling environment (file and
 * line number, and C function name if available) passed in.  This info can
 * then be used for logging the calls, for which one gets a sample
 * implementation unless -DPERL_MEM_LOG_NOIMPL is also defined.
 *
 * Known problems:
 * - not all memory allocs get logged, only those
 *   that go through Newx() and derivatives (while all
 *   Safefrees do get logged)
 * - __FILE__ and __LINE__ do not work everywhere
 * - __func__ or __FUNCTION__ even less so
 * - I think more goes on after the perlio frees but
 *   the thing is that STDERR gets closed (as do all
 *   the file descriptors)
 * - no deeper calling stack than the caller of the Newx()
 *   or the kind, but do I look like a C reflection/introspection
 *   utility to you?
 * - the function prototypes for the logging functions
 *   probably should maybe be somewhere else than handy.h
 * - one could consider inlining (macrofying) the logging
 *   for speed, but I am too lazy
 * - one could imagine recording the allocations in a hash,
 *   (keyed by the allocation address?), and maintain that
 *   through reallocs and frees, but how to do that without
 *   any News() happening...?
 * - lots of -Ddefines to get useful/controllable output
 * - lots of ENV reads
 */

# ifdef PERL_CORE
#  ifndef PERL_MEM_LOG_NOIMPL
enum mem_log_type {
  MLT_ALLOC,
  MLT_REALLOC,
  MLT_FREE,
  MLT_NEW_SV,
  MLT_DEL_SV
};
#  endif
#  if defined(PERL_IN_SV_C)  /* those are only used in sv.c */
void Perl_mem_log_new_sv(const SV *sv, const char *filename, const int linenumber, const char *funcname);
void Perl_mem_log_del_sv(const SV *sv, const char *filename, const int linenumber, const char *funcname);
#  endif
# endif

#endif

#ifdef PERL_MEM_LOG
#define MEM_LOG_ALLOC(n,t,a)     Perl_mem_log_alloc(n,sizeof(t),STRINGIFY(t),a,__FILE__,__LINE__,FUNCTION__)
#define MEM_LOG_REALLOC(n,t,v,a) Perl_mem_log_realloc(n,sizeof(t),STRINGIFY(t),v,a,__FILE__,__LINE__,FUNCTION__)
#define MEM_LOG_FREE(a)          Perl_mem_log_free(a,__FILE__,__LINE__,FUNCTION__)
#endif

#ifndef MEM_LOG_ALLOC
#define MEM_LOG_ALLOC(n,t,a)     (a)
#endif
#ifndef MEM_LOG_REALLOC
#define MEM_LOG_REALLOC(n,t,v,a) (a)
#endif
#ifndef MEM_LOG_FREE
#define MEM_LOG_FREE(a)          (a)
#endif

#define Newx(v,n,t)	(v = (MEM_WRAP_CHECK_(n,t) (t*)MEM_LOG_ALLOC(n,t,safemalloc((MEM_SIZE)((n)*sizeof(t))))))
#define Newxc(v,n,t,c)	(v = (MEM_WRAP_CHECK_(n,t) (c*)MEM_LOG_ALLOC(n,t,safemalloc((MEM_SIZE)((n)*sizeof(t))))))
#define Newxz(v,n,t)	(v = (MEM_WRAP_CHECK_(n,t) (t*)MEM_LOG_ALLOC(n,t,safecalloc((n),sizeof(t)))))

#ifndef PERL_CORE
/* pre 5.9.x compatibility */
#define New(x,v,n,t)	Newx(v,n,t)
#define Newc(x,v,n,t,c)	Newxc(v,n,t,c)
#define Newz(x,v,n,t)	Newxz(v,n,t)
#endif

#define Renew(v,n,t) \
	  (v = (MEM_WRAP_CHECK_(n,t) (t*)MEM_LOG_REALLOC(n,t,v,saferealloc((Malloc_t)(v),(MEM_SIZE)((n)*sizeof(t))))))
#define Renewc(v,n,t,c) \
	  (v = (MEM_WRAP_CHECK_(n,t) (c*)MEM_LOG_REALLOC(n,t,v,saferealloc((Malloc_t)(v),(MEM_SIZE)((n)*sizeof(t))))))

#ifdef PERL_POISON
#define Safefree(d) \
  ((d) ? (void)(safefree(MEM_LOG_FREE((Malloc_t)(d))), Poison(&(d), 1, Malloc_t)) : (void) 0)
#else
#define Safefree(d)	safefree(MEM_LOG_FREE((Malloc_t)(d)))
#endif

/* assert that a valid ptr has been supplied - use this instead of assert(ptr)  *
 * as it handles cases like constant string arguments without throwing warnings *
 * the cast is required, as is the inequality check, to avoid warnings          */
#define perl_assert_ptr(p) assert( ((void*)(p)) != 0 )


#define Move(s,d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), perl_assert_ptr(s), (void)memmove((char*)(d),(const char*)(s), (n) * sizeof(t)))
#define Copy(s,d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), perl_assert_ptr(s), (void)memcpy((char*)(d),(const char*)(s), (n) * sizeof(t)))
#define Zero(d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), (void)memzero((char*)(d), (n) * sizeof(t)))

/* Like above, but returns a pointer to 'd' */
#define MoveD(s,d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), perl_assert_ptr(s), memmove((char*)(d),(const char*)(s), (n) * sizeof(t)))
#define CopyD(s,d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), perl_assert_ptr(s), memcpy((char*)(d),(const char*)(s), (n) * sizeof(t)))
#define ZeroD(d,n,t)	(MEM_WRAP_CHECK_(n,t) perl_assert_ptr(d), memzero((char*)(d), (n) * sizeof(t)))

#define PoisonWith(d,n,t,b)	(MEM_WRAP_CHECK_(n,t) (void)memset((char*)(d), (U8)(b), (n) * sizeof(t)))
#define PoisonNew(d,n,t)	PoisonWith(d,n,t,0xAB)
#define PoisonFree(d,n,t)	PoisonWith(d,n,t,0xEF)
#define Poison(d,n,t)		PoisonFree(d,n,t)

#ifdef PERL_POISON
#  define PERL_POISON_EXPR(x) x
#else
#  define PERL_POISON_EXPR(x)
#endif

#define StructCopy(s,d,t) (*((t*)(d)) = *((t*)(s)))

/*
=head1 Handy Values

=for apidoc Am|STRLEN|C_ARRAY_LENGTH|void *a

Returns the number of elements in the input C array (so you want your
zero-based indices to be less than but not equal to).

=for apidoc Am|void *|C_ARRAY_END|void *a

Returns a pointer to one element past the final element of the input C array.

=cut

C_ARRAY_END is one past the last: half-open/half-closed range, not
last-inclusive range.
*/
#define C_ARRAY_LENGTH(a)	(sizeof(a)/sizeof((a)[0]))
#define C_ARRAY_END(a)		((a) + C_ARRAY_LENGTH(a))

#ifdef NEED_VA_COPY
# ifdef va_copy
#  define Perl_va_copy(s, d) va_copy(d, s)
# elif defined(__va_copy)
#  define Perl_va_copy(s, d) __va_copy(d, s)
# else
#  define Perl_va_copy(s, d) Copy(s, d, 1, va_list)
# endif
#endif

/* convenience debug macros */
#ifdef USE_ITHREADS
#define pTHX_FORMAT  "Perl interpreter: 0x%p"
#define pTHX__FORMAT ", Perl interpreter: 0x%p"
#define pTHX_VALUE_   (void *)my_perl,
#define pTHX_VALUE    (void *)my_perl
#define pTHX__VALUE_ ,(void *)my_perl,
#define pTHX__VALUE  ,(void *)my_perl
#else
#define pTHX_FORMAT
#define pTHX__FORMAT
#define pTHX_VALUE_
#define pTHX_VALUE
#define pTHX__VALUE_
#define pTHX__VALUE
#endif /* USE_ITHREADS */

/* Perl_deprecate was not part of the public API, and did not have a deprecate()
   shortcut macro defined without -DPERL_CORE. Neither codesearch.google.com nor
   CPAN::Unpack show any users outside the core.  */
#ifdef PERL_CORE
#  define deprecate(s) Perl_ck_warner_d(aTHX_ packWARN(WARN_DEPRECATED),    \
                                            "Use of " s " is deprecated")
#  define deprecate_disappears_in(when,message) \
              Perl_ck_warner_d(aTHX_ packWARN(WARN_DEPRECATED),    \
                               message ", and will disappear in Perl " when)
#  define deprecate_fatal_in(when,message) \
              Perl_ck_warner_d(aTHX_ packWARN(WARN_DEPRECATED),    \
                               message ". Its use will be fatal in Perl " when)
#endif

/* Internal macros to deal with gids and uids */
#ifdef PERL_CORE

#  if Uid_t_size > IVSIZE
#    define sv_setuid(sv, uid)       sv_setnv((sv), (NV)(uid))
#    define SvUID(sv)                SvNV(sv)
#  elif Uid_t_sign <= 0
#    define sv_setuid(sv, uid)       sv_setiv((sv), (IV)(uid))
#    define SvUID(sv)                SvIV(sv)
#  else
#    define sv_setuid(sv, uid)       sv_setuv((sv), (UV)(uid))
#    define SvUID(sv)                SvUV(sv)
#  endif /* Uid_t_size */

#  if Gid_t_size > IVSIZE
#    define sv_setgid(sv, gid)       sv_setnv((sv), (NV)(gid))
#    define SvGID(sv)                SvNV(sv)
#  elif Gid_t_sign <= 0
#    define sv_setgid(sv, gid)       sv_setiv((sv), (IV)(gid))
#    define SvGID(sv)                SvIV(sv)
#  else
#    define sv_setgid(sv, gid)       sv_setuv((sv), (UV)(gid))
#    define SvGID(sv)                SvUV(sv)
#  endif /* Gid_t_size */

#endif

#endif  /* PERL_HANDY_H_ */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
