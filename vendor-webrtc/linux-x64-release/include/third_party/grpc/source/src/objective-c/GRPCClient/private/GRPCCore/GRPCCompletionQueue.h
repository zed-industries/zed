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

typedef void (^GRPCQueueCompletionHandler)(bool success);

/**
 * This class lets one more easily use |grpc_completion_queue|. To use it, pass the value of the
 * |unmanagedQueue| property of an instance of this class to |grpc_channel_create_call|. Then for
 * every |grpc_call_*| method that accepts a tag, you can pass a block of type
 * |GRPCQueueCompletionHandler| (remembering to cast it using |__bridge_retained|). The block is
 * guaranteed to eventually be called, by a concurrent queue, and then released. Each such block is
 * passed a |bool| that tells if the operation was successful.
 *
 * Release the GRPCCompletionQueue object only after you are not going to pass any more blocks to
 * the |grpc_call| that's using it.
 */
@interface GRPCCompletionQueue : NSObject
@property(nonatomic, readonly) grpc_completion_queue *unmanagedQueue;

+ (instancetype)completionQueue;
@end
