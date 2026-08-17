// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::base::write::io::offset::AsyncOffsetWriter;

#[tokio::test]
async fn basic() {
    use futures_lite::io::AsyncWriteExt;
    use futures_lite::io::Cursor;

    let mut writer = AsyncOffsetWriter::new(Cursor::new(Vec::new()));
    assert_eq!(writer.offset(), 0);

    writer.write_all(b"Foo. Bar. Foo. Bar.").await.expect("failed to write data");
    assert_eq!(writer.offset(), 19);

    writer.write_all(b"Foo. Foo.").await.expect("failed to write data");
    assert_eq!(writer.offset(), 28);

    writer.write_all(b"Bar. Bar.").await.expect("failed to write data");
    assert_eq!(writer.offset(), 37);
}
