//
//
// Copyright 2016 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_UTIL_MPSCQ_H
#define GRPC_SRC_CORE_UTIL_MPSCQ_H

#include <grpc/support/port_platform.h>

#include <atomic>

#include "absl/log/check.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Multiple-producer single-consumer lock free queue, based upon the
// implementation from Dmitry Vyukov here:
// http://www.1024cores.net/home/lock-free-algorithms/queues/intrusive-mpsc-node-based-queue
class MultiProducerSingleConsumerQueue {
 public:
  // List node.  Application node types can inherit from this.
  struct Node {
    std::atomic<Node*> next{nullptr};
  };

  MultiProducerSingleConsumerQueue() : head_{&stub_}, tail_(&stub_) {}
  ~MultiProducerSingleConsumerQueue() {
    CHECK(head_.load(std::memory_order_relaxed) == &stub_);
    CHECK(tail_ == &stub_);
  }

  // Push a node
  // Thread safe - can be called from multiple threads concurrently
  // Returns true if this was possibly the first node (may return true
  // sporadically, will not return false sporadically)
  bool Push(Node* node);
  // Pop a node (returns NULL if no node is ready - which doesn't indicate that
  // the queue is empty!!)
  // Thread compatible - can only be called from one thread at a time
  Node* Pop();
  // Pop a node; sets *empty to true if the queue is empty, or false if it is
  // not.
  Node* PopAndCheckEnd(bool* empty);

 private:
  // make sure head & tail don't share a cacheline
  union {
    char padding_[GPR_CACHELINE_SIZE];
    std::atomic<Node*> head_{nullptr};
  };
  Node* tail_;
  Node stub_;
};

// An mpscq with a lock: it's safe to pop from multiple threads, but doing
// only one thread will succeed concurrently.
class LockedMultiProducerSingleConsumerQueue {
 public:
  typedef MultiProducerSingleConsumerQueue::Node Node;

  // Push a node
  // Thread safe - can be called from multiple threads concurrently
  // Returns true if this was possibly the first node (may return true
  // sporadically, will not return false sporadically)
  bool Push(Node* node);

  // Pop a node (returns NULL if no node is ready - which doesn't indicate that
  // the queue is empty!!)
  // Thread safe - can be called from multiple threads concurrently
  Node* TryPop();

  // Pop a node.  Returns NULL only if the queue was empty at some point after
  // calling this function
  Node* Pop();

 private:
  MultiProducerSingleConsumerQueue queue_;
  Mutex mu_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_MPSCQ_H
