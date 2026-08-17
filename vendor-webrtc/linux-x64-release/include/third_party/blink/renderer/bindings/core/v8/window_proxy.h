/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WINDOW_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WINDOW_PROXY_H_

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWindow;
class Frame;

// WindowProxy implements the split window model of a window for a frame. In the
// HTML standard, the split window model is composed of the Window interface
// (the inner global object) and the WindowProxy interface (the outer global
// proxy).
//
// The Window interface is backed by the Blink DOMWindow C++ implementation.
// In contrast, the WindowProxy interface does not have a corresponding
// C++ implementation in Blink: the WindowProxy class defined here only manages
// context initialization and detach. Instead, the behavior of the WindowProxy
// interface is defined by JSGlobalProxy in v8 and the prototype chain set up
// during context initialization.
//
// ====== Inner Global Object ======
// The inner global object is the global for the script environment of a Frame.
// Since Window and Document also have a 1:1 relationship, this means that each
// inner global object has an associated Document which does not change. On
// navigation, the new Document receives a new inner global object.
//
// However, there is one exception to the 1:1 DOMWindow:Document rule. If:
// - the previous Document is the initial empty document
// - the new Document is same-origin to the previous Document
// then the inner global object will be reused for the new Document. This is the
// only case where the associated Document of an inner global object can change.
//
// All methods and attributes defined on the Window interface are exposed via
// the inner global object. Global variables defined by script running in the
// Document also live on the inner global object.
//
// ====== Outer Global Proxy ====
// The outer global proxy is reused across navigations. It implements the
// security checks for same-origin/cross-origin access to the Window interface.
// When the check passes (i.e. the access is same-origin), the access is
// forwarded to the inner global object of the active Document in this
// WindowProxy's Frame.
//
// When the security check fails, the access is delegated to the outer global
// proxy's cross-origin interceptors. The cross-origin interceptors may choose
// to return a value (if the property is exposed cross-origin) or throw an
// exception otherwise.
//
// Note that the cross-origin interceptors are only used for cross-origin
// accesses: a same-origin access to a method that is available cross-origin,
// such as Window.postMessage, will be delegated to the inner global object.
//
// ====== LocalWindowProxy vs RemoteWindowProxy ======
// WindowProxy has two concrete subclasses:
// - LocalWindowProxy: implements the split window model for a frame in the same
//   process, i.e. a LocalFrame.
// - RemoteWindowProxy: implements the split window model for a frame in a
//   different process, i.e. a RemoteFrame.
//
// While having a RemoteFrame implies the frame must be cross-origin, the
// opposite is not true: a LocalFrame can be same-origin or cross-origin. One
// additional complexity (which slightly violates the HTML standard): it is
// possible to have SecurityOrigin::CanAccess() return true for a RemoteFrame's
// security origin; however, it is important to still deny access as if the
// frame were cross-origin. This is due to complexities in the process
// allocation model for renderer processes. See https://crbug.com/601629.
//
// ====== LocalWindowProxy ======
// Since a LocalWindowProxy can represent a same-origin or cross-origin frame,
// the entire prototype chain must be available:
//
//   outer global proxy
//     -- has prototype --> inner global object
//     -- has prototype --> Window.prototype
//     -- has prototype --> WindowProperties [1]
//     -- has prototype --> EventTarget.prototype
//     -- has prototype --> Object.prototype
//     -- has prototype --> null
//
// [1] WindowProperties is the named properties object of the Window interface.
//
// ====== RemoteWindowProxy ======
// Since a RemoteWindowProxy only represents a cross-origin frame, it has a much
// simpler prototype chain.
//
//   outer global proxy
//     -- has prototype --> inner global object
//     -- has prototype --> null
//
// Property access to get/set attributes and methods on the outer global proxy
// are redirected through the cross-origin interceptors, since any access will
// fail the security check, by definition.
//
// However, note that method invocations still use the inner global object as
// the receiver object. Blink bindings use v8::Signature to perform a strict
// receiver check, which requires that the FunctionTemplate used to instantiate
// the receiver object matches exactly. However, when creating a new context,
// only inner global object is instantiated using Blink's global template, so by
// definition, it is the only receiver object in the prototype chain that will
// match.
//
//
// ====== References ======
// https://wiki.mozilla.org/Gecko:SplitWindow
// https://whatwg.org/C/browsers.html#the-windowproxy-exotic-object
class CORE_EXPORT WindowProxy : public GarbageCollected<WindowProxy> {
 public:
  virtual ~WindowProxy();

  virtual void Trace(Visitor*) const;

  void InitializeIfNeeded();

  void ClearForClose();
  void ClearForNavigation();
  void ClearForSwap();
  void ClearForV8MemoryPurge();

  v8::Local<v8::Object> GetGlobalProxy();
  v8::Local<v8::Object> ReleaseGlobalProxy();
  // This does not initialize the window proxy, either Initialize or
  // InitializeIfNeeded needs to be called after this method.
  void SetGlobalProxy(v8::Local<v8::Object>);

  // TODO(dcheng): Temporarily exposed to avoid include cycles. Remove the need
  // for this and remove this getter.
  DOMWrapperWorld& World() { return *world_; }

  virtual bool IsLocal() const { return false; }

  enum FrameReuseStatus { kFrameWillNotBeReused, kFrameWillBeReused };

 protected:
  // Lifecycle represents the following four states.
  //
  // * kContextIsUninitialized
  // We lazily initialize WindowProxies for performance reasons, and this state
  // is "to be initialized on demand". WindowProxy basically behaves the same as
  // |kContextIsInitialized| from a point of view of call sites.
  // - Possible next states: kContextIsInitialized
  // It's possible to detach the context's frame from the DOM or navigate to a
  // new page without initializing the WindowProxy, however, there is no
  // transition to |kFrameIsDetached| or |kGlobalObjectIsDetached| or
  // |kV8MemoryIsForciblyPurged| because |DisposeContext| does not change the
  // state if the state is |kContextIsUninitialized|. In either case of a) the
  // browsing context container is detached from the DOM or b) the page is
  // navigated away, there must be no way for author script to access the
  // context of |kContextIsUninitialized| because |kContextIsUninitialized|
  // means that author script has never accessed the context, hence there must
  // exist no reference to the context.
  //
  // * kContextIsInitialized
  // The context is initialized and its frame is still attached to the DOM.
  // - Possible next states: kFrameIsDetached, kGlobalObjectIsDetached,
  // kV8MemoryIsForciblyPurged
  //
  // * kV8MemoryIsForciblyPurged
  // The context is initialized and its frame is still attached to the DOM, but
  // the global object is detached from the global proxy in order to drop all
  // references to v8, hopefully causing all JS objects to be collected for
  // memory reduction.
  // - Possible next states: kGlobalObjectIsDetached,
  // kFrameIsDetachedAndV8MemoryIsPurged
  // Navigation can occur after V8 memory purge, and the state will transition
  // to kGlobalObjectIsDetached in that case. When frame is detached after V8
  // memory purge, the global proxy will be a weak reference and will transition
  // to kFrameIsDetachedAndV8MemoryIsPurged.
  //
  // * kGlobalObjectIsDetached
  // The context is initialized and its frame is still attached to the DOM, but
  // the global object(inner global)'s Document is no longer the active Document
  // of the frame (i.e. it is being navigated away). The global object (inner
  // global) is detached from the global proxy (outer global), but the
  // (detached) global object and context are still alive, and author script may
  // have references to the context.
  // The spec does not support full web features in this state. Blink supports
  // less things than the spec.
  // This state is also used when swapping frames.  See also |WebFrame::Swap|.
  // - Possible next states: kContextIsInitialized
  // This state is in the middle of navigation. Once document loading is
  // completed, the WindowProxy will always be reinitialized, as
  // |DocumentLoader::InstallNewDocument| ends up calling to
  // |WindowProxy::UpdateDocument|, which reinitializes the WindowProxy.
  //
  // * kFrameIsDetached
  // The context was initialized, but its frame has been detached from the DOM.
  // Note that the context is still alive and author script may have references
  // to the context and hence author script may run in the context.
  // The spec does not support some of web features such as setTimeout, etc. on
  // a detached window. Blink supports less things than the spec.
  // V8PerContextData is cut off from the context.  |global_proxy_| becomes a
  // weak reference so that it's collectable when author script has no
  // reference.
  // - Possible next states: n/a
  //
  // * kFrameIsDetachedAndV8MemoryIsPurged
  // V8 memory is purged for memory reduction and thus global object is detached
  // from the global proxy, and also frame is detached from the DOM. Like
  // kFrameIsDetached, |global_proxy_| becomes a weak reference.
  // - Possible next states: n/a
  enum class Lifecycle {
    // v8::Context is not yet initialized.
    kContextIsUninitialized,
    // v8::Context is initialized.
    kContextIsInitialized,
    // The global object (inner global) is detached from the global proxy (outer
    // global). Could transition to kGlobalObjectIsDetached.
    kV8MemoryIsForciblyPurged,
    // The global object (inner global) is detached from the global proxy (outer
    // global).
    kGlobalObjectIsDetached,
    // The context's frame is detached from the DOM.
    kFrameIsDetached,
    // The context's frame is detached from the DOM, and global object is
    // detached from the global proxy.
    kFrameIsDetachedAndV8MemoryIsPurged,
  };

  WindowProxy(v8::Isolate*, Frame&, DOMWrapperWorld*);

  virtual void Initialize() = 0;

  virtual void DisposeContext(Lifecycle next_status, FrameReuseStatus) = 0;

  v8::Isolate* GetIsolate() const { return isolate_; }
  Frame* GetFrame() const { return frame_.Get(); }

#if DCHECK_IS_ON()
  void DidAttachGlobalObject() { is_global_object_attached_ = true; }
  void DidDetachGlobalObject() { is_global_object_attached_ = false; }
#endif

 private:
  v8::Isolate* const isolate_;
  const Member<Frame> frame_;
#if DCHECK_IS_ON()
  bool is_global_object_attached_ = false;
#endif

 protected:
  // TODO(dcheng): Consider making these private and using getters.
  const Member<DOMWrapperWorld> world_;
  // Note: this used to be a ScopedPersistent, making `global_proxy_` a strong
  // GC root from Blink to v8::Context. When the context was disposed (e.g. when
  // detaching the frame or purging v8), it was important to call `SetPhantom()`
  // to convert `global_proxy_` to a weak GC root. Otherwise, even if
  // `WindowProxy` was no longer reachable via Blink GC, the strong reference
  // could create a reference cycle preventing GC of both Blink and v8 objects:
  //
  //   WindowProxy -> global_proxy_ -> v8::Context -> JS object
  //       -> chain of Blink C++ objects -> WindowProxy
  //
  // With unified tracing, `global_proxy_` can simply be a traced v8 reference.
  // Once `WindowProxy` is no longer reachable from a GC root, `global_proxy_`
  // will also become collectible with no need to explicitly break a cycle.
  TraceWrapperV8Reference<v8::Object> global_proxy_;
  Lifecycle lifecycle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WINDOW_PROXY_H_
