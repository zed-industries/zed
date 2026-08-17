/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_DATE_TIME_CHOOSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_DATE_TIME_CHOOSER_H_

#include "third_party/blink/public/mojom/choosers/date_time_chooser.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/date_time_chooser.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class DateTimeChooserClient;
class LocalFrame;
class Element;

class CORE_EXPORT ExternalDateTimeChooser final : public DateTimeChooser {
 public:
  explicit ExternalDateTimeChooser(DateTimeChooserClient*);
  ~ExternalDateTimeChooser() override;
  void Trace(Visitor*) const override;

  // |frame| must not be null.
  void OpenDateTimeChooser(LocalFrame* frame, const DateTimeChooserParameters&);

  void ResponseHandler(bool success, double dialog_value);

  bool IsShowingDateTimeChooserUI() const;

  // DateTimeChooser function:
  void EndChooser() override;
  AXObject* RootAXObject(Element* popup_owner) override;
  bool IsPickerVisible() const override;

 private:
  void DidChooseValue(double);
  void DidCancelChooser();

  mojom::blink::DateTimeChooser& GetDateTimeChooser(LocalFrame* frame);

  HeapMojoRemote<mojom::blink::DateTimeChooser> date_time_chooser_;

  Member<DateTimeChooserClient> client_;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_DATE_TIME_CHOOSER_H_
