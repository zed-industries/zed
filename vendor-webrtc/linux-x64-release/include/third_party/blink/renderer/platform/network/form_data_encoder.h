/*
 * Copyright (C) 2008 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_FORM_DATA_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_FORM_DATA_ENCODER_H_

#include "third_party/blink/renderer/platform/network/encoded_form_data.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace WTF {
class TextEncoding;
}

namespace blink {

class PLATFORM_EXPORT FormDataEncoder {
  STATIC_ONLY(FormDataEncoder);

 public:
  // Specifies how to handle CRs and LFs. When NormalizeCRLF is passed, the
  // method replaces the following characters with a CRLF pair:
  // - a CR not followed by an LF
  // - an LF not preceded by a CR
  enum Mode { kNormalizeCRLF, kDoNotNormalizeCRLF };

  static WTF::TextEncoding EncodingFromAcceptCharset(
      const String& accept_charset,
      const WTF::TextEncoding& fallback_encoding);

  // Helper functions used by HTMLFormElement for multi-part form data
  static Vector<char> GenerateUniqueBoundaryString();
  static void BeginMultiPartHeader(Vector<char>&,
                                   const std::string& boundary,
                                   const std::string& name);
  static void AddBoundaryToMultiPartHeader(Vector<char>&,
                                           const std::string& boundary,
                                           bool is_last_boundary = false);
  static void AddFilenameToMultiPartHeader(Vector<char>&,
                                           const WTF::TextEncoding&,
                                           const String& filename);
  static void AddContentTypeToMultiPartHeader(Vector<char>&,
                                              const String& mime_type);
  static void FinishMultiPartHeader(Vector<char>&);

  // Helper functions used by HTMLFormElement for non multi-part form data. Mode
  // argument is not used for TextPlain type.
  static void AddKeyValuePairAsFormData(
      Vector<char>&,
      const std::string& key,
      const std::string& value,
      EncodedFormData::EncodingType = EncodedFormData::kFormURLEncoded,
      Mode = kNormalizeCRLF);
  static void EncodeStringAsFormData(Vector<char>&, const std::string&, Mode);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_FORM_DATA_ENCODER_H_
