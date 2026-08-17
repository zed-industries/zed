// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CRYPTO_HISTOGRAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CRYPTO_HISTOGRAMS_H_

#include "third_party/blink/public/platform/web_crypto.h"

// -------------------------------------------------------------
// Overview
// -------------------------------------------------------------
//
// The high level goal is to measure the popularity of WebCrypto methods and
// algorithms. This data will inform future optimizations and deprecation.
//
// This is accomplished by decomposing each call to crypto.* into a set of
// "features".
//
// These features include the name of the method, and the algorithm name. For
// the complete list refer to UseCounter.h (features are prefixed by Crypto* or
// SubtleCrypto*).
//
// This approach allows answering questions like:
//   * "How often was SHA1 used?"
//   * "How often was crypto.subtle.wrapKey() called?"
//
// Note that for practical matters, features are counted independently. So we
// cannot answer questions that span multiple features like:
//     "How often was SHA1 used when doing ECDSA signing?"
// But we could separately answer each of:
//   * "How often was SHA1 used?"
//   * "How often was ECDSA used?"
//   * "How often was crypto.subtle.sign() used?"
//
// Lastly, to explore the results of this instrumentation use the public
// dashboard (www.chromestatus.com), or the (internal) UMA histogram
// dashboard.
//
// -------------------------------------------------------------
// Features
// -------------------------------------------------------------
//
// WebCrypto operations are complex and have many parameters. The important
// parameters are distilled to a set of features, the most important being:
//
//   * The name of the operation (i.e. "crypto.subtle.encrypt",
//   "crypto.subtle.exportKey")
//   * The name of the algorithm (i.e. "AES-GCM", "RSA-PSS", "SHA-1")
//
// In the case where the algorithm makes use of an inner hash function (for
// instance HMAC), that hash algorithm is also considered as having been used.
// This means that when reading usage counters for hash functions, they are NOT
// exclusive to crypto.subtle.digest().
//
// Here are some examples to illustrate how it works:
//
//  * Consumer calls crypto.subtle.encrypt() using a 128-bit AES-GCM key:
//    ==> Increment SubtleCryptoEncrypt
//    ==> Increment CryptAlgorithmAesGcm
//
//  * Consumer calls crypto.subtle.sign() using an HMAC key bound to SHA1:
//    ==> Increment SubtleCryptoSign
//    ==> Increment CryptoAlgorithmHmac
//    ==> Increment CryptoAlgorithmSha1
//
//  * Consumer calls crypto.subtle.wrapKey() using a 1024 bit RSA-OAEP key with
//  SHA-256 as the wrapping key, and wrapping a ECDSA P-521 SHA-512 key:
//    ==> Increment SubtleCryptoWrapKey
//    ==> Increment CryptoAlgorithmRsaOaep
//    ==> Increment CryptoAlgorithmSha256
//    ==> Increment CryptoAlgorithmEcdsa
//    ==> Increment CryptoAlgorithmSha512
//
//  (Note that the algorithm parameters of the key being exported/wrapped are
//  also recorded for consistency)
//
//  * Consumer calls crypto.subtle.exportKey() on a 2048 bit RSA-PSS key
//  using SHA-512:
//    ==> Increment SubtleCryptoExportKey
//    ==> Increment CryptoAlgorithmRsaPss
//    ==> Increment CryptoAlgorithmSha512
//
//  (Note that even though the key is just being exported, all of its
//  algorithm parameters are also recorded)
//
// -----------------------
// Caveats
// -----------------------
//
//   * Measurements when there are errors (the Promise is rejected) are
//   inconsistent.
//
//   In some cases the underlying UseCounter is incremented, in
//   others it isn't. It depends how early the error occured. For instance if
//   an error was thrown by the binding layer, then no usage will be recorded
//   for the algorithm/key in question.
//
//   Most of these early errors correspond with caller errors of WebCrypto so
//   they shouldn't skew the stats.
//
//   But it is important to realize that UNSUPPORTED ALGORITHMS WILL similarly
//   error early on and not count the usage.
namespace blink {

class ExecutionContext;
class WebCryptoAlgorithm;
class WebCryptoKey;

// Log the usage of a particular WebCryptoAlgorithm (i.e. operation).
void HistogramAlgorithm(ExecutionContext*, const WebCryptoAlgorithm&);

// Log the usage of a particular WebCryptoKey.
void HistogramKey(ExecutionContext*, const WebCryptoKey&);

// This is a convenience function for calling histogramAlgorithm() and
// histogramKey().
void HistogramAlgorithmAndKey(ExecutionContext*,
                              const WebCryptoAlgorithm&,
                              const WebCryptoKey&);

BLINK_EXPORT void HistogramDeriveBitsTruncation(ExecutionContext*,
                                                std::optional<unsigned int>,
                                                WebCryptoWarningType);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CRYPTO_HISTOGRAMS_H_
