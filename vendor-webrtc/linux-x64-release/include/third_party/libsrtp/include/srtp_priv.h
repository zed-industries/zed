/*
 * srtp_priv.h
 *
 * private internal data structures and functions for libSRTP
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 */
/*
 *
 * Copyright (c) 2001-2017 Cisco Systems, Inc.
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

#ifndef SRTP_PRIV_H
#define SRTP_PRIV_H

// Leave this as the top level import. Ensures the existence of defines
#include "config.h"

#include "srtp.h"
#include "rdbx.h"
#include "rdb.h"
#include "integers.h"
#include "cipher.h"
#include "auth.h"
#include "aes.h"
#include "crypto_kernel.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SRTP_VER_STRING PACKAGE_STRING
#define SRTP_VERSION PACKAGE_VERSION

typedef struct srtp_stream_ctx_t_ srtp_stream_ctx_t;
typedef srtp_stream_ctx_t *srtp_stream_t;
typedef struct srtp_stream_list_ctx_t_ *srtp_stream_list_t;

/*
 * the following declarations are libSRTP internal functions
 */

/*
 * srtp_get_stream(ssrc) returns a pointer to the stream corresponding
 * to ssrc, or NULL if no stream exists for that ssrc
 */
srtp_stream_t srtp_get_stream(srtp_t srtp, uint32_t ssrc);

/*
 * srtp_stream_init_keys(s, k) (re)initializes the srtp_stream_t s by
 * deriving all of the needed keys using the KDF and the key k.
 */
srtp_err_status_t srtp_stream_init_keys(srtp_stream_ctx_t *srtp,
                                        srtp_master_key_t *master_key,
                                        const unsigned int current_mki_index);

/*
 * srtp_stream_init_all_master_keys(s, k, m) (re)initializes the srtp_stream_t s
 * by deriving all of the needed keys for all the master keys using the KDF and
 * the keys from k.
 */
srtp_err_status_t srtp_steam_init_all_master_keys(
    srtp_stream_ctx_t *srtp,
    unsigned char *key,
    srtp_master_key_t **keys,
    const unsigned int max_master_keys);

/*
 * libsrtp internal datatypes
 */
typedef enum direction_t {
    dir_unknown = 0,
    dir_srtp_sender = 1,
    dir_srtp_receiver = 2
} direction_t;

/*
 * srtp_session_keys_t will contain the encryption, hmac, salt keys
 * for both SRTP and SRTCP.  The session keys will also contain the
 * MKI ID which is used to identify the session keys.
 */
typedef struct srtp_session_keys_t {
    srtp_cipher_t *rtp_cipher;
    srtp_cipher_t *rtp_xtn_hdr_cipher;
    srtp_auth_t *rtp_auth;
    srtp_cipher_t *rtcp_cipher;
    srtp_auth_t *rtcp_auth;
    uint8_t salt[SRTP_AEAD_SALT_LEN];
    uint8_t c_salt[SRTP_AEAD_SALT_LEN];
    uint8_t *mki_id;
    unsigned int mki_size;
    srtp_key_limit_ctx_t *limit;
} srtp_session_keys_t;

/*
 * an srtp_stream_t has its own SSRC, encryption key, authentication
 * key, sequence number, and replay database
 *
 * note that the keys might not actually be unique, in which case the
 * srtp_cipher_t and srtp_auth_t pointers will point to the same structures
 */
typedef struct srtp_stream_ctx_t_ {
    uint32_t ssrc;
    srtp_session_keys_t *session_keys;
    unsigned int num_master_keys;
    srtp_rdbx_t rtp_rdbx;
    srtp_sec_serv_t rtp_services;
    srtp_rdb_t rtcp_rdb;
    srtp_sec_serv_t rtcp_services;
    direction_t direction;
    int allow_repeat_tx;
    int *enc_xtn_hdr;
    int enc_xtn_hdr_count;
    uint32_t pending_roc;
    /*
    The next and prev pointers are here to allow for a stream list to be
    implemented as an intrusive doubly-linked list (the former being the
    default).  Other stream list implementations can ignore these fields or use
    them for some other purpose specific to the stream list implementation.
    */
    struct srtp_stream_ctx_t_ *next;
    struct srtp_stream_ctx_t_ *prev;
} strp_stream_ctx_t_;

/*
 * an srtp_ctx_t holds a stream list and a service description
 */
typedef struct srtp_ctx_t_ {
    srtp_stream_list_t stream_list;             /* linked list of streams     */
    struct srtp_stream_ctx_t_ *stream_template; /* act as template for other  */
                                                /* streams                    */
    void *user_data;                            /* user custom data           */
} srtp_ctx_t_;

/*
 * srtp_hdr_t represents an RTP or SRTP header.  The bit-fields in
 * this structure should be declared "unsigned int" instead of
 * "unsigned char", but doing so causes the MS compiler to not
 * fully pack the bit fields.
 *
 * In this implementation, an srtp_hdr_t is assumed to be 32-bit aligned
 *
 * (note that this definition follows that of RFC 1889 Appendix A, but
 * is not identical)
 */

#ifndef WORDS_BIGENDIAN

typedef struct {
    unsigned char cc : 4;      /* CSRC count             */
    unsigned char x : 1;       /* header extension flag  */
    unsigned char p : 1;       /* padding flag           */
    unsigned char version : 2; /* protocol version       */
    unsigned char pt : 7;      /* payload type           */
    unsigned char m : 1;       /* marker bit             */
    uint16_t seq;              /* sequence number        */
    uint32_t ts;               /* timestamp              */
    uint32_t ssrc;             /* synchronization source */
} srtp_hdr_t;

#else /*  BIG_ENDIAN */

typedef struct {
    unsigned char version : 2; /* protocol version       */
    unsigned char p : 1;       /* padding flag           */
    unsigned char x : 1;       /* header extension flag  */
    unsigned char cc : 4;      /* CSRC count             */
    unsigned char m : 1;       /* marker bit             */
    unsigned char pt : 7;      /* payload type           */
    uint16_t seq;              /* sequence number        */
    uint32_t ts;               /* timestamp              */
    uint32_t ssrc;             /* synchronization source */
} srtp_hdr_t;

#endif

typedef struct {
    uint16_t profile_specific; /* profile-specific info               */
    uint16_t length;           /* number of 32-bit words in extension */
} srtp_hdr_xtnd_t;

/*
 * srtcp_hdr_t represents a secure rtcp header
 *
 * in this implementation, an srtcp header is assumed to be 32-bit
 * aligned
 */

#ifndef WORDS_BIGENDIAN

typedef struct {
    unsigned char rc : 5;      /* reception report count */
    unsigned char p : 1;       /* padding flag           */
    unsigned char version : 2; /* protocol version       */
    unsigned char pt : 8;      /* payload type           */
    uint16_t len;              /* length                 */
    uint32_t ssrc;             /* synchronization source */
} srtcp_hdr_t;

typedef struct {
    unsigned int index : 31; /* srtcp packet index in network order!  */
    unsigned int e : 1;      /* encrypted? 1=yes                      */
                             /* optional mikey/etc go here            */
                             /* and then the variable-length auth tag */
} srtcp_trailer_t;

#else /*  BIG_ENDIAN */

typedef struct {
    unsigned char version : 2; /* protocol version       */
    unsigned char p : 1;       /* padding flag           */
    unsigned char rc : 5;      /* reception report count */
    unsigned char pt : 8;      /* payload type           */
    uint16_t len;              /* length                 */
    uint32_t ssrc;             /* synchronization source */
} srtcp_hdr_t;

typedef struct {
    unsigned int e : 1;      /* encrypted? 1=yes                      */
    unsigned int index : 31; /* srtcp packet index                    */
                             /* optional mikey/etc go here            */
                             /* and then the variable-length auth tag */
} srtcp_trailer_t;

#endif

/*
 * srtp_handle_event(srtp, srtm, evnt) calls the event handling
 * function, if there is one.
 *
 * This macro is not included in the documentation as it is
 * an internal-only function.
 */

#define srtp_handle_event(srtp, strm, evnt)                                    \
    if (srtp_event_handler) {                                                  \
        srtp_event_data_t data;                                                \
        data.session = srtp;                                                   \
        data.ssrc = ntohl(strm->ssrc);                                         \
        data.event = evnt;                                                     \
        srtp_event_handler(&data);                                             \
    }

#ifdef __cplusplus
}
#endif

#endif /* SRTP_PRIV_H */
