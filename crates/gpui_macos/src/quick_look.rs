use objc2::{
    ffi::{OBJC_ASSOCIATION_RETAIN_NONATOMIC, objc_getAssociatedObject, objc_setAssociatedObject},
    msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, AnyProtocol, Bool, ClassBuilder, Sel},
    sel,
};
use objc2_foundation::{NSInteger, NSURL};
use std::ptr;

#[link(name = "QuickLookUI", kind = "framework")]
unsafe extern "C" {}

// The view owns its file URL independently of MacWindowState. AppKit can query the
// responder chain synchronously while a GPUI window operation holds the state lock.
static PREVIEW_ITEM_KEY: u8 = 0;

fn set_item(view: &AnyObject, url: Option<&NSURL>) {
    // SAFETY: only the AppKit main thread accesses the association. The runtime
    // retains the URL until it is cleared or the owning view is deallocated.
    unsafe {
        objc_setAssociatedObject(
            ptr::from_ref(view).cast_mut(),
            (&raw const PREVIEW_ITEM_KEY).cast(),
            url.map_or(ptr::null_mut(), |url| ptr::from_ref(url).cast_mut().cast()),
            OBJC_ASSOCIATION_RETAIN_NONATOMIC,
        );
    }
}

fn item(view: &AnyObject) -> *const AnyObject {
    unsafe { objc_getAssociatedObject(view, (&raw const PREVIEW_ITEM_KEY).cast()) }
}

pub(super) fn register(view: &mut ClassBuilder) {
    // SAFETY: the signatures below match NSResponder's preview panel control
    // methods and the two required QLPreviewPanelDataSource methods.
    unsafe {
        if let Some(protocol) = AnyProtocol::get(c"QLPreviewPanelDataSource") {
            view.add_protocol(protocol);
        }
        view.add_method(
            sel!(acceptsPreviewPanelControl:),
            accepts as extern "C" fn(_, _, _) -> _,
        );
        view.add_method(
            sel!(beginPreviewPanelControl:),
            begin as extern "C" fn(_, _, _),
        );
        view.add_method(sel!(endPreviewPanelControl:), end as extern "C" fn(_, _, _));
        view.add_method(
            sel!(numberOfPreviewItemsInPreviewPanel:),
            count as extern "C" fn(_, _, _) -> _,
        );
        view.add_method(
            sel!(previewPanel:previewItemAtIndex:),
            item_at as extern "C" fn(_, _, _, _) -> _,
        );
    }
}

pub(super) fn show(view: &AnyObject, url: &NSURL) {
    let Some(panel_class) = AnyClass::get(c"QLPreviewPanel") else {
        log::warn!("Quick Look preview panel is unavailable");
        return;
    };
    // SAFETY: the caller provides a live GPUIView on the main thread. Quick Look
    // chooses the controller via the responder chain before requesting its data.
    unsafe {
        let window: *mut AnyObject = msg_send![view, window];
        if window.is_null() {
            return;
        }
        let is_key: bool = msg_send![window, isKeyWindow];
        if !is_key {
            return;
        }
        let panel: Retained<AnyObject> = msg_send![panel_class, sharedPreviewPanel];
        set_item(view, Some(url));
        let _: () = msg_send![&*panel, updateController];
        let _: () = msg_send![&*panel, makeKeyAndOrderFront: view];
        let controller: *const AnyObject = msg_send![&*panel, currentController];
        if ptr::eq(controller, view) {
            let _: () = msg_send![&*panel, reloadData];
        } else {
            set_item(view, None);
        }
    }
}

extern "C" fn accepts(view: &AnyObject, _: Sel, _: &AnyObject) -> Bool {
    Bool::new(!item(view).is_null())
}

extern "C" fn begin(view: &AnyObject, _: Sel, panel: &AnyObject) {
    unsafe { msg_send![panel, setDataSource: view] }
}

extern "C" fn end(view: &AnyObject, _: Sel, panel: &AnyObject) {
    unsafe {
        let _: () = msg_send![panel, setDataSource: ptr::null_mut::<AnyObject>()];
    }
    set_item(view, None);
}

extern "C" fn count(view: &AnyObject, _: Sel, _: &AnyObject) -> NSInteger {
    NSInteger::from(!item(view).is_null())
}

extern "C" fn item_at(view: &AnyObject, _: Sel, _: &AnyObject, index: NSInteger) -> *mut AnyObject {
    if index != 0 {
        return ptr::null_mut();
    }
    // SAFETY: the association holds a live NSURL (a QLPreviewItem) or nil.
    unsafe { Retained::retain(item(view).cast_mut()) }
        .map_or(ptr::null_mut(), Retained::autorelease_return)
}

#[cfg(test)]
mod tests {
    use super::*;
    use objc2::{rc::autoreleasepool, runtime::NSObject};

    #[test]
    fn quick_look_keeps_preview_items_scoped_to_their_owner() {
        autoreleasepool(|_| {
            let first = NSObject::new();
            let second = NSObject::new();
            let url = NSURL::from_file_path("/tmp/a #1.txt").unwrap();
            set_item(&first, Some(&url));
            let item_selector = sel!(previewPanel:previewItemAtIndex:);
            assert_eq!(
                item_at(&first, item_selector, &first, 0).cast_const(),
                ptr::from_ref(&*url).cast()
            );
            assert!(item_at(&first, item_selector, &first, 1).is_null());
            assert!(item(&second).is_null());

            set_item(&first, None);
            assert!(item_at(&first, item_selector, &first, 0).is_null());
        });
    }
}
