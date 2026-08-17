/*    mydtrace.h
 *
 *    Copyright (C) 2008, 2010, 2011 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 *	Provides macros that wrap the various DTrace probes we use. We add
 *	an extra level of wrapping to encapsulate the _ENABLED tests.
 */

#if defined(USE_DTRACE) && defined(PERL_CORE)

#  include "perldtrace.h"

#  define PERL_DTRACE_PROBE_ENTRY(cv)               \
    if (PERL_SUB_ENTRY_ENABLED())                   \
        Perl_dtrace_probe_call(aTHX_ cv, TRUE);

#  define PERL_DTRACE_PROBE_RETURN(cv)              \
    if (PERL_SUB_ENTRY_ENABLED())                   \
        Perl_dtrace_probe_call(aTHX_ cv, FALSE);

#  define PERL_DTRACE_PROBE_FILE_LOADING(name)      \
    if (PERL_SUB_ENTRY_ENABLED())                   \
        Perl_dtrace_probe_load(aTHX_ name, TRUE);

#  define PERL_DTRACE_PROBE_FILE_LOADED(name)       \
    if (PERL_SUB_ENTRY_ENABLED())                   \
        Perl_dtrace_probe_load(aTHX_ name, FALSE);

#  define PERL_DTRACE_PROBE_OP(op)                  \
    if (PERL_OP_ENTRY_ENABLED())                    \
        Perl_dtrace_probe_op(aTHX_ op);

#  define PERL_DTRACE_PROBE_PHASE(phase)            \
    if (PERL_OP_ENTRY_ENABLED())                    \
        Perl_dtrace_probe_phase(aTHX_ phase);

#else

/* NOPs */
#  define PERL_DTRACE_PROBE_ENTRY(cv)
#  define PERL_DTRACE_PROBE_RETURN(cv)
#  define PERL_DTRACE_PROBE_FILE_LOADING(cv)
#  define PERL_DTRACE_PROBE_FILE_LOADED(cv)
#  define PERL_DTRACE_PROBE_OP(op)
#  define PERL_DTRACE_PROBE_PHASE(phase)

#endif

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
