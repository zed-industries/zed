/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INSPECTOR_INDEXED_DB_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INSPECTOR_INDEXED_DB_AGENT_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/indexed_db.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

#include <v8-inspector.h>

namespace blink {

class InspectedFrames;

class MODULES_EXPORT InspectorIndexedDBAgent final
    : public InspectorBaseAgent<protocol::IndexedDB::Metainfo> {
 public:
  InspectorIndexedDBAgent(InspectedFrames*, v8_inspector::V8InspectorSession*);
  ~InspectorIndexedDBAgent() override;
  void Trace(Visitor*) const override;

  void Restore() override;
  void DidCommitLoadForLocalFrame(LocalFrame*) override;

  // Called from the front-end.
  protocol::Response enable() override;
  protocol::Response disable() override;
  void requestDatabaseNames(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      std::unique_ptr<RequestDatabaseNamesCallback>) override;
  void requestDatabase(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> in_storageBucket,
      const String& database_name,
      std::unique_ptr<RequestDatabaseCallback>) override;
  void requestData(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      const String& database_name,
      const String& object_store_name,
      const String& index_name,
      int skip_count,
      int page_size,
      std::unique_ptr<protocol::IndexedDB::KeyRange>,
      std::unique_ptr<RequestDataCallback>) override;
  void getMetadata(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      const String& database_name,
      const String& object_store_name,
      std::unique_ptr<GetMetadataCallback>) override;
  void deleteObjectStoreEntries(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      const String& database_name,
      const String& object_store_name,
      std::unique_ptr<protocol::IndexedDB::KeyRange>,
      std::unique_ptr<DeleteObjectStoreEntriesCallback>) override;
  void clearObjectStore(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      const String& database_name,
      const String& object_store_name,
      std::unique_ptr<ClearObjectStoreCallback>) override;
  void deleteDatabase(
      std::optional<String> security_origin,
      std::optional<String> storage_key,
      std::unique_ptr<protocol::Storage::StorageBucket> storage_bucket,
      const String& database_name,
      std::unique_ptr<DeleteDatabaseCallback>) override;

 private:
  Member<InspectedFrames> inspected_frames_;
  raw_ptr<v8_inspector::V8InspectorSession, DanglingUntriaged> v8_session_;
  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INSPECTOR_INDEXED_DB_AGENT_H_
