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

#import "GRXWriter.h"

@interface GRXWriter (Immediate)

/**
 * Returns a writer that pulls values from the passed NSEnumerator instance and pushes them to
 * its writeable. The NSEnumerator is released when it finishes.
 */
+ (instancetype)writerWithEnumerator:(NSEnumerator *)enumerator;

/**
 * Returns a writer that pushes to its writeable the successive values returned by the passed
 * block. When the block first returns nil, it is released.
 */
+ (instancetype)writerWithValueSupplier:(id (^)(void))block;

/**
 * Returns a writer that iterates over the values of the passed container and pushes them to
 * its writeable. The container is released when the iteration is over.
 *
 * Note that the usual speed gain of NSFastEnumeration over NSEnumerator results from not having to
 * call one method per element. Because GRXWriteable instances accept values one by one, that speed
 * gain doesn't happen here.
 */
+ (instancetype)writerWithContainer:(id<NSFastEnumeration>)container;

/**
 * Returns a writer that sends the passed value to its writeable and then finishes (releasing the
 * value).
 */
+ (instancetype)writerWithValue:(id)value;

/**
 * Returns a writer that, as part of its start method, sends the passed error to the writeable
 * (then releasing the error).
 */
+ (instancetype)writerWithError:(NSError *)error;

/**
 * Returns a writer that, as part of its start method, finishes immediately without sending any
 * values to its writeable.
 */
+ (instancetype)emptyWriter;

@end
