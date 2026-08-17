/*
 * Hyper-Text Transport Protocol definitions for CUPS.
 *
 * Copyright © 2007-2018 by Apple Inc.
 * Copyright © 1997-2007 by Easy Software Products, all rights reserved.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_HTTP_H_
#  define _CUPS_HTTP_H_

/*
 * Include necessary headers...
 */

#  include "versioning.h"
#  include "array.h"
#  include <string.h>
#  include <time.h>
#  include <sys/types.h>
#  ifdef _WIN32
#    ifndef __CUPS_SSIZE_T_DEFINED
#      define __CUPS_SSIZE_T_DEFINED
/* Windows does not support the ssize_t type, so map it to off_t... */
typedef off_t ssize_t;			/* @private@ */
#    endif /* !__CUPS_SSIZE_T_DEFINED */
#    include <winsock2.h>
#    include <ws2tcpip.h>
#  else
#    include <unistd.h>
#    include <sys/time.h>
#    include <sys/socket.h>
#    include <netdb.h>
#    include <netinet/in.h>
#    include <arpa/inet.h>
#    include <netinet/in_systm.h>
#    include <netinet/ip.h>
#    if !defined(__APPLE__) || !defined(TCP_NODELAY)
#      include <netinet/tcp.h>
#    endif /* !__APPLE__ || !TCP_NODELAY */
#    if defined(AF_UNIX) && !defined(AF_LOCAL)
#      define AF_LOCAL AF_UNIX		/* Older UNIX's have old names... */
#    endif /* AF_UNIX && !AF_LOCAL */
#    ifdef AF_LOCAL
#      include <sys/un.h>
#    endif /* AF_LOCAL */
#    if defined(LOCAL_PEERCRED) && !defined(SO_PEERCRED)
#      define SO_PEERCRED LOCAL_PEERCRED
#    endif /* LOCAL_PEERCRED && !SO_PEERCRED */
#  endif /* _WIN32 */


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Oh, the wonderful world of IPv6 compatibility.  Apparently some
 * implementations expose the (more logical) 32-bit address parts
 * to everyone, while others only expose it to kernel code...  To
 * make supporting IPv6 even easier, each vendor chose different
 * core structure and union names, so the same defines or code
 * can't be used on all platforms.
 *
 * The following will likely need tweaking on new platforms that
 * support IPv6 - the "s6_addr32" define maps to the 32-bit integer
 * array in the in6_addr union, which is named differently on various
 * platforms.
 */

#if defined(AF_INET6) && !defined(s6_addr32)
#  if defined(__sun)
#    define s6_addr32	_S6_un._S6_u32
#  elif defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__) || defined(__APPLE__)|| defined(__DragonFly__)
#    define s6_addr32	__u6_addr.__u6_addr32
#  elif defined(_WIN32)
/*
 * Windows only defines byte and 16-bit word members of the union and
 * requires special casing of all raw address code...
 */
#    define s6_addr32	error_need_win32_specific_code
#  endif /* __sun */
#endif /* AF_INET6 && !s6_addr32 */


/*
 * Limits...
 */

#  define HTTP_MAX_URI		1024	/* Max length of URI string */
#  define HTTP_MAX_HOST		256	/* Max length of hostname string */
#  define HTTP_MAX_BUFFER	2048	/* Max length of data buffer */
#  define HTTP_MAX_VALUE	256	/* Max header field value length */


/*
 * Types and structures...
 */

typedef enum http_auth_e		/**** HTTP authentication types @exclude all@ ****/
{
  HTTP_AUTH_NONE,			/* No authentication in use */
  HTTP_AUTH_BASIC,			/* Basic authentication in use */
  HTTP_AUTH_MD5,			/* Digest authentication in use */
  HTTP_AUTH_MD5_SESS,			/* MD5-session authentication in use */
  HTTP_AUTH_MD5_INT,			/* Digest authentication in use for body */
  HTTP_AUTH_MD5_SESS_INT,		/* MD5-session authentication in use for body */
  HTTP_AUTH_NEGOTIATE			/* GSSAPI authentication in use @since CUPS 1.3/macOS 10.5@ */
} http_auth_t;

typedef enum http_encoding_e		/**** HTTP transfer encoding values ****/
{
  HTTP_ENCODING_LENGTH,			/* Data is sent with Content-Length */
  HTTP_ENCODING_CHUNKED,		/* Data is chunked */
  HTTP_ENCODING_FIELDS			/* Sending HTTP fields */

#  ifndef _CUPS_NO_DEPRECATED
#    define HTTP_ENCODE_LENGTH	HTTP_ENCODING_LENGTH
#    define HTTP_ENCODE_CHUNKED	HTTP_ENCODING_CHUNKED
#    define HTTP_ENCODE_FIELDS	HTTP_ENCODING_FIELDS
#  endif /* !_CUPS_NO_DEPRECATED */
} http_encoding_t;

typedef enum http_encryption_e		/**** HTTP encryption values ****/
{
  HTTP_ENCRYPTION_IF_REQUESTED,		/* Encrypt if requested (TLS upgrade) */
  HTTP_ENCRYPTION_NEVER,		/* Never encrypt */
  HTTP_ENCRYPTION_REQUIRED,		/* Encryption is required (TLS upgrade) */
  HTTP_ENCRYPTION_ALWAYS		/* Always encrypt (SSL) */

#  ifndef _CUPS_NO_DEPRECATED
#    define HTTP_ENCRYPT_IF_REQUESTED	HTTP_ENCRYPTION_IF_REQUESTED
#    define HTTP_ENCRYPT_NEVER		HTTP_ENCRYPTION_NEVER
#    define HTTP_ENCRYPT_REQUIRED	HTTP_ENCRYPTION_REQUIRED
#    define HTTP_ENCRYPT_ALWAYS		HTTP_ENCRYPTION_ALWAYS
#  endif /* !_CUPS_NO_DEPRECATED */
} http_encryption_t;

typedef enum http_field_e		/**** HTTP field names ****/
{
  HTTP_FIELD_UNKNOWN = -1,		/* Unknown field */
  HTTP_FIELD_ACCEPT_LANGUAGE,		/* Accept-Language field */
  HTTP_FIELD_ACCEPT_RANGES,		/* Accept-Ranges field */
  HTTP_FIELD_AUTHORIZATION,		/* Authorization field */
  HTTP_FIELD_CONNECTION,		/* Connection field */
  HTTP_FIELD_CONTENT_ENCODING,		/* Content-Encoding field */
  HTTP_FIELD_CONTENT_LANGUAGE,		/* Content-Language field */
  HTTP_FIELD_CONTENT_LENGTH,		/* Content-Length field */
  HTTP_FIELD_CONTENT_LOCATION,		/* Content-Location field */
  HTTP_FIELD_CONTENT_MD5,		/* Content-MD5 field */
  HTTP_FIELD_CONTENT_RANGE,		/* Content-Range field */
  HTTP_FIELD_CONTENT_TYPE,		/* Content-Type field */
  HTTP_FIELD_CONTENT_VERSION,		/* Content-Version field */
  HTTP_FIELD_DATE,			/* Date field */
  HTTP_FIELD_HOST,			/* Host field */
  HTTP_FIELD_IF_MODIFIED_SINCE,		/* If-Modified-Since field */
  HTTP_FIELD_IF_UNMODIFIED_SINCE,	/* If-Unmodified-Since field */
  HTTP_FIELD_KEEP_ALIVE,		/* Keep-Alive field */
  HTTP_FIELD_LAST_MODIFIED,		/* Last-Modified field */
  HTTP_FIELD_LINK,			/* Link field */
  HTTP_FIELD_LOCATION,			/* Location field */
  HTTP_FIELD_RANGE,			/* Range field */
  HTTP_FIELD_REFERER,			/* Referer field */
  HTTP_FIELD_RETRY_AFTER,		/* Retry-After field */
  HTTP_FIELD_TRANSFER_ENCODING,		/* Transfer-Encoding field */
  HTTP_FIELD_UPGRADE,			/* Upgrade field */
  HTTP_FIELD_USER_AGENT,		/* User-Agent field */
  HTTP_FIELD_WWW_AUTHENTICATE,		/* WWW-Authenticate field */
  HTTP_FIELD_ACCEPT_ENCODING,		/* Accepting-Encoding field @since CUPS 1.7/macOS 10.9@ */
  HTTP_FIELD_ALLOW,			/* Allow field @since CUPS 1.7/macOS 10.9@ */
  HTTP_FIELD_SERVER,			/* Server field @since CUPS 1.7/macOS 10.9@ */
  HTTP_FIELD_AUTHENTICATION_INFO,	/* Authentication-Info field (@since CUPS 2.2.9) */
  HTTP_FIELD_MAX			/* Maximum field index */
} http_field_t;

typedef enum http_keepalive_e		/**** HTTP keep-alive values ****/
{
  HTTP_KEEPALIVE_OFF = 0,		/* No keep alive support */
  HTTP_KEEPALIVE_ON			/* Use keep alive */
} http_keepalive_t;

typedef enum http_state_e		/**** HTTP state values; states
					 **** are server-oriented...
					 ****/
{
  HTTP_STATE_ERROR = -1,		/* Error on socket */
  HTTP_STATE_WAITING,			/* Waiting for command */
  HTTP_STATE_OPTIONS,			/* OPTIONS command, waiting for blank line */
  HTTP_STATE_GET,			/* GET command, waiting for blank line */
  HTTP_STATE_GET_SEND,			/* GET command, sending data */
  HTTP_STATE_HEAD,			/* HEAD command, waiting for blank line */
  HTTP_STATE_POST,			/* POST command, waiting for blank line */
  HTTP_STATE_POST_RECV,			/* POST command, receiving data */
  HTTP_STATE_POST_SEND,			/* POST command, sending data */
  HTTP_STATE_PUT,			/* PUT command, waiting for blank line */
  HTTP_STATE_PUT_RECV,			/* PUT command, receiving data */
  HTTP_STATE_DELETE,			/* DELETE command, waiting for blank line */
  HTTP_STATE_TRACE,			/* TRACE command, waiting for blank line */
  HTTP_STATE_CONNECT,			/* CONNECT command, waiting for blank line */
  HTTP_STATE_STATUS,			/* Command complete, sending status */
  HTTP_STATE_UNKNOWN_METHOD,		/* Unknown request method, waiting for blank line @since CUPS 1.7/macOS 10.9@ */
  HTTP_STATE_UNKNOWN_VERSION		/* Unknown request method, waiting for blank line @since CUPS 1.7/macOS 10.9@ */

#  ifndef _CUPS_NO_DEPRECATED
#    define HTTP_WAITING	HTTP_STATE_WAITING
#    define HTTP_OPTIONS	HTTP_STATE_OPTIONS
#    define HTTP_GET		HTTP_STATE_GET
#    define HTTP_GET_SEND	HTTP_STATE_GET_SEND
#    define HTTP_HEAD		HTTP_STATE_HEAD
#    define HTTP_POST		HTTP_STATE_POST
#    define HTTP_POST_RECV	HTTP_STATE_POST_RECV
#    define HTTP_POST_SEND	HTTP_STATE_POST_SEND
#    define HTTP_PUT		HTTP_STATE_PUT
#    define HTTP_PUT_RECV	HTTP_STATE_PUT_RECV
#    define HTTP_DELETE		HTTP_STATE_DELETE
#    define HTTP_TRACE		HTTP_STATE_TRACE
#    define HTTP_CLOSE		HTTP_STATE_CONNECT
#    define HTTP_STATUS		HTTP_STATE_STATUS
#  endif /* !_CUPS_NO_DEPRECATED */
} http_state_t;

typedef enum http_status_e		/**** HTTP status codes ****/
{
  HTTP_STATUS_ERROR = -1,		/* An error response from httpXxxx() */
  HTTP_STATUS_NONE = 0,			/* No Expect value @since CUPS 1.7/macOS 10.9@ */

  HTTP_STATUS_CONTINUE = 100,		/* Everything OK, keep going... */
  HTTP_STATUS_SWITCHING_PROTOCOLS,	/* HTTP upgrade to TLS/SSL */

  HTTP_STATUS_OK = 200,			/* OPTIONS/GET/HEAD/POST/TRACE command was successful */
  HTTP_STATUS_CREATED,			/* PUT command was successful */
  HTTP_STATUS_ACCEPTED,			/* DELETE command was successful */
  HTTP_STATUS_NOT_AUTHORITATIVE,	/* Information isn't authoritative */
  HTTP_STATUS_NO_CONTENT,		/* Successful command, no new data */
  HTTP_STATUS_RESET_CONTENT,		/* Content was reset/recreated */
  HTTP_STATUS_PARTIAL_CONTENT,		/* Only a partial file was received/sent */

  HTTP_STATUS_MULTIPLE_CHOICES = 300,	/* Multiple files match request */
  HTTP_STATUS_MOVED_PERMANENTLY,	/* Document has moved permanently */
  HTTP_STATUS_FOUND,			/* Document was found at a different URI */
  HTTP_STATUS_SEE_OTHER,		/* See this other link */
  HTTP_STATUS_NOT_MODIFIED,		/* File not modified */
  HTTP_STATUS_USE_PROXY,		/* Must use a proxy to access this URI */
  HTTP_STATUS_TEMPORARY_REDIRECT = 307,	/* Temporary redirection */

  HTTP_STATUS_BAD_REQUEST = 400,	/* Bad request */
  HTTP_STATUS_UNAUTHORIZED,		/* Unauthorized to access host */
  HTTP_STATUS_PAYMENT_REQUIRED,		/* Payment required */
  HTTP_STATUS_FORBIDDEN,		/* Forbidden to access this URI */
  HTTP_STATUS_NOT_FOUND,		/* URI was not found */
  HTTP_STATUS_METHOD_NOT_ALLOWED,	/* Method is not allowed */
  HTTP_STATUS_NOT_ACCEPTABLE,		/* Not Acceptable */
  HTTP_STATUS_PROXY_AUTHENTICATION,	/* Proxy Authentication is Required */
  HTTP_STATUS_REQUEST_TIMEOUT,		/* Request timed out */
  HTTP_STATUS_CONFLICT,			/* Request is self-conflicting */
  HTTP_STATUS_GONE,			/* Server has gone away */
  HTTP_STATUS_LENGTH_REQUIRED,		/* A content length or encoding is required */
  HTTP_STATUS_PRECONDITION,		/* Precondition failed */
  HTTP_STATUS_REQUEST_TOO_LARGE,	/* Request entity too large */
  HTTP_STATUS_URI_TOO_LONG,		/* URI too long */
  HTTP_STATUS_UNSUPPORTED_MEDIATYPE,	/* The requested media type is unsupported */
  HTTP_STATUS_REQUESTED_RANGE,		/* The requested range is not satisfiable */
  HTTP_STATUS_EXPECTATION_FAILED,	/* The expectation given in an Expect header field was not met */
  HTTP_STATUS_UPGRADE_REQUIRED = 426,	/* Upgrade to SSL/TLS required */

  HTTP_STATUS_SERVER_ERROR = 500,	/* Internal server error */
  HTTP_STATUS_NOT_IMPLEMENTED,		/* Feature not implemented */
  HTTP_STATUS_BAD_GATEWAY,		/* Bad gateway */
  HTTP_STATUS_SERVICE_UNAVAILABLE,	/* Service is unavailable */
  HTTP_STATUS_GATEWAY_TIMEOUT,		/* Gateway connection timed out */
  HTTP_STATUS_NOT_SUPPORTED,		/* HTTP version not supported */

  HTTP_STATUS_CUPS_AUTHORIZATION_CANCELED = 1000,
					/* User canceled authorization @since CUPS 1.4@ */
  HTTP_STATUS_CUPS_PKI_ERROR,		/* Error negotiating a secure connection @since CUPS 1.5/macOS 10.7@ */
  HTTP_STATUS_CUPS_WEBIF_DISABLED	/* Web interface is disabled @private@ */

#  define HTTP_STATUS_MOVED_TEMPORARILY HTTP_STATUS_FOUND /* Renamed in RFC 7231 */

#  ifndef _CUPS_NO_DEPRECATED
/* Old names for this enumeration */
#    define HTTP_ERROR			HTTP_STATUS_ERROR

#    define HTTP_CONTINUE		HTTP_STATUS_CONTINUE
#    define HTTP_SWITCHING_PROTOCOLS	HTTP_STATUS_SWITCHING_PROTOCOLS

#    define HTTP_OK			HTTP_STATUS_OK
#    define HTTP_CREATED		HTTP_STATUS_CREATED
#    define HTTP_ACCEPTED		HTTP_STATUS_ACCEPTED
#    define HTTP_NOT_AUTHORITATIVE	HTTP_STATUS_NOT_AUTHORITATIVE
#    define HTTP_NO_CONTENT		HTTP_STATUS_NO_CONTENT
#    define HTTP_RESET_CONTENT		HTTP_STATUS_RESET_CONTENT
#    define HTTP_PARTIAL_CONTENT	HTTP_STATUS_PARTIAL_CONTENT

#    define HTTP_MULTIPLE_CHOICES	HTTP_STATUS_MULTIPLE_CHOICES
#    define HTTP_MOVED_PERMANENTLY	HTTP_STATUS_MOVED_PERMANENTLY
#    define HTTP_MOVED_TEMPORARILY	HTTP_STATUS_MOVED_TEMPORARILY
#    define HTTP_SEE_OTHER		HTTP_STATUS_SEE_OTHER
#    define HTTP_NOT_MODIFIED		HTTP_STATUS_NOT_MODIFIED
#    define HTTP_USE_PROXY		HTTP_STATUS_USE_PROXY

#    define HTTP_BAD_REQUEST		HTTP_STATUS_BAD_REQUEST
#    define HTTP_UNAUTHORIZED		HTTP_STATUS_UNAUTHORIZED
#    define HTTP_PAYMENT_REQUIRED	HTTP_STATUS_PAYMENT_REQUIRED
#    define HTTP_FORBIDDEN		HTTP_STATUS_FORBIDDEN
#    define HTTP_NOT_FOUND		HTTP_STATUS_NOT_FOUND
#    define HTTP_METHOD_NOT_ALLOWED	HTTP_STATUS_METHOD_NOT_ALLOWED
#    define HTTP_NOT_ACCEPTABLE		HTTP_STATUS_NOT_ACCEPTABLE
#    define HTTP_PROXY_AUTHENTICATION	HTTP_STATUS_PROXY_AUTHENTICATION
#    define HTTP_REQUEST_TIMEOUT	HTTP_STATUS_REQUEST_TIMEOUT
#    define HTTP_CONFLICT		HTTP_STATUS_CONFLICT
#    define HTTP_GONE			HTTP_STATUS_GONE
#    define HTTP_LENGTH_REQUIRED	HTTP_STATUS_LENGTH_REQUIRED
#    define HTTP_PRECONDITION		HTTP_STATUS_PRECONDITION
#    define HTTP_REQUEST_TOO_LARGE	HTTP_STATUS_REQUEST_TOO_LARGE
#    define HTTP_URI_TOO_LONG		HTTP_STATUS_URI_TOO_LONG
#    define HTTP_UNSUPPORTED_MEDIATYPE	HTTP_STATUS_UNSUPPORTED_MEDIATYPE
#    define HTTP_REQUESTED_RANGE	HTTP_STATUS_REQUESTED_RANGE
#    define HTTP_EXPECTATION_FAILED	HTTP_STATUS_EXPECTATION_FAILED
#    define HTTP_UPGRADE_REQUIRED	HTTP_STATUS_UPGRADE_REQUIRED

#    define HTTP_SERVER_ERROR		HTTP_STATUS_SERVER_ERROR
#    define HTTP_NOT_IMPLEMENTED	HTTP_STATUS_NOT_IMPLEMENTED
#    define HTTP_BAD_GATEWAY		HTTP_STATUS_BAD_GATEWAY
#    define HTTP_SERVICE_UNAVAILABLE	HTTP_STATUS_SERVICE_UNAVAILABLE
#    define HTTP_GATEWAY_TIMEOUT	HTTP_STATUS_GATEWAY_TIMEOUT
#    define HTTP_NOT_SUPPORTED		HTTP_STATUS_NOT_SUPPORTED

#    define HTTP_AUTHORIZATION_CANCELED	HTTP_STATUS_CUPS_AUTHORIZATION_CANCELED
#    define HTTP_PKI_ERROR		HTTP_STATUS_CUPS_PKI_ERROR
#    define HTTP_WEBIF_DISABLED		HTTP_STATUS_CUPS_WEBIF_DISABLED
#  endif /* !_CUPS_NO_DEPRECATED */
} http_status_t;

typedef enum http_trust_e		/**** Level of trust for credentials @since CUPS 2.0/OS 10.10@ */
{
  HTTP_TRUST_OK = 0,			/* Credentials are OK/trusted */
  HTTP_TRUST_INVALID,			/* Credentials are invalid */
  HTTP_TRUST_CHANGED,			/* Credentials have changed */
  HTTP_TRUST_EXPIRED,			/* Credentials are expired */
  HTTP_TRUST_RENEWED,			/* Credentials have been renewed */
  HTTP_TRUST_UNKNOWN,			/* Credentials are unknown/new */
} http_trust_t;

typedef enum http_uri_status_e		/**** URI separation status @since CUPS 1.2@ ****/
{
  HTTP_URI_STATUS_OVERFLOW = -8,	/* URI buffer for httpAssembleURI is too small */
  HTTP_URI_STATUS_BAD_ARGUMENTS = -7,	/* Bad arguments to function (error) */
  HTTP_URI_STATUS_BAD_RESOURCE = -6,	/* Bad resource in URI (error) */
  HTTP_URI_STATUS_BAD_PORT = -5,	/* Bad port number in URI (error) */
  HTTP_URI_STATUS_BAD_HOSTNAME = -4,	/* Bad hostname in URI (error) */
  HTTP_URI_STATUS_BAD_USERNAME = -3,	/* Bad username in URI (error) */
  HTTP_URI_STATUS_BAD_SCHEME = -2,	/* Bad scheme in URI (error) */
  HTTP_URI_STATUS_BAD_URI = -1,		/* Bad/empty URI (error) */
  HTTP_URI_STATUS_OK = 0,		/* URI decoded OK */
  HTTP_URI_STATUS_MISSING_SCHEME,	/* Missing scheme in URI (warning) */
  HTTP_URI_STATUS_UNKNOWN_SCHEME,	/* Unknown scheme in URI (warning) */
  HTTP_URI_STATUS_MISSING_RESOURCE	/* Missing resource in URI (warning) */

#  ifndef _CUPS_NO_DEPRECATED
#    define HTTP_URI_OVERFLOW		HTTP_URI_STATUS_OVERFLOW
#    define HTTP_URI_BAD_ARGUMENTS	HTTP_URI_STATUS_BAD_ARGUMENTS
#    define HTTP_URI_BAD_RESOURCE	HTTP_URI_STATUS_BAD_RESOURCE
#    define HTTP_URI_BAD_PORT		HTTP_URI_STATUS_BAD_PORT
#    define HTTP_URI_BAD_HOSTNAME	HTTP_URI_STATUS_BAD_HOSTNAME
#    define HTTP_URI_BAD_USERNAME	HTTP_URI_STATUS_BAD_USERNAME
#    define HTTP_URI_BAD_SCHEME		HTTP_URI_STATUS_BAD_SCHEME
#    define HTTP_URI_BAD_URI		HTTP_URI_STATUS_BAD_URI
#    define HTTP_URI_OK			HTTP_URI_STATUS_OK
#    define HTTP_URI_MISSING_SCHEME	HTTP_URI_STATUS_MISSING_SCHEME
#    define HTTP_URI_UNKNOWN_SCHEME	HTTP_URI_STATUS_UNKNOWN_SCHEME
#    define HTTP_URI_MISSING_RESOURCE	HTTP_URI_STATUS_MISSING_RESOURCE
#  endif /* !_CUPS_NO_DEPRECATED */
} http_uri_status_t;

typedef enum http_uri_coding_e		/**** URI en/decode flags ****/
{
  HTTP_URI_CODING_NONE = 0,		/* Don't en/decode anything */
  HTTP_URI_CODING_USERNAME = 1,		/* En/decode the username portion */
  HTTP_URI_CODING_HOSTNAME = 2,		/* En/decode the hostname portion */
  HTTP_URI_CODING_RESOURCE = 4,		/* En/decode the resource portion */
  HTTP_URI_CODING_MOST = 7,		/* En/decode all but the query */
  HTTP_URI_CODING_QUERY = 8,		/* En/decode the query portion */
  HTTP_URI_CODING_ALL = 15,		/* En/decode everything */
  HTTP_URI_CODING_RFC6874 = 16		/* Use RFC 6874 address format */
} http_uri_coding_t;

typedef enum http_version_e		/**** HTTP version numbers @exclude all@ ****/
{
  HTTP_VERSION_0_9 = 9,			/* HTTP/0.9 */
  HTTP_VERSION_1_0 = 100,		/* HTTP/1.0 */
  HTTP_VERSION_1_1 = 101		/* HTTP/1.1 */

#  ifndef _CUPS_NO_DEPRECATED
#    define HTTP_0_9	HTTP_VERSION_0_9
#    define HTTP_1_0	HTTP_VERSION_1_0
#    define HTTP_1_1	HTTP_VERSION_1_1
#  endif /* !_CUPS_NO_DEPRECATED */
} http_version_t;

typedef union _http_addr_u		/**** Socket address union, which
					 **** makes using IPv6 and other
					 **** address types easier and
					 **** more portable. @since CUPS 1.2/macOS 10.5@
					 ****/
{
  struct sockaddr	addr;		/* Base structure for family value */
  struct sockaddr_in	ipv4;		/* IPv4 address */
#ifdef AF_INET6
  struct sockaddr_in6	ipv6;		/* IPv6 address */
#endif /* AF_INET6 */
#ifdef AF_LOCAL
  struct sockaddr_un	un;		/* Domain socket file */
#endif /* AF_LOCAL */
  char			pad[256];	/* Padding to ensure binary compatibility */
} http_addr_t;

typedef struct http_addrlist_s		/**** Socket address list, which is
					 **** used to enumerate all of the
					 **** addresses that are associated
					 **** with a hostname. @since CUPS 1.2/macOS 10.5@
                                         **** @exclude all@
					 ****/
{
  struct http_addrlist_s *next;		/* Pointer to next address in list */
  http_addr_t		addr;		/* Address */
} http_addrlist_t;

typedef struct _http_s http_t;		/**** HTTP connection type ****/

typedef struct http_credential_s	/**** HTTP credential data @since CUPS 1.5/macOS 10.7@ @exclude all@ ****/
{
  void		*data;			/* Pointer to credential data */
  size_t	datalen;		/* Credential length */
} http_credential_t;

typedef int (*http_timeout_cb_t)(http_t *http, void *user_data);
					/**** HTTP timeout callback @since CUPS 1.5/macOS 10.7@ ****/



/*
 * Prototypes...
 */

extern void		httpBlocking(http_t *http, int b) _CUPS_PUBLIC;
extern int		httpCheck(http_t *http) _CUPS_PUBLIC;
extern void		httpClearFields(http_t *http) _CUPS_PUBLIC;
extern void		httpClose(http_t *http) _CUPS_PUBLIC;
extern http_t		*httpConnect(const char *host, int port) _CUPS_DEPRECATED_1_7_MSG("Use httpConnect2 instead.");
extern http_t		*httpConnectEncrypt(const char *host, int port, http_encryption_t encryption) _CUPS_DEPRECATED_1_7_MSG("Use httpConnect2 instead.");
extern int		httpDelete(http_t *http, const char *uri) _CUPS_PUBLIC;
extern int		httpEncryption(http_t *http, http_encryption_t e) _CUPS_PUBLIC;
extern int		httpError(http_t *http) _CUPS_PUBLIC;
extern void		httpFlush(http_t *http) _CUPS_PUBLIC;
extern int		httpGet(http_t *http, const char *uri) _CUPS_PUBLIC;
extern char		*httpGets(char *line, int length, http_t *http) _CUPS_PUBLIC;
extern const char	*httpGetDateString(time_t t) _CUPS_PUBLIC;
extern time_t		httpGetDateTime(const char *s) _CUPS_PUBLIC;
extern const char	*httpGetField(http_t *http, http_field_t field) _CUPS_PUBLIC;
extern struct hostent	*httpGetHostByName(const char *name) _CUPS_PUBLIC;
extern char		*httpGetSubField(http_t *http, http_field_t field, const char *name, char *value) _CUPS_PUBLIC;
extern int		httpHead(http_t *http, const char *uri) _CUPS_PUBLIC;
extern void		httpInitialize(void) _CUPS_PUBLIC;
extern int		httpOptions(http_t *http, const char *uri) _CUPS_PUBLIC;
extern int		httpPost(http_t *http, const char *uri) _CUPS_PUBLIC;
extern int		httpPrintf(http_t *http, const char *format, ...) _CUPS_FORMAT(2, 3) _CUPS_PUBLIC;
extern int		httpPut(http_t *http, const char *uri) _CUPS_PUBLIC;
extern int		httpRead(http_t *http, char *buffer, int length) _CUPS_DEPRECATED_MSG("Use httpRead2 instead.");
extern int		httpReconnect(http_t *http) _CUPS_DEPRECATED_1_6_MSG("Use httpReconnect2 instead.");
extern void		httpSeparate(const char *uri, char *method, char *username, char *host, int *port, char *resource) _CUPS_DEPRECATED_1_2_MSG("Use httpSeparateURI instead.");
extern void		httpSetField(http_t *http, http_field_t field, const char *value) _CUPS_PUBLIC;
extern const char	*httpStatus(http_status_t status) _CUPS_PUBLIC;
extern int		httpTrace(http_t *http, const char *uri) _CUPS_PUBLIC;
extern http_status_t	httpUpdate(http_t *http) _CUPS_PUBLIC;
extern int		httpWrite(http_t *http, const char *buffer, int length) _CUPS_DEPRECATED_MSG("Use httpWrite2 instead.");
extern char		*httpEncode64(char *out, const char *in) _CUPS_DEPRECATED_MSG("Use httpEncode64_2 instead.");
extern char		*httpDecode64(char *out, const char *in) _CUPS_DEPRECATED_MSG("Use httpDecode64_2 instead.");
extern int		httpGetLength(http_t *http) _CUPS_DEPRECATED_1_2_MSG("Use httpGetLength2 instead.");
extern char		*httpMD5(const char *, const char *, const char *, char [33]) _CUPS_DEPRECATED_MSG("Use cupsDoAuth or cupsHashData instead.");
extern char		*httpMD5Final(const char *, const char *, const char *, char [33]) _CUPS_DEPRECATED_2_2_MSG("Use cupsDoAuth or cupsHashData instead.");
extern char		*httpMD5String(const unsigned char *, char [33]) _CUPS_DEPRECATED_2_2_MSG("Use cupsHashString instead.");

/**** New in CUPS 1.1.19 ****/
extern void		httpClearCookie(http_t *http) _CUPS_API_1_1_19;
extern const char	*httpGetCookie(http_t *http) _CUPS_API_1_1_19;
extern void		httpSetCookie(http_t *http, const char *cookie) _CUPS_API_1_1_19;
extern int		httpWait(http_t *http, int msec) _CUPS_API_1_1_19;

/**** New in CUPS 1.1.21 ****/
extern char		*httpDecode64_2(char *out, int *outlen, const char *in) _CUPS_API_1_1_21;
extern char		*httpEncode64_2(char *out, int outlen, const char *in, int inlen) _CUPS_API_1_1_21;
extern void		httpSeparate2(const char *uri, char *method, int methodlen, char *username, int usernamelen, char *host, int hostlen, int *port, char *resource, int resourcelen) _CUPS_DEPRECATED_1_2_MSG("Use httpSeparateURI instead.");

/**** New in CUPS 1.2/macOS 10.5 ****/
extern int		httpAddrAny(const http_addr_t *addr) _CUPS_API_1_2;
extern http_addrlist_t	*httpAddrConnect(http_addrlist_t *addrlist, int *sock) _CUPS_API_1_2;
extern int		httpAddrEqual(const http_addr_t *addr1, const http_addr_t *addr2) _CUPS_API_1_2;
extern void		httpAddrFreeList(http_addrlist_t *addrlist) _CUPS_API_1_2;
extern http_addrlist_t	*httpAddrGetList(const char *hostname, int family, const char *service) _CUPS_API_1_2;
extern int		httpAddrLength(const http_addr_t *addr) _CUPS_API_1_2;
extern int		httpAddrLocalhost(const http_addr_t *addr) _CUPS_API_1_2;
extern char		*httpAddrLookup(const http_addr_t *addr, char *name, int namelen) _CUPS_API_1_2;
extern char		*httpAddrString(const http_addr_t *addr, char *s, int slen) _CUPS_API_1_2;
extern http_uri_status_t httpAssembleURI(http_uri_coding_t encoding, char *uri, int urilen, const char *scheme, const char *username, const char *host, int port, const char *resource) _CUPS_API_1_2;
extern http_uri_status_t httpAssembleURIf(http_uri_coding_t encoding, char *uri, int urilen, const char *scheme, const char *username, const char *host, int port, const char *resourcef, ...) _CUPS_FORMAT(8, 9) _CUPS_API_1_2;
extern int		httpFlushWrite(http_t *http) _CUPS_API_1_2;
extern int		httpGetBlocking(http_t *http) _CUPS_API_1_2;
extern const char	*httpGetDateString2(time_t t, char *s, int slen) _CUPS_API_1_2;
extern int		httpGetFd(http_t *http) _CUPS_API_1_2;
extern const char	*httpGetHostname(http_t *http, char *s, int slen) _CUPS_API_1_2;
extern off_t		httpGetLength2(http_t *http) _CUPS_API_1_2;
extern http_status_t	httpGetStatus(http_t *http) _CUPS_API_1_2;
extern char		*httpGetSubField2(http_t *http, http_field_t field, const char *name, char *value, int valuelen) _CUPS_API_1_2;
extern ssize_t		httpRead2(http_t *http, char *buffer, size_t length) _CUPS_API_1_2;
extern http_uri_status_t httpSeparateURI(http_uri_coding_t decoding, const char *uri, char *scheme, int schemelen, char *username, int usernamelen, char *host, int hostlen, int *port, char *resource, int resourcelen) _CUPS_API_1_2;
extern void		httpSetExpect(http_t *http, http_status_t expect) _CUPS_API_1_2;
extern void		httpSetLength(http_t *http, size_t length) _CUPS_API_1_2;
extern ssize_t		httpWrite2(http_t *http, const char *buffer, size_t length) _CUPS_API_1_2;

/**** New in CUPS 1.3/macOS 10.5 ****/
extern char		*httpGetAuthString(http_t *http) _CUPS_API_1_3;
extern void		httpSetAuthString(http_t *http, const char *scheme, const char *data) _CUPS_API_1_3;

/**** New in CUPS 1.5/macOS 10.7 ****/
extern int		httpAddCredential(cups_array_t *credentials, const void *data, size_t datalen) _CUPS_API_1_5;
extern int		httpCopyCredentials(http_t *http, cups_array_t **credentials) _CUPS_API_1_5;
extern void		httpFreeCredentials(cups_array_t *certs) _CUPS_API_1_5;
extern int		httpSetCredentials(http_t *http, cups_array_t *certs) _CUPS_API_1_5;
extern void		httpSetTimeout(http_t *http, double timeout, http_timeout_cb_t cb, void *user_data) _CUPS_API_1_5;

/**** New in CUPS 1.6/macOS 10.8 ****/
extern http_addrlist_t	*httpAddrConnect2(http_addrlist_t *addrlist, int *sock, int msec, int *cancel) _CUPS_API_1_6;
extern http_state_t	httpGetState(http_t *http) _CUPS_API_1_6;
extern http_version_t	httpGetVersion(http_t *http) _CUPS_API_1_6;
extern int		httpReconnect2(http_t *http, int msec, int *cancel) _CUPS_API_1_6;


/**** New in CUPS 1.7/macOS 10.9 ****/
extern http_t		*httpAcceptConnection(int fd, int blocking) _CUPS_API_1_7;
extern http_addrlist_t	*httpAddrCopyList(http_addrlist_t *src) _CUPS_API_1_7;
extern int		httpAddrListen(http_addr_t *addr, int port) _CUPS_API_1_7;
extern int		httpAddrPort(http_addr_t *addr) _CUPS_API_1_7;
extern char		*httpAssembleUUID(const char *server, int port, const char *name, int number, char *buffer, size_t bufsize) _CUPS_API_1_7;
extern http_t		*httpConnect2(const char *host, int port, http_addrlist_t *addrlist, int family, http_encryption_t encryption, int blocking, int msec, int *cancel) _CUPS_API_1_7;
extern const char	*httpGetContentEncoding(http_t *http) _CUPS_API_1_7;
extern http_status_t	httpGetExpect(http_t *http) _CUPS_API_1_7;
extern ssize_t		httpPeek(http_t *http, char *buffer, size_t length) _CUPS_API_1_7;
extern http_state_t	httpReadRequest(http_t *http, char *resource, size_t resourcelen) _CUPS_API_1_7;
extern void		httpSetDefaultField(http_t *http, http_field_t field, const char *value) _CUPS_API_1_7;
extern http_state_t	httpWriteResponse(http_t *http, http_status_t status) _CUPS_API_1_7;

/* New in CUPS 2.0/macOS 10.10 */
extern int		httpAddrClose(http_addr_t *addr, int fd) _CUPS_API_2_0;
extern int		httpAddrFamily(http_addr_t *addr) _CUPS_API_2_0;
extern int		httpCompareCredentials(cups_array_t *cred1, cups_array_t *cred2) _CUPS_API_2_0;
extern int		httpCredentialsAreValidForName(cups_array_t *credentials, const char *common_name);
extern time_t		httpCredentialsGetExpiration(cups_array_t *credentials) _CUPS_API_2_0;
extern http_trust_t	httpCredentialsGetTrust(cups_array_t *credentials, const char *common_name) _CUPS_API_2_0;
extern size_t		httpCredentialsString(cups_array_t *credentials, char *buffer, size_t bufsize) _CUPS_API_2_0;
extern http_field_t	httpFieldValue(const char *name) _CUPS_API_2_0;
extern time_t		httpGetActivity(http_t *http) _CUPS_API_2_0;
extern http_addr_t	*httpGetAddress(http_t *http) _CUPS_API_2_0;
extern http_encryption_t httpGetEncryption(http_t *http) _CUPS_API_2_0;
extern http_keepalive_t	httpGetKeepAlive(http_t *http) _CUPS_API_2_0;
extern size_t		httpGetPending(http_t *http) _CUPS_API_2_0;
extern size_t		httpGetReady(http_t *http) _CUPS_API_2_0;
extern size_t		httpGetRemaining(http_t *http) _CUPS_API_2_0;
extern int		httpIsChunked(http_t *http) _CUPS_API_2_0;
extern int		httpIsEncrypted(http_t *http) _CUPS_API_2_0;
extern int		httpLoadCredentials(const char *path, cups_array_t **credentials, const char *common_name) _CUPS_API_2_0;
extern const char	*httpResolveHostname(http_t *http, char *buffer, size_t bufsize) _CUPS_API_2_0;
extern int		httpSaveCredentials(const char *path, cups_array_t *credentials, const char *common_name) _CUPS_API_2_0;
extern void		httpSetKeepAlive(http_t *http, http_keepalive_t keep_alive) _CUPS_API_2_0;
extern void		httpShutdown(http_t *http) _CUPS_API_2_0;
extern const char	*httpStateString(http_state_t state) _CUPS_API_2_0;
extern const char	*httpURIStatusString(http_uri_status_t status) _CUPS_API_2_0;

/*
 * C++ magic...
 */

#  ifdef __cplusplus
}
#  endif /* __cplusplus */
#endif /* !_CUPS_HTTP_H_ */
