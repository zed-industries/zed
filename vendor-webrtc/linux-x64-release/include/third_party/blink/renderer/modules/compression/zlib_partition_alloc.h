#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_ZLIB_PARTITION_ALLOC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_ZLIB_PARTITION_ALLOC_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/zlib/zlib.h"

namespace blink {

// Use PartitionAlloc for zlib allocations.
class ZlibPartitionAlloc {
  STATIC_ONLY(ZlibPartitionAlloc);

 public:
  static void Configure(z_stream* s);

 private:
  static void* Alloc(void*, uint32_t items, uint32_t size);

  static void Free(void*, void*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_ZLIB_PARTITION_ALLOC_H_
