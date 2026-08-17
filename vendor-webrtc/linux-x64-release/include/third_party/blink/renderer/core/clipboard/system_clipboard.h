// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_SYSTEM_CLIPBOARD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_SYSTEM_CLIPBOARD_H_

#include <memory>
#include <optional>

#include "third_party/blink/public/mojom/clipboard/clipboard.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DataObject;
class Image;
class KURL;
class LocalFrame;
class ScopedSystemClipboardSnapshot;

// SystemClipboard:
// - is a LocalFrame bounded object.
// - provides sanitized, platform-neutral read/write access to the clipboard.
// - mediates between core classes and mojom::ClipboardHost.
//
// All calls to write functions must be followed by a call to CommitWrite().
class CORE_EXPORT SystemClipboard final
    : public GarbageCollected<SystemClipboard> {
 public:
  enum SmartReplaceOption { kCanSmartReplace, kCannotSmartReplace };

  explicit SystemClipboard(LocalFrame* frame);
  SystemClipboard(const SystemClipboard&) = delete;
  SystemClipboard& operator=(const SystemClipboard&) = delete;

  ClipboardSequenceNumberToken SequenceNumber();
  bool IsSelectionMode() const;
  void SetSelectionMode(bool);
  Vector<String> ReadAvailableTypes();
  bool IsFormatAvailable(mojom::ClipboardFormat format);

  String ReadPlainText();
  String ReadPlainText(mojom::blink::ClipboardBuffer buffer);
  void ReadPlainText(mojom::blink::ClipboardBuffer buffer,
                     mojom::blink::ClipboardHost::ReadTextCallback callback);
  void WritePlainText(const String&, SmartReplaceOption = kCannotSmartReplace);

  // If no data is read, an empty string will be returned and all out parameters
  // will be cleared. If applicable, the page URL will be assigned to the KURL
  // parameter. fragmentStart and fragmentEnd are indexes into the returned
  // markup that indicate the start and end of the returned markup. If there is
  // no additional context, fragmentStart will be zero and fragmentEnd will be
  // the same as the length of the markup.
  String ReadHTML(KURL& url, unsigned& fragment_start, unsigned& fragment_end);
  void ReadHTML(mojom::blink::ClipboardHost::ReadHtmlCallback callback);
  void WriteHTML(const String& markup,
                 const KURL& document_url,
                 SmartReplaceOption = kCannotSmartReplace);

  void ReadSvg(mojom::blink::ClipboardHost::ReadSvgCallback callback);
  void WriteSvg(const String& markup);

  String ReadRTF();

  mojo_base::BigBuffer ReadPng(mojom::blink::ClipboardBuffer);
  String ReadImageAsImageMarkup(mojom::blink::ClipboardBuffer);

  // Write the image and its associated tag (bookmark/HTML types).
  void WriteImageWithTag(Image*, const KURL&, const String& title);
  // Write the image only.
  void WriteImage(const SkBitmap&);

  // Read files.
  mojom::blink::ClipboardFilesPtr ReadFiles();

  String ReadDataTransferCustomData(const String& type);
  void WriteDataObject(DataObject*);

  // Clipboard write functions must use CommitWrite for changes to reach
  // the OS clipboard.
  void CommitWrite();

  void CopyToFindPboard(const String& text);

  void ReadAvailableCustomAndStandardFormats(
      mojom::blink::ClipboardHost::ReadAvailableCustomAndStandardFormatsCallback
          callback);
  void ReadUnsanitizedCustomFormat(
      const String& type,
      mojom::blink::ClipboardHost::ReadUnsanitizedCustomFormatCallback
          callback);

  void WriteUnsanitizedCustomFormat(const String& type,
                                    mojo_base::BigBuffer data);

  void Trace(Visitor*) const;

 private:
  friend class ScopedSystemClipboardSnapshot;

  // When in scope, forces the specified system clipboard to take a snapshot
  // of its current state and return those value for the ReadXXX methods.
  // Calling WriteXXX methods on the specified clipboard are not allowed and
  // will DCHECK.
  class CORE_EXPORT Snapshot {
   public:
    explicit Snapshot();
    ~Snapshot();

    // The methods below are an API intended for only SystemClipboard.  Each
    // data type XXX has three basic methods:
    //
    //   HasXXX(): whether this snapshot object has cached a value for XXX.
    //   SetXXX(): sets the value in the snapshot for data type XXX.
    //   XXX(): gets the value in the snapshot for data type XXX.

    bool HasPlainText(mojom::blink::ClipboardBuffer buffer) const;
    const String& PlainText(mojom::blink::ClipboardBuffer buffer) const;
    void SetPlainText(mojom::blink::ClipboardBuffer buffer, const String& text);

    bool HasHtml(mojom::blink::ClipboardBuffer buffer) const;
    const KURL& Url(mojom::blink::ClipboardBuffer buffer) const;
    unsigned FragmentStart(mojom::blink::ClipboardBuffer buffer) const;
    unsigned FragmentEnd(mojom::blink::ClipboardBuffer buffer) const;
    const String& Html(mojom::blink::ClipboardBuffer buffer) const;
    void SetHtml(mojom::blink::ClipboardBuffer buffer,
                 const String& html,
                 const KURL& url,
                 unsigned fragment_start,
                 unsigned fragment_end);

    bool HasRtf(mojom::blink::ClipboardBuffer buffer) const;
    const String& Rtf(mojom::blink::ClipboardBuffer buffer) const;
    void SetRtf(mojom::blink::ClipboardBuffer buffer, const String& rtf);

    bool HasPng(mojom::blink::ClipboardBuffer buffer) const;
    mojo_base::BigBuffer Png(mojom::blink::ClipboardBuffer buffer) const;
    void SetPng(mojom::blink::ClipboardBuffer buffer,
                const mojo_base::BigBuffer& png);

    bool HasFiles(mojom::blink::ClipboardBuffer buffer) const;
    mojom::blink::ClipboardFilesPtr Files(
        mojom::blink::ClipboardBuffer buffer) const;
    // Because of the special handling needed to clone ClipboardFilesPtr,
    // SetFiles() needs to change the function argument `files`.
    void SetFiles(mojom::blink::ClipboardBuffer buffer,
                  mojom::blink::ClipboardFilesPtr& files);

    bool HasCustomData(mojom::blink::ClipboardBuffer buffer,
                       const String& type) const;
    String CustomData(mojom::blink::ClipboardBuffer buffer,
                      const String& type) const;
    void SetCustomData(mojom::blink::ClipboardBuffer buffer,
                       const String& type,
                       const String& data);

    static mojom::blink::ClipboardFilesPtr CloneFiles(
        mojom::blink::ClipboardFilesPtr& files);

   private:
    // Called in the set methods to bind this snapshot to the specified buffer.
    // All calls to set data for all types need to specify the same buffer.
    void BindToBuffer(mojom::blink::ClipboardBuffer buffer);

    std::optional<mojom::blink::ClipboardBuffer> buffer_;

    std::optional<String> plain_text_;

    std::optional<String> html_;
    KURL url_;
    unsigned fragment_start_ = 0;
    unsigned fragment_end_ = 0;

    std::optional<String> rtf_;

    std::optional<mojo_base::BigBuffer> png_;

    mutable std::optional<mojom::blink::ClipboardFilesPtr> files_;

    WTF::HashMap<String, String> custom_data_;
  };

  bool IsValidBufferType(mojom::blink::ClipboardBuffer);

  // Methods to enter and leave snapshot mode.  Only the
  // ScopedSystemClipboardSnapshot calls these methods.
  void TakeSnapshot();
  void DropSnapshot();

  HeapMojoRemote<mojom::blink::ClipboardHost> clipboard_;
  // In some Linux environments, |buffer_| may equal ClipboardBuffer::kStandard
  // or kSelection.  In other platforms |buffer_| always equals
  // ClipboardBuffer::kStandard.
  mojom::blink::ClipboardBuffer buffer_ =
      mojom::blink::ClipboardBuffer::kStandard;

  // Whether the selection buffer is available on the underlying platform.
  bool is_selection_buffer_available_ = false;

  // When non-null, the system clipboard uses the cached value in the snapshot
  // rather than making calls to the browser process via the clipboard host.
  // Snapshots can be nested and `snapshot_count_` counts the number of
  // nestings.  The snapshot is created when the first call to TakeSnapshot()
  // is made and released once an equal number of DropSnapshot()s have been
  // made.
  std::unique_ptr<Snapshot> snapshot_;
  size_t snapshot_count_ = 0;
  // Declared SystemClipboardTest class as friend to access the private members
  // of this class as we need to use clipboard_ and buffer_ for unbound remote
  // tests.
  friend class SystemClipboardTest;
};

// When in scope, forces the specified system clipboard to take a snapshot
// of its current state and return those value for the ReadXXX methods.  Calling
// WriteXXX methods on the specified clipboard are not allowed and will DCHECK.
class CORE_EXPORT ScopedSystemClipboardSnapshot {
  STACK_ALLOCATED();

 public:
  explicit ScopedSystemClipboardSnapshot(SystemClipboard& clipboard);
  ~ScopedSystemClipboardSnapshot();

 private:
  SystemClipboard& clipboard_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_SYSTEM_CLIPBOARD_H_
