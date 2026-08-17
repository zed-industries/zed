/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"
#include "third_party/blink/renderer/core/fullscreen/fullscreen_request_type.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LocalDOMWindow;
class FullscreenOptions;

// Internal type used when checking RequestFullscreen conditions.
enum class RequestFullscreenError {
  kNone = 0,
  kElementTypeNotHTMLNorSVG,
  kElementTypeDialog,
  kElementNotConnected,
  kElementOpenAsPopover,
  kDisallowedByPermissionsPolicy,
  kFullscreenNotSupported,
  kPermissionCheckFailed,
  kDocumentIncorrect,
  kNotGranted,
};

// The Fullscreen class implements most of the Fullscreen API Standard,
// https://fullscreen.spec.whatwg.org/, especially its algorithms. It is a
// Document supplement as each document has some fullscreen state, and to
// actually enter and exit fullscreen it (indirectly) uses FullscreenController.
class CORE_EXPORT Fullscreen final : public GarbageCollected<Fullscreen>,
                                     public Supplement<LocalDOMWindow>,
                                     public ExecutionContextLifecycleObserver {
 public:
  static const char kSupplementName[];

  explicit Fullscreen(LocalDOMWindow&);
  ~Fullscreen() override;

  static Element* FullscreenElementFrom(Document&);
  static Element* FullscreenElementForBindingFrom(TreeScope&);
  // Returns true if the Element is the topmost element in its document's top
  // layer whose fullscreen flag is set.
  static bool IsFullscreenElement(const Element&);
  static bool IsInFullscreenElementStack(const Element&);
  static bool HasFullscreenElements();
  // Returns true if the Element's fullscreen flag is set. A Document may have
  // multiple elements with the fullscreen flag set.
  static bool IsFullscreenFlagSetFor(const Element&);

  static void RequestFullscreen(Element&);
  static ScriptPromise<IDLUndefined> RequestFullscreen(
      Element&,
      const FullscreenOptions*,
      FullscreenRequestType,
      ScriptState* state = nullptr,
      ExceptionState* exception_state = nullptr);

  static void FullyExitFullscreen(Document&, bool ua_originated = false);
  static ScriptPromise<IDLUndefined> ExitFullscreen(
      Document&,
      ScriptState* state = nullptr,
      ExceptionState* exception_state = nullptr,
      bool ua_originated = false);

  static bool FullscreenEnabled(
      Document&,
      ReportOptions report_on_failure = ReportOptions::kDoNotReport);

  // Called by FullscreenController to notify that we've entered or exited
  // fullscreen. All frames are notified, so there may be no pending request.
  static void DidResolveEnterFullscreenRequest(Document&, bool granted);
  static void DidExitFullscreen(Document&);

  static void DidUpdateSize(Element&);

  static void ElementRemoved(Element&);

  // ExecutionContextLifecycleObserver:
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

  base::TimeTicks block_automatic_fullscreen_until() const {
    return block_automatic_fullscreen_until_;
  }

 private:
  static Fullscreen& From(LocalDOMWindow&);

  // Run by RequestFullscreen to check conditions and invoke `callback` with any
  // error or `kNone` to proceed. The callback may be invoked asynchronously to
  // check permission when requests do not have transient user activation, etc.
  static void EnforceRequestFullscreenConditions(
      Element& pending,
      Document& document,
      base::OnceCallback<void(RequestFullscreenError)> callback);

  // Run after EnforceRequestFullscreenConditions checks for any `error`.
  static void ContinueRequestFullscreenAfterConditionsEnforcement(
      Element* pending,
      FullscreenRequestType request_type,
      const FullscreenOptions* options,
      ScriptPromiseResolver<IDLUndefined>* resolver,
      RequestFullscreenError error);

  static void ContinueRequestFullscreen(
      Document&,
      Element&,
      FullscreenRequestType,
      const FullscreenOptions*,
      ScriptPromiseResolver<IDLUndefined>* resolver,
      RequestFullscreenError error);

  static void ContinueExitFullscreen(
      Document*,
      ScriptPromiseResolver<IDLUndefined>* resolver,
      bool resize);

  void FullscreenElementChanged(Element* old_element,
                                Element* new_element,
                                FullscreenRequestType new_request_type);

  // Stores the pending request, promise and the type for executing
  // the asynchronous portion of the request.
  class PendingRequest : public GarbageCollected<PendingRequest> {
   public:
    PendingRequest(Element* element,
                   FullscreenRequestType type,
                   const FullscreenOptions* options,
                   ScriptPromiseResolver<IDLUndefined>* resolver);
    PendingRequest(const PendingRequest&) = delete;
    PendingRequest& operator=(const PendingRequest&) = delete;
    virtual ~PendingRequest();
    virtual void Trace(Visitor* visitor) const;

    Element* element() { return element_.Get(); }
    FullscreenRequestType type() { return type_; }
    const FullscreenOptions* options() { return options_.Get(); }
    ScriptPromiseResolver<IDLUndefined>* resolver() { return resolver_.Get(); }

   private:
    Member<Element> element_;
    FullscreenRequestType type_;
    Member<const FullscreenOptions> options_;
    Member<ScriptPromiseResolver<IDLUndefined>> resolver_;
  };
  using PendingRequests = HeapVector<Member<PendingRequest>>;
  PendingRequests pending_requests_;

  using PendingExit = ScriptPromiseResolver<IDLUndefined>;
  using PendingExits = HeapVector<Member<PendingExit>>;
  PendingExits pending_exits_;

  // Used to block automatic fullscreen for a short time after exit.
  base::TimeTicks block_automatic_fullscreen_until_;
};

inline bool Fullscreen::IsFullscreenElement(const Element& element) {
  if (HasFullscreenElements() &&
      FullscreenElementFrom(element.GetDocument()) == &element) [[unlikely]] {
    return true;
  }
  return false;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_FULLSCREEN_H_
