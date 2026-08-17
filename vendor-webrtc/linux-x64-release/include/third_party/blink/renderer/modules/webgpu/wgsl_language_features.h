// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_WGSL_LANGUAGE_FEATURES_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_WGSL_LANGUAGE_FEATURES_H_

#include <bitset>

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_wgsl_language_features.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class WGSLLanguageFeatures : public ScriptWrappable,
                             public ValueSyncIterable<WGSLLanguageFeatures> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit WGSLLanguageFeatures(
      const std::vector<wgpu::WGSLLanguageFeatureName>& features);

  const HashSet<String>& FeatureNameSet() const { return features_; }

  // wgsl_language_features.idl {{{
  bool hasForBinding(ScriptState* script_state,
                     const String& feature,
                     ExceptionState& exception_state) const;
  unsigned size() const { return features_.size(); }
  // }}} End of WebIDL binding implementation.

 private:
  HashSet<String> features_;

  class IterationSource final
      : public ValueSyncIterable<WGSLLanguageFeatures>::IterationSource {
   public:
    explicit IterationSource(const HashSet<String>& features);

    bool FetchNextItem(ScriptState* script_state,
                       String& value,
                       ExceptionState& exception_state) override;

   private:
    HashSet<String> features_;
    HashSet<String>::iterator iter_;
  };

  // Starts iteration over the Setlike.
  // Needed for ValueSyncIterable to work properly.
  WGSLLanguageFeatures::IterationSource* CreateIterationSource(
      ScriptState* script_state,
      ExceptionState& exception_state) override {
    return MakeGarbageCollected<WGSLLanguageFeatures::IterationSource>(
        features_);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_WGSL_LANGUAGE_FEATURES_H_
