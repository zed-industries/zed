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

NS_ASSUME_NONNULL_BEGIN

/** An wrapper class to store pointer to a float array and its size. */
@interface TFLFloatBuffer : NSObject <NSCopying>

/** Capacity of the array in number of elements. */
@property(nonatomic, readonly) NSUInteger size;

/** Pointer to float array wrapped by `TFLFloatBuffer`. */
@property(nonatomic, readonly) float *data;

/**
 * Initializes a new `TFLFloatBuffer` by copying the elements of the given float data array.
 *
 * @param data A pointer to a float data array whose values are to be copied into the buffer.
 * @param size Size of the array float data array.
 *
 * @return A new instance of `TFLFloatBuffer` initialized with the elements of the given float data
 * array.
 */
- (instancetype)initWithData:(float *)data size:(NSUInteger)size;

/**
 * Initializes a `TFLFloatBuffer` of the specified size with zeros.
 *
 * @param size Number of elements the `TFLFloatBuffer` can hold.
 *
 * @return A new instance of `TFLFloatBuffer` of the given size with all elements initialized to
 * zero.
 */
- (instancetype)initWithSize:(NSUInteger)size;

/** Clears the `TFLFloatBuffer` by setting all elements to zero */
- (void)clear;

@end

NS_ASSUME_NONNULL_END
