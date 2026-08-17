/*
 *
 * Copyright 2019 gRPC authors.
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

// The global interceptor feature is experimental and might be modified or removed at any time.

#import "GRPCCall.h"

@protocol GRPCInterceptorFactory;

/**
 * The interface for gRPC global interceptor.
 *
 * \sa GRPCInterceptor
 */
@interface GRPCCall2 (Interceptor)

/**
 * Register a global interceptor's factory in the current process. Only one interceptor can be
 * registered in a process. If another one attempts to be registered, an exception will be raised.
 *
 * \param[in] interceptorFactory The factory object that generates the global interceptor for each
 * call.
 */
+ (void)registerGlobalInterceptor:(nonnull id<GRPCInterceptorFactory>)interceptorFactory;

/**
 * Get the global interceptor's factory object.
 */
+ (nullable id<GRPCInterceptorFactory>)globalInterceptorFactory;

@end
