/* ----------------------------------------------------------------------------
Copyright (c) 2018-2023, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

// This file is included in `src/prim/prim.c`

#include "mimalloc.h"
#include "mimalloc/internal.h"
#include "mimalloc/prim.h"
#include <stdio.h>   // fputs, stderr

// xbox has no console IO
#if !defined(WINAPI_FAMILY_PARTITION) || WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#define MI_HAS_CONSOLE_IO
#endif

//---------------------------------------------
// Dynamically bind Windows API points for portability
//---------------------------------------------

#if defined(_MSC_VER)
#pragma warning(disable:4996)   // don't use GetVersionExW
#endif

static DWORD win_major_version = 6;
static DWORD win_minor_version = 0;

// We use VirtualAlloc2 for aligned allocation, but it is only supported on Windows 10 and Windows Server 2016.
// So, we need to look it up dynamically to run on older systems. (use __stdcall for 32-bit compatibility)
// NtAllocateVirtualAllocEx is used for huge OS page allocation (1GiB)
// We define a minimal MEM_EXTENDED_PARAMETER ourselves in order to be able to compile with older SDK's.
typedef enum MI_MEM_EXTENDED_PARAMETER_TYPE_E {
  MiMemExtendedParameterInvalidType = 0,
  MiMemExtendedParameterAddressRequirements,
  MiMemExtendedParameterNumaNode,
  MiMemExtendedParameterPartitionHandle,
  MiMemExtendedParameterUserPhysicalHandle,
  MiMemExtendedParameterAttributeFlags,
  MiMemExtendedParameterMax
} MI_MEM_EXTENDED_PARAMETER_TYPE;

typedef struct DECLSPEC_ALIGN(8) MI_MEM_EXTENDED_PARAMETER_S {
  struct { DWORD64 Type : 8; DWORD64 Reserved : 56; } Type;
  union  { DWORD64 ULong64; PVOID Pointer; SIZE_T Size; HANDLE Handle; DWORD ULong; } Arg;
} MI_MEM_EXTENDED_PARAMETER;

typedef struct MI_MEM_ADDRESS_REQUIREMENTS_S {
  PVOID  LowestStartingAddress;
  PVOID  HighestEndingAddress;
  SIZE_T Alignment;
} MI_MEM_ADDRESS_REQUIREMENTS;

#define MI_MEM_EXTENDED_PARAMETER_NONPAGED_HUGE   0x00000010

#include <winternl.h>
typedef PVOID (__stdcall *PVirtualAlloc2)(HANDLE, PVOID, SIZE_T, ULONG, ULONG, MI_MEM_EXTENDED_PARAMETER*, ULONG);
typedef LONG  (__stdcall *PNtAllocateVirtualMemoryEx)(HANDLE, PVOID*, SIZE_T*, ULONG, ULONG, MI_MEM_EXTENDED_PARAMETER*, ULONG);  // avoid NTSTATUS as it is not defined on xbox (pr #1084)
static PVirtualAlloc2 pVirtualAlloc2 = NULL;
static PNtAllocateVirtualMemoryEx pNtAllocateVirtualMemoryEx = NULL;

// Similarly, GetNumaProcessorNodeEx is only supported since Windows 7  (and GetNumaNodeProcessorMask is not supported on xbox)
typedef struct MI_PROCESSOR_NUMBER_S { WORD Group; BYTE Number; BYTE Reserved; } MI_PROCESSOR_NUMBER;

typedef VOID (__stdcall *PGetCurrentProcessorNumberEx)(MI_PROCESSOR_NUMBER* ProcNumber);
typedef BOOL (__stdcall *PGetNumaProcessorNodeEx)(MI_PROCESSOR_NUMBER* Processor, PUSHORT NodeNumber);
typedef BOOL (__stdcall* PGetNumaNodeProcessorMaskEx)(USHORT Node, PGROUP_AFFINITY ProcessorMask);
typedef BOOL (__stdcall *PGetNumaProcessorNode)(UCHAR Processor, PUCHAR NodeNumber);
typedef BOOL (__stdcall* PGetNumaNodeProcessorMask)(UCHAR Node, PULONGLONG ProcessorMask);
typedef BOOL (__stdcall* PGetNumaHighestNodeNumber)(PULONG Node);
static PGetCurrentProcessorNumberEx pGetCurrentProcessorNumberEx = NULL;
static PGetNumaProcessorNodeEx      pGetNumaProcessorNodeEx = NULL;
static PGetNumaNodeProcessorMaskEx  pGetNumaNodeProcessorMaskEx = NULL;
static PGetNumaProcessorNode        pGetNumaProcessorNode = NULL;
static PGetNumaNodeProcessorMask    pGetNumaNodeProcessorMask = NULL;
static PGetNumaHighestNodeNumber    pGetNumaHighestNodeNumber = NULL;

// Not available on xbox
typedef SIZE_T(__stdcall* PGetLargePageMinimum)(VOID);
static PGetLargePageMinimum pGetLargePageMinimum = NULL;

// Available after Windows XP
typedef BOOL (__stdcall *PGetPhysicallyInstalledSystemMemory)( PULONGLONG TotalMemoryInKilobytes );
typedef BOOL (__stdcall* PGetVersionExW)(LPOSVERSIONINFOW lpVersionInformation);


//---------------------------------------------
// Enable large page support dynamically (if possible)
//---------------------------------------------

static bool win_enable_large_os_pages(size_t* large_page_size)
{
  static bool large_initialized = false;
  if (large_initialized) return (_mi_os_large_page_size() > 0);
  large_initialized = true;
  if (pGetLargePageMinimum==NULL) return false;  // no large page support (xbox etc.)

  // Try to see if large OS pages are supported
  // To use large pages on Windows, we first need access permission
  // Set "Lock pages in memory" permission in the group policy editor
  // <https://devblogs.microsoft.com/oldnewthing/20110128-00/?p=11643>
  unsigned long err = 0;
  HANDLE token = NULL;
  BOOL ok = OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &token);
  if (ok) {
    TOKEN_PRIVILEGES tp;
    ok = LookupPrivilegeValue(NULL, TEXT("SeLockMemoryPrivilege"), &tp.Privileges[0].Luid);
    if (ok) {
      tp.PrivilegeCount = 1;
      tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
      ok = AdjustTokenPrivileges(token, FALSE, &tp, 0, (PTOKEN_PRIVILEGES)NULL, 0);
      if (ok) {
        err = GetLastError();
        ok = (err == ERROR_SUCCESS);
        if (ok && large_page_size != NULL && pGetLargePageMinimum != NULL) {
          *large_page_size = (*pGetLargePageMinimum)();
        }
      }
    }
    CloseHandle(token);
  }
  if (!ok) {
    if (err == 0) err = GetLastError();
    _mi_warning_message("cannot enable large OS page support, error %lu\n", err);
  }
  return (ok!=0);
}


//---------------------------------------------
// Initialize
//---------------------------------------------

static DWORD win_allocation_granularity = 64*MI_KiB;

void _mi_prim_mem_init( mi_os_mem_config_t* config )
{
  config->has_overcommit = false;
  config->has_partial_free = false;
  config->has_virtual_reserve = true;
  
  // get the page size
  SYSTEM_INFO si; _mi_memzero_var(si);
  GetSystemInfo(&si);
  if (si.dwPageSize > 0) { config->page_size = si.dwPageSize; }
  if (si.dwAllocationGranularity > 0) {
    config->alloc_granularity = si.dwAllocationGranularity;
    win_allocation_granularity = si.dwAllocationGranularity;
  }
  // get virtual address bits
  if ((uintptr_t)si.lpMaximumApplicationAddress > 0) {
    const size_t vbits = MI_SIZE_BITS - mi_clz((uintptr_t)si.lpMaximumApplicationAddress);
    config->virtual_address_bits = vbits;
  }

  // get the VirtualAlloc2 function
  HINSTANCE hDll = LoadLibrary(TEXT("kernelbase.dll"));
  if (hDll != NULL) {
    // use VirtualAlloc2FromApp if possible as it is available to Windows store apps
    pVirtualAlloc2 = (PVirtualAlloc2)(void (*)(void))GetProcAddress(hDll, "VirtualAlloc2FromApp");
    if (pVirtualAlloc2==NULL) pVirtualAlloc2 = (PVirtualAlloc2)(void (*)(void))GetProcAddress(hDll, "VirtualAlloc2");
    FreeLibrary(hDll);
  }
  // NtAllocateVirtualMemoryEx is used for huge page allocation
  hDll = LoadLibrary(TEXT("ntdll.dll"));
  if (hDll != NULL) {
    pNtAllocateVirtualMemoryEx = (PNtAllocateVirtualMemoryEx)(void (*)(void))GetProcAddress(hDll, "NtAllocateVirtualMemoryEx");
    FreeLibrary(hDll);
  }
  // Try to use Win7+ numa API
  hDll = LoadLibrary(TEXT("kernel32.dll"));
  if (hDll != NULL) {
    pGetCurrentProcessorNumberEx = (PGetCurrentProcessorNumberEx)(void (*)(void))GetProcAddress(hDll, "GetCurrentProcessorNumberEx");
    pGetNumaProcessorNodeEx = (PGetNumaProcessorNodeEx)(void (*)(void))GetProcAddress(hDll, "GetNumaProcessorNodeEx");
    pGetNumaNodeProcessorMaskEx = (PGetNumaNodeProcessorMaskEx)(void (*)(void))GetProcAddress(hDll, "GetNumaNodeProcessorMaskEx");
    pGetNumaProcessorNode = (PGetNumaProcessorNode)(void (*)(void))GetProcAddress(hDll, "GetNumaProcessorNode");
    pGetNumaNodeProcessorMask = (PGetNumaNodeProcessorMask)(void (*)(void))GetProcAddress(hDll, "GetNumaNodeProcessorMask");
    pGetNumaHighestNodeNumber = (PGetNumaHighestNodeNumber)(void (*)(void))GetProcAddress(hDll, "GetNumaHighestNodeNumber");
    pGetLargePageMinimum = (PGetLargePageMinimum)(void (*)(void))GetProcAddress(hDll, "GetLargePageMinimum");
    // Get physical memory (not available on XP, so check dynamically)
    PGetPhysicallyInstalledSystemMemory pGetPhysicallyInstalledSystemMemory = (PGetPhysicallyInstalledSystemMemory)(void (*)(void))GetProcAddress(hDll,"GetPhysicallyInstalledSystemMemory");
    if (pGetPhysicallyInstalledSystemMemory != NULL) {
      ULONGLONG memInKiB = 0;
      if ((*pGetPhysicallyInstalledSystemMemory)(&memInKiB)) {
        if (memInKiB > 0 && memInKiB <= SIZE_MAX) {
          config->physical_memory_in_kib = (size_t)memInKiB;
        }
      }
    }
    // Get Windows version
    PGetVersionExW pGetVersionExW = (PGetVersionExW)(void (*)(void))GetProcAddress(hDll, "GetVersionExW");
    if (pGetVersionExW != NULL) {
      OSVERSIONINFOW version; _mi_memzero_var(version);
      version.dwOSVersionInfoSize = sizeof(version);
      if ((*pGetVersionExW)(&version)) {
        win_major_version = version.dwMajorVersion;
        win_minor_version = version.dwMinorVersion;
      }
    }
    FreeLibrary(hDll);
  }
  // Enable large/huge OS page support?
  if (mi_option_is_enabled(mi_option_allow_large_os_pages) || mi_option_is_enabled(mi_option_reserve_huge_os_pages)) {
    win_enable_large_os_pages(&config->large_page_size);
  }
}


//---------------------------------------------
// Free
//---------------------------------------------

int _mi_prim_free(void* addr, size_t size ) {
  MI_UNUSED(size);
  DWORD errcode = 0;
  bool err = (VirtualFree(addr, 0, MEM_RELEASE) == 0);
  if (err) { errcode = GetLastError(); }
  if (errcode == ERROR_INVALID_ADDRESS) {
    // In mi_os_mem_alloc_aligned the fallback path may have returned a pointer inside
    // the memory region returned by VirtualAlloc; in that case we need to free using
    // the start of the region.
    MEMORY_BASIC_INFORMATION info; _mi_memzero_var(info);
    VirtualQuery(addr, &info, sizeof(info));
    if (info.AllocationBase < addr && ((uint8_t*)addr - (uint8_t*)info.AllocationBase) < (ptrdiff_t)(4*MI_MiB)) {
      errcode = 0;
      err = (VirtualFree(info.AllocationBase, 0, MEM_RELEASE) == 0);
      if (err) { errcode = GetLastError(); }
    }
  }
  return (int)errcode;
}


//---------------------------------------------
// VirtualAlloc
//---------------------------------------------

static void* win_virtual_alloc_prim_once(void* addr, size_t size, size_t try_alignment, DWORD flags) {
  #if (MI_INTPTR_SIZE >= 8)
  // on 64-bit systems, try to use the virtual address area after 2TiB for 4MiB aligned allocations
  if (addr == NULL) {
    void* hint = _mi_os_get_aligned_hint(try_alignment,size);
    if (hint != NULL) {
      void* p = VirtualAlloc(hint, size, flags, PAGE_READWRITE);
      if (p != NULL) return p;
      _mi_verbose_message("warning: unable to allocate hinted aligned OS memory (%zu bytes, error code: 0x%x, address: %p, alignment: %zu, flags: 0x%x)\n", size, GetLastError(), hint, try_alignment, flags);
      // fall through on error
    }
  }
  #endif
  // on modern Windows try use VirtualAlloc2 for aligned allocation
  if (addr == NULL && try_alignment > win_allocation_granularity && (try_alignment % _mi_os_page_size()) == 0 && pVirtualAlloc2 != NULL) {
    MI_MEM_ADDRESS_REQUIREMENTS reqs = { 0, 0, 0 };
    reqs.Alignment = try_alignment;
    MI_MEM_EXTENDED_PARAMETER param = { {0, 0}, {0} };
    param.Type.Type = MiMemExtendedParameterAddressRequirements;
    param.Arg.Pointer = &reqs;
    void* p = (*pVirtualAlloc2)(GetCurrentProcess(), addr, size, flags, PAGE_READWRITE, &param, 1);
    if (p != NULL) return p;
    _mi_warning_message("unable to allocate aligned OS memory (0x%zx bytes, error code: 0x%x, address: %p, alignment: 0x%zx, flags: 0x%x)\n", size, GetLastError(), addr, try_alignment, flags);
    // fall through on error
  }
  // last resort
  return VirtualAlloc(addr, size, flags, PAGE_READWRITE);
}

static bool win_is_out_of_memory_error(DWORD err) {
  switch (err) {
    case ERROR_COMMITMENT_MINIMUM:
    case ERROR_COMMITMENT_LIMIT:
    case ERROR_PAGEFILE_QUOTA:
    case ERROR_NOT_ENOUGH_MEMORY:
      return true;
    default:
      return false;
  }
}

static void* win_virtual_alloc_prim(void* addr, size_t size, size_t try_alignment, DWORD flags) {
  long max_retry_msecs = mi_option_get_clamp(mi_option_retry_on_oom, 0, 2000);  // at most 2 seconds
  if (max_retry_msecs == 1) { max_retry_msecs = 100; }  // if one sets the option to "true"
  for (long tries = 1; tries <= 10; tries++) {          // try at most 10 times (=2200ms)
    void* p = win_virtual_alloc_prim_once(addr, size, try_alignment, flags);
    if (p != NULL) {
      // success, return the address
      return p;
    }
    else if (max_retry_msecs > 0 && (try_alignment <= 8*MI_MiB) &&
              (flags&MEM_COMMIT) != 0 && (flags&MEM_LARGE_PAGES) == 0 &&
              win_is_out_of_memory_error(GetLastError())) {
      // if committing regular memory and being out-of-memory,
      // keep trying for a bit in case memory frees up after all. See issue #894
      _mi_warning_message("out-of-memory on OS allocation, try again... (attempt %lu, 0x%zx bytes, error code: 0x%x, address: %p, alignment: 0x%zx, flags: 0x%x)\n", tries, size, GetLastError(), addr, try_alignment, flags);
      long sleep_msecs = tries*40;  // increasing waits
      if (sleep_msecs > max_retry_msecs) { sleep_msecs = max_retry_msecs; }
      max_retry_msecs -= sleep_msecs;
      Sleep(sleep_msecs);
    }
    else {
      // otherwise return with an error
      break;
    }
  }
  return NULL;
}

static void* win_virtual_alloc(void* addr, size_t size, size_t try_alignment, DWORD flags, bool large_only, bool allow_large, bool* is_large) {
  mi_assert_internal(!(large_only && !allow_large));
  static _Atomic(size_t) large_page_try_ok; // = 0;
  void* p = NULL;
  // Try to allocate large OS pages (2MiB) if allowed or required.
  if ((large_only || _mi_os_use_large_page(size, try_alignment))
      && allow_large && (flags&MEM_COMMIT)!=0 && (flags&MEM_RESERVE)!=0) {
    size_t try_ok = mi_atomic_load_acquire(&large_page_try_ok);
    if (!large_only && try_ok > 0) {
      // if a large page allocation fails, it seems the calls to VirtualAlloc get very expensive.
      // therefore, once a large page allocation failed, we don't try again for `large_page_try_ok` times.
      mi_atomic_cas_strong_acq_rel(&large_page_try_ok, &try_ok, try_ok - 1);
    }
    else {
      // large OS pages must always reserve and commit.
      *is_large = true;
      p = win_virtual_alloc_prim(addr, size, try_alignment, flags | MEM_LARGE_PAGES);
      if (large_only) return p;
      // fall back to non-large page allocation on error (`p == NULL`).
      if (p == NULL) {
        mi_atomic_store_release(&large_page_try_ok,10UL);  // on error, don't try again for the next N allocations
      }
    }
  }
  // Fall back to regular page allocation
  if (p == NULL) {
    *is_large = ((flags&MEM_LARGE_PAGES) != 0);
    p = win_virtual_alloc_prim(addr, size, try_alignment, flags);
  }
  //if (p == NULL) { _mi_warning_message("unable to allocate OS memory (%zu bytes, error code: 0x%x, address: %p, alignment: %zu, flags: 0x%x, large only: %d, allow large: %d)\n", size, GetLastError(), addr, try_alignment, flags, large_only, allow_large); }
  return p;
}

int _mi_prim_alloc(void* hint_addr, size_t size, size_t try_alignment, bool commit, bool allow_large, bool* is_large, bool* is_zero, void** addr) {
  mi_assert_internal(size > 0 && (size % _mi_os_page_size()) == 0);
  mi_assert_internal(commit || !allow_large);
  mi_assert_internal(try_alignment > 0);
  *is_zero = true;
  int flags = MEM_RESERVE;
  if (commit) { flags |= MEM_COMMIT; }
  *addr = win_virtual_alloc(hint_addr, size, try_alignment, flags, false, allow_large, is_large);
  return (*addr != NULL ? 0 : (int)GetLastError());
}


//---------------------------------------------
// Commit/Reset/Protect
//---------------------------------------------
#ifdef _MSC_VER
#pragma warning(disable:6250)   // suppress warning calling VirtualFree without MEM_RELEASE (for decommit)
#endif

int _mi_prim_commit(void* addr, size_t size, bool* is_zero) {
  *is_zero = false;
  /*
  // zero'ing only happens on an initial commit... but checking upfront seems expensive..
  _MEMORY_BASIC_INFORMATION meminfo; _mi_memzero_var(meminfo);
  if (VirtualQuery(addr, &meminfo, size) > 0) {
    if ((meminfo.State & MEM_COMMIT) == 0) {
      *is_zero = true;
    }
  }
  */
  // commit
  void* p = VirtualAlloc(addr, size, MEM_COMMIT, PAGE_READWRITE);
  if (p == NULL) return (int)GetLastError();
  return 0;
}

int _mi_prim_decommit(void* addr, size_t size, bool* needs_recommit) {
  BOOL ok = VirtualFree(addr, size, MEM_DECOMMIT);
  *needs_recommit = true;  // for safety, assume always decommitted even in the case of an error.
  return (ok ? 0 : (int)GetLastError());
}

int _mi_prim_reset(void* addr, size_t size) {
  void* p = VirtualAlloc(addr, size, MEM_RESET, PAGE_READWRITE);
  mi_assert_internal(p == addr);
  #if 0
  if (p != NULL) {
    VirtualUnlock(addr,size); // VirtualUnlock after MEM_RESET removes the memory directly from the working set
  }
  #endif
  return (p != NULL ? 0 : (int)GetLastError());
}

int _mi_prim_reuse(void* addr, size_t size) {
  MI_UNUSED(addr); MI_UNUSED(size);
  return 0;
}

int _mi_prim_protect(void* addr, size_t size, bool protect) {
  DWORD oldprotect = 0;
  BOOL ok = VirtualProtect(addr, size, protect ? PAGE_NOACCESS : PAGE_READWRITE, &oldprotect);
  return (ok ? 0 : (int)GetLastError());
}


//---------------------------------------------
// Huge page allocation
//---------------------------------------------

static void* _mi_prim_alloc_huge_os_pagesx(void* hint_addr, size_t size, int numa_node)
{
  const DWORD flags = MEM_LARGE_PAGES | MEM_COMMIT | MEM_RESERVE;

  win_enable_large_os_pages(NULL);

  MI_MEM_EXTENDED_PARAMETER params[3] = { {{0,0},{0}},{{0,0},{0}},{{0,0},{0}} };
  // on modern Windows try use NtAllocateVirtualMemoryEx for 1GiB huge pages
  static bool mi_huge_pages_available = true;
  if (pNtAllocateVirtualMemoryEx != NULL && mi_huge_pages_available) {
    params[0].Type.Type = MiMemExtendedParameterAttributeFlags;
    params[0].Arg.ULong64 = MI_MEM_EXTENDED_PARAMETER_NONPAGED_HUGE;
    ULONG param_count = 1;
    if (numa_node >= 0) {
      param_count++;
      params[1].Type.Type = MiMemExtendedParameterNumaNode;
      params[1].Arg.ULong = (unsigned)numa_node;
    }
    SIZE_T psize = size;
    void* base = hint_addr;
    LONG err = (*pNtAllocateVirtualMemoryEx)(GetCurrentProcess(), &base, &psize, flags, PAGE_READWRITE, params, param_count);
    if (err == 0 && base != NULL) {
      return base;
    }
    else {
      // fall back to regular large pages
      mi_huge_pages_available = false; // don't try further huge pages
      _mi_warning_message("unable to allocate using huge (1GiB) pages, trying large (2MiB) pages instead (status 0x%lx)\n", err);
    }
  }
  // on modern Windows try use VirtualAlloc2 for numa aware large OS page allocation
  if (pVirtualAlloc2 != NULL && numa_node >= 0) {
    params[0].Type.Type = MiMemExtendedParameterNumaNode;
    params[0].Arg.ULong = (unsigned)numa_node;
    return (*pVirtualAlloc2)(GetCurrentProcess(), hint_addr, size, flags, PAGE_READWRITE, params, 1);
  }

  // otherwise use regular virtual alloc on older windows
  return VirtualAlloc(hint_addr, size, flags, PAGE_READWRITE);
}

int _mi_prim_alloc_huge_os_pages(void* hint_addr, size_t size, int numa_node, bool* is_zero, void** addr) {
  *is_zero = true;
  *addr = _mi_prim_alloc_huge_os_pagesx(hint_addr,size,numa_node);
  return (*addr != NULL ? 0 : (int)GetLastError());
}


//---------------------------------------------
// Numa nodes
//---------------------------------------------

size_t _mi_prim_numa_node(void) {
  USHORT numa_node = 0;
  if (pGetCurrentProcessorNumberEx != NULL && pGetNumaProcessorNodeEx != NULL) {
    // Extended API is supported
    MI_PROCESSOR_NUMBER pnum;
    (*pGetCurrentProcessorNumberEx)(&pnum);
    USHORT nnode = 0;
    BOOL ok = (*pGetNumaProcessorNodeEx)(&pnum, &nnode);
    if (ok) { numa_node = nnode; }
  }
  else if (pGetNumaProcessorNode != NULL) {
    // Vista or earlier, use older API that is limited to 64 processors. Issue #277
    DWORD pnum = GetCurrentProcessorNumber();
    UCHAR nnode = 0;
    BOOL ok = pGetNumaProcessorNode((UCHAR)pnum, &nnode);
    if (ok) { numa_node = nnode; }
  }
  return numa_node;
}

size_t _mi_prim_numa_node_count(void) {
  ULONG numa_max = 0;
  if (pGetNumaHighestNodeNumber!=NULL) {
    (*pGetNumaHighestNodeNumber)(&numa_max);
  }
  // find the highest node number that has actual processors assigned to it. Issue #282
  while (numa_max > 0) {
    if (pGetNumaNodeProcessorMaskEx != NULL) {
      // Extended API is supported
      GROUP_AFFINITY affinity;
      if ((*pGetNumaNodeProcessorMaskEx)((USHORT)numa_max, &affinity)) {
        if (affinity.Mask != 0) break;  // found the maximum non-empty node
      }
    }
    else {
      // Vista or earlier, use older API that is limited to 64 processors.
      ULONGLONG mask;
      if (pGetNumaNodeProcessorMask != NULL) {
        if ((*pGetNumaNodeProcessorMask)((UCHAR)numa_max, &mask)) {
          if (mask != 0) break; // found the maximum non-empty node
        }
      };
    }
    // max node was invalid or had no processor assigned, try again
    numa_max--;
  }
  return ((size_t)numa_max + 1);
}


//----------------------------------------------------------------
// Clock
//----------------------------------------------------------------

static mi_msecs_t mi_to_msecs(LARGE_INTEGER t) {
  static LARGE_INTEGER mfreq; // = 0
  if (mfreq.QuadPart == 0LL) {
    LARGE_INTEGER f;
    QueryPerformanceFrequency(&f);
    mfreq.QuadPart = f.QuadPart/1000LL;
    if (mfreq.QuadPart == 0) mfreq.QuadPart = 1;
  }
  return (mi_msecs_t)(t.QuadPart / mfreq.QuadPart);
}

mi_msecs_t _mi_prim_clock_now(void) {
  LARGE_INTEGER t;
  QueryPerformanceCounter(&t);
  return mi_to_msecs(t);
}


//----------------------------------------------------------------
// Process Info
//----------------------------------------------------------------

#include <psapi.h>

static mi_msecs_t filetime_msecs(const FILETIME* ftime) {
  ULARGE_INTEGER i;
  i.LowPart = ftime->dwLowDateTime;
  i.HighPart = ftime->dwHighDateTime;
  mi_msecs_t msecs = (i.QuadPart / 10000); // FILETIME is in 100 nano seconds
  return msecs;
}

typedef BOOL (WINAPI *PGetProcessMemoryInfo)(HANDLE, PPROCESS_MEMORY_COUNTERS, DWORD);
static PGetProcessMemoryInfo pGetProcessMemoryInfo = NULL;

void _mi_prim_process_info(mi_process_info_t* pinfo)
{
  FILETIME ct;
  FILETIME ut;
  FILETIME st;
  FILETIME et;
  GetProcessTimes(GetCurrentProcess(), &ct, &et, &st, &ut);
  pinfo->utime = filetime_msecs(&ut);
  pinfo->stime = filetime_msecs(&st);

  // load psapi on demand
  if (pGetProcessMemoryInfo == NULL) {
    HINSTANCE hDll = LoadLibrary(TEXT("psapi.dll"));
    if (hDll != NULL) {
      pGetProcessMemoryInfo = (PGetProcessMemoryInfo)(void (*)(void))GetProcAddress(hDll, "GetProcessMemoryInfo");
    }
  }

  // get process info
  PROCESS_MEMORY_COUNTERS info; _mi_memzero_var(info);
  if (pGetProcessMemoryInfo != NULL) {
    pGetProcessMemoryInfo(GetCurrentProcess(), &info, sizeof(info));
  }
  pinfo->current_rss    = (size_t)info.WorkingSetSize;
  pinfo->peak_rss       = (size_t)info.PeakWorkingSetSize;
  pinfo->current_commit = (size_t)info.PagefileUsage;
  pinfo->peak_commit    = (size_t)info.PeakPagefileUsage;
  pinfo->page_faults    = (size_t)info.PageFaultCount;
}

//----------------------------------------------------------------
// Output
//----------------------------------------------------------------

void _mi_prim_out_stderr( const char* msg )
{
  // on windows with redirection, the C runtime cannot handle locale dependent output
  // after the main thread closes so we use direct console output.
  if (!_mi_preloading()) {
    // _cputs(msg);  // _cputs cannot be used as it aborts when failing to lock the console
    static HANDLE hcon = INVALID_HANDLE_VALUE;
    static bool hconIsConsole = false;
    if (hcon == INVALID_HANDLE_VALUE) {
      hcon = GetStdHandle(STD_ERROR_HANDLE);
      #ifdef MI_HAS_CONSOLE_IO
      CONSOLE_SCREEN_BUFFER_INFO sbi;
      hconIsConsole = ((hcon != INVALID_HANDLE_VALUE) && GetConsoleScreenBufferInfo(hcon, &sbi));
      #endif  
    }
    const size_t len = _mi_strlen(msg);
    if (len > 0 && len < UINT32_MAX) {
      DWORD written = 0;
      if (hconIsConsole) {
        #ifdef MI_HAS_CONSOLE_IO
        WriteConsoleA(hcon, msg, (DWORD)len, &written, NULL);
        #endif      
      }
      else if (hcon != INVALID_HANDLE_VALUE) {
        // use direct write if stderr was redirected
        WriteFile(hcon, msg, (DWORD)len, &written, NULL);
      }
      else {
        // finally fall back to fputs after all
        fputs(msg, stderr);
      }
    }
  }
}


//----------------------------------------------------------------
// Environment
//----------------------------------------------------------------

// On Windows use GetEnvironmentVariable instead of getenv to work
// reliably even when this is invoked before the C runtime is initialized.
// i.e. when `_mi_preloading() == true`.
// Note: on windows, environment names are not case sensitive.
bool _mi_prim_getenv(const char* name, char* result, size_t result_size) {
  result[0] = 0;
  size_t len = GetEnvironmentVariableA(name, result, (DWORD)result_size);
  return (len > 0 && len < result_size);
}


//----------------------------------------------------------------
// Random
//----------------------------------------------------------------

#if defined(MI_USE_RTLGENRANDOM) // || defined(__cplusplus)
// We prefer to use BCryptGenRandom instead of (the unofficial) RtlGenRandom but when using
// dynamic overriding, we observed it can raise an exception when compiled with C++, and
// sometimes deadlocks when also running under the VS debugger.
// In contrast, issue #623 implies that on Windows Server 2019 we need to use BCryptGenRandom.
// To be continued..
#pragma comment (lib,"advapi32.lib")
#define RtlGenRandom  SystemFunction036
mi_decl_externc BOOLEAN NTAPI RtlGenRandom(PVOID RandomBuffer, ULONG RandomBufferLength);

bool _mi_prim_random_buf(void* buf, size_t buf_len) {
  return (RtlGenRandom(buf, (ULONG)buf_len) != 0);
}

#else

#ifndef BCRYPT_USE_SYSTEM_PREFERRED_RNG
#define BCRYPT_USE_SYSTEM_PREFERRED_RNG 0x00000002
#endif

typedef LONG (NTAPI *PBCryptGenRandom)(HANDLE, PUCHAR, ULONG, ULONG);
static  PBCryptGenRandom pBCryptGenRandom = NULL;

bool _mi_prim_random_buf(void* buf, size_t buf_len) {
  if (pBCryptGenRandom == NULL) {
    HINSTANCE hDll = LoadLibrary(TEXT("bcrypt.dll"));
    if (hDll != NULL) {
      pBCryptGenRandom = (PBCryptGenRandom)(void (*)(void))GetProcAddress(hDll, "BCryptGenRandom");
    }
    if (pBCryptGenRandom == NULL) return false;
  }
  return (pBCryptGenRandom(NULL, (PUCHAR)buf, (ULONG)buf_len, BCRYPT_USE_SYSTEM_PREFERRED_RNG) >= 0);
}

#endif  // MI_USE_RTLGENRANDOM



//----------------------------------------------------------------
// Process & Thread Init/Done
//----------------------------------------------------------------

#if MI_WIN_USE_FIXED_TLS==1
mi_decl_cache_align size_t _mi_win_tls_offset = 0;
#endif

//static void mi_debug_out(const char* s) {
//  HANDLE h = GetStdHandle(STD_ERROR_HANDLE);
//  WriteConsole(h, s, (DWORD)_mi_strlen(s), NULL, NULL);
//}

static void mi_win_tls_init(DWORD reason) {
  if (reason==DLL_PROCESS_ATTACH || reason==DLL_THREAD_ATTACH) {
    #if MI_WIN_USE_FIXED_TLS==1  // we must allocate a TLS slot dynamically
    if (_mi_win_tls_offset == 0 && reason == DLL_PROCESS_ATTACH) {
      const DWORD tls_slot = TlsAlloc();  // usually returns slot 1
      if (tls_slot == TLS_OUT_OF_INDEXES) {
        _mi_error_message(EFAULT, "unable to allocate the a TLS slot (rebuild without MI_WIN_USE_FIXED_TLS?)\n");
      }
      _mi_win_tls_offset = (size_t)tls_slot * sizeof(void*);
    }
    #endif
    #if MI_HAS_TLS_SLOT >= 2  // we must initialize the TLS slot before any allocation
    if (mi_prim_get_default_heap() == NULL) {
      _mi_heap_set_default_direct((mi_heap_t*)&_mi_heap_empty);
      #if MI_DEBUG && MI_WIN_USE_FIXED_TLS==1
      void* const p = TlsGetValue((DWORD)(_mi_win_tls_offset / sizeof(void*)));
      mi_assert_internal(p == (void*)&_mi_heap_empty);
      #endif
    }
    #endif
  }
}

static void NTAPI mi_win_main(PVOID module, DWORD reason, LPVOID reserved) {
  MI_UNUSED(reserved);
  MI_UNUSED(module);
  mi_win_tls_init(reason);
  if (reason==DLL_PROCESS_ATTACH) {
    _mi_auto_process_init();
  }
  else if (reason==DLL_PROCESS_DETACH) {
    _mi_auto_process_done();
  }
  else if (reason==DLL_THREAD_DETACH && !_mi_is_redirected()) {
    _mi_thread_done(NULL);
  }
}


#if defined(MI_SHARED_LIB)
  #define MI_PRIM_HAS_PROCESS_ATTACH  1

  // Windows DLL: easy to hook into process_init and thread_done
  BOOL WINAPI DllMain(HINSTANCE inst, DWORD reason, LPVOID reserved) {
    mi_win_main((PVOID)inst,reason,reserved);
    return TRUE;
  }

  // nothing to do since `_mi_thread_done` is handled through the DLL_THREAD_DETACH event.
  void _mi_prim_thread_init_auto_done(void) { }
  void _mi_prim_thread_done_auto_done(void) { }
  void _mi_prim_thread_associate_default_heap(mi_heap_t* heap) {
    MI_UNUSED(heap);
  }

#elif !defined(MI_WIN_USE_FLS)
  #define MI_PRIM_HAS_PROCESS_ATTACH  1

  static void NTAPI mi_win_main_attach(PVOID module, DWORD reason, LPVOID reserved) {
    if (reason == DLL_PROCESS_ATTACH || reason == DLL_THREAD_ATTACH) {
      mi_win_main(module, reason, reserved);
    }
  }
  static void NTAPI mi_win_main_detach(PVOID module, DWORD reason, LPVOID reserved) {
    if (reason == DLL_PROCESS_DETACH || reason == DLL_THREAD_DETACH) {
      mi_win_main(module, reason, reserved);
    }
  }

  // Set up TLS callbacks in a statically linked library by using special data sections.
  // See <https://stackoverflow.com/questions/14538159/tls-callback-in-windows>
  // We use 2 entries to ensure we call attach events before constructors
  // are called, and detach events after destructors are called.
  #if defined(__cplusplus)
  extern "C" {
  #endif

  #if defined(_WIN64)
    #pragma comment(linker, "/INCLUDE:_tls_used")
    #pragma comment(linker, "/INCLUDE:_mi_tls_callback_pre")
    #pragma comment(linker, "/INCLUDE:_mi_tls_callback_post")
    #pragma const_seg(".CRT$XLB")
    extern const PIMAGE_TLS_CALLBACK _mi_tls_callback_pre[];
    const PIMAGE_TLS_CALLBACK _mi_tls_callback_pre[] = { &mi_win_main_attach };
    #pragma const_seg()
    #pragma const_seg(".CRT$XLY")
    extern const PIMAGE_TLS_CALLBACK _mi_tls_callback_post[];
    const PIMAGE_TLS_CALLBACK _mi_tls_callback_post[] = { &mi_win_main_detach };
    #pragma const_seg()
  #else
    #pragma comment(linker, "/INCLUDE:__tls_used")
    #pragma comment(linker, "/INCLUDE:__mi_tls_callback_pre")
    #pragma comment(linker, "/INCLUDE:__mi_tls_callback_post")
    #pragma data_seg(".CRT$XLB")
    PIMAGE_TLS_CALLBACK _mi_tls_callback_pre[] = { &mi_win_main_attach };
    #pragma data_seg()
    #pragma data_seg(".CRT$XLY")
    PIMAGE_TLS_CALLBACK _mi_tls_callback_post[] = { &mi_win_main_detach };
    #pragma data_seg()
  #endif

  #if defined(__cplusplus)
  }
  #endif

  // nothing to do since `_mi_thread_done` is handled through the DLL_THREAD_DETACH event.
  void _mi_prim_thread_init_auto_done(void) { }
  void _mi_prim_thread_done_auto_done(void) { }
  void _mi_prim_thread_associate_default_heap(mi_heap_t* heap) {
    MI_UNUSED(heap);
  }

#else // deprecated: statically linked, use fiber api

  #if defined(_MSC_VER) // on clang/gcc use the constructor attribute (in `src/prim/prim.c`)
    // MSVC: use data section magic for static libraries
    // See <https://www.codeguru.com/cpp/misc/misc/applicationcontrol/article.php/c6945/Running-Code-Before-and-After-Main.htm>
    #define MI_PRIM_HAS_PROCESS_ATTACH 1

    static int mi_process_attach(void) {
      mi_win_main(NULL,DLL_PROCESS_ATTACH,NULL);
      atexit(&_mi_auto_process_done);
      return 0;
    }
    typedef int(*mi_crt_callback_t)(void);
    #if defined(_WIN64)
      #pragma comment(linker, "/INCLUDE:_mi_tls_callback")
      #pragma section(".CRT$XIU", long, read)
    #else
      #pragma comment(linker, "/INCLUDE:__mi_tls_callback")
    #endif
    #pragma data_seg(".CRT$XIU")
    mi_decl_externc mi_crt_callback_t _mi_tls_callback[] = { &mi_process_attach };
    #pragma data_seg()
  #endif

  // use the fiber api for calling `_mi_thread_done`.
  #include <fibersapi.h>
  #if (_WIN32_WINNT < 0x600)  // before Windows Vista
  WINBASEAPI DWORD WINAPI FlsAlloc( _In_opt_ PFLS_CALLBACK_FUNCTION lpCallback );
  WINBASEAPI PVOID WINAPI FlsGetValue( _In_ DWORD dwFlsIndex );
  WINBASEAPI BOOL  WINAPI FlsSetValue( _In_ DWORD dwFlsIndex, _In_opt_ PVOID lpFlsData );
  WINBASEAPI BOOL  WINAPI FlsFree(_In_ DWORD dwFlsIndex);
  #endif

  static DWORD mi_fls_key = (DWORD)(-1);

  static void NTAPI mi_fls_done(PVOID value) {
    mi_heap_t* heap = (mi_heap_t*)value;
    if (heap != NULL) {
      _mi_thread_done(heap);
      FlsSetValue(mi_fls_key, NULL);  // prevent recursion as _mi_thread_done may set it back to the main heap, issue #672
    }
  }

  void _mi_prim_thread_init_auto_done(void) {
    mi_fls_key = FlsAlloc(&mi_fls_done);
  }

  void _mi_prim_thread_done_auto_done(void) {
    // call thread-done on all threads (except the main thread) to prevent
    // dangling callback pointer if statically linked with a DLL; Issue #208
    FlsFree(mi_fls_key);
  }

  void _mi_prim_thread_associate_default_heap(mi_heap_t* heap) {
    mi_assert_internal(mi_fls_key != (DWORD)(-1));
    FlsSetValue(mi_fls_key, heap);
  }
#endif

// ----------------------------------------------------
// Communicate with the redirection module on Windows
// ----------------------------------------------------
#if defined(MI_SHARED_LIB) && !defined(MI_WIN_NOREDIRECT)
  #define MI_PRIM_HAS_ALLOCATOR_INIT 1

  static bool mi_redirected = false;   // true if malloc redirects to mi_malloc

  bool _mi_is_redirected(void) {
    return mi_redirected;
  }

  #ifdef __cplusplus
  extern "C" {
  #endif
  mi_decl_export void _mi_redirect_entry(DWORD reason) {
    // called on redirection; careful as this may be called before DllMain
    mi_win_tls_init(reason);
    if (reason == DLL_PROCESS_ATTACH) {
      mi_redirected = true;
    }
    else if (reason == DLL_PROCESS_DETACH) {
      mi_redirected = false;
    }
    else if (reason == DLL_THREAD_DETACH) {
      _mi_thread_done(NULL);
    }
  }
  __declspec(dllimport) bool mi_cdecl mi_allocator_init(const char** message);
  __declspec(dllimport) void mi_cdecl mi_allocator_done(void);
  #ifdef __cplusplus
  }
  #endif
  bool _mi_allocator_init(const char** message) {
    return mi_allocator_init(message);
  }
  void _mi_allocator_done(void) {
    mi_allocator_done();
  }
#endif

bool _mi_prim_thread_is_in_threadpool(void) {
  #if (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64)
  if (win_major_version >= 6) {
    // check if this thread belongs to a windows threadpool
    // see: <https://www.geoffchappell.com/studies/windows/km/ntoskrnl/inc/api/pebteb/teb/index.htm>
    struct _TEB* const teb = NtCurrentTeb();
    void* const pool_data = *((void**)((uint8_t*)teb + (MI_SIZE_BITS == 32 ? 0x0F90 : 0x1778)));
    return (pool_data != NULL);
  }
  #endif
  return false;
}
