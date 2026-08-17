//
//
// Copyright 2020 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CRONET_TRANSPORT_CRONET_STATUS_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CRONET_TRANSPORT_CRONET_STATUS_H

#include <grpc/status.h>
#include <grpc/support/port_platform.h>

enum cronet_net_error_code {
  //
  // Ranges:
  //     0- 99 System related errors
  //   100-199 Connection related errors
  //   200-299 Certificate errors
  //   300-399 HTTP errors
  //   400-499 Cache errors
  //   500-599 ?
  //   600-699 FTP errors
  //   700-799 Certificate manager errors
  //   800-899 DNS resolver errors

  // An asynchronous IO operation is not yet complete.  This usually does not
  // indicate a fatal error.  Typically this error will be generated as a
  // notification to wait for some external notification that the IO operation
  // finally completed.
  OK = 0,
  CRONET_NET_ERROR_IO_PENDING = -1,

  // A generic failure occurred.
  CRONET_NET_ERROR_FAILED = -2,

  // An operation was aborted (due to user action,.
  CRONET_NET_ERROR_ABORTED = -3,

  // An argument to the function is incorrect.
  CRONET_NET_ERROR_INVALID_ARGUMENT = -4,

  // The handle or file descriptor is invalid.
  CRONET_NET_ERROR_INVALID_HANDLE = -5,

  // The file or directory cannot be found.
  CRONET_NET_ERROR_FILE_NOT_FOUND = -6,

  // An operation timed out.
  CRONET_NET_ERROR_TIMED_OUT = -7,

  // The file is too large.
  CRONET_NET_ERROR_FILE_TOO_BIG = -8,

  // An unexpected error.  This may be caused by a programming mistake or an
  // invalid assumption.
  CRONET_NET_ERROR_UNEXPECTED = -9,

  // Permission to access a resource = other than the network = was denied.
  CRONET_NET_ERROR_ACCESS_DENIED = -10,

  // The operation failed because of unimplemented functionality.
  CRONET_NET_ERROR_NOT_IMPLEMENTED = -11,

  // There were not enough resources to complete the operation.
  CRONET_NET_ERROR_INSUFFICIENT_RESOURCES = -12,

  // Memory allocation failed.
  CRONET_NET_ERROR_OUT_OF_MEMORY = -13,

  // The file upload failed because the file's modification time was different
  // from the expectation.
  CRONET_NET_ERROR_UPLOAD_FILE_CHANGED = -14,

  // The socket is not connected.
  CRONET_NET_ERROR_SOCKET_NOT_CONNECTED = -15,

  // The file already exists.
  CRONET_NET_ERROR_FILE_EXISTS = -16,

  // The path or file name is too long.
  CRONET_NET_ERROR_FILE_PATH_TOO_LONG = -17,

  // Not enough room left on the disk.
  CRONET_NET_ERROR_FILE_NO_SPACE = -18,

  // The file has a virus.
  CRONET_NET_ERROR_FILE_VIRUS_INFECTED = -19,

  // The client chose to block the request.
  CRONET_NET_ERROR_BLOCKED_BY_CLIENT = -20,

  // The network changed.
  CRONET_NET_ERROR_NETWORK_CHANGED = -21,

  // The request was blocked by the URL block list configured by the domain
  // administrator.
  CRONET_NET_ERROR_BLOCKED_BY_ADMINISTRATOR = -22,

  // The socket is already connected.
  CRONET_NET_ERROR_SOCKET_IS_CONNECTED = -23,

  // The request was blocked because the forced reenrollment check is still
  // pending. This error can only occur on ChromeOS.
  // The error can be emitted by code in
  // chrome/browser/policy/policy_helpers.cc.
  CRONET_NET_ERROR_BLOCKED_ENROLLMENT_CHECK_PENDING = -24,

  // The upload failed because the upload stream needed to be re-read = due to a
  // retry or a redirect = but the upload stream doesn't support that operation.
  CRONET_NET_ERROR_UPLOAD_STREAM_REWIND_NOT_SUPPORTED = -25,

  // The request failed because the URLRequestContext is shutting down = or has
  // been shut down.
  CRONET_NET_ERROR_CONTEXT_SHUT_DOWN = -26,

  // The request failed because the response was delivered along with
  // requirements
  // which are not met ('X-Frame-Options' and 'Content-Security-Policy' ancestor
  // checks and 'Cross-Origin-Resource-Policy' = for instance,.
  CRONET_NET_ERROR_BLOCKED_BY_RESPONSE = -27,

  // Error -28 was removed (BLOCKED_BY_XSS_AUDITOR,.

  // The request was blocked by system policy disallowing some or all cleartext
  // requests. Used for NetworkSecurityPolicy on Android.
  CRONET_NET_ERROR_CLEARTEXT_NOT_PERMITTED = -29,

  // The request was blocked by a Content Security Policy
  CRONET_NET_ERROR_BLOCKED_BY_CSP = -30,

  // The request was blocked because of no H/2 or QUIC session.
  CRONET_NET_ERROR_H2_OR_QUIC_REQUIRED = -31,

  // The request was blocked because it is a private network request coming from
  // an insecure context in a less private IP address space. This is used to
  // enforce CORS-RFC1918: https:  //wicg.github.io/cors-rfc1918.
  CRONET_NET_ERROR_INSECURE_PRIVATE_NETWORK_REQUEST = -32,

  // A connection was closed (corresponding to a TCP FIN,.
  CRONET_NET_ERROR_CONNECTION_CLOSED = -100,

  // A connection was reset (corresponding to a TCP RST,.
  CRONET_NET_ERROR_CONNECTION_RESET = -101,

  // A connection attempt was refused.
  CRONET_NET_ERROR_CONNECTION_REFUSED = -102,

  // A connection timed out as a result of not receiving an ACK for data sent.
  // This can include a FIN packet that did not get ACK'd.
  CRONET_NET_ERROR_CONNECTION_ABORTED = -103,

  // A connection attempt failed.
  CRONET_NET_ERROR_CONNECTION_FAILED = -104,

  // The host name could not be resolved.
  CRONET_NET_ERROR_NAME_NOT_RESOLVED = -105,

  // The Internet connection has been lost.
  CRONET_NET_ERROR_INTERNET_DISCONNECTED = -106,

  // An SSL protocol error occurred.
  CRONET_NET_ERROR_SSL_PROTOCOL_ERROR = -107,

  // The IP address or port number is invalid (e.g. = cannot connect to the IP
  // address 0 or the port 0,.
  CRONET_NET_ERROR_ADDRESS_INVALID = -108,

  // The IP address is unreachable.  This usually means that there is no route
  // to
  // the specified host or network.
  CRONET_NET_ERROR_ADDRESS_UNREACHABLE = -109,

  // The server requested a client certificate for SSL client authentication.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_CERT_NEEDED = -110,

  // A tunnel connection through the proxy could not be established.
  CRONET_NET_ERROR_TUNNEL_CONNECTION_FAILED = -111,

  // No SSL protocol versions are enabled.
  CRONET_NET_ERROR_NO_SSL_VERSIONS_ENABLED = -112,

  // The client and server don't support a common SSL protocol version or
  // cipher suite.
  CRONET_NET_ERROR_SSL_VERSION_OR_CIPHER_MISMATCH = -113,

  // The server requested a renegotiation (rehandshake,.
  CRONET_NET_ERROR_SSL_RENEGOTIATION_REQUESTED = -114,

  // The proxy requested authentication (for tunnel establishment, with an
  // unsupported method.
  CRONET_NET_ERROR_PROXY_AUTH_UNSUPPORTED = -115,

  // During SSL renegotiation (rehandshake, = the server sent a certificate with
  // an error.
  //
  // Note: this error is not in the -2xx range so that it won't be handled as a
  // certificate error.
  CRONET_NET_ERROR_CERT_ERROR_IN_SSL_RENEGOTIATION = -116,

  // The SSL handshake failed because of a bad or missing client certificate.
  CRONET_NET_ERROR_BAD_SSL_CLIENT_AUTH_CERT = -117,

  // A connection attempt timed out.
  CRONET_NET_ERROR_CONNECTION_TIMED_OUT = -118,

  // There are too many pending DNS resolves = so a request in the queue was
  // aborted.
  CRONET_NET_ERROR_HOST_RESOLVER_QUEUE_TOO_LARGE = -119,

  // Failed establishing a connection to the SOCKS proxy server for a target
  // host.
  CRONET_NET_ERROR_SOCKS_CONNECTION_FAILED = -120,

  // The SOCKS proxy server failed establishing connection to the target host
  // because that host is unreachable.
  CRONET_NET_ERROR_SOCKS_CONNECTION_HOST_UNREACHABLE = -121,

  // The request to negotiate an alternate protocol failed.
  CRONET_NET_ERROR_ALPN_NEGOTIATION_FAILED = -122,

  // The peer sent an SSL no_renegotiation alert message.
  CRONET_NET_ERROR_SSL_NO_RENEGOTIATION = -123,

  // Winsock sometimes reports more data written than passed.  This is probably
  // due to a broken LSP.
  CRONET_NET_ERROR_WINSOCK_UNEXPECTED_WRITTEN_BYTES = -124,

  // An SSL peer sent us a fatal decompression_failure alert. This typically
  // occurs when a peer selects DEFLATE compression in the mistaken belief that
  // it supports it.
  CRONET_NET_ERROR_SSL_DECOMPRESSION_FAILURE_ALERT = -125,

  // An SSL peer sent us a fatal bad_record_mac alert. This has been observed
  // from servers with buggy DEFLATE support.
  CRONET_NET_ERROR_SSL_BAD_RECORD_MAC_ALERT = -126,

  // The proxy requested authentication (for tunnel establishment,.
  CRONET_NET_ERROR_PROXY_AUTH_REQUESTED = -127,

  // Error -129 was removed (SSL_WEAK_SERVER_EPHEMERAL_DH_KEY,.

  // Could not create a connection to the proxy server. An error occurred
  // either in resolving its name = or in connecting a socket to it.
  // Note that this does NOT include failures during the actual "CONNECT" method
  // of an HTTP proxy.
  CRONET_NET_ERROR_PROXY_CONNECTION_FAILED = -130,

  // A mandatory proxy configuration could not be used. Currently this means
  // that a mandatory PAC script could not be fetched = parsed or executed.
  CRONET_NET_ERROR_MANDATORY_PROXY_CONFIGURATION_FAILED = -131,

  // -132 was formerly ERR_ESET_ANTI_VIRUS_SSL_INTERCEPTION

  // We've hit the max socket limit for the socket pool while preconnecting.  We
  // don't bother trying to preconnect more sockets.
  CRONET_NET_ERROR_PRECONNECT_MAX_SOCKET_LIMIT = -133,

  // The permission to use the SSL client certificate's private key was denied.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_PRIVATE_KEY_ACCESS_DENIED = -134,

  // The SSL client certificate has no private key.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_CERT_NO_PRIVATE_KEY = -135,

  // The certificate presented by the HTTPS Proxy was invalid.
  CRONET_NET_ERROR_PROXY_CERTIFICATE_INVALID = -136,

  // An error occurred when trying to do a name resolution (DNS,.
  CRONET_NET_ERROR_NAME_RESOLUTION_FAILED = -137,

  // Permission to access the network was denied. This is used to distinguish
  // errors that were most likely caused by a firewall from other access denied
  // errors. See also ERR_ACCESS_DENIED.
  CRONET_NET_ERROR_NETWORK_ACCESS_DENIED = -138,

  // The request throttler module cancelled this request to avoid DDOS.
  CRONET_NET_ERROR_TEMPORARILY_THROTTLED = -139,

  // A request to create an SSL tunnel connection through the HTTPS proxy
  // received a 302 (temporary redirect, response.  The response body might
  // include a description of why the request failed.
  //
  // TODO(crbug.com/928551): This is deprecated and should not be used
  // by new code.
  CRONET_NET_ERROR_HTTPS_PROXY_TUNNEL_RESPONSE_REDIRECT = -140,

  // We were unable to sign the CertificateVerify data of an SSL client auth
  // handshake with the client certificate's private key.
  //
  // Possible causes for this include the user implicitly or explicitly
  // denying access to the private key = the private key may not be valid for
  // signing = the key may be relying on a cached handle which is no longer
  // valid = or the CSP won't allow arbitrary data to be signed.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_SIGNATURE_FAILED = -141,

  // The message was too large for the transport.  (for example a UDP message
  // which exceeds size threshold,.
  CRONET_NET_ERROR_MSG_TOO_BIG = -142,

  // Error -143 was removed (SPDY_SESSION_ALREADY_EXISTS,

  // Error -144 was removed (LIMIT_VIOLATION,.

  // Websocket protocol error. Indicates that we are terminating the connection
  // due to a malformed frame or other protocol violation.
  CRONET_NET_ERROR_WS_PROTOCOL_ERROR = -145,

  // Error -146 was removed (PROTOCOL_SWITCHED,

  // Returned when attempting to bind an address that is already in use.
  CRONET_NET_ERROR_ADDRESS_IN_USE = -147,

  // An operation failed because the SSL handshake has not completed.
  CRONET_NET_ERROR_SSL_HANDSHAKE_NOT_COMPLETED = -148,

  // SSL peer's public key is invalid.
  CRONET_NET_ERROR_SSL_BAD_PEER_PUBLIC_KEY = -149,

  // The certificate didn't match the built-in public key pins for the host
  // name.
  // The pins are set in net/http/transport_security_state.cc and require that
  // one of a set of public keys exist on the path from the leaf to the root.
  CRONET_NET_ERROR_SSL_PINNED_KEY_NOT_IN_CERT_CHAIN = -150,

  // Server request for client certificate did not contain any types we support.
  CRONET_NET_ERROR_CLIENT_AUTH_CERT_TYPE_UNSUPPORTED = -151,

  // Error -152 was removed (ORIGIN_BOUND_CERT_GENERATION_TYPE_MISMATCH,

  // An SSL peer sent us a fatal decrypt_error alert. This typically occurs when
  // a peer could not correctly verify a signature (in CertificateVerify or
  // ServerKeyExchange, or validate a Finished message.
  CRONET_NET_ERROR_SSL_DECRYPT_ERROR_ALERT = -153,

  // There are too many pending WebSocketJob instances = so the new job was not
  // pushed to the queue.
  CRONET_NET_ERROR_WS_THROTTLE_QUEUE_TOO_LARGE = -154,

  // Error -155 was removed (TOO_MANY_SOCKET_STREAMS,

  // The SSL server certificate changed in a renegotiation.
  CRONET_NET_ERROR_SSL_SERVER_CERT_CHANGED = -156,

  // Error -157 was removed (SSL_INAPPROPRIATE_FALLBACK,.

  // Error -158 was removed (CT_NO_SCTS_VERIFIED_OK,.

  // The SSL server sent us a fatal unrecognized_name alert.
  CRONET_NET_ERROR_SSL_UNRECOGNIZED_NAME_ALERT = -159,

  // Failed to set the socket's receive buffer size as requested.
  CRONET_NET_ERROR_SOCKET_SET_RECEIVE_BUFFER_SIZE_ERROR = -160,

  // Failed to set the socket's send buffer size as requested.
  CRONET_NET_ERROR_SOCKET_SET_SEND_BUFFER_SIZE_ERROR = -161,

  // Failed to set the socket's receive buffer size as requested = despite
  // success
  // return code from setsockopt.
  CRONET_NET_ERROR_SOCKET_RECEIVE_BUFFER_SIZE_UNCHANGEABLE = -162,

  // Failed to set the socket's send buffer size as requested = despite success
  // return code from setsockopt.
  CRONET_NET_ERROR_SOCKET_SEND_BUFFER_SIZE_UNCHANGEABLE = -163,

  // Failed to import a client certificate from the platform store into the SSL
  // library.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_CERT_BAD_FORMAT = -164,

  // Error -165 was removed (SSL_FALLBACK_BEYOND_MINIMUM_VERSION,.

  // Resolving a hostname to an IP address list included the IPv4 address
  // "127.0.53.53". This is a special IP address which ICANN has recommended to
  // indicate there was a name collision = and alert admins to a potential
  // problem.
  CRONET_NET_ERROR_ICANN_NAME_COLLISION = -166,

  // The SSL server presented a certificate which could not be decoded. This is
  // not a certificate error code as no X509Certificate object is available.
  // This
  // error is fatal.
  CRONET_NET_ERROR_SSL_SERVER_CERT_BAD_FORMAT = -167,

  // Certificate Transparency: Received a signed tree head that failed to parse.
  CRONET_NET_ERROR_CT_STH_PARSING_FAILED = -168,

  // Certificate Transparency: Received a signed tree head whose JSON parsing
  // was
  // OK but was missing some of the fields.
  CRONET_NET_ERROR_CT_STH_INCOMPLETE = -169,

  // The attempt to reuse a connection to send proxy auth credentials failed
  // before the AuthController was used to generate credentials. The caller
  // should
  // reuse the controller with a new connection. This error is only used
  // internally by the network stack.
  CRONET_NET_ERROR_UNABLE_TO_REUSE_CONNECTION_FOR_PROXY_AUTH = -170,

  // Certificate Transparency: Failed to parse the received consistency proof.
  CRONET_NET_ERROR_CT_CONSISTENCY_PROOF_PARSING_FAILED = -171,

  // The SSL server required an unsupported cipher suite that has since been
  // removed. This error will temporarily be signaled on a fallback for one or
  // two
  // releases immediately following a cipher suite's removal = after which the
  // fallback will be removed.
  CRONET_NET_ERROR_SSL_OBSOLETE_CIPHER = -172,

  // When a WebSocket handshake is done successfully and the connection has been
  // upgraded = the URLRequest is cancelled with this error code.
  CRONET_NET_ERROR_WS_UPGRADE = -173,

  // Socket ReadIfReady support is not implemented. This error should not be
  // user
  // visible = because the normal Read(, method is used as a fallback.
  CRONET_NET_ERROR_READ_IF_READY_NOT_IMPLEMENTED = -174,

  // Error -175 was removed (SSL_VERSION_INTERFERENCE,.

  // No socket buffer space is available.
  CRONET_NET_ERROR_NO_BUFFER_SPACE = -176,

  // There were no common signature algorithms between our client certificate
  // private key and the server's preferences.
  CRONET_NET_ERROR_SSL_CLIENT_AUTH_NO_COMMON_ALGORITHMS = -177,

  // TLS 1.3 early data was rejected by the server. This will be received before
  // any data is returned from the socket. The request should be retried with
  // early data disabled.
  CRONET_NET_ERROR_EARLY_DATA_REJECTED = -178,

  // TLS 1.3 early data was offered = but the server responded with TLS 1.2 or
  // earlier. This is an internal error code to account for a
  // backwards-compatibility issue with early data and TLS 1.2. It will be
  // received before any data is returned from the socket. The request should be
  // retried with early data disabled.
  //
  // See https:  //tools.ietf.org/html/rfc8446#appendix-D.3 for details.
  CRONET_NET_ERROR_WRONG_VERSION_ON_EARLY_DATA = -179,

  // TLS 1.3 was enabled = but a lower version was negotiated and the server
  // returned a value indicating it supported TLS 1.3. This is part of a
  // security
  // check in TLS 1.3 = but it may also indicate the user is behind a buggy
  // TLS-terminating proxy which implemented TLS 1.2 incorrectly. (See
  // https:  //crbug.com/boringssl/226.,
  CRONET_NET_ERROR_TLS13_DOWNGRADE_DETECTED = -180,

  // The server's certificate has a keyUsage extension incompatible with the
  // negotiated TLS key exchange method.
  CRONET_NET_ERROR_SSL_KEY_USAGE_INCOMPATIBLE = -181,

  // Certificate error codes
  //
  // The values of certificate error codes must be consecutive.

  // The server responded with a certificate whose common name did not match
  // the host name.  This could mean:
  //
  // 1. An attacker has redirected our traffic to their server and is
  //    presenting a certificate for which they know the private key.
  //
  // 2. The server is misconfigured and responding with the wrong cert.
  //
  // 3. The user is on a wireless network and is being redirected to the
  //    network's login page.
  //
  // 4. The OS has used a DNS search suffix and the server doesn't have
  //    a certificate for the abbreviated name in the address bar.
  //
  CRONET_NET_ERROR_CERT_COMMON_NAME_INVALID = -200,

  // The server responded with a certificate that = by our clock = appears to
  // either not yet be valid or to have expired.  This could mean:
  //
  // 1. An attacker is presenting an old certificate for which they have
  //    managed to obtain the private key.
  //
  // 2. The server is misconfigured and is not presenting a valid cert.
  //
  // 3. Our clock is wrong.
  //
  CRONET_NET_ERROR_CERT_DATE_INVALID = -201,

  // The server responded with a certificate that is signed by an authority
  // we don't trust.  The could mean:
  //
  // 1. An attacker has substituted the real certificate for a cert that
  //    contains their public key and is signed by their cousin.
  //
  // 2. The server operator has a legitimate certificate from a CA we don't
  //    know about = but should trust.
  //
  // 3. The server is presenting a self-signed certificate = providing no
  //    defense against active attackers (but foiling passive attackers,.
  //
  CRONET_NET_ERROR_CERT_AUTHORITY_INVALID = -202,

  // The server responded with a certificate that contains errors.
  // This error is not recoverable.
  //
  // MSDN describes this error as follows:
  //   "The SSL certificate contains errors."
  // NOTE: It's unclear how this differs from ERR_CERT_INVALID. For consistency
  // =
  // use that code instead of this one from now on.
  //
  CRONET_NET_ERROR_CERT_CONTAINS_ERRORS = -203,

  // The certificate has no mechanism for determining if it is revoked.  In
  // effect = this certificate cannot be revoked.
  CRONET_NET_ERROR_CERT_NO_REVOCATION_MECHANISM = -204,

  // Revocation information for the security certificate for this site is not
  // available.  This could mean:
  //
  // 1. An attacker has compromised the private key in the certificate and is
  //    blocking our attempt to find out that the cert was revoked.
  //
  // 2. The certificate is unrevoked = but the revocation server is busy or
  //    unavailable.
  //
  CRONET_NET_ERROR_CERT_UNABLE_TO_CHECK_REVOCATION = -205,

  // The server responded with a certificate has been revoked.
  // We have the capability to ignore this error = but it is probably not the
  // thing to do.
  CRONET_NET_ERROR_CERT_REVOKED = -206,

  // The server responded with a certificate that is invalid.
  // This error is not recoverable.
  //
  // MSDN describes this error as follows:
  //   "The SSL certificate is invalid."
  //
  CRONET_NET_ERROR_CERT_INVALID = -207,

  // The server responded with a certificate that is signed using a weak
  // signature algorithm.
  CRONET_NET_ERROR_CERT_WEAK_SIGNATURE_ALGORITHM = -208,

  // -209 is available: was CERT_NOT_IN_DNS.

  // The host name specified in the certificate is not unique.
  CRONET_NET_ERROR_CERT_NON_UNIQUE_NAME = -210,

  // The server responded with a certificate that contains a weak key (e.g.
  // a too-small RSA key,.
  CRONET_NET_ERROR_CERT_WEAK_KEY = -211,

  // The certificate claimed DNS names that are in violation of name
  // constraints.
  CRONET_NET_ERROR_CERT_NAME_CONSTRAINT_VIOLATION = -212,

  // The certificate's validity period is too long.
  CRONET_NET_ERROR_CERT_VALIDITY_TOO_LONG = -213,

  // Certificate Transparency was required for this connection = but the server
  // did not provide CT information that complied with the policy.
  CRONET_NET_ERROR_CERTIFICATE_TRANSPARENCY_REQUIRED = -214,

  // The certificate chained to a legacy Symantec root that is no longer
  // trusted.
  // https:  //g.co/chrome/symantecpkicerts
  CRONET_NET_ERROR_CERT_SYMANTEC_LEGACY = -215,

  // -216 was QUIC_CERT_ROOT_NOT_KNOWN which has been renumbered to not be in
  // the
  // certificate error range.

  // The certificate is known to be used for interception by an entity other
  // the device owner.
  CRONET_NET_ERROR_CERT_KNOWN_INTERCEPTION_BLOCKED = -217,

  // The connection uses an obsolete version of SSL/TLS.
  CRONET_NET_ERROR_SSL_OBSOLETE_VERSION = -218,

  // Add new certificate error codes here.
  //
  // Update the value of CERT_END whenever you add a new certificate error
  // code.

  // The value immediately past the last certificate error code.
  CRONET_NET_ERROR_CERT_END = -219,

  // The URL is invalid.
  CRONET_NET_ERROR_INVALID_URL = -300,

  // The scheme of the URL is disallowed.
  CRONET_NET_ERROR_DISALLOWED_URL_SCHEME = -301,

  // The scheme of the URL is unknown.
  CRONET_NET_ERROR_UNKNOWN_URL_SCHEME = -302,

  // Attempting to load an URL resulted in a redirect to an invalid URL.
  CRONET_NET_ERROR_INVALID_REDIRECT = -303,

  // Attempting to load an URL resulted in too many redirects.
  CRONET_NET_ERROR_TOO_MANY_REDIRECTS = -310,

  // Attempting to load an URL resulted in an unsafe redirect (e.g. = a redirect
  // to file:  // is considered unsafe,.
  CRONET_NET_ERROR_UNSAFE_REDIRECT = -311,

  // Attempting to load an URL with an unsafe port number.  These are port
  // numbers that correspond to services = which are not robust to spurious
  // input
  // that may be constructed as a result of an allowed web construct (e.g. =
  // HTTP
  // looks a lot like SMTP = so form submission to port 25 is denied,.
  CRONET_NET_ERROR_UNSAFE_PORT = -312,

  // The server's response was invalid.
  CRONET_NET_ERROR_INVALID_RESPONSE = -320,

  // Error in chunked transfer encoding.
  CRONET_NET_ERROR_INVALID_CHUNKED_ENCODING = -321,

  // The server did not support the request method.
  CRONET_NET_ERROR_METHOD_NOT_SUPPORTED = -322,

  // The response was 407 (Proxy Authentication Required, = yet we did not send
  // the request to a proxy.
  CRONET_NET_ERROR_UNEXPECTED_PROXY_AUTH = -323,

  // The server closed the connection without sending any data.
  CRONET_NET_ERROR_EMPTY_RESPONSE = -324,

  // The headers section of the response is too large.
  CRONET_NET_ERROR_RESPONSE_HEADERS_TOO_BIG = -325,

  // Error -326 was removed (PAC_STATUS_NOT_OK,

  // The evaluation of the PAC script failed.
  CRONET_NET_ERROR_PAC_SCRIPT_FAILED = -327,

  // The response was 416 (Requested range not satisfiable, and the server
  // cannot
  // satisfy the range requested.
  CRONET_NET_ERROR_REQUEST_RANGE_NOT_SATISFIABLE = -328,

  // The identity used for authentication is invalid.
  CRONET_NET_ERROR_MALFORMED_IDENTITY = -329,

  // Content decoding of the response body failed.
  CRONET_NET_ERROR_CONTENT_DECODING_FAILED = -330,

  // An operation could not be completed because all network IO
  // is suspended.
  CRONET_NET_ERROR_NETWORK_IO_SUSPENDED = -331,

  // FLIP data received without receiving a SYN_REPLY on the stream.
  CRONET_NET_ERROR_SYN_REPLY_NOT_RECEIVED = -332,

  // Converting the response to target encoding failed.
  CRONET_NET_ERROR_ENCODING_CONVERSION_FAILED = -333,

  // The server sent an FTP directory listing in a format we do not understand.
  CRONET_NET_ERROR_UNRECOGNIZED_FTP_DIRECTORY_LISTING_FORMAT = -334,

  // Obsolete.  Was only logged in NetLog when an HTTP/2 pushed stream expired.
  //   CRONET_NET_ERROR_INVALID_SPDY_STREAM = -335,

  // There are no supported proxies in the provided list.
  CRONET_NET_ERROR_NO_SUPPORTED_PROXIES = -336,

  // There is an HTTP/2 protocol error.
  CRONET_NET_ERROR_HTTP2_PROTOCOL_ERROR = -337,

  // Credentials could not be established during HTTP Authentication.
  CRONET_NET_ERROR_INVALID_AUTH_CREDENTIALS = -338,

  // An HTTP Authentication scheme was tried which is not supported on this
  // machine.
  CRONET_NET_ERROR_UNSUPPORTED_AUTH_SCHEME = -339,

  // Detecting the encoding of the response failed.
  CRONET_NET_ERROR_ENCODING_DETECTION_FAILED = -340,

  // (GSSAPI, No Kerberos credentials were available during HTTP Authentication.
  CRONET_NET_ERROR_MISSING_AUTH_CREDENTIALS = -341,

  // An unexpected = but documented = SSPI or GSSAPI status code was returned.
  CRONET_NET_ERROR_UNEXPECTED_SECURITY_LIBRARY_STATUS = -342,

  // The environment was not set up correctly for authentication (for
  // example = no KDC could be found or the principal is unknown.
  CRONET_NET_ERROR_MISCONFIGURED_AUTH_ENVIRONMENT = -343,

  // An undocumented SSPI or GSSAPI status code was returned.
  CRONET_NET_ERROR_UNDOCUMENTED_SECURITY_LIBRARY_STATUS = -344,

  // The HTTP response was too big to drain.
  CRONET_NET_ERROR_RESPONSE_BODY_TOO_BIG_TO_DRAIN = -345,

  // The HTTP response contained multiple distinct Content-Length headers.
  CRONET_NET_ERROR_RESPONSE_HEADERS_MULTIPLE_CONTENT_LENGTH = -346,

  // HTTP/2 headers have been received = but not all of them - status or version
  // headers are missing = so we're expecting additional frames to complete
  // them.
  CRONET_NET_ERROR_INCOMPLETE_HTTP2_HEADERS = -347,

  // No PAC URL configuration could be retrieved from DHCP. This can indicate
  // either a failure to retrieve the DHCP configuration = or that there was no
  // PAC URL configured in DHCP.
  CRONET_NET_ERROR_PAC_NOT_IN_DHCP = -348,

  // The HTTP response contained multiple Content-Disposition headers.
  CRONET_NET_ERROR_RESPONSE_HEADERS_MULTIPLE_CONTENT_DISPOSITION = -349,

  // The HTTP response contained multiple Location headers.
  CRONET_NET_ERROR_RESPONSE_HEADERS_MULTIPLE_LOCATION = -350,

  // HTTP/2 server refused the request without processing = and sent either a
  // GOAWAY frame with error code NO_ERROR and Last-Stream-ID lower than the
  // stream id corresponding to the request indicating that this request has not
  // been processed yet = or a RST_STREAM frame with error code REFUSED_STREAM.
  // Client MAY retry (on a different connection,.  See RFC7540 Section 8.1.4.
  CRONET_NET_ERROR_HTTP2_SERVER_REFUSED_STREAM = -351,

  // HTTP/2 server didn't respond to the PING message.
  CRONET_NET_ERROR_HTTP2_PING_FAILED = -352,

  // Obsolete.  Kept here to avoid reuse = as the old error can still appear on
  // histograms.
  //   CRONET_NET_ERROR_PIPELINE_EVICTION = -353,

  // The HTTP response body transferred fewer bytes than were advertised by the
  // Content-Length header when the connection is closed.
  CRONET_NET_ERROR_CONTENT_LENGTH_MISMATCH = -354,

  // The HTTP response body is transferred with Chunked-Encoding = but the
  // terminating zero-length chunk was never sent when the connection is closed.
  CRONET_NET_ERROR_INCOMPLETE_CHUNKED_ENCODING = -355,

  // There is a QUIC protocol error.
  CRONET_NET_ERROR_QUIC_PROTOCOL_ERROR = -356,

  // The HTTP headers were truncated by an EOF.
  CRONET_NET_ERROR_RESPONSE_HEADERS_TRUNCATED = -357,

  // The QUIC crypto handshake failed.  This means that the server was unable
  // to read any requests sent = so they may be resent.
  CRONET_NET_ERROR_QUIC_HANDSHAKE_FAILED = -358,

  // Obsolete.  Kept here to avoid reuse = as the old error can still appear on
  // histograms.
  //   CRONET_NET_ERROR_REQUEST_FOR_SECURE_RESOURCE_OVER_INSECURE_QUIC = -359,

  // Transport security is inadequate for the HTTP/2 version.
  CRONET_NET_ERROR_HTTP2_INADEQUATE_TRANSPORT_SECURITY = -360,

  // The peer violated HTTP/2 flow control.
  CRONET_NET_ERROR_HTTP2_FLOW_CONTROL_ERROR = -361,

  // The peer sent an improperly sized HTTP/2 frame.
  CRONET_NET_ERROR_HTTP2_FRAME_SIZE_ERROR = -362,

  // Decoding or encoding of compressed HTTP/2 headers failed.
  CRONET_NET_ERROR_HTTP2_COMPRESSION_ERROR = -363,

  // Proxy Auth Requested without a valid Client Socket Handle.
  CRONET_NET_ERROR_PROXY_AUTH_REQUESTED_WITH_NO_CONNECTION = -364,

  // HTTP_1_1_REQUIRED error code received on HTTP/2 session.
  CRONET_NET_ERROR_HTTP_1_1_REQUIRED = -365,

  // HTTP_1_1_REQUIRED error code received on HTTP/2 session to proxy.
  CRONET_NET_ERROR_PROXY_HTTP_1_1_REQUIRED = -366,

  // The PAC script terminated fatally and must be reloaded.
  CRONET_NET_ERROR_PAC_SCRIPT_TERMINATED = -367,

  // Obsolete. Kept here to avoid reuse.
  // Request is throttled because of a Backoff header.
  // See: crbug.com/486891.
  //   CRONET_NET_ERROR_TEMPORARY_BACKOFF = -369,

  // The server was expected to return an HTTP/1.x response = but did not.
  // Rather
  // than treat it as HTTP/0.9 = this error is returned.
  CRONET_NET_ERROR_INVALID_HTTP_RESPONSE = -370,

  // Initializing content decoding failed.
  CRONET_NET_ERROR_CONTENT_DECODING_INIT_FAILED = -371,

  // Received HTTP/2 RST_STREAM frame with NO_ERROR error code.  This error
  // should
  // be handled internally by HTTP/2 code = and should not make it above the
  // SpdyStream layer.
  CRONET_NET_ERROR_HTTP2_RST_STREAM_NO_ERROR_RECEIVED = -372,

  // The pushed stream claimed by the request is no longer available.
  CRONET_NET_ERROR_HTTP2_PUSHED_STREAM_NOT_AVAILABLE = -373,

  // A pushed stream was claimed and later reset by the server. When this
  // happens =
  // the request should be retried.
  CRONET_NET_ERROR_HTTP2_CLAIMED_PUSHED_STREAM_RESET_BY_SERVER = -374,

  // An HTTP transaction was retried too many times due for authentication or
  // invalid certificates. This may be due to a bug in the net stack that would
  // otherwise infinite loop = or if the server or proxy continually requests
  // fresh
  // credentials or presents a fresh invalid certificate.
  CRONET_NET_ERROR_TOO_MANY_RETRIES = -375,

  // Received an HTTP/2 frame on a closed stream.
  CRONET_NET_ERROR_HTTP2_STREAM_CLOSED = -376,

  // Client is refusing an HTTP/2 stream.
  CRONET_NET_ERROR_HTTP2_CLIENT_REFUSED_STREAM = -377,

  // A pushed HTTP/2 stream was claimed by a request based on matching URL and
  // request headers = but the pushed response headers do not match the request.
  CRONET_NET_ERROR_HTTP2_PUSHED_RESPONSE_DOES_NOT_MATCH = -378,

  // The server returned a non-2xx HTTP response code.
  //
  // Not that this error is only used by certain APIs that interpret the HTTP
  // response itself. URLRequest for instance just passes most non-2xx
  // response back as success.
  CRONET_NET_ERROR_HTTP_RESPONSE_CODE_FAILURE = -379,

  // The certificate presented on a QUIC connection does not chain to a known
  // root
  // and the origin connected to is not on a list of domains where unknown roots
  // are allowed.
  CRONET_NET_ERROR_QUIC_CERT_ROOT_NOT_KNOWN = -380,

  // The cache does not have the requested entry.
  CRONET_NET_ERROR_CACHE_MISS = -400,

  // Unable to read from the disk cache.
  CRONET_NET_ERROR_CACHE_READ_FAILURE = -401,

  // Unable to write to the disk cache.
  CRONET_NET_ERROR_CACHE_WRITE_FAILURE = -402,

  // The operation is not supported for this entry.
  CRONET_NET_ERROR_CACHE_OPERATION_NOT_SUPPORTED = -403,

  // The disk cache is unable to open this entry.
  CRONET_NET_ERROR_CACHE_OPEN_FAILURE = -404,

  // The disk cache is unable to create this entry.
  CRONET_NET_ERROR_CACHE_CREATE_FAILURE = -405,

  // Multiple transactions are racing to create disk cache entries. This is an
  // internal error returned from the HttpCache to the HttpCacheTransaction that
  // tells the transaction to restart the entry-creation logic because the state
  // of the cache has changed.
  CRONET_NET_ERROR_CACHE_RACE = -406,

  // The cache was unable to read a checksum record on an entry. This can be
  // returned from attempts to read from the cache. It is an internal error =
  // returned by the SimpleCache backend = but not by any URLRequest methods
  // or members.
  CRONET_NET_ERROR_CACHE_CHECKSUM_READ_FAILURE = -407,

  // The cache found an entry with an invalid checksum. This can be returned
  // from
  // attempts to read from the cache. It is an internal error = returned by the
  // SimpleCache backend = but not by any URLRequest methods or members.
  CRONET_NET_ERROR_CACHE_CHECKSUM_MISMATCH = -408,

  // Internal error code for the HTTP cache. The cache lock timeout has fired.
  CRONET_NET_ERROR_CACHE_LOCK_TIMEOUT = -409,

  // Received a challenge after the transaction has read some data = and the
  // credentials aren't available.  There isn't a way to get them at that point.
  CRONET_NET_ERROR_CACHE_AUTH_FAILURE_AFTER_READ = -410,

  // Internal not-quite error code for the HTTP cache. In-memory hints suggest
  // that the cache entry would not have been useable with the transaction's
  // current configuration (e.g. load flags = mode = etc.,
  CRONET_NET_ERROR_CACHE_ENTRY_NOT_SUITABLE = -411,

  // The disk cache is unable to doom this entry.
  CRONET_NET_ERROR_CACHE_DOOM_FAILURE = -412,

  // The disk cache is unable to open or create this entry.
  CRONET_NET_ERROR_CACHE_OPEN_OR_CREATE_FAILURE = -413,

  // The server's response was insecure (e.g. there was a cert error,.
  CRONET_NET_ERROR_INSECURE_RESPONSE = -501,

  // An attempt to import a client certificate failed = as the user's key
  // database lacked a corresponding private key.
  CRONET_NET_ERROR_NO_PRIVATE_KEY_FOR_CERT = -502,

  // An error adding a certificate to the OS certificate database.
  CRONET_NET_ERROR_ADD_USER_CERT_FAILED = -503,

  // An error occurred while handling a signed exchange.
  CRONET_NET_ERROR_INVALID_SIGNED_EXCHANGE = -504,

  // An error occurred while handling a Web Bundle source.
  CRONET_NET_ERROR_INVALID_WEB_BUNDLE = -505,

  // A Trust Tokens protocol operation-executing request failed for one of a
  // number of reasons (precondition failure = internal error = bad response,.
  CRONET_NET_ERROR_TRUST_TOKEN_OPERATION_FAILED = -506,

  // When handling a Trust Tokens protocol operation-executing request = the
  // system
  // found that the request's desired Trust Tokens results were already present
  // in
  // a local cache; as a result = the main request was cancelled.
  CRONET_NET_ERROR_TRUST_TOKEN_OPERATION_CACHE_HIT = -507,

  // *** Code -600 is reserved (was FTP_PASV_COMMAND_FAILED,. ***

  // A generic error for failed FTP control connection command.
  // If possible = please use or add a more specific error code.
  CRONET_NET_ERROR_FTP_FAILED = -601,

  // The server cannot fulfill the request at this point. This is a temporary
  // error.
  // FTP response code 421.
  CRONET_NET_ERROR_FTP_SERVICE_UNAVAILABLE = -602,

  // The server has aborted the transfer.
  // FTP response code 426.
  CRONET_NET_ERROR_FTP_TRANSFER_ABORTED = -603,

  // The file is busy = or some other temporary error condition on opening
  // the file.
  // FTP response code 450.
  CRONET_NET_ERROR_FTP_FILE_BUSY = -604,

  // Server rejected our command because of syntax errors.
  // FTP response codes 500 = 501.
  CRONET_NET_ERROR_FTP_SYNTAX_ERROR = -605,

  // Server does not support the command we issued.
  // FTP response codes 502 = 504.
  CRONET_NET_ERROR_FTP_COMMAND_NOT_SUPPORTED = -606,

  // Server rejected our command because we didn't issue the commands in right
  // order.
  // FTP response code 503.
  CRONET_NET_ERROR_FTP_BAD_COMMAND_SEQUENCE = -607,

  // PKCS #12 import failed due to incorrect password.
  CRONET_NET_ERROR_PKCS12_IMPORT_BAD_PASSWORD = -701,

  // PKCS #12 import failed due to other error.
  CRONET_NET_ERROR_PKCS12_IMPORT_FAILED = -702,

  // CA import failed - not a CA cert.
  CRONET_NET_ERROR_IMPORT_CA_CERT_NOT_CA = -703,

  // Import failed - certificate already exists in database.
  // Note it's a little weird this is an error but reimporting a PKCS12 is ok
  // (no-op,.  That's how Mozilla does it = though.
  CRONET_NET_ERROR_IMPORT_CERT_ALREADY_EXISTS = -704,

  // CA import failed due to some other error.
  CRONET_NET_ERROR_IMPORT_CA_CERT_FAILED = -705,

  // Server certificate import failed due to some internal error.
  CRONET_NET_ERROR_IMPORT_SERVER_CERT_FAILED = -706,

  // PKCS #12 import failed due to invalid MAC.
  CRONET_NET_ERROR_PKCS12_IMPORT_INVALID_MAC = -707,

  // PKCS #12 import failed due to invalid/corrupt file.
  CRONET_NET_ERROR_PKCS12_IMPORT_INVALID_FILE = -708,

  // PKCS #12 import failed due to unsupported features.
  CRONET_NET_ERROR_PKCS12_IMPORT_UNSUPPORTED = -709,

  // Key generation failed.
  CRONET_NET_ERROR_KEY_GENERATION_FAILED = -710,

  // Error -711 was removed (ORIGIN_BOUND_CERT_GENERATION_FAILED,

  // Failure to export private key.
  CRONET_NET_ERROR_PRIVATE_KEY_EXPORT_FAILED = -712,

  // Self-signed certificate generation failed.
  CRONET_NET_ERROR_SELF_SIGNED_CERT_GENERATION_FAILED = -713,

  // The certificate database changed in some way.
  CRONET_NET_ERROR_CERT_DATABASE_CHANGED = -714,

  // Error -715 was removed (CHANNEL_ID_IMPORT_FAILED,

  // DNS error codes.

  // DNS resolver received a malformed response.
  CRONET_NET_ERROR_DNS_MALFORMED_RESPONSE = -800,

  // DNS server requires TCP
  CRONET_NET_ERROR_DNS_SERVER_REQUIRES_TCP = -801,

  // DNS server failed.  This error is returned for all of the following
  // error conditions:
  // 1 - Format error - The name server was unable to interpret the query.
  // 2 - Server failure - The name server was unable to process this query
  //     due to a problem with the name server.
  // 4 - Not Implemented - The name server does not support the requested
  //     kind of query.
  // 5 - Refused - The name server refuses to perform the specified
  //     operation for policy reasons.
  CRONET_NET_ERROR_DNS_SERVER_FAILED = -802,

  // DNS transaction timed out.
  CRONET_NET_ERROR_DNS_TIMED_OUT = -803,

  // The entry was not found in cache or other local sources = for lookups where
  // only local sources were queried.
  CRONET_NET_ERROR_DNS_CACHE_MISS = -804,

  // Suffix search list rules prevent resolution of the given host name.
  CRONET_NET_ERROR_DNS_SEARCH_EMPTY = -805,

  // Failed to sort addresses according to RFC3484.
  CRONET_NET_ERROR_DNS_SORT_ERROR = -806,

  // Error -807 was removed (DNS_HTTP_FAILED,

  // Failed to resolve the hostname of a DNS-over-HTTPS server.
  CRONET_NET_ERROR_DNS_SECURE_RESOLVER_HOSTNAME_RESOLUTION_FAILED = -808,
};

const char* cronet_net_error_as_string(cronet_net_error_code net_error);
grpc_status_code cronet_net_error_to_grpc_error(
    cronet_net_error_code net_error);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CRONET_TRANSPORT_CRONET_STATUS_H
