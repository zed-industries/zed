/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRERENDER_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRERENDER_HANDLE_H_

#include "base/memory/scoped_refptr.h"
#include "base/types/pass_key.h"
#include "third_party/blink/public/mojom/prerender/prerender.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class ExecutionContext;
class Document;

// This is the Blink-side liaison of mojom::PrerenderProcessor to request the
// browser process to start prerendering. This is instantiated per prerender
// request, for example, when a new <link rel=prerender> element is added, when
// the element's href is changed etc.
//
// When you no longer need the prerendering page (e.g., when the
// <link rel=prerender> element is removed), you can ask the browser process to
// cancel the running prerender by Cancel(). If mojo connections are reset
// without Cancel() call, the browser process considers this prerendering
// request to be abandoned and may still use the prerendered page if a
// navigation occurs to that URL shortly after.
//
// TODO(https://crbug.com/1126305): Rename this to PrerenderProcessorClient.
class PrerenderHandle final : public GarbageCollected<PrerenderHandle> {
 public:
  static PrerenderHandle* Create(
      Document&,
      const KURL&,
      mojom::blink::PrerenderTriggerType prerender_trigger_type);

  using PassKey = base::PassKey<PrerenderHandle>;
  PrerenderHandle(PassKey,
                  ExecutionContext*,
                  const KURL&,
                  HeapMojoRemote<mojom::blink::NoStatePrefetchProcessor>);
  PrerenderHandle(const PrerenderHandle&) = delete;
  PrerenderHandle& operator=(const PrerenderHandle&) = delete;
  ~PrerenderHandle();

  // Asks the browser process to cancel the running prerender.
  void Cancel();

  const KURL& Url() const;

  void Trace(Visitor*) const;

 private:
  const KURL url_;
  HeapMojoRemote<mojom::blink::NoStatePrefetchProcessor>
      remote_prefetch_processor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRERENDER_HANDLE_H_
