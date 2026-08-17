/*
 * Side-channel API definitions for CUPS.
 *
 * Copyright © 2007-2019 by Apple Inc.
 * Copyright © 2006 by Easy Software Products.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more information.
 */

#ifndef _CUPS_SIDECHANNEL_H_
#  define _CUPS_SIDECHANNEL_H_

/*
 * Include necessary headers...
 */

#  include "versioning.h"
#  include <sys/types.h>
#  if defined(_WIN32) && !defined(__CUPS_SSIZE_T_DEFINED)
#    define __CUPS_SSIZE_T_DEFINED
#    include <stddef.h>
/* Windows does not support the ssize_t type, so map it to long... */
typedef long ssize_t;			/* @private@ */
#  endif /* _WIN32 && !__CUPS_SSIZE_T_DEFINED */


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Constants...
 */

#define CUPS_SC_FD	4		/* File descriptor for select/poll */


/*
 * Enumerations...
 */

enum cups_sc_bidi_e			/**** Bidirectional capability values ****/
{
  CUPS_SC_BIDI_NOT_SUPPORTED = 0,	/* Bidirectional I/O is not supported */
  CUPS_SC_BIDI_SUPPORTED = 1		/* Bidirectional I/O is supported */
};
typedef enum cups_sc_bidi_e cups_sc_bidi_t;
					/**** Bidirectional capabilities ****/

enum cups_sc_command_e			/**** Request command codes ****/
{
  CUPS_SC_CMD_NONE = 0,			/* No command @private@ */
  CUPS_SC_CMD_SOFT_RESET = 1,		/* Do a soft reset */
  CUPS_SC_CMD_DRAIN_OUTPUT = 2,		/* Drain all pending output */
  CUPS_SC_CMD_GET_BIDI = 3,		/* Return bidirectional capabilities */
  CUPS_SC_CMD_GET_DEVICE_ID = 4,	/* Return the IEEE-1284 device ID */
  CUPS_SC_CMD_GET_STATE = 5,		/* Return the device state */
  CUPS_SC_CMD_SNMP_GET = 6,		/* Query an SNMP OID @since CUPS 1.4/macOS 10.6@ */
  CUPS_SC_CMD_SNMP_GET_NEXT = 7,	/* Query the next SNMP OID @since CUPS 1.4/macOS 10.6@ */
  CUPS_SC_CMD_GET_CONNECTED = 8,	/* Return whether the backend is "connected" to the printer @since CUPS 1.5/macOS 10.7@ */
  CUPS_SC_CMD_MAX			/* End of valid values @private@ */
};
typedef enum cups_sc_command_e cups_sc_command_t;
					/**** Request command codes ****/

enum cups_sc_connected_e		/**** Connectivity values ****/
{
  CUPS_SC_NOT_CONNECTED = 0,		/* Backend is not "connected" to printer */
  CUPS_SC_CONNECTED = 1			/* Backend is "connected" to printer */
};
typedef enum cups_sc_connected_e cups_sc_connected_t;
					/**** Connectivity values ****/


enum cups_sc_state_e			/**** Printer state bits ****/
{
  CUPS_SC_STATE_OFFLINE = 0,		/* Device is offline */
  CUPS_SC_STATE_ONLINE = 1,		/* Device is online */
  CUPS_SC_STATE_BUSY = 2,		/* Device is busy */
  CUPS_SC_STATE_ERROR = 4,		/* Other error condition */
  CUPS_SC_STATE_MEDIA_LOW = 16,		/* Paper low condition */
  CUPS_SC_STATE_MEDIA_EMPTY = 32,	/* Paper out condition */
  CUPS_SC_STATE_MARKER_LOW = 64,	/* Toner/ink low condition */
  CUPS_SC_STATE_MARKER_EMPTY = 128	/* Toner/ink out condition */
};
typedef enum cups_sc_state_e cups_sc_state_t;
					/**** Printer state bits ****/

enum cups_sc_status_e			/**** Response status codes ****/
{
  CUPS_SC_STATUS_NONE,			/* No status */
  CUPS_SC_STATUS_OK,			/* Operation succeeded */
  CUPS_SC_STATUS_IO_ERROR,		/* An I/O error occurred */
  CUPS_SC_STATUS_TIMEOUT,		/* The backend did not respond */
  CUPS_SC_STATUS_NO_RESPONSE,		/* The device did not respond */
  CUPS_SC_STATUS_BAD_MESSAGE,		/* The command/response message was invalid */
  CUPS_SC_STATUS_TOO_BIG,		/* Response too big */
  CUPS_SC_STATUS_NOT_IMPLEMENTED	/* Command not implemented */
};
typedef enum cups_sc_status_e cups_sc_status_t;
					/**** Response status codes ****/

typedef void (*cups_sc_walk_func_t)(const char *oid, const char *data,
                                    int datalen, void *context);
					/**** SNMP walk callback ****/


/*
 * Prototypes...
 */

/**** New in CUPS 1.2/macOS 10.5 ****/
extern ssize_t		cupsBackChannelRead(char *buffer, size_t bytes,
			                    double timeout) _CUPS_API_1_2;
extern ssize_t		cupsBackChannelWrite(const char *buffer, size_t bytes,
			                     double timeout) _CUPS_API_1_2;

/**** New in CUPS 1.3/macOS 10.5 ****/
extern cups_sc_status_t	cupsSideChannelDoRequest(cups_sc_command_t command,
			                         char *data, int *datalen,
						 double timeout) _CUPS_API_1_3;
extern int		cupsSideChannelRead(cups_sc_command_t *command,
			                    cups_sc_status_t *status,
					    char *data, int *datalen,
					    double timeout) _CUPS_API_1_3;
extern int		cupsSideChannelWrite(cups_sc_command_t command,
			                     cups_sc_status_t status,
					     const char *data, int datalen,
					     double timeout) _CUPS_API_1_3;

/**** New in CUPS 1.4/macOS 10.6 ****/
extern cups_sc_status_t	cupsSideChannelSNMPGet(const char *oid, char *data,
			                       int *datalen, double timeout)
					       _CUPS_API_1_4;
extern cups_sc_status_t	cupsSideChannelSNMPWalk(const char *oid, double timeout,
						cups_sc_walk_func_t cb,
						void *context) _CUPS_API_1_4;


#  ifdef __cplusplus
}
#  endif /* __cplusplus */

#endif /* !_CUPS_SIDECHANNEL_H_ */
