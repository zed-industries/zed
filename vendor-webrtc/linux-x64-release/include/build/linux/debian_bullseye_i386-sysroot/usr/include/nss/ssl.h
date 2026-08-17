/*
 * This file contains prototypes for the public SSL functions.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __ssl_h_
#define __ssl_h_

#include "prtypes.h"
#include "prerror.h"
#include "prio.h"
#include "seccomon.h"
#include "cert.h"
#include "keythi.h"

#include "sslt.h" /* public ssl data types */

#if defined(_WIN32) && !defined(IN_LIBSSL) && !defined(NSS_USE_STATIC_LIBS)
#define SSL_IMPORT extern __declspec(dllimport)
#else
#define SSL_IMPORT extern
#endif

SEC_BEGIN_PROTOS

/* constant table enumerating all implemented cipher suites. */
SSL_IMPORT const PRUint16 SSL_ImplementedCiphers[];

/* the same as the above, but is a function */
SSL_IMPORT const PRUint16 *SSL_GetImplementedCiphers(void);

/* number of entries in the above table. */
SSL_IMPORT const PRUint16 SSL_NumImplementedCiphers;

/* the same as the above, but is a function */
SSL_IMPORT PRUint16 SSL_GetNumImplementedCiphers(void);

/* Macro to tell which ciphers in table are SSL2 vs SSL3/TLS. */
#define SSL_IS_SSL2_CIPHER(which) (((which)&0xfff0) == 0xff00)

/*
** Imports fd into SSL, returning a new socket.  Copies SSL configuration
** from model.
*/
SSL_IMPORT PRFileDesc *SSL_ImportFD(PRFileDesc *model, PRFileDesc *fd);

/*
** Imports fd into DTLS, returning a new socket.  Copies DTLS configuration
** from model.
*/
SSL_IMPORT PRFileDesc *DTLS_ImportFD(PRFileDesc *model, PRFileDesc *fd);

/*
** Enable/disable an ssl mode
**
**  SSL_SECURITY:
**    enable/disable use of SSL security protocol before connect
**
**  SSL_SOCKS:
**    enable/disable use of socks before connect
**    (No longer supported).
**
**  SSL_REQUEST_CERTIFICATE:
**    require a certificate during secure connect
*/
/* options */
#define SSL_SECURITY 1            /* (on by default) */
#define SSL_SOCKS 2               /* (off by default) */
#define SSL_REQUEST_CERTIFICATE 3 /* (off by default) */
#define SSL_HANDSHAKE_AS_CLIENT 5 /* force accept to hs as client */
                                  /* (off by default) */
#define SSL_HANDSHAKE_AS_SERVER 6 /* force connect to hs as server */
                                  /* (off by default) */

/* OBSOLETE: SSL v2 is obsolete and may be removed soon. */
#define SSL_ENABLE_SSL2 7 /* enable ssl v2 (off by default) */

/* OBSOLETE: See "SSL Version Range API" below for the replacement and a
** description of the non-obvious semantics of using SSL_ENABLE_SSL3.
*/
#define SSL_ENABLE_SSL3 8 /* enable ssl v3 (on by default) */

#define SSL_NO_CACHE 9             /* don't use the session cache */
                                   /* (off by default) */
#define SSL_REQUIRE_CERTIFICATE 10 /* (SSL_REQUIRE_FIRST_HANDSHAKE */
                                   /* by default) */
#define SSL_ENABLE_FDX 11          /* permit simultaneous read/write */
                                   /* (off by default) */

/* OBSOLETE: SSL v2 compatible hellos are not accepted by some TLS servers
** and cannot negotiate extensions. SSL v2 is obsolete. This option may be
** removed soon.
*/
#define SSL_V2_COMPATIBLE_HELLO 12 /* send v3 client hello in v2 fmt */
                                   /* (off by default) */

/* OBSOLETE: See "SSL Version Range API" below for the replacement and a
** description of the non-obvious semantics of using SSL_ENABLE_TLS.
*/
#define SSL_ENABLE_TLS 13 /* enable TLS (on by default) */

#define SSL_ROLLBACK_DETECTION 14       /* for compatibility, default: on */
#define SSL_NO_STEP_DOWN 15             /* (unsupported, deprecated, off) */
#define SSL_BYPASS_PKCS11 16            /* (unsupported, deprecated, off) */
#define SSL_NO_LOCKS 17                 /* Don't use locks for protection */
#define SSL_ENABLE_SESSION_TICKETS 18   /* Enable TLS SessionTicket       */
                                        /* extension (off by default)     */
#define SSL_ENABLE_DEFLATE 19           /* (unsupported, deprecated, off) */
#define SSL_ENABLE_RENEGOTIATION 20     /* Values below (default: never)  */
#define SSL_REQUIRE_SAFE_NEGOTIATION 21 /* Peer must send Signaling       */
                                        /* Cipher Suite Value (SCSV) or   */
                                        /* Renegotiation  Info (RI)       */
                                        /* extension in ALL handshakes.   */
                                        /* default: off                   */
#define SSL_ENABLE_FALSE_START 22       /* Enable SSL false start (off by */
                                        /* default, applies only to       */
                                        /* clients). False start is a     */
/* mode where an SSL client will start sending application data before
 * verifying the server's Finished message. This means that we could end up
 * sending data to an imposter. However, the data will be encrypted and
 * only the true server can derive the session key. Thus, so long as the
 * cipher isn't broken this is safe. The advantage of false start is that
 * it saves a round trip for client-speaks-first protocols when performing a
 * full handshake.
 *
 * In addition to enabling this option, the application must register a
 * callback using the SSL_SetCanFalseStartCallback function.
 */

/* For SSL 3.0 and TLS 1.0, by default we prevent chosen plaintext attacks
 * on SSL CBC mode cipher suites (see RFC 4346 Section F.3) by splitting
 * non-empty application_data records into two records; the first record has
 * only the first byte of plaintext, and the second has the rest.
 *
 * This only prevents the attack in the sending direction; the connection may
 * still be vulnerable to such attacks if the peer does not implement a similar
 * countermeasure.
 *
 * This protection mechanism is on by default; the default can be overridden by
 * setting NSS_SSL_CBC_RANDOM_IV=0 in the environment prior to execution,
 * and/or by the application setting the option SSL_CBC_RANDOM_IV to PR_FALSE.
 *
 * The per-record IV in TLS 1.1 and later adds one block of overhead per
 * record, whereas this hack will add at least two blocks of overhead per
 * record, so TLS 1.1+ will always be more efficient.
 *
 * Other implementations (e.g. some versions of OpenSSL, in some
 * configurations) prevent the same attack by prepending an empty
 * application_data record to every application_data record they send; we do
 * not do that because some implementations cannot handle empty
 * application_data records. Also, we only split application_data records and
 * not other types of records, because some implementations will not accept
 * fragmented records of some other types (e.g. some versions of NSS do not
 * accept fragmented alerts).
 */
#define SSL_CBC_RANDOM_IV 23
#define SSL_ENABLE_OCSP_STAPLING 24 /* Request OCSP stapling (client) */

/* SSL_ENABLE_NPN is defunct and defaults to false.
 * Using this option will not have any effect but won't produce an error. */
#define SSL_ENABLE_NPN 25

/* SSL_ENABLE_ALPN controls whether the ALPN extension is enabled for the
 * initial handshake when application layer protocol negotiation is used.
 * SSL_SetNextProtoNego or SSL_SetNextProtoCallback can be used to control
 * the application layer protocol negotiation;
 * ALPN is not negotiated for renegotiation handshakes, even though the ALPN
 * specification defines a way to use ALPN during renegotiations.
 * SSL_ENABLE_ALPN is currently enabled by default, but this may change in
 * future versions.
 */
#define SSL_ENABLE_ALPN 26

/* SSL_REUSE_SERVER_ECDHE_KEY controls whether the ECDHE server key is
 * reused for multiple handshakes or generated each time.
 * SSL_REUSE_SERVER_ECDHE_KEY is currently enabled by default.
 * This socket option is for ECDHE, only. It is unrelated to DHE.
 */
#define SSL_REUSE_SERVER_ECDHE_KEY 27

#define SSL_ENABLE_FALLBACK_SCSV 28 /* Send fallback SCSV in \
                                     * handshakes. */

/* SSL_ENABLE_SERVER_DHE controls whether DHE is enabled for the server socket.
 */
#define SSL_ENABLE_SERVER_DHE 29

/* Use draft-ietf-tls-session-hash. Controls whether we offer the
 * extended_master_secret extension which, when accepted, hashes
 * the handshake transcript into the master secret. This option is
 * enabled by default.
 */
#define SSL_ENABLE_EXTENDED_MASTER_SECRET 30

/* Request Signed Certificate Timestamps via TLS extension (client) */
#define SSL_ENABLE_SIGNED_CERT_TIMESTAMPS 31

/* Ordinarily, when negotiating a TLS_DHE_* cipher suite the server picks the
 * group.  draft-ietf-tls-negotiated-ff-dhe changes this to use supported_groups
 * (formerly supported_curves) to signal which pre-defined groups are OK.
 *
 * This option causes an NSS client to use this extension and demand that those
 * groups be used.  A client will signal any enabled DHE groups in the
 * supported_groups extension and reject groups that don't match what it has
 * enabled.  A server will only negotiate TLS_DHE_* cipher suites if the
 * client includes the extension.
 *
 * See SSL_NamedGroupConfig() for how to control which groups are enabled.
 *
 * This option cannot be enabled if NSS is not compiled with ECC support.
 */
#define SSL_REQUIRE_DH_NAMED_GROUPS 32

/* Allow 0-RTT data (for TLS 1.3).
 *
 * When this option is set, the server's session tickets will contain
 * a flag indicating that it accepts 0-RTT. When resuming such a
 * session, PR_Write() on the client will be allowed immediately after
 * starting the handshake and PR_Read() on the server will be allowed
 * on the server to read that data. Calls to
 * SSL_GetPreliminaryChannelInfo() and SSL_GetNextProto()
 * can be made used during this period to learn about the channel
 * parameters.
 *
 * The transition between the 0-RTT and 1-RTT modes is marked by the
 * handshake callback.  However, it is possible to force the completion
 * of the handshake (and cause the handshake callback to be called)
 * prior to reading all 0-RTT data using SSL_ForceHandshake().  To
 * ensure that all early data is read before the handshake callback, any
 * time that SSL_ForceHandshake() returns a PR_WOULD_BLOCK_ERROR, use
 * PR_Read() to read all available data.  If PR_Read() is called
 * multiple times, this will result in the handshake completing, but the
 * handshake callback will occur after early data has all been read.
 *
 * WARNING: 0-RTT data has different anti-replay and PFS properties than
 * the rest of the TLS data. See [draft-ietf-tls-tls13; Section 8]
 * for more details.
 *
 * Note: when DTLS 1.3 is in use, any 0-RTT data received after EndOfEarlyData
 * (e.g., because of reordering) is discarded.
 */
#define SSL_ENABLE_0RTT_DATA 33

/* Sets a limit to the size of encrypted records (see
 * draft-ietf-tls-record-limit). This is the value that is advertised to peers,
 * not a limit on the size of records that will be created.  Setting this value
 * reduces the size of records that will be received (not sent).
 *
 * This limit applies to the plaintext, but the records that appear on the wire
 * will be bigger.  This doesn't include record headers, IVs, block cipher
 * padding, and authentication tags or MACs.
 *
 * NSS always advertises the record size limit extension.  If this option is not
 * set, the extension will contain the maximum allowed size for the selected TLS
 * version (currently this is 16384 or 2^14 for TLS 1.2 and lower and 16385 for
 * TLS 1.3).
 *
 * By default, NSS creates records that are the maximum size possible, using all
 * the data that was written by the application.  Writes larger than the maximum
 * are split into maximum sized records, and any remainder (unless
 * SSL_CBC_RANDOM_IV is enabled and active).  If a peer advertises a record size
 * limit then that value is used instead.
 */
#define SSL_RECORD_SIZE_LIMIT 34

/* Enables TLS 1.3 compatibility mode.  In this mode, the client includes a fake
 * session ID in the handshake and sends a ChangeCipherSpec.  A server will
 * always use the setting chosen by the client, so the value of this option has
 * no effect for a server. This setting is ignored for DTLS. */
#define SSL_ENABLE_TLS13_COMPAT_MODE 35

/* Enables the sending of DTLS records using the short (two octet) record
 * header.  Only do this if there are 2^10 or fewer packets in flight at a time;
 * using this with a larger number of packets in flight could mean that packets
 * are dropped if there is reordering.
 *
 * This applies to TLS 1.3 only.  This is not a parameter that is negotiated
 * during the TLS handshake. Unlike other socket options, this option can be
 * changed after a handshake is complete.
 */
#define SSL_ENABLE_DTLS_SHORT_HEADER 36

/*
 * Enables the processing of the downgrade sentinel that can be added to the
 * ServerHello.random by a server that supports Section 4.1.3 of TLS 1.3
 * [RFC8446].  This sentinel will always be generated by a server that
 * negotiates a version lower than its maximum, this only controls whether a
 * client will treat receipt of a value that indicates a downgrade as an error.
 */
#define SSL_ENABLE_HELLO_DOWNGRADE_CHECK 37

/* Enables the SSLv2-compatible ClientHello for servers. NSS does not support
 * SSLv2 and will never send an SSLv2-compatible ClientHello as a client.  An
 * NSS server with this option enabled will accept a ClientHello that is
 * v2-compatible as defined in Appendix E.1 of RFC 6101.
 *
 * This is disabled by default and will be removed in a future version. */
#define SSL_ENABLE_V2_COMPATIBLE_HELLO 38

/* Enables the post-handshake authentication in TLS 1.3.  If it is set
 * to PR_TRUE, the client will send the "post_handshake_auth"
 * extension to indicate that it will process CertificateRequest
 * messages after handshake.
 *
 * This option applies only to clients.  For a server, the
 * SSL_SendCertificateRequest can be used to request post-handshake
 * authentication.
 */
#define SSL_ENABLE_POST_HANDSHAKE_AUTH 39

/* Enables the delegated credentials extension (draft-ietf-tls-subcerts). When
 * enabled, a client that supports TLS 1.3 will indicate willingness to
 * negotiate a delegated credential (DC). Note that client-delegated credentials
 * are not currently supported.
 *
 * If support is indicated, the peer may use a DC to authenticate itself. The DC
 * is sent as an extension to the peer's end-entity certificate; the end-entity
 * certificate is used to verify the DC, which in turn is used to verify the
 * handshake. DCs effectively extend the certificate chain by one, but only
 * within the context of TLS. Once issued, DCs can't be revoked; in order to
 * mitigate the damage in case the secret key is compromised, the DC is only
 * valid for a short time (days, hours, or even minutes).
 *
 * This library implements draft-07 of the protocol spec.
 */
#define SSL_ENABLE_DELEGATED_CREDENTIALS 40

/* Causes TLS (>=1.3) to suppress the EndOfEarlyData message in stream mode.
 *
 * This is not advisable in general, but the message only exists to delineate
 * early data in a streamed connection.  DTLS does not use this message as a
 * result.  The integration of TLS with QUIC, which uses a record/packet
 * protection layer that is unreliable, also does not use this message.
 *
 * On the server, this requires that SSL_RecordLayerData be used.
 * EndOfEarlyData is otherwise needed to drive key changes.  Additionally,
 * servers that use this API must check that handshake messages (Certificate,
 * CertificateVerify, and Finished in particular) are only received in epoch 2
 * (Handshake).  SSL_RecordLayerData will accept these handshake messages if
 * they are passed as epoch 1 (Early Data) in a single call.
 *
 * Using this option will cause connections to fail if early data is attempted
 * and the peer expects this message.
 */
#define SSL_SUPPRESS_END_OF_EARLY_DATA 41

#ifdef SSL_DEPRECATED_FUNCTION
/* Old deprecated function names */
SSL_IMPORT SECStatus SSL_Enable(PRFileDesc *fd, int option, PRIntn on);
SSL_IMPORT SECStatus SSL_EnableDefault(int option, PRIntn on);
#endif

/* Set (and get) options for sockets and defaults for newly created sockets.
 *
 * While the |val| parameter of these methods is PRIntn, options only support
 * two values by default: PR_TRUE or PR_FALSE.  The documentation of specific
 * options will explain if other values are permitted.
 */
SSL_IMPORT SECStatus SSL_OptionSet(PRFileDesc *fd, PRInt32 option, PRIntn val);
SSL_IMPORT SECStatus SSL_OptionGet(PRFileDesc *fd, PRInt32 option, PRIntn *val);
SSL_IMPORT SECStatus SSL_OptionSetDefault(PRInt32 option, PRIntn val);
SSL_IMPORT SECStatus SSL_OptionGetDefault(PRInt32 option, PRIntn *val);
SSL_IMPORT SECStatus SSL_CertDBHandleSet(PRFileDesc *fd, CERTCertDBHandle *dbHandle);

/* SSLNextProtoCallback is called during the handshake for the server, when an
 * Application-Layer Protocol Negotiation (ALPN) extension has been received
 * from the client. |protos| and |protosLen| define a buffer which contains the
 * client's advertisement.
 * |protoOut| is a buffer provided by the caller, of length 255 (the maximum
 * allowed by the protocol). On successful return, the protocol to be announced
 * to the server will be in |protoOut| and its length in |*protoOutLen|.
 *
 * The callback must return SECFailure or SECSuccess (not SECWouldBlock).
 */
typedef SECStatus(PR_CALLBACK *SSLNextProtoCallback)(
    void *arg,
    PRFileDesc *fd,
    const unsigned char *protos,
    unsigned int protosLen,
    unsigned char *protoOut,
    unsigned int *protoOutLen,
    unsigned int protoMaxOut);

/* SSL_SetNextProtoCallback sets a callback function to handle ALPN Negotiation.
 * It causes a client to advertise ALPN. */
SSL_IMPORT SECStatus SSL_SetNextProtoCallback(PRFileDesc *fd,
                                              SSLNextProtoCallback callback,
                                              void *arg);

/* SSL_SetNextProtoNego can be used as an alternative to
 * SSL_SetNextProtoCallback.
 *
 * Using this function allows client and server to transparently support ALPN.
 * The same set of protocols will be advertised via ALPN and, if the server
 * uses ALPN to select a protocol, SSL_GetNextProto will return
 * SSL_NEXT_PROTO_SELECTED as the state.
 *
 * Because the predecessor to ALPN, NPN, used the first protocol as the fallback
 * protocol, when sending an ALPN extension, the first protocol is moved to the
 * end of the list. This indicates that the fallback protocol is the least
 * preferred. The other protocols should be in preference order.
 *
 * The supported protocols are specified in |data| in wire-format (8-bit
 * length-prefixed). For example: "\010http/1.1\006spdy/2". */
SSL_IMPORT SECStatus SSL_SetNextProtoNego(PRFileDesc *fd,
                                          const unsigned char *data,
                                          unsigned int length);

typedef enum SSLNextProtoState {
    SSL_NEXT_PROTO_NO_SUPPORT = 0, /* No peer support                   */
    SSL_NEXT_PROTO_NEGOTIATED = 1, /* Mutual agreement                  */
    SSL_NEXT_PROTO_NO_OVERLAP = 2, /* No protocol overlap found         */
    SSL_NEXT_PROTO_SELECTED = 3,   /* Server selected proto (ALPN)      */
    SSL_NEXT_PROTO_EARLY_VALUE = 4 /* We are in 0-RTT using this value. */
} SSLNextProtoState;

/* SSL_GetNextProto can be used in the HandshakeCallback or any time after
 * a handshake to retrieve the result of the Next Protocol negotiation.
 *
 * The length of the negotiated protocol, if any, is written into *bufLen.
 * If the negotiated protocol is longer than bufLenMax, then SECFailure is
 * returned. Otherwise, the negotiated protocol, if any, is written into buf,
 * and SECSuccess is returned. */
SSL_IMPORT SECStatus SSL_GetNextProto(PRFileDesc *fd,
                                      SSLNextProtoState *state,
                                      unsigned char *buf,
                                      unsigned int *bufLen,
                                      unsigned int bufLenMax);

/*
** Control ciphers that SSL uses. If on is non-zero then the named cipher
** is enabled, otherwise it is disabled.
** The "cipher" values are defined in sslproto.h (the SSL_EN_* values).
** EnableCipher records user preferences.
** SetPolicy sets the policy according to the policy module.
*/
#ifdef SSL_DEPRECATED_FUNCTION
/* Old deprecated function names */
SSL_IMPORT SECStatus SSL_EnableCipher(long which, PRBool enabled);
SSL_IMPORT SECStatus SSL_SetPolicy(long which, int policy);
#endif

/* New function names */
SSL_IMPORT SECStatus SSL_CipherPrefSet(PRFileDesc *fd, PRInt32 cipher, PRBool enabled);
SSL_IMPORT SECStatus SSL_CipherPrefGet(PRFileDesc *fd, PRInt32 cipher, PRBool *enabled);
SSL_IMPORT SECStatus SSL_CipherPrefSetDefault(PRInt32 cipher, PRBool enabled);
SSL_IMPORT SECStatus SSL_CipherPrefGetDefault(PRInt32 cipher, PRBool *enabled);
SSL_IMPORT SECStatus SSL_CipherPolicySet(PRInt32 cipher, PRInt32 policy);
SSL_IMPORT SECStatus SSL_CipherPolicyGet(PRInt32 cipher, PRInt32 *policy);

/*
** Control for TLS signature schemes for TLS 1.2 and 1.3.
**
** This governs what signature schemes (or algorithms) are sent by a client in
** the signature_algorithms extension.  A client will not accept a signature
** from a server unless it uses an enabled algorithm.
**
** This also governs what the server sends in the supported_signature_algorithms
** field of a CertificateRequest.
**
** This changes what the server uses to sign ServerKeyExchange and
** CertificateVerify messages.  An endpoint uses the first entry from this list
** that is compatible with both its certificate and its peer's supported
** values.
**
** This configuration affects TLS 1.2, but the combination of EC group and hash
** algorithm is interpreted loosely to be compatible with other implementations.
** For TLS 1.2, NSS will ignore the curve group when generating or verifying
** ECDSA signatures.  For example, a P-384 ECDSA certificate is used with
** SHA-256 if ssl_sig_ecdsa_secp256r1_sha256 is enabled.
**
** Omitting SHA-256 schemes from this list might be foolish.  Support is
** mandatory in TLS 1.2 and 1.3 and there might be interoperability issues.
*/
SSL_IMPORT SECStatus SSL_SignatureSchemePrefSet(
    PRFileDesc *fd, const SSLSignatureScheme *schemes, unsigned int count);

/* Deprecated, use SSL_SignatureSchemePrefSet() instead. */
SSL_IMPORT SECStatus SSL_SignaturePrefSet(
    PRFileDesc *fd, const SSLSignatureAndHashAlg *algorithms,
    unsigned int count);

/*
** Get the currently configured signature schemes.
**
** The schemes are written to |schemes| but not if there are more than
** |maxCount| values configured.  The number of schemes that are in use are
** written to |count|.  This fails if |maxCount| is insufficiently large.
*/
SSL_IMPORT SECStatus SSL_SignatureSchemePrefGet(
    PRFileDesc *fd, SSLSignatureScheme *algorithms, unsigned int *count,
    unsigned int maxCount);

/* Deprecated, use SSL_SignatureSchemePrefGet() instead. */
SSL_IMPORT SECStatus SSL_SignaturePrefGet(
    PRFileDesc *fd, SSLSignatureAndHashAlg *algorithms, unsigned int *count,
    unsigned int maxCount);

/*
** Returns the maximum number of signature algorithms that are supported and
** can be set or retrieved using SSL_SignatureSchemePrefSet or
** SSL_SignatureSchemePrefGet.
*/
SSL_IMPORT unsigned int SSL_SignatureMaxCount(void);

/*
** Define custom priorities for EC and FF groups used in DH key exchange and EC
** groups for ECDSA. This only changes the order of enabled lists (and thus
** their priorities) and enables all groups in |groups| while disabling all other
** groups.
*/
SSL_IMPORT SECStatus SSL_NamedGroupConfig(PRFileDesc *fd,
                                          const SSLNamedGroup *groups,
                                          unsigned int num_groups);

/*
** Configure the socket to configure additional key shares.  Normally when a TLS
** 1.3 ClientHello is sent, just one key share is included using the first
** preference group (as set by SSL_NamedGroupConfig).  If the server decides to
** pick a different group for key exchange, it is forced to send a
** HelloRetryRequest, which adds an entire round trip of latency.
**
** This function can be used to configure libssl to generate additional key
** shares when sending a TLS 1.3 ClientHello.  If |count| is set to a non-zero
** value, then additional key shares are generated.  Shares are added in the
** preference order set in SSL_NamedGroupConfig.  |count| can be set to any
** value; NSS limits the number of shares to the number of supported groups.
*/
SSL_IMPORT SECStatus SSL_SendAdditionalKeyShares(PRFileDesc *fd,
                                                 unsigned int count);

/* Deprecated: use SSL_NamedGroupConfig() instead.
** SSL_DHEGroupPrefSet is used to configure the set of allowed/enabled DHE group
** parameters that can be used by NSS for the given server socket.
** The first item in the array is used as the default group, if no other
** selection criteria can be used by NSS.
** The set is provided as an array of identifiers as defined by SSLDHEGroupType.
** If more than one group identifier is provided, NSS will select the one to use.
** For example, a TLS extension sent by the client might indicate a preference.
*/
SSL_IMPORT SECStatus SSL_DHEGroupPrefSet(PRFileDesc *fd,
                                         const SSLDHEGroupType *groups,
                                         PRUint16 num_groups);

/* Enable the use of a DHE group that's smaller than the library default,
** for backwards compatibility reasons. The DH parameters will be created
** at the time this function is called, which might take a very long time.
** The function will block until generation is completed.
** The intention is to enforce that fresh and safe parameters are generated
** each time a process is started.
** At the time this API was initially implemented, the API will enable the
** use of 1024 bit DHE parameters. This value might get increased in future
** versions of NSS.
**
** It is allowed to call this API will a NULL value for parameter fd,
** which will prepare the global parameters that NSS will reuse for the remainder
** of the process lifetime. This can be used early after startup of a process,
** to avoid a delay when handling incoming client connections.
** This preparation with a NULL for parameter fd will NOT enable the weak group
** on sockets. The function needs to be called again for every socket that
** should use the weak group.
**
** It is allowed to use this API in combination with the SSL_NamedGroupConfig API.
** If both APIs have been called, the weakest group will be used, unless it is
** certain that the client supports larger group parameters. The weak group will
** be used as the default group for TLS <= 1.2, overriding the preference for
** the first group potentially set with a call to SSL_NamedGroupConfig.
*/
SSL_IMPORT SECStatus SSL_EnableWeakDHEPrimeGroup(PRFileDesc *fd, PRBool enabled);

/* SSL Version Range API
**
** This API should be used to control SSL 3.0 & TLS support instead of the
** older SSL_Option* API; however, the SSL_Option* API MUST still be used to
** control SSL 2.0 support. In this version of libssl, SSL 3.0 and TLS 1.0 are
** enabled by default. Future versions of libssl may change which versions of
** the protocol are enabled by default.
**
** The SSLProtocolVariant enum indicates whether the protocol is of type
** stream or datagram. This must be provided to the functions that do not
** take an fd. Functions which take an fd will get the variant from the fd,
** which is typed.
**
** Using the new version range API in conjunction with the older
** SSL_OptionSet-based API for controlling the enabled protocol versions may
** cause unexpected results. Going forward, we guarantee only the following:
**
** SSL_OptionGet(SSL_ENABLE_TLS) will return PR_TRUE if *ANY* versions of TLS
** are enabled.
**
** SSL_OptionSet(SSL_ENABLE_TLS, PR_FALSE) will disable *ALL* versions of TLS,
** including TLS 1.0 and later.
**
** The above two properties provide compatibility for applications that use
** SSL_OptionSet to implement the insecure fallback from TLS 1.x to SSL 3.0.
**
** SSL_OptionSet(SSL_ENABLE_TLS, PR_TRUE) will enable TLS 1.0, and may also
** enable some later versions of TLS, if it is necessary to do so in order to
** keep the set of enabled versions contiguous. For example, if TLS 1.2 is
** enabled, then after SSL_OptionSet(SSL_ENABLE_TLS, PR_TRUE), TLS 1.0,
** TLS 1.1, and TLS 1.2 will be enabled, and the call will have no effect on
** whether SSL 3.0 is enabled. If no later versions of TLS are enabled at the
** time SSL_OptionSet(SSL_ENABLE_TLS, PR_TRUE) is called, then no later
** versions of TLS will be enabled by the call.
**
** SSL_OptionSet(SSL_ENABLE_SSL3, PR_FALSE) will disable SSL 3.0, and will not
** change the set of TLS versions that are enabled.
**
** SSL_OptionSet(SSL_ENABLE_SSL3, PR_TRUE) will enable SSL 3.0, and may also
** enable some versions of TLS if TLS 1.1 or later is enabled at the time of
** the call, the same way SSL_OptionSet(SSL_ENABLE_TLS, PR_TRUE) works, in
** order to keep the set of enabled versions contiguous.
*/

/* Returns, in |*vrange|, the range of SSL3/TLS versions supported for the
** given protocol variant by the version of libssl linked-to at runtime.
*/
SSL_IMPORT SECStatus SSL_VersionRangeGetSupported(
    SSLProtocolVariant protocolVariant, SSLVersionRange *vrange);

/* Returns, in |*vrange|, the range of SSL3/TLS versions enabled by default
** for the given protocol variant.
*/
SSL_IMPORT SECStatus SSL_VersionRangeGetDefault(
    SSLProtocolVariant protocolVariant, SSLVersionRange *vrange);

/* Sets the range of enabled-by-default SSL3/TLS versions for the given
** protocol variant to |*vrange|.
*/
SSL_IMPORT SECStatus SSL_VersionRangeSetDefault(
    SSLProtocolVariant protocolVariant, const SSLVersionRange *vrange);

/* Returns, in |*vrange|, the range of enabled SSL3/TLS versions for |fd|. */
SSL_IMPORT SECStatus SSL_VersionRangeGet(PRFileDesc *fd,
                                         SSLVersionRange *vrange);

/* Sets the range of enabled SSL3/TLS versions for |fd| to |*vrange|. */
SSL_IMPORT SECStatus SSL_VersionRangeSet(PRFileDesc *fd,
                                         const SSLVersionRange *vrange);

/* Sets the version to check the server random against for the
 * fallback check defined in [draft-ietf-tls-tls13-11 Section 6.3.1.1].
 * This function is provided to allow for detection of forced downgrade
 * attacks against client-side reconnect-and-fallback outside of TLS
 * by setting |version| to be that of the original connection, rather
 * than that of the new connection.
 *
 * The default, which can also be enabled by setting |version| to
 * zero, is just to check against the max version in the
 * version range (see SSL_VersionRangeSet). */
SSL_IMPORT SECStatus SSL_SetDowngradeCheckVersion(PRFileDesc *fd,
                                                  PRUint16 version);

/* Values for "policy" argument to SSL_CipherPolicySet */
/* Values returned by SSL_CipherPolicyGet. */
#define SSL_NOT_ALLOWED 0 /* or invalid or unimplemented */
#define SSL_ALLOWED 1
#define SSL_RESTRICTED 2 /* only with "Step-Up" certs. */

/* Values for "on" with SSL_REQUIRE_CERTIFICATE. */
#define SSL_REQUIRE_NEVER ((PRBool)0)
#define SSL_REQUIRE_ALWAYS ((PRBool)1)
#define SSL_REQUIRE_FIRST_HANDSHAKE ((PRBool)2)
#define SSL_REQUIRE_NO_ERROR ((PRBool)3)

/* Values for "on" with SSL_ENABLE_RENEGOTIATION */
/* Never renegotiate at all.                                               */
#define SSL_RENEGOTIATE_NEVER ((PRBool)0)
/* Renegotiate without restriction, whether or not the peer's client hello */
/* bears the renegotiation info extension.  Vulnerable, as in the past.    */
#define SSL_RENEGOTIATE_UNRESTRICTED ((PRBool)1)
/* Only renegotiate if the peer's hello bears the TLS renegotiation_info   */
/* extension. This is safe renegotiation.                                  */
#define SSL_RENEGOTIATE_REQUIRES_XTN ((PRBool)2)
/* Disallow unsafe renegotiation in server sockets only, but allow clients */
/* to continue to renegotiate with vulnerable servers.                     */
/* This value should only be used during the transition period when few    */
/* servers have been upgraded.                                             */
#define SSL_RENEGOTIATE_TRANSITIONAL ((PRBool)3)

/*
** Reset the handshake state for fd. This will make the complete SSL
** handshake protocol execute from the ground up on the next i/o
** operation.
*/
SSL_IMPORT SECStatus SSL_ResetHandshake(PRFileDesc *fd, PRBool asServer);

/*
** Force the handshake for fd to complete immediately.  This blocks until
** the complete SSL handshake protocol is finished.
*/
SSL_IMPORT SECStatus SSL_ForceHandshake(PRFileDesc *fd);

/*
** Same as above, but with an I/O timeout.
 */
SSL_IMPORT SECStatus SSL_ForceHandshakeWithTimeout(PRFileDesc *fd,
                                                   PRIntervalTime timeout);

/*
** Query security status of socket. *on is set to one if security is
** enabled. *keySize will contain the stream key size used. *issuer will
** contain the RFC1485 verison of the name of the issuer of the
** certificate at the other end of the connection. For a client, this is
** the issuer of the server's certificate; for a server, this is the
** issuer of the client's certificate (if any). Subject is the subject of
** the other end's certificate. The pointers can be zero if the desired
** data is not needed.  All strings returned by this function are owned
** by the caller, and need to be freed with PORT_Free.
*/
SSL_IMPORT SECStatus SSL_SecurityStatus(PRFileDesc *fd, int *on, char **cipher,
                                        int *keySize, int *secretKeySize,
                                        char **issuer, char **subject);

/* Values for "on" */
#define SSL_SECURITY_STATUS_NOOPT -1
#define SSL_SECURITY_STATUS_OFF 0
#define SSL_SECURITY_STATUS_ON_HIGH 1
#define SSL_SECURITY_STATUS_ON_LOW 2
#define SSL_SECURITY_STATUS_FORTEZZA 3 /* NO LONGER SUPPORTED */

/*
** Return the certificate for our SSL peer. If the client calls this
** it will always return the server's certificate. If the server calls
** this, it may return NULL if client authentication is not enabled or
** if the client had no certificate when asked.
**  "fd" the socket "file" descriptor
*/
SSL_IMPORT CERTCertificate *SSL_PeerCertificate(PRFileDesc *fd);

/*
** Return the certificates presented by the SSL peer. If the SSL peer
** did not present certificates, return NULL with the
** SSL_ERROR_NO_CERTIFICATE error. On failure, return NULL with an error
** code other than SSL_ERROR_NO_CERTIFICATE.
**  "fd" the socket "file" descriptor
*/
SSL_IMPORT CERTCertList *SSL_PeerCertificateChain(PRFileDesc *fd);

/* SSL_PeerStapledOCSPResponses returns the OCSP responses that were provided
 * by the TLS server. The return value is a pointer to an internal SECItemArray
 * that contains the returned OCSP responses; it is only valid until the
 * callback function that calls SSL_PeerStapledOCSPResponses returns.
 *
 * If no OCSP responses were given by the server then the result will be empty.
 * If there was an error, then the result will be NULL.
 *
 * You must set the SSL_ENABLE_OCSP_STAPLING option to enable OCSP stapling.
 * to be provided by a server.
 *
 * libssl does not do any validation of the OCSP response itself; the
 * authenticate certificate hook is responsible for doing so. The default
 * authenticate certificate hook, SSL_AuthCertificate, does not implement
 * any OCSP stapling funtionality, but this may change in future versions.
 */
SSL_IMPORT const SECItemArray *SSL_PeerStapledOCSPResponses(PRFileDesc *fd);

/* SSL_PeerSignedCertTimestamps returns the signed_certificate_timestamp
 * extension data provided by the TLS server. The return value is a pointer
 * to an internal SECItem that contains the returned response (as a serialized
 * SignedCertificateTimestampList, see RFC 6962). The returned pointer is only
 * valid until the callback function that calls SSL_PeerSignedCertTimestamps
 * (e.g. the authenticate certificate hook, or the handshake callback) returns.
 *
 * If no Signed Certificate Timestamps were given by the server then the result
 * will be empty. If there was an error, then the result will be NULL.
 *
 * You must set the SSL_ENABLE_SIGNED_CERT_TIMESTAMPS option to indicate support
 * for Signed Certificate Timestamps to a server.
 *
 * libssl does not do any parsing or validation of the response itself.
 */
SSL_IMPORT const SECItem *SSL_PeerSignedCertTimestamps(PRFileDesc *fd);

/* SSL_SetStapledOCSPResponses stores an array of one or multiple OCSP responses
 * in the fd's data, which may be sent as part of a server side cert_status
 * handshake message. Parameter |responses| is for the server certificate of
 * the key exchange type |kea|.
 * The function will duplicate the responses array.
 *
 * Deprecated: see SSL_ConfigSecureServer for details.
 */
SSL_IMPORT SECStatus
SSL_SetStapledOCSPResponses(PRFileDesc *fd, const SECItemArray *responses,
                            SSLKEAType kea);

/*
 * SSL_SetSignedCertTimestamps stores serialized signed_certificate_timestamp
 * extension data in the fd. The signed_certificate_timestamp data is sent
 * during the handshake (if requested by the client). Parameter |scts|
 * is for the server certificate of the key exchange type |kea|.
 * The function will duplicate the provided data item. To clear previously
 * set data for a given key exchange type |kea|, pass NULL to |scts|.
 *
 * Deprecated: see SSL_ConfigSecureServer for details.
 */
SSL_IMPORT SECStatus
SSL_SetSignedCertTimestamps(PRFileDesc *fd, const SECItem *scts,
                            SSLKEAType kea);

/*
** Authenticate certificate hook. Called when a certificate comes in
** (because of SSL_REQUIRE_CERTIFICATE in SSL_Enable) to authenticate the
** certificate.
**
** The authenticate certificate hook must return SECSuccess to indicate the
** certificate is valid, SECFailure to indicate the certificate is invalid,
** or SECWouldBlock if the application will authenticate the certificate
** asynchronously. SECWouldBlock is only supported for non-blocking sockets.
**
** If the authenticate certificate hook returns SECFailure, then the bad cert
** hook will be called. The bad cert handler is NEVER called if the
** authenticate certificate hook returns SECWouldBlock. If the application
** needs to handle and/or override a bad cert, it should do so before it
** calls SSL_AuthCertificateComplete (modifying the error it passes to
** SSL_AuthCertificateComplete as needed).
**
** See the documentation for SSL_AuthCertificateComplete for more information
** about the asynchronous behavior that occurs when the authenticate
** certificate hook returns SECWouldBlock.
**
** RFC 6066 says that clients should send the bad_certificate_status_response
** alert when they encounter an error processing the stapled OCSP response.
** libssl does not provide a way for the authenticate certificate hook to
** indicate that an OCSP error (SEC_ERROR_OCSP_*) that it returns is an error
** in the stapled OCSP response or an error in some other OCSP response.
** Further, NSS does not provide a convenient way to control or determine
** which OCSP response(s) were used to validate a certificate chain.
** Consequently, the current version of libssl does not ever send the
** bad_certificate_status_response alert. This may change in future releases.
*/
typedef SECStatus(PR_CALLBACK *SSLAuthCertificate)(void *arg, PRFileDesc *fd,
                                                   PRBool checkSig,
                                                   PRBool isServer);

SSL_IMPORT SECStatus SSL_AuthCertificateHook(PRFileDesc *fd,
                                             SSLAuthCertificate f,
                                             void *arg);

/* An implementation of the certificate authentication hook */
SSL_IMPORT SECStatus SSL_AuthCertificate(void *arg, PRFileDesc *fd,
                                         PRBool checkSig, PRBool isServer);

/*
 * Prototype for SSL callback to get client auth data from the application.
 *  arg - application passed argument
 *  caNames - pointer to distinguished names of CAs that the server likes
 *  pRetCert - pointer to pointer to cert, for return of cert
 *  pRetKey - pointer to key pointer, for return of key
 */
typedef SECStatus(PR_CALLBACK *SSLGetClientAuthData)(void *arg,
                                                     PRFileDesc *fd,
                                                     CERTDistNames *caNames,
                                                     CERTCertificate **pRetCert,  /*return */
                                                     SECKEYPrivateKey **pRetKey); /* return */

/*
 * Set the client side callback for SSL to retrieve user's private key
 * and certificate.
 *  fd - the file descriptor for the connection in question
 *  f - the application's callback that delivers the key and cert
 *  a - application specific data
 */
SSL_IMPORT SECStatus SSL_GetClientAuthDataHook(PRFileDesc *fd,
                                               SSLGetClientAuthData f, void *a);

/*
** SNI extension processing callback function.
** It is called when SSL socket receives SNI extension in ClientHello message.
** Upon this callback invocation, application is responsible to reconfigure the
** socket with the data for a particular server name.
** There are three potential outcomes of this function invocation:
**    * application does not recognize the name or the type and wants the
**    "unrecognized_name" alert be sent to the client. In this case the callback
**    function must return SSL_SNI_SEND_ALERT status.
**    * application does not recognize  the name, but wants to continue with
**    the handshake using the current socket configuration. In this case,
**    no socket reconfiguration is needed and the function should return
**    SSL_SNI_CURRENT_CONFIG_IS_USED.
**    * application recognizes the name and reconfigures the socket with
**    appropriate certs, key, etc. There are many ways to reconfigure. NSS
**    provides SSL_ReconfigFD function that can be used to update the socket
**    data from model socket. To continue with the rest of the handshake, the
**    implementation function should return an index of a name it has chosen.
** LibSSL will ignore any SNI extension received in a ClientHello message
** if application does not register a SSLSNISocketConfig callback.
** Each type field of SECItem indicates the name type.
** NOTE: currently RFC3546 defines only one name type: sni_host_name.
** Client is allowed to send only one name per known type. LibSSL will
** send an "unrecognized_name" alert if SNI extension name list contains more
** then one name of a type.
*/
typedef PRInt32(PR_CALLBACK *SSLSNISocketConfig)(PRFileDesc *fd,
                                                 const SECItem *srvNameArr,
                                                 PRUint32 srvNameArrSize,
                                                 void *arg);

/*
** SSLSNISocketConfig should return an index within 0 and srvNameArrSize-1
** when it has reconfigured the socket fd to use certs and keys, etc
** for a specific name. There are two other allowed return values. One
** tells libSSL to use the default cert and key.  The other tells libSSL
** to send the "unrecognized_name" alert.  These values are:
**/
#define SSL_SNI_CURRENT_CONFIG_IS_USED -1
#define SSL_SNI_SEND_ALERT -2

/*
** Set application implemented SNISocketConfig callback.
*/
SSL_IMPORT SECStatus SSL_SNISocketConfigHook(PRFileDesc *fd,
                                             SSLSNISocketConfig f,
                                             void *arg);

/*
** Reconfigure fd SSL socket with model socket parameters. Sets
** server certs and keys, list of trust anchor, socket options
** and all SSL socket call backs and parameters.
*/
SSL_IMPORT PRFileDesc *SSL_ReconfigFD(PRFileDesc *model, PRFileDesc *fd);

/*
 * Set the client side argument for SSL to retrieve PKCS #11 pin.
 *  fd - the file descriptor for the connection in question
 *  a - pkcs11 application specific data
 */
SSL_IMPORT SECStatus SSL_SetPKCS11PinArg(PRFileDesc *fd, void *a);

/*
** These are callbacks for dealing with SSL alerts.
 */

typedef PRUint8 SSLAlertLevel;
typedef PRUint8 SSLAlertDescription;

typedef struct {
    SSLAlertLevel level;
    SSLAlertDescription description;
} SSLAlert;

typedef void(PR_CALLBACK *SSLAlertCallback)(const PRFileDesc *fd, void *arg,
                                            const SSLAlert *alert);

SSL_IMPORT SECStatus SSL_AlertReceivedCallback(PRFileDesc *fd, SSLAlertCallback cb,
                                               void *arg);
SSL_IMPORT SECStatus SSL_AlertSentCallback(PRFileDesc *fd, SSLAlertCallback cb,
                                           void *arg);
/*
** This is a callback for dealing with server certs that are not authenticated
** by the client.  The client app can decide that it actually likes the
** cert by some external means and restart the connection.
**
** The bad cert hook must return SECSuccess to override the result of the
** authenticate certificate hook, SECFailure if the certificate should still be
** considered invalid, or SECWouldBlock if the application will authenticate
** the certificate asynchronously. SECWouldBlock is only supported for
** non-blocking sockets.
**
** See the documentation for SSL_AuthCertificateComplete for more information
** about the asynchronous behavior that occurs when the bad cert hook returns
** SECWouldBlock.
*/
typedef SECStatus(PR_CALLBACK *SSLBadCertHandler)(void *arg, PRFileDesc *fd);
SSL_IMPORT SECStatus SSL_BadCertHook(PRFileDesc *fd, SSLBadCertHandler f,
                                     void *arg);

/*
** Configure SSL socket for running a secure server. Needs the
** certificate for the server and the servers private key. The arguments
** are copied.
**
** This method should be used in preference to SSL_ConfigSecureServer,
** SSL_ConfigSecureServerWithCertChain, SSL_SetStapledOCSPResponses, and
** SSL_SetSignedCertTimestamps.
**
** The authentication method is determined from the certificate and private key
** based on how libssl authenticates peers. Primarily, this uses the value of
** the SSLAuthType enum and is derived from the type of public key in the
** certificate.  For example, different RSA certificates might be saved for
** signing (ssl_auth_rsa_sign) and key encipherment
** (ssl_auth_rsa_decrypt). Unique to RSA, the same certificate can be used for
** both usages. Additional information about the authentication method is also
** used: EC keys with different curves are separately stored.
**
** Only one certificate is stored for each authentication method.
**
** The optional |data| argument contains additional information about the
** certificate:
**
** - |authType| (with a value other than ssl_auth_null) limits the
**   authentication method; this is primarily useful in limiting the use of an
**   RSA certificate to one particular key usage (either signing or key
**   encipherment) when its key usages indicate support for both.
**
** - |certChain| provides an explicit certificate chain, rather than relying on
**   NSS functions for finding a certificate chain.
**
** - |stapledOCSPResponses| provides a response for OCSP stapling.
**
** - |signedCertTimestamps| provides a value for the
**   signed_certificate_timestamp extension used in certificate transparency.
**
** The |data_len| argument provides the length of the data.  This should be set
** to |sizeof(data)|.
**
** This function allows an application to provide certificates with narrow key
** usages attached to them.  For instance, RSA keys can be provided that are
** limited to signing or decryption only.  Multiple EC certificates with keys on
** different named curves can be provided.
**
** Unlike SSL_ConfigSecureServer(WithCertChain), this function does not accept
** NULL for the |cert| and |key| arguments.  It will replace certificates that
** have the same type, but it cannot be used to remove certificates that have
** already been configured.
*/
SSL_IMPORT SECStatus SSL_ConfigServerCert(
    PRFileDesc *fd, CERTCertificate *cert, SECKEYPrivateKey *key,
    const SSLExtraServerCertData *data, unsigned int data_len);

/*
** Deprecated variant of SSL_ConfigServerCert.
**
** This uses values from the SSLKEAType to identify the type of |key| that the
** |cert| contains.  This is incorrect, since key exchange and authentication
** are separated in some cipher suites (in particular, ECDHE_RSA_* suites).
**
** Providing a |kea| parameter of ssl_kea_ecdh (or kt_ecdh) is interpreted as
** providing both ECDH and ECDSA certificates.
*/
SSL_IMPORT SECStatus SSL_ConfigSecureServer(
    PRFileDesc *fd, CERTCertificate *cert,
    SECKEYPrivateKey *key, SSLKEAType kea);

/*
** Deprecated variant of SSL_ConfigSecureServerCert.  The |data| argument to
** SSL_ConfigSecureServerCert can be used to pass a certificate chain.
*/
SSL_IMPORT SECStatus
SSL_ConfigSecureServerWithCertChain(PRFileDesc *fd, CERTCertificate *cert,
                                    const CERTCertificateList *certChainOpt,
                                    SECKEYPrivateKey *key, SSLKEAType kea);

/*
** SSL_SetSessionTicketKeyPair configures an asymmetric key pair for use in
** wrapping session ticket keys, used by the server.  This function currently
** only accepts an RSA public/private key pair.
**
** Prior to the existence of this function, NSS used an RSA private key
** associated with a configured certificate to perform session ticket
** encryption.  If this function isn't used, the keys provided with a configured
** RSA certificate are used for wrapping session ticket keys.
**
** NOTE: This key is used for all self-encryption but is named for
** session tickets for historical reasons.
*/
SSL_IMPORT SECStatus
SSL_SetSessionTicketKeyPair(SECKEYPublicKey *pubKey, SECKEYPrivateKey *privKey);

/*
** Configure a secure server's session-id cache. Define the maximum number
** of entries in the cache, the longevity of the entires, and the directory
** where the cache files will be placed.  These values can be zero, and
** if so, the implementation will choose defaults.
** This version of the function is for use in applications that have only one
** process that uses the cache (even if that process has multiple threads).
*/
SSL_IMPORT SECStatus SSL_ConfigServerSessionIDCache(int maxCacheEntries,
                                                    PRUint32 timeout,
                                                    PRUint32 ssl3_timeout,
                                                    const char *directory);

/* Configure a secure server's session-id cache. Depends on value of
 * enableMPCache, configures malti-proc or single proc cache. */
SSL_IMPORT SECStatus SSL_ConfigServerSessionIDCacheWithOpt(
    PRUint32 timeout,
    PRUint32 ssl3_timeout,
    const char *directory,
    int maxCacheEntries,
    int maxCertCacheEntries,
    int maxSrvNameCacheEntries,
    PRBool enableMPCache);

/*
** Like SSL_ConfigServerSessionIDCache, with one important difference.
** If the application will run multiple processes (as opposed to, or in
** addition to multiple threads), then it must call this function, instead
** of calling SSL_ConfigServerSessionIDCache().
** This has nothing to do with the number of processORs, only processEs.
** This function sets up a Server Session ID (SID) cache that is safe for
** access by multiple processes on the same system.
*/
SSL_IMPORT SECStatus SSL_ConfigMPServerSIDCache(int maxCacheEntries,
                                                PRUint32 timeout,
                                                PRUint32 ssl3_timeout,
                                                const char *directory);

/* Get and set the configured maximum number of mutexes used for the
** server's store of SSL sessions.  This value is used by the server
** session ID cache initialization functions shown above.  Note that on
** some platforms, these mutexes are actually implemented with POSIX
** semaphores, or with unnamed pipes.  The default value varies by platform.
** An attempt to set a too-low maximum will return an error and the
** configured value will not be changed.
*/
SSL_IMPORT PRUint32 SSL_GetMaxServerCacheLocks(void);
SSL_IMPORT SECStatus SSL_SetMaxServerCacheLocks(PRUint32 maxLocks);

/* environment variable set by SSL_ConfigMPServerSIDCache, and queried by
 * SSL_InheritMPServerSIDCache when envString is NULL.
 */
#define SSL_ENV_VAR_NAME "SSL_INHERITANCE"

/* called in child to inherit SID Cache variables.
 * If envString is NULL, this function will use the value of the environment
 * variable "SSL_INHERITANCE", otherwise the string value passed in will be
 * used.
 */
SSL_IMPORT SECStatus SSL_InheritMPServerSIDCache(const char *envString);

/*
** Set the callback that gets called when a TLS handshake is complete. The
** handshake callback is called after verifying the peer's Finished message and
** before processing incoming application data.
**
** For the initial handshake: If the handshake false started (see
** SSL_ENABLE_FALSE_START), then application data may already have been sent
** before the handshake callback is called. If we did not false start then the
** callback will get called before any application data is sent.
*/
typedef void(PR_CALLBACK *SSLHandshakeCallback)(PRFileDesc *fd,
                                                void *client_data);
SSL_IMPORT SECStatus SSL_HandshakeCallback(PRFileDesc *fd,
                                           SSLHandshakeCallback cb, void *client_data);

/* Applications that wish to enable TLS false start must set this callback
** function. NSS will invoke the functon to determine if a particular
** connection should use false start or not. SECSuccess indicates that the
** callback completed successfully, and if so *canFalseStart indicates if false
** start can be used. If the callback does not return SECSuccess then the
** handshake will be canceled. NSS's recommended criteria can be evaluated by
** calling SSL_RecommendedCanFalseStart.
**
** If no false start callback is registered then false start will never be
** done, even if the SSL_ENABLE_FALSE_START option is enabled.
**/
typedef SECStatus(PR_CALLBACK *SSLCanFalseStartCallback)(
    PRFileDesc *fd, void *arg, PRBool *canFalseStart);

SSL_IMPORT SECStatus SSL_SetCanFalseStartCallback(
    PRFileDesc *fd, SSLCanFalseStartCallback callback, void *arg);

/* This function sets *canFalseStart according to the recommended criteria for
** false start. These criteria may change from release to release and may depend
** on which handshake features have been negotiated and/or properties of the
** certifciates/keys used on the connection.
*/
SSL_IMPORT SECStatus SSL_RecommendedCanFalseStart(PRFileDesc *fd,
                                                  PRBool *canFalseStart);

/*
** For the server, request a new handshake.  For the client, begin a new
** handshake.  If flushCache is non-zero, the SSL3 cache entry will be
** flushed first, ensuring that a full SSL handshake will be done.
** If flushCache is zero, and an SSL connection is established, it will
** do the much faster session restart handshake.  This will change the
** session keys without doing another private key operation.
*/
SSL_IMPORT SECStatus SSL_ReHandshake(PRFileDesc *fd, PRBool flushCache);

/*
** Same as above, but with an I/O timeout.
 */
SSL_IMPORT SECStatus SSL_ReHandshakeWithTimeout(PRFileDesc *fd,
                                                PRBool flushCache,
                                                PRIntervalTime timeout);

#ifdef SSL_DEPRECATED_FUNCTION
/* deprecated!
** For the server, request a new handshake.  For the client, begin a new
** handshake.  Flushes SSL3 session cache entry first, ensuring that a
** full handshake will be done.
** This call is equivalent to SSL_ReHandshake(fd, PR_TRUE)
*/
SSL_IMPORT SECStatus SSL_RedoHandshake(PRFileDesc *fd);
#endif

/*
 * Allow the application to pass a URL or hostname into the SSL library.
 */
SSL_IMPORT SECStatus SSL_SetURL(PRFileDesc *fd, const char *url);

/*
 * Allow an application to define a set of trust anchors for peer
 * cert validation.
 */
SSL_IMPORT SECStatus SSL_SetTrustAnchors(PRFileDesc *fd, CERTCertList *list);

/*
** Return the number of bytes that SSL has waiting in internal buffers.
** Return 0 if security is not enabled.
*/
SSL_IMPORT int SSL_DataPending(PRFileDesc *fd);

/*
** Invalidate the SSL session associated with fd.
*/
SSL_IMPORT SECStatus SSL_InvalidateSession(PRFileDesc *fd);

/*
** Return a SECItem containing the SSL session ID associated with the fd.
*/
SSL_IMPORT SECItem *SSL_GetSessionID(PRFileDesc *fd);

/*
** Clear out the client's SSL session cache, not the server's session cache.
*/
SSL_IMPORT void SSL_ClearSessionCache(void);

/*
** Close the server's SSL session cache.
*/
SSL_IMPORT SECStatus SSL_ShutdownServerSessionIDCache(void);

/*
** Set peer information so we can correctly look up SSL session later.
** You only have to do this if you're tunneling through a proxy.
*/
SSL_IMPORT SECStatus SSL_SetSockPeerID(PRFileDesc *fd, const char *peerID);

/*
** Reveal the security information for the peer.
*/
SSL_IMPORT CERTCertificate *SSL_RevealCert(PRFileDesc *socket);
SSL_IMPORT void *SSL_RevealPinArg(PRFileDesc *socket);
SSL_IMPORT char *SSL_RevealURL(PRFileDesc *socket);

/* This callback may be passed to the SSL library via a call to
 * SSL_GetClientAuthDataHook() for each SSL client socket.
 * It will be invoked when SSL needs to know what certificate and private key
 * (if any) to use to respond to a request for client authentication.
 * If arg is non-NULL, it is a pointer to a NULL-terminated string containing
 * the nickname of the cert/key pair to use.
 * If arg is NULL, this function will search the cert and key databases for
 * a suitable match and send it if one is found.
 */
SSL_IMPORT SECStatus
NSS_GetClientAuthData(void *arg,
                      PRFileDesc *socket,
                      struct CERTDistNamesStr *caNames,
                      struct CERTCertificateStr **pRetCert,
                      struct SECKEYPrivateKeyStr **pRetKey);

/*
** Configure DTLS-SRTP (RFC 5764) cipher suite preferences.
** Input is a list of ciphers in descending preference order and a length
** of the list. As a side effect, this causes the use_srtp extension to be
** negotiated.
**
** Invalid or unimplemented cipher suites in |ciphers| are ignored. If at
** least one cipher suite in |ciphers| is implemented, returns SECSuccess.
** Otherwise returns SECFailure.
*/
SSL_IMPORT SECStatus SSL_SetSRTPCiphers(PRFileDesc *fd,
                                        const PRUint16 *ciphers,
                                        unsigned int numCiphers);

/*
** Get the selected DTLS-SRTP cipher suite (if any).
** To be called after the handshake completes.
** Returns SECFailure if not negotiated.
*/
SSL_IMPORT SECStatus SSL_GetSRTPCipher(PRFileDesc *fd,
                                       PRUint16 *cipher);

/*
 * Look to see if any of the signers in the cert chain for "cert" are found
 * in the list of caNames.
 * Returns SECSuccess if so, SECFailure if not.
 * Used by NSS_GetClientAuthData.  May be used by other callback functions.
 */
SSL_IMPORT SECStatus NSS_CmpCertChainWCANames(CERTCertificate *cert,
                                              CERTDistNames *caNames);

/* Deprecated.  This reports a misleading value for certificates that might
 * be used for signing rather than key exchange.
 * Returns key exchange type of the keys in an SSL server certificate.
 */
SSL_IMPORT SSLKEAType NSS_FindCertKEAType(CERTCertificate *cert);

/* Set cipher policies to a predefined Domestic (U.S.A.) policy.
 * This essentially allows all supported ciphers.
 */
SSL_IMPORT SECStatus NSS_SetDomesticPolicy(void);

/* Set cipher policies to a predefined Policy that is exportable from the USA
 *   according to present U.S. policies as we understand them.
 * It is the same as NSS_SetDomesticPolicy now.
 */
SSL_IMPORT SECStatus NSS_SetExportPolicy(void);

/* Set cipher policies to a predefined Policy that is exportable from the USA
 *   according to present U.S. policies as we understand them, and that the
 *   nation of France will permit to be imported into their country.
 * It is the same as NSS_SetDomesticPolicy now.
 */
SSL_IMPORT SECStatus NSS_SetFrancePolicy(void);

SSL_IMPORT SSL3Statistics *SSL_GetStatistics(void);

/* Report more information than SSL_SecurityStatus.
 * Caller supplies the info struct.  This function fills it in.  Caller should
 * pass sizeof(SSLChannelInfo) as the |len| argument.
 *
 * The information here will be zeroed prior to details being confirmed.  The
 * details are confirmed either when a Finished message is received, or - for a
 * client - when the second flight of messages have been sent.  This function
 * therefore produces unreliable results prior to receiving the
 * SSLHandshakeCallback or the SSLCanFalseStartCallback.
 */
SSL_IMPORT SECStatus SSL_GetChannelInfo(PRFileDesc *fd, SSLChannelInfo *info,
                                        PRUintn len);
/* Get preliminary information about a channel.
 * Caller supplies the info struct.  This function fills it in.  Caller should
 * pass sizeof(SSLPreliminaryChannelInfo) as the |len| argument.
 *
 * This function can be called prior to handshake details being confirmed (see
 * SSL_GetChannelInfo above for what that means).  Thus, information provided by
 * this function is available to SSLAuthCertificate, SSLGetClientAuthData,
 * SSLSNISocketConfig, and other callbacks that might be called during the
 * processing of the first flight of client of server handshake messages.
 * Values are marked as being unavailable when renegotiation is initiated.
 */
SSL_IMPORT SECStatus
SSL_GetPreliminaryChannelInfo(PRFileDesc *fd,
                              SSLPreliminaryChannelInfo *info,
                              PRUintn len);
/* Get information about cipher suite with id of |cipherSuite|.
 * Caller supplies the info struct.  This function fills it in.  Caller should
 * pass sizeof(SSLCipherSuiteInfo) as the |len| argument.
 */
SSL_IMPORT SECStatus SSL_GetCipherSuiteInfo(PRUint16 cipherSuite,
                                            SSLCipherSuiteInfo *info, PRUintn len);

/* Returnes negotiated through SNI host info. */
SSL_IMPORT SECItem *SSL_GetNegotiatedHostInfo(PRFileDesc *fd);

/* Export keying material according to RFC 5705.
** fd must correspond to a TLS 1.0 or higher socket and out must
** already be allocated. If hasContext is false, it uses the no-context
** construction from the RFC and ignores the context and contextLen
** arguments.
*/
SSL_IMPORT SECStatus SSL_ExportKeyingMaterial(PRFileDesc *fd,
                                              const char *label,
                                              unsigned int labelLen,
                                              PRBool hasContext,
                                              const unsigned char *context,
                                              unsigned int contextLen,
                                              unsigned char *out,
                                              unsigned int outLen);

/* Early exporters are used if 0-RTT is enabled.  This is TLS 1.3 only.  Note
 * that in TLS 1.3, an empty context is equivalent to an absent context. */
SSL_IMPORT SECStatus SSL_ExportEarlyKeyingMaterial(PRFileDesc *fd,
                                                   const char *label,
                                                   unsigned int labelLen,
                                                   const unsigned char *context,
                                                   unsigned int contextLen,
                                                   unsigned char *out,
                                                   unsigned int outLen);

/*
** Return a new reference to the certificate that was most recently sent
** to the peer on this SSL/TLS connection, or NULL if none has been sent.
*/
SSL_IMPORT CERTCertificate *SSL_LocalCertificate(PRFileDesc *fd);

#define SSL_CBP_SSL3 0x0001   /* (deprecated) */
#define SSL_CBP_TLS1_0 0x0002 /* (deprecated) */

/* DEPRECATED: The PKCS#11 bypass has been removed.
**             This function will now always return false. */
SSL_IMPORT SECStatus SSL_CanBypass(CERTCertificate *cert,
                                   SECKEYPrivateKey *privKey,
                                   PRUint32 protocolmask,
                                   PRUint16 *ciphers, int nciphers,
                                   PRBool *pcanbypass, void *pwArg);

/*
** Did the handshake with the peer negotiate the given extension?
** Output parameter valid only if function returns SECSuccess
*/
SSL_IMPORT SECStatus SSL_HandshakeNegotiatedExtension(PRFileDesc *socket,
                                                      SSLExtensionType extId,
                                                      PRBool *yes);

/*
** How long should we wait before retransmitting the next flight of
** the DTLS handshake? Returns SECFailure if not DTLS or not in a
** handshake.
*/
SSL_IMPORT SECStatus DTLS_GetHandshakeTimeout(PRFileDesc *socket,
                                              PRIntervalTime *timeout);

/*
 * Return a boolean that indicates whether the underlying library
 * will perform as the caller expects.
 *
 * The only argument is a string, which should be the version
 * identifier of the NSS library. That string will be compared
 * against a string that represents the actual build version of
 * the SSL library.
 */
extern PRBool NSSSSL_VersionCheck(const char *importedVersion);

/*
 * Returns a const string of the SSL library version.
 */
extern const char *NSSSSL_GetVersion(void);

/* Restart an SSL connection that was paused to do asynchronous certificate
 * chain validation (when the auth certificate hook or bad cert handler
 * returned SECWouldBlock).
 *
 * This function only works for non-blocking sockets; Do not use it for
 * blocking sockets. Currently, this function works only for the client role of
 * a connection; it does not work for the server role.
 *
 * The application must call SSL_AuthCertificateComplete with 0 as the value of
 * the error parameter after it has successfully validated the peer's
 * certificate, in order to continue the SSL handshake.
 *
 * The application may call SSL_AuthCertificateComplete with a non-zero value
 * for error (e.g. SEC_ERROR_REVOKED_CERTIFICATE) when certificate validation
 * fails, before it closes the connection. If the application does so, an
 * alert corresponding to the error (e.g. certificate_revoked) will be sent to
 * the peer. See the source code of the internal function
 * ssl3_SendAlertForCertError for the current mapping of error to alert. This
 * mapping may change in future versions of libssl.
 *
 * This function will not complete the entire handshake. The application must
 * call SSL_ForceHandshake, PR_Recv, PR_Send, etc. after calling this function
 * to force the handshake to complete.
 *
 * On the first handshake of a connection, libssl will wait for the peer's
 * certificate to be authenticated before calling the handshake callback,
 * sending a client certificate, sending any application data, or returning
 * any application data to the application. On subsequent (renegotiation)
 * handshakes, libssl will block the handshake unconditionally while the
 * certificate is being validated.
 *
 * libssl may send and receive handshake messages while waiting for the
 * application to call SSL_AuthCertificateComplete, and it may call other
 * callbacks (e.g, the client auth data hook) before
 * SSL_AuthCertificateComplete has been called.
 *
 * An application that uses this asynchronous mechanism will usually have lower
 * handshake latency if it has to do public key operations on the certificate
 * chain and/or CRL/OCSP/cert fetching during the authentication, especially if
 * it does so in parallel on another thread. However, if the application can
 * authenticate the peer's certificate quickly then it may be more efficient
 * to use the synchronous mechanism (i.e. returning SECFailure/SECSuccess
 * instead of SECWouldBlock from the authenticate certificate hook).
 *
 * Be careful about converting an application from synchronous cert validation
 * to asynchronous certificate validation. A naive conversion is likely to
 * result in deadlocks; e.g. the application will wait in PR_Poll for network
 * I/O on the connection while all network I/O on the connection is blocked
 * waiting for this function to be called.
 *
 * Returns SECFailure on failure, SECSuccess on success. Never returns
 * SECWouldBlock. Note that SSL_AuthCertificateComplete will (usually) return
 * SECSuccess; do not interpret the return value of SSL_AuthCertificateComplete
 * as an indicator of whether it is OK to continue using the connection. For
 * example, SSL_AuthCertificateComplete(fd, SEC_ERROR_REVOKED_CERTIFICATE) will
 * return SECSuccess (normally), but that does not mean that the application
 * should continue using the connection. If the application passes a non-zero
 * value for second argument (error), or if SSL_AuthCertificateComplete returns
 * anything other than SECSuccess, then the application should close the
 * connection.
 */
SSL_IMPORT SECStatus SSL_AuthCertificateComplete(PRFileDesc *fd,
                                                 PRErrorCode error);

/*
 * This is used to access experimental APIs.  Don't call this directly.  This is
 * used to enable the experimental APIs that are defined in "sslexp.h".
 */
SSL_IMPORT void *SSL_GetExperimentalAPI(const char *name);

SEC_END_PROTOS

#endif /* __ssl_h_ */
