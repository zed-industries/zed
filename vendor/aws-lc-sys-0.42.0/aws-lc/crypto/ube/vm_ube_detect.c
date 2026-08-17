// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/crypto.h>

#include "vm_ube_detect.h"

#if defined(OPENSSL_LINUX)
#include <fcntl.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

#include "../internal.h"

// VM UBE state
#define VM_UBE_STATE_FAILED_INITIALISE 0x00
#define VM_UBE_STATE_SUCCESS_INITIALISE 0x01
#define VM_UBE_STATE_NOT_SUPPORTED 0x02

static CRYPTO_once_t vm_ube_init = CRYPTO_ONCE_INIT;
static int vm_ube_state = 0;

// SysGenID generation number pointer
static volatile uint32_t *sgn_addr = NULL;

static void do_sysgenid_init(void) {
  vm_ube_state = VM_UBE_STATE_NOT_SUPPORTED;
  sgn_addr = NULL;

  struct stat buff;
  if (stat(CRYPTO_get_sysgenid_path(), &buff) != 0) {
    return;
  }
  
  vm_ube_state = VM_UBE_STATE_FAILED_INITIALISE;

  int fd_sgn = open(CRYPTO_get_sysgenid_path(), O_RDONLY);
  if (fd_sgn == -1) {
    return;
  }

  void *addr = mmap(NULL, sizeof(uint32_t), PROT_READ, MAP_SHARED, fd_sgn, 0);

  // Can close file descriptor now per
  // https://man7.org/linux/man-pages/man2/mmap.2.html: "After the mmap() call
  // has returned, the file descriptor, fd, can be closed immediately without
  // invalidating the mapping.". We have initialised vm_ube_state without errors
  // and this function is only executed once. Therefore, try to close file
  // descriptor but don't error if it fails. */
  close(fd_sgn);

  if (addr == MAP_FAILED) {
    return;
  }

  // sgn_addr will now point at the mapped memory and any 4-byte read from
  // this pointer will correspond to the sgn managed by the VMM.
  sgn_addr = addr;
  vm_ube_state = VM_UBE_STATE_SUCCESS_INITIALISE;
}

static uint32_t vm_ube_read_sysgenid_gn(void) {
  if (vm_ube_state == VM_UBE_STATE_SUCCESS_INITIALISE) {
    return *sgn_addr;
  }

  return 0;
}

int CRYPTO_get_vm_ube_generation(uint32_t *vm_ube_generation_number) {
  CRYPTO_once(&vm_ube_init, do_sysgenid_init);

  switch (vm_ube_state) {
    case VM_UBE_STATE_NOT_SUPPORTED:
      *vm_ube_generation_number = 0;
      return 1;
    case VM_UBE_STATE_SUCCESS_INITIALISE:
      *vm_ube_generation_number = vm_ube_read_sysgenid_gn();
      return 1;
    case VM_UBE_STATE_FAILED_INITIALISE:
      *vm_ube_generation_number = 0;
      return 0;
    default:
      // No other state should be possible.
      abort();
  }
}

int CRYPTO_get_vm_ube_active(void) {
  CRYPTO_once(&vm_ube_init, do_sysgenid_init);

  if (vm_ube_state == VM_UBE_STATE_SUCCESS_INITIALISE) {
    return 1;
  }

  return 0;
}

int CRYPTO_get_vm_ube_supported(void) {
  CRYPTO_once(&vm_ube_init, do_sysgenid_init);

  if (vm_ube_state == VM_UBE_STATE_NOT_SUPPORTED) {
    return 0;
  }

  return 1;
}

#else  // !defined(OPENSSL_LINUX)

int CRYPTO_get_vm_ube_generation(uint32_t *vm_ube_generation_number) {
  *vm_ube_generation_number = 0;
  return 1;
}

int CRYPTO_get_vm_ube_active(void) { return 0; }

int CRYPTO_get_vm_ube_supported(void) { return 0; }

#endif  // defined(OPENSSL_LINUX)

const char* CRYPTO_get_sysgenid_path(void) {
  return AWSLC_SYSGENID_PATH;
}

#if defined(OPENSSL_LINUX) && defined(AWSLC_VM_UBE_TESTING)
int HAZMAT_init_sysgenid_file(void) {
  int fd_sgn = open(CRYPTO_get_sysgenid_path(), O_CREAT | O_RDWR,
                    S_IRWXU | S_IRGRP | S_IROTH);
  if (fd_sgn == -1) {
    return 0;
  }
  // If the file is empty, populate it. Otherwise, no change.
  if (0 == lseek(fd_sgn, 0, SEEK_END)) {
    if (0 != lseek(fd_sgn, 0, SEEK_SET)) {
      close(fd_sgn);
      return 0;
    }
    uint32_t value = 0;
    if (0 >= write(fd_sgn, &value, sizeof(uint32_t))) {
      close(fd_sgn);
      return 0;
    }

    if (0 != fsync(fd_sgn)) {
      close(fd_sgn);
      return 0;
    }
  }

  close(fd_sgn);

  return 1;
}
#endif
