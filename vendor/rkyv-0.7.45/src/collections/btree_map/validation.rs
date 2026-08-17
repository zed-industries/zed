//! Validation implementation for BTreeMap.

use super::{
    ArchivedBTreeMap, ClassifiedNode, InnerNode, InnerNodeEntry, LeafNode, LeafNodeEntry, Node,
    NodeHeader, MIN_ENTRIES_PER_INNER_NODE, MIN_ENTRIES_PER_LEAF_NODE,
};
use crate::{
    rel_ptr::RelPtr,
    validation::{ArchiveContext, LayoutRaw},
    Archived, Fallible,
};
use bytecheck::{CheckBytes, Error};
use core::{
    alloc::{Layout, LayoutError},
    convert::{Infallible, TryFrom},
    fmt,
    hint::unreachable_unchecked,
    ptr,
};
use ptr_meta::Pointee;

impl<K, C> CheckBytes<C> for InnerNodeEntry<K>
where
    K: CheckBytes<C>,
    C: ArchiveContext + ?Sized,
    C::Error: Error,
{
    type Error = K::Error;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        RelPtr::manual_check_bytes(ptr::addr_of!((*value).ptr), context)
            .unwrap_or_else(|_| core::hint::unreachable_unchecked());
        K::check_bytes(ptr::addr_of!((*value).key), context)?;

        Ok(&*value)
    }
}

/// An error that can occur while checking a leaf node entry.
#[derive(Debug)]
pub enum LeafNodeEntryError<K, V> {
    /// An error occurred while checking the entry's key.
    KeyCheckError(K),
    /// An error occurred while checking the entry's value.
    ValueCheckError(V),
}

impl<K: fmt::Display, V: fmt::Display> fmt::Display for LeafNodeEntryError<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeafNodeEntryError::KeyCheckError(e) => write!(f, "key check error: {}", e),
            LeafNodeEntryError::ValueCheckError(e) => write!(f, "value check error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl<K: Error + 'static, V: Error + 'static> Error for LeafNodeEntryError<K, V> {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::KeyCheckError(e) => Some(e as &dyn Error),
                Self::ValueCheckError(e) => Some(e as &dyn Error),
            }
        }
    }
};

impl<K, V, C> CheckBytes<C> for LeafNodeEntry<K, V>
where
    K: CheckBytes<C>,
    V: CheckBytes<C>,
    C: Fallible + ?Sized,
    C::Error: Error,
{
    type Error = LeafNodeEntryError<K::Error, V::Error>;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        K::check_bytes(ptr::addr_of!((*value).key), context)
            .map_err(LeafNodeEntryError::KeyCheckError)?;
        V::check_bytes(ptr::addr_of!((*value).value), context)
            .map_err(LeafNodeEntryError::ValueCheckError)?;
        Ok(&*value)
    }
}

/// Errors that can occur while checking an archived B-tree.
#[derive(Debug)]
pub enum ArchivedBTreeMapError<K, V, C> {
    /// An error occurred while checking the bytes of a key
    KeyCheckError(K),
    /// An error occurred while checking the bytes of a value
    ValueCheckError(V),
    /// The number of entries in the inner node is less than the minimum number of entries required
    TooFewInnerNodeEntries(usize),
    /// The number of entries in the leaf node is less than the minimum number of entries
    TooFewLeafNodeEntries(usize),
    /// An error occurred while checking the entries of an inner node
    CheckInnerNodeEntryError {
        /// The index of the inner node entry
        index: usize,
        /// The inner error that occurred
        inner: K,
    },
    /// An error occurred while checking the entries of a leaf node
    CheckLeafNodeEntryError {
        /// The index of the leaf node entry
        index: usize,
        /// The inner error that occurred
        inner: LeafNodeEntryError<K, V>,
    },
    /// The size of an inner node was invalid
    InvalidNodeSize(usize),
    /// The child of an inner node had a first key that did not match the inner node's key
    MismatchedInnerChildKey,
    /// The leaf level of the B-tree contained an inner node
    InnerNodeInLeafLevel,
    /// The leaves of the B-tree were not all located at the same depth
    InvalidLeafNodeDepth {
        /// The depth of the first leaf node in the tree
        expected: usize,
        /// The depth of the invalid leaf node
        actual: usize,
    },
    /// A leaf node did not contain entries in sorted order
    UnsortedLeafNodeEntries,
    /// A leaf node is not linked after a node despite being the next leaf node
    UnlinkedLeafNode,
    /// A leaf node with lesser keys is linked after a leaf node with greater keys
    UnsortedLeafNode,
    /// The forward pointer of the last leaf did not have an offset of 0
    LastLeafForwardPointerNotNull,
    /// The number of entries the B-tree claims to have does not match the actual number of entries
    LengthMismatch {
        /// The number of entries the B-tree claims to have
        expected: usize,
        /// The actual number of entries in the B-tree
        actual: usize,
    },
    /// The keys for an inner node were incorrect
    IncorrectChildKey,
    /// An context error occurred
    ContextError(C),
}

impl<K, V, C> From<Infallible> for ArchivedBTreeMapError<K, V, C> {
    fn from(_: Infallible) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl<K, V, C> fmt::Display for ArchivedBTreeMapError<K, V, C>
where
    K: fmt::Display,
    V: fmt::Display,
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyCheckError(e) => write!(f, "key check error: {}", e),
            Self::ValueCheckError(e) => write!(f, "value check error: {}", e),
            Self::TooFewInnerNodeEntries(n) => write!(
                f,
                "too few inner node entries (expected at least {}): {}",
                MIN_ENTRIES_PER_INNER_NODE, n
            ),
            Self::TooFewLeafNodeEntries(n) => write!(
                f,
                "too few leaf node entries (expected at least {}): {}",
                MIN_ENTRIES_PER_LEAF_NODE, n,
            ),
            Self::CheckInnerNodeEntryError { index, inner } => write!(
                f,
                "inner node entry check error: index {}, error {}",
                index, inner
            ),
            Self::CheckLeafNodeEntryError { index, inner } => write!(
                f,
                "leaf node entry check error: index {}, error {}",
                index, inner
            ),
            Self::InvalidNodeSize(n) => write!(f, "invalid node size: {}", n),
            Self::MismatchedInnerChildKey => write!(f, "mismatched inner child key"),
            Self::InnerNodeInLeafLevel => write!(f, "inner node in leaf level"),
            Self::InvalidLeafNodeDepth { expected, actual } => write!(
                f,
                "expected leaf node depth {} but found leaf node depth {}",
                expected, actual,
            ),
            Self::UnsortedLeafNodeEntries => write!(f, "leaf node contains keys in unsorted order"),
            Self::UnlinkedLeafNode => write!(f, "leaf nodes are not linked in the sorted order"),
            Self::UnsortedLeafNode => write!(f, "leaf nodes are not linked in sorted order"),
            Self::LastLeafForwardPointerNotNull => {
                write!(f, "the forward pointer of the last leaf was not null")
            }
            Self::LengthMismatch { expected, actual } => write!(
                f,
                "expected {} entries but there were actually {} entries",
                expected, actual,
            ),
            Self::IncorrectChildKey => write!(f, "incorrect child key in inner node"),
            Self::ContextError(e) => write!(f, "context error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl<K, V, C> Error for ArchivedBTreeMapError<K, V, C>
    where
        K: Error + 'static,
        V: Error + 'static,
        C: Error + 'static,
    {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::KeyCheckError(e) => Some(e as &dyn Error),
                Self::ValueCheckError(e) => Some(e as &dyn Error),
                Self::TooFewInnerNodeEntries(_) => None,
                Self::TooFewLeafNodeEntries(_) => None,
                Self::CheckInnerNodeEntryError { inner, .. } => Some(inner as &dyn Error),
                Self::CheckLeafNodeEntryError { inner, .. } => Some(inner as &dyn Error),
                Self::InvalidNodeSize(_) => None,
                Self::MismatchedInnerChildKey => None,
                Self::InnerNodeInLeafLevel => None,
                Self::InvalidLeafNodeDepth { .. } => None,
                Self::UnsortedLeafNodeEntries => None,
                Self::UnlinkedLeafNode => None,
                Self::UnsortedLeafNode => None,
                Self::LastLeafForwardPointerNotNull => None,
                Self::LengthMismatch { .. } => None,
                Self::IncorrectChildKey => None,
                Self::ContextError(e) => Some(e as &dyn Error),
            }
        }
    }
};

impl<T> LayoutRaw for Node<[T]> {
    fn layout_raw(metadata: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError> {
        let result = Layout::new::<NodeHeader>()
            .extend(Layout::array::<T>(metadata).unwrap())?
            .0;
        #[cfg(not(feature = "strict"))]
        {
            Ok(result)
        }
        #[cfg(feature = "strict")]
        {
            Ok(result.pad_to_align())
        }
    }
}

type ABTMError<K, V, C> = ArchivedBTreeMapError<
    <K as CheckBytes<C>>::Error,
    <V as CheckBytes<C>>::Error,
    <C as Fallible>::Error,
>;

impl NodeHeader {
    #[inline]
    unsafe fn manual_check_bytes<'a, K, V, C>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, ABTMError<K, V, C>>
    where
        K: CheckBytes<C>,
        V: CheckBytes<C>,
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        let raw_node = Self::manual_check_header(value, context)
            .map_err(ArchivedBTreeMapError::ContextError)?;

        let node_layout = if raw_node.is_inner() {
            InnerNode::<K>::layout_raw(ptr_meta::metadata(raw_node.classify_inner_ptr::<K>()))
                .map_err(C::wrap_layout_error)
                .map_err(ArchivedBTreeMapError::ContextError)?
        } else {
            LeafNode::<K, V>::layout_raw(ptr_meta::metadata(raw_node.classify_leaf_ptr::<K, V>()))
                .map_err(C::wrap_layout_error)
                .map_err(ArchivedBTreeMapError::ContextError)?
        };

        context
            .bounds_check_subtree_ptr_layout((raw_node as *const NodeHeader).cast(), &node_layout)
            .map_err(ArchivedBTreeMapError::ContextError)?;

        Self::manual_check_contents::<K, V, C>(raw_node, context)?;

        Ok(raw_node)
    }

    #[inline]
    unsafe fn manual_check_header<'a, C>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, C::Error>
    where
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        CheckBytes::check_bytes(ptr::addr_of!((*value).meta), context).map_err(
            // SAFETY: Infallible cannot exist
            |_: Infallible| unreachable_unchecked(),
        )?;
        CheckBytes::check_bytes(ptr::addr_of!((*value).size), context).map_err(
            // SAFETY: Infallible cannot exist
            |_: Infallible| unreachable_unchecked(),
        )?;
        RelPtr::manual_check_bytes(ptr::addr_of!((*value).ptr), context).map_err(
            // SAFETY: Infallible cannot exist
            |_: Infallible| unreachable_unchecked(),
        )?;

        // All the fields have been checked and this is a valid RawNode
        Ok(&*value)
    }

    #[inline]
    unsafe fn manual_check_contents<K, V, C>(
        raw_node: &Self,
        context: &mut C,
    ) -> Result<(), ABTMError<K, V, C>>
    where
        K: CheckBytes<C>,
        V: CheckBytes<C>,
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        // Now that the fields have been checked, we can start checking the specific subtype
        let root = (raw_node as *const Self).cast();
        let size = from_archived!(raw_node.size) as usize;
        let offset =
            -isize::try_from(size).map_err(|_| ArchivedBTreeMapError::InvalidNodeSize(size))?;
        let start = context
            .check_ptr(root, offset, ())
            .map_err(ArchivedBTreeMapError::ContextError)?;

        // Push a new suffix range and check the inner or leaf part
        let range = context
            .push_suffix_subtree_range(start, root)
            .map_err(ArchivedBTreeMapError::ContextError)?;
        if raw_node.is_inner() {
            InnerNode::manual_check_bytes::<V, C>(raw_node.classify_inner_ptr::<K>(), context)?;
        } else {
            CheckBytes::check_bytes(raw_node.classify_leaf_ptr::<K, V>(), context)?;
        }
        context
            .pop_suffix_range(range)
            .map_err(ArchivedBTreeMapError::ContextError)?;

        Ok(())
    }
}

impl<K> InnerNode<K> {
    #[allow(clippy::type_complexity)]
    fn verify_integrity<'a, V, C>(
        &'a self,
    ) -> Result<&K, ArchivedBTreeMapError<K::Error, V::Error, C::Error>>
    where
        K: CheckBytes<C> + PartialEq,
        V: CheckBytes<C> + 'a,
        C: Fallible + ?Sized,
    {
        for entry in self.tail.iter() {
            let child = unsafe { &*entry.ptr.as_ptr() }.classify::<K, V>();
            let first_key = match child {
                ClassifiedNode::Inner(c) => c.verify_integrity::<V, C>()?,
                ClassifiedNode::Leaf(c) => &c.tail[0].key,
            };
            if !entry.key.eq(first_key) {
                return Err(ArchivedBTreeMapError::IncorrectChildKey);
            }
        }

        let least_child = unsafe { &*self.header.ptr.as_ptr() }.classify::<K, V>();
        let first_key = match least_child {
            ClassifiedNode::Inner(c) => c.verify_integrity::<V, C>()?,
            ClassifiedNode::Leaf(c) => &c.tail[0].key,
        };

        Ok(first_key)
    }

    #[inline]
    unsafe fn manual_check_bytes<'a, V, C>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, ABTMError<K, V, C>>
    where
        K: CheckBytes<C>,
        V: CheckBytes<C>,
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        // meta, size, and ptr have already been checked by the check_bytes for RawNode
        let len = ptr_meta::metadata(value);

        // Each inner node actually contains one more entry that the length indicates (the least
        // child pointer)
        if len + 1 < MIN_ENTRIES_PER_INNER_NODE {
            return Err(ArchivedBTreeMapError::TooFewInnerNodeEntries(len + 1));
        }

        // The subtree range has already been set up for us so we can just check our tail
        let tail_ptr = ptr::addr_of!((*value).tail) as *const InnerNodeEntry<K>;
        for index in (0..len).rev() {
            CheckBytes::check_bytes(tail_ptr.add(index), context).map_err(|inner| {
                ArchivedBTreeMapError::CheckInnerNodeEntryError { index, inner }
            })?;
        }

        Ok(&*value)
    }
}

impl<K, V, C> CheckBytes<C> for LeafNode<K, V>
where
    K: CheckBytes<C>,
    V: CheckBytes<C>,
    C: ArchiveContext + ?Sized,
    C::Error: Error,
{
    type Error = ArchivedBTreeMapError<K::Error, V::Error, C::Error>;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        // meta, size, and ptr have already been checked by the check_bytes for RawNode
        let len = ptr_meta::metadata(value);

        if len < MIN_ENTRIES_PER_LEAF_NODE {
            return Err(ArchivedBTreeMapError::TooFewLeafNodeEntries(len));
        }

        // The subtree range has already been set up for us so we can just check our tail
        let tail_ptr = ptr::addr_of!((*value).tail) as *const LeafNodeEntry<K, V>;
        for index in (0..len).rev() {
            CheckBytes::check_bytes(tail_ptr.add(index), context)
                .map_err(|inner| ArchivedBTreeMapError::CheckLeafNodeEntryError { index, inner })?;
        }

        Ok(&*value)
    }
}

#[cfg(feature = "alloc")]
const _: () = {
    #[cfg(not(feature = "std"))]
    use alloc::collections::VecDeque;
    #[cfg(feature = "std")]
    use std::collections::VecDeque;

    impl<K, V, C> CheckBytes<C> for ArchivedBTreeMap<K, V>
    where
        K: CheckBytes<C> + Ord,
        V: CheckBytes<C>,
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        type Error = ArchivedBTreeMapError<K::Error, V::Error, C::Error>;

        unsafe fn check_bytes<'a>(
            value: *const Self,
            context: &mut C,
        ) -> Result<&'a Self, Self::Error> {
            let len = from_archived!(*Archived::<usize>::check_bytes(
                ptr::addr_of!((*value).len),
                context,
            )?) as usize;

            if len > 0 {
                let root_rel_ptr =
                    RelPtr::manual_check_bytes(ptr::addr_of!((*value).root), context)?;

                // Walk all the inner nodes, claim their memory, and check their contents
                let mut nodes = VecDeque::new();
                let root_ptr = context
                    .check_subtree_rel_ptr(root_rel_ptr)
                    .map_err(ArchivedBTreeMapError::ContextError)?;

                // Before checking all the nodes, we have to push an additional prefix subtree with
                // the root node
                // Otherwise, when the suffix subtree of the root node is popped it will remove any
                // trailing suffix space that should be checked by subsequent fields
                let root = NodeHeader::manual_check_header(root_ptr, context)
                    .map_err(ArchivedBTreeMapError::ContextError)?;

                // To push the subtree, we need to know the real size of the root node
                // Since the header is checked, we can just classify the pointer and use layout_raw
                let root_layout = if root.is_inner() {
                    InnerNode::<K>::layout_raw(ptr_meta::metadata(root.classify_inner_ptr::<K>()))
                        .map_err(C::wrap_layout_error)
                        .map_err(ArchivedBTreeMapError::ContextError)?
                } else {
                    LeafNode::<K, V>::layout_raw(ptr_meta::metadata(
                        root.classify_leaf_ptr::<K, V>(),
                    ))
                    .map_err(C::wrap_layout_error)
                    .map_err(ArchivedBTreeMapError::ContextError)?
                };

                // Because the layout of the subtree is dynamic, we need to bounds check the layout
                // declared by the root node.
                context
                    .bounds_check_subtree_ptr_layout(root_ptr.cast(), &root_layout)
                    .map_err(ArchivedBTreeMapError::ContextError)?;

                // Now we can push the prefix subtree range.
                let nodes_range = context
                    .push_prefix_subtree_range(
                        root_ptr.cast(),
                        root_ptr.cast::<u8>().add(root_layout.size()),
                    )
                    .map_err(ArchivedBTreeMapError::ContextError)?;

                // Now we're finally ready to check node subtrees
                NodeHeader::manual_check_contents::<K, V, C>(root, context)?;

                nodes.push_back((root, 0));

                while let Some(&(node, depth)) = nodes.front() {
                    // Break when a leaf is found
                    if !node.is_inner() {
                        break;
                    }
                    nodes.pop_front();
                    let inner = node.classify_inner::<K>();

                    let child_ptr = context
                        .check_subtree_rel_ptr(&inner.header.ptr)
                        .map_err(ArchivedBTreeMapError::ContextError)?;
                    let child = NodeHeader::manual_check_bytes::<K, V, C>(child_ptr, context)?;
                    nodes.push_back((child, depth + 1));

                    // The invariant that this node contains keys less than the first key of this node will
                    // be checked when we iterate through the leaf nodes in order and check ordering
                    for entry in inner.tail.iter() {
                        let child_ptr = context
                            .check_subtree_rel_ptr(&entry.ptr)
                            .map_err(ArchivedBTreeMapError::ContextError)?;
                        let child = NodeHeader::manual_check_bytes::<K, V, C>(child_ptr, context)?;
                        nodes.push_back((child, depth + 1));
                    }
                }

                // We're done checking node subtrees now
                context
                    .pop_prefix_range(nodes_range)
                    .map_err(ArchivedBTreeMapError::ContextError)?;

                // The remaining nodes must all be leaf nodes
                let mut entry_count = 0;
                for (node, depth) in nodes.iter() {
                    if !node.is_leaf() {
                        return Err(ArchivedBTreeMapError::InnerNodeInLeafLevel);
                    }
                    let leaf = node.classify_leaf::<K, V>();

                    // Leaf nodes must all be the same depth
                    let expected_depth = nodes.front().unwrap().1;
                    if *depth != expected_depth {
                        return Err(ArchivedBTreeMapError::InvalidLeafNodeDepth {
                            expected: expected_depth,
                            actual: *depth,
                        });
                    }

                    // They must contain entries in sorted order
                    for (prev, next) in leaf.tail.iter().zip(leaf.tail.iter().skip(1)) {
                        if next.key < prev.key {
                            return Err(ArchivedBTreeMapError::UnsortedLeafNodeEntries);
                        }
                    }

                    // Keep track of the number of entries found
                    entry_count += leaf.tail.len();
                }

                for (i, (node, _)) in nodes.iter().enumerate() {
                    let leaf = node.classify_leaf::<K, V>();

                    // And they must link together in sorted order
                    if i < nodes.len() - 1 {
                        let next_ptr = context
                            .check_rel_ptr(&leaf.header.ptr)
                            .map_err(ArchivedBTreeMapError::ContextError)?;
                        let next_node = nodes[i + 1].0.classify_leaf();

                        if next_ptr != (next_node as *const LeafNode<K, V>).cast() {
                            return Err(ArchivedBTreeMapError::UnlinkedLeafNode);
                        }
                        if next_node.tail[0].key < leaf.tail[leaf.tail.len() - 1].key {
                            return Err(ArchivedBTreeMapError::UnsortedLeafNode);
                        }
                    } else {
                        // The last node must have a null pointer forward
                        if !leaf.header.ptr.is_null() {
                            return Err(ArchivedBTreeMapError::LastLeafForwardPointerNotNull);
                        }
                    }
                }

                // Make sure that the number of entries matches the length
                if entry_count != len {
                    return Err(ArchivedBTreeMapError::LengthMismatch {
                        expected: len,
                        actual: entry_count,
                    });
                }

                // Make sure that inner nodes are constructed appropriately
                if root.is_inner() {
                    root.classify_inner::<K>().verify_integrity::<V, C>()?;
                }
            }

            Ok(&*value)
        }
    }
};
