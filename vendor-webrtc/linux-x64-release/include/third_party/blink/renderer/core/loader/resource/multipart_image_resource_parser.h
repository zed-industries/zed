// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/* ***** BEGIN LICENSE BLOCK *****
 * Version: MPL 1.1/GPL 2.0/LGPL 2.1
 *
 * The contents of this file are subject to the Mozilla Public License Version
 * 1.1 (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * Software distributed under the License is distributed on an "AS IS" basis,
 * WITHOUT WARRANTY OF ANY KIND, either express or implied. See the License
 * for the specific language governing rights and limitations under the
 * License.
 *
 * The Original Code is mozilla.org code.
 *
 * The Initial Developer of the Original Code is
 * Netscape Communications Corporation.
 * Portions created by the Initial Developer are Copyright (C) 1998
 * the Initial Developer. All Rights Reserved.
 *
 * Contributor(s):
 *
 * Alternatively, the contents of this file may be used under the terms of
 * either the GNU General Public License Version 2 or later (the "GPL"), or
 * the GNU Lesser General Public License Version 2.1 or later (the "LGPL"),
 * in which case the provisions of the GPL or the LGPL are applicable instead
 * of those above. If you wish to allow use of your version of this file only
 * under the terms of either the GPL or the LGPL, and not to allow others to
 * use your version of this file under the terms of the MPL, indicate your
 * decision by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL or the LGPL. If you do not delete
 * the provisions above, a recipient may use your version of this file under
 * the terms of any one of the MPL, the GPL or the LGPL.
 *
 * ***** END LICENSE BLOCK ***** */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MULTIPART_IMAGE_RESOURCE_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MULTIPART_IMAGE_RESOURCE_PARSER_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// A parser parsing mlutipart/x-mixed-replace resource.
class CORE_EXPORT MultipartImageResourceParser final
    : public GarbageCollected<MultipartImageResourceParser> {
 public:
  class CORE_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    virtual void OnePartInMultipartReceived(const ResourceResponse&) = 0;
    virtual void MultipartDataReceived(base::span<const uint8_t> bytes) = 0;
    void Trace(Visitor* visitor) const override {}
  };

  MultipartImageResourceParser(const ResourceResponse&,
                               const Vector<char>& boundary,
                               Client*);
  MultipartImageResourceParser(const MultipartImageResourceParser&) = delete;
  MultipartImageResourceParser& operator=(const MultipartImageResourceParser&) =
      delete;
  void AppendData(base::span<const char> bytes);
  void Finish();
  void Cancel() { is_cancelled_ = true; }

  void Trace(Visitor*) const;

  static wtf_size_t SkippableLengthForTest(const Vector<char>& data,
                                           wtf_size_t size) {
    return SkippableLength(data, size);
  }
  static wtf_size_t FindBoundaryForTest(const Vector<char>& data,
                                        Vector<char>* boundary) {
    return FindBoundary(data, boundary);
  }

 private:
  bool ParseHeaders();
  bool IsCancelled() const { return is_cancelled_; }
  static wtf_size_t SkippableLength(const Vector<char>&, wtf_size_t);
  // This function updates |*boundary|.
  static wtf_size_t FindBoundary(const Vector<char>& data,
                                 Vector<char>* boundary);

  const ResourceResponse original_response_;
  Vector<char> boundary_;
  Member<Client> client_;

  Vector<char> data_;
  bool is_parsing_top_ = true;
  bool is_parsing_headers_ = false;
  bool saw_last_boundary_ = false;
  bool is_cancelled_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MULTIPART_IMAGE_RESOURCE_PARSER_H_
