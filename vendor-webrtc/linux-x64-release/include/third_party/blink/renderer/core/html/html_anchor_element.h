/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2007, 2008, 2009, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ANCHOR_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ANCHOR_ELEMENT_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/html/rel_list.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/core/url/dom_url_utils.h"
#include "third_party/blink/renderer/platform/link_hash.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"

namespace blink {

class MouseEvent;

// Link relation bitmask values.
// FIXME: Uncomment as the various link relations are implemented.
enum {
  //     RelationAlternate   = 0x00000001,
  //     RelationArchives    = 0x00000002,
  //     RelationAuthor      = 0x00000004,
  //     RelationBoomark     = 0x00000008,
  //     RelationExternal    = 0x00000010,
  //     RelationFirst       = 0x00000020,
  //     RelationHelp        = 0x00000040,
  //     RelationIndex       = 0x00000080,
  //     RelationLast        = 0x00000100,
  //     RelationLicense     = 0x00000200,
  //     RelationNext        = 0x00000400,
  //     RelationNoFolow    = 0x00000800,
  kRelationNoReferrer = 0x00001000,
  //     RelationPrev        = 0x00002000,
  //     RelationSearch      = 0x00004000,
  //     RelationSidebar     = 0x00008000,
  //     RelationTag         = 0x00010000,
  //     RelationUp          = 0x00020000,
  kRelationNoOpener = 0x00040000,
  kRelationOpener = 0x00080000,
  kRelationPrivacyPolicy = 0x00100000,
  kRelationTermsOfService = 0x00200000,
};

// Base class for <a> and <area> (HTMLAnchorElement and HTMLAreaElement).
// Note: If a new element needs to use this as a base, existing callsites and
// features that use this class should be audited (to see if the new element
// should also support these features).
class CORE_EXPORT HTMLAnchorElementBase : public HTMLElement,
                                          public DOMURLUtils {
 public:
  ~HTMLAnchorElementBase() override;

  KURL Href() const;
  void SetHref(const AtomicString&);
  void setHref(const String&);

  const AtomicString& GetName() const;

  // Returns the anchor's |target| attribute, unless it is empty, in which case
  // the BaseTarget from the document is returned.
  const AtomicString& GetEffectiveTarget() const;

  KURL Url() const final;
  void SetURL(const KURL&) final;

  String Input() const final;

  bool IsLiveLink() const final;

  bool WillRespondToMouseClickEvents() final;

  bool HasRel(uint32_t relation) const;
  void SetRel(const AtomicString&);
  DOMTokenList& relList() const {
    return static_cast<DOMTokenList&>(*rel_list_);
  }

  LinkHash VisitedLinkHash() const;
  LinkHash PartitionedVisitedLinkFingerprint() const;
  void InvalidateCachedVisitedLinkHash() { cached_visited_link_hash_ = 0; }

  void SendPings(const KURL& destination_url) const;

  Element* InterestTargetElement() const override;

  void Trace(Visitor*) const override;

 protected:
  HTMLAnchorElementBase(const QualifiedName& tag_name, Document&);

  void ParseAttribute(const AttributeModificationParams&) override;
  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;

  void FinishParsingChildren() final;

 private:
  void AttributeChanged(const AttributeModificationParams&) override;
  bool ShouldHaveFocusAppearance() const final;
  FocusableState IsFocusableState(
      UpdateBehavior update_behavior) const override;
  bool IsKeyboardFocusableSlow(UpdateBehavior update_behavior) const override;
  void DefaultEventHandler(Event&) final;
  bool HasActivationBehavior() const override;
  void SetActive(bool active) final;
  bool IsURLAttribute(const Attribute&) const final;
  bool HasLegalLinkAttribute(const QualifiedName&) const final;
  bool CanStartSelection() const final;
  int DefaultTabIndex() const final;
  bool draggable() const final;
  bool IsInteractiveContent() const final;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void NavigateToHyperlink(ResourceRequest,
                           NavigationPolicy,
                           bool is_trusted,
                           base::TimeTicks platform_time_stamp,
                           KURL);
  void HandleClick(MouseEvent&);

  unsigned link_relations_ : 31;
  mutable LinkHash cached_visited_link_hash_;
  Member<RelList> rel_list_;
};

class CORE_EXPORT HTMLAnchorElement : public HTMLAnchorElementBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLAnchorElement(Document& document);

  void AttachLayoutTree(AttachContext& context) override;
  void DetachLayoutTree(bool performing_reattach) override;

  // Gets the element which is referenced by this anchor fragment
  // (#scroll-target), or nullptr if not found.
  Element* ScrollTargetElement() const;
  // Gets the closest ancestor scrollable area of this anchors scroll target
  // element.
  PaintLayerScrollableArea* AncestorScrollableAreaOfScrollTargetElement() const;
};

template <>
struct DowncastTraits<HTMLAnchorElementBase> {
  static bool AllowFrom(const Element& element) {
    return element.HasTagName(html_names::kATag) ||
           element.HasTagName(html_names::kAreaTag);
  }

  static bool AllowFrom(const Node& node) {
    return node.IsHTMLElement() &&
           IsA<HTMLAnchorElementBase>(UnsafeTo<HTMLElement>(node));
  }
};

inline LinkHash HTMLAnchorElementBase::VisitedLinkHash() const {
  if (!cached_visited_link_hash_) {
    cached_visited_link_hash_ = blink::VisitedLinkHash(
        GetDocument().BaseURL(), FastGetAttribute(html_names::kHrefAttr));
  }
  return cached_visited_link_hash_;
}

inline LinkHash HTMLAnchorElementBase::PartitionedVisitedLinkFingerprint()
    const {
  if (!cached_visited_link_hash_) {
    // Obtain all the elements of the partition key.
    cached_visited_link_hash_ = blink::PartitionedVisitedLinkFingerprint(
        /*base_link_url=*/GetDocument().BaseURL(),
        /*relative_link_url=*/FastGetAttribute(html_names::kHrefAttr),
        /*top_level_site=*/
        GetDocument().GetCachedTopFrameSite(Document::VisitedLinkPassKey()),
        /*frame_origin=*/GetDocument().domWindow()->GetSecurityOrigin());
  }
  return cached_visited_link_hash_;
}

// Functions shared with the other anchor elements (i.e., SVG).

bool IsEnterKeyKeydownEvent(Event&);
bool IsLinkClick(Event&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ANCHOR_ELEMENT_H_
