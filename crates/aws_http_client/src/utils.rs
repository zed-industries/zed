use aws_smithy_types::body::SdkBody;
use futures::AsyncReadExt;
use http_client::{AsyncBody, Inner};
use tokio::runtime::Handle;

pub async fn convert_to_sdk_body(body: AsyncBody, handle: Handle) -> SdkBody {
    match body.0 {
        Inner::Empty => SdkBody::empty(),
        Inner::Bytes(b) => {
            let b = b.into_inner();
            SdkBody::from(b)
        }
        Inner::AsyncReader(mut reader) => {
            let buffer = handle.spawn(async move {
                let mut buffer = Vec::new();
                let _ = reader.read_to_end(&mut buffer).await;
                buffer
            });

            SdkBody::from(buffer.await.unwrap_or_default())
        }
    }
}

pub fn convert_to_async_body(body: SdkBody) -> AsyncBody {
    match body.bytes() {
        Some(bytes) => AsyncBody::from((*bytes).to_vec()),
        None => AsyncBody::empty(),
    }
}
