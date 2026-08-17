/*
 * Copyright (C) 2007 Alp Toker <alp@atoker.com>
 * Copyright (C) 2007 Apple Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PRINT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PRINT_CONTEXT_H_

#include "third_party/blink/public/web/web_print_page_description.h"
#include "third_party/blink/public/web/web_print_params.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Element;
class GraphicsContext;
class LocalFrame;
class Node;
class PropertyTreeStateOrAlias;

class CORE_EXPORT PrintContext : public GarbageCollected<PrintContext> {
 public:
  explicit PrintContext(LocalFrame*);
  virtual ~PrintContext();

  LocalFrame* GetFrame() const { return frame_.Get(); }

  // These are only valid when inside print mode.
  virtual wtf_size_t PageCount() const;
  gfx::Rect PageRect(wtf_size_t page_index) const;

  // Enter print mode, updating layout for paginated layout. WebPrintParams
  // provides a default page size and margins, but this may be overridden by
  // at-page rules for any given page.
  // This function can be called multiple times to apply new print options
  // without going back to screen mode.
  virtual void BeginPrintMode(const WebPrintParams&);

  // Return to screen mode.
  virtual void EndPrintMode();

  // The following static methods are used by web tests:

  // Returns -1 if page isn't found.
  static int PageNumberForElement(Element*,
                                  const gfx::SizeF& page_size_in_pixels);
  static int NumberOfPages(LocalFrame*, const gfx::SizeF& page_size_in_pixels);

  virtual void Trace(Visitor*) const;

 protected:
  friend class PrintContextTest;

  void OutputLinkedDestinations(GraphicsContext&,
                                const PropertyTreeStateOrAlias&,
                                const gfx::Rect& page_rect);
  bool IsFrameValid() const;

  Member<LocalFrame> frame_;

  bool use_paginated_layout_ = true;

 private:
  void ComputePageCount();
  void CollectLinkedDestinations(Node*);

  // Used to prevent misuses of BeginPrintMode() and EndPrintMode() (e.g., call
  // EndPrintMode() without BeginPrintMode()).
  bool is_printing_;

  HeapHashMap<String, Member<Node>> linked_destinations_;
  bool linked_destinations_valid_;
};

class CORE_EXPORT ScopedPrintContext {
  STACK_ALLOCATED();

 public:
  explicit ScopedPrintContext(LocalFrame*);
  ScopedPrintContext(const ScopedPrintContext&) = delete;
  ScopedPrintContext& operator=(const ScopedPrintContext&) = delete;
  ~ScopedPrintContext();

  PrintContext* operator->() const { return context_; }

 private:
  PrintContext* context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PRINT_CONTEXT_H_
