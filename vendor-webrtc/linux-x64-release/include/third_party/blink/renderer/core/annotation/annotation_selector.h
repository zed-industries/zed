// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_SELECTOR_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;
class RangeInFlatTree;

// An AnnotationSelector is an abstract interface that's used by an annotation
// to specify what part of a Document it should be attached to. For example, a
// text-based selector may be configured to attach an annotation to the first
// instance of the text string "the quick brown dog" in the page. A CSS-based
// selector may be configured to attach an annotation to the third child of the
// <p> element with id 'foo'.
//
// Annotation-related data will typically be stored by the client so selectors
// must be able to serialize and deserialize themselves. To instantiate a
// selector from serialized form, use the static Deserialize factory method.
//
// Selectors are scoped to a single Document, meaning that they will not search
// the content of any subframes in the document. However, the search is
// performed on a "flat" DOM tree, meaning it will descend through shadow tree
// boundaries so content inside ShadowDOM is searchable.
class CORE_EXPORT AnnotationSelector
    : public GarbageCollected<AnnotationSelector> {
 public:
  enum SearchType {
    // Will synchronously search the document, the callback is invoked before
    // FindRange returns.
    kSynchronous,

    // Asynchronously searches the document, FindRange may return before the
    // callback is invoked.
    kAsynchronous
  };

  // Tests can inject a function that will be called in place of Deserialize so
  // that they can generate a selector implementation of their choosing.
  using GeneratorFunc =
      base::RepeatingCallback<AnnotationSelector*(const String&)>;
  static void SetGeneratorForTesting(GeneratorFunc generator);
  static void UnsetGeneratorForTesting();

  // Factory method used to instantiate a selector of the correct type from
  // serialized form.
  static AnnotationSelector* Deserialize(const String& serialized);

  virtual ~AnnotationSelector() = default;

  virtual void Trace(Visitor* visitor) const {}

  // Serializes the selector to a form that can be stored and deserialized
  // using the Deserialize factory method.
  virtual String Serialize() const = 0;

  // Applies the selector to find a Range in the given `document`. The found
  // range is returned by invoking `finished_cb`, if no content matching the
  // selector was found in the document, the `finished_cb` is invoked with
  // nullptr.
  //
  // The search can be performed either synchronously or asynchronously. If
  // synchronously, the `finished_cb` is guaranteed to be invoked before
  // FindRange returns.
  using FinishedCallback = base::OnceCallback<void(const RangeInFlatTree*)>;
  virtual void FindRange(Range& search_range,
                         SearchType type,
                         FinishedCallback finished_cb) = 0;

  virtual bool IsTextSelector() const { return false; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_SELECTOR_H_
