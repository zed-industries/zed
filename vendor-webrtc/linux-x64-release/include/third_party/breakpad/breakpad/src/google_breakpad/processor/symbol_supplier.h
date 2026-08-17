// Copyright 2006 Google LLC
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

// The caller may implement the SymbolSupplier abstract base class
// to provide symbols for a given module.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_SYMBOL_SUPPLIER_H__
#define GOOGLE_BREAKPAD_PROCESSOR_SYMBOL_SUPPLIER_H__

#include <string>
#include "common/using_std_string.h"

namespace google_breakpad {

class CodeModule;
struct SystemInfo;

class SymbolSupplier {
 public:
  // Result type for GetSymbolFile
  enum SymbolResult {
    // no symbols were found, but continue processing
    NOT_FOUND,

    // symbols were found, and the path has been placed in symbol_file
    FOUND,

    // stops processing the minidump immediately
    INTERRUPT
  };

  virtual ~SymbolSupplier() {}

  // Retrieves the symbol file for the given CodeModule, placing the
  // path in symbol_file if successful.  system_info contains strings
  // identifying the operating system and CPU; SymbolSupplier may use
  // to help locate the symbol file.  system_info may be NULL or its
  // fields may be empty if these values are unknown.  symbol_file
  // must be a pointer to a valid string
  virtual SymbolResult GetSymbolFile(const CodeModule* module,
                                     const SystemInfo* system_info,
                                     string* symbol_file) = 0;
  // Same as above, except also places symbol data into symbol_data.
  // If symbol_data is NULL, the data is not returned.
  // TODO(nealsid) Once we have symbol data caching behavior implemented
  // investigate making all symbol suppliers implement all methods,
  // and make this pure virtual
  virtual SymbolResult GetSymbolFile(const CodeModule* module,
                                     const SystemInfo* system_info,
                                     string* symbol_file,
                                     string* symbol_data) = 0;

  // Same as above, except allocates data buffer on heap and then places the
  // symbol data into the buffer as C-string.
  // SymbolSupplier is responsible for deleting the data buffer. After the call
  // to GetCStringSymbolData(), the caller should call FreeSymbolData(const
  // Module* module) once the data buffer is no longer needed.
  // If symbol_data is not NULL, symbol supplier won't return FOUND unless it
  // returns a valid buffer in symbol_data, e.g., returns INTERRUPT on memory
  // allocation failure.
  virtual SymbolResult GetCStringSymbolData(const CodeModule* module,
                                            const SystemInfo* system_info,
                                            string* symbol_file,
                                            char** symbol_data,
                                            size_t* symbol_data_size) = 0;

  // Frees the data buffer allocated for the module in GetCStringSymbolData.
  virtual void FreeSymbolData(const CodeModule* module) = 0;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_SYMBOL_SUPPLIER_H__
