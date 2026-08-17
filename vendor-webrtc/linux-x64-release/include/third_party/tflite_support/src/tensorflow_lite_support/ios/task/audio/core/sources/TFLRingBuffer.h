// Copyright 2022 The TensorFlow Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#import <Foundation/Foundation.h>
#import "TFLFloatBuffer.h"

NS_ASSUME_NONNULL_BEGIN

/** An wrapper class which stores a buffer that is written in circular fashion. */
@interface TFLRingBuffer : NSObject

/**
 * A copy of all the internal ring buffer elements in order.
 */
@property(nullable, nonatomic, readonly) TFLFloatBuffer *floatBuffer;

/**
 * Capacity of the ring buffer in number of elements.
 */
@property(nonatomic, readonly) NSUInteger size;

/**
 * Initializes a new `TFLRingBuffer` with the given size. All elements of the
 * `TFLRingBuffer` will be initialized to zero.
 *
 * @param size Size of the ring buffer.
 *
 * @return A new instance of `TFLRingBuffer` with the given size and all elements
 * initialized to zero.
 */
- (instancetype)initWithBufferSize:(NSUInteger)size;

/**
 * Loads a slice of a float array to the ring buffer. If the float array is longer than ring
 * buffer's capacity, samples with lower indices in the array will be ignored.
 *
 * @param data A pointer to a float data array whose values are to be loaded into the ring buffer.
 * @param size Size of the float data array pointed to by param `data`.
 * @param offset Offset in array pointed to by `data` from which elements are to be loaded into the
 * ring buffer.
 * @param size Number of elements to be copied into the ring buffer, starting from `offset`.
 *
 * @return Boolean indicating success or failure of loading operation.
 */
- (BOOL)loadFloatData:(float *)data
             dataSize:(NSUInteger)dataSize
               offset:(NSUInteger)offset
                 size:(NSUInteger)size
                error:(NSError **)error;

/**
 * Returns a `TFLFloatBuffer` with a copy of size number of the ring buffer elements in order
 * starting at offset, i.e, buffer[offset:offset+size].
 *
 * @param offset Offset in the ring buffer from which elements are to be returned.
 *
 * @param size Number of elements to be returned.
 *
 * @return A new `TFLFloatBuffer` if offset + size is within the bounds of the ring buffer,
 * otherwise nil.
 */
- (nullable TFLFloatBuffer *)floatBufferWithOffset:(NSUInteger)offset size:(NSUInteger)size;

/**
 * Clears the `TFLRingBuffer` by setting all the elements to zero .
 */
- (void)clear;

@end

NS_ASSUME_NONNULL_END
