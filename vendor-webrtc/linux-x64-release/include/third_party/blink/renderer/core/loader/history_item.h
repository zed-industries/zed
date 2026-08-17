/*
 * Copyright (C) 2006, 2008, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2012 Research In Motion Limited. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HISTORY_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HISTORY_ITEM_H_

#include <optional>
#include <string>
#include <vector>

#include "third_party/blink/public/mojom/page_state/page_state.mojom-blink.h"
#include "third_party/blink/public/platform/web_scroll_anchor_data.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DocumentState;
class EncodedFormData;
class KURL;
class PageState;
class ResourceRequest;
class SerializedScriptValue;

namespace mojom {
enum class FetchCacheMode : int32_t;
}  // namespace mojom

class CORE_EXPORT HistoryItem final : public GarbageCollected<HistoryItem> {
 public:
  static HistoryItem* Create(const PageState&);

  HistoryItem();
  ~HistoryItem();

  const String& UrlString() const;
  KURL Url() const;

  const String& GetReferrer() const;
  network::mojom::ReferrerPolicy GetReferrerPolicy() const;

  const String& Target() const { return target_; }

  // TODO(dcheng): Try to make this const.
  EncodedFormData* FormData() const;
  const AtomicString& FormContentType() const;

  class ViewState {
    DISALLOW_NEW();

   public:
    ViewState() = default;
    ViewState(const ViewState&) = default;
    ViewState& operator=(const ViewState&) = default;

    ScrollOffset visual_viewport_scroll_offset_;
    ScrollOffset scroll_offset_;
    float page_scale_factor_ = 0;
    ScrollAnchorData scroll_anchor_data_;
  };

  const std::optional<ViewState>& GetViewState() const { return view_state_; }
  void ClearViewState() { view_state_.reset(); }
  void CopyViewStateFrom(HistoryItem* other) {
    view_state_ = other->GetViewState();
  }

  void SetVisualViewportScrollOffset(const ScrollOffset&);
  void SetScrollOffset(const ScrollOffset&);
  void SetPageScaleFactor(float);

  Vector<String> GetReferencedFilePaths() const;
  const Vector<String>& GetDocumentState() const;
  void SetDocumentState(const Vector<String>&);
  void SetDocumentState(DocumentState*);
  void ClearDocumentState();

  void SetURL(const KURL&);
  void SetURLString(const String&);
  void SetReferrer(const String&);
  void SetReferrerPolicy(network::mojom::ReferrerPolicy);
  void SetTarget(const String& target) { target_ = target; }

  void SetStateObject(scoped_refptr<SerializedScriptValue>);
  SerializedScriptValue* StateObject() const { return state_object_.get(); }

  void SetItemSequenceNumber(int64_t number) { item_sequence_number_ = number; }
  int64_t ItemSequenceNumber() const { return item_sequence_number_; }

  void SetDocumentSequenceNumber(int64_t number) {
    document_sequence_number_ = number;
  }
  int64_t DocumentSequenceNumber() const { return document_sequence_number_; }

  void SetScrollRestorationType(mojom::blink::ScrollRestorationType type) {
    scroll_restoration_type_ = type;
  }
  mojom::blink::ScrollRestorationType ScrollRestorationType() const {
    return scroll_restoration_type_;
  }

  void SetScrollAnchorData(const ScrollAnchorData&);

  void SetFormData(scoped_refptr<EncodedFormData>);
  void SetFormContentType(const AtomicString&);

  ResourceRequest GenerateResourceRequest(mojom::FetchCacheMode);

  const String& GetNavigationApiKey() const { return navigation_api_key_; }
  void SetNavigationApiKey(const String& key) { navigation_api_key_ = key; }

  const String& GetNavigationApiId() const { return navigation_api_id_; }
  void SetNavigationApiId(const String& id) { navigation_api_id_ = id; }

  void SetNavigationApiState(scoped_refptr<SerializedScriptValue>);
  // TODO(dcheng): Try to make this const.
  SerializedScriptValue* GetNavigationApiState() const {
    return navigation_api_state_.get();
  }

  PageState ToPageState() const;

  void Trace(Visitor*) const;

 private:
  std::vector<std::optional<std::u16string>>
  GetReferencedFilePathsForSerialization() const;

  ViewState& GetOrCreateViewState();

  String url_string_;

  // The referrer provided when this item was originally requested.
  String referrer_;

  // The referrer policy of the document this item represents.
  network::mojom::ReferrerPolicy referrer_policy_ =
      network::mojom::ReferrerPolicy::kDefault;

  String target_;

  mutable Vector<String> document_state_vector_;
  Member<DocumentState> document_state_;

  std::optional<ViewState> view_state_;

  // If two HistoryItems have the same item sequence number, then they are
  // clones of one another. Traversing history from one such HistoryItem to
  // another is a no-op. HistoryItem clones are created for parent and
  // sibling frames when only a subframe navigates.
  int64_t item_sequence_number_;

  // If two HistoryItems have the same document sequence number, then they
  // refer to the same instance of a document. Traversing history from one
  // such HistoryItem to another preserves the document.
  int64_t document_sequence_number_;

  // Type of the scroll restoration for the history item determines if scroll
  // position should be restored when it is loaded during history traversal.
  mojom::blink::ScrollRestorationType scroll_restoration_type_ =
      mojom::blink::ScrollRestorationType::kAuto;

  // Support for HTML5 History
  scoped_refptr<SerializedScriptValue> state_object_;

  // info used to repost form data
  scoped_refptr<EncodedFormData> form_data_;
  AtomicString form_content_type_;

  String navigation_api_key_;
  String navigation_api_id_;
  scoped_refptr<SerializedScriptValue> navigation_api_state_;
};  // class HistoryItem

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HISTORY_ITEM_H_
