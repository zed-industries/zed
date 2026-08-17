/*
 *
 * Copyright 2016 gRPC authors.
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

/**
 * "X-macro" file that lists the flags names of Apple's Network Reachability
API, along with a nice
 * Objective-C method name used to query each of them.
 *
 * Example usage: To generate a dictionary from flag value to name, one can do:

  NSDictionary *flagNames = @{
#define GRPC_XMACRO_ITEM(methodName, FlagName) \
    @(kSCNetworkReachabilityFlags ## FlagName): @#methodName,
#include "GRXReachabilityFlagNames.xmacro.h"
#undef GRPC_XMACRO_ITEM
  };

  XCTAssertEqualObjects(flagNames[@(kSCNetworkReachabilityFlagsIsWWAN)],
@"isCell");

 */

#ifndef GRPC_XMACRO_ITEM
#error This file is to be used with the "X-macro" pattern: Please #define \
       GRPC_XMACRO_ITEM(methodName, FlagName), then #include this file, and then #undef \
       GRPC_XMACRO_ITEM.
#endif

#if TARGET_OS_IPHONE
GRPC_XMACRO_ITEM(isWWAN, IsWWAN)
#endif
GRPC_XMACRO_ITEM(reachable, Reachable)
GRPC_XMACRO_ITEM(transientConnection, TransientConnection)
GRPC_XMACRO_ITEM(connectionRequired, ConnectionRequired)
GRPC_XMACRO_ITEM(connectionOnTraffic, ConnectionOnTraffic)
GRPC_XMACRO_ITEM(interventionRequired, InterventionRequired)
GRPC_XMACRO_ITEM(connectionOnDemand, ConnectionOnDemand)
GRPC_XMACRO_ITEM(isLocalAddress, IsLocalAddress)
GRPC_XMACRO_ITEM(isDirect, IsDirect)
