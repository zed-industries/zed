/*
 * srtp.h
 *
 * interface to libsrtp
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 */
/*
 *
 * Copyright (c) 2001-2017, Cisco Systems, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 *   Redistributions in binary form must reproduce the above
 *   copyright notice, this list of conditions and the following
 *   disclaimer in the documentation and/or other materials provided
 *   with the distribution.
 *
 *   Neither the name of the Cisco Systems, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef SRTP_SRTP_H
#define SRTP_SRTP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @defgroup SRTP Secure RTP
 *
 * @brief libSRTP provides functions for protecting RTP and RTCP.  See
 * Section @ref Overview for an introduction to the use of the library.
 *
 * @{
 */

/*
 * SRTP_MASTER_KEY_LEN is the nominal master key length supported by libSRTP
 */

#define SRTP_MASTER_KEY_LEN 30

/*
 * SRTP_MAX_KEY_LEN is the maximum key length supported by libSRTP
 */
#define SRTP_MAX_KEY_LEN 64

/*
 * SRTP_MAX_TAG_LEN is the maximum tag length supported by libSRTP
 */

#define SRTP_MAX_TAG_LEN 16

/**
 * SRTP_MAX_MKI_LEN is the maximum size the MKI could be which is
 * 128 bytes
 */
#define SRTP_MAX_MKI_LEN 128

/**
 * SRTP_MAX_TRAILER_LEN is the maximum length of the SRTP trailer
 * (authentication tag and MKI) supported by libSRTP.  This value is
 * the maximum number of octets that will be added to an RTP packet by
 * srtp_protect().
 *
 * @brief the maximum number of octets added by srtp_protect().
 */
#define SRTP_MAX_TRAILER_LEN (SRTP_MAX_TAG_LEN + SRTP_MAX_MKI_LEN)

/**
 * SRTP_SRCTP_INDEX_LEN is the size the SRTCP index which is
 * 4 bytes
 */
#define SRTP_SRCTP_INDEX_LEN 4

/**
 * SRTP_MAX_SRTCP_TRAILER_LEN is the maximum length of the SRTCP trailer
 * (index, authentication tag and MKI) supported by libSRTP.  This value is
 * the maximum number of octets that will be added to an RTCP packet by
 * srtp_protect_rtcp().
 *
 * @brief the maximum number of octets added by srtp_protect().
 */
#define SRTP_MAX_SRTCP_TRAILER_LEN                                             \
    (SRTP_SRCTP_INDEX_LEN + SRTP_MAX_TAG_LEN + SRTP_MAX_MKI_LEN)

/**
 * SRTP_MAX_NUM_MASTER_KEYS is the maximum number of Master keys for
 * MKI supported by libSRTP.
 *
 */
#define SRTP_MAX_NUM_MASTER_KEYS 16

#define SRTP_SALT_LEN 14

/*
 * SRTP_AEAD_SALT_LEN is the length of the SALT values used with
 * GCM mode.  GCM mode requires an IV.  The SALT value is used
 * as part of the IV formation logic applied to each RTP packet.
 */
#define SRTP_AEAD_SALT_LEN 12

#define SRTP_AES_128_KEY_LEN 16
#define SRTP_AES_192_KEY_LEN 24
#define SRTP_AES_256_KEY_LEN 32

#define SRTP_AES_ICM_128_KEY_LEN_WSALT (SRTP_SALT_LEN + SRTP_AES_128_KEY_LEN)
#define SRTP_AES_ICM_192_KEY_LEN_WSALT (SRTP_SALT_LEN + SRTP_AES_192_KEY_LEN)
#define SRTP_AES_ICM_256_KEY_LEN_WSALT (SRTP_SALT_LEN + SRTP_AES_256_KEY_LEN)

#define SRTP_AES_GCM_128_KEY_LEN_WSALT                                         \
    (SRTP_AEAD_SALT_LEN + SRTP_AES_128_KEY_LEN)
#define SRTP_AES_GCM_192_KEY_LEN_WSALT                                         \
    (SRTP_AEAD_SALT_LEN + SRTP_AES_192_KEY_LEN)
#define SRTP_AES_GCM_256_KEY_LEN_WSALT                                         \
    (SRTP_AEAD_SALT_LEN + SRTP_AES_256_KEY_LEN)

/**
 *  @brief A srtp_cipher_type_id_t is an identifier for a particular cipher
 *  type.
 *
 *  A srtp_cipher_type_id_t is an integer that represents a particular
 *  cipher type, e.g. the Advanced Encryption Standard (AES).  A
 *  SRTP_NULL_CIPHER is available; this cipher leaves the data unchanged,
 *  and can be selected to indicate that no encryption is to take
 *  place.
 *
 *  @ingroup Ciphers
 */
typedef uint32_t srtp_cipher_type_id_t;

/**
 *  @brief An srtp_auth_type_id_t is an identifier for a particular
 * authentication
 *   function.
 *
 *  An srtp_auth_type_id_t is an integer that represents a particular
 *  authentication function type, e.g. HMAC-SHA1.  A SRTP_NULL_AUTH is
 *  available; this authentication function performs no computation,
 *  and can be selected to indicate that no authentication is to take
 *  place.
 *
 *  @ingroup Authentication
 */
typedef uint32_t srtp_auth_type_id_t;

/**
 * @brief srtp_err_status_t defines error codes.
 *
 * The enumeration srtp_err_status_t defines error codes.  Note that the
 * value of srtp_err_status_ok is equal to zero, which can simplify error
 * checking somewhat.
 *
 */
typedef enum {
    srtp_err_status_ok = 0,             /**< nothing to report               */
    srtp_err_status_fail = 1,           /**< unspecified failure             */
    srtp_err_status_bad_param = 2,      /**< unsupported parameter           */
    srtp_err_status_alloc_fail = 3,     /**< couldn't allocate memory        */
    srtp_err_status_dealloc_fail = 4,   /**< couldn't deallocate properly    */
    srtp_err_status_init_fail = 5,      /**< couldn't initialize             */
    srtp_err_status_terminus = 6,       /**< can't process as much data as   */
                                        /**< requested                       */
    srtp_err_status_auth_fail = 7,      /**< authentication failure          */
    srtp_err_status_cipher_fail = 8,    /**< cipher failure                  */
    srtp_err_status_replay_fail = 9,    /**< replay check failed (bad index) */
    srtp_err_status_replay_old = 10,    /**< replay check failed (index too  */
                                        /**< old)                            */
    srtp_err_status_algo_fail = 11,     /**< algorithm failed test routine   */
    srtp_err_status_no_such_op = 12,    /**< unsupported operation           */
    srtp_err_status_no_ctx = 13,        /**< no appropriate context found    */
    srtp_err_status_cant_check = 14,    /**< unable to perform desired       */
                                        /**< validation                      */
    srtp_err_status_key_expired = 15,   /**< can't use key any more          */
    srtp_err_status_socket_err = 16,    /**< error in use of socket          */
    srtp_err_status_signal_err = 17,    /**< error in use POSIX signals      */
    srtp_err_status_nonce_bad = 18,     /**< nonce check failed              */
    srtp_err_status_read_fail = 19,     /**< couldn't read data              */
    srtp_err_status_write_fail = 20,    /**< couldn't write data             */
    srtp_err_status_parse_err = 21,     /**< error parsing data              */
    srtp_err_status_encode_err = 22,    /**< error encoding data             */
    srtp_err_status_semaphore_err = 23, /**< error while using semaphores    */
    srtp_err_status_pfkey_err = 24,     /**< error while using pfkey         */
    srtp_err_status_bad_mki = 25,       /**< error MKI present in packet is  */
                                        /**< invalid                         */
    srtp_err_status_pkt_idx_old = 26,   /**< packet index is too old to      */
                                        /**< consider                        */
    srtp_err_status_pkt_idx_adv = 27    /**< packet index advanced, reset    */
                                        /**< needed                          */
} srtp_err_status_t;

typedef struct srtp_ctx_t_ srtp_ctx_t;

/**
 * @brief srtp_sec_serv_t describes a set of security services.
 *
 * A srtp_sec_serv_t enumeration is used to describe the particular
 * security services that will be applied by a particular crypto
 * policy (or other mechanism).
 */
typedef enum {
    sec_serv_none = 0,         /**< no services                        */
    sec_serv_conf = 1,         /**< confidentiality                    */
    sec_serv_auth = 2,         /**< authentication                     */
    sec_serv_conf_and_auth = 3 /**< confidentiality and authentication */
} srtp_sec_serv_t;

/**
 * @brief srtp_crypto_policy_t describes a particular crypto policy that
 * can be applied to an SRTP stream.
 *
 * A srtp_crypto_policy_t describes a particular cryptographic policy that
 * can be applied to an SRTP or SRTCP stream.  An SRTP session policy
 * consists of a list of these policies, one for each SRTP stream
 * in the session.
 */
typedef struct srtp_crypto_policy_t {
    srtp_cipher_type_id_t cipher_type; /**< An integer representing          */
                                       /**< the type of cipher.              */
    int cipher_key_len;                /**< The length of the cipher key     */
                                       /**< in octets.                       */
    srtp_auth_type_id_t auth_type;     /**< An integer representing the      */
                                       /**< authentication function.         */
    int auth_key_len;                  /**< The length of the authentication */
                                       /**< function key in octets.          */
    int auth_tag_len;                  /**< The length of the authentication */
                                       /**< tag in octets.                   */
    srtp_sec_serv_t sec_serv;          /**< The flag indicating the security */
                                       /**< services to be applied.          */
} srtp_crypto_policy_t;

/**
 * @brief srtp_ssrc_type_t describes the type of an SSRC.
 *
 * An srtp_ssrc_type_t enumeration is used to indicate a type of SSRC.  See
 * @ref srtp_policy_t for more information.
 */
typedef enum {
    ssrc_undefined = 0,   /**< Indicates an undefined SSRC type.    */
    ssrc_specific = 1,    /**< Indicates a specific SSRC value      */
    ssrc_any_inbound = 2, /**< Indicates any inbound SSRC value     */
                          /**< (i.e. a value that is used in the    */
                          /**< function srtp_unprotect())           */
    ssrc_any_outbound = 3 /**< Indicates any outbound SSRC value    */
                          /**< (i.e. a value that is used in the    */
                          /**< function srtp_protect())             */
} srtp_ssrc_type_t;

/**
 * @brief An srtp_ssrc_t represents a particular SSRC value, or a `wildcard'
 * SSRC.
 *
 * An srtp_ssrc_t represents a particular SSRC value (if its type is
 * ssrc_specific), or a wildcard SSRC value that will match all
 * outbound SSRCs (if its type is ssrc_any_outbound) or all inbound
 * SSRCs (if its type is ssrc_any_inbound).
 */
typedef struct {
    srtp_ssrc_type_t type; /**< The type of this particular SSRC */
    unsigned int value;    /**< The value of this SSRC, if it is not a */
                           /**< wildcard */
} srtp_ssrc_t;

/**
 * @brief srtp_master_key_t represents a master key.  There will
 * be a Master Key Index and the Master Key associated with the
 * Master Key Index.  Need to also keep track of the Master Key
 * Index Size to correctly read it from a packet.
 */
typedef struct srtp_master_key_t {
    unsigned char *key;
    unsigned char *mki_id;
    unsigned int mki_size;
} srtp_master_key_t;

/**
 * @brief represents the policy for an SRTP session.
 *
 * A single srtp_policy_t struct represents the policy for a single
 * SRTP stream, and a linked list of these elements represents the
 * policy for an entire SRTP session.  Each element contains the SRTP
 * and SRTCP crypto policies for that stream, a pointer to the SRTP
 * master key for that stream, the SSRC describing that stream, or a
 * flag indicating a `wildcard' SSRC value, and a `next' field that
 * holds a pointer to the next element in the list of policy elements,
 * or NULL if it is the last element.
 *
 * The wildcard value SSRC_ANY_INBOUND matches any SSRC from an
 * inbound stream that for which there is no explicit SSRC entry in
 * another policy element.  Similarly, the value SSRC_ANY_OUTBOUND
 * will matches any SSRC from an outbound stream that does not appear
 * in another policy element.  Note that wildcard SSRCs &b cannot be
 * used to match both inbound and outbound traffic.  This restriction
 * is intentional, and it allows libSRTP to ensure that no security
 * lapses result from accidental re-use of SSRC values during key
 * sharing.
 *
 * @warning The final element of the list @b must have its `next' pointer
 *          set to NULL.
 */

typedef struct srtp_policy_t {
    srtp_ssrc_t ssrc;              /**< The SSRC value of stream, or the    */
                                   /**< flags SSRC_ANY_INBOUND or           */
                                   /**< SSRC_ANY_OUTBOUND if key sharing    */
                                   /**< is used for this policy element.    */
    srtp_crypto_policy_t rtp;      /**< SRTP crypto policy.                 */
    srtp_crypto_policy_t rtcp;     /**< SRTCP crypto policy.                */
    unsigned char *key;            /**< Pointer to the SRTP master key for  */
                                   /**< this stream.                        */
    srtp_master_key_t **keys;      /** Array of Master Key structures       */
    unsigned long num_master_keys; /** Number of master keys                */
    void *deprecated_ekt;          /**< DEPRECATED: pointer to the EKT      */
                                   /**< policy structure for this stream    */
    unsigned long window_size;     /**< The window size to use for replay   */
                                   /**< protection.                         */
    int allow_repeat_tx;           /**< Whether retransmissions of          */
                                   /**< packets with the same sequence      */
                                   /**< number are allowed.                 */
                                   /**< (Note that such repeated            */
                                   /**< transmissions must have the same    */
                                   /**< RTP payload, or a severe security   */
                                   /**< weakness is introduced!)            */
    int *enc_xtn_hdr;              /**< List of header ids to encrypt.      */
    int enc_xtn_hdr_count;         /**< Number of entries in list of header */
                                   /**<  ids.                               */
    struct srtp_policy_t *next;    /**< Pointer to next stream policy.      */
} srtp_policy_t;

/**
 * @brief An srtp_t points to an SRTP session structure.
 *
 * The typedef srtp_t is a pointer to a structure that represents
 * an SRTP session.  This datatype is intentionally opaque in
 * order to separate the interface from the implementation.
 *
 * An SRTP session consists of all of the traffic sent to the RTP and
 * RTCP destination transport addresses, using the RTP/SAVP (Secure
 * Audio/Video Profile).  A session can be viewed as a set of SRTP
 * streams, each of which originates with a different participant.
 */
typedef srtp_ctx_t *srtp_t;

/**
 * @brief srtp_init() initializes the srtp library.
 *
 * @warning This function @b must be called before any other srtp
 * functions.
 */
srtp_err_status_t srtp_init(void);

/**
 * @brief srtp_shutdown() de-initializes the srtp library.
 *
 * @warning No srtp functions may be called after calling this function.
 */
srtp_err_status_t srtp_shutdown(void);

/**
 * @brief srtp_protect() is the Secure RTP sender-side packet processing
 * function.
 *
 * The function call srtp_protect(ctx, rtp_hdr, len_ptr) applies SRTP
 * protection to the RTP packet rtp_hdr (which has length *len_ptr) using
 * the SRTP context ctx.  If srtp_err_status_ok is returned, then rtp_hdr
 * points to the resulting SRTP packet and *len_ptr is the number of
 * octets in that packet; otherwise, no assumptions should be made
 * about the value of either data elements.
 *
 * The sequence numbers of the RTP packets presented to this function
 * need not be consecutive, but they @b must be out of order by less
 * than 2^15 = 32,768 packets.
 *
 * @warning This function assumes that it can write the authentication
 * tag into the location in memory immediately following the RTP
 * packet, and assumes that the RTP packet is aligned on a 32-bit
 * boundary.
 *
 * @warning This function assumes that it can write SRTP_MAX_TRAILER_LEN
 * into the location in memory immediately following the RTP packet.
 * Callers MUST ensure that this much writable memory is available in
 * the buffer that holds the RTP packet.
 *
 * @param ctx is the SRTP context to use in processing the packet.
 *
 * @param rtp_hdr is a pointer to the RTP packet (before the call); after
 * the function returns, it points to the srtp packet.
 *
 * @param len_ptr is a pointer to the length in octets of the complete
 * RTP packet (header and body) before the function call, and of the
 * complete SRTP packet after the call, if srtp_err_status_ok was returned.
 * Otherwise, the value of the data to which it points is undefined.
 *
 * @return
 *    - srtp_err_status_ok            no problems
 *    - srtp_err_status_replay_fail   rtp sequence number was non-increasing
 *    - @e other                 failure in cryptographic mechanisms
 */
srtp_err_status_t srtp_protect(srtp_t ctx, void *rtp_hdr, int *len_ptr);

/**
 * @brief srtp_protect_mki() is the Secure RTP sender-side packet processing
 * function that can utilize MKI.
 *
 * The function call srtp_protect(ctx, rtp_hdr, len_ptr) applies SRTP
 * protection to the RTP packet rtp_hdr (which has length *len_ptr) using
 * the SRTP context ctx.  If srtp_err_status_ok is returned, then rtp_hdr
 * points to the resulting SRTP packet and *len_ptr is the number of
 * octets in that packet; otherwise, no assumptions should be made
 * about the value of either data elements.
 *
 * The sequence numbers of the RTP packets presented to this function
 * need not be consecutive, but they @b must be out of order by less
 * than 2^15 = 32,768 packets.
 *
 * @warning This function assumes that it can write the authentication
 * tag into the location in memory immediately following the RTP
 * packet, and assumes that the RTP packet is aligned on a 32-bit
 * boundary.
 *
 * @warning This function assumes that it can write SRTP_MAX_TRAILER_LEN
 * into the location in memory immediately following the RTP packet.
 * Callers MUST ensure that this much writable memory is available in
 * the buffer that holds the RTP packet.
 *
 * @param ctx is the SRTP context to use in processing the packet.
 *
 * @param rtp_hdr is a pointer to the RTP packet (before the call); after
 * the function returns, it points to the srtp packet.
 *
 * @param pkt_octet_len is a pointer to the length in octets of the complete
 * RTP packet (header and body) before the function call, and of the
 * complete SRTP packet after the call, if srtp_err_status_ok was returned.
 * Otherwise, the value of the data to which it points is undefined.
 *
 * @param use_mki is a boolean to tell the system if mki is being used.  If
 * set to false then will use the first set of session keys.  If set to true
 * will
 * use the session keys identified by the mki_index
 *
 * @param mki_index integer value specifying which set of session keys should be
 * used if use_mki is set to true.
 *
 * @return
 *    - srtp_err_status_ok            no problems
 *    - srtp_err_status_replay_fail   rtp sequence number was non-increasing
 *    - @e other                 failure in cryptographic mechanisms
 */
srtp_err_status_t srtp_protect_mki(srtp_ctx_t *ctx,
                                   void *rtp_hdr,
                                   int *pkt_octet_len,
                                   unsigned int use_mki,
                                   unsigned int mki_index);

/**
 * @brief srtp_unprotect() is the Secure RTP receiver-side packet
 * processing function.
 *
 * The function call srtp_unprotect(ctx, srtp_hdr, len_ptr) verifies
 * the Secure RTP protection of the SRTP packet pointed to by srtp_hdr
 * (which has length *len_ptr), using the SRTP context ctx.  If
 * srtp_err_status_ok is returned, then srtp_hdr points to the resulting
 * RTP packet and *len_ptr is the number of octets in that packet;
 * otherwise, no assumptions should be made about the value of either
 * data elements.
 *
 * The sequence numbers of the RTP packets presented to this function
 * need not be consecutive, but they @b must be out of order by less
 * than 2^15 = 32,768 packets.
 *
 * @warning This function assumes that the SRTP packet is aligned on a
 * 32-bit boundary.
 *
 * @param ctx is the SRTP session which applies to the particular packet.
 *
 * @param srtp_hdr is a pointer to the header of the SRTP packet
 * (before the call).  after the function returns, it points to the
 * rtp packet if srtp_err_status_ok was returned; otherwise, the value of
 * the data to which it points is undefined.
 *
 * @param len_ptr is a pointer to the length in octets of the complete
 * srtp packet (header and body) before the function call, and of the
 * complete rtp packet after the call, if srtp_err_status_ok was returned.
 * Otherwise, the value of the data to which it points is undefined.
 *
 * @return
 *    - srtp_err_status_ok          if the RTP packet is valid.
 *    - srtp_err_status_auth_fail   if the SRTP packet failed the message
 *                                  authentication check.
 *    - srtp_err_status_replay_fail if the SRTP packet is a replay (e.g. packet
 *                                  has already been processed and accepted).
 *    - [other]  if there has been an error in the cryptographic mechanisms.
 *
 */
srtp_err_status_t srtp_unprotect(srtp_t ctx, void *srtp_hdr, int *len_ptr);

/**
 * @brief srtp_unprotect_mki() is the Secure RTP receiver-side packet
 * processing function that checks for MKI.
 *
 * The function call srtp_unprotect(ctx, srtp_hdr, len_ptr) verifies
 * the Secure RTP protection of the SRTP packet pointed to by srtp_hdr
 * (which has length *len_ptr), using the SRTP context ctx.  If
 * srtp_err_status_ok is returned, then srtp_hdr points to the resulting
 * RTP packet and *len_ptr is the number of octets in that packet;
 * otherwise, no assumptions should be made about the value of either
 * data elements.
 *
 * The sequence numbers of the RTP packets presented to this function
 * need not be consecutive, but they @b must be out of order by less
 * than 2^15 = 32,768 packets.
 *
 * @warning This function assumes that the SRTP packet is aligned on a
 * 32-bit boundary.
 *
 * @param ctx is the SRTP session which applies to the particular packet.
 *
 * @param srtp_hdr is a pointer to the header of the SRTP packet
 * (before the call).  after the function returns, it points to the
 * rtp packet if srtp_err_status_ok was returned; otherwise, the value of
 * the data to which it points is undefined.
 *
 * @param len_ptr is a pointer to the length in octets of the complete
 * srtp packet (header and body) before the function call, and of the
 * complete rtp packet after the call, if srtp_err_status_ok was returned.
 * Otherwise, the value of the data to which it points is undefined.
 *
 * @param use_mki is a boolean to tell the system if mki is being used.  If
 * set to false then will use the first set of session keys.  If set to true
 * will
 * use the session keys identified by the mki_index
 *
 * @return
 *    - srtp_err_status_ok          if the RTP packet is valid.
 *    - srtp_err_status_auth_fail   if the SRTP packet failed the message
 *                                  authentication check.
 *    - srtp_err_status_replay_fail if the SRTP packet is a replay (e.g. packet
 *                                  has already been processed and accepted).
 *    - srtp_err_status_bad_mki if the MKI in the packet is not a known MKI id
 *    - [other]  if there has been an error in the cryptographic mechanisms.
 *
 */
srtp_err_status_t srtp_unprotect_mki(srtp_t ctx,
                                     void *srtp_hdr,
                                     int *len_ptr,
                                     unsigned int use_mki);

/**
 * @brief srtp_create() allocates and initializes an SRTP session.

 * The function call srtp_create(session, policy) allocates and
 * initializes an SRTP session context, applying the given policy.
 *
 * @param session is a pointer to the SRTP session to which the policy is
 * to be added.
 *
 * @param policy is the srtp_policy_t struct that describes the policy
 * for the session.  The struct may be a single element, or it may be
 * the head of a list, in which case each element of the list is
 * processed.  It may also be NULL, in which case streams should be added
 * later using srtp_add_stream().  The final element of the list @b must
 * have its `next' field set to NULL.
 *
 * @return
 *    - srtp_err_status_ok           if creation succeeded.
 *    - srtp_err_status_alloc_fail   if allocation failed.
 *    - srtp_err_status_init_fail    if initialization failed.
 */
srtp_err_status_t srtp_create(srtp_t *session, const srtp_policy_t *policy);

/**
 * @brief srtp_add_stream() allocates and initializes an SRTP stream
 * within a given SRTP session.
 *
 * The function call srtp_add_stream(session, policy) allocates and
 * initializes a new SRTP stream within a given, previously created
 * session, applying the policy given as the other argument to that
 * stream.
 *
 * @return values:
 *    - srtp_err_status_ok           if stream creation succeeded.
 *    - srtp_err_status_alloc_fail   if stream allocation failed
 *    - srtp_err_status_init_fail    if stream initialization failed.
 */
srtp_err_status_t srtp_add_stream(srtp_t session, const srtp_policy_t *policy);

/**
 * @brief srtp_remove_stream() deallocates an SRTP stream.
 *
 * The function call srtp_remove_stream(session, ssrc) removes
 * the SRTP stream with the SSRC value ssrc from the SRTP session
 * context given by the argument session.
 *
 * @param session is the SRTP session from which the stream
 *        will be removed.
 *
 * @param ssrc is the SSRC value of the stream to be removed
 *             in network byte order.
 *
 * @warning Wildcard SSRC values cannot be removed from a
 *          session.
 *
 * @return
 *    - srtp_err_status_ok     if the stream deallocation succeeded.
 *    - [other]           otherwise.
 *
 */
srtp_err_status_t srtp_remove_stream(srtp_t session, unsigned int ssrc);

/**
 * @brief srtp_update() updates all streams in the session.
 *
 * The function call srtp_update(session, policy) updates
 * all the streams in the session applying the given policy
 * and key. The existing ROC value of all streams will be
 * preserved.
 *
 * @param session is the SRTP session that contains the streams
 *        to be updated.
 *
 * @param policy is the srtp_policy_t struct that describes the policy
 * for the session.  The struct may be a single element, or it may be
 * the head of a list, in which case each element of the list is
 * processed. The final element of the list @b must
 * have its `next' field set to NULL.
 *
 * @return
 *    - srtp_err_status_ok           if stream creation succeed.
 *    - srtp_err_status_alloc_fail   if stream allocation failed
 *    - srtp_err_status_init_fail    if stream initialization failed.
 *    - [other]                 otherwise.
 *
 */
srtp_err_status_t srtp_update(srtp_t session, const srtp_policy_t *policy);

/**
 * @brief srtp_update_stream() updates a SRTP stream.
 *
 * The function call srtp_update_stream(session, policy) updates
 * the stream(s) in the session that match applying the given
 * policy and key. The existing ROC value of all stream(s) will
 * be preserved.
 *
 * @param session is the SRTP session that contains the streams
 *        to be updated.
 *
 * @param policy is the srtp_policy_t struct that describes the policy
 * for the session.
 *
 * @return
 *    - srtp_err_status_ok           if stream creation succeeded.
 *    - srtp_err_status_alloc_fail   if stream allocation failed
 *    - srtp_err_status_init_fail    if stream initialization failed.
 *    - [other]                      otherwise.
 *
 */
srtp_err_status_t srtp_update_stream(srtp_t session,
                                     const srtp_policy_t *policy);

/**
 * @brief srtp_crypto_policy_set_rtp_default() sets a crypto policy
 * structure to the SRTP default policy for RTP protection.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_rtp_default(&p) sets the
 * srtp_crypto_policy_t at location p to the SRTP default policy for RTP
 * protection, as defined in the specification.  This function is a
 * convenience that helps to avoid dealing directly with the policy
 * data structure.  You are encouraged to initialize policy elements
 * with this function call.  Doing so may allow your code to be
 * forward compatible with later versions of libSRTP that include more
 * elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_rtp_default(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_rtcp_default() sets a crypto policy
 * structure to the SRTP default policy for RTCP protection.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_rtcp_default(&p) sets the
 * srtp_crypto_policy_t at location p to the SRTP default policy for RTCP
 * protection, as defined in the specification.  This function is a
 * convenience that helps to avoid dealing directly with the policy
 * data structure.  You are encouraged to initialize policy elements
 * with this function call.  Doing so may allow your code to be
 * forward compatible with later versions of libSRTP that include more
 * elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_rtcp_default(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_128_hmac_sha1_80() sets a crypto
 * policy structure to the SRTP default policy for RTP protection.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function srtp_crypto_policy_set_aes_cm_128_hmac_sha1_80() is a
 * synonym for srtp_crypto_policy_set_rtp_default().  It conforms to the
 * naming convention used in RFC 4568 (SDP Security Descriptions for
 * Media Streams).
 *
 * @return void.
 *
 */
#define srtp_crypto_policy_set_aes_cm_128_hmac_sha1_80(p)                      \
    srtp_crypto_policy_set_rtp_default(p)

/**
 * @brief srtp_crypto_policy_set_aes_cm_128_hmac_sha1_32() sets a crypto
 * policy structure to a short-authentication tag policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_128_hmac_sha1_32(&p)
 * sets the srtp_crypto_policy_t at location p to use policy
 * AES_CM_128_HMAC_SHA1_32 as defined in RFC 4568.
 * This policy uses AES-128
 * Counter Mode encryption and HMAC-SHA1 authentication, with an
 * authentication tag that is only 32 bits long.  This length is
 * considered adequate only for protecting audio and video media that
 * use a stateless playback function.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This crypto policy is intended for use in SRTP, but not in
 * SRTCP.  It is recommended that a policy that uses longer
 * authentication tags be used for SRTCP.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_128_hmac_sha1_32(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_128_null_auth() sets a crypto
 * policy structure to an encryption-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_128_null_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-128 Counter Mode), but to use no authentication method.  This
 * policy is NOT RECOMMENDED unless it is unavoidable; see Section 7.5
 * of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This policy is NOT RECOMMENDED for SRTP unless it is
 * unavoidable, and it is NOT RECOMMENDED at all for SRTCP; see
 * Section 7.5 of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_128_null_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_null_cipher_hmac_sha1_80() sets a crypto
 * policy structure to an authentication-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_null_cipher_hmac_sha1_80(&p)
 * sets the srtp_crypto_policy_t at location p to use HMAC-SHA1 with an 80
 * bit authentication tag to provide message authentication, but to
 * use no encryption.  This policy is NOT RECOMMENDED for SRTP unless
 * there is a requirement to forgo encryption.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This policy is NOT RECOMMENDED for SRTP unless there is a
 * requirement to forgo encryption.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_null_cipher_hmac_sha1_80(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_null_cipher_hmac_null() sets a crypto
 * policy structure to use no encryption or authentication.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_null_cipher_hmac_null(&p)
 * sets the srtp_crypto_policy_t at location p to use no encryption and
 * no authentication.  This policy should only be used for testing and
 * troubleshooting.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This policy is NOT RECOMMENDED for SRTP unless there is a
 * requirement to forgo encryption and authentication.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_null_cipher_hmac_null(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_256_hmac_sha1_80() sets a crypto
 * policy structure to a encryption and authentication policy using AES-256
 * for RTP protection.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_256_hmac_sha1_80(&p)
 * sets the srtp_crypto_policy_t at location p to use policy
 * AES_CM_256_HMAC_SHA1_80 as defined in RFC 6188.  This policy uses AES-256
 * Counter Mode encryption and HMAC-SHA1 authentication, with an 80 bit
 * authentication tag.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_256_hmac_sha1_80(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_256_hmac_sha1_32() sets a crypto
 * policy structure to a short-authentication tag policy using AES-256
 * encryption.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_256_hmac_sha1_32(&p)
 * sets the srtp_crypto_policy_t at location p to use policy
 * AES_CM_256_HMAC_SHA1_32 as defined in RFC 6188.  This policy uses AES-256
 * Counter Mode encryption and HMAC-SHA1 authentication, with an
 * authentication tag that is only 32 bits long.  This length is
 * considered adequate only for protecting audio and video media that
 * use a stateless playback function.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This crypto policy is intended for use in SRTP, but not in
 * SRTCP.  It is recommended that a policy that uses longer
 * authentication tags be used for SRTCP.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_256_hmac_sha1_32(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_256_null_auth() sets a crypto
 * policy structure to an encryption-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_256_null_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-256 Counter Mode), but to use no authentication method.  This
 * policy is NOT RECOMMENDED unless it is unavoidable; see Section 7.5
 * of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This policy is NOT RECOMMENDED for SRTP unless it is
 * unavoidable, and it is NOT RECOMMENDED at all for SRTCP; see
 * Section 7.5 of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_256_null_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_192_hmac_sha1_80() sets a crypto
 * policy structure to a encryption and authentication policy using AES-192
 * for RTP protection.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_192_hmac_sha1_80(&p)
 * sets the srtp_crypto_policy_t at location p to use policy
 * AES_CM_192_HMAC_SHA1_80 as defined in RFC 6188.  This policy uses AES-192
 * Counter Mode encryption and HMAC-SHA1 authentication, with an 80 bit
 * authentication tag.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_192_hmac_sha1_80(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_192_hmac_sha1_32() sets a crypto
 * policy structure to a short-authentication tag policy using AES-192
 * encryption.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_192_hmac_sha1_32(&p)
 * sets the srtp_crypto_policy_t at location p to use policy
 * AES_CM_192_HMAC_SHA1_32 as defined in RFC 6188.  This policy uses AES-192
 * Counter Mode encryption and HMAC-SHA1 authentication, with an
 * authentication tag that is only 32 bits long.  This length is
 * considered adequate only for protecting audio and video media that
 * use a stateless playback function.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This crypto policy is intended for use in SRTP, but not in
 * SRTCP.  It is recommended that a policy that uses longer
 * authentication tags be used for SRTCP.  See Section 7.5 of RFC 3711
 * (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_192_hmac_sha1_32(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_cm_192_null_auth() sets a crypto
 * policy structure to an encryption-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_cm_192_null_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-192 Counter Mode), but to use no authentication method.  This
 * policy is NOT RECOMMENDED unless it is unavoidable; see Section 7.5
 * of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @warning This policy is NOT RECOMMENDED for SRTP unless it is
 * unavoidable, and it is NOT RECOMMENDED at all for SRTCP; see
 * Section 7.5 of RFC 3711 (http://www.ietf.org/rfc/rfc3711.txt).
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_cm_192_null_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_128_8_auth() sets a crypto
 * policy structure to an AEAD encryption policy.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_128_8_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-128 Galois Counter Mode) with 8 octet auth tag.  This
 * policy applies confidentiality and authentication to both the
 * RTP and RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_128_8_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_256_8_auth() sets a crypto
 * policy structure to an AEAD encryption policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_256_8_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-256 Galois Counter Mode) with 8 octet auth tag.  This
 * policy applies confidentiality and authentication to both the
 * RTP and RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_256_8_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_128_8_only_auth() sets a crypto
 * policy structure to an AEAD authentication-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_128_8_only_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-128 Galois Counter Mode) with 8 octet auth tag.  This policy
 * applies confidentiality and authentication to the RTP packets,
 * but only authentication to the RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_128_8_only_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_256_8_only_auth() sets a crypto
 * policy structure to an AEAD authentication-only policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_256_8_only_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-256 Galois Counter Mode) with 8 octet auth tag.  This policy
 * applies confidentiality and authentication to the RTP packets,
 * but only authentication to the RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_256_8_only_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_128_16_auth() sets a crypto
 * policy structure to an AEAD encryption policy.
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_128_16_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-128 Galois Counter Mode) with 16 octet auth tag.  This
 * policy applies confidentiality and authentication to both the
 * RTP and RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_128_16_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_crypto_policy_set_aes_gcm_256_16_auth() sets a crypto
 * policy structure to an AEAD encryption policy
 *
 * @param p is a pointer to the policy structure to be set
 *
 * The function call srtp_crypto_policy_set_aes_gcm_256_16_auth(&p) sets
 * the srtp_crypto_policy_t at location p to use the SRTP default cipher
 * (AES-256 Galois Counter Mode) with 16 octet auth tag.  This
 * policy applies confidentiality and authentication to both the
 * RTP and RTCP packets.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return void.
 *
 */
void srtp_crypto_policy_set_aes_gcm_256_16_auth(srtp_crypto_policy_t *p);

/**
 * @brief srtp_dealloc() deallocates storage for an SRTP session
 * context.
 *
 * The function call srtp_dealloc(s) deallocates storage for the
 * SRTP session context s.  This function should be called no more
 * than one time for each of the contexts allocated by the function
 * srtp_create().
 *
 * @param s is the srtp_t for the session to be deallocated.
 *
 * @return
 *    - srtp_err_status_ok             if there no problems.
 *    - srtp_err_status_dealloc_fail   a memory deallocation failure occurred.
 */
srtp_err_status_t srtp_dealloc(srtp_t s);

/*
 * @brief identifies a particular SRTP profile
 *
 * An srtp_profile_t enumeration is used to identify a particular SRTP
 * profile (that is, a set of algorithms and parameters).
 */
typedef enum {
    srtp_profile_reserved = 0,
    srtp_profile_aes128_cm_sha1_80 = 1,
    srtp_profile_aes128_cm_sha1_32 = 2,
    srtp_profile_null_sha1_80 = 5,
    srtp_profile_null_sha1_32 = 6,
    srtp_profile_aead_aes_128_gcm = 7,
    srtp_profile_aead_aes_256_gcm = 8
} srtp_profile_t;

/**
 * @brief srtp_crypto_policy_set_from_profile_for_rtp() sets a crypto policy
 * structure to the appropriate value for RTP based on an srtp_profile_t
 *
 * @param policy is a pointer to the policy structure to be set
 *
 * @param profile is an enumeration for the policy to be set
 *
 * The function call srtp_crypto_policy_set_rtp_default(&policy, profile)
 * sets the srtp_crypto_policy_t at location policy to the policy for RTP
 * protection, as defined by the srtp_profile_t profile.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return values
 *     - srtp_err_status_ok         no problems were encountered
 *     - srtp_err_status_bad_param  the profile is not supported
 *
 */
srtp_err_status_t srtp_crypto_policy_set_from_profile_for_rtp(
    srtp_crypto_policy_t *policy,
    srtp_profile_t profile);

/**
 * @brief srtp_crypto_policy_set_from_profile_for_rtcp() sets a crypto policy
 * structure to the appropriate value for RTCP based on an srtp_profile_t
 *
 * @param policy is a pointer to the policy structure to be set
 *
 * @param profile is an enumeration for the policy to be set
 *
 * The function call srtp_crypto_policy_set_rtcp_default(&policy, profile)
 * sets the srtp_crypto_policy_t at location policy to the policy for RTCP
 * protection, as defined by the srtp_profile_t profile.
 *
 * This function is a convenience that helps to avoid dealing directly
 * with the policy data structure.  You are encouraged to initialize
 * policy elements with this function call.  Doing so may allow your
 * code to be forward compatible with later versions of libSRTP that
 * include more elements in the srtp_crypto_policy_t datatype.
 *
 * @return values
 *     - srtp_err_status_ok         no problems were encountered
 *     - srtp_err_status_bad_param  the profile is not supported
 *
 */
srtp_err_status_t srtp_crypto_policy_set_from_profile_for_rtcp(
    srtp_crypto_policy_t *policy,
    srtp_profile_t profile);

/**
 * @brief returns the master key length for a given SRTP profile
 */
unsigned int srtp_profile_get_master_key_length(srtp_profile_t profile);

/**
 * @brief returns the master salt length for a given SRTP profile
 */
unsigned int srtp_profile_get_master_salt_length(srtp_profile_t profile);

/**
 * @brief appends the salt to the key
 *
 * The function call srtp_append_salt_to_key(k, klen, s, slen)
 * copies the string s to the location at klen bytes following
 * the location k.
 *
 * @warning There must be at least bytes_in_salt + bytes_in_key bytes
 *          available at the location pointed to by key.
 *
 */
void srtp_append_salt_to_key(unsigned char *key,
                             unsigned int bytes_in_key,
                             unsigned char *salt,
                             unsigned int bytes_in_salt);

/**
 * @}
 */

/**
 * @defgroup SRTCP Secure RTCP
 * @ingroup  SRTP
 *
 * @brief Secure RTCP functions are used to protect RTCP traffic.
 *
 * RTCP is the control protocol for RTP.  libSRTP protects RTCP
 * traffic in much the same way as it does RTP traffic.  The function
 * srtp_protect_rtcp() applies cryptographic protections to outbound
 * RTCP packets, and srtp_unprotect_rtcp() verifies the protections on
 * inbound RTCP packets.
 *
 * A note on the naming convention: srtp_protect_rtcp() has an srtp_t
 * as its first argument, and thus has `srtp_' as its prefix.  The
 * trailing `_rtcp' indicates the protocol on which it acts.
 *
 * @{
 */

/**
 * @brief srtp_protect_rtcp() is the Secure RTCP sender-side packet
 * processing function.
 *
 * The function call srtp_protect_rtcp(ctx, rtp_hdr, len_ptr) applies
 * SRTCP protection to the RTCP packet rtcp_hdr (which has length
 * *len_ptr) using the SRTP session context ctx.  If srtp_err_status_ok is
 * returned, then rtp_hdr points to the resulting SRTCP packet and
 * *len_ptr is the number of octets in that packet; otherwise, no
 * assumptions should be made about the value of either data elements.
 *
 * @warning This function assumes that it can write the authentication
 * tag into the location in memory immediately following the RTCP
 * packet, and assumes that the RTCP packet is aligned on a 32-bit
 * boundary.
 *
 * @warning This function assumes that it can write SRTP_MAX_SRTCP_TRAILER_LEN
 * into the location in memory immediately following the RTCP packet.
 * Callers MUST ensure that this much writable memory is available in
 * the buffer that holds the RTCP packet.
 *
 * @param ctx is the SRTP context to use in processing the packet.
 *
 * @param rtcp_hdr is a pointer to the RTCP packet (before the call); after
 * the function returns, it points to the srtp packet.
 *
 * @param pkt_octet_len is a pointer to the length in octets of the
 * complete RTCP packet (header and body) before the function call,
 * and of the complete SRTCP packet after the call, if srtp_err_status_ok
 * was returned.  Otherwise, the value of the data to which it points
 * is undefined.
 *
 * @return
 *    - srtp_err_status_ok            if there were no problems.
 *    - [other]                  if there was a failure in
 *                               the cryptographic mechanisms.
 */
srtp_err_status_t srtp_protect_rtcp(srtp_t ctx,
                                    void *rtcp_hdr,
                                    int *pkt_octet_len);

/**
 * @brief srtp_protect_rtcp_mki() is the Secure RTCP sender-side packet
 * processing function that can utilize mki.
 *
 * The function call srtp_protect_rtcp(ctx, rtp_hdr, len_ptr) applies
 * SRTCP protection to the RTCP packet rtcp_hdr (which has length
 * *len_ptr) using the SRTP session context ctx.  If srtp_err_status_ok is
 * returned, then rtp_hdr points to the resulting SRTCP packet and
 * *len_ptr is the number of octets in that packet; otherwise, no
 * assumptions should be made about the value of either data elements.
 *
 * @warning This function assumes that it can write the authentication
 * tag into the location in memory immediately following the RTCP
 * packet, and assumes that the RTCP packet is aligned on a 32-bit
 * boundary.
 *
 * @warning This function assumes that it can write SRTP_MAX_SRTCP_TRAILER_LEN
 * into the location in memory immediately following the RTCP packet.
 * Callers MUST ensure that this much writable memory is available in
 * the buffer that holds the RTCP packet.
 *
 * @param ctx is the SRTP context to use in processing the packet.
 *
 * @param rtcp_hdr is a pointer to the RTCP packet (before the call); after
 * the function returns, it points to the srtp packet.
 *
 * @param pkt_octet_len is a pointer to the length in octets of the
 * complete RTCP packet (header and body) before the function call,
 * and of the complete SRTCP packet after the call, if srtp_err_status_ok
 * was returned.  Otherwise, the value of the data to which it points
 * is undefined.
 *
 * @param use_mki is a boolean to tell the system if mki is being used.  If
 * set to false then will use the first set of session keys.  If set to true
 * will
 * use the session keys identified by the mki_index
 *
 * @param mki_index integer value specifying which set of session keys should be
 * used if use_mki is set to true.
 *
 * @return
 *    - srtp_err_status_ok            if there were no problems.
 *    - [other]                  if there was a failure in
 *                               the cryptographic mechanisms.
 */
srtp_err_status_t srtp_protect_rtcp_mki(srtp_t ctx,
                                        void *rtcp_hdr,
                                        int *pkt_octet_len,
                                        unsigned int use_mki,
                                        unsigned int mki_index);

/**
 * @brief srtp_unprotect_rtcp() is the Secure RTCP receiver-side packet
 * processing function.
 *
 * The function call srtp_unprotect_rtcp(ctx, srtp_hdr, len_ptr)
 * verifies the Secure RTCP protection of the SRTCP packet pointed to
 * by srtcp_hdr (which has length *len_ptr), using the SRTP session
 * context ctx.  If srtp_err_status_ok is returned, then srtcp_hdr points
 * to the resulting RTCP packet and *len_ptr is the number of octets
 * in that packet; otherwise, no assumptions should be made about the
 * value of either data elements.
 *
 * @warning This function assumes that the SRTCP packet is aligned on a
 * 32-bit boundary.
 *
 * @param ctx is a pointer to the srtp_t which applies to the
 * particular packet.
 *
 * @param srtcp_hdr is a pointer to the header of the SRTCP packet
 * (before the call).  After the function returns, it points to the
 * rtp packet if srtp_err_status_ok was returned; otherwise, the value of
 * the data to which it points is undefined.
 *
 * @param pkt_octet_len is a pointer to the length in octets of the
 * complete SRTCP packet (header and body) before the function call,
 * and of the complete rtp packet after the call, if srtp_err_status_ok was
 * returned.  Otherwise, the value of the data to which it points is
 * undefined.
 *
 * @return
 *    - srtp_err_status_ok          if the RTCP packet is valid.
 *    - srtp_err_status_auth_fail   if the SRTCP packet failed the message
 *                             authentication check.
 *    - srtp_err_status_replay_fail if the SRTCP packet is a replay (e.g. has
 *                             already been processed and accepted).
 *    - [other]  if there has been an error in the cryptographic mechanisms.
 *
 */
srtp_err_status_t srtp_unprotect_rtcp(srtp_t ctx,
                                      void *srtcp_hdr,
                                      int *pkt_octet_len);

/**
 * @brief srtp_unprotect_rtcp() is the Secure RTCP receiver-side packet
 * processing function.
 *
 * The function call srtp_unprotect_rtcp(ctx, srtp_hdr, len_ptr)
 * verifies the Secure RTCP protection of the SRTCP packet pointed to
 * by srtcp_hdr (which has length *len_ptr), using the SRTP session
 * context ctx.  If srtp_err_status_ok is returned, then srtcp_hdr points
 * to the resulting RTCP packet and *len_ptr is the number of octets
 * in that packet; otherwise, no assumptions should be made about the
 * value of either data elements.
 *
 * @warning This function assumes that the SRTCP packet is aligned on a
 * 32-bit boundary.
 *
 * @param ctx is a pointer to the srtp_t which applies to the
 * particular packet.
 *
 * @param srtcp_hdr is a pointer to the header of the SRTCP packet
 * (before the call).  After the function returns, it points to the
 * rtp packet if srtp_err_status_ok was returned; otherwise, the value of
 * the data to which it points is undefined.
 *
 * @param pkt_octet_len is a pointer to the length in octets of the
 * complete SRTCP packet (header and body) before the function call,
 * and of the complete rtp packet after the call, if srtp_err_status_ok was
 * returned.  Otherwise, the value of the data to which it points is
 * undefined.
 *
 * @param use_mki is a boolean to tell the system if mki is being used.  If
 * set to false then will use the first set of session keys.  If set to true
 * will use the session keys identified by the mki_index
 *
 * @return
 *    - srtp_err_status_ok          if the RTCP packet is valid.
 *    - srtp_err_status_auth_fail   if the SRTCP packet failed the message
 *                                  authentication check.
 *    - srtp_err_status_replay_fail if the SRTCP packet is a replay (e.g. has
 *                                  already been processed and accepted).
 *    - srtp_err_status_bad_mki     if the MKI in the packet is not a known MKI
 *                                  id
 *    - [other]                     if there has been an error in the
 *                                  cryptographic mechanisms.
 *
 */
srtp_err_status_t srtp_unprotect_rtcp_mki(srtp_t ctx,
                                          void *srtcp_hdr,
                                          int *pkt_octet_len,
                                          unsigned int use_mki);

/**
 * @}
 */

/**
 * @defgroup User data associated to a SRTP session.
 * @ingroup  SRTP
 *
 * @brief Store custom user data within a SRTP session.
 *
 * @{
 */

/**
 * @brief srtp_set_user_data() stores the given pointer into the SRTP
 * session for later retrieval.
 *
 * @param ctx is the srtp_t context in which the given data pointer is
 * stored.
 *
 * @param data is a pointer to the custom information (struct, function,
 * etc) associated with the SRTP session.
 *
 * @return void.
 *
 */
void srtp_set_user_data(srtp_t ctx, void *data);

/**
 * @brief srtp_get_user_data() retrieves the pointer to the custom data
 * previously stored with srtp_set_user_data().
 *
 * This function is mostly useful for retrieving data associated to a
 * SRTP session when an event fires. The user can then get such a custom
 * data by calling this function with the session field of the
 * srtp_event_data_t struct as argument.
 *
 * @param ctx is the srtp_t context in which the given data pointer was
 * stored.
 *
 * @return void* pointer to the user data.
 *
 */
void *srtp_get_user_data(srtp_t ctx);

/**
 * @}
 */

/**
 * @defgroup SRTPevents SRTP events and callbacks
 * @ingroup  SRTP
 *
 * @brief libSRTP can use a user-provided callback function to
 * handle events.
 *
 *
 * libSRTP allows a user to provide a callback function to handle
 * events that need to be dealt with outside of the data plane (see
 * the enum srtp_event_t for a description of these events).  Dealing
 * with these events is not a strict necessity; they are not
 * security-critical, but the application may suffer if they are not
 * handled.  The function srtp_set_event_handler() is used to provide
 * the callback function.
 *
 * A default event handler that merely reports on the events as they
 * happen is included.  It is also possible to set the event handler
 * function to NULL, in which case all events will just be silently
 * ignored.
 *
 * @{
 */

/**
 * @brief srtp_event_t defines events that need to be handled
 *
 * The enum srtp_event_t defines events that need to be handled
 * outside the `data plane', such as SSRC collisions and
 * key expirations.
 *
 * When a key expires or the maximum number of packets has been
 * reached, an SRTP stream will enter an `expired' state in which no
 * more packets can be protected or unprotected.  When this happens,
 * it is likely that you will want to either deallocate the stream
 * (using srtp_remove_stream()), and possibly allocate a new one.
 *
 * When an SRTP stream expires, the other streams in the same session
 * are unaffected, unless key sharing is used by that stream.  In the
 * latter case, all of the streams in the session will expire.
 */
typedef enum {
    event_ssrc_collision,    /**< An SSRC collision occurred.           */
    event_key_soft_limit,    /**< An SRTP stream reached the soft key   */
                             /**< usage limit and will expire soon.     */
    event_key_hard_limit,    /**< An SRTP stream reached the hard       */
                             /**< key usage limit and has expired.      */
    event_packet_index_limit /**< An SRTP stream reached the hard       */
                             /**< packet limit (2^48 packets).          */
} srtp_event_t;

/**
 * @brief srtp_event_data_t is the structure passed as a callback to
 * the event handler function
 *
 * The struct srtp_event_data_t holds the data passed to the event
 * handler function.
 */
typedef struct srtp_event_data_t {
    srtp_t session;     /**< The session in which the event happened.       */
    uint32_t ssrc;      /**< The ssrc in host order of the stream in which  */
                        /**< the event happened                             */
    srtp_event_t event; /**< An enum indicating the type of event.          */
} srtp_event_data_t;

/**
 * @brief srtp_event_handler_func_t is the function prototype for
 * the event handler.
 *
 * The typedef srtp_event_handler_func_t is the prototype for the
 * event handler function.  It has as its only argument an
 * srtp_event_data_t which describes the event that needs to be handled.
 * There can only be a single, global handler for all events in
 * libSRTP.
 */
typedef void(srtp_event_handler_func_t)(srtp_event_data_t *data);

/**
 * @brief sets the event handler to the function supplied by the caller.
 *
 * The function call srtp_install_event_handler(func) sets the event
 * handler function to the value func.  The value NULL is acceptable
 * as an argument; in this case, events will be ignored rather than
 * handled.
 *
 * @param func is a pointer to a function that takes an srtp_event_data_t
 *             pointer as an argument and returns void.  This function
 *             will be used by libSRTP to handle events.
 */
srtp_err_status_t srtp_install_event_handler(srtp_event_handler_func_t func);

/**
 * @brief Returns the version string of the library.
 *
 */
const char *srtp_get_version_string(void);

/**
 * @brief Returns the numeric representation of the library version.
 *
 */
unsigned int srtp_get_version(void);

/**
 * @brief srtp_set_debug_module(mod_name, v)
 *
 * sets dynamic debugging to the value v (0 for off, 1 for on) for the
 * debug module with the name mod_name
 *
 * returns err_status_ok on success, err_status_fail otherwise
 */
srtp_err_status_t srtp_set_debug_module(const char *mod_name, int v);

/**
 * @brief srtp_list_debug_modules() outputs a list of debugging modules
 *
 */
srtp_err_status_t srtp_list_debug_modules(void);

/**
 * @brief srtp_log_level_t defines log levels.
 *
 * The enumeration srtp_log_level_t defines log levels reported
 * in the srtp_log_handler_func_t.
 *
 */
typedef enum {
    srtp_log_level_error,   /**< log level is reporting an error message  */
    srtp_log_level_warning, /**< log level is reporting a warning message */
    srtp_log_level_info,    /**< log level is reporting an info message   */
    srtp_log_level_debug    /**< log level is reporting a debug message   */
} srtp_log_level_t;

/**
 * @brief srtp_log_handler_func_t is the function prototype for
 * the log handler.
 *
 * The typedef srtp_event_handler_func_t is the prototype for the
 * event handler function.  It has as srtp_log_level_t, log
 * message and data as arguments.
 * There can only be a single, global handler for all log messages in
 * libSRTP.
 */
typedef void(srtp_log_handler_func_t)(srtp_log_level_t level,
                                      const char *msg,
                                      void *data);

/**
 * @brief sets the log handler to the function supplied by the caller.
 *
 * The function call srtp_install_log_handler(func) sets the log
 * handler function to the value func.  The value NULL is acceptable
 * as an argument; in this case, log messages will be ignored.
 * This function can be called before srtp_init() in order to capture
 * any logging during start up.
 *
 * @param func is a pointer to a function of type srtp_log_handler_func_t.
 *             This function will be used by libSRTP to output log messages.
 * @param data is a user pointer that will be returned as the data argument in
 * func.
 */
srtp_err_status_t srtp_install_log_handler(srtp_log_handler_func_t func,
                                           void *data);

/**
 * @brief srtp_get_protect_trailer_length(session, use_mki, mki_index, length)
 *
 * Determines the length of the amount of data Lib SRTP will add to the
 * packet during the protect process. The length is returned in the length
 * parameter
 *
 * returns err_status_ok on success, err_status_bad_mki if the MKI index is
 * invalid
 *
 */
srtp_err_status_t srtp_get_protect_trailer_length(srtp_t session,
                                                  uint32_t use_mki,
                                                  uint32_t mki_index,
                                                  uint32_t *length);

/**
 * @brief srtp_get_protect_rtcp_trailer_length(session, use_mki, mki_index,
 * length)
 *
 * Determines the length of the amount of data Lib SRTP will add to the
 * packet during the protect process. The length is returned in the length
 * parameter
 *
 * returns err_status_ok on success, err_status_bad_mki if the MKI index is
 * invalid
 *
 */
srtp_err_status_t srtp_get_protect_rtcp_trailer_length(srtp_t session,
                                                       uint32_t use_mki,
                                                       uint32_t mki_index,
                                                       uint32_t *length);

/**
 * @brief srtp_set_stream_roc(session, ssrc, roc)
 *
 * Set the roll-over-counter on a session for a given SSRC
 *
 * returns err_status_ok on success, srtp_err_status_bad_param if there is no
 * stream found
 *
 */
srtp_err_status_t srtp_set_stream_roc(srtp_t session,
                                      uint32_t ssrc,
                                      uint32_t roc);

/**
 * @brief srtp_get_stream_roc(session, ssrc, roc)
 *
 * Get the roll-over-counter on a session for a given SSRC
 *
 * returns err_status_ok on success, srtp_err_status_bad_param if there is no
 * stream found
 *
 */
srtp_err_status_t srtp_get_stream_roc(srtp_t session,
                                      uint32_t ssrc,
                                      uint32_t *roc);

/**
 * @}
 */

/* in host order, so outside the #if */
#define SRTCP_E_BIT 0x80000000

/* for byte-access */
#define SRTCP_E_BYTE_BIT 0x80
#define SRTCP_INDEX_MASK 0x7fffffff

#ifdef __cplusplus
}
#endif

#endif /* SRTP_SRTP_H */
