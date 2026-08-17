/*
 * This file contains prototypes for experimental SSL functions.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __sslexp_h_
#define __sslexp_h_

#include "ssl.h"
#include "sslerr.h"

SEC_BEGIN_PROTOS

/* The functions in this header file are not guaranteed to remain available in
 * future NSS versions. Code that uses these functions needs to safeguard
 * against the function not being available. */

#define SSL_EXPERIMENTAL_API(name, arglist, args)                   \
    (SSL_GetExperimentalAPI(name)                                   \
         ? ((SECStatus(*) arglist)SSL_GetExperimentalAPI(name))args \
         : SECFailure)
#define SSL_DEPRECATED_EXPERIMENTAL_API \
    (PR_SetError(SSL_ERROR_UNSUPPORTED_EXPERIMENTAL_API, 0), SECFailure)

/*
 * SSL_GetExtensionSupport() returns whether NSS supports a particular TLS
 * extension.
 *
 * - ssl_ext_none indicates that NSS does not support the extension and
 *   extension hooks can be installed.
 *
 * - ssl_ext_native indicates that NSS supports the extension natively, but
 *   allows an application to override that support and install its own
 *   extension hooks.
 *
 * - ssl_ext_native_only indicates that NSS supports the extension natively
 *   and does not permit custom extension hooks to be installed.  These
 *   extensions are critical to the functioning of NSS.
 */
typedef enum {
    ssl_ext_none,
    ssl_ext_native,
    ssl_ext_native_only
} SSLExtensionSupport;

#define SSL_GetExtensionSupport(extension, support)        \
    SSL_EXPERIMENTAL_API("SSL_GetExtensionSupport",        \
                         (PRUint16 _extension,             \
                          SSLExtensionSupport * _support), \
                         (extension, support))

/*
 * Custom extension hooks.
 *
 * The SSL_InstallExtensionHooks() registers two callback functions for use
 * with the identified extension type.
 *
 * Installing extension hooks disables the checks in TLS 1.3 that ensure that
 * extensions are only added to the correct messages.  The application is
 * responsible for ensuring that extensions are only sent with the right message
 * or messages.
 *
 * Installing an extension handler does not disable checks for whether an
 * extension can be used in a message that is a response to an extension in
 * another message.  Extensions in ServerHello, EncryptedExtensions and the
 * server Certificate messages are rejected unless the client sends an extension
 * in the ClientHello.  Similarly, a client Certificate message cannot contain
 * extensions that don't appear in a CertificateRequest (in TLS 1.3).
 *
 * Setting both |writer| and |handler| to NULL removes any existing hooks for
 * that extension.
 *
 * == SSLExtensionWriter
 *
 * An SSLExtensionWriter function is responsible for constructing the contents
 * of an extension.  This function is called during the construction of all
 * handshake messages where an extension might be included.
 *
 * - The |fd| argument is the socket file descriptor.
 *
 * - The |message| argument is the TLS handshake message type.  The writer will
 *   be called for every handshake message that NSS sends.  Most extensions
 *   should only be sent in a subset of messages.  NSS doesn’t check that
 *   extension writers don’t violate protocol rules regarding which message an
 *   extension can be sent in.
 *
 * - The |data| argument is a pointer to a buffer that should be written to with
 *   any data for the extension.
 *
 * - The |len| argument is an outparam indicating how many bytes were written to
 *   |data|.  The value referenced by |len| is initialized to zero, so an
 *   extension that is empty does not need to write to this value.
 *
 * - The |maxLen| indicates the maximum number of bytes that can be written to
 *   |data|.
 *
 * - The |arg| argument is the value of the writerArg that was passed during
 *   installation.
 *
 * An SSLExtensionWriter function returns PR_TRUE if an extension should be
 * written, and PR_FALSE otherwise.
 *
 * If there is an error, return PR_FALSE; if the error is truly fatal, the
 * application can mark the connection as failed. However, recursively calling
 * functions that alter the file descriptor in the callback - such as PR_Close()
 * - should be avoided.
 *
 * Note: The ClientHello message can be sent twice in TLS 1.3.  An
 * SSLExtensionWriter will be called twice with the same arguments in that case;
 * NSS does not distinguish between a first and second ClientHello.  It is up to
 * the application to track this if it needs to act differently each time.  In
 * most cases the correct behaviour is to provide an identical extension on each
 * invocation.
 *
 * == SSLExtensionHandler
 *
 * An SSLExtensionHandler function consumes a handshake message.  This function
 * is called when an extension is present.
 *
 * - The |fd| argument is the socket file descriptor.
 *
 * - The |message| argument is the TLS handshake message type. This can be used
 *   to validate that the extension was included in the correct handshake
 *   message.
 *
 * - The |data| argument points to the contents of the extension.
 *
 * - The |len| argument contains the length of the extension.
 *
 * - The |alert| argument is an outparam that allows an application to choose
 *   which alert is sent in the case of a fatal error.
 *
 * - The |arg| argument is the value of the handlerArg that was passed during
 *   installation.
 *
 * An SSLExtensionHandler function returns SECSuccess when the extension is
 * process successfully.  It can return SECFailure to cause the handshake to
 * fail.  If the value of alert is written to, NSS will generate a fatal alert
 * using the provided alert code.  The value of |alert| is otherwise not used.
 */
typedef PRBool(PR_CALLBACK *SSLExtensionWriter)(
    PRFileDesc *fd, SSLHandshakeType message,
    PRUint8 *data, unsigned int *len, unsigned int maxLen, void *arg);

typedef SECStatus(PR_CALLBACK *SSLExtensionHandler)(
    PRFileDesc *fd, SSLHandshakeType message,
    const PRUint8 *data, unsigned int len,
    SSLAlertDescription *alert, void *arg);

#define SSL_InstallExtensionHooks(fd, extension, writer, writerArg,         \
                                  handler, handlerArg)                      \
    SSL_EXPERIMENTAL_API("SSL_InstallExtensionHooks",                       \
                         (PRFileDesc * _fd, PRUint16 _extension,            \
                          SSLExtensionWriter _writer, void *_writerArg,     \
                          SSLExtensionHandler _handler, void *_handlerArg), \
                         (fd, extension, writer, writerArg,                 \
                          handler, handlerArg))

/*
 * Create an anti-replay context for supporting 0-RTT in TLS 1.3 on servers.
 *
 * To use 0-RTT on a server, you must create an anti-replay context using
 * SSL_CreateAntiReplayContext and set that on the socket with
 * SSL_SetAntiReplayContext.  Failing to set a context on the server will result
 * in all 0-RTT being rejected.  Connections will complete, but early data will
 * be rejected.
 *
 * Anti-replay contexts are reference counted and are released with
 * SSL_ReleaseAntiReplayContext.
 *
 * NSS uses a Bloom filter to track the ClientHello messages that it receives
 * (specifically, it uses the PSK binder).  This function initializes a pair of
 * Bloom filters.  The two filters are alternated over time, with new
 * ClientHello messages recorded in the current filter and, if they are not
 * already present, being checked against the previous filter.  If the
 * ClientHello is found, then early data is rejected, but the handshake is
 * allowed to proceed.
 *
 * The false-positive probability of Bloom filters means that some valid
 * handshakes will be marked as potential replays.  Early data will be rejected
 * for a false positive.  To minimize this and to allow a trade-off of space
 * against accuracy, the size of the Bloom filter can be set by this function.
 *
 * The first tuning parameter to consider is |window|, which determines the
 * window over which ClientHello messages will be tracked.  This also causes
 * early data to be rejected if a ClientHello contains a ticket age parameter
 * that is outside of this window (see Section 8.3 of RFC 8446 for details).
 * Set |window| to account for any potential sources of clock error.  |window|
 * is the entire width of the window, which is symmetrical.  Therefore to allow
 * 5 seconds of clock error in both directions, set the value to 10 seconds
 * (i.e., 10 * PR_USEC_PER_SEC).
 *
 * After calling this function, early data will be rejected until |window|
 * elapses.  This prevents replay across crashes and restarts.  Only call this
 * function once to avoid inadvertently disabling 0-RTT (use PR_CallOnce() to
 * avoid this problem).
 *
 * The primary tuning parameter is |bits| which determines the amount of memory
 * allocated to each Bloom filter.  NSS will allocate two Bloom filters, each
 * |2^(bits - 3)| octets in size.  The value of |bits| is primarily driven by
 * the number of connections that are expected in any time window.  Note that
 * this needs to account for there being two filters both of which have
 * (presumably) independent false positive rates.  The following formulae can be
 * used to find a value of |bits| and |k| given a chosen false positive
 * probability |p| and the number of requests expected in a given window |n|:
 *
 *   bits = log2(n) + log2(-ln(1 - sqrt(1 - p))) + 1.0575327458897952
 *   k = -log2(p)
 *
 * ... where log2 and ln are base 2 and e logarithms respectively.  For a target
 * false positive rate of 1% and 1000 handshake attempts, this produces bits=14
 * and k=7.  This results in two Bloom filters that are 2kB each in size.  Note
 * that rounding |k| and |bits| up causes the false positive probability for
 * these values to be a much lower 0.123%.
 *
 * IMPORTANT: This anti-replay scheme has several weaknesses.  See the TLS 1.3
 * specification for the details of the generic problems with this technique.
 *
 * In addition to the generic anti-replay weaknesses, the state that the server
 * maintains is in local memory only.  Servers that operate in a cluster, even
 * those that use shared memory for tickets, will not share anti-replay state.
 * Early data can be replayed at least once with every server instance that will
 * accept tickets that are encrypted with the same key.
 */
typedef struct SSLAntiReplayContextStr SSLAntiReplayContext;
#define SSL_CreateAntiReplayContext(now, window, k, bits, ctx) \
    SSL_EXPERIMENTAL_API("SSL_CreateAntiReplayContext",        \
                         (PRTime _now, PRTime _window,         \
                          unsigned int _k, unsigned int _bits, \
                          SSLAntiReplayContext **_ctx),        \
                         (now, window, k, bits, ctx))

#define SSL_SetAntiReplayContext(fd, ctx)                                 \
    SSL_EXPERIMENTAL_API("SSL_SetAntiReplayContext",                      \
                         (PRFileDesc * _fd, SSLAntiReplayContext * _ctx), \
                         (fd, ctx))

#define SSL_ReleaseAntiReplayContext(ctx)                \
    SSL_EXPERIMENTAL_API("SSL_ReleaseAntiReplayContext", \
                         (SSLAntiReplayContext * _ctx),  \
                         (ctx))

/*
 * This function allows a server application to generate a session ticket that
 * will embed the provided token.
 *
 * This function will cause a NewSessionTicket message to be sent by a server.
 * This happens even if SSL_ENABLE_SESSION_TICKETS is disabled.  This allows a
 * server to suppress the usually automatic generation of a session ticket at
 * the completion of the handshake - which do not include any token - and to
 * control when session tickets are transmitted.
 *
 * This function will fail unless the socket has an active TLS 1.3 session.
 * Earlier versions of TLS do not support the spontaneous sending of the
 * NewSessionTicket message. It will also fail when external PSK
 * authentication has been negotiated.
 */
#define SSL_SendSessionTicket(fd, appToken, appTokenLen)              \
    SSL_EXPERIMENTAL_API("SSL_SendSessionTicket",                     \
                         (PRFileDesc * _fd, const PRUint8 *_appToken, \
                          unsigned int _appTokenLen),                 \
                         (fd, appToken, appTokenLen))

/*
 * A stateless retry handler gives an application some control over NSS handling
 * of ClientHello messages.
 *
 * SSL_HelloRetryRequestCallback() installs a callback that allows an
 * application to control how NSS sends HelloRetryRequest messages.  This
 * handler is only used on servers and will only be called if the server selects
 * TLS 1.3.  Support for older TLS versions could be added in other releases.
 *
 * The SSLHelloRetryRequestCallback is invoked during the processing of a
 * TLS 1.3 ClientHello message.  It takes the following arguments:
 *
 * - |firstHello| indicates if the NSS believes that this is an initial
 *   ClientHello.  An initial ClientHello will never include a cookie extension,
 *   though it may contain a session ticket.
 *
 * - |clientToken| includes a token previously provided by the application.  If
 *   |clientTokenLen| is 0, then |clientToken| may be NULL.
 *
 *   - If |firstHello| is PR_FALSE, the value that was provided in the
 *     |retryToken| outparam of previous invocations of this callback will be
 *     present here.
 *
 *   - If |firstHello| is PR_TRUE, and the handshake is resuming a session, then
 *     this will contain any value that was passed in the |token| parameter of
 *     SSL_SendNewSessionTicket() method (see below).  If this is not resuming a
 *     session, then the token will be empty (and this value could be NULL).
 *
 * - |clientTokenLen| is the length of |clientToken|.
 *
 * - |retryToken| is an item that callback can write to.  This provides NSS with
 *   a token.  This token is encrypted and integrity protected and embedded in
 *   the cookie extension of a HelloRetryRequest.  The value of this field is
 *   only used if the handler returns ssl_stateless_retry_check.  NSS allocates
 *   space for this value.
 *
 * - |retryTokenLen| is an outparam for the length of the token. If this value
 *   is not set, or set to 0, an empty token will be sent.
 *
 * - |retryTokenMax| is the size of the space allocated for retryToken. An
 *   application cannot write more than this many bytes to retryToken.
 *
 * - |arg| is the same value that was passed to
 *   SSL_InstallStatelessRetryHandler().
 *
 * The handler can validate any the value of |clientToken|, query the socket
 * status (using SSL_GetPreliminaryChannelInfo() for example) and decide how to
 * proceed:
 *
 * - Returning ssl_hello_retry_fail causes the handshake to fail.  This might be
 *   used if the token is invalid or the application wishes to abort the
 *   handshake.
 *
 * - Returning ssl_hello_retry_accept causes the handshake to proceed.
 *
 * - Returning ssl_hello_retry_request causes NSS to send a HelloRetryRequest
 *   message and request a second ClientHello.  NSS generates a cookie extension
 *   and embeds the value of |retryToken|.  The value of |retryToken| value may
 *   be left empty if the application does not require any additional context to
 *   validate a second ClientHello attempt.  This return code cannot be used to
 *   reject a second ClientHello (i.e., when firstHello is PR_FALSE); NSS will
 *   abort the handshake if this value is returned from a second call.
 *
 * - Returning ssl_hello_retry_reject_0rtt causes NSS to proceed normally, but
 *   to reject 0-RTT.  Use this if there is something in the token that
 *   indicates that 0-RTT might be unsafe.
 *
 * An application that chooses to perform a stateless retry can discard the
 * server socket.  All necessary state to continue the TLS handshake will be
 * included in the cookie extension.  This makes it possible to use a new socket
 * to handle the remainder of the handshake.  The existing socket can be safely
 * discarded.
 *
 * If the same socket is retained, the information in the cookie will be checked
 * for consistency against the existing state of the socket.  Any discrepancy
 * will result in the connection being closed.
 *
 * Tokens should be kept as small as possible.  NSS sets a limit on the size of
 * tokens, which it passes in |retryTokenMax|.  Depending on circumstances,
 * observing a smaller limit might be desirable or even necessary.  For
 * instance, having HelloRetryRequest and ClientHello fit in a single packet has
 * significant performance benefits.
 */
typedef enum {
    ssl_hello_retry_fail,
    ssl_hello_retry_accept,
    ssl_hello_retry_request,
    ssl_hello_retry_reject_0rtt
} SSLHelloRetryRequestAction;

typedef SSLHelloRetryRequestAction(PR_CALLBACK *SSLHelloRetryRequestCallback)(
    PRBool firstHello, const PRUint8 *clientToken, unsigned int clientTokenLen,
    PRUint8 *retryToken, unsigned int *retryTokenLen, unsigned int retryTokMax,
    void *arg);

#define SSL_HelloRetryRequestCallback(fd, cb, arg)                       \
    SSL_EXPERIMENTAL_API("SSL_HelloRetryRequestCallback",                \
                         (PRFileDesc * _fd,                              \
                          SSLHelloRetryRequestCallback _cb, void *_arg), \
                         (fd, cb, arg))

/* Update traffic keys (TLS 1.3 only).
 *
 * The |requestUpdate| flag determines whether to request an update from the
 * remote peer.
 */
#define SSL_KeyUpdate(fd, requestUpdate)                            \
    SSL_EXPERIMENTAL_API("SSL_KeyUpdate",                           \
                         (PRFileDesc * _fd, PRBool _requestUpdate), \
                         (fd, requestUpdate))

/* This function allows a server application to trigger
 * re-authentication (TLS 1.3 only) after handshake.
 *
 * This function will cause a CertificateRequest message to be sent by
 * a server.  This can be called once at a time, and is not allowed
 * until an answer is received.
 *
 * This function is not allowed for use with DTLS or when external
 * PSK authentication has been negotiated. SECFailure is returned
 * in both cases.
 *
 * The AuthCertificateCallback is called when the answer is received.
 * If the answer is accepted by the server, the value returned by
 * SSL_PeerCertificate() is replaced.  If you need to remember all the
 * certificates, you will need to call SSL_PeerCertificate() and save
 * what you get before calling this.
 *
 * If the AuthCertificateCallback returns SECFailure, the connection
 * is aborted.
 */
#define SSL_SendCertificateRequest(fd)                 \
    SSL_EXPERIMENTAL_API("SSL_SendCertificateRequest", \
                         (PRFileDesc * _fd),           \
                         (fd))

/*
 * Session cache API.
 */

/*
 * Information that can be retrieved about a resumption token.
 * See SSL_GetResumptionTokenInfo for details about how to use this API.
 * Note that peerCert points to a certificate in the NSS database and must be
 * copied by the application if it should be used after NSS shutdown or after
 * calling SSL_DestroyResumptionTokenInfo.
 */
typedef struct SSLResumptionTokenInfoStr {
    PRUint16 length;
    CERTCertificate *peerCert;
    PRUint8 *alpnSelection;
    PRUint32 alpnSelectionLen;
    PRUint32 maxEarlyDataSize;
    PRTime expirationTime; /* added in NSS 3.41 */
} SSLResumptionTokenInfo;

/*
 * Allows applications to retrieve information about a resumption token.
 * This does not require a TLS session.
 *
 * - The |tokenData| argument is a pointer to the resumption token as byte array
 *   of length |tokenLen|.
 * - The |token| argument is a pointer to a SSLResumptionTokenInfo struct of
 *   of |len|. The struct gets filled by this function.
 * See SSL_DestroyResumptionTokenInfo for information about how to manage the
 * |token| memory.
 */
#define SSL_GetResumptionTokenInfo(tokenData, tokenLen, token, len)          \
    SSL_EXPERIMENTAL_API("SSL_GetResumptionTokenInfo",                       \
                         (const PRUint8 *_tokenData, unsigned int _tokenLen, \
                          SSLResumptionTokenInfo *_token, PRUintn _len),     \
                         (tokenData, tokenLen, token, len))

/*
 * SSL_GetResumptionTokenInfo allocates memory in order to populate |tokenInfo|.
 * Any SSLResumptionTokenInfo struct filled with SSL_GetResumptionTokenInfo
 * has to be freed with SSL_DestroyResumptionTokenInfo.
 */
#define SSL_DestroyResumptionTokenInfo(tokenInfo) \
    SSL_EXPERIMENTAL_API(                         \
        "SSL_DestroyResumptionTokenInfo",         \
        (SSLResumptionTokenInfo * _tokenInfo),    \
        (tokenInfo))

/*
 * This is the function signature for function pointers used as resumption
 * token callback. The caller has to copy the memory at |resumptionToken| with
 * length |len| before returning.
 *
 * - The |fd| argument is the socket file descriptor.
 * - The |resumptionToken| is a pointer to the resumption token as byte array
 *   of length |len|.
 * - The |ctx| is a void pointer to the context set by the application in
 *   SSL_SetResumptionTokenCallback.
 */
typedef SECStatus(PR_CALLBACK *SSLResumptionTokenCallback)(
    PRFileDesc *fd, const PRUint8 *resumptionToken, unsigned int len,
    void *ctx);

/*
 * This allows setting a callback for external session caches to store
 * resumption tokens.
 *
 * - The |fd| argument is the socket file descriptor.
 * - The |cb| is a function pointer to an implementation of
 *   SSLResumptionTokenCallback.
 * - The |ctx| is a pointer to some application specific context, which is
 *   returned when |cb| is called.
 */
#define SSL_SetResumptionTokenCallback(fd, cb, ctx)                     \
    SSL_EXPERIMENTAL_API(                                               \
        "SSL_SetResumptionTokenCallback",                               \
        (PRFileDesc * _fd, SSLResumptionTokenCallback _cb, void *_ctx), \
        (fd, cb, ctx))

/*
 * This allows setting a resumption token for a session.
 * The function returns SECSuccess iff the resumption token can be used,
 * SECFailure in any other case. The caller should remove the |token| from its
 * cache when the function returns SECFailure.
 *
 * - The |fd| argument is the socket file descriptor.
 * - The |token| is a pointer to the resumption token as byte array
 *   of length |len|.
 */
#define SSL_SetResumptionToken(fd, token, len)                              \
    SSL_EXPERIMENTAL_API(                                                   \
        "SSL_SetResumptionToken",                                           \
        (PRFileDesc * _fd, const PRUint8 *_token, const unsigned int _len), \
        (fd, token, len))

/* TLS 1.3 allows a server to set a limit on the number of bytes of early data
 * that can be received. This allows that limit to be set. This function has no
 * effect on a client. */
#define SSL_SetMaxEarlyDataSize(fd, size)                    \
    SSL_EXPERIMENTAL_API("SSL_SetMaxEarlyDataSize",          \
                         (PRFileDesc * _fd, PRUint32 _size), \
                         (fd, size))

/* If |enabled|, a GREASE ECH extension will be sent in every ClientHello,
 * unless a valid and supported ECHConfig is configured to the socket
 * (in which case real ECH takes precedence). If |!enabled|, it is not sent.*/
#define SSL_EnableTls13GreaseEch(fd, enabled)        \
    SSL_EXPERIMENTAL_API("SSL_EnableTls13GreaseEch", \
                         (PRFileDesc * _fd, PRBool _enabled), (fd, enabled))

/* Called by the client after an initial ECH connection fails with
 * SSL_ERROR_ECH_RETRY_WITH_ECH. Returns compatible ECHConfigs, which
 * are configured via SetClientEchConfigs for an ECH retry attempt.
 * These configs MUST NOT be used for more than the single retry
 * attempt. Subsequent connections MUST use advertised ECHConfigs. */
#define SSL_GetEchRetryConfigs(fd, out)            \
    SSL_EXPERIMENTAL_API("SSL_GetEchRetryConfigs", \
                         (PRFileDesc * _fd,        \
                          SECItem * _out),         \
                         (fd, out))

/* Called to remove all ECHConfigs from a socket (fd). */
#define SSL_RemoveEchConfigs(fd)                 \
    SSL_EXPERIMENTAL_API("SSL_RemoveEchConfigs", \
                         (PRFileDesc * _fd),     \
                         (fd))

/* Set the ECHConfig and key pair on a socket (server side)
 *
 * fd -- the socket
 * pubKey -- the server's SECKEYPublicKey for HPKE/ECH.
 * privateKey -- the server's SECKEYPrivateKey for HPKE/ECH.
 * record/recordLen -- the encoded DNS record (not base64)
 */
#define SSL_SetServerEchConfigs(fd, pubKey,                                 \
                                privKey, record, recordLen)                 \
    SSL_EXPERIMENTAL_API("SSL_SetServerEchConfigs",                         \
                         (PRFileDesc * _fd,                                 \
                          const SECKEYPublicKey *_pubKey,                   \
                          const SECKEYPrivateKey *_privKey,                 \
                          const PRUint8 *_record, unsigned int _recordLen), \
                         (fd, pubKey, privKey,                              \
                          record, recordLen))

/* Set ECHConfig(s) on a client. The first supported ECHConfig will be used.
 *
 * fd -- the socket
 * echConfigs/echConfigsLen -- the ECHConfigs structure (not base64)
 */
#define SSL_SetClientEchConfigs(fd, echConfigs, echConfigsLen) \
    SSL_EXPERIMENTAL_API("SSL_SetClientEchConfigs",            \
                         (PRFileDesc * _fd,                    \
                          const PRUint8 *_echConfigs,          \
                          unsigned int _echConfigsLen),        \
                         (fd, echConfigs, echConfigsLen))

/*
 * Generate an encoded ECHConfig structure (presumably server side).
 *
 * publicName -- the public_name value to be placed in SNI.
 * hpkeSuites -- the HPKE cipher suites that can be used
 * hpkeSuitesCount -- the number of suites in hpkeSuites
 * kemId -- the HKPE KEM ID value
 * group -- the named group this key corresponds to
 * pubKey -- the public key for the key pair
 * pad -- the maximum length to pad to
 * out/outlen/maxlen -- where to output the data
 */
#define SSL_EncodeEchConfig(publicName, hpkeSuites, hpkeSuitesCount, \
                            kemId, pubKey, maxNameLen, out, outlen,  \
                            maxlen)                                  \
    SSL_EXPERIMENTAL_API("SSL_EncodeEchConfig",                      \
                         (const char *_publicName,                   \
                          const PRUint32 *_hpkeSuites,               \
                          unsigned int _hpkeSuitesCount,             \
                          HpkeKemId _kemId,                          \
                          const SECKEYPublicKey *_pubKey,            \
                          PRUint16 _maxNameLen,                      \
                          PRUint8 *_out, unsigned int *_outlen,      \
                          unsigned int _maxlen),                     \
                         (publicName, hpkeSuites, hpkeSuitesCount,   \
                          kemId, pubKey, maxNameLen, out, outlen,    \
                          maxlen))

/* SSL_SetSecretCallback installs a callback that TLS calls when it installs new
 * traffic secrets.
 *
 * SSLSecretCallback is called with the current epoch and the corresponding
 * secret; this matches the epoch used in DTLS 1.3, even if the socket is
 * operating in stream mode:
 *
 * - client_early_traffic_secret corresponds to epoch 1
 * - {client|server}_handshake_traffic_secret is epoch 2
 * - {client|server}_application_traffic_secret_{N} is epoch 3+N
 *
 * The callback is invoked separately for read secrets (client secrets on the
 * server; server secrets on the client), and write secrets.
 *
 * This callback is only called if (D)TLS 1.3 is negotiated.
 */
typedef void(PR_CALLBACK *SSLSecretCallback)(
    PRFileDesc *fd, PRUint16 epoch, SSLSecretDirection dir, PK11SymKey *secret,
    void *arg);

#define SSL_SecretCallback(fd, cb, arg)                                         \
    SSL_EXPERIMENTAL_API("SSL_SecretCallback",                                  \
                         (PRFileDesc * _fd, SSLSecretCallback _cb, void *_arg), \
                         (fd, cb, arg))

/* SSL_RecordLayerWriteCallback() is used to replace the TLS record layer.  This
 * function installs a callback that TLS calls when it would otherwise encrypt
 * and write a record to the underlying NSPR IO layer.  The application is
 * responsible for ensuring that these records are encrypted and written.
 *
 * Calling this API also disables reads from the underlying NSPR layer.  The
 * application is expected to push data when it is available using
 * SSL_RecordLayerData().
 *
 * When data would be written, the provided SSLRecordWriteCallback with the
 * epoch, TLS content type, and the data. The data provided to the callback is
 * not split into record-sized writes.  If the callback returns SECFailure, the
 * write will be considered to have failed; in particular, PR_WOULD_BLOCK_ERROR
 * is not handled specially.
 *
 * If TLS 1.3 is in use, the epoch indicates the expected level of protection
 * that the record would receive, this matches that used in DTLS 1.3:
 *
 * - epoch 0 corresponds to no record protection
 * - epoch 1 corresponds to 0-RTT
 * - epoch 2 corresponds to TLS handshake
 * - epoch 3 and higher are application data
 *
 * Prior versions of TLS use epoch 1 and higher for application data.
 *
 * This API is not supported for DTLS.
 */
typedef SECStatus(PR_CALLBACK *SSLRecordWriteCallback)(
    PRFileDesc *fd, PRUint16 epoch, SSLContentType contentType,
    const PRUint8 *data, unsigned int len, void *arg);

#define SSL_RecordLayerWriteCallback(fd, writeCb, arg)                   \
    SSL_EXPERIMENTAL_API("SSL_RecordLayerWriteCallback",                 \
                         (PRFileDesc * _fd, SSLRecordWriteCallback _wCb, \
                          void *_arg),                                   \
                         (fd, writeCb, arg))

/* SSL_RecordLayerData() is used to provide new data to TLS.  The application
 * indicates the epoch (see the description of SSL_RecordLayerWriteCallback()),
 * content type, and the data that was received.  The application is responsible
 * for removing any encryption or other protection before passing data to this
 * function.
 *
 * This returns SECSuccess if the data was successfully processed.  If this
 * function is used to drive the handshake and the caller needs to know when the
 * handshake is complete, a call to SSL_ForceHandshake will return SECSuccess
 * when the handshake is complete.
 *
 * This API is not supported for DTLS sockets.
 */
#define SSL_RecordLayerData(fd, epoch, ct, data, len)               \
    SSL_EXPERIMENTAL_API("SSL_RecordLayerData",                     \
                         (PRFileDesc * _fd, PRUint16 _epoch,        \
                          SSLContentType _contentType,              \
                          const PRUint8 *_data, unsigned int _len), \
                         (fd, epoch, ct, data, len))

/*
 * SSL_GetCurrentEpoch() returns the read and write epochs that the socket is
 * currently using.  NULL values for readEpoch or writeEpoch are ignored.
 *
 * See SSL_RecordLayerWriteCallback() for details on epochs.
 */
#define SSL_GetCurrentEpoch(fd, readEpoch, writeEpoch)             \
    SSL_EXPERIMENTAL_API("SSL_GetCurrentEpoch",                    \
                         (PRFileDesc * _fd, PRUint16 * _readEpoch, \
                          PRUint16 * _writeEpoch),                 \
                         (fd, readEpoch, writeEpoch))

/*
 * The following AEAD functions expose an AEAD primitive that uses a ciphersuite
 * to set parameters.  The ciphersuite determines the Hash function used by
 * HKDF, the AEAD function, and the size of key and IV.  This is only supported
 * for TLS 1.3.
 *
 * The key and IV are generated using the TLS KDF with a custom label.  That is
 * HKDF-Expand-Label(secret, labelPrefix + " key" or " iv", "", L).
 *
 * The encrypt and decrypt functions use a nonce construction identical to that
 * used in TLS.  The lower bits of the IV are XORed with the 64-bit counter to
 * produce the nonce.  Otherwise, this is an AEAD interface similar to that
 * described in RFC 5116.
 *
 * Note: SSL_MakeAead internally calls SSL_MakeVariantAead with a variant of
 * "stream", behaving as noted above. If "datagram" variant is passed instead,
 * the Label prefix used in HKDF-Expand is "dtls13" instead of "tls13 ". See
 * 7.1 of RFC 8446 and draft-ietf-tls-dtls13-34. */
typedef struct SSLAeadContextStr SSLAeadContext;

#define SSL_MakeAead(version, cipherSuite, secret,                  \
                     labelPrefix, labelPrefixLen, ctx)              \
    SSL_EXPERIMENTAL_API("SSL_MakeAead",                            \
                         (PRUint16 _version, PRUint16 _cipherSuite, \
                          PK11SymKey * _secret,                     \
                          const char *_labelPrefix,                 \
                          unsigned int _labelPrefixLen,             \
                          SSLAeadContext **_ctx),                   \
                         (version, cipherSuite, secret,             \
                          labelPrefix, labelPrefixLen, ctx))

#define SSL_MakeVariantAead(version, cipherSuite, variant, secret,  \
                            labelPrefix, labelPrefixLen, ctx)       \
    SSL_EXPERIMENTAL_API("SSL_MakeVariantAead",                     \
                         (PRUint16 _version, PRUint16 _cipherSuite, \
                          SSLProtocolVariant _variant,              \
                          PK11SymKey * _secret,                     \
                          const char *_labelPrefix,                 \
                          unsigned int _labelPrefixLen,             \
                          SSLAeadContext **_ctx),                   \
                         (version, cipherSuite, variant, secret,    \
                          labelPrefix, labelPrefixLen, ctx))

#define SSL_AeadEncrypt(ctx, counter, aad, aadLen, in, inLen,            \
                        output, outputLen, maxOutputLen)                 \
    SSL_EXPERIMENTAL_API("SSL_AeadEncrypt",                              \
                         (const SSLAeadContext *_ctx, PRUint64 _counter, \
                          const PRUint8 *_aad, unsigned int _aadLen,     \
                          const PRUint8 *_in, unsigned int _inLen,       \
                          PRUint8 *_out, unsigned int *_outLen,          \
                          unsigned int _maxOut),                         \
                         (ctx, counter, aad, aadLen, in, inLen,          \
                          output, outputLen, maxOutputLen))

#define SSL_AeadDecrypt(ctx, counter, aad, aadLen, in, inLen,            \
                        output, outputLen, maxOutputLen)                 \
    SSL_EXPERIMENTAL_API("SSL_AeadDecrypt",                              \
                         (const SSLAeadContext *_ctx, PRUint64 _counter, \
                          const PRUint8 *_aad, unsigned int _aadLen,     \
                          const PRUint8 *_in, unsigned int _inLen,       \
                          PRUint8 *_output, unsigned int *_outLen,       \
                          unsigned int _maxOut),                         \
                         (ctx, counter, aad, aadLen, in, inLen,          \
                          output, outputLen, maxOutputLen))

#define SSL_DestroyAead(ctx)                      \
    SSL_EXPERIMENTAL_API("SSL_DestroyAead",       \
                         (SSLAeadContext * _ctx), \
                         (ctx))

/* SSL_HkdfExtract and SSL_HkdfExpandLabel implement the functions from TLS,
 * using the version and ciphersuite to set parameters. This allows callers to
 * use these TLS functions as a KDF. This is only supported for TLS 1.3.
 *
 * SSL_HkdfExtract produces a key with a mechanism that is suitable for input to
 * SSL_HkdfExpandLabel (and SSL_HkdfExpandLabelWithMech). */
#define SSL_HkdfExtract(version, cipherSuite, salt, ikm, keyp)      \
    SSL_EXPERIMENTAL_API("SSL_HkdfExtract",                         \
                         (PRUint16 _version, PRUint16 _cipherSuite, \
                          PK11SymKey * _salt, PK11SymKey * _ikm,    \
                          PK11SymKey * *_keyp),                     \
                         (version, cipherSuite, salt, ikm, keyp))

/* SSL_HkdfExpandLabel and SSL_HkdfVariantExpandLabel produce a key with a
 * mechanism that is suitable for input to SSL_HkdfExpandLabel or SSL_MakeAead.
 *
 * Note: SSL_HkdfVariantExpandLabel internally calls SSL_HkdfExpandLabel with
 * a default "stream" variant. If "datagram" variant is passed instead, the
 * Label prefix used in HKDF-Expand is "dtls13" instead of "tls13 ". See 7.1 of
 * RFC 8446 and draft-ietf-tls-dtls13-34. */
#define SSL_HkdfExpandLabel(version, cipherSuite, prk,                     \
                            hsHash, hsHashLen, label, labelLen, keyp)      \
    SSL_EXPERIMENTAL_API("SSL_HkdfExpandLabel",                            \
                         (PRUint16 _version, PRUint16 _cipherSuite,        \
                          PK11SymKey * _prk,                               \
                          const PRUint8 *_hsHash, unsigned int _hsHashLen, \
                          const char *_label, unsigned int _labelLen,      \
                          PK11SymKey **_keyp),                             \
                         (version, cipherSuite, prk,                       \
                          hsHash, hsHashLen, label, labelLen, keyp))

#define SSL_HkdfVariantExpandLabel(version, cipherSuite, prk,                   \
                                   hsHash, hsHashLen, label, labelLen, variant, \
                                   keyp)                                        \
    SSL_EXPERIMENTAL_API("SSL_HkdfVariantExpandLabel",                          \
                         (PRUint16 _version, PRUint16 _cipherSuite,             \
                          PK11SymKey * _prk,                                    \
                          const PRUint8 *_hsHash, unsigned int _hsHashLen,      \
                          const char *_label, unsigned int _labelLen,           \
                          SSLProtocolVariant _variant,                          \
                          PK11SymKey **_keyp),                                  \
                         (version, cipherSuite, prk,                            \
                          hsHash, hsHashLen, label, labelLen, variant,          \
                          keyp))

/* SSL_HkdfExpandLabelWithMech and SSL_HkdfVariantExpandLabelWithMech use the KDF
 * from the selected TLS version and cipher suite, as with the other calls, but
 * the provided mechanism and key size. This allows the key to be used more widely.
 *
 * Note: SSL_HkdfExpandLabelWithMech internally calls SSL_HkdfVariantExpandLabelWithMech
 * with a default "stream" variant. If "datagram" variant is passed instead, the
 * Label prefix used in HKDF-Expand is "dtls13" instead of "tls13 ". See 7.1 of
 * RFC 8446 and draft-ietf-tls-dtls13-34. */
#define SSL_HkdfExpandLabelWithMech(version, cipherSuite, prk,             \
                                    hsHash, hsHashLen, label, labelLen,    \
                                    mech, keySize, keyp)                   \
    SSL_EXPERIMENTAL_API("SSL_HkdfExpandLabelWithMech",                    \
                         (PRUint16 _version, PRUint16 _cipherSuite,        \
                          PK11SymKey * _prk,                               \
                          const PRUint8 *_hsHash, unsigned int _hsHashLen, \
                          const char *_label, unsigned int _labelLen,      \
                          CK_MECHANISM_TYPE _mech, unsigned int _keySize,  \
                          PK11SymKey **_keyp),                             \
                         (version, cipherSuite, prk,                       \
                          hsHash, hsHashLen, label, labelLen,              \
                          mech, keySize, keyp))

#define SSL_HkdfVariantExpandLabelWithMech(version, cipherSuite, prk,          \
                                           hsHash, hsHashLen, label, labelLen, \
                                           mech, keySize, variant, keyp)       \
    SSL_EXPERIMENTAL_API("SSL_HkdfVariantExpandLabelWithMech",                 \
                         (PRUint16 _version, PRUint16 _cipherSuite,            \
                          PK11SymKey * _prk,                                   \
                          const PRUint8 *_hsHash, unsigned int _hsHashLen,     \
                          const char *_label, unsigned int _labelLen,          \
                          CK_MECHANISM_TYPE _mech, unsigned int _keySize,      \
                          SSLProtocolVariant _variant,                         \
                          PK11SymKey **_keyp),                                 \
                         (version, cipherSuite, prk,                           \
                          hsHash, hsHashLen, label, labelLen,                  \
                          mech, keySize, variant, keyp))

/* SSL_SetTimeFunc overrides the default time function (PR_Now()) and provides
 * an alternative source of time for the socket. This is used in testing, and in
 * applications that need better control over how the clock is accessed. Set the
 * function to NULL to use PR_Now().*/
typedef PRTime(PR_CALLBACK *SSLTimeFunc)(void *arg);

#define SSL_SetTimeFunc(fd, f, arg)                                      \
    SSL_EXPERIMENTAL_API("SSL_SetTimeFunc",                              \
                         (PRFileDesc * _fd, SSLTimeFunc _f, void *_arg), \
                         (fd, f, arg))

/* Create a delegated credential (DC) for the draft-ietf-tls-subcerts extension
 * using the given certificate |cert| and its signing key |certPriv| and write
 * the serialized DC to |out|. The
 * parameters are:
 *  - the DC public key |dcPub|;
 *  - the DC signature scheme |dcCertVerifyAlg|, used to verify the handshake.
 *  - the DC time-to-live |dcValidFor|, the number of seconds from now for which
 *    the DC should be valid; and
 *  - the current time |now|.
 *
 *  The signing algorithm used to verify the DC signature is deduced from
 *  |cert|.
 *
 *  It's the caller's responsibility to ensure the input parameters are all
 *  valid. This procedure is meant primarily for testing; for this purpose it is
 *  useful to do no validation.
 */
#define SSL_DelegateCredential(cert, certPriv, dcPub, dcCertVerifyAlg,        \
                               dcValidFor, now, out)                          \
    SSL_EXPERIMENTAL_API("SSL_DelegateCredential",                            \
                         (const CERTCertificate *_cert,                       \
                          const SECKEYPrivateKey *_certPriv,                  \
                          const SECKEYPublicKey *_dcPub,                      \
                          SSLSignatureScheme _dcCertVerifyAlg,                \
                          PRUint32 _dcValidFor,                               \
                          PRTime _now,                                        \
                          SECItem *_out),                                     \
                         (cert, certPriv, dcPub, dcCertVerifyAlg, dcValidFor, \
                          now, out))

/* New functions created to permit get/set the CipherSuites Order for the
 * handshake (Client Hello).
 *
 * The *Get function puts the current set of active (enabled and policy set as
 * PR_TRUE) cipher suites in the cipherOrder outparam. Cipher suites that
 * aren't active aren't included. The paramenters are:
 *   - PRFileDesc *fd = FileDescriptor to get information.
 *   - PRUint16 *cipherOrder = The memory allocated for cipherOrder needs to be
 *     SSL_GetNumImplementedCiphers() * sizeof(PRUint16) or more.
 *   - PRUint16 numCiphers = The number of active ciphersuites listed in
 *     *cipherOrder is written here.
 *
 * The *Set function permits reorder the CipherSuites list for the Handshake
 * (Client Hello). The default ordering defined in ssl3con.c is enough in
 * almost all cases. But, if the client needs some hardening or performance
 * adjusts related to CipherSuites, this can be done with this function.
 * The caller has to be aware about the risk of call this function while a
 * handshake are being processed in this fd/socket. For example, if you disable
 * a cipher after the handshake and this cipher was choosen for that
 * connection, something bad will happen.
 * The parameters are:
 *   - PRFileDesc *fd = FileDescriptor to change.
 *   - const PRUint16 *cipherOrder = Must receive all ciphers to be ordered, in
 *     the desired order. They will be set in the begin of the list. Only
 *     suites listed by SSL_ImplementedCiphers() can be included.
 *   - PRUint16 numCiphers = Must receive the number of items in *cipherOrder.
 * */
#define SSL_CipherSuiteOrderGet(fd, cipherOrder, numCiphers)         \
    SSL_EXPERIMENTAL_API("SSL_CipherSuiteOrderGet",                  \
                         (PRFileDesc * _fd, PRUint16 * _cipherOrder, \
                          unsigned int *_numCiphers),                \
                         (fd, cipherOrder, numCiphers))

#define SSL_CipherSuiteOrderSet(fd, cipherOrder, numCiphers)              \
    SSL_EXPERIMENTAL_API("SSL_CipherSuiteOrderSet",                       \
                         (PRFileDesc * _fd, const PRUint16 *_cipherOrder, \
                          PRUint16 _numCiphers),                          \
                         (fd, cipherOrder, numCiphers))

/*
 * The following functions expose a masking primitive that uses ciphersuite and
 * version information to set paramaters for the masking key and mask generation
 * logic. This is only supported for TLS 1.3.
 *
 * The key and IV are generated using the TLS KDF with a custom label.  That is
 * HKDF-Expand-Label(secret, label, "", L), where |label| is an input to
 * SSL_CreateMaskingContext.
 *
 * The mask generation logic in SSL_CreateMask is determined by the underlying
 * symmetric cipher:
 *  - For AES-ECB, mask = AES-ECB(mask_key, sample). |len| must be <= 16 as
 *    the output is limited to a single block.
 *  - For CHACHA20, mask = ChaCha20(mask_key, sample[0..3], sample[4..15], {0}.len)
 *    That is, the low 4 bytes of |sample| used as the counter, the remaining 12 bytes
 *    the nonce. We encrypt |len| bytes of zeros, returning the raw key stream.
 *
 *  The caller must pre-allocate at least |len| bytes for output. If the underlying
 *  cipher cannot produce the requested amount of data, SECFailure is returned.
 */

typedef struct SSLMaskingContextStr {
    CK_MECHANISM_TYPE mech;
    PRUint16 version;
    PRUint16 cipherSuite;
    PK11SymKey *secret;
} SSLMaskingContext;

#define SSL_CreateMaskingContext(version, cipherSuite, secret,      \
                                 label, labelLen, ctx)              \
    SSL_EXPERIMENTAL_API("SSL_CreateMaskingContext",                \
                         (PRUint16 _version, PRUint16 _cipherSuite, \
                          PK11SymKey * _secret,                     \
                          const char *_label,                       \
                          unsigned int _labelLen,                   \
                          SSLMaskingContext **_ctx),                \
                         (version, cipherSuite, secret, label, labelLen, ctx))

#define SSL_CreateVariantMaskingContext(version, cipherSuite, variant, \
                                        secret, label, labelLen, ctx)  \
    SSL_EXPERIMENTAL_API("SSL_CreateVariantMaskingContext",            \
                         (PRUint16 _version, PRUint16 _cipherSuite,    \
                          SSLProtocolVariant _variant,                 \
                          PK11SymKey * _secret,                        \
                          const char *_label,                          \
                          unsigned int _labelLen,                      \
                          SSLMaskingContext **_ctx),                   \
                         (version, cipherSuite, variant, secret,       \
                          label, labelLen, ctx))

#define SSL_DestroyMaskingContext(ctx)                \
    SSL_EXPERIMENTAL_API("SSL_DestroyMaskingContext", \
                         (SSLMaskingContext * _ctx),  \
                         (ctx))

#define SSL_CreateMask(ctx, sample, sampleLen, mask, maskLen)               \
    SSL_EXPERIMENTAL_API("SSL_CreateMask",                                  \
                         (SSLMaskingContext * _ctx, const PRUint8 *_sample, \
                          unsigned int _sampleLen, PRUint8 *_mask,          \
                          unsigned int _maskLen),                           \
                         (ctx, sample, sampleLen, mask, maskLen))

#define SSL_SetDtls13VersionWorkaround(fd, enabled)        \
    SSL_EXPERIMENTAL_API("SSL_SetDtls13VersionWorkaround", \
                         (PRFileDesc * _fd, PRBool _enabled), (fd, enabled))

/* SSL_AddExternalPsk() and SSL_AddExternalPsk0Rtt() can be used to
 * set an external PSK on a socket. If successful, this PSK will
 * be used in all subsequent connection attempts for this socket.
 * This has no effect if the maximum TLS version is < 1.3.
 *
 * This API currently only accepts a single PSK, so multiple calls to
 * either function will fail. An EPSK can be replaced by calling
 * SSL_RemoveExternalPsk followed by SSL_AddExternalPsk.
 * For both functions, the label is expected to be a unique identifier
 * for the external PSK. Should en external PSK have the same label
 * as a configured resumption PSK identity, the external PSK will
 * take precedence.
 *
 * If you want to enable early data, you need to also provide a
 * cipher suite for 0-RTT and a limit for the early data using
 * SSL_AddExternalPsk0Rtt(). If you want to explicitly disallow
 * certificate authentication, use SSL_AuthCertificateHook to set
 * a callback that rejects all certificate chains.
 */
#define SSL_AddExternalPsk(fd, psk, identity, identityLen, hash)               \
    SSL_EXPERIMENTAL_API("SSL_AddExternalPsk",                                 \
                         (PRFileDesc * _fd, PK11SymKey * _psk,                 \
                          const PRUint8 *_identity, unsigned int _identityLen, \
                          SSLHashType _hash),                                  \
                         (fd, psk, identity, identityLen, hash))

#define SSL_AddExternalPsk0Rtt(fd, psk, identity, identityLen, hash,           \
                               zeroRttSuite, maxEarlyData)                     \
    SSL_EXPERIMENTAL_API("SSL_AddExternalPsk0Rtt",                             \
                         (PRFileDesc * _fd, PK11SymKey * _psk,                 \
                          const PRUint8 *_identity, unsigned int _identityLen, \
                          SSLHashType _hash, PRUint16 _zeroRttSuite,           \
                          PRUint32 _maxEarlyData),                             \
                         (fd, psk, identity, identityLen, hash,                \
                          zeroRttSuite, maxEarlyData))

/* SSLExp_RemoveExternalPsk() removes an external PSK from socket
 * configuration. Returns SECSuccess if the PSK was removed
 * successfully, and SECFailure otherwise. */
#define SSL_RemoveExternalPsk(fd, identity, identityLen)              \
    SSL_EXPERIMENTAL_API("SSL_RemoveExternalPsk",                     \
                         (PRFileDesc * _fd, const PRUint8 *_identity, \
                          unsigned int _identityLen),                 \
                         (fd, identity, identityLen))

/* Deprecated experimental APIs */
#define SSL_UseAltServerHelloType(fd, enable) SSL_DEPRECATED_EXPERIMENTAL_API
#define SSL_SetupAntiReplay(a, b, c) SSL_DEPRECATED_EXPERIMENTAL_API
#define SSL_InitAntiReplay(a, b, c) SSL_DEPRECATED_EXPERIMENTAL_API
#define SSL_EnableESNI(a, b, c, d) SSL_DEPRECATED_EXPERIMENTAL_API
#define SSL_EncodeESNIKeys(a, b, c, d, e, f, g, h, i, j) SSL_DEPRECATED_EXPERIMENTAL_API
#define SSL_SetESNIKeyPair(a, b, c, d) SSL_DEPRECATED_EXPERIMENTAL_API

SEC_END_PROTOS

#endif /* __sslexp_h_ */
