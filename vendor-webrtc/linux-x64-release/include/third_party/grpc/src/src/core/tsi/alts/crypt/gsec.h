//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TSI_ALTS_CRYPT_GSEC_H
#define GRPC_SRC_CORE_TSI_ALTS_CRYPT_GSEC_H

#include <assert.h>
#include <grpc/event_engine/port.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <stdlib.h>

#include <memory>
#include <vector>

#include "absl/types/span.h"

namespace grpc_core {

// Provides accessors to all information representing a gsec key. Owns all the
// read-only or mutable buffers via the accessors. Mutable accessors such as
// `aead_key()` or `kdf_buffer()` are NOT thread-safe.
class GsecKeyInterface {
 public:
  virtual ~GsecKeyInterface() = default;

  virtual bool IsRekey() = 0;
  virtual absl::Span<const uint8_t> key() = 0;

  // All the accessors below should only be called when IsRekey() is true.
  virtual absl::Span<uint8_t> aead_key() = 0;
  virtual absl::Span<const uint8_t> nonce_mask() = 0;
  virtual absl::Span<uint8_t> kdf_counter() = 0;
  virtual absl::Span<uint8_t> kdf_buffer() = 0;
};

// Interface for a GsecKey factory.
class GsecKeyFactoryInterface {
 public:
  virtual ~GsecKeyFactoryInterface() = default;

  // Creates identical and independent GsecKeyInterface objects.
  virtual std::unique_ptr<GsecKeyInterface> Create() const = 0;
};

class GsecKeyFactory : public GsecKeyFactoryInterface {
 public:
  // The exact given parameters will be used to create GsecKey objects.
  GsecKeyFactory(absl::Span<const uint8_t> key, bool is_rekey);
  ~GsecKeyFactory() override = default;

  std::unique_ptr<GsecKeyInterface> Create() const override;

 private:
  std::vector<uint8_t> key_;
  bool is_rekey_;
};

class GsecKey : public GsecKeyInterface {
 public:
  // If `is_rekey` is false, the given key will be copied as the symmetric key.
  // Otherwise, part of the given key is copied as the KDF key and another part
  // is copied as the nonce mask.
  GsecKey(absl::Span<const uint8_t> key, bool is_rekey);
  ~GsecKey() override = default;

  bool IsRekey() override;
  absl::Span<const uint8_t> key() override;
  absl::Span<uint8_t> aead_key() override;
  absl::Span<const uint8_t> nonce_mask() override;
  absl::Span<uint8_t> kdf_counter() override;
  absl::Span<uint8_t> kdf_buffer() override;

 private:
  bool is_rekey_;
  std::vector<uint8_t> key_;
  std::vector<uint8_t> aead_key_;
  std::vector<uint8_t> kdf_buffer_;
  std::vector<uint8_t> nonce_mask_;
  std::vector<uint8_t> kdf_counter_;
};

}  // namespace grpc_core

#ifndef _STRUCT_IOVEC
#if !defined(GRPC_EVENT_ENGINE_POSIX)
struct iovec {
  void* iov_base;
  size_t iov_len;
};
#endif  // GRPC_EVENT_ENGINE_POSIX
#endif  // _STRUCT_IOVEC

//
// A gsec interface for AEAD encryption schemes. The API is thread-compatible.
// Each implementation of this interface should specify supported values for
// key, nonce, and tag lengths.
//

// Key, nonce, and tag length in bytes
const size_t kAesGcmNonceLength = 12;
const size_t kAesGcmTagLength = 16;
const size_t kAes128GcmKeyLength = 16;
const size_t kAes256GcmKeyLength = 32;

// The first 32 bytes are used as a KDF key and the remaining 12 bytes are used
// to mask the nonce.
const size_t kAes128GcmRekeyKeyLength = 44;

typedef struct gsec_aead_crypter gsec_aead_crypter;

//
// The gsec_aead_crypter is an API for different AEAD implementations such as
// AES_GCM. It encapsulates all AEAD-related operations in the format of
// V-table that stores pointers to functions implementing those operations.
// It also provides helper functions to wrap each of those function pointers.
//
// A typical usage of this object would be:
//
//------------------------------------------------------------------------------
//// Declare a gsec_aead_crypter object, and create and assign an instance
//// of specific AEAD implementation e.g., AES_GCM to it. We assume both
//// key and nonce contain cryptographically secure random bytes, and the key
//// can be derived from an upper-layer application.
// gsec_aead_crypter* crypter;
// char* error_in_creation;
//// User can populate the message with any 100 bytes data.
// uint8_t* message = gpr_malloc(100);
// grpc_status_code creation_status = gsec_aes_gcm_aead_crypter_create(key,
//                                                     kAes128GcmKeyLength,
//                                                     kAesGcmNonceLength,
//                                                     kAesGcmTagLength,
//                                                     &crypter,
//                                                     false,
//                                                     0
//                                                     &error_in_creation);
//
// if (creation_status == GRPC_STATUS_OK) {
//   // Allocate a correct amount of memory to hold a ciphertext.
//   size_t clength = 0;
//   gsec_aead_crypter_max_ciphertext_and_tag_length(crypter, 100, &clength,
//                                                   nullptr);
//   uint8_t* ciphertext = gpr_malloc(clength);
//
//   // Perform encryption
//   size_t num_encrypted_bytes = 0;
//   char* error_in_encryption = nullptr;
//   grpc_status_code status = gsec_aead_crypter_encrypt(crypter, nonce,
//                                                       kAesGcmNonceLength,
//                                                       nullptr, 0, message,
//                                                       100, ciphertext,
//                                                       clength,
//                                                       &num_encrypted_bytes,
//                                                       &error_in_encryption);
// if (status == GRPC_STATUS_OK) {
//      // Allocate a correct amount of memory to hold a plaintext.
//      size_t plength = 0;
//      gsec_aead_crypter_max_plaintext_length(crypter, num_encrypted_bytes,
//                                             &plength, nullptr);
//      uint8_t* plaintext = gpr_malloc(plength);
//
//      // Perform decryption.
//      size_t num_decrypted_bytes = 0;
//      char* error_in_decryption = nullptr;
//      status = gsec_aead_crypter_decrypt(crypter, nonce,
//                                         kAesGcmNonceLength, nullptr, 0,
//                                         ciphertext, num_encrypted_bytes,
//                                         plaintext, plength,
//                                         &num_decrypted_bytes,
//                                         &error_in_decryption);
//      if (status != GRPC_STATUS_OK) {
//        fprintf(stderr, "AEAD decrypt operation failed with error code:"
//                        "%d, message: %s\n", status, error_in_decryption);
//      }
//      ...
//      gpr_free(plaintext);
//      gpr_free(error_in_decryption);
//   } else {
//       fprintf(stderr, "AEAD encrypt operation failed with error code:"
//                       "%d, message: %s\n", status, error_in_encryption);
//   }
//   ...
//   gpr_free(ciphertext);
//   gpr_free(error_in_encryption);
//} else {
//  fprintf(stderr, "Creation of AEAD crypter instance failed with error code:"
//                  "%d, message: %s\n", creation_status, error_in_creation);
//}
//
//// Destruct AEAD crypter instance.
// if (creation_status == GRPC_STATUS_OK) {
//  gsec_aead_crypter_destroy(crypter);
//}
// gpr_free(error_in_creation);
// gpr_free(message);
//-----------------------------------------------------------------------------
//

// V-table for gsec AEAD operations
typedef struct gsec_aead_crypter_vtable {
  grpc_status_code (*encrypt_iovec)(
      gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
      const struct iovec* aad_vec, size_t aad_vec_length,
      const struct iovec* plaintext_vec, size_t plaintext_vec_length,
      struct iovec ciphertext_vec, size_t* ciphertext_bytes_written,
      char** error_details);
  grpc_status_code (*decrypt_iovec)(
      gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
      const struct iovec* aad_vec, size_t aad_vec_length,
      const struct iovec* ciphertext_vec, size_t ciphertext_vec_length,
      struct iovec plaintext_vec, size_t* plaintext_bytes_written,
      char** error_details);
  grpc_status_code (*max_ciphertext_and_tag_length)(
      const gsec_aead_crypter* crypter, size_t plaintext_length,
      size_t* max_ciphertext_and_tag_length_to_return, char** error_details);
  grpc_status_code (*max_plaintext_length)(
      const gsec_aead_crypter* crypter, size_t ciphertext_and_tag_length,
      size_t* max_plaintext_length_to_return, char** error_details);
  grpc_status_code (*nonce_length)(const gsec_aead_crypter* crypter,
                                   size_t* nonce_length_to_return,
                                   char** error_details);
  grpc_status_code (*key_length)(const gsec_aead_crypter* crypter,
                                 size_t* key_length_to_return,
                                 char** error_details);
  grpc_status_code (*tag_length)(const gsec_aead_crypter* crypter,
                                 size_t* tag_length_to_return,
                                 char** error_details);
  void (*destruct)(gsec_aead_crypter* crypter);
} gsec_aead_crypter_vtable;

// Main struct for gsec interface
struct gsec_aead_crypter {
  const struct gsec_aead_crypter_vtable* vtable;
};

//
// This method performs an AEAD encrypt operation.
//
//- crypter: AEAD crypter instance.
//- nonce: buffer containing a nonce with its size equal to nonce_length.
//- nonce_length: size of nonce buffer, and must be equal to the value returned
//  from method gsec_aead_crypter_nonce_length.
//- aad: buffer containing data that needs to be authenticated but not
//  encrypted with its size equal to aad_length.
//- aad_length: size of aad buffer, which should be zero if the buffer is
//  nullptr.
//- plaintext: buffer containing data that needs to be both encrypted and
//  authenticated with its size equal to plaintext_length.
//- plaintext_length: size of plaintext buffer, which should be zero if
//  plaintext is nullptr.
//- ciphertext_and_tag: buffer that will contain ciphertext and tags the method
//  produced. The buffer should not overlap the plaintext buffer, and pointers
//  to those buffers should not be equal. Also if the ciphertext+tag buffer is
//  nullptr, the plaintext_length should be zero.
//- ciphertext_and_tag_length: size of ciphertext+tag buffer, which should be
//  at least as long as the one returned from method
//  gsec_aead_crypter_max_ciphertext_and_tag_length.
//- bytes_written: the actual number of bytes written to the ciphertext+tag
//  buffer. If bytes_written is nullptr, the plaintext_length should be zero.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of encryption, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
//
grpc_status_code gsec_aead_crypter_encrypt(
    gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
    const uint8_t* aad, size_t aad_length, const uint8_t* plaintext,
    size_t plaintext_length, uint8_t* ciphertext_and_tag,
    size_t ciphertext_and_tag_length, size_t* bytes_written,
    char** error_details);

//
// This method performs an AEAD encrypt operation.
//
//- crypter: AEAD crypter instance.
//- nonce: buffer containing a nonce with its size equal to nonce_length.
//- nonce_length: size of nonce buffer, and must be equal to the value returned
//  from method gsec_aead_crypter_nonce_length.
//- aad_vec: an iovec array containing data that needs to be authenticated but
//  not encrypted.
//- aad_vec_length: the array length of aad_vec.
//- plaintext_vec: an iovec array containing data that needs to be both
//  encrypted and authenticated.
//- plaintext_vec_length: the array length of plaintext_vec.
//- ciphertext_vec: an iovec containing a ciphertext buffer. The buffer should
//  not overlap the plaintext buffer.
//- ciphertext_bytes_written: the actual number of bytes written to
//  ciphertext_vec.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of encryption, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
//
grpc_status_code gsec_aead_crypter_encrypt_iovec(
    gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
    const struct iovec* aad_vec, size_t aad_vec_length,
    const struct iovec* plaintext_vec, size_t plaintext_vec_length,
    struct iovec ciphertext_vec, size_t* ciphertext_bytes_written,
    char** error_details);

//
// This method performs an AEAD decrypt operation.
//
//- crypter: AEAD crypter instance.
//- nonce: buffer containing a nonce with its size equal to nonce_length.
//- nonce_length: size of nonce buffer, and must be equal to the value returned
//  from method gsec_aead_crypter_nonce_length.
//- aad: buffer containing data that needs to be authenticated only.
//- aad_length: size of aad buffer, which should be zero if the buffer is
//  nullptr.
//- ciphertext_and_tag: buffer containing ciphertext and tag.
//- ciphertext_and_tag_length: length of ciphertext and tag. It should be zero
//  if any of plaintext, ciphertext_and_tag, or bytes_written is nullptr. Also,
//  ciphertext_and_tag_length should be at least as large as the tag length set
//  at AEAD crypter instance construction time.
//- plaintext: buffer containing decrypted and authenticated data the method
//  produced. The buffer should not overlap with the ciphertext+tag buffer, and
//  pointers to those buffers should not be equal.
//- plaintext_length: size of plaintext buffer, which should be at least as
//  long as the one returned from gsec_aead_crypter_max_plaintext_length
//  method.
//- bytes_written: the actual number of bytes written to the plaintext
//  buffer.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of decryption, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_decrypt(
    gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
    const uint8_t* aad, size_t aad_length, const uint8_t* ciphertext_and_tag,
    size_t ciphertext_and_tag_length, uint8_t* plaintext,
    size_t plaintext_length, size_t* bytes_written, char** error_details);

//
// This method performs an AEAD decrypt operation.
//
//- crypter: AEAD crypter instance.
//- nonce: buffer containing a nonce with its size equal to nonce_length.
//- nonce_length: size of nonce buffer, and must be equal to the value returned
//  from method gsec_aead_crypter_nonce_length.
//- aad_vec: an iovec array containing data that needs to be authenticated but
//  not encrypted.
//- aad_vec_length: the array length of aad_vec.
//- ciphertext_vec: an iovec array containing the ciphertext and tag.
//- ciphertext_vec_length: the array length of ciphertext_vec.
//- plaintext_vec: an iovec containing a plaintext buffer. The buffer should
//  not overlap the ciphertext buffer.
//- plaintext_bytes_written: the actual number of bytes written to
//  plaintext_vec.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of decryption, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_decrypt_iovec(
    gsec_aead_crypter* crypter, const uint8_t* nonce, size_t nonce_length,
    const struct iovec* aad_vec, size_t aad_vec_length,
    const struct iovec* ciphertext_vec, size_t ciphertext_vec_length,
    struct iovec plaintext_vec, size_t* plaintext_bytes_written,
    char** error_details);

//
// This method computes the size of ciphertext+tag buffer that must be passed
// to gsec_aead_crypter_encrypt function to ensure correct encryption of a
// plaintext. The actual size of ciphertext+tag written to the buffer could be
// smaller.
//
//- crypter: AEAD crypter instance.
//- plaintext_length: length of plaintext.
//- max_ciphertext_and_tag_length_to_return: the size of ciphertext+tag buffer
//  the method returns.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of execution, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_max_ciphertext_and_tag_length(
    const gsec_aead_crypter* crypter, size_t plaintext_length,
    size_t* max_ciphertext_and_tag_length_to_return, char** error_details);

//
// This method computes the size of plaintext buffer that must be passed to
// gsec_aead_crypter_decrypt function to ensure correct decryption of a
// ciphertext. The actual size of plaintext written to the buffer could be
// smaller.
//
//- crypter: AEAD crypter instance.
//- ciphertext_and_tag_length: length of ciphertext and tag.
//- max_plaintext_length_to_return: the size of plaintext buffer the method
//  returns.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of execution, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_max_plaintext_length(
    const gsec_aead_crypter* crypter, size_t ciphertext_and_tag_length,
    size_t* max_plaintext_length_to_return, char** error_details);

//
// This method returns a valid size of nonce array used at the construction of
// AEAD crypter instance. It is also the size that should be passed to encrypt
// and decrypt methods executed on the instance.
//
//- crypter: AEAD crypter instance.
//- nonce_length_to_return: the length of nonce array the method returns.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of execution, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_nonce_length(
    const gsec_aead_crypter* crypter, size_t* nonce_length_to_return,
    char** error_details);

//
// This method returns a valid size of key array used at the construction of
// AEAD crypter instance. It is also the size that should be passed to encrypt
// and decrypt methods executed on the instance.
//
//- crypter: AEAD crypter instance.
//- key_length_to_return: the length of key array the method returns.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of execution, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_key_length(const gsec_aead_crypter* crypter,
                                              size_t* key_length_to_return,
                                              char** error_details);
//
// This method returns a valid size of tag array used at the construction of
// AEAD crypter instance. It is also the size that should be passed to encrypt
// and decrypt methods executed on the instance.
//
//- crypter: AEAD crypter instance.
//- tag_length_to_return: the length of tag array the method returns.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On the success of execution, the method returns GRPC_STATUS_OK. Otherwise,
// it returns an error status code along with its details specified in
// error_details (if error_details is not nullptr).
//
grpc_status_code gsec_aead_crypter_tag_length(const gsec_aead_crypter* crypter,
                                              size_t* tag_length_to_return,
                                              char** error_details);

//
// This method destroys an AEAD crypter instance by de-allocating all of its
// occupied memory.
//
//- crypter: AEAD crypter instance that needs to be destroyed.
//
void gsec_aead_crypter_destroy(gsec_aead_crypter* crypter);

//
// This method creates an AEAD crypter instance of AES-GCM encryption scheme
// which supports 16 and 32 bytes long keys, 12 and 16 bytes long nonces, and
// 16 bytes long tags. It should be noted that once the lengths of key, nonce,
// and tag are determined at construction time, they cannot be modified later.
//
//- key: an instance of GsecKeyInterface, which will provide accessors to the
//  key, aead_key, kdf_counter, nonce_mask as well as the buffer for aead_key
//  derivation. It also tells whether rekey is supported.
//- nonce_length: length of a nonce in bytes, which should be either 12 or 16.
//- tag_length: length of a tag in bytes, which should be always 16.
//- crypter: address of AES_GCM crypter instance returned from the method.
//- error_details: a buffer containing an error message if the method does not
//  function correctly. It is legal to pass nullptr into error_details, and
//  otherwise, the parameter should be freed with gpr_free.
//
// On success of instance creation, it stores the address of instance at
// crypter. Otherwise, it returns an error status code together with its
// details specified in error_details.
//
grpc_status_code gsec_aes_gcm_aead_crypter_create(
    std::unique_ptr<grpc_core::GsecKeyInterface> key, size_t nonce_length,
    size_t tag_length, gsec_aead_crypter** crypter, char** error_details);

#endif  // GRPC_SRC_CORE_TSI_ALTS_CRYPT_GSEC_H
