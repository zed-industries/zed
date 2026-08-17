/*
 *  Copyright (c) 2016-2021 Erik Doernenburg and contributors
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

@interface OCMQuantifier : NSObject
{
    NSUInteger expectedCount;
}

+ (instancetype)never;
+ (instancetype)exactly:(NSUInteger)count;
+ (instancetype)atLeast:(NSUInteger)count;
+ (instancetype)atMost:(NSUInteger)count;

- (BOOL)isValidCount:(NSUInteger)count;

- (NSString *)description;

@end


#define OCMNever()      ([OCMQuantifier never])
#define OCMTimes(n)     ([OCMQuantifier exactly:(n)])
#define OCMAtLeast(n)   ([OCMQuantifier atLeast:(n)])
#define OCMAtMost(n)    ([OCMQuantifier atMost:(n)])

#ifndef OCM_DISABLE_SHORT_QSYNTAX
#define never()         OCMNever()
#define times(n)        OCMTimes(n)
#define atLeast(n)      OCMAtLeast(n)
#define atMost(n)       OCMAtMost(n)
#endif
