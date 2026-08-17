/*
 * Copyright (C) 2006 Apple Computer, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FRAME_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FRAME_TREE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Frame;
struct FrameLoadRequest;
class KURL;

class CORE_EXPORT FrameTree final {
  DISALLOW_NEW();

 public:
  explicit FrameTree(Frame* this_frame);
  FrameTree(const FrameTree&) = delete;
  FrameTree& operator=(const FrameTree&) = delete;
  ~FrameTree();

  const AtomicString& GetName() const;

  enum ReplicationPolicy {
    // Does not propagate name changes beyond this FrameTree object.
    kDoNotReplicate,

    // Kicks-off propagation of name changes to other renderers.
    kReplicate,
  };

  // TODO(shuuran): remove this once we have gathered the data
  void CrossSiteCrossBrowsingContextGroupSetNulledName();

  void SetName(const AtomicString&, ReplicationPolicy = kDoNotReplicate);

  // TODO(andypaicu): remove this once we have gathered the data
  void ExperimentalSetNulledName();

  Frame* Parent() const;
  Frame& Top() const;
  Frame* NextSibling() const;
  Frame* FirstChild() const;

  bool IsDescendantOf(const Frame* ancestor) const;
  Frame* TraverseNext(const Frame* stay_within = nullptr) const;

  // For plugins and tests only.
  Frame* FindFrameByName(const AtomicString& name) const;

  // https://html.spec.whatwg.org/#the-rules-for-choosing-a-browsing-context-given-a-browsing-context-name
  struct FindResult {
    STACK_ALLOCATED();

   public:
    FindResult(Frame* f, bool is_new) : frame(f), new_window(is_new) {}
    Frame* frame;
    bool new_window;
  };
  FindResult FindOrCreateFrameForNavigation(FrameLoadRequest&,
                                            const AtomicString& name) const;

  unsigned ChildCount() const;

  Frame* ScopedChild(unsigned index) const;
  // https://whatwg.org/C/window-object.html#named-access-on-the-window-object
  // This implements the steps needed for looking up a child browsing context
  // that matches |name|. If |name.IsEmpty()| is true, this is guaranteed to
  // return null: the spec specifically states that browsing contexts with a
  // name are never considered.
  Frame* ScopedChild(const AtomicString& name) const;
  unsigned ScopedChildCount() const;
  void InvalidateScopedChildCount();

  void Trace(Visitor*) const;

 private:
  // TODO(crbug.com/1315802): Refactor _unfencedTop handling.
  Frame* FindFrameForNavigationInternal(
      const AtomicString& name,
      const KURL&,
      FrameLoadRequest* request = nullptr) const;

  Member<Frame> this_frame_;

  AtomicString name_;  // The actual frame name (may be empty).

  mutable unsigned scoped_child_count_;

  // TODO(andypaicu): remove this once we have gathered the data
  bool experimental_set_nulled_name_;

  // TODO(shuuran): remove this once we have gathered the data
  bool cross_site_cross_browsing_context_group_set_nulled_name_;
};

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
void ShowFrameTree(const blink::Frame*);
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FRAME_TREE_H_
