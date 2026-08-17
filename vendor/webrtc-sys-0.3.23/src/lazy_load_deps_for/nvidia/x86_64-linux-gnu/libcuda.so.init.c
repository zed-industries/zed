/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

#ifndef _GNU_SOURCE
#define _GNU_SOURCE // For RTLD_DEFAULT
#endif

#define HAS_DLOPEN_CALLBACK 0
#define HAS_DLSYM_CALLBACK 0
#define NO_DLOPEN 0
#define LAZY_LOAD 1
#define THREAD_SAFE 1

#include <dlfcn.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

#if THREAD_SAFE
#include <pthread.h>
#endif

// Sanity check for ARM to avoid puzzling runtime crashes
#ifdef __arm__
# if defined __thumb__ && ! defined __THUMB_INTERWORK__
#   error "ARM trampolines need -mthumb-interwork to work in Thumb mode"
# endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define CHECK(cond, fmt, ...) do { \
    if(!(cond)) { \
      fprintf(stderr, "implib-gen: libcuda.so.1: " fmt "\n", ##__VA_ARGS__); \
      assert(0 && "Assertion in generated code"); \
      abort(); \
    } \
  } while(0)

static void *lib_handle;
static int dlopened;

#if ! NO_DLOPEN

#if THREAD_SAFE

// We need to consider two cases:
// - different threads calling intercepted APIs in parallel
// - same thread calling 2 intercepted APIs recursively
//   due to dlopen calling library constructors
//   (usually happens only under IMPLIB_EXPORT_SHIMS)

// Current recursive mutex approach will deadlock
// if library constructor starts and joins a new thread
// which (directly or indirectly) calls another library function.
// Such situations should be very rare (although chances
// are higher when -DIMLIB_EXPORT_SHIMS are enabled).
//
// Similar issue is present in Glibc so hopefully it's
// not a big deal: // http://sourceware.org/bugzilla/show_bug.cgi?id=15686
// (also google for "dlopen deadlock).

static pthread_mutex_t mtx;
static int rec_count;

static void init_lock(void) {
  // We need recursive lock because dlopen will call library constructors
  // which may call other intercepted APIs that will call load_library again.
  // PTHREAD_RECURSIVE_MUTEX_INITIALIZER is not portable
  // so we do it hard way.

  pthread_mutexattr_t attr;
  CHECK(0 == pthread_mutexattr_init(&attr), "failed to init mutex");
  CHECK(0 == pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE), "failed to init mutex");

  CHECK(0 == pthread_mutex_init(&mtx, &attr), "failed to init mutex");
}

static int lock(void) {
  static pthread_once_t once = PTHREAD_ONCE_INIT;
  CHECK(0 == pthread_once(&once, init_lock), "failed to init lock");

  CHECK(0 == pthread_mutex_lock(&mtx), "failed to lock mutex");

  return 0 == __sync_fetch_and_add(&rec_count, 1);
}

static void unlock(void) {
  __sync_fetch_and_add(&rec_count, -1);
  CHECK(0 == pthread_mutex_unlock(&mtx), "failed to unlock mutex");
}
#else
static int lock(void) {
  return 1;
}
static void unlock(void) {}
#endif

static int load_library(void) {
  int publish = lock();

  if (lib_handle) {
    unlock();
    return publish;
  }

#if HAS_DLOPEN_CALLBACK
  extern void *(const char *lib_name);
  lib_handle = ("libcuda.so.1");
  CHECK(lib_handle, "failed to load library 'libcuda.so.1' via callback ''");
#else
  lib_handle = dlopen("libcuda.so.1", RTLD_LAZY | RTLD_GLOBAL);
  CHECK(lib_handle, "failed to load library 'libcuda.so.1' via dlopen: %s", dlerror());
#endif

  // With (non-default) IMPLIB_EXPORT_SHIMS we may call dlopen more than once
  // so dlclose it if we are not the first ones
  if (__sync_val_compare_and_swap(&dlopened, 0, 1)) {
    dlclose(lib_handle);
  }

  unlock();

  return publish;
}

// Run dtor as late as possible in case library functions are
// called in other global dtors
// FIXME: this may crash if one thread is calling into library
// while some other thread executes exit(). It's no clear
// how to fix this besides simply NOT dlclosing library at all.
static void __attribute__((destructor(101))) unload_lib(void) {
  if (dlopened) {
    dlclose(lib_handle);
    lib_handle = 0;
    dlopened = 0;
  }
}
#endif

#if ! NO_DLOPEN && ! LAZY_LOAD
static void __attribute__((constructor(101))) load_lib(void) {
  load_library();
}
#endif

// TODO: convert to single 0-separated string
static const char *const sym_names[] = {
  "cuArray3DCreate",
  "cuArray3DCreate_v2",
  "cuArray3DGetDescriptor",
  "cuArray3DGetDescriptor_v2",
  "cuArrayCreate",
  "cuArrayCreate_v2",
  "cuArrayDestroy",
  "cuArrayGetDescriptor",
  "cuArrayGetDescriptor_v2",
  "cuArrayGetMemoryRequirements",
  "cuArrayGetPlane",
  "cuArrayGetSparseProperties",
  "cuCheckpointProcessCheckpoint",
  "cuCheckpointProcessGetRestoreThreadId",
  "cuCheckpointProcessGetState",
  "cuCheckpointProcessLock",
  "cuCheckpointProcessRestore",
  "cuCheckpointProcessUnlock",
  "cuCoredumpGetAttribute",
  "cuCoredumpGetAttributeGlobal",
  "cuCoredumpSetAttribute",
  "cuCoredumpSetAttributeGlobal",
  "cuCtxAttach",
  "cuCtxCreate",
  "cuCtxCreate_v2",
  "cuCtxCreate_v3",
  "cuCtxCreate_v4",
  "cuCtxDestroy",
  "cuCtxDestroy_v2",
  "cuCtxDetach",
  "cuCtxDisablePeerAccess",
  "cuCtxEnablePeerAccess",
  "cuCtxFromGreenCtx",
  "cuCtxGetApiVersion",
  "cuCtxGetCacheConfig",
  "cuCtxGetCurrent",
  "cuCtxGetDevResource",
  "cuCtxGetDevice",
  "cuCtxGetExecAffinity",
  "cuCtxGetFlags",
  "cuCtxGetId",
  "cuCtxGetLimit",
  "cuCtxGetSharedMemConfig",
  "cuCtxGetStreamPriorityRange",
  "cuCtxPopCurrent",
  "cuCtxPopCurrent_v2",
  "cuCtxPushCurrent",
  "cuCtxPushCurrent_v2",
  "cuCtxRecordEvent",
  "cuCtxResetPersistingL2Cache",
  "cuCtxSetCacheConfig",
  "cuCtxSetCurrent",
  "cuCtxSetFlags",
  "cuCtxSetLimit",
  "cuCtxSetSharedMemConfig",
  "cuCtxSynchronize",
  "cuCtxWaitEvent",
  "cuDestroyExternalMemory",
  "cuDestroyExternalSemaphore",
  "cuDevResourceGenerateDesc",
  "cuDevSmResourceSplitByCount",
  "cuDeviceCanAccessPeer",
  "cuDeviceComputeCapability",
  "cuDeviceGet",
  "cuDeviceGetAttribute",
  "cuDeviceGetByPCIBusId",
  "cuDeviceGetCount",
  "cuDeviceGetDefaultMemPool",
  "cuDeviceGetDevResource",
  "cuDeviceGetExecAffinitySupport",
  "cuDeviceGetGraphMemAttribute",
  "cuDeviceGetLuid",
  "cuDeviceGetMemPool",
  "cuDeviceGetName",
  "cuDeviceGetNvSciSyncAttributes",
  "cuDeviceGetP2PAttribute",
  "cuDeviceGetPCIBusId",
  "cuDeviceGetProperties",
  "cuDeviceGetTexture1DLinearMaxWidth",
  "cuDeviceGetUuid",
  "cuDeviceGetUuid_v2",
  "cuDeviceGraphMemTrim",
  "cuDevicePrimaryCtxGetState",
  "cuDevicePrimaryCtxRelease",
  "cuDevicePrimaryCtxRelease_v2",
  "cuDevicePrimaryCtxReset",
  "cuDevicePrimaryCtxReset_v2",
  "cuDevicePrimaryCtxRetain",
  "cuDevicePrimaryCtxSetFlags",
  "cuDevicePrimaryCtxSetFlags_v2",
  "cuDeviceRegisterAsyncNotification",
  "cuDeviceSetGraphMemAttribute",
  "cuDeviceSetMemPool",
  "cuDeviceTotalMem",
  "cuDeviceTotalMem_v2",
  "cuDeviceUnregisterAsyncNotification",
  "cuDriverGetVersion",
  "cuEGLApiInit",
  "cuEGLStreamConsumerAcquireFrame",
  "cuEGLStreamConsumerConnect",
  "cuEGLStreamConsumerConnectWithFlags",
  "cuEGLStreamConsumerDisconnect",
  "cuEGLStreamConsumerReleaseFrame",
  "cuEGLStreamProducerConnect",
  "cuEGLStreamProducerDisconnect",
  "cuEGLStreamProducerPresentFrame",
  "cuEGLStreamProducerReturnFrame",
  "cuEventCreate",
  "cuEventDestroy",
  "cuEventDestroy_v2",
  "cuEventElapsedTime",
  "cuEventElapsedTime_v2",
  "cuEventQuery",
  "cuEventRecord",
  "cuEventRecordWithFlags",
  "cuEventRecordWithFlags_ptsz",
  "cuEventRecord_ptsz",
  "cuEventSynchronize",
  "cuExternalMemoryGetMappedBuffer",
  "cuExternalMemoryGetMappedMipmappedArray",
  "cuFlushGPUDirectRDMAWrites",
  "cuFuncGetAttribute",
  "cuFuncGetModule",
  "cuFuncGetName",
  "cuFuncGetParamInfo",
  "cuFuncIsLoaded",
  "cuFuncLoad",
  "cuFuncSetAttribute",
  "cuFuncSetBlockShape",
  "cuFuncSetCacheConfig",
  "cuFuncSetSharedMemConfig",
  "cuFuncSetSharedSize",
  "cuGLCtxCreate",
  "cuGLCtxCreate_v2",
  "cuGLGetDevices",
  "cuGLGetDevices_v2",
  "cuGLInit",
  "cuGLMapBufferObject",
  "cuGLMapBufferObjectAsync",
  "cuGLMapBufferObjectAsync_v2",
  "cuGLMapBufferObjectAsync_v2_ptsz",
  "cuGLMapBufferObject_v2",
  "cuGLMapBufferObject_v2_ptds",
  "cuGLRegisterBufferObject",
  "cuGLSetBufferObjectMapFlags",
  "cuGLUnmapBufferObject",
  "cuGLUnmapBufferObjectAsync",
  "cuGLUnregisterBufferObject",
  "cuGetErrorName",
  "cuGetErrorString",
  "cuGetExportTable",
  "cuGetProcAddress",
  "cuGetProcAddress_v2",
  "cuGraphAddBatchMemOpNode",
  "cuGraphAddChildGraphNode",
  "cuGraphAddDependencies",
  "cuGraphAddDependencies_v2",
  "cuGraphAddEmptyNode",
  "cuGraphAddEventRecordNode",
  "cuGraphAddEventWaitNode",
  "cuGraphAddExternalSemaphoresSignalNode",
  "cuGraphAddExternalSemaphoresWaitNode",
  "cuGraphAddHostNode",
  "cuGraphAddKernelNode",
  "cuGraphAddKernelNode_v2",
  "cuGraphAddMemAllocNode",
  "cuGraphAddMemFreeNode",
  "cuGraphAddMemcpyNode",
  "cuGraphAddMemsetNode",
  "cuGraphAddNode",
  "cuGraphAddNode_v2",
  "cuGraphBatchMemOpNodeGetParams",
  "cuGraphBatchMemOpNodeSetParams",
  "cuGraphChildGraphNodeGetGraph",
  "cuGraphClone",
  "cuGraphConditionalHandleCreate",
  "cuGraphCreate",
  "cuGraphDebugDotPrint",
  "cuGraphDestroy",
  "cuGraphDestroyNode",
  "cuGraphEventRecordNodeGetEvent",
  "cuGraphEventRecordNodeSetEvent",
  "cuGraphEventWaitNodeGetEvent",
  "cuGraphEventWaitNodeSetEvent",
  "cuGraphExecBatchMemOpNodeSetParams",
  "cuGraphExecChildGraphNodeSetParams",
  "cuGraphExecDestroy",
  "cuGraphExecEventRecordNodeSetEvent",
  "cuGraphExecEventWaitNodeSetEvent",
  "cuGraphExecExternalSemaphoresSignalNodeSetParams",
  "cuGraphExecExternalSemaphoresWaitNodeSetParams",
  "cuGraphExecGetFlags",
  "cuGraphExecHostNodeSetParams",
  "cuGraphExecKernelNodeSetParams",
  "cuGraphExecKernelNodeSetParams_v2",
  "cuGraphExecMemcpyNodeSetParams",
  "cuGraphExecMemsetNodeSetParams",
  "cuGraphExecNodeSetParams",
  "cuGraphExecUpdate",
  "cuGraphExecUpdate_v2",
  "cuGraphExternalSemaphoresSignalNodeGetParams",
  "cuGraphExternalSemaphoresSignalNodeSetParams",
  "cuGraphExternalSemaphoresWaitNodeGetParams",
  "cuGraphExternalSemaphoresWaitNodeSetParams",
  "cuGraphGetEdges",
  "cuGraphGetEdges_v2",
  "cuGraphGetNodes",
  "cuGraphGetRootNodes",
  "cuGraphHostNodeGetParams",
  "cuGraphHostNodeSetParams",
  "cuGraphInstantiate",
  "cuGraphInstantiateWithFlags",
  "cuGraphInstantiateWithParams",
  "cuGraphInstantiateWithParams_ptsz",
  "cuGraphInstantiate_v2",
  "cuGraphKernelNodeCopyAttributes",
  "cuGraphKernelNodeGetAttribute",
  "cuGraphKernelNodeGetParams",
  "cuGraphKernelNodeGetParams_v2",
  "cuGraphKernelNodeSetAttribute",
  "cuGraphKernelNodeSetParams",
  "cuGraphKernelNodeSetParams_v2",
  "cuGraphLaunch",
  "cuGraphLaunch_ptsz",
  "cuGraphMemAllocNodeGetParams",
  "cuGraphMemFreeNodeGetParams",
  "cuGraphMemcpyNodeGetParams",
  "cuGraphMemcpyNodeSetParams",
  "cuGraphMemsetNodeGetParams",
  "cuGraphMemsetNodeSetParams",
  "cuGraphNodeFindInClone",
  "cuGraphNodeGetDependencies",
  "cuGraphNodeGetDependencies_v2",
  "cuGraphNodeGetDependentNodes",
  "cuGraphNodeGetDependentNodes_v2",
  "cuGraphNodeGetEnabled",
  "cuGraphNodeGetType",
  "cuGraphNodeSetEnabled",
  "cuGraphNodeSetParams",
  "cuGraphReleaseUserObject",
  "cuGraphRemoveDependencies",
  "cuGraphRemoveDependencies_v2",
  "cuGraphRetainUserObject",
  "cuGraphUpload",
  "cuGraphUpload_ptsz",
  "cuGraphicsEGLRegisterImage",
  "cuGraphicsGLRegisterBuffer",
  "cuGraphicsGLRegisterImage",
  "cuGraphicsMapResources",
  "cuGraphicsMapResources_ptsz",
  "cuGraphicsResourceGetMappedEglFrame",
  "cuGraphicsResourceGetMappedMipmappedArray",
  "cuGraphicsResourceGetMappedPointer",
  "cuGraphicsResourceGetMappedPointer_v2",
  "cuGraphicsResourceSetMapFlags",
  "cuGraphicsResourceSetMapFlags_v2",
  "cuGraphicsSubResourceGetMappedArray",
  "cuGraphicsUnmapResources",
  "cuGraphicsUnmapResources_ptsz",
  "cuGraphicsUnregisterResource",
  "cuGraphicsVDPAURegisterOutputSurface",
  "cuGraphicsVDPAURegisterVideoSurface",
  "cuGreenCtxCreate",
  "cuGreenCtxDestroy",
  "cuGreenCtxGetDevResource",
  "cuGreenCtxRecordEvent",
  "cuGreenCtxStreamCreate",
  "cuGreenCtxWaitEvent",
  "cuImportExternalMemory",
  "cuImportExternalSemaphore",
  "cuInit",
  "cuIpcCloseMemHandle",
  "cuIpcGetEventHandle",
  "cuIpcGetMemHandle",
  "cuIpcOpenEventHandle",
  "cuIpcOpenMemHandle",
  "cuIpcOpenMemHandle_v2",
  "cuKernelGetAttribute",
  "cuKernelGetFunction",
  "cuKernelGetLibrary",
  "cuKernelGetName",
  "cuKernelGetParamInfo",
  "cuKernelSetAttribute",
  "cuKernelSetCacheConfig",
  "cuLaunch",
  "cuLaunchCooperativeKernel",
  "cuLaunchCooperativeKernelMultiDevice",
  "cuLaunchCooperativeKernel_ptsz",
  "cuLaunchGrid",
  "cuLaunchGridAsync",
  "cuLaunchHostFunc",
  "cuLaunchHostFunc_ptsz",
  "cuLaunchKernel",
  "cuLaunchKernelEx",
  "cuLaunchKernelEx_ptsz",
  "cuLaunchKernel_ptsz",
  "cuLibraryEnumerateKernels",
  "cuLibraryGetGlobal",
  "cuLibraryGetKernel",
  "cuLibraryGetKernelCount",
  "cuLibraryGetManaged",
  "cuLibraryGetModule",
  "cuLibraryGetUnifiedFunction",
  "cuLibraryLoadData",
  "cuLibraryLoadFromFile",
  "cuLibraryUnload",
  "cuLinkAddData",
  "cuLinkAddData_v2",
  "cuLinkAddFile",
  "cuLinkAddFile_v2",
  "cuLinkComplete",
  "cuLinkCreate",
  "cuLinkCreate_v2",
  "cuLinkDestroy",
  "cuMemAddressFree",
  "cuMemAddressReserve",
  "cuMemAdvise",
  "cuMemAdvise_v2",
  "cuMemAlloc",
  "cuMemAllocAsync",
  "cuMemAllocAsync_ptsz",
  "cuMemAllocFromPoolAsync",
  "cuMemAllocFromPoolAsync_ptsz",
  "cuMemAllocHost",
  "cuMemAllocHost_v2",
  "cuMemAllocManaged",
  "cuMemAllocPitch",
  "cuMemAllocPitch_v2",
  "cuMemAlloc_v2",
  "cuMemBatchDecompressAsync",
  "cuMemBatchDecompressAsync_ptsz",
  "cuMemCreate",
  "cuMemExportToShareableHandle",
  "cuMemFree",
  "cuMemFreeAsync",
  "cuMemFreeAsync_ptsz",
  "cuMemFreeHost",
  "cuMemFree_v2",
  "cuMemGetAccess",
  "cuMemGetAddressRange",
  "cuMemGetAddressRange_v2",
  "cuMemGetAllocationGranularity",
  "cuMemGetAllocationPropertiesFromHandle",
  "cuMemGetAttribute",
  "cuMemGetAttribute_v2",
  "cuMemGetHandleForAddressRange",
  "cuMemGetInfo",
  "cuMemGetInfo_v2",
  "cuMemHostAlloc",
  "cuMemHostGetDevicePointer",
  "cuMemHostGetDevicePointer_v2",
  "cuMemHostGetFlags",
  "cuMemHostRegister",
  "cuMemHostRegister_v2",
  "cuMemHostUnregister",
  "cuMemImportFromShareableHandle",
  "cuMemMap",
  "cuMemMapArrayAsync",
  "cuMemMapArrayAsync_ptsz",
  "cuMemPoolCreate",
  "cuMemPoolDestroy",
  "cuMemPoolExportPointer",
  "cuMemPoolExportToShareableHandle",
  "cuMemPoolGetAccess",
  "cuMemPoolGetAttribute",
  "cuMemPoolImportFromShareableHandle",
  "cuMemPoolImportPointer",
  "cuMemPoolSetAccess",
  "cuMemPoolSetAttribute",
  "cuMemPoolTrimTo",
  "cuMemPrefetchAsync",
  "cuMemPrefetchAsync_ptsz",
  "cuMemPrefetchAsync_v2",
  "cuMemPrefetchAsync_v2_ptsz",
  "cuMemRangeGetAttribute",
  "cuMemRangeGetAttributes",
  "cuMemRelease",
  "cuMemRetainAllocationHandle",
  "cuMemSetAccess",
  "cuMemUnmap",
  "cuMemcpy",
  "cuMemcpy2D",
  "cuMemcpy2DAsync",
  "cuMemcpy2DAsync_v2",
  "cuMemcpy2DAsync_v2_ptsz",
  "cuMemcpy2DUnaligned",
  "cuMemcpy2DUnaligned_v2",
  "cuMemcpy2DUnaligned_v2_ptds",
  "cuMemcpy2D_v2",
  "cuMemcpy2D_v2_ptds",
  "cuMemcpy3D",
  "cuMemcpy3DAsync",
  "cuMemcpy3DAsync_v2",
  "cuMemcpy3DAsync_v2_ptsz",
  "cuMemcpy3DBatchAsync",
  "cuMemcpy3DBatchAsync_ptsz",
  "cuMemcpy3DPeer",
  "cuMemcpy3DPeerAsync",
  "cuMemcpy3DPeerAsync_ptsz",
  "cuMemcpy3DPeer_ptds",
  "cuMemcpy3D_v2",
  "cuMemcpy3D_v2_ptds",
  "cuMemcpyAsync",
  "cuMemcpyAsync_ptsz",
  "cuMemcpyAtoA",
  "cuMemcpyAtoA_v2",
  "cuMemcpyAtoA_v2_ptds",
  "cuMemcpyAtoD",
  "cuMemcpyAtoD_v2",
  "cuMemcpyAtoD_v2_ptds",
  "cuMemcpyAtoH",
  "cuMemcpyAtoHAsync",
  "cuMemcpyAtoHAsync_v2",
  "cuMemcpyAtoHAsync_v2_ptsz",
  "cuMemcpyAtoH_v2",
  "cuMemcpyAtoH_v2_ptds",
  "cuMemcpyBatchAsync",
  "cuMemcpyBatchAsync_ptsz",
  "cuMemcpyDtoA",
  "cuMemcpyDtoA_v2",
  "cuMemcpyDtoA_v2_ptds",
  "cuMemcpyDtoD",
  "cuMemcpyDtoDAsync",
  "cuMemcpyDtoDAsync_v2",
  "cuMemcpyDtoDAsync_v2_ptsz",
  "cuMemcpyDtoD_v2",
  "cuMemcpyDtoD_v2_ptds",
  "cuMemcpyDtoH",
  "cuMemcpyDtoHAsync",
  "cuMemcpyDtoHAsync_v2",
  "cuMemcpyDtoHAsync_v2_ptsz",
  "cuMemcpyDtoH_v2",
  "cuMemcpyDtoH_v2_ptds",
  "cuMemcpyHtoA",
  "cuMemcpyHtoAAsync",
  "cuMemcpyHtoAAsync_v2",
  "cuMemcpyHtoAAsync_v2_ptsz",
  "cuMemcpyHtoA_v2",
  "cuMemcpyHtoA_v2_ptds",
  "cuMemcpyHtoD",
  "cuMemcpyHtoDAsync",
  "cuMemcpyHtoDAsync_v2",
  "cuMemcpyHtoDAsync_v2_ptsz",
  "cuMemcpyHtoD_v2",
  "cuMemcpyHtoD_v2_ptds",
  "cuMemcpyPeer",
  "cuMemcpyPeerAsync",
  "cuMemcpyPeerAsync_ptsz",
  "cuMemcpyPeer_ptds",
  "cuMemcpy_ptds",
  "cuMemsetD16",
  "cuMemsetD16Async",
  "cuMemsetD16Async_ptsz",
  "cuMemsetD16_v2",
  "cuMemsetD16_v2_ptds",
  "cuMemsetD2D16",
  "cuMemsetD2D16Async",
  "cuMemsetD2D16Async_ptsz",
  "cuMemsetD2D16_v2",
  "cuMemsetD2D16_v2_ptds",
  "cuMemsetD2D32",
  "cuMemsetD2D32Async",
  "cuMemsetD2D32Async_ptsz",
  "cuMemsetD2D32_v2",
  "cuMemsetD2D32_v2_ptds",
  "cuMemsetD2D8",
  "cuMemsetD2D8Async",
  "cuMemsetD2D8Async_ptsz",
  "cuMemsetD2D8_v2",
  "cuMemsetD2D8_v2_ptds",
  "cuMemsetD32",
  "cuMemsetD32Async",
  "cuMemsetD32Async_ptsz",
  "cuMemsetD32_v2",
  "cuMemsetD32_v2_ptds",
  "cuMemsetD8",
  "cuMemsetD8Async",
  "cuMemsetD8Async_ptsz",
  "cuMemsetD8_v2",
  "cuMemsetD8_v2_ptds",
  "cuMipmappedArrayCreate",
  "cuMipmappedArrayDestroy",
  "cuMipmappedArrayGetLevel",
  "cuMipmappedArrayGetMemoryRequirements",
  "cuMipmappedArrayGetSparseProperties",
  "cuModuleEnumerateFunctions",
  "cuModuleGetFunction",
  "cuModuleGetFunctionCount",
  "cuModuleGetGlobal",
  "cuModuleGetGlobal_v2",
  "cuModuleGetLoadingMode",
  "cuModuleGetSurfRef",
  "cuModuleGetTexRef",
  "cuModuleLoad",
  "cuModuleLoadData",
  "cuModuleLoadDataEx",
  "cuModuleLoadFatBinary",
  "cuModuleUnload",
  "cuMulticastAddDevice",
  "cuMulticastBindAddr",
  "cuMulticastBindMem",
  "cuMulticastCreate",
  "cuMulticastGetGranularity",
  "cuMulticastUnbind",
  "cuOccupancyAvailableDynamicSMemPerBlock",
  "cuOccupancyMaxActiveBlocksPerMultiprocessor",
  "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
  "cuOccupancyMaxActiveClusters",
  "cuOccupancyMaxPotentialBlockSize",
  "cuOccupancyMaxPotentialBlockSizeWithFlags",
  "cuOccupancyMaxPotentialClusterSize",
  "cuParamSetSize",
  "cuParamSetTexRef",
  "cuParamSetf",
  "cuParamSeti",
  "cuParamSetv",
  "cuPointerGetAttribute",
  "cuPointerGetAttributes",
  "cuPointerSetAttribute",
  "cuProfilerInitialize",
  "cuProfilerStart",
  "cuProfilerStop",
  "cuSignalExternalSemaphoresAsync",
  "cuSignalExternalSemaphoresAsync_ptsz",
  "cuStreamAddCallback",
  "cuStreamAddCallback_ptsz",
  "cuStreamAttachMemAsync",
  "cuStreamAttachMemAsync_ptsz",
  "cuStreamBatchMemOp",
  "cuStreamBatchMemOp_ptsz",
  "cuStreamBatchMemOp_v2",
  "cuStreamBatchMemOp_v2_ptsz",
  "cuStreamBeginCapture",
  "cuStreamBeginCaptureToGraph",
  "cuStreamBeginCaptureToGraph_ptsz",
  "cuStreamBeginCapture_ptsz",
  "cuStreamBeginCapture_v2",
  "cuStreamBeginCapture_v2_ptsz",
  "cuStreamCopyAttributes",
  "cuStreamCopyAttributes_ptsz",
  "cuStreamCreate",
  "cuStreamCreateWithPriority",
  "cuStreamDestroy",
  "cuStreamDestroy_v2",
  "cuStreamEndCapture",
  "cuStreamEndCapture_ptsz",
  "cuStreamGetAttribute",
  "cuStreamGetAttribute_ptsz",
  "cuStreamGetCaptureInfo",
  "cuStreamGetCaptureInfo_ptsz",
  "cuStreamGetCaptureInfo_v2",
  "cuStreamGetCaptureInfo_v2_ptsz",
  "cuStreamGetCaptureInfo_v3",
  "cuStreamGetCaptureInfo_v3_ptsz",
  "cuStreamGetCtx",
  "cuStreamGetCtx_ptsz",
  "cuStreamGetCtx_v2",
  "cuStreamGetCtx_v2_ptsz",
  "cuStreamGetDevice",
  "cuStreamGetDevice_ptsz",
  "cuStreamGetFlags",
  "cuStreamGetFlags_ptsz",
  "cuStreamGetGreenCtx",
  "cuStreamGetId",
  "cuStreamGetId_ptsz",
  "cuStreamGetPriority",
  "cuStreamGetPriority_ptsz",
  "cuStreamIsCapturing",
  "cuStreamIsCapturing_ptsz",
  "cuStreamQuery",
  "cuStreamQuery_ptsz",
  "cuStreamSetAttribute",
  "cuStreamSetAttribute_ptsz",
  "cuStreamSynchronize",
  "cuStreamSynchronize_ptsz",
  "cuStreamUpdateCaptureDependencies",
  "cuStreamUpdateCaptureDependencies_ptsz",
  "cuStreamUpdateCaptureDependencies_v2",
  "cuStreamUpdateCaptureDependencies_v2_ptsz",
  "cuStreamWaitEvent",
  "cuStreamWaitEvent_ptsz",
  "cuStreamWaitValue32",
  "cuStreamWaitValue32_ptsz",
  "cuStreamWaitValue32_v2",
  "cuStreamWaitValue32_v2_ptsz",
  "cuStreamWaitValue64",
  "cuStreamWaitValue64_ptsz",
  "cuStreamWaitValue64_v2",
  "cuStreamWaitValue64_v2_ptsz",
  "cuStreamWriteValue32",
  "cuStreamWriteValue32_ptsz",
  "cuStreamWriteValue32_v2",
  "cuStreamWriteValue32_v2_ptsz",
  "cuStreamWriteValue64",
  "cuStreamWriteValue64_ptsz",
  "cuStreamWriteValue64_v2",
  "cuStreamWriteValue64_v2_ptsz",
  "cuSurfObjectCreate",
  "cuSurfObjectDestroy",
  "cuSurfObjectGetResourceDesc",
  "cuSurfRefGetArray",
  "cuSurfRefSetArray",
  "cuTensorMapEncodeIm2col",
  "cuTensorMapEncodeIm2colWide",
  "cuTensorMapEncodeTiled",
  "cuTensorMapReplaceAddress",
  "cuTexObjectCreate",
  "cuTexObjectDestroy",
  "cuTexObjectGetResourceDesc",
  "cuTexObjectGetResourceViewDesc",
  "cuTexObjectGetTextureDesc",
  "cuTexRefCreate",
  "cuTexRefDestroy",
  "cuTexRefGetAddress",
  "cuTexRefGetAddressMode",
  "cuTexRefGetAddress_v2",
  "cuTexRefGetArray",
  "cuTexRefGetBorderColor",
  "cuTexRefGetFilterMode",
  "cuTexRefGetFlags",
  "cuTexRefGetFormat",
  "cuTexRefGetMaxAnisotropy",
  "cuTexRefGetMipmapFilterMode",
  "cuTexRefGetMipmapLevelBias",
  "cuTexRefGetMipmapLevelClamp",
  "cuTexRefGetMipmappedArray",
  "cuTexRefSetAddress",
  "cuTexRefSetAddress2D",
  "cuTexRefSetAddress2D_v2",
  "cuTexRefSetAddress2D_v3",
  "cuTexRefSetAddressMode",
  "cuTexRefSetAddress_v2",
  "cuTexRefSetArray",
  "cuTexRefSetBorderColor",
  "cuTexRefSetFilterMode",
  "cuTexRefSetFlags",
  "cuTexRefSetFormat",
  "cuTexRefSetMaxAnisotropy",
  "cuTexRefSetMipmapFilterMode",
  "cuTexRefSetMipmapLevelBias",
  "cuTexRefSetMipmapLevelClamp",
  "cuTexRefSetMipmappedArray",
  "cuThreadExchangeStreamCaptureMode",
  "cuUserObjectCreate",
  "cuUserObjectRelease",
  "cuUserObjectRetain",
  "cuVDPAUCtxCreate",
  "cuVDPAUCtxCreate_v2",
  "cuVDPAUGetDevice",
  "cuWaitExternalSemaphoresAsync",
  "cuWaitExternalSemaphoresAsync_ptsz",
  "cudbgApiAttach",
  "cudbgApiDetach",
  "cudbgApiInit",
  "cudbgGetAPI",
  "cudbgGetAPIVersion",
  "cudbgMain",
  "cudbgReportDriverApiError",
  "cudbgReportDriverInternalError",
  0
};

#define SYM_COUNT (sizeof(sym_names)/sizeof(sym_names[0]) - 1)

extern void *_libcuda_so_tramp_table[];

// Can be sped up by manually parsing library symtab...
void *_libcuda_so_tramp_resolve(size_t i) {
  assert(i < SYM_COUNT);

  int publish = 1;

  void *h = 0;
#if NO_DLOPEN
  // Library with implementations must have already been loaded.
  if (lib_handle) {
    // User has specified loaded library
    h = lib_handle;
  } else {
    // User hasn't provided us the loaded library so search the global namespace.
#   ifndef IMPLIB_EXPORT_SHIMS
    // If shim symbols are hidden we should search
    // for first available definition of symbol in library list
    h = RTLD_DEFAULT;
#   else
    // Otherwise look for next available definition
    h = RTLD_NEXT;
#   endif
  }
#else
  publish = load_library();
  h = lib_handle;
  CHECK(h, "failed to resolve symbol '%s', library failed to load", sym_names[i]);
#endif

  void *addr;
#if HAS_DLSYM_CALLBACK
  extern void *(void *handle, const char *sym_name);
  addr = (h, sym_names[i]);
  CHECK(addr, "failed to resolve symbol '%s' via callback ", sym_names[i]);
#else
  // Dlsym is thread-safe so don't need to protect it.
  addr = dlsym(h, sym_names[i]);
  CHECK(addr, "failed to resolve symbol '%s' via dlsym: %s", sym_names[i], dlerror());
#endif

  if (publish) {
    // Use atomic to please Tsan and ensure that preceeding writes
    // in library ctors have been delivered before publishing address
    (void)__sync_val_compare_and_swap(&_libcuda_so_tramp_table[i], 0, addr);
  }

  return addr;
}

// Below APIs are not thread-safe
// and it's not clear how make them such
// (we can not know if some other thread is
// currently executing library code).

// Helper for user to resolve all symbols
void _libcuda_so_tramp_resolve_all(void) {
  size_t i;
  for(i = 0; i < SYM_COUNT; ++i)
    _libcuda_so_tramp_resolve(i);
}

// Allows user to specify manually loaded implementation library.
void _libcuda_so_tramp_set_handle(void *handle) {
  // TODO: call unload_lib ?
  lib_handle = handle;
  dlopened = 0;
}

// Resets all resolved symbols. This is needed in case
// client code wants to reload interposed library multiple times.
void _libcuda_so_tramp_reset(void) {
  // TODO: call unload_lib ?
  memset(_libcuda_so_tramp_table, 0, SYM_COUNT * sizeof(_libcuda_so_tramp_table[0]));
  lib_handle = 0;
  dlopened = 0;
}

#ifdef __cplusplus
}  // extern "C"
#endif
