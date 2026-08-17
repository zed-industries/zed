// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_METADATA_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class Blob;
class ScriptState;

struct FontEnumerationEntry {
  String postscript_name;
  String full_name;
  String family;
  String style;
};

class BLINK_EXPORT FontMetadata final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit FontMetadata(const FontEnumerationEntry& entry);

  static FontMetadata* Create(const FontEnumerationEntry& entry);

  // The tables below represent the properties made available via the API.
  //
  // Names:
  //
  // This table shows the properties made available via the API, the name table
  // entries those properties map to, and their localization status as returned
  // by the API. The localized properties are in the system's configured locale.
  //
  // For more about name table entries, go to:
  // https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids
  //
  //  +----------------+---------+-----------+
  //  |    Property    | name ID | Localized |
  //  +----------------+---------+-----------+
  //  | postscriptName |       6 | No        |
  //  | family         |       1 | No        |
  //  | style          |       2 | No        |
  //  | fullName       |       4 | Yes       |
  //  +----------------+---------+-----------+

  String postscriptName() const { return postscriptName_; }
  String fullName() const { return fullName_; }
  String family() const { return family_; }
  String style() const { return style_; }

  ScriptPromise<Blob> blob(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  static void BlobImpl(ScriptPromiseResolver<Blob>* resolver,
                       const String& postscriptName);
  String postscriptName_;
  String fullName_;
  String family_;
  String style_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_METADATA_H_
