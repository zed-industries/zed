// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_TRACKER_H_

#include <stdint.h>

#include <optional>

#include "base/feature_list.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/ad_script_identifier.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_initiator_info.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class Document;
class ExecutionContext;
class LocalFrame;
enum class ResourceType : uint8_t;

namespace probe {
class AsyncTaskContext;
class CallFunction;
class ExecuteScript;
}  // namespace probe

// Tracker for tagging resources as ads based on the call stack scripts.
// The tracker is maintained per local root.
class CORE_EXPORT AdTracker : public GarbageCollected<AdTracker> {
 public:
  enum class StackType { kBottomOnly, kBottomAndTop };

  // Finds an AdTracker for a given ExecutionContext.
  static AdTracker* FromExecutionContext(ExecutionContext*);

  static bool IsAdScriptExecutingInDocument(
      Document* document,
      StackType stack_type = StackType::kBottomAndTop);

  // Instrumenting methods.
  // Called when a script module or script gets executed from native code.
  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);

  // Called when a function gets called from native code.
  void Will(const probe::CallFunction&);
  void Did(const probe::CallFunction&);

  // Called when a subresource request is about to be sent or is redirected.
  // Returns true if any of the following are true:
  // - the resource is loaded in an ad iframe
  // - |known_ad| is true
  // - ad script is in the v8 stack and the resource was not requested by CSS.
  // Virtual for testing.
  virtual bool CalculateIfAdSubresource(
      ExecutionContext* execution_context,
      const KURL& request_url,
      ResourceType resource_type,
      const FetchInitiatorInfo& initiator_info,
      bool known_ad);

  // Called when an async task is created. Check at this point for ad script on
  // the stack and annotate the task if so.
  void DidCreateAsyncTask(probe::AsyncTaskContext* task_context);

  // Called when an async task is eventually run.
  void DidStartAsyncTask(probe::AsyncTaskContext* task_context);

  // Called when the task has finished running.
  void DidFinishAsyncTask(probe::AsyncTaskContext* task_context);

  // Returns true if any script in the pseudo call stack has previously been
  // identified as an ad resource, if the current ExecutionContext is a known ad
  // execution context, or if the script at the top of isolate's
  // stack is ad script. Whether to look at just the bottom of the
  // stack or the top and bottom is indicated by `stack_type`. kBottomAndTop is
  // generally best as it catches more ads, but if you're calling very
  // frequently then consider just the bottom of the stack for performance sake.
  //
  // Output Parameters:
  // - `out_ad_script_ancestry`: if non-null and there is ad script in the
  //   stack, this vector will be populated with the identified ad script and
  //   its ancestor scripts, ordered from the most immediate caller to more
  //   distant ancestors. The ancestry will be traced up to the first script
  //   flagged by the subresource filter.
  virtual bool IsAdScriptInStack(
      StackType stack_type,
      Vector<AdScriptIdentifier>* out_ad_script_ancestry = nullptr);

  virtual void Trace(Visitor*) const;

  void Shutdown();
  explicit AdTracker(LocalFrame*);
  AdTracker(const AdTracker&) = delete;
  AdTracker& operator=(const AdTracker&) = delete;
  virtual ~AdTracker();

 protected:
  // Protected for testing.
  // Note that this outputs the `out_top_script` even when it's not an ad.
  virtual String ScriptAtTopOfStack(
      std::optional<AdScriptIdentifier>* out_top_script);
  virtual ExecutionContext* GetCurrentExecutionContext();

 private:
  friend class FrameFetchContextSubresourceFilterTest;
  friend class AdTrackerSimTest;
  friend class AdTrackerTest;

  // Similar to the public IsAdScriptInStack method but instead of returning an
  // ancestry chain, it returns only one script (the most immediate one).
  bool IsAdScriptInStackHelper(
      StackType stack_type,
      std::optional<AdScriptIdentifier>* out_ad_script);

  // |script_name| will be empty in the case of a dynamically added script with
  // no src attribute set. |script_id| won't be set for module scripts in an
  // errored state or for non-source text modules.
  void WillExecuteScript(ExecutionContext*,
                         const v8::Local<v8::Context>& v8_context,
                         const String& script_name,
                         int script_id);
  void DidExecuteScript();
  bool IsKnownAdScript(ExecutionContext*, const String& url);
  bool IsKnownAdScriptForCheckedContext(
      ExecutionContext&,
      const String& url,
      std::optional<AdScriptIdentifier>* out_ad_script);

  // Adds the given `url` to the set of known ad scripts associated with the
  // provided `execution_context`.
  //
  // If `ancestor_ad_script` is specified, it indicates that `url` was
  // identified as an ad script indirectly, due to its association with another
  // ad script (`ancestor_ad_script`) in the call stack. In such cases, a link
  // is then established between `url` and `ancestor_ad_script`.
  void AppendToKnownAdScripts(
      ExecutionContext& execution_context,
      const String& url,
      const std::optional<AdScriptIdentifier>& ancestor_ad_script);

  // If a known ancestor of `script_name` exists in `execution_context`, creates
  // and links a new AdScriptIdentifier (with `script_id` and
  // `v8_context`) to the ancestor. The link is kept in `ancestor_ad_scripts_`.
  //
  // Prerequisites: `script_name` is a known ad script in `execution_context`.
  void MaybeLinkKnownAdScriptToAncestor(
      ExecutionContext* execution_context,
      const v8::Local<v8::Context>& v8_context,
      const String& script_name,
      int script_id);

  // Retrieves the ancestry chain of a given ad script (inclusive), ordered from
  // the script itself to the root script. The root script is guaranteed to be
  // subresource-filter-flagged, while all preceding scripts in the chain are
  // non-subresource-filter-flagged.
  Vector<AdScriptIdentifier> GetAncestryChain(
      const AdScriptIdentifier& ad_script);

  Member<LocalFrame> local_root_;

  // Each time v8 is started to run a script or function, this records if it was
  // an ad script. Each time the script or function finishes, it pops the stack.
  Vector<bool> stack_frame_is_ad_;

  int num_ads_in_stack_ = 0;

  // Indicates the bottom-most ad script on the stack or `std::nullopt` if
  // there isn't one. A non-null value implies `num_ads_in_stack > 0`.
  std::optional<AdScriptIdentifier> bottom_most_ad_script_;

  // Indicates the bottom-most ad script on the async stack or `std::nullopt`
  // if there isn't one.
  std::optional<AdScriptIdentifier> bottom_most_async_ad_script_;

  // Maps the URL of a detected ad script to an optional `AdScriptIdentifier`
  // representing its ancestor ad script in the creation stack. This ancestor is
  // only recorded if the detected ad script is *not* flagged by the subresource
  // filter.
  //
  // Script Identification:
  // - Scripts with a resource URL are identified by that URL.
  // - Inline scripts (without a URL) are assigned a unique synthetic URL
  //   generated by `GenerateFakeUrlFromScriptId()`.
  using KnownAdScriptsAndAncestor =
      HashMap<String, std::optional<AdScriptIdentifier>>;

  // Tracks ad scripts detected outside of ad-frame contexts.
  HeapHashMap<WeakMember<ExecutionContext>, KnownAdScriptsAndAncestor>
      context_known_ad_scripts_;

  // Maps non-subresource-filter-flagged ad scripts to their ancestor ad script
  // in the creation stack. Following the chain iteratively through this map
  // will eventually lead to a root script that *is* subresource-filter-flagged.
  // This allows derived scripts to be traced back to their flagged origin.
  HashMap<AdScriptIdentifier, AdScriptIdentifier> ancestor_ad_scripts_;

  // The number of ad-related async tasks currently running in the stack.
  int running_ad_async_tasks_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_TRACKER_H_
