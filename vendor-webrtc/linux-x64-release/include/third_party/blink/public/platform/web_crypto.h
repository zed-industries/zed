/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_H_

#include <string_view>
#include <vector>

#include "base/containers/span.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_crypto_algorithm.h"
#include "third_party/blink/public/platform/web_crypto_key.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"

#if INSIDE_BLINK
#include "base/memory/scoped_refptr.h"
#endif

namespace blink {

class CryptoResult;
class CryptoResultCancel;
class WebString;
class ExecutionContext;

enum WebCryptoErrorType {
  kWebCryptoErrorTypeType,
  kWebCryptoErrorTypeNotSupported,
  kWebCryptoErrorTypeSyntax,
  kWebCryptoErrorTypeInvalidAccess,
  kWebCryptoErrorTypeData,
  kWebCryptoErrorTypeOperation,
};

enum WebCryptoWarningType {
  kWebCryptoWarningTypeNone,
  kWebCryptoWarningTypeDeriveBitsTruncated,
};

class BLINK_PLATFORM_EXPORT WebCryptoResult {
 public:
  WebCryptoResult(const WebCryptoResult& o) { Assign(o); }

  ~WebCryptoResult() { Reset(); }

  WebCryptoResult& operator=(const WebCryptoResult& o) {
    Assign(o);
    return *this;
  }

  // Note that WebString is NOT safe to pass across threads.
  //
  // Error details are surfaced in an exception, and MUST NEVER reveal any
  // secret information such as bytes of the key or plain text. An
  // appropriate error would be something like:
  //   "iv must be 16 bytes long".
  void CompleteWithError(WebCryptoErrorType, const WebString&);

  // Makes a copy of the input data given as a span of bytes.
  void CompleteWithBuffer(base::span<const uint8_t>);
  void CompleteWithJson(std::string_view);
  void CompleteWithBoolean(bool);
  void CompleteWithKey(const WebCryptoKey&);
  void CompleteWithKeyPair(const WebCryptoKey& public_key,
                           const WebCryptoKey& private_key);

  // Returns true if the underlying operation was cancelled.
  // This method can be called from any thread.
  bool Cancelled() const;

  ExecutionContext* GetExecutionContext() const;

#if INSIDE_BLINK
  WebCryptoResult(CryptoResult*, scoped_refptr<CryptoResultCancel>);
#endif

 private:
  void Reset();
  void Assign(const WebCryptoResult&);

  WebPrivatePtrForGC<CryptoResult, WebPrivatePtrDestruction::kCrossThread>
      impl_;
  WebPrivatePtrForRefCounted<CryptoResultCancel,
                             WebPrivatePtrDestruction::kCrossThread>
      cancel_;
};

class WebCrypto {
 public:
  // WebCrypto is the interface for starting one-shot cryptographic
  // operations.
  //
  // -----------------------
  // Completing the request
  // -----------------------
  //
  // Implementations signal completion by calling one of the methods on
  // "result". Only a single result/error should be set for the request.
  // Different operations expect different result types based on the
  // algorithm parameters; see the Web Crypto standard for details.
  //
  // The result can be set either synchronously while handling the request,
  // or asynchronously after the method has returned. When completing
  // asynchronously make a copy of the WebCryptoResult and call it from the
  // |result_task_runner| TaskRunner.
  //
  // If the request was cancelled it is not necessary for implementations to
  // set the result.
  //
  // -----------------------
  // Threading
  // -----------------------
  //
  // The WebCrypto interface will be called from blink threads (main or
  // web worker). All communication back to Blink must be on this same thread
  // (|result_task_runner|).
  //
  // Notably:
  //
  //   * The WebCryptoResult can be copied between threads, however all
  //     methods other than the destructor must be called from the origin
  //     Blink thread (|result_task_runner|).
  //
  //   * WebCryptoKey and WebCryptoAlgorithm ARE threadsafe. They can be
  //     safely copied between threads and accessed. Copying is cheap because
  //     they are internally reference counted.
  //
  // -----------------------
  // Inputs
  // -----------------------
  //
  //   * Data buffers are transferred as std::vectors. Implementations are free
  //     to re-use or transfer their storage.
  //
  //   * All WebCryptoKeys are guaranteeed to be !IsNull().
  //
  //   * All WebCryptoAlgorithms are guaranteed to be !IsNull()
  //
  //   * Look to the Web Crypto spec for an explanation of the parameter. The
  //     method names here have a 1:1 correspondence with those of
  //     crypto.subtle, with the exception of "verify" which is here called
  //     "verifySignature".
  //
  // -----------------------
  // Guarantees on input validity
  // -----------------------
  //
  // Implementations MUST carefully sanitize algorithm inputs before using
  // them, as they come directly from the user. Few checks have been done on
  // algorithm parameters prior to passing to the embedder.
  //
  // Only the following checks can be assumed as having already passed:
  //
  //  * The key is extractable when calling into ExportKey/WrapKey.
  //  * The key usages permit the operation being requested.
  //  * The key's algorithm matches that of the requested operation.
  //
  virtual void Encrypt(
      const WebCryptoAlgorithm&,
      const WebCryptoKey&,
      std::vector<unsigned char> data,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void Decrypt(
      const WebCryptoAlgorithm&,
      const WebCryptoKey&,
      std::vector<unsigned char> data,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void Sign(const WebCryptoAlgorithm&,
                    const WebCryptoKey&,
                    std::vector<unsigned char> data,
                    WebCryptoResult result,
                    scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void VerifySignature(
      const WebCryptoAlgorithm&,
      const WebCryptoKey&,
      std::vector<unsigned char> signature,
      std::vector<unsigned char> data,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void Digest(const WebCryptoAlgorithm&,
                      std::vector<unsigned char> data,
                      WebCryptoResult result,
                      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void GenerateKey(
      const WebCryptoAlgorithm&,
      bool extractable,
      WebCryptoKeyUsageMask,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void ImportKey(
      WebCryptoKeyFormat,
      std::vector<unsigned char> key_data,
      const WebCryptoAlgorithm&,
      bool extractable,
      WebCryptoKeyUsageMask,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void ExportKey(
      WebCryptoKeyFormat,
      const WebCryptoKey&,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void WrapKey(
      WebCryptoKeyFormat,
      const WebCryptoKey& key,
      const WebCryptoKey& wrapping_key,
      const WebCryptoAlgorithm&,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void UnwrapKey(
      WebCryptoKeyFormat,
      std::vector<unsigned char> wrapped_key,
      const WebCryptoKey&,
      const WebCryptoAlgorithm& unwrap_algorithm,
      const WebCryptoAlgorithm& unwrapped_key_algorithm,
      bool extractable,
      WebCryptoKeyUsageMask,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void DeriveBits(
      const WebCryptoAlgorithm&,
      const WebCryptoKey&,
      std::optional<unsigned> length,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }
  virtual void DeriveKey(
      const WebCryptoAlgorithm& algorithm,
      const WebCryptoKey& base_key,
      const WebCryptoAlgorithm& import_algorithm,
      const WebCryptoAlgorithm& key_length_algorithm,
      bool extractable,
      WebCryptoKeyUsageMask,
      WebCryptoResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    result.CompleteWithError(kWebCryptoErrorTypeNotSupported, "");
  }

  // -----------------------
  // Structured clone
  // -----------------------
  //
  // DeserializeKeyForClone() and SerializeKeyForClone() are used for
  // implementing structured cloning of WebCryptoKey.
  //
  // Blink is responsible for saving and restoring all of the attributes of
  // WebCryptoKey EXCEPT for the actual key data:
  //
  // In other words, Blink takes care of serializing:
  //   * Key usages
  //   * Key extractability
  //   * Key algorithm
  //   * Key type (public, private, secret)
  //
  // The embedder is responsible for saving the key data itself.
  //
  // Visibility of the serialized key data:
  //
  // The serialized key data will NOT be visible to web pages. So if the
  // serialized format were to include key bytes as plain text, this wouldn't
  // make it available to web pages.
  //
  // Longevity of the key data:
  //
  // The serialized key data is intended to be long lived (years) and MUST
  // be using a stable format. For instance a key might be persisted to
  // IndexedDB and should be able to be deserialized correctly in the
  // future.
  //
  // Error handling and asynchronous completion:
  //
  // Serialization/deserialization must complete synchronously, and will
  // block the JavaScript thread.
  //
  // The only reasons to fail serialization/deserialization are:
  //   * Key serialization not yet implemented
  //   * The bytes to deserialize were corrupted

  // Creates a new key given key data which was written using
  // SerializeKeyForClone(). Returns true on success.
  virtual bool DeserializeKeyForClone(const WebCryptoKeyAlgorithm&,
                                      WebCryptoKeyType,
                                      bool extractable,
                                      WebCryptoKeyUsageMask,
                                      base::span<const unsigned char> key_data,
                                      WebCryptoKey&) {
    return false;
  }

  // Writes the key data into the given std::vector.
  // Returns true on success.
  virtual bool SerializeKeyForClone(const WebCryptoKey&,
                                    std::vector<unsigned char>&) {
    return false;
  }

 protected:
  virtual ~WebCrypto() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_H_
