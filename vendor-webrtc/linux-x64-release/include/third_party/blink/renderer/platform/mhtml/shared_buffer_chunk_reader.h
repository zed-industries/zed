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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_SHARED_BUFFER_CHUNK_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_SHARED_BUFFER_CHUNK_READER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SharedBufferChunkReader final {
  DISALLOW_NEW();

 public:
  SharedBufferChunkReader(scoped_refptr<const SharedBuffer>,
                          std::string_view separator);
  SharedBufferChunkReader(const SharedBufferChunkReader&) = delete;
  SharedBufferChunkReader& operator=(const SharedBufferChunkReader&) = delete;

  void SetSeparator(std::string_view separator);

  // Returns false when the end of the buffer was reached.
  bool NextChunk(Vector<char>& data, bool include_separator = false);

  // Returns a null string when the end of the buffer has been reached.
  String NextChunkAsUTF8StringWithLatin1Fallback(
      bool include_separator = false);

  // Reads size bytes at the current location in the buffer, without changing
  // the buffer position.
  // Returns the number of bytes read. That number might be less than the
  // specified size if the end of the buffer was reached.
  size_t Peek(Vector<char>&, size_t);

 private:
  scoped_refptr<const SharedBuffer> buffer_;
  size_t buffer_position_;
  base::span<const char> segment_;
  uint32_t segment_index_;
  bool reached_end_of_file_;
  Vector<char> separator_;
  uint32_t separator_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MHTML_SHARED_BUFFER_CHUNK_READER_H_
