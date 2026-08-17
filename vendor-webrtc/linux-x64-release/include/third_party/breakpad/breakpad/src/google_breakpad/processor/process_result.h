// Copyright 2014 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_PROCESS_RESULT_H__
#define GOOGLE_BREAKPAD_PROCESSOR_PROCESS_RESULT_H__

namespace google_breakpad {

// Return type for MinidumpProcessor or MicrodumpProcessor's Process()
enum ProcessResult {
  PROCESS_OK,                                  // The dump was processed
                                               // successfully.

  PROCESS_ERROR_MINIDUMP_NOT_FOUND,            // The minidump file was not
                                               // found.

  PROCESS_ERROR_NO_MINIDUMP_HEADER,            // The minidump file had no
                                               // header.

  PROCESS_ERROR_NO_THREAD_LIST,                // The minidump file has no
                                               // thread list.

  PROCESS_ERROR_GETTING_THREAD,                // There was an error getting one
                                               // thread's data from th dump.

  PROCESS_ERROR_GETTING_THREAD_ID,             // There was an error getting a
                                               // thread id from the thread's
                                               // data.

  PROCESS_ERROR_DUPLICATE_REQUESTING_THREADS,  // There was more than one
                                               // requesting thread.

  PROCESS_SYMBOL_SUPPLIER_INTERRUPTED,         // The dump processing was
                                               // interrupted by the
                                               // SymbolSupplier(not fatal).

  PROCESS_ERROR_GETTING_THREAD_NAME,           // There was an error getting one
                                               // thread's name from the dump.

};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_PROCESS_RESULT_H__
