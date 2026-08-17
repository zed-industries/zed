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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_WRAPPER_WORLD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_WRAPPER_WORLD_H_

#include "base/memory/ptr_util.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "v8/include/v8.h"

namespace base {
class UnguessableToken;
}  // namespace base

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class DOMDataStore;
class ScriptWrappable;
class SecurityOrigin;

enum IsolatedWorldId {
  // Embedder isolated worlds can use IDs in [1, 1<<29).
  kEmbedderWorldIdLimit = (1 << 29),
  kDocumentXMLTreeViewerWorldId,
  kDevToolsFirstIsolatedWorldId,
  kDevToolsLastIsolatedWorldId = kDevToolsFirstIsolatedWorldId + (1 << 29),
  kIsolatedWorldIdLimit,
};

// This class represent a collection of DOM wrappers for a specific world. This
// is identified by a world id that is a per-thread global identifier (see
// WorldId enum).
class PLATFORM_EXPORT DOMWrapperWorld final
    : public GarbageCollected<DOMWrapperWorld> {
 public:
  using PassKey = base::PassKey<DOMWrapperWorld>;

  // Per-thread global identifiers for DOMWrapperWorld.
  enum WorldId : int32_t {
    kInvalidWorldId = -1,
    kMainWorldId = 0,

    kDOMWrapperWorldEmbedderWorldIdLimit =
        IsolatedWorldId::kEmbedderWorldIdLimit,
    kDOMWrapperWorldIsolatedWorldIdLimit =
        IsolatedWorldId::kIsolatedWorldIdLimit,

    // Other worlds can use IDs after this. Don't manually pick up an ID from
    // this range. generateWorldIdForType() picks it up on behalf of you.
    kUnspecifiedWorldIdStart,
  };

  // Various world types. Isolated worlds get their own security policy.
  enum class WorldType {
    // The main world for the main rendering thread. World id is 0. Considered
    // an isolated world.
    kMain,
    // An isolated world created via `EnsureIsolatedWorld()`. The caller passes
    // in a world id that should be used. The embedder is supposed to respect
    // the `kEmbedderWorldIdLimit` for creating the isolated world.
    kIsolated,
    // An isolated world for the inspector. The world id is generated
    // internally.
    kInspectorIsolated,
    // A utility world that is not considered an isolated world.
    kRegExp,
    // A utility world for context snapshotting that is not considered an
    // isolated world.
    kForV8ContextSnapshotNonMain,
    // A world for each worker/worklet. Not considered an isolated world.
    kWorkerOrWorklet,
    // Shadow realms do not have a corresponding Frame nor DOMWindow so they're
    // very different from the main world. Shadow realms are not workers nor
    // worklets obviously, nor Chrome extensions' content scripts. So, we use
    // a distinguishable world type. Shadow realms can be created not only in
    // the main isolate but also in worker isolates and other isolates.
    kShadowRealm,
  };

  static constexpr bool IsIsolatedWorldId(int32_t world_id) {
    return DOMWrapperWorld::kMainWorldId < world_id &&
           world_id < DOMWrapperWorld::kDOMWrapperWorldIsolatedWorldIdLimit;
  }

  // Creates a world other than IsolatedWorld. Note this can return nullptr if
  // `GenerateWorldIdForType()` internally fails to allocate a valid id.
  static DOMWrapperWorld* Create(v8::Isolate*,
                                 WorldType,
                                 bool is_default_world_of_isolate = false);

  // Ensures an IsolatedWorld for |worldId|.
  static DOMWrapperWorld* EnsureIsolatedWorld(v8::Isolate*, int32_t world_id);

  DOMWrapperWorld(PassKey,
                  v8::Isolate*,
                  WorldType,
                  int32_t world_id,
                  bool is_default_world_of_isolate);
  ~DOMWrapperWorld();

  // Explicitly dispose internal data of the world. The world itself will only
  // be reclaimed by GC. Note that calling `EnsureIsolatedWorld()` with the same
  // id will yield in the same world object.
  void Dispose();

  // Called from performance-sensitive functions, so we should keep this simple
  // and fast as much as possible.
  static bool NonMainWorldsExistInMainThread() {
    return number_of_non_main_worlds_in_main_thread_;
  }

  static void AllWorldsInIsolate(v8::Isolate* isolate,
                                 HeapVector<Member<DOMWrapperWorld>>& worlds);

  static DOMWrapperWorld& World(v8::Isolate* isolate,
                                v8::Local<v8::Context> context) {
    return ScriptState::From(isolate, context)->World();
  }

  static DOMWrapperWorld& Current(v8::Isolate* isolate) {
    return World(isolate, isolate->GetCurrentContext());
  }

  static DOMWrapperWorld& MainWorld(v8::Isolate* isolate);

  static void SetNonMainWorldStableId(int32_t world_id, const WTF::String&);
  WTF::String NonMainWorldStableId() const;

  static void SetNonMainWorldHumanReadableName(int32_t world_id,
                                               const WTF::String&);
  WTF::String NonMainWorldHumanReadableName() const;

  // Associates an isolated world (see above for description) with a security
  // origin. XMLHttpRequest instances used in that world will be considered
  // to come from that origin, not the frame's.
  // Note: if |security_origin| is null, the security origin stored for the
  // isolated world is cleared.
  static void SetIsolatedWorldSecurityOrigin(
      int32_t world_id,
      scoped_refptr<SecurityOrigin> security_origin);

  // Returns the security origin for the given world with the given
  // |cluster_id|.
  scoped_refptr<SecurityOrigin> IsolatedWorldSecurityOrigin(
      const base::UnguessableToken& cluster_id);
  scoped_refptr<const SecurityOrigin> IsolatedWorldSecurityOrigin(
      const base::UnguessableToken& cluster_id) const;

  bool IsMainWorld() const { return world_type_ == WorldType::kMain; }
  bool IsWorkerOrWorkletWorld() const {
    return world_type_ == WorldType::kWorkerOrWorklet;
  }
  bool IsShadowRealmWorld() const {
    return world_type_ == WorldType::kShadowRealm;
  }
  bool IsIsolatedWorld() const {
    return world_type_ == WorldType::kIsolated ||
           world_type_ == WorldType::kInspectorIsolated;
  }

  WorldType GetWorldType() const { return world_type_; }
  int GetWorldId() const { return world_id_; }
  DOMDataStore& DomDataStore() const { return *dom_data_store_; }

  v8::Isolate* GetIsolate() const { return isolate_; }

  void Trace(Visitor*) const;

  // Methods iterate all worlds and invokes the clearing methods on
  // DOMDataStore. The WorldMap is only known to the DOMWrapperWorld and as such
  // the iteration cannot be folded into DOMDataStore.
  static bool ClearWrapperInAnyNonInlineStorageWorldIfEqualTo(
      ScriptWrappable* object,
      const v8::Local<v8::Object>& handle);
  static bool ClearWrapperInAnyNonInlineStorageWorldIfEqualTo(
      ScriptWrappable* object,
      const v8::TracedReference<v8::Object>& handle);

 private:
  // Returns an identifier for a given world type. This must not be called for
  // WorldType::IsolatedWorld because an identifier for the world is given from
  // out of DOMWrapperWorld.
  static std::optional<int> GenerateWorldIdForType(WorldType);

  static unsigned number_of_non_main_worlds_in_main_thread_;

  const WorldType world_type_;
  const int32_t world_id_;
  const Member<DOMDataStore> dom_data_store_;
  // The pointer does not dangle in production configurations but only in unit
  // tests. Specifically, for test that cycle through V8 isolates and reuse the
  // same CppHeap, Oilpan objects are destroyed after isolate teardown which
  // means that pointers dangle. The objects should be unreachable in the tests
  // though.
  const raw_ptr<v8::Isolate, base::RawPtrTraits::kMayDangle> isolate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_WRAPPER_WORLD_H_
