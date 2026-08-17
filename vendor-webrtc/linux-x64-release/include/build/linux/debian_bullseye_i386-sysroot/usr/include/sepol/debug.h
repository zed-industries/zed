#ifndef _SEPOL_DEBUG_H_
#define _SEPOL_DEBUG_H_

#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Deprecated */
extern void sepol_debug(int on);
/* End deprecated */

#define SEPOL_MSG_ERR  1
#define SEPOL_MSG_WARN 2
#define SEPOL_MSG_INFO 3

extern int sepol_msg_get_level(sepol_handle_t * handle);

extern const char *sepol_msg_get_channel(sepol_handle_t * handle);

extern const char *sepol_msg_get_fname(sepol_handle_t * handle);

/* Set the messaging callback. 
 * By the default, the callback will print
 * the message on standard output, in a 
 * particular format. Passing NULL here
 * indicates that messaging should be suppressed */
extern void sepol_msg_set_callback(sepol_handle_t * handle,
#ifdef __GNUC__
				   __attribute__ ((format(printf, 3, 4)))
#endif
				   void (*msg_callback) (void *varg,
							 sepol_handle_t *
							 handle,
							 const char *fmt, ...),
				   void *msg_callback_arg);

#ifdef __cplusplus
}
#endif

#endif
