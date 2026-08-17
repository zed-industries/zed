// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/fileapi/blob.h"
#include "third_party/blink/renderer/modules/clipboard/clipboard_item.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class Navigator;
class ScriptState;
class ClipboardUnsanitizedFormats;

class Clipboard : public EventTarget, public Supplement<Navigator> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static Clipboard* clipboard(Navigator&);
  explicit Clipboard(Navigator&);

  Clipboard(const Clipboard&) = delete;
  Clipboard& operator=(const Clipboard&) = delete;

  ScriptPromise<IDLSequence<ClipboardItem>>
  read(ScriptState*, ClipboardUnsanitizedFormats* formats, ExceptionState&);
  ScriptPromise<IDLSequence<ClipboardItem>> read(
      ScriptState* script_state,
      ExceptionState& exception_state) {
    return read(script_state, nullptr, exception_state);
  }
  ScriptPromise<IDLString> readText(ScriptState*, ExceptionState&);

  ScriptPromise<IDLUndefined> write(ScriptState*,
                                    const HeapVector<Member<ClipboardItem>>&,
                                    ExceptionState&);
  ScriptPromise<IDLUndefined> writeText(ScriptState*,
                                        const String&,
                                        ExceptionState&);

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // Parses `format` as a web custom format type string. If successful, it
  // returns just the (normalized) MIME type without the "web " prefix;
  // otherwise returns an empty string.
  static String ParseWebCustomFormat(const String& format);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_H_
