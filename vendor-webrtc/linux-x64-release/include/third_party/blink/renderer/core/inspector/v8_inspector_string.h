// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_V8_INSPECTOR_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_V8_INSPECTOR_STRING_H_

#include <memory>
#include <vector>

#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/inspector_protocol/crdtp/protocol_core.h"
#include "third_party/inspector_protocol/crdtp/serializable.h"
#include "v8/include/v8-inspector.h"
#include "v8/include/v8-script.h"

namespace blink {

// Note that passed string must outlive the resulting StringView. This implies
// it must not be a temporary object.
CORE_EXPORT v8_inspector::StringView ToV8InspectorStringView(const StringView&);

CORE_EXPORT std::unique_ptr<v8_inspector::StringBuffer>
ToV8InspectorStringBuffer(const StringView&);
CORE_EXPORT String ToCoreString(const v8_inspector::StringView&);
CORE_EXPORT String ToCoreString(std::unique_ptr<v8_inspector::StringBuffer>);

namespace protocol {
using String = WTF::String;

class CORE_EXPORT StringUtil {
  STATIC_ONLY(StringUtil);

 public:
  static String fromUTF8(const uint8_t* data, size_t length);
  static String fromUTF16LE(const uint16_t* data, size_t length);

  static const uint8_t* CharactersLatin1(const String& s) {
    if (!s.Is8Bit())
      return nullptr;
    return UNSAFE_TODO(s.Characters8());
  }
  static const uint8_t* CharactersUTF8(const String& s) { return nullptr; }
  static const uint16_t* CharactersUTF16(const String& s) {
    if (s.Is8Bit())
      return nullptr;
    return reinterpret_cast<const uint16_t*>(UNSAFE_TODO(s.Characters16()));
  }
  static size_t CharacterCount(const String& s) { return s.length(); }
};

// A read-only sequence of uninterpreted bytes with reference-counted storage.
class CORE_EXPORT Binary : public crdtp::Serializable {
 public:
  class Impl : public RefCounted<Impl> {
   public:
    using iterator = base::CheckedContiguousIterator<const uint8_t>;

    Impl() = default;
    virtual ~Impl() = default;

    virtual const uint8_t* data() const = 0;
    virtual size_t size() const = 0;

    // Iterators, so this type meets the requirements of
    // `std::ranges::contiguous_range`.
    iterator begin() const {
      // SAFETY: `data()` points to at least `size()` valid bytes, so the
      // computed value here is no further than just-past-the-end of the
      // allocation.
      return UNSAFE_BUFFERS(iterator(data(), data() + size()));
    }
    iterator end() const {
      // SAFETY: As in `begin()` above.
      return UNSAFE_BUFFERS(iterator(data(), data() + size(), data() + size()));
    }
  };

  Binary() = default;

  // Implements Serializable.
  void AppendSerialized(std::vector<uint8_t>* out) const override;

  const uint8_t* data() const { return impl_ ? impl_->data() : nullptr; }
  size_t size() const { return impl_ ? impl_->size() : 0; }
  base::span<const uint8_t> Span() const {
    // SAFETY: Safety relies on data() and size() of Impl class.
    return UNSAFE_BUFFERS(base::span(data(), size()));
  }

  String toBase64() const;
  static Binary fromBase64(const String& base64, bool* success);
  static Binary fromVector(Vector<uint8_t> in);
  static Binary fromSpan(base::span<const uint8_t> data);

  // Note: |data.buffer_policy| must be
  // ScriptCompiler::ScriptCompiler::CachedData::BufferOwned.
  static Binary fromCachedData(
      std::unique_ptr<v8::ScriptCompiler::CachedData> data);

 private:
  explicit Binary(scoped_refptr<Impl> impl) : impl_(std::move(impl)) {}
  scoped_refptr<Impl> impl_;
};
}  // namespace protocol

}  // namespace blink

// TODO(dgozman): migrate core/inspector/protocol to wtf::HashMap.

// See third_party/inspector_protocol/crdtp/serializer_traits.h.
namespace crdtp {

template <>
struct ProtocolTypeTraits<WTF::String> {
  static bool Deserialize(DeserializerState* state, String* value);
  static void Serialize(const String& value, std::vector<uint8_t>* bytes);
};

template <>
struct ProtocolTypeTraits<blink::protocol::Binary> {
  static bool Deserialize(DeserializerState* state,
                          blink::protocol::Binary* value);
  static void Serialize(const blink::protocol::Binary& value,
                        std::vector<uint8_t>* bytes);
};

}  // namespace crdtp

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_V8_INSPECTOR_STRING_H_
