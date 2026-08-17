/*
Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.
*/

/*
 * Various and sundry Thread-Safe functions used by X11, Motif, and CDE.
 *
 * Use this file in MT-safe code where you would have included
 *	<dirent.h>	for readdir()
 *	<grp.h>		for getgrgid() or getgrnam()
 *	<netdb.h>	for gethostbyname(), gethostbyaddr(), or getservbyname()
 *	<pwd.h>		for getpwnam() or getpwuid()
 *	<string.h>	for strtok()
 *	<time.h>	for asctime(), ctime(), localtime(), or gmtime()
 *	<unistd.h>	for getlogin() or ttyname()
 * or their thread-safe analogs.
 *
 * If you are on a platform that defines XTHREADS but does not have
 * MT-safe system API (e.g. UnixWare) you must define _Xos_processLock
 * and _Xos_processUnlock macros before including this header.
 *
 * For convenience XOS_USE_XLIB_LOCKING or XOS_USE_XT_LOCKING may be defined
 * to obtain either Xlib-only or Xt-based versions of these macros.  These
 * macros won't result in truly thread-safe calls, but they are better than
 * nothing.  If you do not want locking in this situation define
 * XOS_USE_NO_LOCKING.
 *
 * NOTE: On systems lacking appropriate _r functions Gethostbyname(),
 *	Gethostbyaddr(), and Getservbyname() do NOT copy the host or
 *	protocol lists!
 *
 * NOTE: On systems lacking appropriate _r functions Getgrgid() and
 *	Getgrnam() do NOT copy the list of group members!
 *
 * This header is nominally intended to simplify porting X11, Motif, and
 * CDE; it may be useful to other people too.  The structure below is
 * complicated, mostly because P1003.1c (the IEEE POSIX Threads spec)
 * went through lots of drafts, and some vendors shipped systems based
 * on draft API that were changed later.  Unfortunately POSIX did not
 * provide a feature-test macro for distinguishing each of the drafts.
 */

/*
 * This header has several parts.  Search for "Effective prototypes"
 * to locate the beginning of a section.
 */

/* This header can be included multiple times with different defines! */
#ifndef _XOS_R_H_
# define _XOS_R_H_

# include <X11/Xos.h>
# include <X11/Xfuncs.h>

# ifndef X_NOT_POSIX
#  ifdef _POSIX_SOURCE
#   include <limits.h>
#  else
#   define _POSIX_SOURCE
#   include <limits.h>
#   undef _POSIX_SOURCE
#  endif
#  ifndef LINE_MAX
#   define X_LINE_MAX 2048
#  else
#   define X_LINE_MAX LINE_MAX
#  endif
# endif
#endif /* _XOS_R_H */

#ifndef WIN32

#ifdef __cplusplus
extern "C" {
#endif

# if defined(XOS_USE_XLIB_LOCKING)
#  ifndef XAllocIDs /* Xlibint.h does not have multiple include protection */
typedef struct _LockInfoRec *LockInfoPtr;
extern LockInfoPtr _Xglobal_lock;
#  endif
#  ifndef _Xos_isThreadInitialized
#   define _Xos_isThreadInitialized	(_Xglobal_lock)
#  endif
#  if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
#   ifndef XAllocIDs /* Xlibint.h does not have multiple include protection */
#    include <X11/Xfuncproto.h>	/* for NeedFunctionPrototypes */
extern void (*_XLockMutex_fn)(
#    if NeedFunctionPrototypes
    LockInfoPtr	/* lock */, char * /* file */, int /* line */
#    endif
);
extern void (*_XUnlockMutex_fn)(
#    if NeedFunctionPrototypes
    LockInfoPtr	/* lock */, char * /* file */, int /* line */
#    endif
);
#   endif
#   ifndef _Xos_processLock
#    define _Xos_processLock	\
  (_XLockMutex_fn ? (*_XLockMutex_fn)(_Xglobal_lock,__FILE__,__LINE__) : 0)
#   endif
#   ifndef _Xos_processUnlock
#    define _Xos_processUnlock	\
  (_XUnlockMutex_fn ? (*_XUnlockMutex_fn)(_Xglobal_lock,__FILE__,__LINE__) : 0)
#   endif
#  else
#   ifndef XAllocIDs /* Xlibint.h does not have multiple include protection */
#    include <X11/Xfuncproto.h>	/* for NeedFunctionPrototypes */
extern void (*_XLockMutex_fn)(
#    if NeedFunctionPrototypes
    LockInfoPtr	/* lock */
#    endif
);
extern void (*_XUnlockMutex_fn)(
#    if NeedFunctionPrototypes
    LockInfoPtr	/* lock */
#    endif
);
#   endif
#   ifndef _Xos_processLock
#    define _Xos_processLock	\
  (_XLockMutex_fn ? ((*_XLockMutex_fn)(_Xglobal_lock), 0) : 0)
#   endif
#   ifndef _Xos_processUnlock
#    define _Xos_processUnlock	\
  (_XUnlockMutex_fn ? ((*_XUnlockMutex_fn)(_Xglobal_lock), 0) : 0)
#   endif
#  endif
# elif defined(XOS_USE_XT_LOCKING)
#  ifndef _XtThreadsI_h
extern void (*_XtProcessLock)(void);
#  endif
#  ifndef _XtintrinsicP_h
#   include <X11/Xfuncproto.h>	/* for NeedFunctionPrototypes */
extern void XtProcessLock(
#   if NeedFunctionPrototypes
    void
#   endif
);
extern void XtProcessUnlock(
#   if NeedFunctionPrototypes
    void
#   endif
);
#  endif
#  ifndef _Xos_isThreadInitialized
#   define _Xos_isThreadInitialized	_XtProcessLock
#  endif
#  ifndef _Xos_processLock
#   define _Xos_processLock		XtProcessLock()
#  endif
#  ifndef _Xos_processUnlock
#   define _Xos_processUnlock		XtProcessUnlock()
#  endif
# elif defined(XOS_USE_NO_LOCKING)
#  ifndef _Xos_isThreadInitialized
#   define _Xos_isThreadInitialized	0
#  endif
#  ifndef _Xos_processLock
#   define _Xos_processLock		0
#  endif
#  ifndef _Xos_processUnlock
#   define _Xos_processUnlock		0
#  endif
# endif

#endif /* !defined WIN32 */

/*
 * Solaris defines the POSIX thread-safe feature test macro, but
 * uses the older SVR4 thread-safe functions unless the POSIX ones
 * are specifically requested.  Fix the feature test macro.
 */
#if defined(__sun) && defined(_POSIX_THREAD_SAFE_FUNCTIONS) && \
	(_POSIX_C_SOURCE - 0 < 199506L) && !defined(_POSIX_PTHREAD_SEMANTICS)
# undef _POSIX_THREAD_SAFE_FUNCTIONS
#endif

/***** <pwd.h> wrappers *****/

/*
 * Effective prototypes for <pwd.h> wrappers:
 *
 * #define X_INCLUDE_PWD_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xgetpwparams;
 *
 * struct passwd* _XGetpwnam(const char *name, _Xgetpwparams);
 * struct passwd* _XGetpwuid(uid_t uid, _Xgetpwparams);
 */

#if defined(X_INCLUDE_PWD_H) && !defined(_XOS_INCLUDED_PWD_H)
# include <pwd.h>
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_PWDAPI)
#  define XOS_USE_MTSAFE_PWDAPI 1
# endif
#endif

#undef X_NEEDS_PWPARAMS
#if !defined(X_INCLUDE_PWD_H) || defined(_XOS_INCLUDED_PWD_H)
/* Do nothing */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
# if defined(X_NOT_POSIX) && !defined(__i386__) && !defined(SYSV)
extern struct passwd *getpwuid(), *getpwnam();
# endif
typedef int _Xgetpwparams;	/* dummy */
# define _XGetpwuid(u,p)	getpwuid((u))
# define _XGetpwnam(u,p)	getpwnam((u))

#elif !defined(XOS_USE_MTSAFE_PWDAPI) || defined(XNO_MTSAFE_PWDAPI)
/* UnixWare 2.0, or other systems with thread support but no _r API. */
# define X_NEEDS_PWPARAMS
typedef struct {
  struct passwd pws;
  char   pwbuf[1024];
  struct passwd* pwp;
  size_t len;
} _Xgetpwparams;

/*
 * NetBSD and FreeBSD, at least, are missing several of the unixware passwd
 * fields.
 */

#if defined(__NetBSD__) || defined(__FreeBSD__) || defined(__OpenBSD__) || \
    defined(__APPLE__) || defined(__DragonFly__)
static __inline__ void _Xpw_copyPasswd(_Xgetpwparams p)
{
   memcpy(&(p).pws, (p).pwp, sizeof(struct passwd));

   (p).pws.pw_name = (p).pwbuf;
   (p).len = strlen((p).pwp->pw_name);
   strcpy((p).pws.pw_name, (p).pwp->pw_name);

   (p).pws.pw_passwd = (p).pws.pw_name + (p).len + 1;
   (p).len = strlen((p).pwp->pw_passwd);
   strcpy((p).pws.pw_passwd,(p).pwp->pw_passwd);

   (p).pws.pw_class = (p).pws.pw_passwd + (p).len + 1;
   (p).len = strlen((p).pwp->pw_class);
   strcpy((p).pws.pw_class, (p).pwp->pw_class);

   (p).pws.pw_gecos = (p).pws.pw_class + (p).len + 1;
   (p).len = strlen((p).pwp->pw_gecos);
   strcpy((p).pws.pw_gecos, (p).pwp->pw_gecos);

   (p).pws.pw_dir = (p).pws.pw_gecos + (p).len + 1;
   (p).len = strlen((p).pwp->pw_dir);
   strcpy((p).pws.pw_dir, (p).pwp->pw_dir);

   (p).pws.pw_shell = (p).pws.pw_dir + (p).len + 1;
   (p).len = strlen((p).pwp->pw_shell);
   strcpy((p).pws.pw_shell, (p).pwp->pw_shell);

   (p).pwp = &(p).pws;
}

#else
# define _Xpw_copyPasswd(p) \
   (memcpy(&(p).pws, (p).pwp, sizeof(struct passwd)), \
    ((p).pws.pw_name = (p).pwbuf), \
    ((p).len = strlen((p).pwp->pw_name)), \
    strcpy((p).pws.pw_name, (p).pwp->pw_name), \
    ((p).pws.pw_passwd = (p).pws.pw_name + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_passwd)), \
    strcpy((p).pws.pw_passwd,(p).pwp->pw_passwd), \
    ((p).pws.pw_age = (p).pws.pw_passwd + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_age)), \
    strcpy((p).pws.pw_age, (p).pwp->pw_age), \
    ((p).pws.pw_comment = (p).pws.pw_age + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_comment)), \
    strcpy((p).pws.pw_comment, (p).pwp->pw_comment), \
    ((p).pws.pw_gecos = (p).pws.pw_comment + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_gecos)), \
    strcpy((p).pws.pw_gecos, (p).pwp->pw_gecos), \
    ((p).pws.pw_dir = (p).pws.pw_comment + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_dir)), \
    strcpy((p).pws.pw_dir, (p).pwp->pw_dir), \
    ((p).pws.pw_shell = (p).pws.pw_dir + (p).len + 1), \
    ((p).len = strlen((p).pwp->pw_shell)), \
    strcpy((p).pws.pw_shell, (p).pwp->pw_shell), \
    ((p).pwp = &(p).pws), \
    0 )
#endif
# define _XGetpwuid(u,p) \
( (_Xos_processLock), \
  (((p).pwp = getpwuid((u))) ? _Xpw_copyPasswd(p), 0 : 0), \
  (_Xos_processUnlock), \
  (p).pwp )
# define _XGetpwnam(u,p) \
( (_Xos_processLock), \
  (((p).pwp = getpwnam((u))) ? _Xpw_copyPasswd(p), 0 : 0), \
  (_Xos_processUnlock), \
  (p).pwp )

#elif !defined(_POSIX_THREAD_SAFE_FUNCTIONS) && !defined(__APPLE__)
# define X_NEEDS_PWPARAMS
typedef struct {
  struct passwd pws;
  char pwbuf[X_LINE_MAX];
} _Xgetpwparams;
# if defined(_POSIX_REENTRANT_FUNCTIONS) || !defined(SVR4)
#   define _XGetpwuid(u,p) \
((getpwuid_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf)) == -1) ? NULL : &(p).pws)
#   define _XGetpwnam(u,p) \
((getpwnam_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf)) == -1) ? NULL : &(p).pws)
# else /* SVR4 */
#  define _XGetpwuid(u,p) \
((getpwuid_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf)) == NULL) ? NULL : &(p).pws)
#  define _XGetpwnam(u,p) \
((getpwnam_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf)) == NULL) ? NULL : &(p).pws)
# endif /* SVR4 */

#else /* _POSIX_THREAD_SAFE_FUNCTIONS */
# define X_NEEDS_PWPARAMS
typedef struct {
  struct passwd pws;
  char pwbuf[X_LINE_MAX];
  struct passwd* pwp;
} _Xgetpwparams;
typedef int _Xgetpwret;
# define _XGetpwuid(u,p) \
((getpwuid_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf),&(p).pwp) == 0) ? \
 (p).pwp : NULL)
# define _XGetpwnam(u,p) \
((getpwnam_r((u),&(p).pws,(p).pwbuf,sizeof((p).pwbuf),&(p).pwp) == 0) ? \
 (p).pwp : NULL)
#endif /* X_INCLUDE_PWD_H */

#if defined(X_INCLUDE_PWD_H) && !defined(_XOS_INCLUDED_PWD_H)
# define _XOS_INCLUDED_PWD_H
#endif


/***** <netdb.h> wrappers *****/

/*
 * Effective prototypes for <netdb.h> wrappers:
 *
 * NOTE: On systems lacking the appropriate _r functions Gethostbyname(),
 *	Gethostbyaddr(), and Getservbyname() do NOT copy the host or
 *	protocol lists!
 *
 * #define X_INCLUDE_NETDB_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xgethostbynameparams;
 * typedef ... _Xgetservbynameparams;
 *
 * struct hostent* _XGethostbyname(const char* name,_Xgethostbynameparams);
 * struct hostent* _XGethostbyaddr(const char* addr, int len, int type,
 *				   _Xgethostbynameparams);
 * struct servent* _XGetservbyname(const char* name, const char* proto,
 *				 _Xgetservbynameparams);
 */

#undef XTHREADS_NEEDS_BYNAMEPARAMS
#if defined(X_INCLUDE_NETDB_H) && !defined(_XOS_INCLUDED_NETDB_H) \
    && !defined(WIN32)
# include <netdb.h>
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_NETDBAPI)
#  define XOS_USE_MTSAFE_NETDBAPI 1
# endif
#endif

#if !defined(X_INCLUDE_NETDB_H) || defined(_XOS_INCLUDED_NETDB_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xgethostbynameparams; /* dummy */
typedef int _Xgetservbynameparams; /* dummy */
# define _XGethostbyname(h,hp)		gethostbyname((h))
# define _XGethostbyaddr(a,al,t,hp)	gethostbyaddr((a),(al),(t))
# define _XGetservbyname(s,p,sp)	getservbyname((s),(p))

#elif !defined(XOS_USE_MTSAFE_NETDBAPI) || defined(XNO_MTSAFE_NETDBAPI)
/* WARNING:  The h_addr_list and s_aliases values are *not* copied! */

#if defined(__NetBSD__) || defined(__FreeBSD__) || defined(__DragonFly__)
#include <sys/param.h>
#endif

typedef struct {
  struct hostent hent;
  char           h_name[MAXHOSTNAMELEN];
  struct hostent *hptr;
} _Xgethostbynameparams;
typedef struct {
  struct servent sent;
  char           s_name[255];
  char		 s_proto[255];
  struct servent *sptr;
} _Xgetservbynameparams;

# define XTHREADS_NEEDS_BYNAMEPARAMS

# define _Xg_copyHostent(hp) \
   (memcpy(&(hp).hent, (hp).hptr, sizeof(struct hostent)), \
    strcpy((hp).h_name, (hp).hptr->h_name), \
    ((hp).hent.h_name = (hp).h_name), \
    ((hp).hptr = &(hp).hent), \
     0 )
# define _Xg_copyServent(sp) \
   (memcpy(&(sp).sent, (sp).sptr, sizeof(struct servent)), \
    strcpy((sp).s_name, (sp).sptr->s_name), \
    ((sp).sent.s_name = (sp).s_name), \
    strcpy((sp).s_proto, (sp).sptr->s_proto), \
    ((sp).sent.s_proto = (sp).s_proto), \
    ((sp).sptr = &(sp).sent), \
    0 )
# define _XGethostbyname(h,hp) \
   ((_Xos_processLock), \
    (((hp).hptr = gethostbyname((h))) ? _Xg_copyHostent(hp) : 0), \
    (_Xos_processUnlock), \
    (hp).hptr )
# define _XGethostbyaddr(a,al,t,hp) \
   ((_Xos_processLock), \
    (((hp).hptr = gethostbyaddr((a),(al),(t))) ? _Xg_copyHostent(hp) : 0), \
    (_Xos_processUnlock), \
    (hp).hptr )
# define _XGetservbyname(s,p,sp) \
   ((_Xos_processLock), \
    (((sp).sptr = getservbyname((s),(p))) ? _Xg_copyServent(sp) : 0), \
    (_Xos_processUnlock), \
    (sp).sptr )

#elif defined(XUSE_NETDB_R_API)
/*
 * POSIX does not specify _r equivalents for <netdb.h> API, but some
 * vendors provide them anyway.  Use them only when explicitly asked.
 */
# ifdef _POSIX_REENTRANT_FUNCTIONS
#  ifndef _POSIX_THREAD_SAFE_FUNCTIONS
#  endif
# endif
# ifdef _POSIX_THREAD_SAFE_FUNCTIONS
#  define X_POSIX_THREAD_SAFE_FUNCTIONS 1
# endif

# define XTHREADS_NEEDS_BYNAMEPARAMS

# ifndef X_POSIX_THREAD_SAFE_FUNCTIONS
typedef struct {
    struct hostent      hent;
    char                hbuf[X_LINE_MAX];
    int                 herr;
} _Xgethostbynameparams;
typedef struct {
    struct servent      sent;
    char                sbuf[X_LINE_MAX];
} _Xgetservbynameparams;
#  define _XGethostbyname(h,hp) \
  gethostbyname_r((h),&(hp).hent,(hp).hbuf,sizeof((hp).hbuf),&(hp).herr)
#  define _XGethostbyaddr(a,al,t,hp) \
  gethostbyaddr_r((a),(al),(t),&(hp).hent,(hp).hbuf,sizeof((hp).hbuf),&(hp).herr)
#  define _XGetservbyname(s,p,sp) \
  getservbyname_r((s),(p),&(sp).sent,(sp).sbuf,sizeof((sp).sbuf))
# else
typedef struct {
  struct hostent      hent;
  struct hostent_data hdata;
} _Xgethostbynameparams;
typedef struct {
  struct servent      sent;
  struct servent_data sdata;
} _Xgetservbynameparams;
#  define _XGethostbyname(h,hp) \
  (bzero((char*)&(hp).hdata,sizeof((hp).hdata)),	\
   ((gethostbyname_r((h),&(hp).hent,&(hp).hdata) == -1) ? NULL : &(hp).hent))
#  define _XGethostbyaddr(a,al,t,hp) \
  (bzero((char*)&(hp).hdata,sizeof((hp).hdata)),	\
   ((gethostbyaddr_r((a),(al),(t),&(hp).hent,&(hp).hdata) == -1) ? NULL : &(hp).hent))
#  define _XGetservbyname(s,p,sp) \
  (bzero((char*)&(sp).sdata,sizeof((sp).sdata)),	\
   ((getservbyname_r((s),(p),&(sp).sent,&(sp).sdata) == -1) ? NULL : &(sp).sent) )
# endif
# ifdef X_POSIX_THREAD_SAFE_FUNCTIONS
#  undef X_POSIX_THREAD_SAFE_FUNCTIONS
# endif

#else
/* The regular API is assumed to be MT-safe under POSIX. */
typedef int _Xgethostbynameparams; /* dummy */
typedef int _Xgetservbynameparams; /* dummy */
# define _XGethostbyname(h,hp)		gethostbyname((h))
# define _XGethostbyaddr(a,al,t,hp)	gethostbyaddr((a),(al),(t))
# define _XGetservbyname(s,p,sp)	getservbyname((s),(p))
#endif /* X_INCLUDE_NETDB_H */

#if defined(X_INCLUDE_NETDB_H) && !defined(_XOS_INCLUDED_NETDB_H)
# define _XOS_INCLUDED_NETDB_H
#endif


/***** <dirent.h> wrappers *****/

/*
 * Effective prototypes for <dirent.h> wrappers:
 *
 * #define X_INCLUDE_DIRENT_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xreaddirparams;
 *
 * struct dirent *_XReaddir(DIR *dir_pointer, _Xreaddirparams);
 */

#if defined(X_INCLUDE_DIRENT_H) && !defined(_XOS_INCLUDED_DIRENT_H)
# include <sys/types.h>
# if !defined(X_NOT_POSIX) || defined(SYSV)
#  include <dirent.h>
# else
#  include <sys/dir.h>
#  ifndef dirent
#   define dirent direct
#  endif
# endif
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_DIRENTAPI)
#  define XOS_USE_MTSAFE_DIRENTAPI 1
# endif
#endif

#if !defined(X_INCLUDE_DIRENT_H) || defined(_XOS_INCLUDED_DIRENT_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xreaddirparams;	/* dummy */
# define _XReaddir(d,p)	readdir(d)

#elif !defined(XOS_USE_MTSAFE_DIRENTAPI) || defined(XNO_MTSAFE_DIRENTAPI)
/* Systems with thread support but no _r API. */
typedef struct {
  struct dirent *result;
  struct dirent dir_entry;
# ifdef _POSIX_PATH_MAX
  char buf[_POSIX_PATH_MAX];
# elif defined(NAME_MAX)
  char buf[NAME_MAX];
# else
  char buf[255];
# endif
} _Xreaddirparams;

# define _XReaddir(d,p)	\
 ( (_Xos_processLock),						 \
   (((p).result = readdir((d))) ?				 \
    (memcpy(&((p).dir_entry), (p).result, (p).result->d_reclen), \
     ((p).result = &(p).dir_entry), 0) :			 \
    0),								 \
   (_Xos_processUnlock),					 \
   (p).result )

#else
typedef struct {
  struct dirent *result;
  struct dirent dir_entry;
# ifdef _POSIX_PATH_MAX
  char buf[_POSIX_PATH_MAX];
# elif defined(NAME_MAX)
  char buf[NAME_MAX];
# else
  char buf[255];
# endif
} _Xreaddirparams;

# if defined(_POSIX_THREAD_SAFE_FUNCTIONS) || defined(__APPLE__)
/* POSIX final API, returns (int)0 on success. */
#  define _XReaddir(d,p)						\
    (readdir_r((d), &((p).dir_entry), &((p).result)) ? NULL : (p).result)
# elif defined(_POSIX_REENTRANT_FUNCTIONS)
/* POSIX draft API, returns (int)0 on success. */
#  define _XReaddir(d,p)	\
    (readdir_r((d),&((p).dir_entry)) ? NULL : &((p).dir_entry))
# elif defined(SVR4)
/* Pre-POSIX API, returns non-NULL on success. */
#  define _XReaddir(d,p)	(readdir_r((d), &(p).dir_entry))
# else
/* We have no idea what is going on.  Fake it all using process locks. */
#  define _XReaddir(d,p)	\
    ( (_Xos_processLock),						\
      (((p).result = readdir((d))) ?					\
       (memcpy(&((p).dir_entry), (p).result, (p).result->d_reclen),	\
	((p).result = &(p).dir_entry), 0) :				\
       0),								\
      (_Xos_processUnlock),						\
      (p).result )
# endif
#endif /* X_INCLUDE_DIRENT_H */

#if defined(X_INCLUDE_DIRENT_H) && !defined(_XOS_INCLUDED_DIRENT_H)
# define _XOS_INCLUDED_DIRENT_H
#endif


/***** <unistd.h> wrappers *****/

/*
 * Effective prototypes for <unistd.h> wrappers:
 *
 * #define X_INCLUDE_UNISTD_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xgetloginparams;
 * typedef ... _Xttynameparams;
 *
 * char *_XGetlogin(_Xgetloginparams);
 * char *_XTtyname(int, _Xttynameparams);
 */

#if defined(X_INCLUDE_UNISTD_H) && !defined(_XOS_INCLUDED_UNISTD_H)
/* <unistd.h> already included by <X11/Xos.h> */
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_UNISTDAPI)
#  define XOS_USE_MTSAFE_UNISTDAPI 1
# endif
#endif

#if !defined(X_INCLUDE_UNISTD_H) || defined(_XOS_INCLUDED_UNISTD_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xgetloginparams;	/* dummy */
typedef int _Xttynameparams;	/* dummy */
# define _XGetlogin(p)	getlogin()
# define _XTtyname(f)	ttyname((f))

#elif !defined(XOS_USE_MTSAFE_UNISTDAPI) || defined(XNO_MTSAFE_UNISTDAPI)
/* Systems with thread support but no _r API. */
typedef struct {
  char *result;
# if defined(MAXLOGNAME)
  char buf[MAXLOGNAME];
# elif defined(LOGIN_NAME_MAX)
  char buf[LOGIN_NAME_MAX];
# else
  char buf[64];
# endif
} _Xgetloginparams;
typedef struct {
  char *result;
# ifdef TTY_NAME_MAX
  char buf[TTY_NAME_MAX];
# elif defined(_POSIX_TTY_NAME_MAX)
  char buf[_POSIX_TTY_NAME_MAX];
# elif defined(_POSIX_PATH_MAX)
  char buf[_POSIX_PATH_MAX];
# else
  char buf[256];
# endif
} _Xttynameparams;

# define _XGetlogin(p) \
 ( (_Xos_processLock), \
   (((p).result = getlogin()) ? \
    (strncpy((p).buf, (p).result, sizeof((p).buf)), \
     ((p).buf[sizeof((p).buf)-1] = '\0'), \
     ((p).result = (p).buf), 0) : 0), \
   (_Xos_processUnlock), \
   (p).result )
#define _XTtyname(f,p) \
 ( (_Xos_processLock), \
   (((p).result = ttyname(f)) ? \
    (strncpy((p).buf, (p).result, sizeof((p).buf)), \
     ((p).buf[sizeof((p).buf)-1] = '\0'), \
     ((p).result = (p).buf), 0) : 0), \
   (_Xos_processUnlock), \
   (p).result )

#elif defined(_POSIX_THREAD_SAFE_FUNCTIONS) || defined(_POSIX_REENTRANT_FUNCTIONS)
/* POSIX API.
 *
 * extern int getlogin_r(char *, size_t);
 * extern int ttyname_r(int, char *, size_t);
 */
typedef struct {
# if defined(MAXLOGNAME)
  char buf[MAXLOGNAME];
# elif defined(LOGIN_NAME_MAX)
  char buf[LOGIN_NAME_MAX];
# else
  char buf[64];
# endif
} _Xgetloginparams;
typedef struct {
# ifdef TTY_NAME_MAX
  char buf[TTY_NAME_MAX];
# elif defined(_POSIX_TTY_NAME_MAX)
  char buf[_POSIX_TTY_NAME_MAX];
# elif defined(_POSIX_PATH_MAX)
  char buf[_POSIX_PATH_MAX];
# else
  char buf[256];
# endif
} _Xttynameparams;

# define _XGetlogin(p)	(getlogin_r((p).buf, sizeof((p).buf)) ? NULL : (p).buf)
# define _XTtyname(f,p)	\
	(ttyname_r((f), (p).buf, sizeof((p).buf)) ? NULL : (p).buf)

#else
/* Pre-POSIX API.
 *
 * extern char *getlogin_r(char *, size_t);
 * extern char *ttyname_r(int, char *, size_t);
 */
typedef struct {
# if defined(MAXLOGNAME)
  char buf[MAXLOGNAME];
# elif defined(LOGIN_NAME_MAX)
  char buf[LOGIN_NAME_MAX];
# else
  char buf[64];
# endif
} _Xgetloginparams;
typedef struct {
# ifdef TTY_NAME_MAX
  char buf[TTY_NAME_MAX];
# elif defined(_POSIX_TTY_NAME_MAX)
  char buf[_POSIX_TTY_NAME_MAX];
# elif defined(_POSIX_PATH_MAX)
  char buf[_POSIX_PATH_MAX];
# else
  char buf[256];
# endif
} _Xttynameparams;

# define _XGetlogin(p)	getlogin_r((p).buf, sizeof((p).buf))
# define _XTtyname(f,p)	ttyname_r((f), (p).buf, sizeof((p).buf))
#endif /* X_INCLUDE_UNISTD_H */

#if defined(X_INCLUDE_UNISTD_H) && !defined(_XOS_INCLUDED_UNISTD_H)
# define _XOS_INCLUDED_UNISTD_H
#endif


/***** <string.h> wrappers *****/

/*
 * Effective prototypes for <string.h> wrappers:
 *
 * #define X_INCLUDE_STRING_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xstrtokparams;
 *
 * char *_XStrtok(char *, const char*, _Xstrtokparams);
 */

#if defined(X_INCLUDE_STRING_H) && !defined(_XOS_INCLUDED_STRING_H)
/* <string.h> has already been included by <X11/Xos.h> */
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_STRINGAPI)
#  define XOS_USE_MTSAFE_STRINGAPI 1
# endif
#endif

#if !defined(X_INCLUDE_STRING_H) || defined(_XOS_INCLUDED_STRING_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xstrtokparams;	/* dummy */
# define _XStrtok(s1,s2,p) \
 ( p = 0, (void)p, strtok((s1),(s2)) )

#elif !defined(XOS_USE_MTSAFE_STRINGAPI) || defined(XNO_MTSAFE_STRINGAPI)
/* Systems with thread support but no _r API. */
typedef char *_Xstrtokparams;
# define _XStrtok(s1,s2,p) \
 ( (_Xos_processLock), \
   ((p) = strtok((s1),(s2))), \
   (_Xos_processUnlock), \
   (p) )

#else
/* POSIX or pre-POSIX API. */
typedef char * _Xstrtokparams;
# define _XStrtok(s1,s2,p)	strtok_r((s1),(s2),&(p))
#endif /* X_INCLUDE_STRING_H */


/***** <time.h> wrappers *****/

/*
 * Effective prototypes for <time.h> wrappers:
 *
 * #define X_INCLUDE_TIME_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xatimeparams;
 * typedef ... _Xctimeparams;
 * typedef ... _Xgtimeparams;
 * typedef ... _Xltimeparams;
 *
 * char *_XAsctime(const struct tm *, _Xatimeparams);
 * char *_XCtime(const time_t *, _Xctimeparams);
 * struct tm *_XGmtime(const time_t *, _Xgtimeparams);
 * struct tm *_XLocaltime(const time_t *, _Xltimeparams);
 */

#if defined(X_INCLUDE_TIME_H) && !defined(_XOS_INCLUDED_TIME_H)
# include <time.h>
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_TIMEAPI)
#  define XOS_USE_MTSAFE_TIMEAPI 1
# endif
#endif

#if !defined(X_INCLUDE_TIME_H) || defined(_XOS_INCLUDED_TIME_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xatimeparams;	/* dummy */
# define _XAsctime(t,p)		asctime((t))
typedef int _Xctimeparams;	/* dummy */
# define _XCtime(t,p)		ctime((t))
typedef int _Xgtimeparams;	/* dummy */
# define _XGmtime(t,p)		gmtime((t))
typedef int _Xltimeparams;	/* dummy */
# define _XLocaltime(t,p)	localtime((t))

#elif !defined(XOS_USE_MTSAFE_TIMEAPI) || defined(XNO_MTSAFE_TIMEAPI)
/* Systems with thread support but no _r API. */
typedef struct {
# ifdef TIMELEN
  char buf[TIMELEN];
# else
  char buf[26];
# endif
  char *result;
} _Xctimeparams, _Xatimeparams;
typedef struct {
  struct tm buf;
  struct tm *result;
} _Xgtimeparams, _Xltimeparams;
# define _XAsctime(t,p) \
 ( (_Xos_processLock), \
   (((p).result = asctime((t))) ? \
    (strncpy((p).buf, (p).result, sizeof((p).buf)), (p).result = &(p).buf) : \
    0), \
   (_Xos_processUnlock), \
   (p).result )
# define _XCtime(t,p) \
 ( (_Xos_processLock), \
   (((p).result = ctime((t))) ? \
    (strncpy((p).buf, (p).result, sizeof((p).buf)), (p).result = &(p).buf) : \
    0), \
   (_Xos_processUnlock), \
   (p).result )
# define _XGmtime(t,p) \
 ( (_Xos_processLock), \
   (((p).result = gmtime(t)) ? \
    (memcpy(&(p).buf, (p).result, sizeof((p).buf)), (p).result = &(p).buf) : \
    0), \
   (_Xos_processUnlock), \
   (p).result )
# define _XLocaltime(t,p) \
 ( (_Xos_processLock), \
   (((p).result = localtime(t)) ? \
    (memcpy(&(p).buf, (p).result, sizeof((p).buf)), (p).result = &(p).buf) : \
    0), \
   (_Xos_processUnlock), \
   (p).result )

#elif !defined(_POSIX_THREAD_SAFE_FUNCTIONS) &&  defined(hpV4)
/* Returns (int)0 on success.
 *
 * extern int asctime_r(const struct tm *timeptr, char *buffer, int buflen);
 * extern int ctime_r(const time_t *timer, char *buffer, int buflen);
 * extern int gmtime_r(const time_t *timer, struct tm *result);
 * extern int localtime_r(const time_t *timer, struct tm *result);
 */
# ifdef TIMELEN
typedef char _Xatimeparams[TIMELEN];
typedef char _Xctimeparams[TIMELEN];
# else
typedef char _Xatimeparams[26];
typedef char _Xctimeparams[26];
# endif
typedef struct tm _Xgtimeparams;
typedef struct tm _Xltimeparams;
# define _XAsctime(t,p)		(asctime_r((t),(p),sizeof((p))) ? NULL : (p))
# define _XCtime(t,p)		(ctime_r((t),(p),sizeof((p))) ? NULL : (p))
# define _XGmtime(t,p)		(gmtime_r((t),&(p)) ? NULL : &(p))
# define _XLocaltime(t,p)	(localtime_r((t),&(p)) ? NULL : &(p))

#elif !defined(_POSIX_THREAD_SAFE_FUNCTIONS) && defined(__sun)
/* Returns NULL on failure.  Solaris 2.5
 *
 * extern char *asctime_r(const struct tm *tm,char *buf, int buflen);
 * extern char *ctime_r(const time_t *clock, char *buf, int buflen);
 * extern struct tm *gmtime_r(const time_t *clock, struct tm *res);
 * extern struct tm *localtime_r(const time_t *clock, struct tm *res);
 */
# ifdef TIMELEN
typedef char _Xatimeparams[TIMELEN];
typedef char _Xctimeparams[TIMELEN];
# else
typedef char _Xatimeparams[26];
typedef char _Xctimeparams[26];
# endif
typedef struct tm _Xgtimeparams;
typedef struct tm _Xltimeparams;
# define _XAsctime(t,p)		asctime_r((t),(p),sizeof((p)))
# define _XCtime(t,p)		ctime_r((t),(p),sizeof((p)))
# define _XGmtime(t,p)		gmtime_r((t),&(p))
# define _XLocaltime(t,p)	localtime_r((t),&(p))

#else /* defined(_POSIX_THREAD_SAFE_FUNCTIONS) */
/* POSIX final API.
 * extern char *asctime_r(const struct tm *timeptr, char *buffer);
 * extern char *ctime_r(const time_t *timer, char *buffer);
 * extern struct tm *gmtime_r(const time_t *timer, struct tm *result);
 * extern struct tm *localtime_r(const time_t *timer, struct tm *result);
 */
# ifdef TIMELEN
typedef char _Xatimeparams[TIMELEN];
typedef char _Xctimeparams[TIMELEN];
# else
typedef char _Xatimeparams[26];
typedef char _Xctimeparams[26];
# endif
typedef struct tm _Xgtimeparams;
typedef struct tm _Xltimeparams;
# define _XAsctime(t,p)		asctime_r((t),(p))
# define _XCtime(t,p)		ctime_r((t),(p))
# define _XGmtime(t,p)		gmtime_r((t),&(p))
# define _XLocaltime(t,p)	localtime_r((t),&(p))
#endif /* X_INCLUDE_TIME_H */

#if defined(X_INCLUDE_TIME_H) && !defined(_XOS_INCLUDED_TIME_H)
# define _XOS_INCLUDED_TIME_H
#endif


/***** <grp.h> wrappers *****/

/*
 * Effective prototypes for <grp.h> wrappers:
 *
 * NOTE: On systems lacking appropriate _r functions Getgrgid() and
 *	Getgrnam() do NOT copy the list of group members!
 *
 * Remember that fgetgrent(), setgrent(), getgrent(), and endgrent()
 * are not included in POSIX.
 *
 * #define X_INCLUDE_GRP_H
 * #define XOS_USE_..._LOCKING
 * #include <X11/Xos_r.h>
 *
 * typedef ... _Xgetgrparams;
 *
 * struct group *_XGetgrgid(gid_t, _Xgetgrparams);
 * struct group *_XGetgrnam(const char *, _Xgetgrparams);
 */

#if defined(X_INCLUDE_GRP_H) && !defined(_XOS_INCLUDED_GRP_H)
# include <grp.h>
# if defined(XUSE_MTSAFE_API) || defined(XUSE_MTSAFE_GRPAPI)
#  define XOS_USE_MTSAFE_GRPAPI 1
# endif
#endif

#if !defined(X_INCLUDE_GRP_H) || defined(_XOS_INCLUDED_GRP_H)
/* Do nothing. */

#elif !defined(XTHREADS) && !defined(X_FORCE_USE_MTSAFE_API)
/* Use regular, unsafe API. */
typedef int _Xgetgrparams;	/* dummy */
#define _XGetgrgid(g,p)	getgrgid((g))
#define _XGetgrnam(n,p)	getgrnam((n))

#elif !defined(XOS_USE_MTSAFE_GRPAPI) || defined(XNO_MTSAFE_GRPAPI)
/* Systems with thread support but no _r API.  UnixWare 2.0. */
typedef struct {
  struct group grp;
  char buf[X_LINE_MAX];	/* Should be sysconf(_SC_GETGR_R_SIZE_MAX)? */
  struct group *pgrp;
  size_t len;
} _Xgetgrparams;
#ifdef SVR4
/* Copy the gr_passwd field too. */
# define _Xgrp_copyGroup(p) \
 ( memcpy(&(p).grp, (p).pgrp, sizeof(struct group)), \
   ((p).grp.gr_name = (p).buf), \
   ((p).len = strlen((p).pgrp->gr_name)), \
   strcpy((p).grp.gr_name, (p).pgrp->gr_name), \
   ((p).grp.gr_passwd = (p).grp.gr_name + (p).len + 1), \
   ((p).pgrp = &(p).grp), \
   0 )
#else
# define _Xgrp_copyGroup(p) \
 ( memcpy(&(p).grp, (p).pgrp, sizeof(struct group)), \
   ((p).grp.gr_name = (p).buf), \
   strcpy((p).grp.gr_name, (p).pgrp->gr_name), \
   ((p).pgrp = &(p).grp), \
   0 )
#endif
#define _XGetgrgid(g,p) \
 ( (_Xos_processLock), \
   (((p).pgrp = getgrgid((g))) ? _Xgrp_copyGroup(p) : 0), \
   (_Xos_processUnlock), \
   (p).pgrp )
#define _XGetgrnam(n,p) \
 ( (_Xos_processLock), \
   (((p).pgrp = getgrnam((n))) ? _Xgrp_copyGroup(p) : 0), \
   (_Xos_processUnlock), \
   (p).pgrp )

#elif !defined(_POSIX_THREAD_SAFE_FUNCTIONS) && defined(__sun)
/* Non-POSIX API.  Solaris.
 *
 * extern struct group *getgrgid_r(gid_t, struct group *, char *, int);
 * extern struct group *getgrnam_r(const char *, struct group *, char *, int);
 */
typedef struct {
  struct group grp;
  char buf[X_LINE_MAX];	/* Should be sysconf(_SC_GETGR_R_SIZE_MAX)? */
} _Xgetgrparams;
#define _XGetgrgid(g,p)	getgrgid_r((g), &(p).grp, (p).buf, sizeof((p).buf))
#define _XGetgrnam(n,p)	getgrnam_r((n), &(p).grp, (p).buf, sizeof((p).buf))

#elif !defined(_POSIX_THREAD_SAFE_FUNCTIONS)
/* Non-POSIX API.
 * extern int getgrgid_r(gid_t, struct group *, char *, int);
 * extern int getgrnam_r(const char *, struct group *, char *, int);
 */
typedef struct {
  struct group grp;
  char buf[X_LINE_MAX];	/* Should be sysconf(_SC_GETGR_R_SIZE_MAX)? */
} _Xgetgrparams;
#define _XGetgrgid(g,p)	\
 ((getgrgid_r((g), &(p).grp, (p).buf, sizeof((p).buf)) ? NULL : &(p).grp))
#define _XGetgrnam(n,p)	\
 ((getgrnam_r((n), &(p).grp, (p).buf, sizeof((p).buf)) ? NULL : &(p).grp))

#else
/* POSIX final API.
 *
 * int getgrgid_r(gid_t, struct group *, char *, size_t, struct group **);
 * int getgrnam_r(const char *, struct group *, char *, size_t, struct group **);
 */
typedef struct {
  struct group grp;
  char buf[X_LINE_MAX];	/* Should be sysconf(_SC_GETGR_R_SIZE_MAX)? */
  struct group *result;
} _Xgetgrparams;

#define _XGetgrgid(g,p)	\
 ((getgrgid_r((g), &(p).grp, (p).buf, sizeof((p).buf), &(p).result) ? \
   NULL : (p).result))
#define _XGetgrnam(n,p)	\
 ((getgrnam_r((n), &(p).grp, (p).buf, sizeof((p).buf), &(p).result) ? \
   NULL : (p).result))
#endif

#if defined(X_INCLUDE_GRP_H) && !defined(_XOS_INCLUDED_GRP_H)
# define _XOS_INCLUDED_GRP_H
#endif


#ifdef __cplusplus
}  /* Close scope of 'extern "C"' declaration which encloses file. */
#endif
