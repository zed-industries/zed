/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */
/*
 * if_perl.xs: Main code for Perl interface support.
 *		Mostly written by Sven Verdoolaege.
 */

#define _memory_h	/* avoid memset redeclaration */
#define IN_PERL_FILE	/* don't include if_perl.pro from proto.h */

/*
 * Currently 32-bit version of ActivePerl is built with VC6 (or MinGW since
 * ActivePerl 5.18).
 * (http://community.activestate.com/faq/windows-compilers-perl-modules)
 * It means that time_t should be 32-bit. However the default size of
 * time_t is 64-bit since VC8. So we have to define _USE_32BIT_TIME_T.
 */
#if defined(WIN32) && !defined(_WIN64)
# define _USE_32BIT_TIME_T
#endif

#include "vim.h"

#ifdef _MSC_VER
// Work around for using MSVC and ActivePerl 5.18.
# define __inline__ __inline
// Work around for using MSVC and Strawberry Perl 5.30.
# define __builtin_expect(expr, val) (expr)
// Work around for using MSVC and Strawberry Perl 5.32.
# define NO_THREAD_SAFE_LOCALE
#endif

#if defined(MSWIN) && defined(DYNAMIC_PERL)
// Work around for warning C4273 (inconsistent DLL linkage).
# define PERL_EXT_RE_BUILD
#endif

#ifdef __GNUC__
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-variable"
#endif

#include <EXTERN.h>
#include <perl.h>
#include <XSUB.h>
#if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
# include <perliol.h>
#endif

/* Workaround for perl < 5.8.7 */
#ifndef PERLIO_FUNCS_DECL
# ifdef PERLIO_FUNCS_CONST
#  define PERLIO_FUNCS_DECL(funcs) const PerlIO_funcs funcs
#  define PERLIO_FUNCS_CAST(funcs) (PerlIO_funcs*)(funcs)
# else
#  define PERLIO_FUNCS_DECL(funcs) PerlIO_funcs funcs
#  define PERLIO_FUNCS_CAST(funcs) (funcs)
# endif
#endif
#ifndef SvREFCNT_inc_void_NN
# define SvREFCNT_inc_void_NN SvREFCNT_inc
#endif

/*
 * Work around clashes between Perl and Vim namespace.	proto.h doesn't
 * include if_perl.pro and perlsfio.pro when IN_PERL_FILE is defined, because
 * we need the CV typedef.  proto.h can't be moved to after including
 * if_perl.h, because we get all sorts of name clashes then.
 */
#ifndef PROTO
# ifndef __MINGW32__
#  include "proto/if_perl.pro"
#  include "proto/if_perlsfio.pro"
# endif
#endif

// Perl compatibility stuff. This should ensure compatibility with older
// versions of Perl.
#ifndef PERL_VERSION
# include <patchlevel.h>
# define PERL_REVISION   5
# define PERL_VERSION    PATCHLEVEL
# define PERL_SUBVERSION SUBVERSION
#endif


// Work around for ActivePerl 5.20.3+: Avoid generating (g)vim.lib.
#if defined(ACTIVEPERL_VERSION) && (ACTIVEPERL_VERSION >= 2003) \
	&& defined(MSWIN) && defined(USE_DYNAMIC_LOADING)
# undef XS_EXTERNAL
# define XS_EXTERNAL(name) XSPROTO(name)
#endif

/*
 * Quoting Jan Dubois of Active State:
 *    ActivePerl build 822 still identifies itself as 5.8.8 but already
 *    contains many of the changes from the upcoming Perl 5.8.9 release.
 *
 * The changes include addition of two symbols (Perl_sv_2iv_flags,
 * Perl_newXS_flags) not present in earlier releases.
 *
 * Jan Dubois suggested the following guarding scheme.
 *
 * Active State defined ACTIVEPERL_VERSION as a string in versions before
 * 5.8.8; and so the comparison to 822 below needs to be guarded.
 */
#if (PERL_REVISION == 5) && (PERL_VERSION == 8) && (PERL_SUBVERSION >= 8)
# if (ACTIVEPERL_VERSION >= 822) || (PERL_SUBVERSION >= 9)
#  define PERL589_OR_LATER
# endif
#endif
#if (PERL_REVISION == 5) && (PERL_VERSION >= 9)
# define PERL589_OR_LATER
#endif

#if (PERL_REVISION == 5) && ((PERL_VERSION > 10) || \
    (PERL_VERSION == 10) && (PERL_SUBVERSION >= 1))
# define PERL5101_OR_LATER
#endif

#ifndef pTHX
# define pTHX void
# define pTHX_
#endif

#ifndef EXTERN_C
# define EXTERN_C
#endif

// Suppress Infinite warnings when compiling XS modules under macOS 12 Monterey.
#if defined(__clang__) && defined(__clang_major__) && __clang_major__ > 11
# pragma clang diagnostic ignored "-Wcompound-token-split-by-macro"
#endif

/* Compatibility hacks over */

static PerlInterpreter *perl_interp = NULL;
static void xs_init(pTHX);
static void VIM_init(void);
EXTERN_C void boot_DynaLoader(pTHX_ CV*);

/*
 * For dynamic linked perl.
 */
#if defined(DYNAMIC_PERL) || defined(PROTO)

# ifndef DYNAMIC_PERL /* just generating prototypes */
#  ifdef MSWIN
typedef int HANDLE;
#  endif
typedef int XSINIT_t;
typedef int XSUBADDR_t;
# endif
# ifndef USE_ITHREADS
typedef int perl_key;
# endif

# ifdef MSWIN
#  define PERL_PROC FARPROC
#  define load_dll vimLoadLib
#  define symbol_from_dll GetProcAddress
#  define close_dll FreeLibrary
#  define load_dll_error GetWin32Error
# else
#  include <dlfcn.h>
#  define HANDLE void*
#  define PERL_PROC void*
#  define load_dll(n) dlopen((n), RTLD_LAZY|RTLD_GLOBAL)
#  define symbol_from_dll dlsym
#  define close_dll dlclose
#  define load_dll_error dlerror
# endif
/*
 * Wrapper defines
 */
# define perl_alloc dll_perl_alloc
# define perl_construct dll_perl_construct
# define perl_parse dll_perl_parse
# define perl_run dll_perl_run
# define perl_destruct dll_perl_destruct
# define perl_free dll_perl_free
# if defined(WIN32) || ((PERL_REVISION == 5) && (PERL_VERSION < 38))
#  define Perl_get_context dll_Perl_get_context
# endif
# define Perl_croak dll_Perl_croak
# ifdef PERL5101_OR_LATER
#  define Perl_croak_xs_usage dll_Perl_croak_xs_usage
# endif
# ifndef PROTO
#  ifdef PERL_IMPLICIT_CONTEXT
#   define Perl_croak_nocontext dll_Perl_croak_nocontext
#  endif
#  define Perl_call_argv dll_Perl_call_argv
#  define Perl_call_pv dll_Perl_call_pv
#  define Perl_eval_sv dll_Perl_eval_sv
#  define Perl_get_sv dll_Perl_get_sv
#  define Perl_eval_pv dll_Perl_eval_pv
#  define Perl_call_method dll_Perl_call_method
# endif
# define Perl_dowantarray dll_Perl_dowantarray
# define Perl_free_tmps dll_Perl_free_tmps
# define Perl_gv_stashpv dll_Perl_gv_stashpv
# define Perl_markstack_grow dll_Perl_markstack_grow
# define Perl_mg_find dll_Perl_mg_find
# if (PERL_REVISION == 5) && (PERL_VERSION >= 28)
#  define Perl_mg_get dll_Perl_mg_get
# endif
# define Perl_newXS dll_Perl_newXS
# define Perl_newSV dll_Perl_newSV
# define Perl_newSViv dll_Perl_newSViv
# define Perl_newSVpv dll_Perl_newSVpv
# define Perl_pop_scope dll_Perl_pop_scope
# define Perl_push_scope dll_Perl_push_scope
# define Perl_save_int dll_Perl_save_int
# if (PERL_REVISION == 5) && (PERL_VERSION >= 20)
#  define Perl_save_strlen dll_Perl_save_strlen
# endif
# define Perl_stack_grow dll_Perl_stack_grow
# define Perl_set_context dll_Perl_set_context
# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  define Perl_sv_2bool_flags dll_Perl_sv_2bool_flags
#  if (PERL_REVISION == 5) && (PERL_VERSION < 22)
#   define Perl_xs_apiversion_bootcheck dll_Perl_xs_apiversion_bootcheck
#  endif
# else
#  define Perl_sv_2bool dll_Perl_sv_2bool
# endif
# define Perl_sv_2iv dll_Perl_sv_2iv
# define Perl_sv_2mortal dll_Perl_sv_2mortal
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
#  define Perl_sv_2pv_flags dll_Perl_sv_2pv_flags
#  define Perl_sv_2pv_nolen dll_Perl_sv_2pv_nolen
# else
#  define Perl_sv_2pv dll_Perl_sv_2pv
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 32)
#  define Perl_sv_2pvbyte_flags dll_Perl_sv_2pvbyte_flags
# endif
# define Perl_sv_2pvbyte dll_Perl_sv_2pvbyte
# define Perl_sv_bless dll_Perl_sv_bless
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
#  define Perl_sv_catpvn_flags dll_Perl_sv_catpvn_flags
# else
#  define Perl_sv_catpvn dll_Perl_sv_catpvn
# endif
# ifdef PERL589_OR_LATER
#  define Perl_sv_2iv_flags dll_Perl_sv_2iv_flags
#  define Perl_newXS_flags dll_Perl_newXS_flags
# endif
# define Perl_sv_free dll_Perl_sv_free
# if (PERL_REVISION == 5) && (PERL_VERSION >= 10)
#  define Perl_sv_free2 dll_Perl_sv_free2
# endif
# define Perl_sv_isa dll_Perl_sv_isa
# define Perl_sv_magic dll_Perl_sv_magic
# define Perl_sv_setiv dll_Perl_sv_setiv
# define Perl_sv_setpv dll_Perl_sv_setpv
# define Perl_sv_setpvn dll_Perl_sv_setpvn
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
#  define Perl_sv_setsv_flags dll_Perl_sv_setsv_flags
# else
#  define Perl_sv_setsv dll_Perl_sv_setsv
# endif
# define Perl_sv_upgrade dll_Perl_sv_upgrade
# define Perl_Tstack_sp_ptr dll_Perl_Tstack_sp_ptr
# define Perl_Top_ptr dll_Perl_Top_ptr
# define Perl_Tstack_base_ptr dll_Perl_Tstack_base_ptr
# define Perl_Tstack_max_ptr dll_Perl_Tstack_max_ptr
# define Perl_Ttmps_ix_ptr dll_Perl_Ttmps_ix_ptr
# define Perl_Ttmps_floor_ptr dll_Perl_Ttmps_floor_ptr
# define Perl_Tmarkstack_ptr_ptr dll_Perl_Tmarkstack_ptr_ptr
# define Perl_Tmarkstack_max_ptr dll_Perl_Tmarkstack_max_ptr
# define Perl_TSv_ptr dll_Perl_TSv_ptr
# define Perl_TXpv_ptr dll_Perl_TXpv_ptr
# define Perl_Tna_ptr dll_Perl_Tna_ptr
# define Perl_Idefgv_ptr dll_Perl_Idefgv_ptr
# define Perl_Ierrgv_ptr dll_Perl_Ierrgv_ptr
# define Perl_Isv_yes_ptr dll_Perl_Isv_yes_ptr
# define boot_DynaLoader dll_boot_DynaLoader
# define Perl_Gthr_key_ptr dll_Perl_Gthr_key_ptr

# define Perl_sys_init dll_Perl_sys_init
# define Perl_sys_term dll_Perl_sys_term
# define Perl_ISv_ptr dll_Perl_ISv_ptr
# define Perl_Istack_max_ptr dll_Perl_Istack_max_ptr
# define Perl_Istack_base_ptr dll_Perl_Istack_base_ptr
# define Perl_Itmps_ix_ptr dll_Perl_Itmps_ix_ptr
# define Perl_Itmps_floor_ptr dll_Perl_Itmps_floor_ptr
# define Perl_IXpv_ptr dll_Perl_IXpv_ptr
# define Perl_Ina_ptr dll_Perl_Ina_ptr
# define Perl_Imarkstack_ptr_ptr dll_Perl_Imarkstack_ptr_ptr
# define Perl_Imarkstack_max_ptr dll_Perl_Imarkstack_max_ptr
# define Perl_Istack_sp_ptr dll_Perl_Istack_sp_ptr
# define Perl_Iop_ptr dll_Perl_Iop_ptr
# define Perl_call_list dll_Perl_call_list
# define Perl_Iscopestack_ix_ptr dll_Perl_Iscopestack_ix_ptr
# define Perl_Iunitcheckav_ptr dll_Perl_Iunitcheckav_ptr
# if (PERL_REVISION == 5) && (PERL_VERSION >= 22)
#  define Perl_xs_handshake dll_Perl_xs_handshake
#  define Perl_xs_boot_epilog dll_Perl_xs_boot_epilog
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  ifdef USE_ITHREADS
#   define PL_thr_key *dll_PL_thr_key
#  endif
# endif
# ifdef PERL_USE_THREAD_LOCAL
#  define PL_current_context *dll_PL_current_context
# endif
# define Perl_hv_iternext_flags dll_Perl_hv_iternext_flags
# define Perl_hv_iterinit dll_Perl_hv_iterinit
# define Perl_hv_iterkey dll_Perl_hv_iterkey
# define Perl_hv_iterval dll_Perl_hv_iterval
# define Perl_av_fetch dll_Perl_av_fetch
# define Perl_av_len dll_Perl_av_len
# define Perl_sv_2nv_flags dll_Perl_sv_2nv_flags
# if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
#  define PerlIOBase_pushed dll_PerlIOBase_pushed
#  define PerlIO_define_layer dll_PerlIO_define_layer
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 24)
#  define Perl_savetmps dll_Perl_savetmps
# endif

/*
 * Declare HANDLE for perl.dll and function pointers.
 */
static HANDLE hPerlLib = NULL;

static PerlInterpreter* (*perl_alloc)(void);
static void (*perl_construct)(PerlInterpreter*);
static void (*perl_destruct)(PerlInterpreter*);
static void (*perl_free)(PerlInterpreter*);
static int (*perl_run)(PerlInterpreter*);
static int (*perl_parse)(PerlInterpreter*, XSINIT_t, int, char**, char**);
# if defined(WIN32) || ((PERL_REVISION == 5) && (PERL_VERSION < 38))
static void* (*Perl_get_context)(void);
# endif
static void (*Perl_croak)(pTHX_ const char*, ...) __attribute__noreturn__;
# ifdef PERL5101_OR_LATER
/* Perl-5.18 has a different Perl_croak_xs_usage signature. */
#  if (PERL_REVISION == 5) && (PERL_VERSION >= 18)
static void (*Perl_croak_xs_usage)(const CV *const, const char *const params)
						    __attribute__noreturn__;
#  else
static void (*Perl_croak_xs_usage)(pTHX_ const CV *const, const char *const params)
						    __attribute__noreturn__;
#  endif
# endif
# ifdef PERL_IMPLICIT_CONTEXT
static void (*Perl_croak_nocontext)(const char*, ...) __attribute__noreturn__;
# endif
static I32 (*Perl_dowantarray)(pTHX);
static void (*Perl_free_tmps)(pTHX);
static HV* (*Perl_gv_stashpv)(pTHX_ const char*, I32);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 22)
static I32* (*Perl_markstack_grow)(pTHX);
# else
static void (*Perl_markstack_grow)(pTHX);
# endif
static MAGIC* (*Perl_mg_find)(pTHX_ SV*, int);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 28)
static int (*Perl_mg_get)(pTHX_ SV*);
# endif
static CV* (*Perl_newXS)(pTHX_ char*, XSUBADDR_t, char*);
static SV* (*Perl_newSV)(pTHX_ STRLEN);
static SV* (*Perl_newSViv)(pTHX_ IV);
static SV* (*Perl_newSVpv)(pTHX_ const char*, STRLEN);
static I32 (*Perl_call_argv)(pTHX_ const char*, I32, char**);
static I32 (*Perl_call_pv)(pTHX_ const char*, I32);
static I32 (*Perl_eval_sv)(pTHX_ SV*, I32);
static SV* (*Perl_get_sv)(pTHX_ const char*, I32);
static SV* (*Perl_eval_pv)(pTHX_ const char*, I32);
static SV* (*Perl_call_method)(pTHX_ const char*, I32);
static void (*Perl_pop_scope)(pTHX);
static void (*Perl_push_scope)(pTHX);
static void (*Perl_save_int)(pTHX_ int*);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 20)
static void (*Perl_save_strlen)(pTHX_ STRLEN* ptr);
# endif
static SV** (*Perl_stack_grow)(pTHX_ SV**, SV**p, int);
static SV** (*Perl_set_context)(void*);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
static bool (*Perl_sv_2bool_flags)(pTHX_ SV*, I32);
#  if (PERL_REVISION == 5) && (PERL_VERSION < 22)
static void (*Perl_xs_apiversion_bootcheck)(pTHX_ SV *module, const char *api_p, STRLEN api_len);
#  endif
# else
static bool (*Perl_sv_2bool)(pTHX_ SV*);
# endif
static IV (*Perl_sv_2iv)(pTHX_ SV*);
static SV* (*Perl_sv_2mortal)(pTHX_ SV*);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
static char* (*Perl_sv_2pv_flags)(pTHX_ SV*, STRLEN* const, const U32);
static char* (*Perl_sv_2pv_nolen)(pTHX_ SV*);
# else
static char* (*Perl_sv_2pv)(pTHX_ SV*, STRLEN*);
# endif
static char* (*Perl_sv_2pvbyte)(pTHX_ SV*, STRLEN*);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 32)
static char* (*Perl_sv_2pvbyte_flags)(pTHX_ SV*, STRLEN* const, const U32);
# endif
static SV* (*Perl_sv_bless)(pTHX_ SV*, HV*);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
static void (*Perl_sv_catpvn_flags)(pTHX_ SV* , const char*, STRLEN, I32);
# else
static void (*Perl_sv_catpvn)(pTHX_ SV*, const char*, STRLEN);
# endif
# ifdef PERL589_OR_LATER
static IV (*Perl_sv_2iv_flags)(pTHX_ SV* sv, I32 flags);
static CV * (*Perl_newXS_flags)(pTHX_ const char *name, XSUBADDR_t subaddr, const char *const filename, const char *const proto, U32 flags);
# endif
static void (*Perl_sv_free)(pTHX_ SV*);
static int (*Perl_sv_isa)(pTHX_ SV*, const char*);
static void (*Perl_sv_magic)(pTHX_ SV*, SV*, int, const char*, I32);
static void (*Perl_sv_setiv)(pTHX_ SV*, IV);
static void (*Perl_sv_setpv)(pTHX_ SV*, const char*);
static void (*Perl_sv_setpvn)(pTHX_ SV*, const char*, STRLEN);
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
static void (*Perl_sv_setsv_flags)(pTHX_ SV*, SV*, I32);
# else
static void (*Perl_sv_setsv)(pTHX_ SV*, SV*);
# endif
static bool (*Perl_sv_upgrade)(pTHX_ SV*, U32);
# if (PERL_REVISION == 5) && (PERL_VERSION < 10)
static SV*** (*Perl_Tstack_sp_ptr)(register PerlInterpreter*);
static OP** (*Perl_Top_ptr)(register PerlInterpreter*);
static SV*** (*Perl_Tstack_base_ptr)(register PerlInterpreter*);
static SV*** (*Perl_Tstack_max_ptr)(register PerlInterpreter*);
static I32* (*Perl_Ttmps_ix_ptr)(register PerlInterpreter*);
static I32* (*Perl_Ttmps_floor_ptr)(register PerlInterpreter*);
static I32** (*Perl_Tmarkstack_ptr_ptr)(register PerlInterpreter*);
static I32** (*Perl_Tmarkstack_max_ptr)(register PerlInterpreter*);
static SV** (*Perl_TSv_ptr)(register PerlInterpreter*);
static XPV** (*Perl_TXpv_ptr)(register PerlInterpreter*);
static STRLEN* (*Perl_Tna_ptr)(register PerlInterpreter*);
# else
/* Perl-5.18 has a different Perl_sv_free2 signature. */
#  if (PERL_REVISION == 5) && (PERL_VERSION >= 18)
static void (*Perl_sv_free2)(pTHX_ SV*, const U32);
#  else
static void (*Perl_sv_free2)(pTHX_ SV*);
#  endif
static void (*Perl_sys_init)(int* argc, char*** argv);
static void (*Perl_sys_term)(void);
static void (*Perl_call_list)(pTHX_ I32, AV*);
#  if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  else
static SV** (*Perl_ISv_ptr)(register PerlInterpreter*);
static SV*** (*Perl_Istack_max_ptr)(register PerlInterpreter*);
static SV*** (*Perl_Istack_base_ptr)(register PerlInterpreter*);
static XPV** (*Perl_IXpv_ptr)(register PerlInterpreter*);
static I32* (*Perl_Itmps_ix_ptr)(register PerlInterpreter*);
static I32* (*Perl_Itmps_floor_ptr)(register PerlInterpreter*);
static STRLEN* (*Perl_Ina_ptr)(register PerlInterpreter*);
static I32** (*Perl_Imarkstack_ptr_ptr)(register PerlInterpreter*);
static I32** (*Perl_Imarkstack_max_ptr)(register PerlInterpreter*);
static SV*** (*Perl_Istack_sp_ptr)(register PerlInterpreter*);
static OP** (*Perl_Iop_ptr)(register PerlInterpreter*);
static I32* (*Perl_Iscopestack_ix_ptr)(register PerlInterpreter*);
static AV** (*Perl_Iunitcheckav_ptr)(register PerlInterpreter*);
#  endif
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 22)
static I32 (*Perl_xs_handshake)(const U32, void *, const char *, ...);
static void (*Perl_xs_boot_epilog)(pTHX_ const U32);
# endif

# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  ifdef USE_ITHREADS
static perl_key* dll_PL_thr_key;
#  endif
# else
static GV** (*Perl_Idefgv_ptr)(register PerlInterpreter*);
static GV** (*Perl_Ierrgv_ptr)(register PerlInterpreter*);
static SV* (*Perl_Isv_yes_ptr)(register PerlInterpreter*);
static perl_key* (*Perl_Gthr_key_ptr)_((pTHX));
# endif
# ifdef PERL_USE_THREAD_LOCAL
static void** dll_PL_current_context;
# endif
static void (*boot_DynaLoader)_((pTHX_ CV*));
static HE * (*Perl_hv_iternext_flags)(pTHX_ HV *, I32);
static I32 (*Perl_hv_iterinit)(pTHX_ HV *);
static char * (*Perl_hv_iterkey)(pTHX_ HE *, I32 *);
static SV * (*Perl_hv_iterval)(pTHX_ HV *, HE *);
static SV** (*Perl_av_fetch)(pTHX_ AV *, SSize_t, I32);
static SSize_t (*Perl_av_len)(pTHX_ AV *);
static NV (*Perl_sv_2nv_flags)(pTHX_ SV *const, const I32);
# if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
static IV (*PerlIOBase_pushed)(pTHX_ PerlIO *, const char *, SV *, PerlIO_funcs *);
static void (*PerlIO_define_layer)(pTHX_ PerlIO_funcs *);
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 24)
static void (*Perl_savetmps)(pTHX);
# endif

/*
 * Table of name to function pointer of perl.
 */
static struct {
    char* name;
    PERL_PROC* ptr;
} perl_funcname_table[] = {
    {"perl_alloc", (PERL_PROC*)&perl_alloc},
    {"perl_construct", (PERL_PROC*)&perl_construct},
    {"perl_destruct", (PERL_PROC*)&perl_destruct},
    {"perl_free", (PERL_PROC*)&perl_free},
    {"perl_run", (PERL_PROC*)&perl_run},
    {"perl_parse", (PERL_PROC*)&perl_parse},
# if defined(WIN32) || ((PERL_REVISION == 5) && (PERL_VERSION < 38))
    {"Perl_get_context", (PERL_PROC*)&Perl_get_context},
# endif
    {"Perl_croak", (PERL_PROC*)&Perl_croak},
# ifdef PERL5101_OR_LATER
    {"Perl_croak_xs_usage", (PERL_PROC*)&Perl_croak_xs_usage},
# endif
# ifdef PERL_IMPLICIT_CONTEXT
    {"Perl_croak_nocontext", (PERL_PROC*)&Perl_croak_nocontext},
# endif
    {"Perl_dowantarray", (PERL_PROC*)&Perl_dowantarray},
    {"Perl_free_tmps", (PERL_PROC*)&Perl_free_tmps},
    {"Perl_gv_stashpv", (PERL_PROC*)&Perl_gv_stashpv},
    {"Perl_markstack_grow", (PERL_PROC*)&Perl_markstack_grow},
    {"Perl_mg_find", (PERL_PROC*)&Perl_mg_find},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 28)
    {"Perl_mg_get", (PERL_PROC*)&Perl_mg_get},
# endif
    {"Perl_newXS", (PERL_PROC*)&Perl_newXS},
    {"Perl_newSV", (PERL_PROC*)&Perl_newSV},
    {"Perl_newSViv", (PERL_PROC*)&Perl_newSViv},
    {"Perl_newSVpv", (PERL_PROC*)&Perl_newSVpv},
    {"Perl_call_argv", (PERL_PROC*)&Perl_call_argv},
    {"Perl_call_pv", (PERL_PROC*)&Perl_call_pv},
    {"Perl_eval_sv", (PERL_PROC*)&Perl_eval_sv},
    {"Perl_get_sv", (PERL_PROC*)&Perl_get_sv},
    {"Perl_eval_pv", (PERL_PROC*)&Perl_eval_pv},
    {"Perl_call_method", (PERL_PROC*)&Perl_call_method},
    {"Perl_pop_scope", (PERL_PROC*)&Perl_pop_scope},
    {"Perl_push_scope", (PERL_PROC*)&Perl_push_scope},
    {"Perl_save_int", (PERL_PROC*)&Perl_save_int},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 20)
    {"Perl_save_strlen", (PERL_PROC*)&Perl_save_strlen},
# endif
    {"Perl_stack_grow", (PERL_PROC*)&Perl_stack_grow},
    {"Perl_set_context", (PERL_PROC*)&Perl_set_context},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
    {"Perl_sv_2bool_flags", (PERL_PROC*)&Perl_sv_2bool_flags},
#  if (PERL_REVISION == 5) && (PERL_VERSION < 22)
    {"Perl_xs_apiversion_bootcheck",(PERL_PROC*)&Perl_xs_apiversion_bootcheck},
#  endif
# else
    {"Perl_sv_2bool", (PERL_PROC*)&Perl_sv_2bool},
# endif
    {"Perl_sv_2iv", (PERL_PROC*)&Perl_sv_2iv},
    {"Perl_sv_2mortal", (PERL_PROC*)&Perl_sv_2mortal},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
    {"Perl_sv_2pv_flags", (PERL_PROC*)&Perl_sv_2pv_flags},
    {"Perl_sv_2pv_nolen", (PERL_PROC*)&Perl_sv_2pv_nolen},
# else
    {"Perl_sv_2pv", (PERL_PROC*)&Perl_sv_2pv},
# endif
    {"Perl_sv_2pvbyte", (PERL_PROC*)&Perl_sv_2pvbyte},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 32)
    {"Perl_sv_2pvbyte_flags", (PERL_PROC*)&Perl_sv_2pvbyte_flags},
# endif
# ifdef PERL589_OR_LATER
    {"Perl_sv_2iv_flags", (PERL_PROC*)&Perl_sv_2iv_flags},
    {"Perl_newXS_flags", (PERL_PROC*)&Perl_newXS_flags},
# endif
    {"Perl_sv_bless", (PERL_PROC*)&Perl_sv_bless},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
    {"Perl_sv_catpvn_flags", (PERL_PROC*)&Perl_sv_catpvn_flags},
# else
    {"Perl_sv_catpvn", (PERL_PROC*)&Perl_sv_catpvn},
# endif
    {"Perl_sv_free", (PERL_PROC*)&Perl_sv_free},
    {"Perl_sv_isa", (PERL_PROC*)&Perl_sv_isa},
    {"Perl_sv_magic", (PERL_PROC*)&Perl_sv_magic},
    {"Perl_sv_setiv", (PERL_PROC*)&Perl_sv_setiv},
    {"Perl_sv_setpv", (PERL_PROC*)&Perl_sv_setpv},
    {"Perl_sv_setpvn", (PERL_PROC*)&Perl_sv_setpvn},
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
    {"Perl_sv_setsv_flags", (PERL_PROC*)&Perl_sv_setsv_flags},
# else
    {"Perl_sv_setsv", (PERL_PROC*)&Perl_sv_setsv},
# endif
    {"Perl_sv_upgrade", (PERL_PROC*)&Perl_sv_upgrade},
# if (PERL_REVISION == 5) && (PERL_VERSION < 10)
    {"Perl_Tstack_sp_ptr", (PERL_PROC*)&Perl_Tstack_sp_ptr},
    {"Perl_Top_ptr", (PERL_PROC*)&Perl_Top_ptr},
    {"Perl_Tstack_base_ptr", (PERL_PROC*)&Perl_Tstack_base_ptr},
    {"Perl_Tstack_max_ptr", (PERL_PROC*)&Perl_Tstack_max_ptr},
    {"Perl_Ttmps_ix_ptr", (PERL_PROC*)&Perl_Ttmps_ix_ptr},
    {"Perl_Ttmps_floor_ptr", (PERL_PROC*)&Perl_Ttmps_floor_ptr},
    {"Perl_Tmarkstack_ptr_ptr", (PERL_PROC*)&Perl_Tmarkstack_ptr_ptr},
    {"Perl_Tmarkstack_max_ptr", (PERL_PROC*)&Perl_Tmarkstack_max_ptr},
    {"Perl_TSv_ptr", (PERL_PROC*)&Perl_TSv_ptr},
    {"Perl_TXpv_ptr", (PERL_PROC*)&Perl_TXpv_ptr},
    {"Perl_Tna_ptr", (PERL_PROC*)&Perl_Tna_ptr},
# else
    {"Perl_sv_free2", (PERL_PROC*)&Perl_sv_free2},
    {"Perl_sys_init", (PERL_PROC*)&Perl_sys_init},
    {"Perl_sys_term", (PERL_PROC*)&Perl_sys_term},
    {"Perl_call_list", (PERL_PROC*)&Perl_call_list},
#  if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  else
    {"Perl_ISv_ptr", (PERL_PROC*)&Perl_ISv_ptr},
    {"Perl_Istack_max_ptr", (PERL_PROC*)&Perl_Istack_max_ptr},
    {"Perl_Istack_base_ptr", (PERL_PROC*)&Perl_Istack_base_ptr},
    {"Perl_IXpv_ptr", (PERL_PROC*)&Perl_IXpv_ptr},
    {"Perl_Itmps_ix_ptr", (PERL_PROC*)&Perl_Itmps_ix_ptr},
    {"Perl_Itmps_floor_ptr", (PERL_PROC*)&Perl_Itmps_floor_ptr},
    {"Perl_Ina_ptr", (PERL_PROC*)&Perl_Ina_ptr},
    {"Perl_Imarkstack_ptr_ptr", (PERL_PROC*)&Perl_Imarkstack_ptr_ptr},
    {"Perl_Imarkstack_max_ptr", (PERL_PROC*)&Perl_Imarkstack_max_ptr},
    {"Perl_Istack_sp_ptr", (PERL_PROC*)&Perl_Istack_sp_ptr},
    {"Perl_Iop_ptr", (PERL_PROC*)&Perl_Iop_ptr},
    {"Perl_Iscopestack_ix_ptr", (PERL_PROC*)&Perl_Iscopestack_ix_ptr},
    {"Perl_Iunitcheckav_ptr", (PERL_PROC*)&Perl_Iunitcheckav_ptr},
#  endif
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 22)
    {"Perl_xs_handshake", (PERL_PROC*)&Perl_xs_handshake},
    {"Perl_xs_boot_epilog", (PERL_PROC*)&Perl_xs_boot_epilog},
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  ifdef USE_ITHREADS
    {"PL_thr_key", (PERL_PROC*)&dll_PL_thr_key},
#  endif
# ifdef PERL_USE_THREAD_LOCAL
    {"PL_current_context", (PERL_PROC*)&dll_PL_current_context},
# endif
# else
    {"Perl_Idefgv_ptr", (PERL_PROC*)&Perl_Idefgv_ptr},
    {"Perl_Ierrgv_ptr", (PERL_PROC*)&Perl_Ierrgv_ptr},
    {"Perl_Isv_yes_ptr", (PERL_PROC*)&Perl_Isv_yes_ptr},
    {"Perl_Gthr_key_ptr", (PERL_PROC*)&Perl_Gthr_key_ptr},
# endif
    {"boot_DynaLoader", (PERL_PROC*)&boot_DynaLoader},
    {"Perl_hv_iternext_flags", (PERL_PROC*)&Perl_hv_iternext_flags},
    {"Perl_hv_iterinit", (PERL_PROC*)&Perl_hv_iterinit},
    {"Perl_hv_iterkey", (PERL_PROC*)&Perl_hv_iterkey},
    {"Perl_hv_iterval", (PERL_PROC*)&Perl_hv_iterval},
    {"Perl_av_fetch", (PERL_PROC*)&Perl_av_fetch},
    {"Perl_av_len", (PERL_PROC*)&Perl_av_len},
    {"Perl_sv_2nv_flags", (PERL_PROC*)&Perl_sv_2nv_flags},
# if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
    {"PerlIOBase_pushed", (PERL_PROC*)&PerlIOBase_pushed},
    {"PerlIO_define_layer", (PERL_PROC*)&PerlIO_define_layer},
# endif
# if (PERL_REVISION == 5) && (PERL_VERSION >= 24)
    {"Perl_savetmps", (PERL_PROC*)&Perl_savetmps},
# endif
    {"", NULL},
};

# if (PERL_REVISION == 5) && (PERL_VERSION <= 30)
// In 5.30, GIMME_V requires linking to Perl_block_gimme() instead of being
// completely inline. Just use the deprecated GIMME for simplicity.
#  undef GIMME_V
#  define GIMME_V GIMME
# endif

/*
 * Make all runtime-links of perl.
 *
 * 1. Get module handle using dlopen() or vimLoadLib().
 * 2. Get pointer to perl function by GetProcAddress.
 * 3. Repeat 2, until get all functions will be used.
 *
 * Parameter 'libname' provides name of DLL.
 * Return OK or FAIL.
 */
    static int
perl_runtime_link_init(char *libname, int verbose)
{
    int i;

    if (hPerlLib != NULL)
	return OK;
    if ((hPerlLib = load_dll(libname)) == NULL)
    {
	if (verbose)
	    semsg(_("E370: Could not load library %s"), libname);
	return FAIL;
    }
    for (i = 0; perl_funcname_table[i].ptr; ++i)
    {
	if (!(*perl_funcname_table[i].ptr = symbol_from_dll(hPerlLib,
			perl_funcname_table[i].name)))
	{
	    close_dll(hPerlLib);
	    hPerlLib = NULL;
	    if (verbose)
		semsg((const char *)_(e_could_not_load_library_function_str), perl_funcname_table[i].name);
	    return FAIL;
	}
    }
    return OK;
}

/*
 * If runtime-link-perl(DLL) was loaded successfully, return TRUE.
 * There were no DLL loaded, return FALSE.
 */
    int
perl_enabled(int verbose)
{
    return perl_runtime_link_init((char *)p_perldll, verbose) == OK;
}
#endif /* DYNAMIC_PERL */

#if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
static void vim_IOLayer_init(void);
#endif

/*
 * perl_init(): initialize perl interpreter
 * We have to call perl_parse to initialize some structures,
 * there's nothing to actually parse.
 */
    static void
perl_init(void)
{
    char *bootargs[] = { "VI", NULL };
    int argc = 3;
    static char *argv[] = { "", "-e", "" };

#if (PERL_REVISION == 5) && (PERL_VERSION >= 10)
    Perl_sys_init(&argc, (char***)&argv);
#endif
    perl_interp = perl_alloc();
    perl_construct(perl_interp);
    perl_parse(perl_interp, xs_init, argc, argv, 0);
    perl_call_argv("VIM::bootstrap", (long)G_DISCARD, bootargs);
    VIM_init();
#ifdef USE_SFIO
    sfdisc(PerlIO_stdout(), sfdcnewvim());
    sfdisc(PerlIO_stderr(), sfdcnewvim());
    sfsetbuf(PerlIO_stdout(), NULL, 0);
    sfsetbuf(PerlIO_stderr(), NULL, 0);
#elif defined(PERLIO_LAYERS)
    vim_IOLayer_init();
#endif
}

/*
 * Clean up after ourselves.
 */
    void
perl_end(void)
{
    if (perl_interp)
    {
	perl_run(perl_interp);
	perl_destruct(perl_interp);
	perl_free(perl_interp);
	perl_interp = NULL;
#if (PERL_REVISION == 5) && (PERL_VERSION >= 10)
	Perl_sys_term();
#endif
    }
}

/*
 * msg_split(): send a message to the message handling routines
 * split at '\n' first though.
 */
    void
msg_split(
    char_u	*s,
    int		attr)	/* highlighting attributes */
{
    char *next;
    char *token = (char *)s;

    while ((next = strchr(token, '\n')) && !got_int)
    {
	*next++ = '\0';			/* replace \n with \0 */
	msg_attr(token, attr);
	token = next;
    }
    if (*token && !got_int)
	msg_attr(token, attr);
}

#ifndef FEAT_EVAL
/*
 * This stub is needed because an "#ifdef FEAT_EVAL" around Eval() doesn't
 * work properly.
 */
    char_u *
eval_to_string(
    char_u	*arg UNUSED,
    int		convert UNUSED,
    int		use_simple_function UNUSED)
{
    return NULL;
}
#endif

/*
 * Create a new reference to an SV pointing to the SCR structure
 * The b_perl_private/w_perl_private part of the SCR structure points to the
 * SV, so there can only be one such SV for a particular SCR structure.  When
 * the last reference has gone (DESTROY is called),
 * b_perl_private/w_perl_private is reset; When the screen goes away before
 * all references are gone, the value of the SV is reset;
 * any subsequent use of any of those reference will produce
 * a warning. (see typemap)
 */

    static SV *
newWINrv(SV *rv, win_T *ptr)
{
    sv_upgrade(rv, SVt_RV);
    if (ptr->w_perl_private == NULL)
    {
	ptr->w_perl_private = newSV(0);
	sv_setiv(ptr->w_perl_private, PTR2IV(ptr));
    }
    SvREFCNT_inc_void_NN(ptr->w_perl_private);
    SvRV(rv) = ptr->w_perl_private;
    SvROK_on(rv);
    return sv_bless(rv, gv_stashpv("VIWIN", TRUE));
}

    static SV *
newBUFrv(SV *rv, buf_T *ptr)
{
    sv_upgrade(rv, SVt_RV);
    if (ptr->b_perl_private == NULL)
    {
	ptr->b_perl_private = newSV(0);
	sv_setiv(ptr->b_perl_private, PTR2IV(ptr));
    }
    SvREFCNT_inc_void_NN(ptr->b_perl_private);
    SvRV(rv) = ptr->b_perl_private;
    SvROK_on(rv);
    return sv_bless(rv, gv_stashpv("VIBUF", TRUE));
}

#if 0
SV *__sv_save[1024];
int __sv_save_ix;
#  define D_Save_Sv(sv) do { if (__sv_save_ix < 1024) __sv_save[__sv_save_ix++] = (sv); } while (0)
#else
#  define D_Save_Sv(sv) NOOP
#endif

/*
 * perl_win_free
 *	Remove all references to the window to be destroyed
 */
    void
perl_win_free(win_T *wp)
{
    if (wp->w_perl_private && perl_interp != NULL)
    {
	SV *sv = (SV*)wp->w_perl_private;
	D_Save_Sv(sv);
	sv_setiv(sv, 0);
	SvREFCNT_dec(sv);
    }
    wp->w_perl_private = NULL;
}

    void
perl_buf_free(buf_T *bp)
{
    if (bp->b_perl_private && perl_interp != NULL)
    {
	SV *sv = (SV *)bp->b_perl_private;
	D_Save_Sv(sv);
	sv_setiv(sv, 0);
	SvREFCNT_dec(sv);
    }
    bp->b_perl_private = NULL;
}

#ifndef PROTO
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
I32 cur_val(pTHX_ IV iv, SV *sv);
# else
I32 cur_val(IV iv, SV *sv);
# endif

/*
 * Handler for the magic variables $main::curwin and $main::curbuf.
 * The handler is put into the magic vtbl for these variables.
 * (This is effectively a C-level equivalent of a tied variable).
 * There is no "set" function as the variables are read-only.
 */
# if (PERL_REVISION == 5) && (PERL_VERSION >= 8)
I32 cur_val(pTHX_ IV iv, SV *sv)
# else
I32 cur_val(IV iv, SV *sv)
# endif
{
    SV *rv;

    if (iv == 0)
	rv = newWINrv(newSV(0), curwin);
    else
	rv = newBUFrv(newSV(0), curbuf);

    if (SvRV(sv) != SvRV(rv))
	// XXX: This magic variable is a bit confusing...
	// Is currently refcounted ?
	sv_setsv(sv, rv);

    SvREFCNT_dec(rv);

    return 0;
}
#endif /* !PROTO */

struct ufuncs cw_funcs = { cur_val, 0, 0 };
struct ufuncs cb_funcs = { cur_val, 0, 1 };

/*
 * VIM_init(): Vim-specific initialisation.
 * Make the magical main::curwin and main::curbuf variables
 */
    static void
VIM_init(void)
{
    static char cw[] = "main::curwin";
    static char cb[] = "main::curbuf";
    SV *sv;

    sv = perl_get_sv(cw, TRUE);
    sv_magic(sv, NULL, 'U', (char *)&cw_funcs, sizeof(cw_funcs));
    SvREADONLY_on(sv);

    sv = perl_get_sv(cb, TRUE);
    sv_magic(sv, NULL, 'U', (char *)&cb_funcs, sizeof(cb_funcs));
    SvREADONLY_on(sv);

    /*
     * Setup the Safe compartment.
     * It shouldn't be a fatal error if the Safe module is missing.
     * XXX: Only shares the 'Msg' routine (which has to be called
     * like 'Msg(...)').
     */
    (void)perl_eval_pv( "if ( eval( 'require Safe' ) ) { $VIM::safe = Safe->new(); $VIM::safe->share_from( 'VIM', ['Msg'] ); }", G_DISCARD | G_VOID );

}

#ifdef DYNAMIC_PERL
static char *e_noperl = N_("Sorry, this command is disabled: the Perl library could not be loaded.");
#endif

/*
 * ":perl"
 */
    void
ex_perl(exarg_T *eap)
{
    char	*err;
    char	*script;
    STRLEN	length;
    SV		*sv;
#ifdef HAVE_SANDBOX
    SV		*safe;
#endif

    script = (char *)script_get(eap, eap->arg);
    if (eap->skip)
    {
	vim_free(script);
	return;
    }

    if (perl_interp == NULL)
    {
#ifdef DYNAMIC_PERL
	if (!perl_enabled(TRUE))
	{
	    emsg(_(e_noperl));
	    vim_free(script);
	    return;
	}
#endif
	perl_init();
    }

    {
    dSP;
    ENTER;
    SAVETMPS;

    if (script == NULL)
	sv = newSVpv((char *)eap->arg, 0);
    else
    {
	sv = newSVpv(script, 0);
	vim_free(script);
    }

    if (sandbox || secure)
    {
	safe = perl_get_sv("VIM::safe", FALSE);
# ifndef MAKE_TEST  /* avoid a warning for unreachable code */
	if (safe == NULL || !SvTRUE(safe))
	    emsg(_(e_perl_evaluation_forbidden_in_sandbox_without_safe_module));
	else
# endif
	{
	    PUSHMARK(SP);
	    XPUSHs(safe);
	    XPUSHs(sv);
	    PUTBACK;
	    perl_call_method("reval", G_DISCARD);
	}
    }
    else
	perl_eval_sv(sv, G_DISCARD | G_NOARGS);

    SvREFCNT_dec(sv);

    err = SvPV(GvSV(PL_errgv), length);

    FREETMPS;
    LEAVE;

    if (!length)
	return;

    msg_split((char_u *)err, highlight_attr[HLF_E]);
    return;
    }
}

    static int
replace_line(linenr_T *line, linenr_T *end)
{
    char *str;

    if (SvOK(GvSV(PL_defgv)))
    {
	str = SvPV(GvSV(PL_defgv), PL_na);
	ml_replace(*line, (char_u *)str, 1);
	changed_bytes(*line, 0);
    }
    else
    {
	ml_delete(*line);
	deleted_lines_mark(*line, 1L);
	--(*end);
	--(*line);
    }
    return OK;
}

static struct ref_map_S {
    void *vim_ref;
    SV   *perl_ref;
    struct ref_map_S *next;
} *ref_map = NULL;

    static void
ref_map_free(void)
{
    struct ref_map_S *tofree;
    struct ref_map_S *refs = ref_map;

    while (refs) {
	tofree = refs;
	refs = refs->next;
	vim_free(tofree);
    }
    ref_map = NULL;
}

    static struct ref_map_S *
ref_map_find_SV(SV *const sv)
{
    struct ref_map_S *refs = ref_map;
    int count = 350;

    while (refs) {
	if (refs->perl_ref == sv)
	    break;
	refs = refs->next;
	count--;
    }

    if (!refs && count > 0) {
	refs = (struct ref_map_S *)alloc(sizeof(struct ref_map_S));
	if (!refs)
	    return NULL;
	refs->perl_ref = sv;
	refs->vim_ref = NULL;
	refs->next = ref_map;
	ref_map = refs;
    }

    return refs;
}

    static int
perl_to_vim(SV *sv, typval_T *rettv)
{
    if (SvROK(sv))
	sv = SvRV(sv);

    switch (SvTYPE(sv)) {
	case SVt_NULL:
	    break;
	case SVt_NV:	/* float */
	    rettv->v_type	= VAR_FLOAT;
	    rettv->vval.v_float = SvNV(sv);
	    break;
	case SVt_IV:	/* integer */
	    if (!SvROK(sv)) { /* references should be string */
		rettv->vval.v_number = SvIV(sv);
		break;
	    }
	    /* FALLTHROUGH */
	case SVt_PV:	/* string */
	{
	    size_t  len		= 0;
	    char *  str_from	= SvPV(sv, len);
	    char_u *str_to	= (char_u*)alloc(
				      (unsigned)(sizeof(char_u) * (len + 1)));

	    if (str_to) {
		str_to[len] = '\0';

		while (len--) {
		    if (str_from[len] == '\0')
			str_to[len] = '\n';
		    else
			str_to[len] = str_from[len];
		}
	    }

	    rettv->v_type	    = VAR_STRING;
	    rettv->vval.v_string    = str_to;
	    break;
	}
	case SVt_PVAV:	/* list */
	{
	    SSize_t		size;
	    listitem_T *	item;
	    SV **		item2;
	    list_T *		list;
	    struct ref_map_S *	refs;

	    if ((refs = ref_map_find_SV(sv)) == NULL)
		return FAIL;

	    if (refs->vim_ref)
		list = (list_T *) refs->vim_ref;
	    else
	    {
		if ((list = list_alloc()) == NULL)
		    return FAIL;
		refs->vim_ref = list;

		for (size = av_len((AV*)sv); size >= 0; size--)
		{
		    if ((item = listitem_alloc()) == NULL)
			break;

		    item->li_tv.v_type		= VAR_NUMBER;
		    item->li_tv.v_lock		= 0;
		    item->li_tv.vval.v_number	= 0;
		    list_insert(list, item, list->lv_first);

		    item2 = av_fetch((AV *)sv, size, 0);

		    if (item2 == NULL || *item2 == NULL ||
				    perl_to_vim(*item2, &item->li_tv) == FAIL)
			break;
		}
	    }

	    rettv_list_set(rettv, list);
	    break;
	}
	case SVt_PVHV:	/* dictionary */
	{
	    HE *		entry;
	    I32			key_len;
	    char *		key;
	    dictitem_T *	item;
	    SV *		item2;
	    dict_T *		dict;
	    struct ref_map_S *	refs;

	    if ((refs = ref_map_find_SV(sv)) == NULL)
		return FAIL;

	    if (refs->vim_ref)
		dict = (dict_T *) refs->vim_ref;
	    else
	    {

		if ((dict = dict_alloc()) == NULL)
		    return FAIL;
		refs->vim_ref = dict;

		hv_iterinit((HV *)sv);

		for (entry = hv_iternext((HV *)sv); entry; entry = hv_iternext((HV *)sv))
		{
		    key_len = 0;
		    key = hv_iterkey(entry, &key_len);

		    if (!key || !key_len || strlen(key) < (size_t)key_len) {
			semsg("Malformed key Dictionary '%s'", key && *key ? key : "(empty)");
			break;
		    }

		    if ((item = dictitem_alloc((char_u *)key)) == NULL)
			break;
		    item->di_tv.v_type = VAR_NUMBER;
		    item->di_tv.vval.v_number = 0;

		    if (dict_add(dict, item) == FAIL) {
			dictitem_free(item);
			break;
		    }
		    item2 = hv_iterval((HV *)sv, entry);
		    if (item2 == NULL || perl_to_vim(item2, &item->di_tv) == FAIL)
			break;
		}
	    }

	    rettv_dict_set(rettv, dict);
	    break;
	}
	default:	/* not convertible */
	{
	    char *val	    = SvPV_nolen(sv);
	    rettv->v_type   = VAR_STRING;
	    rettv->vval.v_string = val ? vim_strsave((char_u *)val) : NULL;
	    break;
	}
    }
    return OK;
}

/*
 * "perleval()"
 */
    void
do_perleval(char_u *str, typval_T *rettv)
{
    char	*err = NULL;
    STRLEN	err_len = 0;
    SV		*sv = NULL;
#ifdef HAVE_SANDBOX
    SV		*safe;
#endif

    if (perl_interp == NULL)
    {
#ifdef DYNAMIC_PERL
	if (!perl_enabled(TRUE))
	{
	    emsg(_(e_noperl));
	    return;
	}
#endif
	perl_init();
    }

    {
	dSP;
	ENTER;
	SAVETMPS;

	if (sandbox || secure)
	{
	    safe = get_sv("VIM::safe", FALSE);
# ifndef MAKE_TEST  /* avoid a warning for unreachable code */
	    if (safe == NULL || !SvTRUE(safe))
		emsg(_(e_perl_evaluation_forbidden_in_sandbox_without_safe_module));
	    else
# endif
	    {
		sv = newSVpv((char *)str, 0);
		PUSHMARK(SP);
		XPUSHs(safe);
		XPUSHs(sv);
		PUTBACK;
		call_method("reval", G_SCALAR);
		SPAGAIN;
		SvREFCNT_dec(sv);
		sv = POPs;
		PUTBACK;
	    }
	}
	else
	    sv = eval_pv((char *)str, 0);

	if (sv) {
	    perl_to_vim(sv, rettv);
	    ref_map_free();
	    err = SvPV(GvSV(PL_errgv), err_len);
	}
	FREETMPS;
	LEAVE;
    }
    if (err_len)
	msg_split((char_u *)err, highlight_attr[HLF_E]);
}

/*
 * ":perldo".
 */
    void
ex_perldo(exarg_T *eap)
{
    STRLEN	length;
    SV		*sv;
    char	*str;
    linenr_T	i;
    buf_T	*was_curbuf = curbuf;

    if (BUFEMPTY())
	return;

    if (perl_interp == NULL)
    {
#ifdef DYNAMIC_PERL
	if (!perl_enabled(TRUE))
	{
	    emsg(_(e_noperl));
	    return;
	}
#endif
	perl_init();
    }
    {
    dSP;
    length = strlen((char *)eap->arg);
    sv = newSV(length + sizeof("sub VIM::perldo {") - 1 + 1);
    sv_setpvn(sv, "sub VIM::perldo {", sizeof("sub VIM::perldo {") - 1);
    sv_catpvn(sv, (char *)eap->arg, length);
    sv_catpvn(sv, "}", 1);
    perl_eval_sv(sv, G_DISCARD | G_NOARGS);
    SvREFCNT_dec(sv);
    str = SvPV(GvSV(PL_errgv), length);
    if (length)
	goto err;

    if (u_save(eap->line1 - 1, eap->line2 + 1) != OK)
	return;

    ENTER;
    SAVETMPS;
    for (i = eap->line1; i <= eap->line2; i++)
    {
	/* Check the line number, the command my have deleted lines. */
	if (i > curbuf->b_ml.ml_line_count)
	    break;
	sv_setpv(GvSV(PL_defgv), (char *)ml_get(i));
	PUSHMARK(sp);
	perl_call_pv("VIM::perldo", G_SCALAR | G_EVAL);
	str = SvPV(GvSV(PL_errgv), length);
	if (length || curbuf != was_curbuf || i > curbuf->b_ml.ml_line_count)
	    break;
	SPAGAIN;
	if (SvTRUEx(POPs))
	{
	    if (replace_line(&i, &eap->line2) != OK)
	    {
		PUTBACK;
		break;
	    }
	}
	PUTBACK;
    }
    FREETMPS;
    LEAVE;
    check_cursor();
    update_screen(UPD_NOT_VALID);
    if (!length)
	return;

err:
    msg_split((char_u *)str, highlight_attr[HLF_E]);
    return;
    }
}

#if defined(PERLIO_LAYERS) && !defined(USE_SFIO)
typedef struct {
    struct _PerlIO base;
    int attr;
} PerlIOVim;

    static IV
PerlIOVim_pushed(pTHX_ PerlIO *f, const char *mode,
		 SV *arg, PerlIO_funcs *tab)
{
    PerlIOVim *s = PerlIOSelf(f, PerlIOVim);
    s->attr = 0;
    if (arg && SvPOK(arg))
	s->attr = syn_name2attr((char_u *)SvPV_nolen(arg));
    return PerlIOBase_pushed(aTHX_ f, mode, (SV *)NULL, tab);
}

    static SSize_t
PerlIOVim_write(pTHX_ PerlIO *f, const void *vbuf, Size_t count)
{
    char_u *str;
    PerlIOVim * s = PerlIOSelf(f, PerlIOVim);

    str = vim_strnsave((char_u *)vbuf, count);
    if (str == NULL)
	return 0;
    msg_split((char_u *)str, s->attr);
    vim_free(str);

    return (SSize_t)count;
}

static PERLIO_FUNCS_DECL(PerlIO_Vim) = {
    sizeof(PerlIO_funcs),
    "Vim",
    sizeof(PerlIOVim),
    PERLIO_K_DUMMY,	/* flags */
    PerlIOVim_pushed,
    NULL,		/* popped */
    NULL,		/* open */
    NULL,		/* binmode */
    NULL,		/* arg */
    NULL,		/* fileno */
    NULL,		/* dup */
    NULL,		/* read */
    NULL,		/* unread */
    PerlIOVim_write,
    NULL,		/* seek */
    NULL,		/* tell */
    NULL,		/* close */
    NULL,		/* flush */
    NULL,		/* fill */
    NULL,		/* eof */
    NULL,		/* error */
    NULL,		/* clearerr */
    NULL,		/* setlinebuf */
    NULL,		/* get_base */
    NULL,		/* get_bufsiz */
    NULL,		/* get_ptr */
    NULL,		/* get_cnt */
    NULL		/* set_ptrcnt */
};

/* Use Vim routine for print operator */
    static void
vim_IOLayer_init(void)
{
    PerlIO_define_layer(aTHX_ PERLIO_FUNCS_CAST(&PerlIO_Vim));
    (void)eval_pv(   "binmode(STDOUT, ':Vim')"
                "  && binmode(STDERR, ':Vim(ErrorMsg)');", 0);
}
#endif /* PERLIO_LAYERS && !USE_SFIO */

#ifdef DYNAMIC_PERL

// Certain functionality that we use like SvREFCNT_dec are inlined for
// performance reasons. They reference Perl APIs like Perl_sv_free2(), which
// would cause linking errors in dynamic builds as we don't link against Perl
// during build time. Manually fix it here by redirecting these functions
// towards the dynamically loaded version.

# if (PERL_REVISION == 5) && (PERL_VERSION >= 18)
#  undef Perl_sv_free2
void Perl_sv_free2(pTHX_ SV *const sv, const U32 refcnt)
{
    (*dll_Perl_sv_free2)(aTHX_ sv, refcnt);
}
# else
#  undef Perl_sv_free2
void Perl_sv_free2(pTHX_ SV* sv)
{
    (*dll_Perl_sv_free2)(aTHX_ sv);
}
# endif

# if (PERL_REVISION == 5) && (PERL_VERSION >= 14)
#  undef Perl_sv_2bool_flags
bool Perl_sv_2bool_flags(pTHX_ SV* sv, I32 flags)
{
    return (*dll_Perl_sv_2bool_flags)(aTHX_ sv, flags);
}
# endif

# if (PERL_REVISION == 5) && (PERL_VERSION >= 28)
#  undef Perl_mg_get
int Perl_mg_get(pTHX_ SV* sv)
{
    return (*dll_Perl_mg_get)(aTHX_ sv);
}
# endif

# undef Perl_sv_2nv_flags
NV Perl_sv_2nv_flags(pTHX_ SV *const sv, const I32 flags)
{
    return (*dll_Perl_sv_2nv_flags)(aTHX_ sv, flags);
}

# ifdef PERL589_OR_LATER
#  undef Perl_sv_2iv_flags
IV Perl_sv_2iv_flags(pTHX_ SV *const sv, const I32 flags)
{
    return (*dll_Perl_sv_2iv_flags)(aTHX_ sv, flags);
}
# endif

#endif // DYNAMIC_PERL

XS(boot_VIM);

    static void
xs_init(pTHX)
{
    char *file = __FILE__;

    /* DynaLoader is a special case */
    newXS("DynaLoader::boot_DynaLoader", boot_DynaLoader, file);
    newXS("VIM::bootstrap", boot_VIM, file);
}

typedef win_T *	VIWIN;
typedef buf_T *	VIBUF;

MODULE = VIM	    PACKAGE = VIM

void
Msg(text, hl=NULL)
    char	*text;
    char	*hl;

    PREINIT:
    int		attr;

    PPCODE:
    if (text != NULL)
    {
	attr = 0;
	if (hl != NULL)
	    attr = syn_name2attr((char_u *)hl);
	msg_split((char_u *)text, attr);
    }

void
SetOption(line)
    char *line;

    PPCODE:
    if (line != NULL)
	do_set((char_u *)line, 0);
    update_screen(UPD_NOT_VALID);

void
DoCommand(line)
    char *line;

    PPCODE:
    if (line != NULL)
	do_cmdline_cmd((char_u *)line);

void
Eval(str)
    char *str;

    PREINIT:
	char_u *value;
    PPCODE:
	value = eval_to_string((char_u *)str, TRUE, FALSE);
	if (value == NULL)
	{
	    XPUSHs(sv_2mortal(newSViv(0)));
	    XPUSHs(sv_2mortal(newSVpv("", 0)));
	}
	else
	{
	    XPUSHs(sv_2mortal(newSViv(1)));
	    XPUSHs(sv_2mortal(newSVpv((char *)value, 0)));
	    vim_free(value);
	}

SV*
Blob(SV* sv)
    PREINIT:
    STRLEN	len;
    char	*s;
    unsigned	i;
    char	buf[3];
    SV*		newsv;

    CODE:
    s = SvPVbyte(sv, len);
    newsv = newSVpv("0z", 2);
    for (i = 0; i < len; i++)
    {
	sprintf(buf, "%02X", (unsigned char)(s[i]));
	sv_catpvn(newsv, buf, 2);
    }
    RETVAL = newsv;
    OUTPUT:
    RETVAL

void
Buffers(...)

    PREINIT:
    buf_T *vimbuf;
    int i, b;

    PPCODE:
    if (items == 0)
    {
	if (GIMME_V == G_SCALAR)
	{
	    i = 0;
	    FOR_ALL_BUFFERS(vimbuf)
		++i;

	    XPUSHs(sv_2mortal(newSViv(i)));
	}
	else
	{
	    FOR_ALL_BUFFERS(vimbuf)
		XPUSHs(sv_2mortal(newBUFrv(newSV(0), vimbuf)));
	}
    }
    else
    {
	for (i = 0; i < items; i++)
	{
	    SV *sv = ST(i);
	    if (SvIOK(sv))
		b = (int) SvIV(ST(i));
	    else
	    {
		char_u *pat;
		STRLEN len;

		pat = (char_u *)SvPV(sv, len);
		++emsg_off;
		b = buflist_findpat(pat, pat + len, TRUE, FALSE, FALSE);
		--emsg_off;
	    }

	    if (b >= 0)
	    {
		vimbuf = buflist_findnr(b);
		if (vimbuf)
		    XPUSHs(sv_2mortal(newBUFrv(newSV(0), vimbuf)));
	    }
	}
    }

void
Windows(...)

    PREINIT:
    win_T   *vimwin;
    int	    i, w;

    PPCODE:
    if (items == 0)
    {
	if (GIMME_V == G_SCALAR)
	    XPUSHs(sv_2mortal(newSViv(win_count())));
	else
	{
	    FOR_ALL_WINDOWS(vimwin)
		XPUSHs(sv_2mortal(newWINrv(newSV(0), vimwin)));
	}
    }
    else
    {
	for (i = 0; i < items; i++)
	{
	    w = (int) SvIV(ST(i));
	    vimwin = win_find_nr(w);
	    if (vimwin)
		XPUSHs(sv_2mortal(newWINrv(newSV(0), vimwin)));
	}
    }

MODULE = VIM	    PACKAGE = VIWIN

void
DESTROY(win)
    VIWIN win

    CODE:
    if (win_valid(win))
	win->w_perl_private = 0;

SV *
Buffer(win)
    VIWIN win

    CODE:
    if (!win_valid(win))
	win = curwin;
    RETVAL = newBUFrv(newSV(0), win->w_buffer);
    OUTPUT:
    RETVAL

void
SetHeight(win, height)
    VIWIN win
    int height;

    PREINIT:
    win_T *savewin;

    PPCODE:
    if (!win_valid(win))
	win = curwin;
    savewin = curwin;
    curwin = win;
    win_setheight(height);
    curwin = savewin;

void
Cursor(win, ...)
    VIWIN win

    PPCODE:
    if (items == 1)
    {
      EXTEND(sp, 2);
      if (!win_valid(win))
	  win = curwin;
      PUSHs(sv_2mortal(newSViv(win->w_cursor.lnum)));
      PUSHs(sv_2mortal(newSViv(win->w_cursor.col)));
    }
    else if (items == 3)
    {
      int lnum, col;

      if (!win_valid(win))
	  win = curwin;
      lnum = (int) SvIV(ST(1));
      col = (int) SvIV(ST(2));
      win->w_cursor.lnum = lnum;
      win->w_cursor.col = col;
      win->w_set_curswant = TRUE;
      check_cursor();		    /* put cursor on an existing line */
      update_screen(UPD_NOT_VALID);
    }

MODULE = VIM	    PACKAGE = VIBUF

void
DESTROY(vimbuf)
    VIBUF vimbuf;

    CODE:
    if (buf_valid(vimbuf))
	vimbuf->b_perl_private = 0;

void
Name(vimbuf)
    VIBUF vimbuf;

    PPCODE:
    if (!buf_valid(vimbuf))
	vimbuf = curbuf;
    /* No file name returns an empty string */
    if (vimbuf->b_fname == NULL)
	XPUSHs(sv_2mortal(newSVpv("", 0)));
    else
	XPUSHs(sv_2mortal(newSVpv((char *)vimbuf->b_fname, 0)));

void
Number(vimbuf)
    VIBUF vimbuf;

    PPCODE:
    if (!buf_valid(vimbuf))
	vimbuf = curbuf;
    XPUSHs(sv_2mortal(newSViv(vimbuf->b_fnum)));

void
Count(vimbuf)
    VIBUF vimbuf;

    PPCODE:
    if (!buf_valid(vimbuf))
	vimbuf = curbuf;
    XPUSHs(sv_2mortal(newSViv(vimbuf->b_ml.ml_line_count)));

void
Get(vimbuf, ...)
    VIBUF vimbuf;

    PREINIT:
    char_u *line;
    int i;
    long lnum;
    PPCODE:
    if (buf_valid(vimbuf))
    {
	for (i = 1; i < items; i++)
	{
	    lnum = (long) SvIV(ST(i));
	    if (lnum > 0 && lnum <= vimbuf->b_ml.ml_line_count)
	    {
		line = ml_get_buf(vimbuf, lnum, FALSE);
		XPUSHs(sv_2mortal(newSVpv((char *)line, 0)));
	    }
	}
    }

void
Set(vimbuf, ...)
    VIBUF vimbuf;

    PREINIT:
    int i;
    long lnum;
    char *line;
    PPCODE:
    if (buf_valid(vimbuf))
    {
	if (items < 3)
	    croak("Usage: VIBUF::Set(vimbuf, lnum, @lines)");

	lnum = (long) SvIV(ST(1));
	for(i = 2; i < items; i++, lnum++)
	{
	    line = SvPV(ST(i),PL_na);
	    if (lnum > 0 && lnum <= vimbuf->b_ml.ml_line_count && line != NULL)
	    {
		aco_save_T	aco;

		/* Set curwin/curbuf for "vimbuf" and save some things. */
		aucmd_prepbuf(&aco, vimbuf);
		if (curbuf == vimbuf)
		{
		    /* Only when a window was found. */
		    if (u_savesub(lnum) == OK)
		    {
			ml_replace(lnum, (char_u *)line, TRUE);
			changed_bytes(lnum, 0);
		    }

		    /* restore curwin/curbuf and a few other things */
		    aucmd_restbuf(&aco);
		    /* Careful: autocommands may have made "vimbuf" invalid! */
		}
	    }
	}
    }

void
Delete(vimbuf, ...)
    VIBUF vimbuf;

    PREINIT:
    long i, lnum = 0, count = 0;
    PPCODE:
    if (buf_valid(vimbuf))
    {
	if (items == 2)
	{
	    lnum = (long) SvIV(ST(1));
	    count = 1;
	}
	else if (items == 3)
	{
	    lnum = (long) SvIV(ST(1));
	    count = (long) 1 + SvIV(ST(2)) - lnum;
	    if (count == 0)
		count = 1;
	    if (count < 0)
	    {
		lnum -= count;
		count = -count;
	    }
	}
	if (items >= 2)
	{
	    for (i = 0; i < count; i++)
	    {
		if (lnum > 0 && lnum <= vimbuf->b_ml.ml_line_count)
		{
		    aco_save_T	aco;

		    /* set curwin/curbuf for "vimbuf" and save some things */
		    aucmd_prepbuf(&aco, vimbuf);
		    if (curbuf == vimbuf)
		    {
			/* Only when a window was found. */
			if (u_savedel(lnum, 1) == OK)
			{
			    ml_delete(lnum);
			    check_cursor();
			    deleted_lines_mark(lnum, 1L);
			}

			/* restore curwin/curbuf and a few other things */
			aucmd_restbuf(&aco);
			/* Careful: autocommands may have made "vimbuf"
			 * invalid! */
		    }

		    update_curbuf(UPD_VALID);
		}
	    }
	}
    }

void
Append(vimbuf, ...)
    VIBUF vimbuf;

    PREINIT:
    int		i;
    long	lnum;
    char	*line;
    PPCODE:
    if (buf_valid(vimbuf))
    {
	if (items < 3)
	    croak("Usage: VIBUF::Append(vimbuf, lnum, @lines)");

	lnum = (long) SvIV(ST(1));
	for (i = 2; i < items; i++, lnum++)
	{
	    line = SvPV(ST(i),PL_na);
	    if (lnum >= 0 && lnum <= vimbuf->b_ml.ml_line_count && line != NULL)
	    {
		aco_save_T	aco;

		/* set curwin/curbuf for "vimbuf" and save some things */
		aucmd_prepbuf(&aco, vimbuf);
		if (curbuf == vimbuf)
		{
		    /* Only when a window for "vimbuf" was found. */
		    if (u_inssub(lnum + 1) == OK)
		    {
			ml_append(lnum, (char_u *)line, (colnr_T)0, FALSE);
			appended_lines_mark(lnum, 1L);
		    }

		    /* restore curwin/curbuf and a few other things */
		    aucmd_restbuf(&aco);
		    /* Careful: autocommands may have made "vimbuf" invalid! */
		    }

		update_curbuf(UPD_VALID);
	    }
	}
    }

#ifdef __GNUC__
# pragma GCC diagnostic pop
#endif
