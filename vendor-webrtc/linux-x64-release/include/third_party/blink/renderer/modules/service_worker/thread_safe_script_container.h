// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_THREAD_SAFE_SCRIPT_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_THREAD_SAFE_SCRIPT_CONTAINER_H_

#include <memory>
#include <utility>

#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// ThreadSafeScriptContainer stores the scripts of a service worker for
// startup. This container is created for each service worker. The IO thread
// adds scripts to the container, and the worker thread takes the scripts.
// Note: Due to the above explanations about multi-threads access on this
// container, all the non-thread-safe members like KURL and String need to be
// deep-copied.
//
// This class uses explicit synchronization because it needs to support
// synchronous importScripts() from the worker thread.
//
// This class is RefCounted because there is no ordering guarantee of lifetime
// of its owners, i.e. ServiceWorkerInstalledScriptsManager and its
// Internal class. ServiceWorkerInstalledScriptsManager is destroyed earlier
// than Internal if the worker is terminated before all scripts are streamed,
// and Internal is destroyed earlier if all of scripts are received before
// finishing script evaluation.
class MODULES_EXPORT ThreadSafeScriptContainer
    : public WTF::ThreadSafeRefCounted<ThreadSafeScriptContainer> {
 public:
  using BytesChunk = Vector<char>;

  // Container of a script. All the fields of this class need to be
  // cross-thread-transfer-safe.
  class MODULES_EXPORT RawScriptData {
    USING_FAST_MALLOC(RawScriptData);

   public:
    RawScriptData(const String& encoding,
                  Vector<uint8_t> script_text,
                  Vector<uint8_t> meta_data);
    ~RawScriptData();

    void AddHeader(const String& key, const String& value);

    // The encoding of the script text.
    const String& Encoding() const { return encoding_; }

    // An array of raw bytes representing the script text.
    Vector<uint8_t> TakeScriptText() { return std::move(script_text_); }

    // An array of raw bytes representing the cached metadata.
    Vector<uint8_t> TakeMetaData() { return std::move(meta_data_); }

    // The HTTP headers of the script.
    std::unique_ptr<CrossThreadHTTPHeaderMapData> TakeHeaders() {
      return std::move(headers_);
    }

   private:
    String encoding_;
    Vector<uint8_t> script_text_;
    Vector<uint8_t> meta_data_;
    std::unique_ptr<CrossThreadHTTPHeaderMapData> headers_;
  };

  ThreadSafeScriptContainer();

  enum class ScriptStatus {
    // The script data has been received.
    kReceived,
    // The script data has been received but it has already been taken.
    kTaken,
    // Receiving the script has failed.
    kFailed,
    // The script data has not been received yet.
    kPending
  };

  // Adds a script for the |url| to the container, and records status of the
  // script as kReceived when |data| is non-null, or kFailed when |data| is
  // nullptr. Called on the IO thread.
  void AddOnIOThread(const KURL& url, std::unique_ptr<RawScriptData> data);

  // Called on the worker thread.
  ScriptStatus GetStatusOnWorkerThread(const KURL& url);

  // Removes the script. After calling this, ScriptStatus for the
  // script will be kPending.
  // Called on the worker thread.
  void ResetOnWorkerThread(const KURL& url);

  // Waits until the script is added. The thread is blocked until the script is
  // available or receiving the script fails. Returns false if an error happens
  // and the waiting script won't be available forever.
  // Called on the worker thread.
  bool WaitOnWorkerThread(const KURL& url);

  // Called on the worker thread.
  std::unique_ptr<RawScriptData> TakeOnWorkerThread(const KURL& url);

  // Called if no more data will be added.
  // Called on the IO thread.
  void OnAllDataAddedOnIOThread();

 private:
  friend class WTF::ThreadSafeRefCounted<ThreadSafeScriptContainer>;
  ~ThreadSafeScriptContainer();

  base::Lock lock_;
  // |waiting_cv_| is signaled when a script whose url matches to |waiting_url|
  // is added, or OnAllDataAdded is called. The worker thread waits on this, and
  // the IO thread signals it.
  base::ConditionVariable waiting_cv_ GUARDED_BY(lock_);
  HashMap<KURL, std::pair<ScriptStatus, std::unique_ptr<RawScriptData>>>
      script_data_ GUARDED_BY(lock_);
  KURL waiting_url_ GUARDED_BY(lock_);
  bool are_all_data_added_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_THREAD_SAFE_SCRIPT_CONTAINER_H_
