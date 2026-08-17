/*    pad.h
 *
 *    Copyright (C) 2002, 2003, 2005, 2006, 2007, 2008,
 *    2009, 2010, 2011 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 * This file defines the types and macros associated with the API for
 * manipulating scratchpads, which are used by perl to store lexical
 * variables, op targets and constants.
 */

/*
=head1 Pad Data Structures
*/


/* offsets within a pad */

typedef SSize_t PADOFFSET; /* signed so that -1 is a valid value */
#define NOT_IN_PAD ((PADOFFSET) -1)

/* B.xs expects the first members of these two structs to line up
   (xpadl_max with xpadnl_fill).
 */

struct padlist {
    SSize_t	xpadl_max;	/* max index for which array has space */
    union {
	PAD **	xpadlarr_alloc; /* Pointer to beginning of array of AVs.
				   index 0 is a padnamelist *          */
	struct {
	    PADNAMELIST * padnl;
	    PAD * pad_1;        /* this slice of PAD * array always alloced */
	    PAD * pad_2;        /* maybe unalloced */
	} * xpadlarr_dbg;       /* for use with a C debugger only */
    } xpadl_arr;
    U32		xpadl_id;	/* Semi-unique ID, shared between clones */
    U32		xpadl_outid;	/* ID of outer pad */
};

struct padnamelist {
    SSize_t	xpadnl_fill;	/* max index in use */
    PADNAME **	xpadnl_alloc;	/* pointer to beginning of array */
    SSize_t	xpadnl_max;	/* max index for which array has space */
    PADOFFSET	xpadnl_max_named; /* highest index with len > 0 */
    U32		xpadnl_refcnt;
};

/* PERL_PADNAME_MINIMAL uses less memory, but on some platforms
   PERL_PADNAME_ALIGNED may be faster, so platform-specific hints can
   define one or the other.  */
#if defined(PERL_PADNAME_MINIMAL) && defined (PERL_PADNAME_ALIGNED)
#  error PERL_PADNAME_MINIMAL and PERL_PADNAME_ALIGNED are exclusive
#endif

#if !defined(PERL_PADNAME_MINIMAL) && !defined(PERL_PADNAME_ALIGNED)
#  define PERL_PADNAME_MINIMAL
#endif

#define _PADNAME_BASE \
    char *	xpadn_pv;		\
    HV *	xpadn_ourstash;		\
    union {				\
	HV *	xpadn_typestash;	\
	CV *	xpadn_protocv;		\
    } xpadn_type_u;			\
    U32		xpadn_low;		\
    U32		xpadn_high;		\
    U32		xpadn_refcnt;		\
    int		xpadn_gen;		\
    U8		xpadn_len;		\
    U8		xpadn_flags

struct padname {
    _PADNAME_BASE;
};

struct padname_with_str {
#ifdef PERL_PADNAME_MINIMAL
    _PADNAME_BASE;
#else
    struct padname	xpadn_padname;
#endif
    char		xpadn_str[1];
};

#undef _PADNAME_BASE

#define PADNAME_FROM_PV(s) \
    ((PADNAME *)((s) - STRUCT_OFFSET(struct padname_with_str, xpadn_str)))


/* a value that PL_cop_seqmax is guaranteed never to be,
 * flagging that a lexical is being introduced, or has not yet left scope
 */
#define PERL_PADSEQ_INTRO  U32_MAX
#define COP_SEQMAX_INC \
	(PL_cop_seqmax++, \
	 (void)(PL_cop_seqmax == PERL_PADSEQ_INTRO && PL_cop_seqmax++))


/* B.xs needs these for the benefit of B::Deparse */
/* Low range end is exclusive (valid from the cop seq after this one) */
/* High range end is inclusive (valid up to this cop seq) */

#define COP_SEQ_RANGE_LOW(pn)		(pn)->xpadn_low
#define COP_SEQ_RANGE_HIGH(pn)		(pn)->xpadn_high
#define PARENT_PAD_INDEX(pn)		(pn)->xpadn_low
#define PARENT_FAKELEX_FLAGS(pn)	(pn)->xpadn_high

/* Flags set in the SvIVX field of FAKE namesvs */

#define PAD_FAKELEX_ANON   1 /* the lex is declared in an ANON, or ... */
#define PAD_FAKELEX_MULTI  2 /* the lex can be instantiated multiple times */

/* flags for the pad_new() function */

#define padnew_CLONE	1	/* this pad is for a cloned CV */
#define padnew_SAVE	2	/* save old globals */
#define padnew_SAVESUB	4	/* also save extra stuff for start of sub */

/* values for the pad_tidy() function */

typedef enum {
	padtidy_SUB,		/* tidy up a pad for a sub, */
	padtidy_SUBCLONE,	/* a cloned sub, */
	padtidy_FORMAT		/* or a format */
} padtidy_type;

/* flags for pad_add_name_pvn. */

#define padadd_OUR		0x01	   /* our declaration. */
#define padadd_STATE		0x02	   /* state declaration. */
#define padadd_NO_DUP_CHECK	0x04	   /* skip warning on dups. */
#define padadd_STALEOK		0x08	   /* allow stale lexical in active
					    * sub, but only one level up */

/* ASSERT_CURPAD_LEGAL and ASSERT_CURPAD_ACTIVE respectively determine
 * whether PL_comppad and PL_curpad are consistent and whether they have
 * active values */

#  define pad_peg(label)

#ifdef DEBUGGING
#  define ASSERT_CURPAD_LEGAL(label) \
    pad_peg(label); \
    if (PL_comppad ? (AvARRAY(PL_comppad) != PL_curpad) : (PL_curpad != 0))  \
	Perl_croak(aTHX_ "panic: illegal pad in %s: 0x%" UVxf "[0x%" UVxf "]",\
	    label, PTR2UV(PL_comppad), PTR2UV(PL_curpad));


#  define ASSERT_CURPAD_ACTIVE(label) \
    pad_peg(label); \
    if (!PL_comppad || (AvARRAY(PL_comppad) != PL_curpad))		  \
	Perl_croak(aTHX_ "panic: invalid pad in %s: 0x%" UVxf "[0x%" UVxf "]",\
	    label, PTR2UV(PL_comppad), PTR2UV(PL_curpad));
#else
#  define ASSERT_CURPAD_LEGAL(label)
#  define ASSERT_CURPAD_ACTIVE(label)
#endif



/* Note: the following three macros are actually defined in scope.h, but
 * they are documented here for completeness, since they directly or
 * indirectly affect pads.

=for apidoc m|void|SAVEPADSV	|PADOFFSET po
Save a pad slot (used to restore after an iteration)

XXX DAPM it would make more sense to make the arg a PADOFFSET
=for apidoc m|void|SAVECLEARSV	|SV **svp
Clear the pointed to pad value on scope exit.  (i.e. the runtime action of
C<my>)

=for apidoc m|void|SAVECOMPPAD
save C<PL_comppad> and C<PL_curpad>


=for apidoc Amx|PAD **|PadlistARRAY|PADLIST * padlist
The C array of a padlist, containing the pads.  Only subscript it with
numbers >= 1, as the 0th entry is not guaranteed to remain usable.

=for apidoc Amx|SSize_t|PadlistMAX|PADLIST * padlist
The index of the last allocated space in the padlist.  Note that the last
pad may be in an earlier slot.  Any entries following it will be C<NULL> in
that case.

=for apidoc Amx|PADNAMELIST *|PadlistNAMES|PADLIST * padlist
The names associated with pad entries.

=for apidoc Amx|PADNAME **|PadlistNAMESARRAY|PADLIST * padlist
The C array of pad names.

=for apidoc Amx|SSize_t|PadlistNAMESMAX|PADLIST * padlist
The index of the last pad name.

=for apidoc Amx|U32|PadlistREFCNT|PADLIST * padlist
The reference count of the padlist.  Currently this is always 1.

=for apidoc Amx|PADNAME **|PadnamelistARRAY|PADNAMELIST * pnl
The C array of pad names.

=for apidoc Amx|SSize_t|PadnamelistMAX|PADNAMELIST * pnl
The index of the last pad name.

=for apidoc Amx|SSize_t|PadnamelistREFCNT|PADNAMELIST * pnl
The reference count of the pad name list.

=for apidoc Amx|void|PadnamelistREFCNT_dec|PADNAMELIST * pnl
Lowers the reference count of the pad name list.

=for apidoc Amx|SV **|PadARRAY|PAD * pad
The C array of pad entries.

=for apidoc Amx|SSize_t|PadMAX|PAD * pad
The index of the last pad entry.

=for apidoc Amx|char *|PadnamePV|PADNAME * pn
The name stored in the pad name struct.  This returns C<NULL> for a target
slot.

=for apidoc Amx|STRLEN|PadnameLEN|PADNAME * pn
The length of the name.

=for apidoc Amx|bool|PadnameUTF8|PADNAME * pn
Whether PadnamePV is in UTF-8.  Currently, this is always true.

=for apidoc Amx|SV *|PadnameSV|PADNAME * pn
Returns the pad name as a mortal SV.

=for apidoc m|bool|PadnameIsOUR|PADNAME * pn
Whether this is an "our" variable.

=for apidoc m|HV *|PadnameOURSTASH
The stash in which this "our" variable was declared.

=for apidoc m|bool|PadnameOUTER|PADNAME * pn
Whether this entry belongs to an outer pad.  Entries for which this is true
are often referred to as 'fake'.

=for apidoc m|bool|PadnameIsSTATE|PADNAME * pn
Whether this is a "state" variable.

=for apidoc m|HV *|PadnameTYPE|PADNAME * pn
The stash associated with a typed lexical.  This returns the C<%Foo::> hash
for C<my Foo $bar>.

=for apidoc Amx|SSize_t|PadnameREFCNT|PADNAME * pn
The reference count of the pad name.

=for apidoc Amx|void|PadnameREFCNT_dec|PADNAME * pn
Lowers the reference count of the pad name.


=for apidoc m|SV *|PAD_SETSV	|PADOFFSET po|SV* sv
Set the slot at offset C<po> in the current pad to C<sv>

=for apidoc m|SV *|PAD_SV	|PADOFFSET po
Get the value at offset C<po> in the current pad

=for apidoc m|SV *|PAD_SVl	|PADOFFSET po
Lightweight and lvalue version of C<PAD_SV>.
Get or set the value at offset C<po> in the current pad.
Unlike C<PAD_SV>, does not print diagnostics with -DX.
For internal use only.

=for apidoc m|SV *|PAD_BASE_SV	|PADLIST padlist|PADOFFSET po
Get the value from slot C<po> in the base (DEPTH=1) pad of a padlist

=for apidoc m|void|PAD_SET_CUR	|PADLIST padlist|I32 n
Set the current pad to be pad C<n> in the padlist, saving
the previous current pad.  NB currently this macro expands to a string too
long for some compilers, so it's best to replace it with

    SAVECOMPPAD();
    PAD_SET_CUR_NOSAVE(padlist,n);


=for apidoc m|void|PAD_SET_CUR_NOSAVE	|PADLIST padlist|I32 n
like PAD_SET_CUR, but without the save

=for apidoc m|void|PAD_SAVE_SETNULLPAD
Save the current pad then set it to null.

=for apidoc m|void|PAD_SAVE_LOCAL|PAD *opad|PAD *npad
Save the current pad to the local variable C<opad>, then make the
current pad equal to C<npad>

=for apidoc m|void|PAD_RESTORE_LOCAL|PAD *opad
Restore the old pad saved into the local variable C<opad> by C<PAD_SAVE_LOCAL()>

=cut
*/

#define PadlistARRAY(pl)	(pl)->xpadl_arr.xpadlarr_alloc
#define PadlistMAX(pl)		(pl)->xpadl_max
#define PadlistNAMES(pl)	*((PADNAMELIST **)PadlistARRAY(pl))
#define PadlistNAMESARRAY(pl)	PadnamelistARRAY(PadlistNAMES(pl))
#define PadlistNAMESMAX(pl)	PadnamelistMAX(PadlistNAMES(pl))
#define PadlistREFCNT(pl)	1	/* reserved for future use */

#define PadnamelistARRAY(pnl)		(pnl)->xpadnl_alloc
#define PadnamelistMAX(pnl)		(pnl)->xpadnl_fill
#define PadnamelistMAXNAMED(pnl)	(pnl)->xpadnl_max_named
#define PadnamelistREFCNT(pnl)		(pnl)->xpadnl_refcnt
#define PadnamelistREFCNT_dec(pnl)	Perl_padnamelist_free(aTHX_ pnl)

#define PadARRAY(pad)		AvARRAY(pad)
#define PadMAX(pad)		AvFILLp(pad)

#define PadnamePV(pn)		(pn)->xpadn_pv
#define PadnameLEN(pn)		(pn)->xpadn_len
#define PadnameUTF8(pn)		1
#define PadnameSV(pn) \
	newSVpvn_flags(PadnamePV(pn), PadnameLEN(pn), SVs_TEMP|SVf_UTF8)
#define PadnameFLAGS(pn)	(pn)->xpadn_flags
#define PadnameIsOUR(pn)	(!!(pn)->xpadn_ourstash)
#define PadnameOURSTASH(pn)	(pn)->xpadn_ourstash
#define PadnameTYPE(pn)		(pn)->xpadn_type_u.xpadn_typestash
#define PadnamePROTOCV(pn)	(pn)->xpadn_type_u.xpadn_protocv
#define PadnameREFCNT(pn)	(pn)->xpadn_refcnt
#define PadnameREFCNT_dec(pn)	Perl_padname_free(aTHX_ pn)
#define PadnameOURSTASH_set(pn,s) (PadnameOURSTASH(pn) = (s))
#define PadnameTYPE_set(pn,s)	  (PadnameTYPE(pn) = (s))
#define PadnameOUTER(pn)	(PadnameFLAGS(pn) & PADNAMEt_OUTER)
#define PadnameIsSTATE(pn)	(PadnameFLAGS(pn) & PADNAMEt_STATE)
#define PadnameLVALUE(pn)	(PadnameFLAGS(pn) & PADNAMEt_LVALUE)

#define PadnameLVALUE_on(pn)	(PadnameFLAGS(pn) |= PADNAMEt_LVALUE)
#define PadnameIsSTATE_on(pn)	(PadnameFLAGS(pn) |= PADNAMEt_STATE)

#define PADNAMEt_OUTER	1	/* outer lexical var */
#define PADNAMEt_STATE	2	/* state var */
#define PADNAMEt_LVALUE	4	/* used as lvalue */
#define PADNAMEt_TYPED	8	/* for B; unused by core */
#define PADNAMEt_OUR	16	/* for B; unused by core */

/* backward compatibility */
#define SvPAD_STATE		PadnameIsSTATE
#define SvPAD_TYPED(pn)		(!!PadnameTYPE(pn))
#define SvPAD_OUR(pn)		(!!PadnameOURSTASH(pn))
#define SvPAD_STATE_on		PadnameIsSTATE_on
#define SvPAD_TYPED_on(pn)	(PadnameFLAGS(pn) |= PADNAMEt_TYPED)
#define SvPAD_OUR_on(pn)	(PadnameFLAGS(pn) |= PADNAMEt_OUR)
#define SvOURSTASH		PadnameOURSTASH
#define SvOURSTASH_set		PadnameOURSTASH_set
#define SVpad_STATE		PADNAMEt_STATE
#define SVpad_TYPED		PADNAMEt_TYPED
#define SVpad_OUR		PADNAMEt_OUR

#ifdef DEBUGGING
#  define PAD_SV(po)	   pad_sv(po)
#  define PAD_SETSV(po,sv) pad_setsv(po,sv)
#else
#  define PAD_SV(po)       (PL_curpad[po])
#  define PAD_SETSV(po,sv) PL_curpad[po] = (sv)
#endif

#define PAD_SVl(po)       (PL_curpad[po])

#define PAD_BASE_SV(padlist, po) \
	(PadlistARRAY(padlist)[1])					\
	    ? AvARRAY(MUTABLE_AV((PadlistARRAY(padlist)[1])))[po] \
	    : NULL;


#define PAD_SET_CUR_NOSAVE(padlist,nth) \
	PL_comppad = (PAD*) (PadlistARRAY(padlist)[nth]);	\
	PL_curpad = AvARRAY(PL_comppad);			\
	DEBUG_Xv(PerlIO_printf(Perl_debug_log,			\
	      "Pad 0x%" UVxf "[0x%" UVxf "] set_cur    depth=%d\n",	\
	      PTR2UV(PL_comppad), PTR2UV(PL_curpad), (int)(nth)));


#define PAD_SET_CUR(padlist,nth) \
	SAVECOMPPAD();						\
	PAD_SET_CUR_NOSAVE(padlist,nth);


#define PAD_SAVE_SETNULLPAD()	SAVECOMPPAD(); \
	PL_comppad = NULL; PL_curpad = NULL;	\
	DEBUG_Xv(PerlIO_printf(Perl_debug_log, "Pad set_null\n"));

#define PAD_SAVE_LOCAL(opad,npad) \
	opad = PL_comppad;					\
	PL_comppad = (npad);					\
	PL_curpad =  PL_comppad ? AvARRAY(PL_comppad) : NULL;	\
	DEBUG_Xv(PerlIO_printf(Perl_debug_log,			\
	      "Pad 0x%" UVxf "[0x%" UVxf "] save_local\n",		\
	      PTR2UV(PL_comppad), PTR2UV(PL_curpad)));

#define PAD_RESTORE_LOCAL(opad) \
        assert(!opad || !SvIS_FREED(opad));					\
	PL_comppad = opad;						\
	PL_curpad =  PL_comppad ? AvARRAY(PL_comppad) : NULL;	\
	DEBUG_Xv(PerlIO_printf(Perl_debug_log,			\
	      "Pad 0x%" UVxf "[0x%" UVxf "] restore_local\n",	\
	      PTR2UV(PL_comppad), PTR2UV(PL_curpad)));


/*
=for apidoc m|void|CX_CURPAD_SAVE|struct context
Save the current pad in the given context block structure.

=for apidoc m|SV *|CX_CURPAD_SV|struct context|PADOFFSET po
Access the SV at offset C<po> in the saved current pad in the given
context block structure (can be used as an lvalue).

=cut
*/

#define CX_CURPAD_SAVE(block)  (block).oldcomppad = PL_comppad
#define CX_CURPAD_SV(block,po) (AvARRAY(MUTABLE_AV(((block).oldcomppad)))[po])


/*
=for apidoc m|U32|PAD_COMPNAME_FLAGS|PADOFFSET po
Return the flags for the current compiling pad name
at offset C<po>.  Assumes a valid slot entry.

=for apidoc m|char *|PAD_COMPNAME_PV|PADOFFSET po
Return the name of the current compiling pad name
at offset C<po>.  Assumes a valid slot entry.

=for apidoc m|HV *|PAD_COMPNAME_TYPE|PADOFFSET po
Return the type (stash) of the current compiling pad name at offset
C<po>.  Must be a valid name.  Returns null if not typed.

=for apidoc m|HV *|PAD_COMPNAME_OURSTASH|PADOFFSET po
Return the stash associated with an C<our> variable.
Assumes the slot entry is a valid C<our> lexical.

=for apidoc m|STRLEN|PAD_COMPNAME_GEN|PADOFFSET po
The generation number of the name at offset C<po> in the current
compiling pad (lvalue).

=for apidoc m|STRLEN|PAD_COMPNAME_GEN_set|PADOFFSET po|int gen
Sets the generation number of the name at offset C<po> in the current
ling pad (lvalue) to C<gen>.
=cut

*/

#define PAD_COMPNAME(po)	PAD_COMPNAME_SV(po)
#define PAD_COMPNAME_SV(po)	(PadnamelistARRAY(PL_comppad_name)[(po)])
#define PAD_COMPNAME_FLAGS(po)	PadnameFLAGS(PAD_COMPNAME(po))
#define PAD_COMPNAME_FLAGS_isOUR(po) SvPAD_OUR(PAD_COMPNAME_SV(po))
#define PAD_COMPNAME_PV(po)	PadnamePV(PAD_COMPNAME(po))

#define PAD_COMPNAME_TYPE(po)	PadnameTYPE(PAD_COMPNAME(po))

#define PAD_COMPNAME_OURSTASH(po) \
    (SvOURSTASH(PAD_COMPNAME_SV(po)))

#define PAD_COMPNAME_GEN(po) \
    ((STRLEN)PadnamelistARRAY(PL_comppad_name)[po]->xpadn_gen)

#define PAD_COMPNAME_GEN_set(po, gen) \
    (PadnamelistARRAY(PL_comppad_name)[po]->xpadn_gen = (gen))


/*
=for apidoc m|void|PAD_CLONE_VARS|PerlInterpreter *proto_perl|CLONE_PARAMS* param
Clone the state variables associated with running and compiling pads.

=cut
*/

/* NB - we set PL_comppad to null unless it points at a value that
 * has already been dup'ed, ie it points to part of an active padlist.
 * Otherwise PL_comppad ends up being a leaked scalar in code like
 * the following:
 *     threads->create(sub { threads->create(sub {...} ) } );
 * where the second thread dups the outer sub's comppad but not the
 * sub's CV or padlist. */

#define PAD_CLONE_VARS(proto_perl, param)				\
    PL_comppad			= av_dup(proto_perl->Icomppad, param);	\
    PL_curpad = PL_comppad ?  AvARRAY(PL_comppad) : NULL;		\
    PL_comppad_name		=					\
		  padnamelist_dup(proto_perl->Icomppad_name, param);	\
    PL_comppad_name_fill	= proto_perl->Icomppad_name_fill;	\
    PL_comppad_name_floor	= proto_perl->Icomppad_name_floor;	\
    PL_min_intro_pending	= proto_perl->Imin_intro_pending;	\
    PL_max_intro_pending	= proto_perl->Imax_intro_pending;	\
    PL_padix			= proto_perl->Ipadix;			\
    PL_padix_floor		= proto_perl->Ipadix_floor;		\
    PL_pad_reset_pending	= proto_perl->Ipad_reset_pending;	\
    PL_cop_seqmax		= proto_perl->Icop_seqmax;

/*
=for apidoc Am|PADOFFSET|pad_add_name_pvs|"name"|U32 flags|HV *typestash|HV *ourstash

Exactly like L</pad_add_name_pvn>, but takes a literal string
instead of a string/length pair.

=cut
*/

#define pad_add_name_pvs(name,flags,typestash,ourstash) \
    Perl_pad_add_name_pvn(aTHX_ STR_WITH_LEN(name), flags, typestash, ourstash)

/*
=for apidoc Am|PADOFFSET|pad_findmy_pvs|"name"|U32 flags

Exactly like L</pad_findmy_pvn>, but takes a literal string
instead of a string/length pair.

=cut
*/

#define pad_findmy_pvs(name,flags) \
    Perl_pad_findmy_pvn(aTHX_ STR_WITH_LEN(name), flags)

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
