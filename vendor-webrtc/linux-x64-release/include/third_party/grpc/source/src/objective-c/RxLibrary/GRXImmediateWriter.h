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

#import "GRXWriter.h"

/**
 * Utility to construct GRXWriter instances from values that are immediately available when
 * required.
 *
 * Thread-safety:
 *
 * An object of this class shouldn't be messaged concurrently by more than one thread. It will start
 * messaging the writeable before |startWithWriteable:| returns, in the same thread. That is the
 * only place where the writer can be paused or stopped prematurely.
 *
 * If a paused writer of this class is resumed, it will start messaging the writeable, in the same
 * thread, before |setState:| returns. Because the object can't be legally accessed concurrently,
 * that's the only place where it can be paused again (or stopped).
 */
@interface GRXImmediateWriter : GRXWriter

/**
 * Returns a writer that pulls values from the passed NSEnumerator instance and pushes them to
 * its writeable. The NSEnumerator is released when it finishes.
 */
+ (GRXWriter *)writerWithEnumerator:(NSEnumerator *)enumerator;

/**
 * Returns a writer that pushes to its writeable the successive values returned by the passed
 * block. When the block first returns nil, it is released.
 */
+ (GRXWriter *)writerWithValueSupplier:(id (^)(void))block;

/**
 * Returns a writer that iterates over the values of the passed container and pushes them to
 * its writeable. The container is released when the iteration is over.
 *
 * Note that the usual speed gain of NSFastEnumeration over NSEnumerator results from not having to
 * call one method per element. Because GRXWriteable instances accept values one by one, that speed
 * gain doesn't happen here.
 */
+ (GRXWriter *)writerWithContainer:(id<NSFastEnumeration>)container;

/**
 * Returns a writer that sends the passed value to its writeable and then finishes (releasing the
 * value).
 */
+ (GRXWriter *)writerWithValue:(id)value;

/**
 * Returns a writer that, as part of its start method, sends the passed error to the writeable
 * (then releasing the error).
 */
+ (GRXWriter *)writerWithError:(NSError *)error;

/**
 * Returns a writer that, as part of its start method, finishes immediately without sending any
 * values to its writeable.
 */
+ (GRXWriter *)emptyWriter;

@end
