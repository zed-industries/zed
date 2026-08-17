/*
 * Copyright 2019 Google LLC.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Implementation of transcription for serialization.
//
// Input is a sequence of integers, each containing a j-bit chunk of
// a message. Output encodes the same message in a vector of Ints storing
// the message broken into k-bit chunks. Performs this transcription for the
// first input_bit_length bits encoded in input.

#ifndef RLWE_TRANSCRIPTION_H_
#define RLWE_TRANSCRIPTION_H_

#include <vector>

#include "absl/strings/str_cat.h"
#include "statusor.h"

namespace rlwe {

// Takes as template arguments the input and output integer types. It must hold
// that the input_vector is large enough to contain the input_bit_length bits.
template <typename InputInt, typename OutputInt>
rlwe::StatusOr<std::vector<OutputInt>> TranscribeBits(
    const std::vector<InputInt>& input_vector, int input_bit_length,
    int input_bits_per_int, int output_bits_per_int) {
  // Check that the templating is consistent, i.e., that we do not try to
  // extract/save more bits than available in each type.
  const int bit_size_input_type = sizeof(InputInt) * 8;
  const int bit_size_output_type = sizeof(OutputInt) * 8;
  if (input_bits_per_int > bit_size_input_type) {
    return absl::InvalidArgumentError(
        absl::StrCat("The input type only contains ", bit_size_input_type,
                     " bits, hence we cannot extract ", input_bits_per_int,
                     " bits out of each integer."));
  }
  if (output_bits_per_int > bit_size_output_type) {
    return absl::InvalidArgumentError(
        absl::StrCat("The output type only contains ", bit_size_output_type,
                     " bits, hence we cannot save ", output_bits_per_int,
                     " bits in each integer."));
  }
  if (input_bit_length < 0) {
    return absl::InvalidArgumentError(absl::StrCat(
        "The input bit length, ", input_bit_length, ", cannot be negative."));
  }
  if (input_bit_length == 0) {
    if (input_vector.empty()) {
      return std::vector<OutputInt>();
    } else {
      return absl::InvalidArgumentError(
          "Cannot transcribe an empty output vector with a non-empty input "
          "vector.");
    }
  }
  // Compute the number of input chunks
  const int input_chunks =
      (input_bit_length + input_bits_per_int - 1) / input_bits_per_int;
  // Check that the input_vector is of size at least input_chunks.
  if (input_vector.size() < input_chunks) {
    return absl::InvalidArgumentError(
        absl::StrCat("The input vector of size ", input_vector.size(),
                     " is too small to contain ", input_bit_length, " bits."));
  }
  // Initialize the output string.
  const int output_chunks =
      (input_bit_length + (output_bits_per_int - 1)) / output_bits_per_int;
  std::vector<OutputInt> output(output_chunks, 0);

  // Keep track of how many bits remain in input
  int remaining_bits_in_input = input_bit_length;
  // Iterate over the input elements and process each one completely before
  // moving to the next one. One or several output elements will be filled with
  // the entire input chunk considered.
  OutputInt* output_ptr = output.data();
  int size_output_chunk =
      std::min(remaining_bits_in_input, output_bits_per_int);
  int number_output_bits_needed = size_output_chunk;
  // Loop over all the input chunks.
  for (int i = 0; i < input_chunks; i++) {
    // Number of bits in "input"
    int number_bits_in_input =
        std::min(input_bits_per_int, remaining_bits_in_input);
    // Load input and put the bits in the most significant bits of in.
    InputInt input = input_vector[i]
                     << (sizeof(InputInt) * 8 - number_bits_in_input);

    // Use all the bits in "in" before loading the next input
    while (number_bits_in_input > 0) {
      // If no bit is needed in output, go to the next element, and set the
      // number of bits needed to the minimum of output_bits_per_int and number
      // of remaining bits in case the last output cannot be filled completely.
      if (number_output_bits_needed == 0) {
        output_ptr++;
        size_output_chunk =
            std::min(remaining_bits_in_input, output_bits_per_int);
        number_output_bits_needed = size_output_chunk;
      }
      // Compute the number of bits we can process
      int number_bits_to_process =
          std::min(number_bits_in_input, number_output_bits_needed);
      // Keep only number_bits_to_process bits in the most significant bits of
      // "input" (so shift left by the difference).
      InputInt bits_left = input
                           << (number_bits_in_input - number_bits_to_process);
      // Move these bits to the least significant bits of an OutputInt (hence,
      // shift right by the size of an InputInt minus the number of bits that
      // are being processed.
      OutputInt mask = static_cast<OutputInt>(
          bits_left >> (sizeof(InputInt) * 8 - number_bits_to_process));
      // Xor the mask at the right place (hence shift left by the number of
      // output bits already processed).
      *output_ptr |= (mask << (size_output_chunk - number_output_bits_needed));
      // Update the number of output bits needed and in "input".
      number_bits_in_input -= number_bits_to_process;
      number_output_bits_needed -= number_bits_to_process;
    }
    // At most input_bits_per_int bits just got read, so we update the number of
    // remaining bits in input. This may end up to be negative, but only when we
    // are exiting the loop.
    remaining_bits_in_input -= input_bits_per_int;
  }
  return output;
}

}  // namespace rlwe

#endif  // RLWE_TRANSCRIPTION_H_
