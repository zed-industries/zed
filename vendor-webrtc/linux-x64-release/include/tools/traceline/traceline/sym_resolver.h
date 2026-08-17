// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A smaller wrapper around the dbghelp symbol resolution routines.
// For example:
//   SymResolver resolver("ntdll.dll");
//   resolver.Resolve("ntdll!NtBlahBlah");

#ifndef TRACELINE_SYM_RESOLVER_H_
#define TRACELINE_SYM_RESOLVER_H_

#include <windows.h>

#include <dbghelp.h>

#include <map>
#include <string>
#include <vector>

static BOOL CALLBACK SymEnumer(PCSTR name, DWORD64 base, PVOID context) {
  reinterpret_cast<std::vector<DWORD64>*>(context)->push_back(base);
  return TRUE;
}

class SymResolver {
 public:

  // Constructor to load a single DLL.
  SymResolver(const char* dllname, HANDLE proc = ::GetCurrentProcess())
      : proc_(proc) {

    // TODO(deanm): Would be nice to get this from WinDBG, but it's buried
    // in the workspace data blob... _NT_SYMBOL_PATH is not usually set...
    static char* kSymbolPath =
        "C:\\Program Files\\Debugging Tools for Windows (x86)\\sym;"
        "C:\\Program Files\\Debugging Tools for Windows\\sym";

    // If we want to load a specific DLL, or we want to load all.
    if (::SymInitialize(proc_, kSymbolPath, dllname ? FALSE : TRUE) != TRUE) {
      NOTREACHED("SymInitialize failed: %d", GetLastError());
    }

    base_ = 0;

    if (dllname) {
      base_ = ::SymLoadModuleEx(proc_,
                                NULL,
                                const_cast<char*>(dllname),
                                NULL,
                                reinterpret_cast<DWORD64>(
                                    GetModuleHandleA(dllname)),
                                0,
                                NULL,
                                0);
      if (base_ == 0) {
        NOTREACHED("SymLoadModuleEx(%s) failed: %d", dllname, GetLastError());
      }
    }

    std::vector<DWORD64> bases;
    // The name returned from SymEnumerateModules64 doesn't include the ext,
    // so we can't differentiate between a dll and exe of the same name. So
    // collect all of the base addresses and query for more info.
    // The prototype changed from PSTR to PCSTR, so in order to support older
    // SDKs we have to cast SymEnumer.
    PSYM_ENUMMODULES_CALLBACK64 enumer =
        reinterpret_cast<PSYM_ENUMMODULES_CALLBACK64>(&SymEnumer);
    if (SymEnumerateModules64(proc_, enumer, &bases) != TRUE) {
      NOTREACHED("SymEnumerateModules64 failed: %d\n", GetLastError());
    }
    for (size_t i = 0; i < bases.size(); ++i) {
      // This was failing, turns out I was just using the system32
      // dbghelp.dll which is old, use the one from windbg :(
      IMAGEHLP_MODULE64 info;
      info.SizeOfStruct = sizeof(info);
      if (SymGetModuleInfo64(proc_, bases[i], &info) != TRUE) {
        NOTREACHED("SymGetModuleInfo64 failed: %d\n", GetLastError());
      }
      std::string filename(info.ImageName);
      size_t last_slash = filename.find_last_of('\\');
      if (last_slash != std::string::npos)
        filename = filename.substr(filename.find_last_of('\\') + 1);

      // Map the base address to the image name...
      dlls_[static_cast<int>(bases[i])] = filename;
    }

    // TODO(deanm): check the symbols are rad and stuff...
  }

  char* Resolve(const char* name) {
    // The API writes to the space after SYMBOL_INFO...
    struct {
      SYMBOL_INFO info;
      char buf[128];
    } info = {0};

    info.info.SizeOfStruct = sizeof(info.info);
    info.info.ModBase = base_;
    info.info.MaxNameLen = 127;

    if (SymFromName(proc_, const_cast<char*>(name), &info.info) != TRUE) {
      NOTREACHED("SymFromName(%s) failed: %d", name, GetLastError());
    }

    return reinterpret_cast<char*>(info.info.Address);
  }

  std::string Unresolve(int ptr) {
    // The API writes to the space after SYMBOL_INFO...
    struct {
      SYMBOL_INFO info;
      char buf[128];
    } info = {0};

    info.info.SizeOfStruct = sizeof(info.info);
    info.info.ModBase = base_;
    info.info.MaxNameLen = 127;
    if (!::SymFromAddr(proc_, static_cast<DWORD64>(ptr), NULL, &info.info)) {
      return std::string("failed");
    }

    std::string name;
    int addr = static_cast<int>(info.info.Address);
    int base = static_cast<int>(info.info.ModBase);

    if (dlls_.count(base) == 1) {
      name.append(dlls_[base]);
    } else {
      name.append("unknown_mod");
    }
    name.push_back('!');
    name.append(info.info.Name);

    char buf[32];
    _itoa_s(ptr - addr, buf, sizeof(buf), 16);
    name.append("+0x");
    name.append(buf);

    DWORD disp;
    IMAGEHLP_LINE64 line;
    if (::SymGetLineFromAddr64(
            proc_, static_cast<DWORD64>(ptr), &disp, &line)) {
      name.append(" [ ");
      name.append(line.FileName);
      name.append(":");
      _itoa_s(line.LineNumber, buf, sizeof(buf), 10);
      name.append(buf);
      name.append(" ]");
    }

    return name;
  }

  ~SymResolver() {
    if (::SymCleanup(proc_) != TRUE) {
      NOTREACHED("SymCleanup failed: %d", GetLastError());
    }
  }

 private:
  HANDLE proc_;
  ULONG64 base_;
  std::map<int, std::string> dlls_;
};

#endif  // TRACELINE_SYM_RESOLVER_H_
