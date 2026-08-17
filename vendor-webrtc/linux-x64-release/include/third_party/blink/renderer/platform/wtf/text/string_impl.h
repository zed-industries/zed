/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2005, 2006, 2007, 2008, 2009, 2010, 2013 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_IMPL_H_

#include <limits.h>
#include <string.h>

#include <array>
#include <atomic>
#include <functional>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"
#include "base/numerics/checked_math.h"
#include "base/numerics/safe_conversions.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_fast_path.h"
#include "third_party/blink/renderer/platform/wtf/text/number_parsing_options.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hasher.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

#if BUILDFLAG(IS_APPLE)
#include "base/apple/scoped_cftyperef.h"

typedef const struct __CFString* CFStringRef;
#endif

#ifdef __OBJC__
@class NSString;
#endif

namespace WTF {

enum TextCaseSensitivity {
  kTextCaseSensitive,
  kTextCaseASCIIInsensitive,
};

// Computes a standard StringHasher string for the given buffer,
// with the caveat that the buffer may contain 8-bit data only.
// In that case, it is converted from UChar to LChar on the fly,
// so that we return the same hash as if we hashed the string as
// LChar to begin with. This ensures that the same code points
// are hashed to the same value, even if someone called e.g.
// Ensure16Bit() on the string at some point.
WTF_EXPORT unsigned ComputeHashForWideString(const UChar* str, unsigned length);

enum StripBehavior { kStripExtraWhiteSpace, kDoNotStripWhiteSpace };

typedef bool (*CharacterMatchFunctionPtr)(UChar);
typedef bool (*IsWhiteSpaceFunctionPtr)(UChar);
typedef HashMap<wtf_size_t, StringImpl*, AlreadyHashedTraits>
    StaticStringsTable;

// You can find documentation about this class in this doc:
// https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/wtf/text/README.md
class WTF_EXPORT StringImpl {
 private:
  // StringImpls are allocated out of the WTF buffer partition.
  void* operator new(size_t);
  void* operator new(size_t, void* ptr) { return ptr; }
  void operator delete(void*);

  // Used to construct static strings, which have a special ref_count_ that can
  // never hit zero. This means that the static string will never be destroyed.
  enum ConstructEmptyStringTag { kConstructEmptyString };
  explicit StringImpl(ConstructEmptyStringTag)
      : length_(0),
        hash_and_flags_(kAsciiPropertyCheckDone | kContainsOnlyAscii |
                        kIsLowerAscii | kIs8Bit | kIsStatic) {
    // Ensure that the hash is computed so that AtomicStringHash can call
    // existingHash() with impunity. The empty string is special because it
    // is never entered into AtomicString's HashKey, but still needs to
    // compare correctly.
    GetHash();
  }

  enum ConstructEmptyString16BitTag { kConstructEmptyString16Bit };
  explicit StringImpl(ConstructEmptyString16BitTag)
      : length_(0),
        hash_and_flags_(kAsciiPropertyCheckDone | kContainsOnlyAscii |
                        kIsLowerAscii | kIsStatic) {
    GetHash();
  }

  // FIXME: there has to be a less hacky way to do this.
  enum Force8Bit { kForce8BitConstructor };
  StringImpl(wtf_size_t length, Force8Bit)
      : length_(length), hash_and_flags_(LengthToAsciiFlags(length) | kIs8Bit) {
    DCHECK(length_);
  }

  StringImpl(wtf_size_t length)
      : length_(length), hash_and_flags_(LengthToAsciiFlags(length)) {
    DCHECK(length_);
  }

  enum StaticStringTag { kStaticString };
  StringImpl(wtf_size_t length, wtf_size_t hash, StaticStringTag)
      : length_(length),
        hash_and_flags_(hash << kHashShift | LengthToAsciiFlags(length) |
                        kIs8Bit | kIsStatic) {}

 public:
  REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE();
  static StringImpl* empty_;
  static StringImpl* empty16_bit_;

  StringImpl(const StringImpl&) = delete;
  StringImpl& operator=(const StringImpl&) = delete;
  ~StringImpl();

  static void InitStatics();

  static StringImpl* CreateStatic(base::span<const char> string);
  static void ReserveStaticStringsCapacityForSize(wtf_size_t size);
  static void FreezeStaticStrings();
  static const StaticStringsTable& AllStaticStrings();
  static wtf_size_t HighestStaticStringLength() {
    return highest_static_string_length_;
  }

  static scoped_refptr<StringImpl> Create(base::span<const UChar>);
  static scoped_refptr<StringImpl> Create(base::span<const LChar>);
  static scoped_refptr<StringImpl> Create(
      base::span<const LChar>,
      ASCIIStringAttributes ascii_attributes);
  static scoped_refptr<StringImpl> Create8BitIfPossible(
      base::span<const UChar>);

  ALWAYS_INLINE static scoped_refptr<StringImpl> Create(
      base::span<const char> s) {
    return Create(base::as_bytes(s));
  }

  // Create a StringImpl with space for `length` LChar characters. `data` will
  // be the character data allocated, and _must_be_completely_filled_in_ by the
  // caller.
  static scoped_refptr<StringImpl> CreateUninitialized(size_t length,
                                                       base::span<LChar>& data);
  // Create a StringImpl with space for `length` UChar characters. `data` will
  // be the character data allocated, and _must_be_completely_filled_in_ by the
  // caller.
  static scoped_refptr<StringImpl> CreateUninitialized(size_t length,
                                                       base::span<UChar>& data);

  wtf_size_t length() const { return length_; }
  bool Is8Bit() const {
    return hash_and_flags_.load(std::memory_order_relaxed) & kIs8Bit;
  }

  // Use Span8() instead.
  UNSAFE_BUFFER_USAGE ALWAYS_INLINE const LChar* Characters8() const {
    DCHECK(Is8Bit());
    return reinterpret_cast<const LChar*>(this + 1);
  }
  // Use Span16() instead.
  UNSAFE_BUFFER_USAGE ALWAYS_INLINE const UChar* Characters16() const {
    DCHECK(!Is8Bit());
    return reinterpret_cast<const UChar*>(this + 1);
  }
  ALWAYS_INLINE base::span<const LChar> Span8() const {
    DCHECK(Is8Bit());
    return CharacterBuffer<LChar>();
  }
  ALWAYS_INLINE base::span<const UChar> Span16() const {
    DCHECK(!Is8Bit());
    return CharacterBuffer<UChar>();
  }
  ALWAYS_INLINE base::span<const uint16_t> SpanUint16() const {
    DCHECK(!Is8Bit());
    return CharacterBuffer<uint16_t>();
  }
  ALWAYS_INLINE const void* Bytes() const {
    return reinterpret_cast<const void*>(this + 1);
  }
  ALWAYS_INLINE base::span<const uint8_t> RawByteSpan() const {
    return {reinterpret_cast<const uint8_t*>(this + 1),
            CharactersSizeInBytes()};
  }
  // Create a new std::u16string based on this.
  // The character content is always copied.
  std::u16string ToU16String() const;

  template <typename CharType>
  ALWAYS_INLINE const CharType* GetCharacters() const;

  size_t CharactersSizeInBytes() const {
    return length() * (Is8Bit() ? sizeof(LChar) : sizeof(UChar));
  }

  bool IsAtomic() const {
    return hash_and_flags_.load(std::memory_order_acquire) & kIsAtomic;
  }

  void SetIsAtomic() {
    hash_and_flags_.fetch_or(kIsAtomic, std::memory_order_release);
  }

  void UnsetIsAtomic() {
    hash_and_flags_.fetch_and(~kIsAtomic, std::memory_order_release);
  }

  bool IsStatic() const {
    return hash_and_flags_.load(std::memory_order_relaxed) & kIsStatic;
  }

  bool ContainsOnlyASCIIOrEmpty() const;

  bool IsLowerASCII() const;

  // The high bits of 'hash' are always empty, but we prefer to store our
  // flags in the low bits because it makes them slightly more efficient to
  // access.  So, we shift left and right when setting and getting our hash
  // code.
  void SetHash(wtf_size_t hash) const {
    // Multiple clients assume that StringHasher is the canonical string
    // hash function.
    DCHECK_EQ(hash,
              (Is8Bit() ? StringHasher::ComputeHashAndMaskTop8Bits(
                              (const char*)Characters8(), length_)
                        : ComputeHashForWideString(Characters16(), length_)));
    DCHECK(hash);  // Verify that 0 is a valid sentinel hash value.
    SetHashRaw(hash);
  }

  bool HasHash() const { return GetHashRaw() != 0; }

  wtf_size_t ExistingHash() const {
    DCHECK(HasHash());
    return GetHashRaw();
  }

  wtf_size_t GetHash() const {
    if (wtf_size_t hash = GetHashRaw())
      return hash;
    return HashSlowCase();
  }

  ALWAYS_INLINE bool HasOneRef() const {
    return ref_count_.load(std::memory_order_acquire) == 1;
  }

  ALWAYS_INLINE void AddRef() const {
    if (!IsStatic()) {
      uint32_t previous_ref_count =
          ref_count_.fetch_add(1, std::memory_order_relaxed);
      CHECK_NE(previous_ref_count, std::numeric_limits<uint32_t>::max());
#if DCHECK_IS_ON()
      ref_count_change_count_++;
#endif
    }
  }

  // We explicitly remove the AddRef and Release operations from the tsan
  // bots because even though all data races in the C++ memory model sense
  // are undefined behavior, the use of atomics prevents a data race on
  // ref_count_ itself.

  // Sharing the AtomicStringTable causes other races outside of ref_count_
  // that could lead to an early deletion of the StringImpl while other
  // threads are still holding references to it.
  // Possible races:
  // 1. Races where ref_count_ doesn't reach zero are not harmful.
  // 2. Races involving only release calls are not harmful. The
  //    atomicity of the operations guarantee that only the last subtraction to
  //    be executed will trigger the deletion of the StringImpl.
  // 3. A fetch_add on thread A is ordered after a fetch_sub on thread B that
  //    reaches 0. This can only happen on an AddRef() reached through the
  //    AtomicStringTable::Add* methods, otherwise there should be another
  //    reference on thread A, and the Release() on thread B could not have
  //    reached 0. This race is mitigated by the fact that the Atomic String
  //    Table Add and Removal operations (including the fetch_sub to 0) are
  //    done under a lock.

  ALWAYS_INLINE void Release() const {
    if (!IsStatic()) {
      // This can be a relaxed load as long as the subtraction is performed
      // with acq_rel order. Any modification to `ref_count_` reordered after
      // this load will be caught by the while loop or the fetch_sub inside
      // DestroyIfNeeded().
      uint32_t current_ref = ref_count_.load(std::memory_order_relaxed);
#if DCHECK_IS_ON()
      // In non-DCHECK builds, we can save a bit of time in micro-benchmarks by
      // not checking the arithmetic. We hope that checking in DCHECK builds is
      // enough to catch implementation bugs, and that implementation bugs are
      // the only way we'd experience underflow.
      DCHECK_NE(current_ref, 0u);
      ref_count_change_count_++;
#endif
      // This is a fancy fetch_sub() that allows the actual decrement to 0 to
      // be delegated to the DestroyIfNeeded() function. The result of this
      // compare_exchange_weak() will never be 0. Without this, there would be
      // a potential race by reaching 0 and calling AddRef and Release on
      // another thread before the deletion of the string in this thread,
      // triggering the removal and destruction of the string twice.
      do {
        if (current_ref == 1) {
          DestroyIfNeeded();
          return;
        }
      } while (!ref_count_.compare_exchange_weak(current_ref, current_ref - 1,
                                                 std::memory_order_acq_rel));
    }
  }

#if DCHECK_IS_ON()
  unsigned int RefCountChangeCountForTesting() const {
    return ref_count_change_count_;
  }
  void ResetRefCountChangeCountForTesting() { ref_count_change_count_ = 0; }
#endif

  ALWAYS_INLINE void Adopted() const {}

  // FIXME: Does this really belong in StringImpl?
  template <typename T>
  static void CopyChars(base::span<T> destination, base::span<const T> source) {
    destination.copy_from(source);
  }

  ALWAYS_INLINE static void CopyChars(base::span<UChar> destination,
                                      base::span<const LChar> source) {
    CHECK_EQ(destination.size(), source.size());
    for (size_t i = 0; i < source.size(); ++i) {
      destination[i] = source[i];
    }
  }

  // It is no longer required to create isolated copies for thread-safety
  // purposes.
  scoped_refptr<StringImpl> IsolatedCopy() const;

  scoped_refptr<StringImpl> Substring(wtf_size_t pos,
                                      wtf_size_t len = UINT_MAX) const;

  UChar operator[](wtf_size_t i) const {
    SECURITY_DCHECK(i < length_);
    if (Is8Bit())
      return Characters8()[i];
    return Characters16()[i];
  }
  UChar32 CharacterStartingAt(wtf_size_t);

  bool ContainsOnlyWhitespaceOrEmpty();

  int ToInt(NumberParsingOptions, bool* ok) const;
  wtf_size_t ToUInt(NumberParsingOptions, bool* ok) const;
  int64_t ToInt64(NumberParsingOptions, bool* ok) const;
  uint64_t ToUInt64(NumberParsingOptions, bool* ok) const;

  wtf_size_t HexToUIntStrict(bool* ok);
  uint64_t HexToUInt64Strict(bool* ok);

  // FIXME: Like NumberParsingOptions::kStrict, these give false for "ok" when
  // there is trailing garbage.  Like NumberParsingOptions::kLoose, these return
  // the value when there is trailing garbage.  It would be better if these were
  // more consistent with the above functions instead.
  double ToDouble(bool* ok = nullptr);
  float ToFloat(bool* ok = nullptr);

  scoped_refptr<StringImpl> LowerASCII();
  scoped_refptr<StringImpl> UpperASCII();

  scoped_refptr<StringImpl> Fill(UChar);
  // FIXME: Do we need fill(char) or can we just do the right thing if UChar is
  // ASCII?
  scoped_refptr<StringImpl> FoldCase();

  scoped_refptr<StringImpl> Truncate(wtf_size_t length);

  wtf_size_t LengthWithStrippedWhiteSpace() const;

  scoped_refptr<StringImpl> StripWhiteSpace();
  scoped_refptr<StringImpl> StripWhiteSpace(IsWhiteSpaceFunctionPtr);
  scoped_refptr<StringImpl> SimplifyWhiteSpace(
      StripBehavior = kStripExtraWhiteSpace);
  scoped_refptr<StringImpl> SimplifyWhiteSpace(
      IsWhiteSpaceFunctionPtr,
      StripBehavior = kStripExtraWhiteSpace);

  scoped_refptr<StringImpl> RemoveCharacters(CharacterMatchFunctionPtr);
  template <typename CharType>
  ALWAYS_INLINE scoped_refptr<StringImpl> RemoveCharacters(
      base::span<const CharType> characters,
      CharacterMatchFunctionPtr);

  // Remove characters between [start, start+lengthToRemove). The range is
  // clamped to the size of the string. Does nothing if start >= length().
  scoped_refptr<StringImpl> Remove(wtf_size_t start,
                                   wtf_size_t length_to_remove = 1);

  // Find characters.
  wtf_size_t Find(LChar character, wtf_size_t start = 0) const;
  wtf_size_t Find(char character, wtf_size_t start = 0) const;
  wtf_size_t Find(UChar character, wtf_size_t start = 0) const;
  wtf_size_t Find(CharacterMatchFunctionPtr, wtf_size_t index = 0) const;
  wtf_size_t Find(base::RepeatingCallback<bool(UChar)> match_callback,
                  wtf_size_t index = 0) const;

  // Find substrings.
  wtf_size_t Find(const StringView&, wtf_size_t index = 0) const;
  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  wtf_size_t DeprecatedFindIgnoringCase(const StringView&,
                                        wtf_size_t index = 0) const;
  wtf_size_t FindIgnoringASCIICase(const StringView&,
                                   wtf_size_t index = 0) const;

  wtf_size_t ReverseFind(UChar, wtf_size_t index = UINT_MAX) const;
  wtf_size_t ReverseFind(const StringView&, wtf_size_t index = UINT_MAX) const;

  bool StartsWith(UChar) const;
  bool StartsWith(const StringView&) const;
  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  bool DeprecatedStartsWithIgnoringCase(const StringView&) const;
  bool StartsWithIgnoringCaseAndAccents(const StringView&) const;
  bool StartsWithIgnoringASCIICase(const StringView&) const;

  bool EndsWith(UChar) const;
  bool EndsWith(const StringView&) const;
  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  bool DeprecatedEndsWithIgnoringCase(const StringView&) const;
  bool EndsWithIgnoringASCIICase(const StringView&) const;

  // Replace parts of the string.
  scoped_refptr<StringImpl> Replace(UChar pattern, UChar replacement);
  scoped_refptr<StringImpl> Replace(UChar pattern,
                                    const StringView& replacement);
  scoped_refptr<StringImpl> Replace(const StringView& pattern,
                                    const StringView& replacement);
  scoped_refptr<StringImpl> Replace(wtf_size_t index,
                                    wtf_size_t length_to_replace,
                                    const StringView& replacement);

  scoped_refptr<StringImpl> UpconvertedString();

  // Copy characters from string starting at `start` up until the size of
  // `buffer` or the end of the string is reached. Returns the actual number of
  // characters copied.
  size_t CopyTo(base::span<UChar> buffer, wtf_size_t start) const;

  // Append characters from this string into a buffer. Expects the buffer to
  // have the methods:
  //    append(const UChar*, wtf_size_t length);
  //    append(const LChar*, wtf_size_t length);
  // StringBuilder and Vector conform to this protocol.
  template <typename BufferType>
  void AppendTo(BufferType&,
                wtf_size_t start = 0,
                wtf_size_t length = UINT_MAX) const;

#if BUILDFLAG(IS_APPLE)
  base::apple::ScopedCFTypeRef<CFStringRef> CreateCFString();
#endif
#ifdef __OBJC__
  operator NSString*();
#endif

  static const std::array<UChar, 256> kLatin1CaseFoldTable;

 private:
  friend class AtomicStringTable;
  enum Flags {
    // These two fields are never modified for the lifetime of the StringImpl.
    // It is therefore safe to read them with a relaxed operation.
    kIs8Bit = 1 << 0,
    kIsStatic = 1 << 1,

    // This is the only flag that can be both set and unset. It is safe to do
    // so because all accesses are mediated by the same atomic string table and
    // so protected by a mutex. Thus these accesses can also be relaxed.
    kIsAtomic = 1 << 2,

    // These bits are set atomically together. They are initially all
    // zero, and like the hash computation below, become non-zero only as part
    // of a single atomic bitwise or. Thus concurrent loads will always observe
    // either a state where the ASCII property check has not been completed and
    // all bits are zero, or a state where the state is fully populated.
    //
    // The reason kIsLowerAscii is cached but upper ascii is not is that
    // DOM attributes APIs require a lowercasing check making it fairly hot.
    kAsciiPropertyCheckDone = 1 << 3,
    kContainsOnlyAscii = 1 << 4,
    kIsLowerAscii = 1 << 5,

    // The last 24 bits (past kHashShift) are reserved for the hash.
    // These bits are all zero if the hash is uncomputed, and the hash is
    // atomically stored with bitwise or.
    //
    // Therefore a relaxed read can be used, and will either observe an
    // uncomputed hash (if the fetch_or is not yet visible on this thread)
    // or the correct hash (if it is). It is possible for a thread to compute
    // the hash for a second time if there is a race. This is safe, since
    // storing the same bits again with a bitwise or is idempotent.
  };

  // Hash value is 24 bits.
  constexpr static int kHashShift = (sizeof(unsigned) * 8) - 24;

  static inline constexpr uint32_t LengthToAsciiFlags(int length) {
    return length
               ? 0
               : kAsciiPropertyCheckDone | kContainsOnlyAscii | kIsLowerAscii;
  }

  static inline uint32_t ASCIIStringAttributesToFlags(
      ASCIIStringAttributes ascii_attributes) {
    uint32_t flags = kAsciiPropertyCheckDone;
    if (ascii_attributes.contains_only_ascii)
      flags |= kContainsOnlyAscii;
    if (ascii_attributes.is_lower_ascii)
      flags |= kIsLowerAscii;
    return flags;
  }

  void SetHashRaw(unsigned hash_val) const {
    // Setting the hash is idempotent so fetch_or() is sufficient. DCHECK()
    // as a sanity check.
    unsigned previous_value = hash_and_flags_.fetch_or(
        hash_val << kHashShift, std::memory_order_relaxed);
    DCHECK(((previous_value >> kHashShift) == 0) ||
           ((previous_value >> kHashShift) == hash_val));
  }

  unsigned GetHashRaw() const {
    return hash_and_flags_.load(std::memory_order_relaxed) >> kHashShift;
  }

  template <typename CharType>
  static size_t AllocationSize(wtf_size_t length) {
    static_assert(
        sizeof(CharType) > 1,
        "Don't use this template with 1-byte chars; use a template "
        "specialization to save time and code-size by avoiding a CheckMul.");
    return base::CheckAdd(sizeof(StringImpl),
                          base::CheckMul(length, sizeof(CharType)))
        .ValueOrDie();
  }

  template <typename CharType>
  ALWAYS_INLINE base::span<CharType> CharacterBuffer() {
    // SAFETY: The AllocationSize<CharType>() helper function computes a size
    // that includes `length_` UChar/LChar characters in addition to the size
    // required for the StringImpl.
    return UNSAFE_BUFFERS(
        base::span(reinterpret_cast<CharType*>(this + 1), length_));
  }
  template <typename CharType>
  ALWAYS_INLINE base::span<const CharType> CharacterBuffer() const {
    return const_cast<StringImpl*>(this)->CharacterBuffer<CharType>();
  }

  template <typename DestCharType,
            typename SrcCharType,
            typename ReplacementCharType>
  void DoReplace(base::span<const SrcCharType> source,
                 UChar pattern,
                 base::span<const ReplacementCharType> replacement,
                 base::span<DestCharType> dest) const;
  template <typename DestCharType>
  void DoReplace(const StringView& pattern,
                 const StringView& replacement,
                 base::span<DestCharType> dest) const;

  template <class UCharPredicate>
  scoped_refptr<StringImpl> StripMatchedCharacters(UCharPredicate);
  template <typename CharType, class UCharPredicate>
  scoped_refptr<StringImpl> SimplifyMatchedCharactersToSpace(
      base::span<const CharType>,
      UCharPredicate,
      StripBehavior);
  NOINLINE wtf_size_t HashSlowCase() const;

  void DestroyIfNeeded() const;

  // Calculates the kContainsOnlyAscii and kIsLowerAscii flags. Returns
  // a bitfield with those 2 values.
  unsigned ComputeASCIIFlags() const;

#if DCHECK_IS_ON()
  std::string AsciiForDebugging() const;
#endif

  static wtf_size_t highest_static_string_length_;

#if DCHECK_IS_ON()
  void AssertHashIsCorrect() {
    DCHECK(HasHash());
    DCHECK_EQ(ExistingHash(), StringHasher::ComputeHashAndMaskTop8Bits(
                                  (const char*)Characters8(), length()));
  }
#endif

#if DCHECK_IS_ON()
  mutable std::atomic<unsigned> ref_count_change_count_{0};
#endif
  // TODO (crbug.com/1083392): Use base::AtomicRefCount.
  mutable std::atomic_uint32_t ref_count_{1};
  const unsigned length_;
  mutable std::atomic<uint32_t> hash_and_flags_;
};

template <>
ALWAYS_INLINE const LChar* StringImpl::GetCharacters<LChar>() const {
  return Characters8();
}

template <>
ALWAYS_INLINE const UChar* StringImpl::GetCharacters<UChar>() const {
  return Characters16();
}

// The following template specialization can be moved to the class declaration
// once we officially switch to C++17 (we need C++ DR727 to be implemented).
template <>
ALWAYS_INLINE size_t StringImpl::AllocationSize<LChar>(wtf_size_t length) {
  static_assert(sizeof(LChar) == 1, "sizeof(LChar) should be 1.");
  return base::CheckAdd(sizeof(StringImpl), length).ValueOrDie();
}

// EqualToCString() can be faster than operator== because operator== creates
// a StringView, and it requires strlen(latin1).
//
// `latin1` must not be nullptr, and should point Latin-1 characters.
WTF_EXPORT bool EqualToCString(const StringView& a, const char* latin1);
WTF_EXPORT bool EqualToCString(const StringImpl* a, const char* latin1);

WTF_EXPORT bool Equal(const StringImpl*, const StringImpl*);
WTF_EXPORT bool Equal(const StringImpl*, base::span<const LChar>);
WTF_EXPORT bool Equal(const StringImpl*, base::span<const UChar>);
inline bool Equal(const StringImpl* a, base::span<const char> b) {
  return Equal(a, base::as_bytes(b));
}
WTF_EXPORT bool EqualNonNull(const StringImpl* a, const StringImpl* b);

ALWAYS_INLINE bool StringImpl::ContainsOnlyASCIIOrEmpty() const {
  uint32_t flags = hash_and_flags_.load(std::memory_order_relaxed);
  if (flags & kAsciiPropertyCheckDone)
    return flags & kContainsOnlyAscii;
  return ComputeASCIIFlags() & kContainsOnlyAscii;
}

ALWAYS_INLINE bool StringImpl::IsLowerASCII() const {
  uint32_t flags = hash_and_flags_.load(std::memory_order_relaxed);
  if (flags & kAsciiPropertyCheckDone)
    return flags & kIsLowerAscii;
  return ComputeASCIIFlags() & kIsLowerAscii;
}

// Unicode aware case insensitive string matching. Non-ASCII characters might
// match to ASCII characters. These functions are rarely used to implement web
// platform features.
// These functions are deprecated. Use EqualIgnoringASCIICase(), or introduce
// EqualIgnoringUnicodeCase(). See crbug.com/627682
WTF_EXPORT bool DeprecatedEqualIgnoringCase(base::span<const LChar>,
                                            base::span<const LChar>);
WTF_EXPORT bool DeprecatedEqualIgnoringCase(base::span<const UChar>,
                                            base::span<const LChar>);
inline bool DeprecatedEqualIgnoringCase(base::span<const LChar> a,
                                        base::span<const UChar> b) {
  return DeprecatedEqualIgnoringCase(b, a);
}
WTF_EXPORT bool DeprecatedEqualIgnoringCase(base::span<const UChar>,
                                            base::span<const UChar>);

WTF_EXPORT bool EqualIgnoringNullity(StringImpl*, StringImpl*);

template <typename CharacterTypeA, typename CharacterTypeB>
inline bool EqualIgnoringASCIICase(base::span<const CharacterTypeA> a,
                                   base::span<const CharacterTypeB> b) {
  CHECK_EQ(a.size(), b.size());
  size_t length = a.size();
  const CharacterTypeA* a_data = a.data();
  const CharacterTypeB* b_data = b.data();
  while (length--) {
    // Avoid base::span::operator[] for better performance.
    // SAFETY: This function ensures a_data and b_data move inside their spans.
    if (UNSAFE_BUFFERS(ToASCIILower(*a_data++) != ToASCIILower(*b_data++))) {
      return false;
    }
  }
  return true;
}

WTF_EXPORT int CodeUnitCompareIgnoringASCIICase(const StringImpl*,
                                                const StringImpl*);
WTF_EXPORT int CodeUnitCompareIgnoringASCIICase(const StringImpl*,
                                                const LChar*);

template <typename CharType>
inline wtf_size_t Find(base::span<const CharType> characters,
                       CharType match_character,
                       wtf_size_t index = 0) {
  if (index >= characters.size()) {
    return kNotFound;
  }
  // Pass raw pointers to std::find for better performance.
  const CharType* begin = base::to_address(characters.begin());
  const CharType* end = base::to_address(characters.end());
  const CharType* it = std::find(base::to_address(characters.begin() + index),
                                 end, match_character);
  return it == end ? kNotFound : std::distance(begin, it);
}

ALWAYS_INLINE wtf_size_t Find(base::span<const UChar> characters,
                              LChar match_character,
                              wtf_size_t index = 0) {
  return Find(characters, static_cast<UChar>(match_character), index);
}

inline wtf_size_t Find(base::span<const LChar> characters,
                       UChar match_character,
                       wtf_size_t index = 0) {
  if (match_character & ~0xFF)
    return kNotFound;
  return Find(characters, static_cast<LChar>(match_character), index);
}

template <typename CharacterType>
inline wtf_size_t Find(base::span<const CharacterType> characters,
                       char match_character,
                       wtf_size_t index = 0) {
  return Find(characters, static_cast<LChar>(match_character), index);
}

template <typename CharType>
inline wtf_size_t Find(base::span<const CharType> characters,
                       CharacterMatchFunctionPtr match_function,
                       wtf_size_t index = 0) {
  if (index >= characters.size()) {
    return kNotFound;
  }
  // Pass raw pointers to std::find_if for better performance.
  const CharType* begin = base::to_address(characters.begin());
  const CharType* end = base::to_address(characters.end());
  const CharType* it = std::find_if(
      base::to_address(characters.begin() + index), end, match_function);
  return it == end ? kNotFound : std::distance(begin, it);
}

// Search the `characters` span for `match_character` from the end of the span,
// and returns the found index or WTF::kNotFound.
//
// If the optional `index` parameter is specified, this function searches from
// characters[min(index, characters.size()-1)] to characters[0].
template <typename CharacterType>
inline wtf_size_t ReverseFind(base::span<const CharacterType> characters,
                              CharacterType match_character,
                              wtf_size_t index = UINT_MAX) {
  const size_t length = characters.size();
  if (!length)
    return kNotFound;
  if (index >= length)
    index = length - 1;
  const CharacterType* data = characters.data();
  // We don't use characters[index] for better performance.
  // SAFETY: The above code ensures `index` is less than characters.size().
  while (UNSAFE_BUFFERS(data[index]) != match_character) {
    if (!index--)
      return kNotFound;
  }
  return index;
}

ALWAYS_INLINE wtf_size_t ReverseFind(base::span<const UChar> characters,
                                     LChar match_character,
                                     wtf_size_t index = UINT_MAX) {
  return ReverseFind(characters, static_cast<UChar>(match_character), index);
}

inline wtf_size_t ReverseFind(base::span<const LChar> characters,
                              UChar match_character,
                              wtf_size_t index = UINT_MAX) {
  if (match_character & ~0xFF)
    return kNotFound;
  return ReverseFind(characters, static_cast<LChar>(match_character), index);
}

inline wtf_size_t StringImpl::Find(LChar character, wtf_size_t start) const {
  if (Is8Bit())
    return WTF::Find(Span8(), character, start);
  return WTF::Find(Span16(), character, start);
}

ALWAYS_INLINE wtf_size_t StringImpl::Find(char character,
                                          wtf_size_t start) const {
  return Find(static_cast<LChar>(character), start);
}

inline wtf_size_t StringImpl::Find(UChar character, wtf_size_t start) const {
  if (Is8Bit())
    return WTF::Find(Span8(), character, start);
  return WTF::Find(Span16(), character, start);
}

inline wtf_size_t LengthOfNullTerminatedString(const UChar* string) {
  size_t length = 0;
  while (string[length] != UChar(0))
    ++length;
  return base::checked_cast<wtf_size_t>(length);
}

template <wtf_size_t inlineCapacity>
bool EqualIgnoringNullity(const Vector<UChar, inlineCapacity>& a,
                          StringImpl* b) {
  if (!b)
    return !a.size();
  if (a.size() != b->length())
    return false;
  if (b->Is8Bit())
    return Equal(a.data(), b->Characters8(), b->length());
  return Equal(a.data(), b->Characters16(), b->length());
}

template <typename CharacterType1,
          typename CharacterType2,
          typename Projection = std::identity>
static inline int CodeUnitCompare(base::span<const CharacterType1> c1,
                                  base::span<const CharacterType2> c2,
                                  Projection proj = {}) {
  const size_t lmin = std::min(c1.size(), c2.size());
  size_t pos = 0;
  while (pos < lmin && proj(c1[pos]) == proj(c2[pos])) {
    ++pos;
  }
  if (pos < lmin) {
    return proj(c1[pos]) > proj(c2[pos]) ? 1 : -1;
  }
  if (c1.size() == c2.size()) {
    return 0;
  }
  return c1.size() > c2.size() ? 1 : -1;
}

static inline int CodeUnitCompare(const StringImpl* string1,
                                  const StringImpl* string2) {
  if (!string1)
    return (string2 && string2->length()) ? -1 : 0;

  if (!string2)
    return string1->length() ? 1 : 0;

  bool string1_is_8bit = string1->Is8Bit();
  bool string2_is_8bit = string2->Is8Bit();
  if (string1_is_8bit) {
    if (string2_is_8bit) {
      return CodeUnitCompare(string1->Span8(), string2->Span8());
    }
    return CodeUnitCompare(string1->Span8(), string2->Span16());
  }
  if (string2_is_8bit) {
    return -CodeUnitCompare(string2->Span8(), string1->Span16());
  }
  return CodeUnitCompare(string1->Span16(), string2->Span16());
}

inline scoped_refptr<StringImpl> StringImpl::IsolatedCopy() const {
  if (Is8Bit())
    return Create(Span8());
  return Create(Span16());
}

template <typename BufferType>
inline void StringImpl::AppendTo(BufferType& result,
                                 wtf_size_t start,
                                 wtf_size_t length) const {
  wtf_size_t number_of_characters_to_copy = std::min(length, length_ - start);
  if (!number_of_characters_to_copy)
    return;
  if (Is8Bit())
    result.AppendSpan(Span8().subspan(start, number_of_characters_to_copy));
  else
    result.AppendSpan(Span16().subspan(start, number_of_characters_to_copy));
}

template <typename T>
struct HashTraits;
// Defined in string_hash.h.
template <>
struct HashTraits<StringImpl*>;
template <>
struct HashTraits<scoped_refptr<StringImpl>>;

}  // namespace WTF

using WTF::Equal;
using WTF::EqualNonNull;
using WTF::kTextCaseASCIIInsensitive;
using WTF::kTextCaseSensitive;
using WTF::LengthOfNullTerminatedString;
using WTF::ReverseFind;
using WTF::StringImpl;
using WTF::TextCaseSensitivity;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_IMPL_H_
