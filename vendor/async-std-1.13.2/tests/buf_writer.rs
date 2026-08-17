use async_std::io::{self, BufWriter, SeekFrom};
use async_std::prelude::*;
use async_std::task;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn test_buffered_writer() {
    #![allow(clippy::cognitive_complexity)]
    task::block_on(async {
        let inner: Vec<u8> = Vec::new();
        let mut writer = BufWriter::<Vec<u8>>::with_capacity(2, inner);

        writer.write(&[0, 1]).await.unwrap();
        assert!(writer.buffer().is_empty());
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.write(&[2]).await.unwrap();
        assert_eq!(writer.buffer(), [2]);
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.write(&[3]).await.unwrap();
        assert_eq!(writer.buffer(), [2, 3]);
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.flush().await.unwrap();
        assert!(writer.buffer().is_empty());
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

        writer.write(&[4]).await.unwrap();
        writer.write(&[5]).await.unwrap();
        assert_eq!(writer.buffer(), [4, 5]);
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

        writer.write(&[6]).await.unwrap();
        assert_eq!(writer.buffer(), [6]);
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5]);

        writer.write(&[7, 8]).await.unwrap();
        assert!(writer.buffer().is_empty());
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        writer.write(&[9, 10, 11]).await.unwrap();
        assert!(writer.buffer().is_empty());
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

        writer.flush().await.unwrap();
        assert!(writer.buffer().is_empty());
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn test_buffered_writer_inner_into_inner_flushes() {
    task::block_on(async {
        let mut w = BufWriter::with_capacity(3, Vec::<u8>::new());
        w.write(&[0, 1]).await.unwrap();
        assert!(w.get_ref().is_empty());
        let w = w.into_inner().await.unwrap();
        assert_eq!(w, [0, 1]);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn test_buffered_writer_seek() {
    task::block_on(async {
        let mut w = BufWriter::with_capacity(3, io::Cursor::new(Vec::new()));
        w.write_all(&[0, 1, 2, 3, 4, 5]).await.unwrap();
        w.write_all(&[6, 7]).await.unwrap();
        assert_eq!(w.seek(SeekFrom::Current(0)).await.ok(), Some(8));
        assert_eq!(&w.get_ref().get_ref()[..], &[0, 1, 2, 3, 4, 5, 6, 7][..]);
        assert_eq!(w.seek(SeekFrom::Start(2)).await.ok(), Some(2));
    })
}
