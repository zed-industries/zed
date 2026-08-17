// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_OCMOCK_OCMOCK_EXTENSIONS_H_
#define THIRD_PARTY_OCMOCK_OCMOCK_EXTENSIONS_H_

#import <Foundation/Foundation.h>

#import "third_party/ocmock/OCMock/OCMock.h"

// Some enhancements to OCMock to make it easier to write mocks.
// Pointers to objects still have to be handled with
// - (id)andReturnValue:OCMOCK_VALUE(blah)
// to keep the types working correctly.
@interface OCMStubRecorder(CrExtensions)
- (id)andReturnChar:(char)value;
- (id)andReturnUnsignedChar:(unsigned char)value;
- (id)andReturnShort:(short)value;
- (id)andReturnUnsignedShort:(unsigned short)value;
- (id)andReturnInt:(int)value;
- (id)andReturnUnsignedInt:(unsigned int)value;
- (id)andReturnLong:(long)value;
- (id)andReturnUnsignedLong:(unsigned long)value;
- (id)andReturnLongLong:(long long)value;
- (id)andReturnUnsignedLongLong:(unsigned long long)value;
- (id)andReturnFloat:(float)value;
- (id)andReturnDouble:(double)value;
- (id)andReturnBool:(BOOL)value;
- (id)andReturnInteger:(NSInteger)value;
- (id)andReturnUnsignedInteger:(NSUInteger)value;
#if !TARGET_OS_IPHONE
- (id)andReturnCGFloat:(CGFloat)value;
- (id)andReturnNSRect:(NSRect)rect;
- (id)andReturnCGRect:(CGRect)rect;
- (id)andReturnNSPoint:(NSPoint)point;
- (id)andReturnCGPoint:(CGPoint)point;
#endif
@end

// A constraint for verifying that something conforms to a protocol.
@interface cr_OCMConformToProtocolConstraint : OCMConstraint {
 @private
  Protocol* protocol_;
}
- (id)initWithProtocol:(Protocol*)protocol;
@end

@interface OCMArg(CrExtensions)
+ (id)conformsToProtocol:(Protocol*)protocol;
+ (id)invokeBlockOnQueue:(dispatch_queue_t)queue
                withArgs:(id)first, ... NS_REQUIRES_NIL_TERMINATION;
@end

@interface OCMockObject (CrExtensions)
// Recorded invocations can contain objects that clients expect to be
// deallocated by now, and they can also have a strong reference to self,
// creating a retain cycle. Get rid of all of the invocations to hopefully
// let their objects deallocate, and to break any retain cycles involving self.
// This is similar to `stopMocking`, but calling the latter will also cause the
// mock object to no longer be usable, while sometimes it is desirable to
// clear references while still keeping the mock object iself alive and usable.
- (void)clearInvocations;
@end

#endif  // THIRD_PARTY_OCMOCK_OCMOCK_EXTENSIONS_H_
