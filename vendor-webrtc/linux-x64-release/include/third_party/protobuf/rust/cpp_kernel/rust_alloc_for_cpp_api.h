#ifndef GOOGLE_PROTOBUF_RUST_CPP_KERNEL_RUST_ALLOC_FOR_CPP_API_H__
#define GOOGLE_PROTOBUF_RUST_CPP_KERNEL_RUST_ALLOC_FOR_CPP_API_H__

#include <cstddef>

// Allocates memory using the current Rust global allocator.
//
// This function is defined in `rust_alloc_for_cpp_api.rs`.
extern "C" void* proto2_rust_alloc(size_t size, size_t align);

#endif  // GOOGLE_PROTOBUF_RUST_CPP_KERNEL_RUST_ALLOC_FOR_CPP_API_H__
