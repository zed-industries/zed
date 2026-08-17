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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_ITEM_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_function_string_callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class DataObjectItem;
class DataTransfer;
class ExecutionContext;
class File;
class ScriptState;

namespace probe {
class AsyncTaskContext;
}

class CORE_EXPORT DataTransferItem final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DataTransferItem(DataTransfer*, DataObjectItem*);
  DataTransferItem(const DataTransferItem&) = delete;
  DataTransferItem& operator=(const DataTransferItem&) = delete;

  String kind() const;
  String type() const;

  void getAsString(ScriptState*, V8FunctionStringCallback*);
  File* getAsFile() const;

  DataTransfer* GetDataTransfer() { return data_transfer_.Get(); }
  DataObjectItem* GetDataObjectItem() { return item_.Get(); }

  void Trace(Visitor*) const override;

 private:
  void RunGetAsStringTask(ExecutionContext*,
                          V8FunctionStringCallback*,
                          const String& data,
                          std::unique_ptr<probe::AsyncTaskContext>);

  Member<DataTransfer> data_transfer_;
  Member<DataObjectItem> item_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_ITEM_H_
