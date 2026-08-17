/*    pp.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1994, 1995, 1996, 1998, 1999, 2000, 2001,
 *    2002, 2003, 2004, 2005, 2006, 2007, 2008 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#define PP(s) OP * Perl_##s(pTHX)

/*
=head1 Stack Manipulation Macros

=for apidoc AmnU||SP
Stack pointer.  This is usually handled by C<xsubpp>.  See C<L</dSP>> and
C<SPAGAIN>.

=for apidoc AmnU||MARK
Stack marker variable for the XSUB.  See C<L</dMARK>>.

=for apidoc Am|void|PUSHMARK|SP
Opening bracket for arguments on a callback.  See C<L</PUTBACK>> and
L<perlcall>.

=for apidoc Amns||dSP
Declares a local copy of perl's stack pointer for the XSUB, available via
the C<SP> macro.  See C<L</SP>>.

=for apidoc ms||djSP

Declare Just C<SP>.  This is actually identical to C<dSP>, and declares
a local copy of perl's stack pointer, available via the C<SP> macro.
See C<L<perlapi/SP>>.  (Available for backward source code compatibility with
the old (Perl 5.005) thread model.)

=for apidoc Amns||dMARK
Declare a stack marker variable, C<mark>, for the XSUB.  See C<L</MARK>> and
C<L</dORIGMARK>>.

=for apidoc Amns||dORIGMARK
Saves the original stack mark for the XSUB.  See C<L</ORIGMARK>>.

=for apidoc AmnU||ORIGMARK
The original stack mark for the XSUB.  See C<L</dORIGMARK>>.

=for apidoc Amns||SPAGAIN
Refetch the stack pointer.  Used after a callback.  See L<perlcall>.

=cut */

#undef SP /* Solaris 2.7 i386 has this in /usr/include/sys/reg.h */
#define SP sp
#define MARK mark
#define TARG targ

#define PUSHMARK(p) \
    STMT_START {                                                      \
        I32 * mark_stack_entry;                                       \
        if (UNLIKELY((mark_stack_entry = ++PL_markstack_ptr)          \
                                           == PL_markstack_max))      \
	    mark_stack_entry = markstack_grow();                      \
        *mark_stack_entry  = (I32)((p) - PL_stack_base);              \
        DEBUG_s(DEBUG_v(PerlIO_printf(Perl_debug_log,                 \
                "MARK push %p %" IVdf "\n",                           \
                PL_markstack_ptr, (IV)*mark_stack_entry)));           \
    } STMT_END

#define TOPMARK Perl_TOPMARK(aTHX)
#define POPMARK Perl_POPMARK(aTHX)

#define INCMARK \
    STMT_START {                                                      \
        DEBUG_s(DEBUG_v(PerlIO_printf(Perl_debug_log,                 \
                "MARK inc  %p %" IVdf "\n",                           \
                (PL_markstack_ptr+1), (IV)*(PL_markstack_ptr+1))));   \
        PL_markstack_ptr++;                                           \
    } STMT_END

#define dSP		SV **sp = PL_stack_sp
#define djSP		dSP
#define dMARK		SV **mark = PL_stack_base + POPMARK
#define dORIGMARK	const I32 origmark = (I32)(mark - PL_stack_base)
#define ORIGMARK	(PL_stack_base + origmark)

#define SPAGAIN		sp = PL_stack_sp
#define MSPAGAIN	STMT_START { sp = PL_stack_sp; mark = ORIGMARK; } STMT_END

#define GETTARGETSTACKED targ = (PL_op->op_flags & OPf_STACKED ? POPs : PAD_SV(PL_op->op_targ))
#define dTARGETSTACKED SV * GETTARGETSTACKED

#define GETTARGET targ = PAD_SV(PL_op->op_targ)
#define dTARGET SV * GETTARGET

#define GETATARGET targ = (PL_op->op_flags & OPf_STACKED ? sp[-1] : PAD_SV(PL_op->op_targ))
#define dATARGET SV * GETATARGET

#define dTARG SV *targ

#define NORMAL PL_op->op_next
#define DIE return Perl_die

/*
=for apidoc Amns||PUTBACK
Closing bracket for XSUB arguments.  This is usually handled by C<xsubpp>.
See C<L</PUSHMARK>> and L<perlcall> for other uses.

=for apidoc Amn|SV*|POPs
Pops an SV off the stack.

=for apidoc Amn|char*|POPp
Pops a string off the stack.

=for apidoc Amn|char*|POPpx
Pops a string off the stack.  Identical to POPp.  There are two names for
historical reasons.

=for apidoc Amn|char*|POPpbytex
Pops a string off the stack which must consist of bytes i.e. characters < 256.

=for apidoc Amn|NV|POPn
Pops a double off the stack.

=for apidoc Amn|IV|POPi
Pops an integer off the stack.

=for apidoc Amn|UV|POPu
Pops an unsigned integer off the stack.

=for apidoc Amn|long|POPl
Pops a long off the stack.

=for apidoc Amn|long|POPul
Pops an unsigned long off the stack.

=cut
*/

#define PUTBACK		PL_stack_sp = sp
#define RETURN		return (PUTBACK, NORMAL)
#define RETURNOP(o)	return (PUTBACK, o)
#define RETURNX(x)	return (x, PUTBACK, NORMAL)

#define POPs		(*sp--)
#define POPp		POPpx
#define POPpx		(SvPVx_nolen(POPs))
#define POPpconstx	(SvPVx_nolen_const(POPs))
#define POPpbytex	(SvPVbytex_nolen(POPs))
#define POPn		(SvNVx(POPs))
#define POPi		((IV)SvIVx(POPs))
#define POPu		((UV)SvUVx(POPs))
#define POPl		((long)SvIVx(POPs))
#define POPul		((unsigned long)SvIVx(POPs))

#define TOPs		(*sp)
#define TOPm1s		(*(sp-1))
#define TOPp1s		(*(sp+1))
#define TOPp		TOPpx
#define TOPpx		(SvPV_nolen(TOPs))
#define TOPn		(SvNV(TOPs))
#define TOPi		((IV)SvIV(TOPs))
#define TOPu		((UV)SvUV(TOPs))
#define TOPl		((long)SvIV(TOPs))
#define TOPul		((unsigned long)SvUV(TOPs))

/* Go to some pains in the rare event that we must extend the stack. */

/*
=for apidoc Am|void|EXTEND|SP|SSize_t nitems
Used to extend the argument stack for an XSUB's return values.  Once
used, guarantees that there is room for at least C<nitems> to be pushed
onto the stack.

=for apidoc Am|void|PUSHs|SV* sv
Push an SV onto the stack.  The stack must have room for this element.
Does not handle 'set' magic.  Does not use C<TARG>.  See also
C<L</PUSHmortal>>, C<L</XPUSHs>>, and C<L</XPUSHmortal>>.

=for apidoc Am|void|PUSHp|char* str|STRLEN len
Push a string onto the stack.  The stack must have room for this element.
The C<len> indicates the length of the string.  Handles 'set' magic.  Uses
C<TARG>, so C<dTARGET> or C<dXSTARG> should be called to declare it.  Do not
call multiple C<TARG>-oriented macros to return lists from XSUB's - see
C<L</mPUSHp>> instead.  See also C<L</XPUSHp>> and C<L</mXPUSHp>>.

=for apidoc Am|void|PUSHn|NV nv
Push a double onto the stack.  The stack must have room for this element.
Handles 'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG> should be
called to declare it.  Do not call multiple C<TARG>-oriented macros to
return lists from XSUB's - see C<L</mPUSHn>> instead.  See also C<L</XPUSHn>>
and C<L</mXPUSHn>>.

=for apidoc Am|void|PUSHi|IV iv
Push an integer onto the stack.  The stack must have room for this element.
Handles 'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG> should be
called to declare it.  Do not call multiple C<TARG>-oriented macros to 
return lists from XSUB's - see C<L</mPUSHi>> instead.  See also C<L</XPUSHi>>
and C<L</mXPUSHi>>.

=for apidoc Am|void|PUSHu|UV uv
Push an unsigned integer onto the stack.  The stack must have room for this
element.  Handles 'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG>
should be called to declare it.  Do not call multiple C<TARG>-oriented
macros to return lists from XSUB's - see C<L</mPUSHu>> instead.  See also
C<L</XPUSHu>> and C<L</mXPUSHu>>.

=for apidoc Am|void|XPUSHs|SV* sv
Push an SV onto the stack, extending the stack if necessary.  Does not
handle 'set' magic.  Does not use C<TARG>.  See also C<L</XPUSHmortal>>,
C<PUSHs> and C<PUSHmortal>.

=for apidoc Am|void|XPUSHp|char* str|STRLEN len
Push a string onto the stack, extending the stack if necessary.  The C<len>
indicates the length of the string.  Handles 'set' magic.  Uses C<TARG>, so
C<dTARGET> or C<dXSTARG> should be called to declare it.  Do not call
multiple C<TARG>-oriented macros to return lists from XSUB's - see
C<L</mXPUSHp>> instead.  See also C<L</PUSHp>> and C<L</mPUSHp>>.

=for apidoc Am|void|XPUSHn|NV nv
Push a double onto the stack, extending the stack if necessary.  Handles
'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG> should be called to
declare it.  Do not call multiple C<TARG>-oriented macros to return lists
from XSUB's - see C<L</mXPUSHn>> instead.  See also C<L</PUSHn>> and
C<L</mPUSHn>>.

=for apidoc Am|void|XPUSHi|IV iv
Push an integer onto the stack, extending the stack if necessary.  Handles
'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG> should be called to
declare it.  Do not call multiple C<TARG>-oriented macros to return lists
from XSUB's - see C<L</mXPUSHi>> instead.  See also C<L</PUSHi>> and
C<L</mPUSHi>>.

=for apidoc Am|void|XPUSHu|UV uv
Push an unsigned integer onto the stack, extending the stack if necessary.
Handles 'set' magic.  Uses C<TARG>, so C<dTARGET> or C<dXSTARG> should be
called to declare it.  Do not call multiple C<TARG>-oriented macros to
return lists from XSUB's - see C<L</mXPUSHu>> instead.  See also C<L</PUSHu>> and
C<L</mPUSHu>>.

=for apidoc Am|void|mPUSHs|SV* sv
Push an SV onto the stack and mortalizes the SV.  The stack must have room
for this element.  Does not use C<TARG>.  See also C<L</PUSHs>> and
C<L</mXPUSHs>>.

=for apidoc Amn|void|PUSHmortal
Push a new mortal SV onto the stack.  The stack must have room for this
element.  Does not use C<TARG>.  See also C<L</PUSHs>>, C<L</XPUSHmortal>> and
C<L</XPUSHs>>.

=for apidoc Am|void|mPUSHp|char* str|STRLEN len
Push a string onto the stack.  The stack must have room for this element.
The C<len> indicates the length of the string.  Does not use C<TARG>.
See also C<L</PUSHp>>, C<L</mXPUSHp>> and C<L</XPUSHp>>.

=for apidoc Am|void|mPUSHn|NV nv
Push a double onto the stack.  The stack must have room for this element.
Does not use C<TARG>.  See also C<L</PUSHn>>, C<L</mXPUSHn>> and C<L</XPUSHn>>.

=for apidoc Am|void|mPUSHi|IV iv
Push an integer onto the stack.  The stack must have room for this element.
Does not use C<TARG>.  See also C<L</PUSHi>>, C<L</mXPUSHi>> and C<L</XPUSHi>>.

=for apidoc Am|void|mPUSHu|UV uv
Push an unsigned integer onto the stack.  The stack must have room for this
element.  Does not use C<TARG>.  See also C<L</PUSHu>>, C<L</mXPUSHu>> and
C<L</XPUSHu>>.

=for apidoc Am|void|mXPUSHs|SV* sv
Push an SV onto the stack, extending the stack if necessary and mortalizes
the SV.  Does not use C<TARG>.  See also C<L</XPUSHs>> and C<L</mPUSHs>>.

=for apidoc Amn|void|XPUSHmortal
Push a new mortal SV onto the stack, extending the stack if necessary.
Does not use C<TARG>.  See also C<L</XPUSHs>>, C<L</PUSHmortal>> and
C<L</PUSHs>>.

=for apidoc Am|void|mXPUSHp|char* str|STRLEN len
Push a string onto the stack, extending the stack if necessary.  The C<len>
indicates the length of the string.  Does not use C<TARG>.  See also
C<L</XPUSHp>>, C<mPUSHp> and C<PUSHp>.

=for apidoc Am|void|mXPUSHn|NV nv
Push a double onto the stack, extending the stack if necessary.
Does not use C<TARG>.  See also C<L</XPUSHn>>, C<L</mPUSHn>> and C<L</PUSHn>>.

=for apidoc Am|void|mXPUSHi|IV iv
Push an integer onto the stack, extending the stack if necessary.
Does not use C<TARG>.  See also C<L</XPUSHi>>, C<L</mPUSHi>> and C<L</PUSHi>>.

=for apidoc Am|void|mXPUSHu|UV uv
Push an unsigned integer onto the stack, extending the stack if necessary.
Does not use C<TARG>.  See also C<L</XPUSHu>>, C<L</mPUSHu>> and C<L</PUSHu>>.

=cut
*/

/* EXTEND_HWM_SET: note the high-water-mark to which the stack has been
 * requested to be extended (which is likely to be less than PL_stack_max)
 */
#if defined DEBUGGING && !defined DEBUGGING_RE_ONLY
#  define EXTEND_HWM_SET(p, n)                      \
        STMT_START {                                \
            SSize_t ix = (p) - PL_stack_base + (n); \
            if (ix > PL_curstackinfo->si_stack_hwm) \
                PL_curstackinfo->si_stack_hwm = ix; \
        } STMT_END
#else
#  define EXTEND_HWM_SET(p, n) NOOP
#endif

/* _EXTEND_SAFE_N(n): private helper macro for EXTEND().
 * Tests whether the value of n would be truncated when implicitly cast to
 * SSize_t as an arg to stack_grow(). If so, sets it to -1 instead to
 * trigger a panic. It will be constant folded on platforms where this
 * can't happen.
 */

#define _EXTEND_SAFE_N(n) \
        (sizeof(n) > sizeof(SSize_t) && ((SSize_t)(n) != (n)) ? -1 : (n))

#ifdef STRESS_REALLOC
# define EXTEND_SKIP(p, n) EXTEND_HWM_SET(p, n)

# define EXTEND(p,n)   STMT_START {                                     \
                           sp = stack_grow(sp,p,_EXTEND_SAFE_N(n));     \
                           PERL_UNUSED_VAR(sp);                         \
                       } STMT_END
/* Same thing, but update mark register too. */
# define MEXTEND(p,n)   STMT_START {                                    \
                            const SSize_t markoff = mark - PL_stack_base; \
                            sp = stack_grow(sp,p,_EXTEND_SAFE_N(n));    \
                            mark = PL_stack_base + markoff;             \
                            PERL_UNUSED_VAR(sp);                        \
                        } STMT_END
#else

/* _EXTEND_NEEDS_GROW(p,n): private helper macro for EXTEND().
 * Tests to see whether n is too big and we need to grow the stack. Be
 * very careful if modifying this. There are many ways to get things wrong
 * (wrapping, truncating etc) that could cause a false negative and cause
 * the call to stack_grow() to be skipped. On the other hand, false
 * positives are safe.
 * Bear in mind that sizeof(p) may be less than, equal to, or greater
 * than sizeof(n), and while n is documented to be signed, someone might
 * pass an unsigned value or expression. In general don't use casts to
 * avoid warnings; instead expect the caller to fix their code.
 * It is legal for p to be greater than PL_stack_max.
 * If the allocated stack is already very large but current usage is
 * small, then PL_stack_max - p might wrap round to a negative value, but
 * this just gives a safe false positive
 */

#  define _EXTEND_NEEDS_GROW(p,n) ((n) < 0 || PL_stack_max - (p) < (n))


/* EXTEND_SKIP(): used for where you would normally call EXTEND(), but
 * you know for sure that a previous op will have already extended the
 * stack sufficiently.  For example pp_enteriter ensures that there
 * is always at least 1 free slot, so pp_iter can return &PL_sv_yes/no
 * without checking each time. Calling EXTEND_SKIP() defeats the HWM
 * debugging mechanism which would otherwise whine
 */

#  define EXTEND_SKIP(p, n) STMT_START {                                \
                                EXTEND_HWM_SET(p, n);                   \
                                assert(!_EXTEND_NEEDS_GROW(p,n));       \
                          } STMT_END


#  define EXTEND(p,n)   STMT_START {                                    \
                         EXTEND_HWM_SET(p, n);                          \
                         if (UNLIKELY(_EXTEND_NEEDS_GROW(p,n))) {       \
                           sp = stack_grow(sp,p,_EXTEND_SAFE_N(n));     \
                           PERL_UNUSED_VAR(sp);                         \
                         } } STMT_END
/* Same thing, but update mark register too. */
#  define MEXTEND(p,n)  STMT_START {                                    \
                         EXTEND_HWM_SET(p, n);                          \
                         if (UNLIKELY(_EXTEND_NEEDS_GROW(p,n))) {       \
                           const SSize_t markoff = mark - PL_stack_base;\
                           sp = stack_grow(sp,p,_EXTEND_SAFE_N(n));     \
                           mark = PL_stack_base + markoff;              \
                           PERL_UNUSED_VAR(sp);                         \
                         } } STMT_END
#endif


/* set TARG to the IV value i. If do_taint is false,
 * assume that PL_tainted can never be true */
#define TARGi(i, do_taint) \
    STMT_START {                                                        \
        IV TARGi_iv = i;                                                \
        if (LIKELY(                                                     \
              ((SvFLAGS(TARG) & (SVTYPEMASK|SVf_THINKFIRST|SVf_IVisUV)) == SVt_IV) \
            & (do_taint ? !TAINT_get : 1)))                             \
        {                                                               \
            /* Cheap SvIOK_only().                                      \
             * Assert that flags which SvIOK_only() would test or       \
             * clear can't be set, because we're SVt_IV */              \
            assert(!(SvFLAGS(TARG) &                                    \
                (SVf_OOK|SVf_UTF8|(SVf_OK & ~(SVf_IOK|SVp_IOK)))));     \
            SvFLAGS(TARG) |= (SVf_IOK|SVp_IOK);                         \
            /* SvIV_set() where sv_any points to head */                \
            TARG->sv_u.svu_iv = TARGi_iv;                               \
        }                                                               \
        else                                                            \
            sv_setiv_mg(targ, TARGi_iv);                                \
    } STMT_END

/* set TARG to the UV value u. If do_taint is false,
 * assume that PL_tainted can never be true */
#define TARGu(u, do_taint) \
    STMT_START {                                                        \
        UV TARGu_uv = u;                                                \
        if (LIKELY(                                                     \
              ((SvFLAGS(TARG) & (SVTYPEMASK|SVf_THINKFIRST|SVf_IVisUV)) == SVt_IV) \
            & (do_taint ? !TAINT_get : 1)                               \
            & (TARGu_uv <= (UV)IV_MAX)))                                \
        {                                                               \
            /* Cheap SvIOK_only().                                      \
             * Assert that flags which SvIOK_only() would test or       \
             * clear can't be set, because we're SVt_IV */              \
            assert(!(SvFLAGS(TARG) &                                    \
                (SVf_OOK|SVf_UTF8|(SVf_OK & ~(SVf_IOK|SVp_IOK)))));     \
            SvFLAGS(TARG) |= (SVf_IOK|SVp_IOK);                         \
            /* SvIV_set() where sv_any points to head */                \
            TARG->sv_u.svu_iv = TARGu_uv;                               \
        }                                                               \
        else                                                            \
            sv_setuv_mg(targ, TARGu_uv);                                \
    } STMT_END

/* set TARG to the NV value n. If do_taint is false,
 * assume that PL_tainted can never be true */
#define TARGn(n, do_taint) \
    STMT_START {                                                        \
        NV TARGn_nv = n;                                                \
        if (LIKELY(                                                     \
              ((SvFLAGS(TARG) & (SVTYPEMASK|SVf_THINKFIRST)) == SVt_NV) \
            & (do_taint ? !TAINT_get : 1)))                             \
        {                                                               \
            /* Cheap SvNOK_only().                                      \
             * Assert that flags which SvNOK_only() would test or       \
             * clear can't be set, because we're SVt_NV */              \
            assert(!(SvFLAGS(TARG) &                                    \
                (SVf_OOK|SVf_UTF8|(SVf_OK & ~(SVf_NOK|SVp_NOK)))));     \
            SvFLAGS(TARG) |= (SVf_NOK|SVp_NOK);                         \
            SvNV_set(TARG, TARGn_nv);                                   \
        }                                                               \
        else                                                            \
            sv_setnv_mg(targ, TARGn_nv);                                \
    } STMT_END

#define PUSHs(s)	(*++sp = (s))
#define PUSHTARG	STMT_START { SvSETMAGIC(TARG); PUSHs(TARG); } STMT_END
#define PUSHp(p,l)	STMT_START { sv_setpvn(TARG, (p), (l)); PUSHTARG; } STMT_END
#define PUSHn(n)	STMT_START { TARGn(n,1); PUSHs(TARG); } STMT_END
#define PUSHi(i)	STMT_START { TARGi(i,1); PUSHs(TARG); } STMT_END
#define PUSHu(u)	STMT_START { TARGu(u,1); PUSHs(TARG); } STMT_END

#define XPUSHs(s)	STMT_START { EXTEND(sp,1); *++sp = (s); } STMT_END
#define XPUSHTARG	STMT_START { SvSETMAGIC(TARG); XPUSHs(TARG); } STMT_END
#define XPUSHp(p,l)	STMT_START { sv_setpvn(TARG, (p), (l)); XPUSHTARG; } STMT_END
#define XPUSHn(n)	STMT_START { TARGn(n,1); XPUSHs(TARG); } STMT_END
#define XPUSHi(i)	STMT_START { TARGi(i,1); XPUSHs(TARG); } STMT_END
#define XPUSHu(u)	STMT_START { TARGu(u,1); XPUSHs(TARG); } STMT_END
#define XPUSHundef	STMT_START { SvOK_off(TARG); XPUSHs(TARG); } STMT_END

#define mPUSHs(s)	PUSHs(sv_2mortal(s))
#define PUSHmortal	PUSHs(sv_newmortal())
#define mPUSHp(p,l)	PUSHs(newSVpvn_flags((p), (l), SVs_TEMP))
#define mPUSHn(n)	sv_setnv(PUSHmortal, (NV)(n))
#define mPUSHi(i)	sv_setiv(PUSHmortal, (IV)(i))
#define mPUSHu(u)	sv_setuv(PUSHmortal, (UV)(u))

#define mXPUSHs(s)	XPUSHs(sv_2mortal(s))
#define XPUSHmortal	XPUSHs(sv_newmortal())
#define mXPUSHp(p,l)	STMT_START { EXTEND(sp,1); mPUSHp((p), (l)); } STMT_END
#define mXPUSHn(n)	STMT_START { EXTEND(sp,1); mPUSHn(n); } STMT_END
#define mXPUSHi(i)	STMT_START { EXTEND(sp,1); mPUSHi(i); } STMT_END
#define mXPUSHu(u)	STMT_START { EXTEND(sp,1); mPUSHu(u); } STMT_END

#define SETs(s)		(*sp = s)
#define SETTARG		STMT_START { SvSETMAGIC(TARG); SETs(TARG); } STMT_END
#define SETp(p,l)	STMT_START { sv_setpvn(TARG, (p), (l)); SETTARG; } STMT_END
#define SETn(n)		STMT_START { TARGn(n,1); SETs(TARG); } STMT_END
#define SETi(i)		STMT_START { TARGi(i,1); SETs(TARG); } STMT_END
#define SETu(u)		STMT_START { TARGu(u,1); SETs(TARG); } STMT_END

#define dTOPss		SV *sv = TOPs
#define dPOPss		SV *sv = POPs
#define dTOPnv		NV value = TOPn
#define dPOPnv		NV value = POPn
#define dPOPnv_nomg	NV value = (sp--, SvNV_nomg(TOPp1s))
#define dTOPiv		IV value = TOPi
#define dPOPiv		IV value = POPi
#define dTOPuv		UV value = TOPu
#define dPOPuv		UV value = POPu

#define dPOPXssrl(X)	SV *right = POPs; SV *left = CAT2(X,s)
#define dPOPXnnrl(X)	NV right = POPn; NV left = CAT2(X,n)
#define dPOPXiirl(X)	IV right = POPi; IV left = CAT2(X,i)

#define USE_LEFT(sv) \
	(SvOK(sv) || !(PL_op->op_flags & OPf_STACKED))
#define dPOPXiirl_ul_nomg(X) \
    IV right = (sp--, SvIV_nomg(TOPp1s));		\
    SV *leftsv = CAT2(X,s);				\
    IV left = USE_LEFT(leftsv) ? SvIV_nomg(leftsv) : 0

#define dPOPPOPssrl	dPOPXssrl(POP)
#define dPOPPOPnnrl	dPOPXnnrl(POP)
#define dPOPPOPiirl	dPOPXiirl(POP)

#define dPOPTOPssrl	dPOPXssrl(TOP)
#define dPOPTOPnnrl	dPOPXnnrl(TOP)
#define dPOPTOPnnrl_nomg \
    NV right = SvNV_nomg(TOPs); NV left = (sp--, SvNV_nomg(TOPs))
#define dPOPTOPiirl	dPOPXiirl(TOP)
#define dPOPTOPiirl_ul_nomg dPOPXiirl_ul_nomg(TOP)
#define dPOPTOPiirl_nomg \
    IV right = SvIV_nomg(TOPs); IV left = (sp--, SvIV_nomg(TOPs))

#define RETPUSHYES	RETURNX(PUSHs(&PL_sv_yes))
#define RETPUSHNO	RETURNX(PUSHs(&PL_sv_no))
#define RETPUSHUNDEF	RETURNX(PUSHs(&PL_sv_undef))

#define RETSETYES	RETURNX(SETs(&PL_sv_yes))
#define RETSETNO	RETURNX(SETs(&PL_sv_no))
#define RETSETUNDEF	RETURNX(SETs(&PL_sv_undef))
#define RETSETTARG	STMT_START { SETTARG; RETURN; } STMT_END

#define ARGTARG		PL_op->op_targ

#define MAXARG		(PL_op->op_private & OPpARG4_MASK)

#define SWITCHSTACK(f,t) \
    STMT_START {							\
	AvFILLp(f) = sp - PL_stack_base;				\
	PL_stack_base = AvARRAY(t);					\
	PL_stack_max = PL_stack_base + AvMAX(t);			\
	sp = PL_stack_sp = PL_stack_base + AvFILLp(t);			\
	PL_curstack = t;						\
    } STMT_END

#define EXTEND_MORTAL(n) \
    STMT_START {						\
	SSize_t eMiX = PL_tmps_ix + (n);			\
	if (UNLIKELY(eMiX >= PL_tmps_max))			\
	    (void)Perl_tmps_grow_p(aTHX_ eMiX);			\
    } STMT_END

#define AMGf_noright	1
#define AMGf_noleft	2
#define AMGf_assign	4       /* op supports mutator variant, e.g. $x += 1 */
#define AMGf_unary	8
#define AMGf_numeric	0x10	/* for Perl_try_amagic_bin */

#define AMGf_want_list	0x40
#define AMGf_numarg	0x80


/* do SvGETMAGIC on the stack args before checking for overload */

#define tryAMAGICun_MG(method, flags) STMT_START { \
	if ( UNLIKELY((SvFLAGS(TOPs) & (SVf_ROK|SVs_GMG))) \
		&& Perl_try_amagic_un(aTHX_ method, flags)) \
	    return NORMAL; \
    } STMT_END
#define tryAMAGICbin_MG(method, flags) STMT_START { \
	if ( UNLIKELY(((SvFLAGS(TOPm1s)|SvFLAGS(TOPs)) & (SVf_ROK|SVs_GMG))) \
		&& Perl_try_amagic_bin(aTHX_ method, flags)) \
	    return NORMAL; \
    } STMT_END

#define AMG_CALLunary(sv,meth) \
    amagic_call(sv,&PL_sv_undef, meth, AMGf_noright | AMGf_unary)

/* No longer used in core. Use AMG_CALLunary instead */
#define AMG_CALLun(sv,meth) AMG_CALLunary(sv, CAT2(meth,_amg))

#define tryAMAGICunTARGETlist(meth, jump)			\
    STMT_START {						\
	dSP;							\
	SV *tmpsv;						\
	SV *arg= *sp;						\
        U8 gimme = GIMME_V;                                    \
	if (UNLIKELY(SvAMAGIC(arg) &&				\
	    (tmpsv = amagic_call(arg, &PL_sv_undef, meth,	\
				 AMGf_want_list | AMGf_noright	\
				|AMGf_unary))))                 \
        {                                       		\
	    SPAGAIN;						\
            if (gimme == G_VOID) {                              \
                NOOP;                                           \
            }                                                   \
            else if (gimme == G_ARRAY) {			\
                SSize_t i;                                      \
                SSize_t len;                                    \
                assert(SvTYPE(tmpsv) == SVt_PVAV);              \
                len = av_tindex((AV *)tmpsv) + 1;               \
                (void)POPs; /* get rid of the arg */            \
                EXTEND(sp, len);                                \
                for (i = 0; i < len; ++i)                       \
                    PUSHs(av_shift((AV *)tmpsv));               \
            }                                                   \
            else { /* AMGf_want_scalar */                       \
                dATARGET; /* just use the arg's location */     \
                sv_setsv(TARG, tmpsv);                          \
                if (PL_op->op_flags & OPf_STACKED)              \
                    sp--;                                       \
                SETTARG;                                        \
            }                                                   \
	    PUTBACK;						\
	    if (jump) {						\
	        OP *jump_o = NORMAL->op_next;                   \
		while (jump_o->op_type == OP_NULL)		\
		    jump_o = jump_o->op_next;			\
		assert(jump_o->op_type == OP_ENTERSUB);		\
		(void)POPMARK;                                        \
		return jump_o->op_next;				\
	    }							\
	    return NORMAL;					\
	}							\
    } STMT_END

/* This is no longer used anywhere in the core. You might wish to consider
   calling amagic_deref_call() directly, as it has a cleaner interface.  */
#define tryAMAGICunDEREF(meth)						\
    STMT_START {							\
	sv = amagic_deref_call(*sp, CAT2(meth,_amg));			\
	SPAGAIN;							\
    } STMT_END


/* 2019: no longer used in core */
#define opASSIGN (PL_op->op_flags & OPf_STACKED)

/*
=for apidoc mnU||LVRET
True if this op will be the return value of an lvalue subroutine

=cut */
#define LVRET ((PL_op->op_private & OPpMAYBE_LVSUB) && is_lvalue_sub())

#define SvCANEXISTDELETE(sv) \
 (!SvRMAGICAL(sv)            \
  || !(mg = mg_find((const SV *) sv, PERL_MAGIC_tied))           \
  || (   (stash = SvSTASH(SvRV(SvTIED_obj(MUTABLE_SV(sv), mg)))) \
      && gv_fetchmethod_autoload(stash, "EXISTS", TRUE)          \
      && gv_fetchmethod_autoload(stash, "DELETE", TRUE)          \
     )                       \
  )

#ifdef PERL_CORE

/* These are just for Perl_tied_method(), which is not part of the public API.
   Use 0x04 rather than the next available bit, to help the compiler if the
   architecture can generate more efficient instructions.  */
#  define TIED_METHOD_MORTALIZE_NOT_NEEDED	0x04
#  define TIED_METHOD_ARGUMENTS_ON_STACK	0x08
#  define TIED_METHOD_SAY			0x10

/* Used in various places that need to dereference a glob or globref */
#  define MAYBE_DEREF_GV_flags(sv,phlags)                          \
    (                                                               \
	(void)(phlags & SV_GMAGIC && (SvGETMAGIC(sv),0)),            \
	isGV_with_GP(sv)                                              \
	  ? (GV *)(sv)                                                \
	  : SvROK(sv) && SvTYPE(SvRV(sv)) <= SVt_PVLV &&               \
	    (SvGETMAGIC(SvRV(sv)), isGV_with_GP(SvRV(sv)))              \
	     ? (GV *)SvRV(sv)                                            \
	     : NULL                                                       \
    )
#  define MAYBE_DEREF_GV(sv)      MAYBE_DEREF_GV_flags(sv,SV_GMAGIC)
#  define MAYBE_DEREF_GV_nomg(sv) MAYBE_DEREF_GV_flags(sv,0)

#  define FIND_RUNCV_padid_eq	1
#  define FIND_RUNCV_level_eq	2

#endif

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
