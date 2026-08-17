// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINT_JOB_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINT_JOB_H_

#include "third_party/blink/public/mojom/printing/web_printing.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExecutionContext;
class WebPrintJobAttributes;

class MODULES_EXPORT WebPrintJob
    : public EventTarget,
      public ActiveScriptWrappable<WebPrintJob>,
      public ExecutionContextClient,
      public mojom::blink::WebPrintJobStateObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  WebPrintJob(ExecutionContext* execution_context,
              mojom::blink::WebPrintJobInfoPtr print_job_info);
  ~WebPrintJob() override;

  // Web-exposed interfaces:
  WebPrintJobAttributes* attributes() const { return attributes_; }
  void cancel();
  DEFINE_ATTRIBUTE_EVENT_LISTENER(jobstatechange, kJobstatechange)

  // EventTarget:
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;
  void Trace(Visitor* visitor) const override;

  // ActiveScriptWrappable:
  bool HasPendingActivity() const override;

  // WebPrintJobStateObserver:
  void OnWebPrintJobUpdate(mojom::blink::WebPrintJobUpdatePtr update) override;

 private:
  bool cancel_called_ = false;

  Member<WebPrintJobAttributes> attributes_;

  HeapMojoReceiver<mojom::blink::WebPrintJobStateObserver, WebPrintJob>
      observer_;
  HeapMojoRemote<mojom::blink::WebPrintJobController> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINT_JOB_H_
