/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Nikolas Zimmermann
 * <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_USE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_USE_ELEMENT_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_geometry_element.h"
#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"
#include "third_party/blink/renderer/core/svg/svg_resource_document_observer.h"
#include "third_party/blink/renderer/core/svg/svg_uri_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class IncrementLoadEventDelayCount;
class SVGAnimatedLength;
class SVGResourceDocumentContent;

class SVGUseElement final : public SVGGraphicsElement,
                            public SVGURIReference,
                            public SVGResourceDocumentObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGUseElement(Document&);
  ~SVGUseElement() override;

  void InvalidateShadowTree();
  void InvalidateTargetReference();

  // Return the element that should be used for clipping,
  // or null if a valid clip element is not directly referenced.
  SVGGraphicsElement* VisibleTargetGraphicsElementForClipping() const;

  SVGAnimatedLength* x() const { return x_.Get(); }
  SVGAnimatedLength* y() const { return y_.Get(); }
  SVGAnimatedLength* width() const { return width_.Get(); }
  SVGAnimatedLength* height() const { return height_.Get(); }

  void BuildPendingResource() override;
  String title() const override;

  Path ToClipPath() const;

  void Trace(Visitor*) const override;

 private:
  gfx::RectF GetBBox() override;

  bool IsStructurallyExternal() const override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void DidMoveToNewDocument(Document&) override;

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  void ScheduleShadowTreeRecreation();
  void CancelShadowTreeRecreation();
  bool HaveLoadedRequiredResources() override;
  bool ShadowTreeRebuildPending() const;

  bool SelfHasRelativeLengths() const override;

  ShadowRoot& UseShadowRoot() const {
    CHECK(UserAgentShadowRoot());
    return *UserAgentShadowRoot();
  }

  Element* ResolveTargetElement();
  void AttachShadowTree(SVGElement& target);
  void DetachShadowTree();
  CORE_EXPORT SVGElement* InstanceRoot() const;
  SVGElement* CreateInstanceTree(SVGElement& target_root) const;
  void ClearResourceReference();
  bool HasCycleUseReferencing(const ContainerNode& target_instance,
                              const SVGElement& new_target) const;

  void QueueOrDispatchPendingEvent(const AtomicString&);

  // SVGResourceDocumentObserver:
  void ResourceNotifyFinished(SVGResourceDocumentContent*) override;
  void ResourceContentChanged(SVGResourceDocumentContent*) override {}

  void UpdateDocumentContent(SVGResourceDocumentContent*);
  void UpdateTargetReference();

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;
  void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>& style) override;

  Member<SVGResourceDocumentContent> document_content_;
  Member<SVGResourceTarget> external_resource_target_;

  Member<SVGAnimatedLength> x_;
  Member<SVGAnimatedLength> y_;
  Member<SVGAnimatedLength> width_;
  Member<SVGAnimatedLength> height_;

  TaskHandle pending_event_;
  std::unique_ptr<IncrementLoadEventDelayCount> load_event_delayer_;
  KURL element_url_;
  bool element_url_is_local_;
  bool needs_shadow_tree_recreation_;
  Member<IdTargetObserver> target_id_observer_;

  FRIEND_TEST_ALL_PREFIXES(SVGUseElementTest,
                           NullInstanceRootWhenNotConnectedToDocument);
  FRIEND_TEST_ALL_PREFIXES(SVGUseElementTest,
                           NullInstanceRootWhenConnectedToInactiveDocument);
  FRIEND_TEST_ALL_PREFIXES(SVGUseElementTest,
                           NullInstanceRootWhenShadowTreePendingRebuild);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_USE_ELEMENT_H_
