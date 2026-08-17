/*
 *  Copyright (c) 2009-2021 Erik Doernenburg and contributors
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


__deprecated_msg("Please use XCTNSNotificationExpectation instead.")
    @interface OCObserverMockObject : NSObject
{
    BOOL            expectationOrderMatters;
    NSMutableArray *recorders;
    NSMutableArray *centers;
}

- (void)setExpectationOrderMatters:(BOOL)flag;

- (id)expect;

- (void)verify;
- (void)verifyAtLocation:(OCMLocation *)location;

- (void)handleNotification:(NSNotification *)aNotification;

// internal use

- (void)autoRemoveFromCenter:(NSNotificationCenter *)aCenter;
- (NSNotification *)notificationWithName:(NSString *)name object:(id)sender;

@end
