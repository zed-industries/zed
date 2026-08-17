/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Experimental multilayer metadata defined in CWG-E050.

#ifndef AOM_EXAMPLES_MULTILAYER_METADATA_H_
#define AOM_EXAMPLES_MULTILAYER_METADATA_H_

#include <cstdint>
#include <utility>
#include <vector>

namespace libaom_examples {

// std::pair<T, bool> is used to indicate presence of a field,
// like an std::optional (which cannot be used because it's C++17).
// If the boolean is true, then the value is present.

struct ColorProperties {
  bool color_range;  // true for full range values
  uint8_t color_primaries;
  uint8_t transfer_characteristics;
  uint8_t matrix_coefficients;
};

enum AlphaUse {
  ALPHA_STRAIGHT = 0,
  ALPHA_PREMULTIPLIED = 1,
  ALPHA_UNSPECIFIED = 2,
  // 3 is reserved.
};

struct AlphaInformation {
  AlphaUse alpha_use_idc;   // [0, 3]
  bool alpha_simple_flag;   // If true, all fields below are ignored.
  uint8_t alpha_bit_depth;  // [8, 15]
  uint8_t alpha_clip_idc;   // [0, 3]
  bool alpha_incr_flag;
  uint16_t alpha_transparent_value;  // [0, 1<<(alpha_bit_depth+1))
  uint16_t alpha_opaque_value;       // [0, 1<<(alpha_bit_depth+1))
  std::pair<ColorProperties, bool> alpha_color_description;
};

struct DepthRepresentationElement {
  bool sign_flag;
  uint8_t exponent;      // [0, 126] (biased exponent)
  uint8_t mantissa_len;  // [1, 32]
  uint32_t mantissa;
};

struct DepthInformation {
  std::pair<DepthRepresentationElement, bool> z_near;
  std::pair<DepthRepresentationElement, bool> z_far;
  std::pair<DepthRepresentationElement, bool> d_min;
  std::pair<DepthRepresentationElement, bool> d_max;
  uint8_t depth_representation_type;  // [0, 15]
  // Only relevant if d_min or d_max are present.
  uint8_t disparity_ref_view_id;  // [0, 3]
};

enum MultilayerUseCase {
  MULTILAYER_USE_CASE_UNSPECIFIED = 0,
  MULTILAYER_USE_CASE_GLOBAL_ALPHA = 1,
  MULTILAYER_USE_CASE_GLOBAL_DEPTH = 2,
  MULTILAYER_USE_CASE_ALPHA = 3,
  MULTILAYER_USE_CASE_DEPTH = 4,
  MULTILAYER_USE_CASE_STEREO = 5,
  MULTILAYER_USE_CASE_STEREO_GLOBAL_ALPHA = 6,
  MULTILAYER_USE_CASE_STEREO_GLOBAL_DEPTH = 7,
  MULTILAYER_USE_CASE_STEREO_ALPHA = 8,
  MULTILAYER_USE_CASE_STEREO_DEPTH = 9,
  MULTILAYER_USE_CASE_444_GLOBAL_ALPHA = 10,
  MULTILAYER_USE_CASE_444_GLOBAL_DEPTH = 11,
  MULTILAYER_USE_CASE_444 = 12,
  MULTILAYER_USE_CASE_420_444 = 13,
  // 14 to 63 are reserved.
};

enum LayerType {
  MULTILAYER_LAYER_TYPE_UNSPECIFIED = 0,
  MULTILAYER_LAYER_TYPE_TEXTURE = 1,
  MULTILAYER_LAYER_TYPE_TEXTURE_1 = 2,
  MULTILAYER_LAYER_TYPE_TEXTURE_2 = 3,
  MULTILAYER_LAYER_TYPE_TEXTURE_3 = 4,
  MULTILAYER_LAYER_TYPE_ALPHA = 5,
  MULTILAYER_LAYER_TYPE_DEPTH = 6,
  // 7 to 31 are reserved.
};

enum MultilayerMetadataScope {
  SCOPE_UNSPECIFIED = 0,
  SCOPE_LOCAL = 1,
  SCOPE_GLOBAL = 2,
  SCOPE_MIXED = 3,
};

enum MultilayerViewType {
  VIEW_UNSPECIFIED = 0,
  VIEW_CENTER = 1,
  VIEW_LEFT = 2,
  VIEW_RIGHT = 3,
  // 4 to 7 are reserved.
};

struct LayerMetadata {
  LayerType layer_type;  // [0, 31]
  bool luma_plane_only_flag;
  MultilayerViewType layer_view_type;            // [0, 7]
  uint8_t group_id;                              // [0, 3]
  uint8_t layer_dependency_idc;                  // [0, 7]
  MultilayerMetadataScope layer_metadata_scope;  // [0, 3]

  std::pair<ColorProperties, bool> layer_color_description;

  // Relevant for MULTILAYER_LAYER_TYPE_ALPHA with scope >= SCOPE_GLOBAL.
  AlphaInformation global_alpha_info;
  // Relevant for MULTILAYER_LAYER_TYPE_DEPTH with scope >= SCOPE_GLOBAL.
  DepthInformation global_depth_info;
};

struct MultilayerMetadata {
  MultilayerUseCase use_case;         // [0, 63]
  std::vector<LayerMetadata> layers;  // max size 4
};

// Parses a multilayer metadata file.
// The metadata is expected to be in a subset of the YAML format supporting
// simple lists and maps with integer values, and comments.
// Checks that the metadata is valid and terminates the process in case of
// error.
bool parse_multilayer_file(const char *metadata_path,
                           MultilayerMetadata *multilayer);

// Prints the multilayer metadata to stdout for debugging.
void print_multilayer_metadata(const MultilayerMetadata &multilayer);

// Converts a double value to a DepthRepresentationElement struct.
bool double_to_depth_representation_element(
    double v, DepthRepresentationElement *element);
// Converts a DepthRepresentationElement struct to a double value.
double depth_representation_element_to_double(
    const DepthRepresentationElement &e);

}  // namespace libaom_examples

#endif  // AOM_EXAMPLES_MULTILAYER_METADATA_H_
