/*
 *
 * Copyright 2018 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#import "GRPCChannelPool.h"

NS_ASSUME_NONNULL_BEGIN

/** Test-only interface for \a GRPCPooledChannel. */
@interface GRPCPooledChannel (Test)

/**
 * Initialize a pooled channel with non-default destroy delay for testing purpose.
 */
- (nullable instancetype)initWithChannelConfiguration:
                             (GRPCChannelConfiguration *)channelConfiguration
                                         destroyDelay:(NSTimeInterval)destroyDelay;

/**
 * Return the pointer to the raw channel wrapped.
 */
@property(atomic, readonly, nullable) GRPCChannel *wrappedChannel;

@end

/** Test-only interface for \a GRPCChannelPool. */
@interface GRPCChannelPool (Test)

/**
 * Get an instance of pool isolated from the global shared pool with channels' destroy delay being
 * \a destroyDelay.
 */
- (nullable instancetype)initTestPool;

@end

NS_ASSUME_NONNULL_END
