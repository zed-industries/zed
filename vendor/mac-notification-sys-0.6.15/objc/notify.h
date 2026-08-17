#import <Cocoa/Cocoa.h>
#import <CoreServices/CoreServices.h>
#import <Foundation/Foundation.h>
#import <objc/runtime.h>

NSString* fakeBundleIdentifier = nil;

NSString* getBundleIdentifier(NSString* appName);
BOOL setApplication(NSString* newbundleIdentifier);
void sendNotification(NSString* title, NSString* subtitle, NSString* message, NSDictionary* options, const unsigned char* notificationId, BOOL shouldWait);
void setupDelegate(void);

// Rust callbacks — implemented in lib.rs, called from ObjC delegate
// activationType: 0=none, 1=actionClicked, 2=contentsClicked, 3=replied
// actionValueIndex: selected dropdown index, or -1 if not applicable
extern void rust_notification_activated(const unsigned char* uuid, uint8_t activationType, const char* actionValue, int64_t actionValueIndex);
extern void rust_notification_dismissed(const unsigned char* uuid, const char* buttonTitle);
extern void rust_notification_auto_dismissed(const unsigned char* uuid);
extern BOOL rust_notification_is_done(const unsigned char* uuid);
extern void rust_wait_for_notification(const unsigned char* uuid);
// Delivery-confirmation callbacks (fire-and-forget path)
extern void rust_notification_delivered(const unsigned char* uuid);
extern BOOL rust_notification_is_delivered(const unsigned char* uuid);
extern void rust_wait_for_delivery(const unsigned char* uuid);

@implementation NSBundle (swizzle)
- (NSString*)__bundleIdentifier {
    if (self == [NSBundle mainBundle]) {
        return fakeBundleIdentifier ? fakeBundleIdentifier : @"com.apple.Terminal";
    } else {
        return [self __bundleIdentifier];
    }
}
@end

BOOL installNSBundleHook(void) {
    Class cls = objc_getClass("NSBundle");
    if (cls) {
        method_exchangeImplementations(class_getInstanceMethod(cls, @selector(bundleIdentifier)),
                                       class_getInstanceMethod(cls, @selector(__bundleIdentifier)));
        return YES;
    }
    return NO;
}

@interface NotificationCenterDelegate: NSObject <NSUserNotificationCenterDelegate>
+ (instancetype)sharedDelegate;
@end
