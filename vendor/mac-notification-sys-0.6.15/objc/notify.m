#import "notify.h"

// Seconds to wait for async XPC delivery confirmation before giving up.
// Matched by DELIVERY_TIMEOUT_SECS in bridge.rs.
static const NSTimeInterval kDeliveryTimeoutSecs = 2.0;

// getBundleIdentifier(app_name: &str) -> "com.apple.Terminal"
NSString* getBundleIdentifier(NSString* appName) {
    NSString* findString = [NSString stringWithFormat:@"get id of application \"%@\"", appName];
    NSAppleScript* findScript = [[NSAppleScript alloc] initWithSource:findString];
    NSAppleEventDescriptor* resultDescriptor = [findScript executeAndReturnError:nil];
    return [resultDescriptor stringValue];
}

// setApplication(new_bundle_identifier: &str) -> Result<()>
// invariant: this function should be called at most once and before `sendNotification`
BOOL setApplication(NSString* newbundleIdentifier) {
    @autoreleasepool {
        if (!installNSBundleHook()) {
            return NO;
        }
        if (LSCopyApplicationURLsForBundleIdentifier((CFStringRef)newbundleIdentifier, NULL) != NULL) {
            [fakeBundleIdentifier release];
            fakeBundleIdentifier = newbundleIdentifier;
            [newbundleIdentifier retain];
            return YES;
        }
        return NO;
    }
}

// handles both file:// and bare paths
NSImage* getImageFromURL(NSString* url) {
    NSURL* imageURL = [NSURL URLWithString:url];
    if ([[imageURL scheme] length] == 0) {
        imageURL = [NSURL fileURLWithPath:url];
    }
    return [[NSImage alloc] initWithContentsOfURL:imageURL];
}

// Extracts the raw UUID bytes from a notification's identifier string.
// NSUserNotification.identifier is an NSString (UUID canonical form); we reconstruct
// the NSUUID and write 16 bytes into `out` so Rust callbacks receive plain bytes.
// If the identifier is nil or not a valid UUID string (e.g. on very old OS versions
// or for notifications we did not create), out is zeroed — the Rust side will find
// no matching PENDING entry and silently discard the callback.
static void uuidBytesFromNotification(NSUserNotification* n, unsigned char out[16]) {
    NSUUID* uuid = n.identifier ? [[NSUUID alloc] initWithUUIDString:n.identifier] : nil;
    if (uuid) {
        [uuid getUUIDBytes:out];
        [uuid release];
    } else {
        memset(out, 0, 16);
    }
}

// resolveAutoDismiss — called on the main thread when wasAutoDismissed() fires.
// NSUserNotification has no auto-dismiss delegate callback; polling deliveredNotifications is the
// only signal. We drain any already-queued delegate messages (didActivate / didDismissAlert) before
// falling back to treating the disappearance as a silent auto-dismiss. This removes all timing
// heuristics: if a real callback was queued, it fires during the runUntilDate: drain and wins.
static void resolveAutoDismiss(const unsigned char* uuid) {
    if (rust_notification_is_done(uuid))
        return; // a real callback already won
    // Drain delegate messages already queued on this run loop, then re-check.
    [[NSRunLoop currentRunLoop] runUntilDate:[NSDate date]];
    if (rust_notification_is_done(uuid))
        return;
    rust_notification_auto_dismissed(uuid);
}

// sendNotification — delivers or schedules a notification identified by notificationId (16 bytes).
// shouldWait: if NO, returns immediately after delivery (fire-and-forget).
// if YES, blocks until the user interacts or the notification is auto-dismissed.
// Result is communicated back to Rust via rust_notification_activated / rust_notification_dismissed /
// rust_notification_auto_dismissed callbacks.
void sendNotification(NSString* title, NSString* subtitle, NSString* message, NSDictionary* options, const unsigned char* notificationId, BOOL shouldWait) {
    @autoreleasepool {
        NSUserNotificationCenter* notificationCenter = [NSUserNotificationCenter defaultUserNotificationCenter];

        NSUserNotification* userNotification = [[NSUserNotification alloc] init];
        BOOL isScheduled = NO;

        NSUUID* uuid = [[NSUUID alloc] initWithUUIDBytes:notificationId];
        NSString* identifierString = [uuid UUIDString];
        userNotification.identifier = identifierString;
        [uuid release];

        // Basic text
        userNotification.title = title;
        if (![subtitle isEqualToString:@""]) {
            userNotification.subtitle = subtitle;
        }
        userNotification.informativeText = message;

        // Notification sound
        if (options[@"sound"] && ![options[@"sound"] isEqualToString:@""]) {
            if ([options[@"sound"] isEqualToString:@"NSUserNotificationDefaultSoundName"]) {
                userNotification.soundName = NSUserNotificationDefaultSoundName;
            } else {
                userNotification.soundName = options[@"sound"];
            }
        }

        // Delivery Date/Schedule
        if (options[@"deliveryDate"] && ![options[@"deliveryDate"] isEqualToString:@""]) {
            double deliveryDate = [options[@"deliveryDate"] doubleValue];
            userNotification.deliveryDate = [NSDate dateWithTimeIntervalSince1970:deliveryDate];
            isScheduled = YES;
        }

        // Main Actions Button (defaults to "Show")
        if (options[@"mainButtonLabel"] && ![options[@"mainButtonLabel"] isEqualToString:@""]) {
            userNotification.actionButtonTitle = options[@"mainButtonLabel"];
            userNotification.hasActionButton = 1;
        } else {
            userNotification.hasActionButton = 0;
        }

        // Dropdown actions
        if (options[@"actions"] && ![options[@"actions"] isEqualToString:@""]) {
            [userNotification setValue:@YES forKey:@"_showsButtons"];
            NSArray* myActions = [options[@"actions"] componentsSeparatedByString:@","];
            if (myActions.count > 1) {
                [userNotification setValue:@YES forKey:@"_alwaysShowAlternateActionMenu"];
                [userNotification setValue:myActions forKey:@"_alternateActionButtonTitles"];
            }
        }

        // Close/Other button (defaults to "Cancel")
        if (options[@"closeButtonLabel"] && ![options[@"closeButtonLabel"] isEqualToString:@""]) {
            [userNotification setValue:@YES forKey:@"_showsButtons"];
            userNotification.otherButtonTitle = options[@"closeButtonLabel"];
        }

        // Reply to the notification with a text field
        if (options[@"response"] && ![options[@"response"] isEqualToString:@""]) {
            userNotification.hasReplyButton = 1;
            userNotification.responsePlaceholder = options[@"mainButtonLabel"];
        }

        // Change the icon of the app in the notification
        if (options[@"appIcon"] && ![options[@"appIcon"] isEqualToString:@""]) {
            NSImage* icon = getImageFromURL(options[@"appIcon"]);
            [userNotification setValue:icon forKey:@"_identityImage"];
            [userNotification setValue:@(false) forKey:@"_identityImageHasBorder"];
        }

        // Change the additional content image
        if (options[@"contentImage"] && ![options[@"contentImage"] isEqualToString:@""]) {
            userNotification.contentImage = getImageFromURL(options[@"contentImage"]);
        }

        // Send or schedule notification
        if (isScheduled) {
            [notificationCenter scheduleNotification:userNotification];
        } else {
            [notificationCenter deliverNotification:userNotification];
        }

        if (!shouldWait) {
            // Block until didDeliverNotification: confirms the async XPC delivery completed,
            // so a fire-and-forget caller can't exit before the notification is shown.
            // Scheduled notifications are excluded — their delivery date is in the future and
            // didDeliverNotification: won't fire until then.  Bounded by a safety timeout so
            // we never hang if the callback is never delivered.
            if (!isScheduled) {
                if ([NSThread isMainThread]) {
                    NSDate* deadline = [NSDate dateWithTimeIntervalSinceNow:kDeliveryTimeoutSecs];
                    while (!rust_notification_is_delivered(notificationId) &&
                           [deadline timeIntervalSinceNow] > 0) {
                        [[NSRunLoop currentRunLoop]
                            runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.05]];
                    }
                } else {
                    // Blocks on a timed Condvar; returns once delivered or after 2s.
                    rust_wait_for_delivery(notificationId);
                }
            }
            return;
        }

        // auto-dismiss: notification disappeared from deliveredNotifications without a callback
        BOOL (^wasAutoDismissed)(void) = ^BOOL {
          for (NSUserNotification* n in notificationCenter.deliveredNotifications) {
              if ([n.identifier isEqualToString:identifierString])
                  return NO;
          }
          return YES;
        };

        if ([NSThread isMainThread]) {
            // wait for delivery before checking dismissal. the async delivery window
            // would otherwise look like an immediate dismiss. timeout after 2s so we don't hang
            NSDate* deliveryDeadline = [NSDate dateWithTimeIntervalSinceNow:kDeliveryTimeoutSecs];
            while (!rust_notification_is_delivered(notificationId) && !rust_notification_is_done(notificationId) &&
                   [deliveryDeadline timeIntervalSinceNow] > 0) {
                [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.05]];
            }
            // delivery timed out without confirmation -> treat as auto-dismiss
            if (!rust_notification_is_done(notificationId) && !rust_notification_is_delivered(notificationId))
                resolveAutoDismiss(notificationId);

            // keep the run loop spinning; callbacks arrive here and signal when done
            while (!rust_notification_is_done(notificationId)) {
                [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
                if (rust_notification_is_delivered(notificationId) && wasAutoDismissed())
                    resolveAutoDismiss(notificationId);
            }
        } else {
            // callbacks come in on main thread, start poll timer there (#86)
            // copy UUID to NSData so the block owns the lifetime, not the Rust stack
            NSData* notificationIdData = [NSData dataWithBytes:notificationId length:16];
            NSDate* pollStarted = [NSDate date];
            NSTimer* dismissPoll = [NSTimer timerWithTimeInterval:0.5
                                                          repeats:YES
                                                            block:^(NSTimer* t) {
                                                              // wait for delivery before checking dismiss, same logic as main thread
                                                              if (!rust_notification_is_delivered(notificationIdData.bytes)) {
                                                                  if (!rust_notification_is_done(notificationIdData.bytes) && -[pollStarted timeIntervalSinceNow] > kDeliveryTimeoutSecs) {
                                                                      [t invalidate];
                                                                      resolveAutoDismiss(notificationIdData.bytes);
                                                                  }
                                                                  return;
                                                              }
                                                              if (wasAutoDismissed()) {
                                                                  [t invalidate];
                                                                  resolveAutoDismiss(notificationIdData.bytes);
                                                              }
                                                            }];
            [[NSRunLoop mainRunLoop] addTimer:dismissPoll forMode:NSDefaultRunLoopMode];
            // Blocks until rust_notification_auto_dismissed / rust_notification_activated /
            // rust_notification_dismissed signals the Condvar.
            rust_wait_for_notification(notificationId);
            dispatch_async(dispatch_get_main_queue(), ^{
              [dismissPoll invalidate];
            });
        }
    }
}

@implementation NotificationCenterDelegate

- (void)userNotificationCenter:(NSUserNotificationCenter*)center
        didDeliverNotification:(NSUserNotification*)notification {
    unsigned char bytes[16];
    uuidBytesFromNotification(notification, bytes);
    rust_notification_delivered(bytes);
}

- (void)userNotificationCenter:(NSUserNotificationCenter*)center
       didActivateNotification:(NSUserNotification*)notification {
    unsigned char bytes[16];
    uuidBytesFromNotification(notification, bytes);

    uint8_t activationType = 0;
    const char* actionValue = NULL;
    int64_t actionValueIndex = -1;

    switch (notification.activationType) {
        case NSUserNotificationActivationTypeActionButtonClicked:
        case NSUserNotificationActivationTypeAdditionalActionClicked: {
            NSArray* altTitles = [(NSObject*)notification valueForKey:@"_alternateActionButtonTitles"];
            if ([altTitles count] > 1) {
                NSNumber* altIdx = [(NSObject*)notification valueForKey:@"_alternateActionIndex"];
                unsigned long long idx = [altIdx unsignedLongLongValue];
                if (idx == (unsigned long long)LONG_MAX) {
                    actionValue = [notification.actionButtonTitle UTF8String];
                } else {
                    actionValue = [altTitles[idx] UTF8String];
                    actionValueIndex = (int64_t)idx;
                }
            } else {
                actionValue = [notification.actionButtonTitle UTF8String];
            }
            activationType = 1;
            break;
        }
        case NSUserNotificationActivationTypeContentsClicked:
            activationType = 2;
            break;
        case NSUserNotificationActivationTypeReplied:
            activationType = 3;
            actionValue = [notification.response.string UTF8String];
            break;
        case NSUserNotificationActivationTypeNone:
        default:
            break;
    }

    rust_notification_activated(bytes, activationType, actionValue, actionValueIndex);
    [center removeDeliveredNotification:notification];
}

- (void)userNotificationCenter:(NSUserNotificationCenter*)center
               didDismissAlert:(NSUserNotification*)notification {
    unsigned char bytes[16];
    uuidBytesFromNotification(notification, bytes);
    rust_notification_dismissed(bytes, [notification.otherButtonTitle UTF8String]);
    [center removeDeliveredNotification:notification];
}

+ (instancetype)sharedDelegate {
    static NotificationCenterDelegate* instance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
      instance = [[NotificationCenterDelegate alloc] init];
      [NSUserNotificationCenter defaultUserNotificationCenter].delegate = instance;
    });
    return instance;
}
@end

void setupDelegate(void) {
    [NotificationCenterDelegate sharedDelegate];
}
