//===-- Shared RPC server interface -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SHARED_RPC_SERVER_H
#define LLVM_LIBC_SHARED_RPC_SERVER_H

#include "src/__support/RPC/rpc_server.h"

namespace LIBC_NAMESPACE_DECL {
namespace shared {

using LIBC_NAMESPACE::rpc::handle_libc_opcodes;

} // namespace shared
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SHARED_RPC_SERVER_H
