// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/style_property_map_read_only_main_thread.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class V8UnionCSSStyleValueOrString;

class CORE_EXPORT StylePropertyMap : public StylePropertyMapReadOnlyMainThread {
  DEFINE_WRAPPERTYPEINFO();

 public:
  StylePropertyMap(const StylePropertyMap&) = delete;
  StylePropertyMap& operator=(const StylePropertyMap&) = delete;

  void set(const ExecutionContext* execution_context,
           const String& property_name,
           const HeapVector<Member<V8UnionCSSStyleValueOrString>>& values,
           ExceptionState& exception_state);
  void append(const ExecutionContext* execution_context,
              const String& property_name,
              const HeapVector<Member<V8UnionCSSStyleValueOrString>>& values,
              ExceptionState& exception_state);
  void remove(const ExecutionContext*,
              const String& property_name,
              ExceptionState&);
  void clear();

 protected:
  virtual void SetProperty(CSSPropertyID, const CSSValue&) = 0;
  virtual bool SetShorthandProperty(CSSPropertyID,
                                    const String&,
                                    SecureContextMode) = 0;
  virtual void SetCustomProperty(const AtomicString&, const CSSValue&) = 0;
  virtual void RemoveProperty(CSSPropertyID) = 0;
  virtual void RemoveCustomProperty(const AtomicString&) = 0;
  virtual void RemoveAllProperties() = 0;

  StylePropertyMap() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_H_
