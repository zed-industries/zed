//
//  GTMSenTestCase.h
//
//  Copyright 2007-2008 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License"); you may not
//  use this file except in compliance with the License.  You may obtain a copy
//  of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
//  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
//  License for the specific language governing permissions and limitations under
//  the License.
//

// Portions of this file fall under the following license, marked with
// SENTE_BEGIN - SENTE_END
//
// Copyright (c) 1997-2005, Sen:te (Sente SA).  All rights reserved.
//
// Use of this source code is governed by the following license:
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// (1) Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// (2) Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS''
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL Sente SA OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
// OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
// EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Note: this license is equivalent to the FreeBSD license.
//
// This notice may not be removed from this file.

// Some extra test case macros that would have been convenient for SenTestingKit
// to provide. I didn't stick GTM in front of the Macro names, so that they would
// be easy to remember.

#import "GTMDefines.h"

#if (!GTM_IPHONE_SDK) || (GTM_IPHONE_USE_SENTEST)
#import <SenTestingKit/SenTestingKit.h>
#else
#import <Foundation/Foundation.h>
#ifdef __cplusplus
extern "C" {
#endif
  
#if defined __clang__
// gcc and gcc-llvm do not allow you to use STAssert(blah, nil) with nil
// as a description if you have the NS_FORMAT_FUNCTION on.
// clang however will not compile without warnings if you don't have it.
NSString *STComposeString(NSString *, ...) NS_FORMAT_FUNCTION(1, 2);
#else
NSString *STComposeString(NSString *, ...);
#endif  // __clang__
  
#ifdef __cplusplus
}
#endif

#endif  // !GTM_IPHONE_SDK || GTM_IPHONE_USE_SENTEST

// Generates a failure when a1 != noErr
//  Args:
//    a1: should be either an OSErr or an OSStatus
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNoErr(a1, description, ...) \
do { \
  @try { \
    OSStatus a1value = (a1); \
    if (a1value != noErr) { \
      NSString *_expression = [NSString stringWithFormat:@"Expected noErr, got %ld for (%s)", (long)a1value, #a1]; \
      [self failWithException:([NSException failureInCondition:_expression \
                       isTrue:NO \
                       inFile:[NSString stringWithUTF8String:__FILE__] \
                       atLine:__LINE__ \
              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == noErr fails", #a1] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when a1 != a2
//  Args:
//    a1: received value. Should be either an OSErr or an OSStatus
//    a2: expected value. Should be either an OSErr or an OSStatus
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertErr(a1, a2, description, ...) \
do { \
  @try { \
    OSStatus a1value = (a1); \
    OSStatus a2value = (a2); \
    if (a1value != a2value) { \
      NSString *_expression = [NSString stringWithFormat:@"Expected %s(%ld) but got %ld for (%s)", #a2, (long)a2value, (long)a1value, #a1]; \
      [self failWithException:([NSException failureInCondition:_expression \
                       isTrue:NO \
                       inFile:[NSString stringWithUTF8String:__FILE__] \
                       atLine:__LINE__ \
              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s) fails", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)


// Generates a failure when a1 is NULL
//  Args:
//    a1: should be a pointer (use STAssertNotNil for an object)
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNotNULL(a1, description, ...) \
do { \
  @try { \
    __typeof__(a1) a1value = (a1); \
    if (a1value == (__typeof__(a1))NULL) { \
      NSString *_expression = [NSString stringWithFormat:@"((%s) != NULL)", #a1]; \
      [self failWithException:([NSException failureInCondition:_expression \
                       isTrue:NO \
                       inFile:[NSString stringWithUTF8String:__FILE__] \
                       atLine:__LINE__ \
              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) != NULL fails", #a1] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when a1 is not NULL
//  Args:
//    a1: should be a pointer (use STAssertNil for an object)
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNULL(a1, description, ...) \
do { \
  @try { \
    __typeof__(a1) a1value = (a1); \
    if (a1value != (__typeof__(a1))NULL) { \
      NSString *_expression = [NSString stringWithFormat:@"((%s) == NULL)", #a1]; \
      [self failWithException:([NSException failureInCondition:_expression \
                                                        isTrue:NO \
                                                        inFile:[NSString stringWithUTF8String:__FILE__] \
                                                        atLine:__LINE__ \
                                               withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == NULL fails", #a1] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when a1 is equal to a2. This test is for C scalars,
// structs and unions.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNotEquals(a1, a2, description, ...) \
do { \
  @try { \
    if (strcmp(@encode(__typeof__(a1)), @encode(__typeof__(a2)))) { \
      [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                  atLine:__LINE__ \
                                         withDescription:@"Type mismatch -- %@", STComposeString(description, ##__VA_ARGS__)]]; \
    } else { \
      __typeof__(a1) a1value = (a1); \
      __typeof__(a2) a2value = (a2); \
      NSValue *a1encoded = [NSValue value:&a1value withObjCType:@encode(__typeof__(a1))]; \
      NSValue *a2encoded = [NSValue value:&a2value withObjCType:@encode(__typeof__(a2))]; \
      if ([a1encoded isEqualToValue:a2encoded]) { \
        NSString *_expression = [NSString stringWithFormat:@"((%s) != (%s))", #a1, #a2]; \
        [self failWithException:([NSException failureInCondition:_expression \
                         isTrue:NO \
                         inFile:[NSString stringWithUTF8String:__FILE__] \
                         atLine:__LINE__ \
                withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
      }\
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) != (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
            withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when a1 is equal to a2. This test is for objects.
//  Args:
//    a1: argument 1. object.
//    a2: argument 2. object.
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNotEqualObjects(a1, a2, description, ...) \
do { \
  @try {\
    id a1value = (a1); \
    id a2value = (a2); \
    if ( (strcmp(@encode(__typeof__(a1value)), @encode(id)) == 0) && \
         (strcmp(@encode(__typeof__(a2value)), @encode(id)) == 0) && \
         (![(id)a1value isEqual:(id)a2value]) ) continue; \
    [self failWithException:([NSException failureInEqualityBetweenObject:a1value \
                  andObject:a2value \
                     inFile:[NSString stringWithUTF8String:__FILE__] \
                     atLine:__LINE__ \
            withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
  }\
  @catch (id anException) {\
    [self failWithException:([NSException failureInRaise:[NSString stringWithFormat:@"(%s) != (%s)", #a1, #a2] \
                                               exception:anException \
                                                  inFile:[NSString stringWithUTF8String:__FILE__] \
                                                  atLine:__LINE__ \
                                         withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
  }\
} while(0)

// Generates a failure when a1 is not 'op' to a2. This test is for C scalars.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    op: operation
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertOperation(a1, a2, op, description, ...) \
do { \
  @try { \
    if (strcmp(@encode(__typeof__(a1)), @encode(__typeof__(a2)))) { \
      [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                  atLine:__LINE__ \
                                         withDescription:@"Type mismatch -- %@", STComposeString(description, ##__VA_ARGS__)]]; \
    } else { \
      __typeof__(a1) a1value = (a1); \
      __typeof__(a2) a2value = (a2); \
      if (!(a1value op a2value)) { \
        double a1DoubleValue = a1value; \
        double a2DoubleValue = a2value; \
        NSString *_expression = [NSString stringWithFormat:@"(%s (%lg) %s %s (%lg))", #a1, a1DoubleValue, #op, #a2, a2DoubleValue]; \
        [self failWithException:([NSException failureInCondition:_expression \
                         isTrue:NO \
                         inFile:[NSString stringWithUTF8String:__FILE__] \
                         atLine:__LINE__ \
                withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)])]; \
      } \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException \
             failureInRaise:[NSString stringWithFormat:@"(%s) %s (%s)", #a1, #op, #a2] \
                  exception:anException \
                     inFile:[NSString stringWithUTF8String:__FILE__] \
                     atLine:__LINE__ \
            withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when a1 is not > a2. This test is for C scalars.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    op: operation
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertGreaterThan(a1, a2, description, ...) \
  STAssertOperation(a1, a2, >, description, ##__VA_ARGS__)

// Generates a failure when a1 is not >= a2. This test is for C scalars.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    op: operation
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertGreaterThanOrEqual(a1, a2, description, ...) \
  STAssertOperation(a1, a2, >=, description, ##__VA_ARGS__)

// Generates a failure when a1 is not < a2. This test is for C scalars.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    op: operation
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertLessThan(a1, a2, description, ...) \
  STAssertOperation(a1, a2, <, description, ##__VA_ARGS__)

// Generates a failure when a1 is not <= a2. This test is for C scalars.
//  Args:
//    a1: argument 1
//    a2: argument 2
//    op: operation
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertLessThanOrEqual(a1, a2, description, ...) \
  STAssertOperation(a1, a2, <=, description, ##__VA_ARGS__)

// Generates a failure when string a1 is not equal to string a2. This call
// differs from STAssertEqualObjects in that strings that are different in
// composition (precomposed vs decomposed) will compare equal if their final
// representation is equal.
// ex O + umlaut decomposed is the same as O + umlaut composed.
//  Args:
//    a1: string 1
//    a2: string 2
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertEqualStrings(a1, a2, description, ...) \
do { \
  @try { \
    id a1value = (a1); \
    id a2value = (a2); \
    if (a1value == a2value) continue; \
    if ([a1value isKindOfClass:[NSString class]] && \
        [a2value isKindOfClass:[NSString class]] && \
        [a1value compare:a2value options:0] == NSOrderedSame) continue; \
     [self failWithException:[NSException failureInEqualityBetweenObject:a1value \
                                                               andObject:a2value \
                                                                  inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                  atLine:__LINE__ \
                                                         withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when string a1 is equal to string a2. This call
// differs from STAssertEqualObjects in that strings that are different in
// composition (precomposed vs decomposed) will compare equal if their final
// representation is equal.
// ex O + umlaut decomposed is the same as O + umlaut composed.
//  Args:
//    a1: string 1
//    a2: string 2
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNotEqualStrings(a1, a2, description, ...) \
do { \
  @try { \
    id a1value = (a1); \
    id a2value = (a2); \
    if ([a1value isKindOfClass:[NSString class]] && \
        [a2value isKindOfClass:[NSString class]] && \
        [a1value compare:a2value options:0] != NSOrderedSame) continue; \
     [self failWithException:[NSException failureInEqualityBetweenObject:a1value \
                                                               andObject:a2value \
                                                                  inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                  atLine:__LINE__ \
                                                         withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) != (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when c-string a1 is not equal to c-string a2.
//  Args:
//    a1: string 1
//    a2: string 2
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertEqualCStrings(a1, a2, description, ...) \
do { \
  @try { \
    const char* a1value = (a1); \
    const char* a2value = (a2); \
    if (a1value == a2value) continue; \
    if (strcmp(a1value, a2value) == 0) continue; \
    [self failWithException:[NSException failureInEqualityBetweenObject:[NSString stringWithUTF8String:a1value] \
                                                              andObject:[NSString stringWithUTF8String:a2value] \
                                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                 atLine:__LINE__ \
                                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

// Generates a failure when c-string a1 is equal to c-string a2.
//  Args:
//    a1: string 1
//    a2: string 2
//    description: A format string as in the printf() function. Can be nil or
//                 an empty string but must be present.
//    ...: A variable number of arguments to the format string. Can be absent.
#define STAssertNotEqualCStrings(a1, a2, description, ...) \
do { \
  @try { \
    const char* a1value = (a1); \
    const char* a2value = (a2); \
    if (strcmp(a1value, a2value) != 0) continue; \
    [self failWithException:[NSException failureInEqualityBetweenObject:[NSString stringWithUTF8String:a1value] \
                                                              andObject:[NSString stringWithUTF8String:a2value] \
                                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                 atLine:__LINE__ \
                                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) != (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

/*" Generates a failure when a1 is not equal to a2 within + or - accuracy is false.
  This test is for GLKit types (GLKVector, GLKMatrix) where small differences
  could  make these items not exactly equal. Do not use this version directly.
  Use the explicit STAssertEqualGLKVectors and STAssertEqualGLKMatrices defined
  below.
  _{a1    The GLKType on the left.}
  _{a2    The GLKType on the right.}
  _{accuracy  The maximum difference between a1 and a2 for these values to be
  considered equal.}
  _{description A format string as in the printf() function. Can be nil or
                      an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/

#define STInternalAssertEqualGLKVectorsOrMatrices(a1, a2, accuracy, description, ...) \
do { \
  @try { \
    if (strcmp(@encode(__typeof__(a1)), @encode(__typeof__(a2)))) { \
      [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                                                 atLine:__LINE__ \
                                                                        withDescription:@"Type mismatch -- %@", STComposeString(description, ##__VA_ARGS__)]]; \
    } else { \
      __typeof__(a1) a1GLKValue = (a1); \
      __typeof__(a2) a2GLKValue = (a2); \
      __typeof__(accuracy) accuracyvalue = (accuracy); \
      float *a1FloatValue = ((float*)&a1GLKValue); \
      float *a2FloatValue = ((float*)&a2GLKValue); \
      for (size_t i = 0; i < sizeof(__typeof__(a1)) / sizeof(float); ++i) { \
        float a1value = a1FloatValue[i]; \
        float a2value = a2FloatValue[i]; \
        if (STAbsoluteDifference(a1value, a2value) > accuracyvalue) { \
          NSMutableArray *strings = [NSMutableArray arrayWithCapacity:sizeof(a1) / sizeof(float)]; \
          NSString *string; \
          for (size_t j = 0; j < sizeof(__typeof__(a1)) / sizeof(float); ++j) { \
            string = [NSString stringWithFormat:@"(%0.3f == %0.3f)", a1FloatValue[j], a2FloatValue[j]]; \
            [strings addObject:string]; \
          } \
          string = [strings componentsJoinedByString:@", "]; \
          NSString *desc = STComposeString(description, ##__VA_ARGS__); \
          desc = [NSString stringWithFormat:@"%@ With Accuracy %0.3f: %@", string, (float)accuracyvalue, desc]; \
          [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                      atLine:__LINE__ \
                                             withDescription:@"%@", desc]]; \
          break; \
        } \
      } \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

#define STAssertEqualGLKVectors(a1, a2, accuracy, description, ...) \
     STInternalAssertEqualGLKVectorsOrMatrices(a1, a2, accuracy, description, ##__VA_ARGS__)

#define STAssertEqualGLKMatrices(a1, a2, accuracy, description, ...) \
     STInternalAssertEqualGLKVectorsOrMatrices(a1, a2, accuracy, description, ##__VA_ARGS__)

#define STAssertEqualGLKQuaternions(a1, a2, accuracy, description, ...) \
     STInternalAssertEqualGLKVectorsOrMatrices(a1, a2, accuracy, description, ##__VA_ARGS__)

#if GTM_IPHONE_SDK && !GTM_IPHONE_USE_SENTEST
// When not using the Xcode provided version, define everything ourselves.

// SENTE_BEGIN
/*" Generates a failure when !{ [a1 isEqualTo:a2] } is false
	(or one is nil and the other is not).
	_{a1    The object on the left.}
	_{a2    The object on the right.}
	_{description A format string as in the printf() function. Can be nil or
		an empty string but must be present.}
	_{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertEqualObjects(a1, a2, description, ...) \
do { \
  @try { \
    id a1value = (a1); \
    id a2value = (a2); \
    if (a1value == a2value) continue; \
    if ((strcmp(@encode(__typeof__(a1value)), @encode(id)) == 0) && \
        (strcmp(@encode(__typeof__(a2value)), @encode(id)) == 0) && \
        [(id)a1value isEqual:(id)a2value]) continue; \
    [self failWithException:[NSException failureInEqualityBetweenObject:a1value \
                                                              andObject:a2value \
                                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                 atLine:__LINE__ \
                                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)


/*" Generates a failure when a1 is not equal to a2. This test is for
    C scalars, structs and unions.
    _{a1    The argument on the left.}
    _{a2    The argument on the right.}
    _{description A format string as in the printf() function. Can be nil or
                        an empty string but must be present.}
    _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertEquals(a1, a2, description, ...) \
do { \
  @try { \
    if (strcmp(@encode(__typeof__(a1)), @encode(__typeof__(a2)))) { \
      [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                                                 atLine:__LINE__ \
                                                                        withDescription:@"Type mismatch -- %@", STComposeString(description, ##__VA_ARGS__)]]; \
    } else { \
      __typeof__(a1) a1value = (a1); \
      __typeof__(a2) a2value = (a2); \
      NSValue *a1encoded = [NSValue value:&a1value withObjCType:@encode(__typeof__(a1))]; \
      NSValue *a2encoded = [NSValue value:&a2value withObjCType:@encode(__typeof__(a2))]; \
      if (![a1encoded isEqualToValue:a2encoded]) { \
        [self failWithException:[NSException failureInEqualityBetweenValue:a1encoded \
                                                                  andValue:a2encoded \
                                                              withAccuracy:nil \
                                                                    inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                    atLine:__LINE__ \
                                                           withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
      } \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)

#define STAbsoluteDifference(left,right) (MAX(left,right)-MIN(left,right))


/*" Generates a failure when a1 is not equal to a2 within + or - accuracy is false.
  This test is for scalars such as floats and doubles where small differences
  could make these items not exactly equal, but also works for all scalars.
  _{a1    The scalar on the left.}
  _{a2    The scalar on the right.}
  _{accuracy  The maximum difference between a1 and a2 for these values to be
  considered equal.}
  _{description A format string as in the printf() function. Can be nil or
                      an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/

#define STAssertEqualsWithAccuracy(a1, a2, accuracy, description, ...) \
do { \
  @try { \
    if (strcmp(@encode(__typeof__(a1)), @encode(__typeof__(a2)))) { \
      [self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                                                                 atLine:__LINE__ \
                                                                        withDescription:@"Type mismatch -- %@", STComposeString(description, ##__VA_ARGS__)]]; \
    } else { \
      __typeof__(a1) a1value = (a1); \
      __typeof__(a2) a2value = (a2); \
      __typeof__(accuracy) accuracyvalue = (accuracy); \
      if (STAbsoluteDifference(a1value, a2value) > accuracyvalue) { \
              NSValue *a1encoded = [NSValue value:&a1value withObjCType:@encode(__typeof__(a1))]; \
              NSValue *a2encoded = [NSValue value:&a2value withObjCType:@encode(__typeof__(a2))]; \
              NSValue *accuracyencoded = [NSValue value:&accuracyvalue withObjCType:@encode(__typeof__(accuracy))]; \
              [self failWithException:[NSException failureInEqualityBetweenValue:a1encoded \
                                                                        andValue:a2encoded \
                                                                    withAccuracy:accuracyencoded \
                                                                          inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                          atLine:__LINE__ \
                                                                 withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
      } \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == (%s)", #a1, #a2] \
                                                                         exception:anException \
                                                                            inFile:[NSString stringWithUTF8String:__FILE__] \
                                                                            atLine:__LINE__ \
                                                                   withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)



/*" Generates a failure unconditionally.
  _{description A format string as in the printf() function. Can be nil or
                      an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STFail(description, ...) \
[self failWithException:[NSException failureInFile:[NSString stringWithUTF8String:__FILE__] \
                                            atLine:__LINE__ \
                                   withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]



/*" Generates a failure when a1 is not nil.
  _{a1    An object.}
  _{description A format string as in the printf() function. Can be nil or
    an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertNil(a1, description, ...) \
do { \
  @try { \
    id a1value = (a1); \
    if (a1value != nil) { \
      NSString *_a1 = [NSString stringWithUTF8String:#a1]; \
      NSString *_expression = [NSString stringWithFormat:@"((%@) == nil)", _a1]; \
      [self failWithException:[NSException failureInCondition:_expression \
                                                       isTrue:NO \
                                                       inFile:[NSString stringWithUTF8String:__FILE__] \
                                                       atLine:__LINE__ \
                                              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) == nil fails", #a1] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)


/*" Generates a failure when a1 is nil.
  _{a1    An object.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertNotNil(a1, description, ...) \
do { \
  @try { \
    id a1value = (a1); \
    if (a1value == nil) { \
      NSString *_a1 = [NSString stringWithUTF8String:#a1]; \
      NSString *_expression = [NSString stringWithFormat:@"((%@) != nil)", _a1]; \
      [self failWithException:[NSException failureInCondition:_expression \
                                                       isTrue:NO \
                                                       inFile:[NSString stringWithUTF8String:__FILE__] \
                                                       atLine:__LINE__ \
                                              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) != nil fails", #a1] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while(0)


/*" Generates a failure when expression evaluates to false.
  _{expr    The expression that is tested.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertTrue(expr, description, ...) \
do { \
  BOOL _evaluatedExpression = (expr); \
  if (!_evaluatedExpression) { \
    NSString *_expression = [NSString stringWithUTF8String:#expr]; \
    [self failWithException:[NSException failureInCondition:_expression \
                                                     isTrue:NO \
                                                     inFile:[NSString stringWithUTF8String:__FILE__] \
                                                     atLine:__LINE__ \
                                            withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while (0)


/*" Generates a failure when expression evaluates to false and in addition will
  generate error messages if an exception is encountered.
  _{expr    The expression that is tested.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertTrueNoThrow(expr, description, ...) \
do { \
  @try { \
    BOOL _evaluatedExpression = (expr); \
    if (!_evaluatedExpression) { \
      NSString *_expression = [NSString stringWithUTF8String:#expr]; \
      [self failWithException:[NSException failureInCondition:_expression \
                                                       isTrue:NO \
                                                       inFile:[NSString stringWithUTF8String:__FILE__] \
                                                       atLine:__LINE__ \
                                              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"(%s) ", #expr] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while (0)


/*" Generates a failure when the expression evaluates to true.
  _{expr    The expression that is tested.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertFalse(expr, description, ...) \
do { \
  BOOL _evaluatedExpression = (expr); \
  if (_evaluatedExpression) { \
    NSString *_expression = [NSString stringWithUTF8String:#expr]; \
    [self failWithException:[NSException failureInCondition:_expression \
                                                     isTrue:YES \
                                                     inFile:[NSString stringWithUTF8String:__FILE__] \
                                                     atLine:__LINE__ \
                                            withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while (0)


/*" Generates a failure when the expression evaluates to true and in addition
  will generate error messages if an exception is encountered.
  _{expr    The expression that is tested.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertFalseNoThrow(expr, description, ...) \
do { \
  @try { \
    BOOL _evaluatedExpression = (expr); \
    if (_evaluatedExpression) { \
      NSString *_expression = [NSString stringWithUTF8String:#expr]; \
      [self failWithException:[NSException failureInCondition:_expression \
                                                       isTrue:YES \
                                                       inFile:[NSString stringWithUTF8String:__FILE__] \
                                                       atLine:__LINE__ \
                                              withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
    } \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithFormat:@"!(%s) ", #expr] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while (0)


/*" Generates a failure when expression does not throw an exception.
  _{expression    The expression that is evaluated.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.
"*/
#define STAssertThrows(expr, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (id anException) { \
    continue; \
  } \
  [self failWithException:[NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                                            exception:nil \
                                               inFile:[NSString stringWithUTF8String:__FILE__] \
                                               atLine:__LINE__ \
                                      withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
} while (0)


/*" Generates a failure when expression does not throw an exception of a
  specific class.
  _{expression    The expression that is evaluated.}
  _{specificException    The specified class of the exception.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertThrowsSpecific(expr, specificException, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (specificException *anException) { \
    continue; \
  } \
  @catch (id anException) { \
    NSString *_descrip = STComposeString(@"(Expected exception: %@) %@", NSStringFromClass([specificException class]), description); \
    [self failWithException:[NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
                                            continue; \
  } \
  NSString *_descrip = STComposeString(@"(Expected exception: %@) %@", NSStringFromClass([specificException class]), description); \
  [self failWithException:[NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                                            exception:nil \
                                               inFile:[NSString stringWithUTF8String:__FILE__] \
                                               atLine:__LINE__ \
                                      withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
} while (0)


/*" Generates a failure when expression does not throw an exception of a
  specific class with a specific name.  Useful for those frameworks like
  AppKit or Foundation that throw generic NSException w/specific names
  (NSInvalidArgumentException, etc).
  _{expression    The expression that is evaluated.}
  _{specificException    The specified class of the exception.}
  _{aName    The name of the specified exception.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}

"*/
#define STAssertThrowsSpecificNamed(expr, specificException, aName, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (specificException *anException) { \
    if ([aName isEqualToString:[anException name]]) continue; \
    NSString *_descrip = STComposeString(@"(Expected exception: %@ (name: %@)) %@", NSStringFromClass([specificException class]), aName, description); \
    [self failWithException: \
      [NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                        exception:anException \
                           inFile:[NSString stringWithUTF8String:__FILE__] \
                           atLine:__LINE__ \
                  withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
    continue; \
  } \
  @catch (id anException) { \
    NSString *_descrip = STComposeString(@"(Expected exception: %@) %@", NSStringFromClass([specificException class]), description); \
    [self failWithException: \
      [NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                        exception:anException \
                           inFile:[NSString stringWithUTF8String:__FILE__] \
                           atLine:__LINE__ \
                  withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
    continue; \
  } \
  NSString *_descrip = STComposeString(@"(Expected exception: %@) %@", NSStringFromClass([specificException class]), description); \
  [self failWithException: \
    [NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                      exception:nil \
                         inFile:[NSString stringWithUTF8String:__FILE__] \
                         atLine:__LINE__ \
                withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
} while (0)


/*" Generates a failure when expression does throw an exception.
  _{expression    The expression that is evaluated.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertNoThrow(expr, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (id anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
} while (0)


/*" Generates a failure when expression does throw an exception of the specitied
  class. Any other exception is okay (i.e. does not generate a failure).
  _{expression    The expression that is evaluated.}
  _{specificException    The specified class of the exception.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}
"*/
#define STAssertNoThrowSpecific(expr, specificException, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (specificException *anException) { \
    [self failWithException:[NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                                              exception:anException \
                                                 inFile:[NSString stringWithUTF8String:__FILE__] \
                                                 atLine:__LINE__ \
                                        withDescription:@"%@", STComposeString(description, ##__VA_ARGS__)]]; \
  } \
  @catch (id anythingElse) { \
    ; \
  } \
} while (0)


/*" Generates a failure when expression does throw an exception of a
  specific class with a specific name.  Useful for those frameworks like
  AppKit or Foundation that throw generic NSException w/specific names
  (NSInvalidArgumentException, etc).
  _{expression    The expression that is evaluated.}
  _{specificException    The specified class of the exception.}
  _{aName    The name of the specified exception.}
  _{description A format string as in the printf() function. Can be nil or
  an empty string but must be present.}
  _{... A variable number of arguments to the format string. Can be absent.}

"*/
#define STAssertNoThrowSpecificNamed(expr, specificException, aName, description, ...) \
do { \
  @try { \
    (expr); \
  } \
  @catch (specificException *anException) { \
    if ([aName isEqualToString:[anException name]]) { \
      NSString *_descrip = STComposeString(@"(Expected exception: %@ (name: %@)) %@", NSStringFromClass([specificException class]), aName, description); \
      [self failWithException: \
        [NSException failureInRaise:[NSString stringWithUTF8String:#expr] \
                          exception:anException \
                             inFile:[NSString stringWithUTF8String:__FILE__] \
                             atLine:__LINE__ \
                    withDescription:@"%@", STComposeString(_descrip, ##__VA_ARGS__)]]; \
    } \
    continue; \
  } \
  @catch (id anythingElse) { \
    ; \
  } \
} while (0)



@interface NSException (GTMSenTestAdditions)
+ (NSException *)failureInFile:(NSString *)filename
                        atLine:(int)lineNumber
               withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(3, 4);
+ (NSException *)failureInCondition:(NSString *)condition
                             isTrue:(BOOL)isTrue
                             inFile:(NSString *)filename
                             atLine:(int)lineNumber
                    withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(5, 6);
+ (NSException *)failureInEqualityBetweenObject:(id)left
                                      andObject:(id)right
                                         inFile:(NSString *)filename
                                         atLine:(int)lineNumber
                                withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(5, 6);
+ (NSException *)failureInEqualityBetweenValue:(NSValue *)left
                                      andValue:(NSValue *)right
                                  withAccuracy:(NSValue *)accuracy
                                        inFile:(NSString *)filename
                                        atLine:(int) ineNumber
                               withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(6, 7);
+ (NSException *)failureInRaise:(NSString *)expression
                         inFile:(NSString *)filename
                         atLine:(int)lineNumber
                withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(4, 5);
+ (NSException *)failureInRaise:(NSString *)expression
                      exception:(NSException *)exception
                         inFile:(NSString *)filename
                         atLine:(int)lineNumber
                withDescription:(NSString *)formatString, ... NS_FORMAT_FUNCTION(5, 6);
@end

// SENTE_END

@protocol SenTestCase
+ (id)testCaseWithInvocation:(NSInvocation *)anInvocation;
- (id)initWithInvocation:(NSInvocation *)anInvocation;
- (void)setUp;
- (void)invokeTest;
- (void)tearDown;
- (void)performTest;
- (void)failWithException:(NSException*)exception;
- (NSInvocation *)invocation;
- (SEL)selector;
+ (NSArray *)testInvocations;
@end

@interface SenTestCase : NSObject<SenTestCase> {
 @private
  NSInvocation *invocation_;
}
@end

GTM_EXTERN NSString *const SenTestFailureException;

GTM_EXTERN NSString *const SenTestFilenameKey;
GTM_EXTERN NSString *const SenTestLineNumberKey;

#endif // GTM_IPHONE_SDK && !GTM_IPHONE_USE_SENTEST

// All unittest cases in GTM should inherit from GTMTestCase. It makes sure
// to set up our logging system correctly to verify logging calls.
// See GTMUnitTestDevLog.h for details
@interface GTMTestCase : SenTestCase

// Returns YES if this is an abstract testCase class as opposed to a concrete
// testCase class that you want tests run against. SenTestCase is not designed
// out of the box to handle an abstract class hierarchy descending from it with
// some concrete subclasses.  In some cases we want all the "concrete"
// subclasses of an abstract subclass of SenTestCase to run a test, but we don't
// want that test to be run against an instance of an abstract subclass itself.
// By returning "YES" here, the tests defined by this class won't be run against
// an instance of this class. As an example class hierarchy:
//
//                                            FooExtensionTestCase
// GTMTestCase <- ExtensionAbstractTestCase <
//                                            BarExtensionTestCase
//
// So FooExtensionTestCase and BarExtensionTestCase inherit from
// ExtensionAbstractTestCase (and probably FooExtension and BarExtension inherit
// from a class named Extension). We want the tests in ExtensionAbstractTestCase
// to be run as part of FooExtensionTestCase and BarExtensionTestCase, but we
// don't want them run against ExtensionAbstractTestCase. The default
// implementation checks to see if the name of the class contains the word
// "AbstractTest" (case sensitive).
+ (BOOL)isAbstractTestCase;

@end
