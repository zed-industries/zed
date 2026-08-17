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

/**
 * A GRXWriteable is an object to which a sequence of values can be sent. The
 * sequence finishes with an optional error.
 */
@protocol GRXWriteable <NSObject>

/** Push the next value of the sequence to the receiving object. */
- (void)writeValue:(id)value;

/**
 * Signal that the sequence is completed, or that an error occurred. After this
 * message is sent to the instance, neither it nor writeValue: may be
 * called again.
 */
- (void)writesFinishedWithError:(NSError *)errorOrNil;
@end

typedef void (^GRXValueHandler)(id value);
typedef void (^GRXCompletionHandler)(NSError *errorOrNil);
typedef void (^GRXSingleHandler)(id value, NSError *errorOrNil);
typedef void (^GRXEventHandler)(BOOL done, id value, NSError *error);

/**
 * Utility to create objects that conform to the GRXWriteable protocol, from
 * blocks that handle each of the two methods of the protocol.
 */
@interface GRXWriteable : NSObject <GRXWriteable>

+ (instancetype)writeableWithSingleHandler:(GRXSingleHandler)handler;
+ (instancetype)writeableWithEventHandler:(GRXEventHandler)handler;

- (instancetype)initWithValueHandler:(GRXValueHandler)valueHandler
                   completionHandler:(GRXCompletionHandler)completionHandler
    NS_DESIGNATED_INITIALIZER;
@end
