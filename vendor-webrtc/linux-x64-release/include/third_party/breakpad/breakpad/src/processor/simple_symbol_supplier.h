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

// simple_symbol_supplier.h: A simple SymbolSupplier implementation
//
// SimpleSymbolSupplier is a straightforward implementation of SymbolSupplier
// that stores symbol files in a filesystem tree.  A SimpleSymbolSupplier is
// created with one or more base directories, which are the root paths for all
// symbol files.  Each symbol file contained therein has a directory entry in
// the base directory with a name identical to the corresponding debugging 
// file (pdb).  Within each of these directories, there are subdirectories
// named for the debugging file's identifier.  For recent pdb files, this is
// a concatenation of the pdb's uuid and age, presented in hexadecimal form,
// without any dashes or separators.  The uuid is in uppercase hexadecimal
// and the age is in lowercase hexadecimal.  Within that subdirectory,
// SimpleSymbolSupplier expects to find the symbol file, which is named
// identically to the debug file, but with a .sym extension.  If the original
// debug file had a name ending in .pdb, the .pdb extension will be replaced
// with .sym.  This sample hierarchy is rooted at the "symbols" base
// directory:
//
// symbols
// symbols/test_app.pdb
// symbols/test_app.pdb/63FE4780728D49379B9D7BB6460CB42A1
// symbols/test_app.pdb/63FE4780728D49379B9D7BB6460CB42A1/test_app.sym
// symbols/kernel32.pdb
// symbols/kernel32.pdb/BCE8785C57B44245A669896B6A19B9542
// symbols/kernel32.pdb/BCE8785C57B44245A669896B6A19B9542/kernel32.sym
//
// In this case, the uuid of test_app.pdb is
// 63fe4780-728d-4937-9b9d-7bb6460cb42a and its age is 1.
//
// This scheme was chosen to be roughly analogous to the way that
// symbol files may be accessed from Microsoft Symbol Server.  A hierarchy
// used for Microsoft Symbol Server storage is usable as a hierarchy for
// SimpleSymbolServer, provided that the pdb files are transformed to dumped
// format using a tool such as dump_syms, and given a .sym extension.
//
// SimpleSymbolSupplier will iterate over all root paths searching for
// a symbol file existing in that path.
//
// SimpleSymbolSupplier supports any debugging file which can be identified
// by a CodeModule object's debug_file and debug_identifier accessors.  The
// expected ultimate source of these CodeModule objects are MinidumpModule
// objects; it is this class that is responsible for assigning appropriate
// values for debug_file and debug_identifier.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_SIMPLE_SYMBOL_SUPPLIER_H__
#define PROCESSOR_SIMPLE_SYMBOL_SUPPLIER_H__

#include <map>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/processor/symbol_supplier.h"

namespace google_breakpad {

using std::map;
using std::vector;

class CodeModule;

class SimpleSymbolSupplier : public SymbolSupplier {
 public:
  // Creates a new SimpleSymbolSupplier, using path as the root path where
  // symbols are stored.
  explicit SimpleSymbolSupplier(const string& path) : paths_(1, path) {}

  // Creates a new SimpleSymbolSupplier, using paths as a list of root
  // paths where symbols may be stored.
  explicit SimpleSymbolSupplier(const vector<string>& paths) : paths_(paths) {}

  virtual ~SimpleSymbolSupplier() {}

  // Returns the path to the symbol file for the given module.  See the
  // description above.
  virtual SymbolResult GetSymbolFile(const CodeModule* module,
                                     const SystemInfo* system_info,
                                     string* symbol_file);

  virtual SymbolResult GetSymbolFile(const CodeModule* module,
                                     const SystemInfo* system_info,
                                     string* symbol_file,
                                     string* symbol_data);

  // Allocates data buffer on heap and writes symbol data into buffer.
  // Symbol supplier ALWAYS takes ownership of the data buffer.
  virtual SymbolResult GetCStringSymbolData(const CodeModule* module,
                                            const SystemInfo* system_info,
                                            string* symbol_file,
                                            char** symbol_data,
                                            size_t* symbol_data_size);

  // Free the data buffer allocated in the above GetCStringSymbolData();
  virtual void FreeSymbolData(const CodeModule* module);

 protected:
  SymbolResult GetSymbolFileAtPathFromRoot(const CodeModule* module,
                                           const SystemInfo* system_info,
                                           const string& root_path,
                                           string* symbol_file);

 private:
  map<string, char*> memory_buffers_;
  vector<string> paths_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_SIMPLE_SYMBOL_SUPPLIER_H__
