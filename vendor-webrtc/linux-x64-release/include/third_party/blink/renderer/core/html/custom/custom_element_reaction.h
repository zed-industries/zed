// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class CustomElementDefinition;
class Element;

class CORE_EXPORT CustomElementReaction
    : public GarbageCollected<CustomElementReaction> {
 public:
  CustomElementReaction(CustomElementDefinition&);
  CustomElementReaction(const CustomElementReaction&) = delete;
  CustomElementReaction& operator=(const CustomElementReaction&) = delete;
  virtual ~CustomElementReaction() = default;

  virtual void Invoke(Element&) = 0;

  virtual void Trace(Visitor*) const;

 protected:
  Member<CustomElementDefinition> definition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_H_
