/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2007m 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAME_NODE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAME_NODE_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/live_node_list.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// NodeList which lists all Nodes in a Element with a given "name" attribute
class CORE_EXPORT NameNodeList final : public LiveNodeList {
 public:
  NameNodeList(ContainerNode& root_node, const AtomicString& name);
  NameNodeList(ContainerNode& root_node,
               CollectionType type,
               const AtomicString& name);
  ~NameNodeList() override;

 private:
  bool ElementMatches(const Element&) const override;

  AtomicString name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAME_NODE_LIST_H_
