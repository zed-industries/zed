// Copyright 2014 The BoringSSL Authors
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

// This header is provided in order to make compiling against code that expects
// OpenSSL easier.

#ifndef OPENSSL_HEADER_OPENSSLCONF_H
#define OPENSSL_HEADER_OPENSSLCONF_H

// Keep in sync with the list in rust/bssl-sys/build.rs.

#define OPENSSL_NO_ASYNC
#define OPENSSL_NO_BF
#define OPENSSL_NO_BLAKE2
#define OPENSSL_NO_BUF_FREELISTS
#define OPENSSL_NO_CAMELLIA
#define OPENSSL_NO_CAPIENG
#define OPENSSL_NO_CAST
#define OPENSSL_NO_COMP
#define OPENSSL_NO_CT
#define OPENSSL_NO_DANE
#define OPENSSL_NO_DEPRECATED
#define OPENSSL_NO_DGRAM
#define OPENSSL_NO_DYNAMIC_ENGINE
#define OPENSSL_NO_EC_NISTP_64_GCC_128
#define OPENSSL_NO_EC2M
#define OPENSSL_NO_EGD
#define OPENSSL_NO_ENGINE
#define OPENSSL_NO_GMP
#define OPENSSL_NO_GOST
#define OPENSSL_NO_HEARTBEATS
#define OPENSSL_NO_HW
#define OPENSSL_NO_IDEA
#define OPENSSL_NO_JPAKE
#define OPENSSL_NO_KRB5
#define OPENSSL_NO_MD2
#define OPENSSL_NO_MDC2
#define OPENSSL_NO_OCB
#define OPENSSL_NO_OCSP
#define OPENSSL_NO_RC2
#define OPENSSL_NO_RC5
#define OPENSSL_NO_RFC3779
#define OPENSSL_NO_RIPEMD
#define OPENSSL_NO_RMD160
#define OPENSSL_NO_SCTP
#define OPENSSL_NO_SEED
#define OPENSSL_NO_SM2
#define OPENSSL_NO_SM3
#define OPENSSL_NO_SM4
#define OPENSSL_NO_SRP
#define OPENSSL_NO_SSL_TRACE
#define OPENSSL_NO_SSL2
#define OPENSSL_NO_SSL3
#define OPENSSL_NO_SSL3_METHOD
#define OPENSSL_NO_STATIC_ENGINE
#define OPENSSL_NO_STORE
#define OPENSSL_NO_WHIRLPOOL

// We do not implement OpenSSL's CMS API, except for a tiny subset. Projects
// targeting the tiny subset can define BORINGSSL_NO_NO_CMS to suppress
// OPENSSL_NO_CMS, to make it easier to compile code that expects OpenSSL. This
// option does not change what APIs are exposed by BoringSSL, only this macro.
#if !defined(BORINGSSL_NO_NO_CMS)
#define OPENSSL_NO_CMS
#endif

#endif  // OPENSSL_HEADER_OPENSSLCONF_H
