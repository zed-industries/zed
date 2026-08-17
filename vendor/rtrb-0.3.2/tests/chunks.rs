use rtrb::{chunks::ChunkError, RingBuffer};

#[test]
fn iterators() {
    let (mut p, mut c) = RingBuffer::new(3);
    if let Ok(chunk) = p.write_chunk_uninit(3) {
        chunk.fill_from_iter([10, 11]);
    } else {
        unreachable!();
    }
    assert_eq!(c.slots(), 2);
    if let Ok(chunk) = c.read_chunk(2) {
        let mut iter = chunk.into_iter();
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next().unwrap(), 10);
        assert_eq!(iter.len(), 1);
    }
    assert_eq!(c.slots(), 1);
    if let Ok(chunk) = p.write_chunk_uninit(p.slots()) {
        chunk.fill_from_iter([20, 21]);
    } else {
        unreachable!();
    }
    if let Ok(chunk) = c.read_chunk(c.slots()) {
        let mut iter = chunk.into_iter();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), 11);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next().unwrap(), 20);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next().unwrap(), 21);
        assert_eq!(iter.len(), 0);
        assert!(iter.next().is_none());
        for _ in 0..1000 {
            // FusedIterator continues to yield None:
            assert!(iter.next().is_none());
        }
    }
}

#[test]
fn zero_capacity() {
    let (mut p, mut c) = RingBuffer::<i32>::new(0);

    assert_eq!(p.write_chunk(1).unwrap_err(), ChunkError::TooFewSlots(0));
    assert_eq!(c.read_chunk(1).unwrap_err(), ChunkError::TooFewSlots(0));

    if let Ok(mut chunk) = p.write_chunk(0) {
        assert_eq!(chunk.len(), 0);
        assert!(chunk.is_empty());
        let (first, second) = chunk.as_mut_slices();
        assert!(first.is_empty());
        assert!(second.is_empty());
        chunk.commit_all();
    } else {
        unreachable!();
    }

    if let Ok(chunk) = c.read_chunk(0) {
        assert_eq!(chunk.len(), 0);
        assert!(chunk.is_empty());
        let (first, second) = chunk.as_slices();
        assert!(first.is_empty());
        assert!(second.is_empty());
        chunk.commit_all();
    } else {
        unreachable!();
    }
}

#[test]
fn single_capacity() {
    let (mut p, mut c) = RingBuffer::<i32>::new(1);

    if let Ok(mut chunk) = p.write_chunk(1) {
        assert_eq!(chunk.len(), 1);
        assert!(!chunk.is_empty());
        let (first, second) = chunk.as_mut_slices();
        first[0] = 2;
        assert!(second.is_empty());
        chunk.commit_all();
    } else {
        unreachable!();
    }

    if let Ok(mut chunk) = c.read_chunk(1) {
        assert_eq!(chunk.len(), 1);
        assert!(!chunk.is_empty());
        {
            let (first, second) = chunk.as_mut_slices();
            assert_eq!(first[0], 2);
            first[0] *= 2;
            assert!(second.is_empty());
        }
        {
            let (first, second) = chunk.as_slices();
            assert_eq!(first[0], 4);
            assert!(second.is_empty());
        }
        chunk.commit_all();
    } else {
        unreachable!();
    }
}

#[test]
fn drop_write_chunk() {
    // Static variable to count all drop() invocations
    static mut DROP_COUNT: i32 = 0;

    #[derive(Default)]
    struct Thing;

    impl Drop for Thing {
        fn drop(&mut self) {
            unsafe {
                DROP_COUNT += 1;
            }
        }
    }

    {
        let (mut p, mut c) = RingBuffer::new(3);

        if let Ok(mut chunk) = p.write_chunk(3) {
            let (first, _second) = chunk.as_mut_slices();
            assert_eq!(unsafe { DROP_COUNT }, 0);
            first[0] = Thing;
            // Overwriting drops the original Default element:
            assert_eq!(unsafe { DROP_COUNT }, 1);
            chunk.commit(1);
            // After committing, 2 (unwritten) slots are dropped
            assert_eq!(unsafe { DROP_COUNT }, 3);
        } else {
            unreachable!();
        }

        let chunk = c.read_chunk(1).unwrap();
        // Drop count is unchanged:
        assert_eq!(unsafe { DROP_COUNT }, 3);
        chunk.commit_all();
        // The stored element is never read, but it is dropped:
        assert_eq!(unsafe { DROP_COUNT }, 4);

        let chunk = p.write_chunk(3).unwrap();
        // Drop count is unchanged:
        assert_eq!(unsafe { DROP_COUNT }, 4);
        drop(chunk);
        // All three slots of the chunk are dropped:
        assert_eq!(unsafe { DROP_COUNT }, 7);
    }
    // RingBuffer was already empty, nothing is dropped:
    assert_eq!(unsafe { DROP_COUNT }, 7);
}

#[test]
fn trait_impls() {
    let (mut p, mut c) = RingBuffer::<u8>::new(0);

    if let Ok(chunk) = p.write_chunk(0) {
        assert!(format!("{:?}", chunk).starts_with("WriteChunk"));
    } else {
        unreachable!();
    }
    if let Ok(chunk) = p.write_chunk_uninit(0) {
        assert!(format!("{:?}", chunk).starts_with("WriteChunkUninit"));
    } else {
        unreachable!();
    }
    if let Ok(chunk) = c.read_chunk(0) {
        assert!(format!("{:?}", chunk).starts_with("ReadChunk"));
        let iter = chunk.into_iter();
        assert!(format!("{:?}", iter).starts_with("ReadChunkIntoIter"));
    } else {
        unreachable!();
    }
    let e = c.read_chunk(100).unwrap_err();
    assert_eq!(format!("{:?}", e), "TooFewSlots(0)");
    assert_eq!(e.to_string(), "only 0 slots available in ring buffer");
}
