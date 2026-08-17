// Copyright 2020 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_TEST_IOS_HOST_SHARED_OBJECT_H_
#define CRASHPAD_TEST_IOS_HOST_SHARED_OBJECT_H_

#import <UIKit/UIKit.h>

@interface CPTestSharedObject : NSObject

// Returns the string "crashpad" for testing EDO.
- (NSString*)testEDO;

// Tell Crashpad to process intermediate dumps.
- (void)processIntermediateDumps;

// Clear pending reports from Crashpad database.
- (void)clearPendingReports;

// Returns the number of pending reports, or -1 if there's an error getting
// report.
- (int)pendingReportCount;

// Returns true if there's a single pending report and sets the exception code
// in the out |exception| parameter. Returns false if there's a different number
// of pending reports.
- (bool)pendingReportException:(NSNumber**)exception;

// Returns true if there's a single pending report and sets the second-level
// exception code in the out |exception_info| parameter. Returns false if
// there's a different number of pending reports.
- (bool)pendingReportExceptionInfo:(NSNumber**)exception_info;

// Return an NSDictionary with a dictionary named "simplemap", an array named
// "vector" an array named "objects", and an array named "ringbuffers",
// representing the combination of all modules AnnotationsSimpleMap,
// AnnotationsVector and AnnotationObjects (String and RingBuffer type)
// respectively.
- (NSDictionary*)getAnnotations;

// Return an NSDictionary with a dictionary representing all key value pairs of
// ExtraMemory MemorySnapshots where the data can be converted to an NSString.
- (NSDictionary*)getExtraMemory;

// Returns YES if a minidump contains the expected custom stream data.
- (BOOL)hasExtensionStream;

// Return an NSDictionary representing the ProcessSnapshotMinidump
// AnnotationsSimpleMap.
- (NSDictionary*)getProcessAnnotations;

// Triggers an EXC_BAD_ACCESS exception and crash.
- (void)crashBadAccess;

// Triggers a crash with a call to kill(SIGABRT). This crash runs with
// ReplaceAllocatorsWithHandlerForbidden.
- (void)crashKillAbort;

// Trigger a crash with a __builtin_trap. This crash runs with
// ReplaceAllocatorsWithHandlerForbidden.
- (void)crashTrap;

// Trigger a crash with an abort(). This crash runs with
// ReplaceAllocatorsWithHandlerForbidden.
- (void)crashAbort;

// Trigger a crash with an uncaught exception.
- (void)crashException;

// Trigger a crash with an uncaught NSException.
- (void)crashNSException;

// Trigger a crash throwing something that isn't an NSException (an NSString).
- (void)crashNotAnNSException;

// Trigger a crash with an uncaught and unhandled NSException.
- (void)crashUnhandledNSException;

// Trigger an unrecognized selector after delay.
- (void)crashUnrecognizedSelectorAfterDelay;

// Trigger a caught NSException, this will not crash
- (void)catchNSException;

// Trigger an NSException with sinkholes in CoreAutoLayout.
- (void)crashCoreAutoLayoutSinkhole;

// Trigger a crash with an infinite recursion.
- (void)crashRecursion;

// Trigger a crash dlsym that contains a crash_info message.
- (void)crashWithCrashInfoMessage;

// Trigger an error that will to the dyld error string `_error_string`
- (void)crashWithDyldErrorString;

// Trigger a crash after writing various annotations.
- (void)crashWithAnnotations;

// Triggers a DumpWithoutCrash |dump_count| times in each of |threads| threads.
- (void)generateDumpWithoutCrash:(int)dump_count threads:(int)threads;

// Triggers a simulataneous Mach exception and signal in different threads.
- (void)crashConcurrentSignalAndMach;

// Triggers simultaneous caught NSExceptions
- (void)catchConcurrentNSException;

// Triggers a SIGABRT signal while handling an NSException to test reentrant
// exceptions.
- (void)crashInHandlerReentrant;

// Runs with ReplaceAllocatorsWithHandlerForbidden and allocates memory, testing
// that the handler forbidden allocator works.
- (void)allocateWithForbiddenAllocators;

// Return the contents of the RawLog output from the previous run of the host
// application.
- (NSString*)rawLogContents;

@end

#endif  // CRASHPAD_TEST_IOS_HOST_SHARED_OBJECT_H_
