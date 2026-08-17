/*
 *  Copyright (C) 2003, 2006 Apple Computer, Inc.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *  MA 02110-1301 USA
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOM_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOM_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Document;
class LocalDOMWindow;
class ScriptState;
class V8SupportedType;

class CORE_EXPORT DOMParser final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMParser* Create(ScriptState* script_state) {
    return MakeGarbageCollected<DOMParser>(script_state);
  }

  explicit DOMParser(ScriptState*);

  Document* parseFromString(const WTF::String&, const V8SupportedType& type);

  void Trace(Visitor*) const override;

  LocalDOMWindow* GetWindow() const { return window_.Get(); }

 private:
  WeakMember<LocalDOMWindow> window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOM_PARSER_H_
