/*
 * libbrlapi - A library providing access to braille terminals for applications.
 *
 * Copyright (C) 2002-2020 by
 *   Samuel Thibault <Samuel.Thibault@ens-lyon.org>
 *   Sébastien Hinderer <Sebastien.Hinderer@ens-lyon.org>
 *
 * libbrlapi comes with ABSOLUTELY NO WARRANTY.
 *
 * This is free software, placed under the terms of the
 * GNU Lesser General Public License, as published by the Free Software
 * Foundation; either version 2.1 of the License, or (at your option) any
 * later version. Please see the file LICENSE-LGPL for details.
 *
 * Web Page: http://brltty.app/
 *
 * This software is maintained by Dave Mielke <dave@mielke.cc>.
 */

/** \file
 * \brief types and constants for \e BrlAPI's protocol
 */

#ifndef BRLAPI_INCLUDED_PROTOCOL
#define BRLAPI_INCLUDED_PROTOCOL

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#include "brlapi.h"

/* this is for UINT32_MAX */
#include <inttypes.h>
#ifndef UINT32_MAX
#define UINT32_MAX (4294967295U)
#endif /* UINT32_MAX */

/* The type size_t is defined there! */
#include <unistd.h>

/** \defgroup brlapi_protocol BrlAPI's protocol
 * \brief Instructions and constants for \e BrlAPI 's protocol
 *
 * These are defines for the protocol between \e BrlAPI 's server and clients.
 * Understanding is not needed to use the \e BrlAPI library, so reading this
 * is not needed unless really wanting to connect to \e BrlAPI without
 * \e BrlAPI 's library.
 *
 * @{ */

#define BRLAPI_PROTOCOL_VERSION ((uint32_t) 8) /** Communication protocol version */

/** Maximum packet size for packets exchanged on sockets and with braille
 * terminal */
#define BRLAPI_MAXPACKETSIZE 4096

#define BRLAPI_PACKET_VERSION         'v'   /**< Version                     */
#define BRLAPI_PACKET_AUTH            'a'   /**< Authorization               */
#define BRLAPI_PACKET_GETDRIVERNAME   'n'   /**< Ask which driver is used    */
#define BRLAPI_PACKET_GETMODELID      'd'   /**< Ask which model is used     */
#define BRLAPI_PACKET_GETDISPLAYSIZE  's'   /**< Dimensions of brl display   */
#define BRLAPI_PACKET_ENTERTTYMODE    't'   /**< Asks for a specified tty    */
#define BRLAPI_PACKET_SETFOCUS        'F'   /**< Set current tty focus       */
#define BRLAPI_PACKET_LEAVETTYMODE    'L'   /**< Release the tty             */
#define BRLAPI_PACKET_KEY             'k'   /**< Braille key                 */
#define BRLAPI_PACKET_IGNOREKEYRANGES 'm'   /**< Mask key ranges             */
#define BRLAPI_PACKET_ACCEPTKEYRANGES 'u'   /**< Unmask key ranges           */
#define BRLAPI_PACKET_WRITE           'w'   /**< Write                       */
#define BRLAPI_PACKET_ENTERRAWMODE    '*'   /**< Enter in raw mode           */
#define BRLAPI_PACKET_LEAVERAWMODE    '#'   /**< Leave raw mode              */
#define BRLAPI_PACKET_PACKET          'p'   /**< Raw packets                 */
#define BRLAPI_PACKET_ACK             'A'   /**< Acknowledgement             */
#define BRLAPI_PACKET_ERROR           'e'   /**< non-fatal error             */
#define BRLAPI_PACKET_EXCEPTION       'E'   /**< Exception                   */
#define BRLAPI_PACKET_SUSPENDDRIVER   'S'   /**< Suspend driver              */
#define BRLAPI_PACKET_RESUMEDRIVER    'R'   /**< Resume driver               */
#define BRLAPI_PACKET_PARAM_VALUE     (('P'<<8) + 'V') /**< Parameter value  */
#define BRLAPI_PACKET_PARAM_REQUEST   (('P'<<8) + 'R') /**< Parameter request*/
#define BRLAPI_PACKET_PARAM_UPDATE    (('P'<<8) + 'U') /**< Parameter update */

/** Magic number to give when sending a BRLPACKET_ENTERRAWMODE or BRLPACKET_SUSPEND packet */
#define BRLAPI_DEVICE_MAGIC (0xdeadbeefL)

/** Structure of packets headers */
typedef struct {
  uint32_t size;
  brlapi_packetType_t type;
} brlapi_header_t;

/** Size of packet headers */
#define BRLAPI_HEADERSIZE sizeof(brlapi_header_t)

/** Structure of version packets */
typedef struct {
  uint32_t protocolVersion;
} brlapi_versionPacket_t;

/** Structure of authorization packets */
typedef struct {
  uint32_t type;
  unsigned char key;
} brlapi_authClientPacket_t;

typedef struct {
  uint32_t type[1];
} brlapi_authServerPacket_t;

#define BRLAPI_AUTH_NONE 'N' /**< No or implicit authorization              */
#define BRLAPI_AUTH_KEY  'K' /**< Key authorization                         */
#define BRLAPI_AUTH_CRED 'C' /**< Explicit socket credentials authorization */

/** Structure of error packets */
typedef struct {
  uint32_t code;
  brlapi_packetType_t type;
  unsigned char packet;
} brlapi_errorPacket_t;

/** Structure of enterRawMode / suspend packets */
typedef struct {
  uint32_t magic;
  unsigned char nameLength;
  char name;
} brlapi_getDriverSpecificModePacket_t;

/** Flags for writing */
#define BRLAPI_WF_DISPLAYNUMBER 0X01    /**< Display number                 */
#define BRLAPI_WF_REGION        0X02    /**< Region parameter               */
#define BRLAPI_WF_TEXT          0X04    /**< Contains some text             */
#define BRLAPI_WF_ATTR_AND      0X08    /**< And attributes                 */
#define BRLAPI_WF_ATTR_OR       0X10    /**< Or attributes                  */
#define BRLAPI_WF_CURSOR        0X20    /**< Cursor position                */
#define BRLAPI_WF_CHARSET       0X40    /**< Charset                        */

/** Structure of extended write packets */
typedef struct {
  uint32_t flags; /** Flags to tell which fields are present */
  unsigned char data; /** Fields in the same order as flag weight */
} brlapi_writeArgumentsPacket_t;

/** Flags for parameter values */
#define BRLAPI_PVF_GLOBAL            0X01    /** Value is the global value */

#define BRLAPI_MAXPARAMSIZE (BRLAPI_MAXPACKETSIZE - (sizeof(uint32_t) + sizeof(brlapi_param_t) + 2*sizeof(uint32_t)))

/** Structure of Parameter value or update */
typedef struct {
  uint32_t flags; /** Flags to tell how value was gotten */
  brlapi_param_t param; /** Which parameter being transmitted */
  uint32_t subparam_hi; /** Which sub-parameter being transmitted, hi 32bits */
  uint32_t subparam_lo; /** Which sub-parameter being transmitted, lo 32bits */
  unsigned char data[BRLAPI_MAXPARAMSIZE]; /** Content of the parameter */
} brlapi_paramValuePacket_t;

/** Flags for parameter requests */
#define BRLAPI_PARAMF_GET            0X100    /** Get current parameter value    */
#define BRLAPI_PARAMF_SUBSCRIBE      0X200    /** Subscribe to parameter updates */
#define BRLAPI_PARAMF_UNSUBSCRIBE    0X400    /** Unsubscribe from parameter updates */

/** Structure of Parameter request */
typedef struct {
  uint32_t flags; /** Flags to tell whether/how to get values */
  brlapi_param_t param; /** Which parameter to be transmitted */
  uint32_t subparam_hi; /** Which sub-parameter being transmitted, hi 32bits */
  uint32_t subparam_lo; /** Which sub-parameter being transmitted, lo 32bits */
} brlapi_paramRequestPacket_t;

/** Type for packets.  Should be used instead of a mere char[], since it has
 * correct alignment requirements. */
typedef union {
	unsigned char data[BRLAPI_MAXPACKETSIZE];
	brlapi_versionPacket_t version;
	brlapi_authClientPacket_t authClient;
	brlapi_authServerPacket_t authServer;
	brlapi_errorPacket_t error;
	brlapi_getDriverSpecificModePacket_t getDriverSpecificMode;
	brlapi_writeArgumentsPacket_t writeArguments;
	brlapi_paramValuePacket_t paramValue;
	brlapi_paramRequestPacket_t paramRequest;
	uint32_t uint32;
} brlapi_packet_t;

/* brlapi_writePacket */
/** Send a packet to \e BrlAPI server
 *
 * This function is for internal use, but one might use it if one really knows
 * what one is doing...
 *
 * \e type should only be one of the above defined BRLPACKET_*.
 *
 * The syntax is the same as write()'s.
 *
 * \return 0 on success, -1 on failure.
 *
 * \sa brlapi_readPacketHeader()
 * brlapi_readPacketContent()
 * brlapi_readPacket()
 */
ssize_t brlapi_writePacket(brlapi_fileDescriptor fd, brlapi_packetType_t type, const void *buf, size_t size);

/* brlapi_readPacketHeader */
/** Read the header (type+size) of a packet from \e BrlAPI server
 *
 * This function is for internal use, but one might use it if one really knows
 * what one is doing...
 *
 * \e type is where the function will store the packet type; it should always
 * be one of the above defined BRLPACKET_* (or else something very nasty must
 * have happened :/).
 *
 * \return packet's size, -2 if \c EOF occurred, -1 on error or signal
 * interruption.
 *
 * \sa brlapi_writePacket()
 * brlapi_readPacketContent
 * brlapi_readPacket
 */
ssize_t brlapi_readPacketHeader(brlapi_fileDescriptor fd, brlapi_packetType_t *packetType);

/* brlapi_readPacketContent */
/** Read the content of a packet from \e BrlAPI server
 *
 * This function is for internal use, but one might use it if one really knows
 * what one is doing...
 *
 * \e packetSize is the size announced by \e brlapi_readPacketHeader()
 *
 * \e bufSize is the size of \e buf
 *
 * \return packetSize, -2 if \c EOF occurred, -1 on error.
 *
 * If the packet is larger than the supplied buffer, the buffer will be
 * filled with the beginning of the packet, the rest of the packet being
 * discarded. This follows the semantics of the recv system call when the
 * MSG_TRUNC option is given.
 *
 * \sa brlapi_writePacket()
 * brlapi_readPacketHeader()
 * brlapi_readPacket()
 */
ssize_t brlapi_readPacketContent(brlapi_fileDescriptor fd, size_t packetSize, void *buf, size_t bufSize);

/* brlapi_readPacket */
/** Read a packet from \e BrlAPI server
 *
 * This function is for internal use, but one might use it if one really knows
 * what one is doing...
 *
 * \e type is where the function will store the packet type; it should always
 * be one of the above defined BRLPACKET_* (or else something very nasty must
 * have happened :/).
 *
 * The syntax is the same as read()'s.
 *
 * \return packet's size, -2 if \c EOF occurred, -1 on error or signal
 * interruption.
 *
 * If the packet is larger than the supplied buffer, the buffer will be
 * filled with the beginning of the packet, the rest of the packet being
 * discarded. This follows the semantics of the recv system call when the
 * MSG_TRUNC option is given.
 *
 * \sa brlapi_writePacket()
 */
ssize_t brlapi_readPacket(brlapi_fileDescriptor fd, brlapi_packetType_t *type, void *buf, size_t size);

/* brlapi_fd_mutex */
/** Mutex for protecting concurrent fd access
 *
 * In order to regulate concurrent access to the library's file descriptor and
 * requests to / answers from \e BrlAPI server, every function of the library
 * locks this mutex, namely
 *
 * - brlapi_openConnection()
 * - brlapi_closeConnection()
 * - brlapi_enterRawMode()
 * - brlapi_leaveRawMode()
 * - brlapi_sendRaw()
 * - brlapi_recvRaw()
 * - brlapi_getDriverName()
 * - brlapi_getDisplaySize()
 * - brlapi_enterTtyMode()
 * - brlapi_enterTtyModeWithPath()
 * - brlapi_leaveTtyMode()
 * - brlapi_*write*()
 * - brlapi_(un)?ignorekey(Range|Set)()
 * - brlapi_readKey()
 *
 * If both these functions and brlapi_writePacket() or brlapi_readPacket() are
 * used in a multithreaded application, this mutex must be locked before calling
 * brlapi_writePacket() or brlapi_readPacket(), and unlocked afterwards.
 */
#ifdef __MINGW32__
#include <windows.h>
extern HANDLE brlapi_fd_mutex;
#else /* __MINGW32__ */
#include <pthread.h>
extern pthread_mutex_t brlapi_fd_mutex;
#endif /* __MINGW32__ */

/* @} */

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* BRLAPI_INCLUDED_PROTOCOL */
