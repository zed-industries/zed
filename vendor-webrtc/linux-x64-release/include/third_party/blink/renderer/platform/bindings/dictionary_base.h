// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DICTIONARY_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DICTIONARY_BASE_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8-forward.h"
#include "v8/include/v8-local-handle.h"

namespace blink {

class ScriptState;

namespace bindings {

// InputDictionaryBase is the common base class for all dictionaries. The ones
// that also support conversion to Objects are inheriting DictionaryBase. Most
// importantly these classes provide a way to differentiate dictionary types in
// template specializations (i.e. are being used for constraints).
class PLATFORM_EXPORT InputDictionaryBase
    : public GarbageCollected<InputDictionaryBase> {
 public:
  InputDictionaryBase(const InputDictionaryBase&) = delete;
  InputDictionaryBase(const InputDictionaryBase&&) = delete;
  InputDictionaryBase& operator=(const InputDictionaryBase&) = delete;
  InputDictionaryBase& operator=(const InputDictionaryBase&&) = delete;

  virtual ~InputDictionaryBase() = default;
  virtual void Trace(Visitor*) const {}

 protected:
  InputDictionaryBase() = default;
};

// A dictionary that supports conversion from from and to script value.
// See InputDictionaryBase for additional context.
class PLATFORM_EXPORT DictionaryBase : public InputDictionaryBase {
 public:
  ~DictionaryBase() override = default;

  v8::Local<v8::Value> ToV8(ScriptState* script_state) const;

 protected:
  DictionaryBase() = default;

  virtual const void* TemplateKey() const = 0;
  virtual void FillTemplateProperties(
      WTF::Vector<std::string_view>& properties) const = 0;
  virtual v8::Local<v8::Object> FillValues(
      ScriptState* script_state,
      v8::Local<v8::DictionaryTemplate> dict_template) const = 0;
};

}  // namespace bindings
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DICTIONARY_BASE_H_
