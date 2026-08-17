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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_H_

#include "base/unguessable_token.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/frame_replication_state.mojom-forward.h"
#include "third_party/blink/public/mojom/frame/tree_scope_type.mojom-shared.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_node.h"
#include "v8/include/v8-forward.h"

namespace blink {

enum class DetachReason;

#if INSIDE_BLINK
class Frame;
#endif

class WebLocalFrame;
class WebRemoteFrame;
class WebSecurityOrigin;
class WebView;

// Frames may be rendered in process ('local') or out of process ('remote').
// A remote frame is always cross-site; a local frame may be either same-site or
// cross-site.
// WebFrame is the base class for both WebLocalFrame and WebRemoteFrame and
// contains methods that are valid on both local and remote frames, such as
// getting a frame's parent or its opener.
class BLINK_EXPORT WebFrame {
 public:
  // FIXME: We already have blink::TextGranularity. For now we support only
  // a part of blink::TextGranularity.
  // Ideally it seems blink::TextGranularity should be broken up into
  // blink::TextGranularity and perhaps blink::TextBoundary and then
  // TextGranularity enum could be moved somewhere to public/, and we could
  // just use it here directly without introducing a new enum.
  enum TextGranularity {
    kCharacterGranularity = 0,
    kWordGranularity,
    kTextGranularityLast = kWordGranularity,
  };

  // Returns the number of live WebFrame objects, used for leak checking.
  static int InstanceCount();

  static WebFrame* FromFrameToken(const FrameToken&);

  virtual bool IsWebLocalFrame() const = 0;
  virtual WebLocalFrame* ToWebLocalFrame() = 0;
  virtual const WebLocalFrame* ToWebLocalFrame() const = 0;
  virtual bool IsWebRemoteFrame() const = 0;
  virtual WebRemoteFrame* ToWebRemoteFrame() = 0;
  virtual const WebRemoteFrame* ToWebRemoteFrame() const = 0;

  bool Swap(WebLocalFrame* new_frame);
  bool Swap(
      WebRemoteFrame* new_frame,
      CrossVariantMojoAssociatedRemote<mojom::RemoteFrameHostInterfaceBase>
          remote_frame_host,
      CrossVariantMojoAssociatedReceiver<mojom::RemoteFrameInterfaceBase>
          receiver,
      mojom::FrameReplicationStatePtr replicated_state,
      const std::optional<base::UnguessableToken>& devtools_frame_token);

  // This method closes and deletes the WebFrame. This is typically called by
  // the embedder in response to a frame detached callback to the WebFrame
  // client.
  virtual void Close(DetachReason detach_reason);

  // Called by the embedder when it needs to detach the subtree rooted at this
  // frame.
  void Detach();

  // Basic properties ---------------------------------------------------

  // The security origin of this frame.
  WebSecurityOrigin GetSecurityOrigin() const;

  // The frame's insecure request policy.
  mojom::InsecureRequestPolicy GetInsecureRequestPolicy() const;

  // The frame's upgrade insecure navigations set.
  std::vector<unsigned> GetInsecureRequestToUpgrade() const;

  // Hierarchy ----------------------------------------------------------

  // Returns the containing view.
  virtual WebView* View() const = 0;

  // Returns the frame that opened this frame or 0 if there is none.
  WebFrame* Opener() const;

  // Reset the frame that opened this frame to 0.
  // This is executed between web tests runs
  void ClearOpener();

  // Returns the parent frame or 0 if this is a top-most frame.
  WebFrame* Parent() const;

  // Returns the top-most frame in the hierarchy containing this frame.
  WebFrame* Top() const;

  // Returns the first child frame.
  WebFrame* FirstChild() const;

  // Returns the last child frame.
  WebFrame* LastChild() const;

  // Returns the next sibling frame.
  WebFrame* NextSibling() const;

  // Returns the previous sibling frame.
  WebFrame* PreviousSibling() const;

  // Returns the next frame in "frame traversal order".
  WebFrame* TraverseNext() const;

  // Returns true if this frame is the top-level main frame (associated with
  // the root Document in a WebContents). See content::Page for detailed
  // documentation.
  // This is false for main frames created for fenced-frames.
  bool IsOutermostMainFrame() const;

  // Scripting ----------------------------------------------------------

  // Returns the global proxy object.
  virtual v8::Local<v8::Object> GlobalProxy(v8::Isolate* isolate) const = 0;

  // Returns true if the WebFrame currently executing JavaScript has access
  // to the given WebFrame, or false otherwise.
  static bool ScriptCanAccess(v8::Isolate* isolate, WebFrame*);

  // Navigation ----------------------------------------------------------

  // Will return true if between didStartLoading and didStopLoading
  // notifications.
  virtual bool IsLoading() const;

  // Ad Tagging ---------------------------------------------------------

  // True if the frame is thought (heuristically) to be created for
  // advertising purposes.
  virtual bool IsAdFrame() const = 0;

  // Utility -------------------------------------------------------------

  // Returns the frame inside a given frame or iframe element. Returns 0 if
  // the given node is not a frame, iframe or if the frame is empty.
  static WebFrame* FromFrameOwnerElement(const WebNode&);

  // Whether the owner element of this frame is in the document tree or the
  // shadow tree, per https://w3c.github.io/webcomponents/spec/shadow/.
  mojom::TreeScopeType GetTreeScopeType() const { return scope_; }

  // This identifier represents the stable identifier between a
  // LocalFrame  <--> RenderFrameHostImpl or a
  // RemoteFrame <--> RenderFrameProxyHost in the browser process.
  const FrameToken& GetFrameToken() const { return frame_token_; }

#if INSIDE_BLINK
  static WebFrame* FromCoreFrame(Frame*);
  static Frame* ToCoreFrame(const WebFrame&);
#endif

 protected:
  explicit WebFrame(mojom::TreeScopeType, const FrameToken& frame_token);
  virtual ~WebFrame() = default;

 private:
  const mojom::TreeScopeType scope_;

  // See blink::Frame::frame_token_ for comments.
  // TODO(dtapuska): Remove the need for this variable. This is stored here
  // because a WebRemote's core frame is created inside the bowels of the Swap
  // call.
  const FrameToken frame_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_H_
