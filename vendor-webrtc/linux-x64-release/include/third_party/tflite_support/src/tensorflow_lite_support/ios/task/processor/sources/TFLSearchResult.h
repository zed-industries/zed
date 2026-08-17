/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 ==============================================================================*/
#import <CoreGraphics/CoreGraphics.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/** Encapsulates a single nearest neighbor. */
NS_SWIFT_NAME(NearestNeighbor)
@interface TFLNearestNeighbor : NSObject

/**
 * User-defined metadata about the result. This could be a label, a unique ID, a serialized proto of
 * some sort, etc.
 */
@property(nonatomic, readonly) NSString *metadata;

/**
 * The distance score indicating how confident the result is. Lower is better.
 */
@property(nonatomic, readonly) CGFloat distance;

/**
 * Initializes a new `TFLNearestNeighbor`.
 *
 * @param metadata User-defined metadata about the result. This could be a label, a unique ID, a
 * serialized proto of some sort, etc.User-defined metadata about the result. This could be a label,
 * a unique ID, a serialized proto of some sort, etc.
 * @param distance The distance score indicating how confident the result is.
 *
 * @return An instance of `TFLNearestNeighbor` initialized to the given values.
 */
- (instancetype)initWithMetadata:(NSString *)metadata
                        distance:(CGFloat)distance NS_DESIGNATED_INITIALIZER;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

/** Holds results from a search task as a list of nearest neighbors. */
NS_SWIFT_NAME(SearchResult)
@interface TFLSearchResult : NSObject

/**
 * The nearest neighbors, sorted by increasing distance order.
 */
@property(nonatomic, readonly) NSArray<TFLNearestNeighbor *> *nearestNeighbors;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Initializes a new `TFLSearchResult`.
 *
 * @param nearestNeighbors An array of nearest neighbors detected in the search sorted by increasing
 * disance order.
 *
 * @return An instance of TFLSearchResult initialized to the given values.
 */
- (instancetype)initWithNearestNeighbors:(NSArray<TFLNearestNeighbor *> *)nearestNeighbors
    NS_DESIGNATED_INITIALIZER;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
