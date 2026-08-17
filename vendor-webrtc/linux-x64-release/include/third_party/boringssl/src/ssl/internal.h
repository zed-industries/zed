// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
// Copyright 2005 Nokia. All rights reserved.
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

#ifndef OPENSSL_HEADER_SSL_INTERNAL_H
#define OPENSSL_HEADER_SSL_INTERNAL_H

#include <openssl/base.h>

#include <stdlib.h>

#include <algorithm>
#include <atomic>
#include <bitset>
#include <initializer_list>
#include <limits>
#include <new>
#include <optional>
#include <string_view>
#include <type_traits>
#include <utility>

#include <openssl/aead.h>
#include <openssl/curve25519.h>
#include <openssl/err.h>
#include <openssl/hpke.h>
#include <openssl/mem.h>
#include <openssl/span.h>
#include <openssl/ssl.h>
#include <openssl/stack.h>

#include "../crypto/err/internal.h"
#include "../crypto/internal.h"
#include "../crypto/lhash/internal.h"
#include "../crypto/spake2plus/internal.h"


#if defined(OPENSSL_WINDOWS)
// Windows defines struct timeval in winsock2.h.
#include <winsock2.h>
#else
#include <sys/time.h>
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
struct DeleterImpl<T, std::enable_if_t<T::kAllowUniquePtr>> {
  static void Free(T *t) { Delete(t); }
};
}  // namespace internal

// MakeUnique behaves like |std::make_unique| but returns nullptr on allocation
// error.
template <typename T, typename... Args>
UniquePtr<T> MakeUnique(Args &&...args) {
  return UniquePtr<T>(New<T>(std::forward<Args>(args)...));
}

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

  const T &operator[](size_t i) const {
    BSSL_CHECK(i < size_);
    return data_[i];
  }
  T &operator[](size_t i) {
    BSSL_CHECK(i < size_);
    return data_[i];
  }

  T *begin() { return data_; }
  const T *begin() const { return data_; }
  T *end() { return data_ + size_; }
  const T *end() const { return data_ + size_; }

  void Reset() { Reset(nullptr, 0); }

  // Reset releases the current contents of the array and takes ownership of the
  // raw pointer supplied by the caller.
  void Reset(T *new_data, size_t new_size) {
    std::destroy_n(data_, size_);
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
  // value-constructed copies of |T|. It returns true on success and false on
  // error. If |T| is a primitive type like |uint8_t|, value-construction means
  // it will be zero-initialized.
  [[nodiscard]] bool Init(size_t new_size) {
    if (!InitUninitialized(new_size)) {
      return false;
    }
    std::uninitialized_value_construct_n(data_, size_);
    return true;
  }

  // InitForOverwrite behaves like |Init| but it default-constructs each element
  // instead. This means that, if |T| is a primitive type, the array will be
  // uninitialized and thus must be filled in by the caller.
  [[nodiscard]] bool InitForOverwrite(size_t new_size) {
    if (!InitUninitialized(new_size)) {
      return false;
    }
    std::uninitialized_default_construct_n(data_, size_);
    return true;
  }

  // CopyFrom replaces the array with a newly-allocated copy of |in|. It returns
  // true on success and false on error.
  [[nodiscard]] bool CopyFrom(Span<const T> in) {
    if (!InitUninitialized(in.size())) {
      return false;
    }
    std::uninitialized_copy(in.begin(), in.end(), data_);
    return true;
  }

  // Shrink shrinks the stored size of the array to |new_size|. It crashes if
  // the new size is larger. Note this does not shrink the allocation itself.
  void Shrink(size_t new_size) {
    if (new_size > size_) {
      abort();
    }
    std::destroy_n(data_ + new_size, size_ - new_size);
    size_ = new_size;
  }

 private:
  // InitUninitialized replaces the array with a newly-allocated array of
  // |new_size| elements, but whose constructor has not yet run. On success, the
  // elements must be constructed before returning control to the caller.
  bool InitUninitialized(size_t new_size) {
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
    return true;
  }

  T *data_ = nullptr;
  size_t size_ = 0;
};

// Vector<T> is a resizable array of elements of |T|.
template <typename T>
class Vector {
 public:
  Vector() = default;
  Vector(const Vector &) = delete;
  Vector(Vector &&other) { *this = std::move(other); }
  ~Vector() { clear(); }

  Vector &operator=(const Vector &) = delete;
  Vector &operator=(Vector &&other) {
    clear();
    std::swap(data_, other.data_);
    std::swap(size_, other.size_);
    std::swap(capacity_, other.capacity_);
    return *this;
  }

  const T *data() const { return data_; }
  T *data() { return data_; }
  size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }

  const T &operator[](size_t i) const {
    BSSL_CHECK(i < size_);
    return data_[i];
  }
  T &operator[](size_t i) {
    BSSL_CHECK(i < size_);
    return data_[i];
  }

  T *begin() { return data_; }
  const T *begin() const { return data_; }
  T *end() { return data_ + size_; }
  const T *end() const { return data_ + size_; }

  void clear() {
    std::destroy_n(data_, size_);
    OPENSSL_free(data_);
    data_ = nullptr;
    size_ = 0;
    capacity_ = 0;
  }

  // Push adds |elem| at the end of the internal array, growing if necessary. It
  // returns false when allocation fails.
  [[nodiscard]] bool Push(T elem) {
    if (!MaybeGrow()) {
      return false;
    }
    new (&data_[size_]) T(std::move(elem));
    size_++;
    return true;
  }

  // CopyFrom replaces the contents of the array with a copy of |in|. It returns
  // true on success and false on allocation error.
  [[nodiscard]] bool CopyFrom(Span<const T> in) {
    Array<T> copy;
    if (!copy.CopyFrom(in)) {
      return false;
    }

    clear();
    copy.Release(&data_, &size_);
    capacity_ = size_;
    return true;
  }

 private:
  // If there is no room for one more element, creates a new backing array with
  // double the size of the old one and copies elements over.
  bool MaybeGrow() {
    // No need to grow if we have room for one more T.
    if (size_ < capacity_) {
      return true;
    }
    size_t new_capacity = kDefaultSize;
    if (capacity_ > 0) {
      // Double the array's size if it's safe to do so.
      if (capacity_ > std::numeric_limits<size_t>::max() / 2) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
        return false;
      }
      new_capacity = capacity_ * 2;
    }
    if (new_capacity > std::numeric_limits<size_t>::max() / sizeof(T)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
      return false;
    }
    T *new_data =
        reinterpret_cast<T *>(OPENSSL_malloc(new_capacity * sizeof(T)));
    if (new_data == nullptr) {
      return false;
    }
    size_t new_size = size_;
    std::uninitialized_move(begin(), end(), new_data);
    clear();
    data_ = new_data;
    size_ = new_size;
    capacity_ = new_capacity;
    return true;
  }

  // data_ is a pointer to |capacity_| objects of size |T|, the first |size_| of
  // which are constructed.
  T *data_ = nullptr;
  // |size_| is the number of elements stored in this Vector.
  size_t size_ = 0;
  // |capacity_| is the number of elements allocated in this Vector.
  size_t capacity_ = 0;
  // |kDefaultSize| is the default initial size of the backing array.
  static constexpr size_t kDefaultSize = 16;
};

// A PackedSize is an integer that can store values from 0 to N, represented as
// a minimal-width integer.
template <size_t N>
using PackedSize = std::conditional_t<
    N <= 0xff, uint8_t,
    std::conditional_t<N <= 0xffff, uint16_t,
                       std::conditional_t<N <= 0xffffffff, uint32_t, size_t>>>;

// An InplaceVector is like a Vector, but stores up to N elements inline in the
// object. It is inspired by std::inplace_vector in C++26.
template <typename T, size_t N>
class InplaceVector {
 public:
  InplaceVector() = default;
  InplaceVector(const InplaceVector &other) { *this = other; }
  InplaceVector(InplaceVector &&other) { *this = std::move(other); }
  ~InplaceVector() { clear(); }
  InplaceVector &operator=(const InplaceVector &other) {
    if (this != &other) {
      CopyFrom(other);
    }
    return *this;
  }
  InplaceVector &operator=(InplaceVector &&other) {
    clear();
    std::uninitialized_move(other.begin(), other.end(), data());
    size_ = other.size();
    return *this;
  }

  const T *data() const { return reinterpret_cast<const T *>(storage_); }
  T *data() { return reinterpret_cast<T *>(storage_); }
  size_t size() const { return size_; }
  static constexpr size_t capacity() { return N; }
  bool empty() const { return size_ == 0; }

  const T &operator[](size_t i) const {
    BSSL_CHECK(i < size_);
    return data()[i];
  }
  T &operator[](size_t i) {
    BSSL_CHECK(i < size_);
    return data()[i];
  }

  T *begin() { return data(); }
  const T *begin() const { return data(); }
  T *end() { return data() + size_; }
  const T *end() const { return data() + size_; }

  void clear() { Shrink(0); }

  // Shrink resizes the vector to |new_size|, which must not be larger than the
  // current size. Unlike |Resize|, this can be called when |T| is not
  // default-constructible.
  void Shrink(size_t new_size) {
    BSSL_CHECK(new_size <= size_);
    std::destroy_n(data() + new_size, size_ - new_size);
    size_ = static_cast<PackedSize<N>>(new_size);
  }

  // TryResize resizes the vector to |new_size| and returns true, or returns
  // false if |new_size| is too large. Any newly-added elements are
  // value-initialized.
  [[nodiscard]] bool TryResize(size_t new_size) {
    if (new_size <= size_) {
      Shrink(new_size);
      return true;
    }
    if (new_size > capacity()) {
      return false;
    }
    std::uninitialized_value_construct_n(data() + size_, new_size - size_);
    size_ = static_cast<PackedSize<N>>(new_size);
    return true;
  }

  // TryResizeForOverwrite behaves like |TryResize|, but newly-added elements
  // are default-initialized, so POD types may contain uninitialized values that
  // the caller is responsible for filling in.
  [[nodiscard]] bool TryResizeForOverwrite(size_t new_size) {
    if (new_size <= size_) {
      Shrink(new_size);
      return true;
    }
    if (new_size > capacity()) {
      return false;
    }
    std::uninitialized_default_construct_n(data() + size_, new_size - size_);
    size_ = static_cast<PackedSize<N>>(new_size);
    return true;
  }

  // TryCopyFrom sets the vector to a copy of |in| and returns true, or returns
  // false if |in| is too large.
  [[nodiscard]] bool TryCopyFrom(Span<const T> in) {
    if (in.size() > capacity()) {
      return false;
    }
    clear();
    std::uninitialized_copy(in.begin(), in.end(), data());
    size_ = in.size();
    return true;
  }

  // TryPushBack appends |val| to the vector and returns a pointer to the
  // newly-inserted value, or nullptr if the vector is at capacity.
  [[nodiscard]] T *TryPushBack(T val) {
    if (size() >= capacity()) {
      return nullptr;
    }
    T *ret = &data()[size_];
    new (ret) T(std::move(val));
    size_++;
    return ret;
  }

  // The following methods behave like their |Try*| counterparts, but abort the
  // program on failure.
  void Resize(size_t size) { BSSL_CHECK(TryResize(size)); }
  void ResizeForOverwrite(size_t size) {
    BSSL_CHECK(TryResizeForOverwrite(size));
  }
  void CopyFrom(Span<const T> in) { BSSL_CHECK(TryCopyFrom(in)); }
  T &PushBack(T val) {
    T *ret = TryPushBack(std::move(val));
    BSSL_CHECK(ret != nullptr);
    return *ret;
  }

  template <typename Pred>
  void EraseIf(Pred pred) {
    // See if anything needs to be erased at all. This avoids a self-move.
    auto iter = std::find_if(begin(), end(), pred);
    if (iter == end()) {
      return;
    }

    // Elements before the first to be erased may be left as-is.
    size_t new_size = iter - begin();
    // Swap all subsequent elements in if they are to be kept.
    for (size_t i = new_size + 1; i < size(); i++) {
      if (!pred((*this)[i])) {
        (*this)[new_size] = std::move((*this)[i]);
        new_size++;
      }
    }

    Shrink(new_size);
  }

 private:
  alignas(T) char storage_[sizeof(T[N])];
  PackedSize<N> size_ = 0;
};

// An MRUQueue maintains a queue of up to |N| objects of type |T|. If the queue
// is at capacity, adding to the queue pops the least recently added element.
template <typename T, size_t N>
class MRUQueue {
 public:
  static constexpr bool kAllowUniquePtr = true;

  MRUQueue() = default;

  // If we ever need to make this type movable, we could. (The defaults almost
  // work except we need |start_| to be reset when moved-from.)
  MRUQueue(const MRUQueue &other) = delete;
  MRUQueue &operator=(const MRUQueue &other) = delete;

  bool empty() const { return size() == 0; }
  size_t size() const { return storage_.size(); }

  T &operator[](size_t i) {
    BSSL_CHECK(i < size());
    return storage_[(start_ + i) % N];
  }
  const T &operator[](size_t i) const {
    return (*const_cast<MRUQueue *>(this))[i];
  }

  void Clear() {
    storage_.clear();
    start_ = 0;
  }

  void PushBack(T t) {
    if (storage_.size() < N) {
      assert(start_ == 0);
      storage_.PushBack(std::move(t));
    } else {
      (*this)[0] = std::move(t);
      start_ = (start_ + 1) % N;
    }
  }

 private:
  InplaceVector<T, N> storage_;
  PackedSize<N> start_ = 0;
};

// CBBFinishArray behaves like |CBB_finish| but stores the result in an Array.
OPENSSL_EXPORT bool CBBFinishArray(CBB *cbb, Array<uint8_t> *out);

// GetAllNames helps to implement |*_get_all_*_names| style functions. It
// writes at most |max_out| string pointers to |out| and returns the number that
// it would have liked to have written. The strings written consist of
// |fixed_names_len| strings from |fixed_names| followed by |objects_len|
// strings taken by projecting |objects| through |name|.
template <typename T, typename Name>
inline size_t GetAllNames(const char **out, size_t max_out,
                          Span<const char *const> fixed_names, Name(T::*name),
                          Span<const T> objects) {
  auto span = bssl::Span(out, max_out);
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

// ssl_has_final_version returns whether |ssl| has determined the final version.
// This may be used to distinguish the predictive 0-RTT version from the final
// one.
bool ssl_has_final_version(const SSL *ssl);

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

BSSL_NAMESPACE_BEGIN

// Bits for |algorithm_mkey| (key exchange algorithm).
#define SSL_kRSA 0x00000001u
#define SSL_kECDHE 0x00000002u
// SSL_kPSK is only set for plain PSK, not ECDHE_PSK.
#define SSL_kPSK 0x00000004u
#define SSL_kGENERIC 0x00000008u

// Bits for |algorithm_auth| (server authentication).
#define SSL_aRSA_SIGN 0x00000001u
#define SSL_aRSA_DECRYPT 0x00000002u
#define SSL_aECDSA 0x00000004u
// SSL_aPSK is set for both PSK and ECDHE_PSK.
#define SSL_aPSK 0x00000008u
#define SSL_aGENERIC 0x00000010u

#define SSL_aCERT (SSL_aRSA_SIGN | SSL_aRSA_DECRYPT | SSL_aECDSA)

// Bits for |algorithm_enc| (symmetric encryption).
#define SSL_3DES 0x00000001u
#define SSL_AES128 0x00000002u
#define SSL_AES256 0x00000004u
#define SSL_AES128GCM 0x00000008u
#define SSL_AES256GCM 0x00000010u
#define SSL_CHACHA20POLY1305 0x00000020u

#define SSL_AES (SSL_AES128 | SSL_AES256 | SSL_AES128GCM | SSL_AES256GCM)

// Bits for |algorithm_mac| (symmetric authentication).
#define SSL_SHA1 0x00000001u
#define SSL_SHA256 0x00000002u
// SSL_AEAD is set for all AEADs.
#define SSL_AEAD 0x00000004u

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
// SSL_OP_CIPHER_SERVER_PREFERENCE).
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
                             uint16_t version);

// ssl_get_handshake_digest returns the |EVP_MD| corresponding to |version| and
// |cipher|.
const EVP_MD *ssl_get_handshake_digest(uint16_t version,
                                       const SSL_CIPHER *cipher);

// ssl_create_cipher_list evaluates |rule_str|. It sets |*out_cipher_list| to a
// newly-allocated |SSLCipherPreferenceList| containing the result. It returns
// true on success and false on failure. If |strict| is true, nonsense will be
// rejected. If false, nonsense will be silently ignored. An empty result is
// considered an error regardless of |strict|. |has_aes_hw| indicates if the
// list should be ordered based on having support for AES in hardware or not.
bool ssl_create_cipher_list(UniquePtr<SSLCipherPreferenceList> *out_cipher_list,
                            const bool has_aes_hw, const char *rule_str,
                            bool strict);

// ssl_cipher_auth_mask_for_key returns the mask of cipher |algorithm_auth|
// values suitable for use with |key| in TLS 1.2 and below. |sign_ok| indicates
// whether |key| may be used for signing.
uint32_t ssl_cipher_auth_mask_for_key(const EVP_PKEY *key, bool sign_ok);

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
// available from |cipher_suites| compatible with |version| and |policy|. It
// returns NULL if there isn't a compatible cipher. |has_aes_hw| indicates if
// the choice should be made as if support for AES in hardware is available.
const SSL_CIPHER *ssl_choose_tls13_cipher(CBS cipher_suites, bool has_aes_hw,
                                          uint16_t version,
                                          enum ssl_compliance_policy_t policy);

// ssl_tls13_cipher_meets_policy returns true if |cipher_id| is acceptable given
// |policy|.
bool ssl_tls13_cipher_meets_policy(uint16_t cipher_id,
                                   enum ssl_compliance_policy_t policy);

// ssl_cipher_is_deprecated returns true if |cipher| is deprecated.
OPENSSL_EXPORT bool ssl_cipher_is_deprecated(const SSL_CIPHER *cipher);


// Transcript layer.

// SSLTranscript maintains the handshake transcript as a combination of a
// buffer and running hash.
class SSLTranscript {
 public:
  explicit SSLTranscript(bool is_dtls);
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
    return Span(reinterpret_cast<const uint8_t *>(buffer_->data),
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
  // HashBuffer initializes |ctx| to use |digest| and writes the contents of
  // |buffer_| to |ctx|. If this SSLTranscript is for DTLS 1.3, the appropriate
  // bytes in |buffer_| will be skipped when hashing the buffer.
  bool HashBuffer(EVP_MD_CTX *ctx, const EVP_MD *digest) const;

  // AddToBufferOrHash directly adds the contents of |in| to |buffer_| and/or
  // |hash_|.
  bool AddToBufferOrHash(Span<const uint8_t> in);

  // buffer_, if non-null, contains the handshake transcript.
  UniquePtr<BUF_MEM> buffer_;
  // hash, if initialized with an |EVP_MD|, maintains the handshake hash.
  ScopedEVP_MD_CTX hash_;
  // is_dtls_ indicates whether this is a transcript for a DTLS connection.
  bool is_dtls_ : 1;
  // version_ contains the version for the connection (if known).
  uint16_t version_ = 0;
};

// tls1_prf computes the PRF function for |ssl|. It fills |out|, using |secret|
// as the secret and |label| as the label. |seed1| and |seed2| are concatenated
// to form the seed parameter. It returns true on success and false on failure.
bool tls1_prf(const EVP_MD *digest, Span<uint8_t> out,
              Span<const uint8_t> secret, std::string_view label,
              Span<const uint8_t> seed1, Span<const uint8_t> seed2);


// Encryption layer.

// SSLAEADContext contains information about an AEAD that is being used to
// encrypt an SSL connection.
class SSLAEADContext {
 public:
  explicit SSLAEADContext(const SSL_CIPHER *cipher);
  ~SSLAEADContext();
  static constexpr bool kAllowUniquePtr = true;

  SSLAEADContext(const SSLAEADContext &&) = delete;
  SSLAEADContext &operator=(const SSLAEADContext &&) = delete;

  // CreateNullCipher creates an |SSLAEADContext| for the null cipher.
  static UniquePtr<SSLAEADContext> CreateNullCipher();

  // Create creates an |SSLAEADContext| using the supplied key material. It
  // returns nullptr on error. Only one of |Open| or |Seal| may be used with the
  // resulting object, depending on |direction|. |version| is the wire version.
  static UniquePtr<SSLAEADContext> Create(enum evp_aead_direction_t direction,
                                          uint16_t version,
                                          const SSL_CIPHER *cipher,
                                          Span<const uint8_t> enc_key,
                                          Span<const uint8_t> mac_key,
                                          Span<const uint8_t> fixed_iv);

  // CreatePlaceholderForQUIC creates a placeholder |SSLAEADContext| for the
  // given cipher. The resulting object can be queried for various properties
  // but cannot encrypt or decrypt data.
  static UniquePtr<SSLAEADContext> CreatePlaceholderForQUIC(
      const SSL_CIPHER *cipher);

  const SSL_CIPHER *cipher() const { return cipher_; }

  // is_null_cipher returns true if this is the null cipher.
  bool is_null_cipher() const { return !cipher_; }

  // ExplicitNonceLen returns the length of the explicit nonce.
  size_t ExplicitNonceLen() const;

  // MaxOverhead returns the maximum overhead of calling |Seal|.
  size_t MaxOverhead() const;

  // MaxSealInputLen returns the maximum length for |Seal| that can fit in
  // |max_out| output bytes, or zero if no input may fit.
  size_t MaxSealInputLen(size_t max_out) const;

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
            uint64_t seqnum, Span<const uint8_t> header, Span<uint8_t> in);

  // Seal encrypts and authenticates |in_len| bytes from |in| and writes the
  // result to |out|. It returns true on success and false on error.
  //
  // If |in| and |out| alias then |out| + |ExplicitNonceLen| must be == |in|.
  bool Seal(uint8_t *out, size_t *out_len, size_t max_out, uint8_t type,
            uint16_t record_version, uint64_t seqnum,
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
                   uint8_t type, uint16_t record_version, uint64_t seqnum,
                   Span<const uint8_t> header, const uint8_t *in, size_t in_len,
                   const uint8_t *extra_in, size_t extra_in_len);

  bool GetIV(const uint8_t **out_iv, size_t *out_iv_len) const;

 private:
  // GetAdditionalData returns the additional data, writing into |storage| if
  // necessary.
  Span<const uint8_t> GetAdditionalData(uint8_t storage[13], uint8_t type,
                                        uint16_t record_version,
                                        uint64_t seqnum, size_t plaintext_len,
                                        Span<const uint8_t> header);

  const SSL_CIPHER *cipher_;
  ScopedEVP_AEAD_CTX ctx_;
  // fixed_nonce_ contains any bytes of the nonce that are fixed for all
  // records.
  InplaceVector<uint8_t, 12> fixed_nonce_;
  uint8_t variable_nonce_len_ = 0;
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

// DTLSReplayBitmap maintains a sliding window of sequence numbers to detect
// replayed packets.
class DTLSReplayBitmap {
 public:
  // ShouldDiscard returns true if |seq_num| has been seen in
  // |bitmap| or is stale. Otherwise it returns false.
  bool ShouldDiscard(uint64_t seqnum) const;

  // Record updates the bitmap to record receipt of sequence number
  // |seq_num|. It slides the window forward if needed. It is an error to call
  // this function on a stale sequence number.
  void Record(uint64_t seqnum);

  uint64_t max_seq_num() const { return max_seq_num_; }

 private:
  // map is a bitset of sequence numbers that have been seen. Bit i corresponds
  // to |max_seq_num_ - i|.
  std::bitset<256> map_;
  // max_seq_num_ is the largest sequence number seen so far as a 64-bit
  // integer, or zero if none have been seen.
  uint64_t max_seq_num_ = 0;
};

// reconstruct_seqnum takes the low order bits of a record sequence number from
// the wire and reconstructs the full sequence number. It does so using the
// algorithm described in section 4.2.2 of RFC 9147, where |wire_seq| is the
// low bits of the sequence number as seen on the wire, |seq_mask| is a bitmask
// of 8 or 16 1 bits corresponding to the length of the sequence number on the
// wire, and |max_valid_seqnum| is the largest sequence number of a record
// successfully deprotected in this epoch. This function returns the sequence
// number that is numerically closest to one plus |max_valid_seqnum| that when
// bitwise and-ed with |seq_mask| equals |wire_seq|.
//
// |max_valid_seqnum| must be most 2^48-1, in which case the output will also be
// at most 2^48-1.
OPENSSL_EXPORT uint64_t reconstruct_seqnum(uint16_t wire_seq, uint64_t seq_mask,
                                           uint64_t max_valid_seqnum);


// Record layer.

class DTLSRecordNumber {
 public:
  static constexpr uint64_t kMaxSequence = (uint64_t{1} << 48) - 1;

  DTLSRecordNumber() = default;
  DTLSRecordNumber(uint16_t epoch, uint64_t sequence) {
    BSSL_CHECK(sequence <= kMaxSequence);
    combined_ = (uint64_t{epoch} << 48) | sequence;
  }

  static DTLSRecordNumber FromCombined(uint64_t combined) {
    return DTLSRecordNumber(combined);
  }

  bool operator==(DTLSRecordNumber r) const {
    return combined() == r.combined();
  }
  bool operator!=(DTLSRecordNumber r) const { return !((*this) == r); }
  bool operator<(DTLSRecordNumber r) const { return combined() < r.combined(); }

  uint64_t combined() const { return combined_; }
  uint16_t epoch() const { return combined_ >> 48; }
  uint64_t sequence() const { return combined_ & kMaxSequence; }

  bool HasNext() const { return sequence() < kMaxSequence; }
  DTLSRecordNumber Next() const {
    BSSL_CHECK(HasNext());
    // This will not overflow into the epoch.
    return DTLSRecordNumber::FromCombined(combined_ + 1);
  }

 private:
  explicit DTLSRecordNumber(uint64_t combined) : combined_(combined) {}

  uint64_t combined_ = 0;
};

class RecordNumberEncrypter {
 public:
  static constexpr bool kAllowUniquePtr = true;
  static constexpr size_t kMaxKeySize = 32;

  // Create returns a DTLS 1.3 record number encrypter for |traffic_secret|, or
  // nullptr on error.
  static UniquePtr<RecordNumberEncrypter> Create(
      const SSL_CIPHER *cipher, Span<const uint8_t> traffic_secret);

  virtual ~RecordNumberEncrypter() = default;
  virtual size_t KeySize() = 0;
  virtual bool SetKey(Span<const uint8_t> key) = 0;
  virtual bool GenerateMask(Span<uint8_t> out, Span<const uint8_t> sample) = 0;
};

struct DTLSReadEpoch {
  static constexpr bool kAllowUniquePtr = true;

  // TODO(davidben): This could be made slightly more compact if |bitmap| stored
  // a DTLSRecordNumber.
  uint16_t epoch = 0;
  UniquePtr<SSLAEADContext> aead;
  UniquePtr<RecordNumberEncrypter> rn_encrypter;
  DTLSReplayBitmap bitmap;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> traffic_secret;
};

struct DTLSWriteEpoch {
  static constexpr bool kAllowUniquePtr = true;

  uint16_t epoch() const { return next_record.epoch(); }

  DTLSRecordNumber next_record;
  UniquePtr<SSLAEADContext> aead;
  UniquePtr<RecordNumberEncrypter> rn_encrypter;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> traffic_secret;
};

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
// zero. The caller should read one packet and try again. On success,
// |*out_number| is set to the record number of the record.
enum ssl_open_record_t dtls_open_record(SSL *ssl, uint8_t *out_type,
                                        DTLSRecordNumber *out_number,
                                        Span<uint8_t> *out,
                                        size_t *out_consumed,
                                        uint8_t *out_alert, Span<uint8_t> in);

// ssl_needs_record_splitting returns one if |ssl|'s current outgoing cipher
// state needs record-splitting and zero otherwise.
bool ssl_needs_record_splitting(const SSL *ssl);

// tls_seal_record seals a new record of type |type| and body |in| and writes it
// to |out|. At most |max_out| bytes will be written. It returns true on success
// and false on error. If enabled, |tls_seal_record| implements TLS 1.0 CBC
// 1/n-1 record splitting and may write two records concatenated.
//
// For a large record, the bulk of the ciphertext will begin
// |tls_seal_align_prefix_len| bytes into out. Aligning |out| appropriately may
// improve performance. It writes at most |in_len| + |SSL_max_seal_overhead|
// bytes to |out|.
//
// |in| and |out| may not alias.
bool tls_seal_record(SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
                     uint8_t type, const uint8_t *in, size_t in_len);

// dtls_record_header_write_len returns the length of the record header that
// will be written at |epoch|.
size_t dtls_record_header_write_len(const SSL *ssl, uint16_t epoch);

// dtls_max_seal_overhead returns the maximum overhead, in bytes, of sealing a
// record.
size_t dtls_max_seal_overhead(const SSL *ssl, uint16_t epoch);

// dtls_seal_prefix_len returns the number of bytes of prefix to reserve in
// front of the plaintext when sealing a record in-place.
size_t dtls_seal_prefix_len(const SSL *ssl, uint16_t epoch);

// dtls_seal_max_input_len returns the maximum number of input bytes that can
// fit in a record of up to |max_out| bytes, or zero if none may fit.
size_t dtls_seal_max_input_len(const SSL *ssl, uint16_t epoch, size_t max_out);

// dtls_get_read_epoch and dtls_get_write_epoch return the epoch corresponding
// to |epoch| or nullptr if there is none.
DTLSReadEpoch *dtls_get_read_epoch(const SSL *ssl, uint16_t epoch);
DTLSWriteEpoch *dtls_get_write_epoch(const SSL *ssl, uint16_t epoch);

// dtls_seal_record implements |tls_seal_record| for DTLS. |epoch| selects which
// epoch's cipher state to use. Unlike |tls_seal_record|, |in| and |out| may
// alias but, if they do, |in| must be exactly |dtls_seal_prefix_len| bytes
// ahead of |out|. On success, |*out_number| is set to the record number of the
// record.
bool dtls_seal_record(SSL *ssl, DTLSRecordNumber *out_number, uint8_t *out,
                      size_t *out_len, size_t max_out, uint8_t type,
                      const uint8_t *in, size_t in_len, uint16_t epoch);

// ssl_process_alert processes |in| as an alert and updates |ssl|'s shutdown
// state. It returns one of |ssl_open_record_discard|, |ssl_open_record_error|,
// |ssl_open_record_close_notify|, or |ssl_open_record_fatal_alert| as
// appropriate.
enum ssl_open_record_t ssl_process_alert(SSL *ssl, uint8_t *out_alert,
                                         Span<const uint8_t> in);


// Private key operations.

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

// ssl_pkey_supports_algorithm returns whether |pkey| may be used to sign
// |sigalg|.
bool ssl_pkey_supports_algorithm(const SSL *ssl, EVP_PKEY *pkey,
                                 uint16_t sigalg, bool is_verify);

// ssl_public_key_verify verifies that the |signature| is valid for the public
// key |pkey| and input |in|, using the signature algorithm |sigalg|.
bool ssl_public_key_verify(SSL *ssl, Span<const uint8_t> signature,
                           uint16_t sigalg, EVP_PKEY *pkey,
                           Span<const uint8_t> in);


// Key shares.

// SSLKeyShare abstracts over KEM-like constructions, for use with TLS 1.2 ECDHE
// cipher suites and the TLS 1.3 key_share extension.
//
// TODO(davidben): This class is named SSLKeyShare after the TLS 1.3 key_share
// extension, but it really implements a KEM abstraction. Additionally, we use
// the same type for Encap, which is a one-off, stateless operation, as Generate
// and Decap. Slightly tidier would be for Generate to return a new SSLKEMKey
// (or we introduce EVP_KEM and EVP_KEM_KEY), with a Decap method, and for Encap
// to be static function.
class SSLKeyShare {
 public:
  virtual ~SSLKeyShare() {}
  static constexpr bool kAllowUniquePtr = true;

  // Create returns a SSLKeyShare instance for use with group |group_id| or
  // nullptr on error.
  static UniquePtr<SSLKeyShare> Create(uint16_t group_id);

  // GroupID returns the group ID.
  virtual uint16_t GroupID() const = 0;

  // Generate generates a keypair and writes the public key to |out_public_key|.
  // It returns true on success and false on error.
  virtual bool Generate(CBB *out_public_key) = 0;

  // Encap generates an ephemeral, symmetric secret and encapsulates it with
  // |peer_key|. On success, it returns true, writes the encapsulated secret to
  // |out_ciphertext|, and sets |*out_secret| to the shared secret. On failure,
  // it returns false and sets |*out_alert| to an alert to send to the peer.
  virtual bool Encap(CBB *out_ciphertext, Array<uint8_t> *out_secret,
                     uint8_t *out_alert, Span<const uint8_t> peer_key) = 0;

  // Decap decapsulates the symmetric secret in |ciphertext|. On success, it
  // returns true and sets |*out_secret| to the shared secret. On failure, it
  // returns false and sets |*out_alert| to an alert to send to the peer.
  virtual bool Decap(Array<uint8_t> *out_secret, uint8_t *out_alert,
                     Span<const uint8_t> ciphertext) = 0;

  // SerializePrivateKey writes the private key to |out|, returning true if
  // successful and false otherwise. It should be called after |Generate|.
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

// ssl_nid_to_group_id looks up the group corresponding to |nid|. On success, it
// sets |*out_group_id| to the group ID and returns true. Otherwise, it returns
// false.
bool ssl_nid_to_group_id(uint16_t *out_group_id, int nid);

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

// dtls_clear_outgoing_messages releases all buffered outgoing messages.
void dtls_clear_outgoing_messages(SSL *ssl);

// dtls_clear_unused_write_epochs releases any write epochs that are no longer
// needed.
void dtls_clear_unused_write_epochs(SSL *ssl);


// Callbacks.

// ssl_do_info_callback calls |ssl|'s info callback, if set.
void ssl_do_info_callback(const SSL *ssl, int type, int value);

// ssl_do_msg_callback calls |ssl|'s message callback, if set.
void ssl_do_msg_callback(const SSL *ssl, int is_write, int content_type,
                         Span<const uint8_t> in);


// Transport buffers.

class SSLBuffer {
 public:
  SSLBuffer() {}
  ~SSLBuffer() { Clear(); }

  SSLBuffer(const SSLBuffer &) = delete;
  SSLBuffer &operator=(const SSLBuffer &) = delete;

  uint8_t *data() { return buf_ + offset_; }
  size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }
  size_t cap() const { return cap_; }

  Span<uint8_t> span() { return Span(data(), size()); }

  Span<uint8_t> remaining() { return Span(data() + size(), cap() - size()); }

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

 private:
  // buf_ is the memory allocated for this buffer.
  uint8_t *buf_ = nullptr;
  // offset_ is the offset into |buf_| which the buffer contents start at.
  uint16_t offset_ = 0;
  // size_ is the size of the buffer contents from |buf_| + |offset_|.
  uint16_t size_ = 0;
  // cap_ is how much memory beyond |buf_| + |offset_| is available.
  uint16_t cap_ = 0;
  // inline_buf_ is a static buffer for short reads.
  uint8_t inline_buf_[SSL3_RT_HEADER_LENGTH];
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

enum ssl_key_usage_t {
  key_usage_digital_signature = 0,
  key_usage_encipherment = 2,
};

// ssl_cert_check_key_usage parses the DER-encoded, X.509 certificate in |in|
// and returns true if doesn't specify a key usage or, if it does, if it
// includes |bit|. Otherwise it pushes to the error queue and returns false.
OPENSSL_EXPORT bool ssl_cert_check_key_usage(const CBS *in,
                                             enum ssl_key_usage_t bit);

// ssl_cert_extract_issuer parses the DER-encoded, X.509 certificate in |in|
// and extracts the issuer. On success it returns true and the DER encoded
// issuer is in |out_dn|, otherwise it returns false.
OPENSSL_EXPORT bool ssl_cert_extract_issuer(const CBS *in, CBS *out_dn);

// ssl_cert_matches_issuer parses the DER-encoded, X.509 certificate in |in|
// and returns true if its issuer is an exact match for the DER encoded
// distinguished name in |dn|
bool ssl_cert_matches_issuer(const CBS *in, const CBS *dn);

// ssl_cert_parse_pubkey extracts the public key from the DER-encoded, X.509
// certificate in |in|. It returns an allocated |EVP_PKEY| or else returns
// nullptr and pushes to the error queue.
UniquePtr<EVP_PKEY> ssl_cert_parse_pubkey(const CBS *in);

// SSL_parse_CA_list parses a CA list from |cbs| in the format used by a TLS
// CertificateRequest message and Certificate Authorities extension. On success,
// it returns a newly-allocated |CRYPTO_BUFFER| list and advances
// |cbs|. Otherwise, it returns nullptr and sets |*out_alert| to an alert to
// send to the peer.
UniquePtr<STACK_OF(CRYPTO_BUFFER)> SSL_parse_CA_list(SSL *ssl,
                                                     uint8_t *out_alert,
                                                     CBS *cbs);

// ssl_has_client_CAs returns whether there are configured CAs.
bool ssl_has_client_CAs(const SSL_CONFIG *cfg);

// ssl_add_client_CA_list adds the configured CA list to |cbb| in the format
// used by a TLS CertificateRequest message. It returns true on success and
// false on error.
bool ssl_add_client_CA_list(const SSL_HANDSHAKE *hs, CBB *cbb);

// ssl_has_CA_names returns whether there are configured CA names.
bool ssl_has_CA_names(const SSL_CONFIG *cfg);

// ssl_add_CA_names adds the configured CA_names list to |cbb| in the format
// used by a TLS Certificate Authorities extension. It returns true on success
// and false on error.
bool ssl_add_CA_names(const SSL_HANDSHAKE *hs, CBB *cbb);

// ssl_check_leaf_certificate returns one if |pkey| and |leaf| are suitable as
// a server's leaf certificate for |hs|. Otherwise, it returns zero and pushes
// an error on the error queue.
bool ssl_check_leaf_certificate(SSL_HANDSHAKE *hs, EVP_PKEY *pkey,
                                const CRYPTO_BUFFER *leaf);


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
bool tls13_export_keying_material(const SSL *ssl, Span<uint8_t> out,
                                  Span<const uint8_t> secret,
                                  std::string_view label,
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
bool tls13_derive_session_psk(SSL_SESSION *session, Span<const uint8_t> nonce,
                              bool is_dtls);

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


// Credentials.

enum class SSLCredentialType {
  kX509,
  kDelegated,
  kSPAKE2PlusV1Client,
  kSPAKE2PlusV1Server,
};

BSSL_NAMESPACE_END

// SSL_CREDENTIAL is exported to C, so it must be defined outside the namespace.
struct ssl_credential_st : public bssl::RefCounted<ssl_credential_st> {
  explicit ssl_credential_st(bssl::SSLCredentialType type);
  ssl_credential_st(const ssl_credential_st &) = delete;
  ssl_credential_st &operator=(const ssl_credential_st &) = delete;

  // Dup returns a copy of the credential, or nullptr on error. The |ex_data|
  // values are not copied. This is only used on the legacy credential, whose
  // |ex_data| is inaccessible.
  bssl::UniquePtr<SSL_CREDENTIAL> Dup() const;

  // ClearCertAndKey erases any certificate and private key on the credential.
  void ClearCertAndKey();

  // UsesX509 returns true if the credential type uses an X.509 certificate.
  bool UsesX509() const;

  // UsesPrivateKey returns true if the credential type uses an asymmetric
  // private key.
  bool UsesPrivateKey() const;

  // IsComplete returns whether all required fields in the credential have been
  // filled in.
  bool IsComplete() const;

  // SetLeafCert sets the leaf certificate to |leaf|, leaving the remaining
  // certificates unmodified. It returns true on success and false on error. If
  // |discard_key_on_mismatch| is true and the private key is inconsistent with
  // the new leaf certificate, it is silently discarded.
  bool SetLeafCert(bssl::UniquePtr<CRYPTO_BUFFER> leaf,
                   bool discard_key_on_mismatch);

  // ClearIntermediateCerts clears intermediate certificates in the certificate
  // chain, while preserving the leaf.
  void ClearIntermediateCerts();

  // AppendIntermediateCert appends |cert| to the certificate chain. If there is
  // no leaf certificate configured, it leaves a placeholder null in |chain|. It
  // returns one on success and zero on error.
  bool AppendIntermediateCert(bssl::UniquePtr<CRYPTO_BUFFER> cert);

  // ChainContainsIssuer returns true if |dn| is a byte for byte match with the
  // issuer of any certificate in |chain|, false otherwise.
  bool ChainContainsIssuer(bssl::Span<const uint8_t> dn) const;

  // type is the credential type and determines which other fields apply.
  bssl::SSLCredentialType type;

  // pubkey is the cached public key of the credential. Unlike |privkey|, it is
  // always present and is extracted from the certificate, delegated credential,
  // etc.
  bssl::UniquePtr<EVP_PKEY> pubkey;

  // privkey is the private key of the credential. It may be omitted in favor of
  // |key_method|.
  bssl::UniquePtr<EVP_PKEY> privkey;

  // key_method, if non-null, is a set of callbacks to call for private key
  // operations.
  const SSL_PRIVATE_KEY_METHOD *key_method = nullptr;

  // sigalgs, if non-empty, is the set of signature algorithms supported by the
  // private key in decreasing order of preference. If empty, the default list
  // is used.
  //
  // In delegated credentials, this field is not configurable and is instead
  // computed from the dc_cert_verify_algorithm field.
  bssl::Array<uint16_t> sigalgs;

  // chain contains the certificate chain, with the leaf at the beginning. The
  // first element of |chain| may be nullptr to indicate that the leaf
  // certificate has not yet been set.
  //   If |chain| != nullptr -> len(chain) >= 1
  //   If |chain[0]| == nullptr -> len(chain) >= 2.
  //   |chain[1..]| != nullptr
  bssl::UniquePtr<STACK_OF(CRYPTO_BUFFER)> chain;

  // dc is the DelegatedCredential structure, if this is a delegated credential.
  bssl::UniquePtr<CRYPTO_BUFFER> dc;

  // dc_algorithm is the signature scheme of the signature over the delegated
  // credential itself, made by the end-entity certificate's public key.
  uint16_t dc_algorithm = 0;

  // Signed certificate timestamp list to be sent to the client, if requested
  bssl::UniquePtr<CRYPTO_BUFFER> signed_cert_timestamp_list;

  // OCSP response to be sent to the client, if requested.
  bssl::UniquePtr<CRYPTO_BUFFER> ocsp_response;

  // SPAKE2+-specific information.
  bssl::Array<uint8_t> pake_context;
  bssl::Array<uint8_t> client_identity;
  bssl::Array<uint8_t> server_identity;
  bssl::Array<uint8_t> password_verifier_w0;
  bssl::Array<uint8_t> password_verifier_w1;  // server-only
  bssl::Array<uint8_t> registration_record;   // client-only
  mutable std::atomic<uint32_t> pake_limit;

  // Checks whether there are still permitted PAKE attempts remaining, without
  // changing the counter.
  bool HasPAKEAttempts() const;

  // Atomically decrement |pake_limit|. Return true if successful and false if
  // |pake_limit| is already zero.
  bool ClaimPAKEAttempt() const;

  // Atomically increment |pake_limit|. This must be paired with a
  // |ClaimPAKEAttempt| call.
  void RestorePAKEAttempt() const;

  // trust_anchor_id, if non-empty, is the trust anchor ID for the root of the
  // chain in |chain|.
  bssl::Array<uint8_t> trust_anchor_id;

  CRYPTO_EX_DATA ex_data;

  // must_match_issuer is a flag indicating that this credential should be
  // considered only when it matches a peer request for a particular issuer via
  // a negotiation mechanism (such as the certificate_authorities extension).
  // This also implies that chain is a certificate path ending in a certificate
  // issued by the certificate with that trust anchor identifier.
  bool must_match_issuer = false;

 private:
  friend RefCounted;
  ~ssl_credential_st();
};

BSSL_NAMESPACE_BEGIN

// ssl_get_full_credential_list computes |hs|'s full credential list, including
// the legacy credential. On success, it writes it to |*out| and returns true.
// Otherwise, it returns false. The credential list may be empty, in which case
// this function will successfully output an empty array.
//
// This function should be called at most once during the handshake and is
// intended to be used for certificate-based credentials. It runs the
// auto-chaining logic as part of finishing the legacy credential. Other uses of
// the credential list (e.g. PAKE credentials) should iterate over
// |hs->config->cert->credentials|.
//
// The pointers in the result are only valid until |hs| is next mutated.
bool ssl_get_full_credential_list(SSL_HANDSHAKE *hs,
                                  Array<SSL_CREDENTIAL *> *out);

// ssl_credential_matches_requested_issuers returns true if |cred| is a
// usable match for any requested issuers in |hs|, and false with an error
// otherwise.
bool ssl_credential_matches_requested_issuers(SSL_HANDSHAKE *hs,
                                              const SSL_CREDENTIAL *cred);

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
  state12_cert_callback,
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
  Array<uint8_t> key_share_ciphertext;
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

struct SSLPAKEShare {
  static constexpr bool kAllowUniquePtr = true;
  uint16_t named_pake;
  Array<uint8_t> client_identity;
  Array<uint8_t> server_identity;
  Array<uint8_t> pake_message;
};

struct SSL_HANDSHAKE {
  explicit SSL_HANDSHAKE(SSL *ssl);
  ~SSL_HANDSHAKE();
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

  // max_version is the maximum accepted protocol version, taking account both
  // |SSL_OP_NO_*| and |SSL_CTX_set_max_proto_version| APIs.
  uint16_t max_version = 0;

  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> early_traffic_secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> client_handshake_secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> server_handshake_secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> client_traffic_secret_0;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> server_traffic_secret_0;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> expected_client_finished;

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
  // cookie. Check the received_hello_verify_request field to distinguish an
  // empty cookie from no HelloVerifyRequest message being received.
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

  // key_share_ciphertext, for servers, is encapsulated shared secret to be sent
  // to the client in the TLS 1.3 key_share extension.
  Array<uint8_t> key_share_ciphertext;

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

  // peer_key is the peer's ECDH key for a TLS 1.2 client.
  Array<uint8_t> peer_key;

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

  // ca_names contains the list of CAs received via the Certificate Authorities
  // extension in our peer's CertificateRequest or ClientHello message
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> ca_names;

  // peer_requested_trust_anchors, if not nullopt, contains the trust anchor IDs
  // (possibly none) the peer requested in ClientHello or CertificateRequest. If
  // nullopt, the peer did not send the extension.
  std::optional<Array<uint8_t>> peer_requested_trust_anchors;

  // peer_available_trust_anchors, if not empty, is the list of trust anchor IDs
  // the peer reported as available in EncryptedExtensions. This is only sent by
  // servers to clients.
  Array<uint8_t> peer_available_trust_anchors;

  // cached_x509_ca_names contains a cache of parsed versions of the elements of
  // |ca_names|. This pointer is left non-owning so only
  // |ssl_crypto_x509_method| needs to link against crypto/x509.
  STACK_OF(X509_NAME) *cached_x509_ca_names = nullptr;

  // certificate_types, on the client, contains the set of certificate types
  // received in a CertificateRequest message.
  Array<uint8_t> certificate_types;

  // credential is the credential we are using for the handshake.
  UniquePtr<SSL_CREDENTIAL> credential;

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

  // is_early_version is true if the protocol version configured is not
  // necessarily the final version and is just the predicted 0-RTT version.
  bool is_early_version : 1;

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

  // received_hello_verify_request is true if we received a HelloVerifyRequest
  // message from the server.
  bool received_hello_verify_request : 1;

  // matched_peer_trust_anchor indicates that we have matched a trust anchor
  // the peer requested in the trust anchors extension.
  bool matched_peer_trust_anchor : 1;

  // peer_matched_trust_anchor is true if the peer indicated a match with one of
  // our requested trust anchors.
  bool peer_matched_trust_anchor : 1;

  // client_version is the value sent or received in the ClientHello version.
  uint16_t client_version = 0;

  // early_data_read is the amount of early data that has been read by the
  // record layer.
  uint16_t early_data_read = 0;

  // early_data_written is the amount of early data that has been written by the
  // record layer.
  uint16_t early_data_written = 0;

  // signature_algorithm is the signature algorithm to be used in signing with
  // the selected credential, or zero if not applicable or not yet selected.
  uint16_t signature_algorithm = 0;

  // ech_config_id is the ECH config sent by the client.
  uint8_t ech_config_id = 0;

  // session_id is the session ID in the ClientHello.
  InplaceVector<uint8_t, SSL_MAX_SSL_SESSION_ID_LENGTH> session_id;

  // grease_seed is the entropy for GREASE values.
  uint8_t grease_seed[ssl_grease_last_index + 1] = {0};

  // pake_share is the PAKE message received over the wire, if any.
  UniquePtr<SSLPAKEShare> pake_share;

  // pake_share_bytes are the bytes of the PAKEShare to send, if any.
  Array<uint8_t> pake_share_bytes;

  // pake_prover is the PAKE context for a client.
  UniquePtr<spake2plus::Prover> pake_prover;

  // pake_verifier is the PAKE context for a server.
  UniquePtr<spake2plus::Verifier> pake_verifier;
};

// kMaxTickets is the maximum number of tickets to send immediately after the
// handshake. We use a one-byte ticket nonce, and there is no point in sending
// so many tickets.
constexpr size_t kMaxTickets = 16;

UniquePtr<SSL_HANDSHAKE> ssl_handshake_new(SSL *ssl);

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

// tls13_add_key_update queues a KeyUpdate message on |ssl|. |request_type| must
// be one of |SSL_KEY_UPDATE_REQUESTED| or |SSL_KEY_UPDATE_NOT_REQUESTED|.
bool tls13_add_key_update(SSL *ssl, int request_type);

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

// ssl_setup_pake_shares computes the client PAKE shares and saves them in |hs|.
// It returns true on success and false on failure.
bool ssl_setup_pake_shares(SSL_HANDSHAKE *hs);

bool ssl_ext_key_share_parse_serverhello(SSL_HANDSHAKE *hs,
                                         Array<uint8_t> *out_secret,
                                         uint8_t *out_alert, CBS *contents);
bool ssl_ext_key_share_parse_clienthello(SSL_HANDSHAKE *hs, bool *out_found,
                                         Span<const uint8_t> *out_peer_key,
                                         uint8_t *out_alert,
                                         const SSL_CLIENT_HELLO *client_hello);
bool ssl_ext_pake_add_serverhello(SSL_HANDSHAKE *hs, CBB *out);
bool ssl_ext_key_share_add_serverhello(SSL_HANDSHAKE *hs, CBB *out);

bool ssl_ext_pake_parse_serverhello(SSL_HANDSHAKE *hs,
                                    Array<uint8_t> *out_secret,
                                    uint8_t *out_alert, CBS *contents);

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

// ssl_is_valid_trust_anchor_list returns whether |in| is a valid trust anchor
// identifiers list.
bool ssl_is_valid_trust_anchor_list(Span<const uint8_t> in);

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

// ssl_send_finished adds a Finished message to the current flight of messages.
// It returns true on success and false on error.
bool ssl_send_finished(SSL_HANDSHAKE *hs);

// ssl_send_tls12_certificate adds a TLS 1.2 Certificate message to the current
// flight of messages. It returns true on success and false on error.
bool ssl_send_tls12_certificate(SSL_HANDSHAKE *hs);

// ssl_handshake_session returns the |SSL_SESSION| corresponding to the current
// handshake. Note, in TLS 1.2 resumptions, this session is immutable.
const SSL_SESSION *ssl_handshake_session(const SSL_HANDSHAKE *hs);

// ssl_done_writing_client_hello is called after the last ClientHello is written
// by |hs|. It releases some memory that is no longer needed.
void ssl_done_writing_client_hello(SSL_HANDSHAKE *hs);


// Flags.

// SSLFlags is a bitmask of flags that can be encoded with the TLS flags
// extension, draft-ietf-tls-tlsflags-14. For now, our in-memory representation
// matches the wire representation, and we only support flags up to 32. If
// higher values are needed, we can increase the size of the bitmask, or only
// store the flags we implement in the bitmask.
using SSLFlags = uint32_t;
inline constexpr SSLFlags kSSLFlagResumptionAcrossNames = 1 << 8;

// ssl_add_flags_extension encodes a tls_flags extension (including the header)
// containing the flags in |flags|. It returns true on success and false on
// error. If |flags| is zero (no flags set), it returns true without adding
// anything to |cbb|.
bool ssl_add_flags_extension(CBB *cbb, SSLFlags flags);

// ssl_parse_flags_extension_request parses tls_flags extension value (excluding
// the header) from |cbs|, for a request message (ClientHello,
// CertificateRequest, or NewSessionTicket). Unrecognized flags will be ignored.
//
// On success, it sets |*out| to the parsed flags and returns true. On error, it
// sets |*out_alert| to a TLS alert and returns false.
bool ssl_parse_flags_extension_request(const CBS *cbs, SSLFlags *out,
                                       uint8_t *out_alert);

// ssl_parse_flags_extension_response parses tls_flags extension value
// (excluding the header) from |cbs|, for a response message (HelloRetryRequest,
// ServerHello, EncryptedExtensions, or Certificate). Only the flags in
// |allowed_flags| may be present.
//
// On success, it sets |*out| to the parsed flags and returns true. On error, it
// sets |*out_alert| to a TLS alert and returns false.
bool ssl_parse_flags_extension_response(const CBS *cbs, SSLFlags *out,
                                        uint8_t *out_alert,
                                        SSLFlags allowed_flags);


// SSLKEYLOGFILE functions.

// ssl_log_secret logs |secret| with label |label|, if logging is enabled for
// |ssl|. It returns true on success and false on failure.
bool ssl_log_secret(const SSL *ssl, const char *label,
                    Span<const uint8_t> secret);


// ClientHello functions.

bool ssl_parse_client_hello_with_trailing_data(const SSL *ssl, CBS *cbs,
                                               SSL_CLIENT_HELLO *out);

bool ssl_client_hello_get_extension(const SSL_CLIENT_HELLO *client_hello,
                                    CBS *out, uint16_t extension_type);

bool ssl_client_cipher_list_contains_cipher(
    const SSL_CLIENT_HELLO *client_hello, uint16_t id);


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
// with |cred| based on the peer's preferences and the algorithms supported. It
// returns true on success and false on error.
bool tls1_choose_signature_algorithm(SSL_HANDSHAKE *hs,
                                     const SSL_CREDENTIAL *cred, uint16_t *out);

// tls12_add_verify_sigalgs adds the signature algorithms acceptable for the
// peer signature to |out|. It returns true on success and false on error.
bool tls12_add_verify_sigalgs(const SSL_HANDSHAKE *hs, CBB *out);

// tls12_check_peer_sigalg checks if |sigalg| is acceptable for the peer
// signature from |pkey|. It returns true on success and false on error, setting
// |*out_alert| to an alert to send.
bool tls12_check_peer_sigalg(const SSL_HANDSHAKE *hs, uint8_t *out_alert,
                             uint16_t sigalg, EVP_PKEY *pkey);


// Underdocumented functions.
//
// Functions below here haven't been touched up and may be underdocumented.

#define TLSEXT_CHANNEL_ID_SIZE 128

// From RFC 4492, used in encoding the curve type in ECParameters
#define NAMED_CURVE_TYPE 3

struct CERT {
  static constexpr bool kAllowUniquePtr = true;

  explicit CERT(const SSL_X509_METHOD *x509_method);
  ~CERT();

  bool is_valid() const { return legacy_credential != nullptr; }

  // credentials is the list of credentials to select between. Elements of this
  // array immutable.
  Vector<UniquePtr<SSL_CREDENTIAL>> credentials;

  // legacy_credential is the credential configured by the legacy
  // non-credential-based APIs. If IsComplete() returns true, it is appended to
  // the list of credentials.
  UniquePtr<SSL_CREDENTIAL> legacy_credential;

  // x509_method contains pointers to functions that might deal with |X509|
  // compatibility, or might be a no-op, depending on the application.
  const SSL_X509_METHOD *x509_method = nullptr;

  // x509_chain may contain a parsed copy of |chain[1..]| from the legacy
  // credential. This is only used as a cache in order to implement “get0”
  // functions that return a non-owning pointer to the certificate chain.
  STACK_OF(X509) *x509_chain = nullptr;

  // x509_leaf may contain a parsed copy of the first element of |chain| from
  // the legacy credential. This is only used as a cache in order to implement
  // “get0” functions that return a non-owning pointer to the certificate chain.
  X509 *x509_leaf = nullptr;

  // x509_stash contains the last |X509| object append to the legacy
  // credential's chain. This is a workaround for some third-party code that
  // continue to use an |X509| object even after passing ownership with an
  // “add0” function.
  X509 *x509_stash = nullptr;

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

  // sid_ctx partitions the session space within a shared session cache or
  // ticket key. Only sessions with a matching value will be accepted.
  InplaceVector<uint8_t, SSL_MAX_SID_CTX_LENGTH> sid_ctx;
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
  // finish_flight marks the pending flight as finished and ready to send.
  // |flush| must be called to write it.
  void (*finish_flight)(SSL *ssl);
  // schedule_ack schedules a DTLS 1.3 ACK to be sent, without an ACK delay.
  // |flush| must be called to write it.
  void (*schedule_ack)(SSL *ssl);
  // flush writes any scheduled data to the transport. It returns one on success
  // and <= 0 on error.
  int (*flush)(SSL *ssl);
  // on_handshake_complete is called when the handshake is complete.
  void (*on_handshake_complete)(SSL *ssl);
  // set_read_state sets |ssl|'s read cipher state and level to |aead_ctx| and
  // |level|. In QUIC, |aead_ctx| is a placeholder object. In TLS 1.3,
  // |traffic_secret| is the original traffic secret. This function returns true
  // on success and false on error.
  //
  // TODO(crbug.com/371998381): Take the traffic secrets as input and let the
  // function create the SSLAEADContext.
  bool (*set_read_state)(SSL *ssl, ssl_encryption_level_t level,
                         UniquePtr<SSLAEADContext> aead_ctx,
                         Span<const uint8_t> traffic_secret);
  // set_write_state sets |ssl|'s write cipher state and level to |aead_ctx| and
  // |level|. In QUIC, |aead_ctx| is a placeholder object In TLS 1.3,
  // |traffic_secret| is the original traffic secret. This function returns true
  // on success and false on error.
  //
  // TODO(crbug.com/371998381): Take the traffic secrets as input and let the
  // function create the SSLAEADContext.
  bool (*set_write_state)(SSL *ssl, ssl_encryption_level_t level,
                          UniquePtr<SSLAEADContext> aead_ctx,
                          Span<const uint8_t> traffic_secret);
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
  // check_CA_list returns one if |names| is a good list of X.509 distinguished
  // names and zero otherwise. This is used to ensure that we can reject
  // unparsable values at handshake time when using crypto/x509.
  bool (*check_CA_list)(STACK_OF(CRYPTO_BUFFER) *names);

  // cert_clear frees and NULLs all X509 certificate-related state.
  void (*cert_clear)(CERT *cert);
  // cert_free frees all X509-related state.
  void (*cert_free)(CERT *cert);
  // cert_flush_cached_chain drops any cached |X509|-based certificate chain
  // from |cert|.
  // cert_dup duplicates any needed fields from |cert| to |new_cert|.
  void (*cert_dup)(CERT *new_cert, const CERT *cert);
  void (*cert_flush_cached_chain)(CERT *cert);
  // cert_flush_cached_chain drops any cached |X509|-based leaf certificate
  // from |cert|.
  void (*cert_flush_cached_leaf)(CERT *cert);

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
  // sets |session->verify_result| and returns true on success or false on
  // error.
  bool (*session_verify_cert_chain)(SSL_SESSION *session, SSL_HANDSHAKE *ssl,
                                    uint8_t *out_alert);

  // hs_flush_cached_ca_names drops any cached |X509_NAME|s from |hs|.
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

struct SSL3_STATE {
  static constexpr bool kAllowUniquePtr = true;

  SSL3_STATE();
  ~SSL3_STATE();

  uint64_t read_sequence = 0;
  uint64_t write_sequence = 0;

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

  enum ssl_encryption_level_t quic_read_level = ssl_encryption_initial;
  enum ssl_encryption_level_t quic_write_level = ssl_encryption_initial;

  // version is the protocol version, or zero if the version has not yet been
  // set. In clients offering 0-RTT, this version will initially be set to the
  // early version, then switched to the final version. To distinguish these
  // cases, use |ssl_has_final_version|.
  uint16_t version = 0;

  // early_data_skipped is the amount of early data that has been skipped by the
  // record layer.
  uint16_t early_data_skipped = 0;

  // empty_record_count is the number of consecutive empty records received.
  uint8_t empty_record_count = 0;

  // warning_alert_count is the number of consecutive warning alerts
  // received.
  uint8_t warning_alert_count = 0;

  // key_update_count is the number of consecutive KeyUpdates received.
  uint8_t key_update_count = 0;

  // ech_status indicates whether ECH was accepted by the server.
  ssl_ech_status_t ech_status = ssl_ech_none;

  // skip_early_data instructs the record layer to skip unexpected early data
  // messages when 0RTT is rejected.
  bool skip_early_data : 1;

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

  bool send_connection_binding : 1;

  // channel_id_valid is true if, on the server, the client has negotiated a
  // Channel ID and the |channel_id| field is filled in.
  bool channel_id_valid : 1;

  // key_update_pending is true if we are in the process of sending a KeyUpdate
  // message. As a DoS mitigation (and a requirement in DTLS), we never send
  // more than one KeyUpdate at once. In DTLS, this tracks whether there is an
  // unACKed KeyUpdate.
  bool key_update_pending : 1;

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

  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> write_traffic_secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> read_traffic_secret;
  InplaceVector<uint8_t, SSL_MAX_MD_SIZE> exporter_secret;

  // Connection binding to prevent renegotiation attacks
  InplaceVector<uint8_t, 12> previous_client_finished;
  InplaceVector<uint8_t, 12> previous_server_finished;

  uint8_t send_alert[2] = {0};

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
  uint8_t channel_id[64] = {0};

  // Contains the QUIC transport params received by the peer.
  Array<uint8_t> peer_quic_transport_params;

  // srtp_profile is the selected SRTP protection profile for
  // DTLS-SRTP.
  const SRTP_PROTECTION_PROFILE *srtp_profile = nullptr;
};

// lengths of messages
#define DTLS1_RT_MAX_HEADER_LENGTH 13

// DTLS_PLAINTEXT_RECORD_HEADER_LENGTH is the length of the DTLS record header
// for plaintext records (in DTLS 1.3) or DTLS versions <= 1.2.
#define DTLS_PLAINTEXT_RECORD_HEADER_LENGTH 13

// DTLS1_3_RECORD_HEADER_LENGTH is the length of the DTLS 1.3 record header
// sent by BoringSSL for encrypted records. Note that received encrypted DTLS
// 1.3 records might have a different length header.
#define DTLS1_3_RECORD_HEADER_WRITE_LENGTH 5

static_assert(DTLS1_RT_MAX_HEADER_LENGTH >= DTLS_PLAINTEXT_RECORD_HEADER_LENGTH,
              "DTLS1_RT_MAX_HEADER_LENGTH must not be smaller than defined "
              "record header lengths");
static_assert(DTLS1_RT_MAX_HEADER_LENGTH >= DTLS1_3_RECORD_HEADER_WRITE_LENGTH,
              "DTLS1_RT_MAX_HEADER_LENGTH must not be smaller than defined "
              "record header lengths");

#define DTLS1_HM_HEADER_LENGTH 12

// A DTLSMessageBitmap maintains a list of bits which may be marked to indicate
// a portion of a message was received or ACKed.
class DTLSMessageBitmap {
 public:
  // A Range represents a range of bits from |start|, inclusive, to |end|,
  // exclusive.
  struct Range {
    size_t start = 0;
    size_t end = 0;

    bool empty() const { return start == end; }
    size_t size() const { return end - start; }
    bool operator==(const Range &r) const {
      return start == r.start && end == r.end;
    }
    bool operator!=(const Range &r) const { return !(*this == r); }
  };

  // Init initializes the structure with |num_bits| unmarked bits, from zero
  // to |num_bits - 1|.
  bool Init(size_t num_bits);

  // MarkRange marks the bits from |start|, inclusive, to |end|, exclusive.
  void MarkRange(size_t start, size_t end);

  // NextUnmarkedRange returns the next range of unmarked bits, starting from
  // |start|, inclusive. If all bits after |start| are marked, it returns an
  // empty range.
  Range NextUnmarkedRange(size_t start) const;

  // IsComplete returns whether every bit in the bitmask has been marked.
  bool IsComplete() const { return bytes_.empty(); }

 private:
  // bytes_ contains the unmarked bits. We maintain an invariant: if |bytes_| is
  // not empty, some bit is unset.
  Array<uint8_t> bytes_;
  // first_unmarked_byte_ is the index of first byte in |bytes_| that is not
  // 0xff. This is maintained to amortize checking if the message is complete.
  size_t first_unmarked_byte_ = 0;
};

struct hm_header_st {
  uint8_t type;
  uint32_t msg_len;
  uint16_t seq;
  uint32_t frag_off;
  uint32_t frag_len;
};

// An DTLSIncomingMessage is an incoming DTLS message, possibly not yet
// assembled.
struct DTLSIncomingMessage {
  static constexpr bool kAllowUniquePtr = true;

  Span<uint8_t> msg() { return Span(data).subspan(DTLS1_HM_HEADER_LENGTH); }
  Span<const uint8_t> msg() const {
    return Span(data).subspan(DTLS1_HM_HEADER_LENGTH);
  }
  size_t msg_len() const { return msg().size(); }

  // type is the type of the message.
  uint8_t type = 0;
  // seq is the sequence number of this message.
  uint16_t seq = 0;
  // data contains the message, including the message header of length
  // |DTLS1_HM_HEADER_LENGTH|.
  Array<uint8_t> data;
  // reassembly tracks which parts of the message have been received.
  DTLSMessageBitmap reassembly;
};

struct DTLSOutgoingMessage {
  size_t msg_len() const {
    assert(!is_ccs);
    assert(data.size() >= DTLS1_HM_HEADER_LENGTH);
    return data.size() - DTLS1_HM_HEADER_LENGTH;
  }

  bool IsFullyAcked() const {
    // ACKs only exist in DTLS 1.3, which does not send ChangeCipherSpec.
    return !is_ccs && acked.IsComplete();
  }

  Array<uint8_t> data;
  uint16_t epoch = 0;
  bool is_ccs = false;
  // acked tracks which bits of the message have been ACKed by the peer. If
  // |msg_len| is zero, it tracks one bit for whether the header has been
  // received.
  DTLSMessageBitmap acked;
};

struct OPENSSL_timeval {
  uint64_t tv_sec;
  uint32_t tv_usec;
};

struct DTLSTimer {
 public:
  static constexpr uint64_t kNever = UINT64_MAX;

  // StartMicroseconds schedules the timer to expire the specified number of
  // microseconds from |now|.
  void StartMicroseconds(OPENSSL_timeval now, uint64_t microseconds);

  // Stop disables the timer.
  void Stop();

  // IsExpired returns true if the timer was set and is expired at time |now|.
  bool IsExpired(OPENSSL_timeval now) const;

  // IsSet returns true if the timer is scheduled or expired, and false if it is
  // stopped.
  bool IsSet() const;

  // MicrosecondsRemaining returns the time remaining, in microseconds, at
  // |now|, or |kNever| if the timer is unset.
  uint64_t MicrosecondsRemaining(OPENSSL_timeval now) const;

 private:
  // expire_time_ is the time when the timer expires, or zero if the timer is
  // unset.
  //
  // TODO(crbug.com/366284846): This is an extremely inconvenient time
  // representation. Switch libssl to something like a 64-bit count of
  // microseconds. While it's decidedly past 1970 now, zero is a less obviously
  // sound distinguished value for the monotonic clock, so maybe we should use a
  // different distinguished time, like |INT64_MAX| in the microseconds
  // representation.
  OPENSSL_timeval expire_time_ = {0, 0};
};

// DTLS_MAX_EXTRA_WRITE_EPOCHS is the maximum number of additional write epochs
// that DTLS may need to retain.
//
// The maximum is, as a DTLS 1.3 server, immediately after sending Finished. At
// this point, the current epoch is the application write keys (epoch 3), but we
// may have ServerHello (epoch 0) and EncryptedExtensions (epoch 1) to
// retransmit. KeyUpdate does not increase this count. If the server were to
// initiate KeyUpdate from this state, it would not apply the new epoch until
// the client's ACKs have caught up. At that point, epochs 0 and 1 can be
// discarded.
#define DTLS_MAX_EXTRA_WRITE_EPOCHS 2

// DTLS_MAX_ACK_BUFFER is the maximum number of records worth of data we'll keep
// track of with DTLS 1.3 ACKs. When we exceed this value, information about
// stale records will be dropped. This will not break the connection but may
// cause ACKs to perform worse and retransmit unnecessary information.
#define DTLS_MAX_ACK_BUFFER 32

// A DTLSSentRecord records information about a record we sent. Each record
// covers all bytes from |first_msg_start| (inclusive) of |first_msg| to
// |last_msg_end| (exclusive) of |last_msg|. Messages are referenced by index
// into |outgoing_messages|. |last_msg_end| may be |outgoing_messages.size()| if
// |last_msg_end| is zero.
//
// When the message is empty, |first_msg_start| and |last_msg_end| are
// maintained as if there is a single bit in the message representing the
// header. See |acked| in DTLSOutgoingMessage.
struct DTLSSentRecord {
  DTLSRecordNumber number;
  PackedSize<SSL_MAX_HANDSHAKE_FLIGHT> first_msg = 0;
  PackedSize<SSL_MAX_HANDSHAKE_FLIGHT> last_msg = 0;
  uint32_t first_msg_start = 0;
  uint32_t last_msg_end = 0;
};

enum class QueuedKeyUpdate {
  kNone,
  kUpdateNotRequested,
  kUpdateRequested,
};

// DTLS_PREV_READ_EPOCH_EXPIRE_SECONDS is how long to retain the previous read
// epoch in DTLS 1.3. This value is set based on the following:
//
// - Section 4.2.1 of RFC 9147 recommends retaining past read epochs for the
//   default TCP MSL. This accommodates packet reordering with KeyUpdate.
//
// - Section 5.8.1 of RFC 9147 requires being capable of ACKing the client's
//   final flight for at least twice the default MSL. That requires retaining
//   epoch 2 after the handshake.
//
// - Section 4 of RFC 9293 defines the MSL to be two minutes.
#define DTLS_PREV_READ_EPOCH_EXPIRE_SECONDS (4 * 60)

struct DTLSPrevReadEpoch {
  static constexpr bool kAllowUniquePtr = true;
  DTLSReadEpoch epoch;
  // expire is the expiration time of the read epoch, expressed as a POSIX
  // timestamp in seconds.
  uint64_t expire;
};

struct DTLS1_STATE {
  static constexpr bool kAllowUniquePtr = true;

  DTLS1_STATE();
  ~DTLS1_STATE();

  bool Init();

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

  // handshake_write_overflow and handshake_read_overflow are true if
  // handshake_write_seq and handshake_read_seq, respectively have overflowed.
  bool handshake_write_overflow : 1;
  bool handshake_read_overflow : 1;

  // sending_flight and sending_ack are true if we are in the process of sending
  // a handshake flight and ACK, respectively.
  bool sending_flight : 1;
  bool sending_ack : 1;

  // queued_key_update, if not kNone, indicates we've queued a KeyUpdate message
  // to send after the current flight is ACKed.
  QueuedKeyUpdate queued_key_update : 2;

  uint16_t handshake_write_seq = 0;
  uint16_t handshake_read_seq = 0;

  // read_epoch is the current read epoch.
  DTLSReadEpoch read_epoch;

  // next_read_epoch is the next read epoch in DTLS 1.3. It will become
  // current once a record is received from it.
  UniquePtr<DTLSReadEpoch> next_read_epoch;

  // prev_read_epoch is the previous read epoch in DTLS 1.3.
  UniquePtr<DTLSPrevReadEpoch> prev_read_epoch;

  // write_epoch is the current DTLS write epoch. Non-retransmit records will
  // generally use this epoch.
  // TODO(crbug.com/381113363): 0-RTT will be the exception, when implemented.
  DTLSWriteEpoch write_epoch;

  // extra_write_epochs is the collection available write epochs.
  InplaceVector<UniquePtr<DTLSWriteEpoch>, DTLS_MAX_EXTRA_WRITE_EPOCHS>
      extra_write_epochs;

  // incoming_messages is a ring buffer of incoming handshake messages that have
  // yet to be processed. The front of the ring buffer is message number
  // |handshake_read_seq|, at position |handshake_read_seq| %
  // |SSL_MAX_HANDSHAKE_FLIGHT|.
  UniquePtr<DTLSIncomingMessage> incoming_messages[SSL_MAX_HANDSHAKE_FLIGHT];

  // outgoing_messages is the queue of outgoing messages from the last handshake
  // flight.
  InplaceVector<DTLSOutgoingMessage, SSL_MAX_HANDSHAKE_FLIGHT>
      outgoing_messages;

  // sent_records is a queue of records we sent, for processing ACKs. To save
  // memory in the steady state, the structure is stored on the heap and dropped
  // when empty.
  UniquePtr<MRUQueue<DTLSSentRecord, DTLS_MAX_ACK_BUFFER>> sent_records;

  // records_to_ack is a queue of received records that we should ACK. This is
  // not stored on the heap because, in the steady state, DTLS 1.3 does not
  // necessarily empty this list. (We probably could drop records from here once
  // they are sufficiently old.)
  MRUQueue<DTLSRecordNumber, DTLS_MAX_ACK_BUFFER> records_to_ack;

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

  // retransmit_timer tracks when to schedule the next DTLS retransmit if we do
  // not hear from the peer.
  DTLSTimer retransmit_timer;

  // ack_timer tracks when to send an ACK.
  DTLSTimer ack_timer;

  // timeout_duration_ms is the timeout duration in milliseconds.
  uint32_t timeout_duration_ms = 0;
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

  // crypto
  UniquePtr<SSLCipherPreferenceList> cipher_list;

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

  // For client side, keep the list of CA distinguished names we can use
  // for the Certificate Authorities extension.
  // TODO(bbe) having this separate from the client side (above) is mildly
  // silly, but OpenSSL has *_client_CA API's for this exposed, and for the
  // moment we are not crossing those streams.
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> CA_names;

  // Trust anchor IDs to be requested in the trust_anchors extension.
  std::optional<Array<uint8_t>> requested_trust_anchors;

  Array<uint16_t> supported_group_list;  // our list

  // channel_id_private is the client's Channel ID private key, or null if
  // Channel ID should not be offered on this connection.
  UniquePtr<EVP_PKEY> channel_id_private;

  // For a client, this contains the list of supported protocols in wire
  // format.
  Array<uint8_t> alpn_client_proto_list;

  // alps_configs contains the list of supported protocols to use with ALPS,
  // along with their corresponding ALPS values.
  Vector<ALPSConfig> alps_configs;

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

  // compliance_policy limits the set of ciphers that can be selected when
  // negotiating a TLS 1.3 connection.
  enum ssl_compliance_policy_t compliance_policy = ssl_compliance_policy_none;

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

  // alps_use_new_codepoint if set indicates we use new ALPS extension codepoint
  // to negotiate and convey application settings.
  bool alps_use_new_codepoint : 1;
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
bool ssl_get_new_session(SSL_HANDSHAKE *hs);

// ssl_encrypt_ticket encrypt a ticket for |session| and writes the result to
// |out|. It returns true on success and false on error. If, on success, nothing
// was written to |out|, the caller should skip sending a ticket.
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

enum class SSLSessionType {
  // The session is not resumable.
  kNotResumable,
  // The session uses a TLS 1.2 session ID.
  kID,
  // The session uses a TLS 1.2 ticket.
  kTicket,
  // The session uses a TLS 1.3 pre-shared key.
  kPreSharedKey,
};

// ssl_session_get_type returns the type of |session|.
SSLSessionType ssl_session_get_type(const SSL_SESSION *session);

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
int tls_flush(SSL *ssl);

bool dtls1_init_message(const SSL *ssl, CBB *cbb, CBB *body, uint8_t type);
bool dtls1_finish_message(const SSL *ssl, CBB *cbb, Array<uint8_t> *out_msg);
bool dtls1_add_message(SSL *ssl, Array<uint8_t> msg);
bool dtls1_add_change_cipher_spec(SSL *ssl);
void dtls1_finish_flight(SSL *ssl);
void dtls1_schedule_ack(SSL *ssl);
int dtls1_flush(SSL *ssl);

// ssl_add_message_cbb finishes the handshake message in |cbb| and adds it to
// the pending flight. It returns true on success and false on error.
bool ssl_add_message_cbb(SSL *ssl, CBB *cbb);

// ssl_hash_message incorporates |msg| into the handshake hash. It returns true
// on success and false on allocation failure.
bool ssl_hash_message(SSL_HANDSHAKE *hs, const SSLMessage &msg);

ssl_open_record_t dtls1_process_ack(SSL *ssl, uint8_t *out_alert,
                                    DTLSRecordNumber ack_record_number,
                                    Span<const uint8_t> data);
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
                       uint16_t epoch);

bool dtls1_parse_fragment(CBS *cbs, struct hm_header_st *out_hdr,
                          CBS *out_body);

// DTLS1_MTU_TIMEOUTS is the maximum number of retransmit timeouts to expire
// before starting to decrease the MTU.
#define DTLS1_MTU_TIMEOUTS 2

// DTLS1_MAX_TIMEOUTS is the maximum number of retransmit timeouts to expire
// before failing the DTLS handshake.
#define DTLS1_MAX_TIMEOUTS 12

void dtls1_stop_timer(SSL *ssl);

unsigned int dtls1_min_mtu(void);

bool dtls1_new(SSL *ssl);
void dtls1_free(SSL *ssl);

bool dtls1_process_handshake_fragments(SSL *ssl, uint8_t *out_alert,
                                       DTLSRecordNumber record_number,
                                       Span<const uint8_t> record);
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

// tls1_generate_master_secret computes the master secret from |premaster| and
// writes it to |out|. |out| must have size |SSL3_MASTER_SECRET_SIZE|.
bool tls1_generate_master_secret(SSL_HANDSHAKE *hs, Span<uint8_t> out,
                                 Span<const uint8_t> premaster);

// tls1_get_grouplist returns the locally-configured group preference list.
Span<const uint16_t> tls1_get_grouplist(const SSL_HANDSHAKE *ssl);

// tls1_check_group_id returns whether |group_id| is consistent with locally-
// configured group preferences.
bool tls1_check_group_id(const SSL_HANDSHAKE *ssl, uint16_t group_id);

// tls1_get_shared_group sets |*out_group_id| to the first preferred shared
// group between client and server preferences and returns true. If none may be
// found, it returns false.
bool tls1_get_shared_group(SSL_HANDSHAKE *hs, uint16_t *out_group_id);

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

OPENSSL_timeval ssl_ctx_get_current_time(const SSL_CTX *ctx);

// ssl_reset_error_state resets state for |SSL_get_error|.
void ssl_reset_error_state(SSL *ssl);

// ssl_set_read_error sets |ssl|'s read half into an error state, saving the
// current state of the error queue.
void ssl_set_read_error(SSL *ssl);

BSSL_NAMESPACE_END


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

  // quic_method is the method table corresponding to the QUIC hooks.
  const SSL_QUIC_METHOD *quic_method = nullptr;

  bssl::UniquePtr<bssl::SSLCipherPreferenceList> cipher_list;

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

  // Default values used when no per-SSL value is defined follow

  void (*info_callback)(const SSL *ssl, int type, int value) = nullptr;

  // what we put in client cert requests
  bssl::UniquePtr<STACK_OF(CRYPTO_BUFFER)> client_CA;

  // cached_x509_client_CA is a cache of parsed versions of the elements of
  // |client_CA|.
  STACK_OF(X509_NAME) *cached_x509_client_CA = nullptr;

  // What we put in client hello in the CA extension.
  bssl::UniquePtr<STACK_OF(CRYPTO_BUFFER)> CA_names;

  // What we request in the trust_anchors extension.
  std::optional<bssl::Array<uint8_t>> requested_trust_anchors;

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
  bssl::Vector<bssl::CertCompressionAlg> cert_compression_algs;

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

  // compliance_policy limits the set of ciphers that can be selected when
  // negotiating a TLS 1.3 connection.
  enum ssl_compliance_policy_t compliance_policy = ssl_compliance_policy_none;

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

  // aes_hw_override if set indicates we should override checking for AES
  // hardware support, and use the value in aes_hw_override_value instead.
  bool aes_hw_override : 1;

  // aes_hw_override_value is used for testing to indicate the support or lack
  // of support for AES hardware. The value is only considered if
  // |aes_hw_override| is true.
  bool aes_hw_override_value : 1;

  // resumption_across_names_enabled indicates whether a TLS 1.3 server should
  // signal its sessions may be resumed across names in the server certificate.
  bool resumption_across_names_enabled : 1;

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

  uint16_t max_send_fragment = 0;

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

  // session info

  // initial_timeout_duration_ms is the default DTLS timeout duration in
  // milliseconds. It's used to initialize the timer any time it's restarted. We
  // default to RFC 9147's recommendation for real-time applications, 400ms.
  uint32_t initial_timeout_duration_ms = 400;

  // session is the configured session to be offered by the client. This session
  // is immutable.
  bssl::UniquePtr<SSL_SESSION> session;

  void (*info_callback)(const SSL *ssl, int type, int value) = nullptr;

  bssl::UniquePtr<SSL_CTX> ctx;

  // session_ctx is the |SSL_CTX| used for the session cache and related
  // settings.
  bssl::UniquePtr<SSL_CTX> session_ctx;

  // extra application data
  CRYPTO_EX_DATA ex_data;

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

  // resumption_across_names_enabled indicates whether a TLS 1.3 server should
  // signal its sessions may be resumed across names in the server certificate.
  bool resumption_across_names_enabled : 1;
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
  bssl::InplaceVector<uint8_t, SSL_MAX_MASTER_KEY_LENGTH> secret;

  bssl::InplaceVector<uint8_t, SSL_MAX_SSL_SESSION_ID_LENGTH> session_id;

  // this is used to determine whether the session is being reused in
  // the appropriate context. It is up to the application to set this,
  // via SSL_new
  bssl::InplaceVector<uint8_t, SSL_MAX_SID_CTX_LENGTH> sid_ctx;

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
  bssl::InplaceVector<uint8_t, SSL_MAX_MD_SIZE> original_handshake_hash;

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

  // is_resumable_across_names indicates whether the session may be resumed for
  // any of the identities presented in the certificate.
  bool is_resumable_across_names : 1;

  // quic_early_data_context is used to determine whether early data must be
  // rejected when performing a QUIC handshake.
  bssl::Array<uint8_t> quic_early_data_context;

 private:
  friend RefCounted;
  ~ssl_session_st();
};

struct ssl_ech_keys_st : public bssl::RefCounted<ssl_ech_keys_st> {
  ssl_ech_keys_st() : RefCounted(CheckSubClass()) {}

  bssl::Vector<bssl::UniquePtr<bssl::ECHServerConfig>> configs;

 private:
  friend RefCounted;
  ~ssl_ech_keys_st() = default;
};

#endif  // OPENSSL_HEADER_SSL_INTERNAL_H
