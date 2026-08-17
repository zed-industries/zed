/*    XSUB.h
 *
 *    Copyright (C) 1994, 1995, 1996, 1997, 1998, 1999, 2000, 2001, 2002,
 *    2003, 2004, 2005, 2006, 2007, 2008 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#ifndef PERL_XSUB_H_
#define PERL_XSUB_H_ 1

/* first, some documentation for xsubpp-generated items */

/*
=head1 C<xsubpp> variables and internal functions

=for apidoc Amn|char*|CLASS
Variable which is setup by C<xsubpp> to indicate the 
class name for a C++ XS constructor.  This is always a C<char*>.  See
C<L</THIS>>.

=for apidoc Amn|(whatever)|RETVAL
Variable which is setup by C<xsubpp> to hold the return value for an 
XSUB.  This is always the proper type for the XSUB.  See 
L<perlxs/"The RETVAL Variable">.

=for apidoc Amn|(whatever)|THIS
Variable which is setup by C<xsubpp> to designate the object in a C++ 
XSUB.  This is always the proper type for the C++ object.  See C<L</CLASS>> and
L<perlxs/"Using XS With C++">.

=for apidoc Amn|I32|ax
Variable which is setup by C<xsubpp> to indicate the stack base offset,
used by the C<ST>, C<XSprePUSH> and C<XSRETURN> macros.  The C<dMARK> macro
must be called prior to setup the C<MARK> variable.

=for apidoc Amn|I32|items
Variable which is setup by C<xsubpp> to indicate the number of 
items on the stack.  See L<perlxs/"Variable-length Parameter Lists">.

=for apidoc Amn|I32|ix
Variable which is setup by C<xsubpp> to indicate which of an 
XSUB's aliases was used to invoke it.  See L<perlxs/"The ALIAS: Keyword">.

=for apidoc Am|SV*|ST|int ix
Used to access elements on the XSUB's stack.

=for apidoc AmnU||XS
Macro to declare an XSUB and its C parameter list.  This is handled by
C<xsubpp>.  It is the same as using the more explicit C<XS_EXTERNAL> macro.

=for apidoc AmU||XS_INTERNAL
Macro to declare an XSUB and its C parameter list without exporting the symbols.
This is handled by C<xsubpp> and generally preferable over exporting the XSUB
symbols unnecessarily.

=for apidoc AmnU||XS_EXTERNAL
Macro to declare an XSUB and its C parameter list explicitly exporting the symbols.

=for apidoc Amns||dAX
Sets up the C<ax> variable.
This is usually handled automatically by C<xsubpp> by calling C<dXSARGS>.

=for apidoc Amns||dAXMARK
Sets up the C<ax> variable and stack marker variable C<mark>.
This is usually handled automatically by C<xsubpp> by calling C<dXSARGS>.

=for apidoc Amns||dITEMS
Sets up the C<items> variable.
This is usually handled automatically by C<xsubpp> by calling C<dXSARGS>.

=for apidoc Amns||dXSARGS
Sets up stack and mark pointers for an XSUB, calling C<dSP> and C<dMARK>.
Sets up the C<ax> and C<items> variables by calling C<dAX> and C<dITEMS>.
This is usually handled automatically by C<xsubpp>.

=for apidoc Amns||dXSI32
Sets up the C<ix> variable for an XSUB which has aliases.  This is usually
handled automatically by C<xsubpp>.

=for apidoc Amns||dUNDERBAR
Sets up any variable needed by the C<UNDERBAR> macro.  It used to define
C<padoff_du>, but it is currently a noop.  However, it is strongly advised
to still use it for ensuring past and future compatibility.

=for apidoc AmnU||UNDERBAR
The SV* corresponding to the C<$_> variable.  Works even if there
is a lexical C<$_> in scope.

=cut
*/

#ifndef PERL_UNUSED_ARG
#  define PERL_UNUSED_ARG(x) ((void)x)
#endif
#ifndef PERL_UNUSED_VAR
#  define PERL_UNUSED_VAR(x) ((void)x)
#endif

#define ST(off) PL_stack_base[ax + (off)]

/* XSPROTO() is also used by SWIG like this:
 *
 *     typedef XSPROTO(SwigPerlWrapper);
 *     typedef SwigPerlWrapper *SwigPerlWrapperPtr;
 *
 * This code needs to be compilable under both C and C++.
 *
 * Don't forget to change the __attribute__unused__ version of XS()
 * below too if you change XSPROTO() here.
 */

/* XS_INTERNAL is the explicit static-linkage variant of the default
 * XS macro.
 *
 * XS_EXTERNAL is the same as XS_INTERNAL except it does not include
 * "STATIC", ie. it exports XSUB symbols. You probably don't want that.
 */

#define XSPROTO(name) void name(pTHX_ CV* cv __attribute__unused__)

#undef XS
#undef XS_EXTERNAL
#undef XS_INTERNAL
#if defined(__CYGWIN__) && defined(USE_DYNAMIC_LOADING)
#  define XS_EXTERNAL(name) __declspec(dllexport) XSPROTO(name)
#  define XS_INTERNAL(name) STATIC XSPROTO(name)
#elif defined(__SYMBIAN32__)
#  define XS_EXTERNAL(name) EXPORT_C XSPROTO(name)
#  define XS_INTERNAL(name) EXPORT_C STATIC XSPROTO(name)
#elif defined(__cplusplus)
#  define XS_EXTERNAL(name) extern "C" XSPROTO(name)
#  define XS_INTERNAL(name) static XSPROTO(name)
#elif defined(HASATTRIBUTE_UNUSED)
#  define XS_EXTERNAL(name) void name(pTHX_ CV* cv __attribute__unused__)
#  define XS_INTERNAL(name) STATIC void name(pTHX_ CV* cv __attribute__unused__)
#else
#  define XS_EXTERNAL(name) XSPROTO(name)
#  define XS_INTERNAL(name) STATIC XSPROTO(name)
#endif

/* We do export xsub symbols by default for the public XS macro.
 * Try explicitly using XS_INTERNAL/XS_EXTERNAL instead, please. */
#define XS(name) XS_EXTERNAL(name)

#define dAX const I32 ax = (I32)(MARK - PL_stack_base + 1)

#define dAXMARK				\
	I32 ax = POPMARK;	\
	SV **mark = PL_stack_base + ax++

#define dITEMS I32 items = (I32)(SP - MARK)

#define dXSARGS \
	dSP; dAXMARK; dITEMS
/* These 3 macros are replacements for dXSARGS macro only in bootstrap.
   They factor out common code in every BOOT XSUB. Computation of vars mark
   and items will optimize away in most BOOT functions. Var ax can never be
   optimized away since BOOT must return &PL_sv_yes by default from xsubpp.
   Note these macros are not drop in replacements for dXSARGS since they set
   PL_xsubfilename. */
#define dXSBOOTARGSXSAPIVERCHK  \
	I32 ax = XS_BOTHVERSION_SETXSUBFN_POPMARK_BOOTCHECK;	\
	SV **mark = PL_stack_base + ax; dSP; dITEMS
#define dXSBOOTARGSAPIVERCHK  \
	I32 ax = XS_APIVERSION_SETXSUBFN_POPMARK_BOOTCHECK;	\
	SV **mark = PL_stack_base + ax; dSP; dITEMS
/* dXSBOOTARGSNOVERCHK has no API in xsubpp to choose it so do
#undef dXSBOOTARGSXSAPIVERCHK
#define dXSBOOTARGSXSAPIVERCHK dXSBOOTARGSNOVERCHK */
#define dXSBOOTARGSNOVERCHK  \
	I32 ax = XS_SETXSUBFN_POPMARK;  \
	SV **mark = PL_stack_base + ax; dSP; dITEMS

#define dXSTARG SV * const targ = ((PL_op->op_private & OPpENTERSUB_HASTARG) \
			     ? PAD_SV(PL_op->op_targ) : sv_newmortal())

/* Should be used before final PUSHi etc. if not in PPCODE section. */
#define XSprePUSH (sp = PL_stack_base + ax - 1)

#define XSANY CvXSUBANY(cv)

#define dXSI32 I32 ix = XSANY.any_i32

#ifdef __cplusplus
#  define XSINTERFACE_CVT(ret,name) ret (*name)(...)
#  define XSINTERFACE_CVT_ANON(ret) ret (*)(...)
#else
#  define XSINTERFACE_CVT(ret,name) ret (*name)()
#  define XSINTERFACE_CVT_ANON(ret) ret (*)()
#endif
#define dXSFUNCTION(ret)		XSINTERFACE_CVT(ret,XSFUNCTION)
#define XSINTERFACE_FUNC(ret,cv,f)     ((XSINTERFACE_CVT_ANON(ret))(f))
#define XSINTERFACE_FUNC_SET(cv,f)	\
		CvXSUBANY(cv).any_dxptr = (void (*) (pTHX_ void*))(f)

#define dUNDERBAR dNOOP
#define UNDERBAR  find_rundefsv()

/* Simple macros to put new mortal values onto the stack.   */
/* Typically used to return values from XS functions.       */

/*
=head1 Stack Manipulation Macros

=for apidoc Am|void|XST_mIV|int pos|IV iv
Place an integer into the specified position C<pos> on the stack.  The
value is stored in a new mortal SV.

=for apidoc Am|void|XST_mNV|int pos|NV nv
Place a double into the specified position C<pos> on the stack.  The value
is stored in a new mortal SV.

=for apidoc Am|void|XST_mPV|int pos|char* str
Place a copy of a string into the specified position C<pos> on the stack. 
The value is stored in a new mortal SV.

=for apidoc Am|void|XST_mUV|int pos|UV uv
Place an unsigned integer into the specified position C<pos> on the stack.  The
value is stored in a new mortal SV.

=for apidoc Am|void|XST_mNO|int pos
Place C<&PL_sv_no> into the specified position C<pos> on the
stack.

=for apidoc Am|void|XST_mYES|int pos
Place C<&PL_sv_yes> into the specified position C<pos> on the
stack.

=for apidoc Am|void|XST_mUNDEF|int pos
Place C<&PL_sv_undef> into the specified position C<pos> on the
stack.

=for apidoc Am|void|XSRETURN|int nitems
Return from XSUB, indicating number of items on the stack.  This is usually
handled by C<xsubpp>.

=for apidoc Am|void|XSRETURN_IV|IV iv
Return an integer from an XSUB immediately.  Uses C<XST_mIV>.

=for apidoc Am|void|XSRETURN_UV|IV uv
Return an integer from an XSUB immediately.  Uses C<XST_mUV>.

=for apidoc Am|void|XSRETURN_NV|NV nv
Return a double from an XSUB immediately.  Uses C<XST_mNV>.

=for apidoc Am|void|XSRETURN_PV|char* str
Return a copy of a string from an XSUB immediately.  Uses C<XST_mPV>.

=for apidoc Amns||XSRETURN_NO
Return C<&PL_sv_no> from an XSUB immediately.  Uses C<XST_mNO>.

=for apidoc Amns||XSRETURN_YES
Return C<&PL_sv_yes> from an XSUB immediately.  Uses C<XST_mYES>.

=for apidoc Amns||XSRETURN_UNDEF
Return C<&PL_sv_undef> from an XSUB immediately.  Uses C<XST_mUNDEF>.

=for apidoc Amns||XSRETURN_EMPTY
Return an empty list from an XSUB immediately.

=head1 Variables created by C<xsubpp> and C<xsubpp> internal functions

=for apidoc AmU||newXSproto|char* name|XSUBADDR_t f|char* filename|const char *proto
Used by C<xsubpp> to hook up XSUBs as Perl subs.  Adds Perl prototypes to
the subs.

=for apidoc AmnU||XS_VERSION
The version identifier for an XS module.  This is usually
handled automatically by C<ExtUtils::MakeMaker>.  See
C<L</XS_VERSION_BOOTCHECK>>.

=for apidoc Amns||XS_VERSION_BOOTCHECK
Macro to verify that a PM module's C<$VERSION> variable matches the XS
module's C<XS_VERSION> variable.  This is usually handled automatically by
C<xsubpp>.  See L<perlxs/"The VERSIONCHECK: Keyword">.

=for apidoc Amns||XS_APIVERSION_BOOTCHECK
Macro to verify that the perl api version an XS module has been compiled against
matches the api version of the perl interpreter it's being loaded into.

=head1 Exception Handling (simple) Macros

=for apidoc Amns||dXCPT
Set up necessary local variables for exception handling.
See L<perlguts/"Exception Handling">.

=for apidoc AmnU||XCPT_TRY_START
Starts a try block.  See L<perlguts/"Exception Handling">.

=for apidoc AmnU||XCPT_TRY_END
Ends a try block.  See L<perlguts/"Exception Handling">.

=for apidoc AmnU||XCPT_CATCH
Introduces a catch block.  See L<perlguts/"Exception Handling">.

=for apidoc Amns||XCPT_RETHROW
Rethrows a previously caught exception.  See L<perlguts/"Exception Handling">.

=cut
*/

#define XST_mIV(i,v)  (ST(i) = sv_2mortal(newSViv(v))  )
#define XST_mUV(i,v)  (ST(i) = sv_2mortal(newSVuv(v))  )
#define XST_mNV(i,v)  (ST(i) = sv_2mortal(newSVnv(v))  )
#define XST_mPV(i,v)  (ST(i) = sv_2mortal(newSVpv(v,0)))
#define XST_mPVN(i,v,n)  (ST(i) = newSVpvn_flags(v,n, SVs_TEMP))
#define XST_mNO(i)    (ST(i) = &PL_sv_no   )
#define XST_mYES(i)   (ST(i) = &PL_sv_yes  )
#define XST_mUNDEF(i) (ST(i) = &PL_sv_undef)

#define XSRETURN(off)					\
    STMT_START {					\
	const IV tmpXSoff = (off);			\
	assert(tmpXSoff >= 0);\
	PL_stack_sp = PL_stack_base + ax + (tmpXSoff - 1);	\
	return;						\
    } STMT_END

#define XSRETURN_IV(v) STMT_START { XST_mIV(0,v);  XSRETURN(1); } STMT_END
#define XSRETURN_UV(v) STMT_START { XST_mUV(0,v);  XSRETURN(1); } STMT_END
#define XSRETURN_NV(v) STMT_START { XST_mNV(0,v);  XSRETURN(1); } STMT_END
#define XSRETURN_PV(v) STMT_START { XST_mPV(0,v);  XSRETURN(1); } STMT_END
#define XSRETURN_PVN(v,n) STMT_START { XST_mPVN(0,v,n);  XSRETURN(1); } STMT_END
#define XSRETURN_NO    STMT_START { XST_mNO(0);    XSRETURN(1); } STMT_END
#define XSRETURN_YES   STMT_START { XST_mYES(0);   XSRETURN(1); } STMT_END
#define XSRETURN_UNDEF STMT_START { XST_mUNDEF(0); XSRETURN(1); } STMT_END
#define XSRETURN_EMPTY STMT_START {                XSRETURN(0); } STMT_END

#define newXSproto(a,b,c,d)	newXS_flags(a,b,c,d,0)

#ifdef XS_VERSION
#  define XS_VERSION_BOOTCHECK						\
    Perl_xs_handshake(HS_KEY(FALSE, FALSE, "", XS_VERSION), HS_CXT, __FILE__,	\
        items, ax, XS_VERSION)
#else
#  define XS_VERSION_BOOTCHECK
#endif

#define XS_APIVERSION_BOOTCHECK						\
    Perl_xs_handshake(HS_KEY(FALSE, FALSE, "v" PERL_API_VERSION_STRING, ""),	\
        HS_CXT, __FILE__, items, ax, "v" PERL_API_VERSION_STRING)
/* public API, this is a combination of XS_VERSION_BOOTCHECK and
   XS_APIVERSION_BOOTCHECK in 1, and is backportable */
#ifdef XS_VERSION
#  define XS_BOTHVERSION_BOOTCHECK						\
    Perl_xs_handshake(HS_KEY(FALSE, FALSE, "v" PERL_API_VERSION_STRING, XS_VERSION),	\
        HS_CXT, __FILE__, items, ax, "v" PERL_API_VERSION_STRING, XS_VERSION)
#else
/* should this be a #error? if you want both checked, you better supply XS_VERSION right? */
#  define XS_BOTHVERSION_BOOTCHECK XS_APIVERSION_BOOTCHECK
#endif

/* private API */
#define XS_APIVERSION_POPMARK_BOOTCHECK					\
    Perl_xs_handshake(HS_KEY(FALSE, TRUE, "v" PERL_API_VERSION_STRING, ""),	\
        HS_CXT, __FILE__, "v" PERL_API_VERSION_STRING)
#ifdef XS_VERSION
#  define XS_BOTHVERSION_POPMARK_BOOTCHECK					\
    Perl_xs_handshake(HS_KEY(FALSE, TRUE, "v" PERL_API_VERSION_STRING, XS_VERSION),	\
        HS_CXT, __FILE__, "v" PERL_API_VERSION_STRING, XS_VERSION)
#else
/* should this be a #error? if you want both checked, you better supply XS_VERSION right? */
#  define XS_BOTHVERSION_POPMARK_BOOTCHECK XS_APIVERSION_POPMARK_BOOTCHECK
#endif

#define XS_APIVERSION_SETXSUBFN_POPMARK_BOOTCHECK				\
    Perl_xs_handshake(HS_KEY(TRUE, TRUE, "v" PERL_API_VERSION_STRING, ""),	\
        HS_CXT, __FILE__, "v" PERL_API_VERSION_STRING)
#ifdef XS_VERSION
#  define XS_BOTHVERSION_SETXSUBFN_POPMARK_BOOTCHECK				  \
    Perl_xs_handshake(HS_KEY(TRUE, TRUE, "v" PERL_API_VERSION_STRING, XS_VERSION),\
        HS_CXT, __FILE__, "v" PERL_API_VERSION_STRING, XS_VERSION)
#else
/* should this be a #error? if you want both checked, you better supply XS_VERSION right? */
#  define XS_BOTHVERSION_SETXSUBFN_POPMARK_BOOTCHECK XS_APIVERSION_SETXSUBFN_POPMARK_BOOTCHECK
#endif

/* For a normal bootstrap without API or XS version checking.
   Useful for static XS modules or debugging/testing scenarios.
   If this macro gets heavily used in the future, it should separated into
   a separate function independent of Perl_xs_handshake for efficiency */
#define XS_SETXSUBFN_POPMARK \
    Perl_xs_handshake(HS_KEY(TRUE, TRUE, "", "") | HSf_NOCHK, HS_CXT, __FILE__)

#ifdef NO_XSLOCKS
#  define dXCPT             dJMPENV; int rEtV = 0
#  define XCPT_TRY_START    JMPENV_PUSH(rEtV); if (rEtV == 0)
#  define XCPT_TRY_END      JMPENV_POP;
#  define XCPT_CATCH        if (rEtV != 0)
#  define XCPT_RETHROW      JMPENV_JUMP(rEtV)
#endif

/*
   The DBM_setFilter & DBM_ckFilter macros are only used by
   the *DB*_File modules
*/

#define DBM_setFilter(db_type,code)				\
	STMT_START {						\
	    if (db_type)					\
	        RETVAL = sv_mortalcopy(db_type) ;		\
	    ST(0) = RETVAL ;					\
	    if (db_type && (code == &PL_sv_undef)) {		\
	        SvREFCNT_dec_NN(db_type) ;			\
	        db_type = NULL ;				\
	    }							\
	    else if (code) {					\
	        if (db_type)					\
	            sv_setsv(db_type, code) ;			\
	        else						\
	            db_type = newSVsv(code) ;			\
	    }	    						\
	} STMT_END

#define DBM_ckFilter(arg,type,name)				\
        STMT_START {						\
	if (db->type) {						\
	    if (db->filtering) {				\
	        croak("recursion detected in %s", name) ;	\
	    }                     				\
	    ENTER ;						\
	    SAVETMPS ;						\
	    SAVEINT(db->filtering) ;				\
	    db->filtering = TRUE ;				\
	    SAVE_DEFSV ;					\
            if (name[7] == 's')                                 \
                arg = newSVsv(arg);                             \
	    DEFSV_set(arg) ;					\
	    SvTEMP_off(arg) ;					\
	    PUSHMARK(SP) ;					\
	    PUTBACK ;						\
	    (void) perl_call_sv(db->type, G_DISCARD); 		\
	    SPAGAIN ;						\
	    PUTBACK ;						\
	    FREETMPS ;						\
	    LEAVE ;						\
            if (name[7] == 's'){                                \
                arg = sv_2mortal(arg);                          \
            }                                                   \
	} } STMT_END                                                     

#if 1		/* for compatibility */
#  define VTBL_sv		&PL_vtbl_sv
#  define VTBL_env		&PL_vtbl_env
#  define VTBL_envelem		&PL_vtbl_envelem
#  define VTBL_sigelem		&PL_vtbl_sigelem
#  define VTBL_pack		&PL_vtbl_pack
#  define VTBL_packelem		&PL_vtbl_packelem
#  define VTBL_dbline		&PL_vtbl_dbline
#  define VTBL_isa		&PL_vtbl_isa
#  define VTBL_isaelem		&PL_vtbl_isaelem
#  define VTBL_arylen		&PL_vtbl_arylen
#  define VTBL_glob		&PL_vtbl_glob
#  define VTBL_mglob		&PL_vtbl_mglob
#  define VTBL_nkeys		&PL_vtbl_nkeys
#  define VTBL_taint		&PL_vtbl_taint
#  define VTBL_substr		&PL_vtbl_substr
#  define VTBL_vec		&PL_vtbl_vec
#  define VTBL_pos		&PL_vtbl_pos
#  define VTBL_bm		&PL_vtbl_bm
#  define VTBL_fm		&PL_vtbl_fm
#  define VTBL_uvar		&PL_vtbl_uvar
#  define VTBL_defelem		&PL_vtbl_defelem
#  define VTBL_regexp		&PL_vtbl_regexp
#  define VTBL_regdata		&PL_vtbl_regdata
#  define VTBL_regdatum		&PL_vtbl_regdatum
#  ifdef USE_LOCALE_COLLATE
#    define VTBL_collxfrm	&PL_vtbl_collxfrm
#  endif
#  define VTBL_amagic		&PL_vtbl_amagic
#  define VTBL_amagicelem	&PL_vtbl_amagicelem
#endif

#include "perlapi.h"

#if defined(PERL_IMPLICIT_CONTEXT) && !defined(PERL_NO_GET_CONTEXT) && !defined(PERL_CORE)
#  undef aTHX
#  undef aTHX_
#  define aTHX		PERL_GET_THX
#  define aTHX_		aTHX,
#endif

#if defined(PERL_IMPLICIT_SYS) && !defined(PERL_CORE)
#  ifndef NO_XSLOCKS
# if defined (NETWARE) && defined (USE_STDIO)
#    define times		PerlProc_times
#    define setuid		PerlProc_setuid
#    define setgid		PerlProc_setgid
#    define getpid		PerlProc_getpid
#    define pause		PerlProc_pause
#    define exit		PerlProc_exit
#    define _exit		PerlProc__exit
# else
#    undef closedir
#    undef opendir
#    undef stdin
#    undef stdout
#    undef stderr
#    undef feof
#    undef ferror
#    undef fgetpos
#    undef ioctl
#    undef getlogin
#    undef setjmp
#    undef getc
#    undef ungetc
#    undef fileno

/* Following symbols were giving redefinition errors while building extensions - sgp 17th Oct 2000 */
#ifdef NETWARE
#	undef readdir
#	undef fstat
#	undef stat
#	undef longjmp
#	undef endhostent
#	undef endnetent
#	undef endprotoent
#	undef endservent
#	undef gethostbyaddr
#	undef gethostbyname
#	undef gethostent
#	undef getnetbyaddr
#	undef getnetbyname
#	undef getnetent
#	undef getprotobyname
#	undef getprotobynumber
#	undef getprotoent
#	undef getservbyname
#	undef getservbyport
#	undef getservent
#	undef inet_ntoa
#	undef sethostent
#	undef setnetent
#	undef setprotoent
#	undef setservent
#endif	/* NETWARE */

/* to avoid warnings: "xyz" redefined */
#ifdef WIN32
#    undef  popen
#    undef  pclose
#endif /* WIN32 */

#    undef  socketpair

#    define mkdir		PerlDir_mkdir
#    define chdir		PerlDir_chdir
#    define rmdir		PerlDir_rmdir
#    define closedir		PerlDir_close
#    define opendir		PerlDir_open
#    define readdir		PerlDir_read
#    define rewinddir		PerlDir_rewind
#    define seekdir		PerlDir_seek
#    define telldir		PerlDir_tell
#    define putenv		PerlEnv_putenv
#    define getenv		PerlEnv_getenv
#    define uname		PerlEnv_uname
#    define stdin		PerlSIO_stdin
#    define stdout		PerlSIO_stdout
#    define stderr		PerlSIO_stderr
#    define fopen		PerlSIO_fopen
#    define fclose		PerlSIO_fclose
#    define feof		PerlSIO_feof
#    define ferror		PerlSIO_ferror
#    define clearerr		PerlSIO_clearerr
#    define getc		PerlSIO_getc
#    define fgets		PerlSIO_fgets
#    define fputc		PerlSIO_fputc
#    define fputs		PerlSIO_fputs
#    define fflush		PerlSIO_fflush
#    define ungetc		PerlSIO_ungetc
#    define fileno		PerlSIO_fileno
#    define fdopen		PerlSIO_fdopen
#    define freopen		PerlSIO_freopen
#    define fread		PerlSIO_fread
#    define fwrite		PerlSIO_fwrite
#    define setbuf		PerlSIO_setbuf
#    define setvbuf		PerlSIO_setvbuf
#    define setlinebuf		PerlSIO_setlinebuf
#    define stdoutf		PerlSIO_stdoutf
#    define vfprintf		PerlSIO_vprintf
#    define ftell		PerlSIO_ftell
#    define fseek		PerlSIO_fseek
#    define fgetpos		PerlSIO_fgetpos
#    define fsetpos		PerlSIO_fsetpos
#    define frewind		PerlSIO_rewind
#    define tmpfile		PerlSIO_tmpfile
#    define access		PerlLIO_access
#    define chmod		PerlLIO_chmod
#    define chsize		PerlLIO_chsize
#    define close		PerlLIO_close
#    define dup			PerlLIO_dup
#    define dup2		PerlLIO_dup2
#    define flock		PerlLIO_flock
#    define fstat		PerlLIO_fstat
#    define ioctl		PerlLIO_ioctl
#    define isatty		PerlLIO_isatty
#    define link                PerlLIO_link
#    define lseek		PerlLIO_lseek
#    define lstat		PerlLIO_lstat
#    define mktemp		PerlLIO_mktemp
#    define open		PerlLIO_open
#    define read		PerlLIO_read
#    define rename		PerlLIO_rename
#    define setmode		PerlLIO_setmode
#    define stat(buf,sb)	PerlLIO_stat(buf,sb)
#    define tmpnam		PerlLIO_tmpnam
#    define umask		PerlLIO_umask
#    define unlink		PerlLIO_unlink
#    define utime		PerlLIO_utime
#    define write		PerlLIO_write
#    define malloc		PerlMem_malloc
#    define calloc              PerlMem_calloc
#    define realloc		PerlMem_realloc
#    define free		PerlMem_free
#    define abort		PerlProc_abort
#    define exit		PerlProc_exit
#    define _exit		PerlProc__exit
#    define execl		PerlProc_execl
#    define execv		PerlProc_execv
#    define execvp		PerlProc_execvp
#    define getuid		PerlProc_getuid
#    define geteuid		PerlProc_geteuid
#    define getgid		PerlProc_getgid
#    define getegid		PerlProc_getegid
#    define getlogin		PerlProc_getlogin
#    define kill		PerlProc_kill
#    define killpg		PerlProc_killpg
#    define pause		PerlProc_pause
#    define popen		PerlProc_popen
#    define pclose		PerlProc_pclose
#    define pipe		PerlProc_pipe
#    define setuid		PerlProc_setuid
#    define setgid		PerlProc_setgid
#    define sleep		PerlProc_sleep
#    define times		PerlProc_times
#    define wait		PerlProc_wait
#    define setjmp		PerlProc_setjmp
#    define longjmp		PerlProc_longjmp
#    define signal		PerlProc_signal
#    define getpid		PerlProc_getpid
#    define gettimeofday	PerlProc_gettimeofday
#    define htonl		PerlSock_htonl
#    define htons		PerlSock_htons
#    define ntohl		PerlSock_ntohl
#    define ntohs		PerlSock_ntohs
#    define accept		PerlSock_accept
#    define bind		PerlSock_bind
#    define connect		PerlSock_connect
#    define endhostent		PerlSock_endhostent
#    define endnetent		PerlSock_endnetent
#    define endprotoent		PerlSock_endprotoent
#    define endservent		PerlSock_endservent
#    define gethostbyaddr	PerlSock_gethostbyaddr
#    define gethostbyname	PerlSock_gethostbyname
#    define gethostent		PerlSock_gethostent
#    define gethostname		PerlSock_gethostname
#    define getnetbyaddr	PerlSock_getnetbyaddr
#    define getnetbyname	PerlSock_getnetbyname
#    define getnetent		PerlSock_getnetent
#    define getpeername		PerlSock_getpeername
#    define getprotobyname	PerlSock_getprotobyname
#    define getprotobynumber	PerlSock_getprotobynumber
#    define getprotoent		PerlSock_getprotoent
#    define getservbyname	PerlSock_getservbyname
#    define getservbyport	PerlSock_getservbyport
#    define getservent		PerlSock_getservent
#    define getsockname		PerlSock_getsockname
#    define getsockopt		PerlSock_getsockopt
#    define inet_addr		PerlSock_inet_addr
#    define inet_ntoa		PerlSock_inet_ntoa
#    define listen		PerlSock_listen
#    define recv		PerlSock_recv
#    define recvfrom		PerlSock_recvfrom
#    define select		PerlSock_select
#    define send		PerlSock_send
#    define sendto		PerlSock_sendto
#    define sethostent		PerlSock_sethostent
#    define setnetent		PerlSock_setnetent
#    define setprotoent		PerlSock_setprotoent
#    define setservent		PerlSock_setservent
#    define setsockopt		PerlSock_setsockopt
#    define shutdown		PerlSock_shutdown
#    define socket		PerlSock_socket
#    define socketpair		PerlSock_socketpair
#	endif	/* NETWARE && USE_STDIO */

#    undef fd_set
#    undef FD_SET
#    undef FD_CLR
#    undef FD_ISSET
#    undef FD_ZERO
#    define fd_set		Perl_fd_set
#    define FD_SET(n,p)		PERL_FD_SET(n,p)
#    define FD_CLR(n,p)		PERL_FD_CLR(n,p)
#    define FD_ISSET(n,p)	PERL_FD_ISSET(n,p)
#    define FD_ZERO(p)		PERL_FD_ZERO(p)

#  endif  /* NO_XSLOCKS */
#endif  /* PERL_IMPLICIT_SYS && !PERL_CORE */

#endif /* PERL_XSUB_H_ */		/* include guard */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
