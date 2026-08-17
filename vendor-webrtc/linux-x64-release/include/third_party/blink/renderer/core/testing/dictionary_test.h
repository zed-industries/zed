// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DICTIONARY_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DICTIONARY_TEST_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "v8/include/v8.h"

namespace blink {

class InternalDictionary;
class InternalDictionaryDerived;
class InternalDictionaryDerivedDerived;

class DictionaryTest : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DictionaryTest();
  ~DictionaryTest() override;

  // Saves the given InternalDictionary in this object.
  void set(v8::Isolate* isolate, const InternalDictionary* input_dictionary);
  // Returns an InternalDictionary with filled with the saved dictionary
  // members.
  InternalDictionary* get(v8::Isolate* isolate);

  void setDerived(v8::Isolate* isolate,
                  const InternalDictionaryDerived* input_dictionary);
  InternalDictionaryDerived* getDerived(v8::Isolate* isolate);

  void setDerivedDerived(
      v8::Isolate* isolate,
      const InternalDictionaryDerivedDerived* input_dictionary);
  InternalDictionaryDerivedDerived* getDerivedDerived(v8::Isolate* isolate);

  void Trace(Visitor*) const override;

 private:
  void RestoreInternalDictionary(InternalDictionary* output_dictionary);
  void RestoreInternalDictionaryDerived(
      InternalDictionaryDerived* output_dictionary);
  void RestoreInternalDictionaryDerivedDerived(
      InternalDictionaryDerivedDerived* output_dictionary);

  Member<InternalDictionaryDerivedDerived> dictionary_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DICTIONARY_TEST_H_
