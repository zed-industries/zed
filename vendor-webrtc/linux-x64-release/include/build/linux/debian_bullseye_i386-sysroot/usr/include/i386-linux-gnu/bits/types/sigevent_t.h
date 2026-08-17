#ifndef __sigevent_t_defined
#define __sigevent_t_defined 1

#include <bits/wordsize.h>
#include <bits/types.h>
#include <bits/types/__sigval_t.h>

#define __SIGEV_MAX_SIZE	64
#if __WORDSIZE == 64
# define __SIGEV_PAD_SIZE	((__SIGEV_MAX_SIZE / sizeof (int)) - 4)
#else
# define __SIGEV_PAD_SIZE	((__SIGEV_MAX_SIZE / sizeof (int)) - 3)
#endif

/* Forward declaration.  */
#ifndef __have_pthread_attr_t
typedef union pthread_attr_t pthread_attr_t;
# define __have_pthread_attr_t	1
#endif

/* Structure to transport application-defined values with signals.  */
typedef struct sigevent
  {
    __sigval_t sigev_value;
    int sigev_signo;
    int sigev_notify;

    union
      {
	int _pad[__SIGEV_PAD_SIZE];

	/* When SIGEV_SIGNAL and SIGEV_THREAD_ID set, LWP ID of the
	   thread to receive the signal.  */
	__pid_t _tid;

	struct
	  {
	    void (*_function) (__sigval_t);	/* Function to start.  */
	    pthread_attr_t *_attribute;		/* Thread attributes.  */
	  } _sigev_thread;
      } _sigev_un;
  } sigevent_t;

/* POSIX names to access some of the members.  */
#define sigev_notify_function   _sigev_un._sigev_thread._function
#define sigev_notify_attributes _sigev_un._sigev_thread._attribute

#endif
