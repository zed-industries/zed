use aws_smithy_types::body::SdkBody;
use http_client::{AsyncBody, Inner};

pub fn convert_to_sdk_body(body: AsyncBody) -> SdkBody {
    match body.0 {
        Inner::Empty => SdkBody::empty(),
        Inner::Bytes(b) => {
            let b = b.into_inner();
            SdkBody::from(b)
        },
        Inner::AsyncReader(_) => unimplemented!(),
    }
}

pub fn convert_to_async_body(body: SdkBody) -> AsyncBody {
    match body.bytes() {
        Some(bytes) => AsyncBody::from((*bytes).to_vec()),
        None => AsyncBody::empty(),
    }
}
