// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_H_

#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/core/svg/svg_resource_document_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/cross_origin_attribute_value.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Document;
class Element;
class IdTargetObserver;
class ImageResourceObserver;
class LayoutSVGResourceContainer;
class QualifiedName;
class SVGFilterPrimitiveStandardAttributes;
class SVGResourceClient;
class SVGResourceDocumentContent;
class TreeScope;

// A class tracking a reference to an SVG resource (an element that constitutes
// a paint server, mask, clip-path, filter et.c.)
//
// Elements can be referenced using CSS, for example like:
//
//   filter: url(#baz);                             ("local")
//
//    or
//
//   filter: url(foo.com/bar.svg#baz);              ("external")
//
// SVGResource provide a mechanism to persistently reference an element in such
// cases - regardless of if the element reside in the same document (read: tree
// scope) or in an external (resource) document. Loading events related to the
// external documents case are handled by the SVGResource.
//
// For same document references, changes that could affect the 'id' lookup will
// be tracked, to handle elements being added, removed or having their 'id'
// mutated. (This does not apply for the external document case because it's
// assumed they will not mutate after load, due to scripts not being run etc.)
//
// SVGResources are created, and managed, either by SVGTreeScopeResources
// (local) or CSSURIValue (external), and have SVGResourceClients as a means to
// deliver change notifications. Clients that are interested in change
// notifications hence need to register a SVGResourceClient or a
// ImageResourceObserver with the SVGResource. Most commonly this registration
// would take place when the computed style changes. If an
// ImageResourceObserver is registered, an SVGResourceClient is created
// internally, which can be accessed using
// SVGResource::GetObserverResourceClient() if needed.
//
// The element is bound either when the SVGResource is created (for local
// resources) or after the referenced resource has completed loading (for
// external resources.)
//
// As content is mutated, clients will get notified via the SVGResource.
//
// <event> -> SVG...Element -> SVGResource -> SVGResourceClient(0..N)
//
class SVGResource : public GarbageCollected<SVGResource> {
 public:
  SVGResource(const SVGResource&) = delete;
  SVGResource& operator=(const SVGResource&) = delete;
  virtual ~SVGResource();

  virtual void Load(Document&, CrossOriginAttributeValue) {}
  virtual void LoadWithoutCSP(Document&) {}

  virtual bool IsLoading() const { return false; }

  Element* Target() const { return target_.Get(); }
  // Returns the target's LayoutObject (if target exists and is attached to the
  // layout tree). Also perform cycle-checking, and may thus return nullptr if
  // this SVGResourceClient -> SVGResource reference would start a cycle.
  LayoutSVGResourceContainer* ResourceContainer(SVGResourceClient&) const;
  // Same as the above, minus the cycle-checking.
  LayoutSVGResourceContainer* ResourceContainerNoCycleCheck() const;
  // Run cycle-checking for this SVGResourceClient -> SVGResource
  // reference. Used internally by the cycle-checking, and shouldn't be called
  // directly in general.
  bool FindCycle(SVGResourceClient&) const;

  void AddClient(SVGResourceClient&);
  void RemoveClient(SVGResourceClient&);

  void AddObserver(ImageResourceObserver&);
  void RemoveObserver(ImageResourceObserver&);

  SVGResourceClient* GetObserverResourceClient(ImageResourceObserver&);

  virtual void Trace(Visitor*) const;

 protected:
  SVGResource();

  void InvalidateCycleCache();
  void NotifyContentChanged();

  Member<Element> target_;

  enum CycleState {
    kNeedCheck,
    kPerformingCheck,
    kHasCycle,
    kNoCycle,
  };
  struct ClientEntry {
    int count = 0;
    CycleState cached_cycle_check = kNeedCheck;
  };
  mutable HeapHashMap<Member<SVGResourceClient>, ClientEntry> clients_;

  class ImageResourceObserverWrapper;
  HeapHashMap<Member<ImageResourceObserver>,
              Member<ImageResourceObserverWrapper>>
      observer_wrappers_;
};

// Local resource reference (see SVGResource.)
class LocalSVGResource final : public SVGResource {
 public:
  LocalSVGResource(TreeScope&, const AtomicString& id);

  void Unregister();

  using SVGResource::NotifyContentChanged;

  void NotifyFilterPrimitiveChanged(
      SVGFilterPrimitiveStandardAttributes& primitive,
      const QualifiedName& attribute);

  void Trace(Visitor*) const override;

 private:
  void TargetChanged(const AtomicString& id);

  Member<TreeScope> tree_scope_;
  Member<IdTargetObserver> id_observer_;
};

// External resource reference (see SVGResource) with an
// SVGResourceDocumentContent as the "data source".
class ExternalSVGResourceDocumentContent final
    : public SVGResource,
      public SVGResourceDocumentObserver {
 public:
  explicit ExternalSVGResourceDocumentContent(const KURL&);

  void Load(Document&, CrossOriginAttributeValue) override;
  void LoadWithoutCSP(Document&) override;

  void Trace(Visitor*) const override;

 private:
  bool IsLoading() const override;
  Element* ResolveTarget();

  // SVGResourceDocumentObserver:
  void ResourceNotifyFinished(SVGResourceDocumentContent*) override;
  void ResourceContentChanged(SVGResourceDocumentContent*) override;

  Member<SVGResourceDocumentContent> document_content_;
  KURL url_;
};

// External resource reference (see SVGResource) with an ImageResourceContent
// as the "data source".
class ExternalSVGResourceImageContent final : public SVGResource,
                                              public ImageResourceObserver {
  USING_PRE_FINALIZER(ExternalSVGResourceImageContent, Prefinalize);

 public:
  ExternalSVGResourceImageContent(ImageResourceContent* image_content,
                                  const AtomicString& fragment);

  void Trace(Visitor*) const override;

 private:
  void Prefinalize();

  bool IsLoading() const override;
  Element* ResolveTarget();

  // ImageResourceObserver overrides
  void ImageNotifyFinished(ImageResourceContent*) override;
  WTF::String DebugName() const override;

  Member<ImageResourceContent> image_content_;
  AtomicString fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_H_
