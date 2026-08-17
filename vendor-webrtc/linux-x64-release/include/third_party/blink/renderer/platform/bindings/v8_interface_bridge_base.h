// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_INTERFACE_BRIDGE_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_INTERFACE_BRIDGE_BASE_H_

#include "third_party/blink/public/mojom/origin_trials/origin_trial_feature.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-forward.h"

namespace blink {

class DOMWrapperWorld;

namespace bindings {

// The common base class of code-generated V8-Blink bridge class of IDL
// interfaces and namespaces.
class PLATFORM_EXPORT V8InterfaceBridgeBase {
  STATIC_ONLY(V8InterfaceBridgeBase);

 public:
  // Selects properties to be installed according to origin trial features.
  //
  // There are two usages.
  // 1) At the first call of V8T::InstallContextDependentProperties, install
  //   (a) properties that are not associated to origin trial features (e.g.
  //   secure contexts) plus (b) properties that are associated to origin trial
  //   features and are already enabled by the moment of the call.
  // 2) On 2nd+ call of V8T::InstallContextDependentProperties (when an origin
  //   trial feature gets enabled later on), install only (c) properties that
  //   are associated to the origin trial feature that has got enabled.
  //
  // FeatureSelector() is used for usage 1) and
  // FeatureSelector(feature) is used for usage 2).
  class PLATFORM_EXPORT FeatureSelector final {
   public:
    // Selects all properties not associated to any origin trial feature and
    // properties associated with the origin trial features that are already
    // enabled.
    FeatureSelector();
    // Selects only the properties that are associated to the given origin
    // trial feature.
    explicit FeatureSelector(blink::mojom::blink::OriginTrialFeature feature);
    FeatureSelector(const FeatureSelector&) = default;
    FeatureSelector(FeatureSelector&&) = default;
    ~FeatureSelector() = default;

    FeatureSelector& operator=(const FeatureSelector&) = default;
    FeatureSelector& operator=(FeatureSelector&&) = default;

    // Returns true if all properties that are associated with the features
    // enabled at this moment should be installed.
    bool IsAll() const { return does_select_all_; }

    // Returns true if properties should be installed.  Arguments |featureN|
    // represent the origin trial features to which the properties are
    // associated.
    bool IsAnyOf(blink::mojom::blink::OriginTrialFeature feature1) const {
      return selector_ == feature1;
    }
    bool IsAnyOf(blink::mojom::blink::OriginTrialFeature feature1,
                 blink::mojom::blink::OriginTrialFeature feature2) const {
      return selector_ == feature1 || selector_ == feature2;
    }

   private:
    bool does_select_all_ = false;
    // We intentionally avoid default member initializer for |selector_| in
    // order not to include runtime_enabled_features.h.
    blink::mojom::blink::OriginTrialFeature selector_;
  };

  using InstallInterfaceTemplateFuncType =
      void (*)(v8::Isolate* isolate,
               const DOMWrapperWorld& world,
               v8::Local<v8::Template> interface_template);
  using InstallUnconditionalPropertiesFuncType =
      void (*)(v8::Isolate* isolate,
               const DOMWrapperWorld& world,
               v8::Local<v8::ObjectTemplate> instance_template,
               v8::Local<v8::ObjectTemplate> prototype_template,
               v8::Local<v8::Template> interface_template);
  using InstallContextIndependentPropertiesFuncType =
      void (*)(v8::Isolate* isolate,
               const DOMWrapperWorld& world,
               v8::Local<v8::ObjectTemplate> instance_template,
               v8::Local<v8::ObjectTemplate> prototype_template,
               v8::Local<v8::Template> interface_template);
  using InstallContextDependentPropertiesFuncType =
      void (*)(v8::Local<v8::Context> context,
               const DOMWrapperWorld& world,
               v8::Local<v8::Object> instance_object,
               v8::Local<v8::Object> prototype_object,
               v8::Local<v8::Object> interface_object,
               v8::Local<v8::Template> interface_template,
               FeatureSelector feature_selector);
};

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_INTERFACE_BRIDGE_BASE_H_
