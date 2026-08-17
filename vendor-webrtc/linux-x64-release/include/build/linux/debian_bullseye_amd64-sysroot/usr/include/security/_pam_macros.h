#ifndef PAM_MACROS_H
#define PAM_MACROS_H

/*
 * All kind of macros used by PAM, but usable in some other
 * programs too.
 * Organized by Cristian Gafton <gafton@redhat.com>
 */

/* a 'safe' version of strdup */

#include <stdlib.h>
#include <string.h>

#define  x_strdup(s)  ( (s) ? strdup(s):NULL )

/* Good policy to strike out passwords with some characters not just
   free the memory */

#define _pam_overwrite(x)        \
do {                             \
     register char *__xx__;      \
     if ((__xx__=(x)))           \
          while (*__xx__)        \
               *__xx__++ = '\0'; \
} while (0)

#define _pam_overwrite_n(x,n)   \
do {                             \
     register char *__xx__;      \
     register unsigned int __i__ = 0;    \
     if ((__xx__=(x)))           \
        for (;__i__<n; __i__++) \
            __xx__[__i__] = 0; \
} while (0)

/*
 * Don't just free it, forget it too.
 */

#define _pam_drop(X) \
do {                 \
    if (X) {         \
        free(X);     \
        X=NULL;      \
    }                \
} while (0)

#define _pam_drop_reply(/* struct pam_response * */ reply, /* int */ replies) \
do {                                              \
    int reply_i;                                  \
                                                  \
    for (reply_i=0; reply_i<replies; ++reply_i) { \
	if (reply[reply_i].resp) {                \
	    _pam_overwrite(reply[reply_i].resp);  \
	    free(reply[reply_i].resp);            \
	}                                         \
    }                                             \
    if (reply)                                    \
	free(reply);                              \
} while (0)

/* some debugging code */

#ifdef PAM_DEBUG

/*
 * This provides the necessary function to do debugging in PAM.
 * Cristian Gafton <gafton@redhat.com>
 */

#include <stdio.h>
#include <sys/types.h>
#include <stdarg.h>
#include <errno.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

/*
 * This is for debugging purposes ONLY. DO NOT use on live systems !!!
 * You have been warned :-) - CG
 *
 * to get automated debugging to the log file, it must be created manually.
 * _PAM_LOGFILE must exist and be writable to the programs you debug.
 */

#ifndef _PAM_LOGFILE
#define _PAM_LOGFILE "/var/run/pam-debug.log"
#endif

static void _pam_output_debug_info(const char *file, const char *fn
				   , const int line)
{
    FILE *logfile;
    int must_close = 1, fd;

#ifdef O_NOFOLLOW
    if ((fd = open(_PAM_LOGFILE, O_WRONLY|O_NOFOLLOW|O_APPEND)) != -1) {
#else
    if ((fd = open(_PAM_LOGFILE, O_WRONLY|O_APPEND)) != -1) {
#endif
	if (!(logfile = fdopen(fd,"a"))) {
	    logfile = stderr;
	    must_close = 0;
	    close(fd);
	}
    } else {
        logfile = stderr;
	must_close = 0;
    }
    fprintf(logfile,"[%s:%s(%d)] ",file, fn, line);
    fflush(logfile);
    if (must_close)
        fclose(logfile);
}

static void _pam_output_debug(const char *format, ...)
{
    va_list args;
    FILE *logfile;
    int must_close = 1, fd;

    va_start(args, format);

#ifdef O_NOFOLLOW
    if ((fd = open(_PAM_LOGFILE, O_WRONLY|O_NOFOLLOW|O_APPEND)) != -1) {
#else
    if ((fd = open(_PAM_LOGFILE, O_WRONLY|O_APPEND)) != -1) {
#endif
	if (!(logfile = fdopen(fd,"a"))) {
	    logfile = stderr;
	    must_close = 0;
	    close(fd);
	}
    } else {
	logfile = stderr;
	must_close = 0;
    }
    vfprintf(logfile, format, args);
    fprintf(logfile, "\n");
    fflush(logfile);
    if (must_close)
        fclose(logfile);

    va_end(args);
}

#define D(x) do { \
    _pam_output_debug_info(__FILE__, __FUNCTION__, __LINE__); \
    _pam_output_debug x ; \
} while (0)

#define _pam_show_mem(X,XS) do {                                      \
      int i;                                                          \
      register unsigned char *x;                                      \
      x = (unsigned char *)X;                                         \
      fprintf(stderr, "  <start at %p>\n", X);                        \
      for (i = 0; i < XS ; ++x, ++i) {                                \
          fprintf(stderr, "    %02X. <%p:%02X>\n", i, x, *x);         \
      }                                                               \
      fprintf(stderr, "  <end for %p after %d bytes>\n", X, XS);      \
} while (0)

#define _pam_show_reply(/* struct pam_response * */reply, /* int */replies) \
do {                                                                        \
    int reply_i;                                                            \
    setbuf(stderr, NULL);                                                   \
    fprintf(stderr, "array at %p of size %d\n",reply,replies);              \
    fflush(stderr);                                                         \
    if (reply) {                                                            \
	for (reply_i = 0; reply_i < replies; reply_i++) {                   \
	    fprintf(stderr, "  elem# %d at %p: resp = %p, retcode = %d\n",  \
		    reply_i, reply+reply_i, reply[reply_i].resp,            \
		    reply[reply_i].resp, _retcode);                         \
	    fflush(stderr);                                                 \
	    if (reply[reply_i].resp) {                                      \
		fprintf(stderr, "    resp[%d] = '%s'\n",                    \
			strlen(reply[reply_i].resp), reply[reply_i].resp);  \
		fflush(stderr);                                             \
	    }                                                               \
	}                                                                   \
    }                                                                       \
    fprintf(stderr, "done here\n");                                         \
    fflush(stderr);                                                         \
} while (0)

#else

#define D(x)                             do { } while (0)
#define _pam_show_mem(X,XS)              do { } while (0)
#define _pam_show_reply(reply, replies)  do { } while (0)

#endif /* PAM_DEBUG */

#endif  /* PAM_MACROS_H */
