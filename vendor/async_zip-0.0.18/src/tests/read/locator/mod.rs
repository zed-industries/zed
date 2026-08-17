// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

#[test]
fn search_one_byte_test() {
    let buffer: &[u8] = &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x1];

    let matched = crate::base::read::io::locator::reverse_search_buffer(buffer, signature);
    assert!(matched.is_none());

    let buffer: &[u8] = &[0x2, 0x1, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x1];

    let matched = crate::base::read::io::locator::reverse_search_buffer(buffer, signature);
    assert!(matched.is_some());
    assert_eq!(1, matched.unwrap());
}

#[test]
fn search_two_byte_test() {
    let buffer: &[u8] = &[0x2, 0x1, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x2, 0x1];

    let matched = crate::base::read::io::locator::reverse_search_buffer(buffer, signature);
    assert!(matched.is_some());
    assert_eq!(1, matched.unwrap());
}

#[tokio::test]
async fn locator_empty_test() {
    use futures_lite::io::Cursor;

    let data = &include_bytes!("empty.zip");
    let mut cursor = Cursor::new(data);
    let eocdr = crate::base::read::io::locator::eocdr(&mut cursor).await;

    assert!(eocdr.is_ok());
    assert_eq!(eocdr.unwrap(), 4);
}

#[tokio::test]
async fn locator_empty_max_comment_test() {
    use futures_lite::io::Cursor;

    let data = &include_bytes!("empty-with-max-comment.zip");
    let mut cursor = Cursor::new(data);
    let eocdr = crate::base::read::io::locator::eocdr(&mut cursor).await;

    assert!(eocdr.is_ok());
    assert_eq!(eocdr.unwrap(), 4);
}

#[tokio::test]
async fn locator_buffer_boundary_test() {
    use futures_lite::io::Cursor;

    let data = &include_bytes!("empty-buffer-boundary.zip");
    let mut cursor = Cursor::new(data);
    let eocdr = crate::base::read::io::locator::eocdr(&mut cursor).await;

    assert!(eocdr.is_ok());
    assert_eq!(eocdr.unwrap(), 4);
}
