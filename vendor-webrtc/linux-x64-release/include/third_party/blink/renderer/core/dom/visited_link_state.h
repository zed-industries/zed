/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 2004-2005 Allan Sandfeld Jensen (kde@carewolf.com)
 * Copyright (C) 2006, 2007 Nicholas Shanks (webkit@nickshanks.com)
 * Copyright (C) 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2007 Alexey Proskuryakov <ap@webkit.org>
 * Copyright (C) 2007, 2008 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (c) 2011, Code Aurora Forum. All rights reserved.
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_VISITED_LINK_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_VISITED_LINK_STATE_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/link_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class Document;

class CORE_EXPORT VisitedLinkState final
    : public GarbageCollected<VisitedLinkState> {
 public:
  explicit VisitedLinkState(const Document&);

  void InvalidateStyleForAllLinks(bool invalidate_visited_link_hashes);
  void InvalidateStyleForLink(LinkHash);

  // Allows Documents to set or update the per-origin salt associated with their
  // VisitedLinkState. This function informs the VisitedLinkReader in the
  // corresponding process of the new salt value.
  void UpdateSalt(uint64_t visited_link_salt);

  EInsideLink DetermineLinkState(const Element& element) {
    if (element.IsLink())
      return DetermineLinkStateSlowCase(element);
    return EInsideLink::kNotInsideLink;
  }

  void Trace(Visitor*) const;

 private:
  const Document& GetDocument() const { return *document_; }

  EInsideLink DetermineLinkStateSlowCase(const Element&);

  Member<const Document> document_;
  HashSet<LinkHash, LinkHashHashTraits> links_checked_for_visited_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_VISITED_LINK_STATE_H_
