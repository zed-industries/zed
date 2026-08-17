// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_HUFFER_H_
#define SRC_INCLUDE_PUFFIN_HUFFER_H_

#include <cstddef>
#include <memory>

#include "puffin/common.h"

namespace puffin {

class BitWriterInterface;
class PuffReaderInterface;
class HuffmanTable;

class Huffer {
 public:
  Huffer();
  ~Huffer();

  // Creates a deflate buffer from a puffed buffer. It is the reverse of
  // |PuffDeflate|.
  bool HuffDeflate(PuffReaderInterface* pr, BitWriterInterface* bw) const;

 private:
  std::unique_ptr<HuffmanTable> dyn_ht_;
  std::unique_ptr<HuffmanTable> fix_ht_;

  DISALLOW_COPY_AND_ASSIGN(Huffer);
};

}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_HUFFER_H_
