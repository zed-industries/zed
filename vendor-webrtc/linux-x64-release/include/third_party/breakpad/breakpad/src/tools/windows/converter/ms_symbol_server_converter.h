// Copyright 2007 Google LLC
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

// ms_symbol_server_converter.h: Obtain symbol files from a Microsoft
// symbol server, and convert them to Breakpad's dumped format.
//
// At runtime, MSSymbolServerConverter and code that it calls depend on being
// able to locate suitable versions of dbghelp.dll and symsrv.dll.  For best
// results, place these files in the same directory as the executable.
// dbghelp.dll and symsrv.dll as supplied with Debugging Tools for Windows are
// both redistributable, as indicated by the package's redist.txt file.
//
// When connecting to Microsoft's symbol server at
// http://msdl.microsoft.com/download/symbols/, which provides access to
// symbols for the operating system itself, symsrv.dll requires agreement to
// Microsoft's "Terms of Use for Microsoft Symbols and Binaries."  Because this
// library places the symbol engine into a promptless mode, the dialog with the
// terms will not appear, and use of Microsoft's symbol server will not be
// possible.  To indicate agreement to the terms, create a file called
// symsrv.yes in the same directory as symsrv.dll.  (Note that symsrv.dll will
// also recognize a symsrv.no file as indicating that you do not accept the
// terms; the .yes file takes priority over the .no file.)  The terms of use
// are contained within symsrv.dll; they were formerly available online at
// http://www.microsoft.com/whdc/devtools/debugging/symsrvTOU2.mspx , but
// do not appear to be available online any longer as of January, 2007.  It is
// possible to view the terms from within WinDbg (Debugging Tools for Windows)
// by removing any symsrv.yes and symsrv.no files from WinDbg's directory,
// setting the symbol path to include Microsoft's symbol server (.sympath), and
// attempting to load symbols from their server (.reload).
//
// This code has been tested with dbghelp.dll 6.5.3.7 and symsrv.dll 6.5.3.8,
// included with Microsoft Visual Studio 8 in Common7/IDE.  This has also been
// tested with dbghelp.dll and symsrv.dll versions 6.6.7.5 and 6.12.2.633,
// included with the same versions of Debugging Tools for Windows, available at
// http://www.microsoft.com/whdc/devtools/debugging/ .
//
// Author: Mark Mentovai

#ifndef TOOLS_WINDOWS_MS_SYMBOL_SERVER_CONVERTER_H_
#define TOOLS_WINDOWS_MS_SYMBOL_SERVER_CONVERTER_H_

#include <windows.h>

#include <string>
#include <vector>

namespace google_breakpad {

using std::string;
using std::vector;

// MissingSymbolInfo contains the subset of the information in the processor's
// CodeModule structure relevant to obtaining a missing symbol file.  Only
// debug_file and debug_identifier are relevant in actually obtaining the
// missing file; the other fields are for convenience.
struct MissingSymbolInfo {
  string code_file;
  string code_identifier;
  string debug_file;
  string debug_identifier;
  string version;
};

class GUIDOrSignatureIdentifier {
 public:
  enum GUIDOrSignatureType {
    TYPE_NONE = 0,
    TYPE_GUID,
    TYPE_SIGNATURE
  };

  GUIDOrSignatureIdentifier() : type_(TYPE_NONE) {}

  // Converts |identifier|, a debug_identifier-formatted string, into its
  // component fields: either a GUID and age, or signature and age.  If
  // successful, sets the relevant fields in the object, including the type
  // field, and returns true.  On error, returns false.
  bool InitializeFromString(const string& identifier);

  GUIDOrSignatureType type() const { return type_; }
  GUID guid() const { return guid_; }
  DWORD signature() const { return signature_; }
  int age() const { return age_; }
  const void* guid_or_signature_pointer() const { return &guid_; }

 private:
  GUIDOrSignatureType type_;

  // An identifier contains either a 128-bit uuid or a 32-bit signature.
  union {
    GUID guid_;
    DWORD signature_;
  };

  // All identifiers used here have age fields, which indicate a specific
  // revision given a uuid or signature.
  int age_;
};

class MSSymbolServerConverter {
 public:
  enum LocateResult {
    LOCATE_FAILURE = 0,
    LOCATE_NOT_FOUND,  // Authoritative: the file is not present.
    LOCATE_RETRY,      // Transient (network?) error, try again later.
    LOCATE_SUCCESS,
    LOCATE_HTTP_HTTPS_REDIR
  };

  // Create a new object.  local_cache is the location (pathname) of a local
  // symbol store used to hold downloaded and converted symbol files.  This
  // directory will be created by LocateSymbolFile when it successfully
  // retrieves a symbol file. symbol_servers contains a list of locations (URLs
  // or pathnames) of the upstream symbol server stores, given in order of
  // preference, with the first string in the vector identifying the first
  // store to try.  The vector must contain at least one string.  None of the
  // strings passed to this constructor may contain asterisk ('*') or semicolon
  // (';') characters, as the symbol engine uses these characters as separators.
  // If |trace_symsrv| is set then callbacks from SymSrv will be logged to
  // stderr.
  MSSymbolServerConverter(const string& local_cache,
                          const vector<string>& symbol_servers,
                          bool trace_symsrv);

  // Locates the PE file (DLL or EXE) specified by the identifying information
  // in |missing|, by checking the symbol stores identified when the object
  // was created.  When returning LOCATE_SUCCESS, pe_file is set to
  // the pathname of the decompressed PE file as it is stored in the
  // local cache.
  LocateResult LocatePEFile(const MissingSymbolInfo& missing, string* pe_file);

  // Locates the symbol file specified by the identifying information in
  // |missing|, by checking the symbol stores identified when the object
  // was created.  When returning LOCATE_SUCCESS, symbol_file is set to
  // the pathname of the decompressed symbol file as it is stored in the
  // local cache.
  LocateResult LocateSymbolFile(const MissingSymbolInfo& missing,
                                string* symbol_file);

  // Calls LocateSymbolFile and converts the returned symbol file to the
  // dumped-symbol format, storing it adjacent to the symbol file.  The
  // only conversion supported is from pdb files.  Returns the return
  // value of LocateSymbolFile, or if LocateSymbolFile succeeds but
  // conversion fails, returns LOCATE_FAILURE.  The pathname to the
  // pdb file and to the converted symbol file are returned in
  // |converted_symbol_file|, |symbol_file|, and |pe_file|.  |symbol_file| and
  // |pe_file| are optional and may be NULL.  If only the converted symbol file
  // is desired, set |keep_symbol_file| and |keep_pe_file| to false to indicate
  // that the original symbol file (pdb) and executable file (exe, dll) should
  // be deleted after conversion.
  LocateResult LocateAndConvertSymbolFile(const MissingSymbolInfo& missing,
                                          bool keep_symbol_file,
                                          bool keep_pe_file,
                                          string* converted_symbol_file,
                                          string* symbol_file,
                                          string* pe_file);

  // Calls LocatePEFile and converts the returned PE file to the
  // dumped-symbol format, storing it adjacent to the PE file. The
  // only conversion supported is from PE files. Returns the return
  // value of LocatePEFile, or if LocatePEFile succeeds but
  // conversion fails, returns LOCATE_FAILURE. The pathname to the
  // PE file and to the converted symbol file are returned in
  // |converted_symbol_file| and |pe_file|. |pe_file| is optional and may be
  // NULL.  If only the converted symbol file is desired, set |keep_pe_file|
  // to false to indicate that the executable file (exe, dll) should be deleted
  // after conversion.
  // NOTE: Currrently only supports x64 PEs.
  LocateResult LocateAndConvertPEFile(const MissingSymbolInfo& missing,
                                      bool keep_pe_file,
                                      string* converted_symbol_file,
                                      string* pe_file);

 private:
  // Locates the PDB or PE file (DLL or EXE) specified by the identifying
  // information in |debug_or_code_file| and |debug_or_code_id|, by checking
  // the symbol stores identified when the object was created.  When
  // returning LOCATE_SUCCESS, file_name is set to the pathname of the
  // decompressed PDB or PE file file as it is stored in the local cache.
  LocateResult LocateFile(const string& debug_or_code_file,
                          const string& debug_or_code_id,
                          const string& version, string* file_name);

  // Called by various SymSrv functions to report status as progress is made
  // and to allow the callback to influence processing.  Messages sent to this
  // callback can be used to distinguish between the various failure modes
  // that SymFindFileInPath might encounter.
  static BOOL CALLBACK SymCallback(HANDLE process, ULONG action, ULONG64 data,
                                   ULONG64 context);

  // Called by SymFindFileInPath (in LocateSymbolFile) after a candidate
  // symbol file is located, when it's present in the local cache.
  // SymFindFileInPath actually seems to accept NULL for a callback function
  // and behave properly for our needs in that case, but the documentation
  // doesn't mention it, so this little callback is provided.
  static BOOL CALLBACK SymFindFileInPathCallback(const char* filename,
                                                 void* context);

  // The search path used by SymSrv, built based on the arguments to the
  // constructor.
  string symbol_path_;

  // SymCallback will set at least one of these failure variables if
  // SymFindFileInPath fails for an expected reason.
  bool fail_dns_;        // DNS failures (fail_not_found_ will also be set).
  bool fail_timeout_;    // Timeouts (fail_not_found_ will also be set).
  bool fail_http_https_redir_;  // Bad URL-- we should be using HTTPS.
  bool fail_not_found_;  // The file could not be found.  If this is the only
                         // fail_* member set, then it is authoritative.

  // If set then callbacks from SymSrv will be logged to stderr.
  bool trace_symsrv_;
};

}  // namespace google_breakpad

#endif  // TOOLS_WINDOWS_MS_SYMBOL_SERVER_CONVERTER_H_
