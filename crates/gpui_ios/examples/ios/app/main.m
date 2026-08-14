#import <QuartzCore/QuartzCore.h>
#import <UIKit/UIKit.h>

extern bool gpui_ios_example_run(void);
extern void *gpui_ios_get_window(void);
extern void gpui_ios_request_frame(void *window);
extern void gpui_ios_will_enter_foreground(void *application);
extern void gpui_ios_did_become_active(void *application);
extern void gpui_ios_will_resign_active(void *application);
extern void gpui_ios_did_enter_background(void *application);
extern void gpui_ios_did_receive_memory_warning(void *application);
extern void gpui_ios_will_terminate(void *application);
extern void gpui_ios_handle_open_url(void *url);

@interface GPUIAppDelegate : UIResponder <UIApplicationDelegate>
@property(nonatomic, strong) CADisplayLink *displayLink;
@end

@implementation GPUIAppDelegate

- (BOOL)application:(UIApplication *)application
    didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    if (!gpui_ios_example_run()) {
        return NO;
    }

    self.displayLink = [CADisplayLink displayLinkWithTarget:self
                                                  selector:@selector(renderFrame:)];
    [self.displayLink addToRunLoop:NSRunLoop.mainRunLoop forMode:NSRunLoopCommonModes];
    return YES;
}

- (void)renderFrame:(CADisplayLink *)displayLink {
    void *window = gpui_ios_get_window();
    if (window != NULL) {
        gpui_ios_request_frame(window);
    }
}

- (void)applicationWillEnterForeground:(UIApplication *)application {
    gpui_ios_will_enter_foreground((__bridge void *)application);
}

- (void)applicationDidBecomeActive:(UIApplication *)application {
    gpui_ios_did_become_active((__bridge void *)application);
}

- (void)applicationWillResignActive:(UIApplication *)application {
    gpui_ios_will_resign_active((__bridge void *)application);
}

- (void)applicationDidEnterBackground:(UIApplication *)application {
    gpui_ios_did_enter_background((__bridge void *)application);
}

- (void)applicationDidReceiveMemoryWarning:(UIApplication *)application {
    gpui_ios_did_receive_memory_warning((__bridge void *)application);
}

- (void)applicationWillTerminate:(UIApplication *)application {
    [self.displayLink invalidate];
    gpui_ios_will_terminate((__bridge void *)application);
}

- (BOOL)application:(UIApplication *)application
            openURL:(NSURL *)url
            options:(NSDictionary<UIApplicationOpenURLOptionsKey, id> *)options {
    gpui_ios_handle_open_url((__bridge void *)url.absoluteString);
    return YES;
}

@end

int main(int argc, char *argv[]) {
    @autoreleasepool {
        return UIApplicationMain(
            argc,
            argv,
            nil,
            NSStringFromClass(GPUIAppDelegate.class)
        );
    }
}
