use cocoa::base::id;
use cocoa::foundation::NSRange;
use objc::{class, msg_send, sel, sel_impl};

/// The `cocoa` crate does not define NSAttributedString (and related Cocoa classes),
/// which are needed for copying rich text (that is, text intermingled with images)
/// to the clipboard. This adds access to those APIs.
#[allow(non_snake_case)]
pub trait NSAttributedString: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSAttributedString), alloc]
    }

    unsafe fn init_attributed_string(self, string: id) -> id;
    unsafe fn appendAttributedString_(self, attr_string: id);
    unsafe fn RTFDFromRange_documentAttributes_(self, range: NSRange, attrs: id) -> id;
    unsafe fn RTFFromRange_documentAttributes_(self, range: NSRange, attrs: id) -> id;
    unsafe fn string(self) -> id;
}

impl NSAttributedString for id {
    unsafe fn init_attributed_string(self, string: id) -> id {
        msg_send![self, initWithString: string]
    }

    unsafe fn appendAttributedString_(self, attr_string: id) {
        let _: () = msg_send![self, appendAttributedString: attr_string];
    }

    unsafe fn RTFDFromRange_documentAttributes_(self, range: NSRange, attrs: id) -> id {
        msg_send![self, RTFDFromRange: range documentAttributes: attrs]
    }

    unsafe fn RTFFromRange_documentAttributes_(self, range: NSRange, attrs: id) -> id {
        msg_send![self, RTFFromRange: range documentAttributes: attrs]
    }

    unsafe fn string(self) -> id {
        msg_send![self, string]
    }
}

pub trait NSMutableAttributedString: NSAttributedString {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSMutableAttributedString), alloc]
    }
}

impl NSMutableAttributedString for id {}

#[cfg(test)]
mod tests {
    use crate::platform::mac::ns_string;

    use super::*;
    use cocoa::appkit::NSImage;
    use cocoa::base::nil;
    use cocoa::foundation::NSAutoreleasePool;
    #[test]
    #[ignore] // This was SIGSEGV-ing on CI but not locally; need to investigate https://github.com/zed-industries/zed/actions/runs/10362363230/job/28684225486?pr=15782#step:4:1348
    fn test_nsattributed_string() {
        // TODO move these to parent module once it's actually ready to be used
        #[allow(non_snake_case)]
        pub trait NSTextAttachment: Sized {
            unsafe fn alloc(_: Self) -> id {
                msg_send![class!(NSTextAttachment), alloc]
            }
        }

        impl NSTextAttachment for id {}

        unsafe {
            let image: id = {
                let img: id = msg_send![class!(NSImage), alloc];
                let img: id = msg_send![img, initWithContentsOfFile: ns_string("test.jpeg")];
                let img: id = msg_send![img, autorelease];
                img
            };
            let _size = image.size();

            let string = ns_string("Test String");
            let attr_string = NSMutableAttributedString::alloc(nil)
                .init_attributed_string(string)
                .autorelease();
            let hello_string = ns_string("Hello World");
            let hello_attr_string = NSAttributedString::alloc(nil)
                .init_attributed_string(hello_string)
                .autorelease();
            attr_string.appendAttributedString_(hello_attr_string);

            let attachment: id = msg_send![NSTextAttachment::alloc(nil), autorelease];
            let _: () = msg_send![attachment, setImage: image];
            let image_attr_string =
                msg_send![class!(NSAttributedString), attributedStringWithAttachment: attachment];
            attr_string.appendAttributedString_(image_attr_string);

            let another_string = ns_string("Another String");
            let another_attr_string = NSAttributedString::alloc(nil)
                .init_attributed_string(another_string)
                .autorelease();
            attr_string.appendAttributedString_(another_attr_string);

            let _len: cocoa::foundation::NSUInteger = msg_send![attr_string, length];

            ///////////////////////////////////////////////////
            // pasteboard.clearContents();

            let rtfd_data = attr_string.RTFDFromRange_documentAttributes_(
                NSRange::new(0, msg_send![attr_string, length]),
                nil,
            );
            assert_ne!(rtfd_data, nil);
            // if rtfd_data != nil {
            //     pasteboard.setData_forType(rtfd_data, NSPasteboardTypeRTFD);
            // }

            // let rtf_data = attributed_string.RTFFromRange_documentAttributes_(
            //     NSRange::new(0, attributed_string.length()),
            //     nil,
            // );
            // if rtf_data != nil {
            //     pasteboard.setData_forType(rtf_data, NSPasteboardTypeRTF);
            // }

            // let plain_text = attributed_string.string();
            // pasteboard.setString_forType(plain_text, NSPasteboardTypeString);
        }
    }
}
