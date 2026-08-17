// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// See docs\c_cxx\README.md on generating the Doxygen documentation from this file

/** \mainpage ONNX Runtime
 *
 * ONNX Runtime is a high-performance inference and training graph execution engine for deep learning models.
 *
 * ONNX Runtime's C, C++ APIs offer an easy to use interface to onboard and execute onnx models.
 * - \subpage c_cpp_api "Core C, C++ APIs"
 * - \subpage training_c_cpp_api "Training C, C++ APIs for on-device training"
 *
 * \page c_cpp_api Core C, C++ APIs
 * <h1>C</h1>
 *
 * ::OrtApi - Click here to go to the structure with all C API functions.
 *
 * <h1>C++</h1>
 *
 * ::Ort - Click here to go to the namespace holding all of the C++ wrapper classes
 *
 * It is a set of header only wrapper classes around the C API. The goal is to turn the C style return value error codes into C++ exceptions, and to
 * automate memory management through standard C++ RAII principles.
 *
 * \addtogroup Global
 * ONNX Runtime C API
 * @{
 */

#pragma once
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/** \brief The API version defined in this header
 *
 * This value is used by some API functions to behave as this version of the header expects.
 */
#define ORT_API_VERSION 22

#ifdef __cplusplus
extern "C" {
#endif

//! @}
// SAL2 Definitions
#ifndef _MSC_VER
#define _In_
#define _In_z_
#define _In_opt_
#define _In_opt_z_
#define _Out_
#define _Outptr_
#define _Out_opt_
#define _Inout_
#define _Inout_opt_
#define _Frees_ptr_opt_
#define _Ret_maybenull_
#define _Ret_notnull_
#define _Check_return_
#define _Outptr_result_maybenull_
#define _In_reads_(X)
#define _Inout_updates_(X)
#define _Out_writes_(X)
#define _Inout_updates_all_(X)
#define _Out_writes_bytes_all_(X)
#define _Out_writes_all_(X)
#define _Success_(X)
#define _Outptr_result_buffer_maybenull_(X)
#define ORT_ALL_ARGS_NONNULL __attribute__((nonnull))
#else
#include <specstrings.h>
#define ORT_ALL_ARGS_NONNULL
#endif

#ifdef _WIN32
// Define ORT_DLL_IMPORT if your program is dynamically linked to Ort.
// dllexport is not used, we use a .def file.
#ifdef ORT_DLL_IMPORT
#define ORT_EXPORT __declspec(dllimport)
#else
#define ORT_EXPORT
#endif
#define ORT_API_CALL _stdcall
#define ORT_MUST_USE_RESULT
#define ORTCHAR_T wchar_t
#else
// To make symbols visible on macOS/iOS
#ifdef __APPLE__
#define ORT_EXPORT __attribute__((visibility("default")))
#else
#define ORT_EXPORT
#endif
#define ORT_API_CALL
#define ORT_MUST_USE_RESULT __attribute__((warn_unused_result))
#define ORTCHAR_T char
#endif

/// ORTCHAR_T, ORT_TSTR are reserved specifically for path handling.
/// All other strings are UTF-8 encoded, use char and std::string
#ifndef ORT_TSTR
#ifdef _WIN32
#define ORT_TSTR(X) L##X
// When X is a macro, L##X is not defined. In this case, we need to use ORT_TSTR_ON_MACRO.
#define ORT_TSTR_ON_MACRO(X) L"" X
#else
#define ORT_TSTR(X) X
#define ORT_TSTR_ON_MACRO(X) X
#endif
#endif

// On Windows, ORT_FILE is a wchar_t version of the __FILE__ macro.
// Otherwise, ORT_FILE is equivalent to __FILE__.
#ifndef ORT_FILE
#define ORT_FILE_INTERNAL(x) ORT_TSTR(x)
#define ORT_FILE ORT_FILE_INTERNAL(__FILE__)
#endif

// Any pointer marked with _In_ or _Out_, cannot be NULL.

// Windows users should use unicode paths when possible to bypass the MAX_PATH limitation
// Every pointer marked with _In_ or _Out_, cannot be NULL. Caller should ensure that.
// for ReleaseXXX(...) functions, they can accept NULL pointer.

#ifdef __cplusplus
// For any compiler with C++11 support, MSVC 2015 and greater, or Clang version supporting noexcept.
// Such complex condition is needed because compilers set __cplusplus value differently.
#ifndef __has_feature
#define __has_feature(x) 0
#endif
#if ((__cplusplus >= 201103L) || (_MSC_VER >= 1900) || (defined(__has_feature) && __has_feature(cxx_noexcept)))
#define NO_EXCEPTION noexcept
#else
#define NO_EXCEPTION throw()
#endif
#else
#define NO_EXCEPTION
#endif

// __VA_ARGS__ on Windows and Linux are different
#define ORT_API(RETURN_TYPE, NAME, ...) RETURN_TYPE ORT_API_CALL NAME(__VA_ARGS__) NO_EXCEPTION

#define ORT_API_STATUS(NAME, ...)                                                                   \
  _Success_(return == 0) _Check_return_ _Ret_maybenull_ OrtStatusPtr ORT_API_CALL NAME(__VA_ARGS__) \
  NO_EXCEPTION ORT_MUST_USE_RESULT

// XXX: Unfortunately, SAL annotations are known to not work with function pointers
#define ORT_API2_STATUS(NAME, ...) \
  _Check_return_ _Ret_maybenull_ OrtStatusPtr(ORT_API_CALL* NAME)(__VA_ARGS__) NO_EXCEPTION ORT_MUST_USE_RESULT

// Used in *.cc files. Almost as same as ORT_API_STATUS, except without ORT_MUST_USE_RESULT and ORT_EXPORT
#define ORT_API_STATUS_IMPL(NAME, ...) \
  _Success_(return == 0) _Check_return_ _Ret_maybenull_ OrtStatusPtr ORT_API_CALL NAME(__VA_ARGS__) NO_EXCEPTION

#define ORT_CLASS_RELEASE(X) void(ORT_API_CALL * Release##X)(_Frees_ptr_opt_ Ort##X * input)

#ifdef __DOXYGEN__
#undef ORT_API_STATUS
#define ORT_API_STATUS(NAME, ...) OrtStatus* NAME(__VA_ARGS__)
#undef ORT_API2_STATUS
#define ORT_API2_STATUS(NAME, ...) OrtStatus* NAME(__VA_ARGS__)
#undef ORT_CLASS_RELEASE
#define ORT_CLASS_RELEASE(X) void Release##X(Ort##X* input)
#undef NO_EXCEPTION
#define NO_EXCEPTION
#endif
/** \addtogroup Global
 * ONNX Runtime C API
 * @{
 */

/** Copied from TensorProto::DataType
 * Currently, Ort doesn't support complex64, complex128
 */
typedef enum ONNXTensorElementDataType {
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UNDEFINED,
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT,   // maps to c type float
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8,   // maps to c type uint8_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8,    // maps to c type int8_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16,  // maps to c type uint16_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16,   // maps to c type int16_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32,   // maps to c type int32_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_INT64,   // maps to c type int64_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING,  // maps to c++ type std::string
  ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL,
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT16,
  ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE,      // maps to c type double
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32,      // maps to c type uint32_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64,      // maps to c type uint64_t
  ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX64,   // complex with float32 real and imaginary components
  ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX128,  // complex with float64 real and imaginary components
  ONNX_TENSOR_ELEMENT_DATA_TYPE_BFLOAT16,    // Non-IEEE floating-point format based on IEEE754 single-precision
  // float 8 types were introduced in onnx 1.14, see https://onnx.ai/onnx/technical/float8.html
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT8E4M3FN,    // Non-IEEE floating-point format based on IEEE754 single-precision
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT8E4M3FNUZ,  // Non-IEEE floating-point format based on IEEE754 single-precision
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT8E5M2,      // Non-IEEE floating-point format based on IEEE754 single-precision
  ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT8E5M2FNUZ,  // Non-IEEE floating-point format based on IEEE754 single-precision
  // Int4 types were introduced in ONNX 1.16. See https://onnx.ai/onnx/technical/int4.html
  ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT4,  // maps to a pair of packed uint4 values (size == 1 byte)
  ONNX_TENSOR_ELEMENT_DATA_TYPE_INT4    // maps to a pair of packed int4 values (size == 1 byte)
} ONNXTensorElementDataType;

// Synced with onnx TypeProto oneof
typedef enum ONNXType {
  ONNX_TYPE_UNKNOWN,
  ONNX_TYPE_TENSOR,
  ONNX_TYPE_SEQUENCE,
  ONNX_TYPE_MAP,
  ONNX_TYPE_OPAQUE,
  ONNX_TYPE_SPARSETENSOR,
  ONNX_TYPE_OPTIONAL
} ONNXType;

// These types are synced with internal
// SparseFormatFlags
typedef enum OrtSparseFormat {
  ORT_SPARSE_UNDEFINED = 0,
  ORT_SPARSE_COO = 0x1,
  ORT_SPARSE_CSRC = 0x2,
  ORT_SPARSE_BLOCK_SPARSE = 0x4
} OrtSparseFormat;

// Enum allows to query sparse tensor indices
enum OrtSparseIndicesFormat {
  ORT_SPARSE_COO_INDICES,
  ORT_SPARSE_CSR_INNER_INDICES,
  ORT_SPARSE_CSR_OUTER_INDICES,
  ORT_SPARSE_BLOCK_SPARSE_INDICES
};

/** \brief Logging severity levels
 *
 * In typical API usage, specifying a logging severity level specifies the minimum severity of log messages to show.
 */
typedef enum OrtLoggingLevel {
  ORT_LOGGING_LEVEL_VERBOSE,  ///< Verbose informational messages (least severe).
  ORT_LOGGING_LEVEL_INFO,     ///< Informational messages.
  ORT_LOGGING_LEVEL_WARNING,  ///< Warning messages.
  ORT_LOGGING_LEVEL_ERROR,    ///< Error messages.
  ORT_LOGGING_LEVEL_FATAL,    ///< Fatal error messages (most severe).
} OrtLoggingLevel;

typedef enum OrtErrorCode {
  ORT_OK,
  ORT_FAIL,
  ORT_INVALID_ARGUMENT,
  ORT_NO_SUCHFILE,
  ORT_NO_MODEL,
  ORT_ENGINE_ERROR,
  ORT_RUNTIME_EXCEPTION,
  ORT_INVALID_PROTOBUF,
  ORT_MODEL_LOADED,
  ORT_NOT_IMPLEMENTED,
  ORT_INVALID_GRAPH,
  ORT_EP_FAIL,
} OrtErrorCode;

typedef enum OrtOpAttrType {
  ORT_OP_ATTR_UNDEFINED = 0,
  ORT_OP_ATTR_INT,
  ORT_OP_ATTR_INTS,
  ORT_OP_ATTR_FLOAT,
  ORT_OP_ATTR_FLOATS,
  ORT_OP_ATTR_STRING,
  ORT_OP_ATTR_STRINGS,
} OrtOpAttrType;

//! @}
#define ORT_RUNTIME_CLASS(X) \
  struct Ort##X;             \
  typedef struct Ort##X Ort##X

/** \addtogroup Global
 * ONNX Runtime C API
 * @{
 */
// The actual types defined have an Ort prefix
ORT_RUNTIME_CLASS(Env);
ORT_RUNTIME_CLASS(Status);  // nullptr for Status* indicates success
ORT_RUNTIME_CLASS(MemoryInfo);
ORT_RUNTIME_CLASS(IoBinding);
ORT_RUNTIME_CLASS(Session);  // Don't call ReleaseSession from Dllmain (because session owns a thread pool)
ORT_RUNTIME_CLASS(Value);
ORT_RUNTIME_CLASS(RunOptions);
ORT_RUNTIME_CLASS(TypeInfo);
ORT_RUNTIME_CLASS(TensorTypeAndShapeInfo);
ORT_RUNTIME_CLASS(MapTypeInfo);
ORT_RUNTIME_CLASS(SequenceTypeInfo);
ORT_RUNTIME_CLASS(OptionalTypeInfo);
ORT_RUNTIME_CLASS(SessionOptions);
ORT_RUNTIME_CLASS(CustomOpDomain);
ORT_RUNTIME_CLASS(ModelMetadata);
ORT_RUNTIME_CLASS(ThreadPoolParams);
ORT_RUNTIME_CLASS(ThreadingOptions);
ORT_RUNTIME_CLASS(ArenaCfg);
ORT_RUNTIME_CLASS(PrepackedWeightsContainer);
ORT_RUNTIME_CLASS(TensorRTProviderOptionsV2);
ORT_RUNTIME_CLASS(CUDAProviderOptionsV2);
ORT_RUNTIME_CLASS(CANNProviderOptions);
ORT_RUNTIME_CLASS(DnnlProviderOptions);
ORT_RUNTIME_CLASS(Op);
ORT_RUNTIME_CLASS(OpAttr);
ORT_RUNTIME_CLASS(Logger);
ORT_RUNTIME_CLASS(ShapeInferContext);
ORT_RUNTIME_CLASS(LoraAdapter);
ORT_RUNTIME_CLASS(ValueInfo);
ORT_RUNTIME_CLASS(Node);
ORT_RUNTIME_CLASS(Graph);
ORT_RUNTIME_CLASS(Model);

#ifdef _WIN32
typedef _Return_type_success_(return == 0) OrtStatus* OrtStatusPtr;
#else
typedef OrtStatus* OrtStatusPtr;
#endif

/** \brief Memory allocation interface
 *
 * Structure of function pointers that defines a memory allocator. This can be created and filled in by the user for custom allocators.
 *
 * When an allocator is passed to any function, be sure that the allocator object is not destroyed until the last allocated object using it is freed.
 */
typedef struct OrtAllocator {
  uint32_t version;                                                                   ///< Must be initialized to ORT_API_VERSION
  void*(ORT_API_CALL* Alloc)(struct OrtAllocator* this_, size_t size);                ///< Returns a pointer to an allocated block of `size` bytes
  void(ORT_API_CALL* Free)(struct OrtAllocator* this_, void* p);                      ///< Free a block of memory previously allocated with OrtAllocator::Alloc
  const struct OrtMemoryInfo*(ORT_API_CALL* Info)(const struct OrtAllocator* this_);  ///< Return a pointer to an ::OrtMemoryInfo that describes this allocator
  /**
   * @brief Optional allocation function to use for memory allocations made during session initialization.
   * Use this function if you want to separate allocations made by ORT during Run() calls from
   * those made during session initialization. This allows for separate memory management strategies for these allocations.
   */
  void*(ORT_API_CALL* Reserve)(struct OrtAllocator* this_, size_t size);  ///< Returns a pointer to an allocated block of `size` bytes
} OrtAllocator;

typedef void(ORT_API_CALL* OrtLoggingFunction)(
    void* param, OrtLoggingLevel severity, const char* category, const char* logid, const char* code_location,
    const char* message);

/** \brief Graph optimization level
 *
 * Refer to https://www.onnxruntime.ai/docs/performance/graph-optimizations.html#graph-optimization-levels
 * for an in-depth understanding of the Graph Optimization Levels.
 */
typedef enum GraphOptimizationLevel {
  ORT_DISABLE_ALL = 0,
  ORT_ENABLE_BASIC = 1,
  ORT_ENABLE_EXTENDED = 2,
  ORT_ENABLE_ALL = 99
} GraphOptimizationLevel;

typedef enum ExecutionMode {
  ORT_SEQUENTIAL = 0,
  ORT_PARALLEL = 1,
} ExecutionMode;

/** \brief Language projection identifiers
 * /see OrtApi::SetLanguageProjection
 */
typedef enum OrtLanguageProjection {
  ORT_PROJECTION_C = 0,
  ORT_PROJECTION_CPLUSPLUS = 1,
  ORT_PROJECTION_CSHARP = 2,
  ORT_PROJECTION_PYTHON = 3,
  ORT_PROJECTION_JAVA = 4,
  ORT_PROJECTION_WINML = 5,
  ORT_PROJECTION_NODEJS = 6,
} OrtLanguageProjection;

struct OrtKernelInfo;
typedef struct OrtKernelInfo OrtKernelInfo;
struct OrtKernelContext;
typedef struct OrtKernelContext OrtKernelContext;
struct OrtCustomOp;
typedef struct OrtCustomOp OrtCustomOp;

typedef enum OrtAllocatorType {
  OrtInvalidAllocator = -1,
  OrtDeviceAllocator = 0,
  OrtArenaAllocator = 1
} OrtAllocatorType;

/** \brief Memory types for allocated memory, execution provider specific types should be extended in each provider.
 */
// Whenever this struct is updated, please also update the MakeKey function in onnxruntime / core / framework / execution_provider.cc
typedef enum OrtMemType {
  OrtMemTypeCPUInput = -2,              ///< Any CPU memory used by non-CPU execution provider
  OrtMemTypeCPUOutput = -1,             ///< CPU accessible memory outputted by non-CPU execution provider, i.e. CUDA_PINNED
  OrtMemTypeCPU = OrtMemTypeCPUOutput,  ///< Temporary CPU accessible memory allocated by non-CPU execution provider, i.e. CUDA_PINNED
  OrtMemTypeDefault = 0,                ///< The default allocator for execution provider
} OrtMemType;

/** \brief This mimics OrtDevice type constants so they can be returned in the API
 */
typedef enum OrtMemoryInfoDeviceType {
  OrtMemoryInfoDeviceType_CPU = 0,
  OrtMemoryInfoDeviceType_GPU = 1,
  OrtMemoryInfoDeviceType_FPGA = 2
} OrtMemoryInfoDeviceType;

/** \brief Algorithm to use for cuDNN Convolution Op
 */
typedef enum OrtCudnnConvAlgoSearch {
  OrtCudnnConvAlgoSearchExhaustive,  // expensive exhaustive benchmarking using cudnnFindConvolutionForwardAlgorithmEx
  OrtCudnnConvAlgoSearchHeuristic,   // lightweight heuristic based search using cudnnGetConvolutionForwardAlgorithm_v7
  OrtCudnnConvAlgoSearchDefault,     // default algorithm using CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM
} OrtCudnnConvAlgoSearch;

/** \brief CUDA Provider Options
 *
 * \see OrtApi::SessionOptionsAppendExecutionProvider_CUDA
 */
typedef struct OrtCUDAProviderOptions {
#ifdef __cplusplus
  OrtCUDAProviderOptions()
      : device_id{},
        cudnn_conv_algo_search{OrtCudnnConvAlgoSearchExhaustive},
        gpu_mem_limit{SIZE_MAX},
        arena_extend_strategy{},
        do_copy_in_default_stream{1},
        has_user_compute_stream{},
        user_compute_stream{},
        default_memory_arena_cfg{},
        tunable_op_enable{false},
        tunable_op_tuning_enable{false},
        tunable_op_max_tuning_duration_ms{} {}
#endif

  /** \brief CUDA device Id
   *   Defaults to 0.
   */
  int device_id;

  /** \brief CUDA Convolution algorithm search configuration.
   *   See enum OrtCudnnConvAlgoSearch for more details.
   *   Defaults to OrtCudnnConvAlgoSearchExhaustive.
   */
  OrtCudnnConvAlgoSearch cudnn_conv_algo_search;

  /** \brief CUDA memory limit (To use all possible memory pass in maximum size_t)
   *   Defaults to SIZE_MAX.
   *   \note If a ::OrtArenaCfg has been applied, it will override this field
   */
  size_t gpu_mem_limit;

  /** \brief Strategy used to grow the memory arena
   *   0 = kNextPowerOfTwo<br>
   *   1 = kSameAsRequested<br>
   *   Defaults to 0.
   *   \note If a ::OrtArenaCfg has been applied, it will override this field
   */
  int arena_extend_strategy;

  /** \brief Flag indicating if copying needs to take place on the same stream as the compute stream in the CUDA EP
   *   0 = Use separate streams for copying and compute.
   *   1 = Use the same stream for copying and compute.
   *   Defaults to 1.
   *   WARNING: Setting this to 0 may result in data races for some models.
   *   Please see issue #4829 for more details.
   */
  int do_copy_in_default_stream;

  /** \brief Flag indicating if there is a user provided compute stream
   *   Defaults to 0.
   */
  int has_user_compute_stream;

  /** \brief User provided compute stream.
   *   If provided, please set `has_user_compute_stream` to 1.
   */
  void* user_compute_stream;

  /** \brief CUDA memory arena configuration parameters
   */
  OrtArenaCfg* default_memory_arena_cfg;

  /** \brief Enable TunableOp for using.
   *   Set it to 1/0 to enable/disable TunableOp. Otherwise, it is disabled by default.
   *   This option can be overridden by environment variable ORT_CUDA_TUNABLE_OP_ENABLE.
   */
  int tunable_op_enable;

  /** \brief Enable TunableOp for tuning.
   *   Set it to 1/0 to enable/disable TunableOp tuning. Otherwise, it is disabled by default.
   *   This option can be overridden by environment variable ORT_CUDA_TUNABLE_OP_TUNING_ENABLE.
   */
  int tunable_op_tuning_enable;

  /** \brief Max tuning duration time limit for each instance of TunableOp.
   *   Defaults to 0 to disable the limit.
   */
  int tunable_op_max_tuning_duration_ms;

} OrtCUDAProviderOptions;

/** \brief ROCM Provider Options
 *
 * \see OrtApi::SessionOptionsAppendExecutionProvider_ROCM
 */
typedef struct OrtROCMProviderOptions {
#ifdef __cplusplus
  OrtROCMProviderOptions()
      : device_id{},
        miopen_conv_exhaustive_search{0},
        gpu_mem_limit{SIZE_MAX},
        arena_extend_strategy{},
        do_copy_in_default_stream{1},
        has_user_compute_stream{},
        user_compute_stream{},
        default_memory_arena_cfg{},
        enable_hip_graph{false},
        tunable_op_enable{false},
        tunable_op_tuning_enable{false},
        tunable_op_max_tuning_duration_ms{} {}
#endif

  /** \brief ROCM device Id
   *   Defaults to 0.
   */
  int device_id;

  /** \brief ROCM MIOpen Convolution algorithm exaustive search option.
   *   Defaults to 0 (false).
   */
  int miopen_conv_exhaustive_search;

  /** \brief ROCM memory limit (To use all possible memory pass in maximum size_t)
   *   Defaults to SIZE_MAX.
   *   \note If a ::OrtArenaCfg has been applied, it will override this field
   */
  size_t gpu_mem_limit;

  /** \brief Strategy used to grow the memory arena
   *   0 = kNextPowerOfTwo<br>
   *   1 = kSameAsRequested<br>
   *   Defaults to 0.
   *   \note If a ::OrtArenaCfg has been applied, it will override this field
   */
  int arena_extend_strategy;

  /** \brief Flag indicating if copying needs to take place on the same stream as the compute stream in the ROCM EP
   *   0 = Use separate streams for copying and compute.
   *   1 = Use the same stream for copying and compute.
   *   Defaults to 1.
   *   WARNING: Setting this to 0 may result in data races for some models.
   *   Please see issue #4829 for more details.
   */
  int do_copy_in_default_stream;

  /** \brief Flag indicating if there is a user provided compute stream
   *   Defaults to 0.
   */
  int has_user_compute_stream;

  /** \brief User provided compute stream.
   *   If provided, please set `has_user_compute_stream` to 1.
   */
  void* user_compute_stream;

  /** \brief ROCM memory arena configuration parameters
   */
  OrtArenaCfg* default_memory_arena_cfg;

  int enable_hip_graph;

  /** \brief Enable TunableOp for using.
   *   Set it to 1/0 to enable/disable TunableOp. Otherwise, it is disabled by default.
   *   This option can be overridden by environment variable ORT_ROCM_TUNABLE_OP_ENABLE.
   */
  int tunable_op_enable;

  /** \brief Enable TunableOp for tuning.
   *   Set it to 1/0 to enable/disable TunableOp tuning. Otherwise, it is disabled by default.
   *   This option can be overridden by environment variable ORT_ROCM_TUNABLE_OP_TUNING_ENABLE.
   */
  int tunable_op_tuning_enable;

  /** \brief Max tuning duration time limit for each instance of TunableOp.
   *   Defaults to 0 to disable the limit.
   */
  int tunable_op_max_tuning_duration_ms;

} OrtROCMProviderOptions;

/** \brief TensorRT Provider Options
 *
 * \see OrtApi::SessionOptionsAppendExecutionProvider_TensorRT
 */
typedef struct OrtTensorRTProviderOptions {
  int device_id;                                ///< CUDA device id (0 = default device)
  int has_user_compute_stream;                  // indicator of user specified CUDA compute stream.
  void* user_compute_stream;                    // user specified CUDA compute stream.
  int trt_max_partition_iterations;             // maximum iterations for TensorRT parser to get capability
  int trt_min_subgraph_size;                    // minimum size of TensorRT subgraphs
  size_t trt_max_workspace_size;                // maximum workspace size for TensorRT.
  int trt_fp16_enable;                          // enable TensorRT FP16 precision. Default 0 = false, nonzero = true
  int trt_int8_enable;                          // enable TensorRT INT8 precision. Default 0 = false, nonzero = true
  const char* trt_int8_calibration_table_name;  // TensorRT INT8 calibration table name.
  int trt_int8_use_native_calibration_table;    // use native TensorRT generated calibration table. Default 0 = false, nonzero = true
  int trt_dla_enable;                           // enable DLA. Default 0 = false, nonzero = true
  int trt_dla_core;                             // DLA core number. Default 0
  int trt_dump_subgraphs;                       // dump TRT subgraph. Default 0 = false, nonzero = true
  int trt_engine_cache_enable;                  // enable engine caching. Default 0 = false, nonzero = true
  const char* trt_engine_cache_path;            // specify engine cache path
  int trt_engine_decryption_enable;             // enable engine decryption. Default 0 = false, nonzero = true
  const char* trt_engine_decryption_lib_path;   // specify engine decryption library path
  int trt_force_sequential_engine_build;        // force building TensorRT engine sequentially. Default 0 = false, nonzero = true
  // This is the legacy struct and don't add new fields here.
  // For new field that can be represented by string, please add it in include/onnxruntime/core/providers/tensorrt/tensorrt_provider_options.h
  // For non-string field, need to create a new separate api to handle it.
} OrtTensorRTProviderOptions;

/** \brief MIGraphX Provider Options
 *
 * \see OrtApi::SessionOptionsAppendExecutionProvider_MIGraphX
 */
typedef struct OrtMIGraphXProviderOptions {
  int device_id;                                     // hip device id.
  int migraphx_fp16_enable;                          // MIGraphX FP16 precision. Default 0 = false, nonzero = true
  int migraphx_int8_enable;                          // MIGraphX INT8 precision. Default 0 = false, nonzero = true
  int migraphx_use_native_calibration_table;         // MIGraphx INT8 cal table. Default 0 = false, noznero = true
  const char* migraphx_int8_calibration_table_name;  // MIGraphx INT8 calibration table name
  int migraphx_save_compiled_model;                  // migraphx save compiled model. Default 0 = false, noznero = true
  const char* migraphx_save_model_path;              // migraphx model path name
  int migraphx_load_compiled_model;                  // migraphx int8 cal table. Default 0 = false, noznero = true
  const char* migraphx_load_model_path;              // migraphx model path name
  bool migraphx_exhaustive_tune;                     // migraphx tuned compile  Default = false
} OrtMIGraphXProviderOptions;

/** \brief OpenVINO Provider Options
 *  \brief This Struct is frozen since ORT 1.13.0. Its maintained part of Legacy API for compatibility.
 *  \brief For latest OpenVINO Provider Options update to the ProviderOptions map.
 *  \brief Latest OpenVINO Provider Options are listed in the
 *  \htmlonly
 *  <a href="https://onnxruntime.ai/docs/execution-providers/OpenVINO-ExecutionProvider.html#summary-of-options">onnxruntime document.</a>
 *  \endhtmlonly
 * \see OrtApi::SessionOptionsAppendExecutionProvider()
 */
typedef struct OrtOpenVINOProviderOptions {
#ifdef __cplusplus
  OrtOpenVINOProviderOptions() : device_type{},
                                 enable_npu_fast_compile{},
                                 device_id{},
                                 num_of_threads{},
                                 cache_dir{},
                                 context{},
                                 enable_opencl_throttling{},
                                 enable_dynamic_shapes{} {}
#endif
  /** \brief Device type string
   *
   * Valid settings are one of: "CPU_FP32", "CPU_FP16", "GPU_FP32", "GPU_FP16"
   */
  const char* device_type;
  unsigned char enable_npu_fast_compile;  ///< 0 = disabled, nonzero = enabled
  const char* device_id;
  size_t num_of_threads;  ///< 0 = Use default number of threads
  const char* cache_dir;  // path is set to empty by default
  void* context;
  unsigned char enable_opencl_throttling;  ///< 0 = disabled, nonzero = enabled
  unsigned char enable_dynamic_shapes;     ///< 0 = disabled, nonzero = enabled
} OrtOpenVINOProviderOptions;

struct OrtApi;
typedef struct OrtApi OrtApi;

struct OrtTrainingApi;
typedef struct OrtTrainingApi OrtTrainingApi;

struct OrtModelEditorApi;
typedef struct OrtModelEditorApi OrtModelEditorApi;

/** \brief The helper interface to get the right version of OrtApi
 *
 * Get a pointer to this structure through ::OrtGetApiBase
 */
struct OrtApiBase {
  /** \brief Get a pointer to the requested version of the ::OrtApi
   *
   * \param[in] version Must be ::ORT_API_VERSION
   * \return The ::OrtApi for the version requested, nullptr will be returned if this version is unsupported, for example when using a runtime
   *   older than the version created with this header file.
   *
   * One can call GetVersionString() to get the version of the Onnxruntime library for logging
   * and error reporting purposes.
   */
  const OrtApi*(ORT_API_CALL* GetApi)(uint32_t version)NO_EXCEPTION;

  /** \brief Returns a null terminated string of the version of the Onnxruntime library (eg: "1.8.1")
   *
   *  \return UTF-8 encoded version string. Do not deallocate the returned buffer.
   */
  const char*(ORT_API_CALL* GetVersionString)(void)NO_EXCEPTION;
};

typedef struct OrtApiBase OrtApiBase;

/** \brief The Onnxruntime library's entry point to access the C API
 *
 * Call this to get the a pointer to an ::OrtApiBase
 */
ORT_EXPORT const OrtApiBase* ORT_API_CALL OrtGetApiBase(void) NO_EXCEPTION;

/** \brief Thread work loop function
 *
 * Onnxruntime will provide the working loop on custom thread creation
 * Argument is an onnxruntime built-in type which will be provided when thread pool calls OrtCustomCreateThreadFn
 */
typedef void (*OrtThreadWorkerFn)(void* ort_worker_fn_param);

typedef const struct OrtCustomHandleType {
  char __place_holder;
}* OrtCustomThreadHandle;

/** \brief Ort custom thread creation function
 *
 * The function should return a thread handle to be used in onnxruntime thread pools
 * Onnxruntime will throw exception on return value of nullptr or 0, indicating that the function failed to create a thread
 */
typedef OrtCustomThreadHandle (*OrtCustomCreateThreadFn)(void* ort_custom_thread_creation_options, OrtThreadWorkerFn ort_thread_worker_fn, void* ort_worker_fn_param);

/** \brief Custom thread join function
 *
 * Onnxruntime thread pool destructor will call the function to join a custom thread.
 * Argument ort_custom_thread_handle is the value returned by OrtCustomCreateThreadFn
 */
typedef void (*OrtCustomJoinThreadFn)(OrtCustomThreadHandle ort_custom_thread_handle);

typedef OrtStatus*(ORT_API_CALL* RegisterCustomOpsFn)(OrtSessionOptions* options, const OrtApiBase* api);

/** \brief Callback function for RunAsync
 *
 * \param[in] user_data User specific data that passed back to the callback
 * \param[out] outputs On succeed, outputs host inference results, on error, the value will be nullptr
 * \param[out] num_outputs Number of outputs, on error, the value will be zero
 * \param[out] status On error, status will provide details
 */
typedef void (*RunAsyncCallbackFn)(void* user_data, OrtValue** outputs, size_t num_outputs, OrtStatusPtr status);

/** \brief The C API
 *
 * All C API functions are defined inside this structure as pointers to functions.
 * Call OrtApiBase::GetApi to get a pointer to it
 *
 * \nosubgrouping
 */
struct OrtApi {
  /// \name OrtStatus
  /// @{

  /**
   * \brief Create an OrtStatus from a null terminated string
   *
   * \param[in] code
   * \param[in] msg A null-terminated string. Its contents will be copied.
   * \return A new OrtStatus object, must be destroyed with OrtApi::ReleaseStatus
   */
  OrtStatus*(ORT_API_CALL* CreateStatus)(OrtErrorCode code, _In_ const char* msg)NO_EXCEPTION ORT_ALL_ARGS_NONNULL;

  /** \brief Get OrtErrorCode from OrtStatus
   *
   * \param[in] status
   * \return OrtErrorCode that \p status was created with
   */
  OrtErrorCode(ORT_API_CALL* GetErrorCode)(_In_ const OrtStatus* status) NO_EXCEPTION ORT_ALL_ARGS_NONNULL;

  /** \brief Get error string from OrtStatus
   *
   * \param[in] status
   * \return The error message inside the `status`. Do not free the returned value.
   */
  const char*(ORT_API_CALL* GetErrorMessage)(_In_ const OrtStatus* status)NO_EXCEPTION ORT_ALL_ARGS_NONNULL;

  /// @}
  /// \name OrtEnv
  /// @{

  /** \brief Create an OrtEnv
   *
   * \note Invoking this function will return the same instance of the environment as that returned by a previous call
   * to another env creation function; all arguments to this function will be ignored.
   * \param[in] log_severity_level The log severity level.
   * \param[in] logid The log identifier.
   * \param[out] out Returned newly created OrtEnv. Must be freed with OrtApi::ReleaseEnv
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateEnv, OrtLoggingLevel log_severity_level, _In_ const char* logid, _Outptr_ OrtEnv** out);

  /** \brief Create an OrtEnv
   *
   * \note Invoking this function will return the same instance of the environment as that returned by a previous call
   * to another env creation function; all arguments to this function will be ignored. If you want to provide your
   * own logging function, consider setting it using the SetUserLoggingFunction API instead.
   * \param[in] logging_function A pointer to a logging function.
   * \param[in] logger_param A pointer to arbitrary data passed as the ::OrtLoggingFunction `param` parameter to
   *                         `logging_function`. This parameter is optional.
   * \param[in] log_severity_level The log severity level.
   * \param[in] logid The log identifier.
   * \param[out] out Returned newly created OrtEnv. Must be freed with OrtApi::ReleaseEnv
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateEnvWithCustomLogger, _In_ OrtLoggingFunction logging_function, _In_opt_ void* logger_param,
                  _In_ OrtLoggingLevel log_severity_level, _In_ const char* logid, _Outptr_ OrtEnv** out);

  /** \brief Enable Telemetry
   *
   * \note Telemetry events are on by default since they are lightweight
   * \param[in] env
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(EnableTelemetryEvents, _In_ const OrtEnv* env);
  /** \brief Disable Telemetry
   *
   * \see OrtApi::EnableTelemetryEvents
   * \param[in] env
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(DisableTelemetryEvents, _In_ const OrtEnv* env);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief Create an OrtSession from a model file
   *
   * \param[in] env
   * \param[in] model_path
   * \param[in] options
   * \param[out] out Returned newly created OrtSession. Must be freed with OrtApi::ReleaseSession
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  // TODO: document the path separator convention? '/' vs '\'
  // TODO: should specify the access characteristics of model_path. Is this read only during the
  // execution of CreateSession, or does the OrtSession retain a handle to the file/directory
  // and continue to access throughout the OrtSession lifetime?
  //  What sort of access is needed to model_path : read or read/write?
  ORT_API2_STATUS(CreateSession, _In_ const OrtEnv* env, _In_ const ORTCHAR_T* model_path,
                  _In_ const OrtSessionOptions* options, _Outptr_ OrtSession** out);

  /** \brief Create an OrtSession from memory
   *
   * \param[in] env
   * \param[in] model_data
   * \param[in] model_data_length
   * \param[in] options
   * \param[out] out Returned newly created OrtSession. Must be freed with OrtApi::ReleaseSession
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSessionFromArray, _In_ const OrtEnv* env,
                  _In_ const void* model_data, size_t model_data_length,
                  _In_ const OrtSessionOptions* options, _Outptr_ OrtSession** out);

  /** \brief Run the model in an ::OrtSession
   *
   * Will not return until the model run has completed. Multiple threads might be used to run the model based on
   * the options in the ::OrtSession and settings used when creating the ::OrtEnv
   *
   * \param[in] session
   * \param[in] run_options If nullptr, will use a default ::OrtRunOptions
   * \param[in] input_names Array of null terminated UTF8 encoded strings of the input names
   * \param[in] inputs Array of ::OrtValue%s of the input values
   * \param[in] input_len Number of elements in the input_names and inputs arrays
   * \param[in] output_names Array of null terminated UTF8 encoded strings of the output names
   * \param[in] output_names_len Number of elements in the output_names and outputs array
   * \param[out] outputs Array of ::OrtValue%s that the outputs are stored in. This can also be
   *     an array of nullptr values, in this case ::OrtValue objects will be allocated and pointers
   *     to them will be set into the `outputs` array.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(Run, _Inout_ OrtSession* session, _In_opt_ const OrtRunOptions* run_options,
                  _In_reads_(input_len) const char* const* input_names,
                  _In_reads_(input_len) const OrtValue* const* inputs, size_t input_len,
                  _In_reads_(output_names_len) const char* const* output_names, size_t output_names_len,
                  _Inout_updates_all_(output_names_len) OrtValue** outputs);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Create an ::OrtSessionOptions object
   *
   * To use additional providers, you must build ORT with the extra providers enabled. Then call one of these
   * functions to enable them in the session:<br>
   *   OrtSessionOptionsAppendExecutionProvider_CPU<br>
   *   OrtSessionOptionsAppendExecutionProvider_CUDA<br>
   *   OrtSessionOptionsAppendExecutionProvider_(remaining providers...)<br>
   * The order they are called indicates the preference order as well. In other words call this method
   * on your most preferred execution provider first followed by the less preferred ones.
   * If none are called Ort will use its internal CPU execution provider.
   *
   * \param[out] options The newly created OrtSessionOptions. Must be freed with OrtApi::ReleaseSessionOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSessionOptions, _Outptr_ OrtSessionOptions** options);

  /** \brief Set filepath to save optimized model after graph level transformations
   *
   * \param[in] options
   * \param[in] optimized_model_filepath
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetOptimizedModelFilePath, _Inout_ OrtSessionOptions* options,
                  _In_ const ORTCHAR_T* optimized_model_filepath);

  /** \brief Create a copy of an existing ::OrtSessionOptions
   *
   * \param[in] in_options OrtSessionOptions to copy
   * \param[out] out_options Returned newly created ::OrtSessionOptions. Must be freed with OrtApi::ReleaseSessionOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CloneSessionOptions, _In_ const OrtSessionOptions* in_options,
                  _Outptr_ OrtSessionOptions** out_options);

  /** \brief Set execution mode
   *
   * Controls whether you want to execute operators in your graph sequentially or in parallel. Usually when the model
   *  has many branches, setting this option to ExecutionMode.ORT_PARALLEL will give you better performance.
   *  See [docs/ONNX_Runtime_Perf_Tuning.md] for more details.
   *
   * \param[in] options
   * \param[in] execution_mode
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetSessionExecutionMode, _Inout_ OrtSessionOptions* options, ExecutionMode execution_mode);

  /** \brief Enable profiling for a session
   *
   * \param[in] options
   * \param[in] profile_file_prefix
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(EnableProfiling, _Inout_ OrtSessionOptions* options, _In_ const ORTCHAR_T* profile_file_prefix);

  /** \brief Disable profiling for a session
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(DisableProfiling, _Inout_ OrtSessionOptions* options);

  /** \brief Enable the memory pattern optimization
   *
   * The idea is if the input shapes are the same, we could trace the internal memory allocation
   * and generate a memory pattern for future request. So next time we could just do one allocation
   * with a big chunk for all the internal memory allocation.
   * \note Memory pattern optimization is only available when Sequential Execution mode is enabled (see OrtApi::SetSessionExecutionMode)
   *
   * \see OrtApi::DisableMemPattern
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(EnableMemPattern, _Inout_ OrtSessionOptions* options);

  /** \brief Disable the memory pattern optimization
   *
   * \see OrtApi::EnableMemPattern
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(DisableMemPattern, _Inout_ OrtSessionOptions* options);

  /** \brief Enable the memory arena on CPU
   *
   * Arena may pre-allocate memory for future usage.
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(EnableCpuMemArena, _Inout_ OrtSessionOptions* options);

  /** \brief Disable the memory arena on CPU
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(DisableCpuMemArena, _Inout_ OrtSessionOptions* options);

  /** \brief Set session log id
   *
   * \param[in] options
   * \param[in] logid The log identifier.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetSessionLogId, _Inout_ OrtSessionOptions* options, const char* logid);

  /** \brief Set session log verbosity level
   *
   * Applies to session load, initialization, etc
   *
   * \param[in] options
   * \param[in] session_log_verbosity_level \snippet{doc} snippets.dox Log Verbosity Level
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetSessionLogVerbosityLevel, _Inout_ OrtSessionOptions* options, int session_log_verbosity_level);

  /** \brief Set session log severity level
   *
   * \param[in] options
   * \param[in] session_log_severity_level The log severity level (refer to ::OrtLoggingLevel for possible values).
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetSessionLogSeverityLevel, _Inout_ OrtSessionOptions* options, int session_log_severity_level);

  /** \brief Set the optimization level to apply when loading a graph
   *
   * Please see https://onnxruntime.ai/docs/performance/model-optimizations/graph-optimizations.html for an in-depth explanation
   * \param[in,out] options The session options object
   * \param[in] graph_optimization_level The optimization level
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetSessionGraphOptimizationLevel, _Inout_ OrtSessionOptions* options,
                  GraphOptimizationLevel graph_optimization_level);

  /** \brief Sets the number of threads used to parallelize the execution within nodes
   *
   * When running a single node operation, ex. add, this sets the maximum number of threads to use.
   *
   * \note If built with OpenMP, this has no effect on the number of threads used. In this case
   *       use the OpenMP env variables to configure the number of intra op num threads.
   *
   * \param[in] options
   * \param[in] intra_op_num_threads Number of threads to use<br>
   *   A value of 0 will use the default number of threads<br>
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetIntraOpNumThreads, _Inout_ OrtSessionOptions* options, int intra_op_num_threads);

  /** \brief Sets the number of threads used to parallelize the execution of the graph
   *
   * If nodes can be run in parallel, this sets the maximum number of threads to use to run them in parallel.
   *
   * \note If sequential execution is enabled this value is ignored, it acts as if it was set to 1.
   *
   * \param[in] options
   * \param[in] inter_op_num_threads Number of threads to use<br>
   *   A value of 0 will use the default number of threads<br>
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetInterOpNumThreads, _Inout_ OrtSessionOptions* options, int inter_op_num_threads);

  /// @}
  /// \name OrtCustomOpDomain
  /// @{

  /** \brief Create a custom op domain
   *
   * \param[in] domain
   * \param[out] out Newly created domain. Must be freed with OrtApi::ReleaseCustomOpDomain
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateCustomOpDomain, _In_ const char* domain, _Outptr_ OrtCustomOpDomain** out);

  /** \brief Add a custom op to a custom op domain
   *
   * \note The OrtCustomOp* pointer must remain valid until the ::OrtCustomOpDomain using it is released
   *
   * \param[in] custom_op_domain
   * \param[in] op
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CustomOpDomain_Add, _Inout_ OrtCustomOpDomain* custom_op_domain, _In_ const OrtCustomOp* op);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Add custom op domain to a session options
   *
   * \note The OrtCustomOpDomain* must not be deleted until all sessions using it are released
   *
   * \param[in] options
   * \param[in] custom_op_domain
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(AddCustomOpDomain, _Inout_ OrtSessionOptions* options, _In_ OrtCustomOpDomain* custom_op_domain);

  /** \deprecated Use OrtApi::RegisterCustomOpsLibrary_V2.
   *
   * Registers custom ops from a shared library.
   *
   * Loads a shared library (dll on windows, so on linux, etc) named 'library_path' and looks for this entry point:
   *		OrtStatus* RegisterCustomOps(OrtSessionOptions * options, const OrtApiBase* api);
   * It then passes in the provided session options to this function along with the api base.
   * The handle to the loaded library is returned in library_handle. It can be freed by the caller after all sessions using the passed in
   * session options are destroyed, or if an error occurs and it is non null.
   *
   * \param[in] options
   * \param[in] library_path
   * \param[out] library_handle OS specific handle to the loaded library (Use FreeLibrary on Windows, dlclose on Linux, etc.. to unload)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RegisterCustomOpsLibrary, _Inout_ OrtSessionOptions* options, _In_ const char* library_path, _Outptr_ void** library_handle);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief Get input count for a session
   *
   * This number must also match the number of inputs passed to OrtApi::Run
   *
   * \see OrtApi::SessionGetInputTypeInfo, OrtApi::SessionGetInputName, OrtApi::Session
   *
   * \param[in] session
   * \param[out] out Number of inputs
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetInputCount, _In_ const OrtSession* session, _Out_ size_t* out);

  /** \brief Get output count for a session
   *
   * This number must also match the number of outputs returned by OrtApi::Run
   *
   * \see OrtApi::SessionGetOutputTypeInfo, OrtApi::SessionGetOutputName, OrtApi::Session
   *
   * \param[in] session
   * \param[out] out Number of outputs
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOutputCount, _In_ const OrtSession* session, _Out_ size_t* out);

  /** \brief Get overridable initializer count
   *
   * \see OrtApi::SessionGetOverridableInitializerTypeInfo, OrtApi::SessionGetOverridableInitializerName
   *
   * \param[in] session
   * \param[in] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOverridableInitializerCount, _In_ const OrtSession* session, _Out_ size_t* out);

  /** \brief Get input type information
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetInputCount returns (exclusive)
   * \param[out] type_info Must be freed with OrtApi::ReleaseTypeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetInputTypeInfo, _In_ const OrtSession* session, size_t index, _Outptr_ OrtTypeInfo** type_info);

  /** \brief Get output type information
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetOutputCount returns (exclusive)
   * \param[out] type_info Must be freed with OrtApi::ReleaseTypeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOutputTypeInfo, _In_ const OrtSession* session, size_t index, _Outptr_ OrtTypeInfo** type_info);

  /** \brief Get overridable initializer type information
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetOverridableInitializerCount returns (exclusive)
   * \param[out] type_info Must be freed with OrtApi::ReleaseTypeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOverridableInitializerTypeInfo, _In_ const OrtSession* session, size_t index, _Outptr_ OrtTypeInfo** type_info);

  /** \brief Get input name
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetInputCount returns (exclusive)
   * \param[in] allocator
   * \param[out] value Set to a null terminated UTF-8 encoded string allocated using `allocator`. Must be freed using `allocator`.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetInputName, _In_ const OrtSession* session, size_t index, _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /** \brief Get output name
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetOutputCount returns (exclusive)
   * \param[in] allocator
   * \param[out] value Set to a null terminated UTF-8 encoded string allocated using `allocator`. Must be freed using `allocator`.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOutputName, _In_ const OrtSession* session, size_t index, _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /** \brief Get overridable initializer name
   *
   * \param[in] session
   * \param[in] index Must be between 0 (inclusive) and what OrtApi::SessionGetOverridableInitializerCount returns (exclusive)
   * \param[in] allocator
   * \param[out] value Set to a null terminated UTF-8 encoded string allocated using `allocator`. Must be freed using `allocator`.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetOverridableInitializerName, _In_ const OrtSession* session, size_t index,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /// @}
  /// \name OrtRunOptions
  /// @{

  /** \brief Create an OrtRunOptions
   *
   * \param[out] out Returned newly created ::OrtRunOptions. Must be freed with OrtApi::ReleaseRunOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateRunOptions, _Outptr_ OrtRunOptions** out);

  /** \brief Set per-run log verbosity level
   *
   * \see OrtApi::RunOptionsGetRunLogVerbosityLevel
   *
   * \param[in] options
   * \param[in] log_verbosity_level \snippet{doc} snippets.dox Log Verbosity Level
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RunOptionsSetRunLogVerbosityLevel, _Inout_ OrtRunOptions* options, int log_verbosity_level);

  /** \brief Set per-run log severity level
   *
   * \see OrtApi::RunOptionsGetRunLogSeverityLevel
   *
   * \param[in] options
   * \param[in] log_severity_level The log severity level (refer to ::OrtLoggingLevel for possible values).
   */
  ORT_API2_STATUS(RunOptionsSetRunLogSeverityLevel, _Inout_ OrtRunOptions* options, int log_severity_level);

  /** \brief Set per-run tag
   *
   * This is used in a per-run log identifier.
   *
   * \see OrtApi::RunOptionsGetRunTag
   *
   * \param[in] options
   * \param[in] run_tag The run tag.
   */
  ORT_API2_STATUS(RunOptionsSetRunTag, _Inout_ OrtRunOptions* options, _In_ const char* run_tag);

  /** \brief Get per-run log verbosity level
   *
   * \see OrtApi::RunOptionsSetRunLogVerbosityLevel
   *
   * \param[in] options
   * \param[out] log_verbosity_level \snippet{doc} snippets.dox Log Verbosity Level
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RunOptionsGetRunLogVerbosityLevel, _In_ const OrtRunOptions* options,
                  _Out_ int* log_verbosity_level);

  /** \brief Get per-run log severity level
   *
   * \see OrtApi::RunOptionsSetRunLogSeverityLevel
   *
   * \param[in] options
   * \param[out] log_severity_level The log severity level (refer to ::OrtLoggingLevel for possible values).
   */
  ORT_API2_STATUS(RunOptionsGetRunLogSeverityLevel, _In_ const OrtRunOptions* options, _Out_ int* log_severity_level);

  /** \brief Get per-run tag
   *
   * This is used in a per-run log identifier.
   *
   * \see OrtApi::RunOptionsSetRunTag
   *
   * \param[in] options
   * \param[out] run_tag The run tag.
   *                     Do not free this value, it is owned by `options`. It will be invalidated if the run tag
   *                     changes (i.e., with OrtApi::RunOptionsSetRunTag) or `options` is freed.
   */
  ORT_API2_STATUS(RunOptionsGetRunTag, _In_ const OrtRunOptions* options, _Out_ const char** run_tag);

  /** \brief Set terminate flag
   *
   * If a currently executing session needs to be force terminated, this can be called from another thread to force it to fail with an error.
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RunOptionsSetTerminate, _Inout_ OrtRunOptions* options);

  /** \brief Clears the terminate flag
   *
   * Used so the OrtRunOptions instance can be used in a new OrtApi::Run call without it instantly terminating
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RunOptionsUnsetTerminate, _Inout_ OrtRunOptions* options);

  /// @}
  /// \name OrtValue
  /// @{

  /** \brief Create a tensor
   *
   * Create a tensor using a supplied ::OrtAllocator
   *
   * \param[in] allocator
   * \param[in] shape Pointer to the tensor shape dimensions.
   * \param[in] shape_len The number of tensor shape dimensions.
   * \param[in] type
   * \param[out] out Returns newly created ::OrtValue. Must be freed with OrtApi::ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateTensorAsOrtValue, _Inout_ OrtAllocator* allocator, _In_ const int64_t* shape, size_t shape_len,
                  ONNXTensorElementDataType type, _Outptr_ OrtValue** out);

  /** \brief Create a tensor backed by a user supplied buffer
   *
   * Create a tensor with user's buffer. You can fill the buffer either before calling this function or after.
   * p_data is owned by caller. ReleaseValue won't release p_data.
   *
   * If you wish to transfer ownership of p_data to ORT use CreateTensorWithDataAndDeleterAsOrtValue.
   *
   * \param[in] info Memory description of where the p_data buffer resides (CPU vs GPU etc).
   * \param[in] p_data Pointer to the data buffer.
   * \param[in] p_data_len The number of bytes in the data buffer.
   * \param[in] shape Pointer to the tensor shape dimensions.
   * \param[in] shape_len The number of tensor shape dimensions.
   * \param[in] type The data type.
   * \param[out] out Returns newly created ::OrtValue. Must be freed with OrtApi::ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateTensorWithDataAsOrtValue, _In_ const OrtMemoryInfo* info, _Inout_ void* p_data,
                  size_t p_data_len, _In_ const int64_t* shape, size_t shape_len, ONNXTensorElementDataType type,
                  _Outptr_ OrtValue** out);

  /** \brief Return if an ::OrtValue is a tensor type
   *
   * \param[in] value A tensor type (string tensors are not supported)
   * \param[out] out Set to 1 iff ::OrtValue is a tensor, 0 otherwise
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(IsTensor, _In_ const OrtValue* value, _Out_ int* out);

  /** \brief Get a pointer to the raw data inside a tensor
   *
   * Used to read/write/modify the internal tensor data directly.
   * \note The returned pointer is valid until the \p value is destroyed.
   *
   * \param[in] value A tensor type (string tensors are not supported)
   * \param[out] out Filled in with a pointer to the internal storage
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorMutableData, _In_ OrtValue* value, _Outptr_ void** out);

  /** \brief Set all strings at once in a string tensor
   *
   * \param[in,out] value A tensor of type ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING
   * \param[in] s An array of strings. Each string in this array must be null terminated.
   * \param[in] s_len Count of strings in s (Must match the size of \p value's tensor shape)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(FillStringTensor, _Inout_ OrtValue* value, _In_ const char* const* s, size_t s_len);

  /** \brief Get total byte length for all strings in a string tensor
   *
   * Typically used with OrtApi::GetStringTensorContent
   *
   * \param[in] value A tensor of type ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING
   * \param[out] len Total byte length of all strings (does not include trailing nulls)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetStringTensorDataLength, _In_ const OrtValue* value, _Out_ size_t* len);

  /** \brief Get all strings from a string tensor
   *
   * An example of the results:<br>
   * Given \p value is a string tensor with the strings { "This" "is" "a" "test" }<br>
   * \p s must have a size of 11 bytes<br>
   * \p offsets must have 4 elements<br>
   * After the call, these values will be filled in:<br>
   * \p s will contain "Thisisatest"<br>
   * \p offsets will contain { 0, 4, 6, 7 }<br>
   * The length of the last string is just s_len - offsets[last]
   *
   * \param[in] value A tensor of type ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING
   * \param[in] s Buffer to sequentially write all tensor strings to. Each string is NOT null-terminated.
   * \param[in] s_len Number of bytes of buffer pointed to by \p s (Get it from OrtApi::GetStringTensorDataLength)
   * \param[out] offsets Array of start offsets into the strings written to \p s
   * \param[in] offsets_len Number of elements in offsets
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetStringTensorContent, _In_ const OrtValue* value, _Out_writes_bytes_all_(s_len) void* s,
                  size_t s_len, _Out_writes_all_(offsets_len) size_t* offsets, size_t offsets_len);

  /// @}
  /// \name OrtTypeInfo
  /// @{

  /** \brief Get ::OrtTensorTypeAndShapeInfo from an ::OrtTypeInfo
   *
   * \param[in] type_info
   * \param[out] out Do not free this value, it will be valid until type_info is freed.
   *             If type_info does not represent tensor, this value will be set to nullptr.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CastTypeInfoToTensorInfo, _In_ const OrtTypeInfo* type_info,
                  _Outptr_result_maybenull_ const OrtTensorTypeAndShapeInfo** out);

  /** \brief Get ::ONNXType from ::OrtTypeInfo
   *
   * \param[in] type_info
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetOnnxTypeFromTypeInfo, _In_ const OrtTypeInfo* type_info, _Out_ enum ONNXType* out);

  /// @}
  /// \name OrtTensorTypeAndShapeInfo
  /// @{

  /** \brief Create an ::OrtTensorTypeAndShapeInfo object
   *
   * \param[out] out Returns newly created ::OrtTensorTypeAndShapeInfo. Must be freed with OrtApi::ReleaseTensorTypeAndShapeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateTensorTypeAndShapeInfo, _Outptr_ OrtTensorTypeAndShapeInfo** out);

  /** \brief Set element type in ::OrtTensorTypeAndShapeInfo
   *
   * \param[in] info
   * \param[in] type
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetTensorElementType, _Inout_ OrtTensorTypeAndShapeInfo* info, enum ONNXTensorElementDataType type);

  /** \brief Set shape information in ::OrtTensorTypeAndShapeInfo
   *
   * \param[in] info
   * \param[in] dim_values Array with `dim_count` elements. Can contain negative values.
   * \param[in] dim_count Number of elements in `dim_values`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetDimensions, OrtTensorTypeAndShapeInfo* info, _In_ const int64_t* dim_values, size_t dim_count);

  /** \brief Get element type in ::OrtTensorTypeAndShapeInfo
   *
   * \see OrtApi::SetTensorElementType
   *
   * \param[in] info
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorElementType, _In_ const OrtTensorTypeAndShapeInfo* info,
                  _Out_ enum ONNXTensorElementDataType* out);

  /** \brief Get dimension count in ::OrtTensorTypeAndShapeInfo
   *
   * \see OrtApi::GetDimensions
   *
   * \param[in] info
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetDimensionsCount, _In_ const OrtTensorTypeAndShapeInfo* info, _Out_ size_t* out);

  /** \brief Get dimensions in ::OrtTensorTypeAndShapeInfo
   *
   * \param[in] info
   * \param[out] dim_values Array with `dim_values_length` elements. On return, filled with the dimensions stored in the ::OrtTensorTypeAndShapeInfo
   * \param[in] dim_values_length Number of elements in `dim_values`. Use OrtApi::GetDimensionsCount to get this value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetDimensions, _In_ const OrtTensorTypeAndShapeInfo* info, _Out_ int64_t* dim_values,
                  size_t dim_values_length);

  /** \brief Get symbolic dimension names in ::OrtTensorTypeAndShapeInfo
   *
   * \param[in] info
   * \param[in] dim_params Array with `dim_params_length` elements. On return filled with pointers to null terminated strings of the dimension names
   * \param[in] dim_params_length Number of elements in `dim_params`. Use OrtApi::GetDimensionsCount to get this value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSymbolicDimensions, _In_ const OrtTensorTypeAndShapeInfo* info,
                  _Out_writes_all_(dim_params_length) const char* dim_params[], size_t dim_params_length);

  /** \brief Get total number of elements in a tensor shape from an ::OrtTensorTypeAndShapeInfo
   *
   * Return the number of elements specified by the tensor shape (all dimensions multiplied by each other).
   * For 0 dimensions, 1 is returned. If any dimension is less than 0, the result is always -1.
   *
   * Examples:<br>
   * [] = 1<br>
   * [1,3,4] = 12<br>
   * [2,0,4] = 0<br>
   * [-1,3,4] = -1<br>
   *
   * \param[in] info
   * \param[out] out Number of elements
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorShapeElementCount, _In_ const OrtTensorTypeAndShapeInfo* info, _Out_ size_t* out);

  /// @}
  /// \name OrtValue
  /// @{

  /** \brief Get type and shape information from a tensor ::OrtValue
   *
   * \param[in] value Must be a tensor (not a map/sequence/etc) or will return failure
   * \param[out] out Newly created ::OrtTensorTypeAndShapeInfo. Must be freed with OrtApi::ReleaseTensorTypeAndShapeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorTypeAndShape, _In_ const OrtValue* value, _Outptr_ OrtTensorTypeAndShapeInfo** out);

  /** \brief Get type information of an OrtValue
   *
   * \param[in] value
   * \param[out] out Newly created ::OrtTypeInfo. Must be freed with OrtApi::ReleaseTypeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTypeInfo, _In_ const OrtValue* value, _Outptr_result_maybenull_ OrtTypeInfo** out);

  /** \brief Get ONNXType of an ::OrtValue
   *
   * \param[in] value
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetValueType, _In_ const OrtValue* value, _Out_ enum ONNXType* out);

  /// @}
  /// \name OrtMemoryInfo
  /// @{

  /** \brief Create an ::OrtMemoryInfo
   *
   * \param[in] name
   * \param[in] type
   * \param[in] id
   * \param[in] mem_type
   * \param[out] out Newly created ::OrtMemoryInfo. Must be freed with OrtAPi::ReleaseMemoryInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateMemoryInfo, _In_ const char* name, enum OrtAllocatorType type, int id,
                  enum OrtMemType mem_type, _Outptr_ OrtMemoryInfo** out);

  /** \brief Create an ::OrtMemoryInfo for CPU memory
   *
   * Special case version of OrtApi::CreateMemoryInfo for CPU based memory. Same as using OrtApi::CreateMemoryInfo with name = "Cpu" and id = 0.
   *
   * \param[in] type
   * \param[in] mem_type
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateCpuMemoryInfo, enum OrtAllocatorType type, enum OrtMemType mem_type,
                  _Outptr_ OrtMemoryInfo** out);

  /** \brief Compare ::OrtMemoryInfo objects for equality
   *
   * Compares all settings of each ::OrtMemoryInfo for equality
   *
   * \param[in] info1
   * \param[in] info2
   * \param[out] out Set to 0 if equal, -1 if not equal
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CompareMemoryInfo, _In_ const OrtMemoryInfo* info1, _In_ const OrtMemoryInfo* info2, _Out_ int* out);

  /** \brief Get name from ::OrtMemoryInfo
   *
   * \param[in] ptr
   * \param[out] out Writes null terminated string to this pointer. Do NOT free the returned pointer. It is valid for the lifetime of the ::OrtMemoryInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(MemoryInfoGetName, _In_ const OrtMemoryInfo* ptr, _Out_ const char** out);

  /** \brief Get the id from ::OrtMemoryInfo
   */
  ORT_API2_STATUS(MemoryInfoGetId, _In_ const OrtMemoryInfo* ptr, _Out_ int* out);

  /** \brief Get the ::OrtMemType from ::OrtMemoryInfo
   */
  ORT_API2_STATUS(MemoryInfoGetMemType, _In_ const OrtMemoryInfo* ptr, _Out_ OrtMemType* out);

  /** \brief Get the ::OrtAllocatorType from ::OrtMemoryInfo
   */
  ORT_API2_STATUS(MemoryInfoGetType, _In_ const OrtMemoryInfo* ptr, _Out_ OrtAllocatorType* out);

  /// @}
  /// \name OrtAllocator
  /// @{

  /// \brief Calls OrtAllocator::Alloc function
  ORT_API2_STATUS(AllocatorAlloc, _Inout_ OrtAllocator* ort_allocator, size_t size, _Outptr_ void** out);
  /// \brief Calls OrtAllocator::Free function
  ORT_API2_STATUS(AllocatorFree, _Inout_ OrtAllocator* ort_allocator, void* p);
  /// \brief Calls OrtAllocator::Info function
  ORT_API2_STATUS(AllocatorGetInfo, _In_ const OrtAllocator* ort_allocator, _Outptr_ const struct OrtMemoryInfo** out);

  /** \brief Get the default allocator
   *
   * The default allocator is a CPU based, non-arena. Always returns the same pointer to the same default allocator.
   *
   * \param[out] out Returned value should NOT be freed
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetAllocatorWithDefaultOptions, _Outptr_ OrtAllocator** out);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Override session symbolic dimensions
   *
   * Override symbolic dimensions (by specific denotation strings) with actual values if known at session initialization time to enable
   * optimizations that can take advantage of fixed values (such as memory planning, etc)
   *
   * \param[in] options
   * \param[in] dim_denotation
   * \param[in] dim_value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(AddFreeDimensionOverride, _Inout_ OrtSessionOptions* options, _In_ const char* dim_denotation,
                  _In_ int64_t dim_value);

  /// @}
  /// \name OrtValue
  /// @{

  /* Internal information (not seen in Doxygen)
   *
   * APIs to support non-tensor types - map and sequence.
   * Currently only the following types are supported
   * Note: the following types should be kept in sync with data_types.h
   * Map types
   * =========
   * std::map<std::string, std::string>
   * std::map<std::string, int64_t>
   * std::map<std::string, float>
   * std::map<std::string, double>
   * std::map<int64_t, std::string>
   * std::map<int64_t, int64_t>
   * std::map<int64_t, float>
   * std::map<int64_t, double>
   *
   * Sequence types
   * ==============
   * std::vector<std::string>
   * std::vector<int64_t>
   * std::vector<float>
   * std::vector<double>
   * std::vector<std::map<std::string, float>>
   * std::vector<std::map<int64_t, float>
   */

  /** \brief Get non tensor data from an ::OrtValue
   *
   * If `value` is of type ONNX_TYPE_MAP, you need to retrieve the keys and values
   * separately. Use index=0 to retrieve keys and index=1 to retrieve values.
   * If `value` is of type ONNX_TYPE_SEQUENCE, use index to retrieve the index'th element
   * of the sequence.
   *
   * \param[in] value
   * \param[in] index See above for usage based on `value` type
   * \param[in] allocator Allocator used to allocate ::OrtValue
   * \param[out] out Created ::OrtValue that holds the element requested. Must be freed with OrtApi::ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetValue, _In_ const OrtValue* value, int index, _Inout_ OrtAllocator* allocator,
                  _Outptr_ OrtValue** out);

  /** \brief Get non tensor value count from an ::OrtValue
   *
   * If `value` is of type ONNX_TYPE_MAP 2 will always be returned. For ONNX_TYPE_SEQUENCE
   * the number of elements in the sequence will be returned
   *
   * \param[in] value
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetValueCount, _In_ const OrtValue* value, _Out_ size_t* out);

  /** \brief Create a map or sequence ::OrtValue
   *
   * To construct a map (ONNX_TYPE_MAP), use num_values = 2 and `in` should be an array of 2 ::OrtValue%s
   * representing keys and values.<br>
   *
   * To construct a sequence (ONNX_TYPE_SEQUENCE), use num_values = N where N is the number of the elements in the
   * sequence. 'in' should be an array of N ::OrtValue%s.
   *
   * \param[in] in See above for details
   * \param[in] num_values
   * \param[in] value_type Must be either ONNX_TYPE_MAP or ONNX_TYPE_SEQUENCE
   * \param[out] out Newly created ::OrtValue. Must be freed with OrtApi::ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateValue, _In_reads_(num_values) const OrtValue* const* in, size_t num_values,
                  enum ONNXType value_type, _Outptr_ OrtValue** out);

  /** \brief Create an opaque (custom user defined type) ::OrtValue
   *
   * Constructs an ::OrtValue that contains a value of non-standard type created for
   * experiments or while awaiting standardization. ::OrtValue in this case would contain
   * an internal representation of the Opaque type. Opaque types are distinguished from
   * each other by two strings 1) domain and 2) type name. The combination of the two
   * must be unique, so the type representation is properly identified internally. The combination
   * must be properly registered from within ORT at both compile/run time or by another API.
   *
   * To construct the ::OrtValue pass domain and type names, also a pointer to a data container
   * the type of which must be known to both ORT and the client program. That data container may or may
   * not match the internal representation of the Opaque type. The sizeof(data_container) is passed for
   * verification purposes.
   *
   * \param[in] domain_name Null terminated string of the domain name
   * \param[in] type_name Null terminated string of the type name
   * \param[in] data_container User pointer Data to populate ::OrtValue
   * \param[in] data_container_size Size in bytes of what `data_container` points to
   * \param[out] out Newly created ::OrtValue. Must be freed with OrtApi::ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateOpaqueValue, _In_z_ const char* domain_name, _In_z_ const char* type_name,
                  _In_ const void* data_container, size_t data_container_size, _Outptr_ OrtValue** out);

  /** \brief Get internal data from an opaque (custom user defined type) ::OrtValue
   *
   * Copies internal data from an opaque value into a user provided buffer
   *
   * \see OrtApi::CreateOpaqueValue
   *
   * \param[in] domain_name Null terminated string of the domain name
   * \param[in] type_name Null terminated string of the type name
   * \param[in] in The opaque ::OrtValue
   * \param[out] data_container Buffer to copy data into
   * \param[out] data_container_size Size in bytes of the buffer pointed to by data_container. Must match the size of the internal buffer.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetOpaqueValue, _In_ const char* domain_name, _In_ const char* type_name, _In_ const OrtValue* in,
                  _Out_ void* data_container, size_t data_container_size);

  /// @}
  /// \name OrtKernelInfo
  /// Custom operator APIs.
  /// @{

  /** \brief Get a float stored as an attribute in the graph node
   *
   * \param[in] info ::OrtKernelInfo instance
   * \param[in] name Null terminated string of the name of the attribute
   * \param[out] out Pointer to memory where the attribute will be stored
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttribute_float, _In_ const OrtKernelInfo* info, _In_ const char* name,
                  _Out_ float* out);

  /** \brief Fetch a 64-bit int stored as an attribute in the graph node
   *
   * \param[in] info ::OrtKernelInfo instance
   * \param[in] name Null terminated string of the name of the attribute
   * \param[out] out Pointer to memory where the attribute will be stored
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttribute_int64, _In_ const OrtKernelInfo* info, _In_ const char* name,
                  _Out_ int64_t* out);

  /** \brief Fetch a string stored as an attribute in the graph node
   *
   * If `out` is nullptr, the value of `size` is set to the true size of the string
   * attribute, and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the actual string attribute's size,
   * the value of `size` is set to the true size of the string attribute, the provided memory
   * is filled with the attribute's contents, and a success status is returned.
   *
   * If the `size` parameter is less than the actual string attribute's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the string attribute
   * and a failure status is returned.)
   *
   * \param[in] info ::OrtKernelInfo instance
   * \param[in] name Null terminated string of the name of the attribute
   * \param[out] out Pointer to memory where the attribute will be stored
   * \param[in,out] size See above comments for details
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttribute_string, _In_ const OrtKernelInfo* info, _In_ const char* name, _Out_ char* out,
                  _Inout_ size_t* size);

  /// @}
  /// \name OrtKernelContext
  /// Custom operator APIs.
  /// @{

  /** \brief Used for custom operators, get the input count of a kernel
   *
   * \see ::OrtCustomOp
   */
  ORT_API2_STATUS(KernelContext_GetInputCount, _In_ const OrtKernelContext* context, _Out_ size_t* out);

  /** \brief Used for custom operators, get the output count of a kernel
   *
   * \see ::OrtCustomOp
   */
  ORT_API2_STATUS(KernelContext_GetOutputCount, _In_ const OrtKernelContext* context, _Out_ size_t* out);

  /** \brief Used for custom operators, get an input of a kernel
   *
   * The function attempts fetches the input of the kernel. If the input is optional
   * and not present, the function returns success and out is set to nullptr.
   *
   * \param[in] context ::OrtKernelContext instance
   * \param[in] index See KernelContext_GetInputCount for boundaries check.
   * \param[out] out OrtValue if the input is present otherwise is set nullptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelContext_GetInput, _In_ const OrtKernelContext* context, _In_ size_t index,
                  _Out_ const OrtValue** out);

  /** \brief Used for custom operators, get an output of a kernel
   *
   * The function attempts fetches the output of the kernel. If the output is optional
   * and not present, the function returns success and out is set to nullptr.
   *
   * \param[in] context ::OrtKernelContext instance
   * \param[in] index See KernelContext_GetOutputCount for boundaries check.
   * \param[in] dim_values output dimensions
   * \param[in] dim_count number of dimensions
   * \param[out] out a ptr to OrtValue to output otherwise set to nullptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelContext_GetOutput, _Inout_ OrtKernelContext* context, _In_ size_t index,
                  _In_ const int64_t* dim_values, size_t dim_count, _Outptr_ OrtValue** out);

  /// @}
  /// \name OrtEnv
  /// @{
  ORT_CLASS_RELEASE(Env);
  /// @}
  /// \name OrtStatus
  /// @{
  ORT_CLASS_RELEASE(Status);
  /// @}
  /// \name OrtMemoryInfo
  /// @{
  ORT_CLASS_RELEASE(MemoryInfo);
  /// @}
  /// \name OrtSession
  /// @{
  ORT_CLASS_RELEASE(Session);  // Don't call ReleaseSession from Dllmain (because session owns a thread pool)
  /// @}
  /// \name OrtValue
  /// @{
  ORT_CLASS_RELEASE(Value);
  /// @}
  /// \name OrtRunOptions
  /// @{
  ORT_CLASS_RELEASE(RunOptions);
  /// @}
  /// \name OrtTypeInfo
  /// @{
  ORT_CLASS_RELEASE(TypeInfo);
  /// @}
  /// \name OrtTensorTypeAndShapeInfo
  /// @{
  ORT_CLASS_RELEASE(TensorTypeAndShapeInfo);
  /// @}
  /// \name OrtSessionOptions
  /// @{
  ORT_CLASS_RELEASE(SessionOptions);
  /// @}
  /// \name OrtCustomOpDomain
  /// @{
  ORT_CLASS_RELEASE(CustomOpDomain);

  /// @}
  /// \name OrtTypeInfo
  /// @{

  /** \brief Get denotation from type information
   *
   * Augments ::OrtTypeInfo to return denotations on the type.
   *
   * This is used by WinML to determine if an input/output is intended to be an Image or a Tensor.
   *
   * \param[in] type_info
   * \param[out] denotation Pointer to the null terminated denotation string is written to this pointer. This pointer is valid until the object is destroyed or the name is changed, do not free.
   * \param[out] len Length in bytes of the string returned in `denotation`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetDenotationFromTypeInfo, _In_ const OrtTypeInfo* type_info, _Out_ const char** const denotation,
                  _Out_ size_t* len);

  /** \brief Get detailed map information from an ::OrtTypeInfo
   *
   * This augments ::OrtTypeInfo to return an ::OrtMapTypeInfo when the type is a map.
   * The OrtMapTypeInfo has additional information about the map's key type and value type.
   *
   * This is used by WinML to support model reflection APIs.
   *
   * \param[out] type_info
   * \param[out] out A pointer to the ::OrtMapTypeInfo. Do not free this value. If type_info
   *             does not contain a map, this value will be set to nullptr.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CastTypeInfoToMapTypeInfo, _In_ const OrtTypeInfo* type_info,
                  _Outptr_result_maybenull_ const OrtMapTypeInfo** out);

  /** \brief Cast ::OrtTypeInfo to an ::OrtSequenceTypeInfo
   *
   * This api augments ::OrtTypeInfo to return an ::OrtSequenceTypeInfo when the type is a sequence.
   * The ::OrtSequenceTypeInfo has additional information about the sequence's element type.
   *
   * This is used by WinML to support model reflection APIs.
   *
   * \param[in] type_info
   * \param[out] out A pointer to the OrtSequenceTypeInfo. Do not free this value. If type_info
   *             doesn not contain a sequence, this value will be set to nullptr.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CastTypeInfoToSequenceTypeInfo, _In_ const OrtTypeInfo* type_info,
                  _Outptr_result_maybenull_ const OrtSequenceTypeInfo** out);

  /// @}
  /// \name OrtMapTypeInfo
  /// @{

  /** \brief Get key type from an ::OrtMapTypeInfo
   *
   * Key types are restricted to being scalar types.
   *
   * This is used by WinML to support model reflection APIs.
   *
   * \param[in] map_type_info
   * \param[out] out
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetMapKeyType, _In_ const OrtMapTypeInfo* map_type_info, _Out_ enum ONNXTensorElementDataType* out);

  /** \brief Get the value type from an ::OrtMapTypeInfo
   *
   * \param[in] map_type_info
   * \param[out] type_info A copy of the OrtTypeInfo for the map value type.
   *                       The user must free this value with ReleaseTypeInfo.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetMapValueType, _In_ const OrtMapTypeInfo* map_type_info, _Outptr_ OrtTypeInfo** type_info);

  /// @}
  /// \name OrtSequenceTypeInfo
  /// @{

  /** \brief Get element type from an ::OrtSequenceTypeInfo
   *
   * This is used by WinML to support model reflection APIs.
   *
   * \param[in] sequence_type_info
   * \param[out] type_info A copy of the OrtTypeInfo for the sequence element type.
   *                       The user must free this value with ReleaseTypeInfo.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSequenceElementType, _In_ const OrtSequenceTypeInfo* sequence_type_info,
                  _Outptr_ OrtTypeInfo** type_info);

  /// @}
  /// \name OrtMapTypeInfo
  /// @{
  ORT_CLASS_RELEASE(MapTypeInfo);
  /// @}
  /// \name OrtSequenceTypeInfo
  /// @{
  ORT_CLASS_RELEASE(SequenceTypeInfo);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief End profiling and return filename of the profile data
   *
   * Profiling is turned on through OrtApi::EnableProfiling
   *
   * \param[in] session
   * \param[in] allocator
   * \param[out] out Null terminated string of the filename, allocated using `allocator`. Must be freed using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionEndProfiling, _In_ OrtSession* session, _Inout_ OrtAllocator* allocator, _Outptr_ char** out);

  /** \brief Get ::OrtModelMetadata from an ::OrtSession
   *
   * \param[in] session
   * \param[out] out Newly created ::OrtModelMetadata. Must be freed using OrtApi::ReleaseModelMetadata
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetModelMetadata, _In_ const OrtSession* session, _Outptr_ OrtModelMetadata** out);

  /// @}
  /// \name OrtModelMetadata
  /// @{

  /** \brief Get `producer name` from an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[out] value Set to a null terminated string allocated using `allocator`. Must be freed using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetProducerName, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /** \brief Get `graph name` from an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[out] value Set to a null terminated string allocated using `allocator`. Must be freed using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetGraphName, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /** \brief Get `domain` from an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[out] value Set to a null terminated string allocated using `allocator`. Must be freed using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetDomain, _In_ const OrtModelMetadata* model_metadata, _Inout_ OrtAllocator* allocator,
                  _Outptr_ char** value);

  /** \brief Get `description` from an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[out] value Set to a null terminated string allocated using `allocator`. Must be freed using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetDescription, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /** \brief Return data for a key in the custom metadata map in an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[in] key Null terminated string
   * \param[out] value Set to a null terminated string allocated using `allocator`. Must be freed using `allocator`
   * `value` will be set to nullptr if the given key is not found in the custom metadata map.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataLookupCustomMetadataMap, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _In_ const char* key, _Outptr_result_maybenull_ char** value);

  /** \brief Get version number from an ::OrtModelMetadata
   *
   * \param[in] model_metadata
   * \param[out] value Set to the version number
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetVersion, _In_ const OrtModelMetadata* model_metadata, _Out_ int64_t* value);

  ORT_CLASS_RELEASE(ModelMetadata);

  /// @}
  /// \name OrtEnv
  /// @{

  /** \brief Create an OrtEnv
   *
   * Create an environment with global threadpools that will be shared across sessions.
   * Use this in conjunction with OrtApi::DisablePerSessionThreads or else the session will use
   * its own thread pools.
   *
   * \param[in] log_severity_level The log severity level.
   * \param[in] logid The log identifier.
   * \param[in] tp_options
   * \param[out] out Returned newly created OrtEnv. Must be freed with OrtApi::ReleaseEnv
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateEnvWithGlobalThreadPools, OrtLoggingLevel log_severity_level, _In_ const char* logid,
                  _In_ const OrtThreadingOptions* tp_options, _Outptr_ OrtEnv** out);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Use global thread pool on a session
   *
   * Disable using per session thread pool and use the shared global threadpool.
   * This should be used in conjunction with OrtApi::CreateEnvWithGlobalThreadPools.
   *
   * \param[in] options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(DisablePerSessionThreads, _Inout_ OrtSessionOptions* options);

  /// @}
  /// \name OrtThreadingOptions
  /// @{

  /** \brief Create an ::OrtThreadingOptions
   *
   * \param[out] out Newly created ::OrtThreadingOptions. Must be freed with OrtApi::ReleaseThreadingOptions
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateThreadingOptions, _Outptr_ OrtThreadingOptions** out);

  ORT_CLASS_RELEASE(ThreadingOptions);

  /// @}
  /// \name OrtModelMetadata
  /// @{

  /**
   *
   * \param[in] model_metadata
   * \param[in] allocator
   * \param[out] keys Array of null terminated strings (array count = num_keys) allocated using `allocator`.
   *  The strings and the pointer array must be freed using `allocator`
   *  `keys` will be set to nullptr if the custom metadata map is empty.
   * \param[out] num_keys Set to the number of elements in the `keys` array
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetCustomMetadataMapKeys, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _Outptr_result_buffer_maybenull_(*num_keys) char*** keys, _Out_ int64_t* num_keys);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /**
   *
   * Override symbolic dimensions (by specific name strings) with actual values
   * if known at session initialization time to enable optimizations that can
   * take advantage of fixed values (such as memory planning, etc)
   *
   */
  ORT_API2_STATUS(AddFreeDimensionOverrideByName,
                  _Inout_ OrtSessionOptions* options, _In_ const char* dim_name,
                  _In_ int64_t dim_value);

  /// @}
  /// \name Misc
  /// @{

  /** \brief Get the names of all available providers
   *
   * \note The providers in the list are not guaranteed to be usable. They may fail to load due to missing system dependencies.
   *    For example, if the CUDA/cuDNN libraries are not installed, the CUDA provider will report an error when it is added to the session options.
   *
   * \param[out] out_ptr Set to a pointer to an array of null terminated strings of the available providers. The entries and the
   *    array itself must be freed using OrtApi::ReleaseAvailableProviders
   * \param[out] provider_length Set to the number of entries in the `out_ptr` array
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetAvailableProviders, _Outptr_ char*** out_ptr, _Out_ int* provider_length);

  /** \brief Release data from OrtApi::GetAvailableProviders. This API will never fail
   * so you can rely on it in a noexcept code.
   *
   * \param[in] ptr The `out_ptr` result from OrtApi::GetAvailableProviders.
   * \param[in] providers_length The `provider_length` result from OrtApi::GetAvailableProviders
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ReleaseAvailableProviders, _In_ char** ptr,
                  _In_ int providers_length);

  /// @}
  /// \name OrtValue
  /// @{

  /** \brief Get the length of a single string in a string tensor
   *
   * \param[in] value A string tensor
   * \param[in] index Index of the string in the tensor
   * \param[out] out Set to number of bytes of the string element
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetStringTensorElementLength, _In_ const OrtValue* value, size_t index, _Out_ size_t* out);

  /** \brief Get a single string from a string tensor
   *
   * \param[in] value A string tensor
   * \param[in] s_len Number of bytes in the `s` buffer. Must match the value returned by OrtApi::GetStringTensorElementLength.
   * \param[in] index Index of the string in the tensor
   * \param[out] s The string element contents in UTF-8 encoding. The string is NOT null-terminated.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetStringTensorElement, _In_ const OrtValue* value, size_t s_len, size_t index, _Out_writes_bytes_all_(s_len) void* s);

  /** \brief Set a single string in a string tensor
   *
   * \param[in] value A string tensor
   * \param[in] s A null terminated UTF-8 encoded string
   * \param[in] index Index of the string in the tensor to set
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(FillStringTensorElement, _Inout_ OrtValue* value, _In_ const char* s, size_t index);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Set a session configuration entry as a pair of strings
   *
   * If a configuration with same key exists, this will overwrite the configuration with the given config_value.
   *
   * The config_key and the format of config_value are defined in onnxruntime_session_options_config_keys.h
   *
   * \param[in] options
   * \param[in] config_key A null terminated string representation of the config key
   * \param[in] config_value A null terminated string representation of the config value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(AddSessionConfigEntry, _Inout_ OrtSessionOptions* options,
                  _In_z_ const char* config_key, _In_z_ const char* config_value);

  /// @}
  /// \name OrtAllocator
  /// @{

  /** \brief Create an allocator for an ::OrtSession following an ::OrtMemoryInfo
   *
   * \param[in] session
   * \param[in] mem_info valid ::OrtMemoryInfo instance
   * \param[out] out Newly created ::OrtAllocator. Must be freed with OrtApi::ReleaseAllocator
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateAllocator, _In_ const OrtSession* session, _In_ const OrtMemoryInfo* mem_info,
                  _Outptr_ OrtAllocator** out);

  /** \brief Release an ::OrtAllocator obtained from OrtApi::CreateAllocator
   */
  ORT_CLASS_RELEASE(Allocator);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief Run a model using Io Bindings for the inputs & outputs
   *
   * \see OrtApi::Run
   *
   * \param[in] session
   * \param[in] run_options
   * \param[in] binding_ptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RunWithBinding, _Inout_ OrtSession* session, _In_ const OrtRunOptions* run_options, _In_ const OrtIoBinding* binding_ptr);

  /** \brief Create an ::OrtIoBinding instance
   *
   * An IoBinding object allows one to bind pre-allocated ::OrtValue%s to input names.
   * Thus if you want to use a raw on device buffer as input or output you can avoid
   * extra copy during runtime.
   *
   * \param[in] session
   * \param[out] out Newly created ::OrtIoBinding. Must be freed with OrtApi::ReleaseIoBinding
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateIoBinding, _Inout_ OrtSession* session, _Outptr_ OrtIoBinding** out);

  /// @}
  /// \name OrtIoBinding
  /// @{

  /** \brief Release an ::OrtIoBinding obtained from OrtApi::CreateIoBinding
   */
  ORT_CLASS_RELEASE(IoBinding);

  /** \brief Bind an ::OrtValue to an ::OrtIoBinding input
   *
   * When using OrtApi::RunWithBinding this value is used for the named input
   *
   * \param[in] binding_ptr
   * \param[in] name Name for the model input
   * \param[in] val_ptr ::OrtValue of Tensor type.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(BindInput, _Inout_ OrtIoBinding* binding_ptr, _In_ const char* name, _In_ const OrtValue* val_ptr);

  /** \brief Bind an ::OrtValue to an ::OrtIoBinding output
   *
   * When using OrtApi::RunWithBinding this value is used for the named output
   *
   * \param[in] binding_ptr
   * \param[in] name Null terminated string of the model output name
   * \param[in] val_ptr ::OrtValue of Tensor type.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(BindOutput, _Inout_ OrtIoBinding* binding_ptr, _In_ const char* name, _In_ const OrtValue* val_ptr);

  /** \brief Bind an ::OrtIoBinding output to a device
   *
   * Binds the ::OrtValue to a device which is specified by ::OrtMemoryInfo.
   * You can either create an instance of ::OrtMemoryInfo with a device id or obtain one from the allocator that you have created/are using
   * This is useful when one or more outputs have dynamic shapes and, it is hard to pre-allocate and bind a chunk of
   * memory within ::OrtValue ahead of time.
   *
   * \see OrtApi::RunWithBinding
   *
   * \param[in] binding_ptr
   * \param[in] name Null terminated string of the device name
   * \param[in] mem_info_ptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(BindOutputToDevice, _Inout_ OrtIoBinding* binding_ptr, _In_ const char* name, _In_ const OrtMemoryInfo* mem_info_ptr);

  /** \brief Get the names of an ::OrtIoBinding's outputs
   *
   * Returns the names of the outputs in the order they were bound. This is useful after running the model
   * with bound outputs because the returned names are in order in which output ::OrtValue are returned. This is useful if
   * the order of outputs and their names is not known.
   *
   * \param[in] binding_ptr
   * \param[in] allocator Allocator used to allocate continuous buffers for output strings and lengths.
   * \param[out] buffer Returns an array of non-null terminated UTF-8 strings. The number of strings stored is returned in the count parameter.
   *   This buffer is allocated using `allocator` and must be freed using it.
   * \param[out] lengths Returns an array of `count` lengths of the strings returned in `buffer`
   *   This buffer is allocated using `allocator` and must be freed using it.
   * \param[out] count Number of strings returned. If `binding_ptr` has no bound outputs, zero is returned,
   *              no memory allocation is performed and buffer and lengths are set to nullptr.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetBoundOutputNames, _In_ const OrtIoBinding* binding_ptr, _In_ OrtAllocator* allocator,
                  _Out_ char** buffer, _Out_writes_all_(count) size_t** lengths, _Out_ size_t* count);

  /** \brief Get the output ::OrtValue objects from an ::OrtIoBinding
   *
   * Returns an array of pointers to individually allocated ::OrtValue%s that contain results of a model execution with OrtApi::RunWithBinding
   * The array contains the same number of ::OrtValue%s and they are in the same order as they were bound with OrtApi::BindOutput
   * or OrtApi::BindOutputToDevice.
   *
   * The returned ::OrtValue%s must be released using OrtApi::ReleaseValue after they are no longer needed.
   * The array is allocated using the specified instance of the allocator and must be freed using the same allocator after
   * all the ::OrtValue%s contained therein are individually released.
   *
   * \param[in] binding_ptr
   * \param[in] allocator Allocator used to allocate output array
   * \param[out] output Set to the allocated array of allocated ::OrtValue outputs. Set to nullptr if there are 0 outputs.
   * \param[out] output_count Set to number of ::OrtValue%s returned
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetBoundOutputValues, _In_ const OrtIoBinding* binding_ptr, _In_ OrtAllocator* allocator,
                  _Out_writes_all_(output_count) OrtValue*** output, _Out_ size_t* output_count);

  /** \brief Clears any previously set Inputs for an ::OrtIoBinding
   */
  void(ORT_API_CALL* ClearBoundInputs)(_Inout_ OrtIoBinding* binding_ptr) NO_EXCEPTION ORT_ALL_ARGS_NONNULL;

  /** \brief Clears any previously set Outputs for an ::OrtIoBinding
   */
  void(ORT_API_CALL* ClearBoundOutputs)(_Inout_ OrtIoBinding* binding_ptr) NO_EXCEPTION ORT_ALL_ARGS_NONNULL;

  /// @}
  /// \name OrtValue
  /// @{

  /** \brief Direct memory access to a specified tensor element
   *
   * For example, given a tensor with shape of [3,224,224], a pointer to the element at location [2,150,128] can be retrieved
   *
   * This function only works for numeric type tensors (No strings, etc).
   * This is a no-copy method whose returned pointer is valid until the passed in ::OrtValue is free'd.
   *
   * \param[in] value
   * \param[in] location_values Pointer to an array of index values that specify an element's location relative to its shape
   * \param[in] location_values_count Number of elements in location_values. Must match the number of elements in the tensor's shape.
   * \param[out] out Set to a pointer to the element specified
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(TensorAt, _Inout_ OrtValue* value, const int64_t* location_values, size_t location_values_count, _Outptr_ void** out);

  /// @}
  /// \name OrtEnv
  /// @{

  /** \brief Create an allocator and register it with the ::OrtEnv
   *
   * Enables sharing the allocator between multiple sessions that use the same env instance.
   * Lifetime of the created allocator will be valid for the duration of the environment.
   * Returns an error if an allocator with the same ::OrtMemoryInfo is already registered.
   *
   * See https://onnxruntime.ai/docs/get-started/with-c.html for details.
   *
   * \param[in] env ::OrtEnv instance
   * \param[in] mem_info
   * \param[in] arena_cfg Pass nullptr for defaults
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateAndRegisterAllocator, _Inout_ OrtEnv* env, _In_ const OrtMemoryInfo* mem_info,
                  _In_ const OrtArenaCfg* arena_cfg);

  /** \brief Set language projection
   *
   * Set the language projection for collecting telemetry data when Env is created.
   *
   * The default is ORT_PROJECTION_C, which means it will classify the language not in the list to C also.
   *
   * \param[in] ort_env
   * \param[in] projection
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetLanguageProjection, _In_ const OrtEnv* ort_env, _In_ OrtLanguageProjection projection);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief Return the time that profiling was started
   *
   * \note The timer precision varies per platform. On Windows and MacOS, the precision will be ~100ns
   *
   * \param[in] session
   * \param[out] out nanoseconds of profiling's start time
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionGetProfilingStartTimeNs, _In_ const OrtSession* session, _Outptr_ uint64_t* out);

  /// @}
  /// \name OrtThreadingOptions
  /// @{

  /** \brief Set global intra-op thread count
   *
   * This configures the global thread pool options to be used in the call to OrtApi::CreateEnvWithGlobalThreadPools
   *
   * \param[in] tp_options
   * \param[in] intra_op_num_threads Number of threads, special values:<br>
   *    0 = Use default thread count<br>
   *    1 = The invoking thread will be used; no threads will be created in the thread pool.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalIntraOpNumThreads, _Inout_ OrtThreadingOptions* tp_options, int intra_op_num_threads);

  /** \brief Set global inter-op thread count
   *
   * This configures the global thread pool options to be used in the call to OrtApi::CreateEnvWithGlobalThreadPools
   *
   * \param[in] tp_options
   * \param[in] inter_op_num_threads Number of threads, special values:<br>
   *    0 = Use default thread count<br>
   *    1 = The invoking thread will be used; no threads will be created in the thread pool.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalInterOpNumThreads, _Inout_ OrtThreadingOptions* tp_options, int inter_op_num_threads);

  /** \brief Set global spin control options
   *
   * This will configure the global thread pool options to be used in the call to OrtApi::CreateEnvWithGlobalThreadPools.
   * Allow spinning of thread pools when their queues are empty. This will set the value for both
   * inter_op and intra_op threadpools.
   *
   * \param[in] tp_options
   * \param[in] allow_spinning Valid values are 0 or 1.<br>
   *   0 = It won't spin (recommended if CPU usage is high)<br>
   *   1 = Threadpool will spin to wait for queue to become non-empty
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalSpinControl, _Inout_ OrtThreadingOptions* tp_options, int allow_spinning);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Add a pre-allocated initializer to a session
   *
   * If a model contains an initializer with a name that is same as the name passed to this call,
   * ORT will use this initializer instance instead of deserializing one from the model file. This
   * is useful when you want to share the same initializer across sessions.
   *
   * \param[in] options
   * \param[in] name Null terminated string of the initializer name
   * \param[in] val ::OrtValue containing the initializer. Its lifetime and the underlying initializer buffer must be
   *   managed by the user (created using the OrtApi::CreateTensorWithDataAsOrtValue) and it must outlive the session object
   *   to which it is added.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(AddInitializer, _Inout_ OrtSessionOptions* options, _In_z_ const char* name,
                  _In_ const OrtValue* val);

  /// @}
  /// \name OrtEnv
  /// @{

  /**
   * Create a custom environment with global threadpools and logger that will be shared across sessions.
   * Use this in conjunction with OrtApi::DisablePerSessionThreads or else the session will use
   * its own thread pools.
   *
   * \param[in] logging_function A pointer to a logging function.
   * \param[in] logger_param A pointer to arbitrary data passed as the ::OrtLoggingFunction `param` parameter to
   *                         `logging_function`.
   * \param[in] log_severity_level The log severity level.
   * \param[in] logid The log identifier.
   * \param[in] tp_options
   * \param[out] out Newly created OrtEnv. Must be freed with OrtApi::ReleaseEnv
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateEnvWithCustomLoggerAndGlobalThreadPools, OrtLoggingFunction logging_function, _In_opt_ void* logger_param, OrtLoggingLevel log_severity_level,
                  _In_ const char* logid, _In_ const struct OrtThreadingOptions* tp_options, _Outptr_ OrtEnv** out);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Append CUDA provider to session options
   *
   * If CUDA is not available (due to a non CUDA enabled build, or if CUDA is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] cuda_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_CUDA,
                  _In_ OrtSessionOptions* options, _In_ const OrtCUDAProviderOptions* cuda_options);

  /** \brief Append ROCM execution provider to the session options
   *
   * If ROCM is not available (due to a non ROCM enabled build, or if ROCM is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] rocm_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_ROCM,
                  _In_ OrtSessionOptions* options, _In_ const OrtROCMProviderOptions* rocm_options);

  /** \brief Append OpenVINO execution provider to the session options
   *
   * If OpenVINO is not available (due to a non OpenVINO enabled build, or if OpenVINO is not installed on the system), this function will fail.
   *
   * \param[in] options
   * \param[in] provider_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_OpenVINO,
                  _In_ OrtSessionOptions* options, _In_ const OrtOpenVINOProviderOptions* provider_options);

  /// @}
  /// \name OrtThreadingOptions
  /// @{

  /** \brief Set threading flush-to-zero and denormal-as-zero
   *
   * Sets global thread pool options to be used in the call to OrtApi::CreateEnvWithGlobalThreadPools.
   * Flush-to-zero and denormal-as-zero are applied to threads in both intra and inter global thread pool.
   * \note This option is not needed if the models used have no denormals. Having no denormals is recommended as this option may hurt model accuracy.
   *
   * \param[in] tp_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalDenormalAsZero, _Inout_ OrtThreadingOptions* tp_options);

  /// @}
  /// \name OrtArenaCfg
  /// @{

  /** \deprecated Use OrtApi::CreateArenaCfgV2
   *
   * This will create the configuration of an arena that can eventually be used to define an arena based allocator's behavior
   *
   * \param[in] max_mem Use 0 to allow ORT to choose the default
   * \param[in] arena_extend_strategy Use -1 to allow ORT to choose the default, 0 = kNextPowerOfTwo, 1 = kSameAsRequested
   * \param[in] initial_chunk_size_bytes Use -1 to allow ORT to choose the default
   * \param[in] max_dead_bytes_per_chunk Use -1 to allow ORT to choose the default
   * \param[in] out A pointer to an OrtArenaCfg instance
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateArenaCfg, _In_ size_t max_mem, int arena_extend_strategy, int initial_chunk_size_bytes,
                  int max_dead_bytes_per_chunk, _Outptr_ OrtArenaCfg** out);

  ORT_CLASS_RELEASE(ArenaCfg);

  /// @}
  /// \name OrtModelMetadata
  /// @{

  /**
   * Use this to obtain the description of the graph present in the model
   * (doc_string field of the GraphProto message within the ModelProto message).
   * If it doesn't exist, an empty string will be returned.
   *
   * \param[in] model_metadata An instance of ::OrtModelMetadata
   * \param[in] allocator Allocator used to allocate the string that will be returned back
   * \param[out] value Set to a null terminated string allocated using `allocator`.  The caller is responsible for freeing it using `allocator`
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(ModelMetadataGetGraphDescription, _In_ const OrtModelMetadata* model_metadata,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** value);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Append TensorRT provider to session options
   *
   * If TensorRT is not available (due to a non TensorRT enabled build, or if TensorRT is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] tensorrt_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_TensorRT,
                  _In_ OrtSessionOptions* options, _In_ const OrtTensorRTProviderOptions* tensorrt_options);

  /// @}
  /// \name Misc
  /// @{

  /** \brief Set current GPU device ID
   *
   * Set the current device id of the GPU execution provider (CUDA/tensorrt/rocm). The device id should be less
   * than the total number of devices available. This is only useful when multiple-GPUs are installed and it is
   * required to restrict execution to a single GPU.
   *
   * \param[in] device_id
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetCurrentGpuDeviceId, _In_ int device_id);

  /** \brief Get current GPU device ID
   *
   * Get the current device id of the GPU execution provider (CUDA/tensorrt/rocm).
   *
   * \see OrtApi::SetCurrentGpuDeviceId
   *
   * \param[out] device_id
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetCurrentGpuDeviceId, _In_ int* device_id);

  /// @}
  /// \name OrtKernelInfo
  /// Custom operator APIs.
  /// @{

  /** \brief Fetch an array of int64_t values stored as an attribute in the graph node
   *
   *
   * If `out` is nullptr, the value of `size` is set to the true size of the attribute
   * array's size, and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the actual attribute array's size,
   * the value of `size` is set to the true size of the attribute array's size,
   * the provided memory is filled with the attribute's contents,
   * and a success status is returned.
   *
   * If the `size` parameter is less than the actual attribute array's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the attribute array's size
   * and a failure status is returned.)
   *
   * \param[in] info instance
   * \param[in] name name of the attribute to be parsed
   * \param[out] out pointer to memory where the attribute's contents are to be stored
   * \param[in, out] size actual size of attribute array
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttributeArray_float, _In_ const OrtKernelInfo* info, _In_ const char* name,
                  _Out_ float* out, _Inout_ size_t* size);

  /** \brief Fetch an array of int64_t values stored as an attribute in the graph node
   *
   * If `out` is nullptr, the value of `size` is set to the true size of the attribute
   * array's size, and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the actual attribute array's size,
   * the value of `size` is set to the true size of the attribute array's size,
   * the provided memory is filled with the attribute's contents,
   * and a success status is returned.
   *
   * If the `size` parameter is less than the actual attribute array's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the attribute array's size
   * and a failure status is returned.)
   *
   * \param[in] info instance
   * \param[in] name name of the attribute to be parsed
   * \param[out] out pointer to memory where the attribute's contents are to be stored
   * \param[in, out] size actual size of attribute array
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttributeArray_int64, _In_ const OrtKernelInfo* info, _In_ const char* name,
                  _Out_ int64_t* out, _Inout_ size_t* size);

  /// @}
  /// \name OrtArenaCfg
  /// @{

  /** \brief Create an ::OrtArenaCfg
   *
   * Create the configuration of an arena that can eventually be used to define an arena based allocator's behavior.
   *
   * Supported keys are (See https://onnxruntime.ai/docs/get-started/with-c.html for details on what the
   * following parameters mean and how to choose these values.):
   * "max_mem": Maximum memory that can be allocated by the arena based allocator.
   *  Use 0 for ORT to pick the best value. Default is 0.
   * "arena_extend_strategy": 0 = kNextPowerOfTwo, 1 = kSameAsRequested.
   *  Use -1 to allow ORT to choose the default.
   * "initial_chunk_size_bytes": (Possible) Size of the first allocation in the arena.
   *  Only relevant if arena strategy is `kNextPowerOfTwo`. Use -1 to allow ORT to choose the default.
   *  Ultimately, the first allocation size is determined by the allocation memory request.
   * "max_dead_bytes_per_chunk": Threshold of unused memory in an allocated chunk of arena memory after
   *  crossing which the current chunk is chunked into 2.
   * "initial_growth_chunk_size_bytes": (Possible) Size of the second allocation in the arena.
   *  Only relevant if arena strategy is `kNextPowerOfTwo`. Use -1 to allow ORT to choose the default.
   * "max_power_of_two_extend_bytes": The maximum enxtend size if arena strategy is `kNextPowerOfTwo`.
   *  It is not an allocation limit, it is only a limit for extension when requested byte is less than the limit.
   *  When requested bytes is more than the limit, allocator will still return as requested.
   *  Use -1 to allow ORT to choose the default 1GB for max_power_of_two_extend_bytes.
   *  Ultimately, the allocation size is determined by the allocation memory request.
   *  Further allocation sizes are governed by the arena extend strategy.
   *
   * \param[in] arena_config_keys Keys to configure the arena
   * \param[in] arena_config_values Values to configure the arena
   * \param[in] num_keys Number of keys in `arena_config_keys` and `arena_config_values`
   * \param[out] out Newly created ::OrtArenaCfg. Must be freed with OrtApi::ReleaseArenaCfg
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateArenaCfgV2, _In_reads_(num_keys) const char* const* arena_config_keys,
                  _In_reads_(num_keys) const size_t* arena_config_values, _In_ size_t num_keys,
                  _Outptr_ OrtArenaCfg** out);

  /// @}
  /// \name OrtRunOptions
  /// @{

  /** \brief Set a single run configuration entry as a pair of strings
   *
   * If a configuration with same key exists, this will overwrite the configuration with the given config_value
   *
   * The config_key and the format of config_value are defined in onnxruntime_run_options_config_keys.h
   *
   * \param[in] options
   * \param[in] config_key A null terminated string representation of the config key
   * \param[in] config_value  A null terminated string representation of the config value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(AddRunConfigEntry, _Inout_ OrtRunOptions* options,
                  _In_z_ const char* config_key, _In_z_ const char* config_value);

  /// @}
  /// \name OrtPrepackedWeightsContainer
  /// @{

  /** \brief Create an ::OrtPrepackedWeightsContainer
   *
   * This container will hold pre-packed buffers of shared initializers for sharing between sessions
   * (i.e.) if there are shared initializers that can be shared between sessions, the pre-packed buffers
   * of these (if any) may possibly be shared to provide memory footprint savings. Pass this container
   * to sessions that you would like to share pre-packed buffers of shared initializers at session
   * creation time.
   *
   *  \param[out] out Newly created ::OrtPrepackedWeightsContainer. Must be freed with OrtApi::ReleasePrepackedWeightsContainer
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreatePrepackedWeightsContainer, _Outptr_ OrtPrepackedWeightsContainer** out);

  /** \brief Release OrtPrepackedWeightsContainer instance
   *
   * \note instance must not be released until the sessions using it are released
   */
  ORT_CLASS_RELEASE(PrepackedWeightsContainer);

  /// @}
  /// \name OrtSession
  /// @{

  /** \brief Create session with prepacked weights container
   *
   * Same functionality offered by OrtApi::CreateSession except that a container that contains
   * pre-packed weights' buffers is written into/read from by the created session.
   * This is useful when used in conjunction with OrtApi::AddInitializer which injects
   * shared initializer info into sessions. Wherever possible, the pre-packed versions of these
   * shared initializers are cached in this container so that multiple sessions can just re-use
   * these instead of duplicating these in memory.
   *
   * \param[in] env OrtEnv instance instance
   * \param[in] model_path Null terminated string of the path (wchar on Windows, char otherwise)
   * \param[in] options
   * \param[in] prepacked_weights_container
   * \param[out] out Newly created ::OrtSession. Must be freed with OrtApi::ReleaseSession
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSessionWithPrepackedWeightsContainer, _In_ const OrtEnv* env, _In_ const ORTCHAR_T* model_path,
                  _In_ const OrtSessionOptions* options,
                  _Inout_ OrtPrepackedWeightsContainer* prepacked_weights_container,
                  _Outptr_ OrtSession** out);

  /** \brief Create session from memory with prepacked weights container
   *
   * Same functionality offered by OrtApi::CreateSessionFromArray except that a container that contains
   * pre-packed weights' buffers is written into/read from by the created session.
   * This is useful when used in conjunction with OrtApi::AddInitializer which injects
   * shared initializer info into sessions. Wherever possible, the pre-packed versions of these
   * shared initializers are cached in this container so that multiple sessions can just re-use
   * these instead of duplicating these in memory.
   *
   * \param[in] env
   * \param[in] model_data Array of bytes holding the model
   * \param[in] model_data_length Number of bytes in `model_data_model`
   * \param[in] options
   * \param[in] prepacked_weights_container
   * \param[out] out Newly created ::OrtSession. Must be freed with OrtApi::ReleaseSession
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSessionFromArrayWithPrepackedWeightsContainer, _In_ const OrtEnv* env,
                  _In_ const void* model_data, size_t model_data_length,
                  _In_ const OrtSessionOptions* options,
                  _Inout_ OrtPrepackedWeightsContainer* prepacked_weights_container,
                  _Outptr_ OrtSession** out);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Append TensorRT execution provider to the session options
   *
   * If TensorRT is not available (due to a non TensorRT enabled build), this function will return failure.
   *
   * This is slightly different from OrtApi::SessionOptionsAppendExecutionProvider_TensorRT, it takes an
   * ::OrtTensorRTProviderOptions which is publicly defined. This takes an opaque ::OrtTensorRTProviderOptionsV2
   * which must be created with OrtApi::CreateTensorRTProviderOptions.
   *
   * For OrtApi::SessionOptionsAppendExecutionProvider_TensorRT, the user needs to instantiate ::OrtTensorRTProviderOptions
   * as well as allocate/release buffers for some members of ::OrtTensorRTProviderOptions.
   * Here, OrtApi::CreateTensorRTProviderOptions and Ortapi::ReleaseTensorRTProviderOptions will do the memory management for you.
   *
   * \param[in] options
   * \param[in] tensorrt_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_TensorRT_V2,
                  _In_ OrtSessionOptions* options, _In_ const OrtTensorRTProviderOptionsV2* tensorrt_options);

  /// @}
  /// \name OrtTensorRTProviderOptionsV2
  /// @{

  /** \brief Create an OrtTensorRTProviderOptionsV2
   *
   * \param[out] out Newly created ::OrtTensorRTProviderOptionsV2. Must be released with OrtApi::ReleaseTensorRTProviderOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateTensorRTProviderOptions, _Outptr_ OrtTensorRTProviderOptionsV2** out);

  /** \brief Set options in a TensorRT Execution Provider.
   *
   * Please refer to https://onnxruntime.ai/docs/execution-providers/TensorRT-ExecutionProvider.html#cc
   * to know the available keys and values. Key should be in null terminated string format of the member of ::OrtTensorRTProviderOptionsV2
   * and value should be its related range. Recreates the options and only sets the supplied values.
   *
   * For example, key="trt_max_workspace_size" and value="2147483648"
   *
   * \param[in] tensorrt_options
   * \param[in] provider_options_keys Array of UTF-8 null-terminated string for provider options keys
   * \param[in] provider_options_values Array of UTF-8 null-terminated string for provider options values
   * \param[in] num_keys Number of elements in the `provider_option_keys` and `provider_options_values` arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(UpdateTensorRTProviderOptions, _Inout_ OrtTensorRTProviderOptionsV2* tensorrt_options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /** \brief Get serialized TensorRT provider options string.
   *
   * For example, "trt_max_workspace_size=2147483648;trt_max_partition_iterations=10;trt_int8_enable=1;......"
   *
   * \param tensorrt_options - OrtTensorRTProviderOptionsV2 instance
   * \param allocator - a ptr to an instance of OrtAllocator obtained with OrtApi::CreateAllocator or OrtApi::GetAllocatorWithDefaultOptions
   *                      the specified allocator will be used to allocate continuous buffers for output strings and lengths.
   * \param ptr - is a UTF-8 null terminated string allocated using 'allocator'. The caller is responsible for using the same allocator to free it.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorRTProviderOptionsAsString, _In_ const OrtTensorRTProviderOptionsV2* tensorrt_options, _Inout_ OrtAllocator* allocator, _Outptr_ char** ptr);

  /** \brief Release an ::OrtTensorRTProviderOptionsV2
   *
   * \note This is an exception in the naming convention of other Release* functions, as the name of the method does not have the V2 suffix, but the type does
   */
  void(ORT_API_CALL* ReleaseTensorRTProviderOptions)(_Frees_ptr_opt_ OrtTensorRTProviderOptionsV2* input);

  /// @}
  /// \name OrtSessionOptions
  /// @{

  /** \brief Enable custom operators
   *
   * See onnxruntime-extensions: https://github.com/microsoft/onnxruntime-extensions.git
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(EnableOrtCustomOps, _Inout_ OrtSessionOptions* options);

  /// @}
  /// \name OrtAllocator
  /// @{

  /** \brief Register a custom allocator
   *
   * Enables sharing between multiple sessions that use the same env instance.
   * Returns an error if an allocator with the same ::OrtMemoryInfo is already registered.
   *
   * The behavior of this is exactly the same as OrtApi::CreateAndRegisterAllocator except
   * instead of ORT creating an allocator based on provided info, in this case
   * ORT uses the user-provided custom allocator.
   * See https://onnxruntime.ai/docs/get-started/with-c.html for details.
   *
   * \param[in] env
   * \param[in] allocator User provided allocator
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(RegisterAllocator, _Inout_ OrtEnv* env, _In_ OrtAllocator* allocator);

  /** \brief Unregister a custom allocator
   *
   * It is an error if you provide an ::OrtMemoryInfo not corresponding to any
   * registered allocators for sharing.
   *
   * \param[in] env
   * \param[in] mem_info
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(UnregisterAllocator, _Inout_ OrtEnv* env,
                  _In_ const OrtMemoryInfo* mem_info);

  /// @}
  /// \name OrtValue
  /// @{

  /** \brief Sets *out to 1 iff an ::OrtValue is a SparseTensor, and 0 otherwise
   *
   * \param[in] value existing ::OrtValue
   * \param[out] out unless an error occurs, contains 1 iff the value contains an instance
   *  of sparse tensor or 0 otherwise.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(IsSparseTensor, _In_ const OrtValue* value, _Out_ int* out);

  /** \brief Create an ::OrtValue with a sparse tensor that is empty.
   *
   * Use FillSparseTensor<Format>() functions to populate sparse tensor with non-zero values and
   * format specific indices data.
   * Use ReleaseValue to destroy the sparse tensor, this will also release the buffer inside the output value
   * if any was allocated.
   * \param[in,out] allocator allocator to use when performing an allocation. Allocation will be performed
   *   by FillSparseTensor<Format>() APIs. The lifespan of the allocator instance must eclipse the lifespan
   *   this sparse tensor instance as the same allocator will be used to free memory.
   * \param[in] dense_shape shape of the original dense tensor
   * \param[in] dense_shape_len number of shape dimensions being passed
   * \param[in] type must be one of TENSOR_ELEMENT_DATA_TYPE_xxxx
   * \param[out] out Should be freed by calling ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSparseTensorAsOrtValue, _Inout_ OrtAllocator* allocator, _In_ const int64_t* dense_shape,
                  size_t dense_shape_len, ONNXTensorElementDataType type, _Outptr_ OrtValue** out);

  /**
   * This fills populates an empty tensor that was created using OrtApi::CreateSparseTensorAsOrtValue.
   * This will allocate required memory and copy the supplied NNZ values and COO indices into that memory allocation.
   * Memory allocation is performed using the allocator that was specified with OrtApi::CreateSparseTensorAsOrtValue.
   *
   * \param[in,out] ort_value ::OrtValue to populate with data
   * \param[in] data_mem_info serves to identify the location of the data to be copied. If the allocator specified
   *  at the creation time has memory info that is not the same as mem_info argument to this function a X-device copy will be performed.
   *  String data is assumed to be on CPU and will only be copied into a CPU allocated buffer.
   * \param[in] values_shape pointer to values shape array
   * \param[in] values_shape_len length of the values_shape
   * \param[in] values pointer to an array of values. For strings, pass const char**.
   * \param[in] indices_data pointer to a location of COO indices
   * \param[in] indices_num number of COO indices
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(FillSparseTensorCoo, _Inout_ OrtValue* ort_value, _In_ const OrtMemoryInfo* data_mem_info,
                  _In_ const int64_t* values_shape, size_t values_shape_len, _In_ const void* values,
                  _In_ const int64_t* indices_data, size_t indices_num);

  /**
   * This fills populates an empty tensor that was created using OrtApi::CreateSparseTensorAsOrtValue.
   * This will allocate required memory and copy the supplied NNZ values and CSR indices into that memory allocation.
   * Memory allocation is performed using the allocator that was specified with OrtApi::CreateSparseTensorAsOrtValue.
   *
   * \param[in,out] ort_value ::OrtValue to populate with data
   * \param[in] data_mem_info serves to identify the location of the data to be copied. If the allocator specified
   *  at the creation time has memory info that is not the same as mem_info argument to this function a X-device copy will be performed.
   *  String data is assumed to be on CPU and will only be copied into a CPU allocated buffer.
   * \param[in] values_shape pointer to values shape array
   * \param[in] values_shape_len length of the values_shape
   * \param[in] values - pointer to an array of values. For strings, pass const char**.
   * \param[in] inner_indices_data pointer to a location of CSR inner indices
   * \param[in] inner_indices_num number of CSR inner indices
   * \param[in] outer_indices_data pointer to a location of CSR outer indices
   * \param[in] outer_indices_num number of CSR outer indices
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(FillSparseTensorCsr, _Inout_ OrtValue* ort_value, _In_ const OrtMemoryInfo* data_mem_info,
                  _In_ const int64_t* values_shape, size_t values_shape_len, _In_ const void* values,
                  _In_ const int64_t* inner_indices_data, size_t inner_indices_num,
                  _In_ const int64_t* outer_indices_data, size_t outer_indices_num);

  /**
   * This fills populates an empty tensor that was created using OrtApi::CreateSparseTensorAsOrtValue.
   * This will allocate required memory and copy the supplied NNZ values and BlockSparse indices into that memory allocation.
   * Memory allocation is performed using the allocator that was specified with OrtApi::CreateSparseTensorAsOrtValue.
   *
   * \param[in,out] ort_value ::OrtValue to populate with data
   * \param[in] data_mem_info serves to identify the location of the data to be copied. If the allocator specified
   *  at the creation time has memory info that is not the same as mem_info argument to this function a X-device copy will be performed.
   *  String data is assumed to be on CPU and will only be copied into a CPU allocated buffer.
   * \param[in] values_shape
   * \param[in] values_shape_len
   * \param[in] values structure with values information
   * \param[in] indices_shape_data pointer to a location of indices shape
   * \param[in] indices_shape_len length of the block sparse indices shape
   * \param[in] indices_data pointer to a location of indices data. Shape will determine the length of the indices data.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(FillSparseTensorBlockSparse, _Inout_ OrtValue* ort_value, _In_ const OrtMemoryInfo* data_mem_info,
                  _In_ const int64_t* values_shape, size_t values_shape_len, _In_ const void* values,
                  _In_ const int64_t* indices_shape_data, size_t indices_shape_len,
                  _In_ const int32_t* indices_data);

  /**
   * Create an ::OrtValue with a sparse tensor. This is the first step.
   * Next, use Use<Format>Indices() functions to supply sparse tensor with
   * format specific indices data and set its sparse format to a specific enum value.
   * This will not perform memory allocations. It will
   * use supplied user buffer which should outlive the created sparse tensor.
   * Use OrtApi::ReleaseValue to destroy the sparse tensor. It would not release the supplied values buffer.
   * This function can not be used to map strings from the user allocated memory. Strings must always be copied
   * and have UTF-8 encoding. Therefore, use OrtApi::CreateSparseTensorAsOrtValue above and then fill it with data
   * using appropriate Make*() function.
   *
   * \param[in] info memory info where sparse values reside.
   * \param[in,out] p_data pointer to a user allocated buffer with values. To create a full sparse tensor with no non-zero
   *   values, pass nullptr
   * \param[in] dense_shape shape of the original dense tensor
   * \param[in] dense_shape_len number of shape dimensions being passed
   * \param[in] values_shape shape of the values data. To create a fully sparse tensor with no non-zero values,
   *   pass {0} shape.
   * \param[in] values_shape_len number of values shape dimensions
   * \param[in] type must be one of TENSOR_ELEMENT_DATA_TYPE_xxxx
   * \param[out] out Should be freed by calling ReleaseValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(CreateSparseTensorWithValuesAsOrtValue, _In_ const OrtMemoryInfo* info, _Inout_ void* p_data,
                  _In_ const int64_t* dense_shape, size_t dense_shape_len,
                  _In_ const int64_t* values_shape, size_t values_shape_len,
                  ONNXTensorElementDataType type, _Outptr_ OrtValue** out);

  /**
   * This assigns Coo format indices to the SparseTensor that was created by
   * OrtApi::CreateSparseTensorWithValuesAsOrtValue above. It also sets OrtSparseFormat to
   * ORT_SPARSE_COO. This will not allocate any additional memory for data. The life span of
   * indices_data buffer should eclipse the life span of this ::OrtValue.
   *
   * \param[in,out] ort_value ::OrtValue instance constructed with OrtApi::CreateSparseTensorWithValuesAsOrtValue
   * \param[in,out] indices_data pointer to a user pre-allocated buffer or nullptr for fully sparse tensors.
   * \param[in] indices_num  number of COO indices. Should either be 0 for fully sparse tensors, be equal
   *  to the number of nnz values specified to OrtApi::CreateSparseTensorWithValuesAsOrtValue for 1-D {nnz} indices or
   *  be twice as number of nnz values for a  2-D indices {nnz, 2}
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(UseCooIndices, _Inout_ OrtValue* ort_value, _Inout_ int64_t* indices_data, size_t indices_num);

  /**
   * The assigns CSR format indices to the SparseTensor that was created by
   * OrtApi::CreateSparseTensorWithValuesAsOrtValue above. It also sets OrtSparseFormat to
   * ORT_SPARSE_CSRC. This will not allocate any additional memory for data. The life spans of
   * inner_data and outer_data buffers should eclipse the life span of this ::OrtValue.
   *
   * \param[in,out] ort_value ::OrtValue instance constructed with OrtApi::CreateSparseTensorWithValuesAsOrtValue
   * \param[in,out] inner_data pointer to a user pre-allocated buffer or nullptr for fully sparse tensors.
   * \param[in] inner_num  number of inner CSR indices. Should either be 0 for fully sparse tensors or be equal
   * to the number of nnz values specified to OrtApi::CreateSparseTensorWithValuesAsOrtValue.
   * \param[in,out] outer_data pointer to user pre-allocated buffer or nullptr for fully sparse tensors.
   * \param[in] outer_num number of CSR outer indices. Should either be 0 for fully sparse tensors or
   * equal to rows + 1 of the dense shape.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(UseCsrIndices, _Inout_ OrtValue* ort_value, _Inout_ int64_t* inner_data, size_t inner_num,
                  _Inout_ int64_t* outer_data, size_t outer_num);

  /**
   * The assigns BlockSparse format indices to the SparseTensor that was created by
   * OrtApi::CreateSparseTensorWithValuesAsOrtValue above. It also sets OrtSparseFormat to
   * ORT_SPARSE_BLOCK_SPARSE. This will not allocate any additional memory for data. The life span of
   * indices_data buffer must eclipse the lifespan of this ::OrtValue.
   *
   * \param[in,out] ort_value OrtValue instance constructed with OrtApi::CreateSparseTensorWithValuesAsOrtValue
   * \param[in] indices_shape pointer to indices shape. Use {0} for fully sparse tensors
   * \param[in] indices_shape_len length of the indices shape
   * \param[in,out] indices_data pointer to user pre-allocated buffer or nullptr for fully sparse tensors.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(UseBlockSparseIndices, _Inout_ OrtValue* ort_value, const int64_t* indices_shape, size_t indices_shape_len, _Inout_ int32_t* indices_data);

  /** \brief Returns sparse tensor format enum iff a given ort value contains an instance of sparse tensor.
   *
   * \param[in] ort_value ::OrtValue that contains an instance of sparse tensor
   * \param[out] out pointer to out parameter
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSparseTensorFormat, _In_ const OrtValue* ort_value, _Out_ enum OrtSparseFormat* out);

  /** \brief Returns data type and shape of sparse tensor values (nnz) iff ::OrtValue contains a SparseTensor.
   *
   * \param[in] ort_value An ::OrtValue that contains a fully constructed sparse tensor
   * \param[out] out Must be freed by OrtApi::ReleaseTensorTypeAndShapeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSparseTensorValuesTypeAndShape, _In_ const OrtValue* ort_value, _Outptr_ OrtTensorTypeAndShapeInfo** out);

  /** \brief Returns numeric data for sparse tensor values (nnz). For string values use GetStringTensor*().
   *
   * \param[in] ort_value an instance of ::OrtValue containing sparse tensor
   * \param[out] out returns a pointer to values data.  Do not attempt to free this ptr.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSparseTensorValues, _In_ const OrtValue* ort_value, _Outptr_ const void** out);

  /** \brief Returns data type, shape for the type of indices specified by indices_format.
   *
   * \param[in] ort_value ::OrtValue containing sparse tensor.
   * \param[in] indices_format One of the indices formats. It is an error to request a format that the sparse
   * tensor does not contain.
   * \param[out] out an instance of ::OrtTensorTypeAndShapeInfo. Must be freed by OrtApi::ReleaseTensorTypeAndShapeInfo
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSparseTensorIndicesTypeShape, _In_ const OrtValue* ort_value, enum OrtSparseIndicesFormat indices_format, _Outptr_ OrtTensorTypeAndShapeInfo** out);

  /** \brief Returns indices data for the type of the indices specified by indices_format
   *
   * \param[in] ort_value ::OrtValue containing sparse tensor.
   * \param[in] indices_format One of the indices formats. It is an error to request a format that the sparse tensor does not contain.
   * \param[out] num_indices Pointer to where the number of indices entries is returned
   * \param[out] indices Returned pointer to the indices data. Do not free the returned pointer as it refers to internal data owned by the ::OrtValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetSparseTensorIndices, _In_ const OrtValue* ort_value, enum OrtSparseIndicesFormat indices_format, _Out_ size_t* num_indices, _Outptr_ const void** indices);
  /// @}
  /// \name OrtSessionOptions
  /// @{

  /**
   * \brief Sets out to 1 iff an optional type OrtValue has an element, 0 otherwise (OrtValue is None)
   * Use this API to find if the optional type OrtValue is None or not.
   * If the optional type OrtValue is not None, use the OrtValue just like any other OrtValue.
   * For example, if you get an OrtValue that corresponds to Optional(tensor) and
   * if HasValue() returns true, use it as tensor and so on.

   * \param[in] value Input OrtValue.
   * \param[out] out indicating if the input OrtValue contains data (1) or if it is a None (0)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(HasValue, _In_ const OrtValue* value, _Out_ int* out);

  /// @}
  /// \name OrtKernelContext
  /// Custom operator APIs.
  /// @{

  /** \brief Used for custom operators, gets the GPU compute stream to use to launch the custom a GPU kernel
   *   \see ::OrtCustomOp
   * \param[in]  context OrtKernelContext instance
   * \param[out] out Returns pointer to a GPU compute stream that can be used to launch the custom GPU kernel.
   *             If retrieving the GPU compute stream is not relevant (GPU not enabled in the build, kernel partitioned to
   *             some other EP), then a nullptr is returned as the output param.
   *             Do not free or mutate the returned pointer as it refers to internal data owned by the underlying session.
   *             Only use it for custom kernel launching.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelContext_GetGPUComputeStream, _In_ const OrtKernelContext* context, _Outptr_ void** out);

  /// @}
  /// \name GetTensorMemoryInfo
  /// @{
  /** \brief Returns a pointer to the ::OrtMemoryInfo of a Tensor
   * \param[in] value ::OrtValue containing tensor.
   * \param[out] mem_info ::OrtMemoryInfo of the tensor. Do NOT free the returned pointer. It is valid for the lifetime of the ::OrtValue
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetTensorMemoryInfo, _In_ const OrtValue* value, _Out_ const OrtMemoryInfo** mem_info);

  /// @}
  /// \name GetExecutionProviderApi
  /// @{
  /** \brief Get a pointer to the requested version of the Execution Provider specific
   * API extensions to the OrtApi
   * \param[in] provider_name The name of the execution provider name. Currently only the following
   * values are supported: "DML".
   * \param[in] version Must be ::ORT_API_VERSION.
   * \param[out] provider_api A void pointer containing a reference to the execution provider versioned api structure.
   * For example, the provider_api pointer can be cast to the OrtDmlApi* when the provider_name is "DML".
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetExecutionProviderApi, _In_ const char* provider_name, _In_ uint32_t version, _Outptr_ const void** provider_api);

  /// @}

  /// \name SessionOptions
  /// @{
  /** \brief Set custom thread creation function
   *
   * \param[in] options Session options
   * \param[in] ort_custom_create_thread_fn Custom thread creation function
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsSetCustomCreateThreadFn, _Inout_ OrtSessionOptions* options, _In_ OrtCustomCreateThreadFn ort_custom_create_thread_fn);

  /** \brief Set creation options for custom thread
   *
   * \param[in] options Session options
   * \param[in] ort_custom_thread_creation_options Custom thread creation options (can be nullptr)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsSetCustomThreadCreationOptions, _Inout_ OrtSessionOptions* options, _In_ void* ort_custom_thread_creation_options);

  /** \brief Set custom thread join function
   *
   * \param[in] options Session options
   * \param[in] ort_custom_join_thread_fn Custom join thread function, must not be nullptr when ort_custom_create_thread_fn is set
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SessionOptionsSetCustomJoinThreadFn, _Inout_ OrtSessionOptions* options, _In_ OrtCustomJoinThreadFn ort_custom_join_thread_fn);
  /// @}

  /// \name OrtThreadingOptions
  /// @{
  /** \brief Set custom thread creation function for global thread pools
   *
   * \param[inout] tp_options
   * \param[in] ort_custom_create_thread_fn Custom thread creation function
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalCustomCreateThreadFn, _Inout_ OrtThreadingOptions* tp_options, _In_ OrtCustomCreateThreadFn ort_custom_create_thread_fn);

  /** \brief Set custom thread creation options for global thread pools
   *
   * \param[inout] tp_options
   * \param[in] ort_custom_thread_creation_options Custom thread creation options (can be nullptr)
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalCustomThreadCreationOptions, _Inout_ OrtThreadingOptions* tp_options, _In_ void* ort_custom_thread_creation_options);

  /** \brief Set custom thread join function for global thread pools
   *
   * \param[inout] tp_options
   * \param[in] ort_custom_join_thread_fn Custom thread join function, must not be nullptr when global ort_custom_create_thread_fn is set
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SetGlobalCustomJoinThreadFn, _Inout_ OrtThreadingOptions* tp_options, _In_ OrtCustomJoinThreadFn ort_custom_join_thread_fn);
  /// @}

  /** \brief Synchronize bound inputs. The call may be necessary for some providers, such as cuda,
   *   in case the system that allocated bound memory operated on a different stream. However, the
   *   operation is provider specific and could be a no-op.
   *
   * \param[inout] binding_ptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SynchronizeBoundInputs, _Inout_ OrtIoBinding* binding_ptr);

  /** \brief Synchronize bound outputs. The call may be necessary for some providers, such as cuda,
   *   in case the system that allocated bound memory operated on a different stream. However, the
   *   operation is provider specific and could be a no-op.
   *
   * \param[inout] binding_ptr
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(SynchronizeBoundOutputs, _Inout_ OrtIoBinding* binding_ptr);

  /// \name OrtSessionOptions
  /// @{

  /** \brief Append CUDA execution provider to the session options
   *
   * If CUDA is not available (due to a non CUDA enabled build), this function will return failure.
   *
   * This is slightly different from OrtApi::SessionOptionsAppendExecutionProvider_CUDA, it takes an
   * ::OrtCUDAProviderOptions which is publicly defined. This takes an opaque ::OrtCUDAProviderOptionsV2
   * which must be created with OrtApi::CreateCUDAProviderOptions.
   *
   * For OrtApi::SessionOptionsAppendExecutionProvider_CUDA, the user needs to instantiate ::OrtCUDAProviderOptions
   * as well as allocate/release buffers for some members of ::OrtCUDAProviderOptions.
   * Here, OrtApi::CreateCUDAProviderOptions and Ortapi::ReleaseCUDAProviderOptions will do the memory management for you.
   *
   * \param[in] options
   * \param[in] cuda_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.11.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_CUDA_V2,
                  _In_ OrtSessionOptions* options, _In_ const OrtCUDAProviderOptionsV2* cuda_options);

  /// @}
  /// \name OrtCUDAProviderOptionsV2
  /// @{

  /** \brief Create an OrtCUDAProviderOptionsV2
   *
   * \param[out] out Newly created ::OrtCUDAProviderOptionsV2. Must be released with OrtApi::ReleaseCudaProviderOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.11.
   */
  ORT_API2_STATUS(CreateCUDAProviderOptions, _Outptr_ OrtCUDAProviderOptionsV2** out);

  /** \brief Set options in a CUDA Execution Provider.
   *
   * Please refer to https://onnxruntime.ai/docs/execution-providers/CUDA-ExecutionProvider.html#configuration-options
   * to know the available keys and values. Key should be in null terminated string format of the member of ::OrtCUDAProviderOptionsV2
   * and value should be its related range. Recreates the options and only sets the supplied values.
   *
   * For example, key="device_id" and value="0"
   *
   * \param[in] cuda_options
   * \param[in] provider_options_keys Array of UTF-8 null-terminated string for provider options keys
   * \param[in] provider_options_values Array of UTF-8 null-terminated string for provider options values
   * \param[in] num_keys Number of elements in the `provider_option_keys` and `provider_options_values` arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.11.
   */
  ORT_API2_STATUS(UpdateCUDAProviderOptions, _Inout_ OrtCUDAProviderOptionsV2* cuda_options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /**
   * Get serialized CUDA provider options string.
   *
   * For example, "device_id=0;arena_extend_strategy=0;......"
   *
   * \param cuda_options - OrtCUDAProviderOptionsV2 instance
   * \param allocator - a ptr to an instance of OrtAllocator obtained with CreateAllocator() or GetAllocatorWithDefaultOptions()
   *                      the specified allocator will be used to allocate continuous buffers for output strings and lengths.
   * \param ptr - is a UTF-8 null terminated string allocated using 'allocator'. The caller is responsible for using the same allocator to free it.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.11.
   */
  ORT_API2_STATUS(GetCUDAProviderOptionsAsString, _In_ const OrtCUDAProviderOptionsV2* cuda_options, _Inout_ OrtAllocator* allocator, _Outptr_ char** ptr);

  /** \brief Release an ::OrtCUDAProviderOptionsV2
   *
   * \note This is an exception in the naming convention of other Release* functions, as the name of the method does not have the V2 suffix, but the type does
   *
   * \since Version 1.11.
   */
  void(ORT_API_CALL* ReleaseCUDAProviderOptions)(_Frees_ptr_opt_ OrtCUDAProviderOptionsV2* input);

  /// @}

  /** \brief Append MIGraphX provider to session options
   *
   * If MIGraphX is not available (due to a non MIGraphX enabled build, or if MIGraphX is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] migraphx_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.11.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_MIGraphX,
                  _In_ OrtSessionOptions* options, _In_ const OrtMIGraphXProviderOptions* migraphx_options);

  /** \brief Replace initialized Tensors with external data with the data provided in initializers.
   *
   * The function will find the initialized TensorProtos with external data in the graph with the provided names and
   * replace them with the provided tensors. The API verifies that the TensorProto being replaced
   * has an external data reference and has the same name, dimensions and data type as its replacement. The replacement
   * will occur before any of the optimizations take place. The data will be copied into the graph
   * since TensorProto can't refer to the user provided buffers.
   *
   * Once the model has been loaded, the OrtValue(s) added to SessionOptions instance will be removed
   * from the internal SessionOptions copy to save memory, the user provided buffers can then be deallocated
   * and the SessionOptions instance that refers to them can be destroyed.
   *
   * \param[in] options
   * \param[in] initializer_names Array of null terminated UTF-8 encoded strings of the initializers names.
   * \param[in] initializers Array of ::OrtValue type
   * \param[in] num_initializers Number of elements in the initializer_names and initializers
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(AddExternalInitializers, _In_ OrtSessionOptions* options,
                  _In_reads_(num_initializers) const char* const* initializer_names,
                  _In_reads_(num_initializers) const OrtValue* const* initializers, size_t num_initializers);

  /** \brief: Create attribute of onnxruntime operator
   *
   * \param[in] name Name of the attribute
   * \param[in] data Data content of the attribute
   * \param[in] len Number of bytes stored in data
   * \param[in] type Data type
   * \param[out] op_attr Attribute that has been created, which must be released by OrtApi::ReleaseOpAttr
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(CreateOpAttr,
                  _In_ const char* name,
                  _In_ const void* data,
                  _In_ int len,
                  _In_ OrtOpAttrType type,
                  _Outptr_ OrtOpAttr** op_attr);

  /* \brief: Release op attribute
   *
   * \param[in] opAttr Attribute created by OrtApi::CreateOpAttr
   *
   * \since Version 1.12.
   */
  ORT_CLASS_RELEASE(OpAttr);

  /** \brief: Create onnxruntime native operator
   *
   * \param[in] info Kernel info
   * \param[in] op_name Operator name
   * \param[in] domain Operator domain
   * \param[in] version Operator opset version
   * \param[in] type_constraint_names Name of the type contraints, such as "T" or "T1"
   * \param[in] type_constraint_values Type of each contraints
   * \param[in] type_constraint_count Number of contraints
   * \param[in] attr_values Attributes used to initialize the operator
   * \param[in] attr_count Number of the attributes
   * \param[in] input_count Number of inputs
   * \param[in] output_count Number of outputs
   * \param[out] ort_op Operator that has been created
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(CreateOp,
                  _In_ const OrtKernelInfo* info,
                  _In_z_ const char* op_name,
                  _In_z_ const char* domain,
                  int version,
                  _In_reads_(type_constraint_count) const char** type_constraint_names,
                  _In_reads_(type_constraint_count) const ONNXTensorElementDataType* type_constraint_values,
                  int type_constraint_count,
                  _In_reads_(attr_count) const OrtOpAttr* const* attr_values,
                  int attr_count,
                  int input_count,
                  int output_count,
                  _Outptr_ OrtOp** ort_op);

  /** \brief: Invoke the operator created by OrtApi::CreateOp
   * The inputs must follow the order as specified in onnx specification
   *
   * \param[in] context Kernel context
   * \param[in] ort_op Operator that has been created
   * \param[in] input_values Array of inputs
   * \param[in] input_count Number of inputs
   * \param[in] output_values Array of outputs
   * \param[in] output_count Number of outputs
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(InvokeOp,
                  _In_ const OrtKernelContext* context,
                  _In_ const OrtOp* ort_op,
                  _In_ const OrtValue* const* input_values,
                  _In_ int input_count,
                  _Inout_ OrtValue* const* output_values,
                  _In_ int output_count);

  /* \brief: Release an onnxruntime operator
   *
   * \param[in] Op Operator created by OrtApi::CreateOp
   *
   * \since Version 1.12.
   */
  ORT_CLASS_RELEASE(Op);

  /** \brief: Append execution provider to the session options.
   * \param[in] options
   * \param[in] provider_name - provider to add.
   * \param[in] provider_options_keys - keys to configure the provider options
   * \param[in] provider_options_values - values to configure the provider options
   * \param[in] num_keys - number of keys passed in
   *
   * Currently supported providers:
   *   QNN
   *   SNPE
   *   XNNPACK
   *
   * Note: If an execution provider has a dedicated SessionOptionsAppendExecutionProvider_<provider name> function
   *       that should be used to add it.
   *
   * QNN supported keys:
   *   "backend_path": file path to QNN backend library.
   *   "profiling_level": QNN profiling level, options: "off", "basic", "detailed". Default to off.
   *   "profiling_file_path": QNN profiling file path if ETW not enabled.
   *   "rpc_control_latency": QNN RPC control latency.
   *   "vtcm_mb": QNN VTCM size in MB. default to 0(not set).
   *   "htp_performance_mode": QNN performance mode, options: "burst", "balanced", "default", "high_performance",
   *   "high_power_saver", "low_balanced", "extreme_power_saver", "low_power_saver", "power_saver", "sustained_high_performance". Default to "default".
   *   "qnn_saver_path": File path to the QNN Saver backend library. If specified, QNN Saver will be enabled and will
   *   dump QNN API calls to disk for replay/debugging. QNN Saver produces incorrect model inference results and
   *   may alter model/EP partitioning. Use only for debugging.
   *   "qnn_context_priority": QNN context priority, options: "low", "normal", "normal_high", "high". Default to "normal".
   *   "htp_graph_finalization_optimization_mode": Set the optimization mode for graph finalization on the HTP backend. Available options:
   *     - "0": Default.
   *     - "1": Faster preparation time, less optimal graph.
   *     - "2": Longer preparation time, more optimal graph.
   *     - "3": Longest preparation time, most likely even more optimal graph. See QNN SDK documentation for specific details.
   *   "soc_model": The SoC model number. Refer to the QNN SDK documentation for valid values. Defaults to "0" (unknown).
   *   "htp_arch": The minimum HTP architecture the driver will use to select compatible QNN operators. Available options:
   *     - "0": Default (none).
   *     - "68"
   *     - "69"
   *     - "73"
   *     - "75"
   *   "device_id": The ID of the device to use when setting 'htp_arch'. Defaults to "0" (for single device).
   *   "enable_htp_fp16_precision": Used for float32 model for HTP backend.
   *   Enable the float32 model to be inferenced with fp16 precision. Otherwise, it will be fp32 precision.
   *     - "0": With fp32 precision.
   *     - "1": Default. With fp16 precision.
   *   "enable_htp_weight_sharing": Enable QNN weight sharing feature while compiling multiple graphs into one QNN context.
   *     - "0": Default. Disabled.
   *     - "1": Enabled.
   *   "offload_graph_io_quantization": Offload graph input quantization and graph output dequantization to another
   *   execution provider (typically CPU EP).
   *     - "0": Disabled. QNN EP will handle quantization and dequantization of graph I/O.
   *     - "1": Enabled. This is the default value.
   *   "enable_htp_spill_fill_buffer": Enable HTP spill fill buffer setting. The flag is used while generating context binary.
   *     - "0": Default. Disabled.
   *     - "1": Enabled.
   *   "enable_htp_shared_memory_allocator": Enable the QNN HTP shared memory allocator. Requires libcdsprpc.so/dll to
   *   be available.
   *     - "0": Default. Disabled.
   *     - "1": Enabled.
   *   "dump_json_qnn_graph": Set to "1" to dump QNN graphs generated by QNN EP as JSON files. Each graph partition
   *      assigned to QNN EP is dumped to a separate file.
   *   "json_qnn_graph_dir": Directory in which to dump QNN JSON graphs. If not specified, QNN graphs are dumped in the
   *      program's current working directory. Ignored if "dump_json_qnn_graph" is not set.
   *
   * SNPE supported keys:
   *   "runtime": SNPE runtime engine, options: "CPU", "CPU_FLOAT32", "GPU", "GPU_FLOAT32_16_HYBRID", "GPU_FLOAT16",
   *   "DSP", "DSP_FIXED8_TF", "AIP_FIXED_TF", "AIP_FIXED8_TF".
   *   Mapping to SNPE Runtime_t definition: CPU, CPU_FLOAT32 => zdl::DlSystem::Runtime_t::CPU;
   *   GPU, GPU_FLOAT32_16_HYBRID => zdl::DlSystem::Runtime_t::GPU;
   *   GPU_FLOAT16 => zdl::DlSystem::Runtime_t::GPU_FLOAT16;
   *   DSP, DSP_FIXED8_TF => zdl::DlSystem::Runtime_t::DSP.
   *   AIP_FIXED_TF, AIP_FIXED8_TF => zdl::DlSystem::Runtime_t::AIP_FIXED_TF.
   *   "priority": execution priority, options: "low", "normal".
   *   "buffer_type": ITensor or user buffers, options: "ITENSOR", user buffer with different types - "TF8", "TF16", "UINT8", "FLOAT".
   *   "ITENSOR" -- default, ITensor which is float only.
   *   "TF8" -- quantized model required, "FLOAT" -- for both quantized or non-quantized model
   *   "enable_init_cache": enable SNPE init caching feature, set to 1 to enabled it. Disabled by default.
   *   If SNPE is not available (due to a non Snpe enabled build or its dependencies not being installed), this function will fail.
   *
   * XNNPACK supported keys:
   *   "intra_op_num_threads": number of thread-pool size to use for XNNPACK execution provider.
   *      default value is 0, which means to use the session thread-pool size.
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider, _In_ OrtSessionOptions* options,
                  _In_ const char* provider_name,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /* \brief: Get a copy of kernel info
   *
   * \param[in] info Kernel info
   * \param[out] info_copy Copy of kernel info
   *
   * \since Version 1.12.
   */
  ORT_API2_STATUS(CopyKernelInfo,
                  _In_ const OrtKernelInfo* info,
                  _Outptr_ OrtKernelInfo** info_copy);

  /* \brief: Release kernel info
   *
   * \param[in] KernelInfo A copy of kernel info returned by CopyKernelInfo
   *
   * \since Version 1.12.
   */
  ORT_CLASS_RELEASE(KernelInfo);

  /// \name Ort Training
  /// @{
  /** \brief Gets the Training C Api struct
   *
   * Call this function to access the ::OrtTrainingApi structure that holds pointers to functions that enable
   * training with onnxruntime.
   * \note A NULL pointer will be returned and no error message will be printed if the training api
   * is not supported with this build. A NULL pointer will be returned and an error message will be
   * printed if the provided version is unsupported, for example when using a runtime older than the
   * version created with this header file.
   *
   * \param[in] version Must be ::ORT_API_VERSION
   * \return The ::OrtTrainingApi struct for the version requested.
   *
   * \since Version 1.13
   */
  const OrtTrainingApi*(ORT_API_CALL* GetTrainingApi)(uint32_t version)NO_EXCEPTION;

  /// @}

  /** \brief Append CANN provider to session options
   *
   * If CANN is not available (due to a non CANN enabled build, or if CANN is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] cann_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.13.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_CANN,
                  _In_ OrtSessionOptions* options, _In_ const OrtCANNProviderOptions* cann_options);

  /** \brief Create an OrtCANNProviderOptions
   *
   * \param[out] out created ::OrtCANNProviderOptions. Must be released with OrtApi::ReleaseCANNProviderOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.13.
   */
  ORT_API2_STATUS(CreateCANNProviderOptions, _Outptr_ OrtCANNProviderOptions** out);

  /** \brief Set options in a CANN Execution Provider.
   *
   * \param[in] cann_options
   * \param[in] provider_options_keys Array of UTF-8 null-terminated string for provider options keys
   * \param[in] provider_options_values Array of UTF-8 null-terminated string for provider options values
   * \param[in] num_keys Number of elements in the `provider_option_keys` and `provider_options_values` arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.13.
   */
  ORT_API2_STATUS(UpdateCANNProviderOptions, _Inout_ OrtCANNProviderOptions* cann_options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /** \brief Get serialized CANN provider options string.
   *
   * \param[in] cann_options OrtCANNProviderOptions instance
   * \param[in] allocator a ptr to an instance of OrtAllocator obtained with CreateAllocator()
   *                      or GetAllocatorWithDefaultOptions(), the specified allocator will be used to allocate
   *                      continuous buffers for output strings and lengths.
   * \param[out] ptr is a UTF-8 null terminated string allocated using 'allocator'.
   *                 The caller is responsible for using the same allocator to free it.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.13.
   */
  ORT_API2_STATUS(GetCANNProviderOptionsAsString, _In_ const OrtCANNProviderOptions* cann_options,
                  _Inout_ OrtAllocator* allocator, _Outptr_ char** ptr);

  /** \brief Release an OrtCANNProviderOptions
   *
   * \param[in] input The pointer of OrtCANNProviderOptions which will been deleted
   *
   * \since Version 1.13.
   */
  void(ORT_API_CALL* ReleaseCANNProviderOptions)(_Frees_ptr_opt_ OrtCANNProviderOptions* input);

  /*  \brief Get OrtDevice type from MemoryInfo
   *
   *  \since Version 1.14
   */
  void(ORT_API_CALL* MemoryInfoGetDeviceType)(_In_ const OrtMemoryInfo* ptr, _Out_ OrtMemoryInfoDeviceType* out);

  /* \brief Update the OrtEnv instance with custom log severity level
   *
   * \param[in] ort_env The OrtEnv instance being used
   * \param[in] log_severity_level The log severity level.
   *
   * \since Version 1.14.
   */
  ORT_API2_STATUS(UpdateEnvWithCustomLogLevel, _In_ OrtEnv* ort_env, OrtLoggingLevel log_severity_level);

  /*  \brief Set affinities for intra op threads
   *
   * Affinity string follows format:
   * logical_processor_id,logical_processor_id;logical_processor_id,logical_processor_id
   * Semicolon isolates configurations among threads, while comma split processors where ith thread expected to attach to.
   * e.g. 1,2,3;4,5
   * specifies affinities for two threads, with the 1st thread attach to the 1st, 2nd, and 3rd processor, and 2nd thread to the 4th and 5th.
   * To ease the configuration, an "interval" is also allowed:
   * e.g. 1-8;8-16;17-24
   * orders that the 1st thread runs on first eight processors, 2nd thread runs on next eight processors, and so forth.
   * Note:
   * 1. Once set, the number of thread affinities must equal to intra_op_num_threads - 1,
   *    ort does not set affinity on the main thread which is started and managed by the calling app;
   * 2. For windows, ort will infer the group id from a logical processor id, for example, assuming there are two groups with each has 64 logical processors,
   *    an id of 64 will be inferred as the last processor of the 1st group, while 65 will be interpreted as the 1st processor of the second group.
   *    Hence 64-65 is an invalid configuration, because a windows thread cannot be attached to processors across group boundary.
   *
   *  \since Version 1.14
   */
  ORT_API2_STATUS(SetGlobalIntraOpThreadAffinity, _Inout_ OrtThreadingOptions* tp_options, const char* affinity_string);

  /** \brief Register custom ops from a shared library.
   *
   * Loads a shared library (.dll on windows, .so on linux, etc) named 'library_name' and looks for this entry point:
   *		OrtStatus* RegisterCustomOps(OrtSessionOptions * options, const OrtApiBase* api);
   * It then passes in the provided session options to this function along with the api base.
   *
   * The handle to the loaded library is automatically released by ORT when the last OrtSession that references the
   * library handle is released. If no OrtSession is created, then the library handle is released when the provided
   * OrtSessionOptions is released.
   *
   * \param[in] options The session options.
   * \param[in] library_name The name of the shared library to load and register. Refer to OS-specific dynamic library
   *                         loading utilities (e.g., LoadLibraryEx on Windows or dlopen on Linux/MacOS) for information
   *                         on the format of library names and search paths.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(RegisterCustomOpsLibrary_V2, _Inout_ OrtSessionOptions* options, _In_ const ORTCHAR_T* library_name);

  /** \brief Register custom ops by calling a RegisterCustomOpsFn function.
   *
   * Searches for registration_func_name and if found calls it.
   *
   * The library containing the function must either be linked against or previously loaded by the executable.
   *
   * If you want ONNX Runtime to load the library and manage its lifetime, use RegisterCustomOpsLibrary_V2.
   *
   * RegisterCustomOpsUsingFunction can be used in scenarios where it may not be possible for ONNX Runtime to load
   * the library from a path. e.g. mobile platforms where the library must be linked into the app.
   *
   * The registration function must have the signature of RegisterCustomOpsFn:
   *    OrtStatus* (*fn)(OrtSessionOptions* options, const OrtApiBase* api);
   *
   * See https://onnxruntime.ai/docs/reference/operators/add-custom-op.html for details on how the registration
   * function should be implemented.
   *
   * \param[in] options OrtSessionOptions that is passed through as the first argument in the call to the
   *                    registration function.
   * \param[in] registration_func_name Name of registration function to use.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(RegisterCustomOpsUsingFunction, _Inout_ OrtSessionOptions* options,
                  _In_ const char* registration_func_name);

  /// \name OrtKernelInfo
  /// Custom operator APIs.
  /// @{

  /** \brief Get the number of inputs from ::OrtKernelInfo.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query the number of inputs
   * during kernel/session creation.
   *
   * \param[in] info Instance of ::OrtKernelInfo.
   * \param[out] out Pointer to variable assigned with the result on success.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetInputCount, _In_ const OrtKernelInfo* info, _Out_ size_t* out);

  /** \brief Get the number of outputs from ::OrtKernelInfo.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query the number of outputs
   * during kernel/session creation.
   *
   * \param[in] info Instance of ::OrtKernelInfo.
   * \param[out] out Pointer to variable assigned with the result on success.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetOutputCount, _In_ const OrtKernelInfo* info, _Out_ size_t* out);

  /** \brief Get the name of a ::OrtKernelInfo's input.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query an input's name
   * during kernel/session creation.
   *
   * If `out` is nullptr, the value of `size` is set to the size of the name
   * string (including null-terminator), and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the name string's size,
   * the value of `size` is set to the true size of the string (including null-terminator),
   * the provided memory is filled with the string's contents, and a success status is returned.
   *
   * If the `size` parameter is less than the actual string's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the string
   * and a failure status is returned.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[in] index The index of the input name to get. Returns a failure status if out-of-bounds.
   * \param[out] out Memory location into which to write the UTF-8 null-terminated string representing the input's name.
   * \param[in,out] size Pointer to the size of the `out` buffer. See above comments for details.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetInputName, _In_ const OrtKernelInfo* info, size_t index, _Out_ char* out,
                  _Inout_ size_t* size);

  /** \brief Get the name of a ::OrtKernelInfo's output.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query an output's name
   * during kernel/session creation.
   *
   * If `out` is nullptr, the value of `size` is set to the size of the name
   * string (including null-terminator), and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the name string's size,
   * the value of `size` is set to the true size of the string (including null-terminator),
   * the provided memory is filled with the string's contents, and a success status is returned.
   *
   * If the `size` parameter is less than the actual string's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the string
   * and a failure status is returned.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[in] index The index of the output name to get. Returns a failure status if out-of-bounds.
   * \param[out] out Memory location into which to write the UTF-8 null-terminated string representing the output's
   *                 name.
   * \param[in,out] size Pointer to the size of the `out` buffer. See above comments for details.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetOutputName, _In_ const OrtKernelInfo* info, size_t index, _Out_ char* out,
                  _Inout_ size_t* size);

  /** \brief Get the type information for a ::OrtKernelInfo's input.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query the shape and type information
   * of an input during kernel/session creation.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[in] index Which input to get the type information for
   * \param[out] type_info Pointer set to the resulting ::OrtTypeInfo. Must be freed with OrtApi::ReleaseTypeInfo.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetInputTypeInfo, _In_ const OrtKernelInfo* info, size_t index,
                  _Outptr_ OrtTypeInfo** type_info);

  /** \brief Get the type information for a ::OrtKernelInfo's output.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to query the shape and type information
   * of an output during kernel/session creation.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[in] index Which input to get the type information for
   * \param[out] type_info Pointer set to the resulting ::OrtTypeInfo. Must be freed with OrtApi::ReleaseTypeInfo.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(KernelInfo_GetOutputTypeInfo, _In_ const OrtKernelInfo* info, size_t index,
                  _Outptr_ OrtTypeInfo** type_info);

  /** \brief Get a ::OrtValue tensor stored as an attribute in the graph node.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to get a tensor attribute.
   *
   * \param[in] info ::OrtKernelInfo instance.
   * \param[in] name UTF-8 null-terminated string representing the attribute's name.
   * \param[in] allocator Allocator used to allocate the internal tensor state.
   * \param[out] out Returns newly created ::OrtValue. Must be freed with OrtApi::ReleaseValue,
   *                 which will also free internal tensor state allocated with the provided allocator.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(KernelInfoGetAttribute_tensor, _In_ const OrtKernelInfo* info, _In_z_ const char* name,
                  _Inout_ OrtAllocator* allocator, _Outptr_ OrtValue** out);

  /// @}
  /// \name OrtSessionOptions
  /// Custom operator APIs
  /// @{

  /** \brief Checks if the given session configuration entry exists.
   *
   * The config_key formats are defined in onnxruntime_session_options_config_keys.h
   *
   * Can be used in a custom operator library to check for session configuration entries
   * that target one or more custom operators in the library. Example: The config entry
   * custom_op.myop.some_key targets a custom op named "myop".
   *
   * \param[in] options The ::OrtSessionOptions instance.
   * \param[in] config_key A null-terminated UTF-8 string representation of the configuration key.
   * \param[out] out Pointer set to 1 if the entry exists and 0 otherwise.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(HasSessionConfigEntry, _In_ const OrtSessionOptions* options,
                  _In_z_ const char* config_key, _Out_ int* out);

  /** \brief Get a session configuration value.
   *
   * Returns a failure status if the configuration key does not exist.
   * The config_key and the format of config_value are defined in onnxruntime_session_options_config_keys.h
   *
   * If `config_value` is nullptr, the value of `size` is set to the true size of the string
   * value (including null-terminator), and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the actual string value's size,
   * the value of `size` is set to the true size of the string value, the provided memory
   * is filled with the value's contents, and a success status is returned.
   *
   * If the `size` parameter is less than the actual string value's size and `config_value`
   * is not nullptr, the value of `size` is set to the true size of the string value
   * and a failure status is returned.
   *
   * Can be used in a custom operator library to get session configuration entries
   * that target one or more custom operators in the library. Example: The config entry
   * custom_op.myop.some_key targets a custom op named "myop".
   *
   * \param[in] options The session options.
   * \param[in] config_key A null-terminated UTF-8 string representation of the config key.
   * \param[in] config_value Pointer to memory where the null-terminated UTF-8 string value will be stored.
   * \param[in,out] size Pointer to the size of the `config_value` buffer. See above comments for details.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.14
   */
  ORT_API2_STATUS(GetSessionConfigEntry, _In_ const OrtSessionOptions* options,
                  _In_z_ const char* config_key, _Out_ char* config_value, _Inout_ size_t* size);

  /// @}

  /** \brief Append dnnl provider to session options
   *
   * If oneDNN is not available, this function will return failure.
   *
   * \param[in] options
   * \param[in] dnnl_options
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_Dnnl,
                  _In_ OrtSessionOptions* options, _In_ const OrtDnnlProviderOptions* dnnl_options);

  /** \brief Create an OrtDnnlProviderOptions
   *
   * \param[out] out Newly created ::OrtDnnlProviderOptions. Must be released with OrtApi::ReleaseDnnlProviderOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(CreateDnnlProviderOptions, _Outptr_ OrtDnnlProviderOptions** out);

  /** \brief Set options in a oneDNN Execution Provider.
   *
   * Key should be in null terminated string format of the member of ::OrtDnnlProviderOptions
   * and value should be its related range.
   *
   * For example, key="use_arena" and value="1"
   *
   * \param[in] dnnl_options
   * \param[in] provider_options_keys Array of UTF-8 null-terminated string for provider options keys
   * \param[in] provider_options_values Array of UTF-8 null-terminated string for provider options values
   * \param[in] num_keys Number of elements in the `provider_option_keys` and `provider_options_values` arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(UpdateDnnlProviderOptions, _Inout_ OrtDnnlProviderOptions* dnnl_options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /**
   * Get serialized oneDNN provider options string.
   *
   * For example, "use_arena=1;......"
   *
   * \param dnnl_options - OrtDnnlProviderOptions instance
   * \param allocator - a ptr to an instance of OrtAllocator obtained with CreateAllocator() or GetAllocatorWithDefaultOptions()
   *                      the specified allocator will be used to allocate continuous buffers for output strings and lengths.
   * \param ptr - is a UTF-8 null terminated string allocated using 'allocator'. The caller is responsible for using the same allocator to free it.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(GetDnnlProviderOptionsAsString, _In_ const OrtDnnlProviderOptions* dnnl_options, _Inout_ OrtAllocator* allocator, _Outptr_ char** ptr);

  /** \brief Release an ::OrtDnnlProviderOptions
   *
   * \since Version 1.15.
   */
  void(ORT_API_CALL* ReleaseDnnlProviderOptions)(_Frees_ptr_opt_ OrtDnnlProviderOptions* input);

  /// \name OrtKernelInfo
  /// Custom operator APIs.
  /// @{

  /** \brief Get the graph node name from ::OrtKernelInfo.
   *
   * If `out` is nullptr, the value of `size` is set to the size of the name
   * string (including null-terminator), and a success status is returned.
   *
   * If the `size` parameter is greater than or equal to the name string's size,
   * the value of `size` is set to the true size of the string (including null-terminator),
   * the provided memory is filled with the string's contents, and a success status is returned.
   *
   * If the `size` parameter is less than the actual string's size and `out`
   * is not nullptr, the value of `size` is set to the true size of the string
   * and a failure status is returned.
   *
   * Can be used in a custom operator's CreateKernel callback to get the name of the operator's node name in the graph.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[out] out Memory location into which to write the UTF-8 null-terminated string representing the name.
   * \param[in,out] size Pointer to the size of the `out` buffer. See above comments for details.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.15
   */
  ORT_API2_STATUS(KernelInfo_GetNodeName, _In_ const OrtKernelInfo* info, _Out_ char* out, _Inout_ size_t* size);

  /** \brief Get the session logger from ::OrtKernelInfo.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to get a logger that can be used to log
   * messages.
   *
   * \param[in] info An instance of ::OrtKernelInfo.
   * \param[out] logger Pointer set to the session's ::OrtLogger. Owned by ONNX Runtime, so do not free.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.15
   */
  ORT_API2_STATUS(KernelInfo_GetLogger, _In_ const OrtKernelInfo* info, _Outptr_ const OrtLogger** logger);

  /// @}
  /// \name OrtKernelContext
  /// Custom operator APIs.
  /// @{

  /** \brief Get the runtime logger from ::OrtKernelContext.
   *
   * Used in the KernelCompute callback of an OrtCustomOp to get a logger that can be used to log
   * messages during inference.
   *
   * \param[in] context An instance of ::OrtKernelContext.
   * \param[out] logger Pointer set to the kernel context's ::OrtLogger. Owned by ONNX Runtime, so do not free.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.15
   */
  ORT_API2_STATUS(KernelContext_GetLogger, _In_ const OrtKernelContext* context, _Outptr_ const OrtLogger** logger);

  /// @}
  /// \name OrtLogger
  /// Custom operator APIs.
  /// @{

  /** \brief Logs a message at the given severity level using the provided ::OrtLogger.
   *
   * Only messages with a severity level equal or greater than the ::OrtLogger's logging severity level
   * are logged. Use OrtApi::Logger_GetLoggingSeverityLevel to get the ::OrtLogger's logging severity
   * level.
   *
   * Can be used in custom operators to log messages with the logger retrieved via OrtApi::KernelInfo_GetLogger.
   *
   * \param[in] logger The ::OrtLogger instance.
   * \param[in] log_severity_level The message's severity level.
   * \param[in] message The message to log.
   * \param[in] file_path The filepath of the file in which the message is logged. Usually the value of ORT_FILE.
   * \param[in] line_number The file line number in which the message is logged. Usually the value of __LINE__.
   * \param[in] func_name The name of the function in which the message is logged. Usually the value of __FUNCTION__.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.15
   */
  ORT_API2_STATUS(Logger_LogMessage, _In_ const OrtLogger* logger, OrtLoggingLevel log_severity_level,
                  _In_z_ const char* message, _In_z_ const ORTCHAR_T* file_path, int line_number,
                  _In_z_ const char* func_name);

  /** \brief Get the logging severity level of the ::OrtLogger.
   *
   * Can be used in a custom operator to get the logging serverity level of the ::OrtLogger associated with
   * the ::OrtKernelInfo.
   *
   * \param[in] logger The ::OrtLogger instance.
   * \param[out] out Pointer to variable assigned with the logging severity level on success.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.15
   */
  ORT_API2_STATUS(Logger_GetLoggingSeverityLevel, _In_ const OrtLogger* logger, _Out_ OrtLoggingLevel* out);

  /// @}

  /** \brief Get a ::OrtValue tensor stored as a constant initializer in the graph node.
   *
   * Used in the CreateKernel callback of an OrtCustomOp to get a tensor value.
   *
   * \param[in] info ::OrtKernelInfo instance.
   * \param[in] index The node index.
   * \param[out] is_constant Is it a constant node input or not.
   * \param[out] out The OrtValue tensor value.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(KernelInfoGetConstantInput_tensor, _In_ const OrtKernelInfo* info, size_t index, _Out_ int* is_constant, _Outptr_ const OrtValue** out);

  /** \brief Get Optional Type information from an ::OrtTypeInfo
   *
   * This augments ::OrtTypeInfo to return an ::OrtOptionalTypeInfo when the type is optional.
   * The OrtOptionalTypeInfo also has a nested ::OrtTypeInfo that describes the type of the optional value.
   * ::OrtOptionalTypeInfo type can only appear within model metadata to describe inputs/outputs.
   * The actual OrtValues that are supplied in place of optional type inputs should contain
   * specific type that is described by ::OrtOptionalTypeInfo.
   *
   * So the picture: ::OrtTypeInfo -> ::OrtOptionalTypeInfo -> ::OrtTypeInfo (describes the type that can be supplied
   * in place of the optional type when creating the actual ::OrtValue).
   *
   * \param[in] type_info
   * \param[out] out A pointer to the ::OrtOptionalTypeInfo. Do not free this value,
   *                 it is owned by OrtTypeInfo instance. When the type_info does not represent
   *                 optional type, nullptr is returned in out.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(CastTypeInfoToOptionalTypeInfo, _In_ const OrtTypeInfo* type_info,
                  _Outptr_result_maybenull_ const OrtOptionalTypeInfo** out);

  /** \brief Get OrtTypeInfo for the allowed contained type from an ::OrtOptionalTypeInfo.
   *
   * This augments ::OrtOptionalTypeInfo to return an ::OrtTypeInfo for the contained type.
   * The OrtOptionalTypeInfo has a nested ::OrtTypeInfo that describes the type of the optional value.
   * ::OrtOptionalTypeInfo type can only appear within model metadata to describe inputs/outputs.
   * The actual OrtValues that are supplied in place of optional type inputs should contain
   * specific type that is described by the returned ::OrtTypeInfo.
   *
   * \param[in] optional_type_info
   * \param[out] out A copy of ::OrtTypeInfo for what the optional value could be.
   *                 The user must free this value with ReleaseTypeInfo.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(GetOptionalContainedTypeInfo, _In_ const OrtOptionalTypeInfo* optional_type_info,
                  _Outptr_ OrtTypeInfo** out);

  /** \brief Set a single string in a string tensor
   *  Do not zero terminate the string data.
   *
   * \param[in] value A string tensor
   * \param[in] index - flat index of the element
   * \param[in] length_in_bytes length of the buffer in utf-8 bytes (without the null terminator)
   * \param[inout] buffer - address of return value
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   */
  ORT_API2_STATUS(GetResizedStringTensorElementBuffer, _Inout_ OrtValue* value, _In_ size_t index, _In_ size_t length_in_bytes, _Inout_ char** buffer);

  /** \brief Get Allocator from KernelContext for a specific memoryInfo. Please use C API ReleaseAllocator to release out object
   *
   * \param[in] context OrtKernelContext instance
   * \param[in] mem_info OrtMemoryInfo instance
   * \param[out] out A pointer to OrtAllocator.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.15.
   */
  ORT_API2_STATUS(KernelContext_GetAllocator, _In_ const OrtKernelContext* context, _In_ const OrtMemoryInfo* mem_info, _Outptr_ OrtAllocator** out);

  /** \brief Returns a null terminated string of the build info including git info and cxx flags
   *
   * \return UTF-8 encoded version string. Do not deallocate the returned buffer.
   *
   * \since Version 1.15.
   */
  const char*(ORT_API_CALL* GetBuildInfoString)(void);

  /// \name OrtROCMProviderOptions
  /// @{

  /** \brief Create an OrtROCMProviderOptions
   *
   * \param[out] out Newly created ::OrtROCMProviderOptions. Must be released with OrtApi::ReleaseROCMProviderOptions
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(CreateROCMProviderOptions, _Outptr_ OrtROCMProviderOptions** out);

  /** \brief Set options in a ROCm Execution Provider.
   *
   * Please refer to https://onnxruntime.ai/docs/execution-providers/ROCm-ExecutionProvider.html
   * to know the available keys and values. Key should be in null terminated string format of the member of
   * ::OrtROCMProviderOptions and value should be its related range.
   *
   * For example, key="device_id" and value="0"
   *
   * \param[in] rocm_options
   * \param[in] provider_options_keys Array of UTF-8 null-terminated string for provider options keys
   * \param[in] provider_options_values Array of UTF-8 null-terminated string for provider options values
   * \param[in] num_keys Number of elements in the `provider_option_keys` and `provider_options_values` arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(UpdateROCMProviderOptions, _Inout_ OrtROCMProviderOptions* rocm_options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /**
   * Get serialized ROCm provider options string.
   *
   * For example, "device_id=0;arena_extend_strategy=0;......"
   *
   * \param rocm_options - OrtROCMProviderOptions instance
   * \param allocator - a ptr to an instance of OrtAllocator obtained with CreateAllocator() or GetAllocatorWithDefaultOptions()
   *                      the specified allocator will be used to allocate continuous buffers for output strings and lengths.
   * \param ptr - is a UTF-8 null terminated string allocated using 'allocator'. The caller is responsible for using the same allocator to free it.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(GetROCMProviderOptionsAsString, _In_ const OrtROCMProviderOptions* rocm_options, _Inout_ OrtAllocator* allocator, _Outptr_ char** ptr);

  /** \brief Release an ::OrtROCMProviderOptions
   *
   * \note This is an exception in the naming convention of other Release* functions, as the name of the method does not have the V2 suffix, but the type does
   *
   * \since Version 1.16.
   */
  void(ORT_API_CALL* ReleaseROCMProviderOptions)(_Frees_ptr_opt_ OrtROCMProviderOptions* input);

  /** \brief Create an allocator with specific type and register it with the ::OrtEnv
   *  This API enhance CreateAndRegisterAllocator that it can create an allocator with specific type, not just CPU allocator
   *  Enables sharing the allocator between multiple sessions that use the same env instance.
   *  Lifetime of the created allocator will be valid for the duration of the environment.
   *  Returns an error if an allocator with the same ::OrtMemoryInfo is already registered.
   *  \param[in] env OrtEnv instance
   *  \param[in] provider_type ExecutionProvider type
   *  \param[in] mem_info OrtMemoryInfo instance
   *  \param[in] arena_cfg Arena configuration
   *  \param[in] provider_options_keys key of the provider options map
   *  \param[in] provider_options_values value of the provider options map
   *  \param[in] num_keys Length of the provider options map
   */
  ORT_API2_STATUS(CreateAndRegisterAllocatorV2, _Inout_ OrtEnv* env, _In_ const char* provider_type, _In_ const OrtMemoryInfo* mem_info, _In_ const OrtArenaCfg* arena_cfg,
                  _In_reads_(num_keys) const char* const* provider_options_keys, _In_reads_(num_keys) const char* const* provider_options_values, _In_ size_t num_keys);

  /** \brief Run the model asynchronously in a thread owned by intra op thread pool
   *
   * \param[in] session
   * \param[in] run_options If nullptr, will use a default ::OrtRunOptions
   * \param[in] input_names Array of null terminated UTF8 encoded strings of the input names
   * \param[in] input Array of ::OrtValue%s of the input values
   * \param[in] input_len Number of elements in the input_names and inputs arrays
   * \param[in] output_names Array of null terminated UTF8 encoded strings of the output names
   * \param[in] output_names_len Number of elements in the output_names and outputs array
   * \param[out] output OrtValue* array of size output_names_len.
   *             On calling RunAsync, output[i] could either be a null or a pointer to a preallocated OrtValue.
   *             Later, the output array will be passed to run_async_callback with all null(s) filled with valid
   *             OrtValue pointer(s) allocated by onnxruntime.
   *             NOTE: it is customer's duty to finally release the output array and each of its member,
   *             regardless of whether the member (OrtValue*) is allocated by onnxruntime or preallocated by the customer.
   * \param[in] run_async_callback Callback function on model run completion
   * \param[in] user_data User data that pass back to run_async_callback
   */
  ORT_API2_STATUS(RunAsync, _Inout_ OrtSession* session, _In_opt_ const OrtRunOptions* run_options,
                  _In_reads_(input_len) const char* const* input_names,
                  _In_reads_(input_len) const OrtValue* const* input, size_t input_len,
                  _In_reads_(output_names_len) const char* const* output_names, size_t output_names_len,
                  _Inout_updates_all_(output_names_len) OrtValue** output,
                  _In_ RunAsyncCallbackFn run_async_callback, _In_opt_ void* user_data);

  /**
   * Update TensorRT EP provider option where its data type is pointer, for example 'user_compute_stream'.
   * If the data type of the provider option can be represented by string please use UpdateTensorRTProviderOptions.
   *
   * Note: It's caller's responsibility to properly manage the lifetime of the instance pointed by this pointer.
   *
   * \param tensorrt_options - OrtTensorRTProviderOptionsV2 instance
   * \param key - Name of the provider option
   * \param value - A pointer to the instance that will be assigned to this provider option
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(UpdateTensorRTProviderOptionsWithValue, _Inout_ OrtTensorRTProviderOptionsV2* tensorrt_options, _In_ const char* key, _In_ void* value);

  /**
   * Get TensorRT EP provider option where its data type is pointer.
   * If the data type of the provider option can be represented by string please use GetTensorRTProviderOptionsAsString.
   *
   * \param tensorrt_options - OrtTensorRTProviderOptionsV2 instance
   * \param key - Name of the provider option
   * \param ptr - A pointer to the instance that is kept by the provider option
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(GetTensorRTProviderOptionsByName, _In_ const OrtTensorRTProviderOptionsV2* tensorrt_options, _In_ const char* key, _Outptr_ void** ptr);

  /**
   * Update CUDA EP provider option where its data type is pointer, for example 'user_compute_stream'.
   * If the data type of the provider option can be represented by string please use UpdateCUDAProviderOptions.
   *
   * Note: It's caller's responsibility to properly manage the lifetime of the instance pointed by this pointer.
   *
   * \param cuda_options - OrtCUDAProviderOptionsV2 instance
   * \param key - Name of the provider option
   * \param value - A pointer to the instance that will be assigned to this provider option
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(UpdateCUDAProviderOptionsWithValue, _Inout_ OrtCUDAProviderOptionsV2* cuda_options, _In_ const char* key, _In_ void* value);

  /**
   * Get CUDA EP provider option where its data type is pointer.
   * If the data type of the provider option can be represented by string please use GetCUDAProviderOptionsAsString.
   *
   * \param cuda_options - OrtCUDAProviderOptionsV2 instance
   * \param key - Name of the provider option
   * \param ptr - A pointer to the instance that is kept by the provider option
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(GetCUDAProviderOptionsByName, _In_ const OrtCUDAProviderOptionsV2* cuda_options, _In_ const char* key, _Outptr_ void** ptr);

  /**
   * Get a EP resource.
   * E.g. a cuda stream or a cublas handle
   *
   * \param context - Kernel context
   * \param resource_version - Version of the resource
   * \param resource_id - Type of resource
   * \param resource - A pointer to returned resource
   *
   * \since Version 1.16.
   */
  ORT_API2_STATUS(KernelContext_GetResource, _In_ const OrtKernelContext* context, _In_ int resource_version,
                  _In_ int resource_id, _Outptr_ void** resource);

  /** \brief Set user logging function
   *
   *  By default the logger created by the CreateEnv* functions is used to create the session logger as well.
   *  This function allows a user to override this default session logger with a logger of their own choosing. This way
   *  the user doesn't have to create a separate environment with a custom logger. This addresses the problem when
   *  the user already created an env but now wants to use a different logger for a specific session (for debugging or
   *  other reasons).
   *
   * \param[in] options
   * \param[in] user_logging_function A pointer to a logging function.
   * \param[in] user_logging_param A pointer to arbitrary data passed as the ::OrtLoggingFunction `param` parameter to
   *                         `user_logging_function`. This parameter is optional.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(SetUserLoggingFunction, _Inout_ OrtSessionOptions* options,
                  _In_ OrtLoggingFunction user_logging_function, _In_opt_ void* user_logging_param);

  /**
   * Get number of input from OrtShapeInferContext
   *
   * \param[in] context
   * \param[out] out The number of inputs
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(ShapeInferContext_GetInputCount, _In_ const OrtShapeInferContext* context, _Out_ size_t* out);

  /**
   * Get type and shape info of an input
   *
   * \param[in] context
   * \param[in] index The index of the input
   * \param[out] info Type shape info of the input
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(ShapeInferContext_GetInputTypeShape, _In_ const OrtShapeInferContext* context, _In_ size_t index, _Outptr_ OrtTensorTypeAndShapeInfo** info);

  /**
   * Get attribute from OrtShapeInferContext. Note that OrtShapeInferContext is a per-node context, one could only read attribute from current node.
   *
   * \param[in] context
   * \param[in] attr_name Name of the attribute
   * \param[out] attr Handle of the attribute fetched
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(ShapeInferContext_GetAttribute, _In_ const OrtShapeInferContext* context, _In_ const char* attr_name, _Outptr_ const OrtOpAttr** attr);

  /**
   * Set type and shape info of an output
   *
   * \param[in] context
   * \param[in] index The index of the output
   * \param[out] info Type shape info of the output
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(ShapeInferContext_SetOutputTypeShape, _In_ const OrtShapeInferContext* context, _In_ size_t index, _In_ const OrtTensorTypeAndShapeInfo* info);

  /**
   * Set symbolic shape to type shape info
   *
   * \param[in] info Type shape info
   * \param[in] dim_params Symbolic strings
   * \param[in] dim_params_length Number of strings
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(SetSymbolicDimensions, _In_ OrtTensorTypeAndShapeInfo* info, _In_ const char* dim_params[], _In_ size_t dim_params_length);

  /**
   * Read contents of an attribute to data
   *
   * \param[in] op_attr
   * \param[in] type Attribute type
   * \param[out] data Memory address to save raw content of the attribute
   * \param[in] len Number of bytes allowed to store in data
   * \param[out] out Number of bytes required to save the data when the call failed, or the real number of bytes saved to data on success
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(ReadOpAttr, _In_ const OrtOpAttr* op_attr, _In_ OrtOpAttrType type, _Inout_ void* data, _In_ size_t len, _Out_ size_t* out);

  /** \brief Set whether to use deterministic compute.
   *
   * Default is false. If set to true, this will enable deterministic compute for GPU kernels where possible.
   * Note that this most likely will have a performance cost.
   *
   * \param[in] options
   * \param[in] value
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(SetDeterministicCompute, _Inout_ OrtSessionOptions* options, bool value);

  /**
   * Run fn in parallel
   *
   * \param[in] context
   * \param[in] fn Function accepting usr_data and an integer as iterator
   * \param[in] total The number of times fn is to be invoked
   * \param[in] num_batch Number of batches by which the "total" is to be divided in maximum. When zero, there is no limit
   * \param[in] usr_data User data to be passed back to fn
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(KernelContext_ParallelFor, _In_ const OrtKernelContext* context, _In_ void (*fn)(void*, size_t), _In_ size_t total, _In_ size_t num_batch, _In_ void* usr_data);

  /** \brief Append OpenVINO execution provider to the session options
   *
   * If OpenVINO is not available (due to a non OpenVINO enabled build, or if OpenVINO is not installed on the system), this function will fail.
   *
   * \param[in] options
   * \param[in] provider_options_keys
   * \param[in] provider_options_values
   * \param[in] num_keys
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.17.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_OpenVINO_V2,
                  _In_ OrtSessionOptions* options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /** \brief Append VitisAI provider to session options
   *
   * If VitisAI is not available (due to a non VitisAI enabled build, or if VitisAI is not installed on the system), this function will return failure.
   *
   * \param[in] options
   * \param[in] provider_options_keys
   * \param[in] provider_options_values
   * \param[in] num_keys
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.18.
   */
  ORT_API2_STATUS(SessionOptionsAppendExecutionProvider_VitisAI,
                  _In_ OrtSessionOptions* options,
                  _In_reads_(num_keys) const char* const* provider_options_keys,
                  _In_reads_(num_keys) const char* const* provider_options_values,
                  _In_ size_t num_keys);

  /** \brief Get scratch buffer from the corresponding allocator under the sepcific OrtMemoryInfo object.
   *         NOTE: callers are responsible to release this scratch buffer from the corresponding allocator
   *  \param[in] context OrtKernelContext instance
   *  \param[in] mem_info OrtMemoryInfo instance
   *  \param[in] count_or_bytes How many bytes is this scratch buffer
   *  \param[out] out A pointer to the scrach buffer
   *
   *  \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.18.
   */
  ORT_API2_STATUS(KernelContext_GetScratchBuffer, _In_ const OrtKernelContext* context, _In_ const OrtMemoryInfo* mem_info, _In_ size_t count_or_bytes, _Outptr_ void** out);

  /** \brief Get allocator from KernelInfo for a specific memory type. Please use C API ReleaseAllocator to release out object
   *
   * \param[in] info OrtKernelInfo instance
   * \param[in] mem_type OrtMemType object
   * \param[out] out A pointer to OrtAllocator
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.18.
   */
  ORT_API2_STATUS(KernelInfoGetAllocator, _In_ const OrtKernelInfo* info, _In_ OrtMemType mem_type, _Outptr_ OrtAllocator** out);

  /** \brief Replace initialized Tensors with external data with the provided files in memory
   *
   * The function will find the initialized TensorProtos with external data in the graph with the provided
   * external file names and the file content in memory. The API gets the external file name, offset, data length
   * from TensorProto, and locate the tensor data from the file in memory buffer.
   * It creates a Tensor to replace the existing Tensor in graph. The replacement
   * will occur before any of the optimizations take place. The data will be copied into the graph
   * since TensorProto can't refer to the user provided buffers.
   *
   * \param[in] options
   * \param[in] external_initializer_file_names Array of null terminated UTF-8 encoded strings of the file names
   *            which holds the external initializers.
   * \param[in] external_initializer_file_buffer_array Array of pointers to the buffer of the file content.
   *            The buffer can be freed after session creation.
   * \param[in] external_initializer_file_lengths Array of size_t to indicate the length of file content
   * \param[in] num_external_initializer_files Number of external files
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.18.
   */
  ORT_API2_STATUS(AddExternalInitializersFromFilesInMemory, _In_ OrtSessionOptions* options,
                  _In_reads_(num_external_initializer_files) const ORTCHAR_T* const* external_initializer_file_names,
                  _In_reads_(num_external_initializer_files) char* const* external_initializer_file_buffer_array,
                  _In_reads_(num_external_initializer_files) const size_t* external_initializer_file_lengths,
                  size_t num_external_initializer_files);

  /** \brief Create an OrtLoraAdapter
   *
   * The function attempts to locate file specified by adapter_file_path, read it and create an OrtLoraAdapter
   * instance. The adapter_file_path should be a valid path to a file that contains a valid Lora Adapter
   * format. The function attempts to validate the format at load time. The file will always be memory mapped, unless
   * the platform does not support memory mapping, in which case the file will be read into memory.
   *
   * \param[in] adapter_file_path adapter file path.
   * \param[in] allocator optional pointer to a device allocator. If specified
   *            data is copied to the device at some point before Run() is invoked. If nullptr, data stays on CPU.
   *            The data would still be copied to device if required by the model at inference time.
   * \param[out] out A pointer to a newly created OrtLoraAdapter instance. Must be released with
   *                  OrtApi::ReleaseLoraAdapter.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.20.
   */
  ORT_API2_STATUS(CreateLoraAdapter, const ORTCHAR_T* adapter_file_path, _In_ OrtAllocator* allocator,
                  _Outptr_ OrtLoraAdapter** out);

  /** \brief Create an OrtLoraAdapter
   *
   * The function copies the bytes from the array and creates an OrtLoraAdapter instance.
   *
   *
   * \param[in] bytes pointer to a valid Lora Adapter format buffer.
   * \param[in] num_bytes length of bytes buffer.
   * \param[in] allocator optional pointer to a device allocator. If specified
   *            data is copied to the device at some point before Run() is invoked. If nullptr, data stays on CPU.
   *            The data would still be copied to device if required by the model at inference time.
   * \param[out] out A pointer to a newly created OrtLoraAdapter instance. Must be released with
   *                  OrtApi::ReleaseLoraAdapter.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.20.
   */
  ORT_API2_STATUS(CreateLoraAdapterFromArray, _In_ const void* bytes, size_t num_bytes, _In_ OrtAllocator* allocator,
                  _Outptr_ OrtLoraAdapter** out);

  /** \brief Release an ::OrtLoraAdapter obtained from OrtApi::CreateLoraAdapter
   */
  ORT_CLASS_RELEASE(LoraAdapter);

  /** \brief Add the Lora Adapter to the list of active adapters.
   *
   * The function adds the Lora Adapter to the list of active adapters. The Lora Adapter must be created with
   * OrtApi::CreateLoraAdapter or FromArray. The Lora Adapter will be used by the session to run the model.
   * The instance of the OrtRunOptions can then be used to customize the Run() calls.
   * More than one OrtLoraAdapter can be active at the same time. Lora Parameters that belong to different
   * Lora adapters that will be active at the same time must not overlap.
   * This setting does not affect RunWithBinding.
   *
   * \param[in] options OrtRunOptions instance
   * \param[in] adapter OrtLoraAdapter instance
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.20.
   */
  ORT_API2_STATUS(RunOptionsAddActiveLoraAdapter, _Inout_ OrtRunOptions* options, _In_ const OrtLoraAdapter* adapter);

  /// @}
  /// \name OrtEpDynamicOptions
  /// @{

  /** \brief Set DynamicOptions for EPs (Execution Providers)
   *
   * Valid options can be found in `include\onnxruntime\core\session\onnxruntime_session_options_config_keys.h`
   * Look for `kOrtEpDynamicOptions`
   *
   * \param[in] sess OrtSession
   * \param[in] keys Array of null terminated UTF8 encoded strings of EP dynamic option keys
   * \param[in] values Array of null terminated UTF8 encoded string of EP dynamic option values
   * \param[in] kv_len Number of elements in the keys and values arrays
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.20.
   */
  ORT_API2_STATUS(SetEpDynamicOptions, _Inout_ OrtSession* sess, _In_reads_(kv_len) const char* const* keys,
                  _In_reads_(kv_len) const char* const* values, _In_ size_t kv_len);

  /** \brief Release an OrtValueInfo instance if it was not added to an OrtGraph.
   * \since Version 1.21.
   */
  ORT_CLASS_RELEASE(ValueInfo);

  /** \brief Release an OrtNode if it was not added to an OrtGraph.
   * \since Version 1.21.
   */
  ORT_CLASS_RELEASE(Node);

  /** \brief Release an OrtGraph.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.21.
   */
  ORT_CLASS_RELEASE(Graph);

  /** \brief Release an OrtModel.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.21.
   */
  ORT_CLASS_RELEASE(Model);

  /** \brief Get the value name from an OrtValueInfo instance.
   * \param[in] value_info The OrtValueInfo instance.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.21.
   */
  ORT_API2_STATUS(GetValueInfoName, _In_ const OrtValueInfo* value_info, _Out_ const char** name);

  /** \brief Get the type information from an OrtValueInfo instance.
   * \param[in] value_info The OrtValueInfo instance.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.21.
   */
  ORT_API2_STATUS(GetValueInfoTypeInfo, _In_ const OrtValueInfo* value_info, _Outptr_ const OrtTypeInfo** type_info);

  /** \brief Get the Model Editor API instance
   *
   * Get the Model Editor API instance to create a new model or augment an existing model.
   *
   * \return Model Editor API struct
   *
   * \since Version 1.21.
   */
  const OrtModelEditorApi*(ORT_API_CALL* GetModelEditorApi)();

  /** \brief Create an OrtValue for a Tensor that uses pre-existing memory.
   *
   * ORT will take ownership of the memory and free it using the provided deleter when no longer in use.
   *
   * \param[in] deleter OrtAllocator instance that will be used to free the memory.
   *                    Only the OrtAllocator:Info and OrtAllocator::Release functions are required.
   *                    The OrtMemoryInfo returned by OrtAllocator::Info must match the location of p_data.
   * \param[in] p_data Pointer to the memory that will be used by the Tensor. ORT will take ownership of the memory.
   * \param[in] p_data_len Length of the memory in bytes.
   * \param[in] shape Dimensions of the Tensor. All values should be > 0.
   * \param[in] shape_len Number of dimensions in the shape array.
   * \param[in] type Data type of the Tensor.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateTensorWithDataAndDeleterAsOrtValue, _In_ OrtAllocator* deleter,
                  _In_ void* p_data, size_t p_data_len,
                  _In_ const int64_t* shape, size_t shape_len,
                  ONNXTensorElementDataType type,
                  _Outptr_ OrtValue** out);
};

/*
 * Steps to use a custom op:
 *   1 Create an OrtCustomOpDomain with the domain name used by the custom ops
 *   2 Create an OrtCustomOp structure for each op and add them to the domain
 *   3 Call OrtAddCustomOpDomain to add the custom domain of ops to the session options
 */

// Specifies some characteristics of inputs/outputs of custom ops:
// Specify if the inputs/outputs are one of:
// 1) Non-optional (input/output must be present in the node)
// 2) Optional (input/output may be absent in the node)
// 3) Variadic: A variadic input or output specifies N (i.e., the minimum arity) or more operands.
//              Only the last input or output of a custom op may be marked as variadic.
//              The homogeneity of the variadic input or output determines whether all operands must be of the same
//              tensor element type.
typedef enum OrtCustomOpInputOutputCharacteristic {
  INPUT_OUTPUT_REQUIRED = 0,
  INPUT_OUTPUT_OPTIONAL,
  INPUT_OUTPUT_VARIADIC,
} OrtCustomOpInputOutputCharacteristic;

/*
 * The OrtCustomOp structure defines a custom op's schema and its kernel callbacks. The callbacks are filled in by
 * the implementor of the custom op.
 */
struct OrtCustomOp {
  uint32_t version;  // Must be initialized to ORT_API_VERSION

  // This callback creates the kernel, which is a user defined
  // parameter that is passed to the Kernel* callbacks below. It is
  // recommended to use CreateKernelV2 which allows for a safe error
  // propagation by returning an OrtStatusPtr.
  void*(ORT_API_CALL* CreateKernel)(_In_ const struct OrtCustomOp* op, _In_ const OrtApi* api,
                                    _In_ const OrtKernelInfo* info);

  // Returns the name of the op
  const char*(ORT_API_CALL* GetName)(_In_ const struct OrtCustomOp* op);

  // Returns the type of the execution provider, return nullptr to use CPU execution provider
  const char*(ORT_API_CALL* GetExecutionProviderType)(_In_ const struct OrtCustomOp* op);

  // Returns the count and types of the input & output tensors
  ONNXTensorElementDataType(ORT_API_CALL* GetInputType)(_In_ const struct OrtCustomOp* op, _In_ size_t index);
  size_t(ORT_API_CALL* GetInputTypeCount)(_In_ const struct OrtCustomOp* op);
  ONNXTensorElementDataType(ORT_API_CALL* GetOutputType)(_In_ const struct OrtCustomOp* op, _In_ size_t index);
  size_t(ORT_API_CALL* GetOutputTypeCount)(_In_ const struct OrtCustomOp* op);

  // Perform a computation step.  It is recommended to use
  // KernelComputeV2 which allows for a safe error propagation by
  // returning an OrtStatusPtr.
  void(ORT_API_CALL* KernelCompute)(_In_ void* op_kernel, _In_ OrtKernelContext* context);
  void(ORT_API_CALL* KernelDestroy)(_In_ void* op_kernel);

  // Returns the characteristics of the input & output tensors
  OrtCustomOpInputOutputCharacteristic(ORT_API_CALL* GetInputCharacteristic)(_In_ const struct OrtCustomOp* op, _In_ size_t index);
  OrtCustomOpInputOutputCharacteristic(ORT_API_CALL* GetOutputCharacteristic)(_In_ const struct OrtCustomOp* op, _In_ size_t index);

  // Returns the memory type of the input tensors. This API allows the custom op
  // to place the inputs on specific devices. By default, it returns
  // OrtMemTypeDefault, which means the input is placed on the default device for
  // the execution provider. If the inputs need to be with different memory tyeps,
  // this function can be overridden to return the specific memory types.
  OrtMemType(ORT_API_CALL* GetInputMemoryType)(_In_ const struct OrtCustomOp* op, _In_ size_t index);

  // Returns the minimum number of input arguments expected for the variadic input.
  // Applicable only for custom ops that have a variadic input.
  int(ORT_API_CALL* GetVariadicInputMinArity)(_In_ const struct OrtCustomOp* op);

  // Returns true (non-zero) if all arguments of a variadic input have to be of the same type (homogeneous),
  // and false (zero) otherwise.
  // Applicable only for custom ops that have a variadic input.
  int(ORT_API_CALL* GetVariadicInputHomogeneity)(_In_ const struct OrtCustomOp* op);

  // Returns the minimum number of output values expected for the variadic output.
  // Applicable only for custom ops that have a variadic output.
  int(ORT_API_CALL* GetVariadicOutputMinArity)(_In_ const struct OrtCustomOp* op);

  // Returns true (non-zero) if all outputs values of a variadic output have to be of the same type (homogeneous),
  // and false (zero) otherwise.
  // Applicable only for custom ops that have a variadic output.
  int(ORT_API_CALL* GetVariadicOutputHomogeneity)(_In_ const struct OrtCustomOp* op);

  // Create the kernel state which is passed to each compute call.
  OrtStatusPtr(ORT_API_CALL* CreateKernelV2)(_In_ const struct OrtCustomOp* op, _In_ const OrtApi* api,
                                             _In_ const OrtKernelInfo* info,
                                             _Out_ void** kernel);

  // Perform the computation step.
  OrtStatusPtr(ORT_API_CALL* KernelComputeV2)(_In_ void* op_kernel, _In_ OrtKernelContext* context);

  OrtStatusPtr(ORT_API_CALL* InferOutputShapeFn)(_In_ const struct OrtCustomOp* op, _In_ OrtShapeInferContext*);

  // Get start range
  int(ORT_API_CALL* GetStartVersion)(_In_ const struct OrtCustomOp* op);
  int(ORT_API_CALL* GetEndVersion)(_In_ const struct OrtCustomOp* op);

  // Get the inplace_map that defines which output can reuse which input
  // Callers will provide 2 raw int* and pass in their address, this function will fill these 2 arrays
  // when return, output (*output_index)[i] may reuse the input (*input_index[i]).
  // The return value is the size of these 2 arrays.
  // Callers are responsible to delete these 2 arrays after use by calling OrtCustomOp::ReleaseMayInplace().
  size_t(ORT_API_CALL* GetMayInplace)(_Out_ int** input_index, _Out_ int** output_index);

  // Release the pointer input_index and output_index allocated from GetMayInplace() function.
  // If GetMayInplace() is defined, this function MUST be defined as well.
  void(ORT_API_CALL* ReleaseMayInplace)(_Frees_ptr_opt_ int* input_index, _Frees_ptr_opt_ int* output_index);

  // Same as GetMayInplace() and ReleaseMayInplace()
  size_t(ORT_API_CALL* GetAliasMap)(_Out_ int** input_index, _Out_ int** output_index);
  void(ORT_API_CALL* ReleaseAliasMap)(_Frees_ptr_opt_ int* input_index, _Frees_ptr_opt_ int* output_index);
};

/**
 * ORT Model Editor API
 */

/**
 * \brief The OrtModelEditorApi struct provides functions to create or edit an ONNX model.
 *
 * See onnxruntime/test/shared_lib/test_model_editor_api.cc for example usage.
 *
 * \since Version 1.21.
 */
struct OrtModelEditorApi {
  // Model building/editing requires a full build. We return nullptr from GetModelEditorApi if this is a minimal
  // build, so it doesn't matter if there are no function pointers in this struct as a user will never get an
  // OrtModelEditorApi instance. We do however need a dummy field to avoid empty struct warning.
#if defined(ORT_MINIMAL_BUILD)
  const bool not_defined_in_this_build;
#else
  /** \brief Create an OrtTypeInfo instance for a Tensor.
   *
   * Create an OrtTypeInfo instance for a Tensor to use as graph inputs/outputs with the Model Editor API.
   *
   * User can release `tensor_info` after creating the OrtTypeInfo.
   *
   * \param[in] tensor_info Tensor type and shape information.
   * \param[out] TypeInfo instance for the tensor.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateTensorTypeInfo, _In_ const OrtTensorTypeAndShapeInfo* tensor_info,
                  _Outptr_ OrtTypeInfo** type_info);

  /** \brief Create an OrtTypeInfo instance for a SparseTensor.
   *
   * Create an OrtTypeInfo instance for a SparseTensor to use as graph inputs/outputs with the Model Editor API.
   *
   * User can release `tensor_info` after creating the OrtTypeInfo.
   *
   * \param[in] tensor_info SparseTensor type and shape information.
   * \param[out] TypeInfo instance for the tensor.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateSparseTensorTypeInfo, _In_ const OrtTensorTypeAndShapeInfo* tensor_info,
                  _Outptr_ OrtTypeInfo** type_info);

  /** \brief Create an OrtTypeInfo instance for a Map.
   *
   * Create an OrtTypeInfo instance for a Map to use as graph inputs/outputs with the Model Editor API.
   *
   * User can release `map_value_type` after creating the OrtTypeInfo.
   *
   * \param[in] map_key_type Key type for the map.
   * \param[in] map_value_type Value type for the map.
   * \param[out] TypeInfo instance for the map.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateMapTypeInfo, ONNXTensorElementDataType map_key_type, _In_ const OrtTypeInfo* map_value_type,
                  _Outptr_ OrtTypeInfo** type_info);

  /** \brief Create an OrtTypeInfo instance for a Sequence.
   *
   * Create an OrtTypeInfo instance for a Sequence to use as graph inputs/outputs with the Model Editor API.
   *
   * User can release `sequence_type` after creating the OrtTypeInfo.
   *
   * \param[in] sequence_type Sequence type and shape information.
   * \param[out] TypeInfo instance for the sequence.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateSequenceTypeInfo, _In_ const OrtTypeInfo* sequence_type, _Outptr_ OrtTypeInfo** type_info);

  /** \brief Create an OrtTypeInfo instance for an Optional.
   *
   * Create an OrtTypeInfo instance for an Optional to use as graph inputs/outputs with the Model Editor API.
   *
   * User can release `contained_type` after creating the OrtTypeInfo.
   *
   * \param[in] tensor_info Tensor type and shape information.
   * \param[out] TypeInfo instance for the tensor.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateOptionalTypeInfo, _In_ const OrtTypeInfo* contained_type, _Outptr_ OrtTypeInfo** type_info);

  /** \brief Create an OrtValueInfo for use as an OrtGraph input or output.
   *
   * \param[in] name The name of the input or output.
   * \param[in] type_info The type information for the input or output. The provided value is copied.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateValueInfo, _In_ const char* name, _In_ const OrtTypeInfo* type_info,
                  _Outptr_ OrtValueInfo** value_info);

  /** \brief Create an OrtNode to add to an OrtGraph.
   *
   * Create an OrtNode.
   *
   * Create attributes with CreateOpAttr. OrtOpAttr instances are copied.
   *
   * \param[in] operator_name The name of the operator.
   * \param[in] domain_name The domain of the operator. Use an empty string for ONNX operators.
   * \param[in] node_name The name of the node.
   * \param[in] input_names The names of the inputs.
   * \param[in] input_names_len The number of input names.
   * \param[in] output_names The names of the outputs.
   * \param[in] output_names_len The number of output names.
   * \param[in] attributes The optional attributes of the node.
   * \param[in] attribs_len The number of attributes. May be zero.
   * \param[out] node The OrtNode instance.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateNode, _In_ const char* operator_name, _In_ const char* domain_name, _In_ const char* node_name,
                  _In_reads_(input_names_len) const char* const* input_names, size_t input_names_len,
                  _In_reads_(output_names_len) const char* const* output_names, size_t output_names_len,
                  _In_reads_(attribs_len) _In_opt_ OrtOpAttr** attributes, _In_ size_t attribs_len,
                  _Outptr_ OrtNode** node);

  /** \brief Create an OrtGraph
   * \snippet{doc} snippets.dox OrtStatus Return Value
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateGraph, _Outptr_ OrtGraph** graph);

  /** \brief Set the inputs for the OrtGraph.
   *
   * Set the graph inputs. This will replace any existing inputs with the new values.
   * The OrtGraph takes ownership of the OrtValueInfo instances and you should NOT call ReleaseOrtValueInfo.
   *
   * \param[in] graph The OrtGraph instance to update.
   * \param[in] inputs The input OrtValueInfo instances.
   * \param[in] inputs_len The number of input OrtValueInfo instances.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(SetGraphInputs, _Inout_ OrtGraph* graph,
                  _In_reads_(inputs_len) _In_ OrtValueInfo** inputs, _In_ size_t inputs_len);

  /** \brief Set the outputs for the OrtGraph.
   *
   * Set the graph outputs. This will replace any existing outputs with the new values.
   * The OrtGraph takes ownership of the OrtValueInfo instances provided and you should NOT call ReleaseOrtValueInfo.
   *
   * \param[in] graph The OrtGraph instance to update.
   * \param[in] outputs The output OrtValueInfo instances.
   * \param[in] outputs_len The number of output OrtValueInfo instances.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(SetGraphOutputs, _Inout_ OrtGraph* graph,
                  _In_reads_(outputs_len) _In_ OrtValueInfo** outputs, _In_ size_t outputs_len);

  /** \brief Add an initializer to the OrtGraph
   *
   * ORT will take ownership of the OrtValue and you should NOT call ReleaseOrtValue.
   *
   * Two options:
   *
   * Allocated memory:
   *    Use CreateTensorAsOrtValue (allocates memory) and populate the tensor with the data.
   *    Set `data_is_external` to false.
   *
   * Pre-existing memory:
   *    Use CreateTensorWithDataAsOrtValue or CreateTensorWithDataAndDeleterAsOrtValue to create an OrtValue
   *    with a tensor that contains a pointer to the existing data.
   *    Set `data_is_external` to true.
   *
   *    The pointer must remain valid for the duration of the inference session.
   *    If using CreateTensorWithDataAsOrtValue you are responsible for freeing the memory after the inference session
   *    is released.
   *    If using CreateTensorWithDataAndDeleterAsOrtValue, ORT will free the memory using the provided deleter as
   *    soon as the OrtValue is no longer in use.
   *
   *    NOTE: A tensor containing pre-existing memory MUST have 128 bytes of data or more.
   *          For smaller tensors use CreateTensorAsOrtValue.
   *
   *          ONNX shape inferencing does not support external data. An initializer involved in shape inferencing is
   *          typically small (a single value or limited by the rank of a tensor) and uses less than 128 bytes of
   *          memory, so this limit acts as a simple catch-all rule to avoid issues.
   *          e.g. Reshape's `shape`, Clip's `min` and `max`, various ops `axes`.
   *
   * \param[in] graph The OrtGraph instance to update.
   * \param[in] name The value name for the initializer.
   * \param[in] tensor The OrtValue instance containing the tensor data.
   * \param[in] data_is_external Set to true if the data is external and should not be copied.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(AddInitializerToGraph, _Inout_ OrtGraph* graph, _In_ const char* name, _In_ OrtValue* tensor,
                  bool data_is_external);

  /** \brief Add an OrtNode to an OrtGraph
   *
   * Add the node to the graph. The OrtGraph will take ownership of OrtNode and you should NOT call ReleaseOrtNode.
   *
   * \param[in] graph The OrtGraph instance to update.
   * \param[in] node The OrtNode instance to add to the graph.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(AddNodeToGraph, _Inout_ OrtGraph* graph, _In_ OrtNode* node);

  /** \brief Create an OrtModel.
   *
   * Create an OrtModel.
   *
   * This can be used to build a new model, or to augment an existing model.
   *
   * \param[in] domain_names The domain names for the model.
   *                         If augmenting an existing model add additional domains if needed.
   * \param[in] opset_versions The opset versions for the model.
   *                           If augmenting an existing model add additional opset versions if needed.
   * \param[in] opset_entries_len The number of domain_names and opset_versions entries.
   *                              Domain and opset entries should be 1:1
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateModel,
                  _In_reads_(opset_entries_len) const char* const* domain_names,
                  _In_reads_(opset_entries_len) const int* opset_versions,
                  size_t opset_entries_len,
                  _Outptr_ OrtModel** model);

  /** \brief Add an OrtGraph to an OrtModel.
   *
   * Add the graph to a model. This should be called once when creating a new model.
   *
   * The OrtModel takes ownership of the OrtGraph and you should NOT call ReleaseOrtGraph.
   *
   * \param[in] model The OrtModel instance to update.
   * \param[in] graph The OrtGraph instance to add to the model.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(AddGraphToModel, _Inout_ OrtModel* model, _In_ OrtGraph* graph);

  /** \brief Create an OrtSession using the OrtModel.
   *
   * Create an inference session using the OrtModel instance.
   * The OrtModel should have been populated with an OrtGraph containing nodes and initializers, and SetGraphInputs
   * and SetGraphOutputs must have been called.
   * This will validate the model, run optimizers, and prepare the session for inferencing.
   *
   * ReleaseOrtModel must be called to free the OrtModel after session creation.
   *
   * \param[in] env The OrtEnv instance.
   * \param[in] model The OrtModel instance.
   * \param[in] options The OrtSessionOptions instance.
   * \param[out] out The OrtSession instance.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateSessionFromModel, _In_ const OrtEnv* env, _In_ const OrtModel* model,
                  _In_ const OrtSessionOptions* options, _Outptr_ OrtSession** out);

  /** \brief Create an OrtSession to augment an existing model.
   *
   * Create an OrtSession with an existing model that will be augmented with additional nodes and initializers.
   * Nodes can be added before or after the existing nodes in the model. ONNX Runtime will connect the nodes when the
   * model is finalized.
   *
   * To add nodes and initializers to the existing model, first create an OrtModel using CreateModel.
   * Add nodes and initializers to the OrtModel using AddNodeToGraph and AddInitializerToGraph.
   * Graph inputs/outputs should be updated with SetGraphInputs and SetGraphOutputs as needed to reflect changes made
   * by the new nodes. The list of graph inputs/outputs should be for the overall model and not just the new nodes.
   *
   * Add the new information from the OrtModel to the original model using ApplyModelToSession, and prepare the
   * session for inferencing by calling FinalizeModelEditorSession.
   *
   * \param{in} env The OrtEnv instance.
   * \param{in} model_path The path to the existing ONNX model to augment.
   * \param{in} options The OrtSessionOptions instance.
   * \param{out} out The created OrtSession instance.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateModelEditorSession, _In_ const OrtEnv* env, _In_ const ORTCHAR_T* model_path,
                  _In_ const OrtSessionOptions* options,
                  _Outptr_ OrtSession** out);

  /** \brief Create an OrtSession to augment an existing model.
   *
   * Create an OrtSession with an existing model that will be augmented with additional nodes and initializers.
   * Nodes can be added before or after the existing nodes in the model. ONNX Runtime will connect the nodes when the
   * model is finalized.
   *
   * To add nodes and initializers to the existing model, first create an OrtModel using CreateModel.
   * Add nodes and initializers to the OrtModel using AddNodeToGraph and AddInitializerToGraph.
   * Graph inputs/outputs should be updated with SetGraphInputs and SetGraphOutputs as needed to reflect changes made
   * by the new nodes. The list of graph inputs/outputs should be for the overall model and not just the new nodes.
   *
   * Add the new information from the OrtModel to the original model using ApplyModelToSession, and prepare the
   * session for inferencing by calling FinalizeModelEditorSession.
   *
   * \param{in} env The OrtEnv instance.
   * \param{in} model_data The model data for the existing model to augment.
   * \param{in} model_data_length The length of the model data.
   * \param{in} options The OrtSessionOptions instance.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(CreateModelEditorSessionFromArray, _In_ const OrtEnv* env,
                  _In_ const void* model_data, size_t model_data_length,
                  _In_ const OrtSessionOptions* options,
                  _Outptr_ OrtSession** out);

  /** \brief Query the session for the opset version of a domain.
   *
   * When using the Model Editor API to augment a model, any new nodes must conform to the opset version of the
   * original model. To do that the user must be able to discover that opset version.
   *
   * \param[in] session OrtSession to query
   * \param[in] domain Domain to query. The ONNX domain is an empty string.
   * \param[out] opset The opset version of the domain.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value. Returns an error if the domain is not used in the model.
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(SessionGetOpsetForDomain, _In_ const OrtSession* session, _In_ const char* domain, _Out_ int* opset);

  /** \brief Apply changes to augment the ONNX model in a session created using CreateModelEditorSession[FromArray]
   *
   * Adds new nodes and updates graph inputs/outputs using `model` to augment the original ONNX model in the session.
   * All changes will be validated.
   * Call FinalizeModelEditorSession to prepare the session for inferencing.
   *
   * Existing input/outputs will only be updated if the OrtGraph inputs/outputs are set in the OrtModel.
   *   i.e. you don't need to call SetGraphInputs/SetGraphOutputs if they are unchanged.
   *
   * ReleaseOrtModel must be called to free the OrtModel after it is applied to the session.
   *
   * \param[in] session OrtSession to update. Session must have been created using CreateModelEditorSession[FromArray].
   * \param[in] model OrtModel containing new nodes, new initializers, and updated graph input and/or output info.
   *
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(ApplyModelToModelEditorSession, _Inout_ OrtSession* session, _In_ OrtModel* model);

  /** \brief Finalize the Model Editor session that was created using CreateModelEditorSession[FromArray].
   *
   * Finalize the Model Editor session that augmented an ONNX model by adding new nodes.
   * This will run optimizers and prepare the session for inferencing.
   *
   * \param[in] session OrtSession to finalize. Session must have been created using CreateModelEditorSession[FromArray].
   * \param[in] options OrtSessionOptions to use for the session.
   * \param[in] Optional prepacked_weights_container OrtPrepackedWeightsContainer to use for the session.
                Set to nullptr if not used.
   * \snippet{doc} snippets.dox OrtStatus Return Value
   *
   * \since Version 1.21.
   */
  ORT_API2_STATUS(FinalizeModelEditorSession, _Inout_ OrtSession* session, _In_ const OrtSessionOptions* options,
                  _In_opt_ OrtPrepackedWeightsContainer* prepacked_weights_container);
#endif  // !defined(ORT_MINIMAL_BUILD)
};

/*
 * This is the old way to add the CUDA provider to the session, please use SessionOptionsAppendExecutionProvider_CUDA above to access the latest functionality
 * This function always exists, but will only succeed if Onnxruntime was built with CUDA support and the CUDA provider shared library exists
 *
 * \param device_id CUDA device id, starts from zero.
 */
ORT_API_STATUS(OrtSessionOptionsAppendExecutionProvider_CUDA, _In_ OrtSessionOptions* options, int device_id);

/*
 * This is the old way to add the ROCm provider to the session, please use
 * SessionOptionsAppendExecutionProvider_ROCM above to access the latest functionality
 * This function always exists, but will only succeed if Onnxruntime was built with
 * HIP support and the ROCm provider shared library exists
 *
 * \param device_id HIP device id, starts from zero.
 */
ORT_API_STATUS(OrtSessionOptionsAppendExecutionProvider_ROCM, _In_ OrtSessionOptions* options, int device_id);

/*
 * This is the old way to add the MIGraphX provider to the session, please use
 * SessionOptionsAppendExecutionProvider_MIGraphX above to access the latest functionality
 * This function always exists, but will only succeed if Onnxruntime was built with
 * HIP support and the MIGraphX provider shared library exists
 *
 * \param device_id HIP device id, starts from zero.
 */
ORT_API_STATUS(OrtSessionOptionsAppendExecutionProvider_MIGraphX, _In_ OrtSessionOptions* options, int device_id);

/*
 * This is the old way to add the oneDNN provider to the session, please use
 * SessionOptionsAppendExecutionProvider_oneDNN above to access the latest functionality
 * This function always exists, but will only succeed if Onnxruntime was built with
 * oneDNN support and the oneDNN provider shared library exists
 *
 * \param use_arena zero: false. non-zero: true.
 */
ORT_API_STATUS(OrtSessionOptionsAppendExecutionProvider_Dnnl, _In_ OrtSessionOptions* options, int use_arena);

/*
 * This is the old way to add the TensorRT provider to the session, please use SessionOptionsAppendExecutionProvider_TensorRT_V2 above to access the latest functionality
 * This function always exists, but will only succeed if Onnxruntime was built with TensorRT support and the TensorRT provider shared library exists
 *
 * \param device_id CUDA device id, starts from zero.
 */
ORT_API_STATUS(OrtSessionOptionsAppendExecutionProvider_Tensorrt, _In_ OrtSessionOptions* options, int device_id);

#ifdef __cplusplus
}
#endif
/// @}
