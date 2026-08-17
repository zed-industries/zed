/*
    Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_DOM_PLUGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_DOM_PLUGIN_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/plugins/dom_mime_type.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExceptionState;

class DOMPlugin final : public ScriptWrappable, public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DOMPlugin(LocalDOMWindow*, const PluginInfo&);

  String name() const;
  String filename() const;
  String description() const;

  unsigned length() const;

  DOMMimeType* item(unsigned index);
  DOMMimeType* namedItem(const AtomicString& property_name);
  void NamedPropertyEnumerator(Vector<String>&, ExceptionState&) const;
  bool NamedPropertyQuery(const AtomicString&, ExceptionState&) const;

  void Trace(Visitor*) const override;

 private:
  Member<const PluginInfo> plugin_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_DOM_PLUGIN_H_
