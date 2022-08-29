#import <ScreenCaptureKit/ScreenCaptureKit.h>

@interface MyClass : NSObject <SCStreamOutput>
@end

@implementation MyClass
- (void)stream:(SCStream *)stream
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
                   ofType:(SCStreamOutputType)type {
}

@end