/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PORTAL_PIPEWIRE_UTILS_H_
#define MODULES_PORTAL_PIPEWIRE_UTILS_H_

#include <errno.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

// static
struct dma_buf_sync {
  uint64_t flags;
};
#define DMA_BUF_SYNC_READ (1 << 0)
#define DMA_BUF_SYNC_START (0 << 2)
#define DMA_BUF_SYNC_END (1 << 2)
#define DMA_BUF_BASE 'b'
#define DMA_BUF_IOCTL_SYNC _IOW(DMA_BUF_BASE, 0, struct dma_buf_sync)

struct pw_thread_loop;

namespace webrtc {

constexpr int kInvalidPipeWireFd = -1;

// Prepare PipeWire so that it is ready to be used. If it needs to be dlopen'd
// this will do so. Note that this does not guarantee a PipeWire server is
// running nor does it establish a connection to one.
bool InitializePipeWire();

// Locks pw_thread_loop in the current scope
class PipeWireThreadLoopLock {
 public:
  explicit PipeWireThreadLoopLock(pw_thread_loop* loop);
  ~PipeWireThreadLoopLock();

 private:
  pw_thread_loop* const loop_;
};

// We should synchronize DMA Buffer object access from CPU to avoid potential
// cache incoherency and data loss.
// See
// https://01.org/linuxgraphics/gfx-docs/drm/driver-api/dma-buf.html#cpu-access-to-dma-buffer-objects
static bool SyncDmaBuf(int fd, uint64_t start_or_end) {
  struct dma_buf_sync sync = {0};

  sync.flags = start_or_end | DMA_BUF_SYNC_READ;

  while (true) {
    int ret;
    ret = ioctl(fd, DMA_BUF_IOCTL_SYNC, &sync);
    if (ret == -1 && errno == EINTR) {
      continue;
    } else if (ret == -1) {
      return false;
    } else {
      break;
    }
  }

  return true;
}

class ScopedBuf {
 public:
  ScopedBuf() {}
  ScopedBuf(uint8_t* map, int map_size, int fd, bool is_dma_buf = false)
      : map_(map), map_size_(map_size), fd_(fd), is_dma_buf_(is_dma_buf) {}
  ~ScopedBuf() {
    if (map_ != MAP_FAILED) {
      if (is_dma_buf_) {
        SyncDmaBuf(fd_, DMA_BUF_SYNC_END);
      }
      munmap(map_, map_size_);
    }
  }

  explicit operator bool() { return map_ != MAP_FAILED; }

  void initialize(uint8_t* map, int map_size, int fd, bool is_dma_buf = false) {
    map_ = map;
    map_size_ = map_size;
    is_dma_buf_ = is_dma_buf;
    fd_ = fd;

    if (is_dma_buf_) {
      SyncDmaBuf(fd_, DMA_BUF_SYNC_START);
    }
  }

  uint8_t* get() { return map_; }

 protected:
  uint8_t* map_ = static_cast<uint8_t*>(MAP_FAILED);
  int map_size_;
  int fd_;
  bool is_dma_buf_;
};

}  // namespace webrtc

#endif  // MODULES_PORTAL_PIPEWIRE_UTILS_H_
