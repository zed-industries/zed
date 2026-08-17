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

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>

NS_ASSUME_NONNULL_BEGIN

/* Default test timeout in seconds for interopt test. */
FOUNDATION_EXPORT const NSTimeInterval GRPCInteropTestTimeoutDefault;

// Block typedef for waiting for a target group of expectations via XCTWaiter.
typedef void (^GRPCTestWaiter)(NSArray<XCTestExpectation *> *expectations, NSTimeInterval timeout);

// Block typedef for asserting a given expression value with optional retry.
typedef void (^GRPCTestAssert)(BOOL expressionValue, NSString *message);

// Block typedef for a test run. Test run should call waiter to wait for a group of expectations
// with timeout. Test run can also optionally invoke assertBlock to report assertion failure.
// Failed assertion will be retried up to maximum retry.
typedef void (^GRPCTestRunBlock)(GRPCTestWaiter waiterBlock, GRPCTestAssert assertBlock);

/**
 * Common utility to fetch plain text local interop server address.
 *
 * @return Interop test server address including host and port.
 */
FOUNDATION_EXPORT NSString *GRPCGetLocalInteropTestServerAddressPlainText(void);

/**
 * Common utility to fetch ssl local interop server address.
 *
 * @return Interop test server address including host and port.
 */
FOUNDATION_EXPORT NSString *GRPCGetLocalInteropTestServerAddressSSL(void);

/**
 * Common utility to fetch remote interop test server address.
 *
 * @return Interop test server address including host and port.
 */
FOUNDATION_EXPORT NSString *GRPCGetRemoteInteropTestServerAddress(void);

/**
 * Common utility to print interop server address information to console via NSLog.
 */
FOUNDATION_EXPORT void GRPCPrintInteropTestServerDebugInfo(void);

/**
 * Common utility to run a test block until success, up to predefined number of repeats.
 * @param testCase Associated test case run for reporting test failures.
 * @param testBlock Target test block to be invoked by the utility function. The block will be
 * invoked synchronously before the function returns.
 * @return YES if test run succeeded within the repeat limit. NO otherwise.
 */
FOUNDATION_EXPORT BOOL GRPCTestRunWithFlakeRepeats(XCTestCase *testCase,
                                                   GRPCTestRunBlock testBlock);

/**
 * Common utility to reset gRPC call's active connections.
 */
FOUNDATION_EXPORT void GRPCResetCallConnections(void);

NS_ASSUME_NONNULL_END
