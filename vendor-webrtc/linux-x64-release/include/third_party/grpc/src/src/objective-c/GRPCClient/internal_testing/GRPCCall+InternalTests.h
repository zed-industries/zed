/*
 *
 * Copyright 2017 gRPC authors.
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

#import "../GRPCCall.h"

/**
 * Methods used for gRPC internal tests. DO NOT USE.
 */
@interface GRPCCall (InternalTests)

/**
 * Enables logging of op batches. Memory consumption increases as more ops are logged.
 *
 * This function is for internal testing of gRPC only. It is not part of gRPC's public interface.
 * Do not use in production. To enable, set the preprocessor flag GRPC_TEST_OBJC.
 */
+ (void)enableOpBatchLog:(BOOL)enabled;

/**
 * Obtain the logged op batches. Invoking this method will clean the log.
 *
 * This function is for internal testing of gRPC only. It is not part of gRPC's public interface.
 * Do not use in production. To enable, set the preprocessor flag GRPC_TEST_OBJC.
 */
+ (NSArray *)obtainAndCleanOpBatchLog;

@end

#endif
