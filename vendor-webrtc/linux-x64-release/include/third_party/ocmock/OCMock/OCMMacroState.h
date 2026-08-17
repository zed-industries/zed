/*
 *  Copyright (c) 2014-2021 Erik Doernenburg and contributors
 *
 *  Licensed under the Apache License, Version 2.0 (the "License"); you may
 *  not use these files except in compliance with the License. You may obtain
 *  a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 *  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
 *  License for the specific language governing permissions and limitations
 *  under the License.
 */

#import <Foundation/Foundation.h>

@class OCMLocation;
@class OCMQuantifier;
@class OCMRecorder;
@class OCMStubRecorder;
@class OCMockObject;


@interface OCMMacroState : NSObject
{
    id   recorder;
    BOOL invocationDidThrow;
}

+ (void)beginStubMacro;
+ (OCMStubRecorder *)endStubMacro;

+ (void)beginExpectMacro;
+ (OCMStubRecorder *)endExpectMacro;

+ (void)beginRejectMacro;
+ (OCMStubRecorder *)endRejectMacro;

+ (void)beginVerifyMacroAtLocation:(OCMLocation *)aLocation;
+ (void)beginVerifyMacroAtLocation:(OCMLocation *)aLocation withQuantifier:(OCMQuantifier *)quantifier;
+ (void)endVerifyMacro;

+ (OCMMacroState *)globalState;

- (void)setRecorder:(id)aRecorder;
- (id)recorder;

- (void)switchToClassMethod;

- (void)setInvocationDidThrow:(BOOL)flag;
- (BOOL)invocationDidThrow;

@end
