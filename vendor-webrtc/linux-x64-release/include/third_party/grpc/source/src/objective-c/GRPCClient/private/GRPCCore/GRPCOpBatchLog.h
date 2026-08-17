/*
 *
 * Copyright 2016 gRPC authors.
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

#ifdef GRPC_TEST_OBJC
#import <Foundation/Foundation.h>

/**
 * Logs the op batches of a client. Used for testing.
 */
@interface GRPCOpBatchLog : NSObject

/**
 * Enables logging of op batches. Memory consumption increases as more ops are logged.
 */
+ (void)enableOpBatchLog:(BOOL)enabled;

/**
 * Add an op batch to log.
 */
+ (void)addOpBatchToLog:(NSArray *)batch;

/**
 * Obtain the logged op batches. Invoking this method will clean the log.
 */
+ (NSArray *)obtainAndCleanOpBatchLog;

@end

#endif
