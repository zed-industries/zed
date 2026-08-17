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

// A set of lightweight functions and macros to simplify access to tensorflow
// SequenceExample features, and macros to create getters and setters for common
// types. In general, the definitions in media_sequence.h should be sufficient
// for most cases.
//
// Four low-level patterns can be stored in SequenceExamples:
//  Single elements per sequence.
//  Vector elements per sequence.
//  Single elements per timestep.
//  Vector elements per timestep.
//
// This utility enable creating functions for each pattern for each of the
// data types in SequenceExamples (bytes, floats, ints). Each macro takes a name
// to use in the code and a key to use in the SequenceExample. For each pattern
// the most basic function prototypes for the name='MyFeature' are similar to:
//
// {BYTES,INT64,FLOAT}_CONTEXT_FEATURE:
//   string GetMyFeatureKey(sequence)
//   bool HasMyFeature(sequence)
//   void ClearMyFeature(*sequence)
//   void SetMyFeature(value, *sequence)
//   TYPE GetMyFeature(sequence)
//
// VECTOR_{BYTES,INT64,FLOAT}_CONTEXT_FEATURE:
//   string GetMyFeatureKey(sequence)
//   bool HasMyFeature(sequence)
//   void ClearMyFeature(*sequence)
//   void SetMyFeature(repeated_value, *sequence)
//   void AddMyFeature(value, *sequence)
//   int GetMyFeatureSize(sequence)
//   Repeated<TYPE> GetMyFeature(sequence)
//   TYPE GetMyFeatureAt(sequence)
//
// {BYTES,INT64,FLOAT}_FEATURE_LIST:
//   string GetMyFeatureKey(sequence)
//   bool HasMyFeature(sequence)
//   void ClearMyFeature(*sequence)
//   void AddMyFeature(value, *sequence)
//   int GetMyFeatureSize(sequence)
//   TYPE GetMyFeatureAt(sequence)
//
// VECTOR_{BYTES,INT64,FLOAT}_FEATURE_LIST:
//   string GetMyFeatureKey(sequence)
//   bool HasMyFeature(sequence)
//   void ClearMyFeature(*sequence)
//   void AddMyFeature(repeated_value, *sequence)
//   int GetMyFeatureSize(sequence)
//   Repeated<TYPE> GetMyFeatureAt(sequence)
//
// To see the exact types, please see the actual definitions, but this list
// should be sufficient for quick reference.
//
// Each function is also overloaded to accept a prefix for the key. E.g.
// HasMyFeature(sequence) takes a prefix HasMyFeature(prefix, sequence). In the
// sequence example, if the key was 'my_feature' then adding a prefix of
// 'PREFIX' results in the key 'PREFIX/my_feature'. Note that the / is added
// automatically between the prefix and the key. Prefixes are particularly
// useful when multiple types of data have similar or identical structure but
// are derived from different means. For example, stereo images could be encoded
// AddImageEncoded("LEFT", left_encoded_image_string, &sequence); and
// AddImageEncdedd("RIGHT", right_encoded_image_string, &sequence);.
//
// To avoid needing to repeatedly specify prefixes, prefixes can be baked into
// the functions with a new name. Calling a macro starting with
// FIXED_PREFIX_...(name, key, prefix) will create the same API as the version
// without a prefix, but the prefix will be used for all calls. Calling one
// of the created functions with a new prefix replaces the fixed prefix and does
// not prepend it. Using the example above,
// FIXED_PREFIX_BYTES_FEATURE_LIST(LeftImageEncoded, 'image/encoded', 'LEFT')
// allows calls to AddLeftImageEncoded(left_encoded_image_string, &sequence) and
// AddImageEncoded("LEFT", left_encoded_image_string, &sequence) to have the
// same effect of adding data to 'LEFT/image/encoded'.

#ifndef MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_UTIL_H_
#define MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_UTIL_H_

#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "tensorflow/core/example/example.pb.h"
#include "tensorflow/core/example/feature.pb.h"

namespace mediapipe {
namespace mediasequence {

// Returns true if the key is in the sequence's context.
inline const bool HasContext(const tensorflow::SequenceExample& sequence,
                             const std::string& key) {
  return (sequence.context().feature().find(key) !=
          sequence.context().feature().end());
}

inline const std::string merge_prefix(const std::string& prefix,
                                      const std::string& key) {
  if (prefix.empty()) {
    return key;
  } else {
    return prefix + "/" + key;
  }
}

// Returns a refrerence to the feature in context with the provided key, which
// must exist.
inline const tensorflow::Feature& GetContext(
    const tensorflow::SequenceExample& sequence, const std::string& key) {
  // proto map's at function also checks whether key is present, but it doesn't
  // print the missing key when it check-fails.
  const auto it = sequence.context().feature().find(key);
  ABSL_CHECK(it != sequence.context().feature().end())
      << "Could not find context key " << key << ". Sequence: \n"
      << sequence.DebugString();
  return it->second;
}

// Returns a pointer to the feature in context with the provided key, inserting
// it if necessary.
inline tensorflow::Feature* MutableContext(
    const std::string& key, tensorflow::SequenceExample* sequence) {
  return &((*sequence->mutable_context()->mutable_feature())[key]);
}

// Clears the context key specified then adds a new value.
inline void SetContextFloat(const std::string& key, float value,
                            tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_float_list()->clear_value();
  MutableContext(key, sequence)->mutable_float_list()->add_value(value);
}

inline void SetContextInt64(const std::string& key, int64_t value,
                            tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_int64_list()->clear_value();
  MutableContext(key, sequence)->mutable_int64_list()->add_value(value);
}

inline void SetContextBytes(const std::string& key, const std::string& value,
                            tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_bytes_list()->clear_value();
  MutableContext(key, sequence)->mutable_bytes_list()->add_value(value);
}

template <typename TContainer>
void SetContextFloatList(const std::string& key, const TContainer& values,
                         tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_float_list()->clear_value();
  for (auto value : values) {
    MutableContext(key, sequence)->mutable_float_list()->add_value(value);
  }
}

template <typename TContainer>
void SetContextInt64List(const std::string& key, const TContainer& values,
                         tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_int64_list()->clear_value();
  for (auto value : values) {
    MutableContext(key, sequence)->mutable_int64_list()->add_value(value);
  }
}

template <typename TContainer>
void SetContextBytesList(const std::string& key, const TContainer& values,
                         tensorflow::SequenceExample* sequence) {
  MutableContext(key, sequence)->mutable_bytes_list()->clear_value();
  for (const auto& value : values) {
    MutableContext(key, sequence)->mutable_bytes_list()->add_value(value);
  }
}

// Returns true if the key is in the sequence's FeatureLists.
inline const bool HasFeatureList(const tensorflow::SequenceExample& sequence,
                                 const std::string& key) {
  return (sequence.feature_lists().feature_list().find(key) !=
          sequence.feature_lists().feature_list().end());
}

// Returns a refrerence to the feature list with the provided key, which must
// exist.
inline const tensorflow::FeatureList& GetFeatureList(
    const tensorflow::SequenceExample& sequence, const std::string& key) {
  return sequence.feature_lists().feature_list().at(key);
}

// Returns a pointer to the feature list with the provided key, inserting
// it if necessary.
inline tensorflow::FeatureList* MutableFeatureList(
    const std::string& key, tensorflow::SequenceExample* sequence) {
  return &((*sequence->mutable_feature_lists()->mutable_feature_list())[key]);
}

// Returns the size of the FeatureList or 0 if the feature list is not present.
inline const int GetFeatureListSize(const tensorflow::SequenceExample& sequence,
                                    const std::string& key) {
  if (HasFeatureList(sequence, key)) {
    return GetFeatureList(sequence, key).feature_size();
  } else {
    return 0;
  }
}

// Returns a refrerence to the float values for the feature list indicated by
// key at the provided sequence index.
inline const proto_ns::RepeatedField<float>& GetFloatsAt(
    const tensorflow::SequenceExample& sequence, const std::string& key,
    const int index) {
  const tensorflow::FeatureList& fl = GetFeatureList(sequence, key);
  ABSL_CHECK_LT(index, fl.feature_size())
      << "Sequence: \n " << sequence.DebugString();
  return fl.feature().Get(index).float_list().value();
}

// Returns a refrerence to the int64_t values for the feature list indicated by
// key at the provided sequence index.
inline const proto_ns::RepeatedField<int64_t>& GetInt64sAt(
    const tensorflow::SequenceExample& sequence, const std::string& key,
    const int index) {
  const tensorflow::FeatureList& fl = GetFeatureList(sequence, key);
  ABSL_CHECK_LT(index, fl.feature_size())
      << "Sequence: \n " << sequence.DebugString();
  return fl.feature().Get(index).int64_list().value();
}

// Returns a refrerence to the string values for the feature list indicated by
// key at the provided sequence index.
inline const proto_ns::RepeatedPtrField<std::string>& GetBytesAt(
    const tensorflow::SequenceExample& sequence, const std::string& key,
    const int index) {
  const tensorflow::FeatureList& fl = GetFeatureList(sequence, key);
  ABSL_CHECK_LT(index, fl.feature_size())
      << "Sequence: \n " << sequence.DebugString();
  return fl.feature().Get(index).bytes_list().value();
}

// Adds any iterable (with begin and end) to a FeatureList as a float Feature.
template <typename TContainer>
void AddFloatContainer(const std::string& key, const TContainer& float_list,
                       tensorflow::SequenceExample* sequence) {
  auto* feature = MutableFeatureList(key, sequence)->add_feature();
  std::copy(float_list.begin(), float_list.end(),
            proto_ns::RepeatedFieldBackInserter(
                feature->mutable_float_list()->mutable_value()));
}

// Adds any iterable (with begin and end) to a FeatureList as a int64_t Feature.
template <typename TContainer>
void AddInt64Container(const std::string& key, const TContainer& int64_list,
                       tensorflow::SequenceExample* sequence) {
  auto* feature = MutableFeatureList(key, sequence)->add_feature();
  std::copy(int64_list.begin(), int64_list.end(),
            proto_ns::RepeatedFieldBackInserter(
                feature->mutable_int64_list()->mutable_value()));
}

// Adds any iterable (with begin and end) to a FeatureList as a bytes Feature.
template <typename TContainer>
void AddBytesContainer(const std::string& key, const TContainer& bytes_list,
                       tensorflow::SequenceExample* sequence) {
  auto* feature = MutableFeatureList(key, sequence)->add_feature();
  std::copy(bytes_list.begin(), bytes_list.end(),
            proto_ns::RepeatedPtrFieldBackInserter(
                feature->mutable_bytes_list()->mutable_value()));
}

// The macros provided below are useful for creating getters and setters for
// keys and values in a tf::SequenceExample. You only need to specify the C++
// name to use in the functions and the string key used in the SequenceExample
// proto maps. Macro versions exist for {strings, int64s, and floats} for
// creating singular or repeated context features and singular or repeated
// feature_list features.

// Helpers to create functions names in the macros below.
#define CONCAT_STR2(a, b) a##b
#define CONCAT_STR3(a, b, c) a##b##c

// This macro creates functions for HasX, GetX, ClearX, and SetX where X is a
// name and the value stored is a string in the context.
#define PREFIXED_BYTES_CONTEXT_FEATURE(name, key)                             \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasContext(sequence, merge_prefix(prefix, key));                   \
  }                                                                           \
  inline const std::string& CONCAT_STR2(Get, name)(                           \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .bytes_list()                                                         \
        .value(0);                                                            \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_context()->mutable_feature()->erase(                    \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const std::string& value,                \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextBytes(merge_prefix(prefix, key), value, sequence);              \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_BYTES_CONTEXT_FEATURE(name, key, prefix)                 \
  PREFIXED_BYTES_CONTEXT_FEATURE(name, key);                                  \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const std::string& CONCAT_STR2(                                      \
      Get, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Get, name)(prefix, sequence);                          \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& value,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define BYTES_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_BYTES_CONTEXT_FEATURE(name, key, "")

// This macro creates functions for HasX, GetX, ClearX, and SetX where X is a
// name and the value stored is a int64_t in the context.
#define PREFIXED_INT64_CONTEXT_FEATURE(name, key)                             \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasContext(sequence, merge_prefix(prefix, key));                   \
  }                                                                           \
  inline const int64_t CONCAT_STR2(Get, name)(                                \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .int64_list()                                                         \
        .value(0);                                                            \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_context()->mutable_feature()->erase(                    \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const int64_t& value,                    \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextInt64(merge_prefix(prefix, key), value, sequence);              \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_INT64_CONTEXT_FEATURE(name, key, prefix)                 \
  PREFIXED_INT64_CONTEXT_FEATURE(name, key);                                  \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int64_t CONCAT_STR2(                                           \
      Get, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Get, name)(prefix, sequence);                          \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const int64_t& value,                    \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define INT64_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_INT64_CONTEXT_FEATURE(name, key, "");

// This macro creates functions for HasX, GetX, ClearX, and SetX where X is a
// name and the value stored is a float in the context.
#define PREFIXED_FLOAT_CONTEXT_FEATURE(name, key)                             \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasContext(sequence, merge_prefix(prefix, key));                   \
  }                                                                           \
  inline const float CONCAT_STR2(Get, name)(                                  \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .float_list()                                                         \
        .value(0);                                                            \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_context()->mutable_feature()->erase(                    \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const float& value,                      \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextFloat(merge_prefix(prefix, key), value, sequence);              \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_FLOAT_CONTEXT_FEATURE(name, key, prefix)                 \
  PREFIXED_FLOAT_CONTEXT_FEATURE(name, key);                                  \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const float CONCAT_STR2(                                             \
      Get, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Get, name)(prefix, sequence);                          \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const float& value,                      \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define FLOAT_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_FLOAT_CONTEXT_FEATURE(name, key, "")

// This macro creates functions for HasX, GetX, ClearX, SetX, GetXSize, GetXAt,
// and AddX where X is a name and the value stored is a sequence of strings in
// the context.
#define PREFIXED_VECTOR_BYTES_CONTEXT_FEATURE(name, key)                       \
  inline const bool CONCAT_STR2(Has, name)(                                    \
      const std::string& prefix,                                               \
      const tensorflow::SequenceExample& sequence) {                           \
    return HasContext(sequence, merge_prefix(prefix, key));                    \
  }                                                                            \
  inline const int CONCAT_STR3(Get, name, Size)(                               \
      const std::string& prefix,                                               \
      const tensorflow::SequenceExample& sequence) {                           \
    if (CONCAT_STR2(Has, name)(prefix, sequence)) {                            \
      return GetContext(sequence, merge_prefix(prefix, key))                   \
          .bytes_list()                                                        \
          .value_size();                                                       \
    } else {                                                                   \
      return 0;                                                                \
    }                                                                          \
  }                                                                            \
  inline const proto_ns::RepeatedPtrField<std::string>& CONCAT_STR2(           \
      Get, name)(const std::string& prefix,                                    \
                 const tensorflow::SequenceExample& sequence) {                \
    return GetContext(sequence, merge_prefix(prefix, key))                     \
        .bytes_list()                                                          \
        .value();                                                              \
  }                                                                            \
  inline const std::string& CONCAT_STR3(Get, name, At)(                        \
      const std::string& prefix, const tensorflow::SequenceExample& sequence,  \
      int i) {                                                                 \
    return GetContext(sequence, merge_prefix(prefix, key))                     \
        .bytes_list()                                                          \
        .value(i);                                                             \
  }                                                                            \
  inline void CONCAT_STR2(Clear, name)(                                        \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {      \
    sequence->mutable_context()->mutable_feature()->erase(                     \
        merge_prefix(prefix, key));                                            \
  }                                                                            \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,                \
                                     const ::std::vector<std::string>& values, \
                                     tensorflow::SequenceExample* sequence) {  \
    SetContextBytesList(merge_prefix(prefix, key), values, sequence);          \
  }                                                                            \
  template <typename TContainer>                                               \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,                \
                                     const TContainer& values,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    SetContextBytesList(merge_prefix(prefix, key), values, sequence);          \
  }                                                                            \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,                \
                                     const std::string& value,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    MutableContext(merge_prefix(prefix, key), sequence)                        \
        ->mutable_bytes_list()                                                 \
        ->add_value(value);                                                    \
  }                                                                            \
  inline const std::string CONCAT_STR3(Get, name,                              \
                                       Key)(const std::string& prefix) {       \
    return merge_prefix(prefix, key);                                          \
  }

#define FIXED_PREFIX_VECTOR_BYTES_CONTEXT_FEATURE(name, key, prefix)           \
  PREFIXED_VECTOR_BYTES_CONTEXT_FEATURE(name, key);                            \
  inline const bool CONCAT_STR2(                                               \
      Has, name)(const tensorflow::SequenceExample& sequence) {                \
    return CONCAT_STR2(Has, name)(prefix, sequence);                           \
  }                                                                            \
  inline const int CONCAT_STR3(                                                \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {          \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                     \
  }                                                                            \
  inline const proto_ns::RepeatedPtrField<std::string>& CONCAT_STR2(           \
      Get, name)(const tensorflow::SequenceExample& sequence) {                \
    return CONCAT_STR2(Get, name)(prefix, sequence);                           \
  }                                                                            \
  inline const std::string& CONCAT_STR3(Get, name, At)(                        \
      const tensorflow::SequenceExample& sequence, int i) {                    \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, i);                    \
  }                                                                            \
  inline void CONCAT_STR2(Clear,                                               \
                          name)(tensorflow::SequenceExample * sequence) {      \
    CONCAT_STR2(Clear, name)(prefix, sequence);                                \
  }                                                                            \
  inline void CONCAT_STR2(Set, name)(const ::std::vector<std::string>& values, \
                                     tensorflow::SequenceExample* sequence) {  \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                          \
  }                                                                            \
  template <typename TContainer>                                               \
  inline void CONCAT_STR2(Set, name)(const TContainer& values,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                          \
  }                                                                            \
  inline void CONCAT_STR2(Add, name)(const std::string& value,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                           \
  }                                                                            \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                     \
    return merge_prefix(prefix, key);                                          \
  }

#define VECTOR_BYTES_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_VECTOR_BYTES_CONTEXT_FEATURE(name, key, "");

// This macro creates functions for HasX, GetX, ClearX, SetX, GetXAt, and AddX
// where X is a name and the value stored is a sequence of int64s  in the
// context.
#define PREFIXED_VECTOR_INT64_CONTEXT_FEATURE(name, key)                      \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasContext(sequence, merge_prefix(prefix, key));                   \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    if (CONCAT_STR2(Has, name)(prefix, sequence)) {                           \
      return GetContext(sequence, merge_prefix(prefix, key))                  \
          .int64_list()                                                       \
          .value_size();                                                      \
    } else {                                                                  \
      return 0;                                                               \
    }                                                                         \
  }                                                                           \
  inline const proto_ns::RepeatedField<int64_t>& CONCAT_STR2(Get, name)(      \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .int64_list()                                                         \
        .value();                                                             \
  }                                                                           \
  inline const int64_t CONCAT_STR3(Get, name, At)(                            \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int i) {                                                                \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .int64_list()                                                         \
        .value(i);                                                            \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_context()->mutable_feature()->erase(                    \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const ::std::vector<int64_t>& values,    \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextInt64List(merge_prefix(prefix, key), values, sequence);         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextInt64List(merge_prefix(prefix, key), values, sequence);         \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const int64_t& value,                    \
                                     tensorflow::SequenceExample* sequence) { \
    MutableContext(merge_prefix(prefix, key), sequence)                       \
        ->mutable_int64_list()                                                \
        ->add_value(value);                                                   \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_VECTOR_INT64_CONTEXT_FEATURE(name, key, prefix)          \
  PREFIXED_VECTOR_INT64_CONTEXT_FEATURE(name, key);                           \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const proto_ns::RepeatedField<int64_t>& CONCAT_STR2(                 \
      Get, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Get, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int64_t CONCAT_STR3(Get, name, At)(                            \
      const tensorflow::SequenceExample& sequence, int i) {                   \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, i);                   \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const ::std::vector<int64_t>& values,    \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Set, name)(const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                         \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const int64_t& value,                    \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define VECTOR_INT64_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_VECTOR_INT64_CONTEXT_FEATURE(name, key, "");

// This macro creates functions for HasX, GetX, ClearX, SetX, GetXAt, and AddX
// where X is a name and the value stored is a sequence of floats  in the
// context.
#define PREFIXED_VECTOR_FLOAT_CONTEXT_FEATURE(name, key)                      \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasContext(sequence, merge_prefix(prefix, key));                   \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    if (CONCAT_STR2(Has, name)(prefix, sequence)) {                           \
      return GetContext(sequence, merge_prefix(prefix, key))                  \
          .float_list()                                                       \
          .value_size();                                                      \
    } else {                                                                  \
      return 0;                                                               \
    }                                                                         \
  }                                                                           \
  inline const proto_ns::RepeatedField<float>& CONCAT_STR2(Get, name)(        \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .float_list()                                                         \
        .value();                                                             \
  }                                                                           \
  inline const float CONCAT_STR3(Get, name, At)(                              \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int i) {                                                                \
    return GetContext(sequence, merge_prefix(prefix, key))                    \
        .float_list()                                                         \
        .value(i);                                                            \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_context()->mutable_feature()->erase(                    \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const ::std::vector<float>& values,      \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextFloatList(merge_prefix(prefix, key), values, sequence);         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Set, name)(const std::string& prefix,               \
                                     const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    SetContextFloatList(merge_prefix(prefix, key), values, sequence);         \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const float& value,                      \
                                     tensorflow::SequenceExample* sequence) { \
    MutableContext(merge_prefix(prefix, key), sequence)                       \
        ->mutable_float_list()                                                \
        ->add_value(value);                                                   \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_VECTOR_FLOAT_CONTEXT_FEATURE(name, key, prefix)          \
  PREFIXED_VECTOR_FLOAT_CONTEXT_FEATURE(name, key);                           \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const proto_ns::RepeatedField<float>& CONCAT_STR2(                   \
      Get, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Get, name)(prefix, sequence);                          \
  }                                                                           \
  inline const float CONCAT_STR3(Get, name, At)(                              \
      const tensorflow::SequenceExample& sequence, int i) {                   \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, i);                   \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Set, name)(const ::std::vector<float>& values,      \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Set, name)(const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Set, name)(prefix, values, sequence);                         \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const float& value,                      \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define VECTOR_FLOAT_CONTEXT_FEATURE(name, key) \
  FIXED_PREFIX_VECTOR_FLOAT_CONTEXT_FEATURE(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a string in a feature_list.
#define PREFIXED_BYTES_FEATURE_LIST(name, key)                                \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasFeatureList(sequence, merge_prefix(prefix, key));               \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));           \
  }                                                                           \
  inline const std::string& CONCAT_STR3(Get, name, At)(                       \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int index) {                                                            \
    return GetBytesAt(sequence, merge_prefix(prefix, key), index).Get(0);     \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(         \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const std::string& value,                \
                                     tensorflow::SequenceExample* sequence) { \
    MutableFeatureList(merge_prefix(prefix, key), sequence)                   \
        ->add_feature()                                                       \
        ->mutable_bytes_list()                                                \
        ->add_value(value);                                                   \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_BYTES_FEATURE_LIST(name, key, prefix)                    \
  PREFIXED_BYTES_FEATURE_LIST(name, key);                                     \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const std::string& CONCAT_STR3(Get, name, At)(                       \
      const tensorflow::SequenceExample& sequence, int index) {               \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);               \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& value,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define BYTES_FEATURE_LIST(name, key) \
  FIXED_PREFIX_BYTES_FEATURE_LIST(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a int64_t in a feature_list.
#define PREFIXED_INT64_FEATURE_LIST(name, key)                                \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasFeatureList(sequence, merge_prefix(prefix, key));               \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));           \
  }                                                                           \
  inline const int64_t CONCAT_STR3(Get, name, At)(                            \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int index) {                                                            \
    return GetInt64sAt(sequence, merge_prefix(prefix, key), index).Get(0);    \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(         \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const int64_t value,                     \
                                     tensorflow::SequenceExample* sequence) { \
    MutableFeatureList(merge_prefix(prefix, key), sequence)                   \
        ->add_feature()                                                       \
        ->mutable_int64_list()                                                \
        ->add_value(value);                                                   \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_INT64_FEATURE_LIST(name, key, prefix)                    \
  PREFIXED_INT64_FEATURE_LIST(name, key);                                     \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const int64_t CONCAT_STR3(Get, name, At)(                            \
      const tensorflow::SequenceExample& sequence, int index) {               \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);               \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const int64_t& value,                    \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define INT64_FEATURE_LIST(name, key) \
  FIXED_PREFIX_INT64_FEATURE_LIST(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a float in a feature_list.
#define PREFIXED_FLOAT_FEATURE_LIST(name, key)                                \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasFeatureList(sequence, merge_prefix(prefix, key));               \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));           \
  }                                                                           \
  inline const float CONCAT_STR3(Get, name, At)(                              \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int index) {                                                            \
    return GetFloatsAt(sequence, merge_prefix(prefix, key), index).Get(0);    \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(         \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const float value,                       \
                                     tensorflow::SequenceExample* sequence) { \
    MutableFeatureList(merge_prefix(prefix, key), sequence)                   \
        ->add_feature()                                                       \
        ->mutable_float_list()                                                \
        ->add_value(value);                                                   \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_FLOAT_FEATURE_LIST(name, key, prefix)                    \
  PREFIXED_FLOAT_FEATURE_LIST(name, key);                                     \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const float CONCAT_STR3(Get, name, At)(                              \
      const tensorflow::SequenceExample& sequence, int index) {               \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);               \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const float& value,                      \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, value, sequence);                          \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define FLOAT_FEATURE_LIST(name, key) \
  FIXED_PREFIX_FLOAT_FEATURE_LIST(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a sequence of strings in a
// feature_list.
#define PREFIXED_VECTOR_BYTES_FEATURE_LIST(name, key)                          \
  inline const bool CONCAT_STR2(Has, name)(                                    \
      const std::string& prefix,                                               \
      const tensorflow::SequenceExample& sequence) {                           \
    return HasFeatureList(sequence, merge_prefix(prefix, key));                \
  }                                                                            \
  inline const int CONCAT_STR3(Get, name, Size)(                               \
      const std::string& prefix,                                               \
      const tensorflow::SequenceExample& sequence) {                           \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));            \
  }                                                                            \
  inline const proto_ns::RepeatedPtrField<std::string>& CONCAT_STR3(           \
      Get, name, At)(const std::string& prefix,                                \
                     const tensorflow::SequenceExample& sequence, int index) { \
    return GetBytesAt(sequence, merge_prefix(prefix, key), index);             \
  }                                                                            \
  inline void CONCAT_STR2(Clear, name)(                                        \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {      \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(          \
        merge_prefix(prefix, key));                                            \
  }                                                                            \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,                \
                                     const ::std::vector<std::string>& values, \
                                     tensorflow::SequenceExample* sequence) {  \
    AddBytesContainer(merge_prefix(prefix, key), values, sequence);            \
  }                                                                            \
  template <typename TContainer>                                               \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,                \
                                     const TContainer& values,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    AddBytesContainer(merge_prefix(prefix, key), values, sequence);            \
  }                                                                            \
  inline const std::string CONCAT_STR3(Get, name,                              \
                                       Key)(const std::string& prefix) {       \
    return merge_prefix(prefix, key);                                          \
  }

#define FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(name, key, prefix)              \
  PREFIXED_VECTOR_BYTES_FEATURE_LIST(name, key);                               \
  inline const bool CONCAT_STR2(                                               \
      Has, name)(const tensorflow::SequenceExample& sequence) {                \
    return CONCAT_STR2(Has, name)(prefix, sequence);                           \
  }                                                                            \
  inline const int CONCAT_STR3(                                                \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {          \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                     \
  }                                                                            \
  inline const proto_ns::RepeatedPtrField<std::string>& CONCAT_STR3(           \
      Get, name, At)(const tensorflow::SequenceExample& sequence, int index) { \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);                \
  }                                                                            \
  inline void CONCAT_STR2(Clear,                                               \
                          name)(tensorflow::SequenceExample * sequence) {      \
    CONCAT_STR2(Clear, name)(prefix, sequence);                                \
  }                                                                            \
  inline void CONCAT_STR2(Add, name)(const ::std::vector<std::string>& values, \
                                     tensorflow::SequenceExample* sequence) {  \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                          \
  }                                                                            \
  template <typename TContainer>                                               \
  inline void CONCAT_STR2(Add, name)(const TContainer& values,                 \
                                     tensorflow::SequenceExample* sequence) {  \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                          \
  }                                                                            \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                     \
    return merge_prefix(prefix, key);                                          \
  }

#define VECTOR_BYTES_FEATURE_LIST(name, key) \
  FIXED_PREFIX_VECTOR_BYTES_FEATURE_LIST(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a sequence of int64_t in a
// feature_list.
#define PREFIXED_VECTOR_INT64_FEATURE_LIST(name, key)                         \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasFeatureList(sequence, merge_prefix(prefix, key));               \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));           \
  }                                                                           \
  inline const proto_ns::RepeatedField<int64_t>& CONCAT_STR3(Get, name, At)(  \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int index) {                                                            \
    return GetInt64sAt(sequence, merge_prefix(prefix, key), index);           \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(         \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const ::std::vector<int64_t>& values,    \
                                     tensorflow::SequenceExample* sequence) { \
    AddInt64Container(merge_prefix(prefix, key), values, sequence);           \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    AddInt64Container(merge_prefix(prefix, key), values, sequence);           \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(name, key, prefix)             \
  PREFIXED_VECTOR_INT64_FEATURE_LIST(name, key);                              \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const proto_ns::RepeatedField<int64_t>& CONCAT_STR3(Get, name, At)(  \
      const tensorflow::SequenceExample& sequence, int index) {               \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);               \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const ::std::vector<int64_t>& values,    \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Add, name)(const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                         \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define VECTOR_INT64_FEATURE_LIST(name, key) \
  FIXED_PREFIX_VECTOR_INT64_FEATURE_LIST(name, key, "");

// This macro creates functions for HasX, GetXSize, GetXAt, ClearX, and AddX
// where X is a name and the value stored is a sequence of floats in a
// feature_list.
#define PREFIXED_VECTOR_FLOAT_FEATURE_LIST(name, key)                         \
  inline const bool CONCAT_STR2(Has, name)(                                   \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return HasFeatureList(sequence, merge_prefix(prefix, key));               \
  }                                                                           \
  inline const int CONCAT_STR3(Get, name, Size)(                              \
      const std::string& prefix,                                              \
      const tensorflow::SequenceExample& sequence) {                          \
    return GetFeatureListSize(sequence, merge_prefix(prefix, key));           \
  }                                                                           \
  inline const proto_ns::RepeatedField<float>& CONCAT_STR3(Get, name, At)(    \
      const std::string& prefix, const tensorflow::SequenceExample& sequence, \
      int index) {                                                            \
    return GetFloatsAt(sequence, merge_prefix(prefix, key), index);           \
  }                                                                           \
  inline void CONCAT_STR2(Clear, name)(                                       \
      const std::string& prefix, tensorflow::SequenceExample* sequence) {     \
    sequence->mutable_feature_lists()->mutable_feature_list()->erase(         \
        merge_prefix(prefix, key));                                           \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const ::std::vector<float>& values,      \
                                     tensorflow::SequenceExample* sequence) { \
    AddFloatContainer(merge_prefix(prefix, key), values, sequence);           \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Add, name)(const std::string& prefix,               \
                                     const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    AddFloatContainer(merge_prefix(prefix, key), values, sequence);           \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name,                             \
                                       Key)(const std::string& prefix) {      \
    return merge_prefix(prefix, key);                                         \
  }

#define FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(name, key, prefix)             \
  PREFIXED_VECTOR_FLOAT_FEATURE_LIST(name, key);                              \
  inline const bool CONCAT_STR2(                                              \
      Has, name)(const tensorflow::SequenceExample& sequence) {               \
    return CONCAT_STR2(Has, name)(prefix, sequence);                          \
  }                                                                           \
  inline const int CONCAT_STR3(                                               \
      Get, name, Size)(const tensorflow::SequenceExample& sequence) {         \
    return CONCAT_STR3(Get, name, Size)(prefix, sequence);                    \
  }                                                                           \
  inline const proto_ns::RepeatedField<float>& CONCAT_STR3(Get, name, At)(    \
      const tensorflow::SequenceExample& sequence, int index) {               \
    return CONCAT_STR3(Get, name, At)(prefix, sequence, index);               \
  }                                                                           \
  inline void CONCAT_STR2(Clear,                                              \
                          name)(tensorflow::SequenceExample * sequence) {     \
    CONCAT_STR2(Clear, name)(prefix, sequence);                               \
  }                                                                           \
  inline void CONCAT_STR2(Add, name)(const ::std::vector<float>& values,      \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                         \
  }                                                                           \
  template <typename TContainer>                                              \
  inline void CONCAT_STR2(Add, name)(const TContainer& values,                \
                                     tensorflow::SequenceExample* sequence) { \
    CONCAT_STR2(Add, name)(prefix, values, sequence);                         \
  }                                                                           \
  inline const std::string CONCAT_STR3(Get, name, Key)() {                    \
    return merge_prefix(prefix, key);                                         \
  }

#define VECTOR_FLOAT_FEATURE_LIST(name, key) \
  FIXED_PREFIX_VECTOR_FLOAT_FEATURE_LIST(name, key, "");

}  // namespace mediasequence
}  // namespace mediapipe

#endif  // MEDIAPIPE_TENSORFLOW_SEQUENCE_MEDIA_SEQUENCE_UTIL_H_
