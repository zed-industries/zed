// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_HEADERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_HEADERS_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_headers.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fetch/fetch_header_list.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExceptionState;
class ScriptState;

// http://fetch.spec.whatwg.org/#headers-class
class CORE_EXPORT Headers final : public ScriptWrappable,
                                  public PairSyncIterable<Headers> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum Guard {
    kImmutableGuard,
    kRequestGuard,
    kRequestNoCorsGuard,
    kResponseGuard,
    kNoneGuard
  };

  static Headers* Create(ScriptState* script_state,
                         ExceptionState& exception_state);
  static Headers* Create(ScriptState* script_state,
                         const V8HeadersInit* init,
                         ExceptionState& exception_state);

  // Shares the FetchHeaderList. Called when creating a Request or Response.
  static Headers* Create(FetchHeaderList*);

  Headers();
  // Shares the FetchHeaderList. Called when creating a Request or Response.
  explicit Headers(FetchHeaderList*);

  Headers* Clone() const;

  // Headers.idl implementation.
  void append(ScriptState* script_state,
              const String& name,
              const String& value,
              ExceptionState&);
  void remove(ScriptState* script_state, const String& key, ExceptionState&);
  String get(const String& key, ExceptionState&);
  Vector<String> getSetCookie();
  bool has(const String& key, ExceptionState&);
  void set(ScriptState* script_state,
           const String& key,
           const String& value,
           ExceptionState&);

  void SetGuard(Guard guard) { guard_ = guard; }
  Guard GetGuard() const { return guard_; }

  // These methods should only be called when size() would return 0.
  void FillWith(ScriptState* script_state, const Headers*, ExceptionState&);
  void FillWith(ScriptState* script_state,
                const V8HeadersInit* init,
                ExceptionState& exception_state);

  // https://fetch.spec.whatwg.org/#concept-headers-remove-privileged-no-cors-request-headers
  void RemovePrivilegedNoCorsRequestHeaders();

  FetchHeaderList* HeaderList() const { return header_list_.Get(); }
  void Trace(Visitor*) const override;

 private:
  class HeadersIterationSource final
      : public PairSyncIterable<Headers>::IterationSource {
   public:
    explicit HeadersIterationSource(Headers* headers);
    ~HeadersIterationSource() override;

    bool FetchNextItem(ScriptState* script_state,
                       String& key,
                       String& value,
                       ExceptionState& exception) override;

    void Trace(Visitor*) const override;

    void ResetHeaderList();

   private:
    // https://webidl.spec.whatwg.org/#dfn-value-pairs-to-iterate-over
    Vector<std::pair<String, String>> headers_list_;
    // https://webidl.spec.whatwg.org/#default-iterator-object-index
    wtf_size_t current_ = 0;
    Member<Headers> headers_;
  };

  // These methods should only be called when size() would return 0.
  void FillWith(ScriptState* script_state,
                const Vector<Vector<String>>&,
                ExceptionState&);
  void FillWith(ScriptState* script_state,
                const Vector<std::pair<String, String>>&,
                ExceptionState&);

  Member<FetchHeaderList> header_list_;
  Guard guard_;

  IterationSource* CreateIterationSource(ScriptState*,
                                         ExceptionState&) override;

  HeapHashSet<WeakMember<HeadersIterationSource>> iterators_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_HEADERS_H_
