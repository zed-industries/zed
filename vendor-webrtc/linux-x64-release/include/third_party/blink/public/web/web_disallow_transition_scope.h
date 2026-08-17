// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_DISALLOW_TRANSITION_SCOPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_DISALLOW_TRANSITION_SCOPE_H_

#include "base/dcheck_is_on.h"
#include "base/memory/raw_ref.h"

#if DCHECK_IS_ON()

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_document.h"

namespace blink {

class DocumentLifecycle;

class BLINK_EXPORT WebDisallowTransitionScope {
  // Causes DCHECKs only, does not actually prevent lifecycle changes.
  // This is useful to prevent certain types of crashes that occur, for example,
  // when updating properties in the accessible object hierarchy.
 public:
  explicit WebDisallowTransitionScope(WebDocument* web_document);
  ~WebDisallowTransitionScope();

 private:
  DocumentLifecycle& Lifecycle(WebDocument*) const;

  const raw_ref<DocumentLifecycle> document_lifecycle_;
};

}  // namespace blink

#endif  // DCHECK_IS_ON()

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_DISALLOW_TRANSITION_SCOPE_H_
