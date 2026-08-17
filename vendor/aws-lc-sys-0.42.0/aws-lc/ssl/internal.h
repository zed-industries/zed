// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
// Copyright 2005 Nokia. All rights reserved.
//
// ECC cipher suite support in OpenSSL originally developed by
// SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_SSL_INTERNAL_H
#define OPENSSL_HEADER_SSL_INTERNAL_H

#include <openssl/base.h>

#include <stdlib.h>

#include <algorithm>
#include <bitset>
#include <initializer_list>
#include <limits>
#include <new>
#include <type_traits>
#include <utility>

#include <openssl/aead.h>
#include <openssl/curve25519.h>
#include <openssl/err.h>
#include <openssl/hpke.h>
#include <openssl/lhash.h>
#include <openssl/mem.h>
#include <openssl/span.h>
#include <openssl/ssl.h>
#include <openssl/stack.h>

#include "../crypto/err/internal.h"
#include "../crypto/internal.h"
#include "../crypto/lhash/internal.h"


#if defined(OPENSSL_WINDOWS)
// Windows defines struct timeval in winsock2.h.
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#else
#include <sys/time.h>
#endif


#include <atomic>

typedef int SSL_stats_t;

// Using ATOMIC_INT_LOCK_FREE as SSL_stats_t is an int
#if !defined(OPENSSL_STATS_C11_ATOMIC) && defined(OPENSSL_THREADS) && ATOMIC_INT_LOCK_FREE == 2
#define OPENSSL_STATS_C11_ATOMIC
#endif

// Define the actual storage type for statistics counters
#if defined(OPENSSL_STATS_C11_ATOMIC)
#define SSL_STATS_COUNTER_TYPE std::atomic<SSL_stats_t>
#else
#define SSL_STATS_COUNTER_TYPE SSL_stats_t
#endif

BSSL_NAMESPACE_BEGIN

struct SSL_CONFIG;
struct SSL_HANDSHAKE;
struct SSL_PROTOCOL_METHOD;
struct SSL_X509_METHOD;

// C++ utilities.

// New behaves like |new| but uses |OPENSSL_malloc| for memory allocation. It
// returns nullptr on allocation error. It only implements single-object
// allocation and not new T[n].
//
// Note: unlike |new|, this does not support non-public constructors.
template <typename T, typename... Args>
T *New(Args &&...args) {
  void *t = OPENSSL_malloc(sizeof(T));
  if (t == nullptr) {
    return nullptr;
  }
  return new (t) T(std::forward<Args>(args)...);
}

// Delete behaves like |delete| but uses |OPENSSL_free| to release memory.
//
// Note: unlike |delete| this does not support non-public destructors.
template <typename T>
void Delete(T *t) {
  if (t != nullptr) {
    t->~T();
    OPENSSL_free(t);
  }
}

// All types with kAllowUniquePtr set may be used with UniquePtr. Other types
// may be C structs which require a |BORINGSSL_MAKE_DELETER| registration.
namespace internal {
template <typename T>
struct DeleterImpl<T, typename std::enable_if<T::kAllowUniquePtr>::type> {
  static void Free(T *t) { Delete(t); }
};
}  // namespace internal

// MakeUnique behaves like |std::make_unique| but returns nullptr on allocation
// error.
template <typename T, typename... Args>
UniquePtr<T> MakeUnique(Args &&...args) {
  return UniquePtr<T>(New<T>(std::forward<Args>(args)...));
}

#if defined(BORINGSSL_ALLOW_CXX_RUNTIME)
#define HAS_VIRTUAL_DESTRUCTOR
#define PURE_VIRTUAL = 0
#else
// HAS_VIRTUAL_DESTRUCTOR should be declared in any base class which defines a
// virtual destructor. This avoids a dependency on |_ZdlPv| and prevents the
// class from being used with |delete|.
#define HAS_VIRTUAL_DESTRUCTOR \
  void operator delete(void *) { abort(); }

// PURE_VIRTUAL should be used instead of = 0 when defining pure-virtual
// functions. This avoids a dependency on |__cxa_pure_virtual| but loses
// compile-time checking.
#define PURE_VIRTUAL \
  { abort(); }
#endif

// CONSTEXPR_ARRAY works around a VS 2015 bug where ranged for loops don't work
// on constexpr arrays.
#if defined(_MSC_VER) && !defined(__clang__) && _MSC_VER < 1910
#define CONSTEXPR_ARRAY const
#else
#define CONSTEXPR_ARRAY constexpr
#endif

// Array<T> is an owning array of elements of |T|.
template <typename T>
class Array {
 public:
  // Array's default constructor creates an empty array.
  Array() {}
  Array(const Array &) = delete;
  Array(Array &&other) { *this = std::move(other); }

  ~Array() { Reset(); }

  Array &operator=(const Array &) = delete;
  Array &operator=(Array &&other) {
    Reset();
    other.Release(&data_, &size_);
    return *this;
  }

  const T *data() const { return data_; }
  T *data() { return data_; }
  size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }

  const T &operator[](size_t i) const { return data_[i]; }
  T &operator[](size_t i) { return data_[i]; }

  T *begin() { return data_; }
  const T *begin() const { return data_; }
  T *end() { return data_ + size_; }
  const T *end() const { return data_ + size_; }

  void Reset() { Reset(nullptr, 0); }

  // Reset releases the current contents of the array and takes ownership of the
  // raw pointer supplied by the caller.
  void Reset(T *new_data, size_t new_size) {
    for (size_t i = 0; i < size_; i++) {
      data_[i].~T();
    }
    OPENSSL_free(data_);
    data_ = new_data;
    size_ = new_size;
  }

  // Release releases ownership of the array to a raw pointer supplied by the
  // caller.
  void Release(T **out, size_t *out_size) {
    *out = data_;
    *out_size = size_;
    data_ = nullptr;
    size_ = 0;
  }

  // Init replaces the array with a newly-allocated array of |new_size|
  // default-constructed copies of |T|. It returns true on success and false on
  // error.
  //
  // Note that if |T| is a primitive type like |uint8_t|, it is uninitialized.
  bool Init(size_t new_size) {
    Reset();
    if (new_size == 0) {
      return true;
    }

    if (new_size > std::numeric_limits<size_t>::max() / sizeof(T)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
      return false;
    }
    data_ = reinterpret_cast<T *>(OPENSSL_malloc(new_size * sizeof(T)));
    if (data_ == nullptr) {
      return false;
    }
    size_ = new_size;
    for (size_t i = 0; i < size_; i++) {
      new (&data_[i]) T;
    }
    return true;
  }

  // CopyFrom replaces the array with a newly-allocated copy of |in|. It returns
  // true on success and false on error.
  bool CopyFrom(Span<const T> in) {
    if (!Init(in.size())) {
      return false;
    }
    std::copy(in.begin(), in.end(), data_);
    return true;
  }

  // Shrink shrinks the stored size of the array to |new_size|. It crashes if
  // the new size is larger. Note this does not shrink the allocation itself.
  void Shrink(size_t new_size) {
    if (new_size > size_) {
      abort();
    }
    for (size_t i = new_size; i < size_; i++) {
      data_[i].~T();
    }
    size_ = new_size;
  }

 private:
  T *data_ = nullptr;
  size_t size_ = 0;
};

// GrowableArray<T> is an array that owns elements of |T|, backed by an
// Array<T>. When necessary, pushing will automatically trigger a resize.
//
// Note, for simplicity, this class currently differs from |std::vector| in that
// |T| must be efficiently default-constructible. Allocated elements beyond the
// end of the array are constructed and destructed.
template <typename T>
class GrowableArray {
 public:
  GrowableArray() = default;
  GrowableArray(const GrowableArray &) = delete;
  GrowableArray(GrowableArray &&other) { *this = std::move(other); }
  ~GrowableArray() {}

  GrowableArray &operator=(const GrowableArray &) = delete;
  GrowableArray &operator=(GrowableArray &&other) {
    size_ = other.size_;
    other.size_ = 0;
    array_ = std::move(other.array_);
    return *this;
  }

  const T *data() const { return array_.data(); }
  T *data() { return array_.data(); }
  size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }

  const T &operator[](size_t i) const { return array_[i]; }
  T &operator[](size_t i) { return array_[i]; }

  T *begin() { return array_.data(); }
  const T *begin() const { return array_.data(); }
  T *end() { return array_.data() + size_; }
  const T *end() const { return array_.data() + size_; }

  void clear() {
    size_ = 0;
    array_.Reset();
  }

  // Push adds |elem| at the end of the internal array, growing if necessary. It
  // returns false when allocation fails.
  bool Push(T elem) {
    if (!MaybeGrow()) {
      return false;
    }
    array_[size_] = std::move(elem);
    size_++;
    return true;
  }

  // CopyFrom replaces the contents of the array with a copy of |in|. It returns
  // true on success and false on allocation error.
  bool CopyFrom(Span<const T> in) {
    if (!array_.CopyFrom(in)) {
      return false;
    }
    size_ = in.size();
    return true;
  }

 private:
  // If there is no room for one more element, creates a new backing array with
  // double the size of the old one and copies elements over.
  bool MaybeGrow() {
    if (array_.size() == 0) {
      return array_.Init(kDefaultSize);
    }
    // No need to grow if we have room for one more T.
    if (size_ < array_.size()) {
      return true;
    }
    // Double the array's size if it's safe to do so.
    if (array_.size() > std::numeric_limits<size_t>::max() / 2) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
      return false;
    }
    Array<T> new_array;
    if (!new_array.Init(array_.size() * 2)) {
      return false;
    }
    for (size_t i = 0; i < array_.size(); i++) {
      new_array[i] = std::move(array_[i]);
    }
    array_ = std::move(new_array);

    return true;
  }

  // |size_| is the number of elements stored in this GrowableArray.
  size_t size_ = 0;
  // |array_| is the backing array. Note that |array_.size()| is this
  // GrowableArray's current capacity and that |size_ <= array_.size()|.
  Array<T> array_;
  // |kDefaultSize| is the default initial size of the backing array.
  static constexpr size_t kDefaultSize = 16;
};

// CBBFinishArray behaves like |CBB_finish| but stores the result in an Array.
OPENSSL_EXPORT bool CBBFinishArray(CBB *cbb, Array<uint8_t> *out);

OPENSSL_EXPORT UniquePtr<CRYPTO_BUFFER> x509_to_buffer(X509 *x509);

// GetAllNames helps to implement |*_get_all_*_names| style functions. It
// writes at most |max_out| string pointers to |out| and returns the number that
// it would have liked to have written. The strings written consist of
// |fixed_names_len| strings from |fixed_names| followed by |objects_len|
// strings taken by projecting |objects| through |name|.
template <typename T, typename Name>
inline size_t GetAllNames(const char **out, size_t max_out,
                          Span<const char *const> fixed_names, Name(T::*name),
                          Span<const T> objects) {
  auto span = bssl::MakeSpan(out, max_out);
  for (size_t i = 0; !span.empty() && i < fixed_names.size(); i++) {
    span[0] = fixed_names[i];
    span = span.subspan(1);
  }
  span = span.subspan(0, objects.size());
  for (size_t i = 0; i < span.size(); i++) {
    span[i] = objects[i].*name;
  }
  return fixed_names.size() + objects.size();
}

// RefCounted is a common base for ref-counted types. This is an instance of the
// C++ curiously-recurring template pattern, so a type Foo must subclass
// RefCounted<Foo>. It additionally must friend RefCounted<Foo> to allow calling
// the destructor.
template <typename Derived>
class RefCounted {
 public:
  RefCounted(const RefCounted &) = delete;
  RefCounted &operator=(const RefCounted &) = delete;

  // These methods are intentionally named differently from `bssl::UpRef` to
  // avoid a collision. Only the implementations of `FOO_up_ref` and `FOO_free`
  // should call these.
  void UpRefInternal() { CRYPTO_refcount_inc(&references_); }
  void DecRefInternal() {
    if (CRYPTO_refcount_dec_and_test_zero(&references_)) {
      Derived *d = static_cast<Derived *>(this);
      d->~Derived();
      OPENSSL_free(d);
    }
  }

 protected:
  // Ensure that only `Derived`, which must inherit from `RefCounted<Derived>`,
  // can call the constructor. This catches bugs where someone inherited from
  // the wrong base.
  class CheckSubClass {
   private:
    friend Derived;
    CheckSubClass() = default;
  };
  RefCounted(CheckSubClass) {
    static_assert(std::is_base_of<RefCounted, Derived>::value,
                  "Derived must subclass RefCounted<Derived>");
  }

  ~RefCounted() = default;

 private:
  CRYPTO_refcount_t references_ = 1;
};


// Protocol versions.
//
// Due to DTLS's historical wire version differences, we maintain two notions of
// version.
//
// The "version" or "wire version" is the actual 16-bit value that appears on
// the wire. It uniquely identifies a version and is also used at API
// boundaries. The set of supported versions differs between TLS and DTLS. Wire
// versions are opaque values and may not be compared numerically.
//
// The "protocol version" identifies the high-level handshake variant being
// used. DTLS versions map to the corresponding TLS versions. Protocol versions
// are sequential and may be compared numerically.

// ssl_protocol_version_from_wire sets |*out| to the protocol version
// corresponding to wire version |version| and returns true. If |version| is not
// a valid TLS or DTLS version, it returns false.
//
// Note this simultaneously handles both DTLS and TLS. Use one of the
// higher-level functions below for most operations.
bool ssl_protocol_version_from_wire(uint16_t *out, uint16_t version);

// ssl_get_version_range sets |*out_min_version| and |*out_max_version| to the
// minimum and maximum enabled protocol versions, respectively.
bool ssl_get_version_range(const SSL_HANDSHAKE *hs, uint16_t *out_min_version,
                           uint16_t *out_max_version);

// ssl_supports_version returns whether |hs| supports |version|.
bool ssl_supports_version(const SSL_HANDSHAKE *hs, uint16_t version);

// ssl_method_supports_version returns whether |method| supports |version|.
bool ssl_method_supports_version(const SSL_PROTOCOL_METHOD *method,
                                 uint16_t version);

// ssl_add_supported_versions writes the supported versions of |hs| to |cbb|, in
// decreasing preference order. The version list is filtered to those whose
// protocol version is at least |extra_min_version|.
bool ssl_add_supported_versions(const SSL_HANDSHAKE *hs, CBB *cbb,
                                uint16_t extra_min_version);

// ssl_negotiate_version negotiates a common version based on |hs|'s preferences
// and the peer preference list in |peer_versions|. On success, it returns true
// and sets |*out_version| to the selected version. Otherwise, it returns false
// and sets |*out_alert| to an alert to send.
bool ssl_negotiate_version(SSL_HANDSHAKE *hs, uint8_t *out_alert,
                           uint16_t *out_version, const CBS *peer_versions);

// ssl_protocol_version returns |ssl|'s protocol version. It is an error to
// call this function before the version is determined.
uint16_t ssl_protocol_version(const SSL *ssl);

// Cipher suites.

BSSL_NAMESPACE_END

struct ssl_cipher_st {
  // name is the OpenSSL name for the cipher.
  const char *name;
  // standard_name is the IETF name for the cipher.
  const char *standard_name;
  // id is the cipher suite value bitwise OR-d with 0x03000000.
  uint32_t id;

  // algorithm_* determine the cipher suite. See constants below for the values.
  uint32_t algorithm_mkey;
  uint32_t algorithm_auth;
  uint32_t algorithm_enc;
  uint32_t algorithm_mac;
  uint32_t algorithm_prf;
};

// tls_print_all_supported_cipher_suites prints all supported cipher suites for
// all TLS versions to stdout.
OPENSSL_EXPORT bool tls_print_all_supported_cipher_suites(
    bool use_openssl_name);

BSSL_NAMESPACE_BEGIN

// Bits for |algorithm_mkey| (key exchange algorithm).
#define SSL_kRSA 0x00000001u
#define SSL_kECDHE 0x00000002u
// SSL_kPSK is only set for plain PSK, not ECDHE_PSK.
#define SSL_kPSK 0x00000004u
#define SSL_kGENERIC 0x00000008u

// Bits for |algorithm_auth| (server authentication).
#define SSL_aRSA 0x00000001u
#define SSL_aECDSA 0x00000002u
// SSL_aPSK is set for both PSK and ECDHE_PSK.
#define SSL_aPSK 0x00000004u
#define SSL_aGENERIC 0x00000008u

#define SSL_aCERT (SSL_aRSA | SSL_aECDSA)

// Bits for |algorithm_enc| (symmetric encryption).
#define SSL_3DES 0x00000001u
#define SSL_AES128 0x00000002u
#define SSL_AES256 0x00000004u
#define SSL_AES128GCM 0x00000008u
#define SSL_AES256GCM 0x00000010u
#define SSL_eNULL 0x00000020u
#define SSL_CHACHA20POLY1305 0x00000040u

#define SSL_AES (SSL_AES128 | SSL_AES256 | SSL_AES128GCM | SSL_AES256GCM)

// Bits for |algorithm_mac| (symmetric authentication).
#define SSL_SHA1 0x00000001u
// SSL_AEAD is set for all AEADs.
#define SSL_AEAD 0x00000002u
#define SSL_SHA256 0x00000003u
#define SSL_SHA384 0x00000004u

// Bits for |algorithm_prf| (handshake digest).
#define SSL_HANDSHAKE_MAC_DEFAULT 0x1
#define SSL_HANDSHAKE_MAC_SHA256 0x2
#define SSL_HANDSHAKE_MAC_SHA384 0x4

// SSL_MAX_MD_SIZE is size of the largest hash function used in TLS, SHA-384.
#define SSL_MAX_MD_SIZE 48

// An SSLCipherPreferenceList contains a list of SSL_CIPHERs with equal-
// preference groups. For TLS clients, the groups are moot because the server
// picks the cipher and groups cannot be expressed on the wire. However, for
// servers, the equal-preference groups allow the client's preferences to be
// partially respected. (This only has an effect with
// SSL_OP_CIPHER_SERVER_PREFERENCE). Note that the method used to compute the
// negotiated cipher suite in TLS 1.3 does not use the equal-preference
// data structure described below.
//
// The equal-preference groups are expressed by grouping SSL_CIPHERs together.
// All elements of a group have the same priority: no ordering is expressed
// within a group.
//
// The values in |ciphers| are in one-to-one correspondence with
// |in_group_flags|. (That is, sk_SSL_CIPHER_num(ciphers) is the number of
// bytes in |in_group_flags|.) The bytes in |in_group_flags| are either 1, to
// indicate that the corresponding SSL_CIPHER is not the last element of a
// group, or 0 to indicate that it is.
//
// For example, if |in_group_flags| contains all zeros then that indicates a
// traditional, fully-ordered preference. Every SSL_CIPHER is the last element
// of the group (i.e. they are all in a one-element group).
//
// For a more complex example, consider:
//   ciphers:        A  B  C  D  E  F
//   in_group_flags: 1  1  0  0  1  0
//
// That would express the following, order:
//
//    A         E
//    B -> D -> F
//    C
struct SSLCipherPreferenceList {
  static constexpr bool kAllowUniquePtr = true;

  SSLCipherPreferenceList() = default;
  ~SSLCipherPreferenceList();

  bool Init(UniquePtr<STACK_OF(SSL_CIPHER)> ciphers,
            Span<const bool> in_group_flags);
  bool Init(const SSLCipherPreferenceList &);

  void Remove(const SSL_CIPHER *cipher);

  UniquePtr<STACK_OF(SSL_CIPHER)> ciphers;
  bool *in_group_flags = nullptr;
};

// AllCiphers returns an array of all supported ciphers, sorted by id.
Span<const SSL_CIPHER> AllCiphers();

// ssl_cipher_get_evp_aead sets |*out_aead| to point to the correct EVP_AEAD
// object for |cipher| protocol version |version|. It sets |*out_mac_secret_len|
// and |*out_fixed_iv_len| to the MAC key length and fixed IV length,
// respectively. The MAC key length is zero except for legacy block and stream
// ciphers. It returns true on success and false on error.
bool ssl_cipher_get_evp_aead(const EVP_AEAD **out_aead,
                             size_t *out_mac_secret_len,
                             size_t *out_fixed_iv_len, const SSL_CIPHER *cipher,
                             uint16_t version, bool is_dtls);

// ssl_get_handshake_digest returns the |EVP_MD| corresponding to |version| and
// |cipher|.
const EVP_MD *ssl_get_handshake_digest(uint16_t version,
                                       const SSL_CIPHER *cipher);

// ssl_create_cipher_list evaluates |rule_str|. It sets |*out_cipher_list| to a
// newly-allocated |SSLCipherPreferenceList| containing the result. It returns
// true on success and false on failure. If |strict| is true, nonsense will be
// rejected. If false, nonsense will be silently ignored. If |config_tls13| is
// true, only TLS 1.3 ciphers are considered in |ssl_cipher_collect_ciphers|. If
// false, TLS 1.2 and below ciphers participate in |ssl_cipher_collect_ciphers|.
// An empty result is considered an error regardless of |strict| or
// |config_tls13|. |has_aes_hw| indicates if the list should be ordered based on
// having support for AES in hardware or not.
bool ssl_create_cipher_list(UniquePtr<SSLCipherPreferenceList> *out_cipher_list,
                            const bool has_aes_hw, const char *rule_str,
                            bool strict, bool config_tls13);

// update_cipher_list creates a new |SSLCipherPreferenceList| containing ciphers
// from both |ciphers| and |tls13_ciphers| and assigns it to |dst|. The function:
//
// 1. Creates a copy of |ciphers|
// 2. Removes any stale TLS 1.3 ciphersuites from the copy
// 3. Adds any configured TLS 1.3 ciphersuites from |tls13_ciphers| to the
// front of the list.
// 4. Combines |in_group_flags| from both input lists into |dst->in_group_flags|
//
// Returns one on success, zero on error.
int update_cipher_list(UniquePtr<SSLCipherPreferenceList> &dst,
                       UniquePtr<SSLCipherPreferenceList> &ciphers,
                       UniquePtr<SSLCipherPreferenceList> &tls13_ciphers);

// ssl_get_certificate_slot_index returns the |SSL_PKEY_*| certificate slot
// index corresponding to the private key type of |pkey|. It returns -1 if not
// supported. This was |ssl_cert_type| in OpenSSL 1.0.2.
int ssl_get_certificate_slot_index(const EVP_PKEY *pkey);

// ssl_cipher_auth_mask_for_key returns the mask of cipher |algorithm_auth|
// values suitable for use with |key| in TLS 1.2 and below.
uint32_t ssl_cipher_auth_mask_for_key(const EVP_PKEY *key);

// ssl_cipher_uses_certificate_auth returns whether |cipher| authenticates the
// server and, optionally, the client with a certificate.
bool ssl_cipher_uses_certificate_auth(const SSL_CIPHER *cipher);

// ssl_cipher_requires_server_key_exchange returns whether |cipher| requires a
// ServerKeyExchange message.
//
// This function may return false while still allowing |cipher| an optional
// ServerKeyExchange. This is the case for plain PSK ciphers.
bool ssl_cipher_requires_server_key_exchange(const SSL_CIPHER *cipher);

// ssl_cipher_get_record_split_len, for TLS 1.0 CBC mode ciphers, returns the
// length of an encrypted 1-byte record, for use in record-splitting. Otherwise
// it returns zero.
size_t ssl_cipher_get_record_split_len(const SSL_CIPHER *cipher);

// ssl_choose_tls13_cipher returns an |SSL_CIPHER| corresponding with the best
// available from |client_cipher_suites| compatible with |version| and
// configured |tls13_ciphers|. It returns NULL if there isn't a compatible
// cipher. |has_aes_hw| indicates if the choice should be made as if support for
// AES in hardware is available.
const SSL_CIPHER *ssl_choose_tls13_cipher(
    const STACK_OF(SSL_CIPHER) *client_cipher_suites, bool has_aes_hw, uint16_t version,
    const STACK_OF(SSL_CIPHER) *tls13_ciphers);


// Transcript layer.

// SSLTranscript maintains the handshake transcript as a combination of a
// buffer and running hash.
class SSLTranscript {
 public:
  SSLTranscript();
  ~SSLTranscript();

  SSLTranscript(SSLTranscript &&other) = default;
  SSLTranscript &operator=(SSLTranscript &&other) = default;

  // Init initializes the handshake transcript. If called on an existing
  // transcript, it resets the transcript and hash. It returns true on success
  // and false on failure.
  bool Init();

  // InitHash initializes the handshake hash based on the PRF and contents of
  // the handshake transcript. Subsequent calls to |Update| will update the
  // rolling hash. It returns one on success and zero on failure. It is an error
  // to call this function after the handshake buffer is released. This may be
  // called multiple times to change the hash function.
  bool InitHash(uint16_t version, const SSL_CIPHER *cipher);

  // UpdateForHelloRetryRequest resets the rolling hash with the
  // HelloRetryRequest construction. It returns true on success and false on
  // failure. It is an error to call this function before the handshake buffer
  // is released.
  bool UpdateForHelloRetryRequest();

  // CopyToHashContext initializes |ctx| with |digest| and the data thus far in
  // the transcript. It returns true on success and false on failure. If the
  // handshake buffer is still present, |digest| may be any supported digest.
  // Otherwise, |digest| must match the transcript hash.
  bool CopyToHashContext(EVP_MD_CTX *ctx, const EVP_MD *digest) const;

  Span<const uint8_t> buffer() const {
    return MakeConstSpan(reinterpret_cast<const uint8_t *>(buffer_->data),
                         buffer_->length);
  }

  // FreeBuffer releases the handshake buffer. Subsequent calls to
  // |Update| will not update the handshake buffer.
  void FreeBuffer();

  // DigestLen returns the length of the PRF hash.
  size_t DigestLen() const;

  // Digest returns the PRF hash. For TLS 1.1 and below, this is
  // |EVP_md5_sha1|.
  const EVP_MD *Digest() const;

  // Update adds |in| to the handshake buffer and handshake hash, whichever is
  // enabled. It returns true on success and false on failure.
  bool Update(Span<const uint8_t> in);

  // GetHash writes the handshake hash to |out| which must have room for at
  // least |DigestLen| bytes. On success, it returns true and sets |*out_len| to
  // the number of bytes written. Otherwise, it returns false.
  bool GetHash(uint8_t *out, size_t *out_len) const;

  // GetFinishedMAC computes the MAC for the Finished message into the bytes
  // pointed by |out| and writes the number of bytes to |*out_len|. |out| must
  // have room for |EVP_MAX_MD_SIZE| bytes. It returns true on success and false
  // on failure.
  bool GetFinishedMAC(uint8_t *out, size_t *out_len, const SSL_SESSION *session,
                      bool from_server) const;

 private:
  // buffer_, if non-null, contains the handshake transcript.
  UniquePtr<BUF_MEM> buffer_;
  // hash, if initialized with an |EVP_MD|, maintains the handshake hash.
  ScopedEVP_MD_CTX hash_;
};

// tls1_prf computes the PRF function for |ssl|. It fills |out|, using |secret|
// as the secret and |label| as the label. |seed1| and |seed2| are concatenated
// to form the seed parameter. It returns true on success and false on failure.
bool tls1_prf(const EVP_MD *digest, Span<uint8_t> out,
              Span<const uint8_t> secret, Span<const char> label,
              Span<const uint8_t> seed1, Span<const uint8_t> seed2);


// Encryption layer.

// SSLAEADContext contains information about an AEAD that is being used to
// encrypt an SSL connection.
class SSLAEADContext {
 public:
  SSLAEADContext(uint16_t version, bool is_dtls, const SSL_CIPHER *cipher);
  ~SSLAEADContext();
  static constexpr bool kAllowUniquePtr = true;

  SSLAEADContext(const SSLAEADContext &&) = delete;
  SSLAEADContext &operator=(const SSLAEADContext &&) = delete;

  // CreateNullCipher creates an |SSLAEADContext| for the null cipher.
  static UniquePtr<SSLAEADContext> CreateNullCipher(bool is_dtls);

  // Create creates an |SSLAEADContext| using the supplied key material. It
  // returns nullptr on error. Only one of |Open| or |Seal| may be used with the
  // resulting object, depending on |direction|. |version| is the normalized
  // protocol version, so DTLS 1.0 is represented as 0x0301, not 0xffef.
  static UniquePtr<SSLAEADContext> Create(enum evp_aead_direction_t direction,
                                          uint16_t version, bool is_dtls,
                                          const SSL_CIPHER *cipher,
                                          Span<const uint8_t> enc_key,
                                          Span<const uint8_t> mac_key,
                                          Span<const uint8_t> fixed_iv);

  // CreatePlaceholderForQUIC creates a placeholder |SSLAEADContext| for the
  // given cipher and version. The resulting object can be queried for various
  // properties but cannot encrypt or decrypt data.
  static UniquePtr<SSLAEADContext> CreatePlaceholderForQUIC(
      uint16_t version, const SSL_CIPHER *cipher);

  // SetVersionIfNullCipher sets the version the SSLAEADContext for the null
  // cipher, to make version-specific determinations in the record layer prior
  // to a cipher being selected.
  void SetVersionIfNullCipher(uint16_t version);

  // ProtocolVersion returns the protocol version associated with this
  // SSLAEADContext. It can only be called once |version_| has been set to a
  // valid value.
  uint16_t ProtocolVersion() const;

  // RecordVersion returns the record version that should be used with this
  // SSLAEADContext for record construction and crypto.
  uint16_t RecordVersion() const;

  const SSL_CIPHER *cipher() const { return cipher_; }

  // is_null_cipher returns true if this is the null cipher.
  bool is_null_cipher() const { return !cipher_; }

  // ExplicitNonceLen returns the length of the explicit nonce.
  size_t ExplicitNonceLen() const;

  // MaxOverhead returns the maximum overhead of calling |Seal|.
  size_t MaxOverhead() const;

  // SuffixLen calculates the suffix length written by |SealScatter| and writes
  // it to |*out_suffix_len|. It returns true on success and false on error.
  // |in_len| and |extra_in_len| should equal the argument of the same names
  // passed to |SealScatter|.
  bool SuffixLen(size_t *out_suffix_len, size_t in_len,
                 size_t extra_in_len) const;

  // CiphertextLen calculates the total ciphertext length written by
  // |SealScatter| and writes it to |*out_len|. It returns true on success and
  // false on error. |in_len| and |extra_in_len| should equal the argument of
  // the same names passed to |SealScatter|.
  bool CiphertextLen(size_t *out_len, size_t in_len, size_t extra_in_len) const;

  // Open authenticates and decrypts |in| in-place. On success, it sets |*out|
  // to the plaintext in |in| and returns true.  Otherwise, it returns
  // false. The output will always be |ExplicitNonceLen| bytes ahead of |in|.
  bool Open(Span<uint8_t> *out, uint8_t type, uint16_t record_version,
            const uint8_t seqnum[8], Span<const uint8_t> header,
            Span<uint8_t> in);

  // Seal encrypts and authenticates |in_len| bytes from |in| and writes the
  // result to |out|. It returns true on success and false on error.
  //
  // If |in| and |out| alias then |out| + |ExplicitNonceLen| must be == |in|.
  bool Seal(uint8_t *out, size_t *out_len, size_t max_out, uint8_t type,
            uint16_t record_version, const uint8_t seqnum[8],
            Span<const uint8_t> header, const uint8_t *in, size_t in_len);

  // SealScatter encrypts and authenticates |in_len| bytes from |in| and splits
  // the result between |out_prefix|, |out| and |out_suffix|. It returns one on
  // success and zero on error.
  //
  // On successful return, exactly |ExplicitNonceLen| bytes are written to
  // |out_prefix|, |in_len| bytes to |out|, and |SuffixLen| bytes to
  // |out_suffix|.
  //
  // |extra_in| may point to an additional plaintext buffer. If present,
  // |extra_in_len| additional bytes are encrypted and authenticated, and the
  // ciphertext is written to the beginning of |out_suffix|. |SuffixLen| should
  // be used to size |out_suffix| accordingly.
  //
  // If |in| and |out| alias then |out| must be == |in|. Other arguments may not
  // alias anything.
  bool SealScatter(uint8_t *out_prefix, uint8_t *out, uint8_t *out_suffix,
                   uint8_t type, uint16_t record_version,
                   const uint8_t seqnum[8], Span<const uint8_t> header,
                   const uint8_t *in, size_t in_len, const uint8_t *extra_in,
                   size_t extra_in_len);

  bool GetIV(const uint8_t **out_iv, size_t *out_iv_len) const;

  int SerializeState(CBB *cbb) const;

  int DeserializeState(CBS *cbs) const;

 private:
  // GetAdditionalData returns the additional data, writing into |storage| if
  // necessary.
  Span<const uint8_t> GetAdditionalData(uint8_t storage[13], uint8_t type,
                                        uint16_t record_version,
                                        const uint8_t seqnum[8],
                                        size_t plaintext_len,
                                        Span<const uint8_t> header);

  const SSL_CIPHER *cipher_;
  ScopedEVP_AEAD_CTX ctx_;
  // fixed_nonce_ contains any bytes of the nonce that are fixed for all
  // records.
  uint8_t fixed_nonce_[12];
  uint8_t fixed_nonce_len_ = 0, variable_nonce_len_ = 0;
  // version_ is the wire version that should be used with this AEAD.
  uint16_t version_;
  // is_dtls_ is whether DTLS is being used with this AEAD.
  bool is_dtls_;
  // variable_nonce_included_in_record_ is true if the variable nonce
  // for a record is included as a prefix before the ciphertext.
  bool variable_nonce_included_in_record_ : 1;
  // random_variable_nonce_ is true if the variable nonce is
  // randomly generated, rather than derived from the sequence
  // number.
  bool random_variable_nonce_ : 1;
  // xor_fixed_nonce_ is true if the fixed nonce should be XOR'd into the
  // variable nonce rather than prepended.
  bool xor_fixed_nonce_ : 1;
  // omit_length_in_ad_ is true if the length should be omitted in the
  // AEAD's ad parameter.
  bool omit_length_in_ad_ : 1;
  // ad_is_header_ is true if the AEAD's ad parameter is the record header.
  bool ad_is_header_ : 1;
};


// DTLS replay bitmap.

// DTLS1_BITMAP maintains a sliding window of 64 sequence numbers to detect
// replayed packets. It should be initialized by zeroing every field.
struct DTLS1_BITMAP {
  // map is a bitset of sequence numbers that have been seen. Bit i corresponds
  // to |max_seq_num - i|.
  std::bitset<256> map;
  // max_seq_num is the largest sequence number seen so far as a 64-bit
  // integer.
  uint64_t max_seq_num = 0;
};


// Record layer.

// ssl_record_sequence_update increments the sequence number in |seq|. It
// returns true on success and false on wraparound.
bool ssl_record_sequence_update(uint8_t *seq, size_t seq_len);

// ssl_record_prefix_len returns the length of the prefix before the ciphertext
// of a record for |ssl|.
//
// TODO(davidben): Expose this as part of public API once the high-level
// buffer-free APIs are available.
size_t ssl_record_prefix_len(const SSL *ssl);

enum ssl_open_record_t {
  ssl_open_record_success,
  ssl_open_record_discard,
  ssl_open_record_partial,
  ssl_open_record_close_notify,
  ssl_open_record_error,
};

// tls_open_record decrypts a record from |in| in-place.
//
// If the input did not contain a complete record, it returns
// |ssl_open_record_partial|. It sets |*out_consumed| to the total number of
// bytes necessary. It is guaranteed that a successful call to |tls_open_record|
// will consume at least that many bytes.
//
// Otherwise, it sets |*out_consumed| to the number of bytes of input
// consumed. Note that input may be consumed on all return codes if a record was
// decrypted.
//
// On success, it returns |ssl_open_record_success|. It sets |*out_type| to the
// record type and |*out| to the record body in |in|. Note that |*out| may be
// empty.
//
// If a record was successfully processed but should be discarded, it returns
// |ssl_open_record_discard|.
//
// If a record was successfully processed but is a close_notify, it returns
// |ssl_open_record_close_notify|.
//
// On failure or fatal alert, it returns |ssl_open_record_error| and sets
// |*out_alert| to an alert to emit, or zero if no alert should be emitted.
enum ssl_open_record_t tls_open_record(SSL *ssl, uint8_t *out_type,
                                       Span<uint8_t> *out, size_t *out_consumed,
                                       uint8_t *out_alert, Span<uint8_t> in);

// dtls_open_record implements |tls_open_record| for DTLS. It only returns
// |ssl_open_record_partial| if |in| was empty and sets |*out_consumed| to
// zero. The caller should read one packet and try again.
enum ssl_open_record_t dtls_open_record(SSL *ssl, uint8_t *out_type,
                                        Span<uint8_t> *out,
                                        size_t *out_consumed,
                                        uint8_t *out_alert, Span<uint8_t> in);

// ssl_seal_align_prefix_len returns the length of the prefix before the start
// of the bulk of the ciphertext when sealing a record with |ssl|. Callers may
// use this to align buffers.
//
// Note when TLS 1.0 CBC record-splitting is enabled, this includes the one byte
// record and is the offset into second record's ciphertext. Thus sealing a
// small record may result in a smaller output than this value.
//
// TODO(davidben): Is this alignment valuable? Record-splitting makes this a
// mess.
size_t ssl_seal_align_prefix_len(const SSL *ssl);

// tls_seal_record seals a new record of type |type| and body |in| and writes it
// to |out|. At most |max_out| bytes will be written. It returns true on success
// and false on error. If enabled, |tls_seal_record| implements TLS 1.0 CBC
// 1/n-1 record splitting and may write two records concatenated.
//
// For a large record, the bulk of the ciphertext will begin
// |ssl_seal_align_prefix_len| bytes into out. Aligning |out| appropriately may
// improve performance. It writes at most |in_len| + |SSL_max_seal_overhead|
// bytes to |out|.
//
// |in| and |out| may not alias.
bool tls_seal_record(SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
                     uint8_t type, const uint8_t *in, size_t in_len);

enum dtls1_use_epoch_t {
  dtls1_use_previous_epoch,
  dtls1_use_current_epoch,
};

// dtls_max_seal_overhead returns the maximum overhead, in bytes, of sealing a
// record.
size_t dtls_max_seal_overhead(const SSL *ssl, enum dtls1_use_epoch_t use_epoch);

// dtls_seal_prefix_len returns the number of bytes of prefix to reserve in
// front of the plaintext when sealing a record in-place.
size_t dtls_seal_prefix_len(const SSL *ssl, enum dtls1_use_epoch_t use_epoch);

// dtls_seal_record implements |tls_seal_record| for DTLS. |use_epoch| selects
// which epoch's cipher state to use. Unlike |tls_seal_record|, |in| and |out|
// may alias but, if they do, |in| must be exactly |dtls_seal_prefix_len| bytes
// ahead of |out|.
bool dtls_seal_record(SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
                      uint8_t type, const uint8_t *in, size_t in_len,
                      enum dtls1_use_epoch_t use_epoch);

// ssl_process_alert processes |in| as an alert and updates |ssl|'s shutdown
// state. It returns one of |ssl_open_record_discard|, |ssl_open_record_error|,
// |ssl_open_record_close_notify|, or |ssl_open_record_fatal_alert| as
// appropriate.
enum ssl_open_record_t ssl_process_alert(SSL *ssl, uint8_t *out_alert,
                                         Span<const uint8_t> in);


// Private key operations.

// ssl_has_private_key returns whether |hs| has a private key configured.
bool ssl_has_private_key(const SSL_HANDSHAKE *hs);

// ssl_private_key_* perform the corresponding operation on
// |SSL_PRIVATE_KEY_METHOD|. If there is a custom private key configured, they
// call the corresponding function or |complete| depending on whether there is a
// pending operation. Otherwise, they implement the operation with
// |EVP_PKEY|.

enum ssl_private_key_result_t ssl_private_key_sign(
    SSL_HANDSHAKE *hs, uint8_t *out, size_t *out_len, size_t max_out,
    uint16_t sigalg, Span<const uint8_t> in);

enum ssl_private_key_result_t ssl_private_key_decrypt(SSL_HANDSHAKE *hs,
                                                      uint8_t *out,
                                                      size_t *out_len,
                                                      size_t max_out,
                                                      Span<const uint8_t> in);

// ssl_public_key_supports_signature_algorithm returns whether |hs|'s extracted
// public key supports |sigalg|.
bool ssl_public_key_supports_signature_algorithm(SSL_HANDSHAKE *hs,
                                                 uint16_t sigalg);

// ssl_public_key_supports_legacy_signature_algorithm is the tls1.0/1.1
// version of |ssl_public_key_supports_signature_algorithm|.
bool ssl_public_key_supports_legacy_signature_algorithm(uint16_t *out,
                                                        SSL_HANDSHAKE *hs);

// ssl_cert_private_keys_supports_signature_algorithm returns whether any of
// |hs|'s available private keys supports |sigalg|. If one does, we switch to
// using that private key and the corresponding certificate for the rest of the
// connection. |hs->local_pubkey| is also updated correspondingly.
//
// NOTE: Multiple certificate slots is only supported on the server side, when
// not using designated credentials.
bool ssl_cert_private_keys_supports_signature_algorithm(SSL_HANDSHAKE *hs,
                                                        uint16_t sigalg);

// ssl_cert_private_keys_supports_legacy_signature_algorithm is the tls1.0/1.1
// version of |ssl_cert_private_keys_supports_signature_algorithm|.
bool ssl_cert_private_keys_supports_legacy_signature_algorithm(
    uint16_t *out, SSL_HANDSHAKE *hs);

// ssl_public_key_verify verifies that the |signature| is valid for the public
// key |pkey| and input |in|, using the signature algorithm |sigalg|.
bool ssl_public_key_verify(SSL *ssl, Span<const uint8_t> signature,
                           uint16_t sigalg, EVP_PKEY *pkey,
                           Span<const uint8_t> in);


// Custom extensions

BSSL_NAMESPACE_END

// |SSL_CUSTOM_EXTENSION| is a structure that contains information about
// custom-extension callbacks. It is defined unnamespaced for compatibility with
// |STACK_OF(SSL_CUSTOM_EXTENSION)|.
typedef struct ssl_custom_extension {
  SSL_custom_ext_add_cb add_callback;
  void *add_arg;
  SSL_custom_ext_free_cb free_callback;
  SSL_custom_ext_parse_cb parse_callback;
  void *parse_arg;
  uint16_t value;
} SSL_CUSTOM_EXTENSION;

DEFINE_STACK_OF(SSL_CUSTOM_EXTENSION)

BSSL_NAMESPACE_BEGIN

void SSL_CUSTOM_EXTENSION_free(SSL_CUSTOM_EXTENSION *custom_extension);

int custom_ext_add_clienthello(SSL_HANDSHAKE *hs, CBB *extensions);
int custom_ext_parse_serverhello(SSL_HANDSHAKE *hs, int *out_alert,
                                 uint16_t value, const CBS *extension);
int custom_ext_parse_clienthello(SSL_HANDSHAKE *hs, int *out_alert,
                                 uint16_t value, const CBS *extension);
int custom_ext_add_serverhello(SSL_HANDSHAKE *hs, CBB *extensions);


// Key shares.

// SSLKeyShare abstracts over Diffie-Hellman-like key exchanges.
class SSLKeyShare {
 public:
  virtual ~SSLKeyShare() {}
  static constexpr bool kAllowUniquePtr = true;
  HAS_VIRTUAL_DESTRUCTOR

  // Create returns a SSLKeyShare instance for use with group |group_id| or
  // nullptr on error. Marked with OPENSSL_EXPORT to make it available for
  // unit tests.
  OPENSSL_EXPORT static UniquePtr<SSLKeyShare> Create(uint16_t group_id);

  // GroupID returns the group ID.
  virtual uint16_t GroupID() const PURE_VIRTUAL;

  // Offer generates a keypair and writes the public value to
  // |out_public_key|. It returns true on success and false on error.
  virtual bool Offer(CBB *out_public_key) PURE_VIRTUAL;

  // Accept performs a key exchange against the |peer_key| generated by |Offer|.
  // On success, it returns true, writes the public value to |out_public_key|,
  // and sets |*out_secret| to the shared secret. On failure, it returns false
  // and sets |*out_alert| to an alert to send to the peer.
  //
  // The default implementation calls |Offer| and then |Finish|, assuming a key
  // exchange protocol where the peers are symmetric.
  virtual bool Accept(CBB *out_public_key, Array<uint8_t> *out_secret,
                      uint8_t *out_alert, Span<const uint8_t> peer_key);

  // Finish performs a key exchange against the |peer_key| generated by
  // |Accept|. On success, it returns true and sets |*out_secret| to the shared
  // secret. On failure, it returns false and sets |*out_alert| to an alert to
  // send to the peer.
  virtual bool Finish(Array<uint8_t> *out_secret, uint8_t *out_alert,
                      Span<const uint8_t> peer_key) PURE_VIRTUAL;

  // SerializePrivateKey writes the private key to |out|, returning true if
  // successful and false otherwise. It should be called after |Offer|.
  virtual bool SerializePrivateKey(CBB *out) { return false; }

  // DeserializePrivateKey initializes the state of the key exchange from |in|,
  // returning true if successful and false otherwise.
  virtual bool DeserializePrivateKey(CBS *in) { return false; }
};

struct NamedGroup {
  int nid;
  uint16_t group_id;
  const char name[32], alias[32];
};

// NamedGroups returns all supported groups.
Span<const NamedGroup> NamedGroups();

// Hybrid groups adhere to the draft specification:
// https://datatracker.ietf.org/doc/html/draft-ietf-tls-hybrid-design
#define NUM_HYBRID_COMPONENTS 2
struct HybridGroup {
  // Just like regular groups, each hybrid group has a group ID that
  // identifies the group as a whole unit...
  uint16_t group_id;

  // ...and each component of the hybrid group has its own group ID
  uint16_t component_group_ids[NUM_HYBRID_COMPONENTS];
};

// HybridGroups returns all supported hybrid groups. A hybrid group will likely
// (but not necessarily) contain at least one PQ group. Marked with
// OPENSSL_EXPORT to make it available for unit tests.
OPENSSL_EXPORT Span<const HybridGroup> HybridGroups();

// PQGroups returns all supported post-quantum groups. A post-quantum
// group may be a hybrid group containing at least one PQ
// component or a standalone PQ group.
Span<const uint16_t> PQGroups();

// ssl_nid_to_group_id looks up the group corresponding to |nid|. On success, it
// sets |*out_group_id| to the group ID and returns true. Otherwise, it returns
// false.
bool ssl_nid_to_group_id(uint16_t *out_group_id, int nid);

// ssl_nid_to_group_id looks up the group corresponding to |group_id|. On
// success, it sets |*out_nid| to the group's nid and returns true. Otherwise,
// it returns false.
bool ssl_group_id_to_nid(uint16_t *out_nid, int group_id);

// ssl_name_to_group_id looks up the group corresponding to the |name| string of
// length |len|. On success, it sets |*out_group_id| to the group ID and returns
// true. Otherwise, it returns false.
bool ssl_name_to_group_id(uint16_t *out_group_id, const char *name, size_t len);

// ssl_group_id_to_nid returns the NID corresponding to |group_id| or
// |NID_undef| if unknown.
int ssl_group_id_to_nid(uint16_t group_id);


// Handshake messages.

struct SSLMessage {
  bool is_v2_hello;
  uint8_t type;
  CBS body;
  // raw is the entire serialized handshake message, including the TLS or DTLS
  // message header.
  CBS raw;
};

// SSL_MAX_HANDSHAKE_FLIGHT is the number of messages, including
// ChangeCipherSpec, in the longest handshake flight. Currently this is the
// client's second leg in a full handshake when client certificates, NPN, and
// Channel ID, are all enabled.
#define SSL_MAX_HANDSHAKE_FLIGHT 7

extern const uint8_t kHelloRetryRequest[SSL3_RANDOM_SIZE];
extern const uint8_t kTLS12DowngradeRandom[8];
extern const uint8_t kTLS13DowngradeRandom[8];
extern const uint8_t kJDK11DowngradeRandom[8];

// ssl_max_handshake_message_len returns the maximum number of bytes permitted
// in a handshake message for |ssl|.
size_t ssl_max_handshake_message_len(const SSL *ssl);

// tls_can_accept_handshake_data returns whether |ssl| is able to accept more
// data into handshake buffer.
bool tls_can_accept_handshake_data(const SSL *ssl, uint8_t *out_alert);

// tls_has_unprocessed_handshake_data returns whether there is buffered
// handshake data that has not been consumed by |get_message|.
bool tls_has_unprocessed_handshake_data(const SSL *ssl);

// tls_append_handshake_data appends |data| to the handshake buffer. It returns
// true on success and false on allocation failure.
bool tls_append_handshake_data(SSL *ssl, Span<const uint8_t> data);

// dtls_has_unprocessed_handshake_data behaves like
// |tls_has_unprocessed_handshake_data| for DTLS.
bool dtls_has_unprocessed_handshake_data(const SSL *ssl);

// tls_flush_pending_hs_data flushes any handshake plaintext data.
bool tls_flush_pending_hs_data(SSL *ssl);

struct DTLS_OUTGOING_MESSAGE {
  DTLS_OUTGOING_MESSAGE() {}
  DTLS_OUTGOING_MESSAGE(const DTLS_OUTGOING_MESSAGE &) = delete;
  DTLS_OUTGOING_MESSAGE &operator=(const DTLS_OUTGOING_MESSAGE &) = delete;

  void Clear();

  Array<uint8_t> data;
  uint16_t epoch = 0;
  bool is_ccs = false;
};

// dtls_clear_outgoing_messages releases all buffered outgoing messages.
void dtls_clear_outgoing_messages(SSL *ssl);


// Callbacks.

// ssl_do_info_callback calls |ssl|'s info callback, if set.
void ssl_do_info_callback(const SSL *ssl, int type, int value);

// ssl_do_msg_callback calls |ssl|'s message callback, if set.
void ssl_do_msg_callback(const SSL *ssl, int is_write, int content_type,
                         Span<const uint8_t> in);


// Transport buffers.

enum SSL_BUFFER_SERDE_VERSION {
  SSL_BUFFER_SERDE_VERSION_ONE = 1,
  SSL_BUFFER_SERDE_VERSION_TWO = 2
};

const unsigned kSSLBufferMaxSerDeVersion = SSL_BUFFER_SERDE_VERSION_TWO;

#define SSLBUFFER_READ_AHEAD_MIN_CAPACITY 512
#define SSLBUFFER_MAX_CAPACITY INT_MAX
class OPENSSL_EXPORT SSLBuffer {
 public:
  SSLBuffer() {}
  ~SSLBuffer() { Clear(); }

  SSLBuffer(const SSLBuffer &) = delete;
  SSLBuffer &operator=(const SSLBuffer &) = delete;

  uint8_t *buf_ptr() { return buf_; }
  uint8_t *data() { return buf_ + offset_; }
  size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }
  size_t cap() const { return cap_; }
  size_t buf_size() const { return buf_size_; }

  Span<uint8_t> span() { return MakeSpan(data(), size()); }

  Span<uint8_t> remaining() {
    return MakeSpan(data() + size(), cap() - size());
  }

  // Clear releases the buffer.
  void Clear();

  // EnsureCap ensures the buffer has capacity at least |new_cap|, aligned such
  // that data written after |header_len| is aligned to a
  // |SSL3_ALIGN_PAYLOAD|-byte boundary. It returns true on success and false
  // on error.
  bool EnsureCap(size_t header_len, size_t new_cap);

  // DidWrite extends the buffer by |len|. The caller must have filled in to
  // this point.
  void DidWrite(size_t len);

  // Consume consumes |len| bytes from the front of the buffer.  The memory
  // consumed will remain valid until the next call to |DiscardConsumed| or
  // |Clear|.
  void Consume(size_t len);

  // DiscardConsumed discards the consumed bytes from the buffer. If the buffer
  // is now empty, it releases memory used by it.
  void DiscardConsumed();

  // DoSerialization writes all fields into |cbb|.
  bool DoSerialization(CBB &cbb);

  // DoDeserialization recovers the states encoded via |DoSerialization|.
  bool DoDeserialization(CBS &in);

  bool SerializeBufferView(CBB &cbb, Span<uint8_t> &view);
  bool DeserializeBufferView(CBS &cbb, Span<uint8_t> &view);

 private:
  // buf_ is the memory allocated for this buffer.
  uint8_t *buf_ = nullptr;
  // buf_allocated_ is true if |buf_| points to allocated data and must be freed
  // or false if it points into |inline_buf_|.
  bool buf_allocated_ = false;
  // The total capacity requested for this buffer by |EnsureCap|.
  size_t buf_cap_ = 0;
  // buf_size_ is how much memory allocated for |buf_|. This is needed by
  // |DoSerializationV1|. This is the total size of the buffer with the requested capacity + padding.
  size_t buf_size_ = 0;
  // header length used to calculate initial offset
  size_t header_len_ = 0;
  // offset_ is the offset into |buf_| which the buffer contents start at, and is moved as contents are consumed
  int offset_ = 0;
  // size_ is the size of the buffer contents from |buf_| + |offset_|.
  int size_ = 0;
  // cap_ is how much memory beyond |buf_| + |offset_| is available.
  int cap_ = 0;
  // inline_buf_ is a static buffer for short reads.
  uint8_t inline_buf_[SSL3_RT_HEADER_LENGTH];

  // The V1 version has some intricacies were solved in later serialization versions.
  // This is mainly to capture if a V1 version was restored and whether it needs to be
  // re-serialized as that version.
  uint32_t max_serialization_version_ = SSL_BUFFER_SERDE_VERSION_TWO;

  bool DoSerializationV1(CBB &cbb);
  bool DoSerializationV2(CBB &cbb);

  bool DoDeserializationV1(CBS &in);
  bool DoDeserializationV2(CBS &in);

  bool ValidateBuffersState();
};

// ssl_read_buffer_extend_to extends the read buffer to the desired length. For
// TLS, it reads to the end of the buffer until the buffer is |len| bytes
// long. For DTLS, it reads a new packet and ignores |len|. It returns one on
// success, zero on EOF, and a negative number on error.
//
// It is an error to call |ssl_read_buffer_extend_to| in DTLS when the buffer is
// non-empty.
int ssl_read_buffer_extend_to(SSL *ssl, size_t len);

// ssl_handle_open_record handles the result of passing |ssl->s3->read_buffer|
// to a record-processing function. If |ret| is a success or if the caller
// should retry, it returns one and sets |*out_retry|. Otherwise, it returns <=
// 0.
int ssl_handle_open_record(SSL *ssl, bool *out_retry, ssl_open_record_t ret,
                           size_t consumed, uint8_t alert);

// ssl_write_buffer_flush flushes the write buffer to the transport. It returns
// one on success and <= 0 on error. For DTLS, whether or not the write
// succeeds, the write buffer will be cleared.
int ssl_write_buffer_flush(SSL *ssl);


// Certificate functions.

// ssl_has_certificate returns whether a certificate and private key are
// configured.
bool ssl_has_certificate(const SSL_HANDSHAKE *hs);

// ssl_parse_cert_chain parses a certificate list from |cbs| in the format used
// by a TLS Certificate message. On success, it advances |cbs| and returns
// true. Otherwise, it returns false and sets |*out_alert| to an alert to send
// to the peer.
//
// If the list is non-empty then |*out_chain| and |*out_pubkey| will be set to
// the certificate chain and the leaf certificate's public key
// respectively. Otherwise, both will be set to nullptr.
//
// If the list is non-empty and |out_leaf_sha256| is non-NULL, it writes the
// SHA-256 hash of the leaf to |out_leaf_sha256|.
bool ssl_parse_cert_chain(uint8_t *out_alert,
                          UniquePtr<STACK_OF(CRYPTO_BUFFER)> *out_chain,
                          UniquePtr<EVP_PKEY> *out_pubkey,
                          uint8_t *out_leaf_sha256, CBS *cbs,
                          CRYPTO_BUFFER_POOL *pool);

// ssl_add_cert_chain adds |hs->ssl|'s certificate chain to |cbb| in the format
// used by a TLS Certificate message. If there is no certificate chain, it emits
// an empty certificate list. It returns true on success and false on error.
bool ssl_add_cert_chain(SSL_HANDSHAKE *hs, CBB *cbb);

enum ssl_key_usage_t {
  key_usage_digital_signature = 0,
  key_usage_encipherment = 2,
};

// ssl_cert_check_key_usage parses the DER-encoded, X.509 certificate in |in|
// and returns true if doesn't specify a key usage or, if it does, if it
// includes |bit|. Otherwise it pushes to the error queue and returns false.
OPENSSL_EXPORT bool ssl_cert_check_key_usage(const CBS *in,
                                             enum ssl_key_usage_t bit);

// ssl_cert_parse_pubkey extracts the public key from the DER-encoded, X.509
// certificate in |in|. It returns an allocated |EVP_PKEY| or else returns
// nullptr and pushes to the error queue.
UniquePtr<EVP_PKEY> ssl_cert_parse_pubkey(const CBS *in);

// ssl_cert_parse_leaf_pubkey calls |ssl_cert_parse_pubkey| and extracts the
// public key from the first element of |chain|. It's expected that the first
// element of |chain| is the leaf certificate.
UniquePtr<EVP_PKEY> ssl_cert_parse_leaf_pubkey(STACK_OF(CRYPTO_BUFFER) *chain);

// ssl_parse_client_CA_list parses a CA list from |cbs| in the format used by a
// TLS CertificateRequest message. On success, it returns a newly-allocated
// |CRYPTO_BUFFER| list and advances |cbs|. Otherwise, it returns nullptr and
// sets |*out_alert| to an alert to send to the peer.
UniquePtr<STACK_OF(CRYPTO_BUFFER)> ssl_parse_client_CA_list(SSL *ssl,
                                                            uint8_t *out_alert,
                                                            CBS *cbs);

// ssl_has_client_CAs returns there are configured CAs.
bool ssl_has_client_CAs(const SSL_CONFIG *cfg);

// ssl_add_client_CA_list adds the configured CA list to |cbb| in the format
// used by a TLS CertificateRequest message. It returns true on success and
// false on error.
bool ssl_add_client_CA_list(SSL_HANDSHAKE *hs, CBB *cbb);

// ssl_check_leaf_certificate returns one if |pkey| and |leaf| are suitable as
// a server's leaf certificate for |hs|. Otherwise, it returns zero and pushes
// an error on the error queue.
bool ssl_check_leaf_certificate(SSL_HANDSHAKE *hs, EVP_PKEY *pkey,
                                const CRYPTO_BUFFER *leaf);

// ssl_on_certificate_selected is called once the certificate has been selected.
// It finalizes the certificate and initializes |hs->local_pubkey|. It returns
// true on success and false on error.
bool ssl_on_certificate_selected(SSL_HANDSHAKE *hs);

// ssl_handshake_load_local_pubkey loads |local_pubkey| in |hs| based on the
// current designated certificate.
bool ssl_handshake_load_local_pubkey(SSL_HANDSHAKE *hs);


// TLS 1.3 key derivation.

// tls13_init_key_schedule initializes the handshake hash and key derivation
// state, and incorporates the PSK. The cipher suite and PRF hash must have been
// selected at this point. It returns true on success and false on error.
bool tls13_init_key_schedule(SSL_HANDSHAKE *hs, Span<const uint8_t> psk);

// tls13_init_early_key_schedule initializes the handshake hash and key
// derivation state from |session| for use with 0-RTT. It returns one on success
// and zero on error.
bool tls13_init_early_key_schedule(SSL_HANDSHAKE *hs,
                                   const SSL_SESSION *session);

// tls13_advance_key_schedule incorporates |in| into the key schedule with
// HKDF-Extract. It returns true on success and false on error.
bool tls13_advance_key_schedule(SSL_HANDSHAKE *hs, Span<const uint8_t> in);

// tls13_set_traffic_key sets the read or write traffic keys to
// |traffic_secret|. The version and cipher suite are determined from |session|.
// It returns true on success and false on error.
bool tls13_set_traffic_key(SSL *ssl, enum ssl_encryption_level_t level,
                           enum evp_aead_direction_t direction,
                           const SSL_SESSION *session,
                           Span<const uint8_t> traffic_secret);

// tls13_derive_early_secret derives the early traffic secret. It returns true
// on success and false on error.
bool tls13_derive_early_secret(SSL_HANDSHAKE *hs);

// tls13_derive_handshake_secrets derives the handshake traffic secret. It
// returns true on success and false on error.
bool tls13_derive_handshake_secrets(SSL_HANDSHAKE *hs);

// tls13_rotate_traffic_key derives the next read or write traffic secret. It
// returns true on success and false on error.
bool tls13_rotate_traffic_key(SSL *ssl, enum evp_aead_direction_t direction);

// tls13_derive_application_secrets derives the initial application data traffic
// and exporter secrets based on the handshake transcripts and |master_secret|.
// It returns true on success and false on error.
bool tls13_derive_application_secrets(SSL_HANDSHAKE *hs);

// tls13_derive_resumption_secret derives the |resumption_secret|.
bool tls13_derive_resumption_secret(SSL_HANDSHAKE *hs);

// tls13_export_keying_material provides an exporter interface to use the
// |exporter_secret|.
bool tls13_export_keying_material(SSL *ssl, Span<uint8_t> out,
                                  Span<const uint8_t> secret,
                                  Span<const char> label,
                                  Span<const uint8_t> context);

// tls13_finished_mac calculates the MAC of the handshake transcript to verify
// the integrity of the Finished message, and stores the result in |out| and
// length in |out_len|. |is_server| is true if this is for the Server Finished
// and false for the Client Finished.
bool tls13_finished_mac(SSL_HANDSHAKE *hs, uint8_t *out, size_t *out_len,
                        bool is_server);

// tls13_derive_session_psk calculates the PSK for this session based on the
// resumption master secret and |nonce|. It returns true on success, and false
// on failure.
bool tls13_derive_session_psk(SSL_SESSION *session, Span<const uint8_t> nonce);

// tls13_write_psk_binder calculates the PSK binder value over |transcript| and
// |msg|, and replaces the last bytes of |msg| with the resulting value. It
// returns true on success, and false on failure. If |out_binder_len| is
// non-NULL, it sets |*out_binder_len| to the length of the value computed.
bool tls13_write_psk_binder(const SSL_HANDSHAKE *hs,
                            const SSLTranscript &transcript, Span<uint8_t> msg,
                            size_t *out_binder_len);

// tls13_verify_psk_binder verifies that the handshake transcript, truncated up
// to the binders has a valid signature using the value of |session|'s
// resumption secret. It returns true on success, and false on failure.
bool tls13_verify_psk_binder(const SSL_HANDSHAKE *hs,
                             const SSL_SESSION *session, const SSLMessage &msg,
                             CBS *binders);


// Encrypted ClientHello.

struct ECHConfig {
  static constexpr bool kAllowUniquePtr = true;
  // raw contains the serialized ECHConfig.
  Array<uint8_t> raw;
  // The following fields alias into |raw|.
  Span<const uint8_t> public_key;
  Span<const uint8_t> public_name;
  Span<const uint8_t> cipher_suites;
  uint16_t kem_id = 0;
  uint8_t maximum_name_length = 0;
  uint8_t config_id = 0;
};

class ECHServerConfig {
 public:
  static constexpr bool kAllowUniquePtr = true;
  ECHServerConfig() = default;
  ECHServerConfig(const ECHServerConfig &other) = delete;
  ECHServerConfig &operator=(ECHServerConfig &&) = delete;

  // Init parses |ech_config| as an ECHConfig and saves a copy of |key|.
  // It returns true on success and false on error.
  bool Init(Span<const uint8_t> ech_config, const EVP_HPKE_KEY *key,
            bool is_retry_config);

  // SetupContext sets up |ctx| for a new connection, given the specified
  // HPKE ciphersuite and encapsulated KEM key. It returns true on success and
  // false on error. This function may only be called on an initialized object.
  bool SetupContext(EVP_HPKE_CTX *ctx, uint16_t kdf_id, uint16_t aead_id,
                    Span<const uint8_t> enc) const;

  const ECHConfig &ech_config() const { return ech_config_; }
  bool is_retry_config() const { return is_retry_config_; }

 private:
  ECHConfig ech_config_;
  ScopedEVP_HPKE_KEY key_;
  bool is_retry_config_ = false;
};

enum ssl_client_hello_type_t {
  ssl_client_hello_unencrypted,
  ssl_client_hello_inner,
  ssl_client_hello_outer,
};

// ECH_CLIENT_* are types for the ClientHello encrypted_client_hello extension.
#define ECH_CLIENT_OUTER 0
#define ECH_CLIENT_INNER 1

// ssl_decode_client_hello_inner recovers the full ClientHelloInner from the
// EncodedClientHelloInner |encoded_client_hello_inner| by replacing its
// outer_extensions extension with the referenced extensions from the
// ClientHelloOuter |client_hello_outer|. If successful, it writes the recovered
// ClientHelloInner to |out_client_hello_inner|. It returns true on success and
// false on failure.
//
// This function is exported for fuzzing.
OPENSSL_EXPORT bool ssl_decode_client_hello_inner(
    SSL *ssl, uint8_t *out_alert, Array<uint8_t> *out_client_hello_inner,
    Span<const uint8_t> encoded_client_hello_inner,
    const SSL_CLIENT_HELLO *client_hello_outer);

// ssl_client_hello_decrypt attempts to decrypt and decode the |payload|. It
// writes the result to |*out|. |payload| must point into |client_hello_outer|.
// It returns true on success and false on error. On error, it sets
// |*out_is_decrypt_error| to whether the failure was due to a bad ciphertext.
bool ssl_client_hello_decrypt(SSL_HANDSHAKE *hs, uint8_t *out_alert,
                              bool *out_is_decrypt_error, Array<uint8_t> *out,
                              const SSL_CLIENT_HELLO *client_hello_outer,
                              Span<const uint8_t> payload);

#define ECH_CONFIRMATION_SIGNAL_LEN 8

// ssl_ech_confirmation_signal_hello_offset returns the offset of the ECH
// confirmation signal in a ServerHello message, including the handshake header.
size_t ssl_ech_confirmation_signal_hello_offset(const SSL *ssl);

// ssl_ech_accept_confirmation computes the server's ECH acceptance signal,
// writing it to |out|. The transcript portion is the concatenation of
// |transcript| with |msg|. The |ECH_CONFIRMATION_SIGNAL_LEN| bytes from
// |offset| in |msg| are replaced with zeros before hashing. This function
// returns true on success, and false on failure.
bool ssl_ech_accept_confirmation(const SSL_HANDSHAKE *hs, Span<uint8_t> out,
                                 Span<const uint8_t> client_random,
                                 const SSLTranscript &transcript, bool is_hrr,
                                 Span<const uint8_t> msg, size_t offset);

// ssl_is_valid_ech_public_name returns true if |public_name| is a valid ECH
// public name and false otherwise. It is exported for testing.
OPENSSL_EXPORT bool ssl_is_valid_ech_public_name(
    Span<const uint8_t> public_name);

// ssl_is_valid_ech_config_list returns true if |ech_config_list| is a valid
// ECHConfigList structure and false otherwise.
bool ssl_is_valid_ech_config_list(Span<const uint8_t> ech_config_list);

// ssl_select_ech_config selects an ECHConfig and associated parameters to offer
// on the client and updates |hs|. It returns true on success, whether an
// ECHConfig was found or not, and false on internal error. On success, the
// encapsulated key is written to |out_enc| and |*out_enc_len| is set to the
// number of bytes written. If the function did not select an ECHConfig, the
// encapsulated key is the empty string.
bool ssl_select_ech_config(SSL_HANDSHAKE *hs, Span<uint8_t> out_enc,
                           size_t *out_enc_len);

// ssl_ech_extension_body_length returns the length of the body of a ClientHello
// ECH extension that encrypts |in_len| bytes with |aead| and an 'enc' value of
// length |enc_len|. The result does not include the four-byte extension header.
size_t ssl_ech_extension_body_length(const EVP_HPKE_AEAD *aead, size_t enc_len,
                                     size_t in_len);

// ssl_encrypt_client_hello constructs a new ClientHelloInner, adds it to the
// inner transcript, and encrypts for inclusion in the ClientHelloOuter. |enc|
// is the encapsulated key to include in the extension. It returns true on
// success and false on error. If not offering ECH, |enc| is ignored and the
// function will compute a GREASE ECH extension if necessary, and otherwise
// return success while doing nothing.
//
// Encrypting the ClientHelloInner incorporates all extensions in the
// ClientHelloOuter, so all other state necessary for |ssl_add_client_hello|
// must already be computed.
bool ssl_encrypt_client_hello(SSL_HANDSHAKE *hs, Span<const uint8_t> enc);


// Delegated credentials.

// This structure stores a delegated credential (DC) as defined by RFC 9345.
struct DC {
  static constexpr bool kAllowUniquePtr = true;
  ~DC();

  // Dup returns a copy of this DC and takes references to |raw| and |pkey|.
  UniquePtr<DC> Dup();

  // Parse parses the delegated credential stored in |in|. If successful it
  // returns the parsed structure, otherwise it returns |nullptr| and sets
  // |*out_alert|.
  static UniquePtr<DC> Parse(CRYPTO_BUFFER *in, uint8_t *out_alert);

  // raw is the delegated credential encoded as specified in RFC 9345.
  UniquePtr<CRYPTO_BUFFER> raw;

  // dc_cert_verify_algorithm is the signature scheme of the DC public key.
  uint16_t dc_cert_verify_algorithm = 0;

  // pkey is the public key parsed from |public_key|.
  UniquePtr<EVP_PKEY> pkey;

 private:
  friend DC *New<DC>();
  DC();
};

// ssl_signing_with_dc returns true if the peer has indicated support for
// delegated credentials and this host has sent a delegated credential in
// response. If this is true then we've committed to using the DC in the
// handshake.
bool ssl_signing_with_dc(const SSL_HANDSHAKE *hs);


// Handshake functions.

enum ssl_hs_wait_t {
  ssl_hs_error,
  ssl_hs_ok,
  ssl_hs_read_server_hello,
  ssl_hs_read_message,
  ssl_hs_flush,
  ssl_hs_certificate_selection_pending,
  ssl_hs_handoff,
  ssl_hs_handback,
  ssl_hs_x509_lookup,
  ssl_hs_private_key_operation,
  ssl_hs_pending_session,
  ssl_hs_pending_ticket,
  ssl_hs_early_return,
  ssl_hs_early_data_rejected,
  ssl_hs_read_end_of_early_data,
  ssl_hs_read_change_cipher_spec,
  ssl_hs_certificate_verify,
  ssl_hs_hints_ready,
};

enum ssl_grease_index_t {
  ssl_grease_cipher = 0,
  ssl_grease_group,
  ssl_grease_extension1,
  ssl_grease_extension2,
  ssl_grease_version,
  ssl_grease_ticket_extension,
  ssl_grease_ech_config_id,
  ssl_grease_last_index = ssl_grease_ech_config_id,
};

enum tls12_server_hs_state_t {
  state12_start_accept = 0,
  state12_read_client_hello,
  state12_read_client_hello_after_ech,
  state12_select_certificate,
  state12_tls13,
  state12_select_parameters,
  state12_send_server_hello,
  state12_send_server_certificate,
  state12_send_server_key_exchange,
  state12_send_server_hello_done,
  state12_read_client_certificate,
  state12_verify_client_certificate,
  state12_read_client_key_exchange,
  state12_read_client_certificate_verify,
  state12_read_change_cipher_spec,
  state12_process_change_cipher_spec,
  state12_read_next_proto,
  state12_read_channel_id,
  state12_read_client_finished,
  state12_send_server_finished,
  state12_finish_server_handshake,
  state12_done,
};

enum tls13_server_hs_state_t {
  state13_select_parameters = 0,
  state13_select_session,
  state13_send_hello_retry_request,
  state13_read_second_client_hello,
  state13_send_server_hello,
  state13_send_server_certificate_verify,
  state13_send_server_finished,
  state13_send_half_rtt_ticket,
  state13_read_second_client_flight,
  state13_process_end_of_early_data,
  state13_read_client_encrypted_extensions,
  state13_read_client_certificate,
  state13_read_client_certificate_verify,
  state13_read_channel_id,
  state13_read_client_finished,
  state13_send_new_session_ticket,
  state13_done,
};

// handback_t lists the points in the state machine where a handback can occur.
// These are the different points at which key material is no longer needed.
enum handback_t {
  handback_after_session_resumption = 0,
  handback_after_ecdhe = 1,
  handback_after_handshake = 2,
  handback_tls13 = 3,
  handback_max_value = handback_tls13,
};

// SSL_HANDSHAKE_HINTS contains handshake hints for a connection. See
// |SSL_request_handshake_hints| and related functions.
struct SSL_HANDSHAKE_HINTS {
  static constexpr bool kAllowUniquePtr = true;

  Array<uint8_t> server_random_tls12;
  Array<uint8_t> server_random_tls13;

  uint16_t key_share_group_id = 0;
  Array<uint8_t> key_share_public_key;
  Array<uint8_t> key_share_secret;

  uint16_t signature_algorithm = 0;
  Array<uint8_t> signature_input;
  Array<uint8_t> signature_spki;
  Array<uint8_t> signature;

  Array<uint8_t> decrypted_psk;
  bool ignore_psk = false;

  uint16_t cert_compression_alg_id = 0;
  Array<uint8_t> cert_compression_input;
  Array<uint8_t> cert_compression_output;

  uint16_t ecdhe_group_id = 0;
  Array<uint8_t> ecdhe_public_key;
  Array<uint8_t> ecdhe_private_key;

  Array<uint8_t> decrypted_ticket;
  bool renew_ticket = false;
  bool ignore_ticket = false;
};

struct SSL_HANDSHAKE {
  OPENSSL_EXPORT explicit SSL_HANDSHAKE(SSL *ssl);
  OPENSSL_EXPORT ~SSL_HANDSHAKE();
  static constexpr bool kAllowUniquePtr = true;

  // ssl is a non-owning pointer to the parent |SSL| object.
  SSL *ssl;

  // config is a non-owning pointer to the handshake configuration.
  SSL_CONFIG *config;

  // wait contains the operation the handshake is currently blocking on or
  // |ssl_hs_ok| if none.
  enum ssl_hs_wait_t wait = ssl_hs_ok;

  // state is the internal state for the TLS 1.2 and below handshake. Its
  // values depend on |do_handshake| but the starting state is always zero.
  int state = 0;

  // tls13_state is the internal state for the TLS 1.3 handshake. Its values
  // depend on |do_handshake| but the starting state is always zero.
  int tls13_state = 0;

  // min_version is the minimum accepted protocol version, taking account both
  // |SSL_OP_NO_*| and |SSL_CTX_set_min_proto_version| APIs.
  uint16_t min_version = 0;

  // signature_algorithm is the signature algorithm negotiated for this
  // handshake.
  uint16_t signature_algorithm = 0;

  // max_version is the maximum accepted protocol version, taking account both
  // |SSL_OP_NO_*| and |SSL_CTX_set_max_proto_version| APIs.
  uint16_t max_version = 0;

 private:
  size_t hash_len_ = 0;
  uint8_t secret_[SSL_MAX_MD_SIZE] = {0};
  uint8_t early_traffic_secret_[SSL_MAX_MD_SIZE] = {0};
  uint8_t client_handshake_secret_[SSL_MAX_MD_SIZE] = {0};
  uint8_t server_handshake_secret_[SSL_MAX_MD_SIZE] = {0};
  uint8_t client_traffic_secret_0_[SSL_MAX_MD_SIZE] = {0};
  uint8_t server_traffic_secret_0_[SSL_MAX_MD_SIZE] = {0};
  uint8_t expected_client_finished_[SSL_MAX_MD_SIZE] = {0};

 public:
  void ResizeSecrets(size_t hash_len);

  // GetClientHello, on the server, returns either the normal ClientHello
  // message or the ClientHelloInner if it has been serialized to
  // |ech_client_hello_buf|. This function should only be called when the
  // current message is a ClientHello. It returns true on success and false on
  // error.
  //
  // Note that fields of the returned |out_msg| and |out_client_hello| point
  // into a handshake-owned buffer, so their lifetimes should not exceed this
  // SSL_HANDSHAKE.
  bool GetClientHello(SSLMessage *out_msg, SSL_CLIENT_HELLO *out_client_hello);

  Span<uint8_t> secret() { return MakeSpan(secret_, hash_len_); }
  Span<const uint8_t> secret() const {
    return MakeConstSpan(secret_, hash_len_);
  }
  Span<uint8_t> early_traffic_secret() {
    return MakeSpan(early_traffic_secret_, hash_len_);
  }
  Span<uint8_t> client_handshake_secret() {
    return MakeSpan(client_handshake_secret_, hash_len_);
  }
  Span<uint8_t> server_handshake_secret() {
    return MakeSpan(server_handshake_secret_, hash_len_);
  }
  Span<uint8_t> client_traffic_secret_0() {
    return MakeSpan(client_traffic_secret_0_, hash_len_);
  }
  Span<uint8_t> server_traffic_secret_0() {
    return MakeSpan(server_traffic_secret_0_, hash_len_);
  }
  Span<uint8_t> expected_client_finished() {
    return MakeSpan(expected_client_finished_, hash_len_);
  }

  union {
    // sent is a bitset where the bits correspond to elements of kExtensions
    // in extensions.cc. Each bit is set if that extension was sent in a
    // ClientHello. It's not used by servers.
    uint32_t sent = 0;
    // received is a bitset, like |sent|, but is used by servers to record
    // which extensions were received from a client.
    uint32_t received;
  } extensions;

  // inner_extensions_sent, on clients that offer ECH, is |extensions.sent| for
  // the ClientHelloInner.
  uint32_t inner_extensions_sent = 0;

  union {
    // sent is a bitset where the bits correspond to elements of
    // |client_custom_extensions| in the |SSL_CTX|. Each bit is set if that
    // extension was sent in a ClientHello. It's not used by servers.
    uint16_t sent = 0;
    // received is a bitset, like |sent|, but is used by servers to record
    // which custom extensions were received from a client. The bits here
    // correspond to |server_custom_extensions|.
    uint16_t received;
  } custom_extensions;

  // error, if |wait| is |ssl_hs_error|, is the error the handshake failed on.
  UniquePtr<ERR_SAVE_STATE> error;

  // key_shares are the current key exchange instances. The second is only used
  // as a client if we believe that we should offer two key shares in a
  // ClientHello.
  UniquePtr<SSLKeyShare> key_shares[2];

  // transcript is the current handshake transcript.
  SSLTranscript transcript;

  // inner_transcript, on the client, is the handshake transcript for the
  // ClientHelloInner handshake. It is moved to |transcript| if the server
  // accepts ECH.
  SSLTranscript inner_transcript;

  // inner_client_random is the ClientHello random value used with
  // ClientHelloInner.
  uint8_t inner_client_random[SSL3_RANDOM_SIZE] = {0};

  // cookie is the value of the cookie in HelloRetryRequest, or empty if none
  // was received.
  Array<uint8_t> cookie;

  // dtls_cookie is the value of the cookie in DTLS HelloVerifyRequest. If
  // empty, either none was received or HelloVerifyRequest contained an empty
  // cookie.
  Array<uint8_t> dtls_cookie;

  // ech_client_outer contains the outer ECH extension to send in the
  // ClientHello, excluding the header and type byte.
  Array<uint8_t> ech_client_outer;

  // ech_retry_configs, on the client, contains the retry configs from the
  // server as a serialized ECHConfigList.
  Array<uint8_t> ech_retry_configs;

  // ech_client_hello_buf, on the server, contains the bytes of the
  // reconstructed ClientHelloInner message.
  Array<uint8_t> ech_client_hello_buf;

  // key_share_bytes is the key_share extension that the client should send.
  Array<uint8_t> key_share_bytes;

  // ecdh_public_key, for servers, is the key share to be sent to the client in
  // TLS 1.3.
  Array<uint8_t> ecdh_public_key;

  // peer_sigalgs are the signature algorithms that the peer supports. These are
  // taken from the contents of the signature algorithms extension for a server
  // or from the CertificateRequest for a client.
  Array<uint16_t> peer_sigalgs;

  // peer_supported_group_list contains the supported group IDs advertised by
  // the peer. This is only set on the server's end. The server does not
  // advertise this extension to the client.
  Array<uint16_t> peer_supported_group_list;

  // peer_delegated_credential_sigalgs are the signature algorithms the peer
  // supports with delegated credentials, or empty if the peer does not support
  // delegated credentials.
  Array<uint16_t> peer_delegated_credential_sigalgs;

  // extension_permutation is the permutation to apply to ClientHello
  // extensions. It maps indices into the |kExtensions| table into other
  // indices.
  Array<uint8_t> extension_permutation;

  // cert_compression_alg_id, for a server, contains the negotiated certificate
  // compression algorithm for this client. It is only valid if
  // |cert_compression_negotiated| is true.
  uint16_t cert_compression_alg_id;

  // ech_hpke_ctx is the HPKE context used in ECH. On the server, it is
  // initialized if |ech_status| is |ssl_ech_accepted|. On the client, it is
  // initialized if |selected_ech_config| is not nullptr.
  ScopedEVP_HPKE_CTX ech_hpke_ctx;

  // server_params, in a TLS 1.2 server, stores the ServerKeyExchange
  // parameters. It has client and server randoms prepended for signing
  // convenience.
  Array<uint8_t> server_params;

  // peer_psk_identity_hint, on the client, is the psk_identity_hint sent by the
  // server when using a TLS 1.2 PSK key exchange.
  UniquePtr<char> peer_psk_identity_hint;

  // ca_names, on the client, contains the list of CAs received in a
  // CertificateRequest message.
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> ca_names;

  // certificate_types, on the client, contains the set of certificate types
  // received in a CertificateRequest message.
  Array<uint8_t> certificate_types;

  // local_pubkey is the public key we are authenticating as.
  UniquePtr<EVP_PKEY> local_pubkey;

  // peer_pubkey is the public key parsed from the peer's leaf certificate.
  UniquePtr<EVP_PKEY> peer_pubkey;

  // new_session is the new mutable session being established by the current
  // handshake. It should not be cached.
  UniquePtr<SSL_SESSION> new_session;

  // early_session is the session corresponding to the current 0-RTT state on
  // the client if |in_early_data| is true.
  UniquePtr<SSL_SESSION> early_session;

  // ssl_ech_keys, for servers, is the set of ECH keys to use with this
  // handshake. This is copied from |SSL_CTX| to ensure consistent behavior as
  // |SSL_CTX| rotates keys.
  UniquePtr<SSL_ECH_KEYS> ech_keys;

  // selected_ech_config, for clients, is the ECHConfig the client uses to offer
  // ECH, or nullptr if ECH is not being offered. If non-NULL, |ech_hpke_ctx|
  // will be initialized.
  UniquePtr<ECHConfig> selected_ech_config;

  // new_cipher is the cipher being negotiated in this handshake.
  const SSL_CIPHER *new_cipher = nullptr;

  // key_block is the record-layer key block for TLS 1.2 and earlier.
  Array<uint8_t> key_block;

  // hints contains the handshake hints for this connection. If
  // |hints_requested| is true, this field is non-null and contains the pending
  // hints to filled as the predicted handshake progresses. Otherwise, this
  // field, if non-null, contains hints configured by the caller and will
  // influence the handshake on match.
  UniquePtr<SSL_HANDSHAKE_HINTS> hints;

  // ech_is_inner, on the server, indicates whether the ClientHello contained an
  // inner ECH extension.
  bool ech_is_inner : 1;

  // ech_authenticated_reject, on the client, indicates whether an ECH rejection
  // handshake has been authenticated.
  bool ech_authenticated_reject : 1;

  // scts_requested is true if the SCT extension is in the ClientHello.
  bool scts_requested : 1;

  bool received_custom_extension : 1;

  // handshake_finalized is true once the handshake has completed, at which
  // point accessors should use the established state.
  bool handshake_finalized : 1;

  // accept_psk_mode stores whether the client's PSK mode is compatible with our
  // preferences.
  bool accept_psk_mode : 1;

  // cert_request is true if a client certificate was requested.
  bool cert_request : 1;

  // certificate_status_expected is true if OCSP stapling was negotiated and the
  // server is expected to send a CertificateStatus message. (This is used on
  // both the client and server sides.)
  bool certificate_status_expected : 1;

  // ocsp_stapling_requested is true if a client requested OCSP stapling.
  bool ocsp_stapling_requested : 1;

  // should_ack_sni is used by a server and indicates that the SNI extension
  // should be echoed in the ServerHello.
  bool should_ack_sni : 1;

  // in_false_start is true if there is a pending client handshake in False
  // Start. The client may write data at this point.
  bool in_false_start : 1;

  // in_early_data is true if there is a pending handshake that has progressed
  // enough to send and receive early data.
  bool in_early_data : 1;

  // early_data_offered is true if the client sent the early_data extension.
  bool early_data_offered : 1;

  // can_early_read is true if application data may be read at this point in the
  // handshake.
  bool can_early_read : 1;

  // can_early_write is true if application data may be written at this point in
  // the handshake.
  bool can_early_write : 1;

  // next_proto_neg_seen is one of NPN was negotiated.
  bool next_proto_neg_seen : 1;

  // ticket_expected is true if a TLS 1.2 NewSessionTicket message is to be sent
  // or received.
  bool ticket_expected : 1;

  // extended_master_secret is true if the extended master secret extension is
  // negotiated in this handshake.
  bool extended_master_secret : 1;

  // pending_private_key_op is true if there is a pending private key operation
  // in progress.
  bool pending_private_key_op : 1;

  // handback indicates that a server should pause the handshake after
  // finishing operations that require private key material, in such a way that
  // |SSL_get_error| returns |SSL_ERROR_HANDBACK|.  It is set by
  // |SSL_apply_handoff|.
  bool handback : 1;

  // hints_requested indicates the caller has requested handshake hints. Only
  // the first round-trip of the handshake will complete, after which the
  // |hints| structure can be serialized.
  bool hints_requested : 1;

  // cert_compression_negotiated is true iff |cert_compression_alg_id| is valid.
  bool cert_compression_negotiated : 1;

  // apply_jdk11_workaround is true if the peer is probably a JDK 11 client
  // which implemented TLS 1.3 incorrectly.
  bool apply_jdk11_workaround : 1;

  // can_release_private_key is true if the private key will no longer be used
  // in this handshake.
  bool can_release_private_key : 1;

  // channel_id_negotiated is true if Channel ID should be used in this
  // handshake.
  bool channel_id_negotiated : 1;

  // client_version is the value sent or received in the ClientHello version.
  uint16_t client_version = 0;

  // early_data_read is the amount of early data that has been read by the
  // record layer.
  uint16_t early_data_read = 0;

  // early_data_written is the amount of early data that has been written by the
  // record layer.
  uint16_t early_data_written = 0;

  // ech_config_id is the ECH config sent by the client.
  uint8_t ech_config_id = 0;

  // session_id is the session ID in the ClientHello.
  uint8_t session_id[SSL_MAX_SSL_SESSION_ID_LENGTH] = {0};
  uint8_t session_id_len = 0;

  // grease_seed is the entropy for GREASE values.
  uint8_t grease_seed[ssl_grease_last_index + 1] = {0};
};

// kMaxTickets is the maximum number of tickets to send immediately after the
// handshake. We use a one-byte ticket nonce, and there is no point in sending
// so many tickets.
constexpr size_t kMaxTickets = 16;

OPENSSL_EXPORT UniquePtr<SSL_HANDSHAKE> ssl_handshake_new(SSL *ssl);

// ssl_check_message_type checks if |msg| has type |type|. If so it returns
// one. Otherwise, it sends an alert and returns zero.
bool ssl_check_message_type(SSL *ssl, const SSLMessage &msg, int type);

// ssl_run_handshake runs the TLS handshake. It returns one on success and <= 0
// on error. It sets |out_early_return| to one if we've completed the handshake
// early.
int ssl_run_handshake(SSL_HANDSHAKE *hs, bool *out_early_return);

// The following are implementations of |do_handshake| for the client and
// server.
enum ssl_hs_wait_t ssl_client_handshake(SSL_HANDSHAKE *hs);
enum ssl_hs_wait_t ssl_server_handshake(SSL_HANDSHAKE *hs);
enum ssl_hs_wait_t tls13_client_handshake(SSL_HANDSHAKE *hs);
enum ssl_hs_wait_t tls13_server_handshake(SSL_HANDSHAKE *hs);

// The following functions return human-readable representations of the TLS
// handshake states for debugging.
const char *ssl_client_handshake_state(SSL_HANDSHAKE *hs);
const char *ssl_server_handshake_state(SSL_HANDSHAKE *hs);
const char *tls13_client_handshake_state(SSL_HANDSHAKE *hs);
const char *tls13_server_handshake_state(SSL_HANDSHAKE *hs);

// tls13_add_key_update queues a KeyUpdate message on |ssl|. The
// |update_requested| argument must be one of |SSL_KEY_UPDATE_REQUESTED| or
// |SSL_KEY_UPDATE_NOT_REQUESTED|.
bool tls13_add_key_update(SSL *ssl, int update_requested);

// tls13_post_handshake processes a post-handshake message. It returns true on
// success and false on failure.
bool tls13_post_handshake(SSL *ssl, const SSLMessage &msg);

bool tls13_process_certificate(SSL_HANDSHAKE *hs, const SSLMessage &msg,
                               bool allow_anonymous);
bool tls13_process_certificate_verify(SSL_HANDSHAKE *hs, const SSLMessage &msg);

// tls13_process_finished processes |msg| as a Finished message from the
// peer. If |use_saved_value| is true, the verify_data is compared against
// |hs->expected_client_finished| rather than computed fresh.
bool tls13_process_finished(SSL_HANDSHAKE *hs, const SSLMessage &msg,
                            bool use_saved_value);

bool tls13_add_certificate(SSL_HANDSHAKE *hs);

// tls13_add_certificate_verify adds a TLS 1.3 CertificateVerify message to the
// handshake. If it returns |ssl_private_key_retry|, it should be called again
// to retry when the signing operation is completed.
//
// NOTE: |signature_algorithm| in |hs| should be initialized already before
// this is called.
enum ssl_private_key_result_t tls13_add_certificate_verify(SSL_HANDSHAKE *hs);

bool tls13_add_finished(SSL_HANDSHAKE *hs);
bool tls13_process_new_session_ticket(SSL *ssl, const SSLMessage &msg);
bssl::UniquePtr<SSL_SESSION> tls13_create_session_with_ticket(SSL *ssl,
                                                              CBS *body);

// ssl_setup_extension_permutation computes a ClientHello extension permutation
// for |hs|, if applicable. It returns true on success and false on error.
bool ssl_setup_extension_permutation(SSL_HANDSHAKE *hs);

// ssl_setup_key_shares computes client key shares and saves them in |hs|. It
// returns true on success and false on failure. If |override_group_id| is zero,
// it offers the default groups, including GREASE. If it is non-zero, it offers
// a single key share of the specified group.
bool ssl_setup_key_shares(SSL_HANDSHAKE *hs, uint16_t override_group_id);

bool ssl_ext_key_share_parse_serverhello(SSL_HANDSHAKE *hs,
                                         Array<uint8_t> *out_secret,
                                         Array<uint8_t> *out_peer_key,
                                         uint8_t *out_alert, CBS *contents);
bool ssl_ext_key_share_parse_clienthello(SSL_HANDSHAKE *hs, bool *out_found,
                                         Span<const uint8_t> *out_peer_key,
                                         uint8_t *out_alert,
                                         const SSL_CLIENT_HELLO *client_hello);
bool ssl_ext_key_share_add_serverhello(SSL_HANDSHAKE *hs, CBB *out);

bool ssl_ext_pre_shared_key_parse_serverhello(SSL_HANDSHAKE *hs,
                                              uint8_t *out_alert,
                                              CBS *contents);
bool ssl_ext_pre_shared_key_parse_clienthello(
    SSL_HANDSHAKE *hs, CBS *out_ticket, CBS *out_binders,
    uint32_t *out_obfuscated_ticket_age, uint8_t *out_alert,
    const SSL_CLIENT_HELLO *client_hello, CBS *contents);
bool ssl_ext_pre_shared_key_add_serverhello(SSL_HANDSHAKE *hs, CBB *out);

// ssl_is_sct_list_valid does a shallow parse of the SCT list in |contents| and
// returns whether it's valid.
bool ssl_is_sct_list_valid(const CBS *contents);

// ssl_write_client_hello_without_extensions writes a ClientHello to |out|,
// up to the extensions field. |type| determines the type of ClientHello to
// write. If |omit_session_id| is true, the session ID is empty.
bool ssl_write_client_hello_without_extensions(const SSL_HANDSHAKE *hs,
                                               CBB *cbb,
                                               ssl_client_hello_type_t type,
                                               bool empty_session_id);

// ssl_add_client_hello constructs a ClientHello and adds it to the outgoing
// flight. It returns true on success and false on error.
bool ssl_add_client_hello(SSL_HANDSHAKE *hs);

struct ParsedServerHello {
  CBS raw;
  uint16_t legacy_version = 0;
  CBS random;
  CBS session_id;
  uint16_t cipher_suite = 0;
  uint8_t compression_method = 0;
  CBS extensions;
};

// ssl_parse_server_hello parses |msg| as a ServerHello. On success, it writes
// the result to |*out| and returns true. Otherwise, it returns false and sets
// |*out_alert| to an alert to send to the peer.
bool ssl_parse_server_hello(ParsedServerHello *out, uint8_t *out_alert,
                            const SSLMessage &msg);

enum ssl_cert_verify_context_t {
  ssl_cert_verify_server,
  ssl_cert_verify_client,
  ssl_cert_verify_channel_id,
};

// tls13_get_cert_verify_signature_input generates the message to be signed for
// TLS 1.3's CertificateVerify message. |cert_verify_context| determines the
// type of signature. It sets |*out| to a newly allocated buffer containing the
// result. This function returns true on success and false on failure.
bool tls13_get_cert_verify_signature_input(
    SSL_HANDSHAKE *hs, Array<uint8_t> *out,
    enum ssl_cert_verify_context_t cert_verify_context);

// ssl_is_valid_alpn_list returns whether |in| is a valid ALPN protocol list.
bool ssl_is_valid_alpn_list(Span<const uint8_t> in);

// ssl_is_alpn_protocol_allowed returns whether |protocol| is a valid server
// selection for |hs->ssl|'s client preferences.
bool ssl_is_alpn_protocol_allowed(const SSL_HANDSHAKE *hs,
                                  Span<const uint8_t> protocol);

// ssl_alpn_list_contains_protocol returns whether |list|, a serialized ALPN
// protocol list, contains |protocol|.
bool ssl_alpn_list_contains_protocol(Span<const uint8_t> list,
                                     Span<const uint8_t> protocol);

// ssl_negotiate_alpn negotiates the ALPN extension, if applicable. It returns
// true on successful negotiation or if nothing was negotiated. It returns false
// and sets |*out_alert| to an alert on error.
bool ssl_negotiate_alpn(SSL_HANDSHAKE *hs, uint8_t *out_alert,
                        const SSL_CLIENT_HELLO *client_hello);

// ssl_get_local_application_settings looks up the configured ALPS value for
// |protocol|. If found, it sets |*out_settings| to the value and returns true.
// Otherwise, it returns false.
bool ssl_get_local_application_settings(const SSL_HANDSHAKE *hs,
                                        Span<const uint8_t> *out_settings,
                                        Span<const uint8_t> protocol);

// ssl_negotiate_alps negotiates the ALPS extension, if applicable. It returns
// true on successful negotiation or if nothing was negotiated. It returns false
// and sets |*out_alert| to an alert on error.
bool ssl_negotiate_alps(SSL_HANDSHAKE *hs, uint8_t *out_alert,
                        const SSL_CLIENT_HELLO *client_hello);

struct SSLExtension {
  SSLExtension(uint16_t type_arg, bool allowed_arg = true)
      : type(type_arg), allowed(allowed_arg), present(false) {
    CBS_init(&data, nullptr, 0);
  }

  uint16_t type;
  bool allowed;
  bool present;
  CBS data;
};

// ssl_parse_extensions parses a TLS extensions block out of |cbs| and advances
// it. It writes the parsed extensions to pointers in |extensions|. On success,
// it fills in the |present| and |data| fields and returns true. Otherwise, it
// sets |*out_alert| to an alert to send and returns false. Unknown extensions
// are rejected unless |ignore_unknown| is true.
bool ssl_parse_extensions(const CBS *cbs, uint8_t *out_alert,
                          std::initializer_list<SSLExtension *> extensions,
                          bool ignore_unknown);

// ssl_verify_peer_cert verifies the peer certificate for |hs|.
enum ssl_verify_result_t ssl_verify_peer_cert(SSL_HANDSHAKE *hs);
// ssl_reverify_peer_cert verifies the peer certificate for |hs| when resuming a
// session.
enum ssl_verify_result_t ssl_reverify_peer_cert(SSL_HANDSHAKE *hs,
                                                bool send_alert);

enum ssl_hs_wait_t ssl_get_finished(SSL_HANDSHAKE *hs);
bool ssl_send_finished(SSL_HANDSHAKE *hs);
bool ssl_output_cert_chain(SSL_HANDSHAKE *hs);

// ssl_handshake_session returns the |SSL_SESSION| corresponding to the current
// handshake. Note, in TLS 1.2 resumptions, this session is immutable.
const SSL_SESSION *ssl_handshake_session(const SSL_HANDSHAKE *hs);

// ssl_done_writing_client_hello is called after the last ClientHello is written
// by |hs|. It releases some memory that is no longer needed.
void ssl_done_writing_client_hello(SSL_HANDSHAKE *hs);


// SSLKEYLOGFILE functions.

// ssl_log_secret logs |secret| with label |label|, if logging is enabled for
// |ssl|. It returns true on success and false on failure.
bool ssl_log_secret(const SSL *ssl, const char *label,
                    Span<const uint8_t> secret);


// ClientHello functions.

// ssl_client_hello_init parses |body| as a ClientHello message, excluding the
// message header, and writes the result to |*out|. It returns true on success
// and false on error. This function is exported for testing.
OPENSSL_EXPORT bool ssl_client_hello_init(const SSL *ssl, SSL_CLIENT_HELLO *out,
                                          Span<const uint8_t> body);

bool ssl_parse_client_hello_with_trailing_data(const SSL *ssl, CBS *cbs,
                                               SSL_CLIENT_HELLO *out);

bool ssl_client_hello_get_extension(const SSL_CLIENT_HELLO *client_hello,
                                    CBS *out, uint16_t extension_type);

bool ssl_client_cipher_list_contains_cipher(
    const SSL_CLIENT_HELLO *client_hello, uint16_t id);

// ssl_parse_client_cipher_list returns the ciphers offered by the client
// during handshake that are supported by this library, or null if the handshake hasn't
// occurred or there was an error. It also stores the unparsed raw bytes of cipher suites offered in
// the client hello into |ssl->all_client_cipher_suites|.
bool ssl_parse_client_cipher_list(SSL *ssl, const SSL_CLIENT_HELLO *client_hello,
                                  UniquePtr<STACK_OF(SSL_CIPHER)> *ciphers_out);


// GREASE.

// ssl_get_grease_value returns a GREASE value for |hs|. For a given
// connection, the values for each index will be deterministic. This allows the
// same ClientHello be sent twice for a HelloRetryRequest or the same group be
// advertised in both supported_groups and key_shares.
uint16_t ssl_get_grease_value(const SSL_HANDSHAKE *hs,
                              enum ssl_grease_index_t index);


// Signature algorithms.

// tls1_parse_peer_sigalgs parses |sigalgs| as the list of peer signature
// algorithms and saves them on |hs|. It returns true on success and false on
// error.
bool tls1_parse_peer_sigalgs(SSL_HANDSHAKE *hs, const CBS *sigalgs);

// tls1_get_legacy_signature_algorithm sets |*out| to the signature algorithm
// that should be used with |pkey| in TLS 1.1 and earlier. It returns true on
// success and false if |pkey| may not be used at those versions.
bool tls1_get_legacy_signature_algorithm(uint16_t *out, const EVP_PKEY *pkey);

// tls1_choose_signature_algorithm sets |*out| to a signature algorithm for use
// with |hs|'s private key based on the peer's preferences and the algorithms
// supported. It returns true on success and false on error.
bool tls1_choose_signature_algorithm(SSL_HANDSHAKE *hs, uint16_t *out);

// tls1_get_peer_verify_algorithms returns the signature schemes for which the
// peer indicated support.
//
// NOTE: The related function |SSL_get0_peer_verify_algorithms| only has
// well-defined behavior during the callbacks set by |SSL_CTX_set_cert_cb| and
// |SSL_CTX_set_client_cert_cb|, or when the handshake is paused because of
// them.
Span<const uint16_t> tls1_get_peer_verify_algorithms(const SSL_HANDSHAKE *hs);

// tls12_add_verify_sigalgs adds the signature algorithms acceptable for the
// peer signature to |out|. It returns true on success and false on error.
bool tls12_add_verify_sigalgs(const SSL_HANDSHAKE *hs, CBB *out);

// tls12_check_peer_sigalg checks if |sigalg| is acceptable for the peer
// signature. It returns true on success and false on error, setting
// |*out_alert| to an alert to send.
OPENSSL_EXPORT bool tls12_check_peer_sigalg(const SSL_HANDSHAKE *hs,
                                            uint8_t *out_alert,
                                            uint16_t sigalg);


// Underdocumented functions.
//
// Functions below here haven't been touched up and may be underdocumented.

#define TLSEXT_CHANNEL_ID_SIZE 128

// From RFC 4492, used in encoding the curve type in ECParameters
#define NAMED_CURVE_TYPE 3

// SSL_PKEY_* denote certificate types. These represent an index within
// |CERT->cert_privatekeys|.
#define SSL_PKEY_RSA 0
#define SSL_PKEY_ECC 1
#define SSL_PKEY_ED25519 2
#define SSL_PKEY_PQDSA 3
#define SSL_PKEY_SIZE 4

struct CERT_PKEY {
  UniquePtr<EVP_PKEY> privatekey;

  // chain contains the certificate chain, with the leaf at the beginning. The
  // first element of |chain| may be NULL to indicate that the leaf certificate
  // has not yet been set.
  //   If |chain| != NULL -> len(chain) >= 1
  //   If |chain[0]| == NULL -> len(chain) >= 2.
  //   |chain[1..]| != NULL
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> chain;

  // x509_chain may contain a parsed copy of |chain[1..]|. This is only used as
  // a cache in order to implement “get0” functions that return a non-owning
  // pointer to the certificate chain.
  STACK_OF(X509) *x509_chain = nullptr;

  // x509_leaf retains the |X509| structure of the first element of |chain|.
  // However, if certs are set with |SSL_CTX_use_certificate_ASN1| or
  // |SSL_use_certificate_ASN1|, this is only used as a cache in order to
  // implement “get0” functions that return a non-owning pointer to the
  // certificate chain.
  X509 *x509_leaf = nullptr;
};

struct CERT {
  static constexpr bool kAllowUniquePtr = true;

  explicit CERT(const SSL_X509_METHOD *x509_method);
  ~CERT();

  // SetKeyMethod sets |key_method| for private key callback operations and
  // clears any software private key in slot |slot_idx| to maintain mutual
  // exclusivity between |key_method| and per-slot |privatekey|. It returns
  // true on success. If |slot_idx| is out of range and |method| is non-NULL,
  // it returns false because the mutual exclusivity invariant cannot be
  // enforced.
  bool SetKeyMethod(const SSL_PRIVATE_KEY_METHOD *method, int slot_idx);

  // SetSlotPrivateKey sets the software private key for |slot_idx| and clears
  // |key_method| to maintain mutual exclusivity between per-slot |privatekey|
  // and |key_method|. It returns false if |slot_idx| is out of range.
  bool SetSlotPrivateKey(int slot_idx, EVP_PKEY *pkey);

  // cert_privatekey_idx ALWAYS points to an element of the |cert_pkeys|
  // array. OpenSSL implements this as a pointer, but an index is more
  // efficient.
  int cert_private_key_idx = -1;

  Array<CERT_PKEY> cert_private_keys;

  /// We'lll see what we want to do about the |x509_stash| below later.

  // x509_stash contains the last |X509| object append to the chain. This is a
  // workaround for some third-party code that continue to use an |X509| object
  // even after passing ownership with an “add0” function.
  X509 *x509_stash = nullptr;

  // key_method, if non-NULL, is a set of callbacks to call for private key
  // operations.
  const SSL_PRIVATE_KEY_METHOD *key_method = nullptr;

  // x509_method contains pointers to functions that might deal with |X509|
  // compatibility, or might be a no-op, depending on the application.
  const SSL_X509_METHOD *x509_method = nullptr;

  // sigalgs, if non-empty, is the set of signature algorithms supported by
  // |privatekey| in decreasing order of preference.
  Array<uint16_t> sigalgs;

  // Certificate setup callback: if set is called whenever a
  // certificate may be required (client or server). the callback
  // can then examine any appropriate parameters and setup any
  // certificates required. This allows advanced applications
  // to select certificates on the fly: for example based on
  // supported signature algorithms or curves.
  int (*cert_cb)(SSL *ssl, void *arg) = nullptr;
  void *cert_cb_arg = nullptr;

  // Optional X509_STORE for certificate validation. If NULL the parent SSL_CTX
  // store is used instead.
  X509_STORE *verify_store = nullptr;

  // Signed certificate timestamp list to be sent to the client, if requested
  UniquePtr<CRYPTO_BUFFER> signed_cert_timestamp_list;

  // OCSP response to be sent to the client, if requested.
  UniquePtr<CRYPTO_BUFFER> ocsp_response;

  // sid_ctx partitions the session space within a shared session cache or
  // ticket key. Only sessions with a matching value will be accepted.
  uint8_t sid_ctx_length = 0;
  uint8_t sid_ctx[SSL_MAX_SID_CTX_LENGTH] = {0};

  // Delegated credentials.

  // dc is the delegated credential to send to the peer (if requested).
  UniquePtr<DC> dc = nullptr;

  // dc_privatekey is used instead of |privatekey| or |key_method| to
  // authenticate the host if a delegated credential is used in the handshake.
  UniquePtr<EVP_PKEY> dc_privatekey = nullptr;

  // dc_key_method, if not NULL, is used instead of |dc_privatekey| to
  // authenticate the host.
  const SSL_PRIVATE_KEY_METHOD *dc_key_method = nullptr;
};

// |SSL_PROTOCOL_METHOD| abstracts between TLS and DTLS.
struct SSL_PROTOCOL_METHOD {
  bool is_dtls;
  bool (*ssl_new)(SSL *ssl);
  void (*ssl_free)(SSL *ssl);
  // get_message sets |*out| to the current handshake message and returns true
  // if one has been received. It returns false if more input is needed.
  bool (*get_message)(const SSL *ssl, SSLMessage *out);
  // next_message is called to release the current handshake message.
  void (*next_message)(SSL *ssl);
  // has_unprocessed_handshake_data returns whether there is buffered
  // handshake data that has not been consumed by |get_message|.
  bool (*has_unprocessed_handshake_data)(const SSL *ssl);
  // Use the |ssl_open_handshake| wrapper.
  ssl_open_record_t (*open_handshake)(SSL *ssl, size_t *out_consumed,
                                      uint8_t *out_alert, Span<uint8_t> in);
  // Use the |ssl_open_change_cipher_spec| wrapper.
  ssl_open_record_t (*open_change_cipher_spec)(SSL *ssl, size_t *out_consumed,
                                               uint8_t *out_alert,
                                               Span<uint8_t> in);
  // Use the |ssl_open_app_data| wrapper.
  ssl_open_record_t (*open_app_data)(SSL *ssl, Span<uint8_t> *out,
                                     size_t *out_consumed, uint8_t *out_alert,
                                     Span<uint8_t> in);
  // write_app_data encrypts and writes |in| as application data. On success, it
  // returns one and sets |*out_bytes_written| to the number of bytes of |in|
  // written. Otherwise, it returns <= 0 and sets |*out_needs_handshake| to
  // whether the operation failed because the caller needs to drive the
  // handshake.
  int (*write_app_data)(SSL *ssl, bool *out_needs_handshake,
                        size_t *out_bytes_written, Span<const uint8_t> in);
  int (*dispatch_alert)(SSL *ssl);
  // init_message begins a new handshake message of type |type|. |cbb| is the
  // root CBB to be passed into |finish_message|. |*body| is set to a child CBB
  // the caller should write to. It returns true on success and false on error.
  bool (*init_message)(const SSL *ssl, CBB *cbb, CBB *body, uint8_t type);
  // finish_message finishes a handshake message. It sets |*out_msg| to the
  // serialized message. It returns true on success and false on error.
  bool (*finish_message)(const SSL *ssl, CBB *cbb,
                         bssl::Array<uint8_t> *out_msg);
  // add_message adds a handshake message to the pending flight. It returns
  // true on success and false on error.
  bool (*add_message)(SSL *ssl, bssl::Array<uint8_t> msg);
  // add_change_cipher_spec adds a ChangeCipherSpec record to the pending
  // flight. It returns true on success and false on error.
  bool (*add_change_cipher_spec)(SSL *ssl);
  // flush_flight flushes the pending flight to the transport. It returns one on
  // success and <= 0 on error.
  int (*flush_flight)(SSL *ssl);
  // on_handshake_complete is called when the handshake is complete.
  void (*on_handshake_complete)(SSL *ssl);
  // set_read_state sets |ssl|'s read cipher state and level to |aead_ctx| and
  // |level|. In QUIC, |aead_ctx| is a placeholder object and |secret_for_quic|
  // is the original secret. This function returns true on success and false on
  // error.
  bool (*set_read_state)(SSL *ssl, ssl_encryption_level_t level,
                         UniquePtr<SSLAEADContext> aead_ctx,
                         Span<const uint8_t> secret_for_quic);
  // set_write_state sets |ssl|'s write cipher state and level to |aead_ctx| and
  // |level|. In QUIC, |aead_ctx| is a placeholder object and |secret_for_quic|
  // is the original secret. This function returns true on success and false on
  // error.
  bool (*set_write_state)(SSL *ssl, ssl_encryption_level_t level,
                          UniquePtr<SSLAEADContext> aead_ctx,
                          Span<const uint8_t> secret_for_quic);
};

// The following wrappers call |open_*| but handle |read_shutdown| correctly.

// ssl_open_handshake processes a record from |in| for reading a handshake
// message.
ssl_open_record_t ssl_open_handshake(SSL *ssl, size_t *out_consumed,
                                     uint8_t *out_alert, Span<uint8_t> in);

// ssl_open_change_cipher_spec processes a record from |in| for reading a
// ChangeCipherSpec.
ssl_open_record_t ssl_open_change_cipher_spec(SSL *ssl, size_t *out_consumed,
                                              uint8_t *out_alert,
                                              Span<uint8_t> in);

// ssl_open_app_data processes a record from |in| for reading application data.
// On success, it returns |ssl_open_record_success| and sets |*out| to the
// input. If it encounters a post-handshake message, it returns
// |ssl_open_record_discard|. The caller should then retry, after processing any
// messages received with |get_message|.
ssl_open_record_t ssl_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                    size_t *out_consumed, uint8_t *out_alert,
                                    Span<uint8_t> in);

struct SSL_X509_METHOD {
  // check_client_CA_list returns one if |names| is a good list of X.509
  // distinguished names and zero otherwise. This is used to ensure that we can
  // reject unparsable values at handshake time when using crypto/x509.
  bool (*check_client_CA_list)(STACK_OF(CRYPTO_BUFFER) *names);

  // cert_clear frees and NULLs all X509 certificate-related state.
  void (*cert_clear)(CERT *cert);
  // cert_free frees all X509-related state.
  void (*cert_free)(CERT *cert);
  // cert_dup duplicates any needed fields from |cert| to |new_cert|.
  void (*cert_dup)(CERT *new_cert, const CERT *cert);
  // cert_flush_cached_chain drops any cached |X509|-based certificate chain
  // from |cert|.
  void (*cert_flush_cached_chain)(CERT *cert);
  // cert_flush_leaf drops the |X509|-based leaf certificate from |cert|.
  void (*cert_flush_leaf)(CERT *cert);

  // session_cache_objects fills out |sess->x509_peer| and |sess->x509_chain|
  // from |sess->certs| and erases |sess->x509_chain_without_leaf|. It returns
  // true on success or false on error.
  bool (*session_cache_objects)(SSL_SESSION *session);
  // session_dup duplicates any needed fields from |session| to |new_session|.
  // It returns true on success or false on error.
  bool (*session_dup)(SSL_SESSION *new_session, const SSL_SESSION *session);
  // session_clear frees any X509-related state from |session|.
  void (*session_clear)(SSL_SESSION *session);
  // session_verify_cert_chain verifies the certificate chain in |session|,
  // and sets |session->verify_result| and |session->x509_verified_chain|. It
  // returns true on success or false on error.
  bool (*session_verify_cert_chain)(SSL_SESSION *session, SSL_HANDSHAKE *ssl,
                                    uint8_t *out_alert);

  // hs_flush_cached_ca_names drops any cached peer CA |X509_NAME|s from
  // |hs->ssl->s3|. Called when |hs->ca_names| is reparsed.
  void (*hs_flush_cached_ca_names)(SSL_HANDSHAKE *hs);
  // ssl_new does any necessary initialisation of |hs|. It returns true on
  // success or false on error.
  bool (*ssl_new)(SSL_HANDSHAKE *hs);
  // ssl_free frees anything created by |ssl_new|.
  void (*ssl_config_free)(SSL_CONFIG *cfg);
  // ssl_flush_cached_client_CA drops any cached |X509_NAME|s from |ssl|.
  void (*ssl_flush_cached_client_CA)(SSL_CONFIG *cfg);
  // ssl_auto_chain_if_needed runs the deprecated auto-chaining logic if
  // necessary. On success, it updates |ssl|'s certificate configuration as
  // needed and returns true. Otherwise, it returns false.
  bool (*ssl_auto_chain_if_needed)(SSL_HANDSHAKE *hs);
  // ssl_ctx_new does any necessary initialisation of |ctx|. It returns true on
  // success or false on error.
  bool (*ssl_ctx_new)(SSL_CTX *ctx);
  // ssl_ctx_free frees anything created by |ssl_ctx_new|.
  void (*ssl_ctx_free)(SSL_CTX *ctx);
  // ssl_ctx_flush_cached_client_CA drops any cached |X509_NAME|s from |ctx|.
  void (*ssl_ctx_flush_cached_client_CA)(SSL_CTX *ssl);
};

// ssl_crypto_x509_method provides the |SSL_X509_METHOD| functions using
// crypto/x509.
extern const SSL_X509_METHOD ssl_crypto_x509_method;

// ssl_noop_x509_method provides the |SSL_X509_METHOD| functions that avoid
// crypto/x509.
extern const SSL_X509_METHOD ssl_noop_x509_method;

struct TicketKey {
  static constexpr bool kAllowUniquePtr = true;

  uint8_t name[SSL_TICKET_KEY_NAME_LEN] = {0};
  uint8_t hmac_key[16] = {0};
  uint8_t aes_key[16] = {0};
  // next_rotation_tv_sec is the time (in seconds from the epoch) when the
  // current key should be superseded by a new key, or the time when a previous
  // key should be dropped. If zero, then the key should not be automatically
  // rotated.
  uint64_t next_rotation_tv_sec = 0;
};

struct CertCompressionAlg {
  static constexpr bool kAllowUniquePtr = true;

  ssl_cert_compression_func_t compress = nullptr;
  ssl_cert_decompression_func_t decompress = nullptr;
  uint16_t alg_id = 0;
};

BSSL_NAMESPACE_END

DEFINE_LHASH_OF(SSL_SESSION)

BSSL_NAMESPACE_BEGIN

// An ssl_shutdown_t describes the shutdown state of one end of the connection,
// whether it is alive or has been shutdown via close_notify or fatal alert.
enum ssl_shutdown_t {
  ssl_shutdown_none = 0,
  ssl_shutdown_close_notify = 1,
  ssl_shutdown_error = 2,
};

enum ssl_ech_status_t {
  // ssl_ech_none indicates ECH was not offered, or we have not gotten far
  // enough in the handshake to determine the status.
  ssl_ech_none,
  // ssl_ech_accepted indicates the server accepted ECH.
  ssl_ech_accepted,
  // ssl_ech_rejected indicates the server was offered ECH but rejected it.
  ssl_ech_rejected,
};

#define SSL3_SEND_ALERT_SIZE 2
#define TLS_SEQ_NUM_SIZE 8
#define SSL3_CHANNEL_ID_SIZE 64
#define PREV_FINISHED_MAX_SIZE EVP_MAX_MD_SIZE

struct SSL3_STATE {
  static constexpr bool kAllowUniquePtr = true;

  SSL3_STATE();
  ~SSL3_STATE();

  uint8_t read_sequence[TLS_SEQ_NUM_SIZE] = {0};
  uint8_t write_sequence[TLS_SEQ_NUM_SIZE] = {0};

  uint8_t server_random[SSL3_RANDOM_SIZE] = {0};
  uint8_t client_random[SSL3_RANDOM_SIZE] = {0};

  // read_buffer holds data from the transport to be processed.
  SSLBuffer read_buffer;
  // write_buffer holds data to be written to the transport.
  SSLBuffer write_buffer;

  // pending_app_data is the unconsumed application data. It points into
  // |read_buffer|.
  Span<uint8_t> pending_app_data;

  // unreported_bytes_written is the number of bytes successfully written to the
  // transport, but not yet reported to the caller. The next |SSL_write| will
  // skip this many bytes from the input. This is used if
  // |SSL_MODE_ENABLE_PARTIAL_WRITE| is disabled, in which case |SSL_write| only
  // reports bytes written when the full caller input is written.
  size_t unreported_bytes_written = 0;

  // pending_write, if |has_pending_write| is true, is the caller-supplied data
  // corresponding to the current pending write. This is used to check the
  // caller retried with a compatible buffer.
  Span<const uint8_t> pending_write;

  // pending_write_type, if |has_pending_write| is true, is the record type
  // for the current pending write.
  //
  // TODO(davidben): Remove this when alerts are moved out of this write path.
  uint8_t pending_write_type = 0;

  // read_shutdown is the shutdown state for the read half of the connection.
  enum ssl_shutdown_t read_shutdown = ssl_shutdown_none;

  // write_shutdown is the shutdown state for the write half of the connection.
  enum ssl_shutdown_t write_shutdown = ssl_shutdown_none;

  // read_error, if |read_shutdown| is |ssl_shutdown_error|, is the error for
  // the receive half of the connection.
  UniquePtr<ERR_SAVE_STATE> read_error;

  int total_renegotiations = 0;

  // This holds a variable that indicates what we were doing when a 0 or -1 is
  // returned.  This is needed for non-blocking IO so we know what request
  // needs re-doing when in SSL_accept or SSL_connect
  int rwstate = SSL_ERROR_NONE;

  // key_update_pending will be either |SSL_KEY_UPDATE_REQUESTED| or
  // |SSL_KEY_UPDATE_NOT_REQUESTED| if we have a KeyUpdate acknowledgment
  // outstanding. |SSL_KEY_UPDATE_NONE| indicates that no KeyUpdates
  // acknowledgements are pending.
  int key_update_pending = SSL_KEY_UPDATE_NONE;

  enum ssl_encryption_level_t read_level = ssl_encryption_initial;
  enum ssl_encryption_level_t write_level = ssl_encryption_initial;

  // early_data_skipped is the amount of early data that has been skipped by the
  // record layer.
  uint16_t early_data_skipped = 0;

  // empty_record_count is the number of consecutive empty records received.
  uint8_t empty_record_count = 0;

  // warning_alert_count is the number of consecutive warning alerts
  // received.
  uint8_t warning_alert_count = 0;

  // ech_status indicates whether ECH was accepted by the server.
  ssl_ech_status_t ech_status = ssl_ech_none;

  // skip_early_data instructs the record layer to skip unexpected early data
  // messages when 0RTT is rejected.
  bool skip_early_data : 1;

  // have_version is true if the connection's final version is known. Otherwise
  // the version has not been negotiated yet.
  bool have_version : 1;

  // v2_hello_done is true if the peer's V2ClientHello, if any, has been handled
  // and future messages should use the record layer.
  bool v2_hello_done : 1;

  // is_v2_hello is true if the current handshake message was derived from a
  // V2ClientHello rather than received from the peer directly.
  bool is_v2_hello : 1;

  // has_message is true if the current handshake message has been returned
  // at least once by |get_message| and false otherwise.
  bool has_message : 1;

  // initial_handshake_complete is true if the initial handshake has
  // completed.
  bool initial_handshake_complete : 1;

  // session_reused indicates whether a session was resumed.
  bool session_reused : 1;

  // delegated_credential_used is whether we presented a delegated credential to
  // the peer.
  bool delegated_credential_used : 1;

  bool send_connection_binding : 1;

  // channel_id_valid is true if, on the server, the client has negotiated a
  // Channel ID and the |channel_id| field is filled in.
  bool channel_id_valid : 1;

  // early_data_accepted is true if early data was accepted by the server.
  bool early_data_accepted : 1;

  // alert_dispatch is true there is an alert in |send_alert| to be sent.
  bool alert_dispatch : 1;

  // renegotiate_pending is whether the read half of the channel is blocked on a
  // HelloRequest.
  bool renegotiate_pending : 1;

  // used_hello_retry_request is whether the handshake used a TLS 1.3
  // HelloRetryRequest message.
  bool used_hello_retry_request : 1;

  // was_key_usage_invalid is whether the handshake succeeded despite using a
  // TLS mode which was incompatible with the leaf certificate's keyUsage
  // extension.
  bool was_key_usage_invalid : 1;

  // hs_buf is the buffer of handshake data to process.
  UniquePtr<BUF_MEM> hs_buf;

  // pending_hs_data contains the pending handshake data that has not yet
  // been encrypted to |pending_flight|. This allows packing the handshake into
  // fewer records.
  UniquePtr<BUF_MEM> pending_hs_data;

  // pending_flight is the pending outgoing flight. This is used to flush each
  // handshake flight in a single write. |write_buffer| must be written out
  // before this data.
  UniquePtr<BUF_MEM> pending_flight;

  // pending_flight_offset is the number of bytes of |pending_flight| which have
  // been successfully written.
  uint32_t pending_flight_offset = 0;

  // ticket_age_skew is the difference, in seconds, between the client-sent
  // ticket age and the server-computed value in TLS 1.3 server connections
  // which resumed a session.
  int32_t ticket_age_skew = 0;

  // ssl_early_data_reason stores details on why 0-RTT was accepted or rejected.
  enum ssl_early_data_reason_t early_data_reason = ssl_early_data_unknown;

  // aead_read_ctx is the current read cipher state.
  UniquePtr<SSLAEADContext> aead_read_ctx;

  // aead_write_ctx is the current write cipher state.
  UniquePtr<SSLAEADContext> aead_write_ctx;

  // hs is the handshake state for the current handshake or NULL if there isn't
  // one.
  UniquePtr<SSL_HANDSHAKE> hs;

  // cached_x509_peer_ca_names, on the client, contains the list of CAs
  // received in a CertificateRequest message, as X509_NAMEs. This is eagerly
  // populated from |hs->ca_names| before the handshake object is destroyed, so
  // that |SSL_get_client_CA_list| can return it after the handshake.
  STACK_OF(X509_NAME) *cached_x509_peer_ca_names = nullptr;

  // peer_key is the peer's ECDH key for both TLS 1.2/1.3. This is only used
  // for observing with |SSL_get_peer_tmp_key| and is not serialized as part of
  // the SSL Transfer feature.
  Array<uint8_t> peer_key;

  uint8_t write_traffic_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t read_traffic_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t exporter_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t write_traffic_secret_len = 0;
  uint8_t read_traffic_secret_len = 0;
  uint8_t exporter_secret_len = 0;

  // Connection binding to prevent renegotiation attacks
  uint8_t previous_client_finished[PREV_FINISHED_MAX_SIZE] = {0};
  uint8_t previous_client_finished_len = 0;
  uint8_t previous_server_finished_len = 0;
  uint8_t previous_server_finished[PREV_FINISHED_MAX_SIZE] = {0};

  uint8_t send_alert[SSL3_SEND_ALERT_SIZE] = {0};

  // established_session is the session established by the connection. This
  // session is only filled upon the completion of the handshake and is
  // immutable.
  UniquePtr<SSL_SESSION> established_session;

  // Next protocol negotiation. For the client, this is the protocol that we
  // sent in NextProtocol and is set when handling ServerHello extensions.
  //
  // For a server, this is the client's selected_protocol from NextProtocol and
  // is set when handling the NextProtocol message, before the Finished
  // message.
  Array<uint8_t> next_proto_negotiated;

  // ALPN information
  // (we are in the process of transitioning from NPN to ALPN.)

  // In a server these point to the selected ALPN protocol after the
  // ClientHello has been processed. In a client these contain the protocol
  // that the server selected once the ServerHello has been processed.
  Array<uint8_t> alpn_selected;

  // hostname, on the server, is the value of the SNI extension.
  UniquePtr<char> hostname;

  // For a server:
  //     If |channel_id_valid| is true, then this contains the
  //     verified Channel ID from the client: a P256 point, (x,y), where
  //     each are big-endian values.
  uint8_t channel_id[SSL3_CHANNEL_ID_SIZE] = {0};

  // Contains the QUIC transport params received by the peer.
  Array<uint8_t> peer_quic_transport_params;

  // srtp_profile is the selected SRTP protection profile for
  // DTLS-SRTP.
  const SRTP_PROTECTION_PROFILE *srtp_profile = nullptr;
};

// lengths of messages
#define DTLS1_RT_HEADER_LENGTH 13

#define DTLS1_HM_HEADER_LENGTH 12

#define DTLS1_CCS_HEADER_LENGTH 1

#define DTLS1_AL_HEADER_LENGTH 2

struct hm_header_st {
  uint8_t type;
  uint32_t msg_len;
  uint16_t seq;
  uint32_t frag_off;
  uint32_t frag_len;
};

// An hm_fragment is an incoming DTLS message, possibly not yet assembled.
struct hm_fragment {
  static constexpr bool kAllowUniquePtr = true;

  hm_fragment() {}
  hm_fragment(const hm_fragment &) = delete;
  hm_fragment &operator=(const hm_fragment &) = delete;

  ~hm_fragment();

  // type is the type of the message.
  uint8_t type = 0;
  // seq is the sequence number of this message.
  uint16_t seq = 0;
  // msg_len is the length of the message body.
  uint32_t msg_len = 0;
  // data is a pointer to the message, including message header. It has length
  // |DTLS1_HM_HEADER_LENGTH| + |msg_len|.
  uint8_t *data = nullptr;
  // reassembly is a bitmask of |msg_len| bits corresponding to which parts of
  // the message have been received. It is NULL if the message is complete.
  uint8_t *reassembly = nullptr;
};

struct OPENSSL_timeval {
  uint64_t tv_sec;
  uint32_t tv_usec;
};

struct DTLS1_STATE {
  static constexpr bool kAllowUniquePtr = true;

  DTLS1_STATE();
  ~DTLS1_STATE();

  // has_change_cipher_spec is true if we have received a ChangeCipherSpec from
  // the peer in this epoch.
  bool has_change_cipher_spec : 1;

  // outgoing_messages_complete is true if |outgoing_messages| has been
  // completed by an attempt to flush it. Future calls to |add_message| and
  // |add_change_cipher_spec| will start a new flight.
  bool outgoing_messages_complete : 1;

  // flight_has_reply is true if the current outgoing flight is complete and has
  // processed at least one message. This is used to detect whether we or the
  // peer sent the final flight.
  bool flight_has_reply : 1;

  // The current data and handshake epoch.  This is initially undefined, and
  // starts at zero once the initial handshake is completed.
  uint16_t r_epoch = 0;
  uint16_t w_epoch = 0;

  // records being received in the current epoch
  DTLS1_BITMAP bitmap;

  uint16_t handshake_write_seq = 0;
  uint16_t handshake_read_seq = 0;

  // save last sequence number for retransmissions
  uint8_t last_write_sequence[8] = {0};
  UniquePtr<SSLAEADContext> last_aead_write_ctx;

  // incoming_messages is a ring buffer of incoming handshake messages that have
  // yet to be processed. The front of the ring buffer is message number
  // |handshake_read_seq|, at position |handshake_read_seq| %
  // |SSL_MAX_HANDSHAKE_FLIGHT|.
  UniquePtr<hm_fragment> incoming_messages[SSL_MAX_HANDSHAKE_FLIGHT];

  // outgoing_messages is the queue of outgoing messages from the last handshake
  // flight.
  DTLS_OUTGOING_MESSAGE outgoing_messages[SSL_MAX_HANDSHAKE_FLIGHT];
  uint8_t outgoing_messages_len = 0;

  // outgoing_written is the number of outgoing messages that have been
  // written.
  uint8_t outgoing_written = 0;
  // outgoing_offset is the number of bytes of the next outgoing message have
  // been written.
  uint32_t outgoing_offset = 0;

  unsigned mtu = 0;  // max DTLS packet size

  // num_timeouts is the number of times the retransmit timer has fired since
  // the last time it was reset.
  unsigned num_timeouts = 0;

  // Indicates when the last handshake msg or heartbeat sent will
  // timeout.
  struct OPENSSL_timeval next_timeout = {0, 0};

  // timeout_duration_ms is the timeout duration in milliseconds.
  unsigned timeout_duration_ms = 0;
};

// An ALPSConfig is a pair of ALPN protocol and settings value to use with ALPS.
struct ALPSConfig {
  Array<uint8_t> protocol;
  Array<uint8_t> settings;
};

// SSL_CONFIG contains configuration bits that can be shed after the handshake
// completes.  Objects of this type are not shared; they are unique to a
// particular |SSL|.
//
// See SSL_shed_handshake_config() for more about the conditions under which
// configuration can be shed.
struct SSL_CONFIG {
  static constexpr bool kAllowUniquePtr = true;

  explicit SSL_CONFIG(SSL *ssl_arg);
  ~SSL_CONFIG();

  // ssl is a non-owning pointer to the parent |SSL| object.
  SSL *const ssl = nullptr;

  // conf_max_version is the maximum acceptable version configured by
  // |SSL_set_max_proto_version|. Note this version is not normalized in DTLS
  // and is further constrained by |SSL_OP_NO_*|.
  uint16_t conf_max_version = 0;

  // conf_min_version is the minimum acceptable version configured by
  // |SSL_set_min_proto_version|. Note this version is not normalized in DTLS
  // and is further constrained by |SSL_OP_NO_*|.
  uint16_t conf_min_version = 0;

  X509_VERIFY_PARAM *param = nullptr;

  // cipher_list holds all available cipher suites for tls 1.3,
  // and 1.2 and below. Any configured ciphersuites here take precedence
  // over the parent |SSL_CTX| object.
  UniquePtr<SSLCipherPreferenceList> cipher_list;

  // tls13_cipher_list holds the default or configured tls1.3 and above
  // cipher suites. Any configured ciphersuites here take precedence
  // over the parent |SSL_CTX| object.
  UniquePtr<SSLCipherPreferenceList> tls13_cipher_list;

  // This is used to hold the local certificate used (i.e. the server
  // certificate for a server or the client certificate for a client).
  UniquePtr<CERT> cert;

  int (*verify_callback)(int ok,
                         X509_STORE_CTX *ctx) =
      nullptr;  // fail if callback returns 0

  enum ssl_verify_result_t (*custom_verify_callback)(
      SSL *ssl, uint8_t *out_alert) = nullptr;
  // Server-only: psk_identity_hint is the identity hint to send in
  // PSK-based key exchanges.
  UniquePtr<char> psk_identity_hint;

  unsigned (*psk_client_callback)(SSL *ssl, const char *hint, char *identity,
                                  unsigned max_identity_len, uint8_t *psk,
                                  unsigned max_psk_len) = nullptr;
  unsigned (*psk_server_callback)(SSL *ssl, const char *identity, uint8_t *psk,
                                  unsigned max_psk_len) = nullptr;

  // for server side, keep the list of CA_dn we can use
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> client_CA;

  // cached_x509_client_CA is a cache of parsed versions of the elements of
  // |client_CA|.
  STACK_OF(X509_NAME) *cached_x509_client_CA = nullptr;

  Array<uint16_t> supported_group_list;  // our list

  // channel_id_private is the client's Channel ID private key, or null if
  // Channel ID should not be offered on this connection.
  UniquePtr<EVP_PKEY> channel_id_private;

  // For a client, this contains the list of supported protocols in wire
  // format.
  Array<uint8_t> alpn_client_proto_list;

  // alps_configs contains the list of supported protocols to use with ALPS,
  // along with their corresponding ALPS values.
  GrowableArray<ALPSConfig> alps_configs;

  // Contains the QUIC transport params that this endpoint will send.
  Array<uint8_t> quic_transport_params;

  // Contains the context used to decide whether to accept early data in QUIC.
  Array<uint8_t> quic_early_data_context;

  // verify_sigalgs, if not empty, is the set of signature algorithms
  // accepted from the peer in decreasing order of preference.
  Array<uint16_t> verify_sigalgs;

  // srtp_profiles is the list of configured SRTP protection profiles for
  // DTLS-SRTP.
  UniquePtr<STACK_OF(SRTP_PROTECTION_PROFILE)> srtp_profiles;

  // client_ech_config_list, if not empty, is a serialized ECHConfigList
  // structure for the client to use when negotiating ECH.
  Array<uint8_t> client_ech_config_list;

  // verify_mode is a bitmask of |SSL_VERIFY_*| values.
  uint8_t verify_mode = SSL_VERIFY_NONE;

  // ech_grease_enabled controls whether ECH GREASE may be sent in the
  // ClientHello.
  bool ech_grease_enabled : 1;

  // Enable signed certificate time stamps. Currently client only.
  bool signed_cert_timestamps_enabled : 1;

  // ocsp_stapling_enabled is only used by client connections and indicates
  // whether OCSP stapling will be requested.
  bool ocsp_stapling_enabled : 1;

  // channel_id_enabled is copied from the |SSL_CTX|. For a server, it means
  // that we'll accept Channel IDs from clients. It is ignored on the client.
  bool channel_id_enabled : 1;

  // If enforce_rsa_key_usage is true, the handshake will fail if the
  // keyUsage extension is present and incompatible with the TLS usage.
  // This field is not read until after certificate verification.
  bool enforce_rsa_key_usage : 1;

  // retain_only_sha256_of_client_certs is true if we should compute the SHA256
  // hash of the peer's certificate and then discard it to save memory and
  // session space. Only effective on the server side.
  bool retain_only_sha256_of_client_certs : 1;

  // handoff indicates that a server should stop after receiving the
  // ClientHello and pause the handshake in such a way that |SSL_get_error|
  // returns |SSL_ERROR_HANDOFF|. This is copied in |SSL_new| from the |SSL_CTX|
  // element of the same name and may be cleared if the handoff is declined.
  bool handoff : 1;

  // shed_handshake_config indicates that the handshake config (this object!)
  // should be freed after the handshake completes.
  bool shed_handshake_config : 1;

  // jdk11_workaround is whether to disable TLS 1.3 for JDK 11 clients, as a
  // workaround for https://bugs.openjdk.java.net/browse/JDK-8211806.
  bool jdk11_workaround : 1;

  // QUIC drafts up to and including 32 used a different TLS extension
  // codepoint to convey QUIC's transport parameters.
  bool quic_use_legacy_codepoint : 1;

  // permute_extensions is whether to permute extensions when sending messages.
  bool permute_extensions : 1;

  // aes_hw_override if set indicates we should override checking for aes
  // hardware support, and use the value in aes_hw_override_value instead.
  bool aes_hw_override : 1;

  // aes_hw_override_value is used for testing to indicate the support or lack
  // of support for AES hw. The value is only considered if |aes_hw_override| is
  // true.
  bool aes_hw_override_value : 1;

  // conf_max_version_use_default indicates whether the |SSL_CONFIG| is configured
  // to use the default maximum protocol version for the relevant protocol
  // method. By default, |SSL_new| will set this to true and connections will use
  // the default max version. callers can change the max version used by calling
  // |SSL_set_max_proto_version| with a non-zero value.
  bool conf_max_version_use_default : 1;

  // conf_min_version_use_default indicates whether the |SSL_CONFIG| is configured
  // to use the default minimum protocol version for the relevant protocol
  // method. By default, |SSL_new| will set this to true and connections will use
  // the default min version. callers can change the min version used by calling
  // |SSL_set_min_proto_version| with a non-zero value.
  bool conf_min_version_use_default : 1;

  // alps_use_new_codepoint if set indicates we use new ALPS extension codepoint
  // to negotiate and convey application settings.
  bool alps_use_new_codepoint : 1;

  // check_client_certificate_type indicates whether the client, in TLS 1.2 and
  // below, will check its certificate against the server's requested
  // certificate types.
  bool check_client_certificate_type : 1;
};

// From RFC 8446, used in determining PSK modes.
#define SSL_PSK_DHE_KE 0x1

// kMaxEarlyDataAccepted is the advertised number of plaintext bytes of early
// data that will be accepted. This value should be slightly below
// kMaxEarlyDataSkipped in tls_record.c, which is measured in ciphertext.
static const size_t kMaxEarlyDataAccepted = 14336;

UniquePtr<CERT> ssl_cert_dup(CERT *cert);
bool ssl_set_cert(CERT *cert, UniquePtr<CRYPTO_BUFFER> buffer);
bool ssl_is_key_type_supported(int key_type);
// ssl_compare_public_and_private_key returns true if |pubkey| is the public
// counterpart to |privkey|. Otherwise it returns false and pushes a helpful
// message on the error queue.
bool ssl_compare_public_and_private_key(const EVP_PKEY *pubkey,
                                        const EVP_PKEY *privkey);
bool ssl_cert_check_private_key(const CERT *cert, const EVP_PKEY *privkey);

CRYPTO_BUFFER *buffer_up_ref(const CRYPTO_BUFFER *buffer);

// ssl_cert_check_cert_private_keys_usage returns true if |cert_private_keys|
// in |cert| has a valid index and a sufficient amount of slots.
bool ssl_cert_check_cert_private_keys_usage(const CERT *cert);

bool ssl_get_new_session(SSL_HANDSHAKE *hs);
bool ssl_encrypt_ticket(SSL_HANDSHAKE *hs, CBB *out,
                        const SSL_SESSION *session);
bool ssl_ctx_rotate_ticket_encryption_key(SSL_CTX *ctx);

// ssl_session_new returns a newly-allocated blank |SSL_SESSION| or nullptr on
// error.
UniquePtr<SSL_SESSION> ssl_session_new(const SSL_X509_METHOD *x509_method);

// ssl_hash_session_id returns a hash of |session_id|, suitable for a hash table
// keyed on session IDs.
uint32_t ssl_hash_session_id(Span<const uint8_t> session_id);

// SSL_SESSION_parse parses an |SSL_SESSION| from |cbs| and advances |cbs| over
// the parsed data.
OPENSSL_EXPORT UniquePtr<SSL_SESSION> SSL_SESSION_parse(
    CBS *cbs, const SSL_X509_METHOD *x509_method, CRYPTO_BUFFER_POOL *pool);

// ssl_session_serialize writes |in| to |cbb| as if it were serialising a
// session for Session-ID resumption. It returns true on success and false on
// error.
OPENSSL_EXPORT bool ssl_session_serialize(const SSL_SESSION *in, CBB *cbb);

// ssl_session_is_context_valid returns whether |session|'s session ID context
// matches the one set on |hs|.
bool ssl_session_is_context_valid(const SSL_HANDSHAKE *hs,
                                  const SSL_SESSION *session);

// ssl_session_is_time_valid returns true if |session| is still valid and false
// if it has expired.
bool ssl_session_is_time_valid(const SSL *ssl, const SSL_SESSION *session);

// ssl_session_is_resumable returns whether |session| is resumable for |hs|.
bool ssl_session_is_resumable(const SSL_HANDSHAKE *hs,
                              const SSL_SESSION *session);

// ssl_session_protocol_version returns the protocol version associated with
// |session|. Note that despite the name, this is not the same as
// |SSL_SESSION_get_protocol_version|. The latter is based on upstream's name.
uint16_t ssl_session_protocol_version(const SSL_SESSION *session);

// ssl_session_get_digest returns the digest used in |session|.
const EVP_MD *ssl_session_get_digest(const SSL_SESSION *session);

void ssl_set_session(SSL *ssl, SSL_SESSION *session);

// ssl_get_prev_session looks up the previous session based on |client_hello|.
// On success, it sets |*out_session| to the session or nullptr if none was
// found. If the session could not be looked up synchronously, it returns
// |ssl_hs_pending_session| and should be called again. If a ticket could not be
// decrypted immediately it returns |ssl_hs_pending_ticket| and should also
// be called again. Otherwise, it returns |ssl_hs_error|.
enum ssl_hs_wait_t ssl_get_prev_session(SSL_HANDSHAKE *hs,
                                        UniquePtr<SSL_SESSION> *out_session,
                                        bool *out_tickets_supported,
                                        bool *out_renew_ticket,
                                        const SSL_CLIENT_HELLO *client_hello);

// The following flags determine which parts of the session are duplicated.
#define SSL_SESSION_DUP_AUTH_ONLY 0x0
#define SSL_SESSION_INCLUDE_TICKET 0x1
#define SSL_SESSION_INCLUDE_NONAUTH 0x2
#define SSL_SESSION_DUP_ALL \
  (SSL_SESSION_INCLUDE_TICKET | SSL_SESSION_INCLUDE_NONAUTH)

// SSL_SESSION_dup returns a newly-allocated |SSL_SESSION| with a copy of the
// fields in |session| or nullptr on error. The new session is non-resumable and
// must be explicitly marked resumable once it has been filled in.
OPENSSL_EXPORT UniquePtr<SSL_SESSION> SSL_SESSION_dup(SSL_SESSION *session,
                                                      int dup_flags);

// ssl_session_rebase_time updates |session|'s start time to the current time,
// adjusting the timeout so the expiration time is unchanged.
void ssl_session_rebase_time(SSL *ssl, SSL_SESSION *session);

// ssl_session_renew_timeout calls |ssl_session_rebase_time| and renews
// |session|'s timeout to |timeout| (measured from the current time). The
// renewal is clamped to the session's auth_timeout.
void ssl_session_renew_timeout(SSL *ssl, SSL_SESSION *session,
                               uint32_t timeout);

void ssl_update_cache(SSL *ssl);

void ssl_send_alert(SSL *ssl, int level, int desc);
int ssl_send_alert_impl(SSL *ssl, int level, int desc);
bool tls_get_message(const SSL *ssl, SSLMessage *out);
ssl_open_record_t tls_open_handshake(SSL *ssl, size_t *out_consumed,
                                     uint8_t *out_alert, Span<uint8_t> in);
void tls_next_message(SSL *ssl);

int tls_dispatch_alert(SSL *ssl);
ssl_open_record_t tls_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                    size_t *out_consumed, uint8_t *out_alert,
                                    Span<uint8_t> in);
ssl_open_record_t tls_open_change_cipher_spec(SSL *ssl, size_t *out_consumed,
                                              uint8_t *out_alert,
                                              Span<uint8_t> in);
int tls_write_app_data(SSL *ssl, bool *out_needs_handshake,
                       size_t *out_bytes_written, Span<const uint8_t> in);

bool tls_new(SSL *ssl);
void tls_free(SSL *ssl);

bool tls_init_message(const SSL *ssl, CBB *cbb, CBB *body, uint8_t type);
bool tls_finish_message(const SSL *ssl, CBB *cbb, Array<uint8_t> *out_msg);
bool tls_add_message(SSL *ssl, Array<uint8_t> msg);
bool tls_add_change_cipher_spec(SSL *ssl);
int tls_flush_flight(SSL *ssl);

bool dtls1_init_message(const SSL *ssl, CBB *cbb, CBB *body, uint8_t type);
bool dtls1_finish_message(const SSL *ssl, CBB *cbb, Array<uint8_t> *out_msg);
bool dtls1_add_message(SSL *ssl, Array<uint8_t> msg);
bool dtls1_add_change_cipher_spec(SSL *ssl);
int dtls1_flush_flight(SSL *ssl);

// ssl_add_message_cbb finishes the handshake message in |cbb| and adds it to
// the pending flight. It returns true on success and false on error.
bool ssl_add_message_cbb(SSL *ssl, CBB *cbb);

// ssl_hash_message incorporates |msg| into the handshake hash. It returns true
// on success and false on allocation failure.
bool ssl_hash_message(SSL_HANDSHAKE *hs, const SSLMessage &msg);

ssl_open_record_t dtls1_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                      size_t *out_consumed, uint8_t *out_alert,
                                      Span<uint8_t> in);
ssl_open_record_t dtls1_open_change_cipher_spec(SSL *ssl, size_t *out_consumed,
                                                uint8_t *out_alert,
                                                Span<uint8_t> in);

int dtls1_write_app_data(SSL *ssl, bool *out_needs_handshake,
                         size_t *out_bytes_written, Span<const uint8_t> in);

// dtls1_write_record sends a record. It returns one on success and <= 0 on
// error.
int dtls1_write_record(SSL *ssl, int type, Span<const uint8_t> in,
                       enum dtls1_use_epoch_t use_epoch);

int dtls1_retransmit_outgoing_messages(SSL *ssl);
bool dtls1_parse_fragment(CBS *cbs, struct hm_header_st *out_hdr,
                          CBS *out_body);
bool dtls1_check_timeout_num(SSL *ssl);

void dtls1_start_timer(SSL *ssl);
void dtls1_stop_timer(SSL *ssl);
bool dtls1_is_timer_expired(SSL *ssl);
unsigned int dtls1_min_mtu(void);

bool dtls1_new(SSL *ssl);
void dtls1_free(SSL *ssl);

bool dtls1_get_message(const SSL *ssl, SSLMessage *out);
ssl_open_record_t dtls1_open_handshake(SSL *ssl, size_t *out_consumed,
                                       uint8_t *out_alert, Span<uint8_t> in);
void dtls1_next_message(SSL *ssl);
int dtls1_dispatch_alert(SSL *ssl);

// tls1_configure_aead configures either the read or write direction AEAD (as
// determined by |direction|) using the keys generated by the TLS KDF. The
// |key_block_cache| argument is used to store the generated key block, if
// empty. Otherwise it's assumed that the key block is already contained within
// it. It returns true on success or false on error.
bool tls1_configure_aead(SSL *ssl, evp_aead_direction_t direction,
                         Array<uint8_t> *key_block_cache,
                         const SSL_SESSION *session,
                         Span<const uint8_t> iv_override);

bool tls1_change_cipher_state(SSL_HANDSHAKE *hs,
                              evp_aead_direction_t direction);
int tls1_generate_master_secret(SSL_HANDSHAKE *hs, uint8_t *out,
                                Span<const uint8_t> premaster);

// tls1_get_default_grouplist returns the default group list
OPENSSL_EXPORT Span<const uint16_t> tls1_get_default_grouplist(void);

// tls1_get_grouplist returns the locally-configured group preference list.
Span<const uint16_t> tls1_get_grouplist(const SSL_HANDSHAKE *ssl);

// tls1_check_group_id returns whether |group_id| is consistent with locally-
// configured group preferences.
bool tls1_check_group_id(const SSL_HANDSHAKE *ssl, uint16_t group_id);

// tls1_get_shared_group sets |*out_group_id| to the first preferred shared
// group between client and server preferences and returns true. If none may be
// found, it returns false.
bool tls1_get_shared_group(SSL_HANDSHAKE *hs, uint16_t *out_group_id);

// tls1_set_curves converts the array of NIDs in |curves| into a newly allocated
// array of TLS group IDs. On success, the function returns true and writes the
// array to |*out_group_ids|. Otherwise, it returns false.
bool tls1_set_curves(Array<uint16_t> *out_group_ids, Span<const int> curves);

// tls1_set_curves_list converts the string of curves pointed to by |curves|
// into a newly allocated array of TLS group IDs. On success, the function
// returns true and writes the array to |*out_group_ids|. Otherwise, it returns
// false.
bool tls1_set_curves_list(Array<uint16_t> *out_group_ids, const char *curves);

// tls1_call_ocsp_stapling_callback calls the legacy OCSP logic for TLS
// handshakes. This should be called right after the server certificate has been
// finalized.
bool tls1_call_ocsp_stapling_callback(SSL_HANDSHAKE *hs);

// ssl_add_clienthello_tlsext writes ClientHello extensions to |out| for |type|.
// It returns true on success and false on failure. The |header_len| argument is
// the length of the ClientHello written so far and is used to compute the
// padding length. (It does not include the record header or handshake headers.)
//
// If |type| is |ssl_client_hello_inner|, this function also writes the
// compressed extensions to |out_encoded|. Otherwise, |out_encoded| should be
// nullptr.
//
// On success, the function sets |*out_needs_psk_binder| to whether the last
// ClientHello extension was the pre_shared_key extension and needs a PSK binder
// filled in. The caller should then update |out| and, if applicable,
// |out_encoded| with the binder after completing the whole message.
bool ssl_add_clienthello_tlsext(SSL_HANDSHAKE *hs, CBB *out, CBB *out_encoded,
                                bool *out_needs_psk_binder,
                                ssl_client_hello_type_t type,
                                size_t header_len);

bool ssl_add_serverhello_tlsext(SSL_HANDSHAKE *hs, CBB *out);
bool ssl_parse_clienthello_tlsext(SSL_HANDSHAKE *hs,
                                  const SSL_CLIENT_HELLO *client_hello);
bool ssl_parse_serverhello_tlsext(SSL_HANDSHAKE *hs, const CBS *extensions);


// SSL serialization.

// ssl_transfer_supported returns true when the |in| is supported by
// current SSL transfer scope. Specific support scope is included inside
// the function.
// Otherwise, it returns false.
OPENSSL_EXPORT bool ssl_transfer_supported(const SSL *in);

#define tlsext_tick_md EVP_sha256

// ssl_process_ticket processes a session ticket from the client. It returns
// one of:
//   |ssl_ticket_aead_success|: |*out_session| is set to the parsed session and
//       |*out_renew_ticket| is set to whether the ticket should be renewed.
//   |ssl_ticket_aead_ignore_ticket|: |*out_renew_ticket| is set to whether a
//       fresh ticket should be sent, but the given ticket cannot be used.
//   |ssl_ticket_aead_retry|: the ticket could not be immediately decrypted.
//       Retry later.
//   |ssl_ticket_aead_error|: an error occured that is fatal to the connection.
enum ssl_ticket_aead_result_t ssl_process_ticket(
    SSL_HANDSHAKE *hs, UniquePtr<SSL_SESSION> *out_session,
    bool *out_renew_ticket, Span<const uint8_t> ticket,
    Span<const uint8_t> session_id);

// tls1_verify_channel_id processes |msg| as a Channel ID message, and verifies
// the signature. If the key is valid, it saves the Channel ID and returns true.
// Otherwise, it returns false.
bool tls1_verify_channel_id(SSL_HANDSHAKE *hs, const SSLMessage &msg);

// tls1_write_channel_id generates a Channel ID message and puts the output in
// |cbb|. |ssl->channel_id_private| must already be set before calling.  This
// function returns true on success and false on error.
bool tls1_write_channel_id(SSL_HANDSHAKE *hs, CBB *cbb);

// tls1_channel_id_hash computes the hash to be signed by Channel ID and writes
// it to |out|, which must contain at least |EVP_MAX_MD_SIZE| bytes. It returns
// true on success and false on failure.
bool tls1_channel_id_hash(SSL_HANDSHAKE *hs, uint8_t *out, size_t *out_len);

// tls1_record_handshake_hashes_for_channel_id records the current handshake
// hashes in |hs->new_session| so that Channel ID resumptions can sign that
// data.
bool tls1_record_handshake_hashes_for_channel_id(SSL_HANDSHAKE *hs);

// ssl_can_write returns whether |ssl| is allowed to write.
bool ssl_can_write(const SSL *ssl);

// ssl_can_read returns wheter |ssl| is allowed to read.
bool ssl_can_read(const SSL *ssl);

void ssl_get_current_time(const SSL *ssl, struct OPENSSL_timeval *out_clock);
void ssl_ctx_get_current_time(const SSL_CTX *ctx,
                              struct OPENSSL_timeval *out_clock);

// ssl_reset_error_state resets state for |SSL_get_error|.
void ssl_reset_error_state(SSL *ssl);

// ssl_set_read_error sets |ssl|'s read half into an error state, saving the
// current state of the error queue.
void ssl_set_read_error(SSL *ssl);

// ssl_update_counter updates the stat counters in |SSL_CTX|. lock should be
// set to false when the mutex in |SSL_CTX| has already been locked.
void ssl_update_counter(SSL_CTX *ctx, SSL_STATS_COUNTER_TYPE &counter, bool lock);

BSSL_NAMESPACE_END

// ssl_x509_persist_peer_ca_names eagerly converts the peer CA names from
// |hs->ca_names| to X509_NAMEs and stores them in
// |ssl->s3->cached_x509_peer_ca_names|, so they remain available via
// |SSL_get_client_CA_list| after the handshake object is destroyed. It returns
// true on success and false on allocation failure.
bool ssl_x509_persist_peer_ca_names(SSL *ssl);


// Opaque C types.
//
// The following types are exported to C code as public typedefs, so they must
// be defined outside of the namespace.

// ssl_method_st backs the public |SSL_METHOD| type. It is a compatibility
// structure to support the legacy version-locked methods.
struct ssl_method_st {
  // version, if non-zero, is the only protocol version acceptable to an
  // SSL_CTX initialized from this method.
  uint16_t version;
  // method is the underlying SSL_PROTOCOL_METHOD that initializes the
  // SSL_CTX.
  const bssl::SSL_PROTOCOL_METHOD *method;
  // x509_method contains pointers to functions that might deal with |X509|
  // compatibility, or might be a no-op, depending on the application.
  const bssl::SSL_X509_METHOD *x509_method;
};

// TLS13_DEFAULT_CIPHER_LIST_AES_HW is the default TLS 1.3 cipher suite
// configuration when AES hardware acceleration is enabled.
#define TLS13_DEFAULT_CIPHER_LIST_AES_HW "TLS_AES_128_GCM_SHA256:" \
                                         "TLS_AES_256_GCM_SHA384:" \
                                         "TLS_CHACHA20_POLY1305_SHA256"

// TLS13_DEFAULT_CIPHER_LIST_NO_AES_HW is the default TLS 1.3 cipher suite
// configuration when no AES hardware acceleration is enabled.
#define TLS13_DEFAULT_CIPHER_LIST_NO_AES_HW "TLS_CHACHA20_POLY1305_SHA256:" \
                                            "TLS_AES_128_GCM_SHA256:" \
                                            "TLS_AES_256_GCM_SHA384"

#define MIN_SAFE_FRAGMENT_SIZE 512
struct ssl_ctx_st : public bssl::RefCounted<ssl_ctx_st> {
  explicit ssl_ctx_st(const SSL_METHOD *ssl_method);
  ssl_ctx_st(const ssl_ctx_st &) = delete;
  ssl_ctx_st &operator=(const ssl_ctx_st &) = delete;

  const bssl::SSL_PROTOCOL_METHOD *method = nullptr;
  const bssl::SSL_X509_METHOD *x509_method = nullptr;

  // lock is used to protect various operations on this object.
  CRYPTO_MUTEX lock;

  // conf_max_version is the maximum acceptable protocol version configured by
  // |SSL_CTX_set_max_proto_version|. Note this version is normalized in DTLS
  // and is further constrainted by |SSL_OP_NO_*|.
  uint16_t conf_max_version = 0;

  // conf_min_version is the minimum acceptable protocol version configured by
  // |SSL_CTX_set_min_proto_version|. Note this version is normalized in DTLS
  // and is further constrainted by |SSL_OP_NO_*|.
  uint16_t conf_min_version = 0;

  // num_tickets is the number of tickets to send immediately after the TLS 1.3
  // handshake. TLS 1.3 recommends single-use tickets so, by default, issue two
  /// in case the client makes several connections before getting a renewal.
  uint8_t num_tickets = 2;

  // read_ahead_buffer_size is the amount of data to read if |enable_read_ahead| is true
  size_t read_ahead_buffer_size = SSL3_RT_MAX_PLAIN_LENGTH + SSL3_RT_MAX_ENCRYPTED_OVERHEAD;

  // quic_method is the method table corresponding to the QUIC hooks.
  const SSL_QUIC_METHOD *quic_method = nullptr;

  // cipher_list holds all available cipher suites for tls 1.3,
  // and 1.2 and below
  bssl::UniquePtr<bssl::SSLCipherPreferenceList> cipher_list;

  // tls13_cipher_list holds the default or configured tls1.3 and above
  // cipher suites.
  bssl::UniquePtr<bssl::SSLCipherPreferenceList> tls13_cipher_list;

  X509_STORE *cert_store = nullptr;
  LHASH_OF(SSL_SESSION) *sessions = nullptr;
  // Most session-ids that will be cached, default is
  // SSL_SESSION_CACHE_MAX_SIZE_DEFAULT. 0 is unlimited.
  unsigned long session_cache_size = SSL_SESSION_CACHE_MAX_SIZE_DEFAULT;
  SSL_SESSION *session_cache_head = nullptr;
  SSL_SESSION *session_cache_tail = nullptr;

  // handshakes_since_cache_flush is the number of successful handshakes since
  // the last cache flush.
  int handshakes_since_cache_flush = 0;

  // This can have one of 2 values, ored together,
  // SSL_SESS_CACHE_CLIENT,
  // SSL_SESS_CACHE_SERVER,
  // Default is SSL_SESSION_CACHE_SERVER, which means only
  // SSL_accept which cache SSL_SESSIONS.
  int session_cache_mode = SSL_SESS_CACHE_SERVER;

  // session_timeout is the default lifetime for new sessions in TLS 1.2 and
  // earlier, in seconds.
  uint32_t session_timeout = SSL_DEFAULT_SESSION_TIMEOUT;

  // session_psk_dhe_timeout is the default lifetime for new sessions in TLS
  // 1.3, in seconds.
  uint32_t session_psk_dhe_timeout = SSL_DEFAULT_SESSION_PSK_DHE_TIMEOUT;

  // If this callback is not null, it will be called each time a session id is
  // added to the cache.  If this function returns 1, it means that the
  // callback will do a SSL_SESSION_free() when it has finished using it.
  // Otherwise, on 0, it means the callback has finished with it. If
  // remove_session_cb is not null, it will be called when a session-id is
  // removed from the cache.  After the call, OpenSSL will SSL_SESSION_free()
  // it.
  int (*new_session_cb)(SSL *ssl, SSL_SESSION *sess) = nullptr;
  void (*remove_session_cb)(SSL_CTX *ctx, SSL_SESSION *sess) = nullptr;
  SSL_SESSION *(*get_session_cb)(SSL *ssl, const uint8_t *data, int len,
                                 int *copy) = nullptr;

  struct {
    SSL_STATS_COUNTER_TYPE sess_connect{};              // SSL new conn - started
    SSL_STATS_COUNTER_TYPE sess_connect_renegotiate{};  // SSL reneg - requested
    SSL_STATS_COUNTER_TYPE sess_connect_good{};         // SSL new conne/reneg - finished
    SSL_STATS_COUNTER_TYPE sess_accept{};               // SSL new accept - started
    SSL_STATS_COUNTER_TYPE sess_accept_good{};          // SSL accept/reneg - finished
    SSL_STATS_COUNTER_TYPE sess_miss{};                 // session lookup misses
    SSL_STATS_COUNTER_TYPE sess_timeout{};              // reuse attempt on timeouted session
    SSL_STATS_COUNTER_TYPE sess_cache_full{};           // session removed due to full cache
    SSL_STATS_COUNTER_TYPE sess_hit{};                  // session reuse actually done
    SSL_STATS_COUNTER_TYPE sess_cb_hit{};               // session-id that was not
                                       // in the cache was
                                       // passed back via the callback. This
                                       // indicates that the application is
                                       // supplying session-id's from other
                                       // processes - spooky :-)
  } stats;

  // if defined, these override the X509_verify_cert() calls
  int (*app_verify_callback)(X509_STORE_CTX *store_ctx, void *arg) = nullptr;
  void *app_verify_arg = nullptr;

  ssl_verify_result_t (*custom_verify_callback)(SSL *ssl,
                                                uint8_t *out_alert) = nullptr;

  // Default password callback.
  pem_password_cb *default_passwd_callback = nullptr;

  // Default password callback user data.
  void *default_passwd_callback_userdata = nullptr;

  // get client cert callback
  int (*client_cert_cb)(SSL *ssl, X509 **out_x509,
                        EVP_PKEY **out_pkey) = nullptr;

  CRYPTO_EX_DATA ex_data;

  // custom_*_extensions stores any callback sets for custom extensions. Note
  // that these pointers will be NULL if the stack would otherwise be empty.
  STACK_OF(SSL_CUSTOM_EXTENSION) *client_custom_extensions = nullptr;
  STACK_OF(SSL_CUSTOM_EXTENSION) *server_custom_extensions = nullptr;

  // Default values used when no per-SSL value is defined follow

  void (*info_callback)(const SSL *ssl, int type, int value) = nullptr;

  // what we put in client cert requests
  bssl::UniquePtr<STACK_OF(CRYPTO_BUFFER)> client_CA;

  // cached_x509_client_CA is a cache of parsed versions of the elements of
  // |client_CA|.
  STACK_OF(X509_NAME) *cached_x509_client_CA = nullptr;


  // Default values to use in SSL structures follow (these are copied by
  // SSL_new)

  uint32_t options = 0;
  // Disable the auto-chaining feature by default. wpa_supplicant relies on this
  // feature, but require callers opt into it.
  uint32_t mode = SSL_MODE_NO_AUTO_CHAIN;
  uint32_t max_cert_list = SSL_MAX_CERT_LIST_DEFAULT;

  bssl::UniquePtr<bssl::CERT> cert;

  // callback that allows applications to peek at protocol messages
  void (*msg_callback)(int is_write, int version, int content_type,
                       const void *buf, size_t len, SSL *ssl,
                       void *arg) = nullptr;
  void *msg_callback_arg = nullptr;

  SSL_security_callback security_callback = nullptr;
  void *security_callback_ex_data = nullptr;

  int verify_mode = SSL_VERIFY_NONE;
  int (*default_verify_callback)(int ok, X509_STORE_CTX *ctx) =
      nullptr;  // called 'verify_callback' in the SSL

  X509_VERIFY_PARAM *param = nullptr;

  // select_certificate_cb is called before most ClientHello processing and
  // before the decision whether to resume a session is made. See
  // |ssl_select_cert_result_t| for details of the return values.
  ssl_select_cert_result_t (*select_certificate_cb)(const SSL_CLIENT_HELLO *) =
      nullptr;

  // dos_protection_cb is called once the resumption decision for a ClientHello
  // has been made. It returns one to continue the handshake or zero to
  // abort.
  int (*dos_protection_cb)(const SSL_CLIENT_HELLO *) = nullptr;

  // Controls whether to verify certificates when resuming connections. They
  // were already verified when the connection was first made, so the default is
  // false. For now, this is only respected on clients, not servers.
  bool reverify_on_resume = false;

  // Maximum amount of data to send in one fragment. actual record size can be
  // more than this due to padding and MAC overheads.
  uint16_t max_send_fragment = SSL3_RT_MAX_PLAIN_LENGTH;

  /* ClientHello callback.  Mostly for extensions, but not entirely. */
  SSL_client_hello_cb_fn client_hello_cb = NULL;
  void *client_hello_cb_arg = NULL;

  // TLS extensions servername callback
  int (*servername_callback)(SSL *, int *, void *) = nullptr;
  void *servername_arg = nullptr;

  // RFC 4507 session ticket keys. |ticket_key_current| may be NULL before the
  // first handshake and |ticket_key_prev| may be NULL at any time.
  // Automatically generated ticket keys are rotated as needed at handshake
  // time. Hence, all access must be synchronized through |lock|.
  bssl::UniquePtr<bssl::TicketKey> ticket_key_current;
  bssl::UniquePtr<bssl::TicketKey> ticket_key_prev;

  // Callback to support customisation of ticket key setting
  int (*ticket_key_cb)(SSL *ssl, uint8_t *name, uint8_t *iv,
                       EVP_CIPHER_CTX *ectx, HMAC_CTX *hctx, int enc) = nullptr;

  // Server-only: psk_identity_hint is the default identity hint to send in
  // PSK-based key exchanges.
  bssl::UniquePtr<char> psk_identity_hint;

  unsigned (*psk_client_callback)(SSL *ssl, const char *hint, char *identity,
                                  unsigned max_identity_len, uint8_t *psk,
                                  unsigned max_psk_len) = nullptr;
  unsigned (*psk_server_callback)(SSL *ssl, const char *identity, uint8_t *psk,
                                  unsigned max_psk_len) = nullptr;


  // Next protocol negotiation information
  // (for experimental NPN extension).

  // For a server, this contains a callback function by which the set of
  // advertised protocols can be provided.
  int (*next_protos_advertised_cb)(SSL *ssl, const uint8_t **out,
                                   unsigned *out_len, void *arg) = nullptr;
  void *next_protos_advertised_cb_arg = nullptr;
  // For a client, this contains a callback function that selects the
  // next protocol from the list provided by the server.
  int (*next_proto_select_cb)(SSL *ssl, uint8_t **out, uint8_t *out_len,
                              const uint8_t *in, unsigned in_len,
                              void *arg) = nullptr;
  void *next_proto_select_cb_arg = nullptr;

  // ALPN information
  // (we are in the process of transitioning from NPN to ALPN.)

  // For a server, this contains a callback function that allows the
  // server to select the protocol for the connection.
  //   out: on successful return, this must point to the raw protocol
  //        name (without the length prefix).
  //   outlen: on successful return, this contains the length of |*out|.
  //   in: points to the client's list of supported protocols in
  //       wire-format.
  //   inlen: the length of |in|.
  int (*alpn_select_cb)(SSL *ssl, const uint8_t **out, uint8_t *out_len,
                        const uint8_t *in, unsigned in_len,
                        void *arg) = nullptr;
  void *alpn_select_cb_arg = nullptr;

  // For a client, this contains the list of supported protocols in wire
  // format.
  bssl::Array<uint8_t> alpn_client_proto_list;

  // SRTP profiles we are willing to do from RFC 5764
  bssl::UniquePtr<STACK_OF(SRTP_PROTECTION_PROFILE)> srtp_profiles;

  // Defined compression algorithms for certificates.
  bssl::GrowableArray<bssl::CertCompressionAlg> cert_compression_algs;

  // Supported group values inherited by SSL structure
  bssl::Array<uint16_t> supported_group_list;

  // channel_id_private is the client's Channel ID private key, or null if
  // Channel ID should not be offered on this connection.
  bssl::UniquePtr<EVP_PKEY> channel_id_private;

  // ech_keys contains the server's list of ECHConfig values and associated
  // private keys. This list may be swapped out at any time, so all access must
  // be synchronized through |lock|.
  bssl::UniquePtr<SSL_ECH_KEYS> ech_keys;

  // keylog_callback, if not NULL, is the key logging callback. See
  // |SSL_CTX_set_keylog_callback|.
  void (*keylog_callback)(const SSL *ssl, const char *line) = nullptr;

  // current_time_cb, if not NULL, is the function to use to get the current
  // time. It sets |*out_clock| to the current time. The |ssl| argument is
  // always NULL. See |SSL_CTX_set_current_time_cb|.
  void (*current_time_cb)(const SSL *ssl, struct timeval *out_clock) = nullptr;

  // pool is used for all |CRYPTO_BUFFER|s in case we wish to share certificate
  // memory.
  CRYPTO_BUFFER_POOL *pool = nullptr;

  // ticket_aead_method contains function pointers for opening and sealing
  // session tickets.
  const SSL_TICKET_AEAD_METHOD *ticket_aead_method = nullptr;

  // legacy_ocsp_callback implements an OCSP-related callback for OpenSSL
  // compatibility.
  int (*legacy_ocsp_callback)(SSL *ssl, void *arg) = nullptr;
  void *legacy_ocsp_callback_arg = nullptr;

  // verify_sigalgs, if not empty, is the set of signature algorithms
  // accepted from the peer in decreasing order of preference.
  bssl::Array<uint16_t> verify_sigalgs;

  // retain_only_sha256_of_client_certs is true if we should compute the SHA256
  // hash of the peer's certificate and then discard it to save memory and
  // session space. Only effective on the server side.
  bool retain_only_sha256_of_client_certs : 1;

  // quiet_shutdown is true if the connection should not send a close_notify on
  // shutdown.
  bool quiet_shutdown : 1;

  // ocsp_stapling_enabled is only used by client connections and indicates
  // whether OCSP stapling will be requested.
  bool ocsp_stapling_enabled : 1;

  // If true, a client will request certificate timestamps.
  bool signed_cert_timestamps_enabled : 1;

  // channel_id_enabled is whether Channel ID is enabled. For a server, means
  // that we'll accept Channel IDs from clients.  For a client, means that we'll
  // advertise support.
  bool channel_id_enabled : 1;

  // grease_enabled is whether GREASE (RFC 8701) is enabled.
  bool grease_enabled : 1;

  // permute_extensions is whether to permute extensions when sending messages.
  bool permute_extensions : 1;

  // allow_unknown_alpn_protos is whether the client allows unsolicited ALPN
  // protocols from the peer.
  bool allow_unknown_alpn_protos : 1;

  // false_start_allowed_without_alpn is whether False Start (if
  // |SSL_MODE_ENABLE_FALSE_START| is enabled) is allowed without ALPN.
  bool false_start_allowed_without_alpn : 1;

  // handoff indicates that a server should stop after receiving the
  // ClientHello and pause the handshake in such a way that |SSL_get_error|
  // returns |SSL_ERROR_HANDOFF|.
  bool handoff : 1;

  // If enable_early_data is true, early data can be sent and accepted.
  bool enable_early_data : 1;

  // enable_read_ahead indicates whether the |SSL_CTX| is configured to read as much
  // as will fit in the SSLBuffer from the BIO, or just enough to read the record
  // header and then the length of the body
  bool enable_read_ahead : 1;

  // aes_hw_override if set indicates we should override checking for AES
  // hardware support, and use the value in aes_hw_override_value instead.
  bool aes_hw_override : 1;

  // aes_hw_override_value is used for testing to indicate the support or lack
  // of support for AES hardware. The value is only considered if
  // |aes_hw_override| is true.
  bool aes_hw_override_value : 1;

  // conf_max_version_use_default indicates whether the |SSL_CTX| is configured
  // to use the default maximum protocol version for the relevant protocol
  // method. By default, |SSL_CTX_new| will set this to true and connections will
  // use the default max version. callers can change the max version used by calling
  // |SSL_CTX_set_max_proto_version| with a non-zero value.
  bool conf_max_version_use_default : 1;

  // conf_min_version_use_default indicates whether the |SSL_CTX| is configured
  // to use the default minimum protocol version for the relevant protocol
  // method. By default, |SSL_CTX_new| will set this to true and connections will
  // use the default min version. callers can change the min version used by calling
  // |SSL_CTX_set_min_proto_version| with a non-zero value.
  bool conf_min_version_use_default : 1;

 private:
  friend RefCounted;
  ~ssl_ctx_st();
};

struct ssl_st {
  explicit ssl_st(SSL_CTX *ctx_arg);
  ssl_st(const ssl_st &) = delete;
  ssl_st &operator=(const ssl_st &) = delete;
  ~ssl_st();

  // method is the method table corresponding to the current protocol (DTLS or
  // TLS).
  const bssl::SSL_PROTOCOL_METHOD *method = nullptr;

  // config is a container for handshake configuration.  Accesses to this field
  // should check for nullptr, since configuration may be shed after the
  // handshake completes.  (If you have the |SSL_HANDSHAKE| object at hand, use
  // that instead, and skip the null check.)
  bssl::UniquePtr<bssl::SSL_CONFIG> config;

  // version is the protocol version.
  uint16_t version = 0;

  uint16_t max_send_fragment = 0;

  size_t read_ahead_buffer_size = SSL3_RT_MAX_PLAIN_LENGTH + SSL3_RT_MAX_ENCRYPTED_OVERHEAD;

  // There are 2 BIO's even though they are normally both the same. This is so
  // data can be read and written to different handlers

  bssl::UniquePtr<BIO> rbio;  // used by SSL_read
  bssl::UniquePtr<BIO> wbio;  // used by SSL_write

  // do_handshake runs the handshake. On completion, it returns |ssl_hs_ok|.
  // Otherwise, it returns a value corresponding to what operation is needed to
  // progress.
  bssl::ssl_hs_wait_t (*do_handshake)(bssl::SSL_HANDSHAKE *hs) = nullptr;

  bssl::SSL3_STATE *s3 = nullptr;   // TLS variables
  bssl::DTLS1_STATE *d1 = nullptr;  // DTLS variables

  // callback that allows applications to peek at protocol messages
  void (*msg_callback)(int write_p, int version, int content_type,
                       const void *buf, size_t len, SSL *ssl,
                       void *arg) = nullptr;
  void *msg_callback_arg = nullptr;

  SSL_security_callback security_callback = nullptr;
  void *security_callback_ex_data = nullptr;

  // session info

  // initial_timeout_duration_ms is the default DTLS timeout duration in
  // milliseconds. It's used to initialize the timer any time it's restarted.
  //
  // RFC 6347 states that implementations SHOULD use an initial timer value of 1
  // second.
  unsigned initial_timeout_duration_ms = 1000;

  // session is the configured session to be offered by the client. This session
  // is immutable.
  bssl::UniquePtr<SSL_SESSION> session;

  // client_cipher_suites contains cipher suites offered by the client during
  // the handshake and that are supported by this library, with preference order
  // maintained. This field is NOT serialized and is only populated if used in
  // a server context.
  bssl::UniquePtr<STACK_OF(SSL_CIPHER)> client_cipher_suites;

  // all_client_cipher_suites contains the raw bytes for cipher suites offered
  // by the client during the handshake (including those not supported by this
  // library), with preference order maintained. This field may contain
  // cipher IDs that are invalid.
  bssl::UniquePtr<char> all_client_cipher_suites;

  // Field length of ciphers in client hello. Maximum allowed size is 2^16
  uint16_t all_client_cipher_suites_len = 0;

  void (*info_callback)(const SSL *ssl, int type, int value) = nullptr;

  bssl::UniquePtr<SSL_CTX> ctx;

  // session_ctx is the |SSL_CTX| used for the session cache and related
  // settings.
  bssl::UniquePtr<SSL_CTX> session_ctx;

  // extra application data
  CRYPTO_EX_DATA ex_data;

  // verify_result is the result of certificate verification in the case of
  // non-fatal certificate errors.
  long verify_result = X509_V_ERR_INVALID_CALL;

  uint32_t options = 0;  // protocol behaviour
  uint32_t mode = 0;     // API behaviour
  uint32_t max_cert_list = 0;
  bssl::UniquePtr<char> hostname;

  // quic_method is the method table corresponding to the QUIC hooks.
  const SSL_QUIC_METHOD *quic_method = nullptr;

  // renegotiate_mode controls how peer renegotiation attempts are handled.
  ssl_renegotiate_mode_t renegotiate_mode = ssl_renegotiate_never;

  // server is true iff the this SSL* is the server half. Note: before the SSL*
  // is initialized by either SSL_set_accept_state or SSL_set_connect_state,
  // the side is not determined. In this state, server is always false.
  bool server : 1;

  // quiet_shutdown is true if the connection should not send a close_notify on
  // shutdown.
  bool quiet_shutdown : 1;

  // If enable_early_data is true, early data can be sent and accepted.
  bool enable_early_data : 1;

  // enable_read_ahead indicates whether the |SSL| is configured to read as much
  // as will fit in the SSLBuffer from the BIO, or just enough to read the record
  // header and then the length of the body
  bool enable_read_ahead : 1;

  // is_suspended_state indicates that the |SSL| object has been serialized and
  // operations should not be performed on the connection.
  bool is_suspended_state : 1;
};

struct ssl_session_st : public bssl::RefCounted<ssl_session_st> {
  explicit ssl_session_st(const bssl::SSL_X509_METHOD *method);
  ssl_session_st(const ssl_session_st &) = delete;
  ssl_session_st &operator=(const ssl_session_st &) = delete;

  // ssl_version is the (D)TLS version that established the session.
  uint16_t ssl_version = 0;

  // group_id is the ID of the ECDH group used to establish this session or zero
  // if not applicable or unknown.
  uint16_t group_id = 0;

  // peer_signature_algorithm is the signature algorithm used to authenticate
  // the peer, or zero if not applicable or unknown.
  uint16_t peer_signature_algorithm = 0;

  // secret, in TLS 1.2 and below, is the master secret associated with the
  // session. In TLS 1.3 and up, it is the resumption PSK for sessions handed to
  // the caller, but it stores the resumption secret when stored on |SSL|
  // objects.
  uint8_t secret_length = 0;
  uint8_t secret[SSL_MAX_MASTER_KEY_LENGTH] = {0};

  // session_id - valid?
  uint8_t session_id_length = 0;
  uint8_t session_id[SSL_MAX_SSL_SESSION_ID_LENGTH] = {0};
  // this is used to determine whether the session is being reused in
  // the appropriate context. It is up to the application to set this,
  // via SSL_new
  uint8_t sid_ctx_length = 0;
  uint8_t sid_ctx[SSL_MAX_SID_CTX_LENGTH] = {0};

  bssl::UniquePtr<char> psk_identity;

  // certs contains the certificate chain from the peer, starting with the leaf
  // certificate.
  bssl::UniquePtr<STACK_OF(CRYPTO_BUFFER)> certs;

  const bssl::SSL_X509_METHOD *x509_method = nullptr;

  // x509_peer is the peer's certificate.
  X509 *x509_peer = nullptr;

  // x509_chain is the certificate chain sent by the peer. NOTE: for historical
  // reasons, when a client (so the peer is a server), the chain includes
  // |peer|, but when a server it does not.
  STACK_OF(X509) *x509_chain = nullptr;

  // x509_chain_without_leaf is a lazily constructed copy of |x509_chain| that
  // omits the leaf certificate. This exists because OpenSSL, historically,
  // didn't include the leaf certificate in the chain for a server, but did for
  // a client. The |x509_chain| always includes it and, if an API call requires
  // a chain without, it is stored here.
  STACK_OF(X509) *x509_chain_without_leaf = nullptr;

  // x509_verified_chain is a lazily constructed copy of the chain that has
  // been verified. This is not necessarily the same as |x509_chain| which
  // may include additional certificates that were not used.
  STACK_OF(X509) *x509_verified_chain = nullptr;


  // verify_result is the result of certificate verification in the case of
  // non-fatal certificate errors.
  long verify_result = X509_V_ERR_INVALID_CALL;

  // timeout is the lifetime of the session in seconds, measured from |time|.
  // This is renewable up to |auth_timeout|.
  uint32_t timeout = SSL_DEFAULT_SESSION_TIMEOUT;

  // auth_timeout is the non-renewable lifetime of the session in seconds,
  // measured from |time|.
  uint32_t auth_timeout = SSL_DEFAULT_SESSION_TIMEOUT;

  // time is the time the session was issued, measured in seconds from the UNIX
  // epoch.
  uint64_t time = 0;

  const SSL_CIPHER *cipher = nullptr;

  CRYPTO_EX_DATA ex_data;  // application specific data

  // These are used to make removal of session-ids more efficient and to
  // implement a maximum cache size.
  SSL_SESSION *prev = nullptr, *next = nullptr;

  bssl::Array<uint8_t> ticket;

  bssl::UniquePtr<CRYPTO_BUFFER> signed_cert_timestamp_list;

  // The OCSP response that came with the session.
  bssl::UniquePtr<CRYPTO_BUFFER> ocsp_response;

  // peer_sha256 contains the SHA-256 hash of the peer's certificate if
  // |peer_sha256_valid| is true.
  uint8_t peer_sha256[SHA256_DIGEST_LENGTH] = {0};

  // original_handshake_hash contains the handshake hash (either SHA-1+MD5 or
  // SHA-2, depending on TLS version) for the original, full handshake that
  // created a session. This is used by Channel IDs during resumption.
  uint8_t original_handshake_hash[EVP_MAX_MD_SIZE] = {0};
  uint8_t original_handshake_hash_len = 0;

  uint32_t ticket_lifetime_hint = 0;  // Session lifetime hint in seconds

  uint32_t ticket_age_add = 0;

  // ticket_max_early_data is the maximum amount of data allowed to be sent as
  // early data. If zero, 0-RTT is disallowed.
  uint32_t ticket_max_early_data = 0;

  // early_alpn is the ALPN protocol from the initial handshake. This is only
  // stored for TLS 1.3 and above in order to enforce ALPN matching for 0-RTT
  // resumptions. For the current connection's ALPN protocol, see
  // |alpn_selected| on |SSL3_STATE|.
  bssl::Array<uint8_t> early_alpn;

  // local_application_settings, if |has_application_settings| is true, is the
  // local ALPS value for this connection.
  bssl::Array<uint8_t> local_application_settings;

  // peer_application_settings, if |has_application_settings| is true, is the
  // peer ALPS value for this connection.
  bssl::Array<uint8_t> peer_application_settings;

  // extended_master_secret is whether the master secret in this session was
  // generated using EMS and thus isn't vulnerable to the Triple Handshake
  // attack.
  bool extended_master_secret : 1;

  // peer_sha256_valid is whether |peer_sha256| is valid.
  bool peer_sha256_valid : 1;  // Non-zero if peer_sha256 is valid

  // not_resumable is used to indicate that session resumption is disallowed.
  bool not_resumable : 1;

  // ticket_age_add_valid is whether |ticket_age_add| is valid.
  bool ticket_age_add_valid : 1;

  // is_server is whether this session was created by a server.
  bool is_server : 1;

  // is_quic indicates whether this session was created using QUIC.
  bool is_quic : 1;

  // has_application_settings indicates whether ALPS was negotiated in this
  // session.
  bool has_application_settings : 1;

  // quic_early_data_context is used to determine whether early data must be
  // rejected when performing a QUIC handshake.
  bssl::Array<uint8_t> quic_early_data_context;

 private:
  friend RefCounted;
  ~ssl_session_st();
};

struct ssl_ech_keys_st : public bssl::RefCounted<ssl_ech_keys_st> {
  ssl_ech_keys_st() : RefCounted(CheckSubClass()) {}

  bssl::GrowableArray<bssl::UniquePtr<bssl::ECHServerConfig>> configs;

 private:
  friend RefCounted;
  ~ssl_ech_keys_st() = default;
};

#endif  // OPENSSL_HEADER_SSL_INTERNAL_H
