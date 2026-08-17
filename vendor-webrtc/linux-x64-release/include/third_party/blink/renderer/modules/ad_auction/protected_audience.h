// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_PROTECTED_AUDIENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_PROTECTED_AUDIENCE_H_

#include <utility>
#include <variant>

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class ScriptState;

class ProtectedAudience : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using FeatureVal = std::variant<bool, size_t, double>;

  // `execution_context` is only used by the constructor.
  explicit ProtectedAudience(ExecutionContext* execution_context);
  ScriptValue queryFeatureSupport(ScriptState* script_state,
                                  const String& feature_name);

 private:
  // As the number of entries grows, an actual dictionary type may become
  // appropriate.
  WTF::Vector<std::pair<String, FeatureVal>> feature_status_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_PROTECTED_AUDIENCE_H_
