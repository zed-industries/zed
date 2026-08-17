#ifndef LLVM_LIBC_BENCHMARKS_LIBC_FUNCTION_PROTOTYPES_H
#define LLVM_LIBC_BENCHMARKS_LIBC_FUNCTION_PROTOTYPES_H

#include "llvm/ADT/StringRef.h"

namespace llvm {
namespace libc_benchmarks {

/// Memory function prototype and configuration.
using MemcpyFunction = void *(*)(void *__restrict, const void *__restrict,
                                 size_t);
struct MemcpyConfiguration {
  MemcpyFunction Function;
  llvm::StringRef Name;
};

using MemmoveFunction = void *(*)(void *, const void *, size_t);
struct MemmoveConfiguration {
  MemmoveFunction Function;
  llvm::StringRef Name;
};

using MemsetFunction = void *(*)(void *, int, size_t);
struct MemsetConfiguration {
  MemsetFunction Function;
  llvm::StringRef Name;
};

using BzeroFunction = void (*)(void *, size_t);
struct BzeroConfiguration {
  BzeroFunction Function;
  llvm::StringRef Name;
};

using MemcmpOrBcmpFunction = int (*)(const void *, const void *, size_t);
struct MemcmpOrBcmpConfiguration {
  MemcmpOrBcmpFunction Function;
  llvm::StringRef Name;
};

} // namespace libc_benchmarks
} // namespace llvm

#endif /* LLVM_LIBC_BENCHMARKS_LIBC_FUNCTION_PROTOTYPES_H */
