/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prwin16_h___
#define prwin16_h___

/*
** Condition use of this header on platform.
*/
#if (defined(XP_PC) && !defined(_WIN32) && !defined(XP_OS2) && defined(MOZILLA_CLIENT)) || defined(WIN16)
#include <stdio.h>

PR_BEGIN_EXTERN_C
/*
** Win16 stdio special case.
** To get stdio to work for Win16, all calls to printf() and related
** things must be called from the environment of the .EXE; calls to
** printf() from the .DLL send output to the bit-bucket.
**
** To make sure that PR_fprintf(), and related functions, work correctly,
** the actual stream I/O to stdout, stderr, stdin must be done in the
** .EXE. To do this, a hack is placed in _MD_Write() such that the
** fd for stdio handles results in a call to the .EXE.
**
** file w16stdio.c contains the functions that get called from NSPR
** to do the actual I/O. w16stdio.o must be statically linked with
** any application needing stdio for Win16.
**
** The address of these functions must be made available to the .DLL
** so he can call back to the .EXE. To do this, function
** PR_MD_RegisterW16StdioCallbacks() is called from the .EXE.
** The arguments are the functions defined in w16stdio.c
** At runtime, MD_Write() calls the registered functions, if any
** were registered.
**
** prinit.h contains a macro PR_STDIO_INIT() that calls the registration
** function for Win16; For other platforms, the macro is a No-Op.
**
** Note that stdio is not operational at all on Win16 GUI applications.
** This special case exists to provide stdio capability from the NSPR
** .DLL for command line applications only. NSPR's test cases are
** almost exclusively command line applications.
**
** See also: w16io.c, w16stdio.c
*/
typedef PRInt32 (PR_CALLBACK *PRStdinRead)( void *buf, PRInt32 amount);
typedef PRInt32 (PR_CALLBACK *PRStdoutWrite)( void *buf, PRInt32 amount);
typedef PRInt32 (PR_CALLBACK *PRStderrWrite)( void *buf, PRInt32 amount);

NSPR_API(PRStatus)
PR_MD_RegisterW16StdioCallbacks(
    PRStdinRead inReadf,            /* i: function pointer for stdin read       */
    PRStdoutWrite outWritef,        /* i: function pointer for stdout write     */
    PRStderrWrite errWritef         /* i: function pointer for stderr write     */
);

NSPR_API(PRInt32)
_PL_W16StdioWrite( void *buf, PRInt32 amount );

NSPR_API(PRInt32)
_PL_W16StdioRead( void *buf, PRInt32 amount );

#define PR_STDIO_INIT() PR_MD_RegisterW16StdioCallbacks( \
    _PL_W16StdioRead, _PL_W16StdioWrite, _PL_W16StdioWrite ); \
    PR_INIT_CALLBACKS();

/*
** Win16 hackery.
**
*/
struct PRMethodCallbackStr {
    int     (PR_CALLBACK *auxOutput)(const char *outputString);
    size_t  (PR_CALLBACK *strftime)(char *s, size_t len, const char *fmt, const struct tm *p);
    void *  (PR_CALLBACK *malloc)( size_t size );
    void *  (PR_CALLBACK *calloc)(size_t n, size_t size );
    void *  (PR_CALLBACK *realloc)( void* old_blk, size_t size );
    void    (PR_CALLBACK *free)( void *ptr );
    void *  (PR_CALLBACK *getenv)( const char *name);
    int     (PR_CALLBACK *putenv)( const char *assoc);
    /*    void *  (PR_CALLBACK *perror)( const char *prefix ); */
};

NSPR_API(void) PR_MDRegisterCallbacks(struct PRMethodCallbackStr *);

int PR_CALLBACK _PL_W16CallBackPuts( const char *outputString );
size_t PR_CALLBACK _PL_W16CallBackStrftime(
    char *s,
    size_t len,
    const char *fmt,
    const struct tm *p );
void * PR_CALLBACK _PL_W16CallBackMalloc( size_t size );
void * PR_CALLBACK _PL_W16CallBackCalloc( size_t n, size_t size );
void * PR_CALLBACK _PL_W16CallBackRealloc(
    void *old_blk,
    size_t size );
void   PR_CALLBACK _PL_W16CallBackFree( void *ptr );
void * PR_CALLBACK _PL_W16CallBackGetenv( const char *name );
int PR_CALLBACK _PL_W16CallBackPutenv( const char *assoc );

/*
** Hackery!
**
** These functions are provided as static link points.
** This is to satisfy the quick port of Gromit to NSPR 2.0
** ... Don't do this! ... alas, It may never go away.
**
*/
NSPR_API(int)     PR_MD_printf(const char *, ...);
NSPR_API(void)    PR_MD_exit(int);
NSPR_API(size_t)  PR_MD_strftime(char *, size_t, const char *, const struct tm *);
NSPR_API(int)     PR_MD_sscanf(const char *, const char *, ...);
NSPR_API(void*)   PR_MD_malloc( size_t size );
NSPR_API(void*)   PR_MD_calloc( size_t n, size_t size );
NSPR_API(void*)   PR_MD_realloc( void* old_blk, size_t size );
NSPR_API(void)    PR_MD_free( void *ptr );
NSPR_API(char*)   PR_MD_getenv( const char *name );
NSPR_API(int)     PR_MD_putenv( const char *assoc );
NSPR_API(int)     PR_MD_fprintf(FILE *fPtr, const char *fmt, ...);

#define PR_INIT_CALLBACKS()                         \
    {                                               \
        static struct PRMethodCallbackStr cbf = {   \
            _PL_W16CallBackPuts,                    \
            _PL_W16CallBackStrftime,                \
            _PL_W16CallBackMalloc,                  \
            _PL_W16CallBackCalloc,                  \
            _PL_W16CallBackRealloc,                 \
            _PL_W16CallBackFree,                    \
            _PL_W16CallBackGetenv,                  \
            _PL_W16CallBackPutenv,                  \
        };                                          \
        PR_MDRegisterCallbacks( &cbf );             \
    }


/*
** Get the exception context for Win16 MFC applications threads
*/
NSPR_API(void *) PR_W16GetExceptionContext(void);
/*
** Set the exception context for Win16 MFC applications threads
*/
NSPR_API(void) PR_W16SetExceptionContext(void *context);

PR_END_EXTERN_C
#else
/*
** For platforms other than Win16, define
** PR_STDIO_INIT() as a No-Op.
*/
#define PR_STDIO_INIT()
#endif /* WIN16 || MOZILLA_CLIENT */

#endif /* prwin16_h___ */








