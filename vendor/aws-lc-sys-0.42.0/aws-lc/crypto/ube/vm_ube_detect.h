// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef HEADER_VM_UBE_DETECT
#define HEADER_VM_UBE_DETECT

#include <openssl/base.h>

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(AWSLC_SYSGENID_PATH)
  #define AWSLC_SYSGENID_PATH "/dev/sysgenid"
#endif

// VM UBE-type uniqueness breaking event (ube detection).
//
// CRYPTO_get_vm_ube_generation provides the VM UBE generation number for
// the current process. The VM UBE generation number is a non-zero,
// strictly-monotonic counter with the property that, if queried in an address
// space and then again in a subsequently resumed snapshot/VM, the resumed
// address space will observe a greater value.
//
// We use SysGenID to detect resumed snapshot/VM events. See
// https://lkml.org/lkml/2021/3/8/677 for details about how SysGenID works.
// We make light use of the SysGenId capabilities and only use the following
// supported functions on the device: |open| and |mmap|.
//
// |CRYPTO_get_vm_ube_generation| returns 0 only when the filesystem
// presents SysGenID interface (default is `/dev/sysgenid`) but we are
// is unable to initialize its use. Otherwise, it returns 1.
OPENSSL_EXPORT int CRYPTO_get_vm_ube_generation(
                                          uint32_t *vm_ube_generation_number);

// CRYPTO_get_vm_ube_active returns 1 if the file system presents the SysGenID
// interface and the library has successfully initialized its use. Otherwise,
// it returns 0.
OPENSSL_EXPORT int CRYPTO_get_vm_ube_active(void);

// CRYPTO_get_vm_ube_supported returns 1 if the file system presents the
// SysGenID interface. Otherwise, it returns 0.
OPENSSL_EXPORT int CRYPTO_get_vm_ube_supported(void);

// CRYPTO_get_sysgenid_path returns the path used for the SysGenId interface.
OPENSSL_EXPORT const char *CRYPTO_get_sysgenid_path(void);

#if defined(OPENSSL_LINUX) && defined(AWSLC_VM_UBE_TESTING)
// HAZMAT_init_sysgenid_file should only be used for testing. It creates and
// initializes the sysgenid path indicated by AWSLC_SYSGENID_PATH.
// On success, it returns 1. Otherwise, returns 0.
OPENSSL_EXPORT int HAZMAT_init_sysgenid_file(void);
#endif

#ifdef __cplusplus
}
#endif

#endif /* HEADER_VM_UBE_DETECT */
