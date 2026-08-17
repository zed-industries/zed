/*    thread.h
 *
 *    Copyright (C) 1999, 2000, 2001, 2002, 2003, 2004, 2005, 2006,
 *    by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#if defined(USE_ITHREADS)

#if defined(VMS)
#include <builtins.h>
#endif

#ifdef WIN32
#  include <win32thread.h>
#elif defined(NETWARE)
#  include <nw5thread.h>
#else
#  ifdef OLD_PTHREADS_API /* Here be dragons. */
#    define DETACH(t) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_detach(&(t)->self))) {		\
	    MUTEX_UNLOCK(&(t)->mutex);				\
	    Perl_croak_nocontext("panic: DETACH (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
	}							\
    } STMT_END

#    define PERL_GET_CONTEXT	Perl_get_context()
#    define PERL_SET_CONTEXT(t)	Perl_set_context((void*)t)

#    define PTHREAD_GETSPECIFIC_INT
#    ifdef DJGPP
#      define pthread_addr_t any_t
#      define NEED_PTHREAD_INIT
#      define PTHREAD_CREATE_JOINABLE (1)
#    endif
#    ifdef OEMVS
#      define pthread_addr_t void *
#      define pthread_create(t,a,s,d)        pthread_create(t,&(a),s,d)
#      define pthread_keycreate              pthread_key_create
#    endif
#    ifdef VMS
#      define pthread_attr_init(a) pthread_attr_create(a)
#      define PTHREAD_ATTR_SETDETACHSTATE(a,s) pthread_setdetach_np(a,s)
#      define PTHREAD_CREATE(t,a,s,d) pthread_create(t,a,s,d)
#      define pthread_key_create(k,d) pthread_keycreate(k,(pthread_destructor_t)(d))
#      define pthread_mutexattr_init(a) pthread_mutexattr_create(a)
#      define pthread_mutexattr_settype(a,t) pthread_mutexattr_setkind_np(a,t)
#    endif
#    if defined(__hpux) && defined(__ux_version) && __ux_version <= 1020
#      define pthread_attr_init(a) pthread_attr_create(a)
       /* XXX pthread_setdetach_np() missing in DCE threads on HP-UX 10.20 */
#      define PTHREAD_ATTR_SETDETACHSTATE(a,s)	(0)
#      define PTHREAD_CREATE(t,a,s,d) pthread_create(t,a,s,d)
#      define pthread_key_create(k,d) pthread_keycreate(k,(pthread_destructor_t)(d))
#      define pthread_mutexattr_init(a) pthread_mutexattr_create(a)
#      define pthread_mutexattr_settype(a,t) pthread_mutexattr_setkind_np(a,t)
#    endif
#    if defined(DJGPP) || defined(OEMVS)
#      define PTHREAD_ATTR_SETDETACHSTATE(a,s) pthread_attr_setdetachstate(a,&(s))
#      define YIELD pthread_yield(NULL)
#    endif
#  endif
#  if !defined(__hpux) || !defined(__ux_version) || __ux_version > 1020
#    define pthread_mutexattr_default NULL
#    define pthread_condattr_default  NULL
#  endif
#endif

#ifndef PTHREAD_CREATE
/* You are not supposed to pass NULL as the 2nd arg of PTHREAD_CREATE(). */
#  define PTHREAD_CREATE(t,a,s,d) pthread_create(t,&(a),s,d)
#endif

#ifndef PTHREAD_ATTR_SETDETACHSTATE
#  define PTHREAD_ATTR_SETDETACHSTATE(a,s) pthread_attr_setdetachstate(a,s)
#endif

#ifndef PTHREAD_CREATE_JOINABLE
#  ifdef OLD_PTHREAD_CREATE_JOINABLE
#    define PTHREAD_CREATE_JOINABLE OLD_PTHREAD_CREATE_JOINABLE
#  else
#    define PTHREAD_CREATE_JOINABLE 0 /* Panic?  No, guess. */
#  endif
#endif

#ifdef __VMS
  /* Default is 1024 on VAX, 8192 otherwise */
#  ifdef __ia64
#    define THREAD_CREATE_NEEDS_STACK (48*1024)
#  else
#    define THREAD_CREATE_NEEDS_STACK (32*1024)
#  endif
#endif

#ifdef I_MACH_CTHREADS

/* cthreads interface */

/* #include <mach/cthreads.h> is in perl.h #ifdef I_MACH_CTHREADS */

#define MUTEX_INIT(m) \
    STMT_START {						\
	*m = mutex_alloc();					\
	if (*m) {						\
	    mutex_init(*m);					\
	} else {						\
	    Perl_croak_nocontext("panic: MUTEX_INIT [%s:%d]",	\
				 __FILE__, __LINE__);		\
	}							\
    } STMT_END

#define MUTEX_LOCK(m)			mutex_lock(*m)
#define MUTEX_UNLOCK(m)			mutex_unlock(*m)
#define MUTEX_DESTROY(m) \
    STMT_START {						\
	mutex_free(*m);						\
	*m = 0;							\
    } STMT_END

#define COND_INIT(c) \
    STMT_START {						\
	*c = condition_alloc();					\
	if (*c) {						\
	    condition_init(*c);					\
	}							\
	else {							\
	    Perl_croak_nocontext("panic: COND_INIT [%s:%d]",	\
				 __FILE__, __LINE__);		\
	}							\
    } STMT_END

#define COND_SIGNAL(c)		condition_signal(*c)
#define COND_BROADCAST(c)	condition_broadcast(*c)
#define COND_WAIT(c, m)		condition_wait(*c, *m)
#define COND_DESTROY(c) \
    STMT_START {						\
	condition_free(*c);					\
	*c = 0;							\
    } STMT_END

#define THREAD_CREATE(thr, f)	(thr->self = cthread_fork(f, thr), 0)
#define THREAD_POST_CREATE(thr)	NOOP

#define THREAD_RET_TYPE		any_t
#define THREAD_RET_CAST(x)	((any_t) x)

#define DETACH(t)		cthread_detach(t->self)
#define JOIN(t, avp)		(*(avp) = MUTABLE_AV(cthread_join(t->self)))

#define PERL_SET_CONTEXT(t)	cthread_set_data(cthread_self(), t)
#define PERL_GET_CONTEXT	cthread_data(cthread_self())

#define INIT_THREADS		cthread_init()
#define YIELD			cthread_yield()
#define ALLOC_THREAD_KEY	NOOP
#define FREE_THREAD_KEY		NOOP
#define SET_THREAD_SELF(thr)	(thr->self = cthread_self())

#endif /* I_MACH_CTHREADS */

#ifndef YIELD
#  ifdef SCHED_YIELD
#    define YIELD SCHED_YIELD
#  elif defined(HAS_SCHED_YIELD)
#    define YIELD sched_yield()
#  elif defined(HAS_PTHREAD_YIELD)
    /* pthread_yield(NULL) platforms are expected
     * to have #defined YIELD for themselves. */
#    define YIELD pthread_yield()
#  endif
#endif

#ifdef __hpux
#  define MUTEX_INIT_NEEDS_MUTEX_ZEROED
#endif

#ifndef MUTEX_INIT

#  ifdef MUTEX_INIT_NEEDS_MUTEX_ZEROED
    /* Temporary workaround, true bug is deeper. --jhi 1999-02-25 */
#    define MUTEX_INIT(m) \
    STMT_START {						\
	int _eC_;						\
	Zero((m), 1, perl_mutex);                               \
 	if ((_eC_ = pthread_mutex_init((m), pthread_mutexattr_default)))	\
	    Perl_croak_nocontext("panic: MUTEX_INIT (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#  else
#    define MUTEX_INIT(m) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_mutex_init((m), pthread_mutexattr_default)))	\
	    Perl_croak_nocontext("panic: MUTEX_INIT (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#  endif

#  ifdef PERL_TSA_ACTIVE
#    define perl_pthread_mutex_lock(m) perl_tsa_mutex_lock(m)
#    define perl_pthread_mutex_unlock(m) perl_tsa_mutex_unlock(m)
#  else
#    define perl_pthread_mutex_lock(m) pthread_mutex_lock(m)
#    define perl_pthread_mutex_unlock(m) pthread_mutex_unlock(m)
#  endif

#  define MUTEX_LOCK(m) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = perl_pthread_mutex_lock((m))))			\
	    Perl_croak_nocontext("panic: MUTEX_LOCK (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define MUTEX_UNLOCK(m) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = perl_pthread_mutex_unlock((m))))			\
	    Perl_croak_nocontext("panic: MUTEX_UNLOCK (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define MUTEX_DESTROY(m) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_mutex_destroy((m))))		\
	    Perl_croak_nocontext("panic: MUTEX_DESTROY (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#endif /* MUTEX_INIT */

#ifndef COND_INIT
#  define COND_INIT(c) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_cond_init((c), pthread_condattr_default)))	\
	    Perl_croak_nocontext("panic: COND_INIT (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define COND_SIGNAL(c) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_cond_signal((c))))			\
	    Perl_croak_nocontext("panic: COND_SIGNAL (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define COND_BROADCAST(c) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_cond_broadcast((c))))		\
	    Perl_croak_nocontext("panic: COND_BROADCAST (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define COND_WAIT(c, m) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_cond_wait((c), (m))))		\
	    Perl_croak_nocontext("panic: COND_WAIT (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END

#  define COND_DESTROY(c) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_cond_destroy((c))))			\
	    Perl_croak_nocontext("panic: COND_DESTROY (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#endif /* COND_INIT */

/* DETACH(t) must only be called while holding t->mutex */
#ifndef DETACH
#  define DETACH(t) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_detach((t)->self))) {		\
	    MUTEX_UNLOCK(&(t)->mutex);				\
	    Perl_croak_nocontext("panic: DETACH (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
	}							\
    } STMT_END
#endif /* DETACH */

#ifndef JOIN
#  define JOIN(t, avp) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_join((t)->self, (void**)(avp))))	\
	    Perl_croak_nocontext("panic: pthread_join (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#endif /* JOIN */

/* Use an unchecked fetch of thread-specific data instead of a checked one.
 * It would fail if the key were bogus, but if the key were bogus then
 * Really Bad Things would be happening anyway. --dan */
#if (defined(__ALPHA) && (__VMS_VER >= 70000000)) || \
    (defined(__alpha) && defined(__osf__) && !defined(__GNUC__)) /* Available only on >= 4.0 */
#  define HAS_PTHREAD_UNCHECKED_GETSPECIFIC_NP /* Configure test needed */
#endif

#ifdef HAS_PTHREAD_UNCHECKED_GETSPECIFIC_NP
#  define PTHREAD_GETSPECIFIC(key) pthread_unchecked_getspecific_np(key)
#else
#    define PTHREAD_GETSPECIFIC(key) pthread_getspecific(key)
#endif

#ifndef PERL_GET_CONTEXT
#  define PERL_GET_CONTEXT	PTHREAD_GETSPECIFIC(PL_thr_key)
#endif

#ifndef PERL_SET_CONTEXT
#  define PERL_SET_CONTEXT(t) \
    STMT_START {						\
	int _eC_;						\
	if ((_eC_ = pthread_setspecific(PL_thr_key, (void *)(t))))	\
	    Perl_croak_nocontext("panic: pthread_setspecific (%d) [%s:%d]",	\
				 _eC_, __FILE__, __LINE__);	\
    } STMT_END
#endif /* PERL_SET_CONTEXT */

#ifndef INIT_THREADS
#  ifdef NEED_PTHREAD_INIT
#    define INIT_THREADS pthread_init()
#  endif
#endif

#ifndef ALLOC_THREAD_KEY
#  define ALLOC_THREAD_KEY \
    STMT_START {						\
	if (pthread_key_create(&PL_thr_key, 0)) {		\
            PERL_UNUSED_RESULT(write(2, STR_WITH_LEN("panic: pthread_key_create failed\n"))); \
	    exit(1);						\
	}							\
    } STMT_END
#endif

#ifndef FREE_THREAD_KEY
#  define FREE_THREAD_KEY \
    STMT_START {						\
	pthread_key_delete(PL_thr_key);				\
    } STMT_END
#endif

#ifndef PTHREAD_ATFORK
#  ifdef HAS_PTHREAD_ATFORK
#    define PTHREAD_ATFORK(prepare,parent,child)		\
	pthread_atfork(prepare,parent,child)
#  else
#    define PTHREAD_ATFORK(prepare,parent,child)		\
	NOOP
#  endif
#endif

#ifndef THREAD_RET_TYPE
#  define THREAD_RET_TYPE	void *
#  define THREAD_RET_CAST(p)	((void *)(p))
#endif /* THREAD_RET */

#  define LOCK_DOLLARZERO_MUTEX		MUTEX_LOCK(&PL_dollarzero_mutex)
#  define UNLOCK_DOLLARZERO_MUTEX	MUTEX_UNLOCK(&PL_dollarzero_mutex)

#endif /* USE_ITHREADS */

#ifndef MUTEX_LOCK
#  define MUTEX_LOCK(m)           NOOP
#endif

#ifndef MUTEX_UNLOCK
#  define MUTEX_UNLOCK(m)         NOOP
#endif

#ifndef MUTEX_INIT
#  define MUTEX_INIT(m)           NOOP
#endif

#ifndef MUTEX_DESTROY
#  define MUTEX_DESTROY(m)        NOOP
#endif

#ifndef COND_INIT
#  define COND_INIT(c)            NOOP
#endif

#ifndef COND_SIGNAL
#  define COND_SIGNAL(c)          NOOP
#endif

#ifndef COND_BROADCAST
#  define COND_BROADCAST(c)       NOOP
#endif

#ifndef COND_WAIT
#  define COND_WAIT(c, m)         NOOP
#endif

#ifndef COND_DESTROY
#  define COND_DESTROY(c)         NOOP
#endif

#ifndef LOCK_DOLLARZERO_MUTEX
#  define LOCK_DOLLARZERO_MUTEX   NOOP
#endif

#ifndef UNLOCK_DOLLARZERO_MUTEX
#  define UNLOCK_DOLLARZERO_MUTEX NOOP
#endif

/* THR, SET_THR, and dTHR are there for compatibility with old versions */
#ifndef THR
#  define THR		PERL_GET_THX
#endif

#ifndef SET_THR
#  define SET_THR(t)	PERL_SET_THX(t)
#endif

#ifndef dTHR
#  define dTHR dNOOP
#endif

#ifndef INIT_THREADS
#  define INIT_THREADS NOOP
#endif

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
