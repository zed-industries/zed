// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SEQUENCE_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SEQUENCE_TEST_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_food_enum.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class V8UnionDoubleOrDoubleSequence;

class SequenceTest final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SequenceTest();
  ~SequenceTest() override;

  Vector<Vector<String>> identityByteStringSequenceSequence(
      const Vector<Vector<String>>& arg) const;
  Vector<double> identityDoubleSequence(const Vector<double>& arg) const;
  Vector<V8FoodEnum> identityFoodEnumSequence(
      const Vector<V8FoodEnum>& arg) const;
  Vector<int32_t> identityLongSequence(const Vector<int32_t>& arg) const;
  std::optional<Vector<uint8_t>> identityOctetSequenceOrNull(
      const std::optional<Vector<uint8_t>>& arg) const;

  HeapVector<Member<Element>> getElementSequence() const;
  void setElementSequence(const HeapVector<Member<Element>>& arg);
  void setElementSequenceOfSequences(
      const HeapVector<HeapVector<Member<Element>>>& arg);

  bool unionReceivedSequence(const V8UnionDoubleOrDoubleSequence* arg);

  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<Element>> element_sequence_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SEQUENCE_TEST_H_
