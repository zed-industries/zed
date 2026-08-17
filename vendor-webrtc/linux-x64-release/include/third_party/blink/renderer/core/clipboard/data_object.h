/*
 * Copyright (c) 2008, 2009, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/clipboard/data_object_item.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class KURL;
class SystemClipboard;
class WebDragData;
class ExecutionContext;

enum class PasteMode;

// A data object for holding data that would be in a clipboard or moved
// during a drag-n-drop operation. This is the data that WebCore is aware
// of and is not specific to a platform.
class CORE_EXPORT DataObject : public GarbageCollected<DataObject>,
                               public Supplementable<DataObject> {
 public:
  struct CORE_EXPORT Observer : public GarbageCollectedMixin {
    // Called whenever |item_list_| is modified. Note it can be called multiple
    // times for a single mutation. For example, DataObject::SetData() calls
    // both ClearData() and Add(), each of which can call this method.
    virtual void OnItemListChanged() = 0;
  };

  static DataObject* CreateFromClipboard(ExecutionContext* context,
                                         SystemClipboard*,
                                         PasteMode);
  static DataObject* CreateFromClipboard(SystemClipboard*, PasteMode);
  static DataObject* CreateFromString(const String&);
  static DataObject* Create();
  static DataObject* Create(ExecutionContext* context, const WebDragData&);
  static DataObject* Create(const WebDragData&);

  DataObject();
  virtual ~DataObject();

  // DataTransferItemList support.
  uint32_t length() const;
  DataObjectItem* Item(uint32_t index);
  void DeleteItem(uint32_t index);
  void ClearAll();
  // Removes all the items from |item_list_| whose |kind_| is kStringKind.
  void ClearStringItems();
  // Returns null if an item already exists with the provided type.
  DataObjectItem* Add(const String& data, const String& type);
  DataObjectItem* Add(File*);
  DataObjectItem* Add(File*, const String& file_system_id);

  // WebCore helpers.
  void ClearData(const String& type);

  Vector<String> Types() const;
  String GetData(const String& type) const;
  void SetData(const String& type, const String& data);

  void UrlAndTitle(String& url, String* title = nullptr) const;
  void SetURLAndTitle(const String& url, const String& title);
  void HtmlAndBaseURL(String& html, KURL& base_url) const;
  void SetHTMLAndBaseURL(const String& html, const KURL& base_url);
  Vector<String> Urls() const;

  // Used for dragging in files from the desktop.
  bool ContainsFilenames() const;
  Vector<String> Filenames() const;
  void AddFilename(ExecutionContext* context,
                   const String& filename,
                   const String& display_name,
                   const String& file_system_id,
                   scoped_refptr<FileSystemAccessDropData>
                       file_system_access_entry = nullptr);

  // Used for dragging in filesystem from the desktop.
  void SetFilesystemId(const String& file_system_id) {
    filesystem_id_ = file_system_id;
  }
  const String& FilesystemId() const {
    DCHECK(!filesystem_id_.empty());
    return filesystem_id_;
  }

  // Used to handle files (images) being dragged out.
  void AddFileSharedBuffer(scoped_refptr<SharedBuffer>,
                           bool is_accessible_from_start_frame,
                           const KURL&,
                           const String& filename_extension,
                           const AtomicString& content_disposition);

  int GetModifiers() const { return modifiers_; }
  void SetModifiers(int modifiers) { modifiers_ = modifiers; }

  // Adds an observer (and retains a reference to it) that is notified
  // whenever the underlying item_list_ changes.
  void AddObserver(Observer*);

  void Trace(Visitor*) const override;

  WebDragData ToWebDragData();

 private:
  DataObjectItem* FindStringItem(const String& type) const;
  bool InternalAddStringItem(DataObjectItem*);
  void InternalAddFileItem(DataObjectItem*);

  void NotifyItemListChanged() const;

  HeapVector<Member<DataObjectItem>> item_list_;
  HeapHashSet<Member<Observer>> observers_;

  // State of Shift/Ctrl/Alt/Meta keys and Left/Right/Middle mouse buttons
  int modifiers_;
  String filesystem_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_OBJECT_H_
