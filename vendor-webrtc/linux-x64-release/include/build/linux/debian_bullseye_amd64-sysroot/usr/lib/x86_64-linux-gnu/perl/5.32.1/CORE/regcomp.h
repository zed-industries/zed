/*    regcomp.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999,
 *    2000, 2001, 2002, 2003, 2005, 2006, 2007, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#ifndef PERL_REGCOMP_H_
#define PERL_REGCOMP_H_

#include "regcharclass.h"

/* Convert branch sequences to more efficient trie ops? */
#define PERL_ENABLE_TRIE_OPTIMISATION 1

/* Be really aggressive about optimising patterns with trie sequences? */
#define PERL_ENABLE_EXTENDED_TRIE_OPTIMISATION 1

/* Should the optimiser take positive assertions into account? */
#define PERL_ENABLE_POSITIVE_ASSERTION_STUDY 0

/* Not for production use: */
#define PERL_ENABLE_EXPERIMENTAL_REGEX_OPTIMISATIONS 0

/* Activate offsets code - set to if 1 to enable */
#ifdef DEBUGGING
#define RE_TRACK_PATTERN_OFFSETS
#endif

/*
 * Structure for regexp "program".  This is essentially a linear encoding
 * of a nondeterministic finite-state machine (aka syntax charts or
 * "railroad normal form" in parsing technology).  Each node is an opcode
 * plus a "next" pointer, possibly plus an operand.  "Next" pointers of
 * all nodes except BRANCH implement concatenation; a "next" pointer with
 * a BRANCH on both ends of it is connecting two alternatives.  (Here we
 * have one of the subtle syntax dependencies:  an individual BRANCH (as
 * opposed to a collection of them) is never concatenated with anything
 * because of operator precedence.)  The operand of some types of node is
 * a literal string; for others, it is a node leading into a sub-FSM.  In
 * particular, the operand of a BRANCH node is the first node of the branch.
 * (NB this is *not* a tree structure:  the tail of the branch connects
 * to the thing following the set of BRANCHes.)  The opcodes are defined
 * in regnodes.h which is generated from regcomp.sym by regcomp.pl.
 */

/*
 * A node is one char of opcode followed by two chars of "next" pointer.
 * "Next" pointers are stored as two 8-bit pieces, high order first.  The
 * value is a positive offset from the opcode of the node containing it.
 * An operand, if any, simply follows the node.  (Note that much of the
 * code generation knows about this implicit relationship.)
 *
 * Using two bytes for the "next" pointer is vast overkill for most things,
 * but allows patterns to get big without disasters.
 *
 * [The "next" pointer is always aligned on an even
 * boundary, and reads the offset directly as a short.]
 */

/* This is the stuff that used to live in regexp.h that was truly
   private to the engine itself. It now lives here. */

 typedef struct regexp_internal {
        union {
	    U32 *offsets;           /* offset annotations 20001228 MJD
                                       data about mapping the program to the
                                       string -
                                       offsets[0] is proglen when this is used
                                       */
            U32 proglen;
        } u;

        regnode *regstclass;    /* Optional startclass as identified or constructed
                                   by the optimiser */
        struct reg_data *data;	/* Additional miscellaneous data used by the program.
                                   Used to make it easier to clone and free arbitrary
                                   data that the regops need. Often the ARG field of
                                   a regop is an index into this structure */
	struct reg_code_blocks *code_blocks;/* positions of literal (?{}) */
        int name_list_idx;	/* Optional data index of an array of paren names */
	regnode program[1];	/* Unwarranted chumminess with compiler. */
} regexp_internal;

#define RXi_SET(x,y) (x)->pprivate = (void*)(y)   
#define RXi_GET(x)   ((regexp_internal *)((x)->pprivate))
#define RXi_GET_DECL(r,ri) regexp_internal *ri = RXi_GET(r)
/*
 * Flags stored in regexp->intflags
 * These are used only internally to the regexp engine
 *
 * See regexp.h for flags used externally to the regexp engine
 */
#define RXp_INTFLAGS(rx)        ((rx)->intflags)
#define RX_INTFLAGS(prog)        RXp_INTFLAGS(ReANY(prog))

#define PREGf_SKIP		0x00000001
#define PREGf_IMPLICIT		0x00000002 /* Converted .* to ^.* */
#define PREGf_NAUGHTY		0x00000004 /* how exponential is this pattern? */
#define PREGf_VERBARG_SEEN	0x00000008
#define PREGf_CUTGROUP_SEEN	0x00000010
#define PREGf_USE_RE_EVAL	0x00000020 /* compiled with "use re 'eval'" */
/* these used to be extflags, but are now intflags */
#define PREGf_NOSCAN            0x00000040
                                /* spare */
#define PREGf_GPOS_SEEN         0x00000100
#define PREGf_GPOS_FLOAT        0x00000200

#define PREGf_ANCH_MBOL         0x00000400
#define PREGf_ANCH_SBOL         0x00000800
#define PREGf_ANCH_GPOS         0x00001000
#define PREGf_RECURSE_SEEN      0x00002000

#define PREGf_ANCH              \
    ( PREGf_ANCH_SBOL | PREGf_ANCH_GPOS | PREGf_ANCH_MBOL )

/* this is where the old regcomp.h started */

struct regnode_string {
    U8	str_len;
    U8  type;
    U16 next_off;
    char string[1];
};

struct regnode_lstring { /* Constructed this way to keep the string aligned. */
    U8	flags;
    U8  type;
    U16 next_off;
    U32 str_len;    /* Only 18 bits allowed before would overflow 'next_off' */
    char string[1];
};

struct regnode_anyofhs { /* Constructed this way to keep the string aligned. */
    U8	str_len;
    U8  type;
    U16 next_off;
    U32 arg1;                           /* set by set_ANYOF_arg() */
    char string[1];
};

/* Argument bearing node - workhorse, 
   arg1 is often for the data field */
struct regnode_1 {
    U8	flags;
    U8  type;
    U16 next_off;
    U32 arg1;
};

/* Node whose argument is 'SV *'.  This needs to be used very carefully in
 * situations where pointers won't become invalid because of, say re-mallocs */
struct regnode_p {
    U8	flags;
    U8  type;
    U16 next_off;
    SV * arg1;
};

/* Similar to a regnode_1 but with an extra signed argument */
struct regnode_2L {
    U8	flags;
    U8  type;
    U16 next_off;
    U32 arg1;
    I32 arg2;
};

/* 'Two field' -- Two 16 bit unsigned args */
struct regnode_2 {
    U8	flags;
    U8  type;
    U16 next_off;
    U16 arg1;
    U16 arg2;
};

#define ANYOF_BITMAP_SIZE	(NUM_ANYOF_CODE_POINTS / 8)   /* 8 bits/Byte */

/* Note that these form structs which are supersets of the next smaller one, by
 * appending fields.  Alignment problems can occur if one of those optional
 * fields requires stricter alignment than the base struct.  And formal
 * parameters that can really be two or more of the structs should be
 * declared as the smallest one it could be.  See commit message for
 * 7dcac5f6a5195002b55c935ee1d67f67e1df280b.  Regnode allocation is done
 * without regard to alignment, and changing it to would also require changing
 * the code that inserts and deletes regnodes.  The basic single-argument
 * regnode has a U32, which is what reganode() allocates as a unit.  Therefore
 * no field can require stricter alignment than U32. */

/* also used by trie */
struct regnode_charclass {
    U8	flags;
    U8  type;
    U16 next_off;
    U32 arg1;                           /* set by set_ANYOF_arg() */
    char bitmap[ANYOF_BITMAP_SIZE];	/* only compile-time */
};

/* has runtime (locale) \d, \w, ..., [:posix:] classes */
struct regnode_charclass_posixl {
    U8	flags;                      /* ANYOF_MATCHES_POSIXL bit must go here */
    U8  type;
    U16 next_off;
    U32 arg1;
    char bitmap[ANYOF_BITMAP_SIZE];		/* both compile-time ... */
    U32 classflags;	                        /* and run-time */
};

/* A synthetic start class (SSC); is a regnode_charclass_posixl_fold, plus an
 * extra SV*, used only during its construction and which is not used by
 * regexec.c.  Note that the 'next_off' field is unused, as the SSC stands
 * alone, so there is never a next node.  Also, there is no alignment issue,
 * because these are declared or allocated as a complete unit so the compiler
 * takes care of alignment.  This is unlike the other regnodes which are
 * allocated in terms of multiples of a single-argument regnode.  SSC nodes can
 * have a pointer field because there is no alignment issue, and because it is
 * set to NULL after construction, before any cloning of the pattern */
struct regnode_ssc {
    U8	flags;                      /* ANYOF_MATCHES_POSIXL bit must go here */
    U8  type;
    U16 next_off;
    U32 arg1;
    char bitmap[ANYOF_BITMAP_SIZE];	/* both compile-time ... */
    U32 classflags;	                /* ... and run-time */

    /* Auxiliary, only used during construction; NULL afterwards: list of code
     * points matched */
    SV* invlist;
};

/*  We take advantage of 'next_off' not otherwise being used in the SSC by
 *  actually using it: by setting it to 1.  This allows us to test and
 *  distinguish between an SSC and other ANYOF node types, as 'next_off' cannot
 *  otherwise be 1, because it is the offset to the next regnode expressed in
 *  units of regnodes.  Since an ANYOF node contains extra fields, it adds up
 *  to 12 regnode units on 32-bit systems, (hence the minimum this can be (if
 *  not 0) is 11 there.  Even if things get tightly packed on a 64-bit system,
 *  it still would be more than 1. */
#define set_ANYOF_SYNTHETIC(n) STMT_START{ OP(n) = ANYOF;              \
                                           NEXT_OFF(n) = 1;            \
                               } STMT_END
#define is_ANYOF_SYNTHETIC(n) (PL_regkind[OP(n)] == ANYOF && NEXT_OFF(n) == 1)

/* XXX fix this description.
   Impose a limit of REG_INFTY on various pattern matching operations
   to limit stack growth and to avoid "infinite" recursions.
*/
/* The default size for REG_INFTY is U16_MAX, which is the same as
   USHORT_MAX (see perl.h).  Unfortunately U16 isn't necessarily 16 bits
   (see handy.h).  On the Cray C90, sizeof(short)==4 and hence U16_MAX is
   ((1<<32)-1), while on the Cray T90, sizeof(short)==8 and U16_MAX is
   ((1<<64)-1).  To limit stack growth to reasonable sizes, supply a
   smaller default.
	--Andy Dougherty  11 June 1998
*/
#if SHORTSIZE > 2
#  ifndef REG_INFTY
#    define REG_INFTY ((1<<16)-1)
#  endif
#endif

#ifndef REG_INFTY
#  define REG_INFTY U16_MAX
#endif

#define ARG_VALUE(arg) (arg)
#define ARG__SET(arg,val) ((arg) = (val))

#undef ARG
#undef ARG1
#undef ARG2

#define ARG(p) ARG_VALUE(ARG_LOC(p))
#define ARGp(p) ARG_VALUE(ARGp_LOC(p))
#define ARG1(p) ARG_VALUE(ARG1_LOC(p))
#define ARG2(p) ARG_VALUE(ARG2_LOC(p))
#define ARG2L(p) ARG_VALUE(ARG2L_LOC(p))

#define ARG_SET(p, val) ARG__SET(ARG_LOC(p), (val))
#define ARGp_SET(p, val) ARG__SET(ARGp_LOC(p), (val))
#define ARG1_SET(p, val) ARG__SET(ARG1_LOC(p), (val))
#define ARG2_SET(p, val) ARG__SET(ARG2_LOC(p), (val))
#define ARG2L_SET(p, val) ARG__SET(ARG2L_LOC(p), (val))

#undef NEXT_OFF
#undef NODE_ALIGN

#define NEXT_OFF(p) ((p)->next_off)
#define NODE_ALIGN(node)
/* the following define was set to 0xde in 075abff3
 * as part of some linting logic. I have set it to 0
 * as otherwise in every place where we /might/ set flags
 * we have to set it 0 explicitly, which duplicates
 * assignments and IMO adds an unacceptable level of
 * surprise to working in the regex engine. If this
 * is changed from 0 then at the very least make sure
 * that SBOL for /^/ sets the flags to 0 explicitly.
 * -- Yves */
#define NODE_ALIGN_FILL(node) ((node)->flags = 0)

#define SIZE_ALIGN NODE_ALIGN

#undef OP
#undef OPERAND
#undef STRING

#define	OP(p)		((p)->type)
#define FLAGS(p)	((p)->flags)	/* Caution: Doesn't apply to all      \
					   regnode types.  For some, it's the \
					   character set of the regnode */
#define	STR_LENs(p)	(__ASSERT_(OP(p) != LEXACT && OP(p) != LEXACT_REQ8)  \
                                    ((struct regnode_string *)p)->str_len)
#define	STRINGs(p)	(__ASSERT_(OP(p) != LEXACT && OP(p) != LEXACT_REQ8)  \
                                    ((struct regnode_string *)p)->string)
#define	OPERANDs(p)	STRINGs(p)

/* Long strings.  Currently limited to length 18 bits, which handles a 262000
 * byte string.  The limiting factor is the 16 bit 'next_off' field, which
 * points to the next regnode, so the furthest away it can be is 2**16.  On
 * most architectures, regnodes are 2**2 bytes long, so that yields 2**18
 * bytes.  Should a longer string be desired, we could increase it to 26 bits
 * fairly easily, by changing this node to have longj type which causes the ARG
 * field to be used for the link to the next regnode (although code would have
 * to be changed to account for this), and then use a combination of the flags
 * and next_off fields for the length.  To get 34 bit length, also change the
 * node to be an ARG2L, using the second 32 bit field for the length, and not
 * using the flags nor next_off fields at all.  One could have an llstring node
 * and even an lllstring type. */
#define	STR_LENl(p)	(__ASSERT_(OP(p) == LEXACT || OP(p) == LEXACT_REQ8)  \
                                    (((struct regnode_lstring *)p)->str_len))
#define	STRINGl(p)	(__ASSERT_(OP(p) == LEXACT || OP(p) == LEXACT_REQ8)  \
                                    (((struct regnode_lstring *)p)->string))
#define	OPERANDl(p)	STRINGl(p)

#define	STR_LEN(p)	((OP(p) == LEXACT || OP(p) == LEXACT_REQ8)           \
                                               ? STR_LENl(p) : STR_LENs(p))
#define	STRING(p)	((OP(p) == LEXACT || OP(p) == LEXACT_REQ8)           \
                                               ? STRINGl(p)  : STRINGs(p))
#define	OPERAND(p)	STRING(p)

/* The number of (smallest) regnode equivalents that a string of length l bytes
 * occupies */
#define STR_SZ(l)	(((l) + sizeof(regnode) - 1) / sizeof(regnode))

/* The number of (smallest) regnode equivalents that the EXACTISH node 'p'
 * occupies */
#define NODE_SZ_STR(p)	(STR_SZ(STR_LEN(p)) + 1 + regarglen[(p)->type])

#define setSTR_LEN(p,v)                                                     \
    STMT_START{                                                             \
        if (OP(p) == LEXACT || OP(p) == LEXACT_REQ8)                        \
            ((struct regnode_lstring *)(p))->str_len = (v);                 \
        else                                                                \
            ((struct regnode_string *)(p))->str_len = (v);                  \
    } STMT_END

#define ANYOFR_BASE_BITS    20
#define ANYOFRbase(p)   (ARG(p) & ((1 << ANYOFR_BASE_BITS) - 1))
#define ANYOFRdelta(p)  (ARG(p) >> ANYOFR_BASE_BITS)

#undef NODE_ALIGN
#undef ARG_LOC
#undef NEXTOPER
#undef PREVOPER

#define	NODE_ALIGN(node)
#define	ARG_LOC(p)	(((struct regnode_1 *)p)->arg1)
#define ARGp_LOC(p)	(((struct regnode_p *)p)->arg1)
#define	ARG1_LOC(p)	(((struct regnode_2 *)p)->arg1)
#define	ARG2_LOC(p)	(((struct regnode_2 *)p)->arg2)
#define ARG2L_LOC(p)	(((struct regnode_2L *)p)->arg2)

#define NODE_STEP_REGNODE	1	/* sizeof(regnode)/sizeof(regnode) */
#define EXTRA_STEP_2ARGS	EXTRA_SIZE(struct regnode_2)

#define	NEXTOPER(p)	((p) + NODE_STEP_REGNODE)
#define	PREVOPER(p)	((p) - NODE_STEP_REGNODE)

#define FILL_NODE(offset, op)                                           \
    STMT_START {                                                        \
                    OP(REGNODE_p(offset)) = op;                         \
                    NEXT_OFF(REGNODE_p(offset)) = 0;                    \
    } STMT_END
#define FILL_ADVANCE_NODE(offset, op)                                   \
    STMT_START {                                                        \
                    FILL_NODE(offset, op);                              \
                    (offset)++;                                         \
    } STMT_END
#define FILL_ADVANCE_NODE_ARG(offset, op, arg)                          \
    STMT_START {                                                        \
                    ARG_SET(REGNODE_p(offset), arg);                    \
                    FILL_ADVANCE_NODE(offset, op);                      \
                    /* This is used generically for other operations    \
                     * that have a longer argument */                   \
                    (offset) += regarglen[op];                          \
    } STMT_END
#define FILL_ADVANCE_NODE_ARGp(offset, op, arg)                          \
    STMT_START {                                                        \
                    ARGp_SET(REGNODE_p(offset), arg);                    \
                    FILL_ADVANCE_NODE(offset, op);                      \
                    (offset) += regarglen[op];                          \
    } STMT_END
#define FILL_ADVANCE_NODE_2L_ARG(offset, op, arg1, arg2)                \
    STMT_START {                                                        \
                    ARG_SET(REGNODE_p(offset), arg1);                   \
                    ARG2L_SET(REGNODE_p(offset), arg2);                 \
                    FILL_ADVANCE_NODE(offset, op);                      \
                    (offset) += 2;                                      \
    } STMT_END

#define REG_MAGIC 0234

/* An ANYOF node is basically a bitmap with the index being a code point.  If
 * the bit for that code point is 1, the code point matches;  if 0, it doesn't
 * match (complemented if inverted).  There is an additional mechanism to deal
 * with cases where the bitmap is insufficient in and of itself.  This #define
 * indicates if the bitmap does fully represent what this ANYOF node can match.
 * The ARG is set to this special value (since 0, 1, ... are legal, but will
 * never reach this high). */
#define ANYOF_ONLY_HAS_BITMAP	((U32) -1)

/* When the bitmap isn't completely sufficient for handling the ANYOF node,
 * flags (in node->flags of the ANYOF node) get set to indicate this.  These
 * are perennially in short supply.  Beyond several cases where warnings need
 * to be raised under certain circumstances, currently, there are six cases
 * where the bitmap alone isn't sufficient.  We could use six flags to
 * represent the 6 cases, but to save flags bits, we play some games.  The
 * cases are:
 *
 *  1)  The bitmap has a compiled-in very finite size.  So something else needs
 *      to be used to specify if a code point that is too large for the bitmap
 *      actually matches.  The mechanism currently is an inversion
 *      list.  ANYOF_ONLY_HAS_BITMAP, described above, being TRUE indicates
 *      there are no matches of too-large code points.  But if it is FALSE,
 *      then almost certainly there are matches too large for the bitmap.  (The
 *      other cases, described below, either imply this one or are extremely
 *      rare in practice.)  So we can just assume that a too-large code point
 *      will need something beyond the bitmap if ANYOF_ONLY_HAS_BITMAP is
 *      FALSE, instead of having a separate flag for this.
 *  2)  A subset of item 1) is if all possible code points outside the bitmap
 *      match.  This is a common occurrence when the class is complemented,
 *      like /[^ij]/.  Therefore a bit is reserved to indicate this,
 *      rather than having an inversion list created,
 *      ANYOF_MATCHES_ALL_ABOVE_BITMAP.
 *  3)  Under /d rules, it can happen that code points that are in the upper
 *      latin1 range (\x80-\xFF or their equivalents on EBCDIC platforms) match
 *      only if the runtime target string being matched against is UTF-8.  For
 *      example /[\w[:punct:]]/d.  This happens only for posix classes (with a
 *      couple of exceptions, like \d where it doesn't happen), and all such
 *      ones also have above-bitmap matches.  Thus, 3) implies 1) as well.
 *      Note that /d rules are no longer encouraged; 'use 5.14' or higher
 *      deselects them.  But a flag is required so that they can be properly
 *      handled.  But it can be a shared flag: see 5) below.
 *  4)  Also under /d rules, something like /[\Wfoo]/ will match everything in
 *      the \x80-\xFF range, unless the string being matched against is UTF-8.
 *      An inversion list could be created for this case, but this is
 *      relatively common, and it turns out that it's all or nothing:  if any
 *      one of these code points matches, they all do.  Hence a single bit
 *      suffices.  We use a shared flag that doesn't take up space by itself:
 *      ANYOF_SHARED_d_MATCHES_ALL_NON_UTF8_NON_ASCII_non_d_WARN_SUPER.  This
 *      also implies 1), with one exception: [:^cntrl:].
 *  5)  A user-defined \p{} property may not have been defined by the time the
 *      regex is compiled.  In this case, we don't know until runtime what it
 *      will match, so we have to assume it could match anything, including
 *      code points that ordinarily would be in the bitmap.  A flag bit is
 *      necessary to indicate this , though it can be shared with the item 3)
 *      flag, as that only occurs under /d, and this only occurs under non-d.
 *      This case is quite uncommon in the field, and the /(?[ ...])/ construct
 *      is a better way to accomplish what this feature does.  This case also
 *      implies 1).
 *      ANYOF_SHARED_d_UPPER_LATIN1_UTF8_STRING_MATCHES_non_d_RUNTIME_USER_PROP
 *      is the shared flag.
 *  6)  /[foo]/il may have folds that are only valid if the runtime locale is a
 *      UTF-8 one.  These are quite rare, so it would be good to avoid the
 *      expense of looking for them.  But /l matching is slow anyway, and we've
 *      traditionally not worried too much about its performance.  And this
 *      condition requires the ANYOFL_FOLD flag to be set, so testing for
 *      that flag would be sufficient to rule out most cases of this.  So it is
 *      unclear if this should have a flag or not.  But, this flag can be
 *      shared with another, so it doesn't occupy extra space.
 *
 * At the moment, there is one spare bit, but this could be increased by
 * various tricks:
 *
 * If just one more bit is needed, as of this writing it seems to khw that the
 * best choice would be to make ANYOF_MATCHES_ALL_ABOVE_BITMAP not a flag, but
 * something like
 *
 *      #define ANYOF_MATCHES_ALL_ABOVE_BITMAP      ((U32) -2)
 *
 * and access it through the ARG like ANYOF_ONLY_HAS_BITMAP is.  This flag is
 * used by all ANYOF node types, and it could be used to avoid calling the
 * handler function, as the macro REGINCLASS in regexec.c does now for other
 * cases.
 *
 * Another possibility is based on the fact that ANYOF_MATCHES_POSIXL is
 * redundant with the node type ANYOFPOSIXL.  That flag could be removed, but
 * at the expense of extra code in regexec.c.  The flag has been retained
 * because it allows us to see if we need to call reginsert, or just use the
 * bitmap in one test.
 *
 * If this is done, an extension would be to make all ANYOFL nodes contain the
 * extra 32 bits that ANYOFPOSIXL ones do.  The posix flags only occupy 30
 * bits, so the ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD flags
 * and ANYOFL_FOLD could be moved to that extra space, but it would mean extra
 * instructions, as there are currently places in the code that assume those
 * two bits are zero.
 *
 * All told, 5 bits could be available for other uses if all of the above were
 * done.
 *
 * Some flags are not used in synthetic start class (SSC) nodes, so could be
 * shared should new flags be needed for SSCs, like SSC_MATCHES_EMPTY_STRING
 * now. */

/* If this is set, the result of the match should be complemented.  regexec.c
 * is expecting this to be in the low bit.  Never in an SSC */
#define ANYOF_INVERT		                0x01

/* For the SSC node only, which cannot be inverted, so is shared with that bit.
 * This is used only during regex compilation. */
#define SSC_MATCHES_EMPTY_STRING                ANYOF_INVERT

/* Set if this is a regnode_charclass_posixl vs a regnode_charclass.  This
 * is used for runtime \d, \w, [:posix:], ..., which are used only in locale
 * and the optimizer's synthetic start class.  Non-locale \d, etc are resolved
 * at compile-time.  Only set under /l; can be in SSC */
#define ANYOF_MATCHES_POSIXL                    0x02

/* The fold is calculated and stored in the bitmap where possible at compile
 * time.  However under locale, the actual folding varies depending on
 * what the locale is at the time of execution, so it has to be deferred until
 * then.  Only set under /l; never in an SSC  */
#define ANYOFL_FOLD                             0x04

/* Shared bit set only with ANYOFL and SSC nodes:
 *    If ANYOFL_FOLD is set, this flag indicates there are potential matches
 *      valid only if the locale is a UTF-8 one.
 *    If ANYOFL_FOLD is NOT set, this flag means to warn if the runtime locale
 *       isn't a UTF-8 one (and the generated node assumes a UTF-8 locale).
 *       None of INVERT, POSIXL,
 *       ANYOF_SHARED_d_UPPER_LATIN1_UTF8_STRING_MATCHES_non_d_RUNTIME_USER_PROP
 *       can be set.  */
#define ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD        0x08

/* Convenience macros for teasing apart the meanings when reading the above bit
 * */
#define ANYOFL_SOME_FOLDS_ONLY_IN_UTF8_LOCALE(flags)                        \
    ((flags & ( ANYOFL_FOLD /* Both bits are set */                         \
               |ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD))   \
             == ( ANYOFL_FOLD                                               \
                 |ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD))

#define  ANYOFL_UTF8_LOCALE_REQD(flags)                                     \
    ((flags & ( ANYOFL_FOLD /* Only REQD bit is set */                      \
               |ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD))   \
             == ANYOFL_SHARED_UTF8_LOCALE_fold_HAS_MATCHES_nonfold_REQD)

/* Spare: Be sure to change ANYOF_FLAGS_ALL if this gets used  0x10 */

/* If set, the node matches every code point NUM_ANYOF_CODE_POINTS and above.
 * Can be in an SSC */
#define ANYOF_MATCHES_ALL_ABOVE_BITMAP          0x20

/* Shared bit:
 *      Under /d it means the ANYOFD node matches more things if the target
 *          string is encoded in UTF-8; any such things will be non-ASCII,
 *          characters that are < 256, and can be accessed via the inversion
 *          list.
 *      When not under /d, it means the ANYOF node contains a user-defined
 *      property that wasn't yet defined at the time the regex was compiled,
 *      and so must be looked up at runtime, by creating an inversion list.
 * (These uses are mutually exclusive because a user-defined property is
 * specified by \p{}, and \p{} implies /u which deselects /d).  The long macro
 * name is to make sure that you are cautioned about its shared nature.  Only
 * the non-/d meaning can be in an SSC */
#define ANYOF_SHARED_d_UPPER_LATIN1_UTF8_STRING_MATCHES_non_d_RUNTIME_USER_PROP  0x40

/* Shared bit:
 *      Under /d it means the ANYOFD node matches all non-ASCII Latin1
 *          characters when the target string is not in utf8.
 *      When not under /d, it means the ANYOF node should raise a warning if
 *          matching against an above-Unicode code point.
 * (These uses are mutually exclusive because the warning requires a \p{}, and
 * \p{} implies /u which deselects /d).  An SSC node only has this bit set if
 * what is meant is the warning.  The long macro name is to make sure that you
 * are cautioned about its shared nature */
#define ANYOF_SHARED_d_MATCHES_ALL_NON_UTF8_NON_ASCII_non_d_WARN_SUPER 0x80

#define ANYOF_FLAGS_ALL		(0xff & ~0x10)

#define ANYOF_LOCALE_FLAGS (ANYOFL_FOLD | ANYOF_MATCHES_POSIXL)

/* These are the flags that apply to both regular ANYOF nodes and synthetic
 * start class nodes during construction of the SSC.  During finalization of
 * the SSC, other of the flags may get added to it */
#define ANYOF_COMMON_FLAGS      0

/* Character classes for node->classflags of ANYOF */
/* Should be synchronized with a table in regprop() */
/* 2n should be the normal one, paired with its complement at 2n+1 */

#define ANYOF_ALPHA    ((_CC_ALPHA) * 2)
#define ANYOF_NALPHA   ((ANYOF_ALPHA) + 1)
#define ANYOF_ALPHANUMERIC   ((_CC_ALPHANUMERIC) * 2)    /* [[:alnum:]] isalnum(3), utf8::IsAlnum */
#define ANYOF_NALPHANUMERIC  ((ANYOF_ALPHANUMERIC) + 1)
#define ANYOF_ASCII    ((_CC_ASCII) * 2)
#define ANYOF_NASCII   ((ANYOF_ASCII) + 1)
#define ANYOF_BLANK    ((_CC_BLANK) * 2)     /* GNU extension: space and tab: non-vertical space */
#define ANYOF_NBLANK   ((ANYOF_BLANK) + 1)
#define ANYOF_CASED    ((_CC_CASED) * 2)    /* Pseudo class for [:lower:] or
                                               [:upper:] under /i */
#define ANYOF_NCASED   ((ANYOF_CASED) + 1)
#define ANYOF_CNTRL    ((_CC_CNTRL) * 2)
#define ANYOF_NCNTRL   ((ANYOF_CNTRL) + 1)
#define ANYOF_DIGIT    ((_CC_DIGIT) * 2)     /* \d */
#define ANYOF_NDIGIT   ((ANYOF_DIGIT) + 1)
#define ANYOF_GRAPH    ((_CC_GRAPH) * 2)
#define ANYOF_NGRAPH   ((ANYOF_GRAPH) + 1)
#define ANYOF_LOWER    ((_CC_LOWER) * 2)
#define ANYOF_NLOWER   ((ANYOF_LOWER) + 1)
#define ANYOF_PRINT    ((_CC_PRINT) * 2)
#define ANYOF_NPRINT   ((ANYOF_PRINT) + 1)
#define ANYOF_PUNCT    ((_CC_PUNCT) * 2)
#define ANYOF_NPUNCT   ((ANYOF_PUNCT) + 1)
#define ANYOF_SPACE    ((_CC_SPACE) * 2)     /* \s */
#define ANYOF_NSPACE   ((ANYOF_SPACE) + 1)
#define ANYOF_UPPER    ((_CC_UPPER) * 2)
#define ANYOF_NUPPER   ((ANYOF_UPPER) + 1)
#define ANYOF_WORDCHAR ((_CC_WORDCHAR) * 2)  /* \w, PL_utf8_alnum, utf8::IsWord, ALNUM */
#define ANYOF_NWORDCHAR   ((ANYOF_WORDCHAR) + 1)
#define ANYOF_XDIGIT   ((_CC_XDIGIT) * 2)
#define ANYOF_NXDIGIT  ((ANYOF_XDIGIT) + 1)

/* pseudo classes below this, not stored in the class bitmap, but used as flags
   during compilation of char classes */

#define ANYOF_VERTWS    ((_CC_VERTSPACE) * 2)
#define ANYOF_NVERTWS   ((ANYOF_VERTWS)+1)

/* It is best if this is the last one, as all above it are stored as bits in a
 * bitmap, and it isn't part of that bitmap */
#if _CC_VERTSPACE != _HIGHEST_REGCOMP_DOT_H_SYNC
#   error Problem with handy.h _HIGHEST_REGCOMP_DOT_H_SYNC #define
#endif

#define ANYOF_POSIXL_MAX (ANYOF_VERTWS) /* So upper loop limit is written:
                                         *       '< ANYOF_MAX'
                                         * Hence doesn't include VERTWS, as that
                                         * is a pseudo class */
#define ANYOF_MAX      ANYOF_POSIXL_MAX

#if (ANYOF_POSIXL_MAX > 32)   /* Must fit in 32-bit word */
#   error Problem with handy.h _CC_foo #defines
#endif

#define ANYOF_HORIZWS	((ANYOF_POSIXL_MAX)+2) /* = (ANYOF_NVERTWS + 1) */
#define ANYOF_NHORIZWS	((ANYOF_POSIXL_MAX)+3)

#define ANYOF_UNIPROP   ((ANYOF_POSIXL_MAX)+4)  /* Used to indicate a Unicode
                                                   property: \p{} or \P{} */

/* Backward source code compatibility. */

#define ANYOF_ALNUML	 ANYOF_ALNUM
#define ANYOF_NALNUML	 ANYOF_NALNUM
#define ANYOF_SPACEL	 ANYOF_SPACE
#define ANYOF_NSPACEL	 ANYOF_NSPACE
#define ANYOF_ALNUM ANYOF_WORDCHAR
#define ANYOF_NALNUM ANYOF_NWORDCHAR

/* Utility macros for the bitmap and classes of ANYOF */

#define ANYOF_FLAGS(p)		((p)->flags)

#define ANYOF_BIT(c)		(1U << ((c) & 7))

#define POSIXL_SET(field, c)	((field) |= (1U << (c)))
#define ANYOF_POSIXL_SET(p, c)	POSIXL_SET(((regnode_charclass_posixl*) (p))->classflags, (c))

#define POSIXL_CLEAR(field, c) ((field) &= ~ (1U <<(c)))
#define ANYOF_POSIXL_CLEAR(p, c) POSIXL_CLEAR(((regnode_charclass_posixl*) (p))->classflags, (c))

#define POSIXL_TEST(field, c)	((field) & (1U << (c)))
#define ANYOF_POSIXL_TEST(p, c)	POSIXL_TEST(((regnode_charclass_posixl*) (p))->classflags, (c))

#define POSIXL_ZERO(field)	STMT_START { (field) = 0; } STMT_END
#define ANYOF_POSIXL_ZERO(ret)	POSIXL_ZERO(((regnode_charclass_posixl*) (ret))->classflags)

#define ANYOF_POSIXL_SET_TO_BITMAP(p, bits)                                 \
     STMT_START {                                                           \
                    ((regnode_charclass_posixl*) (p))->classflags = (bits); \
     } STMT_END

/* Shifts a bit to get, eg. 0x4000_0000, then subtracts 1 to get 0x3FFF_FFFF */
#define ANYOF_POSIXL_SETALL(ret) STMT_START { ((regnode_charclass_posixl*) (ret))->classflags = ((1U << ((ANYOF_POSIXL_MAX) - 1))) - 1; } STMT_END
#define ANYOF_CLASS_SETALL(ret) ANYOF_POSIXL_SETALL(ret)

#define ANYOF_POSIXL_TEST_ANY_SET(p)                               \
        ((ANYOF_FLAGS(p) & ANYOF_MATCHES_POSIXL)                           \
	 && (((regnode_charclass_posixl*)(p))->classflags))
#define ANYOF_CLASS_TEST_ANY_SET(p) ANYOF_POSIXL_TEST_ANY_SET(p)

/* Since an SSC always has this field, we don't have to test for that; nor do
 * we want to because the bit isn't set for SSC during its construction */
#define ANYOF_POSIXL_SSC_TEST_ANY_SET(p)                               \
                            cBOOL(((regnode_ssc*)(p))->classflags)
#define ANYOF_POSIXL_SSC_TEST_ALL_SET(p) /* Are all bits set? */       \
        (((regnode_ssc*) (p))->classflags                              \
                        == ((1U << ((ANYOF_POSIXL_MAX) - 1))) - 1)

#define ANYOF_POSIXL_TEST_ALL_SET(p)                                   \
        ((ANYOF_FLAGS(p) & ANYOF_MATCHES_POSIXL)                               \
         && ((regnode_charclass_posixl*) (p))->classflags              \
                        == ((1U << ((ANYOF_POSIXL_MAX) - 1))) - 1)

#define ANYOF_POSIXL_OR(source, dest) STMT_START { (dest)->classflags |= (source)->classflags ; } STMT_END
#define ANYOF_CLASS_OR(source, dest) ANYOF_POSIXL_OR((source), (dest))

#define ANYOF_POSIXL_AND(source, dest) STMT_START { (dest)->classflags &= (source)->classflags ; } STMT_END

#define ANYOF_BITMAP_ZERO(ret)	Zero(((regnode_charclass*)(ret))->bitmap, ANYOF_BITMAP_SIZE, char)
#define ANYOF_BITMAP(p)		((regnode_charclass*)(p))->bitmap
#define ANYOF_BITMAP_BYTE(p, c)	BITMAP_BYTE(ANYOF_BITMAP(p), c)
#define ANYOF_BITMAP_SET(p, c)	(ANYOF_BITMAP_BYTE(p, c) |=  ANYOF_BIT(c))
#define ANYOF_BITMAP_CLEAR(p,c)	(ANYOF_BITMAP_BYTE(p, c) &= ~ANYOF_BIT(c))
#define ANYOF_BITMAP_TEST(p, c)	cBOOL(ANYOF_BITMAP_BYTE(p, c) &   ANYOF_BIT(c))

#define ANYOF_BITMAP_SETALL(p)		\
	memset (ANYOF_BITMAP(p), 255, ANYOF_BITMAP_SIZE)
#define ANYOF_BITMAP_CLEARALL(p)	\
	Zero (ANYOF_BITMAP(p), ANYOF_BITMAP_SIZE)

/*
 * Utility definitions.
 */
#ifndef CHARMASK
#  define UCHARAT(p)	((int)*(const U8*)(p))
#else
#  define UCHARAT(p)	((int)*(p)&CHARMASK)
#endif

/* Number of regnode equivalents that 'guy' occupies beyond the size of the
 * smallest regnode. */
#define EXTRA_SIZE(guy) ((sizeof(guy)-1)/sizeof(struct regnode))

#define REG_ZERO_LEN_SEEN                   0x00000001
#define REG_LOOKBEHIND_SEEN                 0x00000002
#define REG_GPOS_SEEN                       0x00000004
/* spare */
#define REG_RECURSE_SEEN                    0x00000020
#define REG_TOP_LEVEL_BRANCHES_SEEN         0x00000040
#define REG_VERBARG_SEEN                    0x00000080
#define REG_CUTGROUP_SEEN                   0x00000100
#define REG_RUN_ON_COMMENT_SEEN             0x00000200
#define REG_UNFOLDED_MULTI_SEEN             0x00000400
/* spare */
#define REG_UNBOUNDED_QUANTIFIER_SEEN       0x00001000


START_EXTERN_C

#ifdef PLUGGABLE_RE_EXTENSION
#include "re_nodes.h"
#else
#include "regnodes.h"
#endif

#ifndef PLUGGABLE_RE_EXTENSION
#ifndef DOINIT
EXTCONST regexp_engine PL_core_reg_engine;
#else /* DOINIT */
EXTCONST regexp_engine PL_core_reg_engine = { 
        Perl_re_compile,
        Perl_regexec_flags,
        Perl_re_intuit_start,
        Perl_re_intuit_string, 
        Perl_regfree_internal,
        Perl_reg_numbered_buff_fetch,
        Perl_reg_numbered_buff_store,
        Perl_reg_numbered_buff_length,
        Perl_reg_named_buff,
        Perl_reg_named_buff_iter,
        Perl_reg_qr_package,
#if defined(USE_ITHREADS)        
        Perl_regdupe_internal,
#endif        
        Perl_re_op_compile
};
#endif /* DOINIT */
#endif /* PLUGGABLE_RE_EXTENSION */


END_EXTERN_C


/* .what is a character array with one character for each member of .data
 * The character describes the function of the corresponding .data item:
 *   a - AV for paren_name_list under DEBUGGING
 *   f - start-class data for regstclass optimization
 *   l - start op for literal (?{EVAL}) item
 *   L - start op for literal (?{EVAL}) item, with separate CV (qr//)
 *   r - pointer to an embedded code-containing qr, e.g. /ab$qr/
 *   s - inversion list for Unicode-style character class, and the
 *       multicharacter strings resulting from casefolding the single-character
 *       entries in the character class
 *   t - trie struct
 *   u - trie struct's widecharmap (a HV, so can't share, must dup)
 *       also used for revcharmap and words under DEBUGGING
 *   T - aho-trie struct
 *   S - sv for named capture lookup
 * 20010712 mjd@plover.com
 * (Remember to update re_dup() and pregfree() if you add any items.)
 */
struct reg_data {
    U32 count;
    U8 *what;
    void* data[1];
};

/* Code in S_to_utf8_substr() and S_to_byte_substr() in regexec.c accesses
   anchored* and float* via array indexes 0 and 1.  */
#define anchored_substr substrs->data[0].substr
#define anchored_utf8 substrs->data[0].utf8_substr
#define anchored_offset substrs->data[0].min_offset
#define anchored_end_shift substrs->data[0].end_shift

#define float_substr substrs->data[1].substr
#define float_utf8 substrs->data[1].utf8_substr
#define float_min_offset substrs->data[1].min_offset
#define float_max_offset substrs->data[1].max_offset
#define float_end_shift substrs->data[1].end_shift

#define check_substr substrs->data[2].substr
#define check_utf8 substrs->data[2].utf8_substr
#define check_offset_min substrs->data[2].min_offset
#define check_offset_max substrs->data[2].max_offset
#define check_end_shift substrs->data[2].end_shift

#define RX_ANCHORED_SUBSTR(rx)	(ReANY(rx)->anchored_substr)
#define RX_ANCHORED_UTF8(rx)	(ReANY(rx)->anchored_utf8)
#define RX_FLOAT_SUBSTR(rx)	(ReANY(rx)->float_substr)
#define RX_FLOAT_UTF8(rx)	(ReANY(rx)->float_utf8)

/* trie related stuff */

/* a transition record for the state machine. the
   check field determines which state "owns" the
   transition. the char the transition is for is
   determined by offset from the owning states base
   field.  the next field determines which state
   is to be transitioned to if any.
*/
struct _reg_trie_trans {
  U32 next;
  U32 check;
};

/* a transition list element for the list based representation */
struct _reg_trie_trans_list_elem {
    U16 forid;
    U32 newstate;
};
typedef struct _reg_trie_trans_list_elem reg_trie_trans_le;

/* a state for compressed nodes. base is an offset
  into an array of reg_trie_trans array. If wordnum is
  nonzero the state is accepting. if base is zero then
  the state has no children (and will be accepting)
*/
struct _reg_trie_state {
  U16 wordnum;
  union {
    U32                base;
    reg_trie_trans_le* list;
  } trans;
};

/* info per word; indexed by wordnum */
typedef struct {
    U16  prev;	/* previous word in acceptance chain; eg in
		 * zzz|abc|ab/ after matching the chars abc, the
		 * accepted word is #2, and the previous accepted
		 * word is #3 */
    U32 len;	/* how many chars long is this word? */
    U32 accept;	/* accept state for this word */
} reg_trie_wordinfo;


typedef struct _reg_trie_state    reg_trie_state;
typedef struct _reg_trie_trans    reg_trie_trans;


/* anything in here that needs to be freed later
   should be dealt with in pregfree.
   refcount is first in both this and _reg_ac_data to allow a space
   optimisation in Perl_regdupe.  */
struct _reg_trie_data {
    U32             refcount;        /* number of times this trie is referenced */
    U32             lasttrans;       /* last valid transition element */
    U16             *charmap;        /* byte to charid lookup array */
    reg_trie_state  *states;         /* state data */
    reg_trie_trans  *trans;          /* array of transition elements */
    char            *bitmap;         /* stclass bitmap */
    U16 	    *jump;           /* optional 1 indexed array of offsets before tail 
                                        for the node following a given word. */
    reg_trie_wordinfo *wordinfo;     /* array of info per word */
    U16             uniquecharcount; /* unique chars in trie (width of trans table) */
    U32             startstate;      /* initial state - used for common prefix optimisation */
    STRLEN          minlen;          /* minimum length of words in trie - build/opt only? */
    STRLEN          maxlen;          /* maximum length of words in trie - build/opt only? */
    U32             prefixlen;       /* #chars in common prefix */
    U32             statecount;      /* Build only - number of states in the states array 
                                        (including the unused zero state) */
    U32             wordcount;       /* Build only */
#ifdef DEBUGGING
    STRLEN          charcount;       /* Build only */
#endif
};
/* There is one (3 under DEBUGGING) pointers that logically belong in this
   structure, but are held outside as they need duplication on thread cloning,
   whereas the rest of the structure can be read only:
    HV              *widecharmap;    code points > 255 to charid
#ifdef DEBUGGING
    AV              *words;          Array of words contained in trie, for dumping
    AV              *revcharmap;     Map of each charid back to its character representation
#endif
*/

#define TRIE_WORDS_OFFSET 2

typedef struct _reg_trie_data reg_trie_data;

/* refcount is first in both this and _reg_trie_data to allow a space
   optimisation in Perl_regdupe.  */
struct _reg_ac_data {
    U32              refcount;
    U32              trie;
    U32              *fail;
    reg_trie_state   *states;
};
typedef struct _reg_ac_data reg_ac_data;

/* ANY_BIT doesn't use the structure, so we can borrow it here.
   This is simpler than refactoring all of it as wed end up with
   three different sets... */

#define TRIE_BITMAP(p)		(((reg_trie_data *)(p))->bitmap)
#define TRIE_BITMAP_BYTE(p, c)	BITMAP_BYTE(TRIE_BITMAP(p), c)
#define TRIE_BITMAP_SET(p, c)	(TRIE_BITMAP_BYTE(p, c) |=  ANYOF_BIT((U8)c))
#define TRIE_BITMAP_CLEAR(p,c)	(TRIE_BITMAP_BYTE(p, c) &= ~ANYOF_BIT((U8)c))
#define TRIE_BITMAP_TEST(p, c)	(TRIE_BITMAP_BYTE(p, c) &   ANYOF_BIT((U8)c))

#define IS_ANYOF_TRIE(op) ((op)==TRIEC || (op)==AHOCORASICKC)
#define IS_TRIE_AC(op) ((op)>=AHOCORASICK)


#define BITMAP_BYTE(p, c)	(( (U8*) p) [ ( ( (UV) (c)) >> 3) ] )
#define BITMAP_TEST(p, c)	(BITMAP_BYTE(p, c) &   ANYOF_BIT((U8)c))

/* these defines assume uniquecharcount is the correct variable, and state may be evaluated twice */
#define TRIE_NODENUM(state) (((state)-1)/(trie->uniquecharcount)+1)
#define SAFE_TRIE_NODENUM(state) ((state) ? (((state)-1)/(trie->uniquecharcount)+1) : (state))
#define TRIE_NODEIDX(state) ((state) ? (((state)-1)*(trie->uniquecharcount)+1) : (state))

#ifdef DEBUGGING
#define TRIE_CHARCOUNT(trie) ((trie)->charcount)
#else
#define TRIE_CHARCOUNT(trie) (trie_charcount)
#endif

#define RE_TRIE_MAXBUF_INIT 65536
#define RE_TRIE_MAXBUF_NAME "\022E_TRIE_MAXBUF"
#define RE_DEBUG_FLAGS "\022E_DEBUG_FLAGS"

#define RE_COMPILE_RECURSION_INIT 1000
#define RE_COMPILE_RECURSION_LIMIT "\022E_COMPILE_RECURSION_LIMIT"

/*

RE_DEBUG_FLAGS is used to control what debug output is emitted
its divided into three groups of options, some of which interact.
The three groups are: Compile, Execute, Extra. There is room for a
further group, as currently only the low three bytes are used.

    Compile Options:
    
    PARSE
    PEEP
    TRIE
    PROGRAM
    OFFSETS

    Execute Options:

    INTUIT
    MATCH
    TRIE

    Extra Options

    TRIE
    OFFSETS

If you modify any of these make sure you make corresponding changes to
re.pm, especially to the documentation.

*/


/* Compile */
#define RE_DEBUG_COMPILE_MASK      0x0000FF
#define RE_DEBUG_COMPILE_PARSE     0x000001
#define RE_DEBUG_COMPILE_OPTIMISE  0x000002
#define RE_DEBUG_COMPILE_TRIE      0x000004
#define RE_DEBUG_COMPILE_DUMP      0x000008
#define RE_DEBUG_COMPILE_FLAGS     0x000010
#define RE_DEBUG_COMPILE_TEST      0x000020

/* Execute */
#define RE_DEBUG_EXECUTE_MASK      0x00FF00
#define RE_DEBUG_EXECUTE_INTUIT    0x000100
#define RE_DEBUG_EXECUTE_MATCH     0x000200
#define RE_DEBUG_EXECUTE_TRIE      0x000400

/* Extra */
#define RE_DEBUG_EXTRA_MASK              0x3FF0000
#define RE_DEBUG_EXTRA_TRIE              0x0010000
#define RE_DEBUG_EXTRA_OFFSETS           0x0020000
#define RE_DEBUG_EXTRA_OFFDEBUG          0x0040000
#define RE_DEBUG_EXTRA_STATE             0x0080000
#define RE_DEBUG_EXTRA_OPTIMISE          0x0100000
#define RE_DEBUG_EXTRA_BUFFERS           0x0400000
#define RE_DEBUG_EXTRA_GPOS              0x0800000
#define RE_DEBUG_EXTRA_DUMP_PRE_OPTIMIZE 0x1000000
#define RE_DEBUG_EXTRA_WILDCARD          0x2000000
/* combined */
#define RE_DEBUG_EXTRA_STACK             0x0280000

#define RE_DEBUG_FLAG(x) (re_debug_flags & (x))
/* Compile */
#define DEBUG_COMPILE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_MASK)) x  )
#define DEBUG_PARSE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_PARSE)) x  )
#define DEBUG_OPTIMISE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_OPTIMISE)) x  )
#define DEBUG_DUMP_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_DUMP)) x  )
#define DEBUG_TRIE_COMPILE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_TRIE)) x )
#define DEBUG_FLAGS_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_FLAGS)) x )
#define DEBUG_TEST_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_TEST)) x )
/* Execute */
#define DEBUG_EXECUTE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXECUTE_MASK)) x  )
#define DEBUG_INTUIT_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXECUTE_INTUIT)) x  )
#define DEBUG_MATCH_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXECUTE_MATCH)) x  )
#define DEBUG_TRIE_EXECUTE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXECUTE_TRIE)) x )

/* Extra */
#define DEBUG_EXTRA_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_MASK)) x  )
#define DEBUG_OFFSETS_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_OFFSETS)) x  )
#define DEBUG_STATE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_STATE)) x )
#define DEBUG_STACK_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_STACK)) x )
#define DEBUG_BUFFERS_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_BUFFERS)) x )

#define DEBUG_OPTIMISE_MORE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || ((RE_DEBUG_EXTRA_OPTIMISE|RE_DEBUG_COMPILE_OPTIMISE) == \
         RE_DEBUG_FLAG(RE_DEBUG_EXTRA_OPTIMISE|RE_DEBUG_COMPILE_OPTIMISE))) x )
#define MJD_OFFSET_DEBUG(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_OFFDEBUG)) \
        Perl_warn_nocontext x )
#define DEBUG_TRIE_COMPILE_MORE_r(x) DEBUG_TRIE_COMPILE_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_TRIE)) x )
#define DEBUG_TRIE_EXECUTE_MORE_r(x) DEBUG_TRIE_EXECUTE_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_TRIE)) x )

#define DEBUG_TRIE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_COMPILE_TRIE \
        | RE_DEBUG_EXECUTE_TRIE )) x )
#define DEBUG_GPOS_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_GPOS)) x )

#define DEBUG_DUMP_PRE_OPTIMIZE_r(x) DEBUG_r( \
    if (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_DUMP_PRE_OPTIMIZE)) x )

/* initialization */
/* Get the debug flags for code not in regcomp.c nor regexec.c.  This doesn't
 * initialize the variable if it isn't already there, instead it just assumes
 * the flags are 0 */
#define DECLARE_AND_GET_RE_DEBUG_FLAGS_NON_REGEX                               \
    volatile IV re_debug_flags = 0;  PERL_UNUSED_VAR(re_debug_flags);          \
    STMT_START {                                                               \
        SV * re_debug_flags_sv = NULL;                                         \
                     /* get_sv() can return NULL during global destruction. */ \
        re_debug_flags_sv = PL_curcop ? get_sv(RE_DEBUG_FLAGS, GV_ADD) : NULL; \
        if (re_debug_flags_sv && SvIOK(re_debug_flags_sv))                     \
            re_debug_flags=SvIV(re_debug_flags_sv);                            \
    } STMT_END


#ifdef DEBUGGING

/* For use in regcomp.c and regexec.c,  Get the debug flags, and initialize to
 * the defaults if not done already */
#define DECLARE_AND_GET_RE_DEBUG_FLAGS                                         \
    volatile IV re_debug_flags = 0;  PERL_UNUSED_VAR(re_debug_flags);          \
    STMT_START {                                                               \
        SV * re_debug_flags_sv = NULL;                                         \
                     /* get_sv() can return NULL during global destruction. */ \
        re_debug_flags_sv = PL_curcop ? get_sv(RE_DEBUG_FLAGS, GV_ADD) : NULL; \
        if (re_debug_flags_sv) {                                               \
            if (!SvIOK(re_debug_flags_sv)) /* If doesnt exist set to default */\
                sv_setuv(re_debug_flags_sv,                                    \
                        /* These defaults should be kept in sync with re.pm */ \
                            RE_DEBUG_COMPILE_DUMP | RE_DEBUG_EXECUTE_MASK );   \
            re_debug_flags=SvIV(re_debug_flags_sv);                            \
        }                                                                      \
    } STMT_END

#define isDEBUG_WILDCARD (DEBUG_v_TEST || RE_DEBUG_FLAG(RE_DEBUG_EXTRA_WILDCARD))

#define RE_PV_COLOR_DECL(rpv,rlen,isuni,dsv,pv,l,m,c1,c2)   \
    const char * const rpv =                                \
        pv_pretty((dsv), (pv), (l), (m),                    \
            PL_colors[(c1)],PL_colors[(c2)],                \
            PERL_PV_ESCAPE_RE|PERL_PV_ESCAPE_NONASCII |((isuni) ? PERL_PV_ESCAPE_UNI : 0) );         \
    const int rlen = SvCUR(dsv)

/* This is currently unsed in the core */
#define RE_SV_ESCAPE(rpv,isuni,dsv,sv,m)                            \
    const char * const rpv =                                        \
        pv_pretty((dsv), (SvPV_nolen_const(sv)), (SvCUR(sv)), (m),  \
            PL_colors[(c1)],PL_colors[(c2)],                        \
            PERL_PV_ESCAPE_RE|PERL_PV_ESCAPE_NONASCII |((isuni) ? PERL_PV_ESCAPE_UNI : 0) )

#define RE_PV_QUOTED_DECL(rpv,isuni,dsv,pv,l,m)                     \
    const char * const rpv =                                        \
        pv_pretty((dsv), (pv), (l), (m),                            \
            PL_colors[0], PL_colors[1],                             \
            ( PERL_PV_PRETTY_QUOTE | PERL_PV_ESCAPE_RE | PERL_PV_ESCAPE_NONASCII | PERL_PV_PRETTY_ELLIPSES | \
              ((isuni) ? PERL_PV_ESCAPE_UNI : 0))                  \
        )

#define RE_SV_DUMPLEN(ItEm) (SvCUR(ItEm) - (SvTAIL(ItEm)!=0))
#define RE_SV_TAIL(ItEm) (SvTAIL(ItEm) ? "$" : "")
    
#else /* if not DEBUGGING */

#define DECLARE_AND_GET_RE_DEBUG_FLAGS  dNOOP
#define RE_PV_COLOR_DECL(rpv,rlen,isuni,dsv,pv,l,m,c1,c2)  dNOOP
#define RE_SV_ESCAPE(rpv,isuni,dsv,sv,m)
#define RE_PV_QUOTED_DECL(rpv,isuni,dsv,pv,l,m)  dNOOP
#define RE_SV_DUMPLEN(ItEm)
#define RE_SV_TAIL(ItEm)
#define isDEBUG_WILDCARD 0

#endif /* DEBUG RELATED DEFINES */

#define FIRST_NON_ASCII_DECIMAL_DIGIT 0x660  /* ARABIC_INDIC_DIGIT_ZERO */

typedef enum {
	TRADITIONAL_BOUND = _CC_WORDCHAR,
	GCB_BOUND,
	LB_BOUND,
	SB_BOUND,
	WB_BOUND
} bound_type;

/* This unpacks the FLAGS field of ANYOF[HR]x nodes.  The value it contains
 * gives the strict lower bound for the UTF-8 start byte of any code point
 * matchable by the node, and a loose upper bound as well.
 *
 * The low bound is stored in the upper 6 bits, plus 0xC0.
 * The loose upper bound is determined from the lowest 2 bits and the low bound
 * (called x) as follows:
 *
 * 11  The upper limit of the range can be as much as (EF - x) / 8
 * 10  The upper limit of the range can be as much as (EF - x) / 4
 * 01  The upper limit of the range can be as much as (EF - x) / 2
 * 00  The upper limit of the range can be as much as  EF
 *
 * For motivation of this design, see commit message in
 * 3146c00a633e9cbed741e10146662fbcedfdb8d3 */
#ifdef EBCDIC
#  define MAX_ANYOF_HRx_BYTE  0xF4
#else
#  define MAX_ANYOF_HRx_BYTE  0xEF
#endif
#define LOWEST_ANYOF_HRx_BYTE(b) (((b) >> 2) + 0xC0)
#define HIGHEST_ANYOF_HRx_BYTE(b)                                           \
                                  (LOWEST_ANYOF_HRx_BYTE(b)                 \
          + ((MAX_ANYOF_HRx_BYTE - LOWEST_ANYOF_HRx_BYTE(b)) >> ((b) & 3)))

#endif /* PERL_REGCOMP_H_ */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
