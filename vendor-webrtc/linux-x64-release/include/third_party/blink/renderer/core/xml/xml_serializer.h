/*
 *  Copyright (C) 2003, 2006 Apple Computer, Inc.
 *  Copyright (C) 2006 Samuel Weinig (sam@webkit.org)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XML_SERIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XML_SERIALIZER_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Node;

class XMLSerializer final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XMLSerializer* Create() {
    return MakeGarbageCollected<XMLSerializer>();
  }

  XMLSerializer() = default;

  WTF::String serializeToString(Node*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XML_SERIALIZER_H_
