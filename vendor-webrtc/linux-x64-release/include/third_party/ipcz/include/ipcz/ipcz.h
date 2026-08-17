// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_INCLUDE_IPCZ_IPCZ_H_
#define IPCZ_INCLUDE_IPCZ_IPCZ_H_

// ipcz is a cross-platform C library for interprocess communication (IPC) which
// supports efficient routing and data transfer over a large number of
// dynamically relocatable messaging endpoints.
//
// ipcz operates in terms of a small handful of abstractions encapsulated in
// this header: nodes, portals, parcels, drivers, boxes, and traps.
//
// NOTE: This header is intended to compile under C++11 or newer, and C99 or
// newer. The ABI defined here can be considered stable.
//
// Glossary
// --------
// *Nodes* are used by ipcz to model isolated units of an application. A typical
// application will create one ipcz node within each OS process it controls.
//
// *Portals* are messaging endpoints which belong to a specific node. Portals
// are created in entangled pairs: whatever goes into one portal comes out the
// other (its "peer"). Pairs may be created local to a single node, or they may
// be created to span two nodes. Portals may also be transferred freely through
// other portals.
//
// *Parcels* are the unit of communication between portals. Parcels can contain
// arbitrary application data as well as ipcz handles. Handles within a parcel
// are used to transfer objects (namely other portals, or driver-defined
// objects) from one portal to another, potentially on a different node.
//
// *Traps* provide a flexible mechanism to observe interesting portal state
// changes such as new parcels arriving or a portal's peer being closed.
//
// *Drivers* are provided by applications to implement platform-specific IPC
// details. They may also define new types of objects to be transmitted in
// parcels via boxes.
//
// *Boxes* are opaque objects used to wrap driver- or application-defined
// objects for seamless transmission across portals. Applications use the Box()
// and Unbox() APIs to go between concrete objects and transferrable box
// handles, and ipcz delegates to the driver or application to serialize boxed
// objects as needed for transmission.
//
// Overview
// --------
// To use ipcz effectively, an application must create multiple nodes to be
// interconnected. One node must be designated as the "broker" by the
// application (see CreateNode() flags). The broker is used by ipcz to
// coordinate certain types of internal transactions which demand a heightened
// level of trust and capability, so a broker node should always live in a more
// trustworthy process. For example in Chrome, the browser process is
// designated as the broker.
//
// In order for a node to communicate with other nodes in the system, the
// application must explicitly connect it to ONE other node using the
// ConnectNode() API. Once this is done, ipcz can automatically connect the node
// to additional other nodes as needed for efficient portal operation.
//
// In the example below, assume node A is designated as the broker. Nodes A and
// B have been connected directly by ConnectNode() calls in the application.
// Nodes A and C have been similarly connected:
//
//                    ┌───────┐
//     ConnectNode()  │       │  ConnectNode()
//        ┌──────────>O   A   O<───────────┐
//        │           │       │            │
//        │           └───────┘            │
//        │                                │
//        v ConnectNode()                  v ConnectNode()
//    ┌───O───┐                        ┌───O───┐
//    │       │                        │       │
//    │   B   │                        │   C   │
//    │       │                        │       │
//    └───────┘                        └───────┘
//
// ConnectNode() establishes initial portal pairs to link the two nodes
// together, illustrated above as "O"s. Once ConnectNode() returns, the
// application may immediately begin transmitting parcels through these portals.
//
// Now suppose node B creates a new local pair of portals (using OpenPortals())
// and sends one of those new portals in a parcel through its
// already-established portal to node A. The sent portal is effectively
// transferred to node A, and because its entangled peer still lives on node B
// there are now TWO portal pairs between nodes A and B:
//
//                    ┌───────┐
//                    │       │
//        ┌──────────>O   A   O<───────────┐
//        │ ┌────────>O       │            │
//        │ │         └───────┘            │
//        │ │                              │
//        v v                              v
//    ┌───O─O─┐                        ┌───O───┐
//    │       │                        │       │
//    │   B   │                        │   C   │
//    │       │                        │       │
//    └───────┘                        └───────┘
//
// Finally, suppose now the application takes this new portal on node A and
// sends it further along, through node A's already-established portal to node
// C. Because the transferred portal's peer still lives on node B, there is now
// a portal pair spanning nodes B and C:
//
//                    ┌───────┐
//                    │       │
//        ┌──────────>O   A   O<───────────┐
//        │           │       │            │
//        │           └───────┘            │
//        │                                │
//        v                                v
//    ┌───O───┐                        ┌───O───┐
//    │       │                        │       │
//    │   B   O────────────────────────O   C   │
//    │       │                        │       │
//    └───────┘                        └───────┘
//
// These two nodes were never explicitly connected by the application, but ipcz
// ensures that the portals will operate as expected. Behind the scenes, ipcz
// achieves this by establishing a direct, secure, and efficient communication
// channel between nodes B and C.

#include <stddef.h>
#include <stdint.h>

#define IPCZ_NO_FLAGS ((uint32_t)0)

// Helper to clarify flag definitions.
#define IPCZ_FLAG_BIT(bit) ((uint32_t)(1u << bit))

// Opaque handle to an ipcz object.
typedef uintptr_t IpczHandle;

// An IpczHandle value which is always invalid. Note that arbitrary non-zero
// values are not necessarily valid either, but zero is never valid.
#define IPCZ_INVALID_HANDLE ((IpczHandle)0)

// Generic result code for all ipcz operations. See IPCZ_RESULT_* values below.
typedef int IpczResult;

// Specific meaning of each value depends on context, but IPCZ_RESULT_OK always
// indicates success. These values are derived from common status code
// definitions across Google software.
#define IPCZ_RESULT_OK ((IpczResult)0)
#define IPCZ_RESULT_CANCELLED ((IpczResult)1)
#define IPCZ_RESULT_UNKNOWN ((IpczResult)2)
#define IPCZ_RESULT_INVALID_ARGUMENT ((IpczResult)3)
#define IPCZ_RESULT_DEADLINE_EXCEEDED ((IpczResult)4)
#define IPCZ_RESULT_NOT_FOUND ((IpczResult)5)
#define IPCZ_RESULT_ALREADY_EXISTS ((IpczResult)6)
#define IPCZ_RESULT_PERMISSION_DENIED ((IpczResult)7)
#define IPCZ_RESULT_RESOURCE_EXHAUSTED ((IpczResult)8)
#define IPCZ_RESULT_FAILED_PRECONDITION ((IpczResult)9)
#define IPCZ_RESULT_ABORTED ((IpczResult)10)
#define IPCZ_RESULT_OUT_OF_RANGE ((IpczResult)11)
#define IPCZ_RESULT_UNIMPLEMENTED ((IpczResult)12)
#define IPCZ_RESULT_INTERNAL ((IpczResult)13)
#define IPCZ_RESULT_UNAVAILABLE ((IpczResult)14)
#define IPCZ_RESULT_DATA_LOSS ((IpczResult)15)

// Helper to specify explicit struct alignment across C and C++ compilers.
#if defined(__cplusplus)
#define IPCZ_ALIGN(alignment) alignas(alignment)
#elif defined(__GNUC__)
#define IPCZ_ALIGN(alignment) __attribute__((aligned(alignment)))
#elif defined(_MSC_VER)
#define IPCZ_ALIGN(alignment) __declspec(align(alignment))
#else
#error "IPCZ_ALIGN() is not defined for your compiler."
#endif

// Helper to generate the smallest constant value which is aligned with
// `alignment` and at least as large as `value`.
#define IPCZ_ALIGNED(value, alignment) \
  ((((value) + ((alignment)-1)) / (alignment)) * (alignment))

// Helper used to explicitly specify calling convention or other
// compiler-specific annotations for each API function.
#if defined(IPCZ_API_OVERRIDE)
#define IPCZ_API IPCZ_API_OVERRIDE
#elif defined(_WIN32)
#define IPCZ_API __cdecl
#else
#define IPCZ_API
#endif

// An opaque handle value created by an IpczDriver implementation. ipcz uses
// such handles to provide relevant context when calling back into the driver.
typedef uintptr_t IpczDriverHandle;

#define IPCZ_INVALID_DRIVER_HANDLE ((IpczDriverHandle)0)

// Flags which may be passed by a driver to an IpczTransportActivityHandler when
// notifying ipcz about transport activity.
typedef uint32_t IpczTransportActivityFlags;

// Indicates that the driver encountered an unrecoverable error while using the
// transport. This generally results in ipcz deactivating the transport via the
// driver's DeactivateTransport().
#define IPCZ_TRANSPORT_ACTIVITY_ERROR IPCZ_FLAG_BIT(0)

// Informs ipcz that the driver will no longer invoke the activity handler for
// a given listener, as the driver is no longer listening for activity on the
// corresponding transport.
#define IPCZ_TRANSPORT_ACTIVITY_DEACTIVATED IPCZ_FLAG_BIT(1)

#if defined(__cplusplus)
extern "C" {
#endif

struct IpczTransportActivityOptions {
  size_t size;                // Size of this struct, for extensibility.
  IpczDriverHandle envelope;  // Optional driver envelope.
};

// Notifies ipcz of activity on a transport. `listener` must be a handle to an
// active transport's listener, as provided to the driver by ipcz via
// ActivateTransport().
//
// Drivers use this function to feed incoming data and driver handles from a
// transport to ipcz, or to inform ipcz of any unrecoverable dysfunction of the
// transport. In the latter case, drivers specify IPCZ_TRANSPORT_ACTIVITY_ERROR
// in `flags` to instigate deactivation and disposal of the transport by ipcz.
//
// If IPCZ_TRANSPORT_ACTIVITY_DEACTIVATED is set in `flags`, this must be the
// last call made by the driver for the given `listener`. See also
// DeactivateTransport() defined on IpczDriver below.
//
// `options` is currently unused and must be null.
//
// IMPORTANT: Drivers must ensure that all calls to this handler for the same
// `listener` are mutually exclusive. Overlapping calls are unsafe and will
// result in undefined behavior.
typedef IpczResult(IPCZ_API* IpczTransportActivityHandler)(
    IpczHandle listener,                                  // in
    const void* data,                                     // in
    size_t num_bytes,                                     // in
    const IpczDriverHandle* driver_handles,               // in
    size_t num_driver_handles,                            // in
    IpczTransportActivityFlags flags,                     // in
    const struct IpczTransportActivityOptions* options);  // in

// Structure to be filled in by a driver's GetSharedMemoryInfo().
struct IPCZ_ALIGN(8) IpczSharedMemoryInfo {
  // The exact size of this structure in bytes. Set by ipcz before passing the
  // structure to the driver.
  size_t size;

  // The size of the shared memory region in bytes.
  size_t region_num_bytes;
};

// IpczDriver
// ==========
//
// IpczDriver is a function table to be populated by the application and
// provided to ipcz when creating a new node. The driver implements concrete
// I/O operations to facilitate communication between nodes, giving embedding
// systems full control over choice of OS-specific transport mechanisms and I/O
// scheduling decisions.
struct IPCZ_ALIGN(8) IpczDriver {
  // The exact size of this structure in bytes. Must be set accurately by the
  // application before passing this structure to any ipcz API functions.
  size_t size;

  // Close()
  // =======
  //
  // Called by ipcz to request that the driver release the object identified by
  // `handle`.
  IpczResult(IPCZ_API* Close)(IpczDriverHandle handle,  // in
                              uint32_t flags,           // in
                              const void* options);     // in

  // Serialize()
  // ===========
  //
  // Serializes a driver object identified by `handle` into a collection of
  // bytes and readily transmissible driver objects, for eventual transmission
  // over `transport`. At a minimum this must support serialization of transport
  // and memory objects created by the same driver. Any other driver objects
  // intended for applications to box and transmit through portals must also be
  // serializable here.
  //
  // If the object identified by `handle` is invalid or unserializable, the
  // driver must ignore all other arguments (including `transport`) and return
  // IPCZ_RESULT_INVALID_ARGUMENT.
  //
  // If the object can be serialized but success may depend on the value of
  // `transport`, and `transport` is IPCZ_INVALID_DRIVER_HANDLE, the driver must
  // return IPCZ_RESULT_ABORTED. ipcz may invoke Serialize() in this way to
  // query whether a specific object can be serialized at all, even when it
  // doesn't have a specific transport in mind.
  //
  // If the object can be serialized or transmitted as-is and `transport` is
  // valid, but the serialized outputs would not be transmissible over
  // `transport` specifically, the driver must ignore all other arguments and
  // return IPCZ_RESULT_PERMISSION_DENIED. This implies that neither end of
  // `transport` is sufficiently privileged or otherwise able to transfer the
  // object directly, and in this case ipcz may instead attempt to relay the
  // object through a more capable broker node.
  //
  // For all other outcomes, the object identified by `handle` is considered to
  // be serializable.
  //
  // `num_bytes` and `num_handles` on input point to the capacities of the
  // respective output buffers provided by `data` and `handles`. If either
  // capacity pointer is null, a capacity of zero is implied; and if either
  // input capacity is zero, the corresponding output buffer may be null.
  //
  // Except in the failure modes described above, the driver must update any
  // non-null capacity input to reflect the exact capacity required to serialize
  // the object. For example if `num_bytes` is non-null and the object
  // serializes to 8 bytes of data, `*num_bytes` must be set to 8 upon return.
  //
  // If serializing the object requires more data or handle capacity than ipcz
  // provided, the driver must return IPCZ_RESULT_RESUORCE_EXHAUSTED after
  // updating the capacity values as described above. In this case the driver
  // must not touch `data` or `handles`.
  //
  // Finally, if the input capacities were both sufficient, the driver must fill
  // `data` and `handles` with a serialized representation of the object and
  // return IPCZ_RESULT_OK. In this case ipcz relinquishes `handle` and will no
  // longer refer to it.
  IpczResult(IPCZ_API* Serialize)(IpczDriverHandle handle,     // in
                                  IpczDriverHandle transport,  // in
                                  uint32_t flags,              // in
                                  const void* options,         // in
                                  volatile void* data,         // out
                                  size_t* num_bytes,           // in/out
                                  IpczDriverHandle* handles,   // out
                                  size_t* num_handles);        // in/out

  // Deserialize()
  // =============
  //
  // Deserializes a driver object from a collection of bytes and transmissible
  // driver handles that was originally produced by Serialize() and received by
  // activity on `transport`.
  //
  // Any return value other than IPCZ_RESULT_OK indicates an error and implies
  // that `handle` is unmodified. Otherwise `*handle` must be set to a valid
  // driver handle which identifies the deserialized object upon return.
  IpczResult(IPCZ_API* Deserialize)(
      const volatile void* data,               // in
      size_t num_bytes,                        // in
      const IpczDriverHandle* driver_handles,  // in
      size_t num_driver_handles,               // in
      IpczDriverHandle transport,              // in
      uint32_t flags,                          // in
      const void* options,                     // in
      IpczDriverHandle* handle);               // out

  // CreateTransports()
  // ==================
  //
  // Creates a new pair of entangled bidirectional transports, returning them in
  // `new_transport0` and `new_transport1`.
  //
  // The input `transport0` and `transport1` are provided for context which may
  // be important to the driver: the output transport in `new_transport0` will
  // be sent over `transport0`, while the output transport in `new_transport1`
  // will be sent over `transport1`. That is, the new transport is being created
  // to establish a direct link between the remote node on `transport0` and the
  // remote node on `transport1`.
  //
  // Any return value other than IPCZ_RESULT_OK indicates an error and implies
  // that `new_transport0` and `new_transport1` are unmodified.
  //
  // Returned transports may be used immediately by ipcz for Transmit(), even
  // if the transports are not yet activated.
  IpczResult(IPCZ_API* CreateTransports)(
      IpczDriverHandle transport0,        // in
      IpczDriverHandle transport1,        // in
      uint32_t flags,                     // in
      const void* options,                // in
      IpczDriverHandle* new_transport0,   // out
      IpczDriverHandle* new_transport1);  // out

  // ActivateTransport()
  // ===================
  //
  // Called by ipcz to activate a given `transport`, either as given to ipcz via
  // ConnectNode(), or as returned by the driver from CreateTransports().
  //
  // `listener` is a handle the driver must use when calling `activity_handler`
  // to notify ipcz about any incoming data or state changes on the identified
  // transport.
  //
  // Before this returns, the driver should establish any I/O monitoring or
  // scheduling state necessary to support operation of the endpoint.
  //
  // Any return value other than IPCZ_RESULT_OK indicates an error, and the
  // transport will be closed by ipcz. Otherwise the transport may immediately
  // begin to invoke `activity_handler` and may continue to do so until
  // deactivated via DeactivateTransport().
  //
  // Note that while `activity_handler` may be invoked by the driver from any
  // thread, invocations MUST be mutually exclusive for a given `listener`.
  // Overlapping invocations are unsafe and will result in undefined behavior.
  //
  // The driver may elicit forced deactivation and destruction of an active
  // transport by calling `activity_handler` with the
  // IPCZ_TRANSPORT_ACTIVITY_DEACTIVATED flag. Otherwise ipcz will eventually
  // deactivate `transport` when it's no longer in use by calling
  // DeactivateTransport().
  IpczResult(IPCZ_API* ActivateTransport)(
      IpczDriverHandle transport,                     // in
      IpczHandle listener,                            // in
      IpczTransportActivityHandler activity_handler,  // in
      uint32_t flags,                                 // in
      const void* options);                           // in

  // DeactivateTransport()
  // =====================
  //
  // Called by ipcz to deactivate a transport that is no longer needed.
  //
  // The driver does not need to complete deactivation synchronously, but it
  // must eventually (soon) cease operation of the transport and finalize the
  // deactivation by invoking activity handler one final time with
  // IPCZ_TRANSPORT_ACTIVITY_DEACTIVATED. Failure to do this will result in
  // resource leaks.
  //
  // Note that even after deactivation, ipcz may continue to call into
  // `transport` for other operations (e.g. Serialize() or Transmit()) until
  // it's closed by ipcz with an explicit call to the driver's Close().
  IpczResult(IPCZ_API* DeactivateTransport)(IpczDriverHandle transport,  // in
                                            uint32_t flags,              // in
                                            const void* options);        // in

  // Transmit()
  // ==========
  //
  // Called by ipcz to delegate transmission of data and driver handles over the
  // identified transport endpoint. If the driver cannot fulfill the request,
  // it must return a result other than IPCZ_RESULT_OK, and this will cause the
  // transport's connection to be severed.
  //
  // Note that all handles in `driver_handles` were obtained by ipcz from the
  // driver itself, as returned by a prior call to the driver's own Serialize()
  // function. These handles are therefore expected to be directly transmissible
  // by the driver alongside any data in `data`.
  //
  // The driver is responsible for ensuring that every Transmit() on a transport
  // results in a corresponding activity handler invocation on the remote peer's
  // transport, even if `num_bytes` and `num_driver_handles` are both zero.
  //
  // IMPORTANT: For any sequence of Transmit() calls from the same thread, the
  // corresponding activity handler invocations on the peer transport must
  // occur in the same order.
  IpczResult(IPCZ_API* Transmit)(IpczDriverHandle transport,              // in
                                 const void* data,                        // in
                                 size_t num_bytes,                        // in
                                 const IpczDriverHandle* driver_handles,  // in
                                 size_t num_driver_handles,               // in
                                 uint32_t flags,                          // in
                                 const void* options);                    // in

  // ReportBadTransportActivity()
  // ============================
  //
  // The ipcz Reject() API can be used by an application to reject a specific
  // parcel received from a portal. If the parcel in question came from a
  // remote node, ipcz invokes ReportBadTransportActivity() to notify the driver
  // about the `transport` which received the rejected parcel.
  //
  // `context` is an opaque value passed by the application to the Reject() call
  // which elicited this invocation.
  IpczResult(IPCZ_API* ReportBadTransportActivity)(IpczDriverHandle transport,
                                                   uintptr_t context,
                                                   uint32_t flags,
                                                   const void* options);

  // AllocateSharedMemory()
  // ======================
  //
  // Allocates a shared memory region and returns a driver handle in
  // `driver_memory` which can be used to reference it in other calls to the
  // driver.
  IpczResult(IPCZ_API* AllocateSharedMemory)(
      size_t num_bytes,                  // in
      uint32_t flags,                    // in
      const void* options,               // in
      IpczDriverHandle* driver_memory);  // out

  // GetSharedMemoryInfo()
  // =====================
  //
  // Returns information about the shared memory region identified by
  // `driver_memory`.
  IpczResult(IPCZ_API* GetSharedMemoryInfo)(
      IpczDriverHandle driver_memory,      // in
      uint32_t flags,                      // in
      const void* options,                 // in
      struct IpczSharedMemoryInfo* info);  // out

  // DuplicateSharedMemory()
  // =======================
  //
  // Duplicates a shared memory region handle into a new distinct handle
  // referencing the same underlying region.
  IpczResult(IPCZ_API* DuplicateSharedMemory)(
      IpczDriverHandle driver_memory,        // in
      uint32_t flags,                        // in
      const void* options,                   // in
      IpczDriverHandle* new_driver_memory);  // out

  // MapSharedMemory()
  // =================
  //
  // Maps a shared memory region identified by `driver_memory` and returns its
  // mapped address in `address` on success and a driver handle in
  // `driver_mapping` which can be passed to the driver's Close() to unmap the
  // region later.
  //
  // Note that the lifetime of `driver_mapping` should be independent from that
  // of `driver_memory`. That is, if `driver_memory` is closed immediately after
  // this call succeeds, the returned mapping must still remain valid until the
  // mapping itself is closed.
  IpczResult(IPCZ_API* MapSharedMemory)(
      IpczDriverHandle driver_memory,     // in
      uint32_t flags,                     // in
      const void* options,                // in
      volatile void** address,            // out
      IpczDriverHandle* driver_mapping);  // out

  // GenerateRandomBytes()
  // =====================
  //
  // Generates `num_bytes` bytes of random data to fill `buffer`.
  IpczResult(IPCZ_API* GenerateRandomBytes)(size_t num_bytes,     // in
                                            uint32_t flags,       // in
                                            const void* options,  // in
                                            void* buffer);        // out
};

#if defined(__cplusplus)
}  // extern "C"
#endif

// Flags which may be passed via the `memory_flags` field of
// IpczCreateNodeOptions to configure features of ipcz internal memory
// allocation and usage.
typedef uint32_t IpczMemoryFlags;

// If this flag is set, the node will not attempt to expand the shared memory
// pools it uses to allocate parcel data between itself and other nodes.
//
// This means more application messages may fall back onto driver I/O for
// transmission, but also that ipcz' memory footprint will remain largely
// constant. Note that memory may still be expanded to facilitate new portal
// links as needed.
#define IPCZ_MEMORY_FIXED_PARCEL_CAPACITY ((IpczMemoryFlags)(1 << 0))

// Feature identifiers which may be passed through IpczCreateNodeOptions to
// control dynamic runtime features.
typedef uint32_t IpczFeature;

// When this feature is enabled, ipcz will use alternative shared memory layout
// and allocation behavior intended to be more efficient than the v1 scheme.
#define IPCZ_FEATURE_MEM_V2 ((IpczFeature)0xA110C002)

// Options given to CreateNode() to configure the new node's behavior.
struct IPCZ_ALIGN(8) IpczCreateNodeOptions {
  // The exact size of this structure in bytes. Must be set accurately before
  // passing the structure to CreateNode().
  size_t size;

  // See IpczMemoryFlags above.
  IpczMemoryFlags memory_flags;

  // List of features to enable for this node.
  const IpczFeature* enabled_features;
  size_t num_enabled_features;

  // List of features to disable for this node. Note that if a feature is listed
  // both in `enabled_features` and `disabled_features`, it is disabled.
  const IpczFeature* disabled_features;
  size_t num_disabled_features;
};

// See CreateNode() and the IPCZ_CREATE_NODE_* flag descriptions below.
typedef uint32_t IpczCreateNodeFlags;

// Indicates that the created node will serve as the broker in its cluster.
//
// Brokers are expected to live in relatively trusted processes -- not
// necessarily elevated in privilege but also generally not restricted by
// sandbox constraints and not prone to processing risky, untrusted data -- as
// they're responsible for helping other nodes establish direct lines of
// communication as well as in some cases relaying data and driver handles
// between lesser-privileged nodes.
//
// Broker nodes do not expose any additional ipcz APIs or require much other
// special care on the part of the application**, but every cluster of connected
// nodes must have a node designated as the broker. Typically this is the first
// node created by an application's main process or a system-wide service
// coordinator, and all other nodes are created in processes spawned by that one
// or in processes which otherwise trust it.
//
// ** See notes on Close() regarding destruction of broker nodes.
#define IPCZ_CREATE_NODE_AS_BROKER IPCZ_FLAG_BIT(0)

// See ConnectNode() and the IPCZ_CONNECT_NODE_* flag descriptions below.
typedef uint32_t IpczConnectNodeFlags;

// Indicates that the remote node for this connection is expected to be a broker
// node, and it will be treated as such. Do not use this flag when connecting to
// any untrusted process.
#define IPCZ_CONNECT_NODE_TO_BROKER IPCZ_FLAG_BIT(0)

// Indicates that the remote node for this connection is expected not to be a
// broker, but to already have a link to a broker; and that the calling node
// wishes to inherit the remote node's broker as well. This flag must only be
// used when connecting to a node the caller trusts. The calling node must not
// already have an established broker from a previous ConnectNode() call. The
// remote node must specify IPCZ_CONNECT_NODE_SHARE_BROKER as well.
#define IPCZ_CONNECT_NODE_INHERIT_BROKER IPCZ_FLAG_BIT(1)

// Indicates that the calling node already has a broker, and that this broker
// will be inherited by the remote node. The remote node must also specify
// IPCZ_CONNECT_NODE_INHERIT_BROKER in its corresponding ConnectNode() call.
#define IPCZ_CONNECT_NODE_SHARE_BROKER IPCZ_FLAG_BIT(2)

// ipcz may periodically allocate shared memory regions to facilitate
// communication between two nodes. In many runtime environments, even within
// a security sandbox, the driver can do this safely and directly by interfacing
// with the OS. In some environments however, direct allocation is not possible.
// In such cases a node must delegate this responsibility to some other trusted
// node in the system, typically the broker node.
//
// Specifying this flag ensures that all shared memory allocation elicited by
// the connecting node will be delegated to the connectee.
#define IPCZ_CONNECT_NODE_TO_ALLOCATION_DELEGATE IPCZ_FLAG_BIT(3)

// An opaque handle to a transaction returned by BeginGet() or BeginPut().
typedef uintptr_t IpczTransaction;

// See BeginPut() and the IPCZ_BEGIN_PUT_* flags described below.
typedef uint32_t IpczBeginPutFlags;

// Indicates that the caller is willing to produce less data than originally
// requested by their `*num_bytes` argument to BeginPut(). If the implementation
// would prefer a smaller chunk of data, passing this flag may allow the call to
// succeed while returning a smaller acceptable value in `*num_bytes`, rather
// than simply failing the call with IPCZ_RESULT_RESOURCE_EXHAUSTED.
#define IPCZ_BEGIN_PUT_ALLOW_PARTIAL IPCZ_FLAG_BIT(0)

// See EndPut() and the IPCZ_END_PUT_* flags described below.
typedef uint32_t IpczEndPutFlags;

// If this flag is given to EndPut(), the referenced transaction is aborted
// without committing its parcel to the portal.
#define IPCZ_END_PUT_ABORT IPCZ_FLAG_BIT(0)

// See Get() and the IPCZ_GET_* flag descriptions below.
typedef uint32_t IpczGetFlags;

// When given to Get(), this flag indicates that the caller is willing to accept
// a partial retrieval of the next available parcel. This means that in
// situations where Get() would normally return IPCZ_RESULT_RESOURCE_EXHAUSTED,
// it will instead return IPCZ_RESULT_OK with as much data and handles as the
// caller indicated they could accept.
#define IPCZ_GET_PARTIAL IPCZ_FLAG_BIT(0)

// See BeginGet() and the IPCZ_BEGIN_GET_* flag descriptions below.
typedef uint32_t IpczBeginGetFlags;

// Indicates that the caller will accept partial retrieval of a parcel's
// attached handles. When this flag is given handles are only transferred to
// the caller as output capacity allows, and it is not an error for the caller
// to provide insufficient output capacity. See notes on BeginGet().
#define IPCZ_BEGIN_GET_PARTIAL IPCZ_FLAG_BIT(0)

// Indicates that BeginGet() should begin an "overlapped" get-transaction on its
// source, meaning that additional overlapped get-transactions may begin on the
// same source before this one is terminated. Only valid when the source is a
// portal.
#define IPCZ_BEGIN_GET_OVERLAPPED IPCZ_FLAG_BIT(1)

// See EndGet() and the IPCZ_END_GET_* flag descriptions below.
typedef uint32_t IpczEndGetFlags;

// If this flag is given to EndGet() for a non-overlapped transaction on a
// portal, the transaction's parcel is left intact in the portal's queue instead
// of being dequeued. Note that if handles were transferred to the caller via
// BeginGet(), they still remain property of the caller and will no longer be
// attached to the parcel even if the transaction is aborted.
#define IPCZ_END_GET_ABORT IPCZ_FLAG_BIT(0)

// Enumerates the type of contents within a box.
typedef uint32_t IpczBoxType;

// A box which contains an opaque driver object.
#define IPCZ_BOX_TYPE_DRIVER_OBJECT ((IpczBoxType)0)

// A box which contains an opaque application-defined object with a custom
// serializer.
#define IPCZ_BOX_TYPE_APPLICATION_OBJECT ((IpczBoxType)1)

// A box which contains a collection of bytes and ipcz handles.
#define IPCZ_BOX_TYPE_SUBPARCEL ((IpczBoxType)2)

// A function passed to Box() when boxing application objects. This function
// implements serialization for the object identified by `object` in a manner
// similar to IpczDriver's Serialize() function. If the object is not
// serializable this must return IPCZ_RESULT_FAILED_PRECONDITION and ignore
// other arguments.
//
// This may be called with insufficient capacity (`num_bytes` and `num_handles`)
// in which case it should update those values with capacity requirements and
// return IPCZ_RESULT_RESOURCE_EXHAUSTED.
//
// Otherwise the function must fill in `bytes` and `handles` with values that
// effectively represent the opaque object identified by `object`, and return
// IPCZ_RESULT_OK to indicate success.
typedef IpczResult (*IpczApplicationObjectSerializer)(uintptr_t object,
                                                      uint32_t flags,
                                                      const void* options,
                                                      volatile void* data,
                                                      size_t* num_bytes,
                                                      IpczHandle* handles,
                                                      size_t* num_handles);

// A function passed to Box() when boxing application objects. This function
// must clean up any resources associated with the opaque object identified by
// `object`.
typedef void (*IpczApplicationObjectDestructor)(uintptr_t object,
                                                uint32_t flags,
                                                const void* options);

// Describes the contents of a box. Boxes may contain driver objects, arbitrary
// application-defined objects, or collections of bytes and ipcz handles
// (portals or other boxes).
struct IPCZ_ALIGN(8) IpczBoxContents {
  // The exact size of this structure in bytes. Must be set accurately before
  // passing the structure to Box() or Unbox().
  size_t size;

  // The type of contents contained in the box.
  IpczBoxType type;

  union {
    // A handle to a driver object, when `type` is IPCZ_BOX_TYPE_DRIVER_OBJECT.
    IpczDriverHandle driver_object;

    // An opaque object identifier which has meaning to `serializer` and
    // `destructor` when `type` is IPCZ_BOX_TYPE_APPLICATION_OBJECT.
    uintptr_t application_object;

    // A handle to an ipcz parcel object referencing the box contents. This may
    // be used with Get()/BeginGet()/EndGet() APIs to extract its serialized
    // data and handles. Used for boxes of type IPCZ_BOX_TYPE_SUBPARCEL.
    IpczHandle subparcel;
  } object;

  // Used only for IPCZ_BOX_TYPE_APPLICATION_OBJECT. ipcz may use these
  // functions to serialize and/or destroy the opaque object identified by
  // `object.application_object` above.
  IpczApplicationObjectSerializer serializer;
  IpczApplicationObjectDestructor destructor;
};

// See Unbox() and the IPCZ_UNBOX_* flags described below.
typedef uint32_t IpczUnboxFlags;

// If set, the box is not consumed and the driver handle returned is not removed
// from the box.
#define IPCZ_UNBOX_PEEK IPCZ_FLAG_BIT(0)

// Flags given by the `flags` field in IpczPortalStatus.
typedef uint32_t IpczPortalStatusFlags;

// Indicates that the opposite portal is closed. Subsequent put-transactions on
// this portal will always fail with IPCZ_RESULT_NOT_FOUND. If there are not
// currently any unretrieved parcels in the portal either, subsequent
// get-transactions will also fail with the same error.
#define IPCZ_PORTAL_STATUS_PEER_CLOSED IPCZ_FLAG_BIT(0)

// Indicates that the opposite portal is closed AND no more parcels can be
// expected to arrive from it. If this bit is set on a portal's status, the
// portal is essentially useless. Such portals no longer support Put() or
// Get() operations, and those operations will subsequently always return
// IPCZ_RESULT_NOT_FOUND.
#define IPCZ_PORTAL_STATUS_DEAD IPCZ_FLAG_BIT(1)

// Information returned by QueryPortalStatus() or provided to
// IpczTrapEventHandlers when a trap's conditions are met on a portal.
struct IPCZ_ALIGN(8) IpczPortalStatus {
  // The exact size of this structure in bytes. Must be set accurately before
  // passing the structure to any functions.
  size_t size;

  // Flags. See the IPCZ_PORTAL_STATUS_* flags described above for the possible
  // flags combined in this value.
  IpczPortalStatusFlags flags;

  // The number of unretrieved parcels queued on this portal.
  size_t num_local_parcels;

  // The number of unretrieved bytes (across all unretrieved parcels) queued on
  // this portal.
  size_t num_local_bytes;
};

// Flags given to IpczTrapConditions to indicate which types of conditions a
// trap should observe.
//
// Note that each type of condition may be considered edge-triggered or
// level-triggered. An edge-triggered condition is one which is only
// observable momentarily in response to a state change, while a level-triggered
// condition is continuously observable as long as some constraint about a
// portal's state is met.
//
// Level-triggered conditions can cause a Trap() attempt to fail if they're
// already satisfied when attempting to install a trap to monitor them.
typedef uint32_t IpczTrapConditionFlags;

// Triggers a trap event when the trap's portal is itself closed. This condition
// is always observed even if not explicitly set in the IpczTrapConditions given
// to the Trap() call. If a portal is closed while a trap is installed on it,
// an event will fire for the trap with this condition flag set. This condition
// is effectively edge-triggered, because as soon as it becomes true, any
// observing trap as well as its observed subject cease to exist.
#define IPCZ_TRAP_REMOVED IPCZ_FLAG_BIT(0)

// Triggers a trap event whenever the opposite portal is closed. Typically
// applications are interested in the more specific IPCZ_TRAP_DEAD.
// Level-triggered.
#define IPCZ_TRAP_PEER_CLOSED IPCZ_FLAG_BIT(1)

// Triggers a trap event whenever there are no more parcels available to
// retrieve from this portal AND the opposite portal is closed. This means the
// portal will never again have parcels to retrieve and is effectively useless.
// Level-triggered.
#define IPCZ_TRAP_DEAD IPCZ_FLAG_BIT(2)

// Triggers a trap event whenever the number of parcels queued for retrieval by
// this portal exceeds the threshold given by `min_local_parcels` in
// IpczTrapConditions. Level-triggered.
#define IPCZ_TRAP_ABOVE_MIN_LOCAL_PARCELS IPCZ_FLAG_BIT(3)

// Triggers a trap event whenever the number of bytes queued for retrieval by
// this portal exceeds the threshold given by `min_local_bytes` in
// IpczTrapConditions. Level-triggered.
#define IPCZ_TRAP_ABOVE_MIN_LOCAL_BYTES IPCZ_FLAG_BIT(4)

// Triggers a trap event whenever the number of locally available parcels
// increases by any amount. Edge-triggered.
#define IPCZ_TRAP_NEW_LOCAL_PARCEL IPCZ_FLAG_BIT(7)

// Indicates that the trap event is being fired from within the extent of an
// ipcz API call (i.e., as opposed to being fired from within the extent of an
// incoming driver transport notification.) For example if a trap is monitoring
// a portal for incoming parcels, and the application puts a parcel into the
// portal's peer on the same node, the trap event will be fired within the
// extent of the corresponding Put() call, and this flag will be set on the
// event.
//
// This flag is ignored when specifying conditions to watch for Trap(), and it
// may be set on any event dispatched to an IpczTrapEventHandler.
#define IPCZ_TRAP_WITHIN_API_CALL IPCZ_FLAG_BIT(9)

// A structure describing portal conditions necessary to trigger a trap and
// invoke its event handler.
struct IPCZ_ALIGN(8) IpczTrapConditions {
  // The exact size of this structure in bytes. Must be set accurately before
  // passing the structure to Trap().
  size_t size;

  // See the IPCZ_TRAP_* flags described above.
  IpczTrapConditionFlags flags;

  // See IPCZ_TRAP_ABOVE_MIN_LOCAL_PARCELS. If that flag is not set in `flags`,
  // this field is ignored.
  size_t min_local_parcels;

  // See IPCZ_TRAP_ABOVE_MIN_LOCAL_BYTES. If that flag is not set in `flags`,
  // this field is ignored.
  size_t min_local_bytes;
};

// Structure passed to each IpczTrapEventHandler invocation with details about
// the event.
struct IPCZ_ALIGN(8) IpczTrapEvent {
  // The size of this structure in bytes. Populated by ipcz to indicate which
  // version is being provided to the handler.
  size_t size;

  // The context value that was given to Trap() when installing the trap that
  // fired this event.
  uintptr_t context;

  // Flags indicating which condition(s) triggered this event.
  IpczTrapConditionFlags condition_flags;

  // The current status of the portal which triggered this event. This address
  // is only valid through the extent of the event handler invocation.
  const struct IpczPortalStatus* status;
};

// An application-defined function to be invoked by a trap when its observed
// conditions are satisfied on the monitored portal.
typedef void(IPCZ_API* IpczTrapEventHandler)(const struct IpczTrapEvent* event);

#if defined(__cplusplus)
extern "C" {
#endif

// IpczAPI
// =======
//
// Table of API functions defined by ipcz. Instances of this structure may be
// populated by passing them to an implementation of IpczGetAPIFn.
//
// Note that all functions follow a consistent parameter ordering:
//
//   1. Object handle (node or portal) if applicable
//   2. Function-specific strict input values
//   3. Flags - possibly untyped and unused
//   4. Options struct - possibly untyped and unused
//   5. Function-specific in/out values
//   6. Function-specific strict output values
//
// The rationale behind this convention is generally to have order flow from
// input to output. Flags are inputs, and options provide an extension point for
// future versions of these APIs; as such they skirt the boundary between strict
// input values and in/out values.
//
// The order and signature (ABI) of functions defined here must never change,
// but new functions may be added to the end.
struct IPCZ_ALIGN(8) IpczAPI {
  // The exact size of this structure in bytes. Must be set accurately by the
  // application before passing the structure to an implementation of
  // IpczGetAPIFn.
  size_t size;

  // Close()
  // =======
  //
  // Releases the object identified by `handle`. If it's a portal, the portal is
  // closed. If it's a node or parcel, the object is destroyed. If it's a boxed
  // driver object, the object is released via the  driver API's Close(). If
  // it's a boxed application object, the object is destroyed using the object's
  // boxed custom destructor. For portals and parcels, any pending transactions
  // are implicitly aborted.
  //
  // This function is NOT thread-safe. It is the application's responsibility to
  // ensure that no other threads are performing other operations on `handle`
  // concurrently with this call or any time thereafter.
  //
  // Note that while closure is itself a (non-blocking) synchronous operation,
  // closure of various objects may have asynchronous side effects. For example,
  // closing a portal might asynchronously trigger a trap event on the portal's
  // remote peer.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // NOTE: If `handle` is a broker node in its cluster of connected nodes,
  // certain operations across the cluster -- such as driver object transmission
  // through portals or portal transference in general -- may begin to fail
  // spontaneously once destruction is complete.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if `handle` referred to a valid object and was
  //        successfully closed by this operation.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `handle` is invalid.
  IpczResult(IPCZ_API* Close)(IpczHandle handle,     // in
                              uint32_t flags,        // in
                              const void* options);  // in

  // CreateNode()
  // ============
  //
  // Initializes a new ipcz node. Applications typically need only one node in
  // each communicating process, but it's OK to create more. Practical use cases
  // for multiple nodes per process may include various testing scenarios, and
  // any situation where simulating a multiprocess environment is useful.
  //
  // All other ipcz calls are scoped to a specific node, or to a more specific
  // object which is itself scoped to a specific node.
  //
  // `driver` is the driver to use when coordinating internode communication.
  // Nodes which will be interconnected must use the same or compatible driver
  // implementations.
  //
  // If `flags` contains IPCZ_CREATE_NODE_AS_BROKER then the node will act as
  // the broker in its cluster of connected nodes. See details on that flag
  // description above.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if a new node was created. In this case, `*node` is
  //        populated with a valid node handle upon return.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `node` is null, or `driver` is null or
  //        invalid.
  //
  //    IPCZ_RESULT_UNIMPLEMENTED if some condition of the runtime environment
  //        or architecture-specific details of the ipcz build would prevent it
  //        from operating correctly. For example, the is returned if ipcz was
  //        built against a std::atomic implementation which does not provide
  //        lock-free 32-bit and 64-bit atomics.
  IpczResult(IPCZ_API* CreateNode)(
      const struct IpczDriver* driver,              // in
      IpczCreateNodeFlags flags,                    // in
      const struct IpczCreateNodeOptions* options,  // in
      IpczHandle* node);                            // out

  // ConnectNode()
  // =============
  //
  // Connects `node` to another node in the system using an application-provided
  // driver transport handle in `transport` for communication. If this call will
  // succeed, ipcz will call back into the driver to activate the transport via
  // ActivateTransport() before returning, and may call Transmit() before or
  // after that as well.
  //
  // The application is responsible for delivering the other endpoint of the
  // transport to whatever other node will use it with its own corresponding
  // ConnectNode() call.
  //
  // The calling node opens a number of initial portals (given by
  // `num_initial_portals`) linked to corresponding initial portals on the
  // remote node as soon as a two-way connection is fully established.
  //
  // Establishment of this connection is typically asynchronous, but this
  // depends on the driver implementation. In any case, the initial portals are
  // created and returned synchronously and can be used immediately by the
  // application. If the remote node issues a corresponding ConnectNode() call
  // with a smaller `num_initial_portals`, the excess portals created by this
  // node will behave as if their peer has been closed. On the other hand if the
  // remote node gives a larger `num_initial_portals`, then its own excess
  // portals will behave as if their peer has been closed.
  //
  // If IPCZ_CONNECT_NODE_TO_BROKER is given in `flags`, the remote node must
  // be a broker node, and the calling node will treat it as such. If the
  // calling node is also a broker, the brokers' respective networks will be
  // effectively merged as long as both brokers remain alive: nodes in one
  // network will be able to discover and communicate directly with nodes in the
  // other network. Note that when two networks are merged, each broker remains
  // as the only broker within its own network; but brokers share enough
  // information to allow for discovery and interconnection of nodes between
  // networks.
  //
  // Conversely if IPCZ_CONNECT_NODE_TO_BROKER is *not* given and neither the
  // local nor remote nodes is a broker, one of the two nodes MUST NOT have a
  // broker yet and must specify IPCZ_CONNECT_NODE_INHERIT_BROKER in its
  // ConnectNode() call. The other node MUST have a broker already and it must
  // specify IPCZ_CONNECT_NODE_SHARE_BROKER in its own corresponding
  // ConnectNode() call.
  //
  // If IPCZ_CONNECT_NODE_TO_ALLOCATION_DELEGATE is given in `flags`, the
  // calling node delegates all ipcz internal shared memory allocation
  // operations to the remote node. This flag should only be used when the
  // calling node is operating in a restricted environment where direct shared
  // memory allocation is not possible.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if all arguments were valid and connection was initiated.
  //        `num_initial_portals` portal handles are populated in
  //        `initial_portals`. These may be used immediately by the application.
  //
  //        Note that because the underlying connection may be established
  //        asynchronously (depending on the driver implementation), this
  //        operation may still fail after returning this value. If this
  //        happens, all of the returned initial portals will behave as if their
  //        peer has been closed.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `node` is invalid, `num_initial_portals`
  //        is zero, `initial_portals` is null, or `flags` specifies one or more
  //        flags which are invalid for `node` or invalid when combined.
  //
  //    IPCZ_RESULT_OUT_OF_RANGE if `num_initial_portals` is larger than the
  //        ipcz implementation allows. There is no hard limit specified, but
  //        any ipcz implementation must support at least 8 initial portals.
  IpczResult(IPCZ_API* ConnectNode)(IpczHandle node,               // in
                                    IpczDriverHandle transport,    // in
                                    size_t num_initial_portals,    // in
                                    IpczConnectNodeFlags flags,    // in
                                    const void* options,           // in
                                    IpczHandle* initial_portals);  // out

  // OpenPortals()
  // =============
  //
  // Opens two new portals which exist as each other's opposite.
  //
  // Data and handles can be put in a portal with put-transactions (see Put(),
  // BeginPut(), EndPut()). Anything placed into a portal can be retrieved in
  // the same order by get-transactions (Get(), BeginGet(), EndGet()) on the
  // opposite portal.
  //
  // To open portals which span two different nodes at creation time, see
  // ConnectNode().
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if portal creation was successful. `*portal0` and
  //        `*portal1` are each populated with opaque portal handles which
  //        identify the new pair of portals. The new portals are each other's
  //        opposite and are entangled until one of them is closed.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `node` is invalid, or if either
  //        `portal0` or `portal1` is null.
  IpczResult(IPCZ_API* OpenPortals)(IpczHandle node,       // in
                                    uint32_t flags,        // in
                                    const void* options,   // in
                                    IpczHandle* portal0,   // out
                                    IpczHandle* portal1);  // out

  // MergePortals()
  // ==============
  //
  // Merges two portals into each other, effectively destroying both while
  // linking their respective peer portals with each other. A portal cannot
  // merge with its own peer, and a portal cannot be merged into another if one
  // or more parcels have already been put into or taken out of either of them.
  // There are however no restrictions on what can be done to the portal's peer
  // prior to merging the portal with another.
  //
  // If we have two portal pairs:
  //
  //    A ---- B     and    C ---- D
  //
  // some parcels are placed into A, and some parcels are placed into D, and
  // then we merge B with C, the net result will be a single portal pair:
  //
  //    A ---- D
  //
  // All past and future parcels placed into A will arrive at D, and vice versa.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the two portals were merged successfully. Neither
  //        handle is valid past this point. Parcels now travel between the
  //        merged portals' respective peers, including any parcels that were
  //        in flight or queued at the time of this merge.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `first` or `second` is invalid, if
  //        `first` and `second` are each others' peer, or `first` and `second`
  //        refer to the same portal.
  //
  //    IPCZ_RESULT_FAILED_PRECONDITION if either `first` or `second` has
  //        already had one or more parcels put into or gotten out of them.
  IpczResult(IPCZ_API* MergePortals)(IpczHandle first,      // in
                                     IpczHandle second,     // in
                                     uint32_t flags,        // in
                                     const void* options);  // out

  // QueryPortalStatus()
  // ===================
  //
  // Queries specific details regarding the status of a portal, such as the
  // number of unread parcels or data bytes available on the portal or its
  // opposite, or whether the opposite portal has already been closed.
  //
  // Note that because the portal's status is inherently dynamic and may be
  // modified at any time by any thread in any process with a handle to either
  // the portal or its opposite, the information returned in `status` may be
  // stale by the time a successful QueryPortalStatus() call returns.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the requested query was completed successfully.
  //        `status` is populated with details.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT `portal` is invalid. `status` is null or
  //        invalid.
  IpczResult(IPCZ_API* QueryPortalStatus)(
      IpczHandle portal,                 // in
      uint32_t flags,                    // in
      const void* options,               // in
      struct IpczPortalStatus* status);  // out

  // Put()
  // =====
  //
  // Executes a put-transaction to place a combination of data and handles into
  // the portal identified by `portal`. Everything put into a portal can be
  // retrieved in the same order by a corresponding get-transaction on the
  // opposite portal. Depending on the driver and the state of the relevant
  // portals, the data and handles may be delivered and retrievable immediately
  // by the remote portal, or they may be delivered asynchronously.
  //
  // `flags` is unused and must be IPCZ_NO_FLAGS.
  //
  // If this call fails (returning anything other than IPCZ_RESULT_OK), any
  // provided handles remain property of the caller. If it succeeds, their
  // ownership is assumed by ipcz.
  //
  // Data to be submitted is read directly from the address given by the `data`
  // argument, and `num_bytes` specifies how many bytes of data to copy from
  // there.
  //
  // Callers may wish to request a view directly into portal memory for direct
  // writing. In such cases, a two-phase put-transaction can be used instead by
  // calling BeginPut() and EndPut() as defined below.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the provided data and handles were successfully placed
  //        into the portal as a new parcel.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `portal` is invalid, `data` is null but
  //        `num_bytes` is non-zero, `handles` is null but `num_handles` is
  //        non-zero, one of the handles in `handles` is equal to `portal` or
  //        its (local) opposite if applicable, or if any handle in `handles` is
  //        invalid or not serializable.
  //
  //    IPCZ_RESULT_NOT_FOUND if it is known that the opposite portal has
  //        already been closed and anything put into this portal would be lost.
  IpczResult(IPCZ_API* Put)(IpczHandle portal,          // in
                            const void* data,           // in
                            size_t num_bytes,           // in
                            const IpczHandle* handles,  // in
                            size_t num_handles,         // in
                            uint32_t flags,             // in
                            const void* options);       // in

  // BeginPut()
  // ==========
  //
  // Begins a put-transaction on `portal`, returning a transaction handle in
  // `*transaction` and an address to writable portal memory in `*data`. The
  // application can write data directly to this location and complete the
  // transaction by passing the returned value of `*transaction` to EndPut().
  //
  // The input value of `*num_bytes` tells ipcz how much data the caller would
  // like to place into the portal. If the call is successful, the output value
  // of `*num_bytes` conveys the actual capacity available for writing at
  // `data`. Unless IPCZ_BEGIN_PUT_ALLOW_PARTIAL is specified in `flags`, a
  // successful BeginPut() will always return a `*num_bytes` that is at least as
  // large as the input `*num_bytes`. If `num_bytes` is null, the transaction
  // will not include any data.
  //
  // If IPCZ_BEGIN_PUT_ALLOW_PARTIAL is specified, the implementation may select
  // a smaller data size than the input value of `*num_bytes` if it has reason
  // to do so (e.g. performance or resource limitations).
  //
  // Note that any handles to be included in a put-transaction are provided when
  // finalizing it with EndPut().
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the put-transaction has been successfully started.
  //        If `data` is non-null, `*data` is set to the address of a portal
  //        buffer into which the application may write its data; `*num_bytes`
  //        is updated to reflect the capacity of that buffer, which may be
  //        greater (or possibly less than, only if IPCZ_BEGIN_PUT_ALLOW_PARTIAL
  //        was set in `flags`) the capacity requested by the input value of
  //        `*num_bytes`; and `*transaction` is set to a handle which can be
  //        used with EndPut() to finalize the transaction.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `portal` or `transaction` is invalid.
  //
  //    IPCZ_RESULT_NOT_FOUND if it is known that the peer portal has already
  //        been closed and anything put into this portal would be lost.
  IpczResult(IPCZ_API* BeginPut)(IpczHandle portal,              // in
                                 IpczBeginPutFlags flags,        // in
                                 const void* options,            // in
                                 volatile void** data,           // out
                                 size_t* num_bytes,              // in/out
                                 IpczTransaction* transaction);  // out

  // EndPut()
  // ========
  //
  // Ends the put-transaction previously started on `portal` by BeginPut() and
  // identified by `transaction`.
  //
  // `num_bytes_produced` specifies the number of bytes actually written by
  // the application into the data buffer returned by BeginPut().
  //
  // `num_handles` specifies the number of handles to transmit along with the
  // committed data, and `handles` specifies the address of those handles.
  // `handles` may be null if `num_handles` is zero.
  //
  // If this call fails (returning anything other than IPCZ_RESULT_OK), any
  // provided handles remain property of the caller. If it succeeds their
  // ownership is assumed by ipcz.
  //
  // If IPCZ_END_PUT_ABORT is given in `flags` and `transaction` is valid, all
  // other arguments are ignored; the corresponding transaction is aborted and
  // any associated resources are released.
  //
  // `options` is unused and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the put-transaction was successfully completed or
  //        aborted. If not aborted, all data and handles were committed to a
  //        new parcel and which will be enqueued for retrieval by the peer
  //        portal.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `portal` or `transaction` is invalid,
  //        `num_handles` is non-zero but `handles` is null,
  //        `num_bytes_produced` is larger than the capacity of the buffer
  //        originally returned by BeginPut(), or any handle in `handles` is
  //        invalid or not serializable. If `transaction` refers to a valid
  //        transaction, the transaction remains in-progress in this case.
  //
  //    IPCZ_RESULT_NOT_FOUND if it is known that the peer portal has already
  //        been closed and anything put into this portal would be lost. The
  //        transaction referenced by the caller is implicitly aborted and
  //        ownership of all passed handles is retained by the caller.
  IpczResult(IPCZ_API* EndPut)(IpczHandle portal,            // in
                               IpczTransaction transaction,  // in
                               size_t num_bytes_produced,    // in
                               const IpczHandle* handles,    // in
                               size_t num_handles,           // in
                               IpczEndPutFlags flags,        // in
                               const void* options);         // in

  // Get()
  // =====
  //
  // Executes a get-transaction to retrieve some combination of data and handles
  // from a source object.
  //
  // On input the values pointed to by `num_bytes` and `num_handles` specify the
  // the capacity of each corresponding output buffer argument (`buffer` and
  //`handles` respectively). A null capacity pointer implies zero capacity. It
  // is an error to specify a non-zero capacity if the corresponding output
  // buffer is null.
  //
  // Data consumed by this call is copied to the address given by `data`. If an
  // application wishes to read directly from parcel memory instead, a two-phase
  // get-transaction can be used by calling BeginGet() and EndGet() as defined
  // below.
  //
  // If the caller does not provide enough storage capacity for all data and
  // handles in the parcel and does not specify IPCZ_GET_PARTIAL in `flags`,
  // this returns IPCZ_RESULT_RESOURCE_EXHAUSTED and outputs the actual capacity
  // required for the message, without copying any of its contents. See details
  // of the IPCZ_RESULT_RESOURCE_EXHAUSTED result below.
  //
  // If IPCZ_GET_PARTIAL is specified, capacity requirements are not enforced.
  // Instead the call succeeds after copying as much data and handles as will
  // fit in the provided buffers. Upon return `*num_bytes` and `*num_handles`
  // are set to the capacity actually consumed from the parcel.
  //
  // Parcel data is never consumed per se, so multiple consecutive partial gets
  // on a single parcel will copy data from the same buffer every time. This is
  // not true for handles however: any handles returned by Get() are transferred
  // to the caller and removed from the parcel. Subsequent get-transactions on
  // the same parcel will not retrieve handles which have already been consumed.
  //
  // Any unconsumed data and handles during a partial Get() are permanently lost
  // unless the caller retains a handle to the parcel and retreives them with a
  // subsequent get-transaction on the same parcel.
  //
  // In any case, if this call succeeds and `parcel` is non-null, then `*parcel
  // is populated with a new parcel handle which the application can use to
  // refer to the underlying parcel in future operations (e.g. Reject() or
  // additional get-transactions).
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if `source` is a portal and there is a parcel available
  //        in the portal's queue, or `source` is a parcel; and either
  //        IPCZ_GET_PARTIAL was specified or the caller provided sufficient
  //        capacity to receive all data and handles from the parcel.
  //
  //        If `num_bytes` was not null, `*num_bytes` is set to the number of
  //        bytes actually copied out to `data`, which will never be greater
  //        than the input value of `*num_bytes`. Similarly if `num_handles`
  //        was not null, `*num_handles` is set to the number of handles
  //        actually copied out to `handles`, which will never be greater than
  //        the input value of `*num_handles`.
  //
  //        If `parcel` was non-null, it is populated with a handle to the
  //        retrieved parcel object. Any handles receivedby the caller are no
  //        longer attached to the returned parcel object and will not be
  //        present if, for example, another Get() is issued on the parcel.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `source` is invalid or not a portal or
  //        parcel, `data` is null but `*num_bytes` is non-zero, or `handles` is
  //        null but `*num_handles` is non-zero.
  //
  //    IPCZ_RESULT_RESOURCE_EXHAUSTED if the parcel's data and handles could
  //        fit in the caller's provided buffers and the caller did not specify
  //        IPCZ_GET_PARTIAL in `flags`. If `num_bytes` was not null,
  //        `*num_bytes` is set to the size in bytes of the parcel's data. If
  //        `num_handles` was not null, `*num_handles` is set to the number of
  //        handles attached to the parcel.
  //
  //    IPCZ_RESULT_UNAVAILABLE if `source` is a portal whose parcel queue is
  //        currently empty. In this case callers should wait before attempting
  //        to get anything from the same portal again.
  //
  //    IPCZ_RESULT_NOT_FOUND if `source` is a portal which has no more parcels
  //        in its queue and whose peer portal is known to be closed. If this
  //        result is returned, no more parcels can ever be read from `source`.
  //
  //    IPCZ_RESULT_ALREADY_EXISTS if there is a non-overlapped two-phase
  //        get-transaction in progress on `source`.
  IpczResult(IPCZ_API* Get)(IpczHandle source,    // in
                            IpczGetFlags flags,   // in
                            const void* options,  // in
                            void* data,           // out
                            size_t* num_bytes,    // in/out
                            IpczHandle* handles,  // out
                            size_t* num_handles,  // in/out
                            IpczHandle* parcel);  // out

  // BeginGet()
  // ==========
  //
  // Begins a get-transaction on `source` to retrieve data and handles. If
  // `source` is a portal this operates on the parcel at the head of the
  // portal's incoming queue; if `source` is a parcel (as returned by a prior
  // EndGet() or as extracted from a box of type IPCZ_BOX_TYPE_SUBPARCEL), this
  // operates directly on the parcel itself.
  //
  // `transaction` must be non-null. On success `*transaction` is set to a value
  // which can be used to finalize the transaction via `EndGet()`.
  //
  // If `data` is non-null, `*data` is set to the address of readable data for
  // the caller to consume. If `num_bytes` is not null, `*num_bytes` is set to
  // the number of bytes stored at the returned data address. If the parcel has
  // no data, `*data` is set to null and `*num_bytes` is set to zero. Any
  // returned data address remains valid only until the transaction is
  // terminated, either by calling EndGet() or by closing `source`.
  //
  // NOTE: When performing get-transactions, callers should be mindful of
  // time-of-check/time-of-use (TOCTOU) vulnerabilities. Exposed parcel memory
  // may be shared with (and still writable in) the process which transmitted
  // the parcel, and that process may not be trustworthy.
  //
  // Handles
  // -------
  // Unless the parcel has no handles attached or IPCZ_BEGIN_GET_PARTIAL is
  // set in `flags`, `handles` must be non-null and `*num_handles` must convey
  // the storage capacity available in `handles`. If the parcel has more than
  // `*num_handles` handles attached (or any handles, if `num_handles` is null),
  // the call fails with IPCZ_RESULT_RESOURCE_EXHAUSTED. In this case if
  // `num_handles` is not null, `*num_handles` is set to the number of handles
  // actually attached to the parcel.
  //
  // Ownership of any handles copied into `handles` is immediately assumed by
  // the caller once a successful BeginGet() returns, even if the transaction is
  // later aborted (see IPCZ_END_GET_ABORT).
  //
  // If IPCZ_BEGIN_GET_PARTIAL is set in `flags`, handle capacity constraints
  // are not enforced: BeginGet() will copy (and transfer ownership of) only as
  // many handles as will fit in the caller's provided handle buffer and any
  // additional handles will remain attached to the parcel. In this case the
  // output value of `*num_handles` will reflect the number of handles actually
  // copied into the caller's buffer.
  //
  // Concurrent Transactions
  // -----------------------
  // By default BeginGet() establishes exclusive access to `source`, blocking it
  // from starting additional get-transactions until the current one has ended.
  // If, however, `source` is a portal and IPCZ_BEGIN_GET_OVERLAPPED is set in
  // `flags`, the underlying parcel is immediately dequeued from the portal and
  // owned exclusively by the new transaction. This allows the application to
  // begin and maintain concurrent get-transactions on the same portal.
  //
  // If `source` is not a portal, get-transactions are always exclusive and it's
  // invalid to specify IPCZ_BEGIN_GET_OVERLAPPED.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if a get-transaction was successfully started. In this
  //        case `*transaction` is set to a handle which can be used with
  //        EndGet() to finalize the transaction. If `data` is non-null then
  //        `*data` is set to the address of the transaction's data buffer. If
  //        `num_bytes` is non-null, `*num_bytes` is set to the size of the data
  //        in that buffer. If `num_handles` is non-null, `*num_handles` is set
  //        to the number of handles copied into `handles`.
  //
  //        If IPCZ_BEGIN_GET_OVERLAPPED was specified in `flags` and `source`
  //        is a portal, the underlying parcel is dequeued from the portal
  //        before returning, leaving the portal ready for additional
  //        get-transactions to begin immediately.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `source` is not a valid portal or parcel
  //        handle; `*num_handles` is non-zero but `handles` is null;
  //        `transaction` is null; or `flags` contains IPCZ_BEGIN_GET_OVERLAPPED
  //        but `source` is not a portal.
  //
  //    IPCZ_RESULT_RESOURCE_EXHAUSTED if the parcel has more handles attached
  //        than the caller's provided handle buffer can store, and
  //        IPCZ_BEGIN_GET_PARTIAL was not specified in `flags`.
  //
  //    IPCZ_RESULT_UNAVAILABLE if `source` is a portal with no incoming parcels
  //        queued for retrieval. In this case callers should wait before
  //        attempting to get anything from the same portal again. See Trap().
  //
  //    IPCZ_RESULT_NOT_FOUND if `source` is a portal with no incoming parcels
  //        queued for retreival AND its peer portal is known to be closed. In
  //        this case no get-transaction can ever succeed again on this portal.
  //
  //    IPCZ_RESULT_ALREADY_EXISTS if a non-overlapped get-transaction is
  //        already in progress on `source`; or if an overlapped get-transaction
  //        is already in progress on `source` but `flags` does not specify
  //        IPCZ_BEGIN_GET_OVERLAPPED.
  IpczResult(IPCZ_API* BeginGet)(IpczHandle source,              // in
                                 IpczBeginGetFlags flags,        // in
                                 const void* options,            // in
                                 const volatile void** data,     // out
                                 size_t* num_bytes,              // out
                                 IpczHandle* handles,            // out
                                 size_t* num_handles,            // in/out
                                 IpczTransaction* transaction);  // out

  // EndGet()
  // ========
  //
  // Ends a get-transaction identified by `transaction` on `source`, as
  // previously returned by BeginGet().
  //
  // If `source` is a portal and this was a non-overlapped transaction, the
  // underlying parcel is dequeued from the portal's incoming queue unless
  // IPCZ_END_GET_ABORT is set in `flags`.
  //
  // If this was an overlapped transaction or `source` is not a portal,
  // IPCZ_END_GET_ABORT has no effect.
  //
  // If `parcel` is non-null, `*parcel` will be set to a handle the caller can
  // use to refer to the transaction's underlying parcel in future operations,
  // e.g. to Reject() it or to begin another get-transaction on it later.
  //
  // `options` is unused and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the get-transaction was successfully committed or
  //        aborted. If committed and `parcel` was non-null, `*parcel` is
  //        assigned a handle to the transaction's underlying parcel. This may
  //        be used in future calls to Reject(), BeginGet(), or EndGet().
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `source` or `transaction` is invalid.
  IpczResult(IPCZ_API* EndGet)(IpczHandle source,
                               IpczTransaction transaction,  // in
                               IpczEndGetFlags flags,        // in
                               const void* options,          // in
                               IpczHandle* parcel);          // in

  // Trap()
  // ======
  //
  // Attempts to install a trap to catch interesting changes to a portal's
  // state. The condition(s) to observe are specified in `conditions`.
  // Regardless of what conditions the caller specifies, all successfully
  // installed traps also implicitly observe IPCZ_TRAP_REMOVED.
  //
  // If successful, ipcz guarantees that `handler` will be invoked -- with the
  // the `context` field of the invocation's IpczTrapEvent reflecting the value
  // of `context` given here -- once any of the specified conditions have been
  // met.
  //
  // Immediately before invoking its handler, the trap is removed from the
  // portal and must be reinstalled in order to observe further state changes.
  //
  // When a portal is closed, any traps still installed on it are notified by
  // invoking their handler with IPCZ_TRAP_REMOVED in the event's
  // `condition_flags`. This effectively guarantees that all installed traps
  // eventually see a handler invocation.
  //
  // Note that `handler` may be invoked from any thread that can modify the
  // state of the observed portal. This is limited to threads which make direct
  // ipcz calls on the portal, and any threads on which the portal's node may
  // receive notifications from a driver transport.
  //
  // If any of the specified conditions are already met, the trap is not
  // installed and this call returns IPCZ_RESULT_FAILED_PRECONDITION. See below
  // for details.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the trap was installed successfully. In this case
  //        `flags` and `status` arguments are ignored.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `portal` is invalid, `conditions` is
  //        null or invalid, `handler` is null, or `status` is non-null but its
  //        `size` field specifies an invalid value.
  //
  //    IPCZ_RESULT_FAILED_PRECONDITION if the conditions specified are already
  //        met on the portal. If `satisfied_condition_flags` is non-null, then
  //        its pointee value will be updated to reflect the flags in
  //        `conditions` which were already satisfied by the portal's state. If
  //        `status` is non-null, a copy of the portal's last known status will
  //        also be stored there.
  IpczResult(IPCZ_API* Trap)(
      IpczHandle portal,                                  // in
      const struct IpczTrapConditions* conditions,        // in
      IpczTrapEventHandler handler,                       // in
      uintptr_t context,                                  // in
      uint32_t flags,                                     // in
      const void* options,                                // in
      IpczTrapConditionFlags* satisfied_condition_flags,  // out
      struct IpczPortalStatus* status);                   // out

  // Reject()
  // ========
  //
  // Reports an application-level validation failure to ipcz, in reference to
  // a specific `parcel` returned by a previous call to Get() or EndGet().
  //
  // ipcz propagates this rejection to the driver via
  // ReportBadTransportActivity(), if and only if the associated parcel did in
  // fact come from a remote node.
  //
  // `context` is an opaque handle which, on success, is passed to the driver
  // when issuing a corresponding ReportBadTransportActivity() invocation.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the driver was successfully notified about this
  //        rejection via ReportBadTransportActivity().
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `parcel` is not a valid parcel handle
  //        previously returned by Get() or EndGet().
  //
  //    IPCZ_RESULT_FAILED_PRECONDITION if `parcel` is associated with a parcel
  //        that did not come from another node.
  IpczResult(IPCZ_API* Reject)(IpczHandle parcel,
                               uintptr_t context,
                               uint32_t flags,
                               const void* options);

  // Box()
  // =====
  //
  // Boxes an object managed by the driver or application and returns a new
  // IpczHandle to reference the box. If the driver or application is able to
  // serialize the boxed object, the box can be placed into a portal for
  // transmission to another node.
  //
  // Boxes can be sent through portals along with other IpczHandles, effectively
  // allowing drivers and applications to introduce new types of transferrable
  // objects via boxes.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the object was boxed and a new IpczHandle is returned
  //        in `handle`.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `contents` is null or malformed, or if
  //        `handle` is null.
  IpczResult(IPCZ_API* Box)(IpczHandle node,                         // in
                            const struct IpczBoxContents* contents,  // in
                            uint32_t flags,                          // in
                            const void* options,                     // in
                            IpczHandle* handle);                     // out

  // Unbox()
  // =======
  //
  // Unboxes the contents of a box previously produced by Box(). Note that if
  // a box was originally produced from an application object and subsequently
  // transmitted to another node, it will be unboxed as a parcel fragment
  // produced by the object's custom serializer.
  //
  // `flags` is ignored and must be 0.
  //
  // `options` is ignored and must be null.
  //
  // Returns:
  //
  //    IPCZ_RESULT_OK if the object was successfully unboxed. A description of
  //        the box contents is placed in `contents`.
  //
  //    IPCZ_RESULT_INVALID_ARGUMENT if `handle` is invalid or does not
  //        reference a box, or if `contents` is null or malformed.
  IpczResult(IPCZ_API* Unbox)(IpczHandle handle,                  // in
                              IpczUnboxFlags flags,               // in
                              const void* options,                // in
                              struct IpczBoxContents* contents);  // out
};

// A function which populates `api` with a table of ipcz API functions. The
// `size` field must be set by the caller to the size of the structure before
// issuing this call.
//
// In practice ipcz defines IpczGetAPI() as an implementation of this function
// type. How applications acquire a reference to that function depends on how
// the application builds and links against ipcz.
//
// Upon return, `api->size` indicates the size of the function table actually
// populated and therefore which version of the ipcz implementation is in use.
// Note that this size will never exceed the input value of `api->size`: if the
// caller is built against an older version than what is available, the
// available implementation will only populate the functions appropriate for
// that older version. Conversely if the caller is built against a newer version
// than what is available, `api->size` on output may be smaller than its value
// was on input.
//
// Returns:
//
//    IPCZ_RESULT_OK if `api` was successfully populated. In this case
//       `api->size` effectively indicates the API version provided, and the
//       appropriate function pointers within `api` are filled in.
//
//    IPCZ_RESULT_INVALID_ARGUMENT if `api` is null or the caller's provided
//       `api->size` is less than the size of the function table required to
//       host API version 0.
typedef IpczResult(IPCZ_API* IpczGetAPIFn)(struct IpczAPI* api);

#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // IPCZ_INCLUDE_IPCZ_IPCZ_H_
