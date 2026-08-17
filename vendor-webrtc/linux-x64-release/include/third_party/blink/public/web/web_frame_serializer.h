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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_SERIALIZER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_SERIALIZER_H_

#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_thread_safe_data.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

class WebFrameSerializerClient;
class WebFrame;
class WebLocalFrame;

// Serialization of frame contents into html or mhtml.
class BLINK_EXPORT WebFrameSerializer {
 public:
  // Delegate for controling the behavior of generateMHTMLParts method.
  class MHTMLPartsGenerationDelegate {
   public:
    // Tells whether to skip serialization of a subresource with a given URI.
    // Used to deduplicate resources across multiple frames.
    virtual bool ShouldSkipResource(const WebURL&) = 0;

    virtual bool UseBinaryEncoding() = 0;

    virtual bool RemovePopupOverlay() = 0;
  };

  // Generates and returns an MHTML header.
  //
  // Contents of the header (i.e. title and mime type) will be based
  // on the frame passed as an argument (which typically should be
  // the main, top-level frame).
  //
  // Same |boundary| needs to used for all generateMHTMLHeader and
  // generateMHTMLParts and generateMHTMLFooter calls that belong to the same
  // MHTML document (see also rfc1341, section 7.2.1, "boundary" description).
  static WebThreadSafeData GenerateMHTMLHeader(const WebString& boundary,
                                               WebLocalFrame*,
                                               MHTMLPartsGenerationDelegate*);

  // Generates and MHTML parts for the given frame and the savable resources
  // underneath. Calls `callback` with the result which should be appended to
  // the MHTML file.
  //
  // The same `boundary` needs to be used for all generateMHTMLHeader and
  // generateMHTMLParts and generateMHTMLFooter calls that belong to the same
  // MHTML document (see also rfc1341, section 7.2.1, "boundary" description).
  static void GenerateMHTMLParts(
      const WebString& boundary,
      WebLocalFrame*,
      MHTMLPartsGenerationDelegate*,
      base::OnceCallback<void(WebThreadSafeData)> callback);

  // IMPORTANT:
  // The API below is an older implementation of frame serialization that
  // will be removed soon.

  class LinkRewritingDelegate {
   public:
    // Method allowing the delegate control which URLs are written into the
    // generated html document.
    //
    // When URL of the given frame needs to be rewritten, this method should
    // return true and populate |rewrittenLink| with a desired value of the
    // html attribute value to be used in place of the original link (i.e.
    // in place of the original iframe.src or object.data attribute value).
    //
    // If no link rewriting is desired, this method should return false.
    virtual bool RewriteFrameSource(WebFrame*, WebString* rewritten_link) = 0;

    // Method allowing the delegate control which URLs are written into the
    // generated html document.
    //
    // When the given URL needs to be rewritten, this method should
    // return true and populate |rewrittenLink| with a desired value of the
    // html attribute value to be used in place of the original link (i.e.
    // in place of the original img.src or blockquote.cite attribute value).
    //
    // If no link rewriting is desired, this method should return false.
    virtual bool RewriteLink(const WebURL&, WebString* rewritten_link) = 0;
  };

  // This function will serialize the specified frame to HTML data.
  // We have a data buffer to temporary saving generated html data. We will
  // sequentially call WebFrameSerializerClient once the data buffer is full.
  //
  // False is returned if no data has been serialized (i.e. because
  // the target frame didn't have a valid url).
  static bool Serialize(WebLocalFrame*,
                        WebFrameSerializerClient*,
                        LinkRewritingDelegate*,
                        bool);

  // FIXME: The following are here for unit testing purposes. Consider
  // changing the unit tests instead.

  // Generate the META for charset declaration.
  static WebString GenerateMetaCharsetDeclaration(const WebString& charset);
  // Generate the MOTW declaration.
  static WebString GenerateMarkOfTheWebDeclaration(const WebURL&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_SERIALIZER_H_
