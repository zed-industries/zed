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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_HISTORY_ITEM_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_HISTORY_ITEM_H_

#include "third_party/blink/public/mojom/navigation/navigation_api_history_entry_arrays.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

class HistoryItem;
class PageState;
class WebHTTPBody;

// Represents a frame-level navigation entry in session history.
//
// Copying a WebHistoryItem is cheap.
//
class BLINK_EXPORT WebHistoryItem {
 public:
  ~WebHistoryItem() { Reset(); }
  WebHistoryItem() = default;
  explicit WebHistoryItem(const PageState& page_state);
  WebHistoryItem(const WebHistoryItem& h) { Assign(h); }
  WebHistoryItem& operator=(const WebHistoryItem& h) {
    Assign(h);
    return *this;
  }

  // The navigation API uses partially-initialized items for non-current
  // entries via this constructor.
  WebHistoryItem(const WebString& url,
                 const WebString& navigation_api_key,
                 const WebString& navigation_api_id,
                 int64_t item_sequence_number,
                 int64_t document_sequence_number,
                 const WebString& navigation_api_state);

  bool IsNull() const { return private_.IsNull(); }

  int64_t ItemSequenceNumber() const;
  int64_t DocumentSequenceNumber() const;
  WebHTTPBody HttpBody() const;
  WebString GetNavigationApiKey() const;

#if INSIDE_BLINK
  WebHistoryItem(HistoryItem*);
  WebHistoryItem& operator=(HistoryItem*);
  operator HistoryItem*() const;
#endif

 private:
  void Reset();
  void Assign(const WebHistoryItem&);

  WebPrivatePtrForGC<HistoryItem> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_HISTORY_ITEM_H_
