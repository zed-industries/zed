/*    perlvars.h
 *
 *    Copyright (C) 1999, 2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007,
 *    by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
=head1 Global Variables
These variables are global to an entire process.  They are shared between
all interpreters and all threads in a process.  Any variables not documented
here may be changed or removed without notice, so don't use them!
If you feel you really do need to use an unlisted variable, first send email to
L<perl5-porters@perl.org|mailto:perl5-porters@perl.org>.  It may be that
someone there will point out a way to accomplish what you need without using an
internal variable.  But if not, you should get a go-ahead to document and then
use the variable.

=cut
*/

/* Don't forget to re-run regen/embed.pl to propagate changes! */

/* This file describes the "global" variables used by perl
 * This used to be in perl.h directly but we want to abstract out into
 * distinct files which are per-thread, per-interpreter or really global,
 * and how they're initialized.
 *
 * The 'G' prefix is only needed for vars that need appropriate #defines
 * generated in embed*.h.  Such symbols are also used to generate
 * the appropriate export list for win32. */

/* global state */
#if defined(USE_ITHREADS)
PERLVAR(G, op_mutex,	perl_mutex)	/* Mutex for op refcounting */
#endif
PERLVARI(G, curinterp,	PerlInterpreter *, NULL)
					/* currently running interpreter
					 * (initial parent interpreter under
					 * useithreads) */
#if defined(USE_ITHREADS)
PERLVAR(G, thr_key,	perl_key)	/* key to retrieve per-thread struct */
#endif

/* XXX does anyone even use this? */
PERLVARI(G, do_undump,	bool,	FALSE)	/* -u or dump seen? */

#ifndef PERL_USE_SAFE_PUTENV
PERLVARI(G, use_safe_putenv, bool, TRUE)
#endif

#if defined(FAKE_PERSISTENT_SIGNAL_HANDLERS)||defined(FAKE_DEFAULT_SIGNAL_HANDLERS)
PERLVARI(G, sig_handlers_initted, int, 0)
#endif
#ifdef FAKE_PERSISTENT_SIGNAL_HANDLERS
PERLVARA(G, sig_ignoring, SIG_SIZE, int)
					/* which signals we are ignoring */
#endif
#ifdef FAKE_DEFAULT_SIGNAL_HANDLERS
PERLVARA(G, sig_defaulting, SIG_SIZE, int)
#endif

/* XXX signals are process-wide anyway, so we
 * ignore the implications of this for threading */
#ifndef HAS_SIGACTION
PERLVARI(G, sig_trapped, int,	0)
#endif

#ifndef PERL_MICRO
/* If Perl has to ignore SIGPFE, this is its saved state.
 * See perl.h macros PERL_FPU_INIT and PERL_FPU_{PRE,POST}_EXEC. */
PERLVAR(G, sigfpe_saved, Sighandler_t)

/* these ptrs to functions are to avoid linkage problems; see
 * perl-5.8.0-2193-g5c1546dc48
 */
PERLVARI(G, csighandlerp,  Sighandler_t,  Perl_csighandler)
PERLVARI(G, csighandler1p, Sighandler1_t, Perl_csighandler1)
PERLVARI(G, csighandler3p, Sighandler3_t, Perl_csighandler3)
#endif

/* This is constant on most architectures, a global on OS/2 */
#ifdef OS2
PERLVARI(G, sh_path,	char *, SH_PATH) /* full path of shell */
#endif

#ifdef USE_PERLIO

#  if defined(USE_ITHREADS)
PERLVAR(G, perlio_mutex, perl_mutex)    /* Mutex for perlio fd refcounts */
#  endif

PERLVARI(G, perlio_fd_refcnt, int *, 0) /* Pointer to array of fd refcounts.  */
PERLVARI(G, perlio_fd_refcnt_size, int, 0) /* Size of the array */
PERLVARI(G, perlio_debug_fd, int, 0)	/* the fd to write perlio debug into, 0 means not set yet */
#endif

#ifdef HAS_MMAP
PERLVARI(G, mmap_page_size, IV, 0)
#endif

#if defined(USE_ITHREADS)
PERLVAR(G, hints_mutex, perl_mutex)    /* Mutex for refcounted he refcounting */
PERLVAR(G, env_mutex, perl_mutex)      /* Mutex for accessing ENV */
#  if ! defined(USE_THREAD_SAFE_LOCALE) || defined(TS_W32_BROKEN_LOCALECONV)
PERLVAR(G, locale_mutex, perl_mutex)   /* Mutex for setlocale() changing */
#  endif
#  ifndef USE_THREAD_SAFE_LOCALE
PERLVAR(G, lc_numeric_mutex, perl_mutex)   /* Mutex for switching LC_NUMERIC */
#  endif
#endif

#ifdef USE_POSIX_2008_LOCALE
PERLVAR(G, C_locale_obj, locale_t)
#endif

PERLVARI(G, watch_pvx,	char *, NULL)

/*
=for apidoc AmnU|Perl_check_t *|PL_check

Array, indexed by opcode, of functions that will be called for the "check"
phase of optree building during compilation of Perl code.  For most (but
not all) types of op, once the op has been initially built and populated
with child ops it will be filtered through the check function referenced
by the appropriate element of this array.  The new op is passed in as the
sole argument to the check function, and the check function returns the
completed op.  The check function may (as the name suggests) check the op
for validity and signal errors.  It may also initialise or modify parts of
the ops, or perform more radical surgery such as adding or removing child
ops, or even throw the op away and return a different op in its place.

This array of function pointers is a convenient place to hook into the
compilation process.  An XS module can put its own custom check function
in place of any of the standard ones, to influence the compilation of a
particular type of op.  However, a custom check function must never fully
replace a standard check function (or even a custom check function from
another module).  A module modifying checking must instead B<wrap> the
preexisting check function.  A custom check function must be selective
about when to apply its custom behaviour.  In the usual case where
it decides not to do anything special with an op, it must chain the
preexisting op function.  Check functions are thus linked in a chain,
with the core's base checker at the end.

For thread safety, modules should not write directly to this array.
Instead, use the function L</wrap_op_checker>.

=for apidoc Amn|enum perl_phase|PL_phase

A value that indicates the current Perl interpreter's phase. Possible values
include C<PERL_PHASE_CONSTRUCT>, C<PERL_PHASE_START>, C<PERL_PHASE_CHECK>,
C<PERL_PHASE_INIT>, C<PERL_PHASE_RUN>, C<PERL_PHASE_END>, and
C<PERL_PHASE_DESTRUCT>.

For example, the following determines whether the interpreter is in
global destruction:

    if (PL_phase == PERL_PHASE_DESTRUCT) {
        // we are in global destruction
    }

C<PL_phase> was introduced in Perl 5.14; in prior perls you can use
C<PL_dirty> (boolean) to determine whether the interpreter is in global
destruction. (Use of C<PL_dirty> is discouraged since 5.14.)

=cut
*/

#if defined(USE_ITHREADS)
PERLVAR(G, check_mutex,	perl_mutex)	/* Mutex for PL_check */
#endif
#ifdef PERL_GLOBAL_STRUCT 
PERLVAR(G, ppaddr,	Perl_ppaddr_t *) /* or opcode.h */
PERLVAR(G, check,	Perl_check_t *) /* or opcode.h */
PERLVARA(G, fold_locale, 256, unsigned char) /* or perl.h */
#endif

#ifdef PERL_NEED_APPCTX
PERLVAR(G, appctx,	void*)		/* the application context */
#endif

#if defined(HAS_TIMES) && defined(PERL_NEED_TIMESBASE)
PERLVAR(G, timesbase,	struct tms)
#endif

/* allocate a unique index to every module that calls MY_CXT_INIT */

#ifdef PERL_IMPLICIT_CONTEXT
# ifdef USE_ITHREADS
PERLVAR(G, my_ctx_mutex, perl_mutex)
# endif
PERLVARI(G, my_cxt_index, int,	0)
#endif

/* this is currently set without MUTEX protection, so keep it a type which
 * can be set atomically (ie not a bit field) */
PERLVARI(G, veto_cleanup, int, FALSE)	/* exit without cleanup */

/*
=for apidoc AmnUx|Perl_keyword_plugin_t|PL_keyword_plugin

Function pointer, pointing at a function used to handle extended keywords.
The function should be declared as

	int keyword_plugin_function(pTHX_
		char *keyword_ptr, STRLEN keyword_len,
		OP **op_ptr)

The function is called from the tokeniser, whenever a possible keyword
is seen.  C<keyword_ptr> points at the word in the parser's input
buffer, and C<keyword_len> gives its length; it is not null-terminated.
The function is expected to examine the word, and possibly other state
such as L<%^H|perlvar/%^H>, to decide whether it wants to handle it
as an extended keyword.  If it does not, the function should return
C<KEYWORD_PLUGIN_DECLINE>, and the normal parser process will continue.

If the function wants to handle the keyword, it first must
parse anything following the keyword that is part of the syntax
introduced by the keyword.  See L</Lexer interface> for details.

When a keyword is being handled, the plugin function must build
a tree of C<OP> structures, representing the code that was parsed.
The root of the tree must be stored in C<*op_ptr>.  The function then
returns a constant indicating the syntactic role of the construct that
it has parsed: C<KEYWORD_PLUGIN_STMT> if it is a complete statement, or
C<KEYWORD_PLUGIN_EXPR> if it is an expression.  Note that a statement
construct cannot be used inside an expression (except via C<do BLOCK>
and similar), and an expression is not a complete statement (it requires
at least a terminating semicolon).

When a keyword is handled, the plugin function may also have
(compile-time) side effects.  It may modify C<%^H>, define functions, and
so on.  Typically, if side effects are the main purpose of a handler,
it does not wish to generate any ops to be included in the normal
compilation.  In this case it is still required to supply an op tree,
but it suffices to generate a single null op.

That's how the C<*PL_keyword_plugin> function needs to behave overall.
Conventionally, however, one does not completely replace the existing
handler function.  Instead, take a copy of C<PL_keyword_plugin> before
assigning your own function pointer to it.  Your handler function should
look for keywords that it is interested in and handle those.  Where it
is not interested, it should call the saved plugin function, passing on
the arguments it received.  Thus C<PL_keyword_plugin> actually points
at a chain of handler functions, all of which have an opportunity to
handle keywords, and only the last function in the chain (built into
the Perl core) will normally return C<KEYWORD_PLUGIN_DECLINE>.

For thread safety, modules should not set this variable directly.
Instead, use the function L</wrap_keyword_plugin>.

=cut
*/

#if defined(USE_ITHREADS)
PERLVAR(G, keyword_plugin_mutex, perl_mutex)   /* Mutex for PL_keyword_plugin */
#endif
PERLVARI(G, keyword_plugin, Perl_keyword_plugin_t, Perl_keyword_plugin_standard)

PERLVARI(G, op_sequence, HV *, NULL)	/* dump.c */
PERLVARI(G, op_seq,	UV,	0)	/* dump.c */

#ifdef USE_ITHREADS
PERLVAR(G, dollarzero_mutex, perl_mutex) /* Modifying $0 */
#endif

/* Restricted hashes placeholder value.
   In theory, the contents are never used, only the address.
   In practice, &PL_sv_placeholder is returned by some APIs, and the calling
   code is checking SvOK().  */

PERLVAR(G, sv_placeholder, SV)

#if defined(MYMALLOC) && defined(USE_ITHREADS)
PERLVAR(G, malloc_mutex, perl_mutex)	/* Mutex for malloc */
#endif

PERLVARI(G, hash_seed_set, bool, FALSE)	/* perl.c */
PERLVARA(G, hash_seed, PERL_HASH_SEED_BYTES, unsigned char) /* perl.c and hv.h */
#if defined(PERL_HASH_STATE_BYTES)
PERLVARA(G, hash_state, PERL_HASH_STATE_BYTES, unsigned char) /* perl.c and hv.h */
#endif
#if defined(PERL_USE_SINGLE_CHAR_HASH_CACHE)
PERLVARA(G, hash_chars, (1+256) * sizeof(U32), unsigned char) /* perl.c and hv.h */
#endif

/* The path separator can vary depending on whether we're running under DCL or
 * a Unix shell.
 */
#ifdef __VMS
PERLVAR(G, perllib_sep, char)
#endif

/* Definitions of user-defined \p{} properties, as the subs that define them
 * are only called once */
PERLVARI(G, user_def_props,	HV *, NULL)

#if defined(USE_ITHREADS)
PERLVAR(G, user_def_props_aTHX, PerlInterpreter *)  /* aTHX that user_def_props
                                                       was defined in */
PERLVAR(G, user_prop_mutex, perl_mutex)    /* Mutex for manipulating
                                              PL_user_defined_properties */
#endif

/* these record the best way to perform certain IO operations while
 * atomically setting FD_CLOEXEC. On the first call, a probe is done
 * and the result recorded for use by subsequent calls.
 * In theory these variables aren't thread-safe, but the worst that can
 * happen is that two treads will both do an initial probe
 */
PERLVARI(G, strategy_dup,        int, 0)	/* doio.c */
PERLVARI(G, strategy_dup2,       int, 0)	/* doio.c */
PERLVARI(G, strategy_open,       int, 0)	/* doio.c */
PERLVARI(G, strategy_open3,      int, 0)	/* doio.c */
PERLVARI(G, strategy_mkstemp,    int, 0)	/* doio.c */
PERLVARI(G, strategy_socket,     int, 0)	/* doio.c */
PERLVARI(G, strategy_accept,     int, 0)	/* doio.c */
PERLVARI(G, strategy_pipe,       int, 0)	/* doio.c */
PERLVARI(G, strategy_socketpair, int, 0)	/* doio.c */

#ifdef PERL_IMPLICIT_CONTEXT
#  ifdef PERL_GLOBAL_STRUCT_PRIVATE
/* per-module array of pointers to MY_CXT_KEY constants.
 * It simulates each module having a static my_cxt_index var on builds
 * which don't allow static vars */
PERLVARI(G, my_cxt_keys, const char **, NULL)
PERLVARI(G, my_cxt_keys_size, int,	0)	/* size of PL_my_cxt_keys */
#  endif
#endif
