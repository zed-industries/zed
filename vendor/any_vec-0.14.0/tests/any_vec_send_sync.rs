use std::iter;
use impls::impls;
use any_vec::{AnyVec, AnyVecMut, IterMut, IterRef, mem};
use any_vec::any_value::AnyValueWrapper;
use any_vec::mem::MemBuilder;
use any_vec::ops::Drain;
use any_vec::ops::Splice;
use any_vec::traits::None;

const fn is_send(_: &impl Send){}
const fn is_sync(_: &impl Sync){}

#[test]
fn any_vec_heap_send_sync_test() {
    fn test_negative<M: MemBuilder + Default>()
    {
        let mut any_vec: AnyVec<dyn None, M> = AnyVec::new::<String>();
        assert!(!impls!(AnyVec<dyn None, M>: Send));
        assert!(!impls!(AnyVec<dyn None, M>: Sync));
        {
            let iter: IterRef<dyn None, M> = any_vec.iter();
            assert!(!impls!(IterRef<dyn None, M>: Send));
            assert!(!impls!(IterRef<dyn None, M>: Sync));
            drop(iter);
        }
        {
            let iter: IterMut<dyn None, M> = any_vec.iter_mut();
            assert!(!impls!(IterMut<dyn None, M>: Send));
            assert!(!impls!(IterMut<dyn None, M>: Sync));
            drop(iter);
        }

        {
            let vec: AnyVecMut<String, M> = any_vec.downcast_mut::<String>().unwrap();
            assert!(!impls!(AnyVecMut<String, M>: Send));
            assert!(!impls!(AnyVecMut<String, M>: Sync));
            drop(vec);
        }

        {
            let drained: Drain<dyn None, M> = any_vec.drain(..);
            assert!(!impls!(AnyVec<dyn None, M>: Send));
            assert!(!impls!(AnyVec<dyn None, M>: Sync));
            drop(drained);
        }
        {
            let drained: Splice<dyn None, M, iter::Empty::<AnyValueWrapper<String>>> =
                any_vec.splice(.., iter::empty::<AnyValueWrapper<String>>());
            assert!(!impls!(Splice<dyn None, M, iter::Empty::<AnyValueWrapper<String>>>: Send));
            assert!(!impls!(Splice<dyn None, M, iter::Empty::<AnyValueWrapper<String>>>: Sync));
            drop(drained);
        }
    }

    fn test_positive<M: MemBuilder + Default + Sync + Send>()
        where M::Mem: Sync + Send
    {
        let mut any_vec: AnyVec<dyn Sync + Send, M> = AnyVec::new::<String>();
        is_sync(&any_vec);
        is_send(&any_vec);
        is_sync(&any_vec.iter());
        is_send(&any_vec.iter());
        is_sync(&any_vec.iter_mut());
        is_send(&any_vec.iter_mut());
        {
            let mut vec = any_vec.downcast_mut::<String>().unwrap();
            is_sync(&vec);
            is_send(&vec);
            is_sync(&vec.iter());
            is_send(&vec.iter());
            is_sync(&vec.iter_mut());
            is_send(&vec.iter_mut());
        }

        {
            let drained = any_vec.drain(..);
            is_sync(&drained);
            is_send(&drained);
        }
        {
            let drained = any_vec.splice(.., [AnyValueWrapper::new(String::new())]);
            is_sync(&drained);
            is_send(&drained);
        }
    }
    test_positive::<mem::Heap>();
    test_positive::<mem::Stack<123>>();

    test_negative::<mem::Heap>();
    test_negative::<mem::Stack<123>>();
}
