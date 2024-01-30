/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * os_macosx.m -- Mac specific things for Mac OS X.
 */

/* Suppress compiler warnings to non-C89 code. */
#if defined(__clang__) && defined(__STRICT_ANSI__)
# pragma clang diagnostic push
# pragma clang diagnostic ignored "-Wc99-extensions"
# pragma clang diagnostic push
# pragma clang diagnostic ignored "-Wdeclaration-after-statement"
#endif

/* Avoid a conflict for the definition of Boolean between Mac header files and
 * X11 header files. */
#define NO_X11_INCLUDES

#include <stdbool.h>
#include <mach/boolean.h>
#include <sys/errno.h>
#include <stdlib.h>

#ifdef FEAT_RELTIME
#include <dispatch/dispatch.h>
#endif

#include "vim.h"
#import <AppKit/AppKit.h>


/*
 * Clipboard support for the console.
 */
#if defined(FEAT_CLIPBOARD)

/* Used to identify clipboard data copied from Vim. */

NSString *VimPboardType = @"VimPboardType";

    void
clip_mch_lose_selection(Clipboard_T *cbd UNUSED)
{
}


    int
clip_mch_own_selection(Clipboard_T *cbd UNUSED)
{
    /* This is called whenever there is a new selection and 'guioptions'
     * contains the "a" flag (automatically copy selection).  Return TRUE, else
     * the "a" flag does nothing.  Note that there is no concept of "ownership"
     * of the clipboard in Mac OS X.
     */
    return TRUE;
}


    void
clip_mch_request_selection(Clipboard_T *cbd)
{
    NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

    NSPasteboard *pb = [NSPasteboard generalPasteboard];
#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1060
    NSArray *supportedTypes = [NSArray arrayWithObjects:VimPboardType,
	    NSPasteboardTypeString, nil];
#else
    NSArray *supportedTypes = [NSArray arrayWithObjects:VimPboardType,
	    NSStringPboardType, nil];
#endif
    NSString *bestType = [pb availableTypeFromArray:supportedTypes];
    if (!bestType) goto releasepool;

    int motion_type = MAUTO;
    NSString *string = nil;

    if ([bestType isEqual:VimPboardType])
    {
	/* This type should consist of an array with two objects:
	 *   1. motion type (NSNumber)
	 *   2. text (NSString)
	 * If this is not the case we fall back on using NSPasteboardTypeString.
	 */
	id plist = [pb propertyListForType:VimPboardType];
	if ([plist isKindOfClass:[NSArray class]] && [plist count] == 2)
	{
	    id obj = [plist objectAtIndex:1];
	    if ([obj isKindOfClass:[NSString class]])
	    {
		motion_type = [[plist objectAtIndex:0] intValue];
		string = obj;
	    }
	}
    }

    if (!string)
    {
	/* Use NSPasteboardTypeString.  The motion type is detected automatically.
	 */
#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1060
	NSMutableString *mstring =
		[[pb stringForType:NSPasteboardTypeString] mutableCopy];
#else
	NSMutableString *mstring =
		[[pb stringForType:NSStringPboardType] mutableCopy];
#endif
	if (!mstring) goto releasepool;

	/* Replace unrecognized end-of-line sequences with \x0a (line feed). */
	NSRange range = { 0, [mstring length] };
	unsigned n = [mstring replaceOccurrencesOfString:@"\x0d\x0a"
					     withString:@"\x0a" options:0
						  range:range];
	if (0 == n)
	{
	    n = [mstring replaceOccurrencesOfString:@"\x0d" withString:@"\x0a"
					   options:0 range:range];
	}

	string = mstring;
    }

    /* Default to MAUTO, uses MCHAR or MLINE depending on trailing NL. */
    if (!(MCHAR == motion_type || MLINE == motion_type || MBLOCK == motion_type
	    || MAUTO == motion_type))
	motion_type = MAUTO;

    char_u *str = (char_u*)[string UTF8String];
    int len = [string lengthOfBytesUsingEncoding:NSUTF8StringEncoding];

    if (input_conv.vc_type != CONV_NONE)
	str = string_convert(&input_conv, str, &len);

    if (str)
	clip_yank_selection(motion_type, str, len, cbd);

    if (input_conv.vc_type != CONV_NONE)
	vim_free(str);

releasepool:
    [pool release];
}


/*
 * Send the current selection to the clipboard.
 */
    void
clip_mch_set_selection(Clipboard_T *cbd)
{
    NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

    /* If the '*' register isn't already filled in, fill it in now. */
    cbd->owned = TRUE;
    clip_get_selection(cbd);
    cbd->owned = FALSE;

    /* Get the text to put on the pasteboard. */
    long_u llen = 0; char_u *str = 0;
    int motion_type = clip_convert_selection(&str, &llen, cbd);
    if (motion_type < 0)
	goto releasepool;

    /* TODO: Avoid overflow. */
    int len = (int)llen;
    if (output_conv.vc_type != CONV_NONE)
    {
	char_u *conv_str = string_convert(&output_conv, str, &len);
	if (conv_str)
	{
	    vim_free(str);
	    str = conv_str;
	}
    }

    if (len > 0)
    {
	NSString *string = [[NSString alloc]
	    initWithBytes:str length:len encoding:NSUTF8StringEncoding];

	/* See clip_mch_request_selection() for info on pasteboard types. */
	NSPasteboard *pb = [NSPasteboard generalPasteboard];
#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1060
	NSArray *supportedTypes = [NSArray arrayWithObjects:VimPboardType,
		NSPasteboardTypeString, nil];
#else
	NSArray *supportedTypes = [NSArray arrayWithObjects:VimPboardType,
		NSStringPboardType, nil];
#endif
	[pb declareTypes:supportedTypes owner:nil];

	NSNumber *motion = [NSNumber numberWithInt:motion_type];
	NSArray *plist = [NSArray arrayWithObjects:motion, string, nil];
	[pb setPropertyList:plist forType:VimPboardType];

#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1060
	[pb setString:string forType:NSPasteboardTypeString];
#else
	[pb setString:string forType:NSStringPboardType];
#endif

	[string release];
    }

    vim_free(str);
releasepool:
    [pool release];
}

#endif /* FEAT_CLIPBOARD */

#ifdef FEAT_RELTIME
/*
 * The following timer code is based on a Gist by Jorgen Lundman:
 *
 *     https://gist.github.com/lundman
 */

typedef struct macos_timer macos_timer_T;

    static void
_timer_cancel(void *arg UNUSED)
{
    // This is not currently used, but it might be useful in the future and
    // it is non-trivial enough to provide as usable implementation.
#   if 0
    macos_timer_T *timerid = (macos_timer_T *)arg;

    dispatch_release(timerid->tim_timer);
    dispatch_release(timerid->tim_queue);
    timerid->tim_timer = NULL;
    timerid->tim_queue = NULL;
    free(timerid);
#   endif
}

    static void
_timer_handler(void *arg)
{
    macos_timer_T *timerid = (macos_timer_T *)arg;
    union sigval sv;

    sv.sival_ptr = timerid->tim_arg;

    if (timerid->tim_func != NULL)
	timerid->tim_func(sv);
}

    static uint64_t
itime_to_ns(const struct timespec *it)
{
    time_t   sec  = it->tv_sec;
    long     nsec = it->tv_nsec;
    uint64_t ns   = NSEC_PER_SEC * sec + nsec;

    return ns == 0 ? DISPATCH_TIME_FOREVER : ns;
}

/*
 * A partial emulation of the POSIX timer_create function.
 *
 * The limitations and differences include:
 *
 * - Only CLOCK_REALTIME and CLOCK_MONOTONIC are supported as clockid
 *   values.
 * - Even if CLOCK_REALTIME is specified, internally the mach_absolute_time
 *   source is used internally.
 * - The only notification method supported is SIGEV_THREAD.
 */
    inline int
timer_create(clockid_t clockid, struct sigevent *sevp, timer_t *timerid)
{
    macos_timer_T *timer = NULL;

    // We only support real time and monotonic clocks; and SIGEV_THREAD
    // notification. In practice, there is no difference between the two
    // types of clocks on MacOS - we always use the mach_machine_time
    // source.
    if (   (clockid != CLOCK_REALTIME && clockid != CLOCK_MONOTONIC)
        || sevp->sigev_notify != SIGEV_THREAD)
    {
	semsg("clockid: %d %d", clockid, CLOCK_REALTIME);
	semsg("notify:  %d %d", sevp->sigev_notify, SIGEV_THREAD);
	errno = ENOTSUP;
	return -1;
    }

    timer = (macos_timer_T *)malloc(sizeof(macos_timer_T));
    if (timer == NULL)
    {
	errno = ENOMEM;
	return -1;
    }
    *timerid = timer;

    timer->tim_queue = dispatch_queue_create(
	    "org.vim.timerqueue", NULL);
    if (timer->tim_queue == NULL)
    {
	errno = ENOMEM;
	return -1;
    }

    timer->tim_timer = dispatch_source_create(
	    DISPATCH_SOURCE_TYPE_TIMER, 0, 0, timer->tim_queue);
    if (timer->tim_timer == NULL)
    {
	errno = ENOMEM;
	return -1;
    }

    timer->tim_func = sevp->sigev_notify_function;
    timer->tim_arg = sevp->sigev_value.sival_ptr;

    dispatch_set_context(timer->tim_timer, timer);
    dispatch_source_set_event_handler_f(timer->tim_timer, _timer_handler);
    dispatch_source_set_cancel_handler_f(timer->tim_timer, _timer_cancel);

    dispatch_resume(timer->tim_timer);

    return 0;
}

/*
 * A partial emulation of the POSIX timer_settime function.
 *
 * The limitations and differences include:
 *
 * - The flags argument is ignored. The supplied new_value is therefore
 *   always treated as a relative time.
 * - The old_value argument is ignored.
 */
    int
timer_settime(
    timer_t timerid,
    int unused_flags UNUSED,
    const struct itimerspec *new_value,
    struct itimerspec *old_value UNUSED)
{
    uint64_t first_shot = itime_to_ns(&new_value->it_value);

    if (timerid == NULL)
	return 0;

    if (first_shot == DISPATCH_TIME_FOREVER)
    {
	dispatch_source_set_timer(
	    timerid->tim_timer, first_shot, first_shot, 0);
    }
    else
    {
	uint64_t interval = itime_to_ns(&new_value->it_interval);

	dispatch_time_t start = dispatch_time(DISPATCH_TIME_NOW, first_shot);
	dispatch_source_set_timer(timerid->tim_timer, start, interval, 0);
    }

    return 0;
}

/*
 * An emulation of the POSIX timer_delete function.
 *
 * Disabled because it is not currently used, but an implemented provided
 * for completeness and possible future use.
 */
    int
timer_delete(timer_t timerid)
{
    /* Calls _timer_cancel() */
    if (timerid != NULL)
	dispatch_source_cancel(timerid->tim_timer);

    return 0;
}

#endif /* FEAT_RELTIME */

#ifdef FEAT_SOUND

static NSMutableDictionary<NSNumber*, NSSound*> *sounds_list = nil;

/// A delegate for handling when a sound has stopped playing, in
/// order to clean up the sound and to send a callback.
@interface SoundDelegate : NSObject<NSSoundDelegate>;

- (id) init:(long) sound_id callback:(soundcb_T*) callback;
- (void) sound:(NSSound *)sound didFinishPlaying:(BOOL)flag;

@property (readonly) long sound_id;
@property (readonly) soundcb_T *callback;

@end

@implementation SoundDelegate
- (id) init:(long) sound_id callback:(soundcb_T*) callback
{
    if ([super init])
    {
	_sound_id = sound_id;
	_callback = callback;
    }
    return self;
}

- (void) sound:(NSSound *)sound didFinishPlaying:(BOOL)flag
{
    if (sounds_list != nil)
    {
	if (_callback)
	{
	    call_sound_callback(_callback, _sound_id, flag ? 0 : 1);
	    delete_sound_callback(_callback);
	    _callback = NULL;
	}
	[sounds_list removeObjectForKey:[NSNumber numberWithLong:_sound_id]];
    }
    // Release itself. Do that here instead of earlier because NSSound only
    // holds weak reference to this object.
    [self release];
}
@end

    void
process_cfrunloop(void)
{
    if (sounds_list != nil && [sounds_list count] > 0)
    {
	// Continually drain the run loop of events. Currently, this
	// is only used for processing sound callbacks, because
	// NSSound relies of this runloop to call back to the
	// delegate.
	@autoreleasepool
	{
	    while (CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0, true)
		    == kCFRunLoopRunHandledSource)
		;   // do nothing
	}
    }
}

    bool
sound_mch_play(const char_u* sound_name, long sound_id, soundcb_T *callback, bool playfile)
{
    @autoreleasepool
    {
	NSString *sound_name_ns = [[[NSString alloc] initWithUTF8String:(const char*)sound_name] autorelease];
	NSSound* sound = playfile ?
	    [[[NSSound alloc] initWithContentsOfFile:sound_name_ns byReference:YES] autorelease] :
	    [NSSound soundNamed:sound_name_ns];
	if (!sound)
	{
	    return false;
	}

	if (sounds_list == nil)
	{
	    sounds_list = [[NSMutableDictionary<NSNumber*, NSSound*> alloc] init];
	}
	sounds_list[[NSNumber numberWithLong:sound_id]] = sound;

	// Make a delegate to handle when the sound stops. No need to call
	// autorelease because NSSound only holds a weak reference to it.
	SoundDelegate *delegate = [[SoundDelegate alloc] init:sound_id callback:callback];

	[sound setDelegate:delegate];
	[sound play];
    }
    return true;
}

    void
sound_mch_stop(long sound_id)
{
    @autoreleasepool
    {
	NSSound *sound = sounds_list[[NSNumber numberWithLong:sound_id]];
	if (sound != nil)
	{
	    // Stop the sound. No need to release it because the delegate will do
	    // it for us.
	    [sound stop];
	}
    }
}

    void
sound_mch_clear(void)
{
    if (sounds_list != nil)
    {
	@autoreleasepool
	{
	    for (NSSound *sound in [sounds_list allValues])
	    {
		[sound stop];
	    }
	    [sounds_list release];
	    sounds_list = nil;
	}
    }
}

    void
sound_mch_free(void)
{
    sound_mch_clear();
}

#endif // FEAT_SOUND

/* Lift the compiler warning suppression. */
#if defined(__clang__) && defined(__STRICT_ANSI__)
# pragma clang diagnostic pop
# pragma clang diagnostic pop
#endif
