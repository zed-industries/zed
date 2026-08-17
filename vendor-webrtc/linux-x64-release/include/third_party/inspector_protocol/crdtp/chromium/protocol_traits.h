// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CRDTP_CHROMIUM_PROTOCOL_TYPE_TRAITS_H_
#define CRDTP_CHROMIUM_PROTOCOL_TYPE_TRAITS_H_

#include <string_view>
#include <vector>

#include "base/containers/span.h"
#include "base/memory/ref_counted_memory.h"
#include "base/memory/scoped_refptr.h"
#include "base/values.h"
#include "third_party/inspector_protocol/crdtp/protocol_core.h"
#include "third_party/inspector_protocol/crdtp/serializable.h"

namespace base {
class Value;
}

namespace crdtp {
class Serializable;

namespace traits {

// TODO(caseq): these should eventually be replaced with configurable
// types in protocol_config.json, so we can generate code with direct
// types rather than aliases, but for the time being, this is our way
// to specify the type mapping to the rest of the generated code.
using String = std::string;
using Value = base::Value;
using DictionaryValue = base::Value::Dict;
using ListValue = base::Value::List;

}  // namespace traits

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<std::string> {
  static bool Deserialize(DeserializerState* state, std::string* value);
  static void Serialize(const std::string& value, std::vector<uint8_t>* bytes);
};

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<base::Value> {
  static bool Deserialize(DeserializerState* state, base::Value* value);
  static void Serialize(const base::Value& value, std::vector<uint8_t>* bytes);
};

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<traits::DictionaryValue> {
  static bool Deserialize(DeserializerState* state,
                          traits::DictionaryValue* value);
  static void Serialize(const traits::DictionaryValue& value,
                        std::vector<uint8_t>* bytes);
};

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<traits::ListValue> {
  static bool Deserialize(DeserializerState* state, traits::ListValue* value);
  static void Serialize(const traits::ListValue& value,
                        std::vector<uint8_t>* bytes);
};

template <typename T>
struct UniquePtrTraitsHelper {
  static bool Deserialize(DeserializerState* state, std::unique_ptr<T>* value) {
    auto res = std::make_unique<T>();
    if (!ProtocolTypeTraits<T>::Deserialize(state, res.get()))
      return false;
    *value = std::move(res);
    return true;
  }
  static void Serialize(const std::unique_ptr<T>& value,
                        std::vector<uint8_t>* bytes) {
    ProtocolTypeTraits<T>::Serialize(*value, bytes);
  }
};

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<std::unique_ptr<traits::DictionaryValue>>
    : public UniquePtrTraitsHelper<traits::DictionaryValue> {};

// TODO(caseq): get rid of this, make sure Value is stored directly.
template <>
struct CRDTP_EXPORT ProtocolTypeTraits<std::unique_ptr<base::Value>>
    : public UniquePtrTraitsHelper<base::Value> {};

// A read-only sequence of uninterpreted bytes with reference-counted storage.
class CRDTP_EXPORT Binary : public Serializable {
 public:
  Binary(const Binary&);
  Binary();
  ~Binary() override;

  // Implements Serializable.
  void AppendSerialized(std::vector<uint8_t>* out) const override;

  // Allow explicit conversion to `base::span`.
  const uint8_t* data() const { return bytes_->data(); }
  size_t size() const { return bytes_->size(); }
  auto begin() const { return bytes_->begin(); }
  auto end() const { return bytes_->end(); }
  // data()/size() provide access to Binary's data as a span, but each one
  // requires a virtual call. Like RefCountedData, provide this operator as an
  // optimization.
  explicit operator base::span<const uint8_t>() const {
    return base::span(*bytes_);
  }
  scoped_refptr<base::RefCountedMemory> bytes() const { return bytes_; }

  std::string toBase64() const;

  static Binary fromBase64(std::string_view base64, bool* success);
  static Binary fromRefCounted(scoped_refptr<base::RefCountedMemory> memory);
  static Binary fromVector(std::vector<uint8_t> data);
  static Binary fromString(std::string data);
  static Binary fromSpan(base::span<const uint8_t> data);

 private:
  explicit Binary(scoped_refptr<base::RefCountedMemory> bytes);
  scoped_refptr<base::RefCountedMemory> bytes_;
};

template <>
struct CRDTP_EXPORT ProtocolTypeTraits<Binary> {
  static bool Deserialize(DeserializerState* state, Binary* value);
  static void Serialize(const Binary& value, std::vector<uint8_t>* bytes);
};

}  // namespace crdtp

#endif  // CRDTP_CHROMIUM_PROTOCOL_TYPE_TRAITS_H_
