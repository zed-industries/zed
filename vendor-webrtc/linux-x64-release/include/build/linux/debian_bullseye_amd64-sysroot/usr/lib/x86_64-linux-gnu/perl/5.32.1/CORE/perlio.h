/*    perlio.h
 *
 *    Copyright (C) 1996, 1997, 1999, 2000, 2001, 2002, 2003,
 *    2004, 2005, 2006, 2007, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#ifndef PERLIO_H_
#define PERLIO_H_
/*
  Interface for perl to IO functions.
  There is a hierarchy of Configure determined #define controls:
   USE_STDIO   - No longer available via Configure.  Formerly forced
                 PerlIO_xxx() to be #define-d onto stdio functions.
                 Now generates compile-time error.

   USE_PERLIO  - The primary Configure variable that enables PerlIO.
                 PerlIO_xxx() are real functions
                 defined in perlio.c which implement extra functionality
                 required for utf8 support.

*/

#ifndef USE_PERLIO
# define USE_STDIO
#endif

#ifdef USE_STDIO
#  error "stdio is no longer supported as the default base layer -- use perlio."
#endif

/* --------------------  End of Configure controls ---------------------------- */

/*
 * Although we may not want stdio to be used including <stdio.h> here
 * avoids issues where stdio.h has strange side effects
 */
#include <stdio.h>

#if defined(USE_64_BIT_STDIO) && defined(HAS_FTELLO) && !defined(USE_FTELL64)
#define ftell ftello
#endif

#if defined(USE_64_BIT_STDIO) && defined(HAS_FSEEKO) && !defined(USE_FSEEK64)
#define fseek fseeko
#endif

/* BS2000 includes are sometimes a bit non standard :-( */
#if defined(POSIX_BC) && defined(O_BINARY) && !defined(O_TEXT)
#undef O_BINARY
#endif

#ifndef PerlIO
/* ----------- PerlIO implementation ---------- */
/* PerlIO not #define-d to something else - define the implementation */

typedef struct _PerlIO PerlIOl;
typedef struct _PerlIO_funcs PerlIO_funcs;
typedef PerlIOl *PerlIO;
#define PerlIO PerlIO
#define PERLIO_LAYERS 1

/* PERLIO_FUNCS_CONST is now on by default for efficiency, PERLIO_FUNCS_CONST
   can be removed 1 day once stable & then PerlIO vtables are permanently RO */
#ifdef PERLIO_FUNCS_CONST
#define PERLIO_FUNCS_DECL(funcs) const PerlIO_funcs funcs
#define PERLIO_FUNCS_CAST(funcs) (PerlIO_funcs*)(funcs)
#else
#define PERLIO_FUNCS_DECL(funcs) PerlIO_funcs funcs
#define PERLIO_FUNCS_CAST(funcs) (funcs)
#endif

PERL_CALLCONV void PerlIO_define_layer(pTHX_ PerlIO_funcs *tab);
PERL_CALLCONV PerlIO_funcs *PerlIO_find_layer(pTHX_ const char *name,
                                              STRLEN len,
				              int load);
PERL_CALLCONV PerlIO *PerlIO_push(pTHX_ PerlIO *f, PERLIO_FUNCS_DECL(*tab),
			          const char *mode, SV *arg);
PERL_CALLCONV void PerlIO_pop(pTHX_ PerlIO *f);
PERL_CALLCONV AV* PerlIO_get_layers(pTHX_ PerlIO *f);
PERL_CALLCONV void PerlIO_clone(pTHX_ PerlInterpreter *proto,
                                CLONE_PARAMS *param);

#endif				/* PerlIO */

/* ----------- End of implementation choices  ---------- */

/* We now need to determine  what happens if source trys to use stdio.
 * There are three cases based on PERLIO_NOT_STDIO which XS code
 * can set how it wants.
 */

#ifdef PERL_CORE
/* Make a choice for perl core code
   - currently this is set to try and catch lingering raw stdio calls.
     This is a known issue with some non UNIX ports which still use
     "native" stdio features.
*/
#  ifndef PERLIO_NOT_STDIO
#    define PERLIO_NOT_STDIO 1
#  endif
#else
#  ifndef PERLIO_NOT_STDIO
#    define PERLIO_NOT_STDIO 0
#  endif
#endif

#ifdef PERLIO_NOT_STDIO
#if PERLIO_NOT_STDIO
/*
 * PERLIO_NOT_STDIO #define'd as 1
 * Case 1: Strong denial of stdio - make all stdio calls (we can think of) errors
 */
#include "nostdio.h"
#else				/* if PERLIO_NOT_STDIO */
/*
 * PERLIO_NOT_STDIO #define'd as 0
 * Case 2: Declares that both PerlIO and stdio can be used
 */
#endif				/* if PERLIO_NOT_STDIO */
#else				/* ifdef PERLIO_NOT_STDIO */
/*
 * PERLIO_NOT_STDIO not defined
 * Case 3: Try and fake stdio calls as PerlIO calls
 */
#include "fakesdio.h"
#endif				/* ifndef PERLIO_NOT_STDIO */

/* ----------- fill in things that have not got #define'd  ---------- */

#ifndef Fpos_t
#define Fpos_t Off_t
#endif

#ifndef EOF
#define EOF (-1)
#endif

/* This is to catch case with no stdio */
#ifndef BUFSIZ
#define BUFSIZ 1024
#endif

/* The default buffer size for the perlio buffering layer */
#ifndef PERLIOBUF_DEFAULT_BUFSIZ
#define PERLIOBUF_DEFAULT_BUFSIZ (BUFSIZ > 8192 ? BUFSIZ : 8192)
#endif

#ifndef SEEK_SET
#define SEEK_SET 0
#endif

#ifndef SEEK_CUR
#define SEEK_CUR 1
#endif

#ifndef SEEK_END
#define SEEK_END 2
#endif

#define PERLIO_DUP_CLONE	1
#define PERLIO_DUP_FD		2

/* --------------------- Now prototypes for functions --------------- */

START_EXTERN_C
#ifndef __attribute__format__
#  ifdef HASATTRIBUTE_FORMAT
#    define __attribute__format__(x,y,z) __attribute__((format(x,y,z)))
#  else
#    define __attribute__format__(x,y,z)
#  endif
#endif
#ifndef PerlIO_init
PERL_CALLCONV void PerlIO_init(pTHX);
#endif
#ifndef PerlIO_stdoutf
PERL_CALLCONV int PerlIO_stdoutf(const char *, ...)
    __attribute__format__(__printf__, 1, 2);
#endif
#ifndef PerlIO_puts
PERL_CALLCONV int PerlIO_puts(PerlIO *, const char *);
#endif
#ifndef PerlIO_open
PERL_CALLCONV PerlIO *PerlIO_open(const char *, const char *);
#endif
#ifndef PerlIO_openn
PERL_CALLCONV PerlIO *PerlIO_openn(pTHX_ const char *layers, const char *mode,
				   int fd, int imode, int perm, PerlIO *old,
				   int narg, SV **arg);
#endif
#ifndef PerlIO_eof
PERL_CALLCONV int PerlIO_eof(PerlIO *);
#endif
#ifndef PerlIO_error
PERL_CALLCONV int PerlIO_error(PerlIO *);
#endif
#ifndef PerlIO_clearerr
PERL_CALLCONV void PerlIO_clearerr(PerlIO *);
#endif
#ifndef PerlIO_getc
PERL_CALLCONV int PerlIO_getc(PerlIO *);
#endif
#ifndef PerlIO_putc
PERL_CALLCONV int PerlIO_putc(PerlIO *, int);
#endif
#ifndef PerlIO_ungetc
PERL_CALLCONV int PerlIO_ungetc(PerlIO *, int);
#endif
#ifndef PerlIO_fdopen
PERL_CALLCONV PerlIO *PerlIO_fdopen(int, const char *);
#endif
#ifndef PerlIO_importFILE
PERL_CALLCONV PerlIO *PerlIO_importFILE(FILE *, const char *);
#endif
#ifndef PerlIO_exportFILE
PERL_CALLCONV FILE *PerlIO_exportFILE(PerlIO *, const char *);
#endif
#ifndef PerlIO_findFILE
PERL_CALLCONV FILE *PerlIO_findFILE(PerlIO *);
#endif
#ifndef PerlIO_releaseFILE
PERL_CALLCONV void PerlIO_releaseFILE(PerlIO *, FILE *);
#endif
#ifndef PerlIO_read
PERL_CALLCONV SSize_t PerlIO_read(PerlIO *, void *, Size_t);
#endif
#ifndef PerlIO_unread
PERL_CALLCONV SSize_t PerlIO_unread(PerlIO *, const void *, Size_t);
#endif
#ifndef PerlIO_write
PERL_CALLCONV SSize_t PerlIO_write(PerlIO *, const void *, Size_t);
#endif
#ifndef PerlIO_setlinebuf
PERL_CALLCONV void PerlIO_setlinebuf(PerlIO *);
#endif
#ifndef PerlIO_printf
PERL_CALLCONV int PerlIO_printf(PerlIO *, const char *, ...)
    __attribute__format__(__printf__, 2, 3);
#endif
#ifndef PerlIO_vprintf
PERL_CALLCONV int PerlIO_vprintf(PerlIO *, const char *, va_list);
#endif
#ifndef PerlIO_tell
PERL_CALLCONV Off_t PerlIO_tell(PerlIO *);
#endif
#ifndef PerlIO_seek
PERL_CALLCONV int PerlIO_seek(PerlIO *, Off_t, int);
#endif
#ifndef PerlIO_rewind
PERL_CALLCONV void PerlIO_rewind(PerlIO *);
#endif
#ifndef PerlIO_has_base
PERL_CALLCONV int PerlIO_has_base(PerlIO *);
#endif
#ifndef PerlIO_has_cntptr
PERL_CALLCONV int PerlIO_has_cntptr(PerlIO *);
#endif
#ifndef PerlIO_fast_gets
PERL_CALLCONV int PerlIO_fast_gets(PerlIO *);
#endif
#ifndef PerlIO_canset_cnt
PERL_CALLCONV int PerlIO_canset_cnt(PerlIO *);
#endif
#ifndef PerlIO_get_ptr
PERL_CALLCONV STDCHAR *PerlIO_get_ptr(PerlIO *);
#endif
#ifndef PerlIO_get_cnt
PERL_CALLCONV SSize_t PerlIO_get_cnt(PerlIO *);
#endif
#ifndef PerlIO_set_cnt
PERL_CALLCONV void PerlIO_set_cnt(PerlIO *, SSize_t);
#endif
#ifndef PerlIO_set_ptrcnt
PERL_CALLCONV void PerlIO_set_ptrcnt(PerlIO *, STDCHAR *, SSize_t);
#endif
#ifndef PerlIO_get_base
PERL_CALLCONV STDCHAR *PerlIO_get_base(PerlIO *);
#endif
#ifndef PerlIO_get_bufsiz
PERL_CALLCONV SSize_t PerlIO_get_bufsiz(PerlIO *);
#endif
#ifndef PerlIO_tmpfile
PERL_CALLCONV PerlIO *PerlIO_tmpfile(void);
#endif
#ifndef PerlIO_tmpfile_flags
PERL_CALLCONV PerlIO *PerlIO_tmpfile_flags(int flags);
#endif
#ifndef PerlIO_stdin
PERL_CALLCONV PerlIO *PerlIO_stdin(void);
#endif
#ifndef PerlIO_stdout
PERL_CALLCONV PerlIO *PerlIO_stdout(void);
#endif
#ifndef PerlIO_stderr
PERL_CALLCONV PerlIO *PerlIO_stderr(void);
#endif
#ifndef PerlIO_getpos
PERL_CALLCONV int PerlIO_getpos(PerlIO *, SV *);
#endif
#ifndef PerlIO_setpos
PERL_CALLCONV int PerlIO_setpos(PerlIO *, SV *);
#endif
#ifndef PerlIO_fdupopen
PERL_CALLCONV PerlIO *PerlIO_fdupopen(pTHX_ PerlIO *, CLONE_PARAMS *, int);
#endif
#if !defined(PerlIO_modestr)
PERL_CALLCONV char *PerlIO_modestr(PerlIO *, char *buf);
#endif
#ifndef PerlIO_isutf8
PERL_CALLCONV int PerlIO_isutf8(PerlIO *);
#endif
#ifndef PerlIO_apply_layers
PERL_CALLCONV int PerlIO_apply_layers(pTHX_ PerlIO *f, const char *mode,
				      const char *names);
#endif
#ifndef PerlIO_binmode
PERL_CALLCONV int PerlIO_binmode(pTHX_ PerlIO *f, int iotype, int omode,
			  	 const char *names);
#endif
#ifndef PerlIO_getname
PERL_CALLCONV char *PerlIO_getname(PerlIO *, char *);
#endif

PERL_CALLCONV void PerlIO_destruct(pTHX);

PERL_CALLCONV int PerlIO_intmode2str(int rawmode, char *mode, int *writing);

#ifdef PERLIO_LAYERS
PERL_CALLCONV void PerlIO_cleanup(pTHX);

PERL_CALLCONV void PerlIO_debug(const char *fmt, ...)
    __attribute__format__(__printf__, 1, 2);
typedef struct PerlIO_list_s PerlIO_list_t;

#endif

END_EXTERN_C
#endif				/* PERLIO_H_ */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
