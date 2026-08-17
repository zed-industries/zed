/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_FORMATTING_ELEMENT_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_FORMATTING_ELEMENT_LIST_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/html/parser/html_stack_item.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;

// This may end up merged into HTMLElementStack.
class HTMLFormattingElementList {
  DISALLOW_NEW();

 public:
  HTMLFormattingElementList();
  HTMLFormattingElementList(const HTMLFormattingElementList&) = delete;
  HTMLFormattingElementList& operator=(const HTMLFormattingElementList&) =
      delete;

  // Ideally Entry would be private, but HTMLTreeBuilder has to coordinate
  // between the HTMLFormattingElementList and HTMLElementStack and needs access
  // to Entry::isMarker() and Entry::replaceElement() to do so.
  class Entry {
    DISALLOW_NEW();

   public:
    // Inline because they're hot and Vector<T> uses them.
    explicit Entry(HTMLStackItem* item) : item_(item) {}
    enum MarkerEntryType { kMarkerEntry };
    explicit Entry(MarkerEntryType) : item_(nullptr) {}
    ~Entry() = default;

    bool IsMarker() const { return !item_; }

    HTMLStackItem* StackItem() const { return item_.Get(); }
    Element* GetElement() const {
      // The fact that !item_ == IsMarker() is an implementation detail callers
      // should check IsMarker() before calling GetElement().
      DCHECK(item_);
      return item_->GetElement();
    }
    void ReplaceElement(HTMLStackItem* item) { item_ = item; }

    // Needed for use with Vector.  These are super-hot and must be inline.
    bool operator==(Element* element) const {
      return !item_ ? !element : item_->GetElement() == element;
    }
    bool operator!=(Element* element) const {
      return !item_ ? !!element : item_->GetElement() != element;
    }

    void Trace(Visitor* visitor) const { visitor->Trace(item_); }

   private:
    Member<HTMLStackItem> item_;
  };

  class Bookmark {
    STACK_ALLOCATED();

   public:
    explicit Bookmark(Entry* entry) : has_been_moved_(false), mark_(entry) {}

    void MoveToAfter(Entry* before) {
      has_been_moved_ = true;
      mark_ = before;
    }

    bool HasBeenMoved() const { return has_been_moved_; }
    Entry* Mark() const { return mark_; }

   private:
    bool has_been_moved_;
    Entry* mark_;
  };

  bool IsEmpty() const { return !size(); }
  wtf_size_t size() const { return entries_.size(); }

  Element* ClosestElementInScopeWithName(const AtomicString&);

  Entry* Find(Element*);
  bool Contains(Element*);
  void Append(HTMLStackItem*);
  void Remove(Element*);

  Bookmark BookmarkFor(Element*);
  void SwapTo(Element* old_element, HTMLStackItem* new_item, const Bookmark&);

  void AppendMarker();
  // clearToLastMarker also clears the marker (per the HTML5 spec).
  void ClearToLastMarker();

  const Entry& at(wtf_size_t i) const { return entries_[i]; }
  Entry& at(wtf_size_t i) { return entries_[i]; }

  void Trace(Visitor* visitor) const { visitor->Trace(entries_); }

#ifndef NDEBUG
  void Show();
#endif

 private:
  Entry* First() { return &at(0); }

  // http://www.whatwg.org/specs/web-apps/current-work/multipage/parsing.html#list-of-active-formatting-elements
  // These functions enforce the "Noah's Ark" condition, which removes redundant
  // mis-nested elements.
  void TryToEnsureNoahsArkConditionQuickly(
      HTMLStackItem*,
      HeapVector<Member<HTMLStackItem>>& remaining_candiates);
  void EnsureNoahsArkCondition(HTMLStackItem*);

  HeapVector<Entry> entries_;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(
    blink::HTMLFormattingElementList::Entry)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_FORMATTING_ELEMENT_LIST_H_
