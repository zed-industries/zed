/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/public/mojom/page_state/page_state.mojom-blink.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class LocalDOMWindow;
class KURL;
class ExceptionState;
class HistoryItem;
class ScriptState;
class ScriptValue;
class V8ScrollRestoration;

// This class corresponds to the History interface.
class CORE_EXPORT History final : public ScriptWrappable,
                                  public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit History(LocalDOMWindow*);

  unsigned length(ExceptionState&) const;
  ScriptValue state(ScriptState*, ExceptionState&);

  void back(ScriptState*, ExceptionState&);
  void forward(ScriptState*, ExceptionState&);
  void go(ScriptState*, int delta, ExceptionState&);

  void pushState(ScriptState*,
                 const ScriptValue& data,
                 const String& title,
                 const String& url,
                 ExceptionState&);

  void replaceState(ScriptState*,
                    const ScriptValue& data,
                    const String& title,
                    const String& url,
                    ExceptionState& exception_state);

  void setScrollRestoration(const V8ScrollRestoration& value, ExceptionState&);
  V8ScrollRestoration scrollRestoration(ExceptionState&);

  bool IsSameAsCurrentState(SerializedScriptValue*) const;

  ScriptValue StateHelper(ScriptState*, ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  KURL UrlForState(const String& url);

  void StateObjectAdded(scoped_refptr<SerializedScriptValue>,
                        const String& title,
                        const String& url,
                        WebFrameLoadType,
                        ScriptState*,
                        ExceptionState&);
  SerializedScriptValue* StateInternal() const;
  mojom::blink::ScrollRestorationType ScrollRestorationInternal() const;
  HistoryItem* GetHistoryItem() const;

  scoped_refptr<SerializedScriptValue> last_state_object_requested_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_H_
