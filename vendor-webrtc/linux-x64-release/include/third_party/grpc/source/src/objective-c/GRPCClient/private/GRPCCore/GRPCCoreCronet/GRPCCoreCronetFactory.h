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

#import "../GRPCCoreFactory.h"

/**
 * The factory for gRPC Core + Cronet transport implementation. The
 * implementation is not part of the default transports of gRPC and is for
 * testing purpose only on Github.
 *
 * To use this transport, a user must include the GRPCCoreCronet module as a
 * dependency of the project and use gGRPCCoreCronetID in call options to
 * specify that this is the transport to be used for a call.
 */
@interface GRPCCoreCronetFactory : NSObject <GRPCCoreTransportFactory>

@end
