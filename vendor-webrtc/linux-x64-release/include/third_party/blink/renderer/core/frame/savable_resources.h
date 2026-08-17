// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SAVABLE_RESOURCES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SAVABLE_RESOURCES_H_

#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;
class LocalFrame;

class SavableResources {
  STATIC_ONLY(SavableResources);

 public:
  // Class for storage the result of getting all savable resource links
  // for current page. The consumer of the SavableResources::Result is
  // responsible for keeping these pointers valid for the lifetime of the
  // SavableResources::Result instance.
  class Result {
    STACK_ALLOCATED();

   public:
    Result(Vector<KURL>* resources_list,
           Vector<mojom::blink::SavableSubframePtr>* subframes)
        : resources_list_(resources_list), subframes_(subframes) {}

    void AppendSubframe(mojom::blink::SavableSubframePtr subframe);
    void AppendResourceLink(const KURL& url);

   private:
    // Links of all savable resources.
    Vector<KURL>* resources_list_;

    // Subframes.
    Vector<mojom::blink::SavableSubframePtr>* subframes_;
  };

  // Get all the savable resource links from the specified |frame|.
  // Returns true if the saved resources links have been saved successfully.
  // Otherwise returns false (i.e. if the frame contains a non-savable content).
  static bool GetSavableResourceLinksForFrame(LocalFrame* frame,
                                              SavableResources::Result* result);

  // Returns the value in an elements resource url attribute. For IMG, SCRIPT or
  // INPUT TYPE=image, returns the value in "src". For LINK TYPE=text/css,
  // returns the value in "href". For BODY, TABLE, TR, TD, returns the value in
  // "background". For BLOCKQUOTE, Q, DEL, INS, returns the value in "cite"
  // attribute. Otherwise returns an empty String.
  static String GetSubResourceLinkFromElement(Element* element);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SAVABLE_RESOURCES_H_
