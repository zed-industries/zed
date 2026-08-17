// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_MANAGER_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class PreferenceObject;

// Spec: https://wicg.github.io/web-preferences-api/#preference-manager
class PreferenceManager final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PreferenceManager(ExecutionContext*);
  ~PreferenceManager() override;

  PreferenceObject* colorScheme();
  PreferenceObject* contrast();
  PreferenceObject* reducedMotion();
  PreferenceObject* reducedTransparency();
  PreferenceObject* reducedData();

  void PreferenceMaybeChanged();

  void Trace(Visitor*) const override;

 private:
  Member<PreferenceObject> color_scheme_;
  Member<PreferenceObject> contrast_;
  Member<PreferenceObject> reduced_motion_;
  Member<PreferenceObject> reduced_transparency_;
  Member<PreferenceObject> reduced_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_MANAGER_H_
