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

#import "GRXForwardingWriter.h"

/** A "proxy" writer that transforms all the values of its input writer by using a mapping function.
 */
@interface GRXMappingWriter : GRXForwardingWriter
- (instancetype)initWithWriter:(GRXWriter *)writer
                           map:(id (^)(id value))map NS_DESIGNATED_INITIALIZER;
@end
