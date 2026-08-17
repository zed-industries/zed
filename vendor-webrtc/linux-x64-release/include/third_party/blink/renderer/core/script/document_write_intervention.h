// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_WRITE_INTERVENTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_WRITE_INTERVENTION_H_

#include "third_party/blink/renderer/platform/loader/fetch/cross_origin_attribute_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"

// document.write() intervention may
// - block network loading of a script inserted by document.write() and
// - send an asynchronous GET request to the blocked URL (with an
//   intervention header) that doesn't cause script execution, in order to:
//   - Notify the page authors, and
//   - Fill in the disk cache for a future use.
//   This also fills in the MemoryCache, but the ScriptResource will be GCed
//   and removed from the MemoryCache very soon (it's OK to reuse the
//   ScriptResource, but we don't have to keep it on MemoryCache).
// https://developers.google.com/web/updates/2016/08/removing-document-write

namespace blink {

class Document;
class Resource;
class ScriptFetchOptions;

// Returns true if the fetch should be blocked due to the document.write
// intervention. In that case, the request's cache policy is set to
// kReturnCacheDataDontLoad to ensure a network request is not generated. This
// function may also set an Intervention header, log the intervention in the
// console, etc.
//
// The caller should call SetResource() for the returned client.
bool MaybeDisallowFetchForDocWrittenScript(FetchParameters&, Document&);

// Outputs console errors/warnings depending on whether the script is actually
// blocked or not, and sends an asynchronous GET request with an interventions
// header if blocked. Should be called when NotifyFinished() if
// MaybeDisallowFetchForDocWrittenScript() returns true.
void PossiblyFetchBlockedDocWriteScript(const Resource*,
                                        Document&,
                                        const ScriptFetchOptions&,
                                        CrossOriginAttributeValue);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_WRITE_INTERVENTION_H_
