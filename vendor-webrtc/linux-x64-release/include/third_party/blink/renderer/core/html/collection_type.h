/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_COLLECTION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_COLLECTION_TYPE_H_

namespace blink {

enum CollectionType {
  // Unnamed HTMLCollection types cached in the document.
  kDocImages,   // all <img> elements in the document
  kDocApplets,  // all <object> and <applet> elements
  kDocEmbeds,   // all <embed> elements
  kDocForms,    // all <form> elements
  kDocLinks,    // all <a> _and_ <area> elements with a value for href
  kDocAnchors,  // all <a> elements with a value for name
  kDocScripts,  // all <script> elements
  kDocAll,      // "all" elements (IE)

  // Unnamed HTMLCollection types cached in elements.
  kNodeChildren,  // first-level children (ParentNode DOM interface)
  kTableTBodies,  // all <tbody> elements in this table
  kTSectionRows,  // all row elements in this table section
  kTableRows,
  kTRCells,  // all cells in this row
  kSelectOptions,
  kSelectedOptions,
  kDataListOptions,
  kMapAreas,
  kFormControls,
  kPopoverInvokers,
  kCommandInvokers,

  // Named HTMLCollection types cached in the document.
  kWindowNamedItems,
  kDocumentNamedItems,
  kDocumentAllNamedItems,

  // Named HTMLCollection types cached in elements.
  kClassCollectionType,
  kTagCollectionType,
  kHTMLTagCollectionType,
  kTagCollectionNSType,

  // Live NodeList.
  kNameNodeListType,
  kRadioNodeListType,
  kRadioImgNodeListType,
  kLabelsNodeListType,
};

static const CollectionType kFirstNamedCollectionType = kWindowNamedItems;
static const CollectionType kFirstLiveNodeListType = kNameNodeListType;

inline bool IsUnnamedHTMLCollectionType(CollectionType type) {
  return type < kFirstNamedCollectionType;
}

inline bool IsHTMLCollectionType(CollectionType type) {
  return type < kFirstLiveNodeListType;
}

inline bool IsLiveNodeListType(CollectionType type) {
  return type >= kFirstLiveNodeListType;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_COLLECTION_TYPE_H_
