// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_STRING_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_STRING_RESOURCE_H_

#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8.h"

namespace blink {

// StringResource is a helper class for V8ExternalString. It is used
// to manage the life-cycle of the underlying buffer of the external string.
class StringResourceBase {
  USING_FAST_MALLOC(StringResourceBase);

 public:
  explicit StringResourceBase(v8::Isolate* isolate, String string)
      : plain_string_(std::move(string)) {
    DCHECK(!plain_string_.IsNull());
    memory_accounter_.Increase(isolate, plain_string_.CharactersSizeInBytes());
  }

  explicit StringResourceBase(v8::Isolate* isolate, AtomicString string)
      : atomic_string_(std::move(string)) {
    DCHECK(!atomic_string_.IsNull());
    memory_accounter_.Increase(isolate, atomic_string_.CharactersSizeInBytes());
  }

  explicit StringResourceBase(v8::Isolate* isolate, ParkableString string)
      : parkable_string_(string) {
    // TODO(lizeb): This is only true without compression.
    DCHECK(!parkable_string_.IsNull());
    memory_accounter_.Increase(isolate,
                               parkable_string_.CharactersSizeInBytes());
  }

  StringResourceBase(const StringResourceBase&) = delete;
  StringResourceBase& operator=(const StringResourceBase&) = delete;

  void Unaccount(v8::Isolate* isolate) {
    int64_t reduced_external_memory = 0;
    if (!parkable_string_.IsNull()) {
      DCHECK(plain_string_.IsNull());
      DCHECK(atomic_string_.IsNull());
      reduced_external_memory = parkable_string_.CharactersSizeInBytes();
    } else {
      reduced_external_memory = plain_string_.CharactersSizeInBytes();
      if (plain_string_.Impl() != atomic_string_.Impl() &&
          !atomic_string_.IsNull()) {
        reduced_external_memory += atomic_string_.CharactersSizeInBytes();
      }
    }
    memory_accounter_.Decrease(isolate, reduced_external_memory);
  }

  size_t EstimateMemoryUsage() const { return sizeof(*this); }

  void EstimateSharedMemoryUsage(
      v8::String::ExternalStringResourceBase::SharedMemoryUsageRecorder*
          recorder) const {
    if (ParkableStringImpl* parkable_impl = parkable_string_.Impl()) {
      // The ParkableStringImpl may have a pointer to a ref-counted StringImpl,
      // which could also be referred to from elsewhere, so the two impls should
      // be recorded separately.
      ParkableStringImpl::MemoryUsage usage =
          parkable_impl->MemoryUsageForSnapshot();
      recorder->RecordSharedMemoryUsage(parkable_impl, usage.this_size);
      if (usage.string_impl) {
        recorder->RecordSharedMemoryUsage(usage.string_impl,
                                          usage.string_impl_size);
      }
    }
    if (StringImpl* plain_impl = plain_string_.Impl()) {
      recorder->RecordSharedMemoryUsage(
          plain_impl,
          sizeof(*plain_impl) + plain_impl->CharactersSizeInBytes());
    }
    if (StringImpl* atomic_impl = atomic_string_.Impl()) {
      recorder->RecordSharedMemoryUsage(
          atomic_impl,
          sizeof(*atomic_impl) + atomic_impl->CharactersSizeInBytes());
    }
  }

  virtual ~StringResourceBase() = default;

  String GetWTFString() {
    if (!parkable_string_.IsNull()) {
      DCHECK(plain_string_.IsNull());
      DCHECK(atomic_string_.IsNull());
      return parkable_string_.ToString();
    }
    return String(GetStringImpl());
  }

  AtomicString GetAtomicString(v8::Isolate* isolate) {
    if (!parkable_string_.IsNull()) {
      DCHECK(plain_string_.IsNull());
      DCHECK(atomic_string_.IsNull());
      return AtomicString(parkable_string_.ToString());
    }
    if (atomic_string_.IsNull()) {
      atomic_string_ = AtomicString(plain_string_);
      DCHECK(!atomic_string_.IsNull());
      if (plain_string_.Impl() != atomic_string_.Impl()) {
        memory_accounter_.Increase(isolate,
                                   atomic_string_.CharactersSizeInBytes());
      }
    }
    return atomic_string_;
  }

 protected:
  StringImpl* GetStringImpl() const {
    if (!plain_string_.IsNull())
      return plain_string_.Impl();
    DCHECK(!atomic_string_.IsNull());
    return atomic_string_.Impl();
  }

  const ParkableString& GetParkableString() const { return parkable_string_; }

  // Helper functions for derived constructors.
  template <typename Str>
  static inline Str Assert8Bit(Str&& str) {
    DCHECK(str.Is8Bit());
    return str;
  }

  template <typename Str>
  static inline Str Assert16Bit(Str&& str) {
    DCHECK(!str.Is8Bit());
    return str;
  }

 private:
  // If this StringResourceBase was initialized from a String then plain_string_
  // will be non-null. If the string becomes atomic later, the atomic version
  // of the string will be held in atomic_string_. When that happens, it is
  // necessary to keep the original string alive because v8 may keep derived
  // pointers into that string.
  // If this StringResourceBase was initialized from an AtomicString then
  // plain_string_ will be null and atomic_string_ will be non-null.
  String plain_string_;
  AtomicString atomic_string_;

  // If this string is parkable, its value is held here, and the other
  // members above are null.
  ParkableString parkable_string_;

  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase memory_accounter_;
};

// Even though StringResource{8,16}Base are effectively empty in release mode,
// they are needed as they serve as a common ancestor to Parkable and regular
// strings.
//
// See the comment in |ToBlinkString()|'s implementation for the rationale.
class StringResource16Base : public StringResourceBase,
                             public v8::String::ExternalStringResource {
 public:
  explicit StringResource16Base(v8::Isolate* isolate, String string)
      : StringResourceBase(isolate, Assert16Bit(std::move(string))) {}

  explicit StringResource16Base(v8::Isolate* isolate, AtomicString string)
      : StringResourceBase(isolate, Assert16Bit(std::move(string))) {}

  explicit StringResource16Base(v8::Isolate* isolate,
                                ParkableString parkable_string)
      : StringResourceBase(isolate, Assert16Bit(std::move(parkable_string))) {}

  StringResource16Base(const StringResource16Base&) = delete;
  StringResource16Base& operator=(const StringResource16Base&) = delete;

  void Unaccount(v8::Isolate* isolate) override {
    StringResourceBase::Unaccount(isolate);
  }

  size_t EstimateMemoryUsage() const override {
    return StringResourceBase::EstimateMemoryUsage();
  }

  void EstimateSharedMemoryUsage(
      SharedMemoryUsageRecorder* recorder) const override {
    return StringResourceBase::EstimateSharedMemoryUsage(recorder);
  }
};

class StringResource16 final : public StringResource16Base {
 public:
  explicit StringResource16(v8::Isolate* isolate, String string)
      : StringResource16Base(isolate, std::move(string)) {}

  explicit StringResource16(v8::Isolate* isolate, AtomicString string)
      : StringResource16Base(isolate, std::move(string)) {}

  StringResource16(const StringResource16&) = delete;
  StringResource16& operator=(const StringResource16&) = delete;

  size_t length() const override { return GetStringImpl()->length(); }
  const uint16_t* data() const override {
    return GetStringImpl()->SpanUint16().data();
  }
};

class ParkableStringResource16 final : public StringResource16Base {
 public:
  explicit ParkableStringResource16(v8::Isolate* isolate, ParkableString string)
      : StringResource16Base(isolate, std::move(string)) {}

  ParkableStringResource16(const ParkableStringResource16&) = delete;
  ParkableStringResource16& operator=(const ParkableStringResource16&) = delete;

  bool IsCacheable() const override {
    return !GetParkableString().may_be_parked();
  }

  void Lock() const override { GetParkableString().Lock(); }

  void Unlock() const override { GetParkableString().Unlock(); }

  size_t length() const override { return GetParkableString().length(); }

  const uint16_t* data() const override {
    return GetParkableString().SpanUint16().data();
  }
};

class StringResource8Base : public StringResourceBase,
                            public v8::String::ExternalOneByteStringResource {
 public:
  explicit StringResource8Base(v8::Isolate* isolate, String string)
      : StringResourceBase(isolate, Assert8Bit(std::move(string))) {}

  explicit StringResource8Base(v8::Isolate* isolate, AtomicString string)
      : StringResourceBase(isolate, Assert8Bit(std::move(string))) {}

  explicit StringResource8Base(v8::Isolate* isolate,
                               ParkableString parkable_string)
      : StringResourceBase(isolate, Assert8Bit(std::move(parkable_string))) {}

  StringResource8Base(const StringResource8Base&) = delete;
  StringResource8Base& operator=(const StringResource8Base&) = delete;

  void Unaccount(v8::Isolate* isolate) override {
    StringResourceBase::Unaccount(isolate);
  }

  size_t EstimateMemoryUsage() const override {
    return StringResourceBase::EstimateMemoryUsage();
  }

  void EstimateSharedMemoryUsage(
      SharedMemoryUsageRecorder* recorder) const override {
    return StringResourceBase::EstimateSharedMemoryUsage(recorder);
  }
};

class StringResource8 final : public StringResource8Base {
 public:
  explicit StringResource8(v8::Isolate* isolate, String string)
      : StringResource8Base(isolate, std::move(string)) {}

  explicit StringResource8(v8::Isolate* isolate, AtomicString string)
      : StringResource8Base(isolate, std::move(string)) {}

  StringResource8(const StringResource8&) = delete;
  StringResource8& operator=(const StringResource8&) = delete;

  size_t length() const override { return GetStringImpl()->length(); }
  const char* data() const override {
    return base::as_chars(GetStringImpl()->Span8()).data();
  }
};

class ParkableStringResource8 final : public StringResource8Base {
 public:
  explicit ParkableStringResource8(v8::Isolate* isolate, ParkableString string)
      : StringResource8Base(isolate, std::move(string)) {}

  ParkableStringResource8(const ParkableStringResource8&) = delete;
  ParkableStringResource8& operator=(const ParkableStringResource8&) = delete;

  bool IsCacheable() const override {
    return !GetParkableString().may_be_parked();
  }

  void Lock() const override { GetParkableString().Lock(); }

  void Unlock() const override { GetParkableString().Unlock(); }

  size_t length() const override { return GetParkableString().length(); }

  const char* data() const override {
    return GetParkableString().SpanChar().data();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_STRING_RESOURCE_H_
