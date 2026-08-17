/*    av.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1995, 1996, 1997, 1998, 1999, 2000,
 *    2001, 2002, 2005, 2006, 2007, 2008, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

struct xpvav {
    HV*		xmg_stash;	/* class package */
    union _xmgu	xmg_u;
    SSize_t	xav_fill;       /* Index of last element present */
    SSize_t	xav_max;        /* max index for which array has space */
    SV**	xav_alloc;	/* pointer to beginning of C array of SVs */
};

/* SV*	xav_arylen; */

/* SVpav_REAL is set for all AVs whose xav_array contents are refcounted.
 * Some things like "@_" and the scratchpad list do not set this, to
 * indicate that they are cheating (for efficiency) by not refcounting
 * the AV's contents.
 * 
 * SVpav_REIFY is only meaningful on such "fake" AVs (i.e. where SVpav_REAL
 * is not set).  It indicates that the fake AV is capable of becoming
 * real if the array needs to be modified in some way.  Functions that
 * modify fake AVs check both flags to call av_reify() as appropriate.
 *
 * Note that the Perl stack has neither flag set. (Thus,
 * items that go on the stack are never refcounted.)
 *
 * These internal details are subject to change any time.  AV
 * manipulations external to perl should not care about any of this.
 * GSAR 1999-09-10
 */

/*
=head1 Handy Values

=for apidoc ADmnU||Nullav
Null AV pointer.

(deprecated - use C<(AV *)NULL> instead)

=head1 Array Manipulation Functions

=for apidoc Am|int|AvFILL|AV* av
Same as C<av_top_index()> or C<av_tindex()>.

=for apidoc av_tindex
Same as C<av_top_index()>.

=for apidoc m|int|AvFILLp|AV* av

=cut
*/

#ifndef PERL_CORE
#  define Nullav Null(AV*)
#endif

#define AvARRAY(av)	((av)->sv_u.svu_array)
#define AvALLOC(av)	((XPVAV*)  SvANY(av))->xav_alloc
#define AvMAX(av)	((XPVAV*)  SvANY(av))->xav_max
#define AvFILLp(av)	((XPVAV*)  SvANY(av))->xav_fill
#define AvARYLEN(av)	(*Perl_av_arylen_p(aTHX_ MUTABLE_AV(av)))

#define AvREAL(av)	(SvFLAGS(av) & SVpav_REAL)
#define AvREAL_on(av)	(SvFLAGS(av) |= SVpav_REAL)
#define AvREAL_off(av)	(SvFLAGS(av) &= ~SVpav_REAL)
#define AvREAL_only(av)	(AvREIFY_off(av), SvFLAGS(av) |= SVpav_REAL)
#define AvREIFY(av)	(SvFLAGS(av) & SVpav_REIFY)
#define AvREIFY_on(av)	(SvFLAGS(av) |= SVpav_REIFY)
#define AvREIFY_off(av)	(SvFLAGS(av) &= ~SVpav_REIFY)
#define AvREIFY_only(av)	(AvREAL_off(av), SvFLAGS(av) |= SVpav_REIFY)


#define AvREALISH(av)	(SvFLAGS(av) & (SVpav_REAL|SVpav_REIFY))
                                          
#define AvFILL(av)	((SvRMAGICAL((const SV *) (av))) \
			 ? mg_size(MUTABLE_SV(av)) : AvFILLp(av))
#define av_tindex(av)   av_top_index(av)

/* Note that it doesn't make sense to do this:
 *      SvGETMAGIC(av); IV x = av_tindex_nomg(av);
 */
#   define av_top_index_skip_len_mg(av)                                     \
                            (__ASSERT_(SvTYPE(av) == SVt_PVAV) AvFILLp(av))
#   define av_tindex_skip_len_mg(av)  av_top_index_skip_len_mg(av)

#define NEGATIVE_INDICES_VAR "NEGATIVE_INDICES"

/*
=for apidoc newAV

Creates a new AV.  The reference count is set to 1.

Perl equivalent: C<my @array;>.

=cut
*/

#define newAV()	MUTABLE_AV(newSV_type(SVt_PVAV))

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
