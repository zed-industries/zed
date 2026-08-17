/*    cop.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000,
 *    2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 * Control ops (cops) are one of the two ops OP_NEXTSTATE and OP_DBSTATE,
 * that (loosely speaking) are statement separators.
 * They hold information important for lexical state and error reporting.
 * At run time, PL_curcop is set to point to the most recently executed cop,
 * and thus can be used to determine our current state.
 */

/* A jmpenv packages the state required to perform a proper non-local jump.
 * Note that there is a PL_start_env initialized when perl starts, and
 * PL_top_env points to this initially, so PL_top_env should always be
 * non-null.
 *
 * Existence of a non-null PL_top_env->je_prev implies it is valid to call
 * longjmp() at that runlevel (we make sure PL_start_env.je_prev is always
 * null to ensure this).
 *
 * je_mustcatch, when set at any runlevel to TRUE, means eval ops must
 * establish a local jmpenv to handle exception traps.  Care must be taken
 * to restore the previous value of je_mustcatch before exiting the
 * stack frame iff JMPENV_PUSH was not called in that stack frame.
 * GSAR 97-03-27
 */

struct jmpenv {
    struct jmpenv *	je_prev;
    Sigjmp_buf		je_buf;		/* uninit if je_prev is NULL */
    int			je_ret;		/* last exception thrown */
    bool		je_mustcatch;	/* need to call longjmp()? */
    U16                 je_old_delaymagic; /* saved PL_delaymagic */
    SSize_t             je_old_stack_hwm;
};

typedef struct jmpenv JMPENV;

#if defined DEBUGGING && !defined DEBUGGING_RE_ONLY
#  define JE_OLD_STACK_HWM_zero      PL_start_env.je_old_stack_hwm = 0
#  define JE_OLD_STACK_HWM_save(je)  \
        (je).je_old_stack_hwm = PL_curstackinfo->si_stack_hwm
#  define JE_OLD_STACK_HWM_restore(je)  \
        if (PL_curstackinfo->si_stack_hwm < (je).je_old_stack_hwm) \
            PL_curstackinfo->si_stack_hwm = (je).je_old_stack_hwm
#else
#  define JE_OLD_STACK_HWM_zero        NOOP
#  define JE_OLD_STACK_HWM_save(je)    NOOP
#  define JE_OLD_STACK_HWM_restore(je) NOOP
#endif

/*
 * How to build the first jmpenv.
 *
 * top_env needs to be non-zero. It points to an area
 * in which longjmp() stuff is stored, as C callstack
 * info there at least is thread specific this has to
 * be per-thread. Otherwise a 'die' in a thread gives
 * that thread the C stack of last thread to do an eval {}!
 */

#define JMPENV_BOOTSTRAP \
    STMT_START {				\
	PERL_POISON_EXPR(PoisonNew(&PL_start_env, 1, JMPENV));\
	PL_top_env = &PL_start_env;		\
	PL_start_env.je_prev = NULL;		\
	PL_start_env.je_ret = -1;		\
	PL_start_env.je_mustcatch = TRUE;	\
	PL_start_env.je_old_delaymagic = 0;	\
        JE_OLD_STACK_HWM_zero;                  \
    } STMT_END

/*
 *   PERL_FLEXIBLE_EXCEPTIONS
 *
 * All the flexible exceptions code has been removed.
 * See the following threads for details:
 *
 *   Message-Id: 20040713143217.GB1424@plum.flirble.org
 *   https://www.nntp.perl.org/group/perl.perl5.porters/2004/07/msg93041.html
 *
 * Joshua's original patches (which weren't applied) and discussion:
 *
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg01396.html
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg01489.html
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg01491.html
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg01608.html
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg02144.html
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1998-02/msg02998.html
 *
 * Chip's reworked patch and discussion:
 *
 *   http://www.xray.mpe.mpg.de/mailing-lists/perl5-porters/1999-03/msg00520.html
 *
 * The flaw in these patches (which went unnoticed at the time) was
 * that they moved some code that could potentially die() out of the
 * region protected by the setjmp()s.  This caused exceptions within
 * END blocks and such to not be handled by the correct setjmp().
 *
 * The original patches that introduces flexible exceptions were:
 *
 * https://github.com/Perl/perl5/commit/312caa8e97f1c7ee342a9895c2f0e749625b4929
 * https://github.com/Perl/perl5/commit/14dd3ad8c9bf82cf09798a22cc89a9862dfd6d1a
 *
 */

#define dJMPENV		JMPENV cur_env

#define JMPENV_PUSH(v) \
    STMT_START {							\
	DEBUG_l({							\
	    int i = 0; JMPENV *p = PL_top_env;				\
	    while (p) { i++; p = p->je_prev; }				\
	    Perl_deb(aTHX_ "JUMPENV_PUSH level=%d at %s:%d\n",		\
		         i,  __FILE__, __LINE__);})			\
	cur_env.je_prev = PL_top_env;					\
        JE_OLD_STACK_HWM_save(cur_env);                                 \
	cur_env.je_ret = PerlProc_setjmp(cur_env.je_buf, SCOPE_SAVES_SIGNAL_MASK);		\
        JE_OLD_STACK_HWM_restore(cur_env);                              \
	PL_top_env = &cur_env;						\
	cur_env.je_mustcatch = FALSE;					\
	cur_env.je_old_delaymagic = PL_delaymagic;			\
	(v) = cur_env.je_ret;						\
    } STMT_END

#define JMPENV_POP \
    STMT_START {							\
	DEBUG_l({							\
	    int i = -1; JMPENV *p = PL_top_env;				\
	    while (p) { i++; p = p->je_prev; }				\
	    Perl_deb(aTHX_ "JUMPENV_POP level=%d at %s:%d\n",		\
		         i, __FILE__, __LINE__);})			\
	assert(PL_top_env == &cur_env);					\
	PL_delaymagic = cur_env.je_old_delaymagic;			\
	PL_top_env = cur_env.je_prev;					\
    } STMT_END

#define JMPENV_JUMP(v) \
    STMT_START {						\
	DEBUG_l({						\
	    int i = -1; JMPENV *p = PL_top_env;			\
	    while (p) { i++; p = p->je_prev; }			\
	    Perl_deb(aTHX_ "JUMPENV_JUMP(%d) level=%d at %s:%d\n", \
		         (int)v, i, __FILE__, __LINE__);})	\
	if (PL_top_env->je_prev)				\
	    PerlProc_longjmp(PL_top_env->je_buf, (v));		\
	if ((v) == 2)						\
	    PerlProc_exit(STATUS_EXIT);		                \
	PerlIO_printf(PerlIO_stderr(), "panic: top_env, v=%d\n", (int)v); \
	PerlProc_exit(1);					\
    } STMT_END

#define CATCH_GET		(PL_top_env->je_mustcatch)
#define CATCH_SET(v) \
    STMT_START {							\
	DEBUG_l(							\
	    Perl_deb(aTHX_						\
		"JUMPLEVEL set catch %d => %d (for %p) at %s:%d\n",	\
		 PL_top_env->je_mustcatch, v, (void*)PL_top_env,	\
		 __FILE__, __LINE__);)					\
	PL_top_env->je_mustcatch = (v);					\
    } STMT_END

/*
=head1 COP Hint Hashes
*/

typedef struct refcounted_he COPHH;

#define COPHH_KEY_UTF8 REFCOUNTED_HE_KEY_UTF8

/*
=for apidoc Amx|SV *|cophh_fetch_pvn|const COPHH *cophh|const char *keypv|STRLEN keylen|U32 hash|U32 flags

Look up the entry in the cop hints hash C<cophh> with the key specified by
C<keypv> and C<keylen>.  If C<flags> has the C<COPHH_KEY_UTF8> bit set,
the key octets are interpreted as UTF-8, otherwise they are interpreted
as Latin-1.  C<hash> is a precomputed hash of the key string, or zero if
it has not been precomputed.  Returns a mortal scalar copy of the value
associated with the key, or C<&PL_sv_placeholder> if there is no value
associated with the key.

=for apidoc Amnh||COPHH_KEY_UTF8

=cut
*/

#define cophh_fetch_pvn(cophh, keypv, keylen, hash, flags) \
    Perl_refcounted_he_fetch_pvn(aTHX_ cophh, keypv, keylen, hash, flags)

/*
=for apidoc Amx|SV *|cophh_fetch_pvs|const COPHH *cophh|"key"|U32 flags

Like L</cophh_fetch_pvn>, but takes a literal string instead
of a string/length pair, and no precomputed hash.

=cut
*/

#define cophh_fetch_pvs(cophh, key, flags) \
    Perl_refcounted_he_fetch_pvn(aTHX_ cophh, STR_WITH_LEN(key), 0, flags)

/*
=for apidoc Amx|SV *|cophh_fetch_pv|const COPHH *cophh|const char *key|U32 hash|U32 flags

Like L</cophh_fetch_pvn>, but takes a nul-terminated string instead of
a string/length pair.

=cut
*/

#define cophh_fetch_pv(cophh, key, hash, flags) \
    Perl_refcounted_he_fetch_pv(aTHX_ cophh, key, hash, flags)

/*
=for apidoc Amx|SV *|cophh_fetch_sv|const COPHH *cophh|SV *key|U32 hash|U32 flags

Like L</cophh_fetch_pvn>, but takes a Perl scalar instead of a
string/length pair.

=cut
*/

#define cophh_fetch_sv(cophh, key, hash, flags) \
    Perl_refcounted_he_fetch_sv(aTHX_ cophh, key, hash, flags)

/*
=for apidoc Amx|HV *|cophh_2hv|const COPHH *cophh|U32 flags

Generates and returns a standard Perl hash representing the full set of
key/value pairs in the cop hints hash C<cophh>.  C<flags> is currently
unused and must be zero.

=cut
*/

#define cophh_2hv(cophh, flags) \
    Perl_refcounted_he_chain_2hv(aTHX_ cophh, flags)

/*
=for apidoc Amx|COPHH *|cophh_copy|COPHH *cophh

Make and return a complete copy of the cop hints hash C<cophh>.

=cut
*/

#define cophh_copy(cophh) Perl_refcounted_he_inc(aTHX_ cophh)

/*
=for apidoc Amx|void|cophh_free|COPHH *cophh

Discard the cop hints hash C<cophh>, freeing all resources associated
with it.

=cut
*/

#define cophh_free(cophh) Perl_refcounted_he_free(aTHX_ cophh)

/*
=for apidoc Amx|COPHH *|cophh_new_empty

Generate and return a fresh cop hints hash containing no entries.

=cut
*/

#define cophh_new_empty() ((COPHH *)NULL)

/*
=for apidoc Amx|COPHH *|cophh_store_pvn|COPHH *cophh|const char *keypv|STRLEN keylen|U32 hash|SV *value|U32 flags

Stores a value, associated with a key, in the cop hints hash C<cophh>,
and returns the modified hash.  The returned hash pointer is in general
not the same as the hash pointer that was passed in.  The input hash is
consumed by the function, and the pointer to it must not be subsequently
used.  Use L</cophh_copy> if you need both hashes.

The key is specified by C<keypv> and C<keylen>.  If C<flags> has the
C<COPHH_KEY_UTF8> bit set, the key octets are interpreted as UTF-8,
otherwise they are interpreted as Latin-1.  C<hash> is a precomputed
hash of the key string, or zero if it has not been precomputed.

C<value> is the scalar value to store for this key.  C<value> is copied
by this function, which thus does not take ownership of any reference
to it, and later changes to the scalar will not be reflected in the
value visible in the cop hints hash.  Complex types of scalar will not
be stored with referential integrity, but will be coerced to strings.

=cut
*/

#define cophh_store_pvn(cophh, keypv, keylen, hash, value, flags) \
    Perl_refcounted_he_new_pvn(aTHX_ cophh, keypv, keylen, hash, value, flags)

/*
=for apidoc Amx|COPHH *|cophh_store_pvs|const COPHH *cophh|"key"|SV *value|U32 flags

Like L</cophh_store_pvn>, but takes a literal string instead
of a string/length pair, and no precomputed hash.

=cut
*/

#define cophh_store_pvs(cophh, key, value, flags) \
    Perl_refcounted_he_new_pvn(aTHX_ cophh, STR_WITH_LEN(key), 0, value, flags)

/*
=for apidoc Amx|COPHH *|cophh_store_pv|const COPHH *cophh|const char *key|U32 hash|SV *value|U32 flags

Like L</cophh_store_pvn>, but takes a nul-terminated string instead of
a string/length pair.

=cut
*/

#define cophh_store_pv(cophh, key, hash, value, flags) \
    Perl_refcounted_he_new_pv(aTHX_ cophh, key, hash, value, flags)

/*
=for apidoc Amx|COPHH *|cophh_store_sv|const COPHH *cophh|SV *key|U32 hash|SV *value|U32 flags

Like L</cophh_store_pvn>, but takes a Perl scalar instead of a
string/length pair.

=cut
*/

#define cophh_store_sv(cophh, key, hash, value, flags) \
    Perl_refcounted_he_new_sv(aTHX_ cophh, key, hash, value, flags)

/*
=for apidoc Amx|COPHH *|cophh_delete_pvn|COPHH *cophh|const char *keypv|STRLEN keylen|U32 hash|U32 flags

Delete a key and its associated value from the cop hints hash C<cophh>,
and returns the modified hash.  The returned hash pointer is in general
not the same as the hash pointer that was passed in.  The input hash is
consumed by the function, and the pointer to it must not be subsequently
used.  Use L</cophh_copy> if you need both hashes.

The key is specified by C<keypv> and C<keylen>.  If C<flags> has the
C<COPHH_KEY_UTF8> bit set, the key octets are interpreted as UTF-8,
otherwise they are interpreted as Latin-1.  C<hash> is a precomputed
hash of the key string, or zero if it has not been precomputed.

=cut
*/

#define cophh_delete_pvn(cophh, keypv, keylen, hash, flags) \
    Perl_refcounted_he_new_pvn(aTHX_ cophh, keypv, keylen, hash, \
	(SV *)NULL, flags)

/*
=for apidoc Amx|COPHH *|cophh_delete_pvs|const COPHH *cophh|"key"|U32 flags

Like L</cophh_delete_pvn>, but takes a literal string instead
of a string/length pair, and no precomputed hash.

=cut
*/

#define cophh_delete_pvs(cophh, key, flags) \
    Perl_refcounted_he_new_pvn(aTHX_ cophh, STR_WITH_LEN(key), 0, \
	(SV *)NULL, flags)

/*
=for apidoc Amx|COPHH *|cophh_delete_pv|const COPHH *cophh|const char *key|U32 hash|U32 flags

Like L</cophh_delete_pvn>, but takes a nul-terminated string instead of
a string/length pair.

=cut
*/

#define cophh_delete_pv(cophh, key, hash, flags) \
    Perl_refcounted_he_new_pv(aTHX_ cophh, key, hash, (SV *)NULL, flags)

/*
=for apidoc Amx|COPHH *|cophh_delete_sv|const COPHH *cophh|SV *key|U32 hash|U32 flags

Like L</cophh_delete_pvn>, but takes a Perl scalar instead of a
string/length pair.

=cut
*/

#define cophh_delete_sv(cophh, key, hash, flags) \
    Perl_refcounted_he_new_sv(aTHX_ cophh, key, hash, (SV *)NULL, flags)

#include "mydtrace.h"

struct cop {
    BASEOP
    /* On LP64 putting this here takes advantage of the fact that BASEOP isn't
       an exact multiple of 8 bytes to save structure padding.  */
    line_t      cop_line;       /* line # of this command */
    /* label for this construct is now stored in cop_hints_hash */
#ifdef USE_ITHREADS
    PADOFFSET	cop_stashoff;	/* offset into PL_stashpad, for the
				   package the line was compiled in */
    char *	cop_file;	/* file name the following line # is from */
#else
    HV *	cop_stash;	/* package line was compiled in */
    GV *	cop_filegv;	/* file the following line # is from */
#endif
    U32		cop_hints;	/* hints bits from pragmata */
    U32		cop_seq;	/* parse sequence number */
    /* Beware. mg.c and warnings.pl assume the type of this is STRLEN *:  */
    STRLEN *	cop_warnings;	/* lexical warnings bitmask */
    /* compile time state of %^H.  See the comment in op.c for how this is
       used to recreate a hash to return from caller.  */
    COPHH *	cop_hints_hash;
    /* for now just a bitmask stored here.
       If we get sufficient features this may become a pointer.
       How these flags are stored is subject to change without
       notice.  Use the macros to test for features.
    */
    U32		cop_features;
};

#ifdef USE_ITHREADS
#  define CopFILE(c)		((c)->cop_file)
#  define CopFILEGV(c)		(CopFILE(c) \
				 ? gv_fetchfile(CopFILE(c)) : NULL)

#  ifdef NETWARE
#    define CopFILE_set(c,pv)	((c)->cop_file = savepv(pv))
#    define CopFILE_setn(c,pv,l)  ((c)->cop_file = savepvn((pv),(l)))
#  else
#    define CopFILE_set(c,pv)	((c)->cop_file = savesharedpv(pv))
#    define CopFILE_setn(c,pv,l)  ((c)->cop_file = savesharedpvn((pv),(l)))
#  endif

#  define CopFILESV(c)		(CopFILE(c) \
				 ? GvSV(gv_fetchfile(CopFILE(c))) : NULL)
#  define CopFILEAV(c)		(CopFILE(c) \
				 ? GvAV(gv_fetchfile(CopFILE(c))) : NULL)
#  define CopFILEAVx(c)		(assert_(CopFILE(c)) \
				   GvAV(gv_fetchfile(CopFILE(c))))

#  define CopSTASH(c)           PL_stashpad[(c)->cop_stashoff]
#  define CopSTASH_set(c,hv)	((c)->cop_stashoff = (hv)		\
				    ? alloccopstash(hv)			\
				    : 0)
#  ifdef NETWARE
#    define CopFILE_free(c) SAVECOPFILE_FREE(c)
#  else
#    define CopFILE_free(c)	(PerlMemShared_free(CopFILE(c)),(CopFILE(c) = NULL))
#  endif
#else
#  define CopFILEGV(c)		((c)->cop_filegv)
#  define CopFILEGV_set(c,gv)	((c)->cop_filegv = (GV*)SvREFCNT_inc(gv))
#  define CopFILE_set(c,pv)	CopFILEGV_set((c), gv_fetchfile(pv))
#  define CopFILE_setn(c,pv,l)	CopFILEGV_set((c), gv_fetchfile_flags((pv),(l),0))
#  define CopFILESV(c)		(CopFILEGV(c) ? GvSV(CopFILEGV(c)) : NULL)
#  define CopFILEAV(c)		(CopFILEGV(c) ? GvAV(CopFILEGV(c)) : NULL)
#  ifdef DEBUGGING
#    define CopFILEAVx(c)	(assert(CopFILEGV(c)), GvAV(CopFILEGV(c)))
#  else
#    define CopFILEAVx(c)	(GvAV(CopFILEGV(c)))
# endif
#  define CopFILE(c)		(CopFILEGV(c) \
				    ? GvNAME(CopFILEGV(c))+2 : NULL)
#  define CopSTASH(c)		((c)->cop_stash)
#  define CopSTASH_set(c,hv)	((c)->cop_stash = (hv))
#  define CopFILE_free(c)	(SvREFCNT_dec(CopFILEGV(c)),(CopFILEGV(c) = NULL))

#endif /* USE_ITHREADS */

#define CopSTASHPV(c)		(CopSTASH(c) ? HvNAME_get(CopSTASH(c)) : NULL)
   /* cop_stash is not refcounted */
#define CopSTASHPV_set(c,pv)	CopSTASH_set((c), gv_stashpv(pv,GV_ADD))
#define CopSTASH_eq(c,hv)	(CopSTASH(c) == (hv))

#define CopHINTHASH_get(c)	((COPHH*)((c)->cop_hints_hash))
#define CopHINTHASH_set(c,h)	((c)->cop_hints_hash = (h))

/*
=head1 COP Hint Reading
*/

/*
=for apidoc Am|SV *|cop_hints_fetch_pvn|const COP *cop|const char *keypv|STRLEN keylen|U32 hash|U32 flags

Look up the hint entry in the cop C<cop> with the key specified by
C<keypv> and C<keylen>.  If C<flags> has the C<COPHH_KEY_UTF8> bit set,
the key octets are interpreted as UTF-8, otherwise they are interpreted
as Latin-1.  C<hash> is a precomputed hash of the key string, or zero if
it has not been precomputed.  Returns a mortal scalar copy of the value
associated with the key, or C<&PL_sv_placeholder> if there is no value
associated with the key.

=cut
*/

#define cop_hints_fetch_pvn(cop, keypv, keylen, hash, flags) \
    cophh_fetch_pvn(CopHINTHASH_get(cop), keypv, keylen, hash, flags)

/*
=for apidoc Am|SV *|cop_hints_fetch_pvs|const COP *cop|"key"|U32 flags

Like L</cop_hints_fetch_pvn>, but takes a literal string
instead of a string/length pair, and no precomputed hash.

=cut
*/

#define cop_hints_fetch_pvs(cop, key, flags) \
    cophh_fetch_pvs(CopHINTHASH_get(cop), key, flags)

/*
=for apidoc Am|SV *|cop_hints_fetch_pv|const COP *cop|const char *key|U32 hash|U32 flags

Like L</cop_hints_fetch_pvn>, but takes a nul-terminated string instead
of a string/length pair.

=cut
*/

#define cop_hints_fetch_pv(cop, key, hash, flags) \
    cophh_fetch_pv(CopHINTHASH_get(cop), key, hash, flags)

/*
=for apidoc Am|SV *|cop_hints_fetch_sv|const COP *cop|SV *key|U32 hash|U32 flags

Like L</cop_hints_fetch_pvn>, but takes a Perl scalar instead of a
string/length pair.

=cut
*/

#define cop_hints_fetch_sv(cop, key, hash, flags) \
    cophh_fetch_sv(CopHINTHASH_get(cop), key, hash, flags)

/*
=for apidoc Am|HV *|cop_hints_2hv|const COP *cop|U32 flags

Generates and returns a standard Perl hash representing the full set of
hint entries in the cop C<cop>.  C<flags> is currently unused and must
be zero.

=cut
*/

#define cop_hints_2hv(cop, flags) \
    cophh_2hv(CopHINTHASH_get(cop), flags)

/*
=for apidoc Am|const char *|CopLABEL|COP *const cop

Returns the label attached to a cop.

=for apidoc Am|const char *|CopLABEL_len|COP *const cop|STRLEN *len

Returns the label attached to a cop, and stores its length in bytes into
C<*len>.

=for apidoc Am|const char *|CopLABEL_len_flags|COP *const cop|STRLEN *len|U32 *flags

Returns the label attached to a cop, and stores its length in bytes into
C<*len>.  Upon return, C<*flags> will be set to either C<SVf_UTF8> or 0.

=cut
*/

#define CopLABEL(c)  Perl_cop_fetch_label(aTHX_ (c), NULL, NULL)
#define CopLABEL_len(c,len)  Perl_cop_fetch_label(aTHX_ (c), len, NULL)
#define CopLABEL_len_flags(c,len,flags)  Perl_cop_fetch_label(aTHX_ (c), len, flags)
#define CopLABEL_alloc(pv)	((pv)?savepv(pv):NULL)

#define CopSTASH_ne(c,hv)	(!CopSTASH_eq(c,hv))
#define CopLINE(c)		((c)->cop_line)
#define CopLINE_inc(c)		(++CopLINE(c))
#define CopLINE_dec(c)		(--CopLINE(c))
#define CopLINE_set(c,l)	(CopLINE(c) = (l))

/* OutCopFILE() is CopFILE for output (caller, die, warn, etc.) */
#define OutCopFILE(c) CopFILE(c)

#define CopHINTS_get(c)		((c)->cop_hints + 0)
#define CopHINTS_set(c, h)	STMT_START {				\
				    (c)->cop_hints = (h);		\
				} STMT_END

/*
 * Here we have some enormously heavy (or at least ponderous) wizardry.
 */

/* subroutine context */
struct block_sub {
    OP *	retop;	/* op to execute on exit from sub */
    I32         old_cxsubix;  /* previous value of si_cxsubix */
    /* Above here is the same for sub, format and eval.  */
    PAD		*prevcomppad; /* the caller's PL_comppad */
    CV *	cv;
    /* Above here is the same for sub and format.  */
    I32		olddepth;
    AV  	*savearray;
};


/* format context */
struct block_format {
    OP *	retop;	/* op to execute on exit from sub */
    I32         old_cxsubix;  /* previous value of si_cxsubix */
    /* Above here is the same for sub, format and eval.  */
    PAD		*prevcomppad; /* the caller's PL_comppad */
    CV *	cv;
    /* Above here is the same for sub and format.  */
    GV *	gv;
    GV *	dfoutgv;
};

/* return a pointer to the current context */

#define CX_CUR() (&cxstack[cxstack_ix])

/* free all savestack items back to the watermark of the specified context */

#define CX_LEAVE_SCOPE(cx) LEAVE_SCOPE(cx->blk_oldsaveix)

#ifdef DEBUGGING
/* on debugging builds, poison cx afterwards so we know no code
 * uses it - because after doing cxstack_ix--, any ties, exceptions etc
 * may overwrite the current stack frame */
#  define CX_POP(cx)                                                   \
        assert(CX_CUR() == cx);                                        \
        cxstack_ix--;                                                  \
        cx = NULL;
#else
#  define CX_POP(cx) cxstack_ix--;
#endif


/* base for the next two macros. Don't use directly.
 * The context frame holds a reference to the CV so that it can't be
 * freed while we're executing it */


#define CX_PUSHSUB_GET_LVALUE_MASK(func) \
	/* If the context is indeterminate, then only the lvalue */	\
	/* flags that the caller also has are applicable.        */	\
	(								\
	   (PL_op->op_flags & OPf_WANT)					\
	       ? OPpENTERSUB_LVAL_MASK					\
	       : !(PL_op->op_private & OPpENTERSUB_LVAL_MASK)		\
	           ? 0 : (U8)func(aTHX)					\
	)

/* Restore old @_ */
#define CX_POP_SAVEARRAY(cx)						\
    STMT_START {							\
        AV *cx_pop_savearray_av = GvAV(PL_defgv);                       \
	GvAV(PL_defgv) = cx->blk_sub.savearray;				\
        cx->blk_sub.savearray = NULL;                                   \
        SvREFCNT_dec(cx_pop_savearray_av);	 			\
    } STMT_END

/* junk in @_ spells trouble when cloning CVs and in pp_caller(), so don't
 * leave any (a fast av_clear(ary), basically) */
#define CLEAR_ARGARRAY(ary) \
    STMT_START {							\
	AvMAX(ary) += AvARRAY(ary) - AvALLOC(ary);			\
	AvARRAY(ary) = AvALLOC(ary);					\
	AvFILLp(ary) = -1;						\
    } STMT_END


/* eval context */
struct block_eval {
    OP *	retop;	/* op to execute on exit from eval */
    I32         old_cxsubix;  /* previous value of si_cxsubix */
    /* Above here is the same for sub, format and eval.  */
    SV *	old_namesv;
    OP *	old_eval_root;
    SV *	cur_text;
    CV *	cv;
    JMPENV *	cur_top_env; /* value of PL_top_env when eval CX created */
};

/* If we ever need more than 512 op types, change the shift from 7.
   blku_gimme is actually also only 2 bits, so could be merged with something.
*/

/* blk_u16 bit usage for eval contexts: */

#define CxOLD_IN_EVAL(cx)	(((cx)->blk_u16) & 0x3F) /* saved PL in_eval */
#define CxEVAL_TXT_REFCNTED(cx)	(((cx)->blk_u16) & 0x40) /* cur_text rc++ */
#define CxOLD_OP_TYPE(cx)	(((cx)->blk_u16) >> 7)   /* type of eval op */

/* loop context */
struct block_loop {
    LOOP *	my_op;	/* My op, that contains redo, next and last ops.  */
    union {	/* different ways of locating the iteration variable */
	SV      **svp; /* for lexicals: address of pad slot */
	GV      *gv;   /* for package vars */
    } itervar_u;
    SV          *itersave; /* the original iteration var */
    union {
	struct { /* CXt_LOOP_ARY, C<for (@ary)>  */
	    AV *ary; /* array being iterated over */
	    IV  ix;   /* index relative to base of array */
	} ary;
	struct { /* CXt_LOOP_LIST, C<for (list)> */
	    I32 basesp; /* first element of list on stack */
	    IV  ix;      /* index relative to basesp */
	} stack;
	struct { /* CXt_LOOP_LAZYIV, C<for (1..9)> */
	    IV cur;
	    IV end;
	} lazyiv;
	struct { /* CXt_LOOP_LAZYSV C<for ('a'..'z')> */
	    SV * cur;
	    SV * end; /* maxiumum value (or minimum in reverse) */
	} lazysv;
    } state_u;
#ifdef USE_ITHREADS
    PAD *oldcomppad; /* needed to map itervar_u.svp during thread clone */
#endif
};

#define CxITERVAR(c)                                    \
        (CxPADLOOP(c)                                   \
            ? (c)->blk_loop.itervar_u.svp               \
            : ((c)->cx_type & CXp_FOR_GV)               \
                ? &GvSV((c)->blk_loop.itervar_u.gv)     \
                : (SV **)&(c)->blk_loop.itervar_u.gv)

#define CxLABEL(c)	(0 + CopLABEL((c)->blk_oldcop))
#define CxLABEL_len(c,len)	(0 + CopLABEL_len((c)->blk_oldcop, len))
#define CxLABEL_len_flags(c,len,flags)	(0 + CopLABEL_len_flags((c)->blk_oldcop, len, flags))
#define CxHASARGS(c)	(((c)->cx_type & CXp_HASARGS) == CXp_HASARGS)

/* CxLVAL(): the lval flags of the call site: the relevant flag bits from
 * the op_private field of the calling pp_entersub (or its caller's caller
 * if the caller's lvalue context isn't known):
 *  OPpLVAL_INTRO:  sub used in lvalue context, e.g. f() = 1;
 *  OPpENTERSUB_INARGS (in conjunction with OPpLVAL_INTRO): the
 *      function is being used as a sub arg or as a referent, e.g.
 *      g(...,f(),...)  or $r = \f()
 *  OPpDEREF: 2-bit mask indicating e.g. f()->[0].
 *  Note the contrast with CvLVALUE(), which is a property of the sub
 *  rather than the call site.
 */
#define CxLVAL(c)	(0 + ((c)->blk_u16 & 0xff))



/* given/when context */
struct block_givwhen {
	OP *leave_op;
        SV *defsv_save; /* the original $_ */
};



/* context common to subroutines, evals and loops */
struct block {
    U8		blku_type;	/* what kind of context this is */
    U8		blku_gimme;	/* is this block running in list context? */
    U16		blku_u16;	/* used by block_sub and block_eval (so far) */
    I32		blku_oldsaveix; /* saved PL_savestack_ix */
    /* all the fields above must be aligned with same-sized fields as sbu */
    I32		blku_oldsp;	/* current sp floor: where nextstate pops to */
    I32		blku_oldmarksp;	/* mark stack index */
    COP *	blku_oldcop;	/* old curcop pointer */
    PMOP *	blku_oldpm;	/* values of pattern match vars */
    SSize_t     blku_old_tmpsfloor;     /* saved PL_tmps_floor */
    I32		blku_oldscopesp;	/* scope stack index */

    union {
	struct block_sub	blku_sub;
	struct block_format	blku_format;
	struct block_eval	blku_eval;
	struct block_loop	blku_loop;
	struct block_givwhen	blku_givwhen;
    } blk_u;
};
#define blk_oldsp	cx_u.cx_blk.blku_oldsp
#define blk_oldcop	cx_u.cx_blk.blku_oldcop
#define blk_oldmarksp	cx_u.cx_blk.blku_oldmarksp
#define blk_oldscopesp	cx_u.cx_blk.blku_oldscopesp
#define blk_oldpm	cx_u.cx_blk.blku_oldpm
#define blk_gimme	cx_u.cx_blk.blku_gimme
#define blk_u16		cx_u.cx_blk.blku_u16
#define blk_oldsaveix   cx_u.cx_blk.blku_oldsaveix
#define blk_old_tmpsfloor cx_u.cx_blk.blku_old_tmpsfloor
#define blk_sub		cx_u.cx_blk.blk_u.blku_sub
#define blk_format	cx_u.cx_blk.blk_u.blku_format
#define blk_eval	cx_u.cx_blk.blk_u.blku_eval
#define blk_loop	cx_u.cx_blk.blk_u.blku_loop
#define blk_givwhen	cx_u.cx_blk.blk_u.blku_givwhen

#define CX_DEBUG(cx, action)						\
    DEBUG_l(								\
	Perl_deb(aTHX_ "CX %ld %s %s (scope %ld,%ld) (save %ld,%ld) at %s:%d\n",\
		    (long)cxstack_ix,					\
		    action,						\
		    PL_block_type[CxTYPE(cx)],	                        \
		    (long)PL_scopestack_ix,				\
		    (long)(cx->blk_oldscopesp),		                \
		    (long)PL_savestack_ix,				\
		    (long)(cx->blk_oldsaveix),                          \
		    __FILE__, __LINE__));



/* substitution context */
struct subst {
    U8		sbu_type;	/* same as blku_type */
    U8		sbu_rflags;
    U16		sbu_rxtainted;
    I32		sbu_oldsaveix; /* same as blku_oldsaveix */
    /* all the fields above must be aligned with same-sized fields as blk_u */
    SSize_t	sbu_iters;
    SSize_t	sbu_maxiters;
    char *	sbu_orig;
    SV *	sbu_dstr;
    SV *	sbu_targ;
    char *	sbu_s;
    char *	sbu_m;
    char *	sbu_strend;
    void *	sbu_rxres;
    REGEXP *	sbu_rx;
};
#define sb_iters	cx_u.cx_subst.sbu_iters
#define sb_maxiters	cx_u.cx_subst.sbu_maxiters
#define sb_rflags	cx_u.cx_subst.sbu_rflags
#define sb_rxtainted	cx_u.cx_subst.sbu_rxtainted
#define sb_orig		cx_u.cx_subst.sbu_orig
#define sb_dstr		cx_u.cx_subst.sbu_dstr
#define sb_targ		cx_u.cx_subst.sbu_targ
#define sb_s		cx_u.cx_subst.sbu_s
#define sb_m		cx_u.cx_subst.sbu_m
#define sb_strend	cx_u.cx_subst.sbu_strend
#define sb_rxres	cx_u.cx_subst.sbu_rxres
#define sb_rx		cx_u.cx_subst.sbu_rx

#ifdef PERL_CORE
#  define CX_PUSHSUBST(cx) CXINC, cx = CX_CUR(),		        \
	cx->blk_oldsaveix = oldsave,				        \
	cx->sb_iters		= iters,				\
	cx->sb_maxiters		= maxiters,				\
	cx->sb_rflags		= r_flags,				\
	cx->sb_rxtainted	= rxtainted,				\
	cx->sb_orig		= orig,					\
	cx->sb_dstr		= dstr,					\
	cx->sb_targ		= targ,					\
	cx->sb_s		= s,					\
	cx->sb_m		= m,					\
	cx->sb_strend		= strend,				\
	cx->sb_rxres		= NULL,					\
	cx->sb_rx		= rx,					\
	cx->cx_type		= CXt_SUBST | (once ? CXp_ONCE : 0);	\
	rxres_save(&cx->sb_rxres, rx);					\
	(void)ReREFCNT_inc(rx);						\
        SvREFCNT_inc_void_NN(targ)

#  define CX_POPSUBST(cx) \
    STMT_START {							\
        REGEXP *re;                                                     \
        assert(CxTYPE(cx) == CXt_SUBST);                                \
	rxres_free(&cx->sb_rxres);					\
	re = cx->sb_rx;                                                 \
	cx->sb_rx = NULL;                                               \
	ReREFCNT_dec(re);                                               \
        SvREFCNT_dec_NN(cx->sb_targ);                                   \
    } STMT_END
#endif

#define CxONCE(cx)		((cx)->cx_type & CXp_ONCE)

struct context {
    union {
	struct block	cx_blk;
	struct subst	cx_subst;
    } cx_u;
};
#define cx_type cx_u.cx_subst.sbu_type

/* If you re-order these, there is also an array of uppercase names in perl.h
   and a static array of context names in pp_ctl.c  */
#define CXTYPEMASK	0xf
#define CXt_NULL	0 /* currently only used for sort BLOCK */
#define CXt_WHEN	1
#define CXt_BLOCK	2
/* When micro-optimising :-) keep GIVEN next to the LOOPs, as these 5 share a
   jump table in pp_ctl.c
   The first 4 don't have a 'case' in at least one switch statement in pp_ctl.c
*/
#define CXt_GIVEN	3

/* be careful of the ordering of these five. Macros like CxTYPE_is_LOOP,
 * CxFOREACH compare ranges */
#define CXt_LOOP_ARY	4 /* for (@ary)     { ...; } */
#define CXt_LOOP_LAZYSV	5 /* for ('a'..'z') { ...; } */
#define CXt_LOOP_LAZYIV	6 /* for (1..9)     { ...; } */
#define CXt_LOOP_LIST	7 /* for (1,2,3)    { ...; } */
#define CXt_LOOP_PLAIN	8 /* while (...)    { ...; }
                             or plain block { ...; } */
#define CXt_SUB		9
#define CXt_FORMAT     10
#define CXt_EVAL       11
#define CXt_SUBST      12
/* SUBST doesn't feature in all switch statements.  */

/* private flags for CXt_SUB and CXt_FORMAT */
#define CXp_MULTICALL	0x10	/* part of a multicall (so don't tear down
                                   context on exit). (not CXt_FORMAT) */
#define CXp_HASARGS	0x20
#define CXp_SUB_RE	0x40    /* code called within regex, i.e. (?{}) */
#define CXp_SUB_RE_FAKE	0x80    /* fake sub CX for (?{}) in current scope */

/* private flags for CXt_EVAL */
#define CXp_REAL	0x20	/* truly eval'', not a lookalike */
#define CXp_TRYBLOCK	0x40	/* eval{}, not eval'' or similar */

/* private flags for CXt_LOOP */

/* this is only set in conjunction with CXp_FOR_GV */
#define CXp_FOR_DEF	0x10	/* foreach using $_ */
/* these 3 are mutually exclusive */
#define CXp_FOR_LVREF	0x20	/* foreach using \$var */
#define CXp_FOR_GV	0x40	/* foreach using package var */
#define CXp_FOR_PAD	0x80	/* foreach using lexical var */

#define CxPADLOOP(c)	((c)->cx_type & CXp_FOR_PAD)

/* private flags for CXt_SUBST */
#define CXp_ONCE	0x10	/* What was sbu_once in struct subst */

#define CxTYPE(c)	((c)->cx_type & CXTYPEMASK)
#define CxTYPE_is_LOOP(c) (   CxTYPE(cx) >= CXt_LOOP_ARY                \
                           && CxTYPE(cx) <= CXt_LOOP_PLAIN)
#define CxMULTICALL(c)	((c)->cx_type & CXp_MULTICALL)
#define CxREALEVAL(c)	(((c)->cx_type & (CXTYPEMASK|CXp_REAL))		\
			 == (CXt_EVAL|CXp_REAL))
#define CxTRYBLOCK(c)	(((c)->cx_type & (CXTYPEMASK|CXp_TRYBLOCK))	\
			 == (CXt_EVAL|CXp_TRYBLOCK))
#define CxFOREACH(c)	(   CxTYPE(cx) >= CXt_LOOP_ARY                  \
                         && CxTYPE(cx) <= CXt_LOOP_LIST)

#define CXINC (cxstack_ix < cxstack_max ? ++cxstack_ix : (cxstack_ix = cxinc()))

/*
=head1 "Gimme" Values
*/

/*
=for apidoc AmnU||G_SCALAR
Used to indicate scalar context.  See C<L</GIMME_V>>, C<L</GIMME>>, and
L<perlcall>.

=for apidoc AmnU||G_ARRAY
Used to indicate list context.  See C<L</GIMME_V>>, C<L</GIMME>> and
L<perlcall>.

=for apidoc AmnU||G_VOID
Used to indicate void context.  See C<L</GIMME_V>> and L<perlcall>.

=for apidoc AmnU||G_DISCARD
Indicates that arguments returned from a callback should be discarded.  See
L<perlcall>.

=for apidoc AmnU||G_EVAL

Used to force a Perl C<eval> wrapper around a callback.  See
L<perlcall>.

=for apidoc AmnU||G_NOARGS

Indicates that no arguments are being sent to a callback.  See
L<perlcall>.

=cut
*/

#define G_SCALAR	2
#define G_ARRAY		3
#define G_VOID		1
#define G_WANT		3

/* extra flags for Perl_call_* routines */
#define G_DISCARD         0x4	/* Call FREETMPS.
				   Don't change this without consulting the
				   hash actions codes defined in hv.h */
#define G_EVAL	          0x8	/* Assume eval {} around subroutine call. */
#define G_NOARGS         0x10	/* Don't construct a @_ array. */
#define G_KEEPERR        0x20	/* Warn for errors, don't overwrite $@ */
#define G_NODEBUG        0x40	/* Disable debugging at toplevel.  */
#define G_METHOD         0x80   /* Calling method. */
#define G_FAKINGEVAL    0x100	/* Faking an eval context for call_sv or
				   fold_constants. */
#define G_UNDEF_FILL    0x200	/* Fill the stack with &PL_sv_undef
				   A special case for UNSHIFT in
				   Perl_magic_methcall().  */
#define G_WRITING_TO_STDERR 0x400 /* Perl_write_to_stderr() is calling
				    Perl_magic_methcall().  */
#define G_RE_REPARSING  0x800   /* compiling a run-time /(?{..})/ */
#define G_METHOD_NAMED 0x1000	/* calling named method, eg without :: or ' */
#define G_RETHROW      0x2000	/* eval_sv(): re-throw any error */

/* flag bits for PL_in_eval */
#define EVAL_NULL	0	/* not in an eval */
#define EVAL_INEVAL	1	/* some enclosing scope is an eval */
#define EVAL_WARNONLY	2	/* used by yywarn() when calling yyerror() */
#define EVAL_KEEPERR	4	/* set by Perl_call_sv if G_KEEPERR */
#define EVAL_INREQUIRE	8	/* The code is being required. */
#define EVAL_RE_REPARSING 0x10	/* eval_sv() called with G_RE_REPARSING */
/* if adding extra bits, make sure they can fit in CxOLD_OP_TYPE() */

/* Support for switching (stack and block) contexts.
 * This ensures magic doesn't invalidate local stack and cx pointers.
 * Which one to use (or add) is mostly, but not completely arbitrary:  See
 * http://nntp.perl.org/group/perl.perl5.porters/257169
 */

#define PERLSI_UNKNOWN		-1
#define PERLSI_UNDEF		0
#define PERLSI_MAIN		1
#define PERLSI_MAGIC		2
#define PERLSI_SORT		3
#define PERLSI_SIGNAL		4
#define PERLSI_OVERLOAD		5
#define PERLSI_DESTROY		6
#define PERLSI_WARNHOOK		7
#define PERLSI_DIEHOOK		8
#define PERLSI_REQUIRE		9
#define PERLSI_MULTICALL       10
#define PERLSI_REGCOMP         11

struct stackinfo {
    AV *		si_stack;	/* stack for current runlevel */
    PERL_CONTEXT *	si_cxstack;	/* context stack for runlevel */
    struct stackinfo *	si_prev;
    struct stackinfo *	si_next;
    I32			si_cxix;	/* current context index */
    I32			si_cxmax;	/* maximum allocated index */
    I32			si_cxsubix;	/* topmost sub/eval/format */
    I32			si_type;	/* type of runlevel */
    I32			si_markoff;	/* offset where markstack begins for us.
					 * currently used only with DEBUGGING,
					 * but not #ifdef-ed for bincompat */
#if defined DEBUGGING && !defined DEBUGGING_RE_ONLY
/* high water mark: for checking if the stack was correctly extended /
 * tested for extension by each pp function */
    SSize_t             si_stack_hwm;
#endif

};

typedef struct stackinfo PERL_SI;

#define cxstack		(PL_curstackinfo->si_cxstack)
#define cxstack_ix	(PL_curstackinfo->si_cxix)
#define cxstack_max	(PL_curstackinfo->si_cxmax)

#ifdef DEBUGGING
#  define	SET_MARK_OFFSET \
    PL_curstackinfo->si_markoff = PL_markstack_ptr - PL_markstack
#else
#  define	SET_MARK_OFFSET NOOP
#endif

#if defined DEBUGGING && !defined DEBUGGING_RE_ONLY
#  define PUSHSTACK_INIT_HWM(si) ((si)->si_stack_hwm = 0)
#else
#  define PUSHSTACK_INIT_HWM(si) NOOP
#endif

#define PUSHSTACKi(type) \
    STMT_START {							\
	PERL_SI *next = PL_curstackinfo->si_next;			\
	DEBUG_l({							\
	    int i = 0; PERL_SI *p = PL_curstackinfo;			\
	    while (p) { i++; p = p->si_prev; }				\
	    Perl_deb(aTHX_ "push STACKINFO %d at %s:%d\n",		\
		         i, __FILE__, __LINE__);})			\
	if (!next) {							\
	    next = new_stackinfo(32, 2048/sizeof(PERL_CONTEXT) - 1);	\
	    next->si_prev = PL_curstackinfo;				\
	    PL_curstackinfo->si_next = next;				\
	}								\
	next->si_type = type;						\
	next->si_cxix = -1;						\
	next->si_cxsubix = -1;						\
        PUSHSTACK_INIT_HWM(next);                                       \
	AvFILLp(next->si_stack) = 0;					\
	SWITCHSTACK(PL_curstack,next->si_stack);			\
	PL_curstackinfo = next;						\
	SET_MARK_OFFSET;						\
    } STMT_END

#define PUSHSTACK PUSHSTACKi(PERLSI_UNKNOWN)

/* POPSTACK works with PL_stack_sp, so it may need to be bracketed by
 * PUTBACK/SPAGAIN to flush/refresh any local SP that may be active */
#define POPSTACK \
    STMT_START {							\
	dSP;								\
	PERL_SI * const prev = PL_curstackinfo->si_prev;		\
	DEBUG_l({							\
	    int i = -1; PERL_SI *p = PL_curstackinfo;			\
	    while (p) { i++; p = p->si_prev; }				\
	    Perl_deb(aTHX_ "pop  STACKINFO %d at %s:%d\n",		\
		         i, __FILE__, __LINE__);})			\
	if (!prev) {							\
	    Perl_croak_popstack();					\
	}								\
	SWITCHSTACK(PL_curstack,prev->si_stack);			\
	/* don't free prev here, free them all at the END{} */		\
	PL_curstackinfo = prev;						\
    } STMT_END

#define POPSTACK_TO(s) \
    STMT_START {							\
	while (PL_curstack != s) {					\
	    dounwind(-1);						\
	    POPSTACK;							\
	}								\
    } STMT_END

#define IN_PERL_COMPILETIME	cBOOL(PL_curcop == &PL_compiling)
#define IN_PERL_RUNTIME		cBOOL(PL_curcop != &PL_compiling)




/*
=head1 Multicall Functions

=for apidoc Amns||dMULTICALL
Declare local variables for a multicall.  See L<perlcall/LIGHTWEIGHT CALLBACKS>.

=for apidoc Ams||PUSH_MULTICALL|CV* the_cv
Opening bracket for a lightweight callback.
See L<perlcall/LIGHTWEIGHT CALLBACKS>.

=for apidoc Amns||MULTICALL
Make a lightweight callback.  See L<perlcall/LIGHTWEIGHT CALLBACKS>.

=for apidoc Amns||POP_MULTICALL
Closing bracket for a lightweight callback.
See L<perlcall/LIGHTWEIGHT CALLBACKS>.

=cut
*/

#define dMULTICALL \
    OP  *multicall_cop;							\
    bool multicall_oldcatch

#define PUSH_MULTICALL(the_cv) \
    PUSH_MULTICALL_FLAGS(the_cv, 0)

/* Like PUSH_MULTICALL, but allows you to specify extra flags
 * for the CX stack entry (this isn't part of the public API) */

#define PUSH_MULTICALL_FLAGS(the_cv, flags) \
    STMT_START {							\
        PERL_CONTEXT *cx;						\
	CV * const _nOnclAshIngNamE_ = the_cv;				\
	CV * const cv = _nOnclAshIngNamE_;				\
	PADLIST * const padlist = CvPADLIST(cv);			\
 	multicall_oldcatch = CATCH_GET;					\
	CATCH_SET(TRUE);						\
	PUSHSTACKi(PERLSI_MULTICALL);					\
	cx = cx_pushblock((CXt_SUB|CXp_MULTICALL|flags), (U8)gimme,     \
                  PL_stack_sp, PL_savestack_ix);	                \
        cx_pushsub(cx, cv, NULL, 0);                                    \
	SAVEOP();					                \
        if (!(flags & CXp_SUB_RE_FAKE))                                 \
            CvDEPTH(cv)++;						\
	if (CvDEPTH(cv) >= 2)  						\
	    Perl_pad_push(aTHX_ padlist, CvDEPTH(cv));			\
	PAD_SET_CUR_NOSAVE(padlist, CvDEPTH(cv));			\
	multicall_cop = CvSTART(cv);					\
    } STMT_END

#define MULTICALL \
    STMT_START {							\
	PL_op = multicall_cop;						\
	CALLRUNOPS(aTHX);						\
    } STMT_END

#define POP_MULTICALL \
    STMT_START {							\
        PERL_CONTEXT *cx;						\
	cx = CX_CUR();					                \
	CX_LEAVE_SCOPE(cx);                                             \
        cx_popsub_common(cx);                                           \
        gimme = cx->blk_gimme;                                          \
        PERL_UNUSED_VAR(gimme); /* for API */                           \
	cx_popblock(cx);				   		\
	CX_POP(cx);                                                     \
	POPSTACK;							\
	CATCH_SET(multicall_oldcatch);					\
	SPAGAIN;							\
    } STMT_END

/* Change the CV of an already-pushed MULTICALL CxSUB block.
 * (this isn't part of the public API) */

#define CHANGE_MULTICALL_FLAGS(the_cv, flags) \
    STMT_START {							\
	CV * const _nOnclAshIngNamE_ = the_cv;				\
	CV * const cv = _nOnclAshIngNamE_;				\
	PADLIST * const padlist = CvPADLIST(cv);			\
        PERL_CONTEXT *cx = CX_CUR();					\
	assert(CxMULTICALL(cx));                                        \
        cx_popsub_common(cx);                                           \
	cx->cx_type = (CXt_SUB|CXp_MULTICALL|flags);                    \
        cx_pushsub(cx, cv, NULL, 0);			                \
        if (!(flags & CXp_SUB_RE_FAKE))                                 \
            CvDEPTH(cv)++;						\
	if (CvDEPTH(cv) >= 2)  						\
	    Perl_pad_push(aTHX_ padlist, CvDEPTH(cv));			\
	PAD_SET_CUR_NOSAVE(padlist, CvDEPTH(cv));			\
	multicall_cop = CvSTART(cv);					\
    } STMT_END
/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
