#import <ScreenCaptureKit/ScreenCaptureKit.h>

@interface MyClass : NSObject <SCStreamOutput, SCStreamDelegate>
@end

@implementation MyClass

- (void)stream:(SCStream *)stream
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
    ofType:(SCStreamOutputType)type {
    printf("dummy capture handler called");
}

- (void)stream:(SCStream *)stream didStopWithError:(NSError *)error {
    printf("dummy did stop with error called");
}

int main() {
    [[MyClass alloc] init];
}

@end
