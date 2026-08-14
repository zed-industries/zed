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
extern void gpui_ios_set_window_scene(void *scene);

@interface GPUIExampleSceneDelegate : UIResponder <UIWindowSceneDelegate>
@property(nonatomic, strong) CADisplayLink *displayLink;
@end

@interface GPUIAppDelegate : UIResponder <UIApplicationDelegate>
@end

@implementation GPUIExampleSceneDelegate

- (void)scene:(UIScene *)scene
    willConnectToSession:(UISceneSession *)session
                 options:(UISceneConnectionOptions *)connectionOptions {
    if (![scene isKindOfClass:UIWindowScene.class]) {
        return;
    }

    gpui_ios_set_window_scene((__bridge void *)scene);
    if (!gpui_ios_example_run()) {
        return;
    }

    self.displayLink = [CADisplayLink displayLinkWithTarget:self
                                                  selector:@selector(renderFrame:)];
    [self.displayLink addToRunLoop:NSRunLoop.mainRunLoop forMode:NSRunLoopCommonModes];
}

- (void)renderFrame:(CADisplayLink *)displayLink {
    void *window = gpui_ios_get_window();
    if (window != NULL) {
        gpui_ios_request_frame(window);
    }
}

- (void)sceneWillEnterForeground:(UIScene *)scene {
    gpui_ios_will_enter_foreground((__bridge void *)scene);
}

- (void)sceneDidBecomeActive:(UIScene *)scene {
    gpui_ios_did_become_active((__bridge void *)scene);
}

- (void)sceneWillResignActive:(UIScene *)scene {
    gpui_ios_will_resign_active((__bridge void *)scene);
}

- (void)sceneDidEnterBackground:(UIScene *)scene {
    gpui_ios_did_enter_background((__bridge void *)scene);
}

- (void)sceneDidDisconnect:(UIScene *)scene {
    [self.displayLink invalidate];
}

- (void)scene:(UIScene *)scene
    openURLContexts:(NSSet<UIOpenURLContext *> *)URLContexts {
    UIOpenURLContext *context = URLContexts.anyObject;
    if (context != nil) {
        gpui_ios_handle_open_url((__bridge void *)context.URL.absoluteString);
    }
}

@end

@implementation GPUIAppDelegate

- (BOOL)application:(UIApplication *)application
    didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    return YES;
}

- (UISceneConfiguration *)application:(UIApplication *)application
    configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession
                                   options:(UISceneConnectionOptions *)options {
    UISceneConfiguration *configuration =
        [[UISceneConfiguration alloc] initWithName:@"Default Configuration"
                                       sessionRole:connectingSceneSession.role];
    configuration.delegateClass = GPUIExampleSceneDelegate.class;
    return configuration;
}

- (void)applicationDidReceiveMemoryWarning:(UIApplication *)application {
    gpui_ios_did_receive_memory_warning((__bridge void *)application);
}

- (void)applicationWillTerminate:(UIApplication *)application {
    gpui_ios_will_terminate((__bridge void *)application);
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
