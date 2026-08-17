// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This file implements memory allocation primitives for PageAllocator using
// Fuchsia's VMOs (Virtual Memory Objects). VMO API is documented in
// https://fuchsia.dev/fuchsia-src/zircon/objects/vm_object . A VMO is a kernel
// object that corresponds to a set of memory pages. VMO pages may be mapped
// to an address space. The code below creates VMOs for each memory allocations
// and maps them to the default address space of the current process.

#ifndef PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_FUCHSIA_H_
#define PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_FUCHSIA_H_

#include <fidl/fuchsia.kernel/cpp/fidl.h>
#include <lib/component/incoming/cpp/protocol.h>
#include <lib/zx/resource.h>
#include <lib/zx/vmar.h>
#include <lib/zx/vmo.h>

#include <cstdint>

#include "partition_alloc/page_allocator.h"
#include "partition_alloc/partition_alloc_base/fuchsia/fuchsia_logging.h"
#include "partition_alloc/partition_alloc_base/no_destructor.h"
#include "partition_alloc/partition_alloc_base/notreached.h"
#include "partition_alloc/partition_alloc_check.h"

namespace partition_alloc::internal {

zx::resource GetVmexResource() {
  auto vmex_resource_client =
      component::Connect<fuchsia_kernel::VmexResource>();
  if (vmex_resource_client.is_error()) {
    PA_LOG(ERROR) << "Connect(VmexResource):"
                  << vmex_resource_client.status_string();
    return {};
  }

  fidl::SyncClient sync_vmex_resource_client(
      std::move(vmex_resource_client.value()));
  auto result = sync_vmex_resource_client->Get();
  if (result.is_error()) {
    PA_LOG(ERROR) << "VmexResource.Get():"
                  << result.error_value().FormatDescription().c_str();
    return {};
  }

  return std::move(result->resource());
}

const zx::resource& VmexResource() {
  static base::NoDestructor<zx::resource> vmex_resource(GetVmexResource());
  return *vmex_resource;
}

// Returns VMO name for a PageTag.
const char* PageTagToName(PageTag tag) {
  switch (tag) {
    case PageTag::kBlinkGC:
      return "cr_blink_gc";
    case PageTag::kPartitionAlloc:
      return "cr_partition_alloc";
    case PageTag::kChromium:
      return "cr_chromium";
    case PageTag::kV8:
      return "cr_v8";
    case PageTag::kSimulation:
      PA_NOTREACHED();
  }
  PA_NOTREACHED();
}

zx_vm_option_t PageAccessibilityToZxVmOptions(
    PageAccessibilityConfiguration accessibility) {
  switch (accessibility.permissions) {
    case PageAccessibilityConfiguration::kRead:
      return ZX_VM_PERM_READ;
    case PageAccessibilityConfiguration::kReadWrite:
    case PageAccessibilityConfiguration::kReadWriteTagged:
      return ZX_VM_PERM_READ | ZX_VM_PERM_WRITE;
    case PageAccessibilityConfiguration::kReadExecuteProtected:
    case PageAccessibilityConfiguration::kReadExecute:
      return ZX_VM_PERM_READ | ZX_VM_PERM_EXECUTE;
    case PageAccessibilityConfiguration::kReadWriteExecuteProtected:
    case PageAccessibilityConfiguration::kReadWriteExecute:
      return ZX_VM_PERM_READ | ZX_VM_PERM_WRITE | ZX_VM_PERM_EXECUTE;
    case PageAccessibilityConfiguration::kInaccessible:
    case PageAccessibilityConfiguration::kInaccessibleWillJitLater:
      return 0;
  };
  PA_NOTREACHED();
}

// zx_vmar_map() will fail if the VMO cannot be mapped at |vmar_offset|, i.e.
// |hint| is not advisory.
constexpr bool kHintIsAdvisory = false;

std::atomic<int32_t> s_allocPageErrorCode{0};

uintptr_t SystemAllocPagesInternal(
    uintptr_t hint,
    size_t length,
    PageAccessibilityConfiguration accessibility,
    PageTag page_tag,
    [[maybe_unused]] int file_descriptor_for_shared_alloc) {
  zx::vmo vmo;
  zx_status_t status = zx::vmo::create(length, 0, &vmo);
  if (status != ZX_OK) {
    PA_ZX_DLOG(INFO, status) << "zx_vmo_create";
    return 0;
  }

  const char* vmo_name = PageTagToName(page_tag);
  status = vmo.set_property(ZX_PROP_NAME, vmo_name, strlen(vmo_name));

  // VMO names are used only for debugging, so failure to set a name is not
  // fatal.
  PA_ZX_DCHECK(status == ZX_OK, status);

  if (accessibility.permissions ==
          PageAccessibilityConfiguration::kInaccessibleWillJitLater ||
      accessibility.permissions ==
          PageAccessibilityConfiguration::kReadWriteExecute) {
    // V8 uses JIT. Call zx_vmo_replace_as_executable() to allow code execution
    // in the new VMO.
    status = vmo.replace_as_executable(VmexResource(), &vmo);
    if (status != ZX_OK) {
      PA_ZX_DLOG(INFO, status) << "zx_vmo_replace_as_executable";
      return 0;
    }
  }

  zx_vm_option_t options = PageAccessibilityToZxVmOptions(accessibility);

  uint64_t vmar_offset = 0;
  if (hint) {
    vmar_offset = hint;
    options |= ZX_VM_SPECIFIC;
  }

  uint64_t address;
  status = zx::vmar::root_self()->map(options, vmar_offset, vmo,
                                      /*vmo_offset=*/0, length, &address);
  if (status != ZX_OK) {
    // map() is expected to fail if |hint| is set to an already-in-use location.
    if (!hint) {
      PA_ZX_DLOG(ERROR, status) << "zx_vmar_map";
    }
    return 0;
  }

  return address;
}

uintptr_t TrimMappingInternal(uintptr_t base_address,
                              size_t base_length,
                              size_t trim_length,
                              PageAccessibilityConfiguration accessibility,
                              size_t pre_slack,
                              size_t post_slack) {
  PA_DCHECK(base_length == trim_length + pre_slack + post_slack);

  // Unmap head if necessary.
  if (pre_slack) {
    zx_status_t status = zx::vmar::root_self()->unmap(base_address, pre_slack);
    PA_ZX_CHECK(status == ZX_OK, status);
  }

  // Unmap tail if necessary.
  if (post_slack) {
    zx_status_t status = zx::vmar::root_self()->unmap(
        base_address + pre_slack + trim_length, post_slack);
    PA_ZX_CHECK(status == ZX_OK, status);
  }

  return base_address + pre_slack;
}

bool TrySetSystemPagesAccessInternal(
    uint64_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility) {
  zx_status_t status = zx::vmar::root_self()->protect(
      PageAccessibilityToZxVmOptions(accessibility), address, length);
  return status == ZX_OK;
}

void SetSystemPagesAccessInternal(
    uint64_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility) {
  zx_status_t status = zx::vmar::root_self()->protect(
      PageAccessibilityToZxVmOptions(accessibility), address, length);
  PA_ZX_CHECK(status == ZX_OK, status);
}

void FreePagesInternal(uint64_t address, size_t length) {
  zx_status_t status = zx::vmar::root_self()->unmap(address, length);
  PA_ZX_CHECK(status == ZX_OK, status);
}

void DiscardSystemPagesInternal(uint64_t address, size_t length) {
  zx_status_t status = zx::vmar::root_self()->op_range(
      ZX_VMO_OP_DECOMMIT, address, length, nullptr, 0);
  PA_ZX_CHECK(status == ZX_OK, status);
}

bool SealSystemPagesInternal(uint64_t address, size_t length) {
  return false;
}

void DecommitSystemPagesInternal(
    uint64_t address,
    size_t length,
    PageAccessibilityDisposition accessibility_disposition) {
  if (accessibility_disposition ==
      PageAccessibilityDisposition::kRequireUpdate) {
    SetSystemPagesAccess(address, length,
                         PageAccessibilityConfiguration(
                             PageAccessibilityConfiguration::kInaccessible));
  }

  DiscardSystemPagesInternal(address, length);
}

bool DecommitAndZeroSystemPagesInternal(uintptr_t address,
                                        size_t length,
                                        PageTag page_tag) {
  SetSystemPagesAccess(address, length,
                       PageAccessibilityConfiguration(
                           PageAccessibilityConfiguration::kInaccessible));

  DiscardSystemPagesInternal(address, length);
  return true;
}

void RecommitSystemPagesInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility,
    PageAccessibilityDisposition accessibility_disposition) {
  // On Fuchsia systems, the caller needs to simply read the memory to recommit
  // it. However, if decommit changed the permissions, recommit has to change
  // them back.
  if (accessibility_disposition ==
      PageAccessibilityDisposition::kRequireUpdate) {
    SetSystemPagesAccess(address, length, accessibility);
  }
}

bool TryRecommitSystemPagesInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility,
    PageAccessibilityDisposition accessibility_disposition) {
  // On Fuchsia systems, the caller needs to simply read the memory to recommit
  // it. However, if decommit changed the permissions, recommit has to change
  // them back.
  if (accessibility_disposition ==
      PageAccessibilityDisposition::kRequireUpdate) {
    return TrySetSystemPagesAccess(address, length, accessibility);
  }
  return true;
}

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_FUCHSIA_H_
