/*
 *
 * Copyright 2015 gRPC authors.
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

#import <Foundation/Foundation.h>
#include <grpc/grpc.h>

@interface NSError (GRPC)
/**
 * Returns nil if the status code is OK. Otherwise, a NSError whose code is one of |GRPCErrorCode|
 * and whose domain is |kGRPCErrorDomain|.
 */
+ (instancetype)grpc_errorFromStatusCode:(grpc_status_code)statusCode
                                 details:(const char *)details
                             errorString:(const char *)errorString;
@end
