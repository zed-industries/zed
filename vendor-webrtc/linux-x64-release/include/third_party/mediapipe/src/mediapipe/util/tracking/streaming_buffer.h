// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_UTIL_TRACKING_STREAMING_BUFFER_H_
#define MEDIAPIPE_UTIL_TRACKING_STREAMING_BUFFER_H_

#include <deque>
#include <memory>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "absl/container/node_hash_map.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/types/any.h"
#include "mediapipe/framework/tool/type_util.h"

namespace mediapipe {

// Streaming buffer to store arbitrary data over a chunk of frames with overlap
// between chunks.
// Useful to compute solutions that require as input buffered inputs I_ij, of
// type T_j for all frames i of a chunk. Output solutions S_ik of type T_k
// for each frame i (T_k is not associated with input types T_i in any way)
// can than be stored in the buffer as well and made available to the next
// chunk.
// After solution S_ik has been computed, buffered results (I_ij, S_ik) can be
// output and buffer is truncated to discard all elements minus the overlap.
// Remaining elements (I_ij, S_ik) form the basis for the next chunk.
// Again, arbitrary input data I_ij is buffered until buffer is full.
// However, previous solutions S_ik from the overlap still remain in the buffer
// and can be used to compute new solutions in a temporally constrained manner.

// Detailed usage example:
// Setup streaming buffer, that will buffer cv::Mat's and AffineModel's over a
// set of 100 frames. Some algorithm will then compute for each input pair the
// foreground saliency in the form of a SaliencyPointList proto.
//
// // Prepare streaming buffer with 2 inputs and one solution, i.e. 3 different
// // data types.
// vector<TaggedType> data_config = {
//     TaggedPointerType<cv::Mat>("frame"),
//     TaggedPointerType<AffineModel>("motion"),
//     TaggedPointerType<SaliencyPointList>("saliency") };
//
// // Create new buffer with overlap of 10 frames between chunks.
// StreamingBuffer streaming_buffer(data_config, 10);
//
// // Buffer inputs.
// for (int k = 0; k < 100; ++k) {
//   std::unique_ptr<cv::Mat> input_frame = ...
//   std::unique_ptr<AffineModel> affine_model = ...
//   streaming_buffer.AddDatum("frame", std::move(input_frame));
//   streaming_buffer.AddDatum("motion", std::move(affine_model);
//   // OR:
//   streaming_buffer.AddData({"frame", "motion"},
//                              std::move(input_frame),
//                              std::move(affine_model));
//  // Note: WrapUnique from util::gtl is imported into this namespace when
//  // including this file.
//
//  // Get maximum buffer size.
//  int buffer_size = streaming_buffer.MaxBufferSize("frame");
//
//  // Reached chunk boundary?
//  if (buffer_size == 100) {
//    // Check that we buffered one frame for each motion.
//    ABSL_CHECK(streaming_buffer.HaveEqualSize({"frame", "motion"}));
//
//    // Compute saliency.
//    for (int k = 0; k < 100; ++k) {
//      cv::Mat& frame = streaming_buffer.GetDatum<cv::Mat>("frame", k);
//      AffineModel* model = streaming_buffer.GetMutableDatum<AffineModel>(
//          "motion", k);
//
//      // OR:
//      std::tuple<cv::Mat*, AffineModel*> data =
//          streaming_buffer.GetMutableData({"frame", "motion"}, k);
//
//      // Resulting saliency.
//      std::unique_ptr<SaliencyPointList> saliency(new SaliencyPointList);
//
//      // Some Computation using frame and model from above ...
//
//      // Store computed saliency.
//      streaming_buffer.AddDatum("saliency", saliency.release());
//    }
//
//    // Output elements, transfers ownership.
//    streaming_buffer.OutputDatum<cv::Mat>(
//      false /*is_flush*/, "frame", [](int frame_idx,
//                                      std::unique_ptr<cv::Mat> frame) {
//    });
//
//    // OR:
//    streaming_buffer.OutputData(/*is_flush*/, {"frame", "motion", "saliency"},
//      [](int frame_idx,
//         std::unique_ptr<cv::Mat> frame,
//         std::unique_ptr<AffineModel> model,
//         std::unique_ptr<SaliencyPointList> saliency) {
//    });
//
//    // Truncate buffers for next chunk.
//    streaming_buffer.TruncateBuffer(false /* is_flush */);
//
//    // End chunk boundary processing.
//  }

// Stores pair (tag, TypeId of type).
typedef std::pair<std::string, size_t> TaggedType;

// Returns TaggedType for type T* tagged with passed string.
template <class T>
TaggedType TaggedPointerType(const std::string& tag);

// Helper function to create unique_ptr's until std::make_unique is allowed.
template <class T>
std::unique_ptr<T> MakeUnique(T* ptr);

// Note: If any of the function below are called with a tag not registered by
// the constructor, the function will fail with CHECK.
// Also, if any of the functions below is called with an existing tag but
// incompatible type, the function will fail with CHECK.
class StreamingBuffer {
 public:
  // Constructs a new buffer with passed mappings (TAG_NAME, DATA_TYPE) of
  // maximum size buffer_size.
  // Data_configuration must have unique tag for each type.
  StreamingBuffer(const std::vector<TaggedType>& data_configuration,
                  int overlap);

  // Call will transfer ownership to StreamingBuffer.
  // Returns true if datum was successfully stored, false otherwise.
  template <class T>
  void AddDatum(const std::string& tag, std::unique_ptr<T> pointer);

  // Same as above but forwards pointer T* to a unique_ptr<T>.
  // Transfers ownership.
  template <class T>
  void EmplaceDatum(const std::string& tag, T* pointer);

  // Creates a deep copy and stores it in the StreamingBuffer.
  template <class T>
  void AddDatumCopy(const std::string& tag, const T& datum);

  // Convenience function to store multiple pointers at once.
  // Returns true if all data was successfully stored, false if at least one
  // failed.
  template <class... Types>
  void AddData(const std::vector<std::string>& tags,
               std::unique_ptr<Types>... pointers);

  // Convenience function to add a whole vector of objects to the buffer.
  // For each datum a copy will be created.
  template <class T>
  void AddDatumVector(const std::string& tag, const std::vector<T>& datum_vec);

  // Retrieves datum with specified tag and frame index. Returns nullptr if
  // datum does not exist.
  template <class T>
  const T* GetDatum(const std::string& tag, int frame_index) const;

  // Gets a reference on the datum.
  template <class T>
  T& GetDatumRef(const std::string& tag, int frame_index) const;

  // Same as above for mutable pointer.
  template <class T>
  T* GetMutableDatum(const std::string& tag, int frame_index) const;

  // Access all elements for a tag as vector, recommended usage:
  // for (auto ptr : buffer.GetDatumVector<SomeType>("some_tag")) { ... }
  template <class T>
  std::vector<const T*> GetDatumVector(const std::string& tag) const;
  template <class T>
  std::vector<T*> GetMutableDatumVector(const std::string& tag) const;

  // Gets a vector of references.
  template <class T>
  std::vector<std::reference_wrapper<T>> GetReferenceVector(
      const std::string& tag) const;

  template <class T>
  std::vector<std::reference_wrapper<const T>> GetConstReferenceVector(
      const std::string& tag) const;

  // Returns number of buffered inputs for specified tag.
  int BufferSize(const std::string& tag) const;

  // Returns maximum over all tags.
  int MaxBufferSize() const;

  // Returns true if the buffers for all passed tags have equal size.
  // Call with HaveEqualSize(AllTags()) to check if all buffers have equal size.
  bool HaveEqualSize(const std::vector<std::string>& tags) const;

  // Check if all items buffered for the specified tag are initialized (i.e. not
  // equal to nullptr).
  template <class T>
  bool IsInitialized(const std::string& tag) const;

  // Returns all tags.
  std::vector<std::string> AllTags() const;

  // Output function to be called after output values have been computed from
  // input values and stored in the buffer as well.
  // Use to transfer ownership for buffered content out of the streaming buffer,
  // by iteratively calling an output functor for each frame.
  //
  // Specifically, functor is repeatedly called with:
  // void operator()(int frame_index, std::unique_ptr<T> pointer)
  //
  // If flush is set, functor will be called with all frames in the half-open
  // interval [0, MaxBufferSize()), otherwise interval is limited to
  // [0, MaxBufferSize() - overlap).
  // Note that the interval is independent of the tag, i.e. for consistency
  // the interval is constant for all tags (if items are not buffered or
  // exists a nullptr is passed instead).
  // Ownership is transferred to caller via unique_ptr.
  // Element storred in buffer is reset to nullptr.
  // Note: Does not truncate the actual buffer, use TruncateBuffer after
  // outputting all desired data.
  template <class T, class Functor>
  void OutputDatum(bool flush, const std::string& tag, const Functor& functor);

  // Same as above for multiple elements.
  // Functor is repeatedly called with:
  // void operator()(int frame_index, std::unique_ptr<Pointers>... pointers)
  template <class... Pointers, class Functor>
  void OutputData(bool flush, const std::vector<std::string>& tags,
                  const Functor& functor);

  // Releases and returns the input at the specified tag and frame_index.
  // Returns nullptr if datum does not exists.
  // Element stored in buffer is reset to nullptr.
  // It is recommended that above OutputFunctions are used instead.
  template <class T>
  std::unique_ptr<T> ReleaseDatum(const std::string& tag, int frame_index);

  // Truncates the buffer by discarding all its elements within the interval
  // [0, MaxBufferSize() - overlap] if flush is set to false, or
  // [0, MaxBufferSize()] otherwise.
  // Should be called after chunk boundary has been reached, computation of
  // outputs from inputs is done and data has been Output via
  // Output[Datum|Data].
  // Returns true if each element within the truncated interval exists,
  // and all buffers have the same number of remaining elements
  // (#overlap if flush is false, zero otherwise).
  bool TruncateBuffer(bool flush);

  // Discards first num_frames of data for specified tag.
  // This is different from ReleaseDatum, as it will shrink the buffer for
  // the specified tag.
  void DiscardDatum(const std::string& tag, int num_frames);

  /// Same as above, but removes num_frames items from the end of the buffer.
  void DiscardDatumFromEnd(const std::string& tag, int num_frames);

  // Same as above for a list of tags.
  void DiscardData(const std::vector<std::string>& tags, int num_frames);

  // Returns true if tag exist.
  bool HasTag(const std::string& tag) const;
  // Returns true if all passed tags exist.
  bool HasTags(const std::vector<std::string>& tags) const;

  // Returns frame index of the first item in the buffer. This is useful to
  // determine how many frames in total went through the buffer after several
  // *successfull* calls of Truncate().
  int FirstFrameIndex() const { return first_frame_index_; }

  template <class T>
  using PointerType = std::shared_ptr<std::unique_ptr<T>>;
  template <class T>
  PointerType<T> CreatePointer(T* t);

 private:
  // Implementation function with aquiring ownership in case failure while
  // adding single argument pointer occurs.
  template <class Pointer, class... Pointers>
  void AddDataImpl(const std::vector<std::string>& tags,
                   std::unique_ptr<Pointer> pointer,
                   std::unique_ptr<Pointers>... pointers);
  // Terminates recursive template expansion for AddDataImpl. Will never be
  // called.
  void AddDataImpl(const std::vector<std::string>& tags) {
    ABSL_CHECK(tags.empty());
  }

 private:
  int overlap_ = 0;
  int first_frame_index_ = 0;
  absl::node_hash_map<std::string, std::deque<absl::any>> data_;

  // Stores tag, TypeId of corresponding type.
  absl::node_hash_map<std::string, size_t> data_config_;
};

//// Implementation details.
template <class T>
TaggedType TaggedPointerType(const std::string& tag) {
  return std::make_pair(tag,
                        kTypeId<StreamingBuffer::PointerType<T>>.hash_code());
}

template <class T>
StreamingBuffer::PointerType<T> StreamingBuffer::CreatePointer(T* t) {
  return PointerType<T>(new std::unique_ptr<T>(t));
}

template <class T>
void StreamingBuffer::AddDatum(const std::string& tag,
                               std::unique_ptr<T> pointer) {
  ABSL_CHECK(HasTag(tag));
  ABSL_CHECK_EQ(data_config_[tag], kTypeId<PointerType<T>>.hash_code());
  auto& buffer = data_[tag];
  absl::any packet(PointerType<T>(CreatePointer(pointer.release())));
  buffer.push_back(packet);
}

template <class T>
void StreamingBuffer::EmplaceDatum(const std::string& tag, T* pointer) {
  std::unique_ptr<T> forwarded(pointer);
  AddDatum(tag, std::move(forwarded));
}

template <class T>
void StreamingBuffer::AddDatumCopy(const std::string& tag, const T& datum) {
  AddDatum(tag, std::unique_ptr<T>(new T(datum)));
}

template <class... Types>
void StreamingBuffer::AddData(const std::vector<std::string>& tags,
                              std::unique_ptr<Types>... pointers) {
  ABSL_CHECK_EQ(tags.size(), sizeof...(pointers))
      << "Number of tags and data pointers is inconsistent";
  return AddDataImpl(tags, std::move(pointers)...);
}

template <class T>
void StreamingBuffer::AddDatumVector(const std::string& tag,
                                     const std::vector<T>& datum_vec) {
  for (const auto& datum : datum_vec) {
    AddDatumCopy(tag, datum);
  }
}

template <class Pointer, class... Pointers>
void StreamingBuffer::AddDataImpl(const std::vector<std::string>& tags,
                                  std::unique_ptr<Pointer> pointer,
                                  std::unique_ptr<Pointers>... pointers) {
  AddDatum(tags[0], std::move(pointer));
  if (sizeof...(pointers) > 0) {
    return AddDataImpl(std::vector<std::string>(tags.begin() + 1, tags.end()),
                       std::move(pointers)...);
  }
}

template <class T>
std::unique_ptr<T> MakeUnique(T* t) {
  return std::unique_ptr<T>(t);
}

template <class T>
const T* StreamingBuffer::GetDatum(const std::string& tag,
                                   int frame_index) const {
  return GetMutableDatum<T>(tag, frame_index);
}

template <class T>
T& StreamingBuffer::GetDatumRef(const std::string& tag, int frame_index) const {
  return *GetMutableDatum<T>(tag, frame_index);
}

template <class T>
T* StreamingBuffer::GetMutableDatum(const std::string& tag,
                                    int frame_index) const {
  ABSL_CHECK_GE(frame_index, 0);
  ABSL_CHECK(HasTag(tag));
  auto& buffer = data_.find(tag)->second;
  if (frame_index > buffer.size()) {
    return nullptr;
  } else {
    const absl::any& packet = buffer[frame_index];
    if (absl::any_cast<PointerType<T>>(&packet) == nullptr) {
      ABSL_LOG(ERROR) << "Stored item is not of requested type. "
                      << "Check data configuration.";
      return nullptr;
    }

    // Unpack and return.
    const PointerType<T>& pointer =
        *absl::any_cast<const PointerType<T>>(&packet);
    return pointer->get();
  }
}

template <class T>
std::vector<const T*> StreamingBuffer::GetDatumVector(
    const std::string& tag) const {
  std::vector<T*> result(GetMutableDatumVector<T>(tag));
  return std::vector<const T*>(result.begin(), result.end());
}

template <class T>
std::vector<std::reference_wrapper<T>> StreamingBuffer::GetReferenceVector(
    const std::string& tag) const {
  std::vector<T*> ptrs(GetMutableDatumVector<T>(tag));
  std::vector<std::reference_wrapper<T>> refs;
  refs.reserve(ptrs.size());
  for (T* ptr : ptrs) {
    refs.push_back(std::ref(*ptr));
  }
  return refs;
}

template <class T>
std::vector<std::reference_wrapper<const T>>
StreamingBuffer::GetConstReferenceVector(const std::string& tag) const {
  std::vector<const T*> ptrs(GetDatumVector<T>(tag));
  std::vector<std::reference_wrapper<const T>> refs;
  refs.reserve(ptrs.size());
  for (const T* ptr : ptrs) {
    refs.push_back(std::cref(*ptr));
  }
  return refs;
}

template <class T>
bool StreamingBuffer::IsInitialized(const std::string& tag) const {
  ABSL_CHECK(HasTag(tag));
  const auto& buffer = data_.find(tag)->second;
  int idx = 0;
  for (const auto& item : buffer) {
    const PointerType<T>* pointer = absl::any_cast<const PointerType<T>>(&item);
    ABSL_CHECK(pointer != nullptr);
    if (*pointer == nullptr) {
      ABSL_LOG(ERROR) << "Data for " << tag << " at frame " << idx
                      << " is not initialized.";
      return false;
    }
  }
  return true;
}

template <class T>
std::vector<T*> StreamingBuffer::GetMutableDatumVector(
    const std::string& tag) const {
  ABSL_CHECK(HasTag(tag));
  auto& buffer = data_.find(tag)->second;
  std::vector<T*> result;
  for (const auto& packet : buffer) {
    if (absl::any_cast<PointerType<T>>(&packet) == nullptr) {
      ABSL_LOG(ERROR) << "Stored item is not of requested type. "
                      << "Check data configuration.";
      result.push_back(nullptr);
    } else {
      result.push_back(
          absl::any_cast<const PointerType<T>>(&packet)->get()->get());
    }
  }
  return result;
}

template <class T, class Functor>
void StreamingBuffer::OutputDatum(bool flush, const std::string& tag,
                                  const Functor& functor) {
  ABSL_CHECK(HasTag(tag));
  const int end_frame = MaxBufferSize() - (flush ? 0 : overlap_);
  for (int k = 0; k < end_frame; ++k) {
    functor(k, ReleaseDatum<T>(tag, k));
  }
}

template <class T>
std::unique_ptr<T> StreamingBuffer::ReleaseDatum(const std::string& tag,
                                                 int frame_index) {
  ABSL_CHECK(HasTag(tag));
  ABSL_CHECK_GE(frame_index, 0);

  auto& buffer = data_.find(tag)->second;
  if (frame_index >= buffer.size()) {
    return nullptr;
  } else {
    const absl::any& packet = buffer[frame_index];
    if (absl::any_cast<PointerType<T>>(&packet) == nullptr) {
      ABSL_LOG(ERROR) << "Stored item is not of requested type. "
                      << "Check data configuration.";
      return nullptr;
    }

    // Unpack and return.
    const PointerType<T>& pointer =
        *absl::any_cast<const PointerType<T>>(&packet);
    return std::move(*pointer);
  }
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_STREAMING_BUFFER_H_
