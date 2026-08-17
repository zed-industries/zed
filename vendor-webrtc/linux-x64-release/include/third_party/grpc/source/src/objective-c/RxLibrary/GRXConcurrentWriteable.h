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

#import "GRXWriteable.h"
#import "GRXWriter.h"

/**
 * This is a thread-safe wrapper over a GRXWriteable instance. It lets one enqueue calls to a
 * GRXWriteable instance for the thread user provided, guaranteeing that writesFinishedWithError: is
 * the last message sent to it (no matter what messages are sent to the wrapper, in what order, nor
 * from which thread). It also guarantees that, if cancelWithError: is called (e.g. by the app
 * cancelling the writes), no further messages are sent to the writeable except
 * writesFinishedWithError:.
 *
 * TODO(jcanizales): Let the user specify another queue for the writeable callbacks.
 */
@interface GRXConcurrentWriteable : NSObject

/**
 * The GRXWriteable passed is the wrapped writeable.
 * The GRXWriteable instance is retained until writesFinishedWithError: is sent to it, and released
 * after that.
 */
- (instancetype)initWithWriteable:(id<GRXWriteable>)writeable
                    dispatchQueue:(dispatch_queue_t)queue NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithWriteable:(id<GRXWriteable>)writeable;

/**
 * Enqueues writeValue: to be sent to the writeable from the designated dispatch queue.
 * The passed handler is invoked from designated dispatch queue after writeValue: returns.
 */
- (void)enqueueValue:(id)value completionHandler:(void (^)(void))handler;

/**
 * Enqueues writesFinishedWithError:nil to be sent to the writeable in the designated dispatch
 * queue. After that message is sent to the writeable, all other methods of this object are
 * effectively noops.
 */
- (void)enqueueSuccessfulCompletion;

/**
 * If the writeable has not yet received a writesFinishedWithError: message, this will enqueue one
 * to be sent to it in the designated dispatch queue, and cancel all other pending messages to the
 * writeable enqueued by this object (both past and future).
 * The error argument cannot be nil.
 */
- (void)cancelWithError:(NSError *)error;

/**
 * Cancels all pending messages to the writeable enqueued by this object (both past and future).
 * Because the writeable won't receive writesFinishedWithError:, this also releases the writeable.
 */
- (void)cancelSilently;
@end
