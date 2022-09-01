//
//  LKRoom.m
//  LiveKitObjC
//
//  Created by Antonio Scandurra on 01/09/22.
//

#import <Foundation/Foundation.h>
#import <LiveKitObjC-Swift.h>

@interface LKRoom: NSObject {
}

@property (nonatomic, retain) SLKRoom* room;
@end

@implementation LKRoom
-(id)init {
    if (self = [super init]) {
        self.room = [[SLKRoom alloc] init];
    }
    return self;
}

-(void)connectWithURL:(NSString *)url token:(NSString *)token callback:(void(^)(void))callback {
    [self.room connectWithUrl:url token:token callback:callback];
}
@end

LKRoom* BuildLKRoom() {
    return [[LKRoom alloc] init];
}
