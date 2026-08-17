// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SCRIPT_CACHED_METADATA_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SCRIPT_CACHED_METADATA_HANDLER_H_

#include <memory>

#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/loader/fetch/cached_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class CachedMetadata;
class CachedMetadataSender;

// ScriptCachedMetadataHandler should be created when a response is received,
// and can be used independently from its Resource.
// - It doesn't have any references to the Resource. Necessary data are captured
// from the Resource when the handler is created.
// - It is not affected by Resource's revalidation on MemoryCache. The validity
// of the handler is solely checked by |response_url_| and |response_time_|
// (not by Resource) by the browser process, and the cached metadata written to
// the handler is rejected if e.g. the disk cache entry has been updated and the
// handler refers to an older response.
class PLATFORM_EXPORT ScriptCachedMetadataHandler
    : public CachedMetadataHandler {
 public:
  ScriptCachedMetadataHandler(const WTF::TextEncoding& encoding,
                              std::unique_ptr<CachedMetadataSender> sender);
  ~ScriptCachedMetadataHandler() override;

  // CachedMetadataHandler:
  void Trace(Visitor*) const override;
  void SetCachedMetadata(CodeCacheHost* code_cache_host,
                         uint32_t data_type_id,
                         base::span<const uint8_t> data) override;
  void SetSerializedCachedMetadata(mojo_base::BigBuffer data) override;
  void ClearCachedMetadata(CodeCacheHost* code_cache_host,
                           ClearCacheType cache_type) override;
  scoped_refptr<CachedMetadata> GetCachedMetadata(
      uint32_t data_type_id,
      GetCachedMetadataBehavior behavior) const override;
  // This returns the encoding at the time of ResponseReceived(). Therefore this
  // does NOT reflect encoding detection from body contents, but the actual
  // encoding after the encoding detection can be determined uniquely from
  // Encoding(), provided the body content is the same, as we can assume the
  // encoding detection will result in the same final encoding.
  // TODO(hiroshige): Make these semantics cleaner.
  String Encoding() const override;
  ServingSource GetServingSource() const override;
  void OnMemoryDump(WebProcessMemoryDump* pmd,
                    const String& dump_prefix) const override;
  size_t GetCodeCacheSize() const override;

 protected:
  // Asks sender to send `cached_metadata_` to the `code_cache_host`. Virtual to
  // allow subclasses to append additional header metadata.
  virtual void CommitToPersistentStorage(CodeCacheHost* code_cache_host);

  CachedMetadataSender* sender() { return sender_.get(); }

  // Direct accessors for `cached_metadata_` by ScriptCachedMetadataHandler and
  // its subclasses.
  // The public `GetCachedMetadata()` and `SetCachedMetadata()` accessors should
  // be used to get the managed CachedMetadata by clients and incorporate the
  // appropriate data type and behavior checks.
  const scoped_refptr<CachedMetadata>& cached_metadata() const {
    return cached_metadata_;
  }
  void set_cached_metadata(scoped_refptr<CachedMetadata> cached_metadata) {
    cached_metadata_ = std::move(cached_metadata);
  }

 private:
  friend class ModuleScriptTest;

  bool cached_metadata_discarded_ = false;
  std::unique_ptr<CachedMetadataSender> sender_;
  const WTF::TextEncoding encoding_;
  scoped_refptr<CachedMetadata> cached_metadata_;
};

class PLATFORM_EXPORT ScriptCachedMetadataHandlerWithHashing final
    : public ScriptCachedMetadataHandler {
 public:
  static constexpr uint32_t kSha256Bytes = 256 / 8;

  ScriptCachedMetadataHandlerWithHashing(
      const WTF::TextEncoding& encoding,
      std::unique_ptr<CachedMetadataSender> sender);
  ~ScriptCachedMetadataHandlerWithHashing() override = default;

  // ScriptCachedMetadataHandler:
  void SetSerializedCachedMetadata(mojo_base::BigBuffer data) override;
  bool HashRequired() const override { return true; }
  scoped_refptr<CachedMetadata> GetCachedMetadata(
      uint32_t data_type_id,
      GetCachedMetadataBehavior behavior) const override;
  void Check(CodeCacheHost* code_cache_host,
             const ParkableString& source_text) override;

  // Pretend that the current content and hash were loaded from disk, not
  // created by the current process.
  void ResetForTesting();

  // Get the serialized representation of the cached metadata with appropriate
  // with-hashing-specific headers.
  Vector<uint8_t> GetSerializedCachedMetadata() const;

 protected:
  // ScriptCachedMetadataHandler:
  void CommitToPersistentStorage(CodeCacheHost*) override;

 private:
  uint8_t hash_[kSha256Bytes] = {0};

  enum HashState {
    kUninitialized,  // hash_ has not been written.
    kDeserialized,   // hash_ contains data from the code cache that has not yet
                     // been checked for matching the script text.
    kChecked,        // hash_ contains the hash of the script text. Neither
                     // hash_state_ nor hash_ will ever change again.
  };

  HashState hash_state_ = kUninitialized;
};

// The serialized header format for cached metadata which includes a hash of the
// script text. This header is followed by a second header in the format defined
// by CachedMetadataHeader, which is in turn followed by the data.
struct CachedMetadataHeaderWithHash {
  // Only valid value is CachedMetadataHandler::kSingleEntryWithHashAndPadding.
  // Default-initialized to an invalid value to ensure headers are explicitly
  // initialized as valid where appropriate.
  uint32_t marker = 0;
  // Always zero. Ensures 8-byte alignment of subsequent data so that reading
  // the tag is defined behavior and V8 doesn't copy the data again for better
  // alignment (see AlignedCachedData).
  uint32_t padding = 0;
  uint8_t hash[ScriptCachedMetadataHandlerWithHashing::kSha256Bytes] = {0};
};

// Describes a few interesting states of the ScriptCachedMetadataHandler when
// GetCachedMetadata() is called. These values are written to logs. New enum
// values can be added, but existing enums must never be renumbered or deleted
// and reused.
enum class StateOnGet : int {
  kPresent = 0,
  kDataTypeMismatch = 1,
  kWasNeverPresent = 2,
  kWasDiscarded = 3,

  // Must be equal to the greatest among enumeraiton values.
  kMaxValue = kWasDiscarded
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SCRIPT_CACHED_METADATA_HANDLER_H_
