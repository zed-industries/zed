/*
 *  Copyright (c) 2005-2021 Erik Doernenburg and contributors
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

#import "OCMockObject.h"

@interface OCClassMockObject : OCMockObject
{
    Class mockedClass;
    Class originalMetaClass;
    Class classCreatedForNewMetaClass;
}

- (id)initWithClass:(Class)aClass;

- (Class)mockedClass;
- (Class)mockObjectClass; // since -class returns the mockedClass

- (void)assertClassIsSupported:(Class)aClass;

@end
