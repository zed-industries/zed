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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DRAG_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DRAG_DATA_H_

#include <variant>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_data_transfer_token.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_blob_info.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_data.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

using FileSystemAccessDropData =
    base::RefCountedData<blink::CrossVariantMojoRemote<
        mojom::FileSystemAccessDataTransferTokenInterfaceBase>>;

// Holds data that may be exchanged through a drag-n-drop operation. It is
// inexpensive to copy a WebDragData object.
class BLINK_PLATFORM_EXPORT WebDragData {
 public:
  // String data with an associated MIME type. Depending on the MIME type,
  // there may be optional metadata attributes as well.
  struct StringItem {
    WebString type;
    WebString data;

    // Title associated with a link when type == "text/uri-list".
    WebString title;

    // Only valid when type == "text/html". Stores the base URL for the
    // contained markup.
    WebURL base_url;
  };

  // Represents one native file being dragged into the renderer.
  struct FilenameItem {
    WebString filename;
    WebString display_name;
    scoped_refptr<FileSystemAccessDropData> file_system_access_entry;
  };

  // An image being dragged out of the renderer. Contains a buffer holding
  // the image data as well as the suggested name for saving the image to.
  struct BinaryDataItem {
    WebData data;
    bool image_accessible;
    WebURL source_url;
    WebString filename_extension;
    WebString content_disposition;
  };

  // Stores the filesystem URL of one file being dragged into the renderer.
  struct FileSystemFileItem {
    WebURL url;
    int64_t size;
    WebString file_system_id;
    WebBlobInfo blob_info;
  };

  using Item = std::
      variant<StringItem, FilenameItem, BinaryDataItem, FileSystemFileItem>;

  WebDragData() = default;

  WebDragData(const WebDragData& object) = default;

  WebDragData& operator=(const WebDragData& object) = default;

  ~WebDragData() = default;

  const std::vector<Item>& Items() const { return item_list_; }

  void SetItems(std::vector<Item> item_list);

  void AddItem(const Item&);

  WebString FilesystemId() const { return filesystem_id_; }

  void SetFilesystemId(const WebString& filesystem_id) {
    // The ID is an opaque string, given by and validated by chromium port.
    filesystem_id_ = filesystem_id;
  }

  bool ForceDefaultAction() const { return force_default_action_; }

  void SetForceDefaultAction(bool force_default_action) {
    force_default_action_ = force_default_action;
  }

  network::mojom::ReferrerPolicy ReferrerPolicy() const {
    return referrer_policy_;
  }

  void SetReferrerPolicy(network::mojom::ReferrerPolicy referrer_policy) {
    referrer_policy_ = referrer_policy;
  }

 private:
  std::vector<Item> item_list_;
  WebString filesystem_id_;

  // If true, the renderer always performs the default action for the drop.
  // See DragData::force_default_action for complete details.
  bool force_default_action_ = false;

  // Used for items where string_type == "downloadurl". Stores the referrer
  // policy for usage when dragging a link out of the webview results in a
  // download.
  network::mojom::ReferrerPolicy referrer_policy_ =
      network::mojom::ReferrerPolicy::kDefault;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DRAG_DATA_H_
