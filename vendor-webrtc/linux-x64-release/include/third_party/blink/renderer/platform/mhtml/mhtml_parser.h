/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_MHTML_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_MHTML_PARSER_H_

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mhtml/shared_buffer_chunk_reader.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace WTF {
class String;
}

namespace blink {

class ArchiveResource;
class MIMEHeader;

class PLATFORM_EXPORT MHTMLParser final {
  STACK_ALLOCATED();

 public:
  explicit MHTMLParser(scoped_refptr<const SharedBuffer>);

  HeapVector<Member<ArchiveResource>> ParseArchive();
  base::Time CreationDate() const;

  // Translates |contentIDFromMimeHeader| (of the form "<foo@bar.com>")
  // into a cid-scheme URI (of the form "cid:foo@bar.com").
  //
  // Returns KURL() - an invalid URL - if contentID is invalid.
  //
  // See rfc2557 - section 8.3 - "Use of the Content-ID header and CID URLs".
  static KURL ConvertContentIDToURI(const String& content_id);

 private:
  bool ParseArchiveWithHeader(MIMEHeader*,
                              HeapVector<Member<ArchiveResource>>&);
  ArchiveResource* ParseNextPart(const MIMEHeader&,
                                 const String& end_of_part_boundary,
                                 const String& end_of_document_boundary,
                                 bool& end_of_archive_reached);

  base::Time creation_date_;
  SharedBufferChunkReader line_reader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_MHTML_PARSER_H_
