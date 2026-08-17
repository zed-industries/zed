/*
 *  Copyright (c) 2006-2021 Erik Doernenburg and contributors
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

@interface NSInvocation(OCMAdditions)

+ (NSInvocation *)invocationForBlock:(id)block withArguments:(NSArray *)arguments;

- (void)retainObjectArgumentsExcludingObject:(id)objectToExclude;

- (id)getArgumentAtIndexAsObject:(NSInteger)argIndex;

- (NSString *)invocationDescription;

- (NSString *)argumentDescriptionAtIndex:(NSInteger)argIndex;

- (NSString *)objectDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)charDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)unsignedCharDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)intDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)unsignedIntDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)shortDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)unsignedShortDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)longDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)unsignedLongDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)longLongDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)unsignedLongLongDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)doubleDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)floatDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)structDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)pointerDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)cStringDescriptionAtIndex:(NSInteger)anInt;
- (NSString *)selectorDescriptionAtIndex:(NSInteger)anInt;

- (BOOL)methodIsInInitFamily;
- (BOOL)methodIsInCreateFamily;

@end
