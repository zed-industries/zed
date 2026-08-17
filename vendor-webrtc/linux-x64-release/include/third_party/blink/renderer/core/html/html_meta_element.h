/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2010 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_META_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_META_ELEMENT_H_

#include <optional>

#include "services/network/public/cpp/client_hints.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "ui/base/ime/mojom/virtual_keyboard_types.mojom-blink.h"

namespace blink {

enum ViewportErrorCode {
  kUnrecognizedViewportArgumentKeyError,
  kUnrecognizedViewportArgumentValueError,
  kTruncatedViewportArgumentValueError,
  kMaximumScaleTooLargeError,
  kTargetDensityDpiUnsupported,
  kViewportFitUnsupported
};

class CORE_EXPORT HTMLMetaElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static void GetViewportDescriptionFromContentAttribute(
      const String& content,
      ViewportDescription&,
      Document*,
      bool viewport_meta_zero_values_quirk);

  static void ProcessMetaCH(Document&,
                            const AtomicString& content,
                            network::MetaCHType type,
                            bool is_doc_preloader,
                            bool is_sync_parser);

  explicit HTMLMetaElement(Document&, const CreateElementFlags);

  // Encoding computed from processing the http-equiv, charset and content
  // attributes.
  WTF::TextEncoding ComputeEncoding() const;

  const AtomicString& Content() const;
  const AtomicString& HttpEquiv() const;
  const AtomicString& Media() const;
  const AtomicString& GetName() const;
  const AtomicString& Property() const;
  const AtomicString& Itemprop() const;

 private:
  static void ProcessViewportKeyValuePair(Document*,
                                          bool report_warnings,
                                          const String& key,
                                          const String& value,
                                          bool viewport_meta_zero_values_quirk,
                                          ViewportDescription&);
  static void ParseViewportContentAttribute(
      const String& content,
      ViewportDescription&,
      Document*,
      bool viewport_meta_zero_values_quirk);

  void ParseAttribute(const AttributeModificationParams&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void DidNotifySubtreeInsertionsToDocument() override;

  static float ParsePositiveNumber(Document*,
                                   bool report_warnings,
                                   const String& key,
                                   const String& value,
                                   bool* ok = nullptr);

  static Length ParseViewportValueAsLength(Document*,
                                           bool report_warnings,
                                           const String& key,
                                           const String& value);
  static float ParseViewportValueAsZoom(
      Document*,
      bool report_warnings,
      const String& key,
      const String& value,
      bool& computed_value_matches_parsed_value,
      bool viewport_meta_zero_values_quirk);
  static bool ParseViewportValueAsUserZoom(
      Document*,
      bool report_warnings,
      const String& key,
      const String& value,
      bool& computed_value_matches_parsed_value);
  static float ParseViewportValueAsDPI(Document*,
                                       bool report_warnings,
                                       const String& key,
                                       const String& value);

  static mojom::ViewportFit ParseViewportFitValueAsEnum(bool& unknown_value,
                                                        const String& value);

  static std::optional<ui::mojom::blink::VirtualKeyboardMode>
  ParseVirtualKeyboardValueAsEnum(const String& value);

  static void ReportViewportWarning(Document*,
                                    ViewportErrorCode,
                                    const String& replacement1,
                                    const String& replacement2);

  void ProcessContent();
  void ProcessHttpEquiv();
  void NameRemoved(const AtomicString& name_value);
  void ProcessViewportContentAttribute(const String& content,
                                       ViewportDescription::Type origin);
  void ProcessColorScheme(const AtomicString& content);
  void FinishParsingChildren() final;

  // ClientHintsPreferences::UpdateFromMetaCH needs to know if the synchronous
  // parser was used as otherwise the value may be discarded.
  bool is_sync_parser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_META_ELEMENT_H_
