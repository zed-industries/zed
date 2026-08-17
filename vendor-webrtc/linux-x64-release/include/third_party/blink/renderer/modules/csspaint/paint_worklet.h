// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_H_

#include <memory>

#include "third_party/blink/renderer/core/css/css_syntax_definition.h"
#include "third_party/blink/renderer/core/workers/worklet.h"
#include "third_party/blink/renderer/modules/csspaint/document_paint_definition.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_global_scope_proxy.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_pending_generator_registry.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_proxy_client.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CSSPaintImageGeneratorImpl;

// Manages a paint worklet:
// https://drafts.css-houdini.org/css-paint-api/#dom-css-paintworklet
class MODULES_EXPORT PaintWorklet : public Worklet,
                                    public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  // At this moment, paint worklet allows at most two global scopes at any time.
  static const wtf_size_t kNumGlobalScopesPerThread;
  static PaintWorklet* From(LocalDOMWindow&);

  explicit PaintWorklet(LocalDOMWindow&);

  PaintWorklet(const PaintWorklet&) = delete;
  PaintWorklet& operator=(const PaintWorklet&) = delete;

  ~PaintWorklet() override;

  void AddPendingGenerator(const String& name, CSSPaintImageGeneratorImpl*);
  // The |container_size| is without subpixel snapping.
  scoped_refptr<Image> Paint(const String& name,
                             const ImageResourceObserver&,
                             const gfx::SizeF& container_size,
                             const GCedCSSStyleValueVector*);

  int WorkletId() const { return worklet_id_; }
  bool IsOffMainThread() const { return is_paint_off_thread_; }

  void Trace(Visitor*) const override;

  // The DocumentDefinitionMap tracks definitions registered via
  // registerProperty; definitions are only considered valid once all global
  // scopes have registered the same definition for the same thread.
  typedef HashMap<String, std::unique_ptr<DocumentPaintDefinition>>
      DocumentDefinitionMap;
  DocumentDefinitionMap& GetDocumentDefinitionMap() {
    return document_definition_map_;
  }

  // Used for main-thread CSS Paint. Registers a definition for a given painter,
  // ensuring that the same CSSPaintDefinition is registered on all global
  // scopes.
  void RegisterCSSPaintDefinition(const String& name,
                                  CSSPaintDefinition*,
                                  ExceptionState&);

  // Used for off-thread CSS Paint. In this mode we are not responsible for
  // tracking whether a definition is valid - this method should only be called
  // once all global scopes have registered the same |DocumentPaintDefinition|
  // for the same |name|.
  void RegisterMainThreadDocumentPaintDefinition(
      const String& name,
      Vector<CSSPropertyID> native_properties,
      Vector<String> custom_properties,
      Vector<CSSSyntaxDefinition> input_argument_types,
      double alpha);

  HeapVector<Member<WorkletGlobalScopeProxy>>& GetGlobalScopesForTesting() {
    return proxies_;
  }

  void AddGlobalScopeForTesting() { proxies_.push_back(CreateGlobalScope()); }

  bool NeedsToCreateGlobalScopeForTesting() {
    return NeedsToCreateGlobalScope();
  }

  void SetProxyClientForTesting(PaintWorkletProxyClient* proxy_client) {
    proxy_client_ = proxy_client;
  }

  void ResetIsPaintOffThreadForTesting();

 protected:
  // Since paint worklet has more than one global scope, we MUST override this
  // function and provide our own selection logic.
  wtf_size_t SelectGlobalScope() final;
  wtf_size_t GetActiveGlobalScopeForTesting() { return active_global_scope_; }

 private:
  friend class PaintWorkletTest;

  // Implements Worklet.
  bool NeedsToCreateGlobalScope() final;
  WorkletGlobalScopeProxy* CreateGlobalScope() final;

  // This function calculates the number of paints to use before switching
  // global scopes.
  virtual int GetPaintsBeforeSwitching();
  // This function calculates the next global scope to switch to.
  virtual wtf_size_t SelectNewGlobalScope();

  Member<PaintWorkletPendingGeneratorRegistry> pending_generator_registry_;

  // Used for both main and off-thread CSS Paint.
  // For the main thread, this map tracks the definitions created on the main
  // thread, and ensures that all global scopes have the same definition.
  //
  // For the off thread case, both the worklet and main thread have this map.
  // The worklet version is responsible for verifying that all global scopes
  // have the same definition, and the main thread version relies on that.
  //
  // The value of an entry being nullptr means that it is an invalid definition.
  DocumentDefinitionMap document_definition_map_;

  // The last document paint frame a paint worklet painted on. This is used to
  // tell when we begin painting on a new frame.
  size_t active_frame_count_ = 0u;
  // The current global scope being used for painting.
  wtf_size_t active_global_scope_ = 0u;
  // The number of paint calls remaining before Paint will select a new global
  // scope. SelectGlobalScope resets this at the beginning of each frame.
  int paints_before_switching_global_scope_;

  // An atomic sequence number to ensure that it is unique for each paint
  // worklet. This id is integrated in the PaintWorkletInput which will be used
  // in PaintWorkletPaintDispatcher::Paint, to identify the right painter, to
  // paint the image.
  int worklet_id_;

  // The proxy client associated with this PaintWorklet. We keep a reference in
  // to ensure that all global scopes get the same proxy client.
  Member<PaintWorkletProxyClient> proxy_client_;

  // When running layout test, paint worklet has to be on the main thread
  // because "enable-threaded-compositing" is off by default. However, some unit
  // tests may be testing the functionality of the APIs when the paint worklet
  // is off the main thread.
  bool is_paint_off_thread_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_H_
