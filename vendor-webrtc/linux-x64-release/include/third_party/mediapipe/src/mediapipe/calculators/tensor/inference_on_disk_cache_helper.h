#ifndef MEDIAPIPE_CALCULATORS_TENSOR_ON_DISK_CACHE_HELPER_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_ON_DISK_CACHE_HELPER_H_

#include <string>

#include "absl/status/status.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/util/tflite/tflite_gpu_runner.h"

namespace mediapipe::api2 {

// Helper class that saves binary data to disk, or read from disk.
class InferenceOnDiskCacheHelper {
 public:
  absl::Status Init(const mediapipe::InferenceCalculatorOptions& options,
                    const mediapipe::InferenceCalculatorOptions::Delegate::Gpu&
                        gpu_delegate_options);
  absl::Status ReadGpuCaches(tflite::gpu::TFLiteGPURunner& gpu_runner) const;
  // Writes caches to disk based on |cache_writing_behavior_|.
  absl::Status SaveGpuCachesBasedOnBehavior(
      tflite::gpu::TFLiteGPURunner& gpu_runner) const;
  bool UseSerializedModel() const { return use_serialized_model_; }

 private:
  // Writes caches to disk, returns error on failure.
  absl::Status SaveGpuCaches(tflite::gpu::TFLiteGPURunner& gpu_runner) const;

  bool use_kernel_caching_ = false;
  std::string cached_kernel_filename_;
  bool use_serialized_model_ = false;
  std::string serialized_model_path_;
  mediapipe::InferenceCalculatorOptions::Delegate::Gpu::CacheWritingBehavior
      cache_writing_behavior_;
};

}  // namespace mediapipe::api2

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_ON_DISK_CACHE_HELPER_H_
