/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_node.h"
#include "third_party/skia/include/core/SkBitmap.h"
#include "v8/include/v8-forward.h"

namespace gfx {
class Rect;
class Size;
}

namespace blink {

class Element;
class Image;
class WebLabelElement;

// Provides access to some properties of a DOM element node.
class BLINK_EXPORT WebElement : public WebNode {
 public:
  WebElement() : WebNode() {}
  WebElement(const WebElement& e) = default;

  // Returns the empty WebElement if the argument doesn't represent an Element.
  static WebElement FromV8Value(v8::Isolate*, v8::Local<v8::Value>);

  WebElement& operator=(const WebElement& e) {
    WebNode::Assign(e);
    return *this;
  }
  void Assign(const WebElement& e) { WebNode::Assign(e); }

  bool IsFormControlElement() const;
  // If the element is editable, for example by being contenteditable or being
  // an <input> that isn't readonly or disabled.
  bool IsEditable() const;
  // Returns the qualified name, which may contain a prefix and a colon.
  WebString TagName() const;
  // Returns the id attribute.
  WebString GetIdAttribute() const;
  // Check if this element has the specified local tag name, and the HTML
  // namespace. Tag name matching is case-insensitive.
  bool HasHTMLTagName(const WebString&) const;
  bool HasAttribute(const WebString&) const;
  WebString GetAttribute(const WebString&) const;
  void SetAttribute(const WebString& name, const WebString& value);
  WebString TextContent() const;
  WebString TextContentAbridged(unsigned int max_length) const;
  WebString InnerHTML() const;

  void Focus();

  // Returns true if the element's computed writing suggestions value is true.
  // https://html.spec.whatwg.org/#writing-suggestions:computed-writing-suggestions-value
  bool WritingSuggestions() const;

  // Returns true if the frame's selection is inside this editable element.
  bool ContainsFrameSelection() const;

  // Returns the selected text if this element contains the selection.
  // Otherwise returns the empty string.
  WebString SelectedText() const;

  // Selects the text in this element.
  // If `select_all`, then the entire contents of the element is selected.
  // If `!select_all`, then selects only the empty range at the end of the
  // element
  void SelectText(bool select_all);

  // Simulates a paste of `text` event into `this` element.
  //
  // There are three different behaviors depending on `replace_all` and which
  // text is currently selected:
  // - If `replace_all`, the entire contents of the element is selected first,
  //   so that the paste action replaces it.
  // - If `!replace_all` and the selection is not currently in the element, an
  //   empty range at the end of the element is selected, so that the paste
  //   action appends to the element.
  // - Otherwise, the current selection is unchanged, so that the paste replaces
  //   the selected text.
  //
  // This is a no-op if the element is not editable.
  void PasteText(const WebString& text, bool replace_all);

  // Returns all <label> elements associated to this element.
  std::vector<WebLabelElement> Labels() const;

  // Returns true if this is an autonomous custom element.
  bool IsAutonomousCustomElement() const;

  // Returns the owning shadow host for this element, if there is one.
  WebElement OwnerShadowHost() const;

  // Returns an author ShadowRoot attached to this element, regardless
  // of open or closed.  This returns null WebNode if this
  // element has no ShadowRoot or has a UA ShadowRoot.
  WebNode ShadowRoot() const;

  // Returns the open shadow root or the closed shadow root.
  WebNode OpenOrClosedShadowRoot();

  // Returns the bounds of the element in Visual Viewport. The bounds
  // have been adjusted to include any transformations, including page scale.
  // This function will update the layout if required.
  gfx::Rect BoundsInWidget() const;

  // Returns the image contents of this element or a null SkBitmap
  // if there isn't any.
  SkBitmap ImageContents();

  // Returns a copy of original image data of this element or an empty vector
  // if there isn't any.
  std::vector<uint8_t> CopyOfImageData();

  // Returns the original image file extension.
  std::string ImageExtension();

  // Returns the original image size.
  gfx::Size GetImageSize();

  // Returns {clientWidth, clientHeight}.
  gfx::Size GetClientSize() const;

  // Returns {scrollWidth, scrollHeight}.
  gfx::Size GetScrollSize() const;

  // ComputedStyle property values. The following exposure is of CSS property
  // values are part of the ComputedStyle set which is usually exposed through
  // the Window object in WebIDL as window.getComputedStyle(element). Exposing
  // ComputedStyle requires all of CSSComputedStyleDeclaration which is a pretty
  // large interfaces. For now the we are exposing computed property values as
  // strings directly to WebElement and enable public component usage through
  // /public/web interfaces.
  WebString GetComputedValue(const WebString& property_name);

#if INSIDE_BLINK
  WebElement(Element*);
  WebElement& operator=(Element*);
  operator Element*() const;
#endif

 private:
  Image* GetImage();
};

DECLARE_WEB_NODE_TYPE_CASTS(WebElement);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_H_
