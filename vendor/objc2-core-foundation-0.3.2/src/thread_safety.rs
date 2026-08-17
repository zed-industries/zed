//! Thread-safety.

use crate::*;

// SAFETY: CFBundle is immutable, and can be retrieved from any thread.
#[cfg(feature = "CFBundle")]
unsafe impl Send for CFBundle {}
#[cfg(feature = "CFBundle")]
unsafe impl Sync for CFBundle {}

// SAFETY: `NSNotificationCenter` and `NSDistributedNotificationCenter` are
// thread-safe, and those build upon this, so it must be thread-safe too.
//
// Additionally, they can be retrieved from any thread.
//
// NOTE: added notification observers are sent to the main thread, so the
// functions for doing that are not necessarily safe.
#[cfg(feature = "CFNotificationCenter")]
unsafe impl Send for CFNotificationCenter {}
#[cfg(feature = "CFNotificationCenter")]
unsafe impl Sync for CFNotificationCenter {}

// SAFETY: CFUUID is immutable and just stores the 16 bytes.
#[cfg(feature = "CFUUID")]
unsafe impl Send for CFUUID {}
#[cfg(feature = "CFUUID")]
unsafe impl Sync for CFUUID {}

// SAFETY: NSNumber is thread-safe, and this is toll-free bridged to that.
#[cfg(feature = "CFNumber")]
unsafe impl Send for CFBoolean {}
#[cfg(feature = "CFNumber")]
unsafe impl Sync for CFBoolean {}
#[cfg(feature = "CFNumber")]
unsafe impl Send for CFNumber {}
#[cfg(feature = "CFNumber")]
unsafe impl Sync for CFNumber {}

// SAFETY: NSDate is thread-safe, and this is toll-free bridged to that.
#[cfg(feature = "CFDate")]
unsafe impl Send for CFDate {}
#[cfg(feature = "CFDate")]
unsafe impl Sync for CFDate {}

// SAFETY: NSError is thread-safe, and this is toll-free bridged to that.
// Also, Foundation's .swiftinterface adds a `@unchecked Swift.Sendable`
// protocol conformance to CoreFoundation.CFError.
#[cfg(feature = "CFError")]
unsafe impl Send for CFError {}
#[cfg(feature = "CFError")]
unsafe impl Sync for CFError {}

// SAFETY: NSLocale is thread-safe, and this is toll-free bridged to that.
#[cfg(feature = "CFLocale")]
unsafe impl Send for CFLocale {}
#[cfg(feature = "CFLocale")]
unsafe impl Sync for CFLocale {}

// SAFETY: NSNull is thread-safe, and this is toll-free bridged to that.
unsafe impl Send for CFNull {}
unsafe impl Sync for CFNull {}

// SAFETY: NSTimeZone is thread-safe, and this is toll-free bridged to that.
#[cfg(feature = "CFDate")]
unsafe impl Send for CFTimeZone {}
#[cfg(feature = "CFDate")]
unsafe impl Sync for CFTimeZone {}

// SAFETY: NSURL is thread-safe, and this is toll-free bridged to that.
#[cfg(feature = "CFURL")]
unsafe impl Send for CFURL {}
#[cfg(feature = "CFURL")]
unsafe impl Sync for CFURL {}

#[allow(unused_macros)]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! not_thread_safe {
        ($($ty:ty),*) => {$(
            static_assertions::assert_not_impl_any!($ty: Send, Sync);
        )?};
    }

    macro_rules! thread_safe {
        ($($ty:ty),*) => {$(
            // General assumption: Assumes that the allocator this was created
            // with is also thread-safe; that's probably a fair assumption,
            // otherwise nothing in CF would be thread-safe.
            static_assertions::assert_impl_all!($ty: Send, Sync);
        )?};
    }

    #[test]
    fn base() {
        // CFType can hold thread-unsafe objects.
        // Just like AnyObject and NSObject aren't thread-safe.
        not_thread_safe!(CFType, CFPropertyList);

        // Only the built-ins are thread-safe, other allocators aren't
        // guaranteed to be. Usually fine though, since they're placed in
        // statics anyhow, and can thus already be accessed from all threads.
        not_thread_safe!(CFAllocator);
    }

    /// Collections, mutable types and immutable types with mutable variants
    /// aren't thread-safe:
    /// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-SW9>
    ///
    /// Note that things that are known to be the immutable variant are
    /// usually thread-safe (which is why e.g. `CFString` in statics is fine).
    #[test]
    fn mutable_or_mutable_variant() {
        #[cfg(feature = "CFBag")]
        not_thread_safe!(CFBag<u32>, CFMutableBag<u32>);
        #[cfg(feature = "CFBinaryHeap")]
        not_thread_safe!(CFBinaryHeap);
        #[cfg(feature = "CFBitVector")]
        not_thread_safe!(CFBitVector, CFMutableBitVector);
        #[cfg(feature = "CFDateFormatter")]
        not_thread_safe!(CFDateFormatter); // Explicitly stated to not be thread-safe
        #[cfg(feature = "CFFileDescriptor")]
        not_thread_safe!(CFFileDescriptor);
        #[cfg(feature = "CFNumberFormatter")]
        not_thread_safe!(CFNumberFormatter); // Explicitly stated to not be thread-safe
        #[cfg(feature = "CFSocket")]
        not_thread_safe!(CFSocket);
        #[cfg(feature = "CFStringTokenizer")]
        not_thread_safe!(CFStringTokenizer);
        #[cfg(feature = "CFTree")]
        not_thread_safe!(CFTree);
        #[cfg(feature = "CFURLEnumerator")]
        not_thread_safe!(CFURLEnumerator);
        #[cfg(feature = "CFUserNotification")]
        not_thread_safe!(CFUserNotification);

        // Types are not marked as thread-safe if their toll-free
        // bridged type is not thread safe either.
        #[cfg(feature = "CFArray")]
        not_thread_safe!(CFArray<u32>, CFMutableArray<u32>);
        #[cfg(feature = "CFAttributedString")]
        not_thread_safe!(CFAttributedString, CFMutableAttributedString);
        #[cfg(feature = "CFCalendar")]
        not_thread_safe!(CFCalendar);
        #[cfg(feature = "CFCharacterSet")]
        not_thread_safe!(CFCharacterSet, CFMutableCharacterSet);
        #[cfg(feature = "CFData")]
        not_thread_safe!(CFData, CFMutableData);
        #[cfg(feature = "CFDictionary")]
        not_thread_safe!(CFDictionary<u32, u32>, CFMutableDictionary<u32, u32>);
        #[cfg(feature = "CFFileSecurity")]
        not_thread_safe!(CFFileSecurity);
        #[cfg(feature = "CFMachPort")]
        not_thread_safe!(CFMachPort);
        #[cfg(feature = "CFMessagePort")]
        not_thread_safe!(CFMessagePort);
        #[cfg(feature = "CFRunLoop")]
        not_thread_safe!(CFRunLoopTimer);
        #[cfg(feature = "CFSet")]
        not_thread_safe!(CFSet<u32>, CFMutableSet<u32>);
        #[cfg(feature = "CFString")]
        not_thread_safe!(CFString, CFMutableString);
        #[cfg(feature = "CFStream")]
        not_thread_safe!(CFReadStream, CFWriteStream);
    }

    /// Immutable CF types are generally thread-safe.
    #[test]
    fn immutable() {
        #[cfg(feature = "CFBundle")]
        thread_safe!(CFBundle);

        #[cfg(feature = "CFNotificationCenter")]
        thread_safe!(CFNotificationCenter);

        #[cfg(feature = "CFUUID")]
        thread_safe!(CFUUID);

        // Types whose toll-free bridged type is thread-safe are also marked
        // as thread-safe.

        #[cfg(feature = "CFNumber")]
        thread_safe!(CFBoolean, CFNumber);
        #[cfg(feature = "CFDate")]
        thread_safe!(CFDate);
        #[cfg(feature = "CFError")]
        thread_safe!(CFError);
        #[cfg(feature = "CFLocale")]
        thread_safe!(CFLocale);
        thread_safe!(CFNull);
        #[cfg(feature = "CFDate")]
        thread_safe!(CFTimeZone);
        #[cfg(feature = "CFURL")]
        thread_safe!(CFURL);
    }

    #[test]
    fn uncertain() {
        // Uncertain, so marked as thread-unsafe for now.
        #[cfg(feature = "CFBundle")]
        not_thread_safe!(CFPlugIn);
        #[cfg(feature = "CFPlugIn")]
        not_thread_safe!(CFPlugInInstance);

        // Under discussion in https://github.com/madsmtm/objc2/issues/696.
        #[cfg(feature = "CFRunLoop")]
        not_thread_safe!(CFRunLoop, CFRunLoopSource, CFRunLoopObserver);
    }
}
