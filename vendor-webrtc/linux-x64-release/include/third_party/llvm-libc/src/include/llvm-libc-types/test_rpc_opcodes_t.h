//===-- Definition of RPC opcodes used for internal tests -----------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TYPES_TEST_RPC_OPCODES_T_H
#define LLVM_LIBC_TYPES_TEST_RPC_OPCODES_T_H

// We consider the first 32768 opcodes as reserved for libc purposes. We allow
// extensions to use any other number without conflicting with anything else.
typedef enum : unsigned short {
  RPC_TEST_NOOP = 1 << 15,
  RPC_TEST_INCREMENT,
  RPC_TEST_INTERFACE,
  RPC_TEST_STREAM,
} rpc_test_opcode_t;

#endif // LLVM_LIBC_TYPES_TEST_RPC_OPCODES_T_H
