use super::{arc::MiniArc, KeyHashDate, ValueEntry};
use crate::common::{
    deque::{DeqNode, Deque},
    CacheRegion,
};

use std::ptr::NonNull;
use tagptr::TagNonNull;

pub(crate) struct Deques<K> {
    pub(crate) window: Deque<KeyHashDate<K>>, //    Not used yet.
    pub(crate) probation: Deque<KeyHashDate<K>>,
    pub(crate) protected: Deque<KeyHashDate<K>>, // Not used yet.
    pub(crate) write_order: Deque<KeyHashDate<K>>,
}

#[cfg(feature = "future")]
// TODO: https://github.com/moka-rs/moka/issues/54
#[allow(clippy::non_send_fields_in_send_ty)]
// Multi-threaded async runtimes require base_cache::Inner to be Send, but it will
// not be without this `unsafe impl`. This is because DeqNodes have NonNull
// pointers.
unsafe impl<K> Send for Deques<K> {}

impl<K> Default for Deques<K> {
    fn default() -> Self {
        Self {
            window: Deque::new(CacheRegion::Window),
            probation: Deque::new(CacheRegion::MainProbation),
            protected: Deque::new(CacheRegion::MainProtected),
            write_order: Deque::new(CacheRegion::Other),
        }
    }
}

impl<K> Deques<K> {
    pub(crate) fn select_mut(
        &mut self,
        selector: CacheRegion,
    ) -> (&mut Deque<KeyHashDate<K>>, &mut Deque<KeyHashDate<K>>) {
        match selector {
            CacheRegion::Window => (&mut self.window, &mut self.write_order),
            CacheRegion::MainProbation => (&mut self.probation, &mut self.write_order),
            CacheRegion::MainProtected => (&mut self.protected, &mut self.write_order),
            CacheRegion::Other => unreachable!(),
        }
    }

    pub(crate) fn push_back_ao<V>(
        &mut self,
        region: CacheRegion,
        khd: KeyHashDate<K>,
        entry: &MiniArc<ValueEntry<K, V>>,
    ) {
        let node = Box::new(DeqNode::new(khd));
        let node = match region {
            CacheRegion::Window => self.window.push_back(node),
            CacheRegion::MainProbation => self.probation.push_back(node),
            CacheRegion::MainProtected => self.protected.push_back(node),
            CacheRegion::Other => unreachable!(),
        };
        let tagged_node = TagNonNull::compose(node, region as usize);
        entry.set_access_order_q_node(Some(tagged_node));
    }

    pub(crate) fn push_back_wo<V>(
        &mut self,
        kd: KeyHashDate<K>,
        entry: &MiniArc<ValueEntry<K, V>>,
    ) {
        let node = Box::new(DeqNode::new(kd));
        let node = self.write_order.push_back(node);
        entry.set_write_order_q_node(Some(node));
    }

    pub(crate) fn move_to_back_ao<V>(&mut self, entry: &MiniArc<ValueEntry<K, V>>) {
        if let Some(tagged_node) = entry.access_order_q_node() {
            let (node, tag) = tagged_node.decompose();
            let p = unsafe { node.as_ref() };
            match tag.into() {
                CacheRegion::Window if self.window.contains(p) => {
                    unsafe { self.window.move_to_back(node) };
                }
                CacheRegion::MainProbation if self.probation.contains(p) => {
                    unsafe { self.probation.move_to_back(node) };
                }
                CacheRegion::MainProtected if self.protected.contains(p) => {
                    unsafe { self.protected.move_to_back(node) };
                }
                _ => unreachable!(),
            }
        }
    }

    pub(crate) fn move_to_back_ao_in_deque<V>(
        deq_name: &str,
        deq: &mut Deque<KeyHashDate<K>>,
        entry: &MiniArc<ValueEntry<K, V>>,
    ) {
        if let Some(tagged_node) = entry.access_order_q_node() {
            let (node, tag) = tagged_node.decompose();
            let p = unsafe { node.as_ref() };
            assert_eq!(
                deq.region(),
                tag,
                "move_to_back_ao_in_deque - node is not a member of {deq_name} deque. {p:?}"
            );
            if deq.contains(p) {
                unsafe { deq.move_to_back(node) };
            }
        }
    }

    pub(crate) fn move_to_back_wo<V>(&mut self, entry: &MiniArc<ValueEntry<K, V>>) {
        if let Some(node) = entry.write_order_q_node() {
            let p = unsafe { node.as_ref() };
            if self.write_order.contains(p) {
                unsafe { self.write_order.move_to_back(node) };
            }
        }
    }

    pub(crate) fn move_to_back_wo_in_deque<V>(
        deq: &mut Deque<KeyHashDate<K>>,
        entry: &MiniArc<ValueEntry<K, V>>,
    ) {
        if let Some(node) = entry.write_order_q_node() {
            let p = unsafe { node.as_ref() };
            if deq.contains(p) {
                unsafe { deq.move_to_back(node) };
            }
        }
    }

    pub(crate) fn unlink_ao<V>(&mut self, entry: &MiniArc<ValueEntry<K, V>>) {
        if let Some(node) = entry.take_access_order_q_node() {
            self.unlink_node_ao(node);
        }
    }

    pub(crate) fn unlink_ao_from_deque<V>(
        deq_name: &str,
        deq: &mut Deque<KeyHashDate<K>>,
        entry: &MiniArc<ValueEntry<K, V>>,
    ) {
        if let Some(node) = entry.take_access_order_q_node() {
            unsafe { Self::unlink_node_ao_from_deque(deq_name, deq, node) };
        }
    }

    pub(crate) fn unlink_wo<V>(deq: &mut Deque<KeyHashDate<K>>, entry: &MiniArc<ValueEntry<K, V>>) {
        if let Some(node) = entry.take_write_order_q_node() {
            Self::unlink_node_wo(deq, node);
        }
    }

    pub(crate) fn unlink_node_ao(&mut self, tagged_node: TagNonNull<DeqNode<KeyHashDate<K>>, 2>) {
        unsafe {
            match tagged_node.decompose_tag().into() {
                CacheRegion::Window => {
                    Self::unlink_node_ao_from_deque("window", &mut self.window, tagged_node);
                }
                CacheRegion::MainProbation => {
                    Self::unlink_node_ao_from_deque("probation", &mut self.probation, tagged_node);
                }
                CacheRegion::MainProtected => {
                    Self::unlink_node_ao_from_deque("protected", &mut self.protected, tagged_node);
                }
                CacheRegion::Other => unreachable!(),
            }
        }
    }

    unsafe fn unlink_node_ao_from_deque(
        deq_name: &str,
        deq: &mut Deque<KeyHashDate<K>>,
        tagged_node: TagNonNull<DeqNode<KeyHashDate<K>>, 2>,
    ) {
        let (node, tag) = tagged_node.decompose();
        let p = node.as_ref();
        assert_eq!(
            deq.region(),
            tag,
            "unlink_node - node is not a member of {deq_name} deque. {p:?}"
        );
        if deq.contains(p) {
            // https://github.com/moka-rs/moka/issues/64
            deq.unlink_and_drop(node);
        }
    }

    pub(crate) fn unlink_node_wo(
        deq: &mut Deque<KeyHashDate<K>>,
        node: NonNull<DeqNode<KeyHashDate<K>>>,
    ) {
        unsafe {
            let p = node.as_ref();
            if deq.contains(p) {
                // https://github.com/moka-rs/moka/issues/64
                deq.unlink_and_drop(node);
            }
        }
    }
}

// TODO: Add tests and run Miri with them.
