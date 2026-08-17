// Copyright 2024 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_SPAKE2PLUS_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_SPAKE2PLUS_INTERNAL_H

#include <openssl/base.h>

#include <sys/types.h>

#include <openssl/sha.h>
#include <openssl/span.h>

#include "../fipsmodule/ec/internal.h"


BSSL_NAMESPACE_BEGIN

// SPAKE2+.
//
// SPAKE2+ is an augmented password-authenticated key-exchange. It allows
// two parties, a prover and verifier, to derive a strong shared key with no
// risk of disclosing the password, known only to the prover, to the verifier.
// (But note that the verifier can still attempt an offline, brute-force attack
// to recover the password.)
//
// This is an implementation of SPAKE2+ using P-256 as the group, SHA-256 as
// the hash function, HKDF-SHA256 as the key derivation function, and
// HMAC-SHA256 as the message authentication code.
//
// See https://www.rfc-editor.org/rfc/rfc9383.html

namespace spake2plus {

// kShareSize is the size of a SPAKE2+ key share.
constexpr size_t kShareSize = 65;

// kConfirmSize is the size of a SPAKE2+ key confirmation message.
constexpr size_t kConfirmSize = 32;

// kVerifierSize is the size of the w0 and w1 values in the SPAKE2+ protocol.
constexpr size_t kVerifierSize = 32;

// kRegistrationRecordSize is the number of bytes in a registration record,
// which is provided to the verifier.
constexpr size_t kRegistrationRecordSize = 65;

// kSecretSize is the number of bytes of shared secret that the SPAKE2+ protocol
// generates.
constexpr size_t kSecretSize = 32;

// Register computes the values needed in the offline registration
// step of the SPAKE2+ protocol. See the following for more details:
// https://www.rfc-editor.org/rfc/rfc9383.html#section-3.2
//
// The |password| argument is the mandatory prover password. The |out_w0|,
// |out_w1|, and |out_registration_record| arguments are where the password
// verifiers (w0 and w1) and registration record (L) are stored, respectively.
// The prover is given |out_w0| and |out_w1| while the verifier is given
// |out_w0| and |out_registration_record|.
//
// To ensure success, |out_w0| and |out_w1| must be of length |kVerifierSize|,
// and |out_registration_record| of size |kRegistrationRecordSize|.
[[nodiscard]] OPENSSL_EXPORT bool Register(
    Span<uint8_t> out_w0, Span<uint8_t> out_w1,
    Span<uint8_t> out_registration_record, Span<const uint8_t> password,
    Span<const uint8_t> id_prover, Span<const uint8_t> id_verifier);

class OPENSSL_EXPORT Prover {
 public:
  static constexpr bool kAllowUniquePtr = true;

  Prover();
  ~Prover();

  // Init creates a new prover, which can only be used for a single execution of
  // the protocol.
  //
  // The |context| argument is an application-specific value meant to constrain
  // the protocol execution. The |w0| and |w1| arguments are password verifier
  // values computed during the offline registration phase of the protocol. The
  // |id_prover| and |id_verifier| arguments allow optional, opaque names to be
  // bound into the protocol. See the following for more information about how
  // these identities may be chosen:
  // https://www.rfc-editor.org/rfc/rfc9383.html#name-definition-of-spake2
  [[nodiscard]] bool Init(Span<const uint8_t> context,
                          Span<const uint8_t> id_prover,
                          Span<const uint8_t> id_verifier,
                          Span<const uint8_t> w0, Span<const uint8_t> w1,
                          Span<const uint8_t> x = Span<const uint8_t>());

  // GenerateShare computes a SPAKE2+ share and writes it to |out_share|.
  //
  // This function can only be called once for a given |Prover|. To ensure
  // success, |out_share| must be |kShareSize| bytes.
  [[nodiscard]] bool GenerateShare(Span<uint8_t> out_share);

  // ComputeConfirmation computes a SPAKE2+ key confirmation
  // message and writes it to |out_confirm|. It also computes the shared secret
  // and writes it to |out_secret|.
  //
  // This function can only be called once for a given |Prover|.
  //
  // To ensure success, |out_confirm| must be |kConfirmSize| bytes
  // and |out_secret| must be |kSecretSize| bytes.
  [[nodiscard]] bool ComputeConfirmation(Span<uint8_t> out_confirm,
                                         Span<uint8_t> out_secret,
                                         Span<const uint8_t> peer_share,
                                         Span<const uint8_t> peer_confirm);

 private:
  enum class State {
    kInit,
    kShareGenerated,
    kConfirmGenerated,
    kDone,
  };

  State state_ = State::kInit;
  SHA256_CTX transcript_hash_;
  EC_SCALAR w0_;
  EC_SCALAR w1_;
  EC_SCALAR x_;
  EC_AFFINE X_;
  uint8_t share_[kShareSize];
};

class OPENSSL_EXPORT Verifier {
 public:
  static constexpr bool kAllowUniquePtr = true;

  Verifier();
  ~Verifier();

  // Init creates a new verifier, which can only be used for a single execution
  // of the protocol.
  //
  // The |context| argument is an application-specific value meant to constrain
  // the protocol execution. The |w0| and |registration_record| arguments are
  // required, and are computed by the prover via |Register|. Only the prover
  // can produce |w0| and |registration_record|, as they require
  // knowledge of the password. The prover must securely transmit this to the
  // verifier out-of-band. The |id_prover| and |id_verifier| arguments allow
  // optional, opaque names to be bound into the protocol. See the following for
  // more information about how these identities may be chosen:
  // https://www.rfc-editor.org/rfc/rfc9383.html#name-definition-of-spake2
  [[nodiscard]] bool Init(Span<const uint8_t> context,
                          Span<const uint8_t> id_prover,
                          Span<const uint8_t> id_verifier,
                          Span<const uint8_t> w0,
                          Span<const uint8_t> registration_record,
                          Span<const uint8_t> y = Span<const uint8_t>());

  // ProcessProverShare computes a SPAKE2+ share from an input share,
  // |prover_share|, and writes it to |out_share|. It also computes the key
  // confirmation message and writes it to |out_confirm|. Finally, it computes
  // the shared secret and writes it to |out_secret|.
  //
  // This function can only be called once for a given |Verifier|.
  //
  // To ensure success, |out_share| must be |kShareSize| bytes, |out_confirm|
  // must be |kConfirmSize| bytes, and |out_secret| must be |kSecretSize| bytes.
  [[nodiscard]] bool ProcessProverShare(Span<uint8_t> out_share,
                                        Span<uint8_t> out_confirm,
                                        Span<uint8_t> out_secret,
                                        Span<const uint8_t> prover_share);

  // VerifyProverConfirmation verifies a SPAKE2+ key confirmation message,
  // |prover_confirm|.
  //
  // This function can only be called once for a given |Verifier|.
  [[nodiscard]] bool VerifyProverConfirmation(Span<const uint8_t> peer_confirm);

 private:
  enum class State {
    kInit,
    kProverShareSeen,
    kDone,
  };

  State state_ = State::kInit;
  SHA256_CTX transcript_hash_;
  EC_SCALAR w0_;
  EC_AFFINE L_;
  EC_SCALAR y_;
  uint8_t confirm_[kConfirmSize];
};

}  // namespace spake2plus

BSSL_NAMESPACE_END

#endif  // OPENSSL_HEADER_CRYPTO_SPAKE2PLUS_INTERNAL_H
