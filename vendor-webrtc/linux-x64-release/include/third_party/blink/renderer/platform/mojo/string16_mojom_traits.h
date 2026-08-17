// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_STRING16_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_STRING16_MOJOM_TRAITS_H_

#include <string>

#include "base/containers/span.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "mojo/public/mojom/base/string16.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace mojo_base {
class BigBuffer;
}

namespace mojo {

// WTF::String stores string data as 8-bit strings if only Latin-1 characters
// are present. During Mojo serialization, this helper provides a scratch buffer
// that can be used for converting an 8-bit string to a 16-bit string.
class PLATFORM_EXPORT MaybeOwnedString16 {
 public:
  explicit MaybeOwnedString16(std::u16string owned_storage);
  explicit MaybeOwnedString16(base::span<const uint16_t> unowned);
  ~MaybeOwnedString16();

  const uint16_t* data() const { return unowned_.data(); }
  size_t size() const { return unowned_.size(); }

 private:
  std::u16string owned_storage_;
  // TODO(367764863) Rewrite to base::raw_span
  RAW_PTR_EXCLUSION base::span<const uint16_t> unowned_;
};

template <>
struct PLATFORM_EXPORT ArrayTraits<MaybeOwnedString16> {
  using Element = const uint16_t;

  static bool IsNull(const MaybeOwnedString16&) { return false; }
  static size_t GetSize(const MaybeOwnedString16& input) {
    return input.size();
  }
  static const Element* GetData(const MaybeOwnedString16& input) {
    return input.data();
  }
  static const Element& GetAt(const MaybeOwnedString16& input, size_t index) {
    return input.data()[index];
  }
};

template <>
struct PLATFORM_EXPORT
    StructTraits<mojo_base::mojom::String16DataView, WTF::String> {
  static bool IsNull(const WTF::String& input) { return input.IsNull(); }
  static void SetToNull(WTF::String* output) { *output = WTF::String(); }

  static MaybeOwnedString16 data(const WTF::String& input);

  static bool Read(mojo_base::mojom::String16DataView, WTF::String* out);
};

template <>
struct PLATFORM_EXPORT
    StructTraits<mojo_base::mojom::BigString16DataView, WTF::String> {
  static bool IsNull(const WTF::String& input) { return input.IsNull(); }
  static void SetToNull(WTF::String* output) { *output = WTF::String(); }

  static mojo_base::BigBuffer data(const WTF::String& input);
  static bool Read(mojo_base::mojom::BigString16DataView, WTF::String* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_STRING16_MOJOM_TRAITS_H_
