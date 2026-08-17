// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OBJECT_H_

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Document;
class ExecutionContext;
template <typename IDLType>
class FrozenArray;
class MediaValues;

// Spec: https://wicg.github.io/web-preferences-api/#preferenceobject-interface
class CORE_EXPORT PreferenceObject final
    : public EventTarget,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PreferenceObject(ExecutionContext*, AtomicString name);

  ~PreferenceObject() override;

  AtomicString override(ScriptState*);

  AtomicString value(ScriptState*);

  void clearOverride(ScriptState*);

  ScriptPromise<IDLUndefined> requestOverride(ScriptState*,
                                              std::optional<AtomicString>);

  const FrozenArray<IDLString>& validValues();

  void PreferenceMaybeChanged();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  void Trace(Visitor* visitor) const override;

  // From ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

 private:
  AtomicString name_;
  Member<FrozenArray<IDLString>> valid_values_;
  Member<const MediaValues> media_values_;
  mojom::blink::PreferredColorScheme preferred_color_scheme_;
  mojom::blink::PreferredContrast preferred_contrast_;
  bool prefers_reduced_data_;
  bool prefers_reduced_motion_;
  bool prefers_reduced_transparency_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OBJECT_H_
