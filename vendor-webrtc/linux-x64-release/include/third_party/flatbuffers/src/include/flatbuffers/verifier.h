/*
 * Copyright 2021 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_VERIFIER_H_
#define FLATBUFFERS_VERIFIER_H_

#include "flatbuffers/base.h"
#include "flatbuffers/vector.h"

namespace flatbuffers {

// Helper class to verify the integrity of a FlatBuffer
template <bool TrackVerifierBufferSize>
class VerifierTemplate FLATBUFFERS_FINAL_CLASS {
 public:
  struct Options {
    // The maximum nesting of tables and vectors before we call it invalid.
    uoffset_t max_depth = 64;
    // The maximum number of tables we will verify before we call it invalid.
    uoffset_t max_tables = 1000000;
    // If true, verify all data is aligned.
    bool check_alignment = true;
    // If true, run verifier on nested flatbuffers
    bool check_nested_flatbuffers = true;
    // The maximum size of a buffer.
    size_t max_size = FLATBUFFERS_MAX_BUFFER_SIZE;
    // Use assertions to check for errors.
    bool assert = false;
  };

  explicit VerifierTemplate(const uint8_t *const buf, const size_t buf_len,
                            const Options &opts)
      : buf_(buf), size_(buf_len), opts_(opts) {
    FLATBUFFERS_ASSERT(size_ < opts.max_size);
  }

  // Deprecated API, please construct with VerifierTemplate::Options.
  VerifierTemplate(const uint8_t *const buf, const size_t buf_len,
                   const uoffset_t max_depth = 64,
                   const uoffset_t max_tables = 1000000,
                   const bool check_alignment = true)
      : VerifierTemplate(buf, buf_len, [&] {
          Options opts;
          opts.max_depth = max_depth;
          opts.max_tables = max_tables;
          opts.check_alignment = check_alignment;
          return opts;
        }()) {}

  // Central location where any verification failures register.
  bool Check(const bool ok) const {
    // clang-format off
    #ifdef FLATBUFFERS_DEBUG_VERIFICATION_FAILURE
      if (opts_.assert) { FLATBUFFERS_ASSERT(ok); }
    #endif
    // clang-format on
    if (TrackVerifierBufferSize) {
      if (!ok) {
        upper_bound_ = 0;
      }
    }
    return ok;
  }

  // Verify any range within the buffer.
  bool Verify(const size_t elem, const size_t elem_len) const {
    if (TrackVerifierBufferSize) {
      auto upper_bound = elem + elem_len;
      if (upper_bound_ < upper_bound) {
        upper_bound_ =  upper_bound;
      }
    }
    return Check(elem_len < size_ && elem <= size_ - elem_len);
  }

  bool VerifyAlignment(const size_t elem, const size_t align) const {
    return Check((elem & (align - 1)) == 0 || !opts_.check_alignment);
  }

  // Verify a range indicated by sizeof(T).
  template<typename T> bool Verify(const size_t elem) const {
    return VerifyAlignment(elem, sizeof(T)) && Verify(elem, sizeof(T));
  }

  bool VerifyFromPointer(const uint8_t *const p, const size_t len) {
    return Verify(static_cast<size_t>(p - buf_), len);
  }

  // Verify relative to a known-good base pointer.
  bool VerifyFieldStruct(const uint8_t *const base, const voffset_t elem_off,
                         const size_t elem_len, const size_t align) const {
    const auto f = static_cast<size_t>(base - buf_) + elem_off;
    return VerifyAlignment(f, align) && Verify(f, elem_len);
  }

  template<typename T>
  bool VerifyField(const uint8_t *const base, const voffset_t elem_off,
                   const size_t align) const {
    const auto f = static_cast<size_t>(base - buf_) + elem_off;
    return VerifyAlignment(f, align) && Verify(f, sizeof(T));
  }

  // Verify a pointer (may be NULL) of a table type.
  template<typename T> bool VerifyTable(const T *const table) {
    return !table || table->Verify(*this);
  }

  // Verify a pointer (may be NULL) of any vector type.
  template<int &..., typename T, typename LenT>
  bool VerifyVector(const Vector<T, LenT> *const vec) const {
    return !vec || VerifyVectorOrString<LenT>(
                       reinterpret_cast<const uint8_t *>(vec), sizeof(T));
  }

  // Verify a pointer (may be NULL) of a vector to struct.
  template<int &..., typename T, typename LenT>
  bool VerifyVector(const Vector<const T *, LenT> *const vec) const {
    return VerifyVector(reinterpret_cast<const Vector<T, LenT> *>(vec));
  }

  // Verify a pointer (may be NULL) to string.
  bool VerifyString(const String *const str) const {
    size_t end;
    return !str || (VerifyVectorOrString<uoffset_t>(
                        reinterpret_cast<const uint8_t *>(str), 1, &end) &&
                    Verify(end, 1) &&           // Must have terminator
                    Check(buf_[end] == '\0'));  // Terminating byte must be 0.
  }

  // Common code between vectors and strings.
  template<typename LenT = uoffset_t>
  bool VerifyVectorOrString(const uint8_t *const vec, const size_t elem_size,
                            size_t *const end = nullptr) const {
    const auto vec_offset = static_cast<size_t>(vec - buf_);
    // Check we can read the size field.
    if (!Verify<LenT>(vec_offset)) return false;
    // Check the whole array. If this is a string, the byte past the array must
    // be 0.
    const LenT size = ReadScalar<LenT>(vec);
    const auto max_elems = opts_.max_size / elem_size;
    if (!Check(size < max_elems))
      return false;  // Protect against byte_size overflowing.
    const auto byte_size = sizeof(LenT) + elem_size * size;
    if (end) *end = vec_offset + byte_size;
    return Verify(vec_offset, byte_size);
  }

  // Special case for string contents, after the above has been called.
  bool VerifyVectorOfStrings(const Vector<Offset<String>> *const vec) const {
    if (vec) {
      for (uoffset_t i = 0; i < vec->size(); i++) {
        if (!VerifyString(vec->Get(i))) return false;
      }
    }
    return true;
  }

  // Special case for table contents, after the above has been called.
  template<typename T>
  bool VerifyVectorOfTables(const Vector<Offset<T>> *const vec) {
    if (vec) {
      for (uoffset_t i = 0; i < vec->size(); i++) {
        if (!vec->Get(i)->Verify(*this)) return false;
      }
    }
    return true;
  }

  FLATBUFFERS_SUPPRESS_UBSAN("unsigned-integer-overflow")
  bool VerifyTableStart(const uint8_t *const table) {
    // Check the vtable offset.
    const auto tableo = static_cast<size_t>(table - buf_);
    if (!Verify<soffset_t>(tableo)) return false;
    // This offset may be signed, but doing the subtraction unsigned always
    // gives the result we want.
    const auto vtableo =
        tableo - static_cast<size_t>(ReadScalar<soffset_t>(table));
    // Check the vtable size field, then check vtable fits in its entirety.
    if (!(VerifyComplexity() && Verify<voffset_t>(vtableo) &&
          VerifyAlignment(ReadScalar<voffset_t>(buf_ + vtableo),
                          sizeof(voffset_t))))
      return false;
    const auto vsize = ReadScalar<voffset_t>(buf_ + vtableo);
    return Check((vsize & 1) == 0) && Verify(vtableo, vsize);
  }

  template<typename T>
  bool VerifyBufferFromStart(const char *const identifier, const size_t start) {
    // Buffers have to be of some size to be valid. The reason it is a runtime
    // check instead of static_assert, is that nested flatbuffers go through
    // this call and their size is determined at runtime.
    if (!Check(size_ >= FLATBUFFERS_MIN_BUFFER_SIZE)) return false;

    // If an identifier is provided, check that we have a buffer
    if (identifier && !Check((size_ >= 2 * sizeof(flatbuffers::uoffset_t) &&
                              BufferHasIdentifier(buf_ + start, identifier)))) {
      return false;
    }

    // Call T::Verify, which must be in the generated code for this type.
    const auto o = VerifyOffset<uoffset_t>(start);
    if (!Check(o != 0)) return false;
    if (!(reinterpret_cast<const T *>(buf_ + start + o)->Verify(*this))) {
      return false;
    }
    if (TrackVerifierBufferSize) {
      if (GetComputedSize() == 0) return false;
    }
    return true;
  }

  template<typename T, int &..., typename SizeT>
  bool VerifyNestedFlatBuffer(const Vector<uint8_t, SizeT> *const buf,
                              const char *const identifier) {
    // Caller opted out of this.
    if (!opts_.check_nested_flatbuffers) return true;

    // An empty buffer is OK as it indicates not present.
    if (!buf) return true;

    // If there is a nested buffer, it must be greater than the min size.
    if (!Check(buf->size() >= FLATBUFFERS_MIN_BUFFER_SIZE)) return false;

    VerifierTemplate<TrackVerifierBufferSize> nested_verifier(
        buf->data(), buf->size(), opts_);
    return nested_verifier.VerifyBuffer<T>(identifier);
  }

  // Verify this whole buffer, starting with root type T.
  template<typename T> bool VerifyBuffer() { return VerifyBuffer<T>(nullptr); }

  template<typename T> bool VerifyBuffer(const char *const identifier) {
    return VerifyBufferFromStart<T>(identifier, 0);
  }

  template<typename T, typename SizeT = uoffset_t>
  bool VerifySizePrefixedBuffer(const char *const identifier) {
    return Verify<SizeT>(0U) &&
           // Ensure the prefixed size is within the bounds of the provided
           // length.
           Check(ReadScalar<SizeT>(buf_) + sizeof(SizeT) <= size_) &&
           VerifyBufferFromStart<T>(identifier, sizeof(SizeT));
  }

  template<typename OffsetT = uoffset_t, typename SOffsetT = soffset_t>
  size_t VerifyOffset(const size_t start) const {
    if (!Verify<OffsetT>(start)) return 0;
    const auto o = ReadScalar<OffsetT>(buf_ + start);
    // May not point to itself.
    if (!Check(o != 0)) return 0;
    // Can't wrap around larger than the max size.
    if (!Check(static_cast<SOffsetT>(o) >= 0)) return 0;
    // Must be inside the buffer to create a pointer from it (pointer outside
    // buffer is UB).
    if (!Verify(start + o, 1)) return 0;
    return o;
  }

  template<typename OffsetT = uoffset_t>
  size_t VerifyOffset(const uint8_t *const base, const voffset_t start) const {
    return VerifyOffset<OffsetT>(static_cast<size_t>(base - buf_) + start);
  }

  // Called at the start of a table to increase counters measuring data
  // structure depth and amount, and possibly bails out with false if limits set
  // by the constructor have been hit. Needs to be balanced with EndTable().
  bool VerifyComplexity() {
    depth_++;
    num_tables_++;
    return Check(depth_ <= opts_.max_depth && num_tables_ <= opts_.max_tables);
  }

  // Called at the end of a table to pop the depth count.
  bool EndTable() {
    depth_--;
    return true;
  }

  // Returns the message size in bytes.
  //
  // This should only be called after first calling VerifyBuffer or
  // VerifySizePrefixedBuffer.
  //
  // This method should only be called for VerifierTemplate instances
  // where the TrackVerifierBufferSize template parameter is true,
  // i.e. for SizeVerifier.  For instances where TrackVerifierBufferSize
  // is false, this fails at runtime or returns zero.
  size_t GetComputedSize() const {
    if (TrackVerifierBufferSize) {
      uintptr_t size = upper_bound_;
      // Align the size to uoffset_t
      size = (size - 1 + sizeof(uoffset_t)) & ~(sizeof(uoffset_t) - 1);
      return (size > size_) ?  0 : size;
    }
    // Must use SizeVerifier, or (deprecated) turn on
    // FLATBUFFERS_TRACK_VERIFIER_BUFFER_SIZE, for this to work.
    (void)upper_bound_;
    FLATBUFFERS_ASSERT(false);
    return 0;
  }

  std::vector<uint8_t> *GetFlexReuseTracker() { return flex_reuse_tracker_; }

  void SetFlexReuseTracker(std::vector<uint8_t> *const rt) {
    flex_reuse_tracker_ = rt;
  }

 private:
  const uint8_t *buf_;
  const size_t size_;
  const Options opts_;

  mutable size_t upper_bound_ = 0;

  uoffset_t depth_ = 0;
  uoffset_t num_tables_ = 0;
  std::vector<uint8_t> *flex_reuse_tracker_ = nullptr;
};

// Specialization for 64-bit offsets.
template<>
template<>
inline size_t VerifierTemplate<false>::VerifyOffset<uoffset64_t>(
    const size_t start) const {
  return VerifyOffset<uoffset64_t, soffset64_t>(start);
}
template<>
template<>
inline size_t VerifierTemplate<true>::VerifyOffset<uoffset64_t>(
    const size_t start) const {
  return VerifyOffset<uoffset64_t, soffset64_t>(start);
}

// Instance of VerifierTemplate that supports GetComputedSize().
using SizeVerifier = VerifierTemplate</*TrackVerifierBufferSize = */ true>;

// The FLATBUFFERS_TRACK_VERIFIER_BUFFER_SIZE build configuration macro is
// deprecated, and should not be defined, since it is easy to misuse in ways
// that result in ODR violations. Rather than using Verifier and defining
// FLATBUFFERS_TRACK_VERIFIER_BUFFER_SIZE, please use SizeVerifier instead.
#ifdef FLATBUFFERS_TRACK_VERIFIER_BUFFER_SIZE  // Deprecated, see above.
using Verifier = SizeVerifier;
#else
// Instance of VerifierTemplate that is slightly faster, but does not
// support GetComputedSize().
using Verifier = VerifierTemplate</*TrackVerifierBufferSize = */ false>;
#endif

}  // namespace flatbuffers

#endif  // FLATBUFFERS_VERIFIER_H_
