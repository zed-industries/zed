/* Programs/brlapi.h.  Generated from brlapi.h.in by configure.  */
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
 * \brief Types, defines and functions prototypes for \e BrlAPI's library
 */

#ifndef BRLAPI_INCLUDED
#define BRLAPI_INCLUDED

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* #undef BRLAPI_WIN32 */

/** \defgroup brlapi_version Version of the BrlAPI library
 * @{ */

/** Library version. */
#define BRLAPI_RELEASE "0.8.0"

/** Library major version. */
#define BRLAPI_MAJOR 0

/** Library minor version. */
#define BRLAPI_MINOR 8

/** Library revision. */
#define BRLAPI_REVISION 0

/** Returns the version of the library */
void brlapi_getLibraryVersion(int* major, int* minor, int* revision);

/** @} */

/* Types are defined there */
#include <sys/types.h>

#ifdef BRLAPI_WIN32
#include <windows.h>
#define BRLAPI_STDCALL __stdcall
#else /* BRLAPI_WIN32 */
#define BRLAPI_STDCALL
#endif /* BRLAPI_WIN32 */

#ifdef _MSC_VER
typedef __int64 uint64_t;
typedef __int32 uint32_t;
#define UINT64_C(x) (x##Ui64)
#define PRIx64 "I64x"
typedef signed int ssize_t;
#else /* _MSC_VER */

/* this is for uint*_t */
#include <stdint.h>

/* NULL is defined there */
#include <unistd.h>

#include <inttypes.h> /* For PRIx64 */
#endif                /* _MSC_VER */

#include <wchar.h>

/** \defgroup brlapi_handles BrlAPI handles
 *
 * Each function provided by BrlAPI comes in two versions.
 *
 * 1. A version whose name is prefixed by brlapi_ for clients opening only
 * one simultaneous connection with BrlAPI (most frequen case)
 *
 * 2. A version whose name is prefixed by brlapi__ for use by clients
 * wishing to open more than one connection to BrlAPI.
 *
 * A function called brlapi__foo is used in exactly the same way as its
 * brlapi_foo counterpart, except that it takes an additional argument
 * (the first one), which is a handle letting the client refer to a given
 * connection in a similar manner to what file descriptors do.
 *
 * In case you want to check that your code is not erroneously using brlapi_foo
 * functions, define BRLAPI_NO_SINGLE_SESSION before including <brlapi.h>: that
 * will disable the declaration of all single session functions.
 *
 * @{ */

/** Type for BrlAPI hanles */
typedef struct brlapi_handle_t brlapi_handle_t;

/** Returns the size of an object of type brlapi_handle_t in bytes */
size_t BRLAPI_STDCALL brlapi_getHandleSize(void);

/** @} */

/** \defgroup brlapi_connection Connecting to BrlAPI
 *
 * Before calling any other function of the library, calling
 * brlapi_openConnection() is needed to establish a connection to
 * \e BrlAPI 's server.
 * When the connection is not needed any more, brlapi_closeConnection() must be
 * called to close the connection.
 *
 * @{ */

/** Default port number on which connections to \e BrlAPI can be established */
#define BRLAPI_SOCKETPORTNUM 4101
#define BRLAPI_SOCKETPORT "4101"

/** Default unix path on which connections to \e BrlAPI can be established */
#define BRLAPI_SOCKETPATH "/run/brltty/BrlAPI"

/** \e brltty 's settings directory
 *
 * This is where authorization key and driver-dependent key names are found
 * for instance. */
#define BRLAPI_ETCDIR "/etc"

/** Default name of the file containing \e BrlAPI 's authorization key
 *
 * This name is relative to BRLAPI_ETCDIR */
#define BRLAPI_AUTHKEYFILE "brlapi.key"

/** Default authorization setting */
#ifdef BRLAPI_WIN32
/* No authentication by default on Windows */
#define BRLAPI_DEFAUTH "none"
#else /* BRLAPI_WIN32 */
#define BRLAPI_DEFAUTH_KEYFILE "keyfile:" BRLAPI_ETCDIR "/" BRLAPI_AUTHKEYFILE

#ifdef USE_POLKIT
#define BRLAPI_DEFAUTH_POLKIT "+polkit"
#else /* USE_POLKIT */
#define BRLAPI_DEFAUTH_POLKIT ""
#endif /* USE_POLKIT */

#define BRLAPI_DEFAUTH BRLAPI_DEFAUTH_KEYFILE BRLAPI_DEFAUTH_POLKIT
#endif /* BRLAPI_WIN32 */

#ifdef __MINGW32__
typedef HANDLE brlapi_fileDescriptor;
#else  /* __MINGW32__ */
typedef int brlapi_fileDescriptor;
#endif /* __MINGW32__ */

/** \brief Settings structure for \e BrlAPI connection
 *
 * This structure holds every parameter needed to connect to \e BrlAPI: which
 * file the authorization key can be found in and which computer to connect to.
 *
 * \par Examples:
 * \code
 * brlapi_connectionSettings_t settings;
 *
 * settings.auth="/etc/brlapi.key";
 * settings.host="foo";
 * \endcode
 *
 * \e libbrlapi will read authorization key from file \p /etc/brlapi.key
 * and connect to the machine called "foo", on the default TCP port.
 *
 * \code
 * settings.host="10.1.0.2";
 * \endcode
 *
 * lets directly enter an IP address instead of a machine name.
 *
 * \code
 * settings.host=":1";
 * \endcode
 *
 * lets \e libbrlapi connect to the local computer, on port
 * BRLAPI_SOCKETPORTNUM+1
 *
 * \sa brlapi_openConnection()
 */
typedef struct {
  /** For security reasons, \e libbrlapi has to get authorized to connect to the
   * \e BrlAPI server. This can be done via a secret key, for instance. This is
   * the path to the file which holds it; it will hence have to be readable by
   * the application.
   *
   * Setting \c NULL defaults it to local installation setup or to the content
   * of the BRLAPI_AUTH environment variable, if it exists. */
  const char* auth;

  /** This tells where the \e BrlAPI server resides: it might be listening on
   * another computer, on any TCP port. It should look like "foo:1", which
   * means TCP port number BRLAPI_SOCKETPORTNUM+1 on computer called "foo".
   * \note Please check that resolving this name works before complaining
   *
   * Settings \c NULL defaults it to localhost, using the local installation's
   * default TCP port, or to the content of the BRLAPI_HOST environment
   * variable, if it exists. */
  const char* host;
} brlapi_connectionSettings_t;

/* BRLAPI_SETTINGS_INITIALIZER */
/** Allows to initialize a structure of type \e brlapi_connectionSettings_t *
 * with default values. */
#define BRLAPI_SETTINGS_INITIALIZER \
  { NULL, NULL }

/* brlapi_openConnection */
/** Open a socket and connect it to \e BrlAPI 's server
 *
 * This function first loads an authorization key as specified in settings.
 * It then creates a TCP socket and connects it to the specified machine, on
 * the specified port. It writes the authorization key on the socket and
 * waits for acknowledgement.
 *
 * \return the file descriptor, or -1 on error
 *
 * \note The file descriptor is returned in case the client wants to
 * communicate with the server without using \e libbrlapi functions. If it uses
 * them however, it won't have to pass the file descriptor later, since the
 * library keeps a copy of it. But that also means that
 * brlapi_openConnection() may be called several times, but \e libbrlapi
 * functions will always work with the last call's descriptor
 *
 * \par Example:
 * \code
 * if (brlapi_openConnection(&settings,&settings)<0) {
 *  fprintf(stderr,"couldn't connect to BrlAPI at %s: %s\n",
 *   settings.host, brlapi_strerror(&brlapi_error));
 *  exit(1);
 * }
 * \endcode
 *
 * \par Errors:
 * \e BrlAPI might not be on this TCP port, the host name might not be
 * resolvable, the authorization may fail,...
 *
 * \param desiredSettings this gives the desired connection parameters, as
 * described in brlapi_connectionSettings_t. If \c NULL, defaults values are
 * used, so that it is generally a good idea to give \c NULL as default, and
 * only fill a brlapi_connectionSettings_t structure when the user gave
 * parameters to the program for instance; \param actualSettings if not \c NULL,
 * parameters which were actually used are stored here, if the application ever
 * needs them.
 *
 * \sa
 * brlapi_connectionSettings_t
 * brlapi_writePacket()
 * brlapi_readPacketHeader()
 * brlapi_readPacketContent()
 * brlapi_readPacket()
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
brlapi_fileDescriptor BRLAPI_STDCALL
brlapi_openConnection(const brlapi_connectionSettings_t* desiredSettings,
                      brlapi_connectionSettings_t* actualSettings);
#endif /* BRLAPI_NO_SINGLE_SESSION */
brlapi_fileDescriptor BRLAPI_STDCALL
brlapi__openConnection(brlapi_handle_t* handle,
                       const brlapi_connectionSettings_t* desiredSettings,
                       brlapi_connectionSettings_t* actualSettings);

/* brlapi_closeConnection */
/** Cleanly close the socket
 *
 * This function locks until a closing acknowledgement is received from the
 * server. The socket is then freed, so the file descriptor
 * brlapi_openConnection() gave has no meaning any more
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
void BRLAPI_STDCALL brlapi_closeConnection(void);
#endif /* BRLAPI_NO_SINGLE_SESSION */
void BRLAPI_STDCALL brlapi__closeConnection(brlapi_handle_t* handle);

/** @} */

/** \defgroup brlapi_clientData Setting and getting client data
 *
 * Clients can register a pointer to data that can then be used
 * e.g. in exception handlers.
 * @{ */

/* brlapi__setClientData */
/** Register a pointer to client data
 *
 * \param data The pointer to the private client data
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
void BRLAPI_STDCALL brlapi_setClientData(void* data);
#endif /* BRLAPI_NO_SINGLE_SESSION */
void BRLAPI_STDCALL brlapi__setClientData(brlapi_handle_t* handle, void* data);

/* brlapi__getClientData */
/** Retrieves the pointer to the private client data
 *
 * \return the pointer to the private client data
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
void* BRLAPI_STDCALL brlapi_getClientData(void);
#endif /* BRLAPI_NO_SINGLE_SESSION */
void* BRLAPI_STDCALL brlapi__getClientData(brlapi_handle_t* handle);

/** @} */

/** \defgroup brlapi_info Getting Terminal information
 * \brief How to get information about the connected Terminal
 *
 * Before using Raw mode or key codes, the application should always check the
 * type of the connected terminal, to be sure it is really the one it expects.
 *
 * One should also check for display size, so as to adjust further displaying
 * on it.
 * @{
 */

/** Maximum name length for names embeded in BrlAPI packets, not counting any
 * termination \\0 character */
#define BRLAPI_MAXNAMELENGTH 31

/* brlapi_getDriverName */
/** Return the complete name of the driver used by \e brltty
 *
 * This function fills its argument with the whole name of the braille
 * driver if available, terminated with a '\\0'.
 *
 * \param buffer is the buffer provided by the application;
 * \param size is the maximum size for the name buffer.
 *
 * \return -1 on error, otherwise a positive value giving the size of the needed
 * buffer (including '\\0'). If that value is bigger than \p size, the value was
 * truncated and the caller should retry with a bigger buffer accordingly.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_getDriverName(char* buffer, size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__getDriverName(brlapi_handle_t* handle,
                                         char* buffer,
                                         size_t size);

/* brlapi_getModelIdentifier */
/** Return an identifier for the device model used by \e brltty
 *
 * This function fills its argument with the whole identifier of the braille
 * device model if available, terminated with a '\\0'.
 *
 * \param buffer is the buffer given by the application;
 * \param size is the maximum size for the identifier buffer.
 *
 * \return -1 on error, otherwise a positive value giving the size of the needed
 * buffer (including '\\0'). If that value is bigger than \p size, the value was
 * truncated and the caller should retry with a bigger buffer accordingly.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_getModelIdentifier(char* buffer, size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__getModelIdentifier(brlapi_handle_t* handle,
                                              char* buffer,
                                              size_t size);

/* brlapi_getDisplaySize */
/** Return the size of the braille display */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_getDisplaySize(unsigned int* x, unsigned int* y);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__getDisplaySize(brlapi_handle_t* handle,
                                          unsigned int* x,
                                          unsigned int* y);

/** @} */

/** \defgroup brlapi_tty Entering & leaving tty mode
 * \brief How to take control of ttys for direct braille display / read
 *
 * Before being able to write on the braille display, the application must tell
 * the server which tty it will handle.
 *
 * The application must also specify how braille keys will be delivered to it.
 * Two ways are possible: key codes and commands:
 *
 * - key codes are specific to each braille driver, since the raw key code, as
 *   defined in the driver will be given for each key press.
 *   Using them leads to building highly driver-dependent applications, which
 *   can yet sometimes be useful to mimic existing proprietary applications
 *   for instance.
 * - commands means that applications will get exactly the same values as
 *   \e brltty. This allows driver-independent clients, which will hopefully
 *   be nice to use with a lot of different terminals.
 * \sa brlapi_readKey()
 * @{
 */

/* brlapi_enterTtyMode */
/** Ask for some tty, with some key mechanism
 *
 * \param tty
 * - If tty>=0 then take control of the specified tty.
 * - If tty==::BRLAPI_TTY_DEFAULT then take control of the default tty.
 *
 * \param driver tells how the application wants brlapi_readKey() to return
 * key presses. NULL or "" means BRLTTY commands are required,
 * whereas a driver name means that raw key codes returned by this
 * driver are expected.
 *
 * WINDOWPATH and WINDOWID should be propagated when running remote applications
 * via ssh, for instance, along with BRLAPI_HOST and the authorization key (see
 * SendEnv in ssh_config(5) and AcceptEnv in sshd_config(5))
 *
 * \return the used tty number on success, -1 on error
 *
 * \sa brlapi_leaveTtyMode() brlapi_readKey()
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_enterTtyMode(int tty, const char* driver);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__enterTtyMode(brlapi_handle_t* handle,
                                        int tty,
                                        const char* driver);

/** Select the default tty.
 *
 * The library takes the following steps:
 * -# Try to get the tty number from the \c WINDOWID environment variable (for
 * the \c xterm case).
 * -# Try to get the tty number from the \c CONTROLVT environment variable.
 * -# Read \c /proc/self/stat (on \c Linux).
 *
 * \sa brlapi_enterTtyMode()
 */
#define BRLAPI_TTY_DEFAULT -1

/* brlapi_enterTtyModeWithPath */
/** Ask for some tty specified by its path in the tty tree, with some key
 * mechanism
 *
 * \param ttys points on the array of ttys representing the tty path to be got.
 * Can be NULL if nttys is 0.
 * \param count gives the number of elements in ttys.
 * \param driver has the same meaning as in brlapi_enterTtyMode()
 *
 * Providing nttys == 0 means to get the root.
 *
 * \sa brlapi_enterTtyMode()
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_enterTtyModeWithPath(int* ttys,
                                               int count,
                                               const char* driver);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__enterTtyModeWithPath(brlapi_handle_t* handle,
                                                int* ttys,
                                                int count,
                                                const char* driver);

/* brlapi_leaveTtyMode */
/** Stop controlling the tty
 *
 * \return 0 on success, -1 on error.
 *
 * \sa brlapi_enterTtyMode()
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_leaveTtyMode(void);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__leaveTtyMode(brlapi_handle_t* handle);

/* brlapi_setFocus */
/** Tell the current tty to brltty
 *
 * This is intended for focus tellers, such as brltty, xbrlapi, screen, ...
 * brlapi_enterTtyMode() must have been called beforehand to tell where this
 * focus applies in the tty tree.
 *
 * \return 0 on success, -1 on error.
 *
 * \sa brlapi_enterTtyMode() brlapi_leaveTtyMode()
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_setFocus(int tty);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__setFocus(brlapi_handle_t* handle, int tty);

/** @} */

/** \defgroup brlapi_write Writing on the braille display
 * \brief Write text to the braille display
 *
 * After brlapi_enterTtyMode() has been called, the application can
 * call one of these functions to write things on the braille display.
 *
 * \note Be sure to call brlapi_enterTtyMode() \e before calling brlapi_write(),
 * or else you'll get an error. This is particularly not always trivial when
 * writing multithreaded applications.
 *
 * \note Dots are coded as described in ISO/TR 11548-1: a dot pattern is coded
 * by a byte in which bit 0 is set iff dot 1 is up, bit 1 is set iff dot 2 is
 * up, ... bit 7 is set iff dot 8 is up. This also corresponds to the low-order
 * byte of the coding of unicode's braille row U+2800.
 *
 * \note Text is translated by the server one to one, by just using a simple
 * wchar_t to pattern table, i.e. no contraction/expansion is performed, because
 * the client would then have no way to know how wide the output would be and
 * thus the quantity of text to feed.  If contraction/expansion is desired, the
 * client should perform it itself (e.g. thanks to liblouis or gnome-braille)
 * and send the resulting dot patterns.  This is actually exactly the same
 * problem as font rendering on a graphical display: for better control,
 * nowadays all font rasterization is performed on the client side, and mere
 * pixmaps are sent to the X server.
 *
 * @{ */

/* brlapi_writeText */
/** Write the given \\0-terminated string to the braille display
 *
 * If the string is too long, it is truncated. If it's too short,
 * it is padded with spaces. The text is assumed to be in the current
 * locale charset set by setlocale(3) if it was called, or the locale charset
 * from the locale environment variables if setlocale(3) was not called.
 *
 * \param cursor gives the cursor position; if equal to ::BRLAPI_CURSOR_OFF, no
 * cursor is shown at all; if cursor==::BRLAPI_CURSOR_LEAVE, the cursor is left
 * where it is
 *
 * \param text points to the string to be displayed.
 *
 * \return 0 on success, -1 on error.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_writeText(int cursor, const char* text);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__writeText(brlapi_handle_t* handle,
                                     int cursor,
                                     const char* text);

/* brlapi_writeWText */
/** Write the given \\0-terminated unicode string to the braille display
 *
 * If the string is too long, it is truncated. If it's too short,
 * it is padded with spaces.
 *
 * \param cursor gives the cursor position; if equal to ::BRLAPI_CURSOR_OFF, no
 * cursor is shown at all; if cursor==::BRLAPI_CURSOR_LEAVE, the cursor is left
 * where it is
 *
 * \param text points to the string to be displayed.
 *
 * \return 0 on success, -1 on error.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_writeWText(int cursor, const wchar_t* text);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__writeWText(brlapi_handle_t* handle,
                                      int cursor,
                                      const wchar_t* text);

/* brlapi_writeDots */
/** Write the given dots array to the display
 *
 * \param dots points on an array of dot information, one per character. Its
 * size must hence be the same as what brlapi_getDisplaySize() returns.
 *
 * \return 0 on success, -1 on error.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_writeDots(const unsigned char* dots);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__writeDots(brlapi_handle_t* handle,
                                     const unsigned char* dots);

/* brlapi_writeArguments_t */
/** Structure containing arguments to be given to brlapi_write() */
typedef struct {
  int displayNumber /** Display number ::BRLAPI_DISPLAY_DEFAULT == unspecified
                     */
      ;
  unsigned int regionBegin /** Region of display to update, 1st character of
                              display is 1 */
      ;
  unsigned int
      regionSize /** Number of characters held in text, andMask and orMask. */;
  char* text /** Text to display, must hold as many characters as the region
                fields expresses. */
      ;
  int textSize /** Size of text in bytes. If -1, strlen() is used for computing
                  it. */
      ;
  unsigned char* andMask /** And attributes; applied first */;
  unsigned char* orMask /** Or attributes; applied \e after ANDing */;
  int cursor /** ::BRLAPI_CURSOR_LEAVE == don't touch, ::BRLAPI_CURSOR_OFF ==
                turn off, 1 = 1st char of display, ... */
      ;
  char* charset /** Text charset. NULL means it is assumed to be 8bits, and the
                   same as the server's. "" means current locale's charset. If
                   no locale was selected, defaults to NULL's meaning. */
      ;
} brlapi_writeArguments_t;

/** Write to the default display on the braille device.
 *
 * \sa brlapi_write() brlapi_writeArguments_t
 */
#define BRLAPI_DISPLAY_DEFAULT -1

/** Do not change the cursor's state or position.
 *
 * \sa brlapi_writeText() brlapi_write() brlapi_writeArguments_t
 */
#define BRLAPI_CURSOR_LEAVE -1

/** Do not display the cursor.
 *
 * \sa brlapi_writeText() brlapi_write() brlapi_writeArguments_t
 */
#define BRLAPI_CURSOR_OFF 0

/* BRLAPI_WRITEARGUMENTS_INITIALIZER */
/** Allows to initialize a structure of type \e brlapi_writeArguments_t *
 * with default values:
 * displayNumber = ::BRLAPI_DISPLAY_DEFAULT; (unspecified)
 * regionBegin = regionSize = 0; (update the whole display, DEPRECATED and will
 * be forbidden in next release. You must always express the region you wish to
 * update)
 * text = andMask = orMask = NULL; (no text, no attribute)
 * cursor = ::BRLAPI_CURSOR_LEAVE; (don't touch cursor)
 */
#define BRLAPI_WRITEARGUMENTS_INITIALIZER                           \
  {                                                                 \
    .displayNumber = BRLAPI_DISPLAY_DEFAULT, .regionBegin = 0,      \
    .regionSize = 0, .text = NULL, .textSize = -1, .andMask = NULL, \
    .orMask = NULL, .cursor = BRLAPI_CURSOR_LEAVE, .charset = NULL  \
  }

/* brlapi_write */
/** Update a specific region of the braille display and apply and/or masks
 *
 * \param arguments gives information necessary for the update
 *
 * regionBegin and regionSize must be filled for specifying which part of the
 * display will be updated, as well as the size (in characters, not bytes) of
 * the text, andMask and orMask members.
 *
 * If given, the "text" field holds the text that will be displayed in the
 * region.  The char string must hold exactly as many characters as the region
 * fields express.  For multibyte text, this is the number of \e multibyte
 * caracters.  Notably, combining and double-width caracters count for 1.
 *
 * The actual length of the text in \e bytes may be specified thanks to
 * textSize.  If -1 is given, it will be computed thanks to strlen(), so "text"
 * must then be a NUL-terminated string.
 *
 * The "andMask" and "orMask" masks, if present, are then applied on top of
 * the text, one byte per character.  This hence permits the superimposing of
 * attributes over the text.  For instance, setting an andMask mask full of
 * BRLAPI_DOTS(1,1,1,1,1,1,0,0) will only keep (logical AND) dots 1-6,
 * hence dropping dots 7 and 8.  On the contrary, setting an orMask full of
 * BRLAPI_DOT7|BRLAPI_DOT8 will add (logical OR) dots 7 and 8.
 *
 * The "charset" field, if present, specifies the charset of the "text" field.
 * If it is "", the current locale's charset (if any) is assumed.  Else, the
 * 8-bit charset of the server is assumed.
 *
 * A special invocation is with an unmodified initialized structure: this clears
 * the client's whole display, letting the display of other applications on
 * the same tty or of applications "under" the tty appear. See Concurrency
 * management section of the BrlAPI documentation for more details.
 *
 * \return 0 on success, -1 on error.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_write(const brlapi_writeArguments_t* arguments);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__write(brlapi_handle_t* handle,
                                 const brlapi_writeArguments_t* arguments);

/** @} */

#include "brlapi_keycodes.h"

/** \defgroup brlapi_keys Reading key presses
 * \brief How to read key presses from the braille terminal
 *
 * Once brlapi_enterTtyMode() has been called, the application can call
 * brlapi_readKey() to read key presses. Either key codes (see \ref
 * brlapi_keycodes) or commands will be returned, depending on parameters given
 * to brlapi_enterTtyMode().
 *
 * Key presses are buffered, so that calling brlapi_readKey() in non-blocking
 * mode from time to time should suffice.
 *
 * @{
 */

/* brlapi_expandedKeyCode_t */
/** Structure holding the components of a key code as returned by
 * brlapi_expandKeyCode() */
typedef struct {
  unsigned int type /** the type value */;
  unsigned int command /** the command value */;
  unsigned int argument /** the argument value */;
  unsigned int flags /** the flags value */;
} brlapi_expandedKeyCode_t;

/* brlapi_expandKeyCode */
/** Expand the components of a key code
 *
 * \param code the key code to be expanded
 * \param expansion pointer to the structure that receives the components
 *
 * \return 0 on success, -1 on error
 */
int BRLAPI_STDCALL brlapi_expandKeyCode(brlapi_keyCode_t code,
                                        brlapi_expandedKeyCode_t* expansion);

/* brlapi_describedKeyCode_t */
/** Structure holding the components of a key code as returned by
 * brlapi_describeKeyCode() */
typedef struct {
  const char* type /** the  type name */;
  const char* command /** the command name */;
  unsigned int argument /** the argument value */;
  unsigned int flags /** the flag count */;
  const char* flag[64 - BRLAPI_KEY_FLAGS_SHIFT] /** the flag names */;
  brlapi_expandedKeyCode_t values /** the actual values of the components */;
} brlapi_describedKeyCode_t;

/* brlapi_describeKeyCode */
/** Describe the components of a key code.
 *
 * \param code the keycode to be described
 * \param description pointer to the structure that receives the description
 *
 * \return 0 on success, -1 on error
 */
int BRLAPI_STDCALL
brlapi_describeKeyCode(brlapi_keyCode_t code,
                       brlapi_describedKeyCode_t* description);

/** Unicode braille row */
#define BRLAPI_UC_ROW 0x2800UL

/* brlapi_readKey */
/** Read a key from the braille keyboard
 *
 * This function returns one key press's code.
 *
 * If NULL or "" was given to brlapi_enterTtyMode(), a \e brltty command is
 * returned, as described in the documentation for ::brlapi_keyCode_t . It is
 * hence pretty driver-independent, and should be used by default when no other
 * option is possible.
 *
 * By default, all commands but those which restart drivers and switch
 * virtual terminals are returned to the application and not to brltty.
 * If the application doesn't want to see some command events,
 * it should call brlapi_ignoreKeys()
 *
 * If some driver name was given to brlapi_enterTtyMode(), a raw keycode is
 * returned, as specified by the terminal driver, usually in <brltty/brldefs-xy>
 * where xy is the driver's code. It generally corresponds to the very code that
 * the terminal tells to the driver. This should only be used by applications
 * which are dedicated to a particular braille terminal. Hence, checking the
 * terminal type thanks to a call to brlapi_getDriverName() before getting tty
 * control is a pretty good idea.
 *
 * By default, all the keypresses will be passed to the client, none will go
 * through brltty, so the application will have to handle console switching
 * itself for instance.
 *
 * \param wait tells whether the call should block until a key is pressed (1)
 *  or should only probe key presses (0);
 * \param code holds the key code if a key press is indeed read.
 *
 * \return -1 on error or signal interrupt and *code is then undefined, 0 if
 * block was 0 and no key was pressed so far, or 1 and *code holds the key code.
 *
 * Programming hints:
 *
 * If your application is only driven by braille command keypresses, you can
 * just call brlapi_readKey(1, &code) so that it keeps blocking, waiting for
 * keypresses.
 *
 * Else, you'll probably want to use the file descriptor returned by
 * brlapi_openConnection() in your "big polling loop". For instance:
 *
 * - in a \c select() loop, just add it to the \c readfds and \c exceptfds file
 *   descriptor sets;
 * - in a gtk or atspi application, use
 *   \c g_io_add_watch(fileDescriptor, \c G_IO_IN|G_IO_ERR|G_IO_HUP, \c f, \c
 * data) for adding a callback called \c f;
 * - in an Xt/Xaw/motif-based application, use
 *   \c XtAppAddInput(app_context, \c fileDescriptor, \c
 * XtInputReadMask|XtInputExceptMask, \c f, \c data)
 * - etc.
 *
 * and then, when you detect inbound trafic on the file descriptor, do something
 * like this:
 *
 * while (brlapi_readKey(0, &code) {
 *    // process keycode code
 *    // ...
 * }
 *
 * The \c while loop is needed for processing \e all pending key presses, else
 * some of them may be left in libbrlapi's internal key buffer and you wouldn't
 * get them immediately.
 *
 * \note If the read is interrupted by a signal, brlapi_readKey() will return
 * -1, brlapi_errno will be BRLAPI_ERROR_LIBCERR and errno will be EINTR.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_readKey(int wait, brlapi_keyCode_t* code);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__readKey(brlapi_handle_t* handle,
                                   int wait,
                                   brlapi_keyCode_t* code);

/* brlapi_readKeyWithTimeout */
/** Read a key from the braille keyboard, unless a timeout expires
 *
 * This function works like brlapi_readKey, except that parameter \e wait is
 * replaced by a \e timeout_ms parameter
 *
 * \param timeout_ms specifies how long the function should wait for a keypress.
 * \param code holds the key code if a key press is indeed read.
 *
 * \return -1 on error or signal interrupt and *code is then undefined, 0 if
 * the timeout expired and no key was pressed, or 1 and *code holds the key
 * code.
 *
 * If the timeout expires without any key being pressed, 0 is returned.
 *
 * If timeout_ms is set to 0, this function looks for key events that have been
 * already received, but does not wait at all if no event was received.
 *
 * If timeout_ms is set to a negative value, this function behaves like
 * brlapi_readKey, i.e. it uses an infinite timeout.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_readKeyWithTimeout(int timeout_ms,
                                             brlapi_keyCode_t* code);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__readKeyWithTimeout(brlapi_handle_t* handle,
                                              int timeout_ms,
                                              brlapi_keyCode_t* code);

/** types of key ranges */
typedef enum {
  brlapi_rangeType_all,     /**< all keys, code must be 0 */
  brlapi_rangeType_type,    /**< all keys of a given type */
  brlapi_rangeType_command, /**< all keys of a given command block, i.e.
                               matching the key type and command parts */
  brlapi_rangeType_key,     /**< a given key with any flags */
  brlapi_rangeType_code,    /**< a given key code */
} brlapi_rangeType_t;

/* brlapi_ignoreKeys */
/** Ignore some key presses from the braille keyboard
 *
 * This function asks the server to give the provided keys to \e brltty, rather
 * than returning them to the application via brlapi_readKey().
 *
 * \param type type of keys to be ignored
 * \param keys array of keys to be ignored
 * \param count number of keys
 *
 * \note The given codes should be \e brltty commands (NULL or "" was given to
 * brlapi_enterTtyMode())
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_ignoreKeys(brlapi_rangeType_t type,
                                     const brlapi_keyCode_t keys[],
                                     unsigned int count);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__ignoreKeys(brlapi_handle_t* handle,
                                      brlapi_rangeType_t type,
                                      const brlapi_keyCode_t keys[],
                                      unsigned int count);

/* brlapi_acceptKeys */
/** Accept some key presses from the braille keyboard
 *
 * This function asks the server to give the provided keys to the application,
 * and not give them to \e brltty.
 *
 * \param type type of keys to be ignored
 * \param keys array of keys to be ignored
 * \param count number of keys
 *
 * \note The given codes should be \e brltty commands (NULL or "" was given to
 * brlapi_enterTtyMode())
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_acceptKeys(brlapi_rangeType_t type,
                                     const brlapi_keyCode_t keys[],
                                     unsigned int count);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__acceptKeys(brlapi_handle_t* handle,
                                      brlapi_rangeType_t type,
                                      const brlapi_keyCode_t keys[],
                                      unsigned int count);

/* brlapi_ignoreAllKeys */
/** Ignore all key presses from the braille keyboard
 *
 * This function asks the server to give all keys to \e brltty, rather than
 * returning them to the application via brlapi_readKey().
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_ignoreAllKeys(void);
#define brlapi_ignoreAllKeys() brlapi_ignoreKeys(brlapi_rangeType_all, NULL, 0)
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__ignoreAllKeys(brlapi_handle_t* handle);
#define brlapi__ignoreAllKeys(handle) \
  brlapi__ignoreKeys(handle, brlapi_rangeType_all, NULL, 0)

/* brlapi_acceptAllKeys */
/** Accept all key presses from the braille keyboard
 *
 * This function asks the server to give all keys to the application, and not
 * give them to \e brltty.
 *
 * Warning: after calling this function, make sure to call brlapi_ignoreKeys()
 * for ignoring important keys like BRL_CMD_SWITCHVT_PREV/NEXT and such.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_acceptAllKeys(void);
#define brlapi_acceptAllKeys() brlapi_acceptKeys(brlapi_rangeType_all, NULL, 0)
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__acceptAllKeys(brlapi_handle_t* handle);
#define brlapi__acceptAllKeys(handle) \
  brlapi__acceptKeys(handle, brlapi_rangeType_all, NULL, 0)

/** Type for raw keycode ranges
 *
 * Denotes the set of keycodes between \e first and \e last (inclusive)
 */
typedef struct {
  brlapi_keyCode_t first; /**< first key of the range */
  brlapi_keyCode_t last;  /**< last key of the range */
} brlapi_range_t;

/* brlapi_ignoreKeyRanges */
/** Ignore some key presses from the braille keyboard
 *
 * This function asks the server to give the provided key ranges to \e brltty,
 * rather than returning them to the application via brlapi_readKey().
 *
 * \param ranges key ranges, which are inclusive
 * \param count number of ranges
 *
 * \note The given codes should be raw keycodes (i.e. some driver name was given
 * to brlapi_enterTtyMode()) */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_ignoreKeyRanges(const brlapi_range_t ranges[],
                                          unsigned int count);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__ignoreKeyRanges(brlapi_handle_t* handle,
                                           const brlapi_range_t ranges[],
                                           unsigned int count);

/* brlapi_acceptKeyRanges */
/** Accept some key presses from the braille keyboard
 *
 * This function asks the server to return the provided key ranges (inclusive)
 * to the application, and not give them to \e brltty.
 *
 * \param ranges key ranges, which are inclusive
 * \param count number of ranges
 *
 * \note The given codes should be raw keycodes (i.e. some driver name was given
 * to brlapi_enterTtyMode()) */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_acceptKeyRanges(const brlapi_range_t ranges[],
                                          unsigned int count);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__acceptKeyRanges(brlapi_handle_t* handle,
                                           const brlapi_range_t ranges[],
                                           unsigned int count);
/** @} */

/** \defgroup brlapi_driverspecific Driver-Specific modes
 * \brief Raw and Suspend Modes mechanism
 *
 * If the application wants to directly talk to the braille terminal, it should
 * use Raw Mode. In this special mode, the driver gives the whole control of the
 * terminal to it: \e brltty doesn't work any more.
 *
 * For this, it simply has to call brlapi_enterRawMode(), then brlapi_sendRaw()
 * and brlapi_recvRaw(), and finally give control back thanks to
 * brlapi_leaveRawMode().
 *
 * Special care of the terminal should be taken, since one might completely
 * trash the terminal's data, or even lock it! The application should always
 * check for terminal's type thanks to brlapi_getDriverName().
 *
 * The client can also make brltty close the driver by using
 * brlapi_suspendDriver(), and resume it again with brlapi_resumeDriver().  This
 * should not be used if possible: raw mode should be sufficient for any use. If
 * not, please ask for features :)
 *
 * @{
 */

/* brlapi_enterRawMode */
/** Switch to Raw mode
 * \param driver Specifies the name of the driver for which the raw
 * communication will be established.
 * \return 0 on success, -1 on error */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_enterRawMode(const char* driver);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__enterRawMode(brlapi_handle_t* handle,
                                        const char* driver);

/* brlapi_leaveRawMode */
/** Leave Raw mode
 * \return 0 on success, -1 on error */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_leaveRawMode(void);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__leaveRawMode(brlapi_handle_t* handle);

/* brlapi_sendRaw */
/** Send Raw data
 *
 * \param buffer points on the data;
 * \param size holds the packet size.
 * \return size on success, -1 on error */
#ifndef BRLAPI_NO_SINGLE_SESSION
ssize_t BRLAPI_STDCALL brlapi_sendRaw(const void* buffer, size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
ssize_t BRLAPI_STDCALL brlapi__sendRaw(brlapi_handle_t* handle,
                                       const void* buffer,
                                       size_t size);

/* brlapi_recvRaw */
/** Get Raw data
 *
 * \param buffer points on a buffer where the function will store the received
 * data;
 * \param size holds the buffer size.
 * \return its size, -1 on error or signal interruption */
#ifndef BRLAPI_NO_SINGLE_SESSION
ssize_t BRLAPI_STDCALL brlapi_recvRaw(void* buffer, size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
ssize_t BRLAPI_STDCALL brlapi__recvRaw(brlapi_handle_t* handle,
                                       void* buffer,
                                       size_t size);

/* brlapi_suspendDriver */
/** Suspend braille driver
 * \param driver Specifies the name of the driver which will be suspended.
 * \return -1 on error
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_suspendDriver(const char* driver);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__suspendDriver(brlapi_handle_t* handle,
                                         const char* driver);

/* brlapi_resumeDriver */
/** Resume braille driver
 * \return -1 on error
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_resumeDriver(void);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__resumeDriver(brlapi_handle_t* handle);
/** @} */

#include "brlapi_param.h"

/** \defgroup brlapi_parameterManagement Parameter management
 * \brief How to manage BrlAPI parameters
 *
 * There are several kinds of parameters:
 * - states associated with the braille device itself, such as its size or
 * parameters of the device port
 * - states of the BrlAPI connection itself, such as the displaying level or
 * key passing preferences.
 * - general states such as the cut buffer,
 * - braille parameters: braille table, contraction, cursor shape, etc,
 * - browse parameters: line skip, beep, etc.
 *
 * Some of them are subdivided in subparameters.  Others have only subparameter
 * 0.
 *
 * Some of them are read-only, others are read/write.
 *
 * A client can either request the immediate content of a parameter by
 * using brlapi_getParameter(); set the content of a parameter by using
 * brlapi_setParameter(); or register a callback that may be called immediately
 * and on each change of a given parameter, by using brlapi_watchParameter().
 *
 * @{ */

/** Flags for parameter requests */
typedef uint32_t brlapi_param_flags_t;
#define BRLAPI_PARAMF_LOCAL                                                  \
  0X00 /**< Refer to the value local to the connection instead of the global \
          value */
#define BRLAPI_PARAMF_GLOBAL                                            \
  0X01 /**< Refer to the global value instead of the value local to the \
          connection */
#define BRLAPI_PARAMF_SELF \
  0X02 /**< Specify whether to receive notifications of value self-changes */

/* brlapi_getParameter */
/** Get the content of a parameter
 *
 * brlapi_getParameter gets the current content of a parameter
 *
 * \param parameter is the parameter whose content shall be gotten;
 * \param subparam is a specific instance of the parameter;
 * \param flags specify which value and how it should be returned;
 * \param data is a buffer where content of the parameter shall be stored;
 * \param len is the size of the buffer.
 *
 * \return the real size of the parameter's content. If the parameter does not
 * fit in the provided buffer, it is truncated to len bytes (but the real size
 * of the parameter is still returned). In that case, the client must call
 * brlapi_getParameter again with a big enough buffer.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
ssize_t BRLAPI_STDCALL brlapi_getParameter(brlapi_param_t parameter,
                                           brlapi_param_subparam_t subparam,
                                           brlapi_param_flags_t flags,
                                           void* data,
                                           size_t len);
#endif
ssize_t BRLAPI_STDCALL brlapi__getParameter(brlapi_handle_t* handle,
                                            brlapi_param_t parameter,
                                            brlapi_param_subparam_t subparam,
                                            brlapi_param_flags_t flags,
                                            void* data,
                                            size_t len);

/* brlapi_getParameterAlloc */
/** Return the content of a parameter
 *
 * brlapi_getParameterAlloc gets the current content of a parameter, by
 * returning it as a newly-allocated buffer. The buffer is allocated to one byte
 * more than the parameter value. This byte is set to zero. This allows, for
 * string parameters, to be able to immediately use it as a C string.
 *
 * \param parameter is the parameter whose content shall be gotten;
 * \param subparam is a specific instance of the parameter;
 * \param flags specify which value and how it should be returned;
 * \param len is the address where to store the size of the parameter value.
 *
 * \return a newly-allocated buffer that contains the value of the parameter.
 * The caller must call free() on it after use. NULL is returned on errors
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
void* BRLAPI_STDCALL brlapi_getParameterAlloc(brlapi_param_t parameter,
                                              brlapi_param_subparam_t subparam,
                                              brlapi_param_flags_t flags,
                                              size_t* len);
#endif
void* BRLAPI_STDCALL brlapi__getParameterAlloc(brlapi_handle_t* handle,
                                               brlapi_param_t parameter,
                                               brlapi_param_subparam_t subparam,
                                               brlapi_param_flags_t flags,
                                               size_t* len);

/* brlapi_setParameter */
/** Set the content of a parameter
 *
 * brlapi_setParameter sets the content of a parameter
 *
 * \param parameter is the parameter to set;
 * \param subparam is a specific instance of the parameter;
 * \param flags specify which value and how it should be set;
 * \param data is a buffer containing the data to store in the parameter;
 * \param len is the size of the data.
 *
 * \return 0 on success, -1 on error (read-only parameter for instance).
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_setParameter(brlapi_param_t parameter,
                                       brlapi_param_subparam_t subparam,
                                       brlapi_param_flags_t flags,
                                       const void* data,
                                       size_t len);
#endif
int BRLAPI_STDCALL brlapi__setParameter(brlapi_handle_t* handle,
                                        brlapi_param_t parameter,
                                        brlapi_param_subparam_t subparam,
                                        brlapi_param_flags_t flags,
                                        const void* data,
                                        size_t len);

/* brlapi_paramCallback_t */
/** Callback for parameter changes
 *
 * When a parameter gets changed, application-defined callbacks set by the
 * brlapi_watchParameter() function are called.
 *
 * \param parameter is the parameter that changed;
 * \param flags specify which value and how it was changed;
 * \param priv is the void pointer that was passed to the brlapi_watchParameter
 * call which registered the callback; \param data is a buffer containing the
 * new value of the parameter; \param len is the size of the data.
 *
 * This callback only gets called when the application calls some brlapi_
 * function (i.e. BrlAPI gets direct control of the execution).
 */
typedef void (*brlapi_paramCallback_t)(brlapi_param_t parameter,
                                       brlapi_param_subparam_t subparam,
                                       brlapi_param_flags_t flags,
                                       void* priv,
                                       const void* data,
                                       size_t len);

/* brlapi_paramCallbackDescriptor_t */
/** Type for callback descriptors
 * This is returned by brlapi_watchParameter, to be passed to
 * brlapi_unwatchParameter.
 */
typedef void* brlapi_paramCallbackDescriptor_t;

/* brlapi_watchParameter */
/** Set a parameter change callback
 *
 * brlapi_watchParameter registers a parameter change callback: whenever the
 * given parameter changes, the given function is called.
 *
 * \param parameter is the parameter to watch;
 * \param subparam is a specific instance of the parameter;
 * \param flags specify which value and how it should be monitored;
 * \param func is the function to call on parameter change;
 * \param priv is a void pointer which will be passed as such to the function;
 * \param data is a buffer where the current content of the parameter shall be
 * stored;
 * \param len is the size of the buffer.
 *
 * \return the callback descriptor (to be passed to brlapi_unwatchParameter to
 * unregister the callback), or NULL on error.
 *
 * \note Default parameter callbacks don't do anything, except the ones for
 * display size which just raise SIGWINCH.
 * \note If data is NULL, the callback will be called immediately by
 * brlapi_watchParameter, for providing the initial value
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
brlapi_paramCallbackDescriptor_t BRLAPI_STDCALL
brlapi_watchParameter(brlapi_param_t parameter,
                      brlapi_param_subparam_t subparam,
                      brlapi_param_flags_t flags,
                      brlapi_paramCallback_t func,
                      void* priv,
                      void* data,
                      size_t len);
#endif
brlapi_paramCallbackDescriptor_t BRLAPI_STDCALL
brlapi__watchParameter(brlapi_handle_t* handle,
                       brlapi_param_t parameter,
                       brlapi_param_subparam_t subparam,
                       brlapi_param_flags_t flags,
                       brlapi_paramCallback_t func,
                       void* priv,
                       void* data,
                       size_t len);

/* brlapi_unwatchParameter */
/** Clear a parameter change callback
 *
 * brlapi_unwatchParameter unregisters a parameter change callback: the
 * callback function previously registered with brlapi_watchParameter will
 * not be called any longer.
 *
 * \param descriptor refers to the callback to be removed.
 *
 * \return 0 on success, -1 on error.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL
brlapi_unwatchParameter(brlapi_paramCallbackDescriptor_t descriptor);
#endif
int BRLAPI_STDCALL
brlapi__unwatchParameter(brlapi_handle_t* handle,
                         brlapi_paramCallbackDescriptor_t descriptor);

/** @} */

/** \defgroup brlapi_misc Miscellaneous functions
 * @{ */

/* brlapi_pause */
/**
 * Waits until an event is received from the BrlAPI server
 * \param timeout_ms specifies an optional timeout which can be zero for
 * polling, or -1 for infinite wait \return nothing
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
void BRLAPI_STDCALL brlapi_pause(int timeout_ms);
#endif
void BRLAPI_STDCALL brlapi__pause(brlapi_handle_t* handle, int timeout_ms);

/** @} */

/** \defgroup brlapi_error Error handling
 * \brief How to handle errors
 *
 * When a function fails, ::brlapi_errno will hold an error
 * code to explain why it failed. It should always be reported somehow.
 *
 * Although most errors are reported that way, some (called exceptions)
 * are reported asynchronously for efficiency reasons, because they always
 * just report a programming error. The affected functions are: brlapi_setFocus,
 * brlapi_write* and brlapi_sendRaw.  When they happen, the next call to
 * brlapi_something will close the connection and call the \e exception
 * handler. If the exception handler returns, the brlapi_something function will
 * return an end-of-file error.
 *
 * The default exception handler (brlapi_defaultExceptionHandler()) dumps
 * the guilty packet before abort()ing.  It can be replaced by calling
 * brlapi_setExceptionHandler().  For instance, the Java and Python bindings use
 * this for raising a Java or Python exception that may be caught.
 *
 * @{ */

/* Error codes */
#define BRLAPI_ERROR_SUCCESS 0 /**< Success */
#define BRLAPI_ERROR_NOMEM 1   /**< Not enough memory */
#define BRLAPI_ERROR_TTYBUSY \
  2 /**< A connection is already running in this tty */
#define BRLAPI_ERROR_DEVICEBUSY \
  3 /**< A connection is already using RAW or suspend mode */
#define BRLAPI_ERROR_UNKNOWN_INSTRUCTION 4 /**< Not implemented in protocol */
#define BRLAPI_ERROR_ILLEGAL_INSTRUCTION 5 /**< Forbiden in current mode */
#define BRLAPI_ERROR_INVALID_PARAMETER 6   /**< Out of range or have no sense */
#define BRLAPI_ERROR_INVALID_PACKET 7      /**< Invalid size */
#define BRLAPI_ERROR_CONNREFUSED 8         /**< Connection refused */
#define BRLAPI_ERROR_OPNOTSUPP 9           /**< Operation not supported */
#define BRLAPI_ERROR_GAIERR 10             /**< Getaddrinfo error */
#define BRLAPI_ERROR_LIBCERR 11            /**< Libc error */
#define BRLAPI_ERROR_UNKNOWNTTY 12 /**< Couldn't find out the tty number */
#define BRLAPI_ERROR_PROTOCOL_VERSION 13 /**< Bad protocol version */
#define BRLAPI_ERROR_EOF 14              /**< Unexpected end of file */
#define BRLAPI_ERROR_EMPTYKEY 15         /**< Key file empty */
#define BRLAPI_ERROR_DRIVERERROR              \
  16 /**< Packet returned by driver too large \
      */
#define BRLAPI_ERROR_AUTHENTICATION 17 /**< Authentication failed */
#define BRLAPI_ERROR_READONLY_PARAMETER \
  18 /**< Parameter can not be changed  \
      */

/* brlapi_errlist */
/** Error message list
 *
 * These are the string constants used by brlapi_perror().
 */
extern const char* brlapi_errlist[];

/* brlapi_nerr */
/** Number of error messages */
extern const int brlapi_nerr;

/* brlapi_perror */
/** Print a BrlAPI error message
 *
 * brlapi_perror() reads ::brlapi_error, and acts just like perror().
 */
void BRLAPI_STDCALL brlapi_perror(const char* s);

/* brlapi_error_t */
/** All information that is needed to describe brlapi errors */
typedef struct {
  int brlerrno;
  int libcerrno;
  int gaierrno;
  const char* errfun;
} brlapi_error_t;

/** Get per-thread error location
 *
 * In multithreaded software, ::brlapi_error is thread-specific, so api.h
 * cheats about the brlapi_error token and actually calls
 * brlapi_error_location().
 *
 * This gets the thread specific location of global variable ::brlapi_error
 */
brlapi_error_t* BRLAPI_STDCALL brlapi_error_location(void);

/** Global variable brlapi_error
 *
 * ::brlapi_error is a global left-value containing the last error information.
 * Its errno field is not reset to BRLAPI_ERROR_SUCCESS on success.
 *
 * This information may be copied in brlapi_error_t variables for later use
 * with the brlapi_strerror function.
 */
extern brlapi_error_t brlapi_error;

/** Shorthand for brlapi_error.errno */
extern int brlapi_errno;
/** Shorthand for brlapi_error.libcerrno */
extern int brlapi_libcerrno;
/** Shorthand for brlapi_error.gaierrno */
extern int brlapi_gaierrno;
/** Shorthand for brlapi_error.errfun */
extern const char* brlapi_errfun;

/** Cheat about the brlapi_error C token */
#define brlapi_error (*brlapi_error_location())
/** Cheat about the brlapi_errno C token */
#define brlapi_errno (brlapi_error.brlerrno)
/** Cheat about the brlapi_libcerrno C token */
#define brlapi_libcerrno (brlapi_error.libcerrno)
/** Cheat about the brlapi_gaierrno C token */
#define brlapi_gaierrno (brlapi_error.gaierrno)
/** Cheat about the brlapi_errfun C token */
#define brlapi_errfun (brlapi_error.errfun)

/* brlapi_strerror */
/** Get plain error message
 *
 * brlapi_strerror() returns the plain error message corresponding to its
 * argument.
 */
const char* BRLAPI_STDCALL brlapi_strerror(const brlapi_error_t* error);

/** Type for packet type. Only unsigned can cross networks, 32bits */
typedef uint32_t brlapi_packetType_t;

/* brlapi_getPacketTypeName */
/** Get plain packet type
 *
 * brlapi_getPacketTypeName() returns the plain packet type name corresponding
 * to its argument.
 */
const char* BRLAPI_STDCALL brlapi_getPacketTypeName(brlapi_packetType_t type);

/* brlapi_exceptionHandler_t */
/** Types for exception handlers
 *
 * Types of exception handlers which are to be given to
 * brlapi_setExceptionHandler() and brlapi__setExceptionHandler().
 *
 * \param handle is the handle corresponding to the guilty connection;
 * \param error is a BRLAPI_ERROR_ error code;
 * \param type is the type of the guilty packet;
 * \param packet points to the content of the guilty packet (might be a little
 * bit truncated); \param size gives the guilty packet's size.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
typedef void(BRLAPI_STDCALL* brlapi_exceptionHandler_t)(
    int error,
    brlapi_packetType_t type,
    const void* packet,
    size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
typedef void(BRLAPI_STDCALL* brlapi__exceptionHandler_t)(
    brlapi_handle_t* handle,
    int error,
    brlapi_packetType_t type,
    const void* packet,
    size_t size);

/* brlapi_strexception */
/** Describes an exception
 *
 * brlapi_strexception() puts a text describing the given exception in buffer.
 *
 * The beginning of the guilty packet is dumped as a sequence of hex bytes.
 *
 * \return the size of the text describing the exception, following
 * snprintf()'s semantics.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_strexception(char* buffer,
                                       size_t bufferSize,
                                       int error,
                                       brlapi_packetType_t type,
                                       const void* packet,
                                       size_t packetSize);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__strexception(brlapi_handle_t* handle,
                                        char* buffer,
                                        size_t bufferSize,
                                        int error,
                                        brlapi_packetType_t type,
                                        const void* packet,
                                        size_t packetSize);

/* brlapi_setExceptionHandler */
/** Set a new exception handler
 *
 * brlapi_setExceptionHandler() replaces the previous exception handler with
 * the handler parameter. The previous exception handler is returned to make
 * chaining error handlers possible.
 *
 * The default handler just prints the exception and abort()s.
 */
#ifndef BRLAPI_NO_SINGLE_SESSION
brlapi_exceptionHandler_t BRLAPI_STDCALL
brlapi_setExceptionHandler(brlapi_exceptionHandler_t handler);
#endif /* BRLAPI_NO_SINGLE_SESSION */
brlapi__exceptionHandler_t BRLAPI_STDCALL
brlapi__setExceptionHandler(brlapi_handle_t* handle,
                            brlapi__exceptionHandler_t handler);

#ifndef BRLAPI_NO_SINGLE_SESSION
void BRLAPI_STDCALL brlapi_defaultExceptionHandler(int error,
                                                   brlapi_packetType_t type,
                                                   const void* packet,
                                                   size_t size);
#endif /* BRLAPI_NO_SINGLE_SESSION */
void BRLAPI_STDCALL brlapi__defaultExceptionHandler(brlapi_handle_t* handle,
                                                    int error,
                                                    brlapi_packetType_t type,
                                                    const void* packet,
                                                    size_t size);

/** @} */

/* Windows-specific tricks - don't look at this */
#ifdef BRLAPI_WIN32
#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_writeTextWin(int cursor, const void* str, int wide);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__writeTextWin(brlapi_handle_t* handle,
                                        int cursor,
                                        const void* str,
                                        int wide);

#ifndef BRLAPI_NO_SINGLE_SESSION
int BRLAPI_STDCALL brlapi_writeWin(const brlapi_writeArguments_t* s, int wide);
#endif /* BRLAPI_NO_SINGLE_SESSION */
int BRLAPI_STDCALL brlapi__writeWin(brlapi_handle_t* handle,
                                    const brlapi_writeArguments_t* s,
                                    int wide);

#ifdef UNICODE
#ifndef BRLAPI_NO_SINGLE_SESSION
#define brlapi_writeText(cursor, str) brlapi_writeTextWin(cursor, str, 1)
#endif /* BRLAPI_NO_SINGLE_SESSION */
#define brlapi__writeText(handle, cursor, str) \
  brlapi__writeTextWin(handle, cursor, str, 1)

#ifndef BRLAPI_NO_SINGLE_SESSION
#define brlapi_write(s) brlapi_writeWin(s, 1)
#endif /* BRLAPI_NO_SINGLE_SESSION */
#define brlapi__write(handle, s) brlapi__writeWin(handle, s, 1)

#else /* UNICODE */

#ifndef BRLAPI_NO_SINGLE_SESSION
#define brlapi_writeText(cursor, str) brlapi_writeTextWin(cursor, str, 0)
#endif /* BRLAPI_NO_SINGLE_SESSION */
#define brlapi__writeText(handle, cursor, str) \
  brlapi__writeTextWin(handle, cursor, str, 0)

#ifndef BRLAPI_NO_SINGLE_SESSION
#define brlapi_write(s) brlapi_writeWin(s, 0)
#endif /* BRLAPI_NO_SINGLE_SESSION */
#define brlapi__write(handle, s) brlapi__writeWin(handle, s, 0)

#endif /* UNICODE */
#endif /* BRLAPI_WIN32 */

#ifndef BRLAPI_NO_DEPRECATED
/** \defgroup brlapi_deprecated Deprecated names
 *
 * With version 0.5.0, BrlAPI is now provided through including <brlapi.h> and
 * got a big renaming pass. Old names are still available through macros, but
 * they are deprecated since they will get dropped in the next release. This
 * documentation is for you to know the new names.
 *
 * For checking that you have completely switched to new names, just define
 * BRLAPI_NO_DEPRECATED: that will disable compatibility macros.
 *
 * @{ */

#define brlapi_settings_t brlapi_connectionSettings_t

/** brlapi_writeStruct, replaced by brlapi_writeArguments_t */
typedef struct {
  int displayNumber;
  unsigned int regionBegin;
  unsigned int regionSize;
  char* text;
  int textSize;
  unsigned char* attrAnd;
  unsigned char* attrOr;
  int cursor;
  char* charset;
} brlapi_writeStruct;
#define BRLAPI_WRITESTRUCT_INITIALIZER BRLAPI_WRITEARGUMENTS_INITIALIZER

#define brl_keycode_t brlapi_keyCode_t
#define brl_type_t brlapi_packetType_t

#define BRLCOMMANDS NULL
#define BRL_KEYCODE_MAX BRLAPI_KEY_MAX

#ifndef BRLAPI_NO_SINGLE_SESSION
#define brlapi_initializeConnection brlapi_openConnection
#define brlapi_getTty brlapi_enterTtyMode
#define brlapi_getTtyPath brlapi_enterTtyModeWithPath
#define brlapi_leaveTty brlapi_leaveTtyMode
#define brlapi_unignoreKeyRange brlapi_acceptKeyRange
#define brlapi_unignoreKeySet brlapi_acceptKeySet
#define brlapi_getRaw brlapi_enterRawMode
#define brlapi_leaveRaw brlapi_leaveRawMode
#define brlapi_suspend brlapi_suspendDriver
#define brlapi_resume brlapi_resumeDriver
#endif /* BRLAPI_NO_SINGLE_SESSION */

#define BRLERR_SUCCESS BRLAPI_ERROR_SUCCESS
#define BRLERR_NOMEM BRLAPI_ERROR_NOMEM
#define BRLERR_TTYBUSY BRLAPI_ERROR_TTYBUSY
#define BRLERR_DEVICEBUSY BRLAPI_ERROR_DEVICEBUSY
#define BRLERR_UNKNOWN_INSTRUCTION BRLAPI_ERROR_UNKNOWN_INSTRUCTION
#define BRLERR_ILLEGAL_INSTRUCTION BRLAPI_ERROR_ILLEGAL_INSTRUCTION
#define BRLERR_INVALID_PARAMETER BRLAPI_ERROR_INVALID_PARAMETER
#define BRLERR_INVALID_PACKET BRLAPI_ERROR_INVALID_PACKET
#define BRLERR_CONNREFUSED BRLAPI_ERROR_CONNREFUSED
#define BRLERR_OPNOTSUPP BRLAPI_ERROR_OPNOTSUPP
#define BRLERR_GAIERR BRLAPI_ERROR_GAIERR
#define BRLERR_LIBCERR BRLAPI_ERROR_LIBCERR
#define BRLERR_UNKNOWNTTY BRLAPI_ERROR_UNKNOWNTTY
#define BRLERR_PROTOCOL_VERSION BRLAPI_ERROR_PROTOCOL_VERSION
#define BRLERR_EOF BRLAPI_ERROR_EOF
#define BRLERR_EMPTYKEY BRLAPI_ERROR_EMPTYKEY
#define BRLERR_DRIVERERROR BRLAPI_ERROR_DRIVERERROR

/** @} */
#endif /* BRLAPI_NO_DEPRECATED */

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* BRLAPI_INCLUDED */
