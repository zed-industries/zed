//! A parser for reading and writing the X Input Method protocol.
//!
//! This is intended to be used as a building block for higher level libraries. See the
//! [`xim`] crate for an example.
//!
//! [`xim`]: https://crates.io/crates/xim

#![allow(clippy::uninlined_format_args, clippy::needless_borrow)]
#![forbid(unsafe_code, future_incompatible)]
#![no_std]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

use alloc::vec::Vec;

pub mod attrs;
mod parser;

pub use parser::*;

pub fn write_extend_vec(f: impl XimWrite, out: &mut Vec<u8>) {
    let from = out.len();
    out.extend(core::iter::repeat(0).take(f.size()));
    f.write(&mut Writer::new(&mut out[from..]));
}

pub fn write_to_vec(f: impl XimWrite) -> Vec<u8> {
    let mut out: Vec<u8> = core::iter::repeat(0).take(f.size()).collect();
    f.write(&mut Writer::new(&mut out));
    out
}

#[cfg(test)]
mod tests {
    use crate::{parser::*, write_to_vec};
    use alloc::vec;
    use alloc::vec::Vec;
    use pretty_assertions::assert_eq;

    #[cfg(target_endian = "little")]
    #[test]
    fn read_connect_req() {
        let req: Request = read(b"\x01\x00\x00\x00\x6c\x00\x00\x00\x00\x00\x00\x00").unwrap();

        assert_eq!(
            req,
            Request::Connect {
                endian: Endian::Native,
                client_auth_protocol_names: vec![],
                client_minor_protocol_version: 0,
                client_major_protocol_version: 0,
            }
        );
    }

    #[test]
    fn read_open() {
        let req = read::<Request>(&[
            30, 0, 2, 0, 5, 101, 110, 95, 85, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
        .unwrap();
        assert_eq!(
            req,
            Request::Open {
                locale: "en_US".into(),
            }
        );
    }

    #[test]
    fn read_query() {
        let req = read::<Request>(&[
            40, 0, 5, 0, 0, 0, 13, 0, 12, 88, 73, 77, 95, 69, 88, 84, 95, 77, 79, 86, 69, 0, 0, 0,
        ])
        .unwrap();
        assert_eq!(
            req,
            Request::QueryExtension {
                input_method_id: 0,
                extensions: vec!["XIM_EXT_MOVE".into(),],
            }
        );
    }

    #[test]
    fn read_input_styles() {
        let styles: InputStyleList = read(&[1, 0, 0, 0, 4, 1, 0, 0]).unwrap();

        assert_eq!(
            styles,
            InputStyleList {
                styles: vec![InputStyle::PREEDIT_POSITION | InputStyle::STATUS_AREA]
            }
        );
    }

    #[test]
    fn commit() {
        let req = Request::Commit {
            input_method_id: 1,
            input_context_id: 1,
            data: CommitData::Chars {
                commited: xim_ctext::utf8_to_compound_text("맘"),
                syncronous: false,
            },
        };

        write_to_vec(req);
    }

    #[test]
    fn set_event_mask() {
        let req = Request::SetEventMask {
            input_method_id: 2,
            input_context_id: 1,
            forward_event_mask: 3,
            synchronous_event_mask: 4294967292,
        };
        let out = write_to_vec(&req);
        assert_eq!(
            out,
            [37, 0, 3, 0, 2, 0, 1, 0, 3, 0, 0, 0, 252, 255, 255, 255]
        );
        assert_eq!(req, read::<Request>(&out).unwrap());
    }

    #[test]
    fn attr_size() {
        let list = InputStyleList {
            styles: vec![InputStyle::PREEDIT_POSITION | InputStyle::STATUS_AREA],
        };

        assert_eq!(list.size(), 8);

        let attr = Attribute {
            id: 0,
            value: write_to_vec(list),
        };

        assert_eq!(attr.size(), 12);
    }

    #[test]
    fn im_reply() {
        let req = Request::GetImValuesReply {
            input_method_id: 3,
            im_attributes: vec![Attribute {
                id: 0,
                value: write_to_vec(InputStyleList {
                    styles: vec![InputStyle::PREEDIT_POSITION | InputStyle::STATUS_AREA],
                }),
            }],
        };

        let out = write_to_vec(&req);
        assert_eq!(req.size(), out.len());

        let new_req = read(&out).unwrap();

        assert_eq!(req, new_req);
    }

    #[test]
    fn spot_attr() {
        let value = [4, 0, 4, 0, 0, 0, 0, 0];

        let attr = read::<Attribute>(&value).unwrap();

        assert_eq!(attr.id, 4);
        assert_eq!(read::<Point>(&attr.value).unwrap(), Point { x: 0, y: 0 });
    }

    #[test]
    fn read_error() {
        let req: Request = read(&[
            20, 0, 7, 0, 2, 0, 1, 0, 3, 0, 2, 0, 16, 0, 0, 0, 105, 110, 118, 97, 108, 105, 100, 32,
            105, 109, 32, 115, 116, 121, 108, 101,
        ])
        .unwrap();

        assert_eq!(
            req,
            Request::Error {
                input_method_id: 2,
                input_context_id: 1,
                flag: ErrorFlag::INPUT_METHOD_ID_VALID | ErrorFlag::INPUT_CONTEXT_ID_VALID,
                code: ErrorCode::BadStyle,
                detail: "invalid im style".into(),
            }
        );
    }

    #[test]
    fn write_get_im_values() {
        let req = Request::GetImValues {
            input_method_id: 1,
            im_attributes: vec![0],
        };

        let out = write_to_vec(&req);

        assert_eq!(out.len(), req.size());
    }

    #[test]
    fn write_forward_event() {
        let req = Request::ForwardEvent {
            input_method_id: 0,
            input_context_id: 0,
            flag: ForwardEventFlag::empty(),
            serial_number: 0,
            xev: XEvent {
                response_type: 0,
                detail: 0,
                sequence: 0,
                time: 0,
                root: 0,
                event: 0,
                child: 0,
                root_x: 0,
                root_y: 0,
                event_x: 0,
                event_y: 0,
                state: 0,
                same_screen: false,
            },
        };
        assert_eq!(req.size(), 4 + 8 + 32);

        let out = write_to_vec(&req);
        assert!(out.starts_with(b"\x3c\x00\x0a\x00"));
    }

    #[test]
    fn write_create_ic() {
        let req = Request::CreateIc {
            input_method_id: 2,
            ic_attributes: Vec::new(),
        };
        let out = write_to_vec(&req);
        assert_eq!(out, b"\x32\x00\x01\x00\x02\x00\x00\x00");
    }

    #[test]
    fn write_connect_reply() {
        let req = Request::ConnectReply {
            server_minor_protocol_version: 0,
            server_major_protocol_version: 1,
        };
        let out = write_to_vec(&req);
        assert_eq!(out, b"\x02\x00\x01\x00\x01\x00\x00\x00");
    }

    const OPEN_REPLY: &[u8] = b"\x1f\x00\x59\x00\x01\x00\x18\x00\x00\x00\x0a\x00\x0f\x00\x71\x75\x65\x72\x79\x49\x6e\x70\x75\x74\x53\x74\x79\x6c\x65\x00\x00\x00\x44\x01\x00\x00\x01\x00\x03\x00\x0a\x00\x69\x6e\x70\x75\x74\x53\x74\x79\x6c\x65\x02\x00\x05\x00\x0c\x00\x63\x6c\x69\x65\x6e\x74\x57\x69\x6e\x64\x6f\x77\x00\x00\x03\x00\x05\x00\x0b\x00\x66\x6f\x63\x75\x73\x57\x69\x6e\x64\x6f\x77\x00\x00\x00\x04\x00\x03\x00\x0c\x00\x66\x69\x6c\x74\x65\x72\x45\x76\x65\x6e\x74\x73\x00\x00\x05\x00\xff\x7f\x11\x00\x70\x72\x65\x65\x64\x69\x74\x41\x74\x74\x72\x69\x62\x75\x74\x65\x73\x00\x06\x00\xff\x7f\x10\x00\x73\x74\x61\x74\x75\x73\x41\x74\x74\x72\x69\x62\x75\x74\x65\x73\x00\x00\x07\x00\x0d\x00\x07\x00\x66\x6f\x6e\x74\x53\x65\x74\x00\x00\x00\x08\x00\x0b\x00\x04\x00\x61\x72\x65\x61\x00\x00\x09\x00\x0b\x00\x0a\x00\x61\x72\x65\x61\x4e\x65\x65\x64\x65\x64\x0a\x00\x03\x00\x08\x00\x63\x6f\x6c\x6f\x72\x4d\x61\x70\x00\x00\x0b\x00\x03\x00\x0b\x00\x73\x74\x64\x43\x6f\x6c\x6f\x72\x4d\x61\x70\x00\x00\x00\x0c\x00\x03\x00\x0a\x00\x66\x6f\x72\x65\x67\x72\x6f\x75\x6e\x64\x0d\x00\x03\x00\x0a\x00\x62\x61\x63\x6b\x67\x72\x6f\x75\x6e\x64\x0e\x00\x03\x00\x10\x00\x62\x61\x63\x6b\x67\x72\x6f\x75\x6e\x64\x50\x69\x78\x6d\x61\x70\x00\x00\x0f\x00\x0c\x00\x0c\x00\x73\x70\x6f\x74\x4c\x6f\x63\x61\x74\x69\x6f\x6e\x00\x00\x10\x00\x03\x00\x09\x00\x6c\x69\x6e\x65\x53\x70\x61\x63\x65\x00\x11\x00\x00\x00\x15\x00\x73\x65\x70\x61\x72\x61\x74\x6f\x72\x6f\x66\x4e\x65\x73\x74\x65\x64\x4c\x69\x73\x74\x00";

    fn open_reply_value() -> Request {
        Request::OpenReply {
            input_method_id: 1,
            im_attrs: vec![Attr {
                id: 0,
                ty: AttrType::Style,
                name: AttributeName::QueryInputStyle,
            }],
            ic_attrs: vec![
                Attr {
                    id: 1,
                    ty: AttrType::Long,
                    name: AttributeName::InputStyle,
                },
                Attr {
                    id: 2,
                    ty: AttrType::Window,
                    name: AttributeName::ClientWindow,
                },
                Attr {
                    id: 3,
                    ty: AttrType::Window,
                    name: AttributeName::FocusWindow,
                },
                Attr {
                    id: 4,
                    ty: AttrType::Long,
                    name: AttributeName::FilterEvents,
                },
                Attr {
                    id: 5,
                    ty: AttrType::NestedList,
                    name: AttributeName::PreeditAttributes,
                },
                Attr {
                    id: 6,
                    ty: AttrType::NestedList,
                    name: AttributeName::StatusAttributes,
                },
                Attr {
                    id: 7,
                    ty: AttrType::XFontSet,
                    name: AttributeName::FontSet,
                },
                Attr {
                    id: 8,
                    ty: AttrType::XRectangle,
                    name: AttributeName::Area,
                },
                Attr {
                    id: 9,
                    ty: AttrType::XRectangle,
                    name: AttributeName::AreaNeeded,
                },
                Attr {
                    id: 10,
                    ty: AttrType::Long,
                    name: AttributeName::ColorMap,
                },
                Attr {
                    id: 11,
                    ty: AttrType::Long,
                    name: AttributeName::StdColorMap,
                },
                Attr {
                    id: 12,
                    ty: AttrType::Long,
                    name: AttributeName::Foreground,
                },
                Attr {
                    id: 13,
                    ty: AttrType::Long,
                    name: AttributeName::Background,
                },
                Attr {
                    id: 14,
                    ty: AttrType::Long,
                    name: AttributeName::BackgroundPixmap,
                },
                Attr {
                    id: 15,
                    ty: AttrType::XPoint,
                    name: AttributeName::SpotLocation,
                },
                Attr {
                    id: 16,
                    ty: AttrType::Long,
                    name: AttributeName::LineSpace,
                },
                Attr {
                    id: 17,
                    ty: AttrType::Separator,
                    name: AttributeName::SeparatorofNestedList,
                },
            ],
        }
    }

    #[test]
    fn read_open_reply() {
        assert_eq!(read::<Request>(OPEN_REPLY).unwrap(), open_reply_value());
    }

    #[test]
    fn size_open_reply() {
        assert_eq!(open_reply_value().size(), OPEN_REPLY.len());
    }

    #[test]
    fn write_open_reply() {
        let value = open_reply_value();
        let out = write_to_vec(&value);
        assert_eq!(OPEN_REPLY.len(), out.len());
        assert_eq!(OPEN_REPLY, out);
        let new: Request = read(&out).unwrap();
        assert_eq!(value, new);
    }
}
