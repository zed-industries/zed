/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_ITEM_H_

#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_data_transfer_token.mojom-blink.h"
#include "third_party/blink/public/platform/web_drag_data.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fileapi/file.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace blink {

class SystemClipboard;

class CORE_EXPORT DataObjectItem final
    : public GarbageCollected<DataObjectItem> {
 public:
  enum ItemKind { kStringKind, kFileKind };

  static DataObjectItem* CreateFromString(const String& type,
                                          const String& data);
  static DataObjectItem* CreateFromFile(File*);
  // File with non-empty filesystem ID can be converted into FileEntry by using
  // webkitGetAsEntry.
  static DataObjectItem* CreateFromFileWithFileSystemId(
      File*,
      const String& file_system_id,
      scoped_refptr<FileSystemAccessDropData> file_system_access_entry =
          nullptr);
  static DataObjectItem* CreateFromURL(const String& url, const String& title);
  static DataObjectItem* CreateFromHTML(const String& html,
                                        const KURL& base_url);
  static DataObjectItem* CreateFromFileSharedBuffer(
      scoped_refptr<SharedBuffer>,
      bool is_image_accessible,
      const KURL&,
      const String& file_extension,
      const AtomicString& content_disposition);
  static DataObjectItem* CreateFromClipboard(
      SystemClipboard* system_clipboard,
      const String& type,
      const ClipboardSequenceNumberToken& sequence_number);

  DataObjectItem(ItemKind kind, const String& type);
  DataObjectItem(ItemKind,
                 const String& type,
                 const ClipboardSequenceNumberToken& sequence_number,
                 SystemClipboard* system_clipboard);

  ItemKind Kind() const { return kind_; }
  String GetType() const { return type_; }
  String GetAsString() const;
  File* GetAsFile() const;

  // Used to support legacy DataTransfer APIs and renderer->browser
  // serialization.
  scoped_refptr<SharedBuffer> GetSharedBuffer() const { return shared_buffer_; }
  bool IsImageAccessible() const { return is_image_accessible_; }
  String FilenameExtension() const { return filename_extension_; }
  String Title() const { return title_; }
  KURL BaseURL() const { return base_url_; }
  bool IsFilename() const;

  bool HasFileSystemId() const;
  String FileSystemId() const;

  bool HasFileSystemAccessEntry() const;
  mojo::PendingRemote<mojom::blink::FileSystemAccessDataTransferToken>
  CloneFileSystemAccessEntryToken() const;

  void Trace(Visitor*) const;

 private:
  enum class DataSource {
    kClipboardSource,
    kInternalSource,
  };

  scoped_refptr<FileSystemAccessDropData> file_system_access_entry_;
  DataSource source_;
  ItemKind kind_;
  String type_;

  String data_;
  Member<File> file_;
  scoped_refptr<SharedBuffer> shared_buffer_;
  bool is_image_accessible_ = false;
  // Optional metadata. Currently used for URL, HTML, and dragging files in.
  String filename_extension_;
  String title_;
  KURL base_url_;

  ClipboardSequenceNumberToken  // Only valid when |source_| ==
      sequence_number_;         // DataSource::kClipboardSource.
  String file_system_id_;  // Only valid when |file_| is backed by FileEntry.

  // Access to the global system clipboard.
  Member<SystemClipboard> system_clipboard_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_ITEM_H_
