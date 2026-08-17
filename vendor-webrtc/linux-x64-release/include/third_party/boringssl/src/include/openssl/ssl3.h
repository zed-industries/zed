// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_SSL3_H
#define OPENSSL_HEADER_SSL3_H

#include <openssl/aead.h>

#ifdef  __cplusplus
extern "C" {
#endif


// These are kept to support clients that negotiates higher protocol versions
// using SSLv2 client hello records.
#define SSL2_MT_CLIENT_HELLO 1
#define SSL2_VERSION 0x0002

// Signalling cipher suite value from RFC 5746.
#define SSL3_CK_SCSV 0x030000FF
// Fallback signalling cipher suite value from RFC 7507.
#define SSL3_CK_FALLBACK_SCSV 0x03005600

#define SSL3_CK_RSA_NULL_MD5 0x03000001
#define SSL3_CK_RSA_NULL_SHA 0x03000002
#define SSL3_CK_RSA_RC4_40_MD5 0x03000003
#define SSL3_CK_RSA_RC4_128_MD5 0x03000004
#define SSL3_CK_RSA_RC4_128_SHA 0x03000005
#define SSL3_CK_RSA_RC2_40_MD5 0x03000006
#define SSL3_CK_RSA_IDEA_128_SHA 0x03000007
#define SSL3_CK_RSA_DES_40_CBC_SHA 0x03000008
#define SSL3_CK_RSA_DES_64_CBC_SHA 0x03000009
#define SSL3_CK_RSA_DES_192_CBC3_SHA 0x0300000A

#define SSL3_CK_DH_DSS_DES_40_CBC_SHA 0x0300000B
#define SSL3_CK_DH_DSS_DES_64_CBC_SHA 0x0300000C
#define SSL3_CK_DH_DSS_DES_192_CBC3_SHA 0x0300000D
#define SSL3_CK_DH_RSA_DES_40_CBC_SHA 0x0300000E
#define SSL3_CK_DH_RSA_DES_64_CBC_SHA 0x0300000F
#define SSL3_CK_DH_RSA_DES_192_CBC3_SHA 0x03000010

#define SSL3_CK_EDH_DSS_DES_40_CBC_SHA 0x03000011
#define SSL3_CK_EDH_DSS_DES_64_CBC_SHA 0x03000012
#define SSL3_CK_EDH_DSS_DES_192_CBC3_SHA 0x03000013
#define SSL3_CK_EDH_RSA_DES_40_CBC_SHA 0x03000014
#define SSL3_CK_EDH_RSA_DES_64_CBC_SHA 0x03000015
#define SSL3_CK_EDH_RSA_DES_192_CBC3_SHA 0x03000016

#define SSL3_CK_ADH_RC4_40_MD5 0x03000017
#define SSL3_CK_ADH_RC4_128_MD5 0x03000018
#define SSL3_CK_ADH_DES_40_CBC_SHA 0x03000019
#define SSL3_CK_ADH_DES_64_CBC_SHA 0x0300001A
#define SSL3_CK_ADH_DES_192_CBC_SHA 0x0300001B

#define SSL3_TXT_RSA_NULL_MD5 "NULL-MD5"
#define SSL3_TXT_RSA_NULL_SHA "NULL-SHA"
#define SSL3_TXT_RSA_RC4_40_MD5 "EXP-RC4-MD5"
#define SSL3_TXT_RSA_RC4_128_MD5 "RC4-MD5"
#define SSL3_TXT_RSA_RC4_128_SHA "RC4-SHA"
#define SSL3_TXT_RSA_RC2_40_MD5 "EXP-RC2-CBC-MD5"
#define SSL3_TXT_RSA_IDEA_128_SHA "IDEA-CBC-SHA"
#define SSL3_TXT_RSA_DES_40_CBC_SHA "EXP-DES-CBC-SHA"
#define SSL3_TXT_RSA_DES_64_CBC_SHA "DES-CBC-SHA"
#define SSL3_TXT_RSA_DES_192_CBC3_SHA "DES-CBC3-SHA"

#define SSL3_TXT_DH_DSS_DES_40_CBC_SHA "EXP-DH-DSS-DES-CBC-SHA"
#define SSL3_TXT_DH_DSS_DES_64_CBC_SHA "DH-DSS-DES-CBC-SHA"
#define SSL3_TXT_DH_DSS_DES_192_CBC3_SHA "DH-DSS-DES-CBC3-SHA"
#define SSL3_TXT_DH_RSA_DES_40_CBC_SHA "EXP-DH-RSA-DES-CBC-SHA"
#define SSL3_TXT_DH_RSA_DES_64_CBC_SHA "DH-RSA-DES-CBC-SHA"
#define SSL3_TXT_DH_RSA_DES_192_CBC3_SHA "DH-RSA-DES-CBC3-SHA"

#define SSL3_TXT_EDH_DSS_DES_40_CBC_SHA "EXP-EDH-DSS-DES-CBC-SHA"
#define SSL3_TXT_EDH_DSS_DES_64_CBC_SHA "EDH-DSS-DES-CBC-SHA"
#define SSL3_TXT_EDH_DSS_DES_192_CBC3_SHA "EDH-DSS-DES-CBC3-SHA"
#define SSL3_TXT_EDH_RSA_DES_40_CBC_SHA "EXP-EDH-RSA-DES-CBC-SHA"
#define SSL3_TXT_EDH_RSA_DES_64_CBC_SHA "EDH-RSA-DES-CBC-SHA"
#define SSL3_TXT_EDH_RSA_DES_192_CBC3_SHA "EDH-RSA-DES-CBC3-SHA"

#define SSL3_TXT_ADH_RC4_40_MD5 "EXP-ADH-RC4-MD5"
#define SSL3_TXT_ADH_RC4_128_MD5 "ADH-RC4-MD5"
#define SSL3_TXT_ADH_DES_40_CBC_SHA "EXP-ADH-DES-CBC-SHA"
#define SSL3_TXT_ADH_DES_64_CBC_SHA "ADH-DES-CBC-SHA"
#define SSL3_TXT_ADH_DES_192_CBC_SHA "ADH-DES-CBC3-SHA"

#define SSL3_SSL_SESSION_ID_LENGTH 32
#define SSL3_MAX_SSL_SESSION_ID_LENGTH 32

#define SSL3_MASTER_SECRET_SIZE 48
#define SSL3_RANDOM_SIZE 32
#define SSL3_SESSION_ID_SIZE 32
#define SSL3_RT_HEADER_LENGTH 5

#define SSL3_HM_HEADER_LENGTH 4

#ifndef SSL3_ALIGN_PAYLOAD
// Some will argue that this increases memory footprint, but it's not actually
// true. Point is that malloc has to return at least 64-bit aligned pointers,
// meaning that allocating 5 bytes wastes 3 bytes in either case. Suggested
// pre-gaping simply moves these wasted bytes from the end of allocated region
// to its front, but makes data payload aligned, which improves performance.
#define SSL3_ALIGN_PAYLOAD 8
#else
#if (SSL3_ALIGN_PAYLOAD & (SSL3_ALIGN_PAYLOAD - 1)) != 0
#error "insane SSL3_ALIGN_PAYLOAD"
#undef SSL3_ALIGN_PAYLOAD
#endif
#endif

// This is the maximum MAC (digest) size used by the SSL library. Currently
// maximum of 20 is used by SHA1, but we reserve for future extension for
// 512-bit hashes.

#define SSL3_RT_MAX_MD_SIZE 64

// Maximum block size used in all ciphersuites. Currently 16 for AES.

#define SSL_RT_MAX_CIPHER_BLOCK_SIZE 16

// Maximum plaintext length: defined by SSL/TLS standards
#define SSL3_RT_MAX_PLAIN_LENGTH 16384
// Maximum compression overhead: defined by SSL/TLS standards
#define SSL3_RT_MAX_COMPRESSED_OVERHEAD 1024

// The standards give a maximum encryption overhead of 1024 bytes. In practice
// the value is lower than this. The overhead is the maximum number of padding
// bytes (256) plus the mac size.
//
// TODO(davidben): This derivation doesn't take AEADs into account, or TLS 1.1
// explicit nonces. It happens to work because |SSL3_RT_MAX_MD_SIZE| is larger
// than necessary and no true AEAD has variable overhead in TLS 1.2.
#define SSL3_RT_MAX_ENCRYPTED_OVERHEAD (256 + SSL3_RT_MAX_MD_SIZE)

// SSL3_RT_SEND_MAX_ENCRYPTED_OVERHEAD is the maximum overhead in encrypting a
// record. This does not include the record header. Some ciphers use explicit
// nonces, so it includes both the AEAD overhead as well as the nonce.
#define SSL3_RT_SEND_MAX_ENCRYPTED_OVERHEAD \
    (EVP_AEAD_MAX_OVERHEAD + EVP_AEAD_MAX_NONCE_LENGTH)

// SSL3_RT_MAX_COMPRESSED_LENGTH is an alias for
// |SSL3_RT_MAX_PLAIN_LENGTH|. Compression is gone, so don't include the
// compression overhead.
#define SSL3_RT_MAX_COMPRESSED_LENGTH SSL3_RT_MAX_PLAIN_LENGTH

#define SSL3_RT_MAX_ENCRYPTED_LENGTH \
  (SSL3_RT_MAX_ENCRYPTED_OVERHEAD + SSL3_RT_MAX_COMPRESSED_LENGTH)
#define SSL3_RT_MAX_PACKET_SIZE \
  (SSL3_RT_MAX_ENCRYPTED_LENGTH + SSL3_RT_HEADER_LENGTH)

#define SSL3_MD_CLIENT_FINISHED_CONST "\x43\x4C\x4E\x54"
#define SSL3_MD_SERVER_FINISHED_CONST "\x53\x52\x56\x52"

#define SSL3_RT_CHANGE_CIPHER_SPEC 20
#define SSL3_RT_ALERT 21
#define SSL3_RT_HANDSHAKE 22
#define SSL3_RT_APPLICATION_DATA 23
#define SSL3_RT_ACK 26

// Pseudo content type for SSL/TLS header info
#define SSL3_RT_HEADER 0x100
#define SSL3_RT_CLIENT_HELLO_INNER 0x101

#define SSL3_AL_WARNING 1
#define SSL3_AL_FATAL 2

#define SSL3_AD_CLOSE_NOTIFY 0
#define SSL3_AD_UNEXPECTED_MESSAGE 10     // fatal
#define SSL3_AD_BAD_RECORD_MAC 20         // fatal
#define SSL3_AD_DECOMPRESSION_FAILURE 30  // fatal
#define SSL3_AD_HANDSHAKE_FAILURE 40      // fatal
#define SSL3_AD_NO_CERTIFICATE 41
#define SSL3_AD_BAD_CERTIFICATE 42
#define SSL3_AD_UNSUPPORTED_CERTIFICATE 43
#define SSL3_AD_CERTIFICATE_REVOKED 44
#define SSL3_AD_CERTIFICATE_EXPIRED 45
#define SSL3_AD_CERTIFICATE_UNKNOWN 46
#define SSL3_AD_ILLEGAL_PARAMETER 47       // fatal
#define SSL3_AD_INAPPROPRIATE_FALLBACK 86  // fatal

#define SSL3_CT_RSA_SIGN 1

#define SSL3_MT_HELLO_REQUEST 0
#define SSL3_MT_CLIENT_HELLO 1
#define SSL3_MT_SERVER_HELLO 2
#define SSL3_MT_NEW_SESSION_TICKET 4
#define SSL3_MT_END_OF_EARLY_DATA 5
#define SSL3_MT_ENCRYPTED_EXTENSIONS 8
#define SSL3_MT_CERTIFICATE 11
#define SSL3_MT_SERVER_KEY_EXCHANGE 12
#define SSL3_MT_CERTIFICATE_REQUEST 13
#define SSL3_MT_SERVER_HELLO_DONE 14
#define SSL3_MT_CERTIFICATE_VERIFY 15
#define SSL3_MT_CLIENT_KEY_EXCHANGE 16
#define SSL3_MT_FINISHED 20
#define SSL3_MT_CERTIFICATE_STATUS 22
#define SSL3_MT_SUPPLEMENTAL_DATA 23
#define SSL3_MT_KEY_UPDATE 24
#define SSL3_MT_COMPRESSED_CERTIFICATE 25
#define SSL3_MT_NEXT_PROTO 67
#define SSL3_MT_CHANNEL_ID 203
#define SSL3_MT_MESSAGE_HASH 254
#define DTLS1_MT_HELLO_VERIFY_REQUEST 3

// The following are legacy aliases for consumers which use
// |SSL_CTX_set_msg_callback|.
#define SSL3_MT_SERVER_DONE SSL3_MT_SERVER_HELLO_DONE
#define SSL3_MT_NEWSESSION_TICKET SSL3_MT_NEW_SESSION_TICKET


#define SSL3_MT_CCS 1


#ifdef  __cplusplus
}  // extern C
#endif

#endif  // OPENSSL_HEADER_SSL3_H
