/*    op_reg_common.h
 *
 *    Definitions common to by op.h and regexp.h
 *
 *    Copyright (C) 2010, 2011 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/* These defines are used in both op.h and regexp.h  The definitions use the
 * shift form so that ext/B/Makefile.PL will pick them up.
 *
 * Data structures used in the two headers have common fields, and in fact one
 * is copied onto the other.  This makes it easy to keep them in sync */

/* This tells where the first of these bits is.  Setting it to 0 saved cycles
 * and memory.  I (khw) think the code will work if changed back, but haven't
 * tested it */
/* Make sure to update ext/re/re.pm when changing this! */
#ifndef RXf_PMf_STD_PMMOD_SHIFT /* Only expand #include of this file once */

#define RXf_PMf_STD_PMMOD_SHIFT 0

/* The bits need to be ordered so that the msixn are contiguous starting at bit
 * RXf_PMf_STD_PMMOD_SHIFT, followed by the p.  See STD_PAT_MODS and
 * INT_PAT_MODS in regexp.h for the reason contiguity is needed */
/* Make sure to update lib/re.pm when changing these! */
/* Make sure you keep the pure PMf_ versions below in sync */
#define RXf_PMf_MULTILINE      (1U << (RXf_PMf_STD_PMMOD_SHIFT+0))    /* /m */
#define RXf_PMf_SINGLELINE     (1U << (RXf_PMf_STD_PMMOD_SHIFT+1))    /* /s */
#define RXf_PMf_FOLD           (1U << (RXf_PMf_STD_PMMOD_SHIFT+2))    /* /i */
#define RXf_PMf_EXTENDED       (1U << (RXf_PMf_STD_PMMOD_SHIFT+3))    /* /x */
#define RXf_PMf_EXTENDED_MORE  (1U << (RXf_PMf_STD_PMMOD_SHIFT+4))    /* /xx */
#define RXf_PMf_NOCAPTURE      (1U << (RXf_PMf_STD_PMMOD_SHIFT+5))    /* /n */

#define RXf_PMf_KEEPCOPY       (1U << (RXf_PMf_STD_PMMOD_SHIFT+6))    /* /p */

/* The character set for the regex is stored in a field of more than one bit
 * using an enum, for reasons of compactness and to ensure that the options are
 * mutually exclusive */
/* Make sure to update ext/re/re.pm and regcomp.sym (as these are used as
 * offsets for various node types, like POSIXD vs POSIXL, etc) when changing
 * this! */
typedef enum {
    REGEX_DEPENDS_CHARSET = 0,
    REGEX_LOCALE_CHARSET,
    REGEX_UNICODE_CHARSET,
    REGEX_ASCII_RESTRICTED_CHARSET,
    REGEX_ASCII_MORE_RESTRICTED_CHARSET
} regex_charset;

#define _RXf_PMf_CHARSET_SHIFT ((RXf_PMf_STD_PMMOD_SHIFT)+7)
#define RXf_PMf_CHARSET (7U << (_RXf_PMf_CHARSET_SHIFT)) /* 3 bits */

/* Manually decorate these functions here with gcc-style attributes just to
 * avoid making the regex_charset typedef global, which it would need to be for
 * proto.h to understand it */
PERL_STATIC_INLINE void
set_regex_charset(U32 * const flags, const regex_charset cs)
    __attribute__nonnull__(1);

PERL_STATIC_INLINE void
set_regex_charset(U32 * const flags, const regex_charset cs)
{
    /* Sets the character set portion of 'flags' to 'cs', which is a member of
     * the above enum */

    *flags &= ~RXf_PMf_CHARSET;
    *flags |= (cs << _RXf_PMf_CHARSET_SHIFT);
}

PERL_STATIC_INLINE regex_charset
get_regex_charset(const U32 flags)
    __attribute__warn_unused_result__;

PERL_STATIC_INLINE regex_charset
get_regex_charset(const U32 flags)
{
    /* Returns the enum corresponding to the character set in 'flags' */

    return (regex_charset) ((flags & RXf_PMf_CHARSET) >> _RXf_PMf_CHARSET_SHIFT);
}

#define RXf_PMf_STRICT (1U<<(RXf_PMf_STD_PMMOD_SHIFT+10))

#define _RXf_PMf_SHIFT_COMPILETIME (RXf_PMf_STD_PMMOD_SHIFT+11)


/*
  Set in Perl_pmruntime if op_flags & OPf_SPECIAL, i.e. split. Will
  be used by regex engines to check whether they should set
  RXf_SKIPWHITE
*/
#define RXf_PMf_SPLIT (1U<<(RXf_PMf_STD_PMMOD_SHIFT+11))

/* Next available bit after the above.  Name begins with '_' so won't be
 * exported by B */
#define _RXf_PMf_SHIFT_NEXT (RXf_PMf_STD_PMMOD_SHIFT+12)

/* Mask of the above bits.  These need to be transferred from op_pmflags to
 * re->extflags during compilation */
#define RXf_PMf_COMPILETIME    (RXf_PMf_MULTILINE|RXf_PMf_SINGLELINE|RXf_PMf_FOLD|RXf_PMf_EXTENDED|RXf_PMf_EXTENDED_MORE|RXf_PMf_KEEPCOPY|RXf_PMf_NOCAPTURE|RXf_PMf_CHARSET|RXf_PMf_STRICT)
#define RXf_PMf_FLAGCOPYMASK   (RXf_PMf_COMPILETIME|RXf_PMf_SPLIT)

/* Temporary to get Jenkins happy again
 * See thread starting at http://nntp.perl.org/group/perl.perl5.porters/220710
 */
#if 0
    /* Exclude win32 because it can't cope with I32_MAX definition */
#ifndef WIN32
#   if RXf_PMf_COMPILETIME > I32_MAX
#     error RXf_PMf_COMPILETIME wont fit in arg2 field of eval node
#   endif
#endif
#endif

/* These copies need to be numerical or ext/B/Makefile.PL won't think they are
 * constants */
#define PMf_MULTILINE     (1U<<0)
#define PMf_SINGLELINE    (1U<<1)
#define PMf_FOLD          (1U<<2)
#define PMf_EXTENDED      (1U<<3)
#define PMf_EXTENDED_MORE (1U<<4)
#define PMf_NOCAPTURE     (1U<<5)
#define PMf_KEEPCOPY      (1U<<6)
#define PMf_CHARSET       (7U<<7)
#define PMf_STRICT        (1U<<10)
#define PMf_SPLIT         (1U<<11)

#if PMf_MULTILINE != RXf_PMf_MULTILINE || PMf_SINGLELINE != RXf_PMf_SINGLELINE || PMf_FOLD != RXf_PMf_FOLD || PMf_EXTENDED != RXf_PMf_EXTENDED || PMf_EXTENDED_MORE != RXf_PMf_EXTENDED_MORE || PMf_KEEPCOPY != RXf_PMf_KEEPCOPY || PMf_SPLIT != RXf_PMf_SPLIT || PMf_CHARSET != RXf_PMf_CHARSET || PMf_NOCAPTURE != RXf_PMf_NOCAPTURE || PMf_STRICT != RXf_PMf_STRICT
#   error RXf_PMf defines are wrong
#endif

/*  Error check that haven't left something out of this.  This isn't done
 *  directly in the #define because doing so confuses regcomp.pl.
 *  (2**n - 1) is n 1 bits, so the below gets the contiguous bits between the
 *  beginning and ending shifts */
#if RXf_PMf_COMPILETIME != (((1 << (_RXf_PMf_SHIFT_COMPILETIME))-1) \
                            & (~((1 << RXf_PMf_STD_PMMOD_SHIFT)-1)))
#   error RXf_PMf_COMPILETIME is invalid
#endif

#endif /* Include only once */
