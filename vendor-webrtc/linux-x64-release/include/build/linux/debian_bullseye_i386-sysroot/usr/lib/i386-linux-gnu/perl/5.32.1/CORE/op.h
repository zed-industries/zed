/*    op.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000,
 *    2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
 * The fields of BASEOP are:
 *	op_next		Pointer to next ppcode to execute after this one.
 *			(Top level pre-grafted op points to first op,
 *			but this is replaced when op is grafted in, when
 *			this op will point to the real next op, and the new
 *			parent takes over role of remembering starting op.)
 *	op_ppaddr	Pointer to current ppcode's function.
 *	op_type		The type of the operation.
 *	op_opt		Whether or not the op has been optimised by the
 *			peephole optimiser.
 *	op_slabbed	allocated via opslab
 *	op_static	tell op_free() to skip PerlMemShared_free(), when
 *                      !op_slabbed.
 *	op_savefree	on savestack via SAVEFREEOP
 *	op_folded	Result/remainder of a constant fold operation.
 *	op_moresib	this op is not the last sibling
 *	op_spare	One spare bit
 *	op_flags	Flags common to all operations.  See OPf_* below.
 *	op_private	Flags peculiar to a particular operation (BUT,
 *			by default, set to the number of children until
 *			the operation is privatized by a check routine,
 *			which may or may not check number of children).
 */
#include "op_reg_common.h"

#define OPCODE U16

typedef PERL_BITFIELD16 Optype;

#ifdef BASEOP_DEFINITION
#define BASEOP BASEOP_DEFINITION
#else
#define BASEOP				\
    OP*		op_next;		\
    OP*		op_sibparent;		\
    OP*		(*op_ppaddr)(pTHX);	\
    PADOFFSET	op_targ;		\
    PERL_BITFIELD16 op_type:9;		\
    PERL_BITFIELD16 op_opt:1;		\
    PERL_BITFIELD16 op_slabbed:1;	\
    PERL_BITFIELD16 op_savefree:1;	\
    PERL_BITFIELD16 op_static:1;	\
    PERL_BITFIELD16 op_folded:1;	\
    PERL_BITFIELD16 op_moresib:1;       \
    PERL_BITFIELD16 op_spare:1;		\
    U8		op_flags;		\
    U8		op_private;
#endif

/* If op_type:9 is changed to :10, also change cx_pusheval()
   Also, if the type of op_type is ever changed (e.g. to PERL_BITFIELD32)
   then all the other bit-fields before/after it should change their
   types too to let VC pack them into the same 4 byte integer.*/

/* for efficiency, requires OPf_WANT_VOID == G_VOID etc */
#define OP_GIMME(op,dfl) \
	(((op)->op_flags & OPf_WANT) ? ((op)->op_flags & OPf_WANT) : dfl)

#define OP_GIMME_REVERSE(flags)	((flags) & G_WANT)

/*
=head1 "Gimme" Values

=for apidoc Amn|U32|GIMME_V
The XSUB-writer's equivalent to Perl's C<wantarray>.  Returns C<G_VOID>,
C<G_SCALAR> or C<G_ARRAY> for void, scalar or list context,
respectively.  See L<perlcall> for a usage example.

=for apidoc Amn|U32|GIMME
A backward-compatible version of C<GIMME_V> which can only return
C<G_SCALAR> or C<G_ARRAY>; in a void context, it returns C<G_SCALAR>.
Deprecated.  Use C<GIMME_V> instead.

=cut
*/

#define GIMME_V		Perl_gimme_V(aTHX)

/* Public flags */

#define OPf_WANT	3	/* Mask for "want" bits: */
#define  OPf_WANT_VOID	 1	/*   Want nothing */
#define  OPf_WANT_SCALAR 2	/*   Want single value */
#define  OPf_WANT_LIST	 3	/*   Want list of any length */
#define OPf_KIDS	4	/* There is a firstborn child. */
#define OPf_PARENS	8	/* This operator was parenthesized. */
				/*  (Or block needs explicit scope entry.) */
#define OPf_REF		16	/* Certified reference. */
				/*  (Return container, not containee). */
#define OPf_MOD		32	/* Will modify (lvalue). */

#define OPf_STACKED	64	/* Some arg is arriving on the stack. */
                                /*   Indicates mutator-variant of op for those
                                 *     ops which support them, e.g. $x += 1
                                 */

#define OPf_SPECIAL	128	/* Do something weird for this op: */
				/*  On local LVAL, don't init local value. */
				/*  On OP_SORT, subroutine is inlined. */
				/*  On OP_NOT, inversion was implicit. */
				/*  On OP_LEAVE, don't restore curpm, e.g.
                                 *      /(...)/ while ...>;  */
				/*  On truncate, we truncate filehandle */
				/*  On control verbs, we saw no label */
				/*  On flipflop, we saw ... instead of .. */
				/*  On UNOPs, saw bare parens, e.g. eof(). */
				/*  On OP_CHDIR, handle (or bare parens) */
				/*  On OP_NULL, saw a "do". */
				/*  On OP_EXISTS, treat av as av, not avhv.  */
				/*  On OP_(ENTER|LEAVE)EVAL, don't clear $@ */
				/*  On regcomp, "use re 'eval'" was in scope */
				/*  On RV2[ACGHS]V, don't create GV--in
				    defined()*/
				/*  On OP_DBSTATE, indicates breakpoint
				 *    (runtime property) */
				/*  On OP_REQUIRE, was seen as CORE::require */
				/*  On OP_(ENTER|LEAVE)WHEN, there's
				    no condition */
				/*  On OP_SMARTMATCH, an implicit smartmatch */
				/*  On OP_ANONHASH and OP_ANONLIST, create a
				    reference to the new anon hash or array */
				/*  On OP_HELEM, OP_MULTIDEREF and OP_HSLICE,
                                    localization will be followed by assignment,
                                    so do not wipe the target if it is special
                                    (e.g. a glob or a magic SV) */
				/*  On OP_MATCH, OP_SUBST & OP_TRANS, the
				    operand of a logical or conditional
				    that was optimised away, so it should
				    not be bound via =~ */
				/*  On OP_CONST, from a constant CV */
				/*  On OP_GLOB, two meanings:
				    - Before ck_glob, called as CORE::glob
				    - After ck_glob, use Perl glob function
			         */
                                /*  On OP_PADRANGE, push @_ */
                                /*  On OP_DUMP, has no label */
                                /*  On OP_UNSTACK, in a C-style for loop */
                                /*  On OP_READLINE, it's for <<>>, not <> */
/* There is no room in op_flags for this one, so it has its own bit-
   field member (op_folded) instead.  The flag is only used to tell
   op_convert_list to set op_folded.  */
#define OPf_FOLDED      (1<<16)

/* old names; don't use in new code, but don't break them, either */
#define OPf_LIST	OPf_WANT_LIST
#define OPf_KNOW	OPf_WANT

#if !defined(PERL_CORE) && !defined(PERL_EXT)
#  define GIMME \
	  (PL_op->op_flags & OPf_WANT					\
	   ? ((PL_op->op_flags & OPf_WANT) == OPf_WANT_LIST		\
	      ? G_ARRAY							\
	      : G_SCALAR)						\
	   : dowantarray())
#endif


/* NOTE: OPp* flags are now auto-generated and defined in opcode.h,
 *       from data in regen/op_private */


#define OPpTRANS_ALL	(OPpTRANS_USE_SVOP|OPpTRANS_CAN_FORCE_UTF8|OPpTRANS_IDENTICAL|OPpTRANS_SQUASH|OPpTRANS_COMPLEMENT|OPpTRANS_GROWS|OPpTRANS_DELETE)
#define OPpTRANS_FROM_UTF   OPpTRANS_USE_SVOP
#define OPpTRANS_TO_UTF     OPpTRANS_CAN_FORCE_UTF8


/* Mask for OP_ENTERSUB flags, the absence of which must be propagated
 in dynamic context */
#define OPpENTERSUB_LVAL_MASK (OPpLVAL_INTRO|OPpENTERSUB_INARGS)


/* things that can be elements of op_aux */
typedef union {
    PADOFFSET pad_offset;
    SV        *sv;
    IV        iv;
    UV        uv;
    char      *pv;
    SSize_t   ssize;
} UNOP_AUX_item;

#ifdef USE_ITHREADS
#  define UNOP_AUX_item_sv(item) PAD_SVl((item)->pad_offset);
#else
#  define UNOP_AUX_item_sv(item) ((item)->sv);
#endif




struct op {
    BASEOP
};

struct unop {
    BASEOP
    OP *	op_first;
};

struct unop_aux {
    BASEOP
    OP  	  *op_first;
    UNOP_AUX_item *op_aux;
};

struct binop {
    BASEOP
    OP *	op_first;
    OP *	op_last;
};

struct logop {
    BASEOP
    OP *	op_first;
    OP *	op_other;
};

struct listop {
    BASEOP
    OP *	op_first;
    OP *	op_last;
};

struct methop {
    BASEOP
    union {
        /* op_u.op_first *must* be aligned the same as the op_first
         * field of the other op types, and op_u.op_meth_sv *must*
         * be aligned with op_sv */
        OP* op_first;   /* optree for method name */
        SV* op_meth_sv; /* static method name */
    } op_u;
#ifdef USE_ITHREADS
    PADOFFSET op_rclass_targ; /* pad index for redirect class */
#else
    SV*       op_rclass_sv;   /* static redirect class $o->A::meth() */
#endif
};

struct pmop {
    BASEOP
    OP *	op_first;
    OP *	op_last;
#ifdef USE_ITHREADS
    PADOFFSET   op_pmoffset;
#else
    REGEXP *    op_pmregexp;            /* compiled expression */
#endif
    U32         op_pmflags;
    union {
	OP *	op_pmreplroot;		/* For OP_SUBST */
	PADOFFSET op_pmtargetoff;	/* For OP_SPLIT lex ary or thr GV */
	GV *	op_pmtargetgv;	        /* For OP_SPLIT non-threaded GV */
    }	op_pmreplrootu;
    union {
	OP *	op_pmreplstart;	/* Only used in OP_SUBST */
#ifdef USE_ITHREADS
	PADOFFSET op_pmstashoff; /* Only used in OP_MATCH, with PMf_ONCE set */
#else
	HV *	op_pmstash;
#endif
    }		op_pmstashstartu;
    OP *	op_code_list;	/* list of (?{}) code blocks */
};

#ifdef USE_ITHREADS
#define PM_GETRE(o)	(SvTYPE(PL_regex_pad[(o)->op_pmoffset]) == SVt_REGEXP \
		 	 ? (REGEXP*)(PL_regex_pad[(o)->op_pmoffset]) : NULL)
/* The assignment is just to enforce type safety (or at least get a warning).
 */
/* With first class regexps not via a reference one needs to assign
   &PL_sv_undef under ithreads. (This would probably work unthreaded, but NULL
   is cheaper. I guess we could allow NULL, but the check above would get
   more complex, and we'd have an AV with (SV*)NULL in it, which feels bad */
/* BEWARE - something that calls this macro passes (r) which has a side
   effect.  */
#define PM_SETRE(o,r)	STMT_START {					\
                            REGEXP *const _pm_setre = (r);		\
                            assert(_pm_setre);				\
			    PL_regex_pad[(o)->op_pmoffset] = MUTABLE_SV(_pm_setre); \
                        } STMT_END
#else
#define PM_GETRE(o)     ((o)->op_pmregexp)
#define PM_SETRE(o,r)   ((o)->op_pmregexp = (r))
#endif

/* Currently these PMf flags occupy a single 32-bit word.  Not all bits are
 * currently used.  The lower bits are shared with their corresponding RXf flag
 * bits, up to but not including _RXf_PMf_SHIFT_NEXT.  The unused bits
 * immediately follow; finally the used Pmf-only (unshared) bits, so that the
 * highest bit in the word is used.  This gathers all the unused bits as a pool
 * in the middle, like so: 11111111111111110000001111111111
 * where the '1's represent used bits, and the '0's unused.  This design allows
 * us to allocate off one end of the pool if we need to add a shared bit, and
 * off the other end if we need a non-shared bit, without disturbing the other
 * bits.  This maximizes the likelihood of being able to change things without
 * breaking binary compatibility.
 *
 * To add shared bits, do so in op_reg_common.h.  This should change
 * _RXf_PMf_SHIFT_NEXT so that things won't compile.  Then come to regexp.h and
 * op.h and adjust the constant adders in the definitions of PMf_BASE_SHIFT and
 * Pmf_BASE_SHIFT down by the number of shared bits you added.  That's it.
 * Things should be binary compatible.  But if either of these gets to having
 * to subtract rather than add, leave at 0 and adjust all the entries below
 * that are in terms of this according.  But if the first one of those is
 * already PMf_BASE_SHIFT+0, there are no bits left, and a redesign is in
 * order.
 *
 * To remove unshared bits, just delete its entry.  If you're where breaking
 * binary compatibility is ok to do, you might want to adjust things to move
 * the newly opened space so that it gets absorbed into the common pool.
 *
 * To add unshared bits, first use up any gaps in the middle.  Otherwise,
 * allocate off the low end until you get to PMf_BASE_SHIFT+0.  If that isn't
 * enough, move PMf_BASE_SHIFT down (if possible) and add the new bit at the
 * other end instead; this preserves binary compatibility. */
#define PMf_BASE_SHIFT (_RXf_PMf_SHIFT_NEXT+2)

/* Set by the parser if it discovers an error, so the regex shouldn't be
 * compiled */
#define PMf_HAS_ERROR	(1U<<(PMf_BASE_SHIFT+3))

/* 'use re "taint"' in scope: taint $1 etc. if target tainted */
#define PMf_RETAINT	(1U<<(PMf_BASE_SHIFT+4))

/* match successfully only once per reset, with related flag RXf_USED in
 * re->extflags holding state.  This is used only for ?? matches, and only on
 * OP_MATCH and OP_QR */
#define PMf_ONCE	(1U<<(PMf_BASE_SHIFT+5))

/* PMf_ONCE, i.e. ?pat?, has matched successfully.  Not used under threading. */
#define PMf_USED        (1U<<(PMf_BASE_SHIFT+6))

/* subst replacement is constant */
#define PMf_CONST	(1U<<(PMf_BASE_SHIFT+7))

/* keep 1st runtime pattern forever */
#define PMf_KEEP	(1U<<(PMf_BASE_SHIFT+8))

#define PMf_GLOBAL	(1U<<(PMf_BASE_SHIFT+9)) /* pattern had a g modifier */

/* don't reset pos() if //g fails */
#define PMf_CONTINUE	(1U<<(PMf_BASE_SHIFT+10))

/* evaluating replacement as expr */
#define PMf_EVAL	(1U<<(PMf_BASE_SHIFT+11))

/* Return substituted string instead of modifying it. */
#define PMf_NONDESTRUCT	(1U<<(PMf_BASE_SHIFT+12))

/* the pattern has a CV attached (currently only under qr/...(?{}).../) */
#define PMf_HAS_CV	(1U<<(PMf_BASE_SHIFT+13))

/* op_code_list is private; don't free it etc. It may well point to
 * code within another sub, with different pad etc */
#define PMf_CODELIST_PRIVATE	(1U<<(PMf_BASE_SHIFT+14))

/* the PMOP is a QR (we should be able to detect that from the op type,
 * but the regex compilation API passes just the pm flags, not the op
 * itself */
#define PMf_IS_QR	(1U<<(PMf_BASE_SHIFT+15))
#define PMf_USE_RE_EVAL	(1U<<(PMf_BASE_SHIFT+16)) /* use re'eval' in scope */

/* Means that this is a subpattern being compiled while processing a \p{}
 * wildcard.  This isn't called from op.c, but it is passed as a pm flag. */
#define PMf_WILDCARD    (1U<<(PMf_BASE_SHIFT+17))

/* See comments at the beginning of these defines about adding bits.  The
 * highest bit position should be used, so that if PMf_BASE_SHIFT gets
 * increased, the #error below will be triggered so that you will be reminded
 * to adjust things at the other end to keep the bit positions unchanged */
#if PMf_BASE_SHIFT+17 > 31
#   error Too many PMf_ bits used.  See above and regnodes.h for any spare in middle
#endif

#ifdef USE_ITHREADS

#  define PmopSTASH(o)         ((o)->op_pmflags & PMf_ONCE                         \
                                ? PL_stashpad[(o)->op_pmstashstartu.op_pmstashoff]   \
                                : NULL)
#  define PmopSTASH_set(o,hv)	\
	(assert_((o)->op_pmflags & PMf_ONCE)				\
	 (o)->op_pmstashstartu.op_pmstashoff =				\
	    (hv) ? alloccopstash(hv) : 0)
#else
#  define PmopSTASH(o)							\
    (((o)->op_pmflags & PMf_ONCE) ? (o)->op_pmstashstartu.op_pmstash : NULL)
#  if defined (DEBUGGING) && defined(__GNUC__) && !defined(PERL_GCC_BRACE_GROUPS_FORBIDDEN)
#    define PmopSTASH_set(o,hv)		({				\
	assert((o)->op_pmflags & PMf_ONCE);				\
	((o)->op_pmstashstartu.op_pmstash = (hv));			\
    })
#  else
#    define PmopSTASH_set(o,hv)	((o)->op_pmstashstartu.op_pmstash = (hv))
#  endif
#endif
#define PmopSTASHPV(o)	(PmopSTASH(o) ? HvNAME_get(PmopSTASH(o)) : NULL)
   /* op_pmstashstartu.op_pmstash is not refcounted */
#define PmopSTASHPV_set(o,pv)	PmopSTASH_set((o), gv_stashpv(pv,GV_ADD))

struct svop {
    BASEOP
    SV *	op_sv;
};

struct padop {
    BASEOP
    PADOFFSET	op_padix;
};

struct pvop {
    BASEOP
    char *	op_pv;
};

struct loop {
    BASEOP
    OP *	op_first;
    OP *	op_last;
    OP *	op_redoop;
    OP *	op_nextop;
    OP *	op_lastop;
};

#define cUNOPx(o)	((UNOP*)(o))
#define cUNOP_AUXx(o)	((UNOP_AUX*)(o))
#define cBINOPx(o)	((BINOP*)(o))
#define cLISTOPx(o)	((LISTOP*)(o))
#define cLOGOPx(o)	((LOGOP*)(o))
#define cPMOPx(o)	((PMOP*)(o))
#define cSVOPx(o)	((SVOP*)(o))
#define cPADOPx(o)	((PADOP*)(o))
#define cPVOPx(o)	((PVOP*)(o))
#define cCOPx(o)	((COP*)(o))
#define cLOOPx(o)	((LOOP*)(o))
#define cMETHOPx(o)	((METHOP*)(o))

#define cUNOP		cUNOPx(PL_op)
#define cUNOP_AUX	cUNOP_AUXx(PL_op)
#define cBINOP		cBINOPx(PL_op)
#define cLISTOP		cLISTOPx(PL_op)
#define cLOGOP		cLOGOPx(PL_op)
#define cPMOP		cPMOPx(PL_op)
#define cSVOP		cSVOPx(PL_op)
#define cPADOP		cPADOPx(PL_op)
#define cPVOP		cPVOPx(PL_op)
#define cCOP		cCOPx(PL_op)
#define cLOOP		cLOOPx(PL_op)

#define cUNOPo		cUNOPx(o)
#define cUNOP_AUXo	cUNOP_AUXx(o)
#define cBINOPo		cBINOPx(o)
#define cLISTOPo	cLISTOPx(o)
#define cLOGOPo		cLOGOPx(o)
#define cPMOPo		cPMOPx(o)
#define cSVOPo		cSVOPx(o)
#define cPADOPo		cPADOPx(o)
#define cPVOPo		cPVOPx(o)
#define cCOPo		cCOPx(o)
#define cLOOPo		cLOOPx(o)

#define kUNOP		cUNOPx(kid)
#define kUNOP_AUX	cUNOP_AUXx(kid)
#define kBINOP		cBINOPx(kid)
#define kLISTOP		cLISTOPx(kid)
#define kLOGOP		cLOGOPx(kid)
#define kPMOP		cPMOPx(kid)
#define kSVOP		cSVOPx(kid)
#define kPADOP		cPADOPx(kid)
#define kPVOP		cPVOPx(kid)
#define kCOP		cCOPx(kid)
#define kLOOP		cLOOPx(kid)


typedef enum {
    OPclass_NULL,     /*  0 */
    OPclass_BASEOP,   /*  1 */
    OPclass_UNOP,     /*  2 */
    OPclass_BINOP,    /*  3 */
    OPclass_LOGOP,    /*  4 */
    OPclass_LISTOP,   /*  5 */
    OPclass_PMOP,     /*  6 */
    OPclass_SVOP,     /*  7 */
    OPclass_PADOP,    /*  8 */
    OPclass_PVOP,     /*  9 */
    OPclass_LOOP,     /* 10 */
    OPclass_COP,      /* 11 */
    OPclass_METHOP,   /* 12 */
    OPclass_UNOP_AUX  /* 13 */
} OPclass;


#ifdef USE_ITHREADS
#  define	cGVOPx_gv(o)	((GV*)PAD_SVl(cPADOPx(o)->op_padix))
#  ifndef PERL_CORE
#    define	IS_PADGV(v)	(v && isGV(v))
#    define	IS_PADCONST(v) \
	(v && (SvREADONLY(v) || (SvIsCOW(v) && !SvLEN(v))))
#  endif
#  define	cSVOPx_sv(v)	(cSVOPx(v)->op_sv \
				 ? cSVOPx(v)->op_sv : PAD_SVl((v)->op_targ))
#  define	cSVOPx_svp(v)	(cSVOPx(v)->op_sv \
				 ? &cSVOPx(v)->op_sv : &PAD_SVl((v)->op_targ))
#  define	cMETHOPx_rclass(v) PAD_SVl(cMETHOPx(v)->op_rclass_targ)
#else
#  define	cGVOPx_gv(o)	((GV*)cSVOPx(o)->op_sv)
#  ifndef PERL_CORE
#    define	IS_PADGV(v)	FALSE
#    define	IS_PADCONST(v)	FALSE
#  endif
#  define	cSVOPx_sv(v)	(cSVOPx(v)->op_sv)
#  define	cSVOPx_svp(v)	(&cSVOPx(v)->op_sv)
#  define	cMETHOPx_rclass(v) (cMETHOPx(v)->op_rclass_sv)
#endif

#define	cMETHOPx_meth(v)	cSVOPx_sv(v)

#define	cGVOP_gv		cGVOPx_gv(PL_op)
#define	cGVOPo_gv		cGVOPx_gv(o)
#define	kGVOP_gv		cGVOPx_gv(kid)
#define cSVOP_sv		cSVOPx_sv(PL_op)
#define cSVOPo_sv		cSVOPx_sv(o)
#define kSVOP_sv		cSVOPx_sv(kid)

#ifndef PERL_CORE
#  define Nullop ((OP*)NULL)
#endif

/* Lowest byte of PL_opargs */
#define OA_MARK 1
#define OA_FOLDCONST 2
#define OA_RETSCALAR 4
#define OA_TARGET 8
#define OA_TARGLEX 16
#define OA_OTHERINT 32
#define OA_DANGEROUS 64
#define OA_DEFGV 128

/* The next 4 bits (8..11) encode op class information */
#define OCSHIFT 8

#define OA_CLASS_MASK (15 << OCSHIFT)

#define OA_BASEOP (0 << OCSHIFT)
#define OA_UNOP (1 << OCSHIFT)
#define OA_BINOP (2 << OCSHIFT)
#define OA_LOGOP (3 << OCSHIFT)
#define OA_LISTOP (4 << OCSHIFT)
#define OA_PMOP (5 << OCSHIFT)
#define OA_SVOP (6 << OCSHIFT)
#define OA_PADOP (7 << OCSHIFT)
#define OA_PVOP_OR_SVOP (8 << OCSHIFT)
#define OA_LOOP (9 << OCSHIFT)
#define OA_COP (10 << OCSHIFT)
#define OA_BASEOP_OR_UNOP (11 << OCSHIFT)
#define OA_FILESTATOP (12 << OCSHIFT)
#define OA_LOOPEXOP (13 << OCSHIFT)
#define OA_METHOP (14 << OCSHIFT)
#define OA_UNOP_AUX (15 << OCSHIFT)

/* Each remaining nybble of PL_opargs (i.e. bits 12..15, 16..19 etc)
 * encode the type for each arg */
#define OASHIFT 12

#define OA_SCALAR 1
#define OA_LIST 2
#define OA_AVREF 3
#define OA_HVREF 4
#define OA_CVREF 5
#define OA_FILEREF 6
#define OA_SCALARREF 7
#define OA_OPTIONAL 8

/* Op_REFCNT is a reference count at the head of each op tree: needed
 * since the tree is shared between threads, and between cloned closure
 * copies in the same thread. OP_REFCNT_LOCK/UNLOCK is used when modifying
 * this count.
 * The same mutex is used to protect the refcounts of the reg_trie_data
 * and reg_ac_data structures, which are shared between duplicated
 * regexes.
 */

#ifdef USE_ITHREADS
#  define OP_REFCNT_INIT		MUTEX_INIT(&PL_op_mutex)
#  ifdef PERL_CORE
#    define OP_REFCNT_LOCK		MUTEX_LOCK(&PL_op_mutex)
#    define OP_REFCNT_UNLOCK		MUTEX_UNLOCK(&PL_op_mutex)
#  else
#    define OP_REFCNT_LOCK		op_refcnt_lock()
#    define OP_REFCNT_UNLOCK		op_refcnt_unlock()
#  endif
#  define OP_REFCNT_TERM		MUTEX_DESTROY(&PL_op_mutex)
#else
#  define OP_REFCNT_INIT		NOOP
#  define OP_REFCNT_LOCK		NOOP
#  define OP_REFCNT_UNLOCK		NOOP
#  define OP_REFCNT_TERM		NOOP
#endif

#define OpREFCNT_set(o,n)		((o)->op_targ = (n))
#ifdef PERL_DEBUG_READONLY_OPS
#  define OpREFCNT_inc(o)		Perl_op_refcnt_inc(aTHX_ o)
#  define OpREFCNT_dec(o)		Perl_op_refcnt_dec(aTHX_ o)
#else
#  define OpREFCNT_inc(o)		((o) ? (++(o)->op_targ, (o)) : NULL)
#  define OpREFCNT_dec(o)		(--(o)->op_targ)
#endif

/* flags used by Perl_load_module() */
#define PERL_LOADMOD_DENY		0x1	/* no Module */
#define PERL_LOADMOD_NOIMPORT		0x2	/* use Module () */
#define PERL_LOADMOD_IMPORT_OPS		0x4	/* import arguments
						   are passed as a sin-
						   gle op tree, not a
						   list of SVs */

#if defined(PERL_IN_PERLY_C) || defined(PERL_IN_OP_C) || defined(PERL_IN_TOKE_C)
#define ref(o, type) doref(o, type, TRUE)
#endif


/* translation table attached to OP_TRANS/OP_TRANSR ops */

typedef struct {
    Size_t size; /* number of entries in map[], not including final slot */
    short map[1]; /* Unwarranted chumminess */
} OPtrans_map;


/*
=head1 Optree Manipulation Functions

=for apidoc Am|OP*|LINKLIST|OP *o
Given the root of an optree, link the tree in execution order using the
C<op_next> pointers and return the first op executed.  If this has
already been done, it will not be redone, and C<< o->op_next >> will be
returned.  If C<< o->op_next >> is not already set, C<o> should be at
least an C<UNOP>.

=cut
*/

#define LINKLIST(o) ((o)->op_next ? (o)->op_next : op_linklist((OP*)o))

/* no longer used anywhere in core */
#ifndef PERL_CORE
#define cv_ckproto(cv, gv, p) \
   cv_ckproto_len_flags((cv), (gv), (p), (p) ? strlen(p) : 0, 0)
#endif

#ifdef PERL_CORE
#  define my(o)	my_attrs((o), NULL)
#endif

#ifdef USE_REENTRANT_API
#include "reentr.h"
#endif

#define NewOp(m,var,c,type)	\
	(var = (type *) Perl_Slab_Alloc(aTHX_ c*sizeof(type)))
#define NewOpSz(m,var,size)	\
	(var = (OP *) Perl_Slab_Alloc(aTHX_ size))
#define FreeOp(p) Perl_Slab_Free(aTHX_ p)

/*
 * The per-CV op slabs consist of a header (the opslab struct) and a bunch
 * of space for allocating op slots, each of which consists of two pointers
 * followed by an op.  The first pointer points to the next op slot.  The
 * second points to the slab.  At the end of the slab is a null pointer,
 * so that slot->opslot_next - slot can be used to determine the size
 * of the op.
 *
 * Each CV can have multiple slabs; opslab_next points to the next slab, to
 * form a chain.  All bookkeeping is done on the first slab, which is where
 * all the op slots point.
 *
 * Freed ops are marked as freed and attached to the freed chain
 * via op_next pointers.
 *
 * When there is more than one slab, the second slab in the slab chain is
 * assumed to be the one with free space available.  It is used when allo-
 * cating an op if there are no freed ops available or big enough.
 */

#ifdef PERL_CORE
struct opslot {
    U16         opslot_size;        /* size of this slot (in pointers) */
    U16         opslot_offset;      /* offset from start of slab (in ptr units) */
    OP		opslot_op;		/* the op itself */
};

struct opslab {
    OPSLAB *	opslab_next;		/* next slab */
    OPSLAB *	opslab_head;		/* first slab in chain */
    OP **	opslab_freed;		/* array of sized chains of freed ops (head only)*/
    size_t	opslab_refcnt;		/* number of ops (head slab only) */
    U16         opslab_freed_size;      /* allocated size of opslab_freed */
    U16		opslab_size;		/* size of slab in pointers,
                                           including header */
    U16         opslab_free_space;	/* space available in this slab
                                           for allocating new ops (in ptr
                                           units) */
# ifdef PERL_DEBUG_READONLY_OPS
    bool	opslab_readonly;
    U8          opslab_padding;         /* padding to ensure that opslab_slots is always */
# else
    U16         opslab_padding;         /* located at an offset with 32-bit alignment */
# endif
    OPSLOT	opslab_slots;		/* slots begin here */
};

# define OPSLOT_HEADER		STRUCT_OFFSET(OPSLOT, opslot_op)
# define OPSLOT_HEADER_P	(OPSLOT_HEADER/sizeof(I32 *))
# define OpSLOT(o)		(assert_(o->op_slabbed) \
				 (OPSLOT *)(((char *)o)-OPSLOT_HEADER))

/* the first (head) opslab of the chain in which this op is allocated */
# define OpSLAB(o) \
    (((OPSLAB*)( (I32**)OpSLOT(o) - OpSLOT(o)->opslot_offset))->opslab_head)

# define OpslabREFCNT_dec(slab)      \
	(((slab)->opslab_refcnt == 1) \
	 ? opslab_free_nopad(slab)     \
	 : (void)--(slab)->opslab_refcnt)
  /* Variant that does not null out the pads */
# define OpslabREFCNT_dec_padok(slab) \
	(((slab)->opslab_refcnt == 1)  \
	 ? opslab_free(slab)		\
	 : (void)--(slab)->opslab_refcnt)
#endif

struct block_hooks {
    U32	    bhk_flags;
    void    (*bhk_start)	(pTHX_ int full);
    void    (*bhk_pre_end)	(pTHX_ OP **seq);
    void    (*bhk_post_end)	(pTHX_ OP **seq);
    void    (*bhk_eval)		(pTHX_ OP *const saveop);
};

/*
=head1 Compile-time scope hooks

=for apidoc mx|U32|BhkFLAGS|BHK *hk
Return the BHK's flags.

=for apidoc mxu|void *|BhkENTRY|BHK *hk|which
Return an entry from the BHK structure.  C<which> is a preprocessor token
indicating which entry to return.  If the appropriate flag is not set
this will return C<NULL>.  The type of the return value depends on which
entry you ask for.

=for apidoc Amxu|void|BhkENTRY_set|BHK *hk|which|void *ptr
Set an entry in the BHK structure, and set the flags to indicate it is
valid.  C<which> is a preprocessing token indicating which entry to set.
The type of C<ptr> depends on the entry.

=for apidoc Amxu|void|BhkDISABLE|BHK *hk|which
Temporarily disable an entry in this BHK structure, by clearing the
appropriate flag.  C<which> is a preprocessor token indicating which
entry to disable.

=for apidoc Amxu|void|BhkENABLE|BHK *hk|which
Re-enable an entry in this BHK structure, by setting the appropriate
flag.  C<which> is a preprocessor token indicating which entry to enable.
This will assert (under -DDEBUGGING) if the entry doesn't contain a valid
pointer.

=for apidoc mxu|void|CALL_BLOCK_HOOKS|which|arg
Call all the registered block hooks for type C<which>.  C<which> is a
preprocessing token; the type of C<arg> depends on C<which>.

=cut
*/

#define BhkFLAGS(hk)		((hk)->bhk_flags)

#define BHKf_bhk_start	    0x01
#define BHKf_bhk_pre_end    0x02
#define BHKf_bhk_post_end   0x04
#define BHKf_bhk_eval	    0x08

#define BhkENTRY(hk, which) \
    ((BhkFLAGS(hk) & BHKf_ ## which) ? ((hk)->which) : NULL)

#define BhkENABLE(hk, which) \
    STMT_START { \
	BhkFLAGS(hk) |= BHKf_ ## which; \
	assert(BhkENTRY(hk, which)); \
    } STMT_END

#define BhkDISABLE(hk, which) \
    STMT_START { \
	BhkFLAGS(hk) &= ~(BHKf_ ## which); \
    } STMT_END

#define BhkENTRY_set(hk, which, ptr) \
    STMT_START { \
	(hk)->which = ptr; \
	BhkENABLE(hk, which); \
    } STMT_END

#define CALL_BLOCK_HOOKS(which, arg) \
    STMT_START { \
	if (PL_blockhooks) { \
	    SSize_t i; \
	    for (i = av_tindex(PL_blockhooks); i >= 0; i--) { \
		SV *sv = AvARRAY(PL_blockhooks)[i]; \
		BHK *hk; \
		\
		assert(SvIOK(sv)); \
		if (SvUOK(sv)) \
		    hk = INT2PTR(BHK *, SvUVX(sv)); \
		else \
		    hk = INT2PTR(BHK *, SvIVX(sv)); \
		\
		if (BhkENTRY(hk, which)) \
		    BhkENTRY(hk, which)(aTHX_ arg); \
	    } \
	} \
    } STMT_END

/* flags for rv2cv_op_cv */

#define RV2CVOPCV_MARK_EARLY     0x00000001
#define RV2CVOPCV_RETURN_NAME_GV 0x00000002
#define RV2CVOPCV_RETURN_STUB    0x00000004
#ifdef PERL_CORE /* behaviour of this flag is subject to change: */
# define RV2CVOPCV_MAYBE_NAME_GV  0x00000008
#endif
#define RV2CVOPCV_FLAG_MASK      0x0000000f /* all of the above */

#define op_lvalue(op,t) Perl_op_lvalue_flags(aTHX_ op,t,0)

/* flags for op_lvalue_flags */

#define OP_LVALUE_NO_CROAK 1

/*
=head1 Custom Operators

=for apidoc Am|U32|XopFLAGS|XOP *xop
Return the XOP's flags.

=for apidoc Am||XopENTRY|XOP *xop|which
Return a member of the XOP structure.  C<which> is a cpp token
indicating which entry to return.  If the member is not set
this will return a default value.  The return type depends
on C<which>.  This macro evaluates its arguments more than
once.  If you are using C<Perl_custom_op_xop> to retreive a
C<XOP *> from a C<OP *>, use the more efficient L</XopENTRYCUSTOM> instead.

=for apidoc Am||XopENTRYCUSTOM|const OP *o|which
Exactly like C<XopENTRY(XopENTRY(Perl_custom_op_xop(aTHX_ o), which)> but more
efficient.  The C<which> parameter is identical to L</XopENTRY>.

=for apidoc Am|void|XopENTRY_set|XOP *xop|which|value
Set a member of the XOP structure.  C<which> is a cpp token
indicating which entry to set.  See L<perlguts/"Custom Operators">
for details about the available members and how
they are used.  This macro evaluates its argument
more than once.

=for apidoc Am|void|XopDISABLE|XOP *xop|which
Temporarily disable a member of the XOP, by clearing the appropriate flag.

=for apidoc Am|void|XopENABLE|XOP *xop|which
Reenable a member of the XOP which has been disabled.

=cut
*/

struct custom_op {
    U32		    xop_flags;    
    const char	   *xop_name;
    const char	   *xop_desc;
    U32		    xop_class;
    void	  (*xop_peep)(pTHX_ OP *o, OP *oldop);
};

/* return value of Perl_custom_op_get_field, similar to void * then casting but
   the U32 doesn't need truncation on 64 bit platforms in the caller, also
   for easier macro writing */
typedef union {
    const char	   *xop_name;
    const char	   *xop_desc;
    U32		    xop_class;
    void	  (*xop_peep)(pTHX_ OP *o, OP *oldop);
    XOP            *xop_ptr;
} XOPRETANY;

#define XopFLAGS(xop) ((xop)->xop_flags)

#define XOPf_xop_name	0x01
#define XOPf_xop_desc	0x02
#define XOPf_xop_class	0x04
#define XOPf_xop_peep	0x08

/* used by Perl_custom_op_get_field for option checking */
typedef enum {
    XOPe_xop_ptr = 0, /* just get the XOP *, don't look inside it */
    XOPe_xop_name = XOPf_xop_name,
    XOPe_xop_desc = XOPf_xop_desc,
    XOPe_xop_class = XOPf_xop_class,
    XOPe_xop_peep = XOPf_xop_peep
} xop_flags_enum;

#define XOPd_xop_name	PL_op_name[OP_CUSTOM]
#define XOPd_xop_desc	PL_op_desc[OP_CUSTOM]
#define XOPd_xop_class	OA_BASEOP
#define XOPd_xop_peep	((Perl_cpeep_t)0)

#define XopENTRY_set(xop, which, to) \
    STMT_START { \
	(xop)->which = (to); \
	(xop)->xop_flags |= XOPf_ ## which; \
    } STMT_END

#define XopENTRY(xop, which) \
    ((XopFLAGS(xop) & XOPf_ ## which) ? (xop)->which : XOPd_ ## which)

#define XopENTRYCUSTOM(o, which) \
    (Perl_custom_op_get_field(aTHX_ o, XOPe_ ## which).which)

#define XopDISABLE(xop, which) ((xop)->xop_flags &= ~XOPf_ ## which)
#define XopENABLE(xop, which) \
    STMT_START { \
	(xop)->xop_flags |= XOPf_ ## which; \
	assert(XopENTRY(xop, which)); \
    } STMT_END

#define Perl_custom_op_xop(x) \
    (Perl_custom_op_get_field(x, XOPe_xop_ptr).xop_ptr)

/*
=head1 Optree Manipulation Functions

=for apidoc Am|const char *|OP_NAME|OP *o
Return the name of the provided OP.  For core ops this looks up the name
from the op_type; for custom ops from the op_ppaddr.

=for apidoc Am|const char *|OP_DESC|OP *o
Return a short description of the provided OP.

=for apidoc Am|U32|OP_CLASS|OP *o
Return the class of the provided OP: that is, which of the *OP
structures it uses.  For core ops this currently gets the information out
of C<PL_opargs>, which does not always accurately reflect the type used;
in v5.26 onwards, see also the function C<L</op_class>> which can do a better
job of determining the used type.

For custom ops the type is returned from the registration, and it is up
to the registree to ensure it is accurate.  The value returned will be
one of the C<OA_>* constants from F<op.h>.

=for apidoc Am|bool|OP_TYPE_IS|OP *o|Optype type
Returns true if the given OP is not a C<NULL> pointer
and if it is of the given type.

The negation of this macro, C<OP_TYPE_ISNT> is also available
as well as C<OP_TYPE_IS_NN> and C<OP_TYPE_ISNT_NN> which elide
the NULL pointer check.

=for apidoc Am|bool|OP_TYPE_IS_OR_WAS|OP *o|Optype type
Returns true if the given OP is not a NULL pointer and
if it is of the given type or used to be before being
replaced by an OP of type OP_NULL.

The negation of this macro, C<OP_TYPE_ISNT_AND_WASNT>
is also available as well as C<OP_TYPE_IS_OR_WAS_NN>
and C<OP_TYPE_ISNT_AND_WASNT_NN> which elide
the C<NULL> pointer check.

=for apidoc Am|bool|OpHAS_SIBLING|OP *o
Returns true if C<o> has a sibling

=for apidoc Am|OP*|OpSIBLING|OP *o
Returns the sibling of C<o>, or C<NULL> if there is no sibling

=for apidoc Am|void|OpMORESIB_set|OP *o|OP *sib
Sets the sibling of C<o> to the non-zero value C<sib>. See also C<L</OpLASTSIB_set>>
and C<L</OpMAYBESIB_set>>. For a higher-level interface, see
C<L</op_sibling_splice>>.

=for apidoc Am|void|OpLASTSIB_set|OP *o|OP *parent
Marks C<o> as having no further siblings and marks
o as having the specified parent. See also C<L</OpMORESIB_set>> and
C<OpMAYBESIB_set>. For a higher-level interface, see
C<L</op_sibling_splice>>.

=for apidoc Am|void|OpMAYBESIB_set|OP *o|OP *sib|OP *parent
Conditionally does C<OpMORESIB_set> or C<OpLASTSIB_set> depending on whether
C<sib> is non-null. For a higher-level interface, see C<L</op_sibling_splice>>.

=cut
*/

#define OP_NAME(o) ((o)->op_type == OP_CUSTOM \
                    ? XopENTRYCUSTOM(o, xop_name) \
		    : PL_op_name[(o)->op_type])
#define OP_DESC(o) ((o)->op_type == OP_CUSTOM \
                    ? XopENTRYCUSTOM(o, xop_desc) \
		    : PL_op_desc[(o)->op_type])
#define OP_CLASS(o) ((o)->op_type == OP_CUSTOM \
		     ? XopENTRYCUSTOM(o, xop_class) \
		     : (PL_opargs[(o)->op_type] & OA_CLASS_MASK))

#define OP_TYPE_IS(o, type) ((o) && (o)->op_type == (type))
#define OP_TYPE_IS_NN(o, type) ((o)->op_type == (type))
#define OP_TYPE_ISNT(o, type) ((o) && (o)->op_type != (type))
#define OP_TYPE_ISNT_NN(o, type) ((o)->op_type != (type))

#define OP_TYPE_IS_OR_WAS_NN(o, type) \
    ( ((o)->op_type == OP_NULL \
       ? (o)->op_targ \
       : (o)->op_type) \
      == (type) )

#define OP_TYPE_IS_OR_WAS(o, type) \
    ( (o) && OP_TYPE_IS_OR_WAS_NN(o, type) )

#define OP_TYPE_ISNT_AND_WASNT_NN(o, type) \
    ( ((o)->op_type == OP_NULL \
       ? (o)->op_targ \
       : (o)->op_type) \
      != (type) )

#define OP_TYPE_ISNT_AND_WASNT(o, type) \
    ( (o) && OP_TYPE_ISNT_AND_WASNT_NN(o, type) )

/* should match anything that uses ck_ftst in regen/opcodes */
#define OP_IS_STAT(op) (OP_IS_FILETEST(op) || (op) == OP_LSTAT || (op) == OP_STAT)

#define OpHAS_SIBLING(o)	(cBOOL((o)->op_moresib))
#define OpSIBLING(o)		(0 + (o)->op_moresib ? (o)->op_sibparent : NULL)
#define OpMORESIB_set(o, sib) ((o)->op_moresib = 1, (o)->op_sibparent = (sib))
#define OpLASTSIB_set(o, parent) \
    ((o)->op_moresib = 0, (o)->op_sibparent = (parent))
#define OpMAYBESIB_set(o, sib, parent) \
    ((o)->op_sibparent = ((o)->op_moresib = cBOOL(sib)) ? (sib) : (parent))

#if !defined(PERL_CORE) && !defined(PERL_EXT)
/* for backwards compatibility only */
#  define OP_SIBLING(o)		OpSIBLING(o)
#endif

#define newATTRSUB(f, o, p, a, b) Perl_newATTRSUB_x(aTHX_  f, o, p, a, b, FALSE)
#define newSUB(f, o, p, b)	newATTRSUB((f), (o), (p), NULL, (b))

/*
=head1 Hook manipulation
*/

#ifdef USE_ITHREADS
#  define OP_CHECK_MUTEX_INIT		MUTEX_INIT(&PL_check_mutex)
#  define OP_CHECK_MUTEX_LOCK		MUTEX_LOCK(&PL_check_mutex)
#  define OP_CHECK_MUTEX_UNLOCK		MUTEX_UNLOCK(&PL_check_mutex)
#  define OP_CHECK_MUTEX_TERM		MUTEX_DESTROY(&PL_check_mutex)
#else
#  define OP_CHECK_MUTEX_INIT		NOOP
#  define OP_CHECK_MUTEX_LOCK		NOOP
#  define OP_CHECK_MUTEX_UNLOCK		NOOP
#  define OP_CHECK_MUTEX_TERM		NOOP
#endif


/* Stuff for OP_MULTDEREF/pp_multideref. */

/* actions */

/* Load another word of actions/flag bits. Must be 0 */
#define MDEREF_reload                       0

#define MDEREF_AV_pop_rv2av_aelem           1
#define MDEREF_AV_gvsv_vivify_rv2av_aelem   2
#define MDEREF_AV_padsv_vivify_rv2av_aelem  3
#define MDEREF_AV_vivify_rv2av_aelem        4
#define MDEREF_AV_padav_aelem               5
#define MDEREF_AV_gvav_aelem                6

#define MDEREF_HV_pop_rv2hv_helem           8
#define MDEREF_HV_gvsv_vivify_rv2hv_helem   9
#define MDEREF_HV_padsv_vivify_rv2hv_helem 10
#define MDEREF_HV_vivify_rv2hv_helem       11
#define MDEREF_HV_padhv_helem              12
#define MDEREF_HV_gvhv_helem               13

#define MDEREF_ACTION_MASK                0xf

/* key / index type */

#define MDEREF_INDEX_none   0x00 /* run external ops to generate index */
#define MDEREF_INDEX_const  0x10 /* index is const PV/UV */
#define MDEREF_INDEX_padsv  0x20 /* index is lexical var */
#define MDEREF_INDEX_gvsv   0x30 /* index is GV */

#define MDEREF_INDEX_MASK   0x30

/* bit flags */

#define MDEREF_FLAG_last    0x40 /* the last [ah]elem; PL_op flags apply */

#define MDEREF_MASK         0x7F
#define MDEREF_SHIFT           7

#if defined(PERL_IN_DOOP_C) || defined(PERL_IN_PP_C)
#   define FATAL_ABOVE_FF_MSG                                       \
      "Use of strings with code points over 0xFF as arguments to "  \
      "%s operator is not allowed"
#endif
#if defined(PERL_IN_OP_C) || defined(PERL_IN_DOOP_C) || defined(PERL_IN_PERL_C)
#  define TR_UNMAPPED           (UV)-1
#  define TR_DELETE             (UV)-2
#  define TR_R_EMPTY            (UV)-3  /* rhs (replacement) is empty */
#  define TR_OOB                (UV)-4  /* Something that isn't one of the others */
#  define TR_SPECIAL_HANDLING   TR_DELETE /* Can occupy same value */
#  define TR_UNLISTED           TR_UNMAPPED /* A synonym whose name is clearer
                                               at times */
#endif
#if defined(PERL_IN_OP_C) || defined(PERL_IN_TOKE_C)
#define RANGE_INDICATOR  ILLEGAL_UTF8_BYTE
#endif

/* stuff for OP_ARGCHECK */

struct op_argcheck_aux {
    UV   params;     /* number of positional parameters */
    UV   opt_params; /* number of optional positional parameters */
    char slurpy;     /* presence of slurpy: may be '\0', '@' or '%' */
};


/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
