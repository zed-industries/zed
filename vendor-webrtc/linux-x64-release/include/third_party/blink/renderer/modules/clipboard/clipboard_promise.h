// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_PROMISE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_PROMISE_H_

#include <utility>

#include "base/sequence_checker.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits_impl.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_blob_string.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/fileapi/blob.h"
#include "third_party/blink/renderer/modules/clipboard/clipboard_item.h"
#include "third_party/blink/renderer/modules/clipboard/clipboard_reader.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ClipboardWriter;
class LocalFrame;
class ExceptionState;
class ExecutionContext;
class ClipboardUnsanitizedFormats;

// Represents a promise to execute Async Clipboard API functions off the main
// thread. It handles read and write operations on the clipboard, including
// reading and writing text and blobs for different MIME types. This class also
// interacts with the `PermissionService` to check for read and write
// permissions. It uses a `ClipboardItem` object to read/write supported MIME
// types. Spec: https://w3c.github.io/clipboard-apis/#async-clipboard-api
class MODULES_EXPORT ClipboardPromise final
    : public GarbageCollected<ClipboardPromise>,
      public ExecutionContextLifecycleObserver {
 public:
  // Creates a promise for reading clipboard data.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboard-read
  // `formats`: Unsanitized formats to be read from the clipboard.
  // Spec:
  // https://w3c.github.io/clipboard-apis/#dom-clipboardunsanitizedformats-unsanitized
  static ScriptPromise<IDLSequence<ClipboardItem>> CreateForRead(
      ExecutionContext* execution_context,
      ScriptState* script_state,
      ClipboardUnsanitizedFormats* formats,
      ExceptionState& exception_state);

  // Creates a promise for reading plain text from the clipboard.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboard-readtext
  static ScriptPromise<IDLString> CreateForReadText(
      ExecutionContext* execution_context,
      ScriptState* script_state,
      ExceptionState& exception_state);

  // Creates a promise for writing supported MIME types to the clipboard.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboard-write
  static ScriptPromise<IDLUndefined> CreateForWrite(
      ExecutionContext* execution_context,
      ScriptState* script_state,
      const HeapVector<Member<ClipboardItem>>& items,
      ExceptionState& exception_state);

  // Creates a promise for writing text to the clipboard.
  // `text`: The text to be written to the clipboard.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboard-writetext
  static ScriptPromise<IDLUndefined> CreateForWriteText(
      ExecutionContext* execution_context,
      ScriptState* script_state,
      const String& text,
      ExceptionState& exception_state);

  // Use one of the above factories to construct. This ctor is public for
  // `MakeGarbageCollected<>`.
  ClipboardPromise(ExecutionContext* execution_context,
                   ScriptPromiseResolverBase*,
                   ExceptionState& exception_state);
  ClipboardPromise(const ClipboardPromise&) = delete;
  ClipboardPromise& operator=(const ClipboardPromise&) = delete;
  ~ClipboardPromise() override;

  // Finishes writing the current representation and prepares for the next one.
  void CompleteWriteRepresentation();

  // Handles rejections originating from the ClipboardWriter.
  void RejectFromReadOrDecodeFailure();

  // Adds the given `blob` to the `clipboard_item_data_`.
  void OnRead(Blob* blob);

  // Returns the local frame associated with the promise.
  LocalFrame* GetLocalFrame() const;

  // Returns the script state associated with the promise.
  ScriptState* GetScriptState() const;

  // ExecutionContextLifecycleObserver
  void Trace(Visitor* visitor) const override;

 private:
  class ClipboardItemDataPromiseFulfill;
  class ClipboardItemDataPromiseReject;

  void HandlePromiseWrite(
      GCedHeapVector<Member<V8UnionBlobOrString>>* clipboard_item_list);
  void WriteClipboardItemData(
      GCedHeapVector<Member<V8UnionBlobOrString>>* clipboard_item_list);

  // Rejects the promise for blobs that have invalid MIME types or got rejected
  // with the given exception.
  void RejectClipboardItemPromise(ScriptValue);
  void WriteNextRepresentation();

  // Checks Read/Write permission (interacting with `PermissionService`).
  void HandleRead(ClipboardUnsanitizedFormats* formats);
  void HandleReadText();
  void HandleWrite(const HeapVector<Member<ClipboardItem>>& items);
  void HandleWriteText(const String& text);

  // Reads/Writes after permission check.
  void HandleReadWithPermission(mojom::blink::PermissionStatus permission);
  void HandleReadTextWithPermission(mojom::blink::PermissionStatus permission);
  void HandleWriteWithPermission(mojom::blink::PermissionStatus permission);
  void HandleWriteTextWithPermission(mojom::blink::PermissionStatus permission);

  // Callback function called when the available format names for reading are
  // received from the clipboard.
  // `format_names`: The available format names on the clipboard
  void OnReadAvailableFormatNames(const Vector<String>& format_names);

  // Reads the next clipboard representation.
  void ReadNextRepresentation();

  // Resolves the read promise.
  void ResolveRead();

  // Returns the `PermissionService` associated with the promise, or nullptr if
  // the remote connection fails.
  mojom::blink::PermissionService* GetPermissionService();

  // Validates that the action may proceed, including but not limited to
  // requesting permissions from the `PermissionService` as necessary.
  // On failure, will reject via `script_promise_resolver_`.
  //
  // `permission`: The permission to request.
  // `will_be_sanitized`: Whether the data will be sanitized.
  // `callback`: The callback function to be called with the permission status.
  void ValidatePreconditions(
      mojom::blink::PermissionName permission,
      bool will_be_sanitized,
      base::OnceCallback<void(mojom::blink::PermissionStatus)> callback);

  scoped_refptr<base::SingleThreadTaskRunner> GetClipboardTaskRunner();

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  Member<ScriptPromiseResolverBase> script_promise_resolver_;
  Member<ClipboardWriter> clipboard_writer_;
  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
  // When true, the HTML data read from the clipboard will be unprocessed.
  // When false, the HTML data is processed by the fragment parser.
  bool will_read_unprocessed_html_ = false;
  // Plain text data to be written to the clipboard.
  String plain_text_;
  // The list of formats read from the clipboard.
  HeapVector<std::pair<String, Member<V8UnionBlobOrString>>>
      clipboard_item_data_;
  // The list of formats with their corresponding promises to the Blob data to
  // be written to the clipboard.
  HeapVector<std::pair<String, MemberScriptPromise<V8UnionBlobOrString>>>
      clipboard_item_data_with_promises_;
  wtf_size_t clipboard_representation_index_ = 0;
  // List of custom format with "web " prefix.
  Vector<String> write_custom_format_types_;
  // Stores the types provided by the web authors.
  Vector<String> write_clipboard_item_types_;
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_PROMISE_H_
