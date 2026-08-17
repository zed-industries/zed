// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_FUZZER_FUZZER_H_
#define IPCZ_FUZZER_FUZZER_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <vector>

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz::fuzzer {

// Base class for driver objects implemented by the fuzzer.
class DriverObject : public RefCounted<DriverObject> {
 public:
  enum class Type {
    kTransport,
    kMemory,
    kUnknown,
  };

  virtual Type GetType() const = 0;
  virtual IpczResult Close() { return IPCZ_RESULT_OK; }

  template <typename T>
  Ref<T> As() {
    if (GetType() != T::kType) {
      return nullptr;
    }
    return WrapRefCounted(static_cast<T*>(this));
  }

 protected:
  friend class RefCounted<DriverObject>;

  virtual ~DriverObject() = default;
};

// Base class to help driver objects support safe downcasting where needed.
template <DriverObject::Type object_type = DriverObject::Type::kUnknown>
class DriverObjectImpl : public DriverObject {
 public:
  static constexpr Type kType = object_type;

  Type GetType() const override { return kType; }
};

// Used by ipcz fuzzer tests to drive ipcz communication and inject fuzz data.
// Only one object of this type may exist at a time.
class Fuzzer {
 public:
  // Specifies when and where fuzz data is to be injected. Generally taken from
  // the first 2 bytes of available fuzz data.
  struct FuzzConfig {
    // The transmission count at which to inject fuzz data. Transmissions are
    // counted globally across all transports. A fuzzer test case should try to
    // achieve good coverage with as few transmissions as possible to minimize
    // the number of bits needed for this parameter.
    uint8_t transmission_to_fuzz : 7;

    // Whether to target link memory or inline transmission data. When false,
    // some portion of the identified transmission's data (starting at
    // `target_offset` below) is replaced with the fuzz data. Otherwise the
    // selected transmission is carried out normally, but only after injecting
    // fuzz data at `target_offset` within every allocated driver memory object.
    bool fuzz_link_memory : 1;

    // The offset (in 4-byte words) at which to write the remaining (possibly
    // truncated) fuzz data into the selected transmission's payload or each
    // memory object. All interesting code paths can be covered by fuzzing at
    // lower offsets in either case, so this doesn't need many bits.
    uint8_t target_offset;
  };
  static_assert(sizeof(FuzzConfig) == 2);

  // Constructs a fuzzer with no fuzz data. This does not perform any actual
  // fuzzing but could be used to execute testcases unfuzzed e.g. for future
  // seed corpus generation.
  Fuzzer();

  // Constructs a fuzzer using `config` to inject `fuzz_data`. If `fuzz_data` is
  // empty, `config` is effectively ignored and no fuzz data is ever injected.
  Fuzzer(const FuzzConfig& config, absl::Span<const unsigned char> fuzz_data);
  ~Fuzzer();

  static Fuzzer& Get();

  // Flushes all queued transport activity repeatedly until no more activity is
  // queued. This should generally be done before attempting to use or observe
  // side-effects of previous operations (e.g. before attempting to read
  // messages from a portal.)
  //
  // Leaving the fuzzer responsible for manual flushing allows for the
  // simplicity of deterministic, synchronous ipcz behavior without the
  // complexity of transport activity reentrancy.
  void FlushTransports();

  // Registers `object` and returns a new handle for it. Unlike a typical
  // production ipcz driver, our handles are simple sequential integers indexing
  // an object map. This makes them trivial to serialize and deserialize while
  // eliminating the risk of leakage when fuzzing interfers with normal
  // communication.
  IpczDriverHandle RegisterDriverObject(Ref<DriverObject> object);

  // Dispatches a transmission possibly injecting fuzz data either inline or
  // somewhere in link memory.
  void DispatchTransportActivity(IpczHandle listener,
                                 IpczTransportActivityHandler handler,
                                 absl::Span<const uint8_t> data,
                                 absl::Span<const IpczDriverHandle> handles);

  // ipcz driver API implementation.
  IpczResult Close(IpczDriverHandle handle);
  IpczResult Serialize(IpczDriverHandle handle,
                       IpczDriverHandle transport,
                       volatile void* data,
                       size_t* num_bytes,
                       IpczDriverHandle* handles,
                       size_t* num_handles);
  IpczResult Deserialize(const volatile void* data,
                         size_t num_bytes,
                         const IpczDriverHandle* handles,
                         size_t num_handles,
                         IpczDriverHandle transport,
                         IpczDriverHandle* handle);
  IpczResult CreateTransports(IpczDriverHandle* transport0,
                              IpczDriverHandle* transport1,
                              bool may_broker = false);
  IpczResult ActivateTransport(IpczDriverHandle transport,
                               IpczHandle listener,
                               IpczTransportActivityHandler handler);
  IpczResult DeactivateTransport(IpczDriverHandle transport);
  IpczResult Transmit(IpczDriverHandle transport,
                      const void* data,
                      size_t num_bytes,
                      const IpczDriverHandle* handles,
                      size_t num_handles);
  IpczResult AllocateSharedMemory(size_t num_bytes, IpczDriverHandle* memory);
  IpczResult GetSharedMemoryInfo(IpczDriverHandle memory,
                                 IpczSharedMemoryInfo* info);
  IpczResult DuplicateSharedMemory(IpczDriverHandle memory,
                                   IpczDriverHandle* duplicate);
  IpczResult MapSharedMemory(IpczDriverHandle memory,
                             volatile void** address,
                             IpczDriverHandle* mapping);
  IpczResult GenerateRandomBytes(absl::Span<uint8_t> bytes);

 private:
  class Memory;
  class TransportBackend;
  class Transport;

  void InjectFuzzDataIntoMemory();
  Ref<DriverObject> GetDriverObject(IpczDriverHandle handle) const;

  template <typename T>
  Ref<T> GetDriverObjectAs(IpczDriverHandle handle) const {
    static_assert(T::kType != DriverObject::Type::kUnknown);
    Ref<DriverObject> object = GetDriverObject(handle);
    if (!object) {
      return nullptr;
    }
    return object->As<T>();
  }

  const FuzzConfig config_ = {};
  const absl::Span<const unsigned char> fuzz_data_;

  std::vector<Ref<Memory>> memory_objects_;
  std::vector<Ref<TransportBackend>> transport_backends_;

  // Table of all driver objects minted by this driver.
  std::map<IpczDriverHandle, Ref<DriverObject>> driver_objects_;
  IpczDriverHandle next_driver_handle_ = 1;
  uint8_t next_random_byte_ = 0;

  // Count of all transmissions that have been dispatched through this driver.
  int current_transmission_ = 0;
};

}  // namespace ipcz::fuzzer

#endif  // IPCZ_FUZZER_FUZZER_H_
