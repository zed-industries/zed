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

// This header defines a large number of getters and setters for storing
// multimedia, such as video or audio, and related machine learning data in
// tensorflow::SequenceExamples. These getters and setters simplify sharing
// data by enforcing common patterns for storing data in SequenceExample
// key-value pairs.
//
// The constants, macros, and functions are organized into 6 groups: clip
// metadata, clip label related, segment related, bounding-box related, image
// related, feature list related, and keyframe related. The following examples
// will walk through common task structures, but the relevant data to store can
// vary by task.
//
// The clip metadata group is generally data about the media and stored in the
// SequenceExample.context. Specifying the metadata enables media pipelines,
// such as MediaPipe, to retrieve that data. Typically, SetClipDataPath,
// SetClipStartTimestamp, and SetClipEndTimestamp define which data to use
// without storing the data itself. Example:
//   tensorflow::SequenceExample sequence;
//   SetClipDataPath("/relative/path/to/data.mp4", &sequence);
//   SetClipStartTimestamp(0, &sequence);
//   SetClipEndTimestamp(10000000, &sequence);  // 10 seconds in microseconds.
//
// The clip label group adds labels that apply to the entire media clip. To
// annotate that a video clip has a particular label, set the clip metadata
// above and also set the SetClipLabelIndex and SetClipLabelString. Most
// training pipelines will only use the label index or string, but we recommend
// storing both to improve readability while maintaining ease of use.
// Example:
//   SetClipLabelString({"run", "jump"}, &sequence);
//   SetClipLabelIndex({35, 47}, &sequence);
//
// The segment group is generally data about time spans within the media clip
// and stored in the SequenceExample.context. In this code, continuous lengths
// of media are called clips, and each clip may have subregions of interest that
// are called segments. To annotate that a video clip has time spans with labels
// set the clip metadata above and use the functions SetSegmentStartTimestamp,
// SetSegmentEndTimestamp, SetSegmentLabelIndex, and SetSegmentLabelString. Most
// training pipelines will only use the label index or string, but we recommend
// storing both to improve readability while maintaining ease of use. By listing
// segments as times, the frame rate or other properties can change without
// affecting the labels.
// Example:
//   SetSegmentStartTimestamp({500000, 1000000}, &sequence);  // in microseconds
//   SetSegmentEndTimestamp({2000000, 6000000}, &sequence);
//   SetSegmentLabelIndex({35, 47}, &sequence);
//   SetSegmentLabelString({"run", "jump"}, &sequence);
//
// The bounding box group is useful for identifying spatio-temporal annotations
// for detection, tracking, or action recognition. The exact keys that are
// needed can vary by task, but to annotate a video clip for detection set the
// clip metadata above and use repeatedly call AddBBox, AddBBoxTimestamp,
// AddBBoxLabelIndex, and AddBBoxLabelString. Most training pipelines will only
// use the label index or string, but we recommend storing both to improve
// readability while maintaining ease of use. Because bounding boxes are
// assigned to timepoints in a video, changing the image frame rate can can
// change the alignment. The ReconcileMetadata function can align bounding boxes
// to the nearest image.
//
// The image group is useful for storing data as sequential 2D arrays, typically
// encoded as bytes. Images can be RGB images stored as JPEG, discrete masks
// stored as PNG, or some other format. Parameters that are static over time are
// set in the context using SetImageWidth, SetImageHeight, SetImageFormat, etc.
// The series of frames and timestamps are then added with AddImageEncoded and
// AddImageTimestamp. For discrete masks, the class or instance indices can be
// mapped to labels or classes using
// SetClassSegmentationClassLabel{Index,String} and
// SetInstanceSegmentationClassLabelIndex.
//
// The feature list group is useful for storing audio and extracted features,
// such as per-frame embeddings. SequenceExamples only store lists of floats per
// timestep, so the dimensions are stored in the context to enable reshaping.
// For example, SetFeatureDimensions and repeatedly calling AddFeatureFloats and
// AddFeatureTimestamp adds per-frame embeddings. To support audio features,
// additional getters and setters are provided that understand MediaPipe types.
//
// Macros for common patterns are created in media_sequence_util.h and are used
// here extensively. Because these macros are formulaic, I will only include a
// usage example here in the code rather than repeating documentation for every
// instance. This header defines additional functions to simplify working with
// MediaPipe types.
//
// Each {TYPE}_CONTEXT_FEATURE takes a NAME and a KEY. It provides setters and
// getters for SequenceExamples and stores a single value under KEY in the
// context field. The provided functions are Has${NAME}, Get${NAME}, Set${Name},
// and Clear${NAME}.
// Eg.
//   tensorflow::SequenceExample example;
//   SetDataPath("data_path", &example);
//   if (HasDataPath(example)) {
//      string data_path = GetDataPath(example);
//      ClearDataPath(&example);
//   }
//
// Each VECTOR_{TYPE}_CONTEXT_FEATURE takes a NAME and a KEY. It provides
// setters and getters for SequenceExamples and stores a sequence of values
// under KEY in the context field. The provided functions are Has${NAME},
// Get${NAME}, Set${Name}, Clear${NAME}, Get${NAME}At, and Add${NAME}.
// Eg.
//   tensorflow::SequenceExample example;
//   SetClipLabelString({"run", "jump"}, &example);
//   if (HasClipLabelString(example)) {
//      std::vector<std::string> values = GetClipLabelString(example);
//      ClearClipLabelString(&example);
//   }
//
// Each {TYPE}_FEATURE_LIST takes a NAME and a KEY. It provides setters and
// getters for SequenceExamples and stores a single value in each feature field
// under KEY of the feature_lists field. The provided functions are Has${NAME},
// Get${NAME}, Clear${NAME}, Get${NAME}Size, Get${NAME}At, and Add${NAME}.
//   tensorflow::SequenceExample example;
//   AddImageTimestamp(1000000, &example);
//   AddImageTimestamp(2000000, &example);
//   if (HasImageTimestamp(example)) {
//     for (int i = 0; i < GetImageTimestampSize(); ++i) {
//       int64 timestamp = GetImageTimestampAt(example, i);
//     }
//     ClearImageTimestamp(&example);
//   }
//
// Each VECTOR_{TYPE}_FEATURE_LIST takes a NAME and a KEY. It provides setters
// and getters for SequenceExamples and stores a sequence of values in each
// feature field under KEY of the feature_lists field. The provided functions
// are Has${NAME}, Get${NAME}, Clear${NAME}, Get${NAME}Size, Get${NAME}At, and
// Add${NAME}.
//   tensorflow::SequenceExample example;
//   AddBBoxLabelString({"run", "jump"}, &example);
//   AddBBoxLabelString({"run", "fall"}, &example);
//   if (HasBBoxLabelString(example)) {
//     for (int i = 0; i < GetBBoxLabelStringSize(); ++i) {
//       std::vector<std::string> labels = GetBBoxLabelStringAt(example, i);
//     }
//     ClearBBoxLabelString(&example);
//   }
//
// As described in media_sequence_util.h, each of these functions can take an
// additional string prefix argument as their first argument. The prefix can
// be fixed with a new NAME by calling a FIXED_PREFIX_... macro. Prefixes are
// used to identify common storage patterns (e.g. storing an image along with
// the height and width) under different names (e.g. storing a left and right
// image in a stereo pair.) An example creating functions such as
// AddLeftImageEncoded that adds a string under the key "LEFT/image/encoded":
//  FIXED_PREFIX_STRING_FEATURE_LIST("LEFT", LeftImageEncoded, "image/encoded");

#ifndef MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_H_
#define MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_H_

#include <string>
#include <vector>

#include "absl/memory/memory.h"
#include "mediapipe/framework/formats/location.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/util/sequence/media_sequence_util.h"
#include "tensorflow/core/example/example.pb.h"
#include "tensorflow/core/example/feature.pb.h"

namespace mediapipe {
namespace mediasequence {

// ***********************    METADATA    *************************************
// Context Keys:
// A unique identifier for each example.
const char kExampleIdKey[] = "example/id";
// The name of the data set, including the version.
const char kExampleDatasetNameKey[] = "example/dataset_name";
// String flags or attributes for this example within a data set.
const char kExampleDatasetFlagStringKey[] = "example/dataset/flag/string";

// The relative path to the data on disk from some root directory.
const char kClipDataPathKey[] = "clip/data_path";
// Any identifier for the media beyond the data path.
const char kClipMediaId[] = "clip/media_id";
// Yet another alternative identifier.
const char kClipAlternativeMediaId[] = "clip/alternative_media_id";
// The encoded bytes for storing media directly in the SequenceExample.
const char kClipEncodedMediaBytesKey[] = "clip/encoded_media_bytes";
// The start time for the encoded media if not preserved during encoding.
const char kClipEncodedMediaStartTimestampKey[] =
    "clip/encoded_media_start_timestamp";
// The start time, in microseconds, for the start of the clip in the media.
const char kClipStartTimestampKey[] = "clip/start/timestamp";
// The end time, in microseconds, for the end of the clip in the media.
const char kClipEndTimestampKey[] = "clip/end/timestamp";
// A list of label indices for this clip.
const char kClipLabelIndexKey[] = "clip/label/index";
// A list of label strings for this clip.
const char kClipLabelStringKey[] = "clip/label/string";
// A list of label confidences for this clip.
const char kClipLabelConfidenceKey[] = "clip/label/confidence";
// A list of label start timestamps for this clip.
const char kClipLabelStartTimestampKey[] = "clip/label/start/timestamp";
// A list of label end timestamps for this clip.
const char kClipLabelEndTimestampKey[] = "clip/label/end/timestamp";

BYTES_CONTEXT_FEATURE(ExampleId, kExampleIdKey);
BYTES_CONTEXT_FEATURE(ExampleDatasetName, kExampleDatasetNameKey);
VECTOR_BYTES_CONTEXT_FEATURE(ExampleDatasetFlagString,
                             kExampleDatasetFlagStringKey);

BYTES_CONTEXT_FEATURE(ClipDataPath, kClipDataPathKey);
BYTES_CONTEXT_FEATURE(ClipAlternativeMediaId, kClipAlternativeMediaId);
BYTES_CONTEXT_FEATURE(ClipMediaId, kClipMediaId);
BYTES_CONTEXT_FEATURE(ClipEncodedMediaBytes, kClipEncodedMediaBytesKey);
INT64_CONTEXT_FEATURE(ClipEncodedMediaStartTimestamp,
                      kClipEncodedMediaStartTimestampKey);
INT64_CONTEXT_FEATURE(ClipStartTimestamp, kClipStartTimestampKey);
INT64_CONTEXT_FEATURE(ClipEndTimestamp, kClipEndTimestampKey);
VECTOR_BYTES_CONTEXT_FEATURE(ClipLabelString, kClipLabelStringKey);
VECTOR_INT64_CONTEXT_FEATURE(ClipLabelIndex, kClipLabelIndexKey);
VECTOR_FLOAT_CONTEXT_FEATURE(ClipLabelConfidence, kClipLabelConfidenceKey);
VECTOR_INT64_CONTEXT_FEATURE(ClipLabelStartTimestamp,
                             kClipLabelStartTimestampKey);
VECTOR_INT64_CONTEXT_FEATURE(ClipLabelEndTimestamp, kClipLabelEndTimestampKey);

// ***********************    SEGMENTS    *************************************
// Context Keys:
// A list of segment start times in microseconds.
const char kSegmentStartTimestampKey[] = "segment/start/timestamp";
// A list of indices marking the first frame index >= the start time.
const char kSegmentStartIndexKey[] = "segment/start/index";
// A list of segment end times in microseconds.
const char kSegmentEndTimestampKey[] = "segment/end/timestamp";
// A list of indices marking the last frame index <= the end time.
const char kSegmentEndIndexKey[] = "segment/end/index";
// A list with the label index for each segment.
// Multiple labels for the same segment are encoded as repeated segments.
const char kSegmentLabelIndexKey[] = "segment/label/index";
// A list with the label string for each segment.
// Multiple labels for the same segment are encoded as repeated segments.
const char kSegmentLabelStringKey[] = "segment/label/string";
// A list with the label confidence for each segment.
// Multiple labels for the same segment are encoded as repeated segments.
const char kSegmentLabelConfidenceKey[] = "segment/label/confidence";

VECTOR_BYTES_CONTEXT_FEATURE(SegmentLabelString, kSegmentLabelStringKey);
VECTOR_INT64_CONTEXT_FEATURE(SegmentStartTimestamp, kSegmentStartTimestampKey);
VECTOR_INT64_CONTEXT_FEATURE(SegmentEndTimestamp, kSegmentEndTimestampKey);
VECTOR_INT64_CONTEXT_FEATURE(SegmentStartIndex, kSegmentStartIndexKey);
VECTOR_INT64_CONTEXT_FEATURE(SegmentEndIndex, kSegmentEndIndexKey);
VECTOR_INT64_CONTEXT_FEATURE(SegmentLabelIndex, kSegmentLabelIndexKey);
VECTOR_FLOAT_CONTEXT_FEATURE(SegmentLabelConfidence,
                             kSegmentLabelConfidenceKey);

// *****************    REGIONS / BOUNDING BOXES    ***************************
// Context keys:
// The dimensions of each embedding per region / bounding box.
const char kRegionEmbeddingDimensionsPerRegionKey[] =
    "region/embedding/dimensions_per_region";
// The format encoding embeddings as strings.
const char kRegionEmbeddingFormatKey[] = "region/embedding/format";
// The list of region parts expected in this example.
const char kRegionPartsKey[] = "region/parts";

// Feature list keys:
// The normalized coordinates of the bounding boxes are provided in four lists
// to avoid order ambiguity, but we provide additional accessors for complete
// bounding boxes below.
const char kRegionBBoxYMinKey[] = "region/bbox/ymin";
const char kRegionBBoxXMinKey[] = "region/bbox/xmin";
const char kRegionBBoxYMaxKey[] = "region/bbox/ymax";
const char kRegionBBoxXMaxKey[] = "region/bbox/xmax";
// The point and radius can denote keypoints.
const char kRegionPointXKey[] = "region/point/x";
const char kRegionPointYKey[] = "region/point/y";
const char kRegionRadiusKey[] = "region/radius";
// The 3d point can denote keypoints.
const char kRegion3dPointXKey[] = "region/3d_point/x";
const char kRegion3dPointYKey[] = "region/3d_point/y";
const char kRegion3dPointZKey[] = "region/3d_point/z";
// The number of regions at that timestep.
const char kRegionNumRegionsKey[] = "region/num_regions";
// Whether that timestep is annotated for bounding regions.
// (Distinguishes between multiple meanings of num_regions = 0.
const char kRegionIsAnnotatedKey[] = "region/is_annotated";
// A list indicating if each region is generated (1) or manually annotated (0).
const char kRegionIsGeneratedKey[] = "region/is_generated";
// A list indicating if each region is occluded (1) or visible (0).
const char kRegionIsOccludedKey[] = "region/is_occluded";
// Lists with a label for each region.
// Multiple labels for the same region require duplicating the region.
const char kRegionLabelIndexKey[] = "region/label/index";
const char kRegionLabelStringKey[] = "region/label/string";
const char kRegionLabelConfidenceKey[] = "region/label/confidence";
// Lists with a track identifier for each region.
const char kRegionTrackIndexKey[] = "region/track/index";
const char kRegionTrackStringKey[] = "region/track/string";
const char kRegionTrackConfidenceKey[] = "region/track/confidence";
// A list with a class for each region. In general, prefer to use the label
// fields. These class fields exist to distinguish tracks when different classes
// have overlapping track ids.
const char kRegionClassIndexKey[] = "region/class/index";
const char kRegionClassStringKey[] = "region/class/string";
const char kRegionClassConfidenceKey[] = "region/class/confidence";
// The timestamp of the region annotations in microseconds.
const char kRegionTimestampKey[] = "region/timestamp";
// An embedding for each region. The length of each list must be the product of
// the number of regions and the product of the embedding dimensions.
const char kRegionEmbeddingFloatKey[] = "region/embedding/float";
// A string encoded embedding for each region.
const char kRegionEmbeddingEncodedKey[] = "region/embedding/encoded";
// The confidence of the embedding.
const char kRegionEmbeddingConfidenceKey[] = "region/embedding/confidence";
// The original timestamp in microseconds for region annotations.
// ReconcileMetadata can align region annotations to image frames, and this
// field preserves the original timestamps.
const char kUnmodifiedRegionTimestampKey[] = "region/unmodified_timestamp";

// Functions:
// These functions get and set bounding boxes as MediaPipe::Location to avoid
// needing to get and set each box coordinate separately.
int GetBBoxSize(const std::string& prefix,
                const tensorflow::SequenceExample& sequence);
std::vector<::mediapipe::Location> GetBBoxAt(
    const std::string& prefix, const tensorflow::SequenceExample& sequence,
    int index);
void AddBBox(const std::string& prefix,
             const std::vector<::mediapipe::Location>& bboxes,
             tensorflow::SequenceExample* sequence);
void ClearBBox(const std::string& prefix,
               tensorflow::SequenceExample* sequence);

// The input and output format is a pair of <y, x> coordinates to match the
// order of bounding box coordinates.
int GetPointSize(const std::string& prefix,
                 const tensorflow::SequenceExample& sequence);
std::vector<std::pair<float, float>> GetPointAt(
    const std::string& prefix, const tensorflow::SequenceExample& sequence,
    int index);
void AddPoint(const std::string& prefix,
              const std::vector<std::pair<float, float>>& points,
              tensorflow::SequenceExample* sequence);
void ClearPoint(const std::string& prefix,
                tensorflow::SequenceExample* sequence);

// The input and output format is a pair of <y, x> coordinates to match the
// order of bounding box coordinates.
int Get3dPointSize(const std::string& prefix,
                   const tensorflow::SequenceExample& sequence);
std::vector<std::tuple<float, float, float>> Get3dPointAt(
    const std::string& prefix, const tensorflow::SequenceExample& sequence,
    int index);
void Add3dPoint(const std::string& prefix,
                const std::vector<std::tuple<float, float, float>>& points,
                tensorflow::SequenceExample* sequence);
void Clear3dPoint(const std::string& prefix,
                  tensorflow::SequenceExample* sequence);
#define FIXED_PREFIX_BBOX_ACCESSORS(identifier, prefix)                        \
  inline int CONCAT_STR3(Get, identifier,                                      \
                         Size)(const tensorflow::SequenceExample& sequence) {  \
    return GetBBoxSize(prefix, sequence);                                      \
  }                                                                            \
  inline std::vector<::mediapipe::Location> CONCAT_STR3(Get, identifier, At)(  \
      const tensorflow::SequenceExample& sequence, int index) {                \
    return GetBBoxAt(prefix, sequence, index);                                 \
  }                                                                            \
  inline void CONCAT_STR2(Add, identifier)(                                    \
      const std::vector<::mediapipe::Location>& bboxes,                        \
      tensorflow::SequenceExample* sequence) {                                 \
    return AddBBox(prefix, bboxes, sequence);                                  \
  }                                                                            \
  inline void CONCAT_STR2(                                                     \
      Clear, identifier)(tensorflow::SequenceExample * sequence) {             \
    return ClearBBox(prefix, sequence);                                        \
  }                                                                            \
  inline int CONCAT_STR3(Get, identifier, PointSize)(                          \
      const tensorflow::SequenceExample& sequence) {                           \
    return GetPointSize(prefix, sequence);                                     \
  }                                                                            \
  inline int CONCAT_STR3(Get, identifier, PointSize)(                          \
      const std::string& name, const tensorflow::SequenceExample& sequence) {  \
    return GetPointSize(name, sequence);                                       \
  }                                                                            \
  inline std::vector<std::pair<float, float>> CONCAT_STR3(                     \
      Get, identifier, PointAt)(const tensorflow::SequenceExample& sequence,   \
                                int index) {                                   \
    return GetPointAt(prefix, sequence, index);                                \
  }                                                                            \
  inline std::vector<std::pair<float, float>> CONCAT_STR3(                     \
      Get, identifier, PointAt)(const std::string& name,                       \
                                const tensorflow::SequenceExample& sequence,   \
                                int index) {                                   \
    return GetPointAt(name, sequence, index);                                  \
  }                                                                            \
  inline void CONCAT_STR3(Add, identifier, Point)(                             \
      const std::vector<std::pair<float, float>>& points,                      \
      tensorflow::SequenceExample* sequence) {                                 \
    return AddPoint(prefix, points, sequence);                                 \
  }                                                                            \
  inline void CONCAT_STR3(Add, identifier, Point)(                             \
      const std::string& name,                                                 \
      const std::vector<std::pair<float, float>>& points,                      \
      tensorflow::SequenceExample* sequence) {                                 \
    return AddPoint(name, points, sequence);                                   \
  }                                                                            \
  inline void CONCAT_STR3(Clear, identifier,                                   \
                          Point)(tensorflow::SequenceExample * sequence) {     \
    return ClearPoint(prefix, sequence);                                       \
  }                                                                            \
  inline void CONCAT_STR3(Clear, identifier, Point)(                           \
      std::string name, tensorflow::SequenceExample * sequence) {              \
    return ClearPoint(name, sequence);                                         \
  }                                                                            \
  inline int CONCAT_STR3(Get, identifier, 3dPointSize)(                        \
      const tensorflow::SequenceExample& sequence) {                           \
    return Get3dPointSize(prefix, sequence);                                   \
  }                                                                            \
  inline int CONCAT_STR3(Get, identifier, 3dPointSize)(                        \
      const std::string& name, const tensorflow::SequenceExample& sequence) {  \
    return Get3dPointSize(name, sequence);                                     \
  }                                                                            \
  inline std::vector<std::tuple<float, float, float>> CONCAT_STR3(             \
      Get, identifier, 3dPointAt)(const tensorflow::SequenceExample& sequence, \
                                  int index) {                                 \
    return Get3dPointAt(prefix, sequence, index);                              \
  }                                                                            \
  inline std::vector<std::tuple<float, float, float>> CONCAT_STR3(             \
      Get, identifier, 3dPointAt)(const std::string& name,                     \
                                  const tensorflow::SequenceExample& sequence, \
                                  int index) {                                 \
    return Get3dPointAt(name, sequence, index);                                \
  }                                                                            \
  inline void CONCAT_STR3(Add, identifier, 3dPoint)(                           \
      const std::vector<std::tuple<float, float, float>>& points,              \
      tensorflow::SequenceExample* sequence) {                                 \
    return Add3dPoint(prefix, points, sequence);                               \
  }                                                                            \
  inline void CONCAT_STR3(Add, identifier, 3dPoint)(                           \
      const std::string& name,                                                 \
      const std::vector<std::tuple<float, float, float>>& points,              \
      tensorflow::SequenceExample* sequence) {                                 \
    return Add3dPoint(name, points, sequence);                                 \
  }                                                                            \
  inline void CONCAT_STR3(Clear, identifier,                                   \
                          3dPoint)(tensorflow::SequenceExample * sequence) {   \
    return Clear3dPoint(prefix, sequence);                                     \
  }                                                                            \
  inline void CONCAT_STR3(Clear, identifier, 3dPoint)(                         \
      std::string name, tensorflow::SequenceExample * sequence) {              \
    return Clear3dPoint(name, sequence);                                       \
  }

#define PREFIXED_BBOX(identifier, prefix)                                      \
  FIXED_PREFIX_BBOX_ACCESSORS(identifier, prefix)                              \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(CONCAT_STR2(identifier, LabelString), \
                                         kRegionLabelStringKey, prefix)        \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(CONCAT_STR2(identifier, ClassString), \
                                         kRegionClassStringKey, prefix)        \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(CONCAT_STR2(identifier, TrackString), \
                                         kRegionTrackStringKey, prefix)        \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, LabelIndex),  \
                                         kRegionLabelIndexKey, prefix)         \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, ClassIndex),  \
                                         kRegionClassIndexKey, prefix)         \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, TrackIndex),  \
                                         kRegionTrackIndexKey, prefix)         \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, LabelConfidence), kRegionLabelConfidenceKey,     \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, ClassConfidence), kRegionClassConfidenceKey,     \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, TrackConfidence), kRegionTrackConfidenceKey,     \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, IsGenerated), \
                                         kRegionIsGeneratedKey, prefix)        \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, IsOccluded),  \
                                         kRegionIsOccludedKey, prefix)         \
  FIXED_PREFIX_INT64_FEATURE_LIST(CONCAT_STR2(identifier, NumRegions),         \
                                  kRegionNumRegionsKey, prefix)                \
  FIXED_PREFIX_INT64_FEATURE_LIST(CONCAT_STR2(identifier, IsAnnotated),        \
                                  kRegionIsAnnotatedKey, prefix)               \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, YMin),        \
                                         kRegionBBoxYMinKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, XMin),        \
                                         kRegionBBoxXMinKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, YMax),        \
                                         kRegionBBoxYMaxKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, XMax),        \
                                         kRegionBBoxXMaxKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, PointX),      \
                                         kRegionPointXKey, prefix)             \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, PointY),      \
                                         kRegionPointYKey, prefix)             \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, Radius),      \
                                         kRegionRadiusKey, prefix)             \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, 3dPointX),    \
                                         kRegion3dPointXKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, 3dPointY),    \
                                         kRegion3dPointYKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(CONCAT_STR2(identifier, 3dPointZ),    \
                                         kRegion3dPointZKey, prefix)           \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, EmbeddingFloats), kRegionEmbeddingFloatKey,      \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, EmbeddingEncoded), kRegionEmbeddingEncodedKey,   \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, EmbeddingConfidence),                            \
      kRegionEmbeddingConfidenceKey, prefix)                                   \
  FIXED_PREFIX_VECTOR_INT64_CONTEXT_FEATURE(                                   \
      CONCAT_STR2(identifier, EmbeddingDimensionsPerRegion),                   \
      kRegionEmbeddingDimensionsPerRegionKey, prefix)                          \
  FIXED_PREFIX_BYTES_CONTEXT_FEATURE(CONCAT_STR2(identifier, EmbeddingFormat), \
                                     kRegionEmbeddingFormatKey, prefix)        \
  FIXED_PREFIX_VECTOR_BYTES_CONTEXT_FEATURE(CONCAT_STR2(identifier, Parts),    \
                                            kRegionPartsKey, prefix)           \
  FIXED_PREFIX_INT64_FEATURE_LIST(CONCAT_STR2(identifier, Timestamp),          \
                                  kRegionTimestampKey, prefix)                 \
  FIXED_PREFIX_INT64_FEATURE_LIST(                                             \
      CONCAT_STR3(Unmodified, identifier, Timestamp),                          \
      kUnmodifiedRegionTimestampKey, prefix)

// Provides suites of functions for working with bounding boxes and predicted
// bounding boxes such as
// GetBBoxNumBoxes, GetBBoxSize, GetBBoxAt, GetBBoxLabelIndexAt, etc., and
// GetPredictedBBoxNumBoxes, GetPredictedBBoxSize, GetPredictedBBoxAt, etc.
const char kPredictedPrefix[] = "PREDICTED";
PREFIXED_BBOX(BBox, "");
PREFIXED_BBOX(PredictedBBox, kPredictedPrefix);

// ************************    IMAGES    **************************************
// Context keys:
// The format the images are encoded as (e.g. "JPEG", "PNG")
const char kImageFormatKey[] = "image/format";
// The number of channels in the image.
const char kImageChannelsKey[] = "image/channels";
// The colorspace of the image.
const char kImageColorspaceKey[] = "image/colorspace";
// The height of the image in pixels.
const char kImageHeightKey[] = "image/height";
// The width of the image in pixels.
const char kImageWidthKey[] = "image/width";
// The frame rate in images/second of media.
const char kImageFrameRateKey[] = "image/frame_rate";
// The maximum value if the images were saturated and normalized for encoding.
const char kImageSaturationKey[] = "image/saturation";
// The listing from discrete image values (as indices) to class indices.
const char kImageClassLabelIndexKey[] = "image/class/label/index";
// The listing from discrete image values (as indices) to class strings.
const char kImageClassLabelStringKey[] = "image/class/label/string";
// The listing from discrete instance indices to class indices they embody.
const char kImageObjectClassIndexKey[] = "image/object/class/index";
// The path of the image file if it did not come from a media clip.
const char kImageDataPathKey[] = "image/data_path";

// Feature list keys:
// The encoded image frame.
const char kImageEncodedKey[] = "image/encoded";
// Multiple images for the same timestep (e.g. multiview video).
const char kImageMultiEncodedKey[] = "image/multi_encoded";
// The timestamp of the frame in microseconds.
const char kImageTimestampKey[] = "image/timestamp";
// A per image label if specific frames have labels.
// If time spans have labels, segments are preferred to allow changing rates.
const char kImageLabelIndexKey[] = "image/label/index";
const char kImageLabelStringKey[] = "image/label/string";
const char kImageLabelConfidenceKey[] = "image/label/confidence";

#define PREFIXED_IMAGE(identifier, prefix)                                     \
  FIXED_PREFIX_INT64_CONTEXT_FEATURE(CONCAT_STR2(identifier, Height),          \
                                     kImageHeightKey, prefix)                  \
  FIXED_PREFIX_INT64_CONTEXT_FEATURE(CONCAT_STR2(identifier, Width),           \
                                     kImageWidthKey, prefix)                   \
  FIXED_PREFIX_INT64_CONTEXT_FEATURE(CONCAT_STR2(identifier, Channels),        \
                                     kImageChannelsKey, prefix)                \
  FIXED_PREFIX_BYTES_CONTEXT_FEATURE(CONCAT_STR2(identifier, Format),          \
                                     kImageFormatKey, prefix)                  \
  FIXED_PREFIX_BYTES_CONTEXT_FEATURE(CONCAT_STR2(identifier, Colorspace),      \
                                     kImageColorspaceKey, prefix)              \
  FIXED_PREFIX_FLOAT_CONTEXT_FEATURE(CONCAT_STR2(identifier, FrameRate),       \
                                     kImageFrameRateKey, prefix)               \
  FIXED_PREFIX_FLOAT_CONTEXT_FEATURE(CONCAT_STR2(identifier, Saturation),      \
                                     kImageSaturationKey, prefix)              \
  FIXED_PREFIX_BYTES_CONTEXT_FEATURE(CONCAT_STR2(identifier, DataPath),        \
                                     kImageDataPathKey, prefix)                \
  FIXED_PREFIX_VECTOR_INT64_CONTEXT_FEATURE(                                   \
      CONCAT_STR2(identifier, ClassLabelIndex), kImageClassLabelIndexKey,      \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_BYTES_CONTEXT_FEATURE(                                   \
      CONCAT_STR2(identifier, ClassLabelString), kImageClassLabelStringKey,    \
      prefix)                                                                  \
  FIXED_PREFIX_VECTOR_INT64_CONTEXT_FEATURE(                                   \
      CONCAT_STR2(identifier, ObjectClassIndex), kImageObjectClassIndexKey,    \
      prefix)                                                                  \
  FIXED_PREFIX_BYTES_FEATURE_LIST(CONCAT_STR2(identifier, Encoded),            \
                                  kImageEncodedKey, prefix)                    \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, MultiEncoded), kImageMultiEncodedKey, prefix)    \
  FIXED_PREFIX_INT64_FEATURE_LIST(CONCAT_STR2(identifier, Timestamp),          \
                                  kImageTimestampKey, prefix)                  \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(CONCAT_STR2(identifier, LabelIndex),  \
                                         kImageLabelIndexKey, prefix)          \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(CONCAT_STR2(identifier, LabelString), \
                                         kImageLabelStringKey, prefix)         \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(                                      \
      CONCAT_STR2(identifier, LabelConfidence), kImageLabelConfidenceKey,      \
      prefix)

// Provides suites of functions for working with images and data encoded in
// images such as
// AddImageEncoded, GetImageEncodedAt, AddImageTimestamp, GetImageHeight, etc.,
// AddForwardFlowEncoded, GetForwardFlowEncodedAt, AddForwardFlowTimestamp, etc.
// AddClassSegmentationEncoded, GetClassSegmentationEncodedAt, etc., and
// AddInstanceSegmentationEncoded, GetInstanceSegmentationEncodedAt, etc.
const char kForwardFlowPrefix[] = "FORWARD_FLOW";
const char kClassSegmentationPrefix[] = "CLASS_SEGMENTATION";
const char kInstanceSegmentationPrefix[] = "INSTANCE_SEGMENTATION";
PREFIXED_IMAGE(Image, "");
PREFIXED_IMAGE(ForwardFlow, kForwardFlowPrefix);
PREFIXED_IMAGE(ClassSegmentation, kClassSegmentationPrefix);
PREFIXED_IMAGE(InstanceSegmentation, kInstanceSegmentationPrefix);

// **************************   TEXT   ****************************************
// Context keys:
// Which language text tokens are likely to be in.
const char kTextLanguageKey[] = "text/language";
// A large block of text that applies to the media.
const char kTextContextContentKey[] = "text/context/content";
// A large block of text that applies to the media as token ids.
const char kTextContextTokenIdKey[] = "text/context/token_id";
// A large block of text that applies to the media as embeddings.
const char kTextContextEmbeddingKey[] = "text/context/embedding";

// Feature list keys:
// The text contents for a given time.
const char kTextContentKey[] = "text/content";
// The start time for the text becoming relevant.
const char kTextTimestampKey[] = "text/timestamp";
// The duration where the text is relevant.
const char kTextDurationKey[] = "text/duration";
// The confidence that this is the correct text.
const char kTextConfidenceKey[] = "text/confidence";
// A floating point embedding corresponding to the text.
const char kTextEmbeddingKey[] = "text/embedding";
// An integer id corresponding to the text.
const char kTextTokenIdKey[] = "text/token/id";

BYTES_CONTEXT_FEATURE(TextLanguage, kTextLanguageKey);
BYTES_CONTEXT_FEATURE(TextContextContent, kTextContextContentKey);
VECTOR_INT64_CONTEXT_FEATURE(TextContextTokenId, kTextContextTokenIdKey);
VECTOR_FLOAT_CONTEXT_FEATURE(TextContextEmbedding, kTextContextEmbeddingKey);
BYTES_FEATURE_LIST(TextContent, kTextContentKey);
INT64_FEATURE_LIST(TextTimestamp, kTextTimestampKey);
INT64_FEATURE_LIST(TextDuration, kTextDurationKey);
FLOAT_FEATURE_LIST(TextConfidence, kTextConfidenceKey);
VECTOR_FLOAT_FEATURE_LIST(TextEmbedding, kTextEmbeddingKey);
INT64_FEATURE_LIST(TextTokenId, kTextTokenIdKey);

// ***********************    FEATURES    *************************************
// Context keys:
// The dimensions of the feature.
const char kFeatureDimensionsKey[] = "feature/dimensions";
// The rate the features are extracted per second of media.
const char kFeatureRateKey[] = "feature/rate";
// The encoding format if any for the feature.
const char kFeatureBytesFormatKey[] = "feature/bytes/format";
// For audio, the rate the samples are extracted per second of media.
const char kFeatureSampleRateKey[] = "feature/sample_rate";
// For audio, the number of channels per extracted feature.
const char kFeatureNumChannelsKey[] = "feature/num_channels";
// For audio, the number of samples per extracted feature.
const char kFeatureNumSamplesKey[] = "feature/num_samples";
// For audio, the rate the features are extracted per second of media.
const char kFeaturePacketRateKey[] = "feature/packet_rate";
// For audio, the original audio sampling rate the feature is derived from.
const char kFeatureAudioSampleRateKey[] = "feature/audio_sample_rate";
// The feature as a list of floats.
const char kContextFeatureFloatsKey[] = "context_feature/floats";
// The feature as a list of floats.
const char kContextFeatureBytesKey[] = "context_feature/bytes";
// The feature as a list of floats.
const char kContextFeatureIntsKey[] = "context_feature/ints";

// Feature list keys:
// The feature as a list of floats.
const char kFeatureFloatsKey[] = "feature/floats";
// The feature as a list of bytes. May be encoded.
const char kFeatureBytesKey[] = "feature/bytes";
// The feature as a list of ints.
const char kFeatureIntsKey[] = "feature/ints";
// The timestamp, in microseconds, of the feature.
const char kFeatureTimestampKey[] = "feature/timestamp";

// It is occasionally useful to indicate that a feature applies to a given
// range. This should be used for features only and annotations should be
// provided as segments.
const char kFeatureDurationKey[] = "feature/duration";
// Encodes an optional confidence score for generated features.
const char kFeatureConfidenceKey[] = "feature/confidence";

// Functions:

// Returns/sets a MediaPipe::Matrix for the stream with that prefix.
std::unique_ptr<mediapipe::Matrix> GetAudioFromFeatureAt(
    const std::string& prefix, const tensorflow::SequenceExample& sequence,
    int index);
void AddAudioAsFeature(const std::string& prefix,
                       const mediapipe::Matrix& audio,
                       tensorflow::SequenceExample* sequence);

PREFIXED_VECTOR_INT64_CONTEXT_FEATURE(FeatureDimensions, kFeatureDimensionsKey);
PREFIXED_FLOAT_CONTEXT_FEATURE(FeatureRate, kFeatureRateKey);
PREFIXED_VECTOR_FLOAT_CONTEXT_FEATURE(ContextFeatureFloats,
                                      kContextFeatureFloatsKey);
PREFIXED_VECTOR_BYTES_CONTEXT_FEATURE(ContextFeatureBytes,
                                      kContextFeatureBytesKey);
PREFIXED_VECTOR_INT64_CONTEXT_FEATURE(ContextFeatureInts,
                                      kContextFeatureIntsKey);
PREFIXED_BYTES_CONTEXT_FEATURE(FeatureBytesFormat, kFeatureBytesFormatKey);
PREFIXED_VECTOR_FLOAT_FEATURE_LIST(FeatureFloats, kFeatureFloatsKey);
PREFIXED_VECTOR_BYTES_FEATURE_LIST(FeatureBytes, kFeatureBytesKey);
PREFIXED_VECTOR_INT64_FEATURE_LIST(FeatureInts, kFeatureIntsKey);
PREFIXED_INT64_FEATURE_LIST(FeatureTimestamp, kFeatureTimestampKey);
PREFIXED_VECTOR_INT64_FEATURE_LIST(FeatureDuration, kFeatureDurationKey);
PREFIXED_VECTOR_FLOAT_FEATURE_LIST(FeatureConfidence, kFeatureConfidenceKey);

PREFIXED_FLOAT_CONTEXT_FEATURE(FeatureSampleRate, kFeatureSampleRateKey);
PREFIXED_INT64_CONTEXT_FEATURE(FeatureNumChannels, kFeatureNumChannelsKey);
PREFIXED_INT64_CONTEXT_FEATURE(FeatureNumSamples, kFeatureNumSamplesKey);
PREFIXED_FLOAT_CONTEXT_FEATURE(FeaturePacketRate, kFeaturePacketRateKey);
PREFIXED_FLOAT_CONTEXT_FEATURE(FeatureAudioSampleRate,
                               kFeatureAudioSampleRateKey);

// Modifies the context features to match the metadata of the features in the
// sequences. Specifically, it sets the frame indices corresponding to the
// timestamps in the label meta data based on the image timestamps. For
// encoded images, encoded optical flow, and encoded human pose puppets the
// image format, height, width, channels, and frame rate are written as
// metadata. For float feature lists, the frame rate and dimensions are
// calculated. If the float feature dimensions are already present, then the
// code verifies the number of elements matches the dimensions.
// Reconciling bounding box annotations is optional because will remove
// annotations if the sequence rate is lower than the annotation rate.
absl::Status ReconcileMetadata(bool reconcile_bbox_annotations,
                               bool reconcile_region_annotations,
                               tensorflow::SequenceExample* sequence);
}  // namespace mediasequence
}  // namespace mediapipe

#endif  // MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_H_
